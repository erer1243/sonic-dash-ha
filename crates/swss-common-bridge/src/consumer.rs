use crate::encoding::encode;
use std::{future::Future, sync::Arc};
use swbus_edge::{
    simple_client::{MessageBody, OutgoingMessage, SimpleSwbusEdgeClient},
    swbus_proto::swbus::ServicePath,
    SwbusEdgeRuntime,
};
use swss_common::{ConsumerStateTable, KeyOpFieldValues, SubscriberStateTable, ZmqConsumerStateTable};
use tokio::task::JoinHandle;

pub fn spawn_consumer_bridge<T, F>(
    rt: Arc<SwbusEdgeRuntime>,
    source: ServicePath,
    mut table: T,
    mut dest_generator: F,
) -> JoinHandle<()>
where
    T: ConsumerTable,
    F: FnMut(&KeyOpFieldValues) -> ServicePath + Send + 'static,
{
    let simple_client = SimpleSwbusEdgeClient::new(rt, source, false);
    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                _ = table.read_data() => {
                    for kfvs in table.pops().await {
                        let destination = dest_generator(&kfvs);
                        let payload = encode(&kfvs);
                        simple_client
                            .send(OutgoingMessage { destination, body: MessageBody::Request { payload } })
                            .await
                            .expect("Sending swbus message");
                    }
                }

                // Ignore all messages received.
                // It is a programming error to send a request to a consumer table.
                // Responses are ignored because we don't resend updates if the receiver fails.
                maybe_msg = simple_client.recv() => {
                    if maybe_msg.is_none() {
                        // Swbus shut down, we might as well quit.
                        break;
                    }
                }
            }
        }
    })
}

pub trait ConsumerTable: Send + 'static {
    fn read_data(&mut self) -> impl Future<Output = ()> + Send;
    fn pops(&mut self) -> impl Future<Output = Vec<KeyOpFieldValues>> + Send;
}

macro_rules! impl_consumertable {
    ($($t:ty)*) => {
        $(impl ConsumerTable for $t {
            async fn read_data(&mut self) {
                <$t>::read_data_async(self)
                    .await
                    .expect(concat!(stringify!($t::read_data_async), " io error"));
            }

            async fn pops(&mut self) -> Vec<KeyOpFieldValues> {
                <$t>::pops_async(self)
                    .await
                    .expect(concat!(stringify!($t::pops_async), " threw an exception"))
            }
        })*
    };
}

impl_consumertable! { ConsumerStateTable SubscriberStateTable ZmqConsumerStateTable }

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use swbus_edge::{swbus_proto::swbus::ServicePath, SwbusEdgeRuntime};
    use swss_common::{ConsumerStateTable, ProducerStateTable};
    use swss_common_testing::Redis;

    #[tokio::test]
    async fn consumer_state_table_bridge() {
        let redis = Redis::start();
        let mut swbus_edge = SwbusEdgeRuntime::new(
            "<none>".to_string(),
            ServicePath::from_string("test.test.test/test/test/test/test").unwrap(),
        );
        swbus_edge.start().await.unwrap();
        let rt = Arc::new(swbus_edge);
        let pst = ProducerStateTable::new(redis.db_connector(), "mytable").unwrap();
        let cst = ConsumerStateTable::new(redis.db_connector(), "mytable", None, None).unwrap();
    }
}
