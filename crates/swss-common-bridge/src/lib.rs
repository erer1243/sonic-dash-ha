pub mod payload;
pub mod table_info;
mod table_watcher;

use swbus_actor::prelude::*;
use table_watcher::{TableWatcher, TableWatcherMessage};
use tokio::sync::{
    mpsc::{channel, Sender},
    oneshot,
};
use tokio_util::task::AbortOnDropHandle;

/// A bridge that converts between Swbus messages and swss tables.
pub struct SwssCommonBridge {
    outbox_tx: Option<oneshot::Sender<Outbox>>,
    tw_msg_tx: Sender<TableWatcherMessage>,
    _table_watcher_task: AbortOnDropHandle<()>,
}

impl SwssCommonBridge {
    pub fn new() -> Self {
        let (tw_msg_tx, tw_msg_rx) = channel(1024);
        let (outbox_tx, outbox_rx) = oneshot::channel();
        let _table_watcher_task = AbortOnDropHandle::new(tokio::spawn(TableWatcher::run(outbox_rx, tw_msg_rx)));
        Self {
            tw_msg_tx,
            outbox_tx: Some(outbox_tx),
            _table_watcher_task,
        }
    }
}

impl Actor for SwssCommonBridge {
    async fn init(&mut self, outbox: Outbox) {
        self.outbox_tx
            .take()
            .unwrap()
            .send(outbox.clone())
            .unwrap_or_else(|_| unreachable!("outbox_tx.send failed"));
    }

    async fn handle_message(&mut self, message: IncomingMessage, outbox: Outbox) {
        match &message.body {
            MessageBody::Request(req) => match payload::decode_table_watcher_message(&req.payload) {
                Ok(tw_msg) => {
                    self.tw_msg_tx.send(tw_msg).await.expect("TableWatcher task died");
                    let msg = OutgoingMessage::ok_response(message);
                    outbox.send(msg).await;
                }
                Err(e) => {
                    let msg = OutgoingMessage::error_response(message, SwbusErrorCode::InvalidPayload, e.to_string());
                    outbox.send(msg).await;
                }
            },
            MessageBody::Response(_) => (),
        }
    }

    async fn handle_message_failure(&mut self, _id: MessageId, _outbox: Outbox) {}
}
