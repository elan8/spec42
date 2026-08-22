//! The library-closure service: which files under the configured library roots a workspace needs.
//!
//! Hosts decide where libraries are (the roots and their provenance); this service decides which
//! of their files the workspace's imports, typing references and unit literals reach. Its package
//! index is memoised by the roots' listing and every file is parsed through the syntax memo, so a
//! later publication admitting the same documents parses nothing again.

use std::sync::Arc;

use sysml_resolution::library::LibraryClosureAuthority;

use crate::source::{SourceError, SourceService};
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

    /// A stable signature of the workspace facts that seed closure resolution.
    pub fn seed_signature(
        &self,
        workspace: &[ParsedSource],
        options: &LibraryClosureOptions,
    ) -> Vec<String> {
        self.inner.seed_signature(workspace, options)
    }
}
