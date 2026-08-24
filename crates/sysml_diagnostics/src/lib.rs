//! Transport-neutral diagnostic values and the host reporting policy over them.
//!
//! This crate decides nothing semantic. `sysml_resolution` settles every code, severity, range,
//! message, and related location before a publication becomes visible; `sysml_query` exposes them;
//! and this crate turns one publication's answer into the neutral shape a CLI report, an LSP
//! adapter, and a workspace validation report all consume, applying only the explicit reporting
//! policy a host asked for.
//!
//! The line matters: a host may decide *whether* to show a diagnostic, and how to render it, but
//! never what a diagnostic means. Nothing here inspects a semantic model, resolves a name, or
//! reads a diagnostic's message as an input.

mod postprocess;
mod reporting;
pub mod types;

pub use postprocess::{
    diagnostics_dominated_by_cascades, postprocess_document_diagnostics, PostprocessPolicy,
};
pub use reporting::{document_diagnostics, severity_label, ReportingPolicy};
pub use types::{
    DiagnosticRelatedInfo, DiagnosticSeverity, SemanticDiagnostic, PARSER_SOURCE, SEMANTIC_SOURCE,
};
