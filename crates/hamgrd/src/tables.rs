//! Redis DB table schemata.
//!
//! This module is defined in terms of `smart-switch-ha-detailed-design.md` in the SONiC repo.
//!
//! <https://github.com/sonic-net/SONiC/blob/master/doc/smart-switch/high-availability/smart-switch-ha-detailed-design.md>

/// Section 2.1 "External facing configuration tables"
pub mod config_tables;

/// Section 2.2 "External facing state tables"
pub mod state_tables;

/// Section 2.3 "Tables used by HA internally"
pub mod internal_tables;

pub mod support;
