//! Kind-compatibility tables still used by unmigrated diagnostic families.
//!
//! The typing/specialization/subsetting tables the P1 kind checks used are gone: those rules are
//! settled by the immutable publication, which relates metaclass families through the SysML
//! metamodel hierarchy instead of a caller-supplied slice. What remains is the one table
//! `behavior_conformance` still reads.

pub use sysml_model::semantic::kinds::{allowed_typing_target_kinds, is_compatible_kind};
