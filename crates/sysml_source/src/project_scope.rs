//! Project-owned source selection shared by batch and editor hosts.
use std::path::{Path, PathBuf};

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use serde::Deserialize;

use crate::{path_to_file_url, SourceError};

fn normalized_path(path: &Path) -> PathBuf {
    path_to_file_url(path)
        .ok()
        .and_then(|uri| uri.to_file_path().ok())
        .unwrap_or_else(|| path.to_path_buf())
}

pub const PROJECT_CONFIG_FILE: &str = "spec42.toml";

/// Project source admission inside an editor discovery ceiling. Installed projects may own
/// `.project.json` manifests, but do not become workspace projects when their files are opened.
pub fn workspace_source_selected(source: &Path, ceiling: &Path) -> Result<bool, SourceError> {
    let source = normalized_path(source);
    let ceiling = normalized_path(ceiling);
    let Some(boundary) = crate::discover_project_boundary(&source, &ceiling) else {
        return Ok(false);
    };
    if !ProjectSourceScope::load(boundary.project_root())?.includes(&source) {
        return Ok(false);
    }
    for ancestor in boundary
        .project_root()
        .parent()
        .into_iter()
        .flat_map(Path::ancestors)
    {
        let ancestor = normalized_path(ancestor);
        if !ancestor.starts_with(&ceiling) {
            break;
        }
        if ancestor == ceiling || ancestor.join(crate::PROJECT_MANIFEST_FILE).is_file() {
            let scope = ProjectSourceScope::load(&ancestor)?;
            if source.starts_with(normalized_path(scope.sysand_dir())) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SourceConfig {
    include: Option<Vec<String>>,
    #[serde(default)]
    exclude: Vec<String>,
    sysand_dir: Option<PathBuf>,
    /// External library roots. The key stays `library_paths` so it matches the host config
    /// file; the struct's kebab-case rename would otherwise spell it `library-paths`.
    #[serde(default, rename = "library_paths")]
    library_paths: Vec<PathBuf>,
}

/// Project source selection and extra library roots from `spec42.toml`.
///
/// Paths are relative to the project configuration directory. `include` and `exclude` select
/// files inside that directory. `library_paths` adds directories, including siblings, as library
/// roots. Spec42 does not read `syside.toml`.
#[derive(Debug, Clone)]
pub struct ProjectSourceScope {
    root: PathBuf,
    include: Option<GlobSet>,
    exclude: GlobSet,
    sysand_dir: PathBuf,
    library_paths: Vec<PathBuf>,
}

impl ProjectSourceScope {
    pub fn load(root: &Path) -> Result<Self, SourceError> {
        let root = normalized_path(root);
        let path = root.join(PROJECT_CONFIG_FILE);
        let config: SourceConfig = match std::fs::read_to_string(&path) {
            Ok(text) => toml::from_str(&text).map_err(|error| {
                SourceError::Provider(format!(
                    "Invalid source configuration {}: {error}",
                    path.display()
                ))
            })?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => SourceConfig::default(),
            Err(error) => {
                return Err(SourceError::Provider(format!(
                    "Could not read {}: {error}",
                    path.display()
                )))
            }
        };
        if let Some(include) = &config.include {
            reject_escaping_globs(include, &path)?;
        }
        reject_escaping_globs(&config.exclude, &path)?;
        let library_paths = config
            .library_paths
            .iter()
            .map(|raw| resolve_library_path(&root, raw, &path))
            .collect::<Result<Vec<_>, _>>()?;
        let mut library_paths_deduped = Vec::new();
        for library in library_paths {
            if !library_paths_deduped
                .iter()
                .any(|existing: &PathBuf| existing == &library)
            {
                library_paths_deduped.push(library);
            }
        }
        let compile = |patterns: &[String]| -> Result<GlobSet, SourceError> {
            let mut builder = GlobSetBuilder::new();
            for pattern in patterns {
                let glob = GlobBuilder::new(pattern)
                    .literal_separator(true)
                    .build()
                    .map_err(|error| {
                        SourceError::Provider(format!(
                            "Invalid source glob in {}: {error}",
                            path.display()
                        ))
                    })?;
                builder.add(glob);
            }
            builder
                .build()
                .map_err(|error| SourceError::Provider(error.to_string()))
        };
        Ok(Self {
            include: config.include.as_deref().map(compile).transpose()?,
            exclude: compile(&config.exclude)?,
            sysand_dir: root.join(
                config
                    .sysand_dir
                    .unwrap_or_else(|| PathBuf::from(".sysand")),
            ),
            library_paths: library_paths_deduped,
            root,
        })
    }

    /// Directories named by `library_paths`, canonicalized. Empty when the file omits the key.
    pub fn library_paths(&self) -> &[PathBuf] {
        &self.library_paths
    }

    pub fn includes(&self, path: &Path) -> bool {
        let path = normalized_path(path);
        let Ok(relative) = path.strip_prefix(&self.root) else {
            return false;
        };
        if relative
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return false;
        }
        // Always use portable separators, including for not-yet-created watcher paths.
        let relative = relative.to_string_lossy().replace('\\', "/");
        self.include
            .as_ref()
            .is_none_or(|include| include.is_match(&relative))
            && !self.exclude.is_match(&relative)
            && !path.starts_with(normalized_path(&self.sysand_dir))
    }

    pub fn sysand_dir(&self) -> &Path {
        &self.sysand_dir
    }
}

fn reject_escaping_globs(patterns: &[String], config: &Path) -> Result<(), SourceError> {
    for pattern in patterns {
        if pattern_stays_in_project(pattern) {
            continue;
        }
        return Err(SourceError::Provider(format!(
            "Source pattern `{pattern}` in {} escapes the project root. include and exclude select files inside the project; add an external folder with library_paths.",
            config.display()
        )));
    }
    Ok(())
}

fn pattern_stays_in_project(pattern: &str) -> bool {
    let normalized = pattern.replace('\\', "/");
    if normalized.starts_with('/') {
        return false;
    }
    let bytes = normalized.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' {
        return false;
    }
    normalized.split('/').all(|component| component != "..")
}

fn resolve_library_path(root: &Path, raw: &Path, config: &Path) -> Result<PathBuf, SourceError> {
    let joined = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        root.join(raw)
    };
    if !joined.is_dir() {
        return Err(SourceError::Provider(format!(
            "Library path {} in {} is not a directory",
            joined.display(),
            config.display()
        )));
    }
    Ok(normalized_path(&joined))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FilesystemProvider, SourceAuthority, SourceKind, SourceProvider};

    #[test]
    fn includes_are_relative_and_exclusions_win_for_both_languages() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join(PROJECT_CONFIG_FILE),
            "include = ['model/**']\nexclude = ['**/draft.*']",
        )
        .unwrap();
        let scope = ProjectSourceScope::load(temp.path()).unwrap();
        assert!(scope.includes(&temp.path().join("model/Vehicle.sysml")));
        assert!(scope.includes(&temp.path().join("model/Types.kerml")));
        assert!(!scope.includes(&temp.path().join("model/draft.sysml")));
        assert!(!scope.includes(&temp.path().join("scratch/Vehicle.sysml")));
        assert!(!scope.includes(&temp.path().join("../outside.sysml")));
    }

    #[test]
    fn empty_include_means_no_automatic_sources() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), "include = []").unwrap();
        assert!(!ProjectSourceScope::load(temp.path())
            .unwrap()
            .includes(&temp.path().join("Model.sysml")));
    }

    #[test]
    fn default_scope_excludes_the_installed_environment() {
        let temp = tempfile::tempdir().unwrap();
        let scope = ProjectSourceScope::load(temp.path()).unwrap();
        assert!(scope.includes(&temp.path().join("Model.sysml")));
        assert!(!scope.includes(&temp.path().join(".sysand/lib/Model.sysml")));
    }

    #[test]
    fn configuration_is_reloaded_without_stale_membership() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("Model.sysml");
        assert!(ProjectSourceScope::load(temp.path())
            .unwrap()
            .includes(&path));
        std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), "exclude = ['**']").unwrap();
        assert!(!ProjectSourceScope::load(temp.path())
            .unwrap()
            .includes(&path));
        std::fs::remove_file(temp.path().join(PROJECT_CONFIG_FILE)).unwrap();
        assert!(ProjectSourceScope::load(temp.path())
            .unwrap()
            .includes(&path));
    }

    #[test]
    fn invalid_configuration_and_globs_are_explicit_errors() {
        let temp = tempfile::tempdir().unwrap();
        for text in ["include = 4", "exclude = ['[']", "include = ["] {
            std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), text).unwrap();
            assert!(ProjectSourceScope::load(temp.path()).is_err());
        }
    }

    #[test]
    fn filesystem_provider_uses_scope_without_flattening_nested_projects() {
        let temp = tempfile::tempdir().unwrap();
        for directory in ["model", "scratch", "child"] {
            std::fs::create_dir(temp.path().join(directory)).unwrap();
            std::fs::write(
                temp.path().join(directory).join("Model.sysml"),
                "package P;",
            )
            .unwrap();
        }
        std::fs::write(temp.path().join("child/.project.json"), "{}").unwrap();
        std::fs::write(
            temp.path().join(PROJECT_CONFIG_FILE),
            "exclude = ['scratch/**']",
        )
        .unwrap();
        let report =
            FilesystemProvider::new(vec![temp.path().to_path_buf()], SourceKind::Workspace)
                .within_project_boundary(true)
                .load(&SourceAuthority::new())
                .unwrap();
        assert_eq!(report.documents.len(), 1);
        assert_eq!(
            report.documents[0].uri().to_file_path().unwrap(),
            normalized_path(&temp.path().join("model/Model.sysml"))
        );
    }

    #[test]
    fn batch_file_targets_find_the_owning_project_without_walking_past_git() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(temp.path().join("project/model")).unwrap();
        std::fs::create_dir(temp.path().join("project/.git")).unwrap();
        let source = temp.path().join("project/model/Model.sysml");
        std::fs::write(&source, "package Model;").unwrap();
        std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), "include = []").unwrap();
        assert!(crate::discover_enclosing_project_root(&source).is_none());
        std::fs::write(temp.path().join("project/.project.json"), "{}").unwrap();
        assert_eq!(
            crate::discover_enclosing_project_root(&source),
            Some(std::fs::canonicalize(temp.path().join("project")).unwrap())
        );
    }

    #[test]
    fn installed_projects_are_not_discovered_as_editor_workspace_projects() {
        let temp = tempfile::tempdir().unwrap();
        for directory in ["", "child", ".sysand/lib/default", "child/installed/types"] {
            let root = temp.path().join(directory);
            std::fs::create_dir_all(&root).unwrap();
            std::fs::write(root.join(".project.json"), "{}").unwrap();
        }
        std::fs::write(
            temp.path().join("child").join(PROJECT_CONFIG_FILE),
            "sysand-dir = 'installed'",
        )
        .unwrap();
        assert_eq!(
            crate::discover_project_roots(&[temp.path().to_path_buf()]),
            vec![
                std::fs::canonicalize(temp.path()).unwrap(),
                std::fs::canonicalize(temp.path().join("child")).unwrap(),
            ]
        );
    }

    #[test]
    fn explicit_discovery_keeps_library_collections_while_project_admission_is_scoped() {
        let temp = tempfile::tempdir().unwrap();
        for directory in ["a", "b"] {
            std::fs::create_dir(temp.path().join(directory)).unwrap();
            std::fs::write(temp.path().join(directory).join(".project.json"), "{}").unwrap();
            std::fs::write(
                temp.path().join(directory).join("Model.sysml"),
                "package Model;",
            )
            .unwrap();
        }
        std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), "include = []").unwrap();
        let source = SourceAuthority::new();
        assert_eq!(
            source.discover(&[temp.path().to_path_buf()]).unwrap().len(),
            2
        );
        let report =
            FilesystemProvider::new(vec![temp.path().to_path_buf()], SourceKind::Workspace)
                .within_project_boundary(true)
                .load(&source)
                .unwrap();
        assert!(report.documents.is_empty());
    }

    #[test]
    fn opening_a_manifest_inside_an_installed_environment_does_not_create_workspace_membership() {
        let temp = tempfile::tempdir().unwrap();
        for directory in ["", "child", "installed/types"] {
            std::fs::create_dir_all(temp.path().join(directory)).unwrap();
            std::fs::write(temp.path().join(directory).join(".project.json"), "{}").unwrap();
            std::fs::write(
                temp.path().join(directory).join("Model.sysml"),
                "package Model;",
            )
            .unwrap();
        }
        std::fs::write(
            temp.path().join(PROJECT_CONFIG_FILE),
            "sysand-dir = 'installed'",
        )
        .unwrap();
        assert!(
            workspace_source_selected(&temp.path().join("child/Model.sysml"), temp.path()).unwrap()
        );
        assert!(!workspace_source_selected(
            &temp.path().join("installed/types/Model.sysml"),
            temp.path()
        )
        .unwrap());
    }

    #[test]
    fn escaping_include_patterns_are_configuration_errors() {
        let temp = tempfile::tempdir().unwrap();
        for text in ["include = ['../library']", "exclude = ['C:/secret/**']"] {
            std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), text).unwrap();
            let error = ProjectSourceScope::load(temp.path()).unwrap_err();
            assert!(
                error.to_string().contains("library_paths"),
                "{text}: {error}"
            );
        }
    }

    #[test]
    fn syside_toml_is_not_a_project_config() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("Model.sysml");
        std::fs::write(&source, "package Model;").unwrap();
        std::fs::write(temp.path().join("syside.toml"), "include = []").unwrap();
        assert!(crate::discover_enclosing_project_root(&source).is_none());
        assert!(ProjectSourceScope::load(temp.path())
            .unwrap()
            .includes(&source));
        std::fs::write(temp.path().join(PROJECT_CONFIG_FILE), "include = []").unwrap();
        assert_eq!(
            crate::discover_enclosing_project_root(&source),
            Some(std::fs::canonicalize(temp.path()).unwrap())
        );
        assert!(!ProjectSourceScope::load(temp.path())
            .unwrap()
            .includes(&source));
    }

    #[test]
    fn library_paths_resolve_relative_to_the_project_root() {
        let temp = tempfile::tempdir().unwrap();
        let library = temp.path().join("sibling");
        std::fs::create_dir(&library).unwrap();
        std::fs::write(
            temp.path().join(PROJECT_CONFIG_FILE),
            "library_paths = ['sibling', 'sibling']",
        )
        .unwrap();
        let scope = ProjectSourceScope::load(temp.path()).unwrap();
        assert_eq!(scope.library_paths(), &[normalized_path(&library)]);
    }

    #[test]
    fn a_missing_library_path_is_an_explicit_error() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join(PROJECT_CONFIG_FILE),
            "library_paths = ['missing']",
        )
        .unwrap();
        let error = ProjectSourceScope::load(temp.path()).unwrap_err();
        assert!(error.to_string().contains("not a directory"), "{error}");
    }
}
