//! Library catalog resolution for host embedding.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sysml_query::source::identity::{RootDigest, SourceManifest, SourceManifestEntry, SourceRole};

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
use crate::{CatalogError, CatalogResult};

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
    pub disabled_kpar_libraries: BTreeSet<String>,
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
    /// Verified content identity of every admitted library source byte under every configured
    /// package root, in configured precedence order (plan §5.2/§5.3). Computed by scanning and
    /// hashing actual file content, not only paths and configured versions.
    pub root_digest: RootDigest,
    pub package_roots: Vec<PathBuf>,
    pub stdlib: StdlibComponent,
    pub kpar_libraries: Vec<KparLibraryComponent>,
    pub standard_library: StandardLibraryConfig,
    pub standard_library_paths: StandardLibraryPaths,
}

pub fn resolve_library_catalog(request: &HostLibraryRequest) -> CatalogResult<LibraryCatalog> {
    let standard_library_paths = standard_library_paths_from_data_dir(request.cache_dir.clone());
    let stdlib = resolve_stdlib_component(request, &standard_library_paths)?;
    let kpar_libraries = resolve_kpar_libraries(request)?;

    let package_roots = merge_package_roots(
        &request.library_paths,
        &request.extra_library_paths,
        &stdlib.roots,
        &kpar_libraries,
    );

    let root_digest = hash_package_roots(&package_roots, &stdlib.roots);

    Ok(LibraryCatalog {
        root_digest,
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
) -> CatalogResult<StdlibComponent> {
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
            .map_err(CatalogError::from)?;
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
            .map_err(CatalogError::from)?;
        return Ok(StdlibComponent {
            path: Some(resolved.install_path),
            roots: resolved.package_roots.roots,
            source: Some("env".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }
    if let Some(path) = request.config_stdlib_path.as_ref() {
        let resolved = resolve_explicit_library_path(path, &request.cache_dir, "standard-library")
            .map_err(CatalogError::from)?;
        return Ok(StdlibComponent {
            path: Some(resolved.install_path),
            roots: resolved.package_roots.roots,
            source: Some("config".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(metadata) =
        load_managed_metadata(standard_library_paths).map_err(CatalogError::from)?
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
                .map_err(CatalogError::from)?;
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
) -> CatalogResult<Vec<KparLibraryComponent>> {
    let mut components = Vec::new();
    let mut registered_ids = BTreeSet::new();
    for config in registry_configs() {
        registered_ids.insert(config.id.clone());
        let paths = kpar_library_paths_from_data_dir(&request.cache_dir, &config.id);
        let component = resolve_one_kpar_library(request, config, paths)?;
        components.push(component);
    }

    // Any override id that isn't a registered library is treated as a manually
    // added, ad-hoc KPAR library (a `.kpar` file or a materialized install root).
    for (id, path) in &request.kpar_library_path_overrides {
        if registered_ids.contains(id) || request.disabled_kpar_libraries.contains(id) {
            continue;
        }
        let resolved = resolve_explicit_library_path(path, &request.cache_dir, id)
            .map_err(CatalogError::from)?;
        let paths = kpar_library_paths_from_data_dir(&request.cache_dir, id);
        let config = KparLibraryConfig {
            id: id.clone(),
            display_name: id.clone(),
            version: "local".to_string(),
            repo: String::new(),
            content_path: String::new(),
            format: "kpar".to_string(),
            artifact: None,
        };
        components.push(KparLibraryComponent {
            id: id.clone(),
            display_name: id.clone(),
            path: Some(resolved.install_path),
            source: Some("custom".to_string()),
            config,
            paths,
        });
    }

    Ok(components)
}

fn resolve_one_kpar_library(
    request: &HostLibraryRequest,
    config: KparLibraryConfig,
    paths: KparLibraryPaths,
) -> CatalogResult<KparLibraryComponent> {
    if request.disabled_kpar_libraries.contains(&config.id) {
        return Ok(KparLibraryComponent {
            id: config.id.clone(),
            display_name: config.display_name.clone(),
            path: None,
            source: Some("disabled".to_string()),
            config,
            paths,
        });
    }

    if let Some(path) = request.kpar_library_path_overrides.get(&config.id) {
        let resolved = resolve_explicit_library_path(path, &request.cache_dir, &config.id)
            .map_err(CatalogError::from)?;
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
            .map_err(CatalogError::from)?;
        return Ok(KparLibraryComponent {
            id: config.id.clone(),
            display_name: config.display_name.clone(),
            path: Some(resolved.install_path),
            source: Some("env".to_string()),
            config,
            paths,
        });
    }

    if let Some(metadata) = load_kpar_library_metadata(&paths).map_err(CatalogError::from)? {
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

/// Scans every configured package root (in configured precedence order) and hashes every
/// admitted source file's actual bytes into a [`RootDigest`] (plan §5.2/§5.3). A version string
/// or install directory alone is never sufficient identity for a mutable local library root;
/// managed/embedded roots are content-addressed exactly the same way here so their digest also
/// transitively commits every installed file.
fn hash_package_roots(package_roots: &[PathBuf], stdlib_roots: &[PathBuf]) -> RootDigest {
    let mut library_root_groups: Vec<Vec<SourceManifestEntry>> = Vec::new();
    for (slot, root) in package_roots.iter().enumerate() {
        let role = if stdlib_roots.contains(root) {
            SourceRole::StandardLibrary
        } else {
            SourceRole::Library
        };
        library_root_groups.push(scan_library_root(root, slot as u32, role));
    }
    SourceManifest::new(Vec::new(), library_root_groups).root_digest()
}

fn scan_library_root(root: &Path, slot: u32, role: SourceRole) -> Vec<SourceManifestEntry> {
    let mut entries = Vec::new();
    if !root.exists() {
        return entries;
    }
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let is_admitted = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                ext.eq_ignore_ascii_case("sysml") || ext.eq_ignore_ascii_case("kerml")
            });
        if !is_admitted {
            continue;
        }
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        let relative_path = path
            .strip_prefix(root)
            .ok()
            .map(|relative| relative.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|| path.display().to_string());
        let uri = format!("file://{}", path.display());
        entries.push(SourceManifestEntry {
            uri,
            path_hint: Some(relative_path.clone()),
            role,
            content_digest: sysml_query::source::ContentDigest::of_bytes(&bytes),
            byte_len: bytes.len() as u64,
            library_root_slot: Some(slot),
            relative_path: Some(relative_path),
        });
    }
    entries
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

pub fn resolve_stdlib_component_for_test(
    request: &HostLibraryRequest,
    standard_library_paths: &StandardLibraryPaths,
) -> CatalogResult<StdlibComponent> {
    resolve_stdlib_component(request, standard_library_paths)
}

pub fn resolve_kpar_libraries_for_test(
    request: &HostLibraryRequest,
) -> CatalogResult<Vec<KparLibraryComponent>> {
    resolve_kpar_libraries(request)
}
