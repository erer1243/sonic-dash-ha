pub mod state;

use crate::swss_bridge::{Key, OwnedKey};
use state::State;
use std::{collections::HashSet, future::Future, time::Duration};
use swbus_edge::{
    simple_client::{IncomingMessage, SimpleSwbusEdgeClient},
    swbus_proto::swbus::{ServicePath, SwbusMessage},
};
use tokio::{select, time::Interval};

/// `Box<dyn Error + ...>`
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// The main callbacks an actor must implement.
pub trait Actor {
    fn handle_request(
        &mut self,
        state: &mut State,
        outbox: &mut Outbox,
        source: ServicePath,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    fn handle_table_update(
        &mut self,
        state: &mut State,
        outbox: &mut Outbox,
        key: Key<'_>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

/// Outgoing messages, which will be sent when the current callback returns.
pub struct Outbox {
    outgoing_messages: Vec<()>,
}

// Drives a single actor
pub(crate) struct ActorDriver<A> {
    /// The actor being driven
    actor: A,

    /// The actor's state, i.e. its local copies of swss tables
    state: State,

    /// Interval at which maintenence will trigger
    maintenence_interval: Interval,

    /// Messages that need to be resent during maintenence
    unacked_outgoing_messages: Vec<SwbusMessage>,

    /// Tables that were updated, but where handle_table_update failed
    failed_table_updates: HashSet<OwnedKey>,

    /// Swbus client
    swbus_client: SimpleSwbusEdgeClient,
}

impl<A: Actor> ActorDriver<A> {
    pub(crate) fn new(actor: A, maintenence_period: Duration) -> Self {
        let mut maintenence_interval = tokio::time::interval(maintenence_period);
        maintenence_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        todo!()
    }

    pub(crate) async fn run(&mut self) {
        loop {
            select! {
                _ = self.maintenence_interval.tick() => {
                    self.maintenence().await;
                }

                maybe_msg = self.swbus_client.recv() => {
                    let msg = maybe_msg.expect("Swbus error");
                    self.handle_swbus_message(msg).await;
                }
            }
        }
    }

    async fn maintenence(&mut self) {}

    async fn handle_swbus_message(&mut self, msg: IncomingMessage) {}
}
