//! Repository-owned parser selection boundary.
//!
//! Production crates depend on this crate, never on an upstream parser source.
//! There is now exactly one parser: the pinned arena-backed revision. The
//! [`next`] namespace is retained as an alias of the root for one more commit
//! so the ~170 `next::` spellings can be stripped mechanically, separately from
//! the dependency change that made them redundant.

pub use parser_next::*;

/// Alias of the crate root, kept only until the `next::` prefix is stripped.
///
/// TODO(parser-convergence): delete this module and `crates/sysml_parser` with
/// it; consumers then depend on the pinned revision through the workspace.
pub mod next {
    pub use parser_next::*;
}
