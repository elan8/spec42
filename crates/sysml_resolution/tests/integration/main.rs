//! The sysml_resolution crate's integration tests, one binary.

#[path = "../common/mod.rs"]
mod common;

mod diagnostics_contract;
mod evaluation_contract;
mod incremental_reuse;
mod lowering_contract;
mod navigation_contract;
mod phase_order;
mod resolution_contract;
mod construction_schedule_parity;
