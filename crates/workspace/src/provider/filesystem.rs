//! The batch host's source provider: the workspace tree plus either the whole library roots or
//! the import closure the workspace needs.
//!
//! Walking and reading are the source authority's; this type only decides *which* roots are in
//! play and what provenance each one carries.

use std::path::{Path, PathBuf};

use sysml_query::source::{
    path_to_file_url, FilesystemProvider, SourceAuthority, SourceDocument, SourceError, SourceKind,
    SourceLoadReport, SourceProvider,
};

use crate::library::{resolve_library_closure, LibraryClosureOptions, WorkspaceSource};

#[derive(Debug, Clone)]
pub struct FileSystemDocumentProvider {
    target: PathBuf,
    workspace_root: Option<PathBuf>,
    library_paths: Vec<PathBuf>,
    standard_library_paths: Vec<PathBuf>,
    full_library_scan: bool,
    library_seed_packages: Vec<String>,
}

pub type HostFilesystemProvider = FileSystemDocumentProvider;

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
            standard_library_paths: Vec::new(),
            full_library_scan: false,
            library_seed_packages: Vec::new(),
        }
    }

    pub fn from_paths(
        target: &Path,
        workspace_root: Option<&Path>,
        library_paths: &[PathBuf],
    ) -> Self {
        Self::new(
            target.to_path_buf(),
            workspace_root.map(Path::to_path_buf),
            library_paths.to_vec(),
        )
    }

    pub fn from_paths_with_standard_library(
        target: &Path,
        workspace_root: Option<&Path>,
        library_paths: &[PathBuf],
        standard_library_paths: &[PathBuf],
    ) -> Self {
        Self::from_paths(target, workspace_root, library_paths)
            .with_standard_library_paths(standard_library_paths.to_vec())
    }

    /// When enabled, every file under each library root is loaded wholesale
    /// instead of only the files reachable from the workspace's import closure.
    pub fn with_full_library_scan(mut self, enabled: bool) -> Self {
        self.full_library_scan = enabled;
        self
    }

    /// Adds package names that seed the otherwise reference-scoped library closure.
    pub fn with_library_seed_packages(mut self, packages: Vec<String>) -> Self {
        self.library_seed_packages = packages;
        self
    }

    /// Marks which configured library roots are the canonical SysML standard library. Other
    /// library roots remain dependencies and cannot satisfy universal implied relationships.
    pub fn with_standard_library_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.standard_library_paths = paths;
        self
    }
}

impl SourceProvider for FileSystemDocumentProvider {
    fn load(&self, authority: &SourceAuthority) -> Result<SourceLoadReport, SourceError> {
        let workspace_root = resolve_workspace_root(&self.target, self.workspace_root.as_deref());
        let workspace_root = canonicalize_or_self(&workspace_root);
        let standard_library_paths = self
            .standard_library_paths
            .iter()
            .map(|path| canonicalize_or_self(path))
            .collect::<Vec<_>>();

        let mut report = SourceLoadReport::default();
        if workspace_root.exists() {
            let workspace =
                FilesystemProvider::new(vec![workspace_root.clone()], SourceKind::Workspace)
                    .load(authority)?;
            merge(&mut report, workspace);
        }

        if self.full_library_scan {
            for library_path in &self.library_paths {
                let library_root = canonicalize_or_self(library_path);
                if !library_root.exists() {
                    continue;
                }
                let kind = library_source_kind(&library_root, &standard_library_paths);
                merge(&mut report, authority.list(&[library_root], kind)?);
            }
            return Ok(report);
        }

        let library_roots: Vec<String> = self
            .library_paths
            .iter()
            .map(|path| {
                canonicalize_or_self(path)
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        let workspace_documents = report
            .documents
            .iter()
            .filter(|document| document.kind() == SourceKind::Workspace)
            .map(|document| {
                (
                    document.path_hint().unwrap_or("").to_owned(),
                    document.content().to_owned(),
                )
            })
            .collect::<Vec<_>>();
        if library_roots.is_empty() || workspace_documents.is_empty() {
            return Ok(report);
        }
        let workspace_sources: Vec<WorkspaceSource<'_>> = workspace_documents
            .iter()
            .map(|(path, content)| WorkspaceSource {
                path: path.as_str(),
                content: content.as_str(),
            })
            .collect();
        let options = LibraryClosureOptions {
            seed_packages: self.library_seed_packages.clone(),
            ..LibraryClosureOptions::default()
        };
        let loaded = resolve_library_closure(&workspace_sources, &library_roots, &options)
            .map_err(SourceError::Provider)?;
        for file in loaded {
            let path = PathBuf::from(&file.root).join(&file.path);
            let kind = library_source_kind(
                &canonicalize_or_self(&PathBuf::from(&file.root)),
                &standard_library_paths,
            );
            let uri = path_to_file_url(&path)?;
            let document: SourceDocument = authority
                .admit_url(uri, &file.content, kind)
                .with_path_hint(file.path.replace('\\', "/"));
            report.documents.push(document);
        }
        Ok(report)
    }
}

fn merge(into: &mut SourceLoadReport, from: SourceLoadReport) {
    into.documents.extend(from.documents);
    into.skipped.extend(from.skipped);
    into.roots_scanned += from.roots_scanned;
    into.roots_skipped += from.roots_skipped;
    into.candidate_files += from.candidate_files;
}

fn library_source_kind(root: &Path, standard_library_paths: &[PathBuf]) -> SourceKind {
    if standard_library_paths.iter().any(|path| path == root) {
        SourceKind::StandardLibrary
    } else {
        SourceKind::Library
    }
}

fn canonicalize_or_self(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
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
