//! Exact derived Feature relationship-collection contracts.
//!
//! These are views over the publication's one canonical relationship store. They deliberately do
//! not materialize a second collection of relationship facts: provenance, unresolved targets, and
//! implied edges remain exactly those published for ordinary element inspection.

pub use sysml_contract::FeatureDerivedRelationshipCollection;
