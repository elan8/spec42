//! Opaque facade for the parser-owned resolved semantic slice.

use std::fmt;

use crate::source::Url;

pub use sysml_resolution::{
    requirement_collection_from_kind, ActionDerivedFactCollection, ActionDerivedFactKind,
    ActionDerivedFactOutcome, ActionDerivedFactPrerequisite, AffectedDocument, AnalysisEvaluation,
    AnnotationForm, AuthoredUnit, AuthoredValue, BindingConnector, BindingConnectorCheckKind,
    BindingConnectorValidationOutcome, BindingConnectorValidationPrerequisite, BuildMeasurements,
    Conformance, ConformanceObstacle, ConnectedElement, DefinitionUsageDerivedKind,
    DefinitionUsageDerivedOutcome, DefinitionUsageDerivedPrerequisite, DerivedElementOwner,
    Diagnostic, DiagnosticCategory, DiagnosticCode, DiagnosticLocation, DiagnosticOrigin,
    DiagnosticSeverity, DiagramCompartment, DiagramCompartmentKind, DiagramCompartmentProvenance,
    DiagramEdge, DiagramEdgeKind, DiagramElement, DiagramElementTyping, DiagramEndpointOccurrence,
    DiagramIncompleteReason, DiagramOccurrenceIdentity, DiagramRelationship,
    DiagramRelationshipEndpoint, DiagramRelationshipKind, DiagramRelationshipTarget, DiagramScene,
    DiagramSemanticReference, DiagramStateTransition, DiagramStateTransitionScene,
    DiagramStateVertex, DiagramStateVertexKind, DiagramTransitionFeature, DiagramViewCatalogEntry,
    DiagramViewKind, DiagramViewProjection, DocumentId, DocumentToken, Documentation,
    EffectiveType, EffectiveTypeEntry, EffectiveTypeOrigin, EffectiveTyping,
    ElementDerivedDocumentationCollection, ElementDetails, ElementDetailsAt, ElementEvaluation,
    ElementInspection, ElementInspectionAt, ElementKind, ElementModifier, ElementRelationship,
    ElementSearch, ElementSource, EvaluatedScalar, EvaluationFailure, EvaluationState,
    ExpectedMeasurement, FeatureDerivedRelationshipCollection, FeatureDirection, InheritedFeature,
    LibrarySpecializationAnchorBranch, MembershipFacts, MembershipKind, MembershipRole,
    MultiplicityBound, MultiplicityFacts, NamespaceDerivedElementCollection,
    NamespaceImportDerivedElement, NavigationTarget, OccurrenceRole, PortionKind,
    PublicationCompleteness, PublicationIdentity, PublishedDiagnostics, QualifiedElementReference,
    QualifiedReferenceOutcome, QualifiedReferenceTarget, QueryOutcome, RedefinitionCheckKind,
    RedefinitionCheckOutcome, RedefinitionCheckPrerequisite, ReferenceAt, ReferencedDetails,
    RelatedLocation, RelationshipFamily, RelationshipOutcome, RelationshipProvenance,
    RelationshipTarget, RenameOutcome, RequirementConstraintKind, RequirementDerivedFactCollection,
    RequirementDerivedFactKind, RequirementDerivedFactOutcome, RequirementDerivedFactPrerequisite,
    RequirementUsageTyping, RequirementVerification, ResolvedUnit, SatisfyEndpoint,
    SatisfyPolarity, SatisfyRelationship, SourceLocation, SpecializationCheckKind,
    SpecializationCheckOutcome, SpecializationCheckPrerequisite, SpecializationScope,
    StateSubactionKind, SubsettingConformance, SymbolEntry, SymbolId, SymbolToken, TextId,
    TextPosition, TextRange, TypeDerivedElementCollection, TypeDerivedFactCollection,
    TypeDerivedFactKind, TypeDerivedFactOutcome, TypeDerivedFactPrerequisite, TypeDerivedFactValue,
    TypeDerivedRelationshipCollection, TypeFeaturingCheckKind, TypeFeaturingCheckOutcome,
    TypeFeaturingCheckPrerequisite, TypeReference, UnitResolution, ValueKind, VerificationOutcome,
    VerificationRequirement, Visibility, VisibilityProvenance, VisibleMemberRef, VisibleMembers,
};

/// Provenance of an admitted source; the one enum the source authority defines.
pub use sysml_resolution::source::SourceKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedSource {
    inner: sysml_resolution::SourceInput,
}

impl AdmittedSource {
    pub fn from_uri(
        uri: &str,
        content: String,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        if uri.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::new(uri, content, source_kind),
        })
    }

    pub fn from_memory_path(
        namespace: &str,
        path: &str,
        content: String,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        if namespace.is_empty() || normalized.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::new(
                format!("memory://{namespace}/{normalized}"),
                content,
                source_kind,
            ),
        })
    }

    /// Admit a tree the syntax service already parsed; the editor's parse and the build's parse
    /// are then the same tree.
    pub fn from_parsed(
        uri: &str,
        parsed: crate::syntax::ParsedSource,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        if uri.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::from_parsed(uri, parsed, source_kind),
        })
    }

    /// The identity queries and published facts address this document by.
    pub fn identity(&self) -> &str {
        self.inner.identity()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(&'static str);

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for SourceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionStrategy {
    Sequential,
    Parallel,
}

#[derive(Debug)]
pub struct BuildRequest {
    inner: sysml_resolution::BuildRequest,
}

impl BuildRequest {
    pub fn resolved(
        sources: Vec<AdmittedSource>,
        construction: ConstructionStrategy,
    ) -> Result<Self, BuildError> {
        let schedule = match construction {
            ConstructionStrategy::Sequential => sysml_resolution::ConstructionSchedule::Sequential,
            ConstructionStrategy::Parallel => sysml_resolution::ConstructionSchedule::Parallel,
        };
        sysml_resolution::BuildRequest::new(
            sources.into_iter().map(|source| source.inner).collect(),
            schedule,
            sysml_resolution::RESOLVED_CONTRACT,
        )
        .map(|inner| Self { inner })
        .map_err(BuildError)
    }

    /// Builds `sources` against a library that has already been parsed and solved.
    ///
    /// `sources` carries only the workspace documents; the library's come from the stratum.
    pub fn resolved_with_library(
        sources: Vec<AdmittedSource>,
        construction: ConstructionStrategy,
        library: &LibraryStratum,
    ) -> Result<Self, BuildError> {
        let schedule = match construction {
            ConstructionStrategy::Sequential => sysml_resolution::ConstructionSchedule::Sequential,
            ConstructionStrategy::Parallel => sysml_resolution::ConstructionSchedule::Parallel,
        };
        sysml_resolution::BuildRequest::with_library(
            sources.into_iter().map(|source| source.inner).collect(),
            schedule,
            sysml_resolution::RESOLVED_CONTRACT,
            library.handle(),
        )
        .map(|inner| Self { inner })
        .map_err(BuildError)
    }

    /// Also reports diagnostics for these admitted documents, beyond the workspace-authored ones.
    ///
    /// A publication reports its workspace by default. That default is about provenance, which is
    /// not the same question as which documents are an authoring surface: an editor with a library
    /// file open is authoring it, and only the host knows that. Naming the document here is how it
    /// says so, and it is part of the publication's identity because it changes what the
    /// publication answers.
    pub fn reporting(self, documents: impl IntoIterator<Item = Box<str>>) -> Self {
        Self {
            inner: self.inner.reporting(documents),
        }
    }

    /// The identity the publication built from this request will carry.
    ///
    /// Available before the build so a publication owner can record what it scheduled and reject
    /// a result built from anything else, rather than trusting whatever comes back.
    pub fn identity(&self) -> &PublicationIdentity {
        self.inner.identity()
    }
}

/// A library parsed and solved once, reusable by any number of later publications.
///
/// Build it from the library's own sources when a session opens, then hand it to every workspace
/// build. Reuse is conditional: a workspace that could change what a library reference resolves to
/// gets a full solve instead, so the published result never depends on whether a stratum was
/// supplied.
#[derive(Debug)]
pub struct LibraryStratum {
    inner: std::sync::Arc<sysml_resolution::LibraryStratum>,
}

impl LibraryStratum {
    pub fn build(sources: Vec<AdmittedSource>) -> Result<Self, BuildError> {
        sysml_resolution::build_library_stratum(
            sources.into_iter().map(|source| source.inner).collect(),
        )
        .map(|inner| Self {
            inner: std::sync::Arc::new(inner),
        })
        .map_err(BuildError)
    }

    /// How many documents this stratum admits.
    pub fn document_count(&self) -> usize {
        self.inner.document_count()
    }

    fn handle(&self) -> std::sync::Arc<sysml_resolution::LibraryStratum> {
        std::sync::Arc::clone(&self.inner)
    }
}

/// Opaque published semantic state. Share it behind `Arc`; do not duplicate its owner.
///
/// ```compile_fail
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<sysml_query::resolved_slice::PublishedModel>();
/// ```
#[derive(Debug)]
pub struct PublishedModel {
    inner: sysml_resolution::PublishedResolution,
}

impl PublishedModel {
    pub(crate) fn from_resolution(inner: sysml_resolution::PublishedResolution) -> Self {
        Self { inner }
    }
}

pub fn build(request: BuildRequest) -> Result<PublishedModel, BuildError> {
    sysml_resolution::build(request.inner)
        .map(|inner| PublishedModel { inner })
        .map_err(BuildError)
}

/// Builds one publication and returns timings measured at its semantic phase barriers.
pub fn build_measured(
    request: BuildRequest,
) -> Result<(PublishedModel, BuildMeasurements), BuildError> {
    sysml_resolution::build_measured(request.inner)
        .map(|(inner, measurements)| (PublishedModel { inner }, measurements))
        .map_err(BuildError)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildError(sysml_resolution::BuildFailure);

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for BuildError {}

impl PublishedModel {
    pub fn debug(&self) -> DebugQueries<'_> {
        DebugQueries { model: &self.inner }
    }

    /// The authored name of one element, borrowed from this publication.
    ///
    /// `None` where the element is anonymous. Results carry the handle rather than a copy of the
    /// name; this is where a host that has to render one reads it.
    pub fn symbol_name(&self, symbol: SymbolId) -> Option<&str> {
        self.inner.symbol_name(symbol)
    }

    /// The `::`-joined display path of one element, borrowed from this publication.
    ///
    /// A display convenience, not an identity: two elements under an anonymous ancestor can share
    /// one. Borrowed, so showing a name costs no allocation.
    pub fn qualified_name(&self, symbol: SymbolId) -> Option<&str> {
        self.inner.qualified_name(symbol)
    }

    /// One run of authored text, borrowed from this publication.
    ///
    /// Published facts carry a [`TextId`] rather than a copy of text the publication already
    /// interned; this is where a host that has to render one reads it. `None` for a handle this
    /// publication never minted.
    pub fn text(&self, id: TextId) -> Option<&str> {
        self.inner.text(id)
    }

    /// The stable, serialisable form of one element handle.
    ///
    /// A [`SymbolId`] addresses an element of *this* publication and must not outlive it. Take a
    /// [`SymbolToken`] for anything that crosses a process or protocol boundary -- an LSP DTO, a
    /// JSON report, an archive entry, the generator protocol -- or that has to survive a rebuild.
    /// Materialising one allocates; it is a boundary operation, asked for explicitly.
    pub fn symbol_token(&self, symbol: SymbolId) -> Option<SymbolToken> {
        self.inner.symbol_token(symbol)
    }

    /// The handle a token names in this publication, if it still names one.
    pub fn resolve_token(&self, token: &SymbolToken) -> Option<SymbolId> {
        self.inner.resolve_token(token)
    }

    /// The normalised identity -- the URI -- of one document, borrowed from this publication.
    ///
    /// Every published location names its document by [`DocumentId`], not by string. This is the
    /// one place that turns a handle back into text, and it borrows: a host that groups a
    /// thousand references by document pays for one lookup per group, not one copy per result.
    pub fn document_identity(&self, document: DocumentId) -> Option<&str> {
        self.inner.document_identity(document)
    }

    /// The stable, serialisable form of one document handle.
    ///
    /// A [`DocumentId`] addresses a document of *this* publication and must not outlive it. Take
    /// a [`DocumentToken`] for anything that crosses a process or protocol boundary or has to
    /// survive a rebuild; its text is byte-for-byte what [`Self::document_identity`] borrows.
    pub fn document_token(&self, document: DocumentId) -> Option<DocumentToken> {
        self.inner.document_token(document)
    }

    /// The handle a document token names in this publication, if it still names one.
    pub fn resolve_document_token(&self, token: &DocumentToken) -> Option<DocumentId> {
        self.inner.resolve_document_token(token)
    }

    /// The handle for a document identity a host already holds as text.
    ///
    /// The boundary an editor request crosses: it names a document by URI once, and every
    /// answer after that is handles.
    pub fn document_of(&self, identity: &str) -> Option<DocumentId> {
        self.inner.document_of(identity)
    }

    pub fn publication(&self) -> PublicationQueries<'_> {
        PublicationQueries { model: &self.inner }
    }

    pub fn dependencies(&self) -> DependencyQueries<'_> {
        DependencyQueries { model: &self.inner }
    }

    pub fn navigation(&self) -> NavigationQueries<'_> {
        NavigationQueries { model: &self.inner }
    }

    pub fn edits(&self) -> EditQueries<'_> {
        EditQueries { model: &self.inner }
    }

    pub fn completion(&self) -> CompletionQueries<'_> {
        CompletionQueries { model: &self.inner }
    }

    pub fn inspection(&self) -> InspectionQueries<'_> {
        InspectionQueries { model: &self.inner }
    }

    pub fn types(&self) -> TypeQueries<'_> {
        TypeQueries { model: &self.inner }
    }

    pub fn evaluation(&self) -> EvaluationQueries<'_> {
        EvaluationQueries { model: &self.inner }
    }

    pub fn diagnostics(&self) -> DiagnosticQueries<'_> {
        DiagnosticQueries { model: &self.inner }
    }

    pub fn diagrams(&self) -> DiagramQueries<'_> {
        DiagramQueries { model: &self.inner }
    }
}

/// Publication topology and dependency queries.
pub struct DependencyQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl DependencyQueries<'_> {
    pub fn affected_documents(
        &self,
        changed_document: &str,
    ) -> QueryOutcome<Box<[AffectedDocument]>> {
        self.model.affected_documents(changed_document)
    }

    /// The workspace documents whose diagnostics may change when `provider` changes.
    ///
    /// The derivation a host needs to decide what to republish, answered once here rather than
    /// per host: the publication's settled import and alias facts name the dependents, and the
    /// answer is narrowed to workspace documents because library documents are not republished.
    ///
    /// `workspace` is the host's own set of workspace document identities. It is used only for
    /// the conservative answer, which stays explicit in the result rather than being indexed:
    /// only a publication that could not settle the dependency graph produces a conservative
    /// answer, and a caller can always tell over-invalidation from an exact empty answer.
    pub fn workspace_documents_affected_by(
        &self,
        workspace: impl IntoIterator<Item = Url>,
        provider: &Url,
    ) -> AffectedWorkspaceDocuments {
        let mut all_workspace = workspace.into_iter().collect::<Vec<_>>();
        all_workspace.sort();
        all_workspace.dedup();
        all_workspace.retain(|uri| uri != provider);

        let documents = match self.affected_documents(provider.as_str()) {
            QueryOutcome::Resolved(documents) => documents,
            QueryOutcome::Recovered(_)
            | QueryOutcome::UnsupportedWith(_)
            | QueryOutcome::Unresolved
            | QueryOutcome::Ambiguous(_)
            | QueryOutcome::Unsupported
            | QueryOutcome::Recovery
            | QueryOutcome::Incomplete => {
                return AffectedWorkspaceDocuments {
                    uris: all_workspace,
                    conservative: true,
                }
            }
        };

        let mut uris = documents
            .iter()
            .filter(|document| document.source == ElementSource::Workspace)
            .filter_map(|document| Url::parse(&document.identity).ok())
            .collect::<Vec<_>>();
        uris.sort();
        uris.dedup();
        AffectedWorkspaceDocuments {
            uris,
            conservative: false,
        }
    }
}

/// Which workspace documents a host must republish after one document changed.
///
/// `is_conservative` is the explicit unsettled state: the publication could not settle the
/// dependency graph, so every workspace document is named rather than a narrower set being
/// guessed. It is never a resolved answer wearing an exact set's clothes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffectedWorkspaceDocuments {
    uris: Vec<Url>,
    conservative: bool,
}

impl AffectedWorkspaceDocuments {
    /// True when the dependency graph was not settled and the answer over-invalidates.
    pub fn is_conservative(&self) -> bool {
        self.conservative
    }

    pub fn uris(&self) -> &[Url] {
        &self.uris
    }

    pub fn into_uris(self) -> Vec<Url> {
        self.uris
    }
}

pub struct DiagramQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl DiagramQueries<'_> {
    pub fn catalog(&self) -> QueryOutcome<Box<[DiagramViewCatalogEntry]>> {
        self.model.diagram_view_catalog()
    }

    pub fn view(&self, identity: SymbolId) -> QueryOutcome<DiagramViewProjection> {
        self.model.diagram_view(identity)
    }

    /// The display name of one catalogued view, borrowed from the publication.
    ///
    /// The authored name, falling back to the `::`-joined display path for an anonymous view
    /// usage. A catalog entry carries the handle rather than a copy of this text.
    pub fn view_name(&self, view: SymbolId) -> Option<&str> {
        self.model.diagram_view_name(view)
    }

    /// The occurrence identity the projection uses for one exposed root.
    ///
    /// An occurrence carries the scene key generators and reports publish, which is derived from
    /// boundary tokens; a consumer takes the occurrence from here rather than assembling a path.
    pub fn root_occurrence(&self, root: SymbolId) -> Option<DiagramOccurrenceIdentity> {
        self.model.diagram_root_occurrence(root)
    }
}

/// The resolution-owned diagnostics this publication settled.
///
/// The facade adapts the owner's contract; it does not evaluate a rule of its own. Every code,
/// severity, range, and related location a consumer sees here was decided by `sysml_resolution`
/// at the publication barrier, so a host, a generator, and the canonical snapshot projection
/// cannot disagree about what one publication reported.
///
/// This is the whole validation surface a host reports. `sysml_resolution::diagnostics`'s module
/// documentation lists the families it decides and the rules it deliberately leaves absent.
pub struct DiagnosticQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl<'a> DiagnosticQueries<'a> {
    /// The published diagnostics, canonically ordered, with the completeness of the publication
    /// that produced them. Only workspace-authored documents are reported.
    pub fn published(&self) -> PublishedDiagnostics<'a> {
        self.model.diagnostics()
    }

    /// The diagnostics of one admitted document, read from the publication's own index.
    ///
    /// The cost is proportional to what is returned rather than to the model, and nothing here
    /// computes: repeating the query, or asking about documents in any order, answers identically.
    /// A document this publication did not admit answers with no diagnostics and the same
    /// completeness, which is why the completeness travels with the answer.
    pub fn for_document(&self, document: &str) -> PublishedDiagnostics<'a> {
        self.model.document_diagnostics(document)
    }
}

/// Evaluated values, evaluation states, authored units and required measurement references.
///
/// One cohesive answer per element, so a consumer showing a value with its unit makes one call
/// rather than combining an inspection query, a type query and a relationship query and deciding
/// for itself how they relate. The facade adapts; it evaluates nothing, resolves no unit, and
/// manufactures no outcome -- every field was settled by `sysml_resolution` at the publication
/// barrier.
pub struct EvaluationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl EvaluationQueries<'_> {
    /// What this publication settled for one element's authored expression.
    pub fn evaluate(&self, symbol: SymbolId) -> QueryOutcome<ElementEvaluation> {
        self.model.evaluate(symbol)
    }
}

/// Direct types, supertypes, subtypes, effective types, featuring types and conformance.
///
/// Every answer is read from facts the publication settled before it became visible; none of these
/// calls traverses the model, and repeating one cannot change what it returns.
pub struct TypeQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl TypeQueries<'_> {
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
    /// Missing and ambiguous anchors stay explicit query outcomes; this facade never substitutes
    /// a name from a rendered model or from fixture metadata.
    pub fn part_definition_specialization_anchor(&self) -> QueryOutcome<SymbolId> {
        self.model.part_definition_specialization_anchor()
    }

    /// The typed canonical anchor outcome for one generated `specializesFromLibrary` rule.
    ///
    /// The rule ID identifies an authoritative manifest entry. An absent or unresolved anchor is
    /// `Unresolved`; competing standard-library declarations remain `Ambiguous` candidates.
    pub fn library_specialization_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolId> {
        self.model.library_specialization_anchor(rule_id)
    }

    /// The typed canonical branch outcome for an exact conditional specialization rule.
    /// `Default` retains the compatible single-anchor projection.
    pub fn library_specialization_anchor_branch(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> QueryOutcome<SymbolId> {
        self.model
            .library_specialization_anchor_branch(rule_id, branch)
    }

    /// The typed canonical anchor outcome for any generated exact library rule, including
    /// `specializesFromLibrary` and `redefinesFromLibrary` contracts.
    pub fn library_rule_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolId> {
        self.model.library_rule_anchor(rule_id)
    }

    /// Whether a generated `redefinesFromLibrary` rule has an exact lowered source projection.
    ///
    /// This preserves the distinction between an unresolved library anchor and a rule source the
    /// current semantic model does not yet represent.
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

    /// Every supertype, reflexively including `symbol` itself, as the Pilot's `allSupertypes` does.
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

    /// Every effective TypeFeaturing target, retaining authored versus implied provenance.
    pub fn featuring_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[TypeReference]>> {
        self.model.featuring_types(symbol)
    }

    /// Whether `specific` conforms to `general` (KerML §8.4.3.2).
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

pub struct PublicationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl<'a> PublicationQueries<'a> {
    pub fn completeness(&self) -> PublicationCompleteness {
        self.model.completeness()
    }

    /// The dependency-complete identity of every input this publication committed to.
    ///
    /// Borrowed from the publication rather than the query handle, so an owner can hold it
    /// against the publication itself instead of cloning at every comparison.
    pub fn identity(&self) -> &'a PublicationIdentity {
        self.model.identity()
    }

    /// Dependency-complete digest of every source admitted to this publication.
    pub fn source_digest(&self) -> String {
        self.model.identity().source_digest().to_string()
    }

    pub fn model_digest(&self) -> String {
        self.model.identity().model_digest()
    }
}

pub struct NavigationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl NavigationQueries<'_> {
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
}

pub struct EditQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl EditQueries<'_> {
    pub fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        self.model.prepare_rename(document, position, new_name)
    }
}

pub struct CompletionQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl<'a> CompletionQueries<'a> {
    /// The members visible at a position, as a borrowed view over the publication.
    ///
    /// The view outlives this query handle: it borrows the publication, not the handle, so a
    /// caller can read it after the `completion()` temporary is gone.
    pub fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<VisibleMembers<'a>> {
        self.model.visible_members(document, position, qualifier)
    }
}

/// Element inspection and document symbols.
///
/// The `PRODUCTION_CUTOVER.md` row this serves names `sysml_query` as the owner of the typed
/// service, so the contract is reachable here rather than only from the owning crate that
/// consumers are not permitted to depend on.
pub struct InspectionQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl InspectionQueries<'_> {
    /// Resolves a readable KerML qualified reference through the semantic owner.
    pub fn resolve_qualified_reference(
        &self,
        reference: &QualifiedElementReference,
    ) -> QualifiedReferenceOutcome {
        self.model.resolve_qualified_reference(reference)
    }

    /// Everything the publication knows about one element.
    pub fn inspect(&self, symbol: SymbolId) -> QueryOutcome<ElementInspection> {
        self.model.inspect(symbol)
    }

    /// The exact derived `Element::owner` fact, from the publication's canonical ownership
    /// structure. A root element resolves to [`DerivedElementOwner::NoOwner`]; it is not an
    /// unresolved query.
    pub fn derived_element_owner(&self, symbol: SymbolId) -> QueryOutcome<DerivedElementOwner> {
        self.model.derived_element_owner(symbol)
    }

    /// One exact derived `Element` documentation collection, selected by the pinned manifest
    /// contract and projected from canonical documentation facts.
    pub fn element_derived_documentation(
        &self,
        symbol: SymbolId,
        collection: ElementDerivedDocumentationCollection,
    ) -> QueryOutcome<Box<[Documentation]>> {
        self.model.element_derived_documentation(symbol, collection)
    }

    /// The element whose declaration encloses `position`, and what a reference there points at.
    pub fn inspect_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ElementInspectionAt> {
        self.model.inspect_at(document, position)
    }

    /// Everything the publication settled about one element, as one coherent answer.
    ///
    /// The service a feature inspector consumes: one call rather than an inspection query, a type
    /// query, an evaluation query and a relationship query whose results the consumer would have
    /// to decide how to combine. The facade adapts nothing here -- every field, including which
    /// relationship families are applicable and what each of them settled to, was decided by
    /// `sysml_resolution` at the publication barrier.
    pub fn element_details(&self, symbol: SymbolId) -> QueryOutcome<ElementDetails> {
        self.model.element_details(symbol)
    }

    /// The element whose declaration encloses `position` and the element a reference there points
    /// at, both in full detail.
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

    /// Elements matching a typed kind and authored-source provenance filter.
    pub fn search_elements(&self, search: ElementSearch) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.search_elements(search)
    }

    /// Workspace-authored satisfy statements, with directional ends and explicit outcomes.
    pub fn satisfy_relationships(&self) -> QueryOutcome<Box<[SatisfyRelationship]>> {
        self.model.satisfy_relationships()
    }

    /// One exact Feature relationship collection from the canonical relationship store.
    pub fn feature_derived_relationships(
        &self,
        symbol: SymbolId,
        collection: FeatureDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        self.model.feature_derived_relationships(symbol, collection)
    }

    /// One exact Type relationship collection or operand projection from canonical facts.
    pub fn type_derived_relationships(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        self.model.type_derived_relationships(symbol, collection)
    }

    /// One exact Type element-valued derivation from canonical ownership and membership facts.
    pub fn type_derived_elements(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.type_derived_elements(symbol, collection)
    }

    /// One exact Type derivation that retains an explicit typed unavailable-fact outcome until
    /// its canonical semantic owner can publish the normative values.
    pub fn type_derived_fact(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedFactCollection,
    ) -> QueryOutcome<TypeDerivedFactOutcome> {
        self.model.type_derived_fact(symbol, collection)
    }

    /// One exact manifest-selected Systems::DefinitionAndUsage derivation from the canonical
    /// semantic publication. The façade does not reconstruct direct or inherited membership.
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

    /// One exact manifest-selected Systems::Requirements property. Membership roles and
    /// documentation records remain owned by the resolved semantic publication.
    pub fn requirement_derived_fact(
        &self,
        symbol: SymbolId,
        collection: RequirementDerivedFactCollection,
    ) -> QueryOutcome<RequirementDerivedFactOutcome> {
        self.model.requirement_derived_fact(symbol, collection)
    }

    /// The manifest-scoped outcome for one exact TypeFeaturing check.
    pub fn type_featuring_check(
        &self,
        symbol: SymbolId,
        rule: TypeFeaturingCheckKind,
    ) -> QueryOutcome<TypeFeaturingCheckOutcome> {
        self.model.type_featuring_check(symbol, rule)
    }

    /// The manifest-scoped result for an exact redefinition check.
    pub fn redefinition_check(
        &self,
        rule: RedefinitionCheckKind,
    ) -> QueryOutcome<RedefinitionCheckOutcome> {
        self.model.redefinition_check(rule)
    }

    /// The manifest-scoped result for one exact specialization predicate.
    pub fn specialization_check(
        &self,
        rule: SpecializationCheckKind,
    ) -> QueryOutcome<SpecializationCheckOutcome> {
        self.model.specialization_check(rule)
    }

    /// One exact Namespace element-valued derivation from canonical declaration and membership
    /// facts. This facade does not recreate Namespace membership from syntax or scope labels.
    pub fn namespace_derived_elements(
        &self,
        symbol: SymbolId,
        collection: NamespaceDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        self.model.namespace_derived_elements(symbol, collection)
    }

    /// Exact `NamespaceImport::importedElement` facts for imports owned by one Namespace. The
    /// opaque facade preserves each import's canonical identity and typed target outcome.
    pub fn namespace_import_derived_elements(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<Box<[NamespaceImportDerivedElement]>> {
        self.model.namespace_import_derived_elements(symbol)
    }

    /// Workspace-authored binding connectors, including both paired endpoint outcomes.
    pub fn binding_connectors(&self) -> QueryOutcome<Box<[BindingConnector]>> {
        self.model.binding_connectors()
    }

    /// The applicability outcome for one closed binding-connector validation rule.
    pub fn binding_connector_validation(
        &self,
        rule: BindingConnectorCheckKind,
    ) -> QueryOutcome<BindingConnectorValidationOutcome> {
        self.model.binding_connector_validation(rule)
    }

    pub fn requirement_verifications(&self) -> QueryOutcome<Box<[RequirementVerification]>> {
        self.model.requirement_verifications()
    }

    /// Effective features, direct first and inherited nearest-first with name shadowing.
    pub fn effective_features(&self, symbol: SymbolId) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.effective_features(symbol)
    }
}

pub struct DebugQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorProbe {
    pub document: String,
    pub position: TextPosition,
    pub qualifier: Option<String>,
    pub rename_to: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedReferenceProbe {
    pub document: Option<String>,
    pub qualified_name: String,
    pub expected_kind: Option<ElementKind>,
}

impl DebugQueries<'_> {
    pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_semantic_sexpr(output)
    }

    pub fn write_diagnostics_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_diagnostics_sexpr(output)
    }

    pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_navigation_sexpr(output)
    }

    pub fn write_types_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_types_sexpr(output)
    }

    pub fn write_editor_queries_sexpr(
        &self,
        probes: &[EditorProbe],
        output: &mut dyn fmt::Write,
    ) -> fmt::Result {
        let model = &self.model;
        writeln!(output, "(editor-queries")?;
        for probe in probes {
            writeln!(
                output,
                "  (probe (document {:?}) (position {} {})",
                probe.document, probe.position.line, probe.position.character
            )?;
            let target = self.model.target_at(&probe.document, probe.position);
            write_target_outcome(model, output, "target", &target)?;
            if let QueryOutcome::Resolved(target)
            | QueryOutcome::Recovered(target)
            | QueryOutcome::UnsupportedWith(target) = &target
            {
                write_locations_outcome(
                    model,
                    output,
                    "references",
                    &self.model.references(target.symbol, true),
                )?;
            }
            write_rename_outcome(
                model,
                output,
                &self.model.prepare_rename(
                    &probe.document,
                    probe.position,
                    probe.rename_to.as_deref(),
                ),
            )?;
            write_members_outcome(
                output,
                &self.model.visible_members(
                    &probe.document,
                    probe.position,
                    probe.qualifier.as_deref(),
                ),
            )?;
            write_details_at_outcome(
                model,
                output,
                &self
                    .model
                    .element_details_at(&probe.document, probe.position),
            )?;
            writeln!(output, "  )")?;
        }
        // Once per probed document rather than once per probe: the outline is a property of the
        // document, and repeating it would make a fixture's probe count decide how much of the
        // snapshot is outline.
        let mut written = Vec::new();
        for probe in probes {
            if written.contains(&probe.document) {
                continue;
            }
            written.push(probe.document.clone());
            write_document_symbols(
                model,
                output,
                &probe.document,
                &self.model.document_symbols(&probe.document),
            )?;
        }
        write!(output, ")")
    }

    pub fn write_qualified_reference_queries_sexpr(
        &self,
        probes: &[QualifiedReferenceProbe],
        output: &mut dyn fmt::Write,
    ) -> fmt::Result {
        writeln!(output, "(qualified-reference-queries")?;
        for probe in probes {
            write!(output, "  (reference")?;
            if let Some(document) = &probe.document {
                write!(output, " (document {document:?})")?;
            } else {
                write!(output, " (document any)")?;
            }
            write!(output, " (qualified-name {:?})", probe.qualified_name)?;
            if let Some(kind) = probe.expected_kind {
                write!(output, " (expected-kind {:?})", kind.as_str())?;
            }
            writeln!(output)?;
            let outcome = self
                .model
                .resolve_qualified_reference(&QualifiedElementReference {
                    document: probe.document.clone().map(Into::into),
                    qualified_name: probe.qualified_name.clone().into(),
                    expected_kind: probe.expected_kind,
                });
            write_qualified_reference_outcome(self.model, output, &outcome)?;
            writeln!(output, "  )")?;
        }
        write!(output, ")")
    }
}

fn write_qualified_reference_target(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    target: &QualifiedReferenceTarget,
) -> fmt::Result {
    // The probe report is a baseline artefact, so it carries the boundary token, not the handle:
    // a handle is a rank inside one publication and means nothing in a recorded file.
    write!(
        output,
        "(candidate (identity {:?}) (kind {:?}) (qualified-name {:?}) ",
        model
            .symbol_token(target.identity)
            .as_ref()
            .map(SymbolToken::as_str)
            .unwrap_or_default(),
        target.kind.as_str(),
        model.qualified_name(target.identity).unwrap_or_default()
    )?;
    write_location(model, output, &target.location)?;
    write!(output, ")")
}

fn write_qualified_reference_outcome(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    outcome: &QualifiedReferenceOutcome,
) -> fmt::Result {
    write!(output, "    (outcome")?;
    match outcome {
        QualifiedReferenceOutcome::Resolved(target) => {
            write!(output, " (status resolved) ")?;
            write_qualified_reference_target(model, output, target)?;
        }
        QualifiedReferenceOutcome::Recovered(target) => {
            write!(output, " (status recovery) ")?;
            write_qualified_reference_target(model, output, target)?;
        }
        QualifiedReferenceOutcome::UnsupportedWith(target) => {
            write!(output, " (status unsupported) ")?;
            write_qualified_reference_target(model, output, target)?;
        }
        QualifiedReferenceOutcome::Ambiguous(targets)
        | QualifiedReferenceOutcome::WrongKind(targets) => {
            let status = if matches!(outcome, QualifiedReferenceOutcome::Ambiguous(_)) {
                "ambiguous"
            } else {
                "wrong-kind"
            };
            write!(output, " (status {status}) (candidates")?;
            for target in targets {
                write!(output, " ")?;
                write_qualified_reference_target(model, output, target)?;
            }
            write!(output, ")")?;
        }
        QualifiedReferenceOutcome::Unresolved => write!(output, " (status unresolved)")?,
        QualifiedReferenceOutcome::Unsupported => write!(output, " (status unsupported)")?,
        QualifiedReferenceOutcome::Recovery => write!(output, " (status recovery)")?,
        QualifiedReferenceOutcome::Incomplete => write!(output, " (status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_document_symbols(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    document: &str,
    outcome: &QueryOutcome<Box<[SymbolEntry]>>,
) -> fmt::Result {
    writeln!(output, "  (document-symbols (document {document:?})")?;
    match outcome {
        QueryOutcome::Resolved(entries)
        | QueryOutcome::Recovered(entries)
        | QueryOutcome::UnsupportedWith(entries) => {
            writeln!(output, "    (status {})", outcome_status(outcome))?;
            for entry in entries.iter() {
                write!(output, "    (symbol (kind {:?})", entry.kind.as_str())?;
                if let Some(name) = &entry.name {
                    write!(output, " (name {name:?})")?;
                }
                write!(
                    output,
                    " (qualified-name {:?}) ",
                    model.qualified_name(entry.identity).unwrap_or_default()
                )?;
                write_location(model, output, &entry.location)?;
                write!(output, " (declaration ")?;
                write_range(output, entry.declaration_range)?;
                writeln!(output, "))")?;
            }
        }
        _ => writeln!(output, "    (status {})", outcome_status(outcome))?,
    }
    writeln!(output, "  )")
}

fn write_range(output: &mut dyn fmt::Write, range: TextRange) -> fmt::Result {
    write!(
        output,
        "(range (start {} {}) (end {} {}))",
        range.start.line, range.start.character, range.end.line, range.end.character
    )
}

/// A location, rendered with its document identity spelled out.
///
/// The published location names its document by handle; a baseline artefact records the URI, so
/// the identity is materialised here -- at the boundary that writes the file -- and nowhere in
/// the result itself.
fn write_location(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    location: &SourceLocation,
) -> fmt::Result {
    write!(
        output,
        "(location (document {:?}) ",
        model
            .document_identity(location.document)
            .unwrap_or_default()
    )?;
    write_range(output, location.range)?;
    write!(output, " (role {:?}))", location.role)
}

fn write_target(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    target: &NavigationTarget,
) -> fmt::Result {
    write!(
        output,
        "(candidate (name {:?}) ",
        model.symbol_name(target.symbol).unwrap_or_default()
    )?;
    write_location(model, output, &target.location)?;
    write!(output, ")")
}

fn write_target_outcome(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    label: &str,
    outcome: &QueryOutcome<NavigationTarget>,
) -> fmt::Result {
    write!(output, "    ({label} ")?;
    match outcome {
        QueryOutcome::Resolved(target) => {
            write!(output, "(status resolved) ")?;
            write_target(model, output, target)?;
        }
        QueryOutcome::Recovered(target) => {
            write!(output, "(status recovery) ")?;
            write_target(model, output, target)?;
        }
        QueryOutcome::UnsupportedWith(target) => {
            write!(output, "(status unsupported) ")?;
            write_target(model, output, target)?;
        }
        QueryOutcome::Ambiguous(targets) => {
            write!(output, "(status ambiguous) (candidates")?;
            for target in targets {
                write!(output, " ")?;
                write_target(model, output, target)?;
            }
            write!(output, ")")?;
        }
        QueryOutcome::Unresolved => write!(output, "(status unresolved)")?,
        QueryOutcome::Unsupported => write!(output, "(status unsupported)")?,
        QueryOutcome::Recovery => write!(output, "(status recovery)")?,
        QueryOutcome::Incomplete => write!(output, "(status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_locations_outcome(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    label: &str,
    outcome: &QueryOutcome<Box<[SourceLocation]>>,
) -> fmt::Result {
    write!(output, "    ({label} ")?;
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => {
            write!(output, "(locations")?;
            for value in values.iter() {
                write!(output, " ")?;
                write_location(model, output, value)?;
            }
            write!(output, ")")?;
        }
        _ => write!(output, "(status unavailable)")?,
    }
    writeln!(output, ")")
}

fn write_rename_outcome(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    outcome: &RenameOutcome,
) -> fmt::Result {
    write!(output, "    (rename ")?;
    match outcome {
        RenameOutcome::Ready {
            symbol,
            range,
            occurrences,
        } => {
            let name = model.symbol_name(*symbol).unwrap_or_default();
            write!(output, "(status ready) (name {name:?}) ")?;
            write_range(output, *range)?;
            write!(output, " (occurrences {})", occurrences.len())?;
        }
        RenameOutcome::Collision(targets) => {
            write!(output, "(status collision) (candidates")?;
            for target in targets.iter() {
                write!(output, " ")?;
                write_target(model, output, target)?;
            }
            write!(output, ")")?;
        }
        // No trailing `)` here: the shared `writeln!` below closes `(rename` for every arm.
        RenameOutcome::Ambiguous(targets) => {
            write!(output, "(status ambiguous) (candidates {})", targets.len())?
        }
        RenameOutcome::InvalidName => write!(output, "(status invalid-name)")?,
        RenameOutcome::Unresolved => write!(output, "(status unresolved)")?,
        RenameOutcome::Unsupported => write!(output, "(status unsupported)")?,
        RenameOutcome::Recovery => write!(output, "(status recovery)")?,
        RenameOutcome::Incomplete => write!(output, "(status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_members_outcome(
    output: &mut dyn fmt::Write,
    outcome: &QueryOutcome<VisibleMembers<'_>>,
) -> fmt::Result {
    write!(output, "    (visible-members ")?;
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => {
            write!(output, "(candidates")?;
            for value in values.iter() {
                write!(
                    output,
                    " (member (name {:?}) (qualified-name {:?}) (kind {:?})",
                    value.name(),
                    value.qualified_name(),
                    value.kind().as_str()
                )?;
                if let Some(role) = value.role() {
                    write!(output, " (role {:?})", role.as_str())?;
                }
                write!(output, ")")?;
            }
            write!(output, ")")?;
        }
        _ => write!(output, "(status unavailable)")?,
    }
    writeln!(output, ")")
}

fn write_details_at_outcome(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    outcome: &QueryOutcome<ElementDetailsAt>,
) -> fmt::Result {
    writeln!(output, "    (inspection")?;
    match outcome {
        QueryOutcome::Resolved(at)
        | QueryOutcome::Recovered(at)
        | QueryOutcome::UnsupportedWith(at) => {
            writeln!(output, "      (status {})", outcome_status(outcome))?;
            match &at.containing {
                Some(containing) => {
                    writeln!(output, "      (containing")?;
                    write_element(model, output, "        ", containing)?;
                    writeln!(output, "      )")?;
                }
                None => writeln!(output, "      (containing (status none))")?,
            }
            write_referenced_details(model, output, &at.referenced)?;
        }
        _ => writeln!(output, "      (status {})", outcome_status(outcome))?,
    }
    writeln!(output, "    )")
}

fn outcome_status<T>(outcome: &QueryOutcome<T>) -> &'static str {
    match outcome {
        QueryOutcome::Resolved(_) => "resolved",
        QueryOutcome::Recovered(_) | QueryOutcome::Recovery => "recovery",
        QueryOutcome::UnsupportedWith(_) | QueryOutcome::Unsupported => "unsupported",
        QueryOutcome::Ambiguous(_) => "ambiguous",
        QueryOutcome::Unresolved => "unresolved",
        QueryOutcome::Incomplete => "incomplete",
    }
}

fn write_referenced_details(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    referenced: &ReferencedDetails,
) -> fmt::Result {
    match referenced {
        ReferencedDetails::None => writeln!(output, "      (referenced (status none))"),
        ReferencedDetails::Unresolved => {
            writeln!(output, "      (referenced (status unresolved))")
        }
        ReferencedDetails::Unsupported => {
            writeln!(output, "      (referenced (status unsupported))")
        }
        ReferencedDetails::Incomplete => {
            writeln!(output, "      (referenced (status incomplete))")
        }
        ReferencedDetails::Resolved(details) => {
            writeln!(output, "      (referenced (status resolved)")?;
            write_element(model, output, "        ", details)?;
            writeln!(output, "      )")
        }
        ReferencedDetails::Ambiguous(candidates) => {
            writeln!(output, "      (referenced (status ambiguous)")?;
            for candidate in candidates.iter() {
                write_element(model, output, "        ", candidate)?;
            }
            writeln!(output, "      )")
        }
    }
}

/// One element's published facts.
///
/// Absent facts are omitted rather than rendered as an empty form, so a snapshot diff that gains a
/// line is a fact that started being published, not a formatting change.
fn write_element(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    indent: &str,
    details: &ElementDetails,
) -> fmt::Result {
    let inspection = &details.inspection;
    writeln!(
        output,
        "{indent}(element (kind {:?})",
        inspection.kind.as_str()
    )?;
    if let Some(role) = inspection.role {
        writeln!(output, "{indent}  (role {:?})", role.as_str())?;
    }
    if let Some(name) = &inspection.name {
        writeln!(output, "{indent}  (name {name:?})")?;
    }
    if let Some(short_name) = &inspection.short_name {
        writeln!(output, "{indent}  (short-name {short_name:?})")?;
    }
    writeln!(
        output,
        "{indent}  (qualified-name {:?})",
        model
            .qualified_name(inspection.identity)
            .unwrap_or_default()
    )?;
    write!(output, "{indent}  ")?;
    write_location(model, output, &inspection.location)?;
    writeln!(output)?;
    write!(output, "{indent}  (declaration ")?;
    write_range(output, inspection.declaration_range)?;
    writeln!(output, ")")?;
    let membership = inspection.membership;
    writeln!(
        output,
        "{indent}  (membership (kind {}) (visibility {}) (provenance {}))",
        membership_kind_name(membership.kind),
        visibility_name(membership.visibility),
        visibility_provenance_name(membership.provenance)
    )?;
    write_multiplicity(output, indent, inspection.multiplicity)?;
    if !inspection.modifiers.is_empty() {
        write!(output, "{indent}  (modifiers")?;
        for modifier in inspection.modifiers.iter() {
            write!(output, " {:?}", modifier.as_str())?;
        }
        writeln!(output, ")")?;
    }
    if let Some(portion) = inspection.portion_kind {
        writeln!(output, "{indent}  (portion {})", portion_name(portion))?;
    }
    if let Some(direction) = inspection.direction {
        writeln!(
            output,
            "{indent}  (direction {})",
            direction_name(direction)
        )?;
    }
    if let Some(value) = inspection.value {
        writeln!(
            output,
            "{indent}  (value (kind {}) (default {}) (operator {}))",
            value_kind_name(value.kind),
            value.is_default,
            value.has_operator
        )?;
    }
    if inspection.evaluation != EvaluationState::NotApplicable {
        write!(output, "{indent}  (evaluation {}", inspection.evaluation)?;
        if let Some(scalar) = inspection.evaluation.value() {
            write!(output, " ")?;
            write_scalar(output, scalar)?;
        }
        writeln!(output, ")")?;
    }
    for documentation in inspection.documentation.iter() {
        write!(
            output,
            "{indent}  (documentation (form {})",
            annotation_form_name(documentation.form)
        )?;
        if let Some(locale) = &documentation.locale {
            write!(output, " (locale {locale:?})")?;
        }
        if let Some(language) = &documentation.language {
            write!(output, " (language {language:?})")?;
        }
        writeln!(output, " (text {:?}))", documentation.text)?;
    }
    for relationship in inspection.relationships.iter() {
        write_relationship(output, indent, relationship)?;
    }
    for (label, family) in [
        ("typing", &details.typing),
        ("specialization", &details.specialization),
        ("subsetting", &details.subsetting),
        ("redefinition", &details.redefinition),
    ] {
        write_family(model, output, indent, label, family)?;
    }
    if details.effective_typing.outcome != RelationshipOutcome::NotApplicable {
        write!(
            output,
            "{indent}  (effective-typing (outcome {})",
            details.effective_typing.outcome.as_str()
        )?;
        for entry in details.effective_typing.types.iter() {
            write!(
                output,
                " (type (qualified-name {:?})",
                model
                    .qualified_name(entry.element.identity)
                    .unwrap_or_default()
            )?;
            match &entry.origin {
                EffectiveTypeOrigin::Direct => write!(output, " (origin direct))")?,
                EffectiveTypeOrigin::Inherited(_) => write!(output, " (origin inherited))")?,
            }
        }
        writeln!(output, ")")?;
    }
    for feature in details.inherited_features.iter() {
        writeln!(
            output,
            "{indent}  (inherited-feature (qualified-name {:?}) (declared-in {:?}))",
            model
                .qualified_name(feature.feature.identity)
                .unwrap_or_default(),
            model
                .qualified_name(feature.declared_in.identity)
                .unwrap_or_default()
        )?;
    }
    for entry in details.metadata.iter() {
        writeln!(
            output,
            "{indent}  (metadata (qualified-name {:?}))",
            model.qualified_name(entry.identity).unwrap_or_default()
        )?;
    }
    for (label, connected) in [
        ("incoming", &details.incoming),
        ("outgoing", &details.outgoing),
    ] {
        for entry in connected.iter() {
            writeln!(
                output,
                "{indent}  ({label} (kind {:?}) (peer {:?}) (provenance {}))",
                entry.kind,
                model
                    .qualified_name(entry.peer.identity)
                    .unwrap_or_default(),
                match entry.provenance {
                    RelationshipProvenance::Authored => "authored",
                    RelationshipProvenance::Implied => "implied",
                }
            )?;
        }
    }
    if details.analysis != AnalysisEvaluation::NotApplicable {
        write!(output, "{indent}  (analysis {}", details.analysis.as_str())?;
        match &details.analysis {
            AnalysisEvaluation::Verdict(passed) => write!(output, " {passed}")?,
            AnalysisEvaluation::Computed(value) => {
                write!(output, " ")?;
                write_scalar(output, value)?;
            }
            AnalysisEvaluation::Unsettled(state) => write!(output, " {state}")?,
            AnalysisEvaluation::NotApplicable | AnalysisEvaluation::NotRun => {}
        }
        writeln!(output, ")")?;
    }
    writeln!(output, "{indent})")
}

/// One relationship family, omitted entirely when the element declares nothing of its kind.
fn write_family(
    model: &sysml_resolution::PublishedResolution,
    output: &mut dyn fmt::Write,
    indent: &str,
    label: &str,
    family: &RelationshipFamily,
) -> fmt::Result {
    if family.outcome == RelationshipOutcome::NotApplicable {
        return Ok(());
    }
    write!(
        output,
        "{indent}  ({label} (outcome {})",
        family.outcome.as_str()
    )?;
    for target in family.targets.iter() {
        write!(
            output,
            " (target {:?})",
            model.qualified_name(target.identity).unwrap_or_default()
        )?;
    }
    for candidate in family.candidates.iter() {
        write!(
            output,
            " (candidate {:?})",
            model.qualified_name(candidate.identity).unwrap_or_default()
        )?;
    }
    writeln!(output, ")")
}

fn write_relationship(
    output: &mut dyn fmt::Write,
    indent: &str,
    relationship: &ElementRelationship,
) -> fmt::Result {
    write!(
        output,
        "{indent}  (relationship (kind {:?}) (provenance {})",
        relationship.kind,
        match relationship.provenance {
            RelationshipProvenance::Authored => "authored",
            RelationshipProvenance::Implied => "implied",
        }
    )?;
    if let Some(authored) = &relationship.authored {
        write!(output, " (authored {authored:?})")?;
    }
    match &relationship.target {
        RelationshipTarget::Resolved(_) => write!(output, " (target resolved)")?,
        RelationshipTarget::Ambiguous(candidates) => {
            write!(output, " (target ambiguous {})", candidates.len())?
        }
        RelationshipTarget::Unresolved => write!(output, " (target unresolved)")?,
        RelationshipTarget::Unsupported => write!(output, " (target unsupported)")?,
    }
    writeln!(output, ")")
}

fn write_multiplicity(
    output: &mut dyn fmt::Write,
    indent: &str,
    multiplicity: MultiplicityFacts,
) -> fmt::Result {
    match multiplicity {
        // Absence is the common case, and printing it on every element would bury the declared
        // ones. `[*]` still prints, because writing it is not the same as omitting it.
        MultiplicityFacts::Absent => Ok(()),
        MultiplicityFacts::Declared {
            lower,
            upper,
            ordered,
            nonunique,
        } => {
            write!(output, "{indent}  (multiplicity (lower ")?;
            write_bound(output, lower)?;
            write!(output, ") (upper ")?;
            write_bound(output, upper)?;
            writeln!(output, ") (ordered {ordered}) (nonunique {nonunique}))")
        }
    }
}

fn write_bound(output: &mut dyn fmt::Write, bound: MultiplicityBound) -> fmt::Result {
    match bound {
        MultiplicityBound::Unbounded => write!(output, "unbounded"),
        MultiplicityBound::Literal(value) => write!(output, "{value}"),
        MultiplicityBound::Expression => write!(output, "expression"),
    }
}

fn write_scalar(output: &mut dyn fmt::Write, scalar: &EvaluatedScalar) -> fmt::Result {
    match scalar {
        EvaluatedScalar::Boolean(value) => write!(output, "{value}"),
        EvaluatedScalar::Integer(value) => write!(output, "{value}"),
        EvaluatedScalar::Real(value) => write!(output, "{value}"),
        EvaluatedScalar::String(value) => write!(output, "{value:?}"),
        EvaluatedScalar::Quantity { magnitude, unit } => {
            write!(output, "(quantity ")?;
            write_scalar(output, magnitude)?;
            write!(output, " {unit:?})")
        }
    }
}

fn membership_kind_name(kind: MembershipKind) -> &'static str {
    match kind {
        MembershipKind::Owning => "owning",
        MembershipKind::Feature => "feature",
        MembershipKind::Import => "import",
        MembershipKind::Alias => "alias",
    }
}

fn visibility_name(visibility: Visibility) -> &'static str {
    match visibility {
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::Protected => "protected",
    }
}

fn visibility_provenance_name(provenance: VisibilityProvenance) -> &'static str {
    match provenance {
        VisibilityProvenance::Authored => "authored",
        VisibilityProvenance::Default => "default",
    }
}

fn portion_name(portion: PortionKind) -> &'static str {
    match portion {
        PortionKind::Snapshot => "snapshot",
        PortionKind::Timeslice => "timeslice",
    }
}

fn direction_name(direction: FeatureDirection) -> &'static str {
    match direction {
        FeatureDirection::In => "in",
        FeatureDirection::Out => "out",
        FeatureDirection::InOut => "inout",
    }
}

fn value_kind_name(kind: ValueKind) -> &'static str {
    match kind {
        ValueKind::Bind => "bind",
        ValueKind::Assign => "assign",
    }
}

fn annotation_form_name(form: AnnotationForm) -> &'static str {
    match form {
        AnnotationForm::Documentation => "doc",
        AnnotationForm::Comment => "comment",
        AnnotationForm::TextualRepresentation => "rep",
    }
}

/// Storage and implementation models are not part of this facade.
///
/// ```compile_fail
/// use sysml_query::resolved_slice::{ResolutionResults, SemanticModelStorage};
/// ```
pub struct RawStorageIsNotPublic;

#[cfg(test)]
mod tests {
    use super::{
        build, build_measured, AdmittedSource, BuildRequest, ConstructionStrategy,
        LibrarySpecializationAnchorBranch, PublishedModel, QueryOutcome, SourceKind, SymbolId,
    };

    /// The display path of an anchor handle, for assertions that name an element in prose.
    ///
    /// A handle carries no text, so a test that wants to say "this anchor is the one in `Items`"
    /// asks the publication, exactly as a host would.
    fn qualified(model: &PublishedModel, symbol: impl std::borrow::Borrow<SymbolId>) -> &str {
        model.qualified_name(*symbol.borrow()).unwrap_or_default()
    }

    #[test]
    fn immutable_publication_can_be_shared_by_async_hosts() {
        fn requires_send_sync<T: Send + Sync>() {}
        requires_send_sync::<PublishedModel>();
    }

    #[test]
    fn measured_and_unmeasured_builds_publish_the_same_semantics() {
        fn request() -> BuildRequest {
            BuildRequest::resolved(
                vec![AdmittedSource::from_memory_path(
                    "measurement-parity",
                    "model.sysml",
                    "package P { part def Vehicle; part vehicle : Vehicle; }".into(),
                    SourceKind::Workspace,
                )
                .unwrap()],
                ConstructionStrategy::Sequential,
            )
            .unwrap()
        }

        let ordinary = build(request()).unwrap();
        let (measured, _) = build_measured(request()).unwrap();
        let mut ordinary_output = String::new();
        let mut measured_output = String::new();
        ordinary
            .debug()
            .write_semantic_sexpr(&mut ordinary_output)
            .unwrap();
        measured
            .debug()
            .write_semantic_sexpr(&mut measured_output)
            .unwrap();
        assert_eq!(ordinary_output, measured_output);
    }

    #[test]
    fn type_facade_exposes_generated_library_anchor_outcomes() {
        const ITEM_RULE: &str = "sysml-2.0:8.3.10.2:checkItemDefinitionSpecialization";
        let publication = build(
            BuildRequest::resolved(
                vec![
                    AdmittedSource::from_memory_path(
                        "library",
                        "items.sysml",
                        "standard library package Items { item def Item; }".into(),
                        SourceKind::StandardLibrary,
                    )
                    .unwrap(),
                    AdmittedSource::from_memory_path(
                        "workspace",
                        "model.sysml",
                        "package Model { item def Component; }".into(),
                        SourceKind::Workspace,
                    )
                    .unwrap(),
                ],
                ConstructionStrategy::Sequential,
            )
            .unwrap(),
        )
        .unwrap();

        assert!(matches!(
            publication.types().library_specialization_anchor(ITEM_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("Items")
        ));
        assert!(matches!(
            publication
                .types()
                .library_specialization_anchor("not-a-generated-rule"),
            QueryOutcome::Unresolved
        ));
    }

    #[test]
    fn type_facade_exposes_typed_conditional_anchor_branches() {
        const POLARITY_RULE: &str =
            "sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageSpecialization";
        const MEMBERSHIP_RULE: &str =
            "sysml-2.0:8.3.20.4:checkConstraintUsageRequirementConstraintSpecialization";
        const IF_ACTION_RULE: &str = "sysml-2.0:8.3.17.10:checkIfActionUsageSpecialization";
        const FLOW_BINARY_RULE: &str = "sysml-2.0:8.3.16.2:checkFlowDefinitionBinarySpecialization";
        const FLOW_USAGE_RULE: &str = "sysml-2.0:8.3.16.3:checkFlowUsageFlowSpecialization";
        const FLOW_WITH_ENDS_RULE: &str = "kerml-1.0:8.3.4.9.2:checkFlowWithEndsSpecialization";
        const FEATURE_DATA_VALUE_RULE: &str =
            "kerml-1.0:8.3.3.3.4:checkFeatureDataValueSpecialization";
        const FEATURE_END_RULE: &str = "kerml-1.0:8.3.3.3.4:checkFeatureEndSpecialization";
        let publication = build(
            BuildRequest::resolved(
                vec![
                    AdmittedSource::from_memory_path(
                        "library",
                        "requirements.sysml",
                        "standard library package Requirements { constraint def satisfiedRequirementChecks; constraint def notSatisfiedRequirementChecks; package RequirementCheck { constraint def assumptions; constraint def constraints; } }".into(),
                        SourceKind::StandardLibrary,
                    )
                    .unwrap(),
                    AdmittedSource::from_memory_path(
                        "library",
                        "actions.sysml",
                        "standard library package Actions { action ifThenActions; action ifThenElseActions; }".into(),
                        SourceKind::StandardLibrary,
                    )
                    .unwrap(),
                    AdmittedSource::from_memory_path(
                        "library",
                        "flows.sysml",
                        "standard library package Flows { flow def Message; flow def flows; } standard library package Transfers { flow def flowTransfers; }".into(),
                        SourceKind::StandardLibrary,
                    )
                    .unwrap(),
                    AdmittedSource::from_memory_path(
                        "library",
                        "feature-anchors.sysml",
                        "standard library package Base { feature dataValues; } standard library package Links { class Link { feature participant; } }".into(),
                        SourceKind::StandardLibrary,
                    )
                    .unwrap(),
                    AdmittedSource::from_memory_path(
                        "workspace",
                        "model.sysml",
                        "package Model {}".into(),
                        SourceKind::Workspace,
                    )
                    .unwrap(),
                ],
                ConstructionStrategy::Sequential,
            )
            .unwrap(),
        )
        .unwrap();

        assert!(matches!(
            publication.types().library_specialization_anchor(POLARITY_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("satisfiedRequirementChecks")
        ));
        assert!(matches!(
            publication.types().library_specialization_anchor_branch(
                POLARITY_RULE,
                LibrarySpecializationAnchorBranch::PredicateTrue,
            ),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("notSatisfiedRequirementChecks")
        ));
        assert!(matches!(
            publication
                .types()
                .library_specialization_anchor(MEMBERSHIP_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("constraints")
        ));
        assert!(matches!(
            publication.types().library_specialization_anchor_branch(
                MEMBERSHIP_RULE,
                LibrarySpecializationAnchorBranch::PredicateTrue,
            ),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("assumptions")
        ));
        assert!(matches!(
            publication.types().library_specialization_anchor(IF_ACTION_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("ifThenActions")
        ));
        assert!(matches!(
            publication.types().library_specialization_anchor_branch(
                IF_ACTION_RULE,
                LibrarySpecializationAnchorBranch::PredicateTrue,
            ),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("ifThenElseActions")
        ));
        let flow_binary_anchor = publication
            .types()
            .library_specialization_anchor(FLOW_BINARY_RULE);
        assert!(
            matches!(flow_binary_anchor, QueryOutcome::Resolved(ref anchor) if qualified(&publication, anchor).contains("Message")),
            "expected the flow-definition anchor, got {flow_binary_anchor:?}"
        );
        assert!(matches!(
            publication.types().library_specialization_anchor(FLOW_USAGE_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("flows")
        ));
        assert!(matches!(
            publication.types().library_specialization_anchor(FLOW_WITH_ENDS_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("flowTransfers")
        ));
        assert!(matches!(
            publication
                .types()
                .library_specialization_anchor(FEATURE_DATA_VALUE_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("dataValues")
        ));
        assert!(matches!(
            publication.types().library_specialization_anchor(FEATURE_END_RULE),
            QueryOutcome::Resolved(anchor) if qualified(&publication, anchor).contains("participant")
        ));
    }

    #[test]
    fn type_facade_distinguishes_redefinition_anchor_from_unsupported_source_projection() {
        const PAYLOAD_RULE: &str = "kerml-1.0:8.3.4.9.5:checkPayloadFeatureRedefinition";
        let publication = build(
            BuildRequest::resolved(
                vec![
                    AdmittedSource::from_memory_path(
                        "library",
                        "transfers.sysml",
                        "standard library package Transfers { part def Transfer { attribute payload; } }"
                            .into(),
                        SourceKind::StandardLibrary,
                    )
                    .unwrap(),
                    AdmittedSource::from_memory_path(
                        "workspace",
                        "model.sysml",
                        "package Model {}".into(),
                        SourceKind::Workspace,
                    )
                    .unwrap(),
                ],
                ConstructionStrategy::Sequential,
            )
            .unwrap(),
        )
        .unwrap();

        let anchor_outcome = publication.types().library_rule_anchor(PAYLOAD_RULE);
        assert!(
            matches!(
                anchor_outcome,
                QueryOutcome::Resolved(ref anchor)
                    if qualified(&publication, anchor).contains("Transfers")
                        && qualified(&publication, anchor).contains("Transfer")
                        && qualified(&publication, anchor).contains("payload")
            ),
            "{anchor_outcome:?}"
        );
        assert!(matches!(
            publication
                .types()
                .library_redefinition_applicability(PAYLOAD_RULE),
            QueryOutcome::Unsupported
        ));
        assert!(matches!(
            publication
                .types()
                .library_redefinition_applicability("not-a-generated-rule"),
            QueryOutcome::Unresolved
        ));
    }

    #[test]
    fn type_facade_exposes_canonical_type_featuring_provenance() {
        let publication = build(
            BuildRequest::resolved(
                vec![AdmittedSource::from_memory_path(
                    "workspace",
                    "model.sysml",
                    "package Model { part def Vehicle { attribute mass; } }".into(),
                    SourceKind::Workspace,
                )
                .unwrap()],
                ConstructionStrategy::Sequential,
            )
            .unwrap(),
        )
        .unwrap();
        let symbols = match publication
            .inspection()
            .document_symbols("memory://workspace/model.sysml")
        {
            QueryOutcome::Resolved(symbols) => symbols,
            outcome => panic!("unexpected document-symbol outcome: {outcome:?}"),
        };
        let mass = symbols
            .iter()
            .find(|symbol| {
                publication.qualified_name(symbol.identity) == Some("Model::Vehicle::mass")
            })
            .expect("mass declaration")
            .identity;
        assert!(matches!(
            publication.types().featuring_types(mass),
            QueryOutcome::Resolved(values)
                if values.len() == 1
                    && values[0].provenance == sysml_resolution::RelationshipProvenance::Implied
        ));
    }
}
