use hamgrd_macro::{ToFromFieldValue, ToFromFieldValues};

/// DPU_APPL_DB::DASH_HA_SET_TABLE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaSetTable {
    version: String,
    vip_v4: String,
    vip_v6: String,
    owner: Owner,
    local_npu_ip: String,
    local_ip: String,
    peer_ip: String,
    cp_data_channel_port: u16,
    dp_channel_dst_port: u16,
    dp_channel_src_port_min: u16,
    dp_channel_src_port_max: u16,
    dp_channel_probe_interval_ms: u64,
    dp_channel_probe_fail_threshold: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValue)]
pub enum Owner {
    Controller,
    Switch,
}

/// DPU_APPL_DB::DASH_HA_SCOPE_TABLE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaScopeTable {
    version: String,
    disabled: bool,
    ha_role: HaRole,
    flow_reconcile_requested: Option<bool>,
    activate_role_requested: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValue)]
pub enum HaRole {
    Dead,
    Active,
    Standby,
    Standalone,
    SwitchingToActive,
}

/// DPU_APPL_DB::DASH_FLOW_SYNC_SESSION_TABLE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashFlowSyncSessionTable {
    ha_set_id: String,
    target_server_ip: String,
    target_server_port: u16,
}

/// APPL_DB::DASH_ENI_FORWARD_TABLE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashEniForwardTable {
    vdpu_ids: Vec<String>,
    primary_vdpu: String,
    outbound_vni: Option<String>,
    outbound_eni_mac_lookup: Option<OutboundEniMacLookup>,
}

#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValue)]
pub enum OutboundEniMacLookup {
    Dst,
    Src,
}

/// DPU_STATE_DB::DASH_HA_SET_STATE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaSetState {
    last_updated_time: u64,
    dp_channel_is_alive: bool,
}

/// DPU_STATE_DB::DASH_HA_SCOPE_STATE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaScopeState {
    last_updated_time: u64,
    ha_role: String, // TODO make this an enum
    ha_role_start_time: u64,
    ha_term: u64,
    activate_role_pending: bool,
    flow_reconcile_pending: bool,
    brainsplit_recover_pending: bool,
}

/// DPU_STATE_DB::DASH_FLOW_SYNC_SESSION_STATE
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashFlowSyncSessionState {
    state: SyncState,
    creation_time_in_ms: u64,
    last_state_start_time_in_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValue)]
pub enum SyncState {
    Created,
    InProgress,
    Completed,
    Failed,
}
