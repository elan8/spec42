//! Core library for Spec42 LSP server: semantic model, DTOs, language utilities.
//! Used by the spec42 and spec42-pro binaries.

pub mod analysis;
pub mod common;
pub mod host;
pub mod language;
mod lsp_runtime;
pub(crate) use semantic::root_element::root_element_body;

pub mod semantic_tokens;
#[cfg(feature = "software-architecture")]
pub mod software_architecture {
    pub use plugins::software_architecture::*;
}
#[cfg(not(feature = "software-architecture"))]
pub mod software_architecture {
    use std::path::Path;
    use tower_lsp::lsp_types::Range;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SourceAnchor {
        pub file_path: String,
        pub range: Option<Range>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct SoftwareComponent {
        pub id: String,
        pub name: String,
        pub kind: String,
        pub parent_id: Option<String>,
        pub crate_name: String,
        pub module_path: String,
        pub anchors: Vec<SourceAnchor>,
        pub is_external: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct SoftwareDependency {
        pub from: String,
        pub to: String,
        pub kind: String,
        pub source_anchor: Option<SourceAnchor>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct SoftwareArchitectureModel {
        pub components: Vec<SoftwareComponent>,
        pub dependencies: Vec<SoftwareDependency>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct SoftwareAnalysisSummary {
        pub crate_count: usize,
        pub module_count: usize,
        pub dependency_count: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct SoftwareWorkspaceModel {
        pub workspace_root: String,
        pub architecture: SoftwareArchitectureModel,
        pub summary: SoftwareAnalysisSummary,
    }

    pub fn workspace_contains_rust_code(_workspace_root: &Path) -> bool {
        false
    }

    pub fn analyze_rust_workspace(workspace_root: &Path) -> SoftwareWorkspaceModel {
        SoftwareWorkspaceModel {
            workspace_root: workspace_root.display().to_string(),
            architecture: SoftwareArchitectureModel::default(),
            summary: SoftwareAnalysisSummary::default(),
        }
    }
}
pub mod semantic;
pub mod syntax;
pub mod validation;
pub mod views;
pub(crate) mod workspace;

// Host contract exports (intended stable composition surface for edition hosts).
pub use host::config::{
    CapabilityAugmenter, CapabilityMetadata, CapabilityProvider, CheckProvider,
    CustomMethodProvider, PipelineHook, SemanticCheckProvider, Spec42Config,
    ValidationPipelineHook, KERNEL_INTERFACE_VERSION,
};
pub use host::default_config::default_config as default_server_config;
pub use lsp_runtime::run as run_lsp;

// Core data model exports.
pub use analysis::{
    add_cross_document_edges_for_uri, ast_semantic_ranges, build_graph_from_doc,
    compute_semantic_diagnostics, legend, semantic_tokens_full, semantic_tokens_range,
    DefaultSemanticChecks, NodeId, RelationshipKind, SemanticGraph, SemanticNode, SymbolEntry,
};
pub use common::util::{merge_host_and_client_library_paths, parse_library_paths_from_value};
pub use syntax::ast_util::{identification_name, span_to_range, span_to_source_range, SourceRange};
pub use validation::{
    validate_paths, validate_paths_with_semantics, SemanticModelNode, SemanticModelProjection,
    SemanticModelRelationship, SemanticValidationReport, ValidationReport, ValidationRequest,
    ValidationSummary,
};
pub use views::dto::{
    GraphEdgeDto, GraphNodeDto, SoftwareAnalysisSummaryDto, SoftwareArchitectureModelDto,
    SoftwareComponentDto, SoftwareDependencyDto, SoftwareWorkspaceModelDto, SourceAnchorDto,
    SysmlClearCacheResultDto, SysmlFeatureInspectorElementDto, SysmlFeatureInspectorElementRefDto,
    SysmlFeatureInspectorParamsDto, SysmlFeatureInspectorRelationshipDto,
    SysmlFeatureInspectorResolutionDto, SysmlFeatureInspectorResultDto, SysmlGraphDto,
    SysmlModelResultDto, SysmlModelStatsDto, SysmlServerCachesDto, SysmlServerMemoryDto,
    SysmlServerStatsDto, SysmlVisualizationResultDto, SysmlVisualizationViewCandidateDto,
    WorkspaceFileModelDto, WorkspaceModelDto, WorkspaceModelSummaryDto,
};
pub use views::ibd::{build_ibd_for_uri, is_port_like, IbdDataDto};
pub use views::{
    build_sysml_model_response, build_sysml_visualization_for_paths,
    empty_feature_inspector_response, empty_model_response, parse_sysml_feature_inspector_params,
    parse_sysml_model_params,
};

/// SysML v2 textual parser (`sysml-v2-parser`). Version is pinned in the Spec42 workspace;
/// hosts should use this module instead of depending on `sysml-v2-parser` directly.
pub mod sysml_v2 {
    pub use ::sysml_v2_parser::*;
}
