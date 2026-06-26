//! Graph-first unit resolution for Spec42.

pub mod graph_ingest;
pub mod registry;
pub mod type_resolver;

pub use registry::{UnitDef, UnitError, UnitRegistry};
pub use type_resolver::{
    is_measurement_unit_compatible, is_unit_type_name, quantity_value_to_unit_type_name,
    unit_type_for_quantity_type_name, unit_type_for_quantity_value,
};
