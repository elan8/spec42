use std::path::{Path, PathBuf};

use crate::catalog::{HostLibraryRequest, LibraryCatalog, resolve_library_catalog};
use crate::error::{WorkspaceError, WorkspaceResult};
use crate::library::{domain::DomainLibrariesConfig, stdlib::StandardLibraryConfig};
use crate::snapshot::{HostContext, HostWorkspaceSnapshot, WorkspaceLoadRequest};
use crate::version::HostSchemaVersions;
use std::sync::Arc;
use sysml_model::SysmlDocumentProvider;

/// Engine-level metadata (version identity for built snapshots).
#[derive(Debug, Clone)]
pub struct HostEngineMetadata {
    pub engine_version: String,
    pub schema_versions: HostSchemaVersions,
}

#[derive(Debug)]
pub struct Spec42Engine {
    cache_dir: PathBuf,
    catalog: LibraryCatalog,
    metadata: HostEngineMetadata,
    experimental_incremental_updates: bool,
}

#[derive(Debug)]
pub struct EngineBuilder {
    cache_dir: Option<PathBuf>,
    server_embedding_mode: bool,
    no_stdlib: bool,
    stdlib_path_override: Option<PathBuf>,
    domain_libraries_path_override: Option<PathBuf>,
    library_paths: Vec<PathBuf>,
    extra_library_paths: Vec<PathBuf>,
    standard_library: StandardLibraryConfig,
    domain_libraries: DomainLibrariesConfig,
    use_embedded_stdlib: bool,
    use_embedded_domain_libraries: bool,
    config_stdlib_path: Option<PathBuf>,
    config_no_stdlib: bool,
    experimental_incremental_updates: bool,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            cache_dir: None,
            server_embedding_mode: false,
            no_stdlib: false,
            stdlib_path_override: None,
            domain_libraries_path_override: None,
            library_paths: Vec::new(),
            extra_library_paths: Vec::new(),
            standard_library: StandardLibraryConfig::default(),
            domain_libraries: DomainLibrariesConfig::default(),
            use_embedded_stdlib: false,
            use_embedded_domain_libraries: false,
            config_stdlib_path: None,
            config_no_stdlib: false,
            // Default flipped from `false` to `true` once `try_incremental_update` had
            // sufficient correctness coverage (parity + fallback tests in
            // `workspace/tests/incremental_*.rs`). Note the measured performance win is not
            // yet proven at the snapshot-assembly layer — see
            // `docs/engineering/TIER2-UNIFIED-INCREMENTAL-ENGINE-DESIGN.md` — this default
            // reflects "one correct code path" over "two paths that can drift," not a
            // confirmed speedup. Still overridable via `.experimental_incremental_updates(false)`.
            experimental_incremental_updates: true,
        }
    }
}

impl Spec42Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn library_catalog(&self) -> &LibraryCatalog {
        &self.catalog
    }

    pub fn package_roots(&self) -> &[PathBuf] {
        &self.catalog.package_roots
    }

    pub fn metadata(&self) -> &HostEngineMetadata {
        &self.metadata
    }

    pub fn schema_versions(&self) -> HostSchemaVersions {
        self.metadata.schema_versions
    }

    pub fn experimental_incremental_updates(&self) -> bool {
        self.experimental_incremental_updates
    }

    pub fn load_workspace(
        &self,
        provider: impl SysmlDocumentProvider,
        request: WorkspaceLoadRequest,
        context: HostContext,
    ) -> WorkspaceResult<Arc<HostWorkspaceSnapshot>> {
        crate::snapshot::load_workspace_snapshot(self, provider, request, context)
    }

    pub fn update_snapshot(
        &self,
        previous: &HostWorkspaceSnapshot,
        changes: crate::snapshot::DocumentChanges,
        request: WorkspaceLoadRequest,
        context: HostContext,
    ) -> WorkspaceResult<Arc<HostWorkspaceSnapshot>> {
        crate::snapshot::update_workspace_snapshot(self, previous, changes, request, context)
    }
}

impl EngineBuilder {
    pub fn cache_dir(mut self, cache_dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = Some(cache_dir.into());
        self
    }

    pub fn server_embedding_mode(mut self, enabled: bool) -> Self {
        self.server_embedding_mode = enabled;
        self
    }

    pub fn no_stdlib(mut self, disabled: bool) -> Self {
        self.no_stdlib = disabled;
        self
    }

    pub fn standard_library_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.stdlib_path_override = Some(path.into());
        self
    }

    pub fn domain_libraries_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.domain_libraries_path_override = Some(path.into());
        self
    }

    pub fn library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.library_paths = paths;
        self
    }

    pub fn extra_library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.extra_library_paths = paths;
        self
    }

    pub fn standard_library_config(mut self, config: StandardLibraryConfig) -> Self {
        self.standard_library = config;
        self
    }

    pub fn domain_libraries_config(mut self, config: DomainLibrariesConfig) -> Self {
        self.domain_libraries = config;
        self
    }

    pub fn config_stdlib_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.config_stdlib_path = Some(path.into());
        self
    }

    pub fn config_no_stdlib(mut self, disabled: bool) -> Self {
        self.config_no_stdlib = disabled;
        self
    }

    pub fn embed_standard_library(mut self) -> Self {
        self.use_embedded_stdlib = true;
        self
    }

    pub fn embed_domain_libraries(mut self) -> Self {
        self.use_embedded_domain_libraries = true;
        self
    }

    pub fn experimental_incremental_updates(mut self, enabled: bool) -> Self {
        self.experimental_incremental_updates = enabled;
        self
    }

    pub fn build(self) -> WorkspaceResult<Spec42Engine> {
        let cache_dir = self.cache_dir.ok_or_else(|| {
            WorkspaceError::unresolved_library_environment(
                "cache_dir is required to build a Spec42Engine",
            )
        })?;
        if self.server_embedding_mode
            && self.use_embedded_stdlib
            && self.stdlib_path_override.is_none()
        {
            // Embedding hosts must either supply explicit library roots or opt into embedded bundles
            // with an explicit cache directory. No implicit profile writes occur in this mode.
        }

        let request = HostLibraryRequest {
            cache_dir: cache_dir.clone(),
            no_stdlib: self.no_stdlib,
            stdlib_path_override: self.stdlib_path_override,
            domain_libraries_path_override: self.domain_libraries_path_override,
            library_paths: self.library_paths,
            standard_library: self.standard_library,
            domain_libraries: self.domain_libraries,
            use_embedded_stdlib: self.use_embedded_stdlib,
            use_embedded_domain_libraries: self.use_embedded_domain_libraries,
            config_stdlib_path: self.config_stdlib_path,
            config_no_stdlib: self.config_no_stdlib,
            extra_library_paths: self.extra_library_paths,
        };

        let catalog = resolve_library_catalog(&request)?;
        Ok(Spec42Engine {
            cache_dir,
            catalog,
            metadata: HostEngineMetadata {
                engine_version: env!("CARGO_PKG_VERSION").to_string(),
                schema_versions: HostSchemaVersions::current(),
            },
            experimental_incremental_updates: self.experimental_incremental_updates,
        })
    }

    pub fn from_request(request: HostLibraryRequest) -> Self {
        let mut builder = Self::default()
            .cache_dir(request.cache_dir)
            .no_stdlib(request.no_stdlib)
            .config_no_stdlib(request.config_no_stdlib)
            .library_paths(request.library_paths)
            .extra_library_paths(request.extra_library_paths)
            .standard_library_config(request.standard_library)
            .domain_libraries_config(request.domain_libraries);
        if let Some(path) = request.stdlib_path_override {
            builder = builder.standard_library_path(path);
        }
        if let Some(path) = request.domain_libraries_path_override {
            builder = builder.domain_libraries_path(path);
        }
        if let Some(path) = request.config_stdlib_path {
            builder = builder.config_stdlib_path(path);
        }
        if request.use_embedded_stdlib {
            builder = builder.embed_standard_library();
        }
        if request.use_embedded_domain_libraries {
            builder = builder.embed_domain_libraries();
        }
        builder
    }
}
