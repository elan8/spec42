//! Kind-compatibility tables for P1 typing, specialization, and redefinition checks.

pub use sysml_model::semantic::kinds::{
    allowed_subset_redefine_target_kinds, allowed_typing_target_kinds,
    expected_typing_definition_label, is_compatible_kind, is_compatible_specializes_target,
    is_compatible_typing_target,
};
