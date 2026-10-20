//! Local-only decoding of Sysand's versioned lock and installed-environment contracts.
//!
//! Contract evidence: sensmetry/sysand core/src/lock.rs and
//! core/src/env/local_directory/{mod,metadata}.rs, revision recorded in LIBRARIES.md.
//! Installation paths and identifiers come from env.toml, never from project display names.
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use crate::{ProjectDependencyCandidate, ProjectDependencyCandidateKind};

pub const SYSAND_LOCK_FILE: &str = "sysand-lock.toml";
pub const SYSAND_ENV_FILE: &str = "env.toml";

#[derive(Deserialize)]
struct Lock {
    lock_version: String,
    #[serde(default, rename = "project")]
    projects: Vec<LockedProject>,
}

#[derive(Deserialize)]
struct LockedProject {
    name: String,
    version: String,
    #[serde(default)]
    identifiers: Vec<String>,
    #[serde(default)]
    usages: Vec<String>,
}

#[derive(Deserialize)]
struct Environment {
    version: String,
    #[serde(default, rename = "project")]
    projects: Vec<InstalledProject>,
}

#[derive(Deserialize)]
struct InstalledProject {
    name: String,
    version: String,
    path: String,
    #[serde(default)]
    identifiers: Vec<String>,
    #[serde(default)]
    usages: Vec<String>,
    #[serde(default)]
    editable: bool,
}

/// Installed candidates for the exact resource usages authored by this project manifest.
pub fn installed_manifest_candidates(
    root: &Path,
) -> Result<Vec<ProjectDependencyCandidate>, String> {
    let path = root.join(kpar::PROJECT_FILE);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let project: kpar::Project = serde_json::from_slice(
        &std::fs::read(&path)
            .map_err(|error| format!("Could not read {}: {error}", path.display()))?,
    )
    .map_err(|error| format!("Invalid {}: {error}", path.display()))?;
    let resources = project
        .usage
        .iter()
        .map(|usage| usage.resource.as_str())
        .collect::<Vec<_>>();
    installed_project_candidates(root, &resources, &[])
}

fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    toml::from_str(&text)
        .map_err(|error| format!("Invalid Sysand metadata {}: {error}", path.display()))
}

/// Resolve only identities requested by the owning manifest. Each candidate includes the local
/// transitive lock closure necessary to resolve references inside that dependency.
pub fn installed_project_candidates(
    project_root: &Path,
    resources: &[&str],
    baseline_resources: &[&str],
) -> Result<Vec<ProjectDependencyCandidate>, String> {
    if resources.is_empty() {
        return Ok(Vec::new());
    }
    let scope = sysml_query::source::ProjectSourceScope::load(project_root)
        .map_err(|error| error.to_string())?;
    let env_root = scope.sysand_dir();
    let env_path = env_root.join(SYSAND_ENV_FILE);
    let lock_path = project_root.join(SYSAND_LOCK_FILE);
    if !lock_path.is_file() && !env_path.is_file() {
        return Ok(Vec::new());
    }
    // A lock without an installed environment is explicitly unresolved rather than a request
    // to fetch packages. Similarly an environment without a lock is not reproducible input.
    let lock: Lock = read(&lock_path)?;
    if !matches!(lock.lock_version.as_str(), "0.5" | "0.6") {
        return Err(format!(
            "Unsupported Sysand lock version {} in {}",
            lock.lock_version,
            lock_path.display()
        ));
    }
    let environment: Environment = read(&env_path)?;
    if !matches!(environment.version.as_str(), "0.1" | "0.2") {
        return Err(format!(
            "Unsupported Sysand environment version {} in {}",
            environment.version,
            env_path.display()
        ));
    }
    let mut candidates = Vec::new();
    for &resource in resources {
        let matches = lock
            .projects
            .iter()
            .filter(|project| project.identifiers.iter().any(|id| id == resource))
            .collect::<Vec<_>>();
        let project = match matches.as_slice() {
            [] => continue, // The host catalog may own this dependency (e.g. a standard library).
            [project] => *project,
            _ => return Err(format!("Ambiguous Sysand lock identity {resource}")),
        };
        let mut roots = BTreeSet::new();
        InstalledClosure {
            lock: &lock,
            environment: &environment,
            env_root,
            baseline_resources,
        }
        .collect_roots(project, &mut BTreeSet::new(), &mut roots)?;
        candidates.push(ProjectDependencyCandidate {
            resource: resource.to_owned(),
            project_name: project.name.clone(),
            version: project.version.clone(),
            package_roots: roots.into_iter().collect(),
            kind: ProjectDependencyCandidateKind::Project,
        });
    }
    Ok(candidates)
}

struct InstalledClosure<'a> {
    lock: &'a Lock,
    environment: &'a Environment,
    env_root: &'a Path,
    baseline_resources: &'a [&'a str],
}

impl InstalledClosure<'_> {
    fn collect_roots(
        &self,
        project: &LockedProject,
        visited: &mut BTreeSet<PathBuf>,
        roots: &mut BTreeSet<PathBuf>,
    ) -> Result<(), String> {
        let Self {
            lock,
            environment,
            env_root,
            baseline_resources,
        } = self;
        let matches = environment
            .projects
            .iter()
            .filter(|installed| {
                installed.name == project.name
                    && installed.version == project.version
                    && project
                        .identifiers
                        .iter()
                        .any(|id| installed.identifiers.contains(id))
            })
            .collect::<Vec<_>>();
        let installed = match matches.as_slice() {
            [installed] => *installed,
            [] => {
                return Err(format!(
                "Sysand dependency {}@{} is locked but not installed; run sysand sync explicitly",
                project.name, project.version
            ))
            }
            _ => {
                return Err(format!(
                    "Ambiguous Sysand installation {}@{}",
                    project.name, project.version
                ))
            }
        };
        let mut expected_usages = project.usages.clone();
        expected_usages.sort();
        let mut installed_usages = installed.usages.clone();
        installed_usages.sort();
        if installed_usages != expected_usages {
            return Err(format!(
                "Sysand installation {}@{} has stale dependency metadata",
                project.name, project.version
            ));
        }
        let path = Path::new(&installed.path);
        if installed.path.is_empty()
            || installed.path.contains('\\')
            || installed.path.contains(':')
            || (!installed.editable
                && (path.is_absolute()
                    || path
                        .components()
                        .any(|component| !matches!(component, Component::Normal(_)))))
        {
            return Err(format!("Invalid installed Sysand path {}", installed.path));
        }
        let base = if installed.editable {
            env_root
                .parent()
                .ok_or_else(|| "Sysand environment has no workspace parent".to_owned())?
        } else {
            env_root
        };
        let root = std::fs::canonicalize(base.join(path))
            .map_err(|error| format!("Missing Sysand installation {}: {error}", installed.path))?;
        if !installed.editable
            && !root
                .starts_with(std::fs::canonicalize(env_root).map_err(|error| error.to_string())?)
        {
            return Err(format!(
                "Installed Sysand path escapes environment: {}",
                root.display()
            ));
        }
        let manifest_path = root.join(kpar::PROJECT_FILE);
        let manifest: kpar::Project = serde_json::from_slice(
            &std::fs::read(&manifest_path)
                .map_err(|error| format!("Could not read {}: {error}", manifest_path.display()))?,
        )
        .map_err(|error| format!("Invalid {}: {error}", manifest_path.display()))?;
        let mut declared_usages = manifest
            .usage
            .iter()
            .map(|usage| usage.resource.clone())
            .collect::<Vec<_>>();
        declared_usages.sort();
        if declared_usages != expected_usages {
            return Err(format!(
                "Sysand installed project usages differ from its lock: {}",
                manifest_path.display()
            ));
        }
        if manifest.name != installed.name || manifest.version != installed.version {
            return Err(format!(
                "Sysand installed project identity differs from metadata: {}",
                manifest_path.display()
            ));
        }
        if !visited.insert(root.clone()) {
            return Ok(());
        }
        roots.insert(root);
        for usage in &project.usages {
            if baseline_resources.contains(&usage.as_str()) {
                continue;
            }
            let matches = lock
                .projects
                .iter()
                .filter(|dependency| dependency.identifiers.contains(usage))
                .collect::<Vec<_>>();
            let dependency = match matches.as_slice() {
                [dependency] => *dependency,
                _ => {
                    return Err(format!(
                        "Missing or ambiguous transitive Sysand lock identity {usage}"
                    ))
                }
            };
            self.collect_roots(dependency, visited, roots)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESOURCE: &str = "https://example.test/types";

    fn fixture() -> tempfile::TempDir {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir_all(root.join(".sysand/lib/installed")).unwrap();
        std::fs::write(
            root.join(".sysand/lib/installed/.project.json"),
            r#"{"name":"Types","version":"1.0.0"}"#,
        )
        .unwrap();
        std::fs::write(root.join(".project.json"), format!(r#"{{"name":"Model","version":"1.0.0","usage":[{{"resource":"{RESOURCE}","versionConstraint":">=1.0.0, <3.0.0"}}]}}"#)).unwrap();
        std::fs::write(root.join(SYSAND_LOCK_FILE), format!("lock_version = '0.6'\n[[project]]\nname = 'Types'\nversion = '1.0.0'\nidentifiers = ['{RESOURCE}']\n")).unwrap();
        std::fs::write(root.join(".sysand/env.toml"), format!("version = '0.2'\n[[project]]\nname = 'Types'\nversion = '1.0.0'\nidentifiers = ['{RESOURCE}']\npath = 'lib/installed'\n")).unwrap();
        temp
    }

    #[test]
    fn installed_paths_are_metadata_owned_and_only_requested_identities_are_admitted() {
        let temp = fixture();
        let candidates = installed_manifest_candidates(temp.path()).unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].resource, RESOURCE);
        assert_eq!(candidates[0].version, "1.0.0");
        assert_eq!(
            candidates[0].package_roots,
            vec![temp
                .path()
                .join(".sysand/lib/installed")
                .canonicalize()
                .unwrap()]
        );
        assert!(installed_project_candidates(temp.path(), &["Types"], &[])
            .unwrap()
            .is_empty());
    }

    #[test]
    fn unsupported_versions_and_malformed_metadata_fail_explicitly() {
        for (path, text) in [
            (SYSAND_LOCK_FILE, "lock_version = '99'"),
            (".sysand/env.toml", "version = '99'"),
            (SYSAND_LOCK_FILE, "not toml"),
            (".sysand/env.toml", "version = 2"),
        ] {
            let temp = fixture();
            std::fs::write(temp.path().join(path), text).unwrap();
            assert!(installed_manifest_candidates(temp.path()).is_err());
        }
    }

    #[test]
    fn missing_lock_environment_or_installation_is_not_an_empty_success() {
        for path in [
            SYSAND_LOCK_FILE,
            ".sysand/env.toml",
            ".sysand/lib/installed/.project.json",
        ] {
            let temp = fixture();
            std::fs::remove_file(temp.path().join(path)).unwrap();
            assert!(installed_manifest_candidates(temp.path()).is_err());
        }
    }

    #[test]
    fn lock_pin_cannot_be_overridden_by_a_higher_catalog_version() {
        let temp = fixture();
        let mut catalog = crate::resolve_library_catalog(&crate::HostLibraryRequest {
            cache_dir: temp.path().join("cache"),
            no_stdlib: true,
            stdlib_path_override: None,
            kpar_library_path_overrides: Default::default(),
            project_library_paths: Default::default(),
            disabled_kpar_libraries: Default::default(),
            library_paths: Vec::new(),
            standard_library: Default::default(),
            use_embedded_stdlib: false,
            use_embedded_kpar_libraries: false,
            config_stdlib_path: None,
            config_no_stdlib: false,
            extra_library_paths: Vec::new(),
        })
        .unwrap();
        catalog
            .dependency_candidates
            .push(ProjectDependencyCandidate {
                resource: RESOURCE.to_owned(),
                project_name: "Types".into(),
                version: "2.0.0".into(),
                package_roots: vec![temp.path().join("other")],
                kind: ProjectDependencyCandidateKind::Project,
            });
        let admission = crate::resolve_project_dependency_admission(temp.path(), &catalog).unwrap();
        assert!(
            matches!(&admission.resolutions[0], crate::ProjectDependencyResolution::Satisfied { selected_version, .. } if selected_version == "1.0.0")
        );
        let manifest_path = temp.path().join(kpar::PROJECT_FILE);
        let manifest = std::fs::read_to_string(&manifest_path)
            .unwrap()
            .replace(">=1.0.0, <3.0.0", "2.0.0");
        std::fs::write(manifest_path, manifest).unwrap();
        assert!(crate::resolve_project_dependency_admission(temp.path(), &catalog).is_err());
        assert_eq!(
            admission.selected_candidate_roots,
            vec![temp
                .path()
                .join(".sysand/lib/installed")
                .canonicalize()
                .unwrap()]
        );
    }

    #[test]
    fn duplicate_installation_and_lock_identities_are_rejected() {
        for path in [SYSAND_LOCK_FILE, ".sysand/env.toml"] {
            let temp = fixture();
            let file = temp.path().join(path);
            let text = std::fs::read_to_string(&file).unwrap();
            let project = &text[text.find("[[project]]").unwrap()..];
            std::fs::write(&file, format!("{text}\n{project}")).unwrap();
            assert!(installed_manifest_candidates(temp.path()).is_err());
        }
    }

    #[test]
    fn traversal_is_rejected_and_editable_paths_are_explicit() {
        for path in [
            "../outside",
            "/absolute",
            "lib/../installed",
            "C:/absolute",
            "lib\\installed",
        ] {
            let temp = fixture();
            let file = temp.path().join(".sysand/env.toml");
            let text = std::fs::read_to_string(&file)
                .unwrap()
                .replace("lib/installed", path);
            std::fs::write(&file, text).unwrap();
            assert!(installed_manifest_candidates(temp.path()).is_err());
        }
        let temp = fixture();
        let file = temp.path().join(".sysand/env.toml");
        let text = std::fs::read_to_string(&file).unwrap().replace(
            "path = 'lib/installed'",
            "editable = true\npath = '.sysand/lib/installed'",
        );
        std::fs::write(file, text).unwrap();
        assert_eq!(installed_manifest_candidates(temp.path()).unwrap().len(), 1);
    }

    #[test]
    fn changes_to_installed_identity_are_seen_without_a_cache() {
        let temp = fixture();
        assert_eq!(installed_manifest_candidates(temp.path()).unwrap().len(), 1);
        std::fs::write(
            temp.path().join(".sysand/lib/installed/.project.json"),
            r#"{"name":"Types","version":"9.0.0"}"#,
        )
        .unwrap();
        assert!(installed_manifest_candidates(temp.path()).is_err());
    }

    #[test]
    fn custom_environments_resolve_editables_from_their_workspace_parent() {
        let temp = fixture();
        let root = temp.path();
        std::fs::create_dir(root.join("tools")).unwrap();
        std::fs::rename(root.join(".sysand"), root.join("tools/env")).unwrap();
        std::fs::write(root.join("syside.toml"), "sysand-dir = 'tools/env'").unwrap();
        let file = root.join("tools/env/env.toml");
        let text = std::fs::read_to_string(&file).unwrap().replace(
            "path = 'lib/installed'",
            "editable = true\npath = 'env/lib/installed'",
        );
        std::fs::write(file, text).unwrap();
        assert_eq!(
            installed_manifest_candidates(root).unwrap()[0].package_roots,
            vec![root.join("tools/env/lib/installed").canonicalize().unwrap()]
        );
    }

    #[test]
    fn a_transitive_standard_library_uses_the_host_baseline() {
        let temp = fixture();
        let baseline = "https://example.test/language-baseline";
        std::fs::write(
            temp.path().join(".sysand/lib/installed/.project.json"),
            format!(
                r#"{{"name":"Types","version":"1.0.0","usage":[{{"resource":"{baseline}"}}]}}"#
            ),
        )
        .unwrap();
        for path in [SYSAND_LOCK_FILE, ".sysand/env.toml"] {
            let file = temp.path().join(path);
            let text = std::fs::read_to_string(&file).unwrap();
            std::fs::write(file, format!("{text}usages = ['{baseline}']\n")).unwrap();
        }
        assert_eq!(
            installed_project_candidates(temp.path(), &[RESOURCE], &[baseline]).unwrap()[0]
                .package_roots
                .len(),
            1
        );
        assert!(installed_project_candidates(temp.path(), &[RESOURCE], &[]).is_err());
    }

    #[test]
    fn transitive_closure_is_local_and_unrelated_environment_entries_are_ignored() {
        let temp = fixture();
        let root = temp.path();
        std::fs::create_dir(root.join(".sysand/lib/transitive")).unwrap();
        std::fs::write(
            root.join(".sysand/lib/transitive/.project.json"),
            r#"{"name":"Base","version":"1.0.0"}"#,
        )
        .unwrap();
        std::fs::write(root.join(".sysand/lib/installed/.project.json"), r#"{"name":"Types","version":"1.0.0","usage":[{"resource":"https://example.test/base"}]}"#).unwrap();
        for path in [SYSAND_LOCK_FILE, ".sysand/env.toml"] {
            let file = root.join(path);
            let mut text = std::fs::read_to_string(&file).unwrap();
            text.push_str("usages = ['https://example.test/base']\n[[project]]\nname = 'Base'\nversion = '1.0.0'\nidentifiers = ['https://example.test/base']\n");
            if path.ends_with("env.toml") {
                text.push_str("path = 'lib/transitive'\n[[project]]\nname = 'Unused'\nversion = '1.0.0'\npath = 'missing'\nidentifiers = ['unused']\n");
            }
            std::fs::write(file, text).unwrap();
        }
        assert_eq!(
            installed_manifest_candidates(root).unwrap()[0]
                .package_roots
                .len(),
            2
        );
    }
}
