//! The source service: admission of text as identified documents, providers, and the one URI and
//! line-ending policy.
//!
//! Consumers obtain a [`SourceService`] from their host's `Services` and never construct a
//! [`SourceDocument`] from parts; the authority decides identity and normalisation.

use std::path::{Path, PathBuf};
use std::sync::Arc;

pub use sysml_resolution::source::source_identity as identity;
pub use sysml_resolution::source::{
    is_sysml_like, normalize_line_endings, normalize_uri, path_to_file_url, uri_under_any,
    ContentDigest, FilesystemProvider, InMemoryProvider, RootDigest, SkippedSource,
    SourceAuthority, SourceDocument, SourceError, SourceKind, SourceLoadReport, SourceProvider,
    Url,
};

/// Handle on the source authority. Cheap to clone; all clones share one authority.
///
/// Providers receive the underlying [`SourceAuthority`] so they can admit what they find; that
/// is the only reason the authority type is visible here.
#[derive(Debug, Clone, Default)]
pub struct SourceService {
    inner: Arc<SourceAuthority>,
}

impl SourceService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn admit(
        &self,
        uri: &str,
        content: impl AsRef<str>,
        kind: SourceKind,
    ) -> Result<SourceDocument, SourceError> {
        self.inner.admit(uri, content, kind)
    }

    pub fn admit_url(&self, uri: Url, content: &str, kind: SourceKind) -> SourceDocument {
        self.inner.admit_url(uri, content, kind)
    }

    pub fn admit_memory(
        &self,
        scope: &str,
        path: &str,
        content: impl AsRef<str>,
        kind: SourceKind,
    ) -> Result<SourceDocument, SourceError> {
        self.inner.admit_memory(scope, path, content, kind)
    }

    pub fn admit_path(&self, path: &Path, kind: SourceKind) -> Result<SourceDocument, SourceError> {
        self.inner.admit_path(path, kind)
    }

    pub fn load(&self, provider: &dyn SourceProvider) -> Result<SourceLoadReport, SourceError> {
        self.inner.load(provider)
    }

    pub fn list(
        &self,
        roots: &[PathBuf],
        kind: SourceKind,
    ) -> Result<SourceLoadReport, SourceError> {
        self.inner.list(roots, kind)
    }

    pub fn discover(&self, targets: &[PathBuf]) -> Result<Vec<PathBuf>, SourceError> {
        self.inner.discover(targets)
    }
}
