use hamgrd_macro::{ToFromFieldValue, ToFromFieldValues};

/// STATE_DB::DASH_HA_SCOPE_STATE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaScopeState {
    // Basic information
    creation_time_in_ms: u64,
    last_heartbeat_time_in_ms: u64,
    vip_v4: String,
    vip_v6: String,
    local_ip: String,
    peer_ip: String,

    // HA related state
    local_ha_state: String,
    local_ha_state_last_updated_time_in_ms: u64,
    local_ha_state_last_updated_reason: String,
    local_target_asic_ha_state: String,
    local_acked_asic_ha_state: String,
    local_target_term: u64,
    local_acked_term: u64,
    peer_ha_state: String,
    peer_term: u64,

    // Aggregated health signals for HA scope
    local_vdpu_midplane_state: PlaneState,
    local_vdpu_midplane_state_last_updated_time_in_ms: u64,
    local_vdpu_control_plane_state: PlaneState,
    local_vdpu_control_plane_state_last_updated_time_in_ms: u64,
    local_vdpu_data_plane_state: PlaneState,
    local_vdpu_data_plane_state_last_updated_time_in_ms: u64,
    local_vdpu_up_bfd_sessions_v4: Vec<String>,
    local_vdpu_up_bfd_sessions_v4_update_time_in_ms: u64,
    local_vdpu_up_bfd_sessions_v6: Vec<String>,
    local_vdpu_up_bfd_sessions_v6_update_time_in_ms: u64,

    // Ongoing HA operation state
    pending_operation_ids: Vec<String>,
    pending_operation_types: Vec<OperationType>,
    pending_operation_list_last_updated_time: u64,
    switchover_id: String,
    switchover_state: String,
    switchover_start_time_in_ms: String,
    switchover_end_time_in_ms: String,
    switchover_approved_time_in_ms: String,
    flow_sync_session_id: String,
    flow_sync_session_state: String,
    flow_sync_session_start_time_in_ms: String,
    flow_sync_session_target_server: String,
}

#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValue)]
pub enum PlaneState {
    Unknown,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValue)]
pub enum OperationType {
    Switchover,
    ActivateRole,
    FlowReconcile,
    BrainsplitRecover,
}
