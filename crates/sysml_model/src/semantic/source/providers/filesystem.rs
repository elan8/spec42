use std::fs;
use std::path::{Path, PathBuf};

use url::Url;
use walkdir::WalkDir;

use crate::semantic::library_loader::{
    resolve_library_closure, LibraryClosureOptions, WorkspaceSource,
};
use crate::semantic::source::{SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind};

#[derive(Debug, Clone)]
pub struct FileSystemDocumentProvider {
    target: PathBuf,
    workspace_root: Option<PathBuf>,
    library_paths: Vec<PathBuf>,
    full_library_scan: bool,
}

impl FileSystemDocumentProvider {
    pub fn new(
        target: PathBuf,
        workspace_root: Option<PathBuf>,
        library_paths: Vec<PathBuf>,
    ) -> Self {
        Self {
            target,
            workspace_root,
            library_paths,
            full_library_scan: false,
        }
    }

    /// When enabled, every file under each library root is loaded wholesale
    /// instead of only the files reachable from the workspace's import closure.
    pub fn with_full_library_scan(mut self, enabled: bool) -> Self {
        self.full_library_scan = enabled;
        self
    }
}

impl SysmlDocumentProvider for FileSystemDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        let workspace_root = resolve_workspace_root(&self.target, self.workspace_root.as_deref());
        let workspace_root = canonicalize_or_self(&workspace_root);

        let mut documents = Vec::new();
        let mut workspace_file_contents = Vec::new();
        let mut workspace_path_hints = Vec::new();

        if workspace_root.exists() {
            for path in collect_sysml_files(&workspace_root)? {
                let content = fs::read_to_string(&path)
                    .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
                let path_hint = path
                    .strip_prefix(&workspace_root)
                    .ok()
                    .map(|relative| relative.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|| path.display().to_string());
                workspace_path_hints.push(path_hint);
                workspace_file_contents.push(content);
            }
        }

        for (path_hint, content) in workspace_path_hints
            .iter()
            .zip(workspace_file_contents.iter())
        {
            let path = workspace_root.join(path_hint);
            let uri = path_to_url(&path)?;
            documents.push(SysmlDocument {
                uri,
                content: content.clone(),
                path_hint: Some(path_hint.clone()),
                source_kind: SysmlDocumentSourceKind::Workspace,
                sha256: None,
                byte_size: None,
            });
        }

        if self.full_library_scan {
            for library_path in &self.library_paths {
                let library_root = canonicalize_or_self(library_path);
                if !library_root.exists() {
                    continue;
                }
                for path in collect_sysml_files(&library_root)? {
                    let content = fs::read_to_string(&path)
                        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
                    let path_hint = path
                        .strip_prefix(&library_root)
                        .ok()
                        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|| path.display().to_string());
                    let uri = path_to_url(&path)?;
                    documents.push(SysmlDocument {
                        uri,
                        content,
                        path_hint: Some(path_hint),
                        source_kind: SysmlDocumentSourceKind::Library,
                        sha256: None,
                        byte_size: None,
                    });
                }
            }
        } else {
            let library_roots: Vec<String> = self
                .library_paths
                .iter()
                .map(|path| {
                    canonicalize_or_self(path)
                        .to_string_lossy()
                        .replace('\\', "/")
                })
                .collect();
            if !library_roots.is_empty() && !workspace_file_contents.is_empty() {
                let workspace_sources: Vec<WorkspaceSource<'_>> = workspace_path_hints
                    .iter()
                    .zip(workspace_file_contents.iter())
                    .map(|(path_hint, content)| WorkspaceSource {
                        path: path_hint.as_str(),
                        content: content.as_str(),
                    })
                    .collect();
                let loaded = resolve_library_closure(
                    &workspace_sources,
                    &library_roots,
                    &LibraryClosureOptions::default(),
                )?;
                for file in loaded {
                    let path = PathBuf::from(&file.root).join(&file.path);
                    let uri = path_to_url(&path)?;
                    documents.push(SysmlDocument {
                        uri,
                        content: file.content,
                        path_hint: Some(file.path.replace('\\', "/")),
                        source_kind: SysmlDocumentSourceKind::Library,
                        sha256: None,
                        byte_size: None,
                    });
                }
            }
        }

        Ok(documents)
    }
}

fn collect_sysml_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !entry.file_type().is_file() {
            continue;
        }
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                ext.eq_ignore_ascii_case("sysml") || ext.eq_ignore_ascii_case("kerml")
            })
        {
            paths.push(path.to_path_buf());
        }
    }
    paths.sort();
    Ok(paths)
}

fn canonicalize_or_self(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn path_to_url(path: &Path) -> Result<Url, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|err| format!("failed to resolve current directory: {err}"))?
            .join(path)
    };
    let canonical = canonicalize_or_self(&absolute);
    let url = Url::from_file_path(&canonical).map_err(|_| {
        format!(
            "failed to convert path to file URI: {}",
            canonical.display()
        )
    })?;
    Ok(normalize_file_url_drive_letter(url))
}

/// Lowercases the Windows drive letter in a `file://` URL so all paths use a
/// consistent form (`file:///c:/...` not `file:///C:/...`). This matches the
/// normalisation applied by the kernel/LSP layer and ensures graph node URIs
/// are comparable to the target URLs used in workspace lookups.
fn normalize_file_url_drive_letter(url: Url) -> Url {
    if url.scheme() != "file" {
        return url;
    }
    let path = url.path();
    // Windows path: /C:/... — lowercase the drive letter at index 1.
    if path.len() >= 3 {
        let bytes = path.as_bytes();
        if bytes[0] == b'/' && bytes[1].is_ascii_uppercase() && bytes[2] == b':' {
            let new_path = format!("/{}{}", (bytes[1] as char).to_ascii_lowercase(), &path[2..]);
            if let Ok(normalized) = Url::parse(&format!("file://{new_path}")) {
                return normalized;
            }
        }
    }
    url
}

fn resolve_workspace_root(target: &Path, workspace_root: Option<&Path>) -> PathBuf {
    workspace_root.map(Path::to_path_buf).unwrap_or_else(|| {
        if target.is_dir() {
            target.to_path_buf()
        } else {
            target
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_semantic_graph_with_provider;

    #[test]
    fn provider_loads_libraries_only_through_import_closure() {
        let temp = tempfile::tempdir().expect("tempdir");
        let workspace = temp.path().join("workspace");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&workspace).expect("workspace dir");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            workspace.join("App.sysml"),
            r#"
package App {
    import Local::*;
    part appRoot;
}
package Local {
    private import ScalarValues::Real;
    part def LocalPart { attribute x : Real; }
}
"#,
        )
        .expect("workspace model");
        fs::write(
            lib.join("Unused.sysml"),
            "package Unused { part def NeverLoaded; }",
        )
        .expect("unused library");
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("scalar values");

        let provider = FileSystemDocumentProvider::new(
            workspace.clone(),
            Some(workspace.clone()),
            vec![lib.clone()],
        );
        let documents = provider.load_documents().expect("documents");
        let paths: Vec<_> = documents
            .iter()
            .filter_map(|doc| doc.path_hint.as_deref())
            .collect();
        assert!(paths.iter().any(|path| path.ends_with("App.sysml")));
        assert!(
            paths.iter().any(|path| path.contains("ScalarValues.sysml")),
            "ScalarValues should load via import closure, got {paths:?}"
        );
        assert!(
            !paths.iter().any(|path| path.contains("Unused.sysml")),
            "unreferenced library file must not load, got {paths:?}"
        );

        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        assert_eq!(
            graph
                .node_ids_for_qualified_name("Local::LocalPart")
                .map(|ids| ids.len())
                .unwrap_or(0),
            1
        );
        assert!(graph
            .node_ids_for_qualified_name("Unused::NeverLoaded")
            .is_none());
    }

    #[test]
    fn provider_with_full_library_scan_loads_unreferenced_library_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let workspace = temp.path().join("workspace");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&workspace).expect("workspace dir");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            workspace.join("App.sysml"),
            r#"
package App {
    import Local::*;
    part appRoot;
}
package Local {
    private import ScalarValues::Real;
    part def LocalPart { attribute x : Real; }
}
"#,
        )
        .expect("workspace model");
        fs::write(
            lib.join("Unused.sysml"),
            "package Unused { part def NeverLoaded; }",
        )
        .expect("unused library");
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("scalar values");

        let provider = FileSystemDocumentProvider::new(
            workspace.clone(),
            Some(workspace.clone()),
            vec![lib.clone()],
        )
        .with_full_library_scan(true);
        let documents = provider.load_documents().expect("documents");
        let paths: Vec<_> = documents
            .iter()
            .filter_map(|doc| doc.path_hint.as_deref())
            .collect();
        assert!(
            paths.iter().any(|path| path.contains("Unused.sysml")),
            "full library scan should load every library file, including unreferenced ones, got {paths:?}"
        );

        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        assert!(
            graph
                .node_ids_for_qualified_name("Unused::NeverLoaded")
                .is_some(),
            "Unused::NeverLoaded should reach the graph under full library scan"
        );
    }

    #[test]
    fn provider_loads_kerml_files_from_workspace_root() {
        let temp = tempfile::tempdir().expect("tempdir");
        let workspace = temp.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace dir");
        fs::write(workspace.join("App.sysml"), "package App { part appRoot; }")
            .expect("workspace model");
        fs::write(
            workspace.join("Core.kerml"),
            "package Core { classifier Thing; }",
        )
        .expect("kerml source");

        let provider =
            FileSystemDocumentProvider::new(workspace.clone(), Some(workspace.clone()), Vec::new());
        let documents = provider.load_documents().expect("documents");
        let paths: Vec<_> = documents
            .iter()
            .filter_map(|doc| doc.path_hint.as_deref())
            .collect();
        assert!(
            paths.iter().any(|path| path.ends_with("Core.kerml")),
            "expected Core.kerml to be discovered, got {paths:?}"
        );

        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        assert_eq!(
            graph
                .node_ids_for_qualified_name("Core::Thing")
                .map(|ids| ids.len())
                .unwrap_or(0),
            1,
            "Core::Thing from the .kerml source should reach the semantic graph"
        );
    }

    #[test]
    fn provider_does_not_load_library_package_declared_in_workspace() {
        let temp = tempfile::tempdir().expect("tempdir");
        let workspace = temp.path().join("workspace");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&workspace).expect("workspace dir");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            workspace.join("Views.sysml"),
            r#"
package Views {
    import Demo::*;
    view v { expose Demo::workspacePart; }
}
package Demo {
    part def workspacePart;
}
"#,
        )
        .expect("workspace views");
        fs::write(
            lib.join("Demo.sysml"),
            "package Demo { part def libraryPart; }",
        )
        .expect("library duplicate");

        let provider =
            FileSystemDocumentProvider::new(workspace.clone(), Some(workspace.clone()), vec![lib]);
        let (graph, _) = build_semantic_graph_with_provider(&provider).expect("graph");
        assert_eq!(
            graph
                .node_ids_for_qualified_name("Demo::workspacePart")
                .map(|ids| ids.len())
                .unwrap_or(0),
            1,
            "workspace Demo should satisfy import Demo::*"
        );
        assert!(
            graph
                .node_ids_for_qualified_name("Demo::libraryPart")
                .is_none(),
            "library Demo must not be merged when workspace declares Demo"
        );
    }
}
