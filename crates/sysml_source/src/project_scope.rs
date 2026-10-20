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

pub const PROJECT_CONFIG_FILE: &str = "syside.toml";

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
}

/// Paths are relative to the project configuration directory. Other Syside settings are ignored;
/// this is a source-selection contract, not an implementation of the Syside tool configuration.
#[derive(Debug, Clone)]
pub struct ProjectSourceScope {
    root: PathBuf,
    include: Option<GlobSet>,
    exclude: GlobSet,
    sysand_dir: PathBuf,
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
            root,
        })
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
        std::fs::write(temp.path().join("syside.toml"), "include = []").unwrap();
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
            temp.path().join("child/syside.toml"),
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
}
