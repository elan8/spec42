pub(crate) mod diagnostics_adapter;
pub(crate) mod diagnostics_core;
pub(crate) mod diagnostics_postprocess;

pub use crate::language::SymbolEntry;
pub use crate::semantic_tokens::{
    ast_semantic_ranges, legend, semantic_tokens_full, semantic_tokens_range,
};
