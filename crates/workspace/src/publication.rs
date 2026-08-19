//! Canonical immutable-publication construction and library-stratum reuse.
//!
//! Hosts provide admitted source documents. This owner alone partitions their provenance,
//! decides whether a settled library stratum can be reused, constructs the query request, and
//! returns the immutable publication. Library construction failure is explicit: it is never
//! disguised by silently selecting a different construction path.

use std::sync::{Arc, Mutex};

use sysml_query::resolved_slice::{
    build, BuildRequest, ConstructionStrategy, LibraryStratum, PublishedModel, SourceDocument,
    SourceKind,
};
use sysml_source::{SysmlDocument, SysmlDocumentSourceKind};

use crate::error::{WorkspaceError, WorkspaceResult};

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
        documents: &[SysmlDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> WorkspaceResult<Arc<PublishedModel>> {
        self.prepare(documents, reported_documents)
            .and_then(PreparedPublication::build)
            .map_err(|error| WorkspaceError::internal_invariant_failure(error.to_string()))
    }

    /// Prepares the canonical request without building it, allowing an atomic publication owner
    /// to capture its dependency-complete identity before background construction starts.
    pub fn prepare(
        &self,
        documents: &[SysmlDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        let mut ordered = documents.iter().collect::<Vec<_>>();
        ordered.sort_unstable_by(|left, right| left.uri.as_str().cmp(right.uri.as_str()));

        let mut workspace = Vec::new();
        let mut libraries = Vec::new();
        for document in ordered {
            let source = SourceDocument::from_uri(
                document.uri.as_str(),
                document.content.clone(),
                source_kind(document.source_kind),
            )
            .map_err(|error| {
                PublicationBuildFailure::at(
                    PublicationFailureStage::SourceAdmission,
                    format!("admitting {}: {error}", document.uri),
                )
            })?;
            if matches!(
                document.source_kind,
                SysmlDocumentSourceKind::StandardLibrary | SysmlDocumentSourceKind::Library
            ) {
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
        libraries: &[(&SysmlDocument, SourceDocument)],
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

fn library_key<'a>(documents: impl IntoIterator<Item = &'a SysmlDocument>) -> blake3::Hash {
    let mut digest = blake3::Hasher::new();
    digest.update(b"spec42-library-stratum-v1\0");
    for document in documents {
        let identity = document.uri.as_str().as_bytes();
        digest.update(&(identity.len() as u64).to_le_bytes());
        digest.update(identity);
        digest.update(&[match document.source_kind {
            SysmlDocumentSourceKind::Workspace => 0,
            SysmlDocumentSourceKind::StandardLibrary => 1,
            SysmlDocumentSourceKind::Library => 2,
            SysmlDocumentSourceKind::External => 3,
        }]);
        digest.update(&(document.content.len() as u64).to_le_bytes());
        digest.update(document.content.as_bytes());
    }
    digest.finalize()
}

fn source_kind(kind: SysmlDocumentSourceKind) -> SourceKind {
    match kind {
        SysmlDocumentSourceKind::Workspace => SourceKind::Workspace,
        SysmlDocumentSourceKind::StandardLibrary => SourceKind::StandardLibrary,
        SysmlDocumentSourceKind::Library => SourceKind::Library,
        SysmlDocumentSourceKind::External => SourceKind::External,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_query::resolved_slice::{DiagramSemanticReference, QueryOutcome};
    use url::Url;

    fn document(uri: &str, content: &str, source_kind: SysmlDocumentSourceKind) -> SysmlDocument {
        SysmlDocument {
            uri: Url::parse(uri).unwrap(),
            content: content.to_owned(),
            path_hint: None,
            source_kind,
            content_digest: None,
            byte_size: None,
        }
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
            SysmlDocumentSourceKind::StandardLibrary,
        );
        let workspace = document(
            "memory://workspace/model.sysml",
            "package W { part w : Lib::Wheel; }",
            SysmlDocumentSourceKind::Workspace,
        );
        coordinator
            .publish(&[library.clone(), workspace.clone()], [])
            .unwrap();
        let initial = cached_key(&coordinator);

        let mut edited_workspace = workspace;
        edited_workspace.content =
            "package W { part w : Lib::Wheel; part x : Lib::Wheel; }".to_owned();
        coordinator
            .publish(&[library.clone(), edited_workspace], [])
            .unwrap();
        assert_eq!(cached_key(&coordinator), initial);

        let mut changed_library = library.clone();
        changed_library.content =
            "standard library package Lib { part def Wheel; part def Axle; }".to_owned();
        coordinator.publish(&[changed_library], []).unwrap();
        assert_ne!(cached_key(&coordinator), initial);

        let mut changed_role = library;
        changed_role.source_kind = SysmlDocumentSourceKind::Library;
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
                SysmlDocumentSourceKind::StandardLibrary,
            ),
            document(
                "memory://workspace/model.sysml",
                concat!(
                    "package W { import StandardViewDefinitions::*; part root; ",
                    "view selected : GeneralView { expose root; filter @SysML::PartUsage; } }"
                ),
                SysmlDocumentSourceKind::Workspace,
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
            SysmlDocumentSourceKind::StandardLibrary,
        );
        let error = coordinator
            .prepare(&[duplicate.clone(), duplicate], [])
            .expect_err("duplicate library identities must fail stratum construction");

        assert_eq!(error.stage(), PublicationFailureStage::LibraryConstruction);
        assert!(coordinator.library.lock().unwrap().is_none());
    }
}
