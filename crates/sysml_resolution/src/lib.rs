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
    DocumentId, DocumentToken, ElementKind, ElementSearch, ElementSource,
    LibrarySpecializationAnchorBranch, MembershipRole, OccurrenceRole, PublicationCompleteness,
    RequirementConstraintKind, StateSubactionKind, SymbolId, SymbolToken, TextId, TextPosition,
    TextRange,
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
    DiagramRelationshipEndpoint, DiagramRelationshipKind, DiagramRelationshipTarget, DiagramScene,
    DiagramSemanticReference, DiagramStateTransition, DiagramStateTransitionScene,
    DiagramStateVertex, DiagramStateVertexKind, DiagramTransitionFeature, DiagramViewCatalogEntry,
    DiagramViewKind, DiagramViewProjection,
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
pub use model::query::VisibleMemberRef;
pub use model::query::VisibleMembers;
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
    /// How many admitted documents this build lowered itself: a memo miss, or no memo at all.
    ///
    /// `documents_lowered + documents_reused` is every admitted document, library and workspace
    /// alike. A counted fact rather than a timing, so a caller can state that one edit lowered
    /// exactly one document without reading a clock.
    pub documents_lowered: usize,
    /// How many admitted documents this build took from the lowering memo unchanged.
    ///
    /// Reuse is keyed by content digest at every provenance. It cannot change what is built: the
    /// spliced product is the one this document's own walk produced, relocated.
    pub documents_reused: usize,
    /// How many admitted sources this build parsed itself.
    ///
    /// Sources admitted as parsed handles are not counted: they enter the build as the trees the
    /// syntax authority already produced. This is the typed fact that a pre-parsed admission did
    /// not re-parse — the durations above cannot state it without a wall-clock threshold.
    pub sources_parsed: usize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    /// The document this fact was found in, as a handle into *this* publication.
    ///
    /// Not a URI: the identity string is materialised on demand by
    /// [`PublishedResolution::document_identity`] (borrowed) or
    /// [`PublishedResolution::document_token`] (owned, boundary-crossing). A location is one of
    /// the densest results the facade returns -- every reference, every rename occurrence -- and
    /// a copy of the URI per occurrence is an allocation per keystroke that most consumers group
    /// away again immediately.
    pub document: DocumentId,
    pub range: TextRange,
    pub role: OccurrenceRole,
}

/// One navigation candidate: which element, and where it was found.
///
/// The name is not carried. It is a copy of text the publication already stores, and a
/// definition query that returns a handful of candidates would allocate one per candidate for a
/// string most consumers only need at the editor edge. Read it there with
/// [`PublishedResolution::symbol_name`], which borrows from the settled symbol blob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationTarget {
    pub symbol: SymbolId,
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
        symbol: SymbolId,
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
    /// The memo unchanged documents take their lowering product from; absent, every document is
    /// lowered by this build. Never observable from a publication: `design.md` keeps memos behind
    /// service handles, and the only trace a consumer sees is the counted facts on
    /// [`BuildMeasurements`].
    lowering: Option<std::sync::Arc<lower::memo::LoweringMemo>>,
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
    build_library_stratum_memoized(sources, syntax, None)
}

/// [`build_library_stratum_with`] reusing the lowering product of every unchanged document.
///
/// Crate-private: the memo is the publication authority's, and `design.md` keeps memos behind
/// service handles rather than in a build entry point a consumer can call.
pub(crate) fn build_library_stratum_memoized(
    sources: Vec<SourceInput>,
    syntax: Option<std::sync::Arc<syntax::SyntaxAuthority>>,
    lowering: Option<std::sync::Arc<lower::memo::LoweringMemo>>,
) -> Result<LibraryStratum, BuildFailure> {
    let mut request = BuildRequest::new(
        sources,
        ConstructionSchedule::Parallel,
        LIBRARY_STRATUM_CONTRACT,
    )?;
    request.syntax = syntax;
    request.lowering = lowering;
    let manifest_entries = request.sources.iter().map(manifest_entry).collect();
    let identities = request
        .sources
        .iter()
        .map(|source| source.identity.clone())
        .collect();
    let (published, _, sources) = build_parts(request)?;
    let prepared = published
        .model
        .prepared_library(sources)
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
            lowering: None,
        })
    }

    /// Parse pending sources through `syntax`'s memo rather than cold.
    pub fn with_syntax(mut self, syntax: std::sync::Arc<syntax::SyntaxAuthority>) -> Self {
        self.syntax = Some(syntax);
        self
    }

    /// Builds through `memo`, reusing the lowering product of every document whose content is
    /// unchanged. Owned by the publication authority; not part of the request's identity, because
    /// reuse cannot change what is built.
    pub(crate) fn with_lowering(mut self, memo: std::sync::Arc<lower::memo::LoweringMemo>) -> Self {
        self.lowering = Some(memo);
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
    build_parts(request).map(|(publication, measurements, _)| (publication, measurements))
}

/// [`build_measured`] plus the build's own parse product, which only a library-stratum build keeps.
///
/// The publication never holds it: `design.md` gives the parse tree to the syntax service, and the
/// stratum is the one consumer that legitimately reuses it.
fn build_parts(
    request: BuildRequest,
) -> Result<
    (
        PublishedResolution,
        BuildMeasurements,
        crate::lower::storage::ParsedSources,
    ),
    BuildFailure,
> {
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
            digest: source.content_digest,
            payload: source.payload,
            syntax: syntax.clone(),
        })
        .collect();
    let (model, sources, measurements) =
        SemanticModelBuildCoordinator::build_measured_with_library(
            sources,
            schedule,
            request.policy,
            request.library.as_deref().map(|library| &library.prepared),
            &request.reported,
            request.lowering.as_deref(),
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
            sources_parsed: measurements.sources_parsed,
            documents_lowered: measurements.documents_lowered,
            documents_reused: measurements.documents_reused,
        },
        sources,
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
    pub fn diagnostics(&self) -> PublishedDiagnostics<'_> {
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
    pub fn document_diagnostics(&self, document: &str) -> PublishedDiagnostics<'_> {
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
        symbol: SymbolId,
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
    ) -> QueryOutcome<VisibleMembers<'_>> {
        self.model.visible_members(document, position, qualifier)
    }

    /// The settled evaluation of one element: value, state, authored units and required
    /// measurement, from facts this publication fixed before it became visible.
    pub fn evaluate(&self, symbol: SymbolId) -> QueryOutcome<ElementEvaluation> {
        self.model.evaluate(symbol)
    }

    /// Everything this publication knows about one element.
    pub fn inspect(&self, symbol: SymbolId) -> QueryOutcome<ElementInspection> {
        self.model.inspect(symbol)
    }

    /// The authored name of one element, borrowed from this publication.
    ///
    /// `None` where the element is anonymous. Results that identify elements carry the handle,
    /// not the text; this is where a consumer that has to show a name reads one, at the cost of
    /// a slice.
    pub fn symbol_name(&self, symbol: SymbolId) -> Option<&str> {
        self.model.symbol_name(symbol)
    }

    /// The `::`-joined owner path of one element, borrowed from this publication.
    ///
    /// A display convenience, not an identity: an anonymous ancestor contributes an empty
    /// segment, so two elements can share a qualified name. The path is settled at the barrier,
    /// so this costs a slice, not an allocation; [`PublishedResolution::symbol_token`] is what a
    /// consumer takes when it needs to keep something.
    pub fn qualified_name(&self, symbol: SymbolId) -> Option<&str> {
        self.model.symbol_qualified_name(symbol)
    }

    /// One run of authored text, borrowed from this publication.
    ///
    /// Published facts carry a [`TextId`] rather than a copy of the text the publication already
    /// interned; this is where a consumer that has to render one reads it, at the cost of a
    /// slice. `None` for a handle this publication never minted.
    pub fn text(&self, id: TextId) -> Option<&str> {
        self.model.text(id)
    }

    /// The stable, serialisable form of one element handle.
    ///
    /// A [`SymbolId`] addresses a slot in *this* publication and must not outlive it. A
    /// [`SymbolToken`] is derived from the element's structure, so it is equal across builds of
    /// the same sources and is what crosses a process or protocol boundary. Materialising one
    /// walks the owner chain and allocates: it is a boundary operation, asked for explicitly.
    pub fn symbol_token(&self, symbol: SymbolId) -> Option<SymbolToken> {
        self.model.symbol_token(symbol)
    }

    /// The handle a token names in this publication, if it still names one.
    ///
    /// The inverse of [`PublishedResolution::symbol_token`]; `None` when the element the token
    /// describes is not in this publication.
    pub fn resolve_token(&self, token: &SymbolToken) -> Option<SymbolId> {
        self.model.resolve_token(token)
    }

    /// The normalised identity of one document, borrowed from the publication.
    ///
    /// The identity is settled at the barrier, so this costs a slice, not an allocation;
    /// [`PublishedResolution::document_token`] is what a consumer takes when it needs to keep
    /// one. `None` when the handle is not one of this publication's.
    pub fn document_identity(&self, document: DocumentId) -> Option<&str> {
        self.model.document_identity(document)
    }

    /// The stable, serialisable form of one document handle.
    ///
    /// A [`DocumentId`] addresses a slot in *this* publication; a [`DocumentToken`] carries the
    /// normalised identity itself -- byte-for-byte what `document_identity` borrows -- so it is
    /// equal across builds and is what crosses a process or protocol boundary. It allocates: a
    /// consumer that only wants to display or compare takes `document_identity`.
    pub fn document_token(&self, document: DocumentId) -> Option<DocumentToken> {
        self.model.document_token(document)
    }

    /// The handle a document token names in this publication, if it still names one.
    pub fn resolve_document_token(&self, token: &DocumentToken) -> Option<DocumentId> {
        self.model.resolve_document_token(token)
    }

    /// The handle for a document identity a consumer already holds as text.
    ///
    /// The boundary a host crosses when it starts from a URI -- an editor request names a
    /// document by string, and everything after the lookup is handles.
    pub fn document_of(&self, identity: &str) -> Option<DocumentId> {
        self.model.document_of(identity)
    }

    /// The exact derived `Element::owner` fact from the canonical ownership structure.
    pub fn derived_element_owner(&self, symbol: SymbolId) -> QueryOutcome<DerivedElementOwner> {
        self.model.derived_element_owner(symbol)
    }

    /// One exact derived `Element` documentation collection from canonical documentation facts.
    pub fn element_derived_documentation(
        &self,
        symbol: SymbolId,
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
    pub fn element_details(&self, symbol: SymbolId) -> QueryOutcome<ElementDetails> {
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
        symbol: SymbolId,
        collection: FeatureDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        self.model.feature_derived_relationships(symbol, collection)
    }

    /// One exact Type relationship collection or operand projection from the canonical
    /// relationship store.
    pub fn type_derived_relationships(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        self.model.type_derived_relationships(symbol, collection)
    }

    /// One exact Type element-valued derivation over canonical declaration ownership and
    /// membership facts. The result never creates a public Membership relationship identity.
    pub fn type_derived_elements(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.type_derived_elements(symbol, collection)
    }

    /// One exact Type derivation with an explicit unavailable-fact outcome where no canonical
    /// result owner exists yet.
    pub fn type_derived_fact(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedFactCollection,
    ) -> QueryOutcome<TypeDerivedFactOutcome> {
        self.model.type_derived_fact(symbol, collection)
    }

    /// One exact Systems::DefinitionAndUsage derivation selected by the manifest-owned closed
    /// kind. Direct owner/member projections are resolved from canonical facts; inherited,
    /// variant, and time-variation predicates retain a typed unavailable-fact outcome.
    pub fn definition_usage_derived(
        &self,
        symbol: SymbolId,
        kind: DefinitionUsageDerivedKind,
    ) -> QueryOutcome<DefinitionUsageDerivedOutcome> {
        self.model.definition_usage_derived(symbol, kind)
    }

    pub fn action_derived_fact(
        &self,
        symbol: SymbolId,
        collection: ActionDerivedFactCollection,
    ) -> QueryOutcome<ActionDerivedFactOutcome> {
        self.model.action_derived_fact(symbol, collection)
    }

    /// One exact Systems::Requirements derivation over the publication's canonical membership
    /// roles or documentation records.
    pub fn requirement_derived_fact(
        &self,
        symbol: SymbolId,
        collection: RequirementDerivedFactCollection,
    ) -> QueryOutcome<RequirementDerivedFactOutcome> {
        self.model.requirement_derived_fact(symbol, collection)
    }

    /// The rule-scoped outcome for one closed exact TypeFeaturing check, derived only from the
    /// canonical FeatureMembership and TypeFeaturing publication.
    pub fn type_featuring_check(
        &self,
        symbol: SymbolId,
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
        symbol: SymbolId,
        collection: NamespaceDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.namespace_derived_elements(symbol, collection)
    }

    /// Exact `NamespaceImport::importedElement` projections for the direct anonymous imports a
    /// Namespace owns. Each result carries the canonical import identity and target outcome.
    pub fn namespace_import_derived_elements(
        &self,
        symbol: SymbolId,
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
    pub fn effective_features(&self, symbol: SymbolId) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.effective_features(symbol)
    }

    /// Applies every owned and inherited condition of `view` to one candidate element.
    pub fn view_selection(
        &self,
        view: SymbolId,
        candidate: SymbolId,
    ) -> QueryOutcome<ViewSelection> {
        self.model.view_selection(view, candidate)
    }

    /// The types a feature declares.
    pub fn direct_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[TypeReference]>> {
        self.model.direct_types(symbol)
    }

    pub fn requirement_usage_typing(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<RequirementUsageTyping> {
        self.model.requirement_usage_typing(symbol)
    }

    /// The types a feature has, directly or inherited along its subsetting/redefinition chain.
    pub fn effective_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[EffectiveType]>> {
        self.model.effective_types(symbol)
    }

    /// The canonical standard-library target used to satisfy
    /// `checkPartDefinitionSpecialization`.
    ///
    /// This is the semantic owner's typed anchor outcome, not a lookup reconstructed from a
    /// display name. A missing library is `Unresolved`; competing library declarations are
    /// returned as `Ambiguous` candidates.
    pub fn part_definition_specialization_anchor(&self) -> QueryOutcome<SymbolId> {
        self.model.part_definition_specialization_anchor()
    }

    /// The canonical anchor outcome for one generated unconditional library-specialization rule.
    pub fn library_specialization_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolId> {
        self.model.library_specialization_anchor(rule_id)
    }

    /// The canonical anchor outcome for one typed branch of a generated conditional
    /// specialization rule. [`LibrarySpecializationAnchorBranch::Default`] is the compatible
    /// single-anchor view used by [`Self::library_specialization_anchor`].
    pub fn library_specialization_anchor_branch(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> QueryOutcome<SymbolId> {
        self.model
            .library_specialization_anchor_branch(rule_id, branch)
    }

    /// The canonical anchor outcome for any generated exact library rule.
    ///
    /// Unlike [`Self::library_specialization_anchor`], this includes generated
    /// `redefinesFromLibrary` contracts. The stable manifest rule ID is the only selector;
    /// callers cannot recover a rule from a display name or metaclass spelling.
    pub fn library_rule_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolId> {
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
        symbol: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.direct_supertypes(symbol, scope)
    }

    /// Every supertype, reflexively including `symbol` itself.
    pub fn all_supertypes(
        &self,
        symbol: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.all_supertypes(symbol, scope)
    }

    /// The declarations one specialization edge below `symbol`.
    pub fn direct_subtypes(
        &self,
        symbol: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.direct_subtypes(symbol, scope)
    }

    /// The type that features `symbol`, if any.
    pub fn featuring_type(&self, symbol: SymbolId) -> QueryOutcome<Option<SymbolId>> {
        self.model.featuring_type(symbol)
    }

    /// Every effective TypeFeaturing target, retaining whether it was authored or implied.
    ///
    /// A variable FeatureMembership without a canonical `snapshots` prerequisite is explicitly
    /// unsupported rather than treated as an unfeatured ordinary member.
    pub fn featuring_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[TypeReference]>> {
        self.model.featuring_types(symbol)
    }

    /// Whether `specific` conforms to `general` (KerML §8.4.3.2), reflexively and transitively.
    pub fn conforms_to(
        &self,
        specific: SymbolId,
        general: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Conformance> {
        self.model.conforms_to(specific, general, scope)
    }

    /// Whether the specific feature's types conform to the general feature's (KerML §7.4.12).
    pub fn feature_typing_conforms(
        &self,
        specific: SymbolId,
        general: SymbolId,
    ) -> QueryOutcome<Conformance> {
        self.model.feature_typing_conforms(specific, general)
    }

    /// Both halves of the subsetting rule (KerML §8.4.3.4), reported separately.
    pub fn subsetting_conforms(
        &self,
        subsetting: SymbolId,
        subsetted: SymbolId,
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
            published.references(symbol, false)
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
    ) -> SymbolId {
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
            crate::index::documents::measure_visited_index_entries(|| published.evaluate(symbol));
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
            crate::index::documents::measure_visited_index_entries(|| published.evaluate(symbol))
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
                published.evaluate(symbol)
            });
            let inspection = published.inspect(symbol);
            (evaluation, inspection, cost)
        };
        let inspection_first = {
            let published = publication_for(&[("memory://e.sysml", EVALUATED)]);
            let symbol = probe_symbol(&published, EVALUATED, "memory://e.sysml", "1750");
            let inspection = published.inspect(symbol);
            let (evaluation, cost) = crate::index::documents::measure_visited_index_entries(|| {
                published.evaluate(symbol)
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

    // --- Type queries -------------------------------------------------------------------------
    //
    // The `# TYPES` snapshot section already shows the published facts these queries read. What it
    // cannot show is the rules layered over them: reflexivity, scope selection, what a cycle does
    // to an answer, and the two conformance rules' treatment of untyped and unrelated features.

    fn symbol_named(published: &PublishedResolution, document: &str, qualified: &str) -> SymbolId {
        match published.document_symbols(document) {
            QueryOutcome::Resolved(entries)
            | QueryOutcome::Recovered(entries)
            | QueryOutcome::UnsupportedWith(entries) => {
                entries
                    .iter()
                    .find(|entry| published.qualified_name(entry.identity) == Some(qualified))
                    .unwrap_or_else(|| panic!("no declaration named {qualified}"))
                    .identity
            }
            other => panic!("expected document symbols, got: {other:?}"),
        }
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
        // A handle ranking past the end of the publication: the only stale handle the authority
        // can tell apart from a live one. `SymbolId` documents why the token form exists for the
        // rest.
        let missing =
            SymbolId::from_index(u32::MAX as usize - 1).expect("a handle beyond any publication");

        assert!(
            matches!(
                published.direct_supertypes(missing, SpecializationScope::AnySpecialization),
                QueryOutcome::Unresolved
            ),
            "an identity that names nothing must not answer with an empty supertype list"
        );
        assert!(
            matches!(
                published.conforms_to(a, missing, SpecializationScope::AnySpecialization),
                QueryOutcome::Unresolved
            ),
            "conformance against an unknown identity is unanswerable, not false"
        );
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
    ) -> SymbolId {
        settled(published.document_symbols(document))
            .iter()
            .find(|entry| published.qualified_name(entry.identity) == Some(qualified_name))
            .unwrap_or_else(|| panic!("no declaration named {qualified_name} in {document}"))
            .identity
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
                published.element_details(symbol)
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

        let (text_model, text_measurements) = build_measured(from_text).unwrap();
        let (parsed_model, parsed_measurements) = build_measured(from_parsed).unwrap();
        assert_eq!(text_model.identity(), parsed_model.identity());
        // The text admission parses every document; the handle admission parses none. A counted
        // fact, not a wall-clock budget: it cannot be met by a machine that happens to be idle.
        assert_eq!(
            text_measurements.sources_parsed,
            documents.len(),
            "text admission parses every admitted document"
        );
        assert_eq!(
            parsed_measurements.sources_parsed, 0,
            "handles are admitted without a parse"
        );
    }
}
