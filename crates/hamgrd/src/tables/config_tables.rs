use hamgrd_macro::{ToFromFieldValue, ToFromFieldValues};

// use swbus_actor::prelude::ServicePath;
// /// The bind address of each table's SwssCommonBridge instance.
// #[derive(Debug, Clone)]
// pub struct ExternalConfigTableServicePaths {
//     // CONFIG_DB
//     dpu: ServicePath,
//     vdpu: ServicePath,
//     dash_ha_global_config: ServicePath,

//     // APPL_DB
//     dash_ha_set_config_table: ServicePath,
//     dash_ha_scope_config_table: ServicePath,
//     dash_eni_placement_table: Option<ServicePath>,

//     // DPU_APPL_DB
//     dash_eni_table: Vec<ServicePath>,
// }

/// CONFIG_DB::DPU table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct Dpu {
    #[rename = "type"]
    typ: DpuType,
    state: String,
    slot_id: String,
    pa_ipv4: String,
    pa_ipv6: String,
    npu_ipv4: String,
    npu_ipv6: String,
    probe_ip: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToFromFieldValue)]
pub enum DpuType {
    Local,
    Cluster,
    External,
}

/// CONFIG_DB::VDPU table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct VDpu {
    profile: String,
    tier: String,
    main_dpu_ids: String,
}

/// CONFIG_DB::DASH_HA_GLOBAL_CONFIG schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaGlobalConfig {
    cp_data_channel_port: u16,
    dp_channel_dst_port: u16,
    dp_channel_srt_port_min: u16,
    dp_channel_src_port_max: u16,
    dp_channel_probe_interval_ms: u64,
    dp_channel_probe_fail_threshold: u64,
    dpu_bfd_probe_interval_in_ms: u64,
    dpu_bfd_probe_multiplier: u64,
}

/// APPL_DB::DASH_HA_SET_CONFIG_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaSetConfigTable {
    version: String,
    vip_v4: String,
    vip_v6: String,
    owner: HaOwner,
    scope: HaScope,
    vdpu_ids: String,
    pinned_vdpu_bfd_probe_states: Vec<PinnedVDpuBfdProbeState>,
    preferred_vdpu_id: String,
    preferred_standalone_vdpu_index: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToFromFieldValue)]
pub enum PinnedVDpuBfdProbeState {
    None,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToFromFieldValue)]
pub enum HaOwner {
    Dpu,
    Switch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToFromFieldValue)]
pub enum HaScope {
    Dpu,
    Eni,
}

/// APPL_DB::DASH_HA_SET_CONFIG_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashHaScopeConfigTable {
    version: String,
    disabled: bool,
    desired_ha_state: DesiredHaState,
    approved_pending_operation_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToFromFieldValue)]
pub enum DesiredHaState {
    None,
    Dead,
    Active,
    Standalone,
}

/// APPL_DB::DASH_ENI_PLACEMENT_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashEniPlacementTable {
    version: String,
    eni_mac: String,
    ha_set_id: String,
    pinned_next_hop_index: Option<u64>,
}

/// DPU_APPL_DB::DASH_ENI_TABLE table entry schema
#[derive(Debug, Clone, PartialEq, Eq, ToFromFieldValues)]
pub struct DashEniTable {
    admin_state: String,
    ha_scope_id: String,
    // ...
}
