//! The vocabulary every SysML answer is spoken in, and nothing that computes one.
//!
//! This crate defines the semantic value types -- element kinds, visibilities, relationship
//! families, outcome and prerequisite enums, positions and ranges, diagnostic severities and
//! codes -- together with the opaque identity newtypes and the sealed traits that let an
//! authority hand back a borrowed view instead of an owned collection. It computes no semantic
//! fact, holds no state, performs no I/O, and names no authority: `sysml_resolution`
//! *implements* this contract, `sysml_query` *re-exports* it verbatim, and consumers depend on
//! it only through the facade. A rename inside the authority is then invisible; a change here is
//! a deliberate, versioned contract change.

#![forbid(unsafe_code)]

mod diagnostic;
mod version;

pub use diagnostic::{DiagnosticCategory, DiagnosticOrigin, DiagnosticSeverity};
pub use version::{SemanticContractVersion, SEMANTIC_CONTRACT_VERSION};
