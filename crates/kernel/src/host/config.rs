//! Extension traits and server configuration for pluggable checks and host hooks.

use std::path::PathBuf;
use std::sync::Arc;
use tower_lsp::lsp_types::{Diagnostic, ServerCapabilities, Url};

use crate::semantic::SemanticGraph;
use crate::validation::{ValidationReport, ValidationRequest};

pub const KERNEL_INTERFACE_VERSION: u32 = 1;

pub type CheckProvider = Arc<dyn SemanticCheckProvider>;
pub type PipelineHook = Arc<dyn ValidationPipelineHook>;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityMetadata {
    pub capability_id: String,
    pub version: String,
    pub min_kernel_version: u32,
    pub feature_flags: Vec<String>,
}

impl CapabilityMetadata {
    pub fn new(capability_id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            capability_id: capability_id.into(),
            version: version.into(),
            min_kernel_version: KERNEL_INTERFACE_VERSION,
            feature_flags: Vec::new(),
        }
    }

    pub fn with_min_kernel_version(mut self, min_kernel_version: u32) -> Self {
        self.min_kernel_version = min_kernel_version;
        self
    }

    pub fn with_feature_flags(mut self, feature_flags: Vec<String>) -> Self {
        self.feature_flags = feature_flags;
        self
    }
}

pub trait CapabilityProvider: Send + Sync {
    fn metadata(&self) -> CapabilityMetadata;
    fn check_providers(&self) -> Vec<CheckProvider> {
        Vec::new()
    }
    fn pipeline_hooks(&self) -> Vec<PipelineHook> {
        Vec::new()
    }
}

pub trait ValidationPipelineHook: Send + Sync {
    fn before_validate(&self, _request: &ValidationRequest) -> Result<(), String> {
        Ok(())
    }
    fn after_validate(&self, _report: &mut ValidationReport) -> Result<(), String> {
        Ok(())
    }
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
    /// Optional validation pipeline hooks for host-side behavior.
    pub pipeline_hooks: Vec<PipelineHook>,
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
            .field("pipeline_hooks", &self.pipeline_hooks.len())
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

    /// Add a validation pipeline hook.
    pub fn with_pipeline_hook(mut self, hook: PipelineHook) -> Self {
        self.pipeline_hooks.push(hook);
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

    #[test]
    fn capability_metadata_defaults_to_current_kernel_interface() {
        let metadata = CapabilityMetadata::new("pro.example", "0.1.0");
        assert_eq!(metadata.min_kernel_version, KERNEL_INTERFACE_VERSION);
        assert_eq!(metadata.capability_id, "pro.example");
        assert_eq!(metadata.version, "0.1.0");
        assert!(metadata.feature_flags.is_empty());
    }
}
