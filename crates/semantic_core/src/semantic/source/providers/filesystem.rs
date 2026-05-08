use std::fs;
use std::path::{Path, PathBuf};

use tower_lsp::lsp_types::Url;
use walkdir::WalkDir;

use crate::semantic::source::{SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind};

#[derive(Debug, Clone)]
pub struct FileSystemDocumentProvider {
    target: PathBuf,
    workspace_root: Option<PathBuf>,
    library_paths: Vec<PathBuf>,
}

impl FileSystemDocumentProvider {
    pub fn new(target: PathBuf, workspace_root: Option<PathBuf>, library_paths: Vec<PathBuf>) -> Self {
        Self {
            target,
            workspace_root,
            library_paths,
        }
    }
}

impl SysmlDocumentProvider for FileSystemDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        let workspace_root = resolve_workspace_root(&self.target, self.workspace_root.as_deref());
        let workspace_root = canonicalize_or_self(&workspace_root);
        let library_roots = self
            .library_paths
            .iter()
            .map(|path| canonicalize_or_self(path))
            .collect::<Vec<_>>();
        let roots = std::iter::once((workspace_root.clone(), SysmlDocumentSourceKind::Workspace))
            .chain(
                library_roots
                    .iter()
                    .cloned()
                    .map(|root| (root, SysmlDocumentSourceKind::Library)),
            )
            .collect::<Vec<_>>();

        let mut documents = Vec::new();
        for (root, source_kind) in roots {
            if !root.exists() {
                continue;
            }
            for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
                let path = entry.path();
                if !entry.file_type().is_file() {
                    continue;
                }
                if !path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("sysml"))
                {
                    continue;
                }
                let Ok(content) = fs::read_to_string(path) else {
                    continue;
                };
                let uri = path_to_url(path)?;
                let path_hint = path
                    .strip_prefix(&root)
                    .ok()
                    .map(|relative| relative.to_string_lossy().replace('\\', "/"));
                documents.push(SysmlDocument {
                    uri,
                    content,
                    path_hint,
                    source_kind,
                    sha256: None,
                    byte_size: None,
                });
            }
        }
        Ok(documents)
    }
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
    Url::from_file_path(&canonical)
        .map_err(|_| format!("failed to convert path to file URI: {}", canonical.display()))
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
