use crate::{oneshot_lazy, payload, table::Table};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use swbus_actor::prelude::*;
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize)]
pub(crate) enum TableWatcherMessage {
    Subscribe { subscriber: ServicePath },
    Unsubscribe { subscriber: ServicePath },
}

pub(crate) struct TableWatcher<T> {
    outbox_rx: oneshot_lazy::Receiver<Outbox>,
    tw_msg_rx: mpsc::Receiver<TableWatcherMessage>,
    table: T,
    subscribers: HashSet<ServicePath>,
}

impl<T: Table> TableWatcher<T> {
    pub(crate) fn new(
        table: T,
        outbox_rx: oneshot_lazy::Receiver<Outbox>,
        tw_msg_rx: mpsc::Receiver<TableWatcherMessage>,
    ) -> Self {
        Self {
            outbox_rx,
            tw_msg_rx,
            table,
            subscribers: HashSet::new(),
        }
    }

    pub(crate) async fn run(mut self) {
        loop {
            tokio::select! {
                maybe_msg = self.tw_msg_rx.recv() => {
                    let Some(msg) = maybe_msg else { break };
                    self.handle_tw_msg(msg).await;
                },

                () = self.table.read_data_async() => {
                    self.handle_table_read().await;
                }
            }
        }
    }

    async fn handle_tw_msg(&mut self, msg: TableWatcherMessage) {
        match msg {
            TableWatcherMessage::Subscribe { subscriber } => {
                self.subscribers.insert(subscriber);
            }

            TableWatcherMessage::Unsubscribe { subscriber } => {
                self.subscribers.remove(&subscriber);
            }
        }
    }

    async fn handle_table_read(&mut self) {
        let kfvs = self.table.pops();
        for kfv in kfvs {
            let payload = payload::encode_kfvs(&kfv);
            let outbox = self.outbox_rx.get().await;

            for sub in &self.subscribers {
                outbox
                    .send(OutgoingMessage::request(sub.clone(), payload.clone()))
                    .await;
            }
        }
    }
}
