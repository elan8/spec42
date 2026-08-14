//! The only supported consumer facade over Spec42's semantic model implementation.
//!
//! [`PublishedModel`] is opaque. Consumers select a cohesive service and receive typed answers or
//! stream an owner-defined debug projection; they cannot obtain the structural graph, resolver
//! state, fact collections, or query-index storage.

#[cfg(feature = "immutable-resolution")]
pub mod resolved_slice;

#[cfg(feature = "legacy-model")]
mod legacy {
    use std::fmt;

    pub use sysml_model::{
        AuthoredReferenceId, NavigationOutcome, NavigationReference, NavigationTarget, NodeId,
        ReferenceKind, RelationshipKind, ResolutionOutcome, TextPosition, TextRange,
    };
    use url::Url;

    /// Source provenance admitted to one immutable semantic publication.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SourceKind {
        Workspace,
        StandardLibrary,
        Library,
        External,
    }

    impl From<SourceKind> for sysml_model::SysmlDocumentSourceKind {
        fn from(value: SourceKind) -> Self {
            match value {
                SourceKind::Workspace => Self::Workspace,
                SourceKind::StandardLibrary => Self::StandardLibrary,
                SourceKind::Library => Self::Library,
                SourceKind::External => Self::External,
            }
        }
    }

    /// Immutable source input. Its implementation document is never exposed to consumers.
    #[derive(Debug, Clone)]
    pub struct SourceDocument {
        inner: sysml_model::SysmlDocument,
    }

    impl SourceDocument {
        pub fn from_uri(
            uri: &str,
            content: String,
            source_kind: SourceKind,
        ) -> Result<Self, SourceError> {
            sysml_model::SysmlDocument::from_uri(uri, content, None, source_kind.into(), None, None)
                .map(|inner| Self { inner })
                .map_err(SourceError)
        }

        pub fn from_memory_path(
            namespace: &str,
            path: &str,
            content: String,
            source_kind: SourceKind,
        ) -> Result<Self, SourceError> {
            sysml_model::SysmlDocument::from_memory_path(
                namespace,
                path,
                content,
                source_kind.into(),
                None,
                None,
            )
            .map(|inner| Self { inner })
            .map_err(SourceError)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SourceError(String);

    impl fmt::Display for SourceError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl std::error::Error for SourceError {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ConstructionStrategy {
        Sequential,
        Parallel,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EvaluationPolicy {
        ResolvedOnly,
        Evaluate,
    }

    /// Complete inputs for one coherent publication. There is no constructor from a graph, resolver,
    /// partial index, or prior mutable state.
    #[derive(Debug)]
    pub struct BuildRequest {
        inner: sysml_model::PreparedSemanticBuildRequest,
        identity: PublicationIdentity,
    }

    impl BuildRequest {
        pub fn new(
            sources: Vec<SourceDocument>,
            construction: ConstructionStrategy,
            evaluation: EvaluationPolicy,
            semantic_contract_version: impl Into<String>,
        ) -> Result<Self, BuildError> {
            let sources = sources.into_iter().map(|source| source.inner).collect();
            let sources = sysml_model::ImmutableSourceSnapshot::new(sources)
                .map_err(|error| BuildError(error.to_string()))?;
            let inner = sysml_model::SemanticBuildRequest {
                sources,
                construction: match construction {
                    ConstructionStrategy::Sequential => {
                        sysml_model::ConstructionStrategy::Sequential
                    }
                    ConstructionStrategy::Parallel => sysml_model::ConstructionStrategy::Parallel,
                },
                evaluation: match evaluation {
                    EvaluationPolicy::ResolvedOnly => sysml_model::EvaluationPolicy::ResolvedOnly,
                    EvaluationPolicy::Evaluate => sysml_model::EvaluationPolicy::Evaluate,
                },
                configuration: sysml_model::SemanticConfiguration {
                    semantic_contract_version: semantic_contract_version.into(),
                },
            };
            let inner = inner.prepare();
            let identity = PublicationIdentity {
                inner: inner.identity().clone(),
            };
            Ok(Self { inner, identity })
        }

        pub fn evaluated(
            sources: Vec<SourceDocument>,
            construction: ConstructionStrategy,
        ) -> Result<Self, BuildError> {
            Self::new(
                sources,
                construction,
                EvaluationPolicy::Evaluate,
                "canonical-resolution-v1",
            )
        }

        /// Returns the precomputed dependency-complete identity without revisiting source contents.
        pub fn identity(&self) -> &PublicationIdentity {
            &self.identity
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BuildError(String);

    impl fmt::Display for BuildError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl std::error::Error for BuildError {}

    /// Opaque, immutable, settled semantic publication.
    #[derive(Debug)]
    pub struct PublishedModel {
        inner: sysml_model::SemanticModel,
        identity: PublicationIdentity,
    }

    pub fn build(request: BuildRequest) -> Result<PublishedModel, BuildError> {
        let BuildRequest { inner, identity } = request;
        let inner = sysml_model::build_prepared_semantic_model(inner)
            .map_err(|error| BuildError(error.to_string()))?;
        debug_assert_eq!(inner.identity(), &identity.inner);
        Ok(PublishedModel { inner, identity })
    }

    impl PublishedModel {
        pub fn debug(&self) -> DebugQueries<'_> {
            DebugQueries { model: &self.inner }
        }

        pub fn diagnostics(&self) -> DiagnosticQueries<'_> {
            DiagnosticQueries { model: &self.inner }
        }

        pub fn navigation(&self) -> NavigationQueries<'_> {
            NavigationQueries { model: &self.inner }
        }

        pub fn resolution(&self) -> ResolutionQueries<'_> {
            ResolutionQueries { model: &self.inner }
        }

        pub fn publication(&self) -> PublicationQueries<'_> {
            PublicationQueries { model: self }
        }
    }

    /// Opaque identity committing every input and phase that can affect a publication.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PublicationIdentity {
        inner: sysml_model::SemanticModelIdentity,
    }

    impl PublicationIdentity {
        /// Content component of the identity. This digest is not the complete publication identity;
        /// phase and contract inputs below participate in equality as well.
        pub fn source_digest(&self) -> &str {
            &self.inner.source_digest
        }

        pub fn semantic_contract_version(&self) -> &str {
            &self.inner.semantic_contract_version
        }

        pub fn construction_strategy(&self) -> ConstructionStrategy {
            match self.inner.construction {
                sysml_model::ConstructionStrategy::Sequential => ConstructionStrategy::Sequential,
                sysml_model::ConstructionStrategy::Parallel => ConstructionStrategy::Parallel,
            }
        }

        pub fn evaluation_policy(&self) -> EvaluationPolicy {
            match self.inner.evaluation {
                sysml_model::EvaluationPolicy::ResolvedOnly => EvaluationPolicy::ResolvedOnly,
                sysml_model::EvaluationPolicy::Evaluate => EvaluationPolicy::Evaluate,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PublicationCompleteness {
        Complete,
        EditorRecovery,
    }

    /// Publication metadata needed by atomic owners without exposing the model implementation.
    pub struct PublicationQueries<'a> {
        model: &'a PublishedModel,
    }

    impl<'a> PublicationQueries<'a> {
        /// Borrows the small identity stored beside the exact request consumed by this publication.
        pub fn identity(&self) -> &'a PublicationIdentity {
            &self.model.identity
        }

        pub fn completeness(&self) -> PublicationCompleteness {
            match self.model.inner.completeness() {
                sysml_model::SemanticModelCompleteness::Complete => {
                    PublicationCompleteness::Complete
                }
                sysml_model::SemanticModelCompleteness::EditorRecovery => {
                    PublicationCompleteness::EditorRecovery
                }
            }
        }
    }

    /// Local resolution and relationship queries backed by eager publication indexes.
    pub struct ResolutionQueries<'a> {
        model: &'a sysml_model::SemanticModel,
    }

    impl<'a> ResolutionQueries<'a> {
        pub fn outcome(&self, reference: &AuthoredReferenceId) -> Option<&'a ResolutionOutcome> {
            self.model.authored_reference_outcome(reference)
        }

        pub fn outgoing(&self, source: &NodeId, kind: RelationshipKind) -> &'a [NodeId] {
            self.model.resolved_outgoing(source, kind)
        }

        pub fn incoming(&self, target: &NodeId, kind: RelationshipKind) -> &'a [NodeId] {
            self.model.resolved_incoming(target, kind)
        }
    }

    /// Owner-defined diagnostic projections. This service streams canonical formats and never returns
    /// a fact inventory that callers could reinterpret as a second semantic system.
    pub struct DebugQueries<'a> {
        model: &'a sysml_model::SemanticModel,
    }

    impl DebugQueries<'_> {
        pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
            self.model.write_debug_sexpr(output)
        }

        pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
            self.model.write_navigation_debug_sexpr(output)
        }
    }

    /// Canonical diagnostics over the same settled publication.
    pub struct DiagnosticQueries<'a> {
        model: &'a sysml_model::SemanticModel,
    }

    impl DiagnosticQueries<'_> {
        pub fn write_document_sexpr(
            &self,
            source: &SourceDocument,
            output: &mut dyn fmt::Write,
        ) -> fmt::Result {
            let diagnostics = sysml_diagnostics::collect_document_diagnostics_from_model(
                self.model,
                false,
                &source.inner.uri,
                &source.inner.content,
                false,
                sysml_diagnostics::DiagnosticsOptions::default(),
            );
            sysml_diagnostics::write_diagnostics_sexpr(&diagnostics, output)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NavigationError(String);

    impl fmt::Display for NavigationError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl std::error::Error for NavigationError {}

    /// Source-position navigation over the immutable publication interval index.
    pub struct NavigationQueries<'a> {
        model: &'a sysml_model::SemanticModel,
    }

    impl NavigationQueries<'_> {
        pub fn references_at_position(
            &self,
            source: &SourceDocument,
            position: TextPosition,
        ) -> Result<Vec<NavigationReference>, NavigationError> {
            self.model
                .navigation_references_at_position(&source.inner.uri, position)
                .map_err(|error| NavigationError(error.to_string()))
        }

        pub fn references_at_uri(
            &self,
            uri: &Url,
            position: TextPosition,
        ) -> Result<Vec<NavigationReference>, NavigationError> {
            self.model
                .navigation_references_at_position(uri, position)
                .map_err(|error| NavigationError(error.to_string()))
        }
    }

    /// Raw semantic implementation types are intentionally not part of this crate's public API.
    ///
    /// ```compile_fail
    /// use sysml_query::{ResolutionFact, ResolutionState, SemanticQueryIndexes};
    /// ```
    ///
    /// ```compile_fail
    /// fn leak(model: &sysml_query::PublishedModel) {
    ///     let _ = model.view();
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// fn leak(model: &sysml_query::PublishedModel) {
    ///     let _ = &model.inner;
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// fn forge() {
    ///     let _ = sysml_query::PublishedModel::from_raw_parts();
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// fn leak(request: &sysml_query::BuildRequest) {
    ///     let _ = &request.sources;
    /// }
    /// ```
    ///
    /// Requests and publications deliberately cannot be deep-cloned. Publication sharing uses
    /// `Arc<PublishedModel>`.
    ///
    /// ```compile_fail
    /// fn require_clone<T: Clone>() {}
    /// require_clone::<sysml_query::BuildRequest>();
    /// ```
    ///
    /// ```compile_fail
    /// fn require_clone<T: Clone>() {}
    /// require_clone::<sysml_query::PublishedModel>();
    /// ```
    pub struct RawStorageIsNotPublic;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn build_request_rejects_duplicate_source_identity_before_publication() {
            let source = SourceDocument::from_uri(
                "memory://request/duplicate.sysml",
                String::new(),
                SourceKind::Workspace,
            )
            .expect("valid URI");
            let result = BuildRequest::new(
                vec![source.clone(), source],
                ConstructionStrategy::Sequential,
                EvaluationPolicy::ResolvedOnly,
                "test-contract",
            );

            assert!(result.is_err());
        }

        #[test]
        fn publication_identity_includes_contract_and_phase_inputs() {
            let source = SourceDocument::from_uri(
                "memory://request/identity.sysml",
                String::new(),
                SourceKind::Workspace,
            )
            .expect("valid URI");
            let request = |construction, evaluation, contract| {
                BuildRequest::new(vec![source.clone()], construction, evaluation, contract)
                    .expect("unique source")
                    .identity()
                    .clone()
            };
            let base = request(
                ConstructionStrategy::Sequential,
                EvaluationPolicy::ResolvedOnly,
                "contract-v1",
            );

            assert_ne!(
                base,
                request(
                    ConstructionStrategy::Parallel,
                    EvaluationPolicy::ResolvedOnly,
                    "contract-v1"
                )
            );
            assert_ne!(
                base,
                request(
                    ConstructionStrategy::Sequential,
                    EvaluationPolicy::Evaluate,
                    "contract-v1"
                )
            );
            assert_ne!(
                base,
                request(
                    ConstructionStrategy::Sequential,
                    EvaluationPolicy::ResolvedOnly,
                    "contract-v2"
                )
            );
            assert_eq!(base.semantic_contract_version(), "contract-v1");
            assert_eq!(
                base.construction_strategy(),
                ConstructionStrategy::Sequential
            );
            assert_eq!(base.evaluation_policy(), EvaluationPolicy::ResolvedOnly);
        }

        #[test]
        fn built_publication_keeps_the_identity_precomputed_for_its_request() {
            let request = BuildRequest::new(
                Vec::new(),
                ConstructionStrategy::Parallel,
                EvaluationPolicy::ResolvedOnly,
                "identity-consistency-v1",
            )
            .expect("empty source snapshot is valid");
            let expected = request.identity().clone();

            let model = build(request).expect("empty publication builds");

            assert_eq!(model.publication().identity(), &expected);
        }
    }
}

#[cfg(feature = "legacy-model")]
pub use legacy::{
    build, AuthoredReferenceId, BuildError, BuildRequest, ConstructionStrategy, DebugQueries,
    DiagnosticQueries, EvaluationPolicy, NavigationError, NavigationOutcome, NavigationQueries,
    NavigationReference, NavigationTarget, NodeId, PublicationCompleteness, PublicationIdentity,
    PublicationQueries, PublishedModel, RawStorageIsNotPublic, ReferenceKind, RelationshipKind,
    ResolutionOutcome, ResolutionQueries, SourceDocument, SourceError, SourceKind, TextPosition,
    TextRange,
};
