//! Core library for Spec42 LSP server: semantic model, DTOs, language utilities.
//! Used by the spec42 and spec42-pro binaries.

pub mod analysis;
pub mod common;
pub mod host;
pub mod language;
mod lsp_runtime;
pub mod semantic_model;
pub mod semantic_tokens;
pub mod syntax;
pub mod views;
pub(crate) mod workspace;

// Host contract exports (intended stable composition surface for edition hosts).
pub use host::config::{
    CapabilityAugmenter, CustomMethodProvider, SemanticCheckProvider, Spec42Config,
};
pub use host::default_config::default_config as default_server_config;
pub use lsp_runtime::run as run_lsp;

// Core data model exports.
pub use syntax::ast_util::{identification_name, span_to_range, span_to_source_range, SourceRange};
pub use views::dto::{
    GraphEdgeDto, GraphNodeDto, SysmlClearCacheResultDto, SysmlGraphDto, SysmlModelResultDto,
    SysmlModelStatsDto, SysmlServerCachesDto, SysmlServerMemoryDto, SysmlServerStatsDto,
};
pub use views::ibd::{build_ibd_for_uri, is_port_like, IbdDataDto};
pub use views::{build_sysml_model_response, empty_model_response, parse_sysml_model_params};
pub use {analysis::*, common::util::parse_library_paths_from_value};
