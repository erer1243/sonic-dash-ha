//! Configuration DB table schemata.
//!
//! This module is defined in terms of section 2.1 "External facing configuration tables" of `smart-switch-ha-detailed-design.md` in the SONiC repo.
//!
//! <https://github.com/sonic-net/SONiC/blob/master/doc/smart-switch/high-availability/smart-switch-ha-hld.md>

use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};
use strum::EnumString;
use swbus_actor::prelude::ServicePath;
use swss_common::CxxString;

/// Error parsing FieldValues from a table update message.
#[derive(Debug)]
pub enum FieldValueError {
    /// A field is not present
    Missing { field: &'static str },

    /// A field is present but couldn't be parsed
    Invalid {
        field: &'static str,
        data: String,
        error: Box<dyn Error>,
    },
}

impl Display for FieldValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldValueError::Missing { field } => write!(f, "Missing field: {field}"),
            FieldValueError::Invalid { field, data, error } => {
                write!(f, "Invalid field: {field} = '{data}' ({error})")
            }
        }
    }
}

impl Error for FieldValueError {}

/// Parse a `T` from the given string.
fn parse_val<T>(field: &'static str, value: &str) -> Result<T, FieldValueError>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    match T::from_str(value) {
        Ok(t) => Ok(t),
        Err(err) => Err(FieldValueError::Invalid {
            field,
            data: value.to_string(),
            error: Box::new(err),
        }),
    }
}

/// Parse a `T` from the given `field` in `fvs`.
///
/// Returns an error if `field` is not present in `fvs`, or if `T::from_str` returns an error.
fn parse_fv<T>(fvs: &HashMap<String, CxxString>, field: &'static str) -> Result<T, FieldValueError>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    match fvs.get(field) {
        Some(val_cxx) => {
            let val_str = val_cxx.to_string_lossy();
            parse_val(field, &val_str)
        }
        None => Err(FieldValueError::Missing { field }),
    }
}

/// Parse an `Option<T>` from the given `field` in `fvs`.
///
/// Returns `None` if `field` is not present in `fvs` or if the value is `""` or `"none"`.
/// Returns an error if `T::from_str` returns an error.
fn parse_fv_option<T>(fvs: &HashMap<String, CxxString>, field: &'static str) -> Result<Option<T>, FieldValueError>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    match fvs.get(field) {
        Some(val_cxx) => {
            if val_cxx.is_empty() || val_cxx.as_bytes() == b"none" {
                Ok(None)
            } else {
                let val = parse_val(field, &val_cxx.to_string_lossy())?;
                Ok(Some(val))
            }
        }
        None => Ok(None),
    }
}

/// Parse a `Vec<T>` from a comma-separated list of values in the given `field` in `fvs`.
///
/// Returns an empty `Vec` only if the value is an empty string.
/// Returns an error if `field` is not present in `fvs` or if `T::from_str` returns an error for any value.
fn parse_fv_comma_separated_list<T>(
    fvs: &HashMap<String, CxxString>,
    field: &'static str,
) -> Result<Vec<T>, FieldValueError>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    match fvs.get(field) {
        Some(val_cxx) => {
            let val_str = val_cxx.to_string_lossy();
            val_str.split(',').map(|s| parse_val(field, s)).collect()
        }
        None => Err(FieldValueError::Missing { field }),
    }
}

/// The bind address of each table's SwssCommonBridge instance.
#[derive(Debug, Clone)]
pub struct ExternalConfigTableServicePaths {
    // CONFIG_DB
    dpu: ServicePath,
    vdpu: ServicePath,
    dash_ha_global_config: ServicePath,

    // APPL_DB
    dash_ha_set_config_table: ServicePath,
    dash_ha_scope_config_table: ServicePath,
    dash_eni_placement_table: Option<ServicePath>,

    // DPU_APPL_DB
    dash_eni_table: Vec<ServicePath>,
}

/// CONFIG_DB::DPU table entry schema
#[derive(Debug, Clone)]
pub struct Dpu {
    type_: DpuType,
    state: String,
    slot_id: String,
    pa_ipv4: String,
    pa_ipv6: String,
    npu_ipv4: String,
    npu_ipv6: String,
    probe_ip: Option<String>,
}

impl Dpu {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Dpu {
            type_: parse_fv(fvs, "type")?,
            state: parse_fv(fvs, "state")?,
            slot_id: parse_fv(fvs, "slot_id")?,
            pa_ipv4: parse_fv(fvs, "pa_ipv4")?,
            pa_ipv6: parse_fv(fvs, "pa_ipv6")?,
            npu_ipv4: parse_fv(fvs, "npu_ipv4")?,
            npu_ipv6: parse_fv(fvs, "npu_ipv6")?,
            probe_ip: parse_fv_option(fvs, "probe_ip")?,
        })
    }
}

#[derive(Debug, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum DpuType {
    Local,
    Cluster,
    External,
}

/// CONFIG_DB::VDPU table entry schema
#[derive(Debug, Clone)]
pub struct VDpu {
    profile: String,
    tier: String,
    main_dpu_ids: String,
}

impl VDpu {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Self {
            profile: parse_fv(fvs, "profile")?,
            tier: parse_fv(fvs, "tier")?,
            main_dpu_ids: parse_fv(fvs, "main_dpu_ids")?,
        })
    }
}

/// CONFIG_DB::DASH_HA_GLOBAL_CONFIG schema
#[derive(Debug, Clone)]
pub struct DashHAGlobalConfig {
    cp_data_channel_port: u16,
    dp_channel_dst_port: u16,
    dp_channel_srt_port_min: u16,
    dp_channel_src_port_max: u16,
    dp_channel_probe_interval_ms: u64,
    dp_channel_probe_fail_threshold: u64,
    dpu_bfd_probe_interval_in_ms: u64,
    dpu_bfd_probe_multiplier: u64,
}

impl DashHAGlobalConfig {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Self {
            cp_data_channel_port: parse_fv(fvs, "cp_data_channel_port")?,
            dp_channel_dst_port: parse_fv(fvs, "dp_channel_dst_port")?,
            dp_channel_srt_port_min: parse_fv(fvs, "dp_channel_src_port_min")?,
            dp_channel_src_port_max: parse_fv(fvs, "dp_channel_src_port_max")?,
            dp_channel_probe_interval_ms: parse_fv(fvs, "dp_channel_probe_interval_ms")?,
            dp_channel_probe_fail_threshold: parse_fv(fvs, "dp_channel_probe_fail_threshold")?,
            dpu_bfd_probe_interval_in_ms: parse_fv(fvs, "dpu_bfd_probe_interval_in_ms")?,
            dpu_bfd_probe_multiplier: parse_fv(fvs, "dpu_bfd_probe_multiplier")?,
        })
    }
}

/// APPL_DB::DASH_HA_SET_CONFIG_TABLE table entry schema
#[derive(Debug, Clone)]
pub struct DashHASetConfigTable {
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

impl DashHASetConfigTable {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Self {
            version: parse_fv(fvs, "version")?,
            vip_v4: parse_fv(fvs, "vip_v4")?,
            vip_v6: parse_fv(fvs, "vip_v6")?,
            owner: parse_fv(fvs, "owner")?,
            scope: parse_fv(fvs, "scope")?,
            vdpu_ids: parse_fv(fvs, "vdpu_ids")?,
            pinned_vdpu_bfd_probe_states: parse_fv_comma_separated_list(fvs, "pinned_vdpu_bfd_probe_states")?,
            preferred_vdpu_id: parse_fv(fvs, "prefered_vdpu_id")?,
            preferred_standalone_vdpu_index: parse_fv_option(fvs, "preferred_standalone_vdpu_index")?,
        })
    }
}

#[derive(Debug, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PinnedVDpuBfdProbeState {
    None,
    Up,
    Down,
}

#[derive(Debug, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HaOwner {
    Dpu,
    Switch,
}

#[derive(Debug, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HaScope {
    Dpu,
    Eni,
}

/// APPL_DB::DASH_HA_SET_CONFIG_TABLE table entry schema
#[derive(Debug, Clone)]
pub struct DashHAScopeConfigTable {
    version: String,
    disabled: bool,
    desired_ha_state: DesiredHaState,
    approved_pending_operation_ids: Vec<String>,
}

impl DashHAScopeConfigTable {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Self {
            version: parse_fv(fvs, "version")?,
            disabled: parse_fv(fvs, "disabled")?,
            desired_ha_state: parse_fv(fvs, "desired_ha_state")?,
            approved_pending_operation_ids: parse_fv_comma_separated_list(fvs, "approved_pending_operation_ids")?,
        })
    }
}

#[derive(Debug, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum DesiredHaState {
    None,
    Dead,
    Active,
    Standalone,
}

/// APPL_DB::DASH_ENI_PLACEMENT_TABLE table entry schema
#[derive(Debug, Clone)]
pub struct DashEniPlacementTable {
    version: String,
    eni_mac: String,
    ha_set_id: String,
    pinned_next_hop_index: Option<u64>,
}

impl DashEniPlacementTable {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Self {
            version: parse_fv(fvs, "version")?,
            eni_mac: parse_fv(fvs, "eni_mac")?,
            ha_set_id: parse_fv(fvs, "ha_set_id")?,
            pinned_next_hop_index: parse_fv_option(fvs, "pinned_next_hop_index")?,
        })
    }
}

/// DPU_APPL_DB::DASH_ENI_TABLE table entry schema
#[derive(Debug, Clone)]
pub struct DashEniTable {
    admin_state: String,
    ha_scope_id: String,
    // ...
}

impl DashEniTable {
    pub fn from_fvs(fvs: &HashMap<String, CxxString>) -> Result<Self, FieldValueError> {
        Ok(Self {
            admin_state: parse_fv(fvs, "admin_state")?,
            ha_scope_id: parse_fv(fvs, "ha_scope_id")?,
        })
    }
}
