//! Library provisioning for hosts.
//!
//! Where libraries come from: the bundled OMG standard library and Elan8 KPAR bundles embedded at
//! build time, managed installs under the user's data directory, explicit overrides, and the
//! configuration that selects between them. The result is a [`LibraryCatalog`] of library
//! *roots* with their provenance. Which files under those roots a workspace needs is the
//! library-closure service's question, not this crate's; this crate reads no SysML.

pub mod catalog;
pub mod dependencies;
pub mod library;

pub use catalog::{
    resolve_library_catalog, HostConfigFile, HostLibraryRequest, KparLibraryComponent,
    LibraryCatalog, StdlibComponent,
};
pub use dependencies::{
    resolve_project_dependencies, resolve_project_dependency_admission,
    resolve_project_manifest_dependencies, ProjectDependencyAdmission, ProjectDependencyCandidate,
    ProjectDependencyResolution,
};
pub use kpar::PROJECT_FILE;
pub use library::{
    bundle::LibraryBundleConfig,
    managed::{
        kpar_library_paths_from_data_dir, registry_configs, KparLibraryConfig, KparLibraryPaths,
        KparLibraryStatus,
    },
    resolve_explicit_library_path,
    stdlib::{
        project_dirs, standard_library_paths_from_data_dir, StandardLibraryConfig,
        StandardLibraryPaths, StandardLibraryProjectConfig, StandardLibraryStatus,
    },
    LibraryArchive, LibraryBundle, LibraryInstallRoot, LibraryPackageRoots, LibrarySource,
    ResolvedExplicitLibrary,
};

/// Why a catalog could not be resolved. One message; the host maps it to its own error domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogError(pub String);

impl From<String> for CatalogError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CatalogError {}

pub type CatalogResult<T> = Result<T, CatalogError>;
