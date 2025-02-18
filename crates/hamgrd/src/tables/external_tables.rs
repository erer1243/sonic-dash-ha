use serde::{Deserialize, Serialize};
use swbus_actor::prelude::ServicePath;

/// The bind address of each table's SwssCommonBridge instance.
#[derive(Debug, Clone)]
pub struct ServicePaths {
    // STATE_DB
    pub dash_ha_scope_state: ServicePath,
}

/// STATE_DB::DASH_HA_SCOPE_STATE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaScopeState {
    // Basic information
    pub creation_time_in_ms: u64,
    pub last_heartbeat_time_in_ms: u64,
    pub vip_v4: String,
    pub vip_v6: String,
    pub local_ip: String,
    pub peer_ip: String,

    // HA related state
    pub local_ha_state: HaState,
    pub local_ha_state_last_updated_time_in_ms: u64,
    pub local_ha_state_last_updated_reason: String,
    pub local_target_asic_ha_state: String,
    pub local_acked_asic_ha_state: String,
    pub local_target_term: u64,
    pub local_acked_term: u64,
    pub peer_ha_state: HaState,
    pub peer_term: u64,

    // Aggregated health signals for HA scope
    pub local_vdpu_midplane_state: PlaneState,
    pub local_vdpu_midplane_state_last_updated_time_in_ms: u64,
    pub local_vdpu_control_plane_state: PlaneState,
    pub local_vdpu_control_plane_state_last_updated_time_in_ms: u64,
    pub local_vdpu_data_plane_state: PlaneState,
    pub local_vdpu_data_plane_state_last_updated_time_in_ms: u64,
    pub local_vdpu_up_bfd_sessions_v4: Vec<String>,
    pub local_vdpu_up_bfd_sessions_v4_update_time_in_ms: u64,
    pub local_vdpu_up_bfd_sessions_v6: Vec<String>,
    pub local_vdpu_up_bfd_sessions_v6_update_time_in_ms: u64,

    // Ongoing HA operation state
    pub pending_operation_ids: Vec<String>,
    pub pending_operation_types: Vec<OperationType>,
    pub pending_operation_list_last_updated_time: u64,
    pub switchover_id: String,
    pub switchover_state: String,
    pub switchover_start_time_in_ms: String,
    pub switchover_end_time_in_ms: String,
    pub switchover_approved_time_in_ms: String,
    pub flow_sync_session_id: String,
    pub flow_sync_session_state: String,
    pub flow_sync_session_start_time_in_ms: String,
    pub flow_sync_session_target_server: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaneState {
    Unknown,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    Switchover,
    ActivateRole,
    FlowReconcile,
    BrainsplitRecover,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HaState {
    Dead,
    Connecting,
    Connected,
    InitializingToActive,
    InitializingToStandby,
    Destroying,
    Active,
    Standby,
    Standalone,
    SwitchingToActive,
    SwitchingToStandby,
}
