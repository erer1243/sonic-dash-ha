mod messages;
mod tables;

// use std::{sync::Arc, time::Duration};
// use swbus_actor::prelude::*;
// use swss_common::{DbConnector, SubscriberStateTable, ZmqConsumerStateTable, ZmqServer};

#[tokio::main]
async fn main() {
    /*
    let resend_config = ResendQueueConfig {
        resend_time: Duration::from_millis(3000),
        max_tries: 20,
    };

    let mut swbus_edge = SwbusEdgeRuntime::new("localhost:8000".to_string());
    swbus_edge.start().await.unwrap();
    let mut runtime = ActorRuntime::new(Arc::new(swbus_edge), resend_config);

    let mut zmqs = ZmqServer::new_async("localhost:8000").await;
    // Upstream config programming state tables
    // https://github.com/r12f/SONiC/blob/user/r12f/ha2/doc/smart-switch/high-availability/smart-switch-ha-detailed-design.md#11-upstream-config-programming-path
    {
        const TIMEOUT: u32 = 10000;

        let mut config_db = DbConnector::new_named_async("CONFIG_DB", true, TIMEOUT).await;
        let mut appl_db = DbConnector::new_named_async("APPL_DB", true, TIMEOUT).await;

        let bind_addr_todo = ServicePath {
            region_id: "".into(),
            cluster_id: "".into(),
            node_id: "".into(),
            service_type: "".into(),
            service_id: "".into(),
            resource_type: "".into(),
            resource_id: "".into(),
        };

        let dpu_sst =
            SubscriberStateTable::new_async(config_db.clone_timeout_async(TIMEOUT).await, "DPU", None, None).await;
        let dpu_sst_bridge = SwssCommonBridge::new_subscriber_state_table(dpu_sst);
        runtime.spawn(bind_addr_todo.clone(), dpu_sst_bridge).await;

        let vdpu_sst =
            SubscriberStateTable::new_async(config_db.clone_timeout_async(TIMEOUT).await, "VDPU", None, None).await;
        let vdpu_sst_bridge = SwssCommonBridge::new_subscriber_state_table(vdpu_sst);
        runtime.spawn(bind_addr_todo.clone(), vdpu_sst_bridge).await;

        let dash_ha_global_config_sst = SubscriberStateTable::new_async(
            config_db.clone_timeout_async(TIMEOUT).await,
            "DASH_HA_GLOBAL_CONFIG",
            None,
            None,
        )
        .await;
        let dash_ha_global_config_sst_bridge = SwssCommonBridge::new_subscriber_state_table(dash_ha_global_config_sst);
        runtime
            .spawn(bind_addr_todo.clone(), dash_ha_global_config_sst_bridge)
            .await;

        let dash_ha_set_config_table_zcst = ZmqConsumerStateTable::new_async(
            appl_db.clone_timeout_async(TIMEOUT).await,
            "DASH_HA_SET_CONFIG_TABLE",
            &mut zmqs,
            None,
            None,
        )
        .await;
        let dash_ha_set_config_table_zcst_bridge =
            SwssCommonBridge::new_zmq_consumer_state_table(dash_ha_set_config_table_zcst);
        runtime
            .spawn(bind_addr_todo.clone(), dash_ha_set_config_table_zcst_bridge)
            .await;

        let dash_eni_placement_table_zcst = ZmqConsumerStateTable::new_async(
            appl_db.clone_timeout_async(TIMEOUT).await,
            "DASH_ENI_PLACEMENT_TABLE",
            &mut zmqs,
            None,
            None,
        )
        .await;
        let dash_eni_placement_table_zcst_bridge =
            SwssCommonBridge::new_zmq_consumer_state_table(dash_eni_placement_table_zcst);
        runtime
            .spawn(bind_addr_todo.clone(), dash_eni_placement_table_zcst_bridge)
            .await;

        let dash_ha_scope_config_table_zcst = ZmqConsumerStateTable::new_async(
            appl_db.clone_timeout_async(TIMEOUT).await,
            "DASH_HA_SCOPE_CONFIG_TABLE",
            &mut zmqs,
            None,
            None,
        )
        .await;
        let dash_ha_scope_config_table_zcst_bridge =
            SwssCommonBridge::new_zmq_consumer_state_table(dash_ha_scope_config_table_zcst);
        runtime
            .spawn(bind_addr_todo.clone(), dash_ha_scope_config_table_zcst_bridge)
            .await;
    }

    let _hamgrd_bind_addr = ServicePath {
        region_id: "region_a".into(),
        cluster_id: "switch-cluster-a".into(),
        node_id: "10.0.0.1".into(),
        service_type: "hamgrd".into(),
        service_id: "dpu0".into(),
        resource_type: "hascope".into(),
        resource_id: "dpu".into(),
    };

    runtime.join().await;
    */
}
