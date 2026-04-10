//! Extension traits and server configuration for pluggable checks and host hooks.

use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::lsp_types::{Diagnostic, ServerCapabilities, Url};

use crate::semantic_model::SemanticGraph;

/// Provider of semantic/quality diagnostics. Implement this to add custom checks (e.g. naming rules, complexity).
pub trait SemanticCheckProvider: Send + Sync {
    /// Returns LSP diagnostics for the given document using the semantic graph.
    fn compute_diagnostics(&self, graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic>;
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

/// Server configuration built by the binary and passed to the core server.
#[derive(Default, Clone)]
pub struct Spec42Config {
    /// Optional library roots supplied by the host (e.g. materialized standard library), merged
    /// before client `libraryPaths` during LSP initialize / configuration.
    pub default_library_paths: Vec<PathBuf>,
    /// Semantic/quality check providers run when publishing diagnostics after a successful parse.
    pub check_providers: Vec<Arc<dyn SemanticCheckProvider>>,
    /// Optional capability augmenters for additive host composition.
    pub capability_augmenters: Vec<Arc<dyn CapabilityAugmenter>>,
    /// Optional custom-method declaration providers for additive host composition.
    pub custom_method_providers: Vec<Arc<dyn CustomMethodProvider>>,
}

impl std::fmt::Debug for Spec42Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spec42Config")
            .field("default_library_paths", &self.default_library_paths)
            .field("check_providers", &self.check_providers.len())
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

    /// Host-provided library roots (prepended when merging with client `libraryPaths`).
    pub fn with_default_library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.default_library_paths = paths;
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
