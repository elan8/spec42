//! The batch host's source provider: the workspace tree plus either the whole library roots or
//! the import closure the workspace needs.
//!
//! Walking and reading are the source authority's; this type only decides *which* roots are in
//! play and what provenance each one carries.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use sysml_query::library::{LibraryClosureOptions, LibraryRoot};
use sysml_query::source::{
    FilesystemProvider, SourceAuthority, SourceError, SourceKind, SourceLoadReport, SourceProvider,
};
use sysml_query::Services;

#[derive(Debug, Clone)]
pub struct FileSystemDocumentProvider {
    target: PathBuf,
    workspace_root: Option<PathBuf>,
    library_paths: Vec<PathBuf>,
    standard_library_paths: Vec<PathBuf>,
    full_library_scan: bool,
    library_seed_packages: Vec<String>,
    services: Services,
}

pub type HostFilesystemProvider = FileSystemDocumentProvider;

impl FileSystemDocumentProvider {
    /// `services` are the host's: the closure is resolved through them so the library documents
    /// this provider yields are memo hits for the publication that admits them.
    pub fn new(
        target: PathBuf,
        workspace_root: Option<PathBuf>,
        library_paths: Vec<PathBuf>,
        services: Services,
    ) -> Self {
        Self {
            target,
            workspace_root,
            library_paths,
            standard_library_paths: Vec::new(),
            full_library_scan: false,
            library_seed_packages: Vec::new(),
            services,
        }
    }

    pub fn from_paths(
        target: &Path,
        workspace_root: Option<&Path>,
        library_paths: &[PathBuf],
        services: Services,
    ) -> Self {
        Self::new(
            target.to_path_buf(),
            workspace_root.map(Path::to_path_buf),
            library_paths.to_vec(),
            services,
        )
    }

    pub fn from_paths_with_standard_library(
        target: &Path,
        workspace_root: Option<&Path>,
        library_paths: &[PathBuf],
        standard_library_paths: &[PathBuf],
        services: Services,
    ) -> Self {
        Self::from_paths(target, workspace_root, library_paths, services)
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
                    .within_project_boundary(true)
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
            coalesce_source_identities(&mut report)?;
            return Ok(report);
        }

        let roots: Vec<LibraryRoot> = self
            .library_paths
            .iter()
            .map(|path| {
                let path = canonicalize_or_self(path);
                let kind = library_source_kind(&path, &standard_library_paths);
                LibraryRoot { path, kind }
            })
            .collect();
        let workspace: Vec<_> = report
            .documents
            .iter()
            .filter(|document| document.kind() == SourceKind::Workspace)
            .map(|document| self.services.syntax.parse(document))
            .collect();
        if roots.is_empty() || workspace.is_empty() {
            return Ok(report);
        }
        let options = LibraryClosureOptions {
            seed_packages: self.library_seed_packages.clone(),
            ..LibraryClosureOptions::default()
        };
        let closure = self
            .services
            .library
            .resolve(&workspace, &roots, &options)?;
        report.documents.extend(closure.documents);
        coalesce_source_identities(&mut report)?;
        Ok(report)
    }
}

/// An enclosing workspace root may contain a configured library root. Both scans then encounter
/// the same physical source, but the configured library root owns its provenance. Coalesce that
/// overlap before publication while rejecting a source that changed between the two reads.
fn coalesce_source_identities(report: &mut SourceLoadReport) -> Result<(), SourceError> {
    let mut positions = HashMap::with_capacity(report.documents.len());
    let mut documents = Vec::with_capacity(report.documents.len());

    for document in report.documents.drain(..) {
        let Some(&position) = positions.get(document.uri()) else {
            positions.insert(document.uri().clone(), documents.len());
            documents.push(document);
            continue;
        };
        let existing: &mut sysml_query::source::SourceDocument = &mut documents[position];
        if existing.digest() != document.digest() {
            return Err(SourceError::Provider(format!(
                "source identity {} changed while overlapping workspace and library roots were loaded",
                document.uri()
            )));
        }
        if source_kind_precedence(document.kind()) > source_kind_precedence(existing.kind()) {
            *existing = document;
        }
    }

    report.documents = documents;
    Ok(())
}

fn source_kind_precedence(kind: SourceKind) -> u8 {
    match kind {
        SourceKind::External => 0,
        SourceKind::Workspace => 1,
        SourceKind::Library => 2,
        SourceKind::StandardLibrary => 3,
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
    let fallback = workspace_root.map(Path::to_path_buf).unwrap_or_else(|| {
        if target.is_dir() {
            target.to_path_buf()
        } else {
            target
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
        }
    });
    sysml_query::source::discover_project_boundary(target, &fallback)
        .map(|boundary| boundary.project_root().to_path_buf())
        .unwrap_or(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_library_provenance_owns_an_overlapping_workspace_source() {
        let temp = tempfile::tempdir().expect("tempdir");
        let library = temp.path().join("stdlib");
        std::fs::create_dir(&library).expect("library directory");
        std::fs::write(library.join("Base.kerml"), "package Base;").expect("library source");

        let services = Services::new();
        let provider = FileSystemDocumentProvider::from_paths_with_standard_library(
            temp.path(),
            Some(temp.path()),
            std::slice::from_ref(&library),
            std::slice::from_ref(&library),
            services.clone(),
        )
        .with_full_library_scan(true);

        let report = services.source.load(&provider).expect("overlapping load");
        assert_eq!(report.documents.len(), 1);
        assert_eq!(report.documents[0].kind(), SourceKind::StandardLibrary);
    }
}
