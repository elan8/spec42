//! Filesystem-backed workspace provider wired to a resolved engine catalog.

use std::path::{Path, PathBuf};

use sysml_model::{FileSystemDocumentProvider, SysmlDocument, SysmlDocumentProvider};

/// Filesystem workspace provider using an engine's resolved library package roots.
#[derive(Debug, Clone)]
pub struct HostFilesystemProvider {
    inner: FileSystemDocumentProvider,
}

impl HostFilesystemProvider {
    pub fn new(
        target: impl Into<PathBuf>,
        workspace_root: Option<PathBuf>,
        library_paths: &[PathBuf],
    ) -> Self {
        const IMPLIED_SEMANTIC_PACKAGES: &[&str] = &[
            "Base",
            "Occurrences",
            "Items",
            "Parts",
            "Ports",
            "Connections",
            "Interfaces",
            "Allocations",
            "Flows",
            "Actions",
            "States",
            "Calculations",
            "Constraints",
            "Requirements",
            "Cases",
            "AnalysisCases",
            "VerificationCases",
            "UseCases",
            "Views",
            "Metadata",
        ];
        Self {
            inner: FileSystemDocumentProvider::new(
                target.into(),
                workspace_root,
                library_paths.to_vec(),
            )
            .with_library_seed_packages(
                IMPLIED_SEMANTIC_PACKAGES
                    .iter()
                    .map(|package| (*package).to_owned())
                    .collect(),
            ),
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
            library_paths,
        )
    }
}

impl SysmlDocumentProvider for HostFilesystemProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        self.inner.load_documents()
    }
}
