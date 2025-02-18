use serde::{Deserialize, Serialize};
use swbus_actor::prelude::ServicePath;

/// The bind address of each table's SwssCommonBridge instance.
#[derive(Debug, Clone)]
pub struct ServicePaths {
    // CONFIG_DB
    pub dpu: ServicePath,
    pub vdpu: ServicePath,
    pub dash_ha_global_config: ServicePath,

    // APPL_DB
    pub dash_ha_set_config_table: ServicePath,
    pub dash_ha_scope_config_table: ServicePath,
    pub dash_eni_placement_table: ServicePath,

    // DPU_APPL_DB
    pub dash_eni_table: ServicePath,
}

/// CONFIG_DB::DPU table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dpu {
    #[serde(rename = "type")]
    pub typ: DpuType,
    pub state: String,
    pub slot_id: String,
    pub pa_ipv4: String,
    pub pa_ipv6: String,
    pub npu_ipv4: String,
    pub npu_ipv6: String,
    pub probe_ip: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpuType {
    Local,
    Cluster,
    External,
}

/// CONFIG_DB::VDPU table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VDpu {
    pub profile: String,
    pub tier: String,
    pub main_dpu_ids: String,
}

/// CONFIG_DB::DASH_HA_GLOBAL_CONFIG schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaGlobalConfig {
    pub cp_data_channel_port: u16,
    pub dp_channel_dst_port: u16,
    pub dp_channel_srt_port_min: u16,
    pub dp_channel_src_port_max: u16,
    pub dp_channel_probe_interval_ms: u64,
    pub dp_channel_probe_fail_threshold: u64,
    pub dpu_bfd_probe_interval_in_ms: u64,
    pub dpu_bfd_probe_multiplier: u64,
}

/// APPL_DB::DASH_HA_SET_CONFIG_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaSetConfigTable {
    pub version: String,
    pub vip_v4: String,
    pub vip_v6: String,
    pub owner: HaOwner,
    pub scope: HaScope,
    pub vdpu_ids: String,
    pub pinned_vdpu_bfd_probe_states: Vec<PinnedVDpuBfdProbeState>,
    pub preferred_vdpu_id: String,
    pub preferred_standalone_vdpu_index: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinnedVDpuBfdProbeState {
    None,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HaOwner {
    Dpu,
    Switch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HaScope {
    Dpu,
    Eni,
}

/// APPL_DB::DASH_HA_SET_CONFIG_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashHaScopeConfigTable {
    pub version: String,
    pub disabled: bool,
    pub desired_ha_state: DesiredHaState,
    pub approved_pending_operation_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesiredHaState {
    None,
    Dead,
    Active,
    Standalone,
}

/// APPL_DB::DASH_ENI_PLACEMENT_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashEniPlacementTable {
    pub version: String,
    pub eni_mac: String,
    pub ha_set_id: String,
    pub pinned_next_hop_index: Option<u64>,
}

/// DPU_APPL_DB::DASH_ENI_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DashEniTable {
    pub admin_state: String,
    pub ha_scope_id: String,
    // ...
}
