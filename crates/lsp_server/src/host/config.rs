//! Extension traits and server configuration for pluggable checks and host hooks.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tower_lsp::lsp_types::ServerCapabilities;

pub const KERNEL_INTERFACE_VERSION: u32 = 1;

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

#[derive(Clone, Copy)]
pub struct CustomRpcContext<'a> {
    pub config: &'a Spec42Config,
    pub server_name: &'a str,
    pub server_start_time: Instant,
}

/// Generic custom JSON-RPC provider contract.
///
/// Hosts can register one or more providers from composition crates.
/// Kernel stays plugin-agnostic and only dispatches by method name.
pub trait CustomRpcProvider: Send + Sync {
    /// Returns custom JSON-RPC method names introduced by this provider.
    fn custom_method_names(&self) -> Vec<String>;

    /// Attempt handling a custom RPC method.
    ///
    /// Returns:
    /// - `Ok(Some(value))` when method was handled successfully
    /// - `Ok(None)` when method is not handled by this provider
    /// - `Err(...)` when method matches but handling fails
    fn try_handle(
        &self,
        method: &str,
        params: serde_json::Value,
        context: CustomRpcContext<'_>,
    ) -> tower_lsp::jsonrpc::Result<Option<serde_json::Value>>;
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
}

/// Server configuration built by the binary and passed to the core server.
#[derive(Default, Clone)]
pub struct Spec42Config {
    /// The one set of services this host publishes through, shared with the engine that embeds
    /// it so the two never hold separate memos or library strata.
    pub services: sysml_query::Services,
    /// The resolved library catalog used to enforce `.project.json` dependency admission for
    /// each independently published editor project.
    pub project_library_catalog: Option<Arc<library_catalog::LibraryCatalog>>,
    /// Optional library roots supplied by the host (e.g. materialized standard library), merged
    /// before client `libraryPaths` during LSP initialize / configuration.
    pub default_library_paths: Vec<PathBuf>,
    /// Canonical standard-library roots, kept distinct from generic host/client libraries.
    pub standard_library_paths: Vec<PathBuf>,
    /// Optional capability augmenters for additive host composition.
    pub capability_augmenters: Vec<Arc<dyn CapabilityAugmenter>>,
    /// Optional custom-method declaration providers for additive host composition.
    pub custom_method_providers: Vec<Arc<dyn CustomMethodProvider>>,
    /// Optional custom JSON-RPC method handlers for plugin composition.
    pub custom_rpc_providers: Vec<Arc<dyn CustomRpcProvider>>,
}

impl std::fmt::Debug for Spec42Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spec42Config")
            .field("services", &"shared")
            .field(
                "project_library_catalog",
                &self.project_library_catalog.is_some(),
            )
            .field("default_library_paths", &self.default_library_paths)
            .field("standard_library_paths", &self.standard_library_paths)
            .field("capability_augmenters", &self.capability_augmenters.len())
            .field(
                "custom_method_providers",
                &self.custom_method_providers.len(),
            )
            .field("custom_rpc_providers", &self.custom_rpc_providers.len())
            .finish()
    }
}

impl Spec42Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_services(mut self, services: sysml_query::Services) -> Self {
        self.services = services;
        self
    }

    pub fn with_project_library_catalog(
        mut self,
        catalog: library_catalog::LibraryCatalog,
    ) -> Self {
        self.project_library_catalog = Some(Arc::new(catalog));
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

    /// Add a custom RPC provider.
    pub fn with_custom_rpc_provider(mut self, p: Arc<dyn CustomRpcProvider>) -> Self {
        self.custom_rpc_providers.push(p);
        self
    }

    /// Host-provided library roots (prepended when merging with client `libraryPaths`).
    pub fn with_default_library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.default_library_paths = paths;
        self
    }

    pub fn with_standard_library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.standard_library_paths = paths;
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

    /// Returns custom method names contributed by RPC providers.
    pub fn custom_rpc_method_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for provider in &self.custom_rpc_providers {
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
