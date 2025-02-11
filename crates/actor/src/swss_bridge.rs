mod key;
mod table;

use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use std::{borrow::Borrow, collections::HashMap};
use swss_common::{ConsumerStateTable, FieldValues, ProducerStateTable, SubscriberStateTable, ZmqConsumerStateTable};
use table::{InputTable, OutputTable};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub use key::{Key, OwnedKey};
pub(crate) use key::{OwnedTableId, TableId};

pub(crate) struct SwssBridge {
    input_tables: HashMap<OwnedTableId, InputTable>,
    input_table_update_senders: HashMap<OwnedKey, Sender<TableUpdate>>,

    output_tables: HashMap<OwnedTableId, OutputTable>,
    output_table_update_receiver: Receiver<TableUpdate>,
    output_table_update_sender: Sender<TableUpdate>,
}

impl SwssBridge {
    pub(crate) fn new() -> Self {
        let (output_table_update_sender, output_table_update_receiver) = channel(1000);
        Self {
            input_tables: HashMap::new(),
            input_table_update_senders: HashMap::new(),
            output_tables: HashMap::new(),
            output_table_update_receiver,
            output_table_update_sender,
        }
    }

    pub(crate) fn add_input_table(&mut self, id: OwnedTableId, t: impl Into<InputTable>) {
        self.input_tables.insert(id, t.into());
    }

    pub(crate) fn add_output_table(&mut self, id: OwnedTableId, t: impl Into<OutputTable>) {
        self.output_tables.insert(id, t.into());
    }

    pub(crate) fn client(&mut self, input_subscriptions: &[OwnedKey]) {
        for k in input_subscriptions.into_iter().cloned() {
            self.input_table_update_senders.insert(k, todo!());
        }
    }

    pub(crate) async fn run(&mut self) {}

    async fn input_table_update(&mut self) -> OwnedTableId {
        let mut futs: FuturesUnordered<_> = self
            .input_tables
            .iter_mut()
            .map(|(id, t)| t.read_data().map(|res| (id.clone(), res)))
            .collect();

        match futs.next().await {
            Some((id, res)) => {
                res.unwrap_or_else(|e| panic!("IO error waiting for update on table {id}: {e}"));
                id
            }
            None => futures::future::pending().await,
        }
    }
}

pub(crate) struct TableUpdate {
    pub(crate) key: OwnedKey,
    pub(crate) fvs: FieldValues,
}

pub(crate) struct SwssBridgeClient {
    tx: Sender<TableUpdate>,
    rx: Receiver<TableUpdate>,
}

impl SwssBridgeClient {
    pub(crate) async fn send(&self, value: TableUpdate) {
        self.tx.send(value).await.expect("SWSS bridge is down")
    }

    pub(crate) async fn recv(&mut self) -> TableUpdate {
        self.rx.recv().await.expect("SWSS bridge is down")
    }
}
