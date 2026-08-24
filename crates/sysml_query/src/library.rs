//! The library-closure service: which files under the configured library roots a workspace needs.
//!
//! Hosts decide where libraries are (the roots and their provenance); this service decides which
//! of their files the workspace's imports, typing references and unit literals reach. Its package
//! index is memoised by the roots' listing and every file is parsed through the syntax memo, so a
//! later publication admitting the same documents parses nothing again.

use std::sync::Arc;

use sysml_resolution::library::LibraryClosureAuthority;

use crate::source::{SourceDocument, SourceError, SourceKind, SourceService, Url};
use crate::syntax::{ParsedSource, SyntaxService};

pub use sysml_resolution::library::{LibraryClosure, LibraryClosureOptions, LibraryRoot};

/// Handle on the library-closure authority. Cheap to clone; all clones share one index.
#[derive(Debug, Clone)]
pub struct LibraryClosureService {
    inner: Arc<LibraryClosureAuthority>,
}

impl LibraryClosureService {
    pub fn new(source: &SourceService, syntax: &SyntaxService) -> Self {
        Self {
            inner: Arc::new(LibraryClosureAuthority::new(
                Arc::clone(source.authority()),
                Arc::clone(syntax.authority()),
            )),
        }
    }

    /// The library documents `workspace` needs from `roots`, with the seed signature that
    /// produced them.
    pub fn resolve(
        &self,
        workspace: &[ParsedSource],
        roots: &[LibraryRoot],
        options: &LibraryClosureOptions,
    ) -> Result<LibraryClosure, SourceError> {
        self.inner.resolve(workspace, roots, options)
    }

    /// The library documents `workspace` needs from roots a host names by URI.
    ///
    /// Hosts configure library roots as document URIs and know which of them the standard library
    /// was installed to; turning that configuration into the authority's [`LibraryRoot`] values —
    /// including the provenance each root's files carry — is part of asking the question, not a
    /// derivation for each host to repeat. A root that is not a local file is skipped: the closure
    /// is resolved from files on disk, and a URI that names none contributes nothing.
    ///
    /// No configured root means no closure, which is an exact answer and not a failure.
    pub fn resolve_for_roots(
        &self,
        workspace: &[ParsedSource],
        library_roots: &[Url],
        standard_library_roots: &[Url],
        options: &LibraryClosureOptions,
    ) -> Result<Vec<SourceDocument>, SourceError> {
        let roots = library_roots
            .iter()
            .filter_map(|uri| {
                let path = uri.to_file_path().ok()?;
                let kind = if standard_library_roots.contains(uri) {
                    SourceKind::StandardLibrary
                } else {
                    SourceKind::Library
                };
                Some(LibraryRoot { path, kind })
            })
            .collect::<Vec<_>>();
        if roots.is_empty() {
            return Ok(Vec::new());
        }
        self.resolve(workspace, &roots, options)
            .map(|closure| closure.documents)
    }

    /// A stable signature of the workspace facts that seed closure resolution.
    pub fn seed_signature(
        &self,
        workspace: &[ParsedSource],
        options: &LibraryClosureOptions,
    ) -> Vec<String> {
        self.inner.seed_signature(workspace, options)
    }
}
