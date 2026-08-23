#![recursion_limit = "256"]

//! The semantic authority.
//!
//! The only crate that calls the parser, holds parsed trees ([`syntax`]), lowers and resolves,
//! decides diagnostics, computes library closure ([`library`]), and constructs and publishes a
//! model with its lifecycle ([`publication`]). Parser documents, dense IDs, semantic storage,
//! solver state and indexes stay private; its single dependant is the `sysml_query` facade, and it
//! never reads a file — documents come from the source authority it re-exports as [`source`].

use std::fmt;

use source_identity::{ContentDigest, RootDigest, SourceManifest, SourceManifestEntry, SourceRole};

mod action_query;
mod check;
mod definition_usage_query;
mod details;
mod diagnose;
mod diagnostics;
mod diagram_query;
mod evaluate;
mod evaluation;
mod feature_query;
mod index;
mod inspection;
pub mod library;
mod lower;
mod model;
mod namespace_query;
mod pipeline;
pub mod publication;
mod qualified_reference;
mod redefinition_query;
mod requirement_query;
mod resolve;
mod specialization_query;
pub mod syntax;

/// The semantic contract version every resolved publication is recorded under.
///
/// The value is [`sysml_contract::SEMANTIC_CONTRACT_VERSION`] and is defined there: the crate that
/// defines the vocabulary carries the version of that vocabulary, so this authority cannot bump
/// the version its own answers are recorded under without editing the contract. This alias is the
/// `&str` form the build request and the model writer still take; it is retired once those take
/// the typed value.
pub const RESOLVED_CONTRACT: &str = sysml_contract::SEMANTIC_CONTRACT_VERSION.as_str();

/// The source authority, re-exported so the facade reaches it through this crate and the
/// authority chain stays linear: `sysml_source` has exactly one dependant.
pub use sysml_contract::{
    ElementKind, ElementSearch, ElementSource, LibrarySpecializationAnchorBranch, MembershipRole,
    OccurrenceRole, PublicationCompleteness, RequirementConstraintKind, StateSubactionKind,
    TextPosition, TextRange,
};

pub use sysml_source as source;
mod traceability;
mod type_query;
mod verification;

pub use action_query::{
    ActionDerivedFactCollection, ActionDerivedFactKind, ActionDerivedFactOutcome,
    ActionDerivedFactPrerequisite,
};
pub use definition_usage_query::{
    DefinitionUsageDerivedKind, DefinitionUsageDerivedOutcome, DefinitionUsageDerivedPrerequisite,
};
pub use details::{
    ConnectedElement, EffectiveTypeEntry, EffectiveTyping, ElementDetails, ElementDetailsAt,
    InheritedFeature, ReferencedDetails, RelationshipFamily, RelationshipOutcome, ViewSelection,
    ViewSelectionObstacle, ViewSelectionOutcome,
};
pub use diagnostics::{
    Diagnostic, DiagnosticCategory, DiagnosticCode, DiagnosticLocation, DiagnosticOrigin,
    DiagnosticSeverity, PublishedDiagnostics, RelatedLocation,
};
pub use diagram_query::{
    DiagramCompartment, DiagramCompartmentKind, DiagramCompartmentProvenance, DiagramEdge,
    DiagramEdgeKind, DiagramElement, DiagramElementTyping, DiagramEndpointOccurrence,
    DiagramIncompleteReason, DiagramOccurrenceIdentity, DiagramRelationship,
    DiagramRelationshipEndpoint, DiagramRelationshipTarget, DiagramScene, DiagramSemanticReference,
    DiagramStateTransition, DiagramStateTransitionScene, DiagramStateVertex,
    DiagramStateVertexKind, DiagramTransitionFeature, DiagramViewCatalogEntry, DiagramViewKind,
    DiagramViewProjection,
};
pub use evaluation::{
    AnalysisEvaluation, AuthoredUnit, ElementEvaluation, EvaluatedScalar, EvaluationFailure,
    EvaluationPolicy, EvaluationState, ExpectedMeasurement, ResolvedUnit, UnitResolution,
};
pub use feature_query::FeatureDerivedRelationshipCollection;
pub use inspection::{
    AnnotationForm, AuthoredValue, DerivedElementOwner, Documentation,
    ElementDerivedDocumentationCollection, ElementInspection, ElementInspectionAt, ElementModifier,
    ElementRelationship, FeatureDirection, MembershipFacts, MembershipKind, MultiplicityBound,
    MultiplicityFacts, PortionKind, ReferenceAt, RelationshipProvenance, RelationshipTarget,
    SymbolEntry, ValueKind, Visibility, VisibilityProvenance,
};
pub use namespace_query::{NamespaceDerivedElementCollection, NamespaceImportDerivedElement};
pub use qualified_reference::{
    QualifiedElementReference, QualifiedReferenceOutcome, QualifiedReferenceTarget,
};
pub use redefinition_query::{
    RedefinitionCheckKind, RedefinitionCheckOutcome, RedefinitionCheckPrerequisite,
};
pub use requirement_query::{
    requirement_collection_from_kind, RequirementDerivedFactCollection, RequirementDerivedFactKind,
    RequirementDerivedFactOutcome, RequirementDerivedFactPrerequisite,
};
pub use specialization_query::{
    SpecializationCheckKind, SpecializationCheckOutcome, SpecializationCheckPrerequisite,
};
pub use traceability::{
    BindingConnector, BindingConnectorCheckKind, BindingConnectorValidationOutcome,
    BindingConnectorValidationPrerequisite, BindingEndpoint, SatisfyEndpoint, SatisfyPolarity,
    SatisfyRelationship,
};
pub use type_query::{
    Conformance, ConformanceObstacle, EffectiveType, EffectiveTypeOrigin, RequirementUsageTyping,
    SpecializationScope, SubsettingConformance, TypeDerivedElementCollection,
    TypeDerivedFactCollection, TypeDerivedFactKind, TypeDerivedFactOutcome,
    TypeDerivedFactPrerequisite, TypeDerivedFactValue, TypeDerivedRelationshipCollection,
    TypeFeaturingCheckKind, TypeFeaturingCheckOutcome, TypeFeaturingCheckPrerequisite,
    TypeReference,
};
pub use verification::{RequirementVerification, VerificationOutcome, VerificationRequirement};

use model::resolver::ResolvedSemanticModel;
use pipeline::schedule::BuildSchedule;
use pipeline::{CoordinatorError, OwnedSourceRecord, SemanticModelBuildCoordinator};

/// Owner-measured elapsed times for the stable publication barriers.
///
/// Parallel phase durations are wall time rather than summed worker CPU time. Source acquisition
/// and request construction happen outside these measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildMeasurements {
    pub parse: std::time::Duration,
    pub lowering: std::time::Duration,
    pub resolution: std::time::Duration,
}

/// Provenance of an admitted source. Defined by the source authority; one enum everywhere.
pub use sysml_source::SourceKind;

/// One admitted document whose settled semantic dependencies reach a changed document.
///
/// The source role is carried by the semantic publication so hosts never reconstruct provenance
/// from URI layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffectedDocument {
    pub identity: Box<str>,
    pub source: ElementSource,
}

/// What a source was admitted as: text the build parses itself, or a tree already parsed by the
/// syntax authority. Hosts admit handles so the editor's parse and the build's parse are one;
/// stateless callers (benchmarks, fuzzing, tests) may still admit text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourcePayload {
    Text(String),
    Parsed(syntax::ParsedSource),
    /// An admitted document the build parses through the syntax authority's memo.
    Pending(sysml_source::SourceDocument),
}

impl SourcePayload {
    fn byte_len(&self) -> u64 {
        match self {
            SourcePayload::Text(text) => text.len() as u64,
            SourcePayload::Parsed(parsed) => parsed.source().len() as u64,
            SourcePayload::Pending(document) => document.byte_len() as u64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInput {
    identity: Box<str>,
    payload: SourcePayload,
    kind: SourceKind,
    content_digest: ContentDigest,
}

impl SourceInput {
    /// The normalized identity every query and published fact addresses this source by.
    ///
    /// Exposed so a caller can name a document it admitted without re-deriving the identity from
    /// a path; this crate owns that normalization and a second spelling of it would drift.
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn new(identity: impl Into<Box<str>>, content: String, kind: SourceKind) -> Self {
        let content_digest = ContentDigest::of_bytes(content.as_bytes());
        Self {
            identity: identity.into(),
            payload: SourcePayload::Text(content),
            kind,
            content_digest,
        }
    }

    /// Admit a document the build will parse through the syntax authority's memo. The identity
    /// is known now; the tree is fetched (a memo hit, or one parse) when the request is built.
    pub fn pending(identity: impl Into<Box<str>>, document: sysml_source::SourceDocument) -> Self {
        Self {
            identity: identity.into(),
            content_digest: document.digest(),
            kind: document.kind(),
            payload: SourcePayload::Pending(document),
        }
    }

    /// Admit a tree the syntax authority already parsed. The publication identity is the same as
    /// admitting the text: the digest and byte length come from the handle.
    pub fn from_parsed(
        identity: impl Into<Box<str>>,
        parsed: syntax::ParsedSource,
        kind: SourceKind,
    ) -> Self {
        Self {
            identity: identity.into(),
            content_digest: parsed.digest(),
            payload: SourcePayload::Parsed(parsed),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolIdentity(Box<str>);

impl SymbolIdentity {
    /// The canonical, opaque identity encoding used for equality and boundary handles.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub document: Box<str>,
    pub range: TextRange,
    pub role: OccurrenceRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTarget {
    pub symbol: SymbolIdentity,
    pub name: Box<str>,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryOutcome<T> {
    Resolved(T),
    Recovered(T),
    UnsupportedWith(T),
    Unresolved,
    Ambiguous(Box<[T]>),
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenameOutcome {
    Ready {
        symbol: SymbolIdentity,
        name: Box<str>,
        range: TextRange,
        occurrences: Box<[SourceLocation]>,
    },
    Unresolved,
    Ambiguous(Box<[NavigationTarget]>),
    InvalidName,
    Collision(Box<[NavigationTarget]>),
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleMember {
    pub symbol: SymbolIdentity,
    pub name: Box<str>,
    pub kind: ElementKind,
    /// The role this member plays in its owner, where the OMG carries that on the owning
    /// membership rather than on the element; `None` for an ordinary member.
    pub role: Option<MembershipRole>,
    pub qualified_name: Box<str>,
    pub container_name: Option<Box<str>>,
    pub declaring_document: Box<str>,
    pub declaration_range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstructionSchedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationIdentity {
    source_digest: RootDigest,
    semantic_contract_version: Box<str>,
    evaluation_policy: EvaluationPolicy,
    /// The admitted documents the host asked to have reported beyond its workspace, in canonical
    /// order. Part of the identity because it changes what the publication answers.
    reported_documents: Box<[Box<str>]>,
}

impl PublicationIdentity {
    pub fn source_digest(&self) -> &RootDigest {
        &self.source_digest
    }

    pub fn semantic_contract_version(&self) -> &str {
        &self.semantic_contract_version
    }

    pub fn evaluation_policy(&self) -> EvaluationPolicy {
        self.evaluation_policy
    }

    /// Dependency-complete identity of every input that can change published semantic answers.
    pub fn model_digest(&self) -> String {
        let mut digest = blake3::Hasher::new();
        digest.update(b"spec42-publication-model-v1\0");
        digest.update(self.source_digest.to_string().as_bytes());
        digest.update(b"\0");
        digest.update(self.semantic_contract_version.as_bytes());
        digest.update(&[match self.evaluation_policy {
            EvaluationPolicy::Evaluate => 0,
            EvaluationPolicy::Skip => 1,
        }]);
        for document in &self.reported_documents {
            digest.update(&(document.len() as u64).to_le_bytes());
            digest.update(document.as_bytes());
        }
        format!("blake3:{}", digest.finalize().to_hex())
    }

    /// The admitted documents reported beyond the workspace, in canonical order.
    pub fn reported_documents(&self) -> &[Box<str>] {
        &self.reported_documents
    }
}

#[derive(Debug)]
pub struct BuildRequest {
    sources: Vec<SourceInput>,
    schedule: ConstructionSchedule,
    policy: EvaluationPolicy,
    library: Option<std::sync::Arc<LibraryStratum>>,
    reported: Vec<Box<str>>,
    identity: PublicationIdentity,
    /// The memo pending sources are parsed through; absent, they are parsed cold.
    syntax: Option<std::sync::Arc<syntax::SyntaxAuthority>>,
}

fn manifest_entry(source: &SourceInput) -> SourceManifestEntry {
    SourceManifestEntry {
        uri: source.identity.to_string(),
        path_hint: None,
        role: source_role(source.kind),
        content_digest: source.content_digest,
        byte_len: source.payload.byte_len(),
        library_root_slot: None,
        relative_path: None,
    }
}

/// A library that has been parsed and solved once, ready to be reused.
///
/// Building one costs a full publication. Reusing it costs neither the library's parse nor its
/// solve, so a workspace publication pays for the workspace. Share it behind the `Arc` the build
/// request takes; it is immutable and safe to use from any number of concurrent builds.
///
/// Reuse is conditional, not assumed. If a workspace declaration could change what a library
/// reference resolves to -- by declaring a root the library also declares, or one that answers a
/// lookup the library left unsettled -- the settled outcomes are discarded and that publication
/// solves everything from scratch. The result is identical either way; only the cost differs.
#[derive(Debug)]
pub struct LibraryStratum {
    prepared: pipeline::PreparedLibrary,
    manifest_entries: Vec<SourceManifestEntry>,
    identities: std::collections::BTreeSet<Box<str>>,
}

// SAFETY: the same invariant `PublishedResolution` states. A stratum is fully constructed before
// it is shared and exposes only shared reads; its parsed documents own immutable source and AST
// storage whose only interior mutation is `OnceLock`-backed source line indexing. The auto-trait
// solver overflows on the parser's deeply recursive owned AST enum, so the boundary states this
// rather than deriving it.
unsafe impl Send for LibraryStratum {}
unsafe impl Sync for LibraryStratum {}

impl LibraryStratum {
    fn contains(&self, identity: &str) -> bool {
        self.identities.contains(identity)
    }

    /// How many documents this stratum admits.
    pub fn document_count(&self) -> usize {
        self.prepared.documents.len()
    }
}

/// Parses and solves `sources` once so later publications can reuse the result.
///
/// The sources are the library's own; a workspace document admitted here would become part of the
/// stratum and be reused by every publication built against it.
pub fn build_library_stratum(sources: Vec<SourceInput>) -> Result<LibraryStratum, BuildFailure> {
    build_library_stratum_with(sources, None)
}

/// [`build_library_stratum`] parsing pending sources through `syntax`'s memo.
pub fn build_library_stratum_with(
    sources: Vec<SourceInput>,
    syntax: Option<std::sync::Arc<syntax::SyntaxAuthority>>,
) -> Result<LibraryStratum, BuildFailure> {
    let mut request = BuildRequest::new(
        sources,
        ConstructionSchedule::Parallel,
        LIBRARY_STRATUM_CONTRACT,
    )?;
    request.syntax = syntax;
    let manifest_entries = request.sources.iter().map(manifest_entry).collect();
    let identities = request
        .sources
        .iter()
        .map(|source| source.identity.clone())
        .collect();
    let published = build(request)?;
    let prepared = published
        .model
        .prepared_library()
        .map_err(|_| BuildFailure::ConstructionFailed)?;
    Ok(LibraryStratum {
        prepared,
        manifest_entries,
        identities,
    })
}

/// The contract version a stratum build is recorded under.
///
/// Never reaches a publication identity: `BuildRequest::with_library` recomputes the digest from
/// the merged manifest and keeps the caller's own contract version.
const LIBRARY_STRATUM_CONTRACT: &str = "library-stratum";

impl BuildRequest {
    pub fn new(
        mut sources: Vec<SourceInput>,
        schedule: ConstructionSchedule,
        semantic_contract_version: impl Into<Box<str>>,
    ) -> Result<Self, BuildFailure> {
        sources.sort_unstable_by(|left, right| left.identity.cmp(&right.identity));
        if sources
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(BuildFailure::DuplicateSourceIdentity);
        }
        let semantic_contract_version = semantic_contract_version.into();
        let entries = sources.iter().map(manifest_entry).collect();
        let source_digest = SourceManifest::new(entries, Vec::new()).root_digest();
        Ok(Self {
            sources,
            schedule,
            policy: EvaluationPolicy::default(),
            library: None,
            reported: Vec::new(),
            identity: PublicationIdentity {
                source_digest,
                semantic_contract_version,
                evaluation_policy: EvaluationPolicy::default(),
                reported_documents: Box::default(),
            },
            syntax: None,
        })
    }

    /// Parse pending sources through `syntax`'s memo rather than cold.
    pub fn with_syntax(mut self, syntax: std::sync::Arc<syntax::SyntaxAuthority>) -> Self {
        self.syntax = Some(syntax);
        self
    }

    /// Builds against a library that has already been parsed and solved.
    ///
    /// `sources` carries only the workspace documents; the library's own documents come from the
    /// stratum and are admitted ahead of them. The publication's identity still commits every
    /// admitted source, library included, so a workspace built against two different library
    /// versions can never share an identity.
    pub fn with_library(
        sources: Vec<SourceInput>,
        schedule: ConstructionSchedule,
        semantic_contract_version: impl Into<Box<str>>,
        library: std::sync::Arc<LibraryStratum>,
    ) -> Result<Self, BuildFailure> {
        let mut request = Self::new(sources, schedule, semantic_contract_version)?;
        if request
            .sources
            .iter()
            .any(|source| library.contains(source.identity.as_ref()))
        {
            return Err(BuildFailure::DuplicateSourceIdentity);
        }
        let mut entries = library.manifest_entries.clone();
        entries.extend(request.sources.iter().map(manifest_entry));
        request.identity.source_digest = SourceManifest::new(entries, Vec::new()).root_digest();
        request.library = Some(library);
        Ok(request)
    }

    /// Also reports diagnostics for these admitted documents, beyond the workspace-authored ones.
    ///
    /// A publication reports its workspace by default: a workspace does not inherit its library's
    /// diagnostics, and deriving every admitted document would make the barrier cost the whole
    /// library on every rebuild. That default is about *provenance*, which is not the same
    /// question as which documents are an authoring surface -- an editor with a library file open
    /// is authoring it, and only the host knows that.
    ///
    /// Naming a document here is how the host says so. It is a build input rather than a query
    /// option because a diagnostic is settled before the publication becomes visible, and it is
    /// part of the publication's identity because two publications that report different documents
    /// are observably different answers.
    ///
    /// An identity that names no admitted document is ignored: the host asked about something this
    /// publication does not contain, which the empty answer already says.
    pub fn reporting(mut self, documents: impl IntoIterator<Item = Box<str>>) -> Self {
        self.reported = documents.into_iter().collect();
        self.reported.sort_unstable();
        self.reported.dedup();
        self.identity.reported_documents = self.reported.clone().into_boxed_slice();
        self
    }

    /// Sets whether this build evaluates constant expressions.
    ///
    /// Skipping publishes a coherent resolved model whose elements report
    /// [`EvaluationState::NotRun`], which is a different answer from an element having no
    /// expression or having one that could not be folded.
    pub fn with_evaluation_policy(mut self, policy: EvaluationPolicy) -> Self {
        self.policy = policy;
        self.identity.evaluation_policy = policy;
        self
    }

    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildFailure {
    DuplicateSourceIdentity,
    ConstructionFailed,
}

impl fmt::Display for BuildFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for BuildFailure {}

/// An opaque, immutable resolved publication.
///
/// Publications are shared by reference; cloning the semantic owner is intentionally impossible.
///
/// ```compile_fail
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<sysml_resolution::PublishedResolution>();
/// ```
///
/// Dense storage identities and indexes are private implementation details.
///
/// ```compile_fail
/// use sysml_resolution::{DeclarationId, ResolutionResults, SemanticModelStorage};
/// ```
#[derive(Debug)]
pub struct PublishedResolution {
    identity: PublicationIdentity,
    model: ResolvedSemanticModel,
}

// SAFETY: a publication is fully constructed before this type is created and exposes only shared
// queries. Its parser documents own immutable source/AST storage; the only interior mutation is
// `OnceLock`-backed source line indexing, whose implementation is thread-safe. The parser AST is a
// deeply recursive owned enum for which rustc's auto-trait solver overflows in downstream async
// hosts, so the publication boundary states the invariant explicitly.
unsafe impl Send for PublishedResolution {}
unsafe impl Sync for PublishedResolution {}

pub fn build(request: BuildRequest) -> Result<PublishedResolution, BuildFailure> {
    build_measured(request).map(|(publication, _)| publication)
}

/// Builds one coherent publication and returns measurements captured by its phase owner.
pub fn build_measured(
    request: BuildRequest,
) -> Result<(PublishedResolution, BuildMeasurements), BuildFailure> {
    let schedule = match request.schedule {
        ConstructionSchedule::Sequential => BuildSchedule::Sequential,
        ConstructionSchedule::Parallel => BuildSchedule::Parallel,
    };
    let syntax = request.syntax;
    let sources = request
        .sources
        .into_iter()
        .map(|source| OwnedSourceRecord {
            identity: source.identity,
            role: source_role(source.kind),
            payload: source.payload,
            syntax: syntax.clone(),
        })
        .collect();
    let (model, measurements) = SemanticModelBuildCoordinator::build_measured_with_library(
        sources,
        schedule,
        request.policy,
        request.library.as_deref().map(|library| &library.prepared),
        &request.reported,
    )
    .map_err(|error| match error {
        CoordinatorError::DuplicateSourceIdentity => BuildFailure::DuplicateSourceIdentity,
        CoordinatorError::ConstructionFailed => BuildFailure::ConstructionFailed,
    })?;
    Ok((
        PublishedResolution {
            identity: request.identity,
            model,
        },
        BuildMeasurements {
            parse: measurements.parse,
            lowering: measurements.lowering,
            resolution: measurements.resolution,
        },
    ))
}

impl PublishedResolution {
    pub fn identity(&self) -> &PublicationIdentity {
        &self.identity
    }

    /// Documents transitively dependent on `changed_document` through settled imports and alias
    /// bindings. Recovery and unsupported publications are exposed by the typed outcome; callers
    /// may then deliberately over-invalidate without pretending the dependency graph was settled.
    pub fn affected_documents(
        &self,
        changed_document: &str,
    ) -> QueryOutcome<Box<[AffectedDocument]>> {
        self.model.affected_documents(changed_document)
    }

    pub fn debug(&self) -> DebugQueries<'_> {
        DebugQueries {
            identity: &self.identity,
            model: &self.model,
        }
    }

    pub fn completeness(&self) -> PublicationCompleteness {
        self.model.completeness()
    }

    /// The resolution-owned diagnostics this publication settled, in canonical order.
    ///
    /// These are facts, not rendered text: the canonical S-expression projection is one adapter
    /// over exactly these values, so no consumer recovers a code, severity, or outcome from
    /// presentation output or re-decides a rule.
    ///
    /// This is the complete production validation surface; see [`DiagnosticCode`] for every
    /// family it decides.
    pub fn diagnostics(&self) -> PublishedDiagnostics {
        self.model.published_diagnostics()
    }

    /// The diagnostics of one admitted document, read from the publication's own index.
    ///
    /// A slice of the settled sequence, so the cost is proportional to what is returned rather
    /// than to the model. Repeating the query, or asking about documents in any order, returns
    /// the same values: nothing here computes.
    ///
    /// A document this publication did not admit answers with no diagnostics and the same
    /// completeness, which is why the completeness travels with them.
    pub fn document_diagnostics(&self, document: &str) -> PublishedDiagnostics {
        self.model.published_document_diagnostics(document)
    }

    pub fn target_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<NavigationTarget> {
        self.model.target_at(document, position)
    }

    pub fn references(
        &self,
        symbol: &SymbolIdentity,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        self.model.references(symbol, include_declaration)
    }

    pub fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        self.model.prepare_rename(document, position, new_name)
    }

    pub fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<Box<[VisibleMember]>> {
        self.model.visible_members(document, position, qualifier)
    }

    /// The settled evaluation of one element: value, state, authored units and required
    /// measurement, from facts this publication fixed before it became visible.
    pub fn evaluate(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementEvaluation> {
        self.model.evaluate(symbol)
    }

    /// Everything this publication knows about one element.
    pub fn inspect(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementInspection> {
        self.model.inspect(symbol)
    }

    /// The exact derived `Element::owner` fact from the canonical ownership structure.
    pub fn derived_element_owner(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<DerivedElementOwner> {
        self.model.derived_element_owner(symbol)
    }

    /// One exact derived `Element` documentation collection from canonical documentation facts.
    pub fn element_derived_documentation(
        &self,
        symbol: &SymbolIdentity,
        collection: ElementDerivedDocumentationCollection,
    ) -> QueryOutcome<Box<[Documentation]>> {
        self.model.element_derived_documentation(symbol, collection)
    }

    /// The element whose declaration encloses `position`, and the element a reference there
    /// resolves to.
    ///
    /// Both, because they are usually different: the cursor sits inside one element's declaration
    /// while pointing at a reference to another, and an inspector needs to show each.
    pub fn inspect_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ElementInspectionAt> {
        self.model.inspect_at(document, position)
    }

    /// Everything this publication settled about one element, as one coherent answer.
    ///
    /// The cohesive form of [`PublishedResolution::inspect`]: the same inspection, plus the
    /// relationship families, effective typing, inherited features, metadata bindings, incoming
    /// and outgoing relationships, and both evaluation channels. Assembled from the same settled
    /// facts, so the two can never disagree.
    pub fn element_details(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementDetails> {
        self.model.element_details(symbol)
    }

    /// The element whose declaration encloses `position` and the element a reference there
    /// resolves to, both in full detail.
    pub fn element_details_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ElementDetailsAt> {
        self.model.element_details_at(document, position)
    }

    /// Every element declared in one document, in source order.
    pub fn document_symbols(&self, document: &str) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.document_symbols(document)
    }

    /// Elements of `search.kind` authored in `search.source`, in canonical source order.
    pub fn search_elements(&self, search: ElementSearch) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.search_elements(search)
    }

    /// Workspace-authored satisfy statements in canonical declaration order.
    pub fn satisfy_relationships(&self) -> QueryOutcome<Box<[SatisfyRelationship]>> {
        self.model.satisfy_relationships()
    }

    /// One exact derived relationship collection for a lowered Feature.
    ///
    /// The collection is a projection of canonical authored/implied relationships; it does not
    /// reconstruct a relationship from names or source text.
    pub fn feature_derived_relationships(
        &self,
        symbol: &SymbolIdentity,
        collection: FeatureDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        self.model.feature_derived_relationships(symbol, collection)
    }

    /// One exact Type relationship collection or operand projection from the canonical
    /// relationship store.
    pub fn type_derived_relationships(
        &self,
        symbol: &SymbolIdentity,
        collection: TypeDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        self.model.type_derived_relationships(symbol, collection)
    }

    /// One exact Type element-valued derivation over canonical declaration ownership and
    /// membership facts. The result never creates a public Membership relationship identity.
    pub fn type_derived_elements(
        &self,
        symbol: &SymbolIdentity,
        collection: TypeDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.type_derived_elements(symbol, collection)
    }

    /// One exact Type derivation with an explicit unavailable-fact outcome where no canonical
    /// result owner exists yet.
    pub fn type_derived_fact(
        &self,
        symbol: &SymbolIdentity,
        collection: TypeDerivedFactCollection,
    ) -> QueryOutcome<TypeDerivedFactOutcome> {
        self.model.type_derived_fact(symbol, collection)
    }

    /// One exact Systems::DefinitionAndUsage derivation selected by the manifest-owned closed
    /// kind. Direct owner/member projections are resolved from canonical facts; inherited,
    /// variant, and time-variation predicates retain a typed unavailable-fact outcome.
    pub fn definition_usage_derived(
        &self,
        symbol: &SymbolIdentity,
        kind: DefinitionUsageDerivedKind,
    ) -> QueryOutcome<DefinitionUsageDerivedOutcome> {
        self.model.definition_usage_derived(symbol, kind)
    }

    pub fn action_derived_fact(
        &self,
        symbol: &SymbolIdentity,
        collection: ActionDerivedFactCollection,
    ) -> QueryOutcome<ActionDerivedFactOutcome> {
        self.model.action_derived_fact(symbol, collection)
    }

    /// One exact Systems::Requirements derivation over the publication's canonical membership
    /// roles or documentation records.
    pub fn requirement_derived_fact(
        &self,
        symbol: &SymbolIdentity,
        collection: RequirementDerivedFactCollection,
    ) -> QueryOutcome<RequirementDerivedFactOutcome> {
        self.model.requirement_derived_fact(symbol, collection)
    }

    /// The rule-scoped outcome for one closed exact TypeFeaturing check, derived only from the
    /// canonical FeatureMembership and TypeFeaturing publication.
    pub fn type_featuring_check(
        &self,
        symbol: &SymbolIdentity,
        rule: TypeFeaturingCheckKind,
    ) -> QueryOutcome<TypeFeaturingCheckOutcome> {
        self.model.type_featuring_check(symbol, rule)
    }

    /// The manifest-scoped outcome for an exact redefinition check. This query consumes only
    /// resolver-owned relationship facts and published applicability facts; unavailable
    /// prerequisites remain typed rather than being reconstructed from source or display names.
    pub fn redefinition_check(
        &self,
        rule: RedefinitionCheckKind,
    ) -> QueryOutcome<RedefinitionCheckOutcome> {
        self.model.redefinition_check(rule)
    }

    /// The manifest-scoped outcome for one complete specialization check whose exact predicate
    /// needs more than a generic graph edge.  Missing role facts are published as typed outcomes.
    pub fn specialization_check(
        &self,
        rule: SpecializationCheckKind,
    ) -> QueryOutcome<SpecializationCheckOutcome> {
        self.model.specialization_check(rule)
    }

    /// One exact Namespace element-valued derivation over canonical declaration ownership and
    /// membership facts. Unsupported and incomplete outcomes remain explicit.
    pub fn namespace_derived_elements(
        &self,
        symbol: &SymbolIdentity,
        collection: NamespaceDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.namespace_derived_elements(symbol, collection)
    }

    /// Exact `NamespaceImport::importedElement` projections for the direct anonymous imports a
    /// Namespace owns. Each result carries the canonical import identity and target outcome.
    pub fn namespace_import_derived_elements(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[NamespaceImportDerivedElement]>> {
        self.model.namespace_import_derived_elements(symbol)
    }

    /// Workspace-authored binding connectors with their paired canonical endpoints.
    pub fn binding_connectors(&self) -> QueryOutcome<Box<[BindingConnector]>> {
        self.model.binding_connectors()
    }

    /// The explicit applicability outcome for a closed named binding-connector validation.
    pub fn binding_connector_validation(
        &self,
        rule: BindingConnectorCheckKind,
    ) -> QueryOutcome<BindingConnectorValidationOutcome> {
        self.model.binding_connector_validation(rule)
    }

    /// Workspace-authored requirement-verification memberships in canonical declaration order.
    pub fn requirement_verifications(&self) -> QueryOutcome<Box<[RequirementVerification]>> {
        self.model.requirement_verifications()
    }

    /// Effective features, direct first and inherited nearest-first with name shadowing.
    pub fn effective_features(&self, symbol: &SymbolIdentity) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.effective_features(symbol)
    }

    /// Applies every owned and inherited condition of `view` to one candidate element.
    pub fn view_selection(
        &self,
        view: &SymbolIdentity,
        candidate: &SymbolIdentity,
    ) -> QueryOutcome<ViewSelection> {
        self.model.view_selection(view, candidate)
    }

    /// The types a feature declares.
    pub fn direct_types(&self, symbol: &SymbolIdentity) -> QueryOutcome<Box<[TypeReference]>> {
        self.model.direct_types(symbol)
    }

    pub fn requirement_usage_typing(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<RequirementUsageTyping> {
        self.model.requirement_usage_typing(symbol)
    }

    /// The types a feature has, directly or inherited along its subsetting/redefinition chain.
    pub fn effective_types(&self, symbol: &SymbolIdentity) -> QueryOutcome<Box<[EffectiveType]>> {
        self.model.effective_types(symbol)
    }

    /// The canonical standard-library target used to satisfy
    /// `checkPartDefinitionSpecialization`.
    ///
    /// This is the semantic owner's typed anchor outcome, not a lookup reconstructed from a
    /// display name. A missing library is `Unresolved`; competing library declarations are
    /// returned as `Ambiguous` candidates.
    pub fn part_definition_specialization_anchor(&self) -> QueryOutcome<SymbolIdentity> {
        self.model.part_definition_specialization_anchor()
    }

    /// The canonical anchor outcome for one generated unconditional library-specialization rule.
    pub fn library_specialization_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolIdentity> {
        self.model.library_specialization_anchor(rule_id)
    }

    /// The canonical anchor outcome for one typed branch of a generated conditional
    /// specialization rule. [`LibrarySpecializationAnchorBranch::Default`] is the compatible
    /// single-anchor view used by [`Self::library_specialization_anchor`].
    pub fn library_specialization_anchor_branch(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> QueryOutcome<SymbolIdentity> {
        self.model
            .library_specialization_anchor_branch(rule_id, branch)
    }

    /// The canonical anchor outcome for any generated exact library rule.
    ///
    /// Unlike [`Self::library_specialization_anchor`], this includes generated
    /// `redefinesFromLibrary` contracts. The stable manifest rule ID is the only selector;
    /// callers cannot recover a rule from a display name or metaclass spelling.
    pub fn library_rule_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolIdentity> {
        self.model.library_rule_anchor(rule_id)
    }

    /// Whether a generated exact `redefinesFromLibrary` rule has a lowered source projection.
    ///
    /// An exact manifest rule that the current parser cannot represent is `Unsupported`, rather
    /// than an invented relationship or a misleading successful no-op.
    pub fn library_redefinition_applicability(&self, rule_id: &str) -> QueryOutcome<()> {
        self.model.library_redefinition_applicability(rule_id)
    }

    /// The supertypes one specialization edge away.
    pub fn direct_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.direct_supertypes(symbol, scope)
    }

    /// Every supertype, reflexively including `symbol` itself.
    pub fn all_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.all_supertypes(symbol, scope)
    }

    /// The declarations one specialization edge below `symbol`.
    pub fn direct_subtypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.direct_subtypes(symbol, scope)
    }

    /// The type that features `symbol`, if any.
    pub fn featuring_type(&self, symbol: &SymbolIdentity) -> QueryOutcome<Option<SymbolIdentity>> {
        self.model.featuring_type(symbol)
    }

    /// Every effective TypeFeaturing target, retaining whether it was authored or implied.
    ///
    /// A variable FeatureMembership without a canonical `snapshots` prerequisite is explicitly
    /// unsupported rather than treated as an unfeatured ordinary member.
    pub fn featuring_types(&self, symbol: &SymbolIdentity) -> QueryOutcome<Box<[TypeReference]>> {
        self.model.featuring_types(symbol)
    }

    /// Whether `specific` conforms to `general` (KerML §8.4.3.2), reflexively and transitively.
    pub fn conforms_to(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Conformance> {
        self.model.conforms_to(specific, general, scope)
    }

    /// Whether the specific feature's types conform to the general feature's (KerML §7.4.12).
    pub fn feature_typing_conforms(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
    ) -> QueryOutcome<Conformance> {
        self.model.feature_typing_conforms(specific, general)
    }

    /// Both halves of the subsetting rule (KerML §8.4.3.4), reported separately.
    pub fn subsetting_conforms(
        &self,
        subsetting: &SymbolIdentity,
        subsetted: &SymbolIdentity,
    ) -> QueryOutcome<SubsettingConformance> {
        self.model.subsetting_conforms(subsetting, subsetted)
    }
}

pub struct DebugQueries<'a> {
    identity: &'a PublicationIdentity,
    model: &'a ResolvedSemanticModel,
}

impl DebugQueries<'_> {
    pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_semantic_sexpr(
            &self.identity.source_digest,
            &self.identity.semantic_contract_version,
            output,
        )
    }

    pub fn write_diagnostics_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_diagnostics_sexpr(output)
    }

    pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_navigation_sexpr(output)
    }

    pub fn write_types_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.write_types_sexpr(output)
    }
}

fn source_role(kind: SourceKind) -> SourceRole {
    match kind {
        SourceKind::Workspace => SourceRole::Workspace,
        SourceKind::StandardLibrary => SourceRole::StandardLibrary,
        SourceKind::Library => SourceRole::Library,
        SourceKind::External => SourceRole::External,
    }
}

/// Raw semantic storage is deliberately inaccessible.
///
/// ```compile_fail
/// use sysml_resolution::{DeclarationId, ResolutionResults, SemanticModelStorage};
/// ```
///
/// ```compile_fail
/// fn require_clone<T: Clone>() {}
/// require_clone::<sysml_resolution::BuildRequest>();
/// require_clone::<sysml_resolution::PublishedResolution>();
/// ```
pub struct RawStorageIsNotPublic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_schedule_does_not_change_semantic_publication_identity() {
        let sequential =
            BuildRequest::new(Vec::new(), ConstructionSchedule::Sequential, "contract-v1").unwrap();
        let parallel =
            BuildRequest::new(Vec::new(), ConstructionSchedule::Parallel, "contract-v1").unwrap();

        assert_eq!(sequential.identity(), parallel.identity());
    }

    fn semantic_sexpr_for(source: &str) -> String {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        let published = build(request).unwrap();
        let mut output = String::new();
        published.debug().write_semantic_sexpr(&mut output).unwrap();
        output
    }

    /// Like `semantic_sexpr_for`, but renders the per-document diagnostics sexpr (which carries
    /// the actual `unsupported_*_definition_member` diagnostic codes) instead of the semantic
    /// model sexpr (which only carries the coarser `(completeness unsupported-syntax)` summary
    /// flag) -- needed for tests asserting a *specific* diagnostic code is present, not merely
    /// that publication completeness is degraded.
    fn diagnostics_sexpr_for(source: &str) -> String {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        let published = build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        output
    }

    /// The typed diagnostics of one single-document publication.
    fn diagnostics_for(source: &str) -> Vec<Diagnostic> {
        published_for(source).diagnostics().diagnostics.into_vec()
    }

    fn published_for(source: &str) -> PublishedResolution {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        build(request).unwrap()
    }

    #[test]
    fn state_transition_scene_owns_vertices_and_composed_transitions() {
        let request = BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://standard-views.sysml",
                    "standard library package StandardViewDefinitions { view def StateTransitionView; }".to_owned(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://timer.sysml",
                    concat!(
                        "package Timer { import StandardViewDefinitions::*; item def StartPressed; ",
                        "state def Machine { entry; then idle; state idle; state running; ",
                        "transition start first idle accept StartPressed then running; } ",
                        "view stateView : StateTransitionView { expose Machine; } }",
                    ).to_owned(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        ).unwrap();
        let published = build(request).unwrap();
        let catalog = match published.diagram_view_catalog() {
            QueryOutcome::Resolved(catalog) => catalog,
            other => panic!("expected diagram catalog, got {other:?}"),
        };
        let view = catalog
            .iter()
            .find(|view| view.kind == DiagramViewKind::StateTransition)
            .unwrap();
        let projection = match published.diagram_view(&view.semantic_id) {
            QueryOutcome::Resolved(projection) => projection,
            other => panic!("expected state scene, got {other:?}"),
        };
        let DiagramScene::StateTransition(scene) = projection.scene else {
            panic!("expected typed State Transition scene");
        };
        assert_eq!(
            scene
                .vertices
                .iter()
                .filter(|vertex| vertex.kind == DiagramStateVertexKind::Initial)
                .count(),
            1
        );
        assert_eq!(
            scene
                .vertices
                .iter()
                .filter(|vertex| vertex.kind == DiagramStateVertexKind::State)
                .count(),
            2
        );
        assert_eq!(scene.transitions.len(), 2);
        assert!(scene.transitions.iter().any(|transition| matches!(
            &transition.trigger,
            DiagramTransitionFeature::Resolved { label, .. } if label.as_ref() == "StartPressed"
        )));
        assert!(!scene
            .vertices
            .iter()
            .any(|vertex| vertex.label.as_ref() == "start"));
    }

    #[test]
    fn diagram_projection_preserves_resolved_facts_from_unsupported_inspections() {
        let request = BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://standard-views.sysml",
                    concat!(
                        "standard library package StandardViewDefinitions { view def GeneralView; ",
                        "view def StateTransitionView; } standard library package SysML { ",
                        "metaclass PartUsage; }",
                    ).to_owned(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    concat!(
                        "package Model { import StandardViewDefinitions::*; ",
                        "part def Board; part def Assembly { part pcb : Board; } part root : Assembly; ",
                        "state def Machine { state idle; state running; transition start first idle then running; } ",
                        "view structure : GeneralView { expose root; filter @SysML::PartUsage; } ",
                        "view behavior : StateTransitionView { expose Machine; } }",
                    ).to_owned(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        ).unwrap();
        let published = build(request).unwrap();
        let catalog = match published.diagram_view_catalog() {
            QueryOutcome::Resolved(catalog) | QueryOutcome::UnsupportedWith(catalog) => catalog,
            other => panic!("expected diagram catalog, got {other:?}"),
        };
        let structure = catalog
            .iter()
            .find(|view| view.kind == DiagramViewKind::General)
            .unwrap();
        let projection = match published.diagram_view(&structure.semantic_id) {
            QueryOutcome::Resolved(projection) => projection,
            other => panic!("expected General View projection, got {other:?}"),
        };
        let root = projection
            .elements
            .iter()
            .find(|element| element.name.as_deref() == Some("root"))
            .unwrap();
        assert!(matches!(root.typing, DiagramElementTyping::Resolved(_)));
        assert!(projection.relationships.iter().any(|relationship| {
            relationship.source == root.occurrence_id
                && relationship.source_semantic_id == root.semantic_id
                && relationship.kind.as_ref() == "featureTyping"
        }));

        let behavior = catalog
            .iter()
            .find(|view| view.kind == DiagramViewKind::StateTransition)
            .unwrap();
        let projection = match published.diagram_view(&behavior.semantic_id) {
            QueryOutcome::Resolved(projection) => projection,
            other => panic!("expected State Transition projection, got {other:?}"),
        };
        let DiagramScene::StateTransition(scene) = projection.scene else {
            panic!("expected State Transition scene");
        };
        assert_eq!(scene.transitions.len(), 1);
    }

    #[test]
    fn diagram_projection_keeps_inherited_features_distinct_in_each_usage_context() {
        let request = BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://standard-views.sysml",
                    "standard library package StandardViewDefinitions { view def GeneralView; }"
                        .to_owned(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://model.sysml",
                    concat!(
                        "package Model { import StandardViewDefinitions::*; ",
                        "part def Board; part def Module { part pcb : Board; part spare : Board; connection wire connect pcb to spare; } ",
                        "part def Assembly { part left : Module; part right : Module; } ",
                        "part root : Assembly; view structure : GeneralView { expose root; } }",
                    )
                    .to_owned(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        let published = build(request).unwrap();
        let catalog = match published.diagram_view_catalog() {
            QueryOutcome::Resolved(catalog) | QueryOutcome::UnsupportedWith(catalog) => catalog,
            other => panic!("expected diagram catalog, got {other:?}"),
        };
        let view = catalog
            .iter()
            .find(|view| view.kind == DiagramViewKind::General)
            .unwrap();
        let projection = match published.diagram_view(&view.semantic_id) {
            QueryOutcome::Resolved(projection) => projection,
            other => panic!("expected General View projection, got {other:?}"),
        };

        let pcbs = projection
            .elements
            .iter()
            .filter(|element| element.name.as_deref() == Some("pcb"))
            .collect::<Vec<_>>();
        assert_eq!(
            pcbs.len(),
            2,
            "one declaration must occur under both module usages"
        );
        assert_eq!(pcbs[0].semantic_id, pcbs[1].semantic_id);
        assert_ne!(pcbs[0].occurrence_id, pcbs[1].occurrence_id);
        assert_ne!(pcbs[0].owner, pcbs[1].owner);
        assert!(pcbs
            .iter()
            .all(|pcb| pcb.occurrence_id.semantic_path.len() == 3));

        let connectors = projection
            .edges
            .iter()
            .filter(|edge| edge.kind == DiagramEdgeKind::Connector)
            .collect::<Vec<_>>();
        assert_eq!(
            connectors.len(),
            2,
            "the inherited connector occurs in both modules"
        );
        assert_eq!(
            connectors[0].source_semantic_id,
            connectors[1].source_semantic_id
        );
        assert_eq!(
            connectors[0].target_semantic_id,
            connectors[1].target_semantic_id
        );
        assert_ne!(connectors[0].source, connectors[1].source);
        assert_ne!(connectors[0].target, connectors[1].target);
    }

    /// Reporting a document changes the answer, so it changes the publication's identity.
    #[test]
    fn the_reported_document_set_is_part_of_the_publication_identity() {
        let request = || {
            BuildRequest::new(
                vec![SourceInput::new(
                    "memory://workspace.sysml",
                    "package W { part w; }".to_string(),
                    SourceKind::Workspace,
                )],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap()
        };
        let plain = request();
        let reporting = request().reporting([Box::from("memory://lib.sysml")]);
        assert_ne!(plain.identity(), reporting.identity());
        assert_eq!(
            reporting.identity().reported_documents(),
            [Box::<str>::from("memory://lib.sysml")]
        );
    }

    #[test]
    fn a_document_query_answers_from_the_publication_index_and_repeats_identically() {
        let published = published_for("package P { part def A; part def A; part b; }");
        let first = published.document_diagnostics("memory://test.sysml");
        let second = published.document_diagnostics("memory://test.sysml");
        assert_eq!(first, second, "a repeated query returns identical values");
        assert_eq!(
            first.diagnostics.as_ref(),
            published.diagnostics().diagnostics.as_ref(),
            "the document slice is the publication's own sequence"
        );
        let absent = published.document_diagnostics("memory://absent.sysml");
        assert!(absent.diagnostics.is_empty());
        assert_eq!(
            absent.completeness, first.completeness,
            "completeness travels with the answer even when there is nothing to report"
        );
    }

    #[test]
    fn every_diagnostic_carries_an_owner_produced_message() {
        let diagnostics = diagnostics_for(
            "package P { part def A; part def A; part b; port def PD; \
             part def D { port p : PD; } }",
        );
        assert!(!diagnostics.is_empty());
        for diagnostic in &diagnostics {
            assert!(
                !diagnostic.message.trim().is_empty(),
                "empty message: {diagnostic:#?}"
            );
            assert!(
                !diagnostic.category().as_str().is_empty(),
                "diagnostic has no typed category: {diagnostic:#?}"
            );
        }
    }

    /// A `first X then Y;` succession whose `then` target is not declared anywhere in the model
    /// must stay an explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn first_then_succession_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for(
            "package P { action def ExecuteMission { action validateRoute; first validateRoute then missingAction; } }",
        );
        assert!(
            sexpr.contains("(kind succession)"),
            "expected a succession reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable `then` target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// An `entry action X;` binding whose target is not declared anywhere in the model must stay
    /// an explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn entry_action_binding_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { state def S { entry action missingAction; } }");
        assert!(
            sexpr.contains("(kind entryActionBinding)"),
            "expected an entryActionBinding reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable entry action target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A `then <target>;` initial-state marker whose target is not declared anywhere in the model
    /// must stay an explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn then_initial_state_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { state def S { then missingState; } }");
        assert!(
            sexpr.contains("(kind initialState)"),
            "expected an initialState reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable `then` target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A `transition ... first X then Y;` body element's `source`/`target` operands must each
    /// resolve to their sibling state declarations, not fall through to
    /// `unsupported_state_definition_member`.
    #[test]
    fn transition_source_and_target_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; state on; transition first off then on; } }",
        );
        assert!(
            sexpr.contains("(kind transitionSource)"),
            "expected a transitionSource relationship kind, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind transitionTarget)"),
            "expected a transitionTarget relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected both transition ends to resolve to their sibling state declarations, got: {sexpr}"
        );
    }

    /// A transition whose `source`/`target` are not declared anywhere in the model must stay an
    /// explicit unresolved reference fact, not a fabricated or guessed target.
    #[test]
    fn transition_source_and_target_unresolvable_stay_unresolved() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { transition first missingOff then missingOn; } }",
        );
        assert!(
            sexpr.contains("(kind transitionSource)") && sexpr.contains("(kind transitionTarget)"),
            "expected transitionSource/transitionTarget references to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable transition ends to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// A transition `if <guard>;` boolean expression with literal comparison operands must
    /// evaluate to a constant `Boolean` through the exact same `classify_expression`/
    /// `EvalNode` machinery a `constraint`/`calc` body uses (see `9f63c5a4` and earlier
    /// expression/evaluation slices), not a separate transition-specific evaluator.
    #[test]
    fn transition_guard_with_literal_operands_evaluates() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; state on; transition first off if 1 < 2 then on; } }",
        );
        assert!(
            sexpr.contains("(value (boolean true))") || sexpr.contains("(boolean true)"),
            "expected the literal guard `1 < 2` to fold to a constant true, got: {sexpr}"
        );
    }

    /// A transition guard referencing an operand with no known constant value must stay
    /// non-constant, not fabricate a truth value.
    #[test]
    fn transition_guard_with_unresolvable_operand_stays_non_constant() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; state on; transition first off if missingFlag then on; } }",
        );
        assert!(
            sexpr.contains("(kind expressionOperand)"),
            "expected the guard's feature reference to be lowered as an expressionOperand, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(value (boolean true))") && !sexpr.contains("(value (boolean false))"),
            "did not expect an unresolvable guard operand to fold to a concrete boolean, got: {sexpr}"
        );
    }

    /// A bare `then <target>;` continuation (`ThenTarget::Feature`) referencing an
    /// already-declared sibling action must resolve as a `thenTarget` reference sourced at the
    /// enclosing action, not fall through to `unsupported_action_definition_member`.
    #[test]
    fn then_target_feature_resolves() {
        let sexpr = semantic_sexpr_for("package P { action def A { action x; then x; } }");
        assert!(
            sexpr.contains("(kind thenTarget)"),
            "expected a thenTarget relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the `then` target to resolve to its sibling action, got: {sexpr}"
        );
    }

    /// A standalone `action <name> send via <source> to <target>;` action-usage shorthand (an
    /// `ActionUsage` with `send`/`via`/`to` all set on the usage itself, distinct from the
    /// `then send ...;` continuation form blocked by planning/UPSTREAM_PARSER_GAPS.md Gap 30) must resolve
    /// its `via`/`to` operands, mirroring satisfy/allocate/bind's two-operand pattern via
    /// `lower_satisfy_operand`.
    #[test]
    fn send_action_usage_via_and_to_targets_resolve() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action aa; action b; action snd2 send via aa to b; } }",
        );
        assert!(
            sexpr.contains("(kind sendTarget)"),
            "expected a sendTarget reference for the `to b` clause, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind acceptVia)"),
            "expected an acceptVia reference for the `via aa` clause, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected both the send usage's `via` and `to` targets to resolve, got: {sexpr}"
        );
    }

    /// A bare `require;`-less-constraint shorthand (`has_constraint_keyword == false`, e.g.
    /// `require someExistingConstraint;`) references an existing constraint rather than declaring
    /// one. Upstream now carries that role on `RequireConstraint::target`, but nothing lowers it
    /// yet (planning/UPSTREAM_PARSER_GAPS.md, "Typed upstream, not yet lowered here"), so it must
    /// stay an explicit unsupported diagnostic rather than being silently dropped or guessed at.
    #[test]
    fn require_shorthand_reference_without_constraint_keyword_stays_unsupported() {
        let sexpr =
            diagnostics_sexpr_for("package P { constraint c; requirement def R { require c; } }");
        assert!(
            sexpr.contains("unsupported_requirement_definition_member"),
            "expected the constraint-keyword-less `require c;` shorthand to remain unsupported, got: {sexpr}"
        );
    }

    /// A state def/usage body's bare `entry;`/`do;`/`exit;` (no `action` reference, no body
    /// content) is a legal no-op marker -- pervasive in the training/validation corpus (e.g.
    /// `entry; then off;`) -- and must not be reported as `unsupported_state_definition_member`
    /// merely because it has no bound action reference to lower.
    #[test]
    fn bare_entry_do_exit_with_no_reference_or_body_is_not_unsupported() {
        let sexpr = semantic_sexpr_for(
            "package P { state def S { state off; entry; do; exit; then off; } }",
        );
        assert!(
            !sexpr.contains("unsupported_state_definition_member"),
            "did not expect unsupported_state_definition_member for bare entry/do/exit, got: {sexpr}"
        );
    }

    /// An inline `entry { <members> }` anonymous action body (non-empty brace, no `action`
    /// reference) genuinely has no representation in the `EntryAction` typed AST and must stay an
    /// explicit unsupported diagnostic, distinguishing it from the empty/semicolon no-op case
    /// above.
    #[test]
    fn entry_with_inline_body_content_and_no_reference_stays_unsupported() {
        let sexpr = diagnostics_sexpr_for("package P { state def S { entry { state inner; } } }");
        assert!(
            sexpr.contains("unsupported_state_definition_member"),
            "expected an inline non-empty entry body with no reference to remain unsupported, got: {sexpr}"
        );
    }

    /// The `then send new S() to b;` continuation shorthand (formerly parser Gap 30) now parses as
    /// `ThenTarget::Send`, carrying the same `ActionUsage` shape a standalone `send ...;` statement
    /// produces, so it lowers through `lower_action_usage` exactly like `then action ...;` does:
    /// the payload constructor resolves as an `invocationCallee` and the `to` clause as a
    /// `sendTarget`, and nothing is left to parser recovery.
    #[test]
    fn then_send_continuation_resolves_payload_and_target() {
        let sexpr = semantic_sexpr_for(
            "package P { attribute def S; action def A { action b; then send new S() to b; } }",
        );
        assert!(
            !sexpr.contains("(completeness parse-recovery)"),
            "expected `then send ...;` to parse, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind invocationCallee)") && sexpr.contains("(kind sendTarget)"),
            "expected the send payload and target to resolve as references, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A bare `flow <source> to <target>;` statement (distinct from a named/typed flow usage or
    /// def) must lower as its own `DeclarationKind::Flow` feature. Its ends are typed
    /// `KermlConnectorEnd`s upstream -- the same connector-end shape the KerML connector, binding
    /// and succession members carry -- so each resolves directly as a `flowSource`/`flowTarget`
    /// reference to the feature it names, including the dotted feature-chain spelling.
    #[test]
    fn bare_flow_stmt_resolves_source_and_target() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action aa { out part target; } action snd { in receiver; } flow aa.target to snd.receiver; } }",
        );
        assert!(
            sexpr.contains("(kind flow)"),
            "expected a flow declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains(
                "(kind flowSource) (ordinal 0))\n      (authored-target \"aa::target\")\n      (outcome (status resolved)"
            ) && sexpr.contains(
                "(kind flowTarget) (ordinal 0))\n      (authored-target \"snd::receiver\")\n      (outcome (status resolved)"
            ),
            "expected both flow ends to resolve to the features they name, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// `terminate <name>;` nested inside a `then action <name> { ... }` self-named action usage
    /// (the representative fixture shape, e.g. `then action c1 { terminate c1; }`) must resolve
    /// its target through the shared `DeclarationDomain::Any` lexical lookup, sourced directly at
    /// the enclosing action usage's own declaration (not an anonymous nested one): the terminate
    /// statement's own enclosing scope is the action usage's *parent*'s children, where its own
    /// self-name is declared -- a genuine self-termination idiom.
    #[test]
    fn terminate_stmt_with_target_resolves() {
        let sexpr =
            semantic_sexpr_for("package P { action def A { then action c1 { terminate c1; } } }");
        assert!(
            sexpr.contains("(kind terminateTarget)"),
            "expected a terminateTarget reference, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the terminate target to resolve to its enclosing self-named action, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// A bare `terminate;` (no target) has nothing to resolve and must not be flagged as
    /// unsupported -- it is a legitimate no-op self-termination form, not a parser gap.
    #[test]
    fn bare_terminate_stmt_is_not_unsupported() {
        let sexpr = semantic_sexpr_for("package P { action def A { terminate; } }");
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    /// An `assign <target> := <value>;` reassignment statement must lower as an anonymous
    /// `assign` declaration whose `lhs` resolves as an `assignTarget` reference to its sibling
    /// action and whose `rhs` value expression resolves/evaluates, not fall through to
    /// `unsupported_action_definition_member`.
    #[test]
    fn assign_stmt_target_and_value_resolve() {
        let sexpr = semantic_sexpr_for("package P { action def A { action x; assign x := 5; } }");
        assert!(
            sexpr.contains("(kind assign)"),
            "expected an assign declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind assignTarget)"),
            "expected an assignTarget relationship kind, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(status unresolved)"),
            "expected the assign target to resolve to its sibling action, got: {sexpr}"
        );
    }

    /// An `assign` statement whose value expression references an unresolvable operand must
    /// still publish the target/value references, staying explicitly unresolved rather than
    /// silently dropped.
    #[test]
    fn assign_stmt_unresolvable_target_stays_unresolved() {
        let sexpr = semantic_sexpr_for("package P { action def A { assign missing := 5; } }");
        assert!(
            sexpr.contains("(kind assignTarget)"),
            "expected an assignTarget reference to be authored, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(status unresolved)"),
            "expected the unresolvable assign target to remain explicitly unresolved, got: {sexpr}"
        );
    }

    /// `PerformBodyElement::InOut` (BNF `PerformInOutBinding`, the `in`/`out <target> = <value>;`
    /// parameter-argument-binding shorthand used when invoking a nested `perform action`, e.g.
    /// `perform action dynamics : StraightLineDynamics { in power = vehiclePower; }`) was
    /// unconditionally unsupported -- wires it via `lower_perform_inout_binding`.
    #[test]
    fn perform_inout_binding_resolves_target_and_value() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { in power; perform action dynamics : A { in power = vehiclePower; } } action def Outer { attribute vehiclePower; } }",
        );
        assert!(
            sexpr.contains("(kind perform-parameter-binding)"),
            "expected an anonymous perform-parameter-binding declaration, got: {sexpr}"
        );
        assert!(
            sexpr.contains("(kind performParameterTarget)"),
            "expected the `in power` target to resolve as performParameterTarget, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_usage_member"),
            "did not expect unsupported_action_usage_member, got: {sexpr}"
        );
    }

    /// `lower_succession_end` (used for `AssignTarget` among others) handled `Expression::
    /// MemberAccess` but not the sibling `Expression::FeatureChainRef` shape the parser actually
    /// produces for a dotted assign target (e.g. `assign a.b := 1;`), mirroring the fix already
    /// applied to `lower_satisfy_operand`.
    #[test]
    fn assign_target_dotted_feature_chain_resolves() {
        let sexpr = semantic_sexpr_for(
            "package P { part def A { part def B { attribute count; } part b : B; } action def Act { part a : A; assign a.b.count := 1; } }",
        );
        assert!(
            sexpr.contains("(kind memberAccessOperand)"),
            "expected the dotted assign target to resolve as memberAccessOperand, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("unsupported_action_definition_member"),
            "did not expect unsupported_action_definition_member, got: {sexpr}"
        );
    }

    // --- Canonical element identity ---------------------------------------------------------

    fn publication_for(sources: &[(&str, &str)]) -> PublishedResolution {
        let request = BuildRequest::new(
            sources
                .iter()
                .map(|(identity, source)| {
                    SourceInput::new(*identity, source.to_string(), SourceKind::Workspace)
                })
                .collect(),
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap();
        build(request).unwrap()
    }

    fn target_symbol(
        published: &PublishedResolution,
        document: &str,
        line: u32,
        character: u32,
    ) -> SymbolIdentity {
        match published.target_at(document, TextPosition { line, character }) {
            QueryOutcome::Resolved(target) => target.symbol,
            other => panic!("expected a resolved navigation target, got: {other:?}"),
        }
    }

    /// Anonymous ordinals are allocated per `(document, owner, kind)`, so an identity that named
    /// only the kind and ordinal could not tell two same-kind anonymous declarations under
    /// different owners apart. The identity spells out the owner chain for exactly this reason.
    #[test]
    fn anonymous_declarations_under_different_owners_get_distinct_identities() {
        let sexpr = semantic_sexpr_for(
            "package P { action def A { action x; if x { action y; } else { action z; } } action def B { action x; if x { action y; } else { action z; } } }",
        );
        assert!(
            sexpr.contains(r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "A")) (anonymous (kind if) (ordinal 0)))"#),
            "expected the if-scope under A to carry its owner in its identity, got: {sexpr}"
        );
        assert!(
            sexpr.contains(r#"(path (named (kind package) (name "P")) (named (kind action-def) (name "B")) (anonymous (kind if) (ordinal 0)))"#),
            "expected the if-scope under B to carry its owner in its identity, got: {sexpr}"
        );
    }

    /// The identity is structural, so editing an unrelated document cannot change it. A dense
    /// storage ordinal would shift as soon as any earlier document gained a declaration.
    #[test]
    fn element_identity_survives_an_edit_to_an_unrelated_document() {
        let before = publication_for(&[
            ("memory://a.sysml", "package A { part def Wheel; }"),
            ("memory://b.sysml", "package B { part def Engine; }"),
        ]);
        let after = publication_for(&[
            (
                "memory://a.sysml",
                "package A { part def Wheel; part def Axle; part def Frame; }",
            ),
            ("memory://b.sysml", "package B { part def Engine; }"),
        ]);

        let engine_before = target_symbol(&before, "memory://b.sysml", 0, 21);
        let engine_after = target_symbol(&after, "memory://b.sysml", 0, 21);
        assert_eq!(
            engine_before, engine_after,
            "expected an unrelated document's edit to leave this element's identity unchanged"
        );
    }

    /// Two identically named siblings of the same kind are distinguished by an occurrence ordinal,
    /// so each remains addressable. The Pilot does the same: its `qualifiedName` derivation yields
    /// null for every same-named member after the first, and `path()` then falls through to a
    /// positional form.
    ///
    /// The first occurrence keeps the plain name, so authoring a duplicate later never disturbs
    /// the identity already published for the declaration that was there first.
    #[test]
    fn duplicate_sibling_names_stay_separately_addressable() {
        let published = publication_for(&[(
            "memory://dup.sysml",
            "package P { part def Failure; part def Failure; }",
        )]);

        let first = target_symbol(&published, "memory://dup.sysml", 0, 21);
        let second = target_symbol(&published, "memory://dup.sysml", 0, 39);
        assert_ne!(
            first, second,
            "expected identically named siblings to carry distinct identities"
        );

        for symbol in [&first, &second] {
            match published.references(symbol, true) {
                QueryOutcome::Resolved(locations) => assert_eq!(
                    locations.len(),
                    1,
                    "expected each sibling to resolve to its own declaration site"
                ),
                other => panic!("expected a resolved references outcome, got: {other:?}"),
            }
        }
    }

    // --- Element inspection -----------------------------------------------------------------

    fn inspect_named(
        published: &PublishedResolution,
        document: &str,
        line: u32,
        character: u32,
    ) -> ElementInspection {
        let symbol = target_symbol(published, document, line, character);
        match published.inspect(&symbol) {
            QueryOutcome::Resolved(inspection) => inspection,
            other => panic!("expected a resolved inspection, got: {other:?}"),
        }
    }

    /// The facts an inspector needs arrive as one typed answer, each from its own producer --
    /// no attribute map, and nothing recovered by re-reading source text.
    #[test]
    fn inspection_publishes_every_authored_fact_of_an_element() {
        let published = publication_for(&[(
            "memory://i.sysml",
            "package P {\n  part def Wheel;\n  /* doc */\n  part def Car {\n    doc /* the car */\n    part wheels : Wheel[0..4] ordered;\n  }\n}",
        )]);
        let wheels = inspect_named(&published, "memory://i.sysml", 5, 9);

        assert_eq!(wheels.kind, ElementKind::PartUsage);
        assert_eq!(wheels.name.as_deref(), Some("wheels"));
        assert_eq!(&*wheels.qualified_name, "P::Car::wheels");
        assert_eq!(wheels.membership.kind, MembershipKind::Feature);
        assert_eq!(
            wheels.membership.provenance,
            VisibilityProvenance::Default,
            "no visibility keyword was written, so the default applies and says so"
        );
        assert_eq!(
            wheels.multiplicity,
            MultiplicityFacts::Declared {
                lower: MultiplicityBound::Literal(0),
                upper: MultiplicityBound::Literal(4),
                ordered: true,
                nonunique: false,
            }
        );
        assert_eq!(&*wheels.modifiers, &[ElementModifier::Ordered]);
        assert_eq!(wheels.evaluation, EvaluationState::NotApplicable);

        let typing = wheels
            .relationships
            .iter()
            .find(|relationship| relationship.kind == "featureTyping")
            .expect("expected a featureTyping relationship");
        assert_eq!(typing.authored.as_deref(), Some("Wheel"));
        assert_eq!(typing.provenance, RelationshipProvenance::Authored);
        assert!(matches!(typing.target, RelationshipTarget::Resolved(_)));

        let car = inspect_named(&published, "memory://i.sysml", 3, 11);
        assert_eq!(car.documentation.len(), 1, "expected the doc comment");
        assert_eq!(&*car.documentation[0].text, " the car ");
        assert_eq!(car.documentation[0].form, AnnotationForm::Documentation);
    }

    /// An unresolved reference keeps its own outcome instead of being dropped, so an inspector can
    /// show what was written alongside the fact that it did not resolve.
    #[test]
    fn inspection_keeps_an_unresolved_reference_and_its_authored_text() {
        let published = publication_for(&[(
            "memory://i.sysml",
            "package P {\n  part broken : NoSuchType;\n}",
        )]);
        let broken = inspect_named(&published, "memory://i.sysml", 1, 8);
        let typing = broken
            .relationships
            .iter()
            .find(|relationship| relationship.kind == "featureTyping")
            .expect("expected the authored typing reference to survive");
        assert_eq!(typing.authored.as_deref(), Some("NoSuchType"));
        assert_eq!(typing.target, RelationshipTarget::Unresolved);
    }

    /// A position identifies two different elements, and an inspector needs both: the declaration
    /// the cursor sits in, and what the reference under it points at.
    #[test]
    fn inspect_at_reports_the_containing_and_the_referenced_element() {
        let published = publication_for(&[(
            "memory://i.sysml",
            "package P {\n  part def Wheel;\n  part w : Wheel;\n}",
        )]);
        let at = match published.inspect_at(
            "memory://i.sysml",
            TextPosition {
                line: 2,
                character: 12,
            },
        ) {
            QueryOutcome::Resolved(at) => at,
            other => panic!("expected a resolved inspection, got: {other:?}"),
        };

        assert_eq!(
            at.containing.as_ref().and_then(|c| c.name.as_deref()),
            Some("w"),
            "the cursor sits inside `w`'s declaration"
        );
        match &at.referenced {
            ReferenceAt::Resolved(referenced) => assert_eq!(
                referenced.name.as_deref(),
                Some("Wheel"),
                "and points at a reference resolving to `Wheel`"
            ),
            other => panic!("expected a resolved reference at the position, got: {other:?}"),
        }
    }

    fn position_of(source: &str, needle: &str) -> TextPosition {
        let (line, character) = source
            .lines()
            .enumerate()
            .find_map(|(line, text)| text.find(needle).map(|column| (line, column)))
            .unwrap_or_else(|| panic!("{needle:?} does not occur in the fixture"));
        TextPosition {
            line: u32::try_from(line).expect("fixture line fits"),
            character: u32::try_from(character).expect("fixture column fits"),
        }
    }

    /// How many index entries one `inspect_at` visits against a given workspace.
    fn inspect_at_cost(sources: &[(&str, &str)], document: &str, needle: &str) -> u64 {
        let published = publication_for(sources);
        let source = sources
            .iter()
            .find(|(identity, _)| *identity == document)
            .expect("the probed document is in the workspace")
            .1;
        let position = position_of(source, needle);
        let (outcome, visited) = crate::index::documents::measure_visited_index_entries(|| {
            published.inspect_at(document, position)
        });
        assert!(
            matches!(outcome, QueryOutcome::Resolved(_)),
            "the probe must land on a resolved inspection, got: {outcome:?}"
        );
        visited
    }

    /// How many reverse-index entries one references query visits for the declaration at `needle`.
    fn references_cost(sources: &[(&str, &str)], document: &str, needle: &str) -> u64 {
        let published = publication_for(sources);
        let source = sources
            .iter()
            .find(|(identity, _)| *identity == document)
            .expect("the probed document is in the workspace")
            .1;
        let position = position_of(source, needle);
        let symbol = match published.target_at(document, position) {
            QueryOutcome::Resolved(target) => target.symbol,
            other => panic!("the probe must resolve to a target, got: {other:?}"),
        };
        let (outcome, visited) = crate::index::documents::measure_visited_index_entries(|| {
            published.references(&symbol, false)
        });
        assert!(
            matches!(outcome, QueryOutcome::Resolved(_)),
            "the references query must resolve, got: {outcome:?}"
        );
        visited
    }

    /// How many cursor/span, name-range and effective-scope entries one completion query visits.
    fn visible_members_cost(
        sources: &[(&str, &str)],
        document: &str,
        needle: &str,
        qualifier: Option<&str>,
    ) -> u64 {
        let published = publication_for(sources);
        let source = sources
            .iter()
            .find(|(identity, _)| *identity == document)
            .expect("the probed document is in the workspace")
            .1;
        let position = position_of(source, needle);
        let (outcome, visited) = crate::index::documents::measure_visited_index_entries(|| {
            published.visible_members(document, position, qualifier)
        });
        assert!(
            matches!(outcome, QueryOutcome::Resolved(_)),
            "the visible-members query must resolve, got: {outcome:?}"
        );
        visited
    }

    /// The probed document, unchanged across every variant below.
    const PROBED: &str = "package P {\n  part def Wheel;\n  part w : Wheel;\n}";

    /// Declarations that are cheap to write and land in every published fact table -- a name, a
    /// documentation record, a reference and an evaluated value -- so that a scan of any of them
    /// shows up in the measurement.
    fn padding(members: usize) -> String {
        (0..members)
            .map(|index| {
                format!(
                    "  part def Pad{index} {{ doc /* pad */ }}\n                       part padUse{index} : Pad{index};\n                       attribute padValue{index} = {index} + 1;\n"
                )
            })
            .collect()
    }

    /// The identity of the declaration containing `needle`, and the publication it belongs to.
    fn probe_symbol(
        published: &PublishedResolution,
        source: &str,
        document: &str,
        needle: &str,
    ) -> SymbolIdentity {
        match published.inspect_at(document, position_of(source, needle)) {
            QueryOutcome::Resolved(at) => {
                at.containing
                    .expect("the probe must land inside a declaration")
                    .identity
            }
            other => panic!("the probe must resolve to an inspection, got: {other:?}"),
        }
    }

    /// How many index entries one settled evaluation query visits.
    fn evaluate_cost(sources: &[(&str, &str)], document: &str, needle: &str) -> u64 {
        let published = publication_for(sources);
        let source = sources
            .iter()
            .find(|(identity, _)| *identity == document)
            .expect("the probed document is in the workspace")
            .1;
        let symbol = probe_symbol(&published, source, document, needle);
        let (outcome, visited) =
            crate::index::documents::measure_visited_index_entries(|| published.evaluate(&symbol));
        assert!(
            matches!(outcome, QueryOutcome::Resolved(_)),
            "the evaluation query must resolve, got: {outcome:?}"
        );
        visited
    }

    /// A workspace whose probed declaration carries a value and a unit token.
    const EVALUATED: &str = "package P {\n  attribute mass = 1750 [kg];\n}";

    /// An evaluation answer is three indexed lookups and the rows they name, so a workspace that
    /// grows elsewhere costs nothing here. A scan of the evaluation, unit or measurement tables
    /// would return the same answer; only the measurement separates them.
    #[test]
    fn evaluation_cost_is_independent_of_the_rest_of_the_workspace() {
        let small = evaluate_cost(
            &[("memory://e.sysml", EVALUATED)],
            "memory://e.sysml",
            "1750",
        );
        let large_source = format!("package Other {{\n{}}}\n", padding(500));
        let large = evaluate_cost(
            &[
                ("memory://e.sysml", EVALUATED),
                ("memory://other.sysml", &large_source),
            ],
            "memory://e.sysml",
            "1750",
        );
        assert_eq!(
            small, large,
            "500 declarations in another document changed what one evaluation query reads"
        );
    }

    /// Repeating the query repeats the same lookups: nothing is resolved, folded or memoised on
    /// the way out, so a second call cannot be cheaper -- or more expensive -- than the first.
    #[test]
    fn repeating_an_evaluation_query_does_the_same_work() {
        let published = publication_for(&[("memory://e.sysml", EVALUATED)]);
        let symbol = probe_symbol(&published, EVALUATED, "memory://e.sysml", "1750");
        let measure = || {
            crate::index::documents::measure_visited_index_entries(|| published.evaluate(&symbol))
        };
        let (first, first_cost) = measure();
        let (second, second_cost) = measure();
        assert_eq!(
            first, second,
            "a repeated evaluation query changed its answer"
        );
        assert_eq!(
            first_cost, second_cost,
            "a repeated evaluation query changed its work"
        );
    }

    /// Inspection and evaluation project the same settled state, so asking one first cannot
    /// change what the other answers or what it costs.
    #[test]
    fn inspection_and_evaluation_agree_in_either_order() {
        let evaluation_first = {
            let published = publication_for(&[("memory://e.sysml", EVALUATED)]);
            let symbol = probe_symbol(&published, EVALUATED, "memory://e.sysml", "1750");
            let (evaluation, cost) = crate::index::documents::measure_visited_index_entries(|| {
                published.evaluate(&symbol)
            });
            let inspection = published.inspect(&symbol);
            (evaluation, inspection, cost)
        };
        let inspection_first = {
            let published = publication_for(&[("memory://e.sysml", EVALUATED)]);
            let symbol = probe_symbol(&published, EVALUATED, "memory://e.sysml", "1750");
            let inspection = published.inspect(&symbol);
            let (evaluation, cost) = crate::index::documents::measure_visited_index_entries(|| {
                published.evaluate(&symbol)
            });
            (evaluation, inspection, cost)
        };
        assert_eq!(
            evaluation_first.0, inspection_first.0,
            "query order changed the evaluation answer"
        );
        assert_eq!(
            evaluation_first.2, inspection_first.2,
            "query order changed the evaluation query's work"
        );
        let QueryOutcome::Resolved(evaluation) = &evaluation_first.0 else {
            panic!("the probe must resolve");
        };
        let QueryOutcome::Resolved(inspection) = &evaluation_first.1 else {
            panic!("the probe must resolve");
        };
        assert_eq!(
            evaluation.state, inspection.evaluation,
            "inspection and the evaluation service must project one canonical result"
        );
    }

    /// The contract the publication-time index exists to keep: what an inspection reads is this
    /// declaration's own facts, so a workspace that grows elsewhere costs nothing here.
    ///
    /// A scan and an index return the same answer, so only the measurement separates them.
    #[test]
    fn inspection_cost_is_independent_of_the_rest_of_the_workspace() {
        let small = inspect_at_cost(
            &[("memory://i.sysml", PROBED)],
            "memory://i.sysml",
            ": Wheel",
        );
        let large_source = format!("package Other {{\n{}}}\n", padding(500));
        let large = inspect_at_cost(
            &[
                ("memory://i.sysml", PROBED),
                ("memory://other.sysml", &large_source),
            ],
            "memory://i.sysml",
            ": Wheel",
        );
        assert_eq!(
            small, large,
            "500 declarations in another document changed what one inspection reads"
        );
    }

    /// A reverse-reference query visits only the selected target's CSR range, not unrelated
    /// authored references elsewhere in the publication.
    #[test]
    fn references_cost_is_independent_of_unrelated_references() {
        let small = references_cost(
            &[("memory://i.sysml", PROBED)],
            "memory://i.sysml",
            "Wheel;",
        );
        let large_source = format!("package Other {{\n{}}}\n", padding(500));
        let large = references_cost(
            &[
                ("memory://i.sysml", PROBED),
                ("memory://other.sysml", &large_source),
            ],
            "memory://i.sysml",
            "Wheel;",
        );
        assert_eq!(small, 1, "Wheel has exactly one authored reference");
        assert_eq!(
            small, large,
            "500 unrelated references changed the selected target's query cost"
        );
    }

    /// Cursor ownership and effective-scope enumeration are indexed: growing an unrelated
    /// package changes neither the span-tree descent nor the selected scopes' member ranges.
    #[test]
    fn visible_members_cost_is_independent_of_unrelated_scope_contents() {
        let thin_other = format!("package Other {{\n{}}}\n", padding(1));
        let fat_other = format!("package Other {{\n{}}}\n", padding(500));
        let cost = |other: &str, qualifier| {
            visible_members_cost(
                &[
                    ("memory://i.sysml", PROBED),
                    ("memory://other.sysml", other),
                ],
                "memory://i.sysml",
                ": Wheel",
                qualifier,
            )
        };
        assert_eq!(
            cost(&thin_other, None),
            cost(&fat_other, None),
            "499 unrelated members changed unqualified completion cost"
        );
        assert_eq!(
            cost(&thin_other, Some("P")),
            cost(&fat_other, Some("P")),
            "499 unrelated members changed qualified completion cost"
        );
    }

    /// A qualifier is resolved through lexical scope indexes. Two same-name candidates therefore
    /// remain explicitly ambiguous instead of having their members silently merged by display
    /// name.
    #[test]
    fn visible_members_keeps_ambiguous_qualifier_scopes_separate() {
        let source = "package P { part def A; } package P { part def B; } package Use { part x; }";
        let published = publication_for(&[("memory://i.sysml", source)]);
        let outcome =
            published.visible_members("memory://i.sysml", position_of(source, "part x"), Some("P"));
        let QueryOutcome::Ambiguous(candidates) = outcome else {
            panic!("expected ambiguous qualifier scopes, got: {outcome:?}");
        };
        assert_eq!(candidates.len(), 2);
        let mut names = candidates
            .iter()
            .flat_map(|members| members.iter().map(|member| member.name.as_ref()))
            .collect::<Vec<_>>();
        names.sort_unstable();
        assert_eq!(names, ["A", "B"]);
    }

    /// A preceding sibling that does not contain the position is skipped whole, not descended
    /// into: the containment descent costs the nesting depth, not the document's size.
    #[test]
    fn inspection_cost_is_independent_of_preceding_sibling_subtrees() {
        let thin = format!("package Before {{\n{}}}\n{PROBED}", padding(1));
        let fat = format!("package Before {{\n{}}}\n{PROBED}", padding(500));
        assert_eq!(
            inspect_at_cost(
                &[("memory://i.sysml", &thin)],
                "memory://i.sysml",
                ": Wheel"
            ),
            inspect_at_cost(&[("memory://i.sysml", &fat)], "memory://i.sysml", ": Wheel"),
            "499 extra members of an earlier package were visited rather than skipped"
        );
    }

    /// Declarations that begin after the position end the descent rather than being filtered out
    /// one by one.
    #[test]
    fn inspection_cost_is_independent_of_what_follows_the_position() {
        let thin = format!("{PROBED}\npackage After {{\n{}}}\n", padding(1));
        let fat = format!("{PROBED}\npackage After {{\n{}}}\n", padding(500));
        assert_eq!(
            inspect_at_cost(
                &[("memory://i.sysml", &thin)],
                "memory://i.sysml",
                ": Wheel"
            ),
            inspect_at_cost(&[("memory://i.sysml", &fat)], "memory://i.sysml", ": Wheel"),
            "declarations beginning after the position were visited"
        );
    }

    /// The outline lists every declaration in the document, including anonymous ones, each with
    /// the identity that addresses it.
    #[test]
    fn document_symbols_lists_every_declaration_with_its_identity() {
        let published = publication_for(&[(
            "memory://i.sysml",
            "package P {\n  part def Wheel;\n  part w : Wheel;\n}",
        )]);
        let symbols = match published.document_symbols("memory://i.sysml") {
            QueryOutcome::Resolved(symbols) => symbols,
            other => panic!("expected resolved symbols, got: {other:?}"),
        };
        let names = symbols
            .iter()
            .filter_map(|entry| entry.name.as_deref())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["P", "Wheel", "w"]);

        let wheel = symbols
            .iter()
            .find(|entry| entry.name.as_deref() == Some("Wheel"))
            .expect("Wheel");
        assert_eq!(wheel.kind, ElementKind::PartDefinition);
        assert!(
            matches!(
                published.inspect(&wheel.identity),
                QueryOutcome::Resolved(_)
            ),
            "an outline entry's identity must address the same element"
        );
    }

    #[test]
    fn inspecting_an_unknown_document_is_unresolved() {
        let published = publication_for(&[("memory://i.sysml", "package P { }")]);
        assert!(matches!(
            published.document_symbols("memory://absent.sysml"),
            QueryOutcome::Unresolved
        ));
    }

    #[test]
    fn typed_element_search_filters_by_kind_and_authored_source_in_canonical_order() {
        let request = BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://z.sysml",
                    "package Z { requirement def Later; part def NotARequirement; }".into(),
                    SourceKind::Workspace,
                ),
                SourceInput::new(
                    "memory://standard.sysml",
                    "package Standard { requirement def LibraryRequirement; }".into(),
                    SourceKind::StandardLibrary,
                ),
                SourceInput::new(
                    "memory://a.sysml",
                    "package A { requirement def First; requirement def Second; }".into(),
                    SourceKind::Workspace,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .expect("request");
        let published = build(request).expect("publication");

        let requirements = match published.search_elements(ElementSearch {
            kind: ElementKind::RequirementDefinition,
            source: ElementSource::Workspace,
        }) {
            QueryOutcome::Resolved(entries) => entries,
            other => panic!("expected resolved search, got: {other:?}"),
        };
        assert_eq!(
            requirements
                .iter()
                .map(|entry| (
                    entry.location.document.as_ref(),
                    entry.qualified_name.as_ref()
                ))
                .collect::<Vec<_>>(),
            vec![
                ("memory://a.sysml", "A::First"),
                ("memory://a.sysml", "A::Second"),
                ("memory://z.sysml", "Z::Later"),
            ]
        );
        assert!(requirements
            .iter()
            .all(|entry| entry.kind == ElementKind::RequirementDefinition));

        let library = match published.search_elements(ElementSearch {
            kind: ElementKind::RequirementDefinition,
            source: ElementSource::StandardLibrary,
        }) {
            QueryOutcome::Resolved(entries) => entries,
            other => panic!("expected resolved search, got: {other:?}"),
        };
        assert_eq!(library.len(), 1);
        assert_eq!(
            library[0].qualified_name.as_ref(),
            "Standard::LibraryRequirement"
        );
    }

    #[test]
    fn satisfy_query_pairs_directional_ends_preserves_identity_polarity_and_unresolved() {
        let published = publication_for(&[(
            "memory://trace.sysml",
            r#"
package Trace {
    requirement def Safety;
    requirement def Performance;
    part def Vehicle;
    part vehicle : Vehicle;
    satisfy Performance by vehicle;
    not satisfy Safety by vehicle;
    satisfy Missing by vehicle;
}
"#,
        )]);
        let values = match published.satisfy_relationships() {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected resolved satisfy query, got {other:?}"),
        };
        assert_eq!(values.len(), 3);
        let requirements = match published.search_elements(ElementSearch {
            kind: ElementKind::RequirementDefinition,
            source: ElementSource::Workspace,
        }) {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected requirements, got {other:?}"),
        };
        let performance = requirements
            .iter()
            .find(|value| value.qualified_name.as_ref() == "Trace::Performance")
            .expect("Performance");
        let parts = match published.search_elements(ElementSearch {
            kind: ElementKind::PartUsage,
            source: ElementSource::Workspace,
        }) {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected parts, got {other:?}"),
        };
        let vehicle = parts
            .iter()
            .find(|value| value.qualified_name.as_ref() == "Trace::vehicle")
            .expect("vehicle");
        assert!(
            matches!(&values[0].requirement, SatisfyEndpoint::Resolved(value) if value == &performance.identity)
        );
        assert!(
            matches!(&values[0].satisfying_element, SatisfyEndpoint::Resolved(value) if value == &vehicle.identity)
        );
        assert_eq!(values[0].polarity, SatisfyPolarity::Satisfied);
        assert_eq!(values[1].polarity, SatisfyPolarity::NotSatisfied);
        assert!(matches!(values[2].requirement, SatisfyEndpoint::Unresolved));
        assert_eq!(values[0].provenance, RelationshipProvenance::Authored);
        assert_ne!(values[0].identity, values[1].identity);
    }

    #[test]
    fn binding_connector_query_pairs_ends_preserves_duplicates_and_unresolved_outcomes() {
        let published = publication_for(&[(
            "memory://binding.sysml",
            r#"
package Binding {
    action def Act {
        action start;
        action done;
        bind start = done;
        bind Missing = done;
        bind start = done;
    }
}
"#,
        )]);
        let values = match published.binding_connectors() {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected resolved binding connectors, got {other:?}"),
        };
        assert_eq!(
            values.len(),
            3,
            "each authored binding must remain a separate fact"
        );
        let actions = match published.search_elements(ElementSearch {
            kind: ElementKind::ActionUsage,
            source: ElementSource::Workspace,
        }) {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected actions, got {other:?}"),
        };
        let start = actions
            .iter()
            .find(|value| value.qualified_name.as_ref() == "Binding::Act::start")
            .expect("start action");
        let done = actions
            .iter()
            .find(|value| value.qualified_name.as_ref() == "Binding::Act::done")
            .expect("done action");
        assert!(
            matches!(&values[0].source, BindingEndpoint::Resolved(value) if value == &start.identity)
        );
        assert!(
            matches!(&values[0].target, BindingEndpoint::Resolved(value) if value == &done.identity)
        );
        assert!(matches!(values[1].source, BindingEndpoint::Unresolved));
        assert!(
            matches!(&values[2].source, BindingEndpoint::Resolved(value) if value == &start.identity)
        );
        assert_eq!(values[0].provenance, RelationshipProvenance::Authored);
        assert_ne!(values[0].identity, values[2].identity);
    }

    #[test]
    fn feature_reference_expression_binding_check_is_explicitly_unsupported_without_owned_facts() {
        let published = publication_for(&[(
            "memory://binding-rule.sysml",
            "package Binding { action def Act { action start; action done; bind start = done; } }",
        )]);
        assert!(matches!(
            published.binding_connector_validation(
                BindingConnectorCheckKind::FeatureReferenceExpression
            ),
            QueryOutcome::Resolved(BindingConnectorValidationOutcome::Unsupported {
                prerequisite:
                    BindingConnectorValidationPrerequisite::FeatureReferenceExpressionTargetAndResult,
            })
        ));
        assert!(matches!(
            published.binding_connectors(),
            QueryOutcome::Resolved(values) if values.len() == 1
        ));
    }

    #[test]
    fn verification_query_owns_case_direction_endpoint_status_and_unsupported_outcome() {
        let published = publication_for(&[(
            "memory://verification.sysml",
            r#"
package V {
    requirement required;
    verification def Check {
        objective { verify required; }
        objective second { verify Missing; }
    }
}
"#,
        )]);
        let values = match published.requirement_verifications() {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected resolved verification query, got {other:?}"),
        };
        assert_eq!(values.len(), 2);
        let cases = match published.search_elements(ElementSearch {
            kind: ElementKind::VerificationCaseDefinition,
            source: ElementSource::Workspace,
        }) {
            QueryOutcome::Resolved(values) => values,
            other => panic!("expected verification cases, got {other:?}"),
        };
        let check = cases
            .iter()
            .find(|value| value.qualified_name.as_ref() == "V::Check")
            .expect("Check");
        assert!(values
            .iter()
            .all(|value| value.verification_case == check.identity));
        assert!(matches!(
            values[0].requirement,
            VerificationRequirement::Resolved(_)
        ));
        assert!(matches!(
            values[1].requirement,
            VerificationRequirement::Unresolved
        ));
        assert!(values
            .iter()
            .all(|value| value.provenance == RelationshipProvenance::Authored));
        assert!(values
            .iter()
            .all(|value| value.outcome == VerificationOutcome::Unsupported));
        assert_ne!(values[0].identity, values[1].identity);
    }

    // --- Evaluation states ------------------------------------------------------------------

    /// `EvaluationPolicy::Skip` publishes a coherent resolved model in which every element that
    /// has an expression says so explicitly, rather than an empty table a consumer cannot tell
    /// from "there was nothing to evaluate".
    #[test]
    fn skipping_evaluation_publishes_not_run_rather_than_nothing() {
        let source = "package P { attribute mass : Integer = 5; }";

        let evaluated = semantic_sexpr_for(source);
        assert!(
            evaluated.contains("(state literal) (value (kind integer) (integer 5))"),
            "expected the default policy to evaluate, got: {evaluated}"
        );

        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap()
        .with_evaluation_policy(EvaluationPolicy::Skip);
        let published = build(request).unwrap();
        let mut skipped = String::new();
        published
            .debug()
            .write_semantic_sexpr(&mut skipped)
            .unwrap();

        assert!(
            skipped.contains("(state not-run)"),
            "expected a declared not-run state, got: {skipped}"
        );
        assert!(
            !skipped.contains("(value (kind"),
            "a skipped build must publish no value, got: {skipped}"
        );
    }

    /// A value that was *written* and one that was *computed* are both constants, but only the
    /// expression's shape tells them apart, and a consumer showing "declared" versus "computed"
    /// needs the distinction.
    #[test]
    fn a_written_literal_and_a_computed_constant_report_different_states() {
        let written = semantic_sexpr_for("package P { attribute mass : Integer = 5; }");
        assert!(
            written.contains("(state literal) (value (kind integer) (integer 5))"),
            "expected a written literal, got: {written}"
        );

        let computed = semantic_sexpr_for("package P { attribute mass : Integer = 2 + 3; }");
        assert!(
            computed.contains("(state evaluated) (value (kind integer) (integer 5))"),
            "expected a computed constant, got: {computed}"
        );
    }

    /// A value that depends on itself is a property of the model, published as its own state --
    /// never a fabricated value, an infinite loop, or a panic.
    #[test]
    fn a_self_referential_value_reports_the_cyclic_state() {
        let sexpr = semantic_sexpr_for(
            "package P { constraint def C { a } attribute a : Integer = b; attribute b : Integer = a; }",
        );
        assert!(
            sexpr.contains("(state cyclic)"),
            "expected a cyclic evaluation state for the mutually dependent values, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(state cyclic) (value"),
            "a cyclic state must carry no value, got: {sexpr}"
        );
    }

    /// A failure is not a value: the rendered fact names the failure and stops, so a consumer
    /// cannot mistake it for a value of some fallback kind.
    #[test]
    fn a_division_by_zero_reports_a_failure_and_no_value() {
        let sexpr = semantic_sexpr_for("package P { calc def C { return : Integer = 1 / 0; } }");
        assert!(
            sexpr.contains("(state division-by-zero)"),
            "expected a division-by-zero failure state, got: {sexpr}"
        );
        assert!(
            !sexpr.contains("(state division-by-zero) (value"),
            "a failure must carry no value, got: {sexpr}"
        );
    }

    // --- Type queries -------------------------------------------------------------------------
    //
    // The `# TYPES` snapshot section already shows the published facts these queries read. What it
    // cannot show is the rules layered over them: reflexivity, scope selection, what a cycle does
    // to an answer, and the two conformance rules' treatment of untyped and unrelated features.

    fn symbol_named(
        published: &PublishedResolution,
        document: &str,
        qualified: &str,
    ) -> SymbolIdentity {
        match published.document_symbols(document) {
            QueryOutcome::Resolved(entries)
            | QueryOutcome::Recovered(entries)
            | QueryOutcome::UnsupportedWith(entries) => entries
                .iter()
                .find(|entry| entry.qualified_name.as_ref() == qualified)
                .unwrap_or_else(|| panic!("no declaration named {qualified}"))
                .identity
                .clone(),
            other => panic!("expected document symbols, got: {other:?}"),
        }
    }

    fn conformance(outcome: QueryOutcome<Conformance>) -> Conformance {
        match outcome {
            QueryOutcome::Resolved(value)
            | QueryOutcome::Recovered(value)
            | QueryOutcome::UnsupportedWith(value) => value,
            other => panic!("expected a settled conformance answer, got: {other:?}"),
        }
    }

    fn symbols(outcome: QueryOutcome<Box<[SymbolIdentity]>>) -> Vec<SymbolIdentity> {
        match outcome {
            QueryOutcome::Resolved(values)
            | QueryOutcome::Recovered(values)
            | QueryOutcome::UnsupportedWith(values) => values.into_vec(),
            other => panic!("expected settled symbols, got: {other:?}"),
        }
    }

    #[test]
    fn a_cyclic_hierarchy_yields_no_conformance_answer() {
        let published = publication_for(&[(
            "memory://types.sysml",
            "package P { part def A :> B; part def B :> A; part def C; }",
        )]);
        let a = symbol_named(&published, "memory://types.sysml", "P::A");
        let c = symbol_named(&published, "memory://types.sysml", "P::C");

        assert_eq!(
            conformance(published.conforms_to(&a, &c, SpecializationScope::AnySpecialization)),
            Conformance::Indeterminate(ConformanceObstacle::CyclicSpecialization),
            "a malformed hierarchy must not produce a published conformance fact"
        );
        assert_eq!(
            conformance(published.conforms_to(&a, &a, SpecializationScope::AnySpecialization)),
            Conformance::Conforms,
            "reflexivity holds even inside a cycle"
        );
    }

    #[test]
    fn feature_typing_conformance_rejects_an_unrelated_type() {
        let published = publication_for(&[(
            "memory://types.sysml",
            "package P { part def T; part def U; part def A { part x : T; } part def B :> A { part y : U :>> x; } }",
        )]);
        let general = symbol_named(&published, "memory://types.sysml", "P::A::x");
        let specific = symbol_named(&published, "memory://types.sysml", "P::B::y");

        assert_eq!(
            conformance(published.feature_typing_conforms(&specific, &general)),
            Conformance::DoesNotConform,
            "U neither is nor specializes T"
        );
    }

    // --- Reusing a settled library --------------------------------------------------------------

    const LIBRARY_SOURCE: &str = "standard library package Lib { part def Base; part def Wheel :> Base; attribute def Mass; }";

    fn library_stratum() -> std::sync::Arc<LibraryStratum> {
        std::sync::Arc::new(
            build_library_stratum(vec![SourceInput::new(
                "memory://lib.sysml",
                LIBRARY_SOURCE.to_string(),
                SourceKind::StandardLibrary,
            )])
            .expect("library stratum"),
        )
    }

    fn seeded_and_unseeded_with_library(library_source: &str, workspace: &str) -> (String, String) {
        let library = || {
            std::sync::Arc::new(
                build_library_stratum(vec![SourceInput::new(
                    "memory://lib.sysml",
                    library_source.to_string(),
                    SourceKind::StandardLibrary,
                )])
                .expect("library stratum"),
            )
        };
        let seeded = build(
            BuildRequest::with_library(
                vec![SourceInput::new(
                    "memory://workspace.sysml",
                    workspace.to_string(),
                    SourceKind::Workspace,
                )],
                ConstructionSchedule::Sequential,
                "contract-v1",
                library(),
            )
            .expect("seeded request"),
        )
        .expect("seeded build");
        let unseeded = build(
            BuildRequest::new(
                vec![
                    SourceInput::new(
                        "memory://workspace.sysml",
                        workspace.to_string(),
                        SourceKind::Workspace,
                    ),
                    SourceInput::new(
                        "memory://lib.sysml",
                        library_source.to_string(),
                        SourceKind::StandardLibrary,
                    ),
                ],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .expect("unseeded request"),
        )
        .expect("unseeded build");
        let render = |published: &PublishedResolution| {
            let mut semantic = String::new();
            published
                .debug()
                .write_semantic_sexpr(&mut semantic)
                .expect("semantic");
            let mut types = String::new();
            published
                .debug()
                .write_types_sexpr(&mut types)
                .expect("types");
            let mut diagnostics = String::new();
            published
                .debug()
                .write_diagnostics_sexpr(&mut diagnostics)
                .expect("diagnostics");
            format!("{semantic}\n{types}\n{diagnostics}")
        };
        (render(&seeded), render(&unseeded))
    }

    fn seeded_and_unseeded(workspace: &str) -> (String, String) {
        seeded_and_unseeded_with_library(LIBRARY_SOURCE, workspace)
    }

    /// A minimal but structurally faithful measurement library.
    ///
    /// The unit rules are rooted in library declarations, so parity for them cannot be shown
    /// against a library that declares none. This mirrors the standard library's shape exactly
    /// where the rules read it: `MeasurementUnit` as the root of unit types, `TensorQuantityValue`
    /// with the `mRef` feature a quantity value redefines, and units declared as attribute usages
    /// carrying a short-name symbol.
    const MEASUREMENT_LIBRARY_SOURCE: &str = concat!(
        "standard library package ScalarValues { datatype Boolean; datatype String; ",
        "datatype Real; datatype Integer :> Real; }\n",
        "standard library package MeasurementReferences { ",
        "abstract attribute def MeasurementUnit; ",
        "attribute def MassUnit :> MeasurementUnit; ",
        "attribute def DurationUnit :> MeasurementUnit; }\n",
        "standard library package Quantities { ",
        "abstract attribute def TensorQuantityValue { ",
        "attribute mRef : MeasurementReferences::MeasurementUnit; } ",
        "attribute def MassValue :> TensorQuantityValue { ",
        "attribute :>> mRef : MeasurementReferences::MassUnit; } }\n",
        "standard library package SI { ",
        "attribute <kg> kilogram : MeasurementReferences::MassUnit; ",
        "attribute <s> second : MeasurementReferences::DurationUnit; }",
    );

    /// A workspace exercising every migrated expression-conformance rule that reads the library.
    const MEASUREMENT_WORKSPACE: &str = concat!(
        "package W { ",
        "attribute good : Quantities::MassValue = 1 [kg]; ",
        "attribute wrongDimension : Quantities::MassValue = 1 [s]; ",
        "attribute unknownUnit : Quantities::MassValue = 1 [zz]; ",
        "attribute mistyped : ScalarValues::Boolean = \"no\"; ",
        "constraint def Counted { 1 + 2 } }",
    );

    /// One publication of `workspace` against the measurement library above.
    fn against_measurement_library(
        workspace: &str,
        schedule: ConstructionSchedule,
    ) -> PublishedResolution {
        build(
            BuildRequest::new(
                vec![
                    SourceInput::new(
                        "memory://workspace.sysml",
                        workspace.to_string(),
                        SourceKind::Workspace,
                    ),
                    SourceInput::new(
                        "memory://measurement.sysml",
                        MEASUREMENT_LIBRARY_SOURCE.to_string(),
                        SourceKind::StandardLibrary,
                    ),
                ],
                schedule,
                "contract-v1",
            )
            .expect("measurement request"),
        )
        .expect("measurement build")
    }

    fn measurement_publication(schedule: ConstructionSchedule) -> String {
        render_publication(&against_measurement_library(
            MEASUREMENT_WORKSPACE,
            schedule,
        ))
    }

    /// Whether an element is quantity-typed can only be answered against the library that declares
    /// what a quantity value is. Without it the answer is unknown, and publishing "not a quantity"
    /// would state as a fact about the model what is really a missing input -- silently ruling out
    /// the unit rules rather than reporting that they could not be applied.
    #[test]
    fn a_missing_quantity_library_leaves_measurement_applicability_unavailable() {
        let workspace = "package P { attribute plain = 1; }";
        let published = publication_for(&[("memory://q.sysml", workspace)]);
        let symbol = probe_symbol(&published, workspace, "memory://q.sysml", "plain");
        let QueryOutcome::Resolved(evaluation) = published.evaluate(&symbol) else {
            panic!("the probe must resolve");
        };
        assert_eq!(
            evaluation.expected_measurement,
            ExpectedMeasurement::Unavailable
        );
    }

    /// With the library admitted, the same shape of element gets the affirmative answer.
    #[test]
    fn an_admitted_quantity_library_answers_a_non_quantity_element_affirmatively() {
        let workspace = "package P { attribute plain : ScalarValues::Integer = 1; }";
        let published = against_measurement_library(workspace, ConstructionSchedule::Sequential);
        let symbol = probe_symbol(&published, workspace, "memory://workspace.sysml", "plain");
        let QueryOutcome::Resolved(evaluation) = published.evaluate(&symbol) else {
            panic!("the probe must resolve");
        };
        assert_eq!(
            evaluation.expected_measurement,
            ExpectedMeasurement::NotApplicable
        );
    }

    fn render_publication(published: &PublishedResolution) -> String {
        let mut semantic = String::new();
        published
            .debug()
            .write_semantic_sexpr(&mut semantic)
            .expect("semantic");
        let mut types = String::new();
        published
            .debug()
            .write_types_sexpr(&mut types)
            .expect("types");
        let mut diagnostics = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut diagnostics)
            .expect("diagnostics");
        format!("{semantic}\n{types}\n{diagnostics}")
    }

    /// Every migrated expression rule the parity cases below rely on actually firing.
    const MEASUREMENT_CODES: [&str; 4] = [
        "incompatible_unit_dimension",
        "unknown_unit_symbol",
        "attribute_value_type_mismatch",
        "non_boolean_expression",
    ];

    /// Evaluation, unit resolution and the decisions they feed must not depend on the schedule
    /// that built the publication.
    #[test]
    fn parallel_and_sequential_construction_publish_the_same_evaluation_and_units() {
        let sequential = measurement_publication(ConstructionSchedule::Sequential);
        let parallel = measurement_publication(ConstructionSchedule::Parallel);
        assert_eq!(
            sequential, parallel,
            "evaluation, unit and measurement facts must not depend on construction schedule"
        );
        for code in MEASUREMENT_CODES {
            assert!(
                sequential.contains(code),
                "the parity workspace must actually exercise {code}, got: {sequential}"
            );
        }
    }

    /// The same facts, reached through a settled library stratum rather than a cold solve.
    #[test]
    fn a_seeded_publication_matches_an_unseeded_one_for_evaluation_and_units() {
        let library = std::sync::Arc::new(
            build_library_stratum(vec![SourceInput::new(
                "memory://measurement.sysml",
                MEASUREMENT_LIBRARY_SOURCE.to_string(),
                SourceKind::StandardLibrary,
            )])
            .expect("measurement stratum"),
        );
        let seeded = build(
            BuildRequest::with_library(
                vec![SourceInput::new(
                    "memory://workspace.sysml",
                    MEASUREMENT_WORKSPACE.to_string(),
                    SourceKind::Workspace,
                )],
                ConstructionSchedule::Sequential,
                "contract-v1",
                library,
            )
            .expect("seeded request"),
        )
        .expect("seeded build");
        let seeded = render_publication(&seeded);
        assert_eq!(
            seeded,
            measurement_publication(ConstructionSchedule::Sequential),
            "unit and evaluation decisions must not depend on library-stratum reuse"
        );
        for code in MEASUREMENT_CODES {
            assert!(
                seeded.contains(code),
                "the parity workspace must actually exercise {code}, got: {seeded}"
            );
        }
    }

    /// The identity has to commit the library, or the same workspace built against two different
    /// library versions would claim the same publication identity.
    #[test]
    fn the_library_participates_in_the_publication_identity() {
        let workspace = || {
            vec![SourceInput::new(
                "memory://workspace.sysml",
                "package W { part def Car; }".to_string(),
                SourceKind::Workspace,
            )]
        };
        let with_library = BuildRequest::with_library(
            workspace(),
            ConstructionSchedule::Sequential,
            "contract-v1",
            library_stratum(),
        )
        .expect("seeded request");
        let without =
            BuildRequest::new(workspace(), ConstructionSchedule::Sequential, "contract-v1")
                .expect("plain request");
        assert_ne!(
            with_library.identity().source_digest,
            without.identity().source_digest,
            "admitting a library must change the publication identity"
        );
    }

    #[test]
    fn an_unknown_identity_is_unresolved_rather_than_empty() {
        let published = publication_for(&[("memory://types.sysml", "package P { part def A; }")]);
        let a = symbol_named(&published, "memory://types.sysml", "P::A");
        let missing = SymbolIdentity("no-such-declaration".into());

        assert!(
            matches!(
                published.direct_supertypes(&missing, SpecializationScope::AnySpecialization),
                QueryOutcome::Unresolved
            ),
            "an identity that names nothing must not answer with an empty supertype list"
        );
        assert!(
            matches!(
                published.conforms_to(&a, &missing, SpecializationScope::AnySpecialization),
                QueryOutcome::Unresolved
            ),
            "conformance against an unknown identity is unanswerable, not false"
        );
    }

    #[test]
    fn namespace_import_derived_elements_preserve_canonical_target_outcomes() {
        let published = detail_publication(
            &[
                (
                    "memory://library.sysml",
                    "package Library { part def Imported; }",
                ),
                (
                    "memory://model.sysml",
                    "package Model { import Library::*; part def Owned; }",
                ),
            ],
            ConstructionSchedule::Sequential,
        );
        let model = identity_of(&published, "memory://model.sysml", "Model");
        let library = identity_of(&published, "memory://library.sysml", "Library");
        let values = settled(published.namespace_import_derived_elements(&model));
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].relationship.kind, "namespaceImport");
        assert_eq!(
            values[0].relationship.provenance,
            RelationshipProvenance::Authored
        );
        assert_eq!(
            values[0].relationship.target,
            RelationshipTarget::Resolved(library),
            "the scalar derivation must retain the import reference's canonical target outcome"
        );
    }

    // --- Element details --------------------------------------------------------------------

    fn detail_publication(
        sources: &[(&str, &str)],
        schedule: ConstructionSchedule,
    ) -> PublishedResolution {
        let request = BuildRequest::new(
            sources
                .iter()
                .map(|(identity, source)| {
                    SourceInput::new(*identity, source.to_string(), SourceKind::Workspace)
                })
                .collect(),
            schedule,
            "contract-v1",
        )
        .unwrap();
        build(request).unwrap()
    }

    fn settled<T: fmt::Debug>(outcome: QueryOutcome<T>) -> T {
        match outcome {
            QueryOutcome::Resolved(value)
            | QueryOutcome::Recovered(value)
            | QueryOutcome::UnsupportedWith(value) => value,
            other => panic!("expected a settled outcome, got: {other:?}"),
        }
    }

    fn identity_of(
        published: &PublishedResolution,
        document: &str,
        qualified_name: &str,
    ) -> SymbolIdentity {
        settled(published.document_symbols(document))
            .iter()
            .find(|entry| entry.qualified_name.as_ref() == qualified_name)
            .unwrap_or_else(|| panic!("no declaration named {qualified_name} in {document}"))
            .identity
            .clone()
    }

    fn details_of(
        published: &PublishedResolution,
        document: &str,
        qualified_name: &str,
    ) -> ElementDetails {
        settled(published.element_details(&identity_of(published, document, qualified_name)))
    }

    fn names(entries: &[SymbolEntry]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| entry.name.as_deref().unwrap_or("<anonymous>"))
            .collect()
    }

    /// One deterministic rendering of an element's details, for equivalence assertions.
    fn render_details(details: &ElementDetails) -> String {
        let mut output = String::new();
        let family = |output: &mut String, label: &str, family: &RelationshipFamily| {
            output.push_str(&format!(
                "{label} {} {:?} {:?}\n",
                family.outcome.as_str(),
                names(&family.targets),
                names(&family.candidates)
            ));
        };
        output.push_str(&format!(
            "element {} {}\n",
            details.inspection.qualified_name,
            details.inspection.kind.as_str()
        ));
        output.push_str(&format!(
            "owner {:?}\n",
            details
                .owner
                .as_ref()
                .map(|owner| owner.qualified_name.clone())
        ));
        family(&mut output, "typing", &details.typing);
        family(&mut output, "specialization", &details.specialization);
        family(&mut output, "subsetting", &details.subsetting);
        family(&mut output, "redefinition", &details.redefinition);
        output.push_str(&format!(
            "effective-typing {} {:?}\n",
            details.effective_typing.outcome.as_str(),
            details
                .effective_typing
                .types
                .iter()
                .map(|entry| (
                    entry.element.qualified_name.clone(),
                    format!("{:?}", entry.origin)
                ))
                .collect::<Vec<_>>()
        ));
        output.push_str(&format!(
            "inherited {:?}\n",
            details
                .inherited_features
                .iter()
                .map(|entry| (
                    entry.feature.qualified_name.clone(),
                    entry.declared_in.qualified_name.clone()
                ))
                .collect::<Vec<_>>()
        ));
        output.push_str(&format!("metadata {:?}\n", names(&details.metadata)));
        for (label, connected) in [
            ("incoming", &details.incoming),
            ("outgoing", &details.outgoing),
        ] {
            output.push_str(&format!(
                "{label} {:?}\n",
                connected
                    .iter()
                    .map(|entry| (
                        entry.kind,
                        entry.peer.qualified_name.clone(),
                        format!("{:?}", entry.provenance)
                    ))
                    .collect::<Vec<_>>()
            ));
        }
        output.push_str(&format!("evaluation {}\n", details.evaluation.state));
        output.push_str(&format!("analysis {}\n", details.analysis.as_str()));
        output
    }

    /// Never select the first candidate of an ambiguous reference: every candidate is retained and
    /// the outcome says none was chosen.
    #[test]
    fn an_ambiguous_relationship_target_keeps_every_candidate_and_chooses_none() {
        let published = detail_publication(
            &[(
                "memory://model.sysml",
                concat!(
                    "package P {\n",
                    "  package A { part def Shared; }\n",
                    "  package B { part def Shared; }\n",
                    "  package C { import A::*; import B::*; part unit : Shared; }\n",
                    "}\n",
                ),
            )],
            ConstructionSchedule::Sequential,
        );
        let usage = details_of(&published, "memory://model.sysml", "P::C::unit");
        assert_eq!(usage.typing.outcome, RelationshipOutcome::Ambiguous);
        assert!(
            usage.typing.targets.is_empty(),
            "an ambiguous family must publish no chosen target"
        );
        assert_eq!(usage.typing.candidates.len(), 2, "{:?}", usage.typing);
        assert_eq!(
            usage
                .effective_typing
                .candidates
                .iter()
                .map(|candidate| candidate.element.qualified_name.as_ref())
                .collect::<Vec<_>>(),
            vec!["P::A::Shared", "P::B::Shared"]
        );
        assert_eq!(
            usage.effective_typing.outcome,
            RelationshipOutcome::Ambiguous
        );
    }

    #[test]
    fn view_selection_keeps_unresolved_and_unsupported_predicates_explicit() {
        let document = "memory://views.sysml";
        let published = detail_publication(
            &[(
                document,
                concat!(
                    "package P {\n",
                    "  part candidate;\n",
                    "  view unresolved { filter @Missing; }\n",
                    "  view unsupported { filter 1; }\n",
                    "}\n",
                ),
            )],
            ConstructionSchedule::Sequential,
        );
        let candidate = identity_of(&published, document, "P::candidate");
        let unresolved = identity_of(&published, document, "P::unresolved");
        assert_eq!(
            settled(published.view_selection(&unresolved, &candidate)).outcome,
            ViewSelectionOutcome::Indeterminate(Box::new([
                ViewSelectionObstacle::UnresolvedPredicate
            ]))
        );
        let unsupported = identity_of(&published, document, "P::unsupported");
        assert_eq!(
            settled(published.view_selection(&unsupported, &candidate)).outcome,
            ViewSelectionOutcome::Indeterminate(Box::new([
                ViewSelectionObstacle::UnsupportedPredicate
            ]))
        );
    }

    /// The verdict channel is a projection of the same settled value channel, gated by the
    /// element's kind, so the two cannot disagree.
    #[test]
    fn analysis_evaluation_is_a_second_channel_over_the_settled_value() {
        let published = detail_publication(
            &[(
                "memory://analysis.sysml",
                concat!(
                    "package P {\n",
                    "  attribute plain = 1;\n",
                    "  constraint holds { true }\n",
                    "  constraint fails { false }\n",
                    "  constraint broken { missing }\n",
                    "}\n",
                ),
            )],
            ConstructionSchedule::Sequential,
        );

        let plain = details_of(&published, "memory://analysis.sysml", "P::plain");
        assert_eq!(
            plain.analysis,
            AnalysisEvaluation::NotApplicable,
            "an attribute's value is not a verdict"
        );
        assert_eq!(
            plain.evaluation.state,
            EvaluationState::Literal(EvaluatedScalar::Integer(1))
        );

        assert_eq!(
            details_of(&published, "memory://analysis.sysml", "P::holds").analysis,
            AnalysisEvaluation::Verdict(true)
        );
        assert_eq!(
            details_of(&published, "memory://analysis.sysml", "P::fails").analysis,
            AnalysisEvaluation::Verdict(false)
        );

        let broken = details_of(&published, "memory://analysis.sysml", "P::broken");
        assert!(
            matches!(broken.analysis, AnalysisEvaluation::Unsettled(_)),
            "an unsettled constraint must not read as a failing verdict, got {:?}",
            broken.analysis
        );
    }

    /// A build that does not evaluate reports the verdict channel as not run, which is neither a
    /// verdict nor an inapplicable element.
    #[test]
    fn a_skipped_evaluation_policy_reports_the_verdict_channel_as_not_run() {
        let request = BuildRequest::new(
            vec![SourceInput::new(
                "memory://skip.sysml",
                "package P { constraint holds { true } }".to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap()
        .with_evaluation_policy(EvaluationPolicy::Skip);
        let published = build(request).unwrap();
        let holds = details_of(&published, "memory://skip.sysml", "P::holds");
        assert_eq!(holds.evaluation.state, EvaluationState::NotRun);
        assert_eq!(holds.analysis, AnalysisEvaluation::NotRun);
    }

    const VEHICLE_MODEL: &str = concat!(
        "package P {\n",
        "  metadata def Safety;\n",
        "  part def Wheel;\n",
        "  part def Vehicle {\n",
        "    @Safety;\n",
        "    part wheel[4] : Wheel;\n",
        "    part spare[0..*] : Wheel;\n",
        "  }\n",
        "  part def Rover :> Vehicle {\n",
        "    part :>> wheel[4];\n",
        "  }\n",
        "  part rover : Rover;\n",
        "  part broken : Missing;\n",
        "  part selected subsets rover;\n",
        "}\n",
    );

    /// The cohesive answer and the individual services read the same settled facts, so a consumer
    /// choosing one cannot see a different model from a consumer choosing the other.
    #[test]
    fn element_details_agrees_with_the_services_it_is_assembled_from() {
        let published = detail_publication(
            &[("memory://model.sysml", VEHICLE_MODEL)],
            ConstructionSchedule::Sequential,
        );
        let symbol = identity_of(&published, "memory://model.sysml", "P::rover");
        let details = settled(published.element_details(&symbol));
        assert_eq!(details.inspection, settled(published.inspect(&symbol)));
        assert_eq!(details.evaluation, settled(published.evaluate(&symbol)));
        let effective = settled(published.effective_types(&symbol));
        assert_eq!(
            details
                .effective_typing
                .types
                .iter()
                .map(|entry| entry.element.identity.clone())
                .collect::<Vec<_>>(),
            effective
                .iter()
                .map(|entry| entry.symbol.clone())
                .collect::<Vec<_>>()
        );
    }

    /// The whole point of the publication-time child, reverse-reference and implied indexes: an
    /// element's details are its own facts, so a workspace that grows elsewhere costs nothing here.
    ///
    /// Before the child index existed, the inherited-feature walk filtered every declaration in the
    /// publication per owner it visited, which made one element's answer cost the size of the model.
    #[test]
    fn element_details_cost_is_independent_of_the_rest_of_the_workspace() {
        let cost = |sources: &[(&str, &str)]| {
            let published = publication_for(sources);
            let symbol = identity_of(&published, "memory://i.sysml", "P::w");
            let (outcome, visited) = crate::index::documents::measure_visited_index_entries(|| {
                published.element_details(&symbol)
            });
            assert!(
                matches!(outcome, QueryOutcome::Resolved(_)),
                "the probe must resolve, got: {outcome:?}"
            );
            visited
        };

        let small = cost(&[("memory://i.sysml", PROBED)]);
        let large_source = format!("package Other {{\n{}}}\n", padding(500));
        let large = cost(&[
            ("memory://i.sysml", PROBED),
            ("memory://other.sysml", &large_source),
        ]);
        assert_eq!(
            small, large,
            "500 declarations in another document changed what one element-details answer reads"
        );
    }

    /// Sequential and parallel construction publish the same details, and so do the same sources
    /// admitted in a different order.
    #[test]
    fn construction_strategy_and_source_order_publish_equivalent_details() {
        let sources = [
            ("memory://a.sysml", "package P { part def Wheel; }"),
            (
                "memory://b.sysml",
                "package P { part def Vehicle { part wheel : Wheel; } part car : Vehicle; }",
            ),
        ];
        let permuted = [sources[1], sources[0]];

        let render = |published: &PublishedResolution| {
            ["P::car", "P::Vehicle", "P::Wheel"]
                .iter()
                .map(|name| {
                    let document = if *name == "P::Wheel" {
                        "memory://a.sysml"
                    } else {
                        "memory://b.sysml"
                    };
                    render_details(&details_of(published, document, name))
                })
                .collect::<Vec<_>>()
        };

        let sequential = detail_publication(&sources, ConstructionSchedule::Sequential);
        let parallel = detail_publication(&sources, ConstructionSchedule::Parallel);
        let reordered = detail_publication(&permuted, ConstructionSchedule::Sequential);
        assert_eq!(render(&sequential), render(&parallel));
        assert_eq!(render(&sequential), render(&reordered));
    }

    /// A publication that reuses a solved library stratum answers exactly as a full solve does.
    #[test]
    fn library_stratum_reuse_publishes_the_same_details_as_a_full_solve() {
        let library = SourceInput::new(
            "memory://lib.sysml",
            "standard library package Lib { part def Wheel; }".to_string(),
            SourceKind::StandardLibrary,
        );
        let workspace = SourceInput::new(
            "memory://model.sysml",
            "package W { part w : Lib::Wheel; }".to_string(),
            SourceKind::Workspace,
        );

        let full = build(
            BuildRequest::new(
                vec![library.clone(), workspace.clone()],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();
        let stratum = std::sync::Arc::new(build_library_stratum(vec![library]).unwrap());
        let warm = build(
            BuildRequest::with_library(
                vec![workspace],
                ConstructionSchedule::Sequential,
                "contract-v1",
                stratum,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            render_details(&details_of(&full, "memory://model.sysml", "W::w")),
            render_details(&details_of(&warm, "memory://model.sysml", "W::w")),
        );
    }

    fn part_definition_library() -> SourceInput {
        SourceInput::new(
            "memory://parts.sysml",
            concat!(
                "standard library package Parts { ",
                "part def Part; ",
                "part def Vehicle specializes Part; ",
                "}"
            )
            .to_string(),
            SourceKind::StandardLibrary,
        )
    }

    fn part_definition_workspace() -> SourceInput {
        SourceInput::new(
            "memory://model.sysml",
            concat!(
                "package Model { import Parts::*; ",
                "part def Component; ",
                "part def Equivalent specializes Part; ",
                "part def Specific specializes Vehicle; ",
                "}"
            )
            .to_string(),
            SourceKind::Workspace,
        )
    }

    fn specialization_relationships(
        published: &PublishedResolution,
        document: &str,
        qualified_name: &str,
    ) -> Vec<ElementRelationship> {
        settled(published.inspect(&identity_of(published, document, qualified_name)))
            .relationships
            .into_vec()
            .into_iter()
            .filter(|relationship| relationship.kind == "specialization")
            .collect()
    }

    fn type_featuring_relationships(
        published: &PublishedResolution,
        document: &str,
        qualified_name: &str,
    ) -> Vec<ElementRelationship> {
        settled(published.inspect(&identity_of(published, document, qualified_name)))
            .relationships
            .into_vec()
            .into_iter()
            .filter(|relationship| relationship.kind == "typeFeaturing")
            .collect()
    }

    #[test]
    fn feature_relationship_collection_keeps_an_unresolved_canonical_edge_visible() {
        let published = detail_publication(
            &[(
                "memory://model.sysml",
                "package Model { classifier Vehicle { feature derived chains Missing; } }",
            )],
            ConstructionSchedule::Sequential,
        );
        let derived = identity_of(
            &published,
            "memory://model.sysml",
            "Model::Vehicle::derived",
        );
        assert!(matches!(
            settled(published.feature_derived_relationships(
                &derived,
                FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
            ))
            .as_ref(),
            [ElementRelationship {
                kind: "featureChaining",
                target: RelationshipTarget::Unresolved,
                provenance: RelationshipProvenance::Authored,
                ..
            }]
        ));
    }

    #[test]
    fn exact_type_relationship_collections_project_canonical_authored_and_unresolved_facts() {
        let published = detail_publication(
            &[ (
                "memory://model.sysml",
                "package Model { classifier Base; classifier Derived specializes Base unions Base intersects Base differences Base disjoint from Base; classifier Partial unions Missing; }",
            ) ],
            ConstructionSchedule::Sequential,
        );
        let base = identity_of(&published, "memory://model.sysml", "Model::Base");
        let derived = identity_of(&published, "memory://model.sysml", "Model::Derived");
        let partial = identity_of(&published, "memory://model.sysml", "Model::Partial");
        let values = |collection| {
            settled(published.type_derived_relationships(&derived, collection)).into_vec()
        };
        for (collection, kind) in [
            (
                TypeDerivedRelationshipCollection::OwnedSpecialization,
                "specialization",
            ),
            (TypeDerivedRelationshipCollection::OwnedUnioning, "unioning"),
            (
                TypeDerivedRelationshipCollection::OwnedIntersecting,
                "intersecting",
            ),
            (
                TypeDerivedRelationshipCollection::OwnedDifferencing,
                "differencing",
            ),
            (
                TypeDerivedRelationshipCollection::OwnedDisjoining,
                "disjoining",
            ),
            (TypeDerivedRelationshipCollection::UnioningType, "unioning"),
            (
                TypeDerivedRelationshipCollection::IntersectingType,
                "intersecting",
            ),
            (
                TypeDerivedRelationshipCollection::DifferencingType,
                "differencing",
            ),
        ] {
            assert!(matches!(
                values(collection).as_slice(),
                [ElementRelationship {
                    kind: actual_kind,
                    provenance: RelationshipProvenance::Authored,
                    target: RelationshipTarget::Resolved(target),
                    ..
                }] if *actual_kind == kind && target == &base
            ));
        }
        assert!(matches!(
            settled(published.type_derived_relationships(
                &partial,
                TypeDerivedRelationshipCollection::UnioningType,
            ))
            .as_ref(),
            [ElementRelationship {
                kind: "unioning",
                provenance: RelationshipProvenance::Authored,
                target: RelationshipTarget::Unresolved,
                ..
            }]
        ));
    }

    #[test]
    fn variable_feature_membership_is_explicitly_unsupported_without_snapshots() {
        let published = detail_publication(
            &[(
                "memory://model.sysml",
                "package Model { classifier Vehicle { var feature mass; } }",
            )],
            ConstructionSchedule::Sequential,
        );
        let mass = identity_of(&published, "memory://model.sysml", "Model::Vehicle::mass");
        assert!(matches!(
            published.featuring_types(&mass),
            QueryOutcome::Unsupported
        ));
        assert!(matches!(
            published.featuring_type(&mass),
            QueryOutcome::Unsupported
        ));
        assert!(type_featuring_relationships(
            &published,
            "memory://model.sysml",
            "Model::Vehicle::mass"
        )
        .is_empty());
    }

    #[test]
    fn polarity_branch_anchor_failures_are_explicit_and_deduplicated() {
        const RULE: &str = "sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageSpecialization";
        let workspace = || {
            SourceInput::new(
                "memory://model.sysml",
                "package Model { requirement def Safety; part def Vehicle; not satisfy Safety by Vehicle; not satisfy Safety by Vehicle; }".to_string(),
                SourceKind::Workspace,
            )
        };
        let missing = build(
            BuildRequest::new(
                vec![
                    SourceInput::new(
                        "memory://requirements.sysml",
                        "standard library package Requirements { constraint def satisfiedRequirementChecks; }".to_string(),
                        SourceKind::StandardLibrary,
                    ),
                    workspace(),
                ],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            missing.library_specialization_anchor_branch(
                RULE,
                LibrarySpecializationAnchorBranch::PredicateTrue,
            ),
            QueryOutcome::Unresolved
        ));
        let missing_published_diagnostics = missing.diagnostics();
        let missing_diagnostics = missing_published_diagnostics
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code.as_str() == "missing_library_anchor"
                    && diagnostic
                        .message
                        .contains("Requirements::notSatisfiedRequirementChecks")
            })
            .collect::<Vec<_>>();
        assert_eq!(missing_diagnostics.len(), 1);

        let ambiguous = build(
            BuildRequest::new(
                vec![
                    SourceInput::new(
                        "memory://requirements-a.sysml",
                        "standard library package Requirements { constraint def notSatisfiedRequirementChecks; }".to_string(),
                        SourceKind::StandardLibrary,
                    ),
                    SourceInput::new(
                        "memory://requirements-b.sysml",
                        "standard library package Requirements { constraint def notSatisfiedRequirementChecks; }".to_string(),
                        SourceKind::StandardLibrary,
                    ),
                    workspace(),
                ],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            ambiguous.library_specialization_anchor_branch(
                RULE,
                LibrarySpecializationAnchorBranch::PredicateTrue,
            ),
            QueryOutcome::Ambiguous(candidates) if candidates.len() == 2
        ));
        let ambiguous_published_diagnostics = ambiguous.diagnostics();
        let ambiguous_diagnostics = ambiguous_published_diagnostics
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code.as_str() == "ambiguous_library_anchor"
                    && diagnostic
                        .message
                        .contains("Requirements::notSatisfiedRequirementChecks")
            })
            .collect::<Vec<_>>();
        assert_eq!(ambiguous_diagnostics.len(), 1);
        assert_eq!(ambiguous_diagnostics[0].related.len(), 2);
    }

    /// Anchor failures remain typed published states and report one actionable cause, rather than
    /// one warning per `part def` or a guessed workspace substitute.
    #[test]
    fn part_definition_anchor_failures_are_explicit_and_report_one_root_cause() {
        let missing_library = SourceInput::new(
            "memory://incomplete-standard.sysml",
            "standard library package NotParts {}".to_string(),
            SourceKind::StandardLibrary,
        );
        let workspace = SourceInput::new(
            "memory://model.sysml",
            "package Model { part def Component; part def Other; }".to_string(),
            SourceKind::Workspace,
        );
        let missing = build(
            BuildRequest::new(
                vec![missing_library, workspace.clone()],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            missing.part_definition_specialization_anchor(),
            QueryOutcome::Unresolved
        ));
        assert_eq!(
            missing
                .diagnostics()
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.as_str())
                .collect::<Vec<_>>(),
            vec!["missing_library_anchor"]
        );
        assert!(
            specialization_relationships(&missing, "memory://model.sysml", "Model::Component")
                .is_empty()
        );
        assert_eq!(
            missing.diagnostics().diagnostics[0].category(),
            DiagnosticCategory::MissingContext
        );

        let ambiguous = build(
            BuildRequest::new(
                vec![
                    SourceInput::new(
                        "memory://parts-a.sysml",
                        "standard library package Parts { part def Part; }".to_string(),
                        SourceKind::StandardLibrary,
                    ),
                    SourceInput::new(
                        "memory://parts-b.sysml",
                        "standard library package Parts { part def Part; }".to_string(),
                        SourceKind::StandardLibrary,
                    ),
                    workspace,
                ],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            ambiguous.part_definition_specialization_anchor(),
            QueryOutcome::Ambiguous(candidates) if candidates.len() == 2
        ));
        let diagnostics = ambiguous.diagnostics().diagnostics;
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.as_str(), "ambiguous_library_anchor");
        assert_eq!(diagnostics[0].category(), DiagnosticCategory::Ambiguous);
        assert_eq!(diagnostics[0].related.len(), 2);
    }

    /// A missing generated anchor is reported once for every affected document and anchor, not
    /// once per matching declaration. The stored rule outcome remains the query result.
    #[test]
    fn generated_library_anchor_diagnostics_deduplicate_by_anchor_and_document() {
        const ITEM_RULE: &str = "sysml-2.0:8.3.10.2:checkItemDefinitionSpecialization";
        let published = build(
            BuildRequest::new(
                vec![
                    SourceInput::new(
                        "memory://incomplete.sysml",
                        "standard library package Incomplete {}".to_string(),
                        SourceKind::StandardLibrary,
                    ),
                    SourceInput::new(
                        "memory://model.sysml",
                        "package Model { item def First; item def Second; }".to_string(),
                        SourceKind::Workspace,
                    ),
                ],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();

        assert!(matches!(
            published.library_specialization_anchor(ITEM_RULE),
            QueryOutcome::Unresolved
        ));
        let diagnostics = published.diagnostics().diagnostics;
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.as_str(), "missing_library_anchor");
        assert!(diagnostics[0].message.contains("Items::Item"));
    }

    /// A cross-document reference is a resolved relationship with the target's own document, not a
    /// name the consumer has to look up.
    #[test]
    fn element_details_resolve_a_cross_document_relationship_to_its_declaring_document() {
        let published = detail_publication(
            &[
                (
                    "memory://defs.sysml",
                    "package Defs { requirement def Endurance; }",
                ),
                (
                    "memory://usage.sysml",
                    "package Usage { import Defs::*; requirement check : Endurance; }",
                ),
            ],
            ConstructionSchedule::Sequential,
        );
        let check = details_of(&published, "memory://usage.sysml", "Usage::check");
        assert_eq!(check.typing.outcome, RelationshipOutcome::Resolved);
        assert_eq!(
            check.typing.targets[0].location.document.as_ref(),
            "memory://defs.sysml"
        );
    }

    /// Recovery-produced input is still answered, and the outcome says the publication recovered
    /// rather than presenting the answer as complete.
    #[test]
    fn element_details_over_recovery_produced_input_keep_their_recovery_outcome() {
        let published = detail_publication(
            &[(
                "memory://recovery.sysml",
                "package P { part def Wheel; part broken : ; }",
            )],
            ConstructionSchedule::Sequential,
        );
        let symbol = identity_of(&published, "memory://recovery.sysml", "P::Wheel");
        assert!(
            matches!(
                published.element_details(&symbol),
                QueryOutcome::Recovered(_) | QueryOutcome::UnsupportedWith(_)
            ),
            "expected a degraded publication to say so, got: {:?}",
            published.completeness()
        );
    }

    /// A position identifies two different elements, and both are answered in full.
    #[test]
    fn element_details_at_a_position_answer_the_container_and_the_reference_separately() {
        let published = detail_publication(
            &[(
                "memory://at.sysml",
                "package P {\n  part def Engine;\n  part motor : Engine;\n}\n",
            )],
            ConstructionSchedule::Sequential,
        );
        let at = settled(published.element_details_at(
            "memory://at.sysml",
            TextPosition {
                line: 2,
                character: 15,
            },
        ));
        assert_eq!(
            at.containing
                .as_ref()
                .and_then(|details| details.inspection.name.as_deref()),
            Some("motor")
        );
        match &at.referenced {
            ReferencedDetails::Resolved(details) => {
                assert_eq!(details.inspection.name.as_deref(), Some("Engine"))
            }
            other => panic!("expected the reference under the cursor, got: {other:?}"),
        }

        // A position with no reference under it says so rather than reporting an unresolved one.
        let at = settled(published.element_details_at(
            "memory://at.sysml",
            TextPosition {
                line: 2,
                character: 8,
            },
        ));
        assert_eq!(at.referenced, ReferencedDetails::None);
    }

    #[test]
    fn affected_documents_are_transitive_across_public_imports_and_aliases() {
        let sources = vec![
            SourceInput::new(
                "memory://a.sysml",
                "package A { part def T; }".into(),
                SourceKind::Workspace,
            ),
            SourceInput::new(
                "memory://b.sysml",
                "package B { public import A::*; alias AliasT for T; }".into(),
                SourceKind::Workspace,
            ),
            SourceInput::new(
                "memory://c.sysml",
                "package C { import B::*; part p : AliasT; }".into(),
                SourceKind::Workspace,
            ),
        ];
        let published = build(
            BuildRequest::new(sources, ConstructionSchedule::Sequential, "contract-v1").unwrap(),
        )
        .unwrap();
        let QueryOutcome::Resolved(affected) = published.affected_documents("memory://a.sysml")
        else {
            panic!("complete imports must publish a settled dependency outcome")
        };
        assert_eq!(
            affected
                .iter()
                .map(|document| document.identity.as_ref())
                .collect::<Vec<_>>(),
            vec!["memory://b.sysml", "memory://c.sysml"]
        );
    }

    #[test]
    fn an_unresolved_import_makes_dependency_selection_explicitly_recovered() {
        let published = build(
            BuildRequest::new(
                vec![SourceInput::new(
                    "memory://a.sysml",
                    "package A { import Missing::*; }".into(),
                    SourceKind::Workspace,
                )],
                ConstructionSchedule::Sequential,
                "contract-v1",
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            published.affected_documents("memory://a.sysml"),
            QueryOutcome::Recovered(_)
        ));
    }
}

#[cfg(test)]
mod parsed_admission_tests {
    use super::*;
    use sysml_source::{SourceAuthority, SourceKind};

    /// Admitting a parsed handle and admitting its text are the same publication: same identity,
    /// same manifest, same model digest. The examples corpus is the evidence.
    #[test]
    fn parsed_and_text_admission_publish_the_same_identity_over_the_examples() {
        let examples = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let documents = SourceAuthority::new()
            .list(&[examples], SourceKind::Workspace)
            .expect("examples corpus")
            .documents;
        assert!(documents.len() > 5, "examples corpus present");
        let authority = syntax::SyntaxAuthority::new();

        let text = documents
            .iter()
            .map(|document| {
                SourceInput::new(
                    document.uri().as_str(),
                    document.content().to_owned(),
                    document.kind(),
                )
            })
            .collect();
        let parsed = documents
            .iter()
            .map(|document| {
                SourceInput::from_parsed(
                    document.uri().as_str(),
                    authority.parse(document),
                    document.kind(),
                )
            })
            .collect();

        let from_text = BuildRequest::new(text, ConstructionSchedule::Parallel, "test").unwrap();
        let from_parsed =
            BuildRequest::new(parsed, ConstructionSchedule::Parallel, "test").unwrap();
        assert_eq!(from_text.identity(), from_parsed.identity());

        let (text_model, _) = build_measured(from_text).unwrap();
        let (parsed_model, parsed_timing) = build_measured(from_parsed).unwrap();
        assert_eq!(text_model.identity(), parsed_model.identity());
        assert!(
            parsed_timing.parse < std::time::Duration::from_millis(5),
            "handles are admitted without a parse: {:?}",
            parsed_timing.parse
        );
    }
}
