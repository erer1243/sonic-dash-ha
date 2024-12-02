use std::{sync::Arc, time::Duration};
use swbus_actor::prelude::*;

#[tokio::main]
async fn main() {
    let bind_addr = ServicePath {
        region_id: "region_a".into(),
        cluster_id: "switch-cluster-a".into(),
        node_id: "10.0.0.1".into(),
        service_type: "hamgrd".into(),
        service_id: "dpu0".into(),
        resource_type: "hascope".into(),
        resource_id: "dpu".into(),
    };

    let resend_config = ResendQueueConfig {
        resend_time: Duration::from_millis(500),
        max_tries: 120,
    };

    let mut swbus_edge = SwbusEdgeRuntime::new("localhost:8000".to_string());
    swbus_edge.start().await.unwrap();
    let mut runtime = ActorRuntime::new(Arc::new(swbus_edge), resend_config);
    runtime.spawn(bind_addr, TestActor).await;
    runtime.join().await;
}

struct TestActor;

impl Actor for TestActor {
    async fn init(&mut self, _outbox: Outbox) {
        println!("TestActor started");
    }

    async fn handle_message(&mut self, msg: IncomingMessage, _outbox: Outbox) {
        println!("Received message {msg:?}")
    }

    async fn handle_message_failure(&mut self, id: u64, _destination: ServicePath, _outbox: Outbox) {
        println!("Message failed to send: {id:?}");
    }
}
