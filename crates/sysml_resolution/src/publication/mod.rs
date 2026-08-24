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

use source_identity::{LibrarySourceIdentity, LibraryStratumKey, SourceRole};
use sysml_source::{SourceDocument, SourceKind};

use crate::lower::memo::LoweringMemo;
use crate::syntax::SyntaxAuthority;
use crate::{
    build_library_stratum_memoized, BuildRequest, ConstructionSchedule, LibraryStratum,
    PublicationIdentity, PublishedResolution, SourceInput,
};

pub use session::{
    BuildToken, PublicationOutcome, PublicationToken, Published, Session, SessionLifecycle,
};

/// The semantic phase which rejected a publication request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationFailureStage {
    SourceAdmission,
    LibraryConstruction,
    RequestConstruction,
    ModelConstruction,
    ConstructionWorker,
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

    /// Construction worker failed before it could return its typed semantic result.
    pub fn construction_worker(error: impl std::fmt::Display) -> Self {
        Self::at(PublicationFailureStage::ConstructionWorker, error)
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
        self.build_measured().map(|(publication, _)| publication)
    }

    /// [`Self::build`], plus the counted facts its phase owner measured -- among them how many
    /// documents this build lowered and how many it took from the authority's memo.
    ///
    /// The memo itself stays behind the authority: this reports what happened, and there is no
    /// handle on an entry to be had from it.
    pub fn build_measured(
        self,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        crate::build_measured(self.request).map_err(|error| {
            PublicationBuildFailure::at(PublicationFailureStage::ModelConstruction, error)
        })
    }
}

#[derive(Debug)]
struct CachedLibraryStratum {
    key: LibraryStratumKey,
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

    /// Publishes through the canonical owner and reports measurements captured at phase barriers.
    pub fn publish_measured(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        self.prepare(documents, reported_documents)
            .and_then(PreparedPublication::build_measured)
    }

    #[doc(hidden)]
    pub fn publish_measured_sequential_for_testing(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        self.prepare_with_schedule(
            documents,
            reported_documents,
            ConstructionSchedule::Sequential,
        )
        .and_then(PreparedPublication::build_measured)
    }

    /// Prepares the canonical request without parsing or building it, so an atomic publication
    /// owner can capture its dependency-complete identity before background construction starts.
    pub fn prepare(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        self.prepare_with_schedule(
            documents,
            reported_documents,
            ConstructionSchedule::Parallel,
        )
    }

    fn prepare_with_schedule(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
        schedule: ConstructionSchedule,
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
            BuildRequest::new(workspace, schedule, crate::RESOLVED_CONTRACT)
        } else {
            let stratum = self.library_stratum(&libraries)?;
            BuildRequest::with_library(workspace, schedule, crate::RESOLVED_CONTRACT, stratum)
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
fn library_key<'a>(documents: impl IntoIterator<Item = &'a SourceDocument>) -> LibraryStratumKey {
    LibraryStratumKey::new(documents.into_iter().map(library_source_identity))
}

fn library_source_identity(document: &SourceDocument) -> LibrarySourceIdentity<'_> {
    LibrarySourceIdentity::new(
        document.uri().as_str(),
        match document.kind() {
            SourceKind::Workspace => SourceRole::Workspace,
            SourceKind::StandardLibrary => SourceRole::StandardLibrary,
            SourceKind::Library => SourceRole::Library,
            SourceKind::External => SourceRole::External,
        },
        document.digest(),
        document.library_location(),
    )
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

    fn cached_key(authority: &PublicationAuthority) -> LibraryStratumKey {
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
    fn publication_identity_commits_library_root_precedence_and_relative_path() {
        let authority = authority();
        let first = document(
            "memory://roots/first/collision.sysml",
            "package Collision { part def First; }",
            SourceKind::Library,
        );
        let second = document(
            "memory://roots/second/collision.sysml",
            "package Collision { part def Second; }",
            SourceKind::Library,
        );
        let ordered = authority
            .prepare(
                &[
                    first.with_library_location(0, "collision.sysml"),
                    second.with_library_location(1, "collision.sysml"),
                ],
                [],
            )
            .unwrap()
            .identity()
            .clone();
        let reordered = authority
            .prepare(
                &[
                    first.with_library_location(1, "collision.sysml"),
                    second.with_library_location(0, "collision.sysml"),
                ],
                [],
            )
            .unwrap()
            .identity()
            .clone();

        assert_ne!(ordered.source_digest(), reordered.source_digest());
    }

    #[test]
    fn publication_model_identity_is_invariant_to_host_document_order() {
        let authority = authority();
        let first = document(
            "memory://workspace/a.sysml",
            "package A;",
            SourceKind::Workspace,
        );
        let second = document(
            "memory://workspace/b.sysml",
            "package B;",
            SourceKind::Workspace,
        );
        let forward = authority
            .prepare(&[first.clone(), second.clone()], [])
            .unwrap()
            .identity()
            .model_digest();
        let reverse = authority
            .prepare(&[second, first], [])
            .unwrap()
            .identity()
            .model_digest();
        assert_eq!(forward, reverse);
    }

    #[test]
    fn reporting_identity_keeps_only_canonical_admitted_non_workspace_additions() {
        let authority = authority();
        let workspace = document(
            "memory://workspace/model.sysml",
            "package Model;",
            SourceKind::Workspace,
        );
        let external = document(
            "memory://external/model.sysml",
            "package External;",
            SourceKind::External,
        );
        let documents = [workspace, external];

        let baseline = authority
            .prepare(&documents, [])
            .unwrap()
            .identity()
            .clone();
        let irrelevant = authority
            .prepare(
                &documents,
                [
                    Box::from("memory://workspace/model.sysml"),
                    Box::from("memory://unknown/model.sysml"),
                ],
            )
            .unwrap()
            .identity()
            .clone();
        assert_eq!(baseline, irrelevant);

        let canonical = authority
            .prepare(
                &documents,
                [
                    Box::from("memory://external/model.sysml"),
                    Box::from("memory://external/model.sysml"),
                ],
            )
            .unwrap()
            .identity()
            .clone();
        assert_ne!(baseline, canonical);
        assert_eq!(
            canonical.reported_documents(),
            [Box::<str>::from("memory://external/model.sysml")]
        );
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
    fn the_lowering_memo_holds_the_last_publication_and_no_history() {
        let authority = authority();
        let stable = document(
            "memory://workspace/stable.sysml",
            "package Stable { part def S; }",
            SourceKind::Workspace,
        );
        let edited = |revision: usize| {
            document(
                "memory://workspace/edited.sysml",
                &format!("package Edited {{ part def E; }} // revision {revision}"),
                SourceKind::Workspace,
            )
        };
        let (_, first) = authority
            .prepare(&[stable.clone(), edited(0)], [])
            .unwrap()
            .build_measured()
            .unwrap();
        assert_eq!(first.documents_lowered, 2);
        assert_eq!(first.documents_reused, 0);
        assert_eq!(authority.lowering.len(), 2);

        let (_, second) = authority
            .prepare(&[stable.clone(), edited(1)], [])
            .unwrap()
            .build_measured()
            .unwrap();
        assert_eq!(second.documents_lowered, 1, "only the edited document");
        assert_eq!(second.documents_reused, 1);
        assert_eq!(
            authority.lowering.len(),
            2,
            "the superseded revision is evicted rather than accumulated"
        );

        // Dropping a document from the source set drops its entry one build later.
        authority.publish(&[stable], []).unwrap();
        assert_eq!(authority.lowering.len(), 1);
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

    #[test]
    fn construction_worker_failure_has_a_distinct_typed_stage() {
        let failure = PublicationBuildFailure::construction_worker("worker panicked");
        assert_eq!(failure.stage(), PublicationFailureStage::ConstructionWorker);
        assert_eq!(failure.message(), "worker panicked");
    }
}
