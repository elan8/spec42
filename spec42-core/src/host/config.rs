//! Extension traits and server configuration for pluggable checks, diagrams, and host hooks.

use std::sync::Arc;
use tower_lsp::lsp_types::{Diagnostic, ServerCapabilities, Url};

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

/// Optional host hook for capability augmentation.
///
/// Intended for downstream edition composition (for example a private PRO host)
/// to add extra capability metadata without changing core OSS defaults.
pub trait CapabilityAugmenter: Send + Sync {
    /// Mutate server capabilities before they are returned from initialize.
    fn augment_capabilities(&self, capabilities: &mut ServerCapabilities);
}

/// Optional host hook for declaring additional custom methods.
///
/// This does not register handlers by itself; it provides a stable contract for
/// downstream hosts to publish/track extra method names.
pub trait CustomMethodProvider: Send + Sync {
    /// Returns custom JSON-RPC method names introduced by this provider.
    fn custom_method_names(&self) -> Vec<String>;
}

/// Server configuration: list of check and diagram providers. Built by the binary and passed to the core server.
#[derive(Default, Clone)]
pub struct Spec42Config {
    /// Semantic/quality check providers run when publishing diagnostics after a successful parse.
    pub check_providers: Vec<Arc<dyn SemanticCheckProvider>>,
    /// Diagram providers run when building the sysml/model rendered_diagrams response.
    pub diagram_providers: Vec<Arc<dyn DiagramProvider>>,
    /// Optional capability augmenters for additive host composition.
    pub capability_augmenters: Vec<Arc<dyn CapabilityAugmenter>>,
    /// Optional custom-method declaration providers for additive host composition.
    pub custom_method_providers: Vec<Arc<dyn CustomMethodProvider>>,
}

impl std::fmt::Debug for Spec42Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spec42Config")
            .field("check_providers", &self.check_providers.len())
            .field("diagram_providers", &self.diagram_providers.len())
            .field("capability_augmenters", &self.capability_augmenters.len())
            .field(
                "custom_method_providers",
                &self.custom_method_providers.len(),
            )
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

    /// Add a capability augmenter.
    pub fn with_capability_augmenter(mut self, p: Arc<dyn CapabilityAugmenter>) -> Self {
        self.capability_augmenters.push(p);
        self
    }

    /// Add a custom method provider.
    pub fn with_custom_method_provider(mut self, p: Arc<dyn CustomMethodProvider>) -> Self {
        self.custom_method_providers.push(p);
        self
    }

    /// Returns custom method names contributed by host providers.
    pub fn extra_custom_method_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for provider in &self.custom_method_providers {
            names.extend(provider.custom_method_names());
        }
        names.sort();
        names.dedup();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct DummyMethodsA;
    impl CustomMethodProvider for DummyMethodsA {
        fn custom_method_names(&self) -> Vec<String> {
            vec!["sysml/proA".to_string(), "sysml/shared".to_string()]
        }
    }

    #[derive(Debug)]
    struct DummyMethodsB;
    impl CustomMethodProvider for DummyMethodsB {
        fn custom_method_names(&self) -> Vec<String> {
            vec!["sysml/shared".to_string(), "sysml/proB".to_string()]
        }
    }

    #[test]
    fn extra_custom_method_names_are_sorted_and_deduplicated() {
        let cfg = Spec42Config::new()
            .with_custom_method_provider(Arc::new(DummyMethodsA))
            .with_custom_method_provider(Arc::new(DummyMethodsB));
        assert_eq!(
            cfg.extra_custom_method_names(),
            vec![
                "sysml/proA".to_string(),
                "sysml/proB".to_string(),
                "sysml/shared".to_string()
            ]
        );
    }
}
