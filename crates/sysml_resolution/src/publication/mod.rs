//! The publication authority: canonical construction of an immutable publication from admitted
//! documents, library-stratum reuse, and the lifecycle a long-lived host drives it through.
//!
//! Hosts hand over [`SourceDocument`]s. This owner alone partitions their provenance, decides
//! whether a settled library stratum can be reused, builds the request, and constructs the
//! publication. Every admitted document is parsed through the syntax authority's memo, so the
//! editor's parse and the build's parse are one tree. Library construction failure is explicit:
//! it is never disguised by silently selecting a different construction path.

mod session;

use std::sync::{Arc, Mutex};

use sysml_source::{SourceDocument, SourceKind};

use crate::lower::memo::LoweringMemo;
use crate::syntax::SyntaxAuthority;
use crate::{
    build, build_library_stratum_memoized, BuildRequest, ConstructionSchedule, LibraryStratum,
    PublicationIdentity, PublishedResolution, SourceInput,
};

pub use session::{
    BuildToken, PublicationOutcome, PublicationToken, Published, RelinkToken, Session,
    SessionLifecycle,
};

/// The semantic phase which rejected a publication request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationFailureStage {
    SourceAdmission,
    LibraryConstruction,
    RequestConstruction,
    ModelConstruction,
}

/// A typed failure from the sole publication construction path.
#[derive(Debug, Clone)]
pub struct PublicationBuildFailure {
    stage: PublicationFailureStage,
    message: String,
}

impl PublicationBuildFailure {
    pub fn stage(&self) -> PublicationFailureStage {
        self.stage
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn at(stage: PublicationFailureStage, error: impl std::fmt::Display) -> Self {
        Self {
            stage,
            message: error.to_string(),
        }
    }
}

impl std::fmt::Display for PublicationBuildFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}: {}", self.stage, self.message)
    }
}

impl std::error::Error for PublicationBuildFailure {}

/// An immutable, dependency-complete request prepared by [`PublicationAuthority::prepare`].
///
/// Preparing never parses: the identity is computed from document digests, and the documents are
/// parsed (a memo hit, or one parse) when the request is built. A host therefore captures the
/// identity on its executor and does the work on a blocking thread.
#[derive(Debug)]
pub struct PreparedPublication {
    request: BuildRequest,
}

impl PreparedPublication {
    pub fn identity(&self) -> &PublicationIdentity {
        self.request.identity()
    }

    pub fn request(&self) -> &BuildRequest {
        &self.request
    }

    pub fn build(self) -> Result<PublishedResolution, PublicationBuildFailure> {
        build(self.request).map_err(|error| {
            PublicationBuildFailure::at(PublicationFailureStage::ModelConstruction, error)
        })
    }
}

#[derive(Debug)]
struct CachedLibraryStratum {
    key: blake3::Hash,
    stratum: Arc<LibraryStratum>,
}

/// One build/cache authority shared by every publication in a host process.
#[derive(Debug)]
pub struct PublicationAuthority {
    syntax: Arc<SyntaxAuthority>,
    library: Mutex<Option<CachedLibraryStratum>>,
    /// Phase 2's memo: one lowering product per admitted document, keyed by content digest at
    /// every provenance rather than only at the library boundary. Owned here so it is reached
    /// only through this handle, and so its lifetime is the host session's.
    lowering: Arc<LoweringMemo>,
}

impl PublicationAuthority {
    pub fn new(syntax: Arc<SyntaxAuthority>) -> Self {
        Self {
            syntax,
            library: Mutex::new(None),
            lowering: Arc::new(LoweringMemo::new()),
        }
    }

    pub fn syntax(&self) -> &Arc<SyntaxAuthority> {
        &self.syntax
    }

    /// Publishes the exact admitted source set, reusing a solved library only when its complete
    /// canonical identity is unchanged.
    pub fn publish(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PublishedResolution, PublicationBuildFailure> {
        self.prepare(documents, reported_documents)
            .and_then(PreparedPublication::build)
    }

    /// Prepares the canonical request without parsing or building it, so an atomic publication
    /// owner can capture its dependency-complete identity before background construction starts.
    pub fn prepare(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        let mut ordered = documents.iter().collect::<Vec<_>>();
        ordered.sort_unstable_by(|left, right| left.uri().as_str().cmp(right.uri().as_str()));

        let mut workspace = Vec::new();
        let mut libraries = Vec::new();
        for document in ordered {
            if document.uri().as_str().is_empty() {
                return Err(PublicationBuildFailure::at(
                    PublicationFailureStage::SourceAdmission,
                    "source identity must not be empty",
                ));
            }
            let source = SourceInput::pending(document.uri().as_str(), document.clone());
            if document.kind().is_library() {
                libraries.push((document, source));
            } else {
                workspace.push(source);
            }
        }

        let reported = reported_documents.into_iter().collect::<Vec<_>>();
        let request = if libraries.is_empty() {
            BuildRequest::new(
                workspace,
                ConstructionSchedule::Parallel,
                crate::RESOLVED_CONTRACT,
            )
        } else {
            let stratum = self.library_stratum(&libraries)?;
            BuildRequest::with_library(
                workspace,
                ConstructionSchedule::Parallel,
                crate::RESOLVED_CONTRACT,
                stratum,
            )
        }
        .map_err(|error| {
            PublicationBuildFailure::at(PublicationFailureStage::RequestConstruction, error)
        })?
        .reporting(reported)
        .with_syntax(Arc::clone(&self.syntax))
        .with_lowering(Arc::clone(&self.lowering));
        Ok(PreparedPublication { request })
    }

    fn library_stratum(
        &self,
        libraries: &[(&SourceDocument, SourceInput)],
    ) -> Result<Arc<LibraryStratum>, PublicationBuildFailure> {
        let key = library_key(libraries.iter().map(|(document, _)| *document));
        let mut cached = self
            .library
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(entry) = cached.as_ref() {
            if entry.key == key {
                return Ok(Arc::clone(&entry.stratum));
            }
        }
        let stratum = Arc::new(
            build_library_stratum_memoized(
                libraries.iter().map(|(_, source)| source.clone()).collect(),
                Some(Arc::clone(&self.syntax)),
                Some(Arc::clone(&self.lowering)),
            )
            .map_err(|error| {
                PublicationBuildFailure::at(PublicationFailureStage::LibraryConstruction, error)
            })?,
        );
        *cached = Some(CachedLibraryStratum {
            key,
            stratum: Arc::clone(&stratum),
        });
        Ok(stratum)
    }
}

/// The stratum identity: every library document's URI, provenance, and content digest, in URI
/// order. Digests rather than bytes, so a warm publication hashes a few kilobytes, not the corpus.
fn library_key<'a>(documents: impl IntoIterator<Item = &'a SourceDocument>) -> blake3::Hash {
    let mut digest = blake3::Hasher::new();
    digest.update(b"spec42-library-stratum-v2\0");
    for document in documents {
        let identity = document.uri().as_str().as_bytes();
        digest.update(&(identity.len() as u64).to_le_bytes());
        digest.update(identity);
        digest.update(&[match document.kind() {
            SourceKind::Workspace => 0,
            SourceKind::StandardLibrary => 1,
            SourceKind::Library => 2,
            SourceKind::External => 3,
        }]);
        digest.update(document.digest().as_bytes());
    }
    digest.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_source::SourceAuthority;

    fn document(uri: &str, content: &str, kind: SourceKind) -> SourceDocument {
        SourceAuthority::new().admit(uri, content, kind).unwrap()
    }

    fn authority() -> PublicationAuthority {
        PublicationAuthority::new(Arc::new(SyntaxAuthority::new()))
    }

    fn cached_key(authority: &PublicationAuthority) -> blake3::Hash {
        authority
            .library
            .lock()
            .unwrap()
            .as_ref()
            .expect("cached library")
            .key
    }

    #[test]
    fn library_identity_covers_content_and_provenance_but_not_workspace_edits() {
        let authority = authority();
        let library = document(
            "memory://library/lib.sysml",
            "standard library package Lib { part def Wheel; }",
            SourceKind::StandardLibrary,
        );
        let workspace = document(
            "memory://workspace/model.sysml",
            "package W { part w : Lib::Wheel; }",
            SourceKind::Workspace,
        );
        authority
            .publish(&[library.clone(), workspace.clone()], [])
            .unwrap();
        let initial = cached_key(&authority);

        let edited_workspace = document(
            "memory://workspace/model.sysml",
            "package W { part w : Lib::Wheel; part x : Lib::Wheel; }",
            SourceKind::Workspace,
        );
        authority
            .publish(&[library.clone(), edited_workspace], [])
            .unwrap();
        assert_eq!(cached_key(&authority), initial);

        let changed_library = document(
            "memory://library/lib.sysml",
            "standard library package Lib { part def Wheel; part def Axle; }",
            SourceKind::StandardLibrary,
        );
        authority.publish(&[changed_library], []).unwrap();
        assert_ne!(cached_key(&authority), initial);

        let changed_role = library.with_kind(SourceKind::Library);
        authority.publish(&[changed_role], []).unwrap();
        assert_ne!(cached_key(&authority), initial);
    }

    #[test]
    fn prepare_does_not_parse_and_build_parses_through_the_memo() {
        let authority = authority();
        let workspace = document(
            "memory://workspace/model.sysml",
            "package W { part def P; }",
            SourceKind::Workspace,
        );
        let prepared = authority
            .prepare(std::slice::from_ref(&workspace), [])
            .unwrap();
        assert_eq!(authority.syntax().memo_len(), 0, "prepare never parses");
        prepared.build().unwrap();
        assert_eq!(
            authority.syntax().memo_len(),
            1,
            "the build parsed through the memo"
        );
        let parsed = authority.syntax().parse(&workspace);
        authority.publish(&[workspace], []).unwrap();
        assert_eq!(authority.syntax().memo_len(), 1);
        drop(parsed);
    }

    #[test]
    fn library_construction_failure_is_explicit_and_never_flattened() {
        let authority = authority();
        let duplicate = document(
            "memory://library/duplicate.sysml",
            "standard library package Lib;",
            SourceKind::StandardLibrary,
        );
        let error = authority
            .prepare(&[duplicate.clone(), duplicate], [])
            .expect_err("duplicate library identities must fail stratum construction");

        assert_eq!(error.stage(), PublicationFailureStage::LibraryConstruction);
        assert!(authority.library.lock().unwrap().is_none());
    }
}
