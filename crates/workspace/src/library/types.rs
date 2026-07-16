//! Typed library inputs for host embedding.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::bundle::{
    discover_library_roots, is_kpar_bytes, materialize_kpar_bytes, LibraryBundleConfig,
};

/// A `.kpar` file or in-memory archive bytes.
#[derive(Debug, Clone)]
pub enum LibraryArchive {
    Path(PathBuf),
    Bytes(Vec<u8>),
}

impl LibraryArchive {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }

    pub fn read_bytes(&self) -> Result<Vec<u8>, String> {
        match self {
            Self::Path(path) => fs::read(path)
                .map_err(|err| format!("Failed to read archive {}: {err}", path.display())),
            Self::Bytes(bytes) => Ok(bytes.clone()),
        }
    }
}

/// Versioned managed install descriptor.
pub type LibraryBundle = LibraryBundleConfig;

/// Materialized directory tree used for validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibraryInstallRoot(pub PathBuf);

/// Resolved search roots passed to validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibraryPackageRoots {
    pub roots: Vec<PathBuf>,
}

impl LibraryPackageRoots {
    pub fn from_install_root(root: &LibraryInstallRoot) -> Self {
        let roots = discover_library_roots(&root.0);
        Self {
            roots: if roots.is_empty() {
                vec![root.0.clone()]
            } else {
                roots
            },
        }
    }
}

/// Typed library input for stdlib or domain libraries.
#[derive(Debug, Clone)]
pub enum LibrarySource {
    Archive(LibraryArchive),
    Bundle(LibraryBundle),
    InstallRoot(LibraryInstallRoot),
}

/// Result of resolving an explicit host-provided library path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedExplicitLibrary {
    pub install_path: PathBuf,
    pub package_roots: LibraryPackageRoots,
    pub source: String,
}

/// Resolve a host-provided path as either a materialized archive or install root.
pub fn resolve_explicit_library_path(
    path: &Path,
    cache_dir: &Path,
    materialize_label: &str,
) -> Result<ResolvedExplicitLibrary, String> {
    let path = canonicalize_lossy(path);
    if path.is_file() {
        let bytes = fs::read(&path)
            .map_err(|err| format!("Failed to read library path {}: {err}", path.display()))?;
        if is_kpar_bytes(&bytes) {
            let materialize_root = cache_dir
                .join("materialized")
                .join(materialize_label)
                .join(stable_path_label(&path));
            if materialize_root.exists() {
                fs::remove_dir_all(&materialize_root).map_err(|err| {
                    format!(
                        "Failed to clear materialized library at {}: {err}",
                        materialize_root.display()
                    )
                })?;
            }
            fs::create_dir_all(&materialize_root)
                .map_err(|err| format!("Failed to create {}: {err}", materialize_root.display()))?;
            materialize_kpar_bytes(&bytes, &materialize_root)?;
            let install_root = LibraryInstallRoot(materialize_root);
            return Ok(ResolvedExplicitLibrary {
                install_path: install_root.0.clone(),
                package_roots: LibraryPackageRoots::from_install_root(&install_root),
                source: "archive-materialized".to_string(),
            });
        }
        return Err(format!(
            "Library path {} is a file but not a KPAR archive; pass a materialized install root instead.",
            path.display()
        ));
    }
    if path.is_dir() {
        let install_root = LibraryInstallRoot(path.clone());
        return Ok(ResolvedExplicitLibrary {
            install_path: path,
            package_roots: LibraryPackageRoots::from_install_root(&install_root),
            source: "install-root".to_string(),
        });
    }
    Err(format!(
        "Library path {} does not exist or is not readable.",
        path.display()
    ))
}

fn stable_path_label(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .unwrap_or_else(|| "library".to_string())
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}
