//! The workspace crate's integration tests, one binary. Each module was a separate test binary
//! that linked the whole engine; one binary links it once.

#[path = "../support/comparison_fixtures.rs"]
mod comparison_fixtures;

mod artifact_metadata_serde;
mod cancellation;
mod comparison_diagnostics;
mod comparison_identity;
mod deferred_validation;
mod dependency_guardrails;
mod document_changes;
mod host_errors;
mod library_source_resolution;
mod resource_limits;
mod semantic_ownership_guardrails;
mod skip_guardrails;
mod validation_diagnostics;
mod validation_mbse_vacuum_baseline;
mod validation_postprocess;
mod validation_webshop_library_closure;
