//! Extension traits and server configuration for pluggable semantic checks and diagram providers.

use std::sync::Arc;
use tower_lsp::lsp_types::{Diagnostic, Url};

use crate::diagram_types::RenderedDiagram;
use crate::dto::SysmlGraphDto;
use crate::ibd::IbdDataDto;
use crate::semantic_model::SemanticGraph;

/// Provider of semantic/quality diagnostics. Implement this to add custom checks (e.g. naming rules, complexity).
pub trait SemanticCheckProvider: Send + Sync {
    /// Returns LSP diagnostics for the given document using the semantic graph.
    fn compute_diagnostics(&self, graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic>;
}

/// Context passed to diagram providers when building the sysml/model response.
/// Carries the graph DTO, optional IBD, and document URI.
#[derive(Debug, Clone)]
pub struct DiagramContext<'a> {
    /// Graph nodes and edges for the document (for general view and similar diagrams).
    pub graph: Option<&'a SysmlGraphDto>,
    /// IBD data for the document (for interconnection view).
    pub ibd: Option<&'a IbdDataDto>,
    /// Document URI.
    pub uri: &'a Url,
}

/// Provider of a single diagram type. Implement this to add custom diagrams (e.g. extra views in Pro).
pub trait DiagramProvider: Send + Sync {
    /// Unique id for this diagram (e.g. `"generalView"`, `"interconnectionView"`). Used as the key in the rendered_diagrams map.
    fn diagram_id(&self) -> &str;

    /// Renders the diagram if this provider can produce one for the given context.
    /// Returns None if the diagram is not applicable or rendering failed.
    fn render(&self, context: &DiagramContext<'_>) -> Option<RenderedDiagram>;
}

/// Server configuration: list of check and diagram providers. Built by the binary and passed to the core server.
#[derive(Default, Clone)]
pub struct Spec42Config {
    /// Semantic/quality check providers run when publishing diagnostics after a successful parse.
    pub check_providers: Vec<Arc<dyn SemanticCheckProvider>>,
    /// Diagram providers run when building the sysml/model rendered_diagrams response.
    pub diagram_providers: Vec<Arc<dyn DiagramProvider>>,
}

impl std::fmt::Debug for Spec42Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spec42Config")
            .field("check_providers", &self.check_providers.len())
            .field("diagram_providers", &self.diagram_providers.len())
            .finish()
    }
}

impl Spec42Config {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a semantic check provider.
    pub fn with_check_provider(mut self, p: Arc<dyn SemanticCheckProvider>) -> Self {
        self.check_providers.push(p);
        self
    }

    /// Add a diagram provider.
    pub fn with_diagram_provider(mut self, p: Arc<dyn DiagramProvider>) -> Self {
        self.diagram_providers.push(p);
        self
    }
}
