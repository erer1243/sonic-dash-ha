mod tables;

use serde::{Deserialize, Serialize};
use swbus_actor::prelude::*;
use tables::{config_tables::DesiredHaState, external_tables::HaState};

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

// async fn load_tables() {
//     const TIMEOUT: u32 = 10000;

//     async fn db(name: &str) -> DbConnector {
//         DbConnector::new_named_async(name, true, TIMEOUT).await.expect(name)
//     }
//     let mut config_db = db("CONFIG_DB").await;
//     let mut appl_db = db("APPL_DB").await;
//     let mut state_db = db("STATE_DB").await;
//     let mut dpu_appl_db = db("DPU_APPL_DB").await;
//     let mut dpu_state_db = db("DPU_STATE_DB").await;
//     let mut chassis_state_db = db("CHASSIS_STATE_DB").await;

//     async fn tbl<T: DeserializeOwned>(db: &mut DbConnector, name: &str, key: &str) -> T {
//         let db = db.clone_timeout_async(TIMEOUT).await.unwrap();
//         let tbl = Table::new_async(db, name).await.expect(name);
//         let fvs = tbl.get(key).expect(key).expect(key);
//         swss_serde::from_field_values(&fvs).expect(key)
//     }
// }

struct TableServicePaths {
    config: tables::config_tables::ServicePaths,
    external: tables::external_tables::ServicePaths,
    internal: tables::internal_tables::ServicePaths,
}

struct Tables {}

struct HamgrdActor {
    table_service_paths: TableServicePaths,

    /// The other hamgrd in this HA pair
    peer: ServicePath,
}

impl HamgrdActor {
    async fn handle_control_message(&mut self, m: ControlMessage, outbox: Outbox) {
        match m {
            ControlMessage::RequestVote {
                term,
                desired_state,
                retry_count,
            } => {
                let local_term = todo!();
                let local_state = todo!();
                let local_desired_state = todo!();
                let response = primary_election(
                    local_term,
                    local_state,
                    local_desired_state,
                    term,
                    desired_state,
                    retry_count,
                )
                .await;
                outbox
                    .send(OutgoingMessage::request(self.peer.clone(), response.serialize()))
                    .await;
            }
            ControlMessage::RetryLater => todo!(),
            ControlMessage::BecomeActive => self.become_active().await,
            ControlMessage::BecomeStandby => self.become_standby().await,
            ControlMessage::BecomeStandalone => self.become_standalone().await,

            ControlMessage::HaStateChanged { state } => todo!(),
            ControlMessage::BulkSyncDone => todo!(),
        }
    }

    async fn become_active(&mut self) {}
    async fn become_standby(&mut self) {}
    async fn become_standalone(&mut self) {}
}
impl Actor for HamgrdActor {
    async fn init(&mut self, outbox: Outbox) {}

    async fn handle_message(&mut self, message: IncomingMessage, outbox: Outbox) {
        let TableServicePaths {
            config,
            external,
            internal,
        } = &self.table_service_paths;

        macro_rules! switch {
            ($val:expr; $($arm:expr => $ans:expr,)* _ => $default:expr $(,)?) => {
                match $val { $(_val if _val == $arm => $ans,)* _ => $default, }
            }
        }

        switch! { message.source;
            self.peer => {},

            // Upstream config programming
            config.dpu => {},
            config.vdpu => {},
            config.dash_ha_global_config => {},
            config.dash_ha_set_config_table => {},
            config.dash_ha_scope_config_table => {},
            config.dash_eni_placement_table => {},

            // State generation and handling path
            external.dash_ha_scope_state => {},
            internal.dash_ha_set_state => {},
            internal.dash_flow_sync_session_state => {},
            internal.dash_bfd_probe_state => {},

            // Anything else
            _ => {
                outbox.send(OutgoingMessage::error_response(message, SwbusErrorCode::InvalidSource, "Unknown sender")).await;
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
enum ControlMessage {
    // Messages as defined in 7.3 Primary Election
    // https://github.com/sonic-net/SONiC/blob/master/doc/smart-switch/high-availability/smart-switch-ha-hld.md#73-primary-election
    /// Connect to peer and request a vote. Peer should respond with one of the following 4 messages.
    RequestVote {
        term: u64,
        desired_state: DesiredHaState,
        retry_count: u64,
    },
    /// Retry the RequestVote "later"
    RetryLater,
    /// Become active peer in an active/standby pair
    BecomeActive,
    /// Become standby peer in an active/standby pair
    BecomeStandby,
    /// Become standalone peer in a standalone/dead pair
    BecomeStandalone,

    //
    HaStateChanged {
        state: HaState,
    },
    BulkSyncDone,
}

impl ControlMessage {
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn deserialize(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }
}

/// The primary election algorithm as described here:
/// <https://github.com/sonic-net/SONiC/blob/master/doc/smart-switch/high-availability/smart-switch-ha-hld.md#73-primary-election>
async fn primary_election(
    local_term: u64,
    local_state: HaState,
    local_desired_state: DesiredHaState,
    peer_term: u64,
    peer_desired_state: DesiredHaState,
    retry_count: u64,
) -> ControlMessage {
    use ControlMessage::*;

    const MAX_RETRY_COUNT: u64 = 100;

    // 1
    // let eni_found: bool = todo!("determine if ENI is 'found'");
    // if !eni_found {
    //     return if retry_count < MAX_RETRY_COUNT {
    //         RetryLater
    //     } else {
    //         BecomeStandalone
    //     };
    // }

    // 2
    if local_desired_state == DesiredHaState::Standalone {
        return RetryLater;
    }

    // 3
    if local_state == HaState::Active {
        return BecomeStandby;
    }

    // 4
    if local_state == HaState::Dead && local_desired_state == DesiredHaState::Dead {
        return BecomeStandalone;
    }

    // 5
    if local_state == HaState::Dead || local_state == HaState::Connecting {
        return if retry_count < MAX_RETRY_COUNT {
            RetryLater
        } else {
            BecomeStandalone
        };
    }

    // 6
    if local_term != peer_term {
        return if local_term > peer_term {
            BecomeStandby
        } else {
            BecomeActive
        };
    }

    // 7
    if local_desired_state == DesiredHaState::Active && peer_desired_state == DesiredHaState::None {
        return BecomeStandby;
    } else if local_desired_state == DesiredHaState::None && peer_desired_state == DesiredHaState::Active {
        return BecomeActive;
    }

    // 8
    if retry_count > MAX_RETRY_COUNT {
        todo!("Fire alert, we are not configured properly")
    }

    RetryLater
}
