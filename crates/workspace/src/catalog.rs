//! Library catalog resolution for host embedding.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::{WorkspaceError, WorkspaceResult};
use crate::library::{
    managed::{
        install_embedded_kpar_library, kpar_library_paths_from_data_dir,
        load_managed_metadata as load_kpar_library_metadata,
        managed_install_path as kpar_managed_install_path, registry_configs, KparLibraryConfig,
        KparLibraryPaths, EMBEDDED_KPAR_LIBRARY_REPO,
    },
    resolve_explicit_library_path,
    stdlib::{
        install_embedded_standard_library, legacy_vscode_stdlib_path, load_managed_metadata,
        standard_library_paths_from_data_dir, stdlib_library_roots, StandardLibraryConfig,
        StandardLibraryPaths, EMBEDDED_STDLIB_ARCHIVE, EMBEDDED_STDLIB_REPO,
    },
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct HostConfigFile {
    pub library_paths: Option<Vec<String>>,
    pub stdlib_path: Option<String>,
    pub no_stdlib: Option<bool>,
    pub standard_library_version: Option<String>,
    pub standard_library_repo: Option<String>,
    pub standard_library_content_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HostLibraryRequest {
    pub cache_dir: PathBuf,
    pub no_stdlib: bool,
    pub stdlib_path_override: Option<PathBuf>,
    pub kpar_library_path_overrides: BTreeMap<String, PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub standard_library: StandardLibraryConfig,
    pub use_embedded_stdlib: bool,
    pub use_embedded_kpar_libraries: bool,
    pub config_stdlib_path: Option<PathBuf>,
    pub config_no_stdlib: bool,
    pub extra_library_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StdlibComponent {
    pub path: Option<PathBuf>,
    pub roots: Vec<PathBuf>,
    pub source: Option<String>,
    pub used_legacy_vscode_fallback: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct KparLibraryComponent {
    pub id: String,
    pub display_name: String,
    pub path: Option<PathBuf>,
    pub source: Option<String>,
    pub config: KparLibraryConfig,
    pub paths: KparLibraryPaths,
}

#[derive(Debug, Clone, Serialize)]
pub struct LibraryCatalog {
    pub content_hash: String,
    pub package_roots: Vec<PathBuf>,
    pub stdlib: StdlibComponent,
    pub kpar_libraries: Vec<KparLibraryComponent>,
    pub standard_library: StandardLibraryConfig,
    pub standard_library_paths: StandardLibraryPaths,
}

pub fn resolve_library_catalog(request: &HostLibraryRequest) -> WorkspaceResult<LibraryCatalog> {
    let standard_library_paths = standard_library_paths_from_data_dir(request.cache_dir.clone());
    let stdlib = resolve_stdlib_component(request, &standard_library_paths)?;
    let kpar_libraries = resolve_kpar_libraries(request)?;

    let package_roots = merge_package_roots(
        &request.library_paths,
        &request.extra_library_paths,
        &stdlib.roots,
        &kpar_libraries,
    );

    let content_hash =
        hash_package_roots(&package_roots, &request.standard_library, &kpar_libraries);

    Ok(LibraryCatalog {
        content_hash,
        package_roots,
        stdlib,
        kpar_libraries,
        standard_library: request.standard_library.clone(),
        standard_library_paths,
    })
}

fn resolve_stdlib_component(
    request: &HostLibraryRequest,
    standard_library_paths: &StandardLibraryPaths,
) -> WorkspaceResult<StdlibComponent> {
    if request.no_stdlib
        || request.config_no_stdlib
        || std::env::var("SPEC42_NO_STDLIB")
            .map(|value| matches!(value.trim(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false)
    {
        return Ok(StdlibComponent {
            path: None,
            roots: Vec::new(),
            source: Some("disabled".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(path) = request.stdlib_path_override.as_ref() {
        let resolved = resolve_explicit_library_path(path, &request.cache_dir, "standard-library")
            .map_err(WorkspaceError::unresolved_library_environment)?;
        return Ok(StdlibComponent {
            path: Some(resolved.install_path),
            roots: resolved.package_roots.roots,
            source: Some("flag".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }
    if let Some(value) = std::env::var_os("SPEC42_STDLIB_PATH") {
        let path = PathBuf::from(value);
        let resolved = resolve_explicit_library_path(&path, &request.cache_dir, "standard-library")
            .map_err(WorkspaceError::unresolved_library_environment)?;
        return Ok(StdlibComponent {
            path: Some(resolved.install_path),
            roots: resolved.package_roots.roots,
            source: Some("env".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }
    if let Some(path) = request.config_stdlib_path.as_ref() {
        let resolved = resolve_explicit_library_path(path, &request.cache_dir, "standard-library")
            .map_err(WorkspaceError::unresolved_library_environment)?;
        return Ok(StdlibComponent {
            path: Some(resolved.install_path),
            roots: resolved.package_roots.roots,
            source: Some("config".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(metadata) = load_managed_metadata(standard_library_paths)
        .map_err(WorkspaceError::unresolved_library_environment)?
    {
        let managed_path = PathBuf::from(&metadata.install_path);
        let expected_path = crate::library::stdlib::managed_install_path(
            standard_library_paths,
            &request.standard_library,
        );
        let metadata_is_current = metadata.installed_version == request.standard_library.version
            && canonicalize_lossy(&managed_path) == canonicalize_lossy(&expected_path);
        if metadata_is_current && crate::library::stdlib::install_path_is_ready(&managed_path) {
            let source = if metadata.repo == EMBEDDED_STDLIB_REPO {
                "bundled".to_string()
            } else {
                "managed".to_string()
            };
            return Ok(StdlibComponent {
                path: Some(managed_path.clone()),
                roots: stdlib_resolution_roots(&managed_path, Some(&metadata)),
                source: Some(source),
                used_legacy_vscode_fallback: false,
            });
        }
    }

    #[allow(clippy::const_is_empty)]
    if request.use_embedded_stdlib && !EMBEDDED_STDLIB_ARCHIVE.is_empty() {
        let metadata =
            install_embedded_standard_library(standard_library_paths, &request.standard_library)
                .map_err(WorkspaceError::unresolved_library_environment)?;
        let path = PathBuf::from(&metadata.install_path);
        return Ok(StdlibComponent {
            roots: stdlib_resolution_roots(&path, Some(&metadata)),
            path: Some(path),
            source: Some("bundled".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(path) = legacy_vscode_stdlib_path(&request.standard_library) {
        return Ok(StdlibComponent {
            roots: stdlib_resolution_roots(&path, None),
            path: Some(path),
            source: Some("legacy-vscode".to_string()),
            used_legacy_vscode_fallback: true,
        });
    }

    Ok(StdlibComponent {
        path: None,
        roots: Vec::new(),
        source: None,
        used_legacy_vscode_fallback: false,
    })
}

fn resolve_kpar_libraries(
    request: &HostLibraryRequest,
) -> WorkspaceResult<Vec<KparLibraryComponent>> {
    let mut components = Vec::new();
    for config in registry_configs() {
        let paths = kpar_library_paths_from_data_dir(&request.cache_dir, &config.id);
        let component = resolve_one_kpar_library(request, config, paths)?;
        components.push(component);
    }
    Ok(components)
}

fn resolve_one_kpar_library(
    request: &HostLibraryRequest,
    config: KparLibraryConfig,
    paths: KparLibraryPaths,
) -> WorkspaceResult<KparLibraryComponent> {
    if let Some(path) = request.kpar_library_path_overrides.get(&config.id) {
        let resolved = resolve_explicit_library_path(path, &request.cache_dir, &config.id)
            .map_err(WorkspaceError::unresolved_library_environment)?;
        return Ok(KparLibraryComponent {
            id: config.id.clone(),
            display_name: config.display_name.clone(),
            path: Some(resolved.install_path),
            source: Some("flag".to_string()),
            config,
            paths,
        });
    }

    let env_key = format!(
        "SPEC42_KPAR_LIBRARY_PATH_{}",
        config.id.to_ascii_uppercase().replace('-', "_")
    );
    if let Some(value) = std::env::var_os(&env_key) {
        let path = PathBuf::from(value);
        let resolved = resolve_explicit_library_path(&path, &request.cache_dir, &config.id)
            .map_err(WorkspaceError::unresolved_library_environment)?;
        return Ok(KparLibraryComponent {
            id: config.id.clone(),
            display_name: config.display_name.clone(),
            path: Some(resolved.install_path),
            source: Some("env".to_string()),
            config,
            paths,
        });
    }

    if let Some(metadata) = load_kpar_library_metadata(&paths)
        .map_err(WorkspaceError::unresolved_library_environment)?
    {
        let managed_path = PathBuf::from(&metadata.install_path);
        let expected_path = kpar_managed_install_path(&paths, &config);
        let metadata_is_current = metadata.installed_version == config.version
            && canonicalize_lossy(&managed_path) == canonicalize_lossy(&expected_path);
        if metadata_is_current && crate::library::stdlib::install_path_is_ready(&managed_path) {
            let source = if metadata.repo == EMBEDDED_KPAR_LIBRARY_REPO {
                "bundled".to_string()
            } else {
                "managed".to_string()
            };
            return Ok(KparLibraryComponent {
                id: config.id.clone(),
                display_name: config.display_name.clone(),
                path: Some(managed_path),
                source: Some(source),
                config,
                paths,
            });
        }
    }

    if request.use_embedded_kpar_libraries {
        if let Ok(metadata) = install_embedded_kpar_library(&paths, &config) {
            return Ok(KparLibraryComponent {
                id: config.id.clone(),
                display_name: config.display_name.clone(),
                path: Some(PathBuf::from(metadata.install_path)),
                source: Some("bundled".to_string()),
                config,
                paths,
            });
        }
    }

    Ok(KparLibraryComponent {
        id: config.id.clone(),
        display_name: config.display_name.clone(),
        path: None,
        source: None,
        config,
        paths,
    })
}

fn merge_package_roots(
    library_paths: &[PathBuf],
    extra_library_paths: &[PathBuf],
    stdlib_roots: &[PathBuf],
    kpar_libraries: &[KparLibraryComponent],
) -> Vec<PathBuf> {
    let mut paths = library_paths.to_vec();
    paths.extend(extra_library_paths.iter().cloned());
    paths.extend(stdlib_roots.iter().cloned());
    for library in kpar_libraries {
        if let Some(path) = &library.path {
            paths.push(path.clone());
        }
    }

    let mut deduped = BTreeSet::new();
    paths
        .into_iter()
        .filter(|path| deduped.insert(path.display().to_string()))
        .collect()
}

fn stdlib_resolution_roots(
    install_path: &Path,
    metadata: Option<&crate::library::stdlib::StandardLibraryMetadata>,
) -> Vec<PathBuf> {
    let roots = stdlib_library_roots(install_path, metadata);
    if roots.is_empty() {
        vec![install_path.to_path_buf()]
    } else {
        roots
    }
}

fn hash_package_roots(
    package_roots: &[PathBuf],
    standard_library: &StandardLibraryConfig,
    kpar_libraries: &[KparLibraryComponent],
) -> String {
    let mut hasher = Sha256::new();
    for root in package_roots {
        hasher.update(root.display().to_string().as_bytes());
        hasher.update([0]);
    }
    hasher.update(standard_library.version.as_bytes());
    hasher.update([0]);
    for library in kpar_libraries {
        hasher.update(library.id.as_bytes());
        hasher.update([0]);
        hasher.update(library.config.version.as_bytes());
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub fn resolve_stdlib_component_for_test(
    request: &HostLibraryRequest,
    standard_library_paths: &StandardLibraryPaths,
) -> WorkspaceResult<StdlibComponent> {
    resolve_stdlib_component(request, standard_library_paths)
}

pub fn resolve_kpar_libraries_for_test(
    request: &HostLibraryRequest,
) -> WorkspaceResult<Vec<KparLibraryComponent>> {
    resolve_kpar_libraries(request)
}
