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
use crate::ProjectDependencyCandidate;
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
    /// Explicit resource identity to local KPAR archive bindings. Unlike managed library ids,
    /// these identities are authored verbatim by `.project.json` usages.
    pub project_library_paths: BTreeMap<String, PathBuf>,
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
    /// Installed projects with authoritative resource identities that may satisfy KPAR usages.
    /// This is empty for an installation whose provisioning contract does not declare identity.
    pub dependency_candidates: Vec<ProjectDependencyCandidate>,
    pub standard_library: StandardLibraryConfig,
    pub standard_library_paths: StandardLibraryPaths,
}

#[derive(Debug, Clone)]
struct ProjectLibraryComponent {
    candidate: ProjectDependencyCandidate,
    package_roots: Vec<PathBuf>,
}

pub fn resolve_library_catalog(request: &HostLibraryRequest) -> CatalogResult<LibraryCatalog> {
    let standard_library_paths = standard_library_paths_from_data_dir(request.cache_dir.clone());
    let stdlib = resolve_stdlib_component(request, &standard_library_paths)?;
    let kpar_libraries = resolve_kpar_libraries(request)?;
    let project_libraries = resolve_project_libraries(request)?;

    let package_roots = merge_package_roots(
        &request.library_paths,
        &request.extra_library_paths,
        &stdlib.roots,
        &kpar_libraries,
        &project_libraries,
    );

    let root_digest = hash_package_roots(&package_roots, &stdlib.roots)?;
    let mut dependency_candidates =
        stdlib_dependency_candidates(&stdlib, &request.standard_library);
    dependency_candidates.extend(kpar_dependency_candidates(&kpar_libraries));
    dependency_candidates.extend(
        project_libraries
            .iter()
            .map(|library| library.candidate.clone()),
    );

    Ok(LibraryCatalog {
        root_digest,
        package_roots,
        stdlib,
        kpar_libraries,
        dependency_candidates,
        standard_library: request.standard_library.clone(),
        standard_library_paths,
    })
}

fn resolve_project_libraries(
    request: &HostLibraryRequest,
) -> CatalogResult<Vec<ProjectLibraryComponent>> {
    request
        .project_library_paths
        .iter()
        .map(|(resource, path)| {
            if resource.trim().is_empty() {
                return Err(CatalogError(
                    "project library resource must not be empty".into(),
                ));
            }
            let archive = kpar::open_kpar_path(path).map_err(|error| {
                CatalogError(format!(
                    "Could not open project library archive {} for resource '{}': {error}",
                    path.display(),
                    resource
                ))
            })?;
            archive.project.validate_identity().map_err(|error| {
                CatalogError(format!(
                    "Invalid project library metadata in {} for resource '{}': {error}",
                    path.display(),
                    resource
                ))
            })?;
            let resolved =
                resolve_explicit_library_path(path, &request.cache_dir, "project-libraries")
                    .map_err(CatalogError::from)?;
            let package_roots = resolved.package_roots.roots;
            Ok(ProjectLibraryComponent {
                candidate: ProjectDependencyCandidate {
                    resource: resource.clone(),
                    project_name: archive.project.name,
                    version: archive.project.version,
                    package_roots: package_roots.clone(),
                },
                package_roots,
            })
        })
        .collect()
}

fn kpar_dependency_candidates(
    libraries: &[KparLibraryComponent],
) -> Vec<ProjectDependencyCandidate> {
    libraries
        .iter()
        .filter_map(|library| {
            Some(ProjectDependencyCandidate {
                resource: library.config.resource.clone()?,
                project_name: library.config.display_name.clone(),
                version: library.config.version.clone(),
                package_roots: vec![library.path.clone()?],
            })
        })
        .collect()
}

/// The KPAR schema has no self-resource field, so identities and archive/root bindings come from
/// the selected standard-library configuration. No display-name or filename inference occurs.
fn stdlib_dependency_candidates(
    stdlib: &StdlibComponent,
    config: &StandardLibraryConfig,
) -> Vec<ProjectDependencyCandidate> {
    config
        .projects
        .iter()
        .filter_map(|project| {
            let root_name = project.archive.strip_suffix(".kpar")?;
            let root = stdlib
                .roots
                .iter()
                .find(|root| root.file_name().and_then(|name| name.to_str()) == Some(root_name))?;
            Some(
                project
                    .resources
                    .iter()
                    .map(move |resource| ProjectDependencyCandidate {
                        resource: resource.clone(),
                        project_name: project.name.clone(),
                        version: project.version.clone(),
                        package_roots: vec![root.clone()],
                    }),
            )
        })
        .flatten()
        .collect()
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
                roots: stdlib_library_roots(&managed_path, Some(&metadata)),
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
            roots: stdlib_library_roots(&path, Some(&metadata)),
            path: Some(path),
            source: Some("bundled".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(path) = legacy_vscode_stdlib_path(&request.standard_library) {
        return Ok(StdlibComponent {
            roots: stdlib_library_roots(&path, None),
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
            resource: None,
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
    project_libraries: &[ProjectLibraryComponent],
) -> Vec<PathBuf> {
    let mut paths = library_paths.to_vec();
    paths.extend(extra_library_paths.iter().cloned());
    paths.extend(stdlib_roots.iter().cloned());
    for library in kpar_libraries {
        if let Some(path) = &library.path {
            paths.push(path.clone());
        }
    }
    for library in project_libraries {
        paths.extend(library.package_roots.iter().cloned());
    }

    let mut deduped = BTreeSet::new();
    paths
        .into_iter()
        .filter(|path| deduped.insert(path.display().to_string()))
        .collect()
}

/// Scans every configured package root (in configured precedence order) and hashes every
/// admitted source file's actual bytes into a [`RootDigest`] (plan §5.2/§5.3). A version string
/// or install directory alone is never sufficient identity for a mutable local library root;
/// managed/embedded roots are content-addressed exactly the same way here so their digest also
/// transitively commits every installed file.
fn hash_package_roots(
    package_roots: &[PathBuf],
    stdlib_roots: &[PathBuf],
) -> CatalogResult<RootDigest> {
    let mut library_root_groups: Vec<Vec<SourceManifestEntry>> = Vec::new();
    for (slot, root) in package_roots.iter().enumerate() {
        let role = if stdlib_roots.contains(root) {
            SourceRole::StandardLibrary
        } else {
            SourceRole::Library
        };
        library_root_groups.push(scan_library_root(root, slot as u32, role)?);
    }
    Ok(SourceManifest::new(Vec::new(), library_root_groups).root_digest())
}

fn scan_library_root(
    root: &Path,
    slot: u32,
    role: SourceRole,
) -> CatalogResult<Vec<SourceManifestEntry>> {
    let mut entries = Vec::new();
    if !root.exists() {
        return Ok(entries);
    }
    for entry in walkdir::WalkDir::new(root).follow_links(false).into_iter() {
        let entry = entry.map_err(|error| {
            CatalogError(format!(
                "cannot completely scan library root {}: {error}",
                root.display()
            ))
        })?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if !sysml_query::source::is_sysml_like(path) {
            continue;
        }
        let bytes = std::fs::read(path).map_err(|error| {
            CatalogError(format!(
                "cannot read library source {} while computing root identity: {error}",
                path.display()
            ))
        })?;
        let relative_path = path
            .strip_prefix(root)
            .map_err(|error| {
                CatalogError(format!(
                    "library source {} is not under configured root {}: {error}",
                    path.display(),
                    root.display()
                ))
            })?
            .to_string_lossy()
            .replace('\\', "/");
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
    Ok(entries)
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

#[cfg(test)]
mod tests {
    use super::*;
    use kpar::{ArchiveCompression, ArchiveTimestamp, PackOptions, Project, ProjectUsage};

    fn project(name: &str, version: &str) -> Project {
        Project {
            name: name.into(),
            version: version.into(),
            description: None,
            license: None,
            publisher: None,
            maintainer: Vec::new(),
            website: None,
            topic: Vec::new(),
            usage: Vec::new(),
        }
    }

    #[test]
    fn explicit_project_archive_is_an_ordinary_versioned_dependency_candidate() {
        let temp = tempfile::tempdir().unwrap();
        let sources = temp.path().join("sources");
        std::fs::create_dir(&sources).unwrap();
        std::fs::write(sources.join("Alternative.sysml"), "package Alternative;").unwrap();
        let archive_path = temp.path().join("anything-at-all.kpar");
        kpar::build_kpar(
            &PackOptions {
                project: project("Alternative-Standard-Library", "9.0.0"),
                source_roots: vec![sources],
                named_source_roots: Vec::new(),
                excludes: Vec::new(),
                timestamp: ArchiveTimestamp::default(),
                compression: ArchiveCompression::Stored,
            },
            &archive_path,
        )
        .unwrap();

        let resource = "https://example.test/stdlib";
        let request = HostLibraryRequest {
            cache_dir: temp.path().join("cache"),
            no_stdlib: true,
            stdlib_path_override: None,
            kpar_library_path_overrides: BTreeMap::new(),
            project_library_paths: BTreeMap::from([(resource.into(), archive_path)]),
            disabled_kpar_libraries: BTreeSet::new(),
            library_paths: Vec::new(),
            standard_library: StandardLibraryConfig::default(),
            use_embedded_stdlib: false,
            use_embedded_kpar_libraries: false,
            config_stdlib_path: None,
            config_no_stdlib: false,
            extra_library_paths: Vec::new(),
        };
        let mut catalog = resolve_library_catalog(&request).unwrap();
        let provided = catalog
            .dependency_candidates
            .iter()
            .find(|candidate| candidate.resource == resource)
            .cloned()
            .unwrap();
        assert_eq!(provided.project_name, "Alternative-Standard-Library");
        assert_eq!(provided.version, "9.0.0");
        assert!(provided.package_roots.iter().all(|root| root.is_dir()));
        catalog.stdlib.roots = provided.package_roots.clone();
        assert_eq!(
            crate::manifest_usages_for_standard_library(&catalog).unwrap(),
            vec![ProjectUsage {
                resource: resource.into(),
                version_constraint: Some("9.0.0".into()),
            }]
        );

        let mut candidates = vec![ProjectDependencyCandidate {
            resource: resource.into(),
            project_name: "Bundled-Default".into(),
            version: "1.0.0".into(),
            package_roots: vec![PathBuf::from("bundled")],
        }];
        candidates.push(provided.clone());
        let mut consumer = project("Consumer", "1.0.0");
        consumer.usage.push(ProjectUsage {
            resource: resource.into(),
            version_constraint: Some("9.0.0".into()),
        });
        let resolved = crate::resolve_project_dependencies(&consumer, &candidates);
        assert!(matches!(
            &resolved[0],
            crate::ProjectDependencyResolution::Satisfied { selected_version, project_name, .. }
                if selected_version == "9.0.0" && project_name == "Alternative-Standard-Library"
        ));
    }

    #[test]
    fn root_digest_commits_configured_precedence_for_colliding_relative_paths() {
        let temp = tempfile::tempdir().unwrap();
        let first = temp.path().join("first");
        let second = temp.path().join("second");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        std::fs::write(first.join("collision.sysml"), "package First;").unwrap();
        std::fs::write(second.join("collision.sysml"), "package Second;").unwrap();

        let forward = hash_package_roots(&[first.clone(), second.clone()], &[]).unwrap();
        let reverse = hash_package_roots(&[second, first], &[]).unwrap();
        assert_ne!(forward, reverse);
    }

    #[test]
    fn bundled_stdlib_dependency_mapping_has_no_missing_pinned_roots() {
        let roots = [
            "Kernel_Semantic_Library-1.0.0",
            "Kernel_Data_Type_Library-1.0.0",
            "Kernel_Function_Library-1.0.0",
            "SysML_Systems_Library-2.0.0",
            "SysML_Quantities_and_Units_Library-2.0.0",
            "SysML_Analysis_Library-2.0.0",
            "SysML_Cause_and_Effect_Library-2.0.0",
            "SysML_Geometry_Library-2.0.0",
            "SysML_Metadata_Library-2.0.0",
            "SysML_Requirement_Derivation_Library-2.0.0",
        ]
        .map(PathBuf::from)
        .to_vec();
        let candidates = stdlib_dependency_candidates(
            &StdlibComponent {
                path: Some(PathBuf::from("stdlib")),
                roots,
                source: Some("bundled".into()),
                used_legacy_vscode_fallback: false,
            },
            &StandardLibraryConfig::default(),
        );
        assert_eq!(candidates.len(), 11, "pinned mapping and roots drifted");
        assert!(candidates
            .iter()
            .all(|candidate| candidate.package_roots.len() == 1));
    }

    #[test]
    fn explicitly_provisioned_stdlib_uses_its_declared_project_resources() {
        let root = PathBuf::from("Custom_Library-3.0.0");
        let stdlib = StdlibComponent {
            path: Some(PathBuf::from("custom")),
            roots: vec![root.clone()],
            source: Some("flag".into()),
            used_legacy_vscode_fallback: false,
        };
        let config = StandardLibraryConfig {
            projects: vec![crate::StandardLibraryProjectConfig {
                archive: "Custom_Library-3.0.0.kpar".into(),
                name: "Custom Library".into(),
                version: "3.0.0".into(),
                resources: vec!["urn:example:custom-library".into()],
            }],
            ..StandardLibraryConfig::default()
        };
        assert_eq!(
            stdlib_dependency_candidates(&stdlib, &config),
            vec![ProjectDependencyCandidate {
                resource: "urn:example:custom-library".into(),
                project_name: "Custom Library".into(),
                version: "3.0.0".into(),
                package_roots: vec![root],
            }]
        );
    }

    #[cfg(unix)]
    #[test]
    fn unreadable_source_makes_catalog_identity_fail() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("unreadable.sysml");
        std::fs::write(&source, "package Hidden;").unwrap();
        std::fs::set_permissions(&source, std::fs::Permissions::from_mode(0o000)).unwrap();
        let result = hash_package_roots(&[temp.path().to_path_buf()], &[]);
        std::fs::set_permissions(&source, std::fs::Permissions::from_mode(0o600)).unwrap();

        assert!(
            result.is_err(),
            "unreadable bytes must not be silently omitted"
        );
    }
}
