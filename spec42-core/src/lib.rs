//! Core library for Spec42 LSP server: semantic model, DTOs, language utilities.
//! Used by the spec42 and spec42-pro binaries.

pub mod analysis;
pub mod ast_util;
pub mod config;
mod default_diagram_providers;
pub mod diagram_types;
mod dto;
mod ibd;
pub mod language;
pub mod logging;
pub(crate) mod lsp;
mod lsp_runtime;
mod lsp_server;
mod model;
pub mod semantic_checks;
pub mod semantic_model;
pub mod semantic_tokens;
pub mod util;
pub mod views;
pub(crate) mod workspace;

// Host contract exports (intended stable composition surface for edition hosts).
pub use config::{
    CapabilityAugmenter, CustomMethodProvider, DiagramContext, DiagramProvider,
    SemanticCheckProvider, Spec42Config,
};
pub use default_diagram_providers::default_config as default_server_config;
pub use lsp_server::run as run_lsp;

// Core data model exports.
pub use ast_util::{identification_name, span_to_range, span_to_source_range, SourceRange};
pub use diagram_types::{
    Bounds, HitRegion, HitRegionKind, LayoutMetrics, RenderedDiagram, ViewState,
};
pub use dto::{
    DiagramBoundsDto, GraphEdgeDto, GraphNodeDto, RenderedDiagramDto, RenderedDiagramsDto,
    SysmlClearCacheResultDto, SysmlGraphDto, SysmlModelResultDto, SysmlModelStatsDto,
    SysmlServerCachesDto, SysmlServerMemoryDto, SysmlServerStatsDto,
};
pub use ibd::{build_ibd_for_uri, is_port_like, IbdDataDto};
pub use views::{build_sysml_model_response, empty_model_response, parse_sysml_model_params};
pub use {analysis::*, util::parse_library_paths_from_value};
