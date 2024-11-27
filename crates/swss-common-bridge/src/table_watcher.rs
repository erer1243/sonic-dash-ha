use futures::{future::select_all, FutureExt};
use serde::{ser::SerializeStructVariant, Deserialize, Serialize};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    future::Future,
    hash::Hash,
    sync::Arc,
};
use swbus_actor::prelude::*;
use swss_common::{
    ConsumerStateTable, DbConnector, KeyOpFieldValues, SubscriberStateTable, ZmqConsumerStateTable, ZmqServer,
};
use tokio::sync::{mpsc::Receiver, oneshot};

use crate::table_info::{RedisTableInfo, TableInfo, ZmqTableInfo};

#[derive(Serialize, Deserialize)]
pub(crate) enum TableWatcherMessage {
    Subscribe {
        subscriber: ServicePath,
        table_info: TableInfo,
    },
}

pub(crate) struct TableWatcher {
    outbox: LazyOutbox,
    tw_msg_rx: Receiver<TableWatcherMessage>,

    csts: HashMap<Arc<RedisTableInfo>, TableSubscribers<ConsumerStateTable>>,
    ssts: HashMap<Arc<RedisTableInfo>, TableSubscribers<SubscriberStateTable>>,
    zcsts: HashMap<Arc<ZmqTableInfo>, TableSubscribers<(ZmqServer, ZmqConsumerStateTable)>>,
}

impl TableWatcher {
    pub(crate) async fn run(outbox_rx: oneshot::Receiver<Outbox>, tw_msg_rx: Receiver<TableWatcherMessage>) {
        let self_ = Self {
            outbox: LazyOutbox::Waiting(outbox_rx),
            tw_msg_rx,
            csts: HashMap::new(),
            ssts: HashMap::new(),
            zcsts: HashMap::new(),
        };
        self_.main().await;
    }

    async fn main(mut self) {
        loop {
            let sst_futures = self
                .ssts
                .iter_mut()
                .map(|(key, ts)| Box::pin(ts.table.read_data_async().map(move |res| (key, res))));
            let cst_futures = self
                .csts
                .iter_mut()
                .map(|(key, ts)| Box::pin(ts.table.read_data_async().map(move |res| (key, res))));
            let zcst_futures = self
                .zcsts
                .iter_mut()
                .map(|(key, ts)| Box::pin(ts.table.1.read_data_async().map(move |res| (key, res))));

            let sst_read_data = select_all(sst_futures);
            let cst_read_data = select_all(cst_futures);
            let zcst_read_data = select_all(zcst_futures);

            enum Action {
                HandleTWMessage(TableWatcherMessage),
                HandleCSTRead(Arc<RedisTableInfo>),
                HandleSSTRead(Arc<RedisTableInfo>),
                HandleZCSTRead(Arc<ZmqTableInfo>),
            }

            let action = tokio::select! {
                msg = self.tw_msg_rx.recv() => {
                    // If msg is None, the `SwssCommonBridge` actor must have exited, so we should exit too.
                    let Some(msg) = msg else { return };
                    Action::HandleTWMessage(msg)
                }

                ((ti, res), _, _) = sst_read_data => {
                    res.expect("SubscriberStateTable::read_data_async failed");
                    Action::HandleSSTRead(ti.clone())
                }

                ((ti, res), _, _) = cst_read_data => {
                    res.expect("ConsumerStateTable::read_data_async failed");
                    Action::HandleCSTRead(ti.clone())
                }

                ((ti, res), _, _) = zcst_read_data => {
                    res.expect("ZmqConsumerStateTable::read_data_async failed");
                    Action::HandleZCSTRead(ti.clone())
                }
            };

            match action {
                Action::HandleTWMessage(m) => self.handle_tw_msg(m).await,
                Action::HandleCSTRead(ti) => self.handle_cst_read(&ti).await,
                Action::HandleSSTRead(ti) => self.handle_sst_read(&ti).await,
                Action::HandleZCSTRead(ti) => self.handle_zcst_read(&ti).await,
            }
        }
    }

    async fn handle_tw_msg(&mut self, msg: TableWatcherMessage) {
        async fn get_or_insert_with_key<K, V, Fut: Future<Output = V>>(
            map: &mut HashMap<K, V>,
            key: K,
            or_insert: impl FnOnce(K) -> Fut,
        ) -> &mut V
        where
            K: Eq + Hash + Clone,
        {
            match map.entry(key.clone()) {
                Entry::Occupied(oe) => oe.into_mut(),
                Entry::Vacant(ve) => ve.insert(or_insert(key).await),
            }
        }

        async fn new_cst(table_info: &RedisTableInfo) -> ConsumerStateTable {
            let db_connection_info = table_info.db_connection_info.clone();
            let db = DbConnector::new_async(table_info.db_id, db_connection_info, 0).await;
            ConsumerStateTable::new_async(db, &table_info.table_name, None, None).await
        }

        async fn new_sst(table_info: &RedisTableInfo) -> SubscriberStateTable {
            let db_connection_info = table_info.db_connection_info.clone();
            let db = DbConnector::new_async(table_info.db_id, db_connection_info, 0).await;
            SubscriberStateTable::new_async(db, &table_info.table_name, None, None).await
        }

        async fn new_zcst(table_info: &ZmqTableInfo) -> (ZmqServer, ZmqConsumerStateTable) {
            let mut zmq_server = ZmqServer::new_async(&table_info.zmq_endpoint).await;
            let db_connection_info = table_info.db_connection_info.clone();
            let db = DbConnector::new_async(table_info.db_id, db_connection_info, 0).await;
            let zcst = ZmqConsumerStateTable::new_async(db, &table_info.table_name, &mut zmq_server, None, None).await;
            (zmq_server, zcst)
        }

        match msg {
            TableWatcherMessage::Subscribe { subscriber, table_info } => match table_info {
                TableInfo::ConsumerStateTable(table_info) => {
                    let table_subscribers =
                        get_or_insert_with_key(&mut self.csts, Arc::new(table_info), |table_info| async move {
                            TableSubscribers::new(new_cst(&table_info).await)
                        })
                        .await;
                    table_subscribers.subscribers.insert(subscriber);
                }
                TableInfo::SubscriberStateTable(table_info) => {
                    let table_subscribers =
                        get_or_insert_with_key(&mut self.ssts, Arc::new(table_info), |table_info| async move {
                            TableSubscribers::new(new_sst(&table_info).await)
                        })
                        .await;
                    table_subscribers.subscribers.insert(subscriber);
                }
                TableInfo::ZmqConsumerStateTable(table_info) => {
                    let table_subscribers =
                        get_or_insert_with_key(&mut self.zcsts, Arc::new(table_info), |table_info| async move {
                            TableSubscribers::new(new_zcst(&table_info).await)
                        })
                        .await;
                    table_subscribers.subscribers.insert(subscriber);
                }
            },
        }
    }

    async fn handle_cst_read(&mut self, ti: &RedisTableInfo) {
        let table_subs = self.csts.get(ti).unwrap();
    }

    async fn handle_sst_read(&mut self, ti: &RedisTableInfo) {
        todo!()
    }

    async fn handle_zcst_read(&mut self, ti: &ZmqTableInfo) {
        todo!()
    }
}

enum LazyOutbox {
    Waiting(oneshot::Receiver<Outbox>),
    Received(Outbox),
}

impl LazyOutbox {
    async fn get(&mut self) -> &Outbox {
        match self {
            LazyOutbox::Waiting(receiver) => {
                let outbox = receiver.await.unwrap();
                *self = LazyOutbox::Received(outbox);
                let LazyOutbox::Received(outbox) = self else {
                    unreachable!()
                };
                outbox
            }
            LazyOutbox::Received(outbox) => outbox,
        }
    }
}

struct TableSubscribers<T> {
    table: T,
    subscribers: HashSet<ServicePath>,
}

impl<T> TableSubscribers<T> {
    fn new(table: T) -> Self {
        Self {
            table,
            subscribers: HashSet::new(),
        }
    }
}

// enum Table {
//     ConsumerStateTable(ConsumerStateTable),
//     SubscriberStateTable(SubscriberStateTable),
//     ZmqConsumerStateTable(ZmqConsumerStateTable),
// }

// impl Table {
//     async fn connect_async(ti: TableInfo) -> Self {
//         let db = DbConnector::new_async(ti.db_id, ti.db_connection_info, 10000).await;
//         match ti.table_type {
//             TableType::ConsumerStateTable => {
//                 Table::ConsumerStateTable(ConsumerStateTable::new(db, &ti.table_name, None, None))
//             }
//             TableType::SubscriberStateTable => {
//                 Table::SubscriberStateTable(SubscriberStateTable::new(db, &ti.table_name, None, None))
//             }
//             TableType::ZmqConsumerStateTable => {
//                 let zmqs = ZmqServer::new();
//                 Table::ZmqConsumerStateTable(ZmqConsumerStateTable::new(db, &ti.table_name))
//             }
//         }
//     }

//     async fn read_data_async(&mut self) -> std::io::Result<()> {
//         match self {
//             Table::ConsumerStateTable(cst) => cst.read_data_async().await,
//             Table::SubscriberStateTable(sst) => sst.read_data_async().await,
//             Table::ZmqConsumerStateTable(zcst) => zcst.read_data_async().await,
//         }
//     }

//     async fn pops(&self) -> Vec<KeyOpFieldValues> {
//         match self {
//             Table::ConsumerStateTable(cst) => cst.pops(),
//             Table::SubscriberStateTable(sst) => sst.pops(),
//             Table::ZmqConsumerStateTable(zcst) => zcst.pops(),
//         }
//     }
// }
