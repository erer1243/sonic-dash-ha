//! Redis DB table schemata.
//!
//! This module is defined in terms of `smart-switch-ha-detailed-design.md` in the SONiC repo.
//!
//! <https://github.com/sonic-net/SONiC/blob/master/doc/smart-switch/high-availability/smart-switch-ha-detailed-design.md>

/// Section 2.1 "External facing configuration tables"
pub mod config_tables;

/// Section 2.2 "External facing state tables"
pub mod external_tables;

/// Section 2.3 "Tables used by HA internally"
pub mod internal_tables;

pub fn update_from_field_values<T: serde::Serialize + serde::de::DeserializeOwned>(
    fvs: swss_common::FieldValues,
    value: &mut T,
) -> Result<(), swss_serde::Error> {
    let mut existing_fvs = swss_serde::to_field_values(value)?;
    for (f, v) in fvs {
        existing_fvs.insert(f, v);
    }
    *value = swss_serde::from_field_values(&existing_fvs)?;
    Ok(())
}
