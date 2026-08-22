//! Canonical immutable-publication construction and library-stratum reuse.
//!
//! Hosts provide admitted source documents. This owner alone partitions their provenance,
//! decides whether a settled library stratum can be reused, constructs the query request, and
//! returns the immutable publication. Library construction failure is explicit: it is never
//! disguised by silently selecting a different construction path.
#![recursion_limit = "256"]

use std::sync::{Arc, Mutex};

use sysml_query::resolved_slice::{
    build, AdmittedSource, BuildRequest, ConstructionStrategy, LibraryStratum, PublishedModel,
};
use sysml_query::source::{SourceDocument, SourceKind};

/// The semantic phase which rejected a publication request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationFailureStage {
    SourceAdmission,
    LibraryConstruction,
    RequestConstruction,
    ModelConstruction,
}

/// A typed failure from the sole publication construction path.
#[derive(Debug)]
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

/// An immutable, dependency-complete request prepared by [`PublicationCoordinator`].
#[derive(Debug)]
pub struct PreparedPublication {
    request: BuildRequest,
}

impl PreparedPublication {
    pub fn request(&self) -> &BuildRequest {
        &self.request
    }

    pub fn build(self) -> Result<Arc<PublishedModel>, PublicationBuildFailure> {
        build(self.request).map(Arc::new).map_err(|error| {
            PublicationBuildFailure::at(PublicationFailureStage::ModelConstruction, error)
        })
    }
}

#[derive(Debug)]
struct CachedLibraryStratum {
    key: blake3::Hash,
    stratum: Arc<LibraryStratum>,
}

/// One build/cache authority shared by every workspace publication in an engine environment.
#[derive(Debug, Default)]
pub struct PublicationCoordinator {
    library: Mutex<Option<CachedLibraryStratum>>,
}

impl PublicationCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes the exact admitted source set, reusing a solved library only when its complete
    /// canonical identity is unchanged.
    pub fn publish(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<Arc<PublishedModel>, PublicationBuildFailure> {
        self.prepare(documents, reported_documents)
            .and_then(PreparedPublication::build)
    }

    /// Prepares the canonical request without building it, allowing an atomic publication owner
    /// to capture its dependency-complete identity before background construction starts.
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
            let source = AdmittedSource::from_uri(
                document.uri().as_str(),
                document.content().to_owned(),
                document.kind(),
            )
            .map_err(|error| {
                PublicationBuildFailure::at(
                    PublicationFailureStage::SourceAdmission,
                    format!("admitting {}: {error}", document.uri()),
                )
            })?;
            if document.kind().is_library() {
                libraries.push((document, source));
            } else {
                workspace.push(source);
            }
        }

        let reported = reported_documents.into_iter().collect::<Vec<_>>();
        let request = if libraries.is_empty() {
            BuildRequest::resolved(workspace, ConstructionStrategy::Parallel)
        } else {
            let stratum = self.library_stratum(&libraries)?;
            BuildRequest::resolved_with_library(workspace, ConstructionStrategy::Parallel, &stratum)
        }
        .map_err(|error| {
            PublicationBuildFailure::at(PublicationFailureStage::RequestConstruction, error)
        })?
        .reporting(reported);
        Ok(PreparedPublication { request })
    }

    fn library_stratum(
        &self,
        libraries: &[(&SourceDocument, AdmittedSource)],
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
            LibraryStratum::build(libraries.iter().map(|(_, source)| source.clone()).collect())
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
    use sysml_query::resolved_slice::{DiagramSemanticReference, QueryOutcome};
    use sysml_query::source::SourceService;

    fn document(uri: &str, content: &str, kind: SourceKind) -> SourceDocument {
        SourceService::new().admit(uri, content, kind).unwrap()
    }

    fn cached_key(coordinator: &PublicationCoordinator) -> blake3::Hash {
        coordinator
            .library
            .lock()
            .unwrap()
            .as_ref()
            .expect("cached library")
            .key
    }

    #[test]
    fn library_identity_covers_content_and_provenance_but_not_workspace_edits() {
        let coordinator = PublicationCoordinator::new();
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
        coordinator
            .publish(&[library.clone(), workspace.clone()], [])
            .unwrap();
        let initial = cached_key(&coordinator);

        let edited_workspace = document(
            "memory://workspace/model.sysml",
            "package W { part w : Lib::Wheel; part x : Lib::Wheel; }",
            SourceKind::Workspace,
        );
        coordinator
            .publish(&[library.clone(), edited_workspace], [])
            .unwrap();
        assert_eq!(cached_key(&coordinator), initial);

        let changed_library = document(
            "memory://library/lib.sysml",
            "standard library package Lib { part def Wheel; part def Axle; }",
            SourceKind::StandardLibrary,
        );
        coordinator.publish(&[changed_library], []).unwrap();
        assert_ne!(cached_key(&coordinator), initial);

        let changed_role = library.with_kind(SourceKind::Library);
        coordinator.publish(&[changed_role], []).unwrap();
        assert_ne!(cached_key(&coordinator), initial);
    }

    #[test]
    fn warm_publication_preserves_library_public_imports() {
        let coordinator = PublicationCoordinator::new();
        let documents = vec![
            document(
                "memory://library/lib.sysml",
                concat!(
                    "standard library package StandardViewDefinitions { view def GeneralView; } ",
                    "standard library package SysML { public import Systems::*; ",
                    "package Systems { metaclass PartUsage; } }"
                ),
                SourceKind::StandardLibrary,
            ),
            document(
                "memory://workspace/model.sysml",
                concat!(
                    "package W { import StandardViewDefinitions::*; part root; ",
                    "view selected : GeneralView { expose root; filter @SysML::PartUsage; } }"
                ),
                SourceKind::Workspace,
            ),
        ];

        for _ in 0..2 {
            let model = coordinator.publish(&documents, []).unwrap();
            let catalog = match model.diagrams().catalog() {
                QueryOutcome::Resolved(catalog)
                | QueryOutcome::Recovered(catalog)
                | QueryOutcome::UnsupportedWith(catalog) => catalog,
                other => panic!("catalog: {other:?}"),
            };
            let view = catalog
                .iter()
                .find(|entry| {
                    matches!(
                        &entry.reference,
                        DiagramSemanticReference::Qualified { qualified_name, .. }
                            if qualified_name.as_ref() == "W::selected"
                    )
                })
                .unwrap();
            let projection = match model.diagrams().view(&view.semantic_id) {
                QueryOutcome::Resolved(projection) => projection,
                other => panic!("projection: {other:?}"),
            };
            assert!(projection.incomplete_reasons.is_empty());
        }
    }

    #[test]
    fn library_construction_failure_is_explicit_and_never_flattened() {
        let coordinator = PublicationCoordinator::new();
        let duplicate = document(
            "memory://library/duplicate.sysml",
            "standard library package Lib;",
            SourceKind::StandardLibrary,
        );
        let error = coordinator
            .prepare(&[duplicate.clone(), duplicate], [])
            .expect_err("duplicate library identities must fail stratum construction");

        assert_eq!(error.stage(), PublicationFailureStage::LibraryConstruction);
        assert!(coordinator.library.lock().unwrap().is_none());
    }
}
