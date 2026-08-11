//! The only supported consumer facade over Spec42's semantic model implementation.
//!
//! [`PublishedModel`] is opaque. Consumers select a cohesive service and receive typed answers or
//! stream an owner-defined debug projection; they cannot obtain the structural graph, resolver
//! state, fact collections, or query-index storage.

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
#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub sources: Vec<SourceDocument>,
    pub construction: ConstructionStrategy,
    pub evaluation: EvaluationPolicy,
    pub semantic_contract_version: String,
}

impl BuildRequest {
    pub fn evaluated(sources: Vec<SourceDocument>, construction: ConstructionStrategy) -> Self {
        Self {
            sources,
            construction,
            evaluation: EvaluationPolicy::Evaluate,
            semantic_contract_version: "canonical-resolution-v1".to_string(),
        }
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
#[derive(Debug, Clone)]
pub struct PublishedModel {
    inner: sysml_model::SemanticModel,
}

pub fn build(request: BuildRequest) -> Result<PublishedModel, BuildError> {
    let sources = request
        .sources
        .into_iter()
        .map(|source| source.inner)
        .collect();
    let sources = sysml_model::ImmutableSourceSnapshot::new(sources)
        .map_err(|error| BuildError(error.to_string()))?;
    let inner = sysml_model::build_semantic_model(sysml_model::SemanticBuildRequest {
        sources,
        construction: match request.construction {
            ConstructionStrategy::Sequential => sysml_model::ConstructionStrategy::Sequential,
            ConstructionStrategy::Parallel => sysml_model::ConstructionStrategy::Parallel,
        },
        evaluation: match request.evaluation {
            EvaluationPolicy::ResolvedOnly => sysml_model::EvaluationPolicy::ResolvedOnly,
            EvaluationPolicy::Evaluate => sysml_model::EvaluationPolicy::Evaluate,
        },
        configuration: sysml_model::SemanticConfiguration {
            semantic_contract_version: request.semantic_contract_version,
        },
    })
    .map_err(|error| BuildError(error.to_string()))?;
    Ok(PublishedModel { inner })
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
pub struct RawStorageIsNotPublic;
