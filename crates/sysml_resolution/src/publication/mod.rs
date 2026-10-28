//! The publication authority: canonical construction of an immutable publication from admitted
//! documents, library-stratum reuse, and the lifecycle a long-lived host drives it through.
//!
//! Hosts hand over [`SourceDocument`]s. This owner alone partitions their provenance, decides
//! whether a settled library stratum can be reused, builds the request, and constructs the
//! publication. Every admitted document is parsed through the syntax authority's memo, so the
//! editor's parse and the build's parse are one tree. Library construction failure is explicit:
//! it is never disguised by silently selecting a different construction path.

mod cancellation;
mod session;
pub use cancellation::BuildCancellation;

use std::sync::{Arc, Mutex};

use source_identity::{LibrarySourceIdentity, LibraryStratumKey, SourceRole};
use sysml_contract::StandardLibraryAvailability;
use sysml_source::{SourceDocument, SourceKind};

use crate::lower::memo::LoweringMemo;
use crate::syntax::SyntaxAuthority;
use crate::{
    BuildRequest, ConstructionSchedule, LibraryStratum, PublicationIdentity, PublishedResolution,
    SourceInput,
};

pub use session::{
    AnalysisStatus, BuildToken, InputRevisionToken, PublicationOutcome, PublicationToken,
    Published, Session, SessionLifecycle,
};

/// The semantic phase which rejected a publication request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationFailureStage {
    SourceAdmission,
    LibraryConstruction,
    RequestConstruction,
    ModelConstruction,
    PreparationWorker,
    ConstructionWorker,
    PublicationAdmission,
    Cancelled,
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

    /// Cooperative cancellation before or during request preparation.
    pub fn cancelled() -> Self {
        Self::at(PublicationFailureStage::Cancelled, "publication cancelled")
    }

    pub fn is_cancelled(&self) -> bool {
        self.stage == PublicationFailureStage::Cancelled
    }

    fn from_build_failure(stage: PublicationFailureStage, error: crate::BuildFailure) -> Self {
        let stage = if matches!(error, crate::BuildFailure::Cancelled { .. }) {
            PublicationFailureStage::Cancelled
        } else {
            stage
        };
        Self::at(stage, error)
    }

    fn at(stage: PublicationFailureStage, error: impl std::fmt::Display) -> Self {
        Self {
            stage,
            message: error.to_string(),
        }
    }

    /// A completed model must belong to the exact request admitted by the session.
    pub fn admission_identity_mismatch() -> Self {
        Self::at(
            PublicationFailureStage::PublicationAdmission,
            "finished model identity differs from the admitted build request",
        )
    }

    /// Preparation worker failed before returning its request or typed preparation error.
    pub fn preparation_worker(error: impl std::fmt::Display) -> Self {
        Self::at(PublicationFailureStage::PreparationWorker, error)
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
/// Workspace documents are parsed when the request is built. Preparing can construct a missing
/// library stratum, including parsing and solving its sources. Hosts perform preparation off their
/// executor and verify their captured input revision before starting the prepared build.
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

    pub fn with_cancellation(mut self, cancellation: BuildCancellation) -> Self {
        self.request = self.request.with_cancellation(Some(cancellation));
        self
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
            PublicationBuildFailure::from_build_failure(
                PublicationFailureStage::ModelConstruction,
                error,
            )
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
        self.publish_with_standard_library_availability(
            documents,
            StandardLibraryAvailability::Unavailable,
            reported_documents,
        )
    }

    pub fn publish_with_standard_library_availability(
        &self,
        documents: &[SourceDocument],
        standard_library_availability: StandardLibraryAvailability,
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PublishedResolution, PublicationBuildFailure> {
        self.prepare_with_standard_library_availability(
            documents,
            standard_library_availability,
            reported_documents,
        )
        .and_then(PreparedPublication::build)
    }

    /// Publishes through the canonical owner and reports measurements captured at phase barriers.
    pub fn publish_measured(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        self.publish_measured_with_standard_library_availability(
            documents,
            StandardLibraryAvailability::Unavailable,
            reported_documents,
        )
    }

    pub fn publish_measured_with_standard_library_availability(
        &self,
        documents: &[SourceDocument],
        standard_library_availability: StandardLibraryAvailability,
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        self.prepare_with_standard_library_availability(
            documents,
            standard_library_availability,
            reported_documents,
        )
        .and_then(PreparedPublication::build_measured)
    }

    #[doc(hidden)]
    pub fn publish_measured_sequential_for_testing(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        self.publish_measured_sequential_with_standard_library_availability_for_testing(
            documents,
            StandardLibraryAvailability::Unavailable,
            reported_documents,
        )
    }

    #[doc(hidden)]
    pub fn publish_measured_sequential_with_standard_library_availability_for_testing(
        &self,
        documents: &[SourceDocument],
        standard_library_availability: StandardLibraryAvailability,
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(PublishedResolution, crate::BuildMeasurements), PublicationBuildFailure> {
        self.prepare_with_schedule(
            documents,
            standard_library_availability,
            reported_documents,
            ConstructionSchedule::Sequential,
            None,
        )
        .and_then(PreparedPublication::build_measured)
    }

    /// Prepares the canonical request and its dependency-complete identity. A cache miss can
    /// construct a library stratum; this operation belongs on a blocking worker in async hosts.
    pub fn prepare(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        self.prepare_with_standard_library_availability(
            documents,
            StandardLibraryAvailability::Unavailable,
            reported_documents,
        )
    }

    pub fn prepare_with_standard_library_availability(
        &self,
        documents: &[SourceDocument],
        standard_library_availability: StandardLibraryAvailability,
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        self.prepare_with_schedule(
            documents,
            standard_library_availability,
            reported_documents,
            ConstructionSchedule::Parallel,
            None,
        )
    }

    pub fn prepare_cancellable(
        &self,
        documents: &[SourceDocument],
        availability: StandardLibraryAvailability,
        reported: impl IntoIterator<Item = Box<str>>,
        cancellation: &BuildCancellation,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        self.prepare_with_schedule(
            documents,
            availability,
            reported,
            ConstructionSchedule::Parallel,
            Some(cancellation),
        )
    }

    fn prepare_with_schedule(
        &self,
        documents: &[SourceDocument],
        standard_library_availability: StandardLibraryAvailability,
        reported_documents: impl IntoIterator<Item = Box<str>>,
        schedule: ConstructionSchedule,
        cancellation: Option<&BuildCancellation>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        checkpoint(cancellation, crate::ConstructionPhase::SourceAdmission)?;
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
            let stratum = self.library_stratum(&libraries, cancellation)?;
            BuildRequest::with_library(workspace, schedule, crate::RESOLVED_CONTRACT, stratum)
        }
        .map_err(|error| {
            PublicationBuildFailure::at(PublicationFailureStage::RequestConstruction, error)
        })?
        .reporting(reported)
        .with_standard_library_availability(standard_library_availability)
        .with_syntax(Arc::clone(&self.syntax))
        .with_lowering(Arc::clone(&self.lowering))
        .with_cancellation(cancellation.cloned());
        checkpoint(cancellation, crate::ConstructionPhase::SourceAdmission)?;
        Ok(PreparedPublication { request })
    }

    fn library_stratum(
        &self,
        libraries: &[(&SourceDocument, SourceInput)],
        cancellation: Option<&BuildCancellation>,
    ) -> Result<Arc<LibraryStratum>, PublicationBuildFailure> {
        checkpoint(cancellation, crate::ConstructionPhase::LibrarySeed)?;
        let key = library_key(libraries.iter().map(|(document, _)| *document));
        {
            let cached = self
                .library
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(entry) = cached.as_ref() {
                if entry.key == key {
                    return Ok(Arc::clone(&entry.stratum));
                }
            }
        }
        // Construction can run nested Rayon jobs, which may steal another publication request
        // from the same pool. Holding this cache lock would let that request wait on itself.
        // Concurrent misses may build the same immutable stratum; reconcile only after building.
        let stratum = Arc::new(
            crate::build_library_stratum_cancellable(
                libraries.iter().map(|(_, source)| source.clone()).collect(),
                Some(Arc::clone(&self.syntax)),
                Some(Arc::clone(&self.lowering)),
                cancellation.cloned(),
            )
            .map_err(|error| {
                PublicationBuildFailure::from_build_failure(
                    PublicationFailureStage::LibraryConstruction,
                    error,
                )
            })?,
        );
        checkpoint(cancellation, crate::ConstructionPhase::LibrarySeed)?;
        let mut cached = self
            .library
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(entry) = cached.as_ref() {
            if entry.key == key {
                return Ok(Arc::clone(&entry.stratum));
            }
        }
        *cached = Some(CachedLibraryStratum {
            key,
            stratum: Arc::clone(&stratum),
        });
        Ok(stratum)
    }
}

fn checkpoint(
    cancellation: Option<&BuildCancellation>,
    phase: crate::ConstructionPhase,
) -> Result<(), PublicationBuildFailure> {
    cancellation.map_or(Ok(()), |control| {
        control.checkpoint(phase).map_err(|error| {
            PublicationBuildFailure::from_build_failure(PublicationFailureStage::Cancelled, error)
        })
    })
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
        document.language(),
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
    fn concurrent_library_preparation_does_not_hold_the_cache_lock_across_nested_work() {
        use rayon::prelude::*;

        let authority = authority();
        let documents: Vec<_> = (0..8)
            .map(|index| {
                document(
                    &format!("memory://library/lib{index}.sysml"),
                    &format!("standard library package Lib{index} {{ part def P; }}"),
                    SourceKind::StandardLibrary,
                )
            })
            .collect();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .stack_size(16 * 1024 * 1024)
            .build()
            .unwrap();
        let identities: Vec<_> = pool.install(|| {
            (0..8)
                .into_par_iter()
                .map(|_| {
                    authority
                        .prepare(&documents, [])
                        .unwrap()
                        .identity()
                        .clone()
                })
                .collect()
        });
        let warm = authority
            .prepare(&documents, [])
            .unwrap()
            .identity()
            .clone();
        let cold = self::authority()
            .prepare(&documents, [])
            .unwrap()
            .identity()
            .clone();
        assert!(identities.iter().all(|identity| identity == &warm));
        assert_eq!(warm, cold);
    }

    #[test]
    fn cancellation_at_each_expensive_boundary_returns_no_publication() {
        use crate::ConstructionPhase;
        let phases = [
            ConstructionPhase::SourceAdmission,
            ConstructionPhase::Lowering,
            ConstructionPhase::Splicing,
            ConstructionPhase::Resolution,
            ConstructionPhase::Evaluation,
            ConstructionPhase::Indexing,
            ConstructionPhase::Diagnostics,
        ];
        for phase in phases {
            let authority = authority();
            let workspace = document(
                "memory://cancellation/model.sysml",
                "package P { part def A; part a : A; }",
                SourceKind::Workspace,
            );
            let cancellation = BuildCancellation::at_phase(phase);
            let result = authority
                .prepare_cancellable(
                    &[workspace.clone()],
                    StandardLibraryAvailability::Unavailable,
                    [],
                    &cancellation,
                )
                .and_then(PreparedPublication::build_measured);
            let error = result.unwrap_err();
            assert_eq!(
                error.stage(),
                PublicationFailureStage::Cancelled,
                "{phase:?}"
            );
            assert!(error.is_cancelled());
            assert!(authority.library.lock().unwrap().is_none());
            // A cancelled model is never cached as a publication. Completed per-document phase
            // products may be reused by a later complete build under the same complete key.
            let recovered = authority.publish(&[workspace], []);
            assert!(recovered.is_ok(), "{phase:?}: {recovered:?}");
        }
    }

    #[test]
    fn cancelling_library_preparation_does_not_install_a_partial_stratum() {
        let authority = authority();
        let library = document(
            "memory://cancellation/lib.sysml",
            "library package Lib { part def A; }",
            SourceKind::Library,
        );
        let cancellation = BuildCancellation::at_phase(crate::ConstructionPhase::LibrarySeed);
        let error = authority
            .prepare_cancellable(
                &[library.clone()],
                StandardLibraryAvailability::Unavailable,
                [],
                &cancellation,
            )
            .unwrap_err();
        assert!(error.is_cancelled());
        assert!(authority.library.lock().unwrap().is_none());
        authority.publish(&[library], []).unwrap();
        assert!(authority.library.lock().unwrap().is_some());
    }

    #[test]
    fn preparation_worker_failure_has_a_distinct_typed_stage() {
        let failure = PublicationBuildFailure::preparation_worker("worker panicked");
        assert_eq!(failure.stage(), PublicationFailureStage::PreparationWorker);
        assert_eq!(failure.message(), "worker panicked");
    }

    #[test]
    fn construction_worker_failure_has_a_distinct_typed_stage() {
        let failure = PublicationBuildFailure::construction_worker("worker panicked");
        assert_eq!(failure.stage(), PublicationFailureStage::ConstructionWorker);
        assert_eq!(failure.message(), "worker panicked");
    }
}
