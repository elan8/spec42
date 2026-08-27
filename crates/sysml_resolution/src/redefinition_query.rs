//! Rule-scoped outcomes for exact normative redefinition checks.
//!
//! The resolver already owns authored and implied `Redefinition` relationship facts. The 15
//! predicates represented here additionally require role-specific membership, endpoint,
//! direction, or identity facts. Until those facts are published, callers receive the first
//! missing prerequisite rather than a relationship inferred from a declaration name or a rendered
//! semantic projection.

pub use spec42_constraint_manifest::RedefinitionCheckKind;
pub use sysml_contract::{RedefinitionCheckOutcome, RedefinitionCheckPrerequisite};
