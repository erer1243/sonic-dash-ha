use serde::{Deserialize, Serialize};
use swbus_actor::prelude::ServicePath;

/// The bind address of each table's SwssCommonBridge instance.
#[derive(Debug, Clone)]
pub struct ServicePaths {
    // DPU_APPL_DB
    pub dash_ha_set_table: ServicePath,
    pub dash_ha_scope_table: ServicePath,
    pub dash_flow_sync_session_table: ServicePath,

    // APPL_DB
    pub dash_eni_forward_table: ServicePath,
    pub dpu_state: ServicePath,
    pub vdpu_state: ServicePath,

    // DPU_STATE_DB
    pub dash_ha_set_state: ServicePath,
    pub dash_ha_scope_state: ServicePath,
    pub dash_flow_sync_session_state: ServicePath,
    pub dash_bfd_probe_state: ServicePath,
}

/// DPU_APPL_DB::DASH_HA_SET_TABLE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaSetTable {
    pub version: String,
    pub vip_v4: String,
    pub vip_v6: String,
    pub owner: Owner,
    pub local_npu_ip: String,
    pub local_ip: String,
    pub peer_ip: String,
    pub cp_data_channel_port: u16,
    pub dp_channel_dst_port: u16,
    pub dp_channel_src_port_min: u16,
    pub dp_channel_src_port_max: u16,
    pub dp_channel_probe_interval_ms: u64,
    pub dp_channel_probe_fail_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Owner {
    Controller,
    Switch,
}

/// DPU_APPL_DB::DASH_HA_SCOPE_TABLE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaScopeTable {
    pub version: String,
    pub disabled: bool,
    pub ha_role: HaRole,
    pub flow_reconcile_requested: Option<bool>,
    pub activate_role_requested: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HaRole {
    Dead,
    Active,
    Standby,
    Standalone,
    SwitchingToActive,
}

/// DPU_APPL_DB::DASH_FLOW_SYNC_SESSION_TABLE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashFlowSyncSessionTable {
    pub ha_set_id: String,
    pub target_server_ip: String,
    pub target_server_port: u16,
}

/// APPL_DB::DASH_ENI_FORWARD_TABLE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashEniForwardTable {
    pub vdpu_ids: Vec<String>,
    pub primary_vdpu: String,
    pub outbound_vni: Option<String>,
    pub outbound_eni_mac_lookup: Option<OutboundEniMacLookup>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboundEniMacLookup {
    Dst,
    Src,
}

/// CHASSIS_STATE_DB::DPU_STATE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpuState {}

/// CHASSIS_STATE_DB::VDPU_STATE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VDpuState {}

/// DPU_STATE_DB::DASH_HA_SET_STATE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaSetState {
    pub last_updated_time: u64,
    pub dp_channel_is_alive: bool,
}

/// DPU_STATE_DB::DASH_HA_SCOPE_STATE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaScopeState {
    pub last_updated_time: u64,
    pub ha_role: String, // TODO make this an enum
    pub ha_role_start_time: u64,
    pub ha_term: u64,
    pub activate_role_pending: bool,
    pub flow_reconcile_pending: bool,
    pub brainsplit_recover_pending: bool,
}

/// DPU_STATE_DB::DASH_FLOW_SYNC_SESSION_STATE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashFlowSyncSessionState {
    pub state: SyncState,
    pub creation_time_in_ms: u64,
    pub last_state_start_time_in_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncState {
    Created,
    InProgress,
    Completed,
    Failed,
}

/// DPU_STATE_DB::DASH_BFD_PROBE_STATE
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashBfdProbeState {
    v4_bfd_up_sessions: Vec<String>,
    v6_bfd_up_sessions: Vec<String>,
}
