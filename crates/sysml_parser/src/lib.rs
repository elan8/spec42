//! Repository-owned parser selection boundary.
//!
//! Production crates depend on this crate, never on an upstream parser source.
//! The root API is the compatibility AST used by syntax-fidelity consumers.
//! Resolution uses [`next`] while its arena-backed AST migration is completed.
//! Keeping this exception here makes the remaining split explicit, pinned, and
//! mechanically removable without allowing each consumer to choose an authority.

pub use parser_legacy::*;

/// Canonical arena-backed parser used by semantic construction.
///
/// TODO(parser-convergence): migrate syntax-fidelity adapters to this API and
/// remove `parser-legacy` plus the root compatibility re-export.
pub mod next {
    pub use parser_next::*;
}
