//! Immutable semantic publication and canonical relationship-resolution results.
//!
//! The graph builder is deliberately an implementation detail of this module's build service.
//! A [`SemanticModel`] is the only settled semantic state that new consumers should retain.  The
//! existing graph is used while migrating the older builders, then is frozen behind this value;
//! no resolver or query method is allowed to mutate it after publication.

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use source_identity::{Blake3Digest, RootDigest};

use crate::semantic::graph::{DeclaredExpressionRelationshipRecord, SemanticGraph};
pub use crate::semantic::model::DerivedRelationshipRule;
use crate::semantic::model::{
    DeclaredExpressionRelationship, DeclaredRelationshipFacts, DeclaredRelationshipTarget,
    ElementKind, EvaluatedValue, EvaluationStatus, ImpliedRelationshipRule, ImportOrigin,
    ImportShape, ImportTargetPresence, NodeEvaluationFacts, NodeId, RelationshipKind, SemanticEdge,
    SemanticNode,
};
use crate::semantic::pipeline::build_structural_graph;
use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
use crate::semantic::text_span::{TextPosition, TextRange};

/// An exact set of source documents admitted to one semantic build.
#[derive(Debug, Clone)]
pub struct ImmutableSourceSnapshot {
    documents: Vec<SysmlDocument>,
}

impl ImmutableSourceSnapshot {
    pub fn new(mut documents: Vec<SysmlDocument>) -> Result<Self, SemanticBuildFailure> {
        documents.sort_by(|left, right| left.uri.as_str().cmp(right.uri.as_str()));
        for pair in documents.windows(2) {
            if pair[0].uri == pair[1].uri {
                return Err(SemanticBuildFailure::InvalidInput(format!(
                    "duplicate source URI admitted to semantic snapshot: {}",
                    pair[0].uri
                )));
            }
        }
        Ok(Self { documents })
    }

    pub fn documents(&self) -> &[SysmlDocument] {
        &self.documents
    }

    pub fn into_documents(self) -> Vec<SysmlDocument> {
        self.documents
    }
}

/// Non-source inputs that can affect semantic construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticConfiguration {
    /// Bumped whenever the resolver's observable contract changes.
    pub semantic_contract_version: String,
}

impl Default for SemanticConfiguration {
    fn default() -> Self {
        Self {
            semantic_contract_version: "canonical-resolution-v1".to_string(),
        }
    }
}

/// Content-complete identity for an immutable semantic input and publication phase.
///
/// The construction and evaluation policies are included even when two policies currently
/// produce equivalent resolution facts.  They are phase-affecting inputs: changing either may
/// change completeness, evaluation facts, indexes, diagnostics, or the publication contract in a
/// later implementation.  A cache or publication owner must never treat those phases as the same
/// model merely because the source bytes match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticModelIdentity {
    pub source_digest: String,
    pub semantic_contract_version: String,
    pub construction: ConstructionStrategy,
    pub evaluation: EvaluationPolicy,
}

impl SemanticModelIdentity {
    fn for_request(request: &SemanticBuildRequest) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"spec42-semantic-input\0");
        hasher.update(request.configuration.semantic_contract_version.as_bytes());
        hasher.update([0]);
        // Construction strategy is an execution choice. It remains part of the typed
        // publication identity below, but must not enter the content digest rendered in SMG:
        // sequential and parallel construction are required to publish byte-equivalent facts.
        hasher.update(evaluation_tag(request.evaluation));
        hasher.update([0]);
        for document in request.sources.documents() {
            hasher.update(document.uri.as_str().as_bytes());
            hasher.update([0]);
            hasher.update(source_kind_tag(document.source_kind));
            hasher.update([0]);
            hasher.update(document.content.as_bytes());
            hasher.update([0xff]);
        }
        let digest = hasher.finalize();
        Self {
            source_digest: digest.iter().map(|byte| format!("{byte:02x}")).collect(),
            semantic_contract_version: request.configuration.semantic_contract_version.clone(),
            construction: request.construction,
            evaluation: request.evaluation,
        }
    }
}

fn evaluation_tag(policy: EvaluationPolicy) -> &'static [u8] {
    match policy {
        EvaluationPolicy::ResolvedOnly => b"resolved-only",
        EvaluationPolicy::Evaluate => b"evaluate",
    }
}

fn source_kind_tag(kind: SysmlDocumentSourceKind) -> &'static [u8] {
    match kind {
        SysmlDocumentSourceKind::Workspace => b"workspace",
        SysmlDocumentSourceKind::StandardLibrary => b"standard-library",
        SysmlDocumentSourceKind::Library => b"library",
        SysmlDocumentSourceKind::External => b"external",
    }
}

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

#[derive(Debug, Clone)]
pub struct SemanticBuildRequest {
    pub sources: ImmutableSourceSnapshot,
    pub construction: ConstructionStrategy,
    pub evaluation: EvaluationPolicy,
    pub configuration: SemanticConfiguration,
}

impl SemanticBuildRequest {
    /// Returns the complete identity of this immutable build request.
    ///
    /// Publication owners use this method when issuing a background-build token.  Keeping the
    /// derivation on the request prevents callers from supplying an identity that does not match
    /// the source, construction, evaluation, or configuration inputs they actually build.
    pub fn identity(&self) -> SemanticModelIdentity {
        SemanticModelIdentity::for_request(self)
    }

    /// Precomputes the dependency-complete identity once and binds it to the exact request that
    /// will be consumed by semantic construction.
    pub fn prepare(self) -> PreparedSemanticBuildRequest {
        let identity = self.identity();
        PreparedSemanticBuildRequest {
            request: self,
            identity,
        }
    }
}

/// Owner-to-owner construction seam used by `sysml_query`.
///
/// Its fields remain private so an identity cannot be paired with different semantic inputs.
#[derive(Debug)]
pub struct PreparedSemanticBuildRequest {
    request: SemanticBuildRequest,
    identity: SemanticModelIdentity,
}

impl PreparedSemanticBuildRequest {
    pub fn identity(&self) -> &SemanticModelIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticBuildFailure {
    InvalidInput(String),
    Resolution(ResolutionFailure),
}

impl fmt::Display for SemanticBuildFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(formatter, "invalid semantic input: {message}"),
            Self::Resolution(failure) => write!(formatter, "semantic resolution failed: {failure}"),
        }
    }
}

impl std::error::Error for SemanticBuildFailure {}

/// Identity of one authored reference site.  The ordinal preserves repeated source clauses.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthoredReferenceId {
    pub source: NodeId,
    pub kind: ReferenceKind,
    pub authored_ordinal: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReferenceKind {
    FeatureTyping,
    Specialization,
    Subsetting,
    Redefinition,
    ReferenceSubsetting,
    CrossSubsetting,
    ConnectionSource,
    ConnectionTarget,
    BindSource,
    BindTarget,
    SatisfySource,
    SatisfyTarget,
    AllocateSource,
    AllocateTarget,
    FlowSource,
    FlowTarget,
    SuccessionFlowSource,
    SuccessionFlowTarget,
    PerformSource,
    PerformTarget,
    TransitionSource,
    TransitionTarget,
    InitialStateSource,
    InitialStateTarget,
    ReferenceSource,
    ReferenceTarget,
    DependencySource,
    DependencyTarget,
    DerivationSource,
    DerivationTarget,
    NamespaceImport,
    MembershipImport,
}

impl ReferenceKind {
    fn relationship_kind(self) -> Option<RelationshipKind> {
        Some(match self {
            Self::FeatureTyping => RelationshipKind::Typing,
            Self::Specialization => RelationshipKind::Specializes,
            Self::Subsetting => RelationshipKind::Subsetting,
            Self::Redefinition => RelationshipKind::Redefinition,
            Self::ReferenceSubsetting => RelationshipKind::ReferenceSubsetting,
            Self::CrossSubsetting => RelationshipKind::CrossSubsetting,
            Self::NamespaceImport | Self::MembershipImport => return None,
            Self::ConnectionSource | Self::ConnectionTarget => RelationshipKind::Connection,
            Self::BindSource | Self::BindTarget => RelationshipKind::Bind,
            Self::SatisfySource | Self::SatisfyTarget => RelationshipKind::Satisfy,
            Self::AllocateSource | Self::AllocateTarget => RelationshipKind::Allocate,
            Self::FlowSource | Self::FlowTarget => RelationshipKind::Flow,
            Self::SuccessionFlowSource | Self::SuccessionFlowTarget => {
                RelationshipKind::SuccessionFlow
            }
            Self::PerformSource | Self::PerformTarget => RelationshipKind::Perform,
            Self::TransitionSource | Self::TransitionTarget => RelationshipKind::Transition,
            Self::InitialStateSource | Self::InitialStateTarget => RelationshipKind::InitialState,
            Self::ReferenceSource | Self::ReferenceTarget => RelationshipKind::Reference,
            Self::DependencySource | Self::DependencyTarget => RelationshipKind::Dependency,
            Self::DerivationSource | Self::DerivationTarget => RelationshipKind::Derivation,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionOutcome {
    Resolved { target: NodeId },
    Unresolved,
    Ambiguous { candidates: Vec<NodeId> },
    UnsupportedFiltered,
}

impl ResolutionOutcome {
    pub fn resolved_target(&self) -> Option<&NodeId> {
        match self {
            Self::Resolved { target } => Some(target),
            Self::Unresolved | Self::Ambiguous { .. } | Self::UnsupportedFiltered => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolutionFact {
    pub reference: AuthoredReferenceId,
    pub authored_target: String,
    pub authored_range: Option<TextRange>,
    pub outcome: ResolutionOutcome,
    pub import: Option<ResolutionImportFact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionImportFact {
    pub origin: ImportOrigin,
    pub shape: ImportShape,
    pub recursive: bool,
    pub conformance: ImportConformanceOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportConformanceOutcome {
    Valid,
    MissingTarget,
    NotCheckedUnresolved,
    NotCheckedAmbiguous,
    NotCheckedUnsupportedFiltered,
    NamespaceKindMismatch { actual: ElementKind },
    RecursiveNonNamespace { actual: ElementKind },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionProvenance {
    Authored,
    Implied(ImpliedRelationshipRule),
    Derived(DerivedRelationshipRule),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedRelationship {
    pub source: NodeId,
    pub target: NodeId,
    pub kind: RelationshipKind,
    pub provenance: ResolutionProvenance,
    pub authored_reference: Option<AuthoredReferenceId>,
    pub expression: Option<DeclaredExpressionRelationship>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionFailure {
    DependencyDeadlock {
        passes: usize,
        changing_families: Vec<String>,
        pending_references: Vec<AuthoredReferenceId>,
    },
    Oscillation {
        passes: usize,
        changing_families: Vec<String>,
        pending_references: Vec<AuthoredReferenceId>,
    },
    SafetyBound {
        passes: usize,
        changing_families: Vec<String>,
        pending_references: Vec<AuthoredReferenceId>,
    },
}

impl fmt::Display for ResolutionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DependencyDeadlock {
                passes,
                changing_families,
                pending_references,
            }
            | Self::Oscillation {
                passes,
                changing_families,
                pending_references,
            }
            | Self::SafetyBound {
                passes,
                changing_families,
                pending_references,
            } => write!(
                formatter,
                "did not converge after {passes} passes (families: {changing_families:?}, pending references: {})",
                pending_references.len()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkingOutcome<K, V> {
    Pending { dependencies: Vec<K> },
    Final(V),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FixedPointFailure<K> {
    DependencyDeadlock {
        passes: usize,
        pending: Vec<K>,
        changing: Vec<K>,
    },
    Oscillation {
        passes: usize,
        pending: Vec<K>,
        changing: Vec<K>,
    },
    SafetyBound {
        passes: usize,
        pending: Vec<K>,
        changing: Vec<K>,
    },
}

fn solve_fixed_point<K, V, F>(
    mut state: BTreeMap<K, WorkingOutcome<K, V>>,
    max_passes: usize,
    mut step: F,
) -> Result<BTreeMap<K, WorkingOutcome<K, V>>, FixedPointFailure<K>>
where
    K: Clone + Ord,
    V: Clone + Eq,
    F: FnMut(&BTreeMap<K, WorkingOutcome<K, V>>) -> BTreeMap<K, WorkingOutcome<K, V>>,
{
    let pending_keys = |state: &BTreeMap<K, WorkingOutcome<K, V>>| {
        state
            .iter()
            .filter_map(|(key, outcome)| {
                matches!(outcome, WorkingOutcome::Pending { .. }).then_some(key.clone())
            })
            .collect::<Vec<_>>()
    };
    let changed_keys = |before: &BTreeMap<K, WorkingOutcome<K, V>>,
                        after: &BTreeMap<K, WorkingOutcome<K, V>>| {
        before
            .keys()
            .chain(after.keys())
            .cloned()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .filter(|key| before.get(key) != after.get(key))
            .collect::<Vec<_>>()
    };
    if max_passes == 0 {
        return Err(FixedPointFailure::SafetyBound {
            passes: 0,
            pending: pending_keys(&state),
            changing: Vec::new(),
        });
    }
    let mut seen = Vec::new();
    for pass in 1..=max_passes {
        let next = step(&state);
        if next == state {
            let pending = pending_keys(&next);
            if pending.is_empty() {
                return Ok(next);
            }
            return Err(FixedPointFailure::DependencyDeadlock {
                passes: pass,
                pending,
                changing: Vec::new(),
            });
        }
        if seen.iter().any(|previous| previous == &next) {
            return Err(FixedPointFailure::Oscillation {
                passes: pass,
                pending: pending_keys(&next),
                changing: changed_keys(&state, &next),
            });
        }
        if pass == max_passes {
            return Err(FixedPointFailure::SafetyBound {
                passes: pass,
                pending: pending_keys(&next),
                changing: changed_keys(&state, &next),
            });
        }
        seen.push(next.clone());
        state = next;
    }
    unreachable!("positive fixed-point bound always returns from the loop")
}

/// Settled relationship results.  Adjacency is derived from `facts` and is never an independent
/// source of semantic truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolutionState {
    facts: Vec<ResolutionFact>,
    relationships: Vec<ResolvedRelationship>,
    inherited_features: Vec<StructuralDiagnosticFact>,
}

impl ResolutionState {
    pub(crate) fn facts(&self) -> &[ResolutionFact] {
        &self.facts
    }

    pub(crate) fn outcome(&self, reference: &AuthoredReferenceId) -> Option<&ResolutionOutcome> {
        self.facts
            .binary_search_by(|fact| fact.reference.cmp(reference))
            .ok()
            .map(|index| &self.facts[index].outcome)
    }

    pub(crate) fn relationships(&self) -> &[ResolvedRelationship] {
        &self.relationships
    }

    pub(crate) fn inherited_features(&self) -> &[StructuralDiagnosticFact] {
        &self.inherited_features
    }

    pub(crate) fn install_relationship_projection(&self, graph: &mut SemanticGraph) {
        for relationship in &self.relationships {
            crate::semantic::relationships::add_semantic_edge_once(
                graph,
                &relationship.source,
                &relationship.target,
                match relationship.provenance {
                    ResolutionProvenance::Authored => SemanticEdge::plain(
                        relationship.kind.clone(),
                        crate::semantic::model::ConstructionOwner::DocumentConstruction,
                    ),
                    ResolutionProvenance::Implied(rule) => {
                        SemanticEdge::implied(relationship.kind.clone(), rule)
                    }
                    ResolutionProvenance::Derived(rule) => {
                        SemanticEdge::derived(relationship.kind.clone(), rule)
                    }
                },
            );
        }
    }
}

/// Private resolver database.  The current implementation consumes the already-built structural
/// graph at the publication barrier; future fixed-point families belong here and not in graph
/// mutation or downstream consumers.
pub(crate) struct ResolutionDb<'a> {
    graph: &'a SemanticGraph,
}

impl<'a> ResolutionDb<'a> {
    pub(crate) fn new(graph: &'a SemanticGraph) -> Self {
        Self { graph }
    }

    fn solve_with_max_passes(
        self,
        max_passes: usize,
    ) -> Result<ResolutionState, ResolutionFailure> {
        let references = authored_reference_ids(self.graph);
        let initial = references
            .iter()
            .map(|reference| {
                (
                    reference.clone(),
                    WorkingOutcome::Pending {
                        dependencies: reference_dependencies(self.graph, reference),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        let result = solve_fixed_point(initial, max_passes, |previous| {
            let solved = self.solve_once(Some(previous));
            // Keep unresolved slots in the working state.  Omitting a pending key would make
            // the generic fixed-point driver mistake a partial pass for convergence.
            let mut next = previous.clone();
            for fact in solved.facts() {
                let Some(WorkingOutcome::Pending { dependencies }) = previous.get(&fact.reference)
                else {
                    next.insert(
                        fact.reference.clone(),
                        WorkingOutcome::Final(fact.outcome.clone()),
                    );
                    continue;
                };
                let mut failed_dependency = false;
                let mut waiting_dependency = false;
                for dependency in dependencies {
                    match previous.get(dependency) {
                        Some(WorkingOutcome::Final(ResolutionOutcome::Resolved { .. })) => {}
                        Some(WorkingOutcome::Final(_)) => failed_dependency = true,
                        Some(WorkingOutcome::Pending { .. }) | None => waiting_dependency = true,
                    }
                }
                if failed_dependency {
                    next.insert(
                        fact.reference.clone(),
                        WorkingOutcome::Final(ResolutionOutcome::Unresolved),
                    );
                } else if !waiting_dependency {
                    next.insert(
                        fact.reference.clone(),
                        WorkingOutcome::Final(fact.outcome.clone()),
                    );
                }
            }
            next
        });
        match result {
            Ok(settled) => Ok(self.solve_once(Some(&settled))),
            Err(FixedPointFailure::DependencyDeadlock {
                passes,
                pending,
                changing,
            }) => Err(ResolutionFailure::DependencyDeadlock {
                passes,
                changing_families: reference_families(&changing),
                pending_references: pending,
            }),
            Err(FixedPointFailure::Oscillation {
                passes,
                pending,
                changing,
            }) => Err(ResolutionFailure::Oscillation {
                passes,
                changing_families: reference_families(&changing),
                pending_references: pending,
            }),
            Err(FixedPointFailure::SafetyBound {
                passes,
                pending,
                changing,
            }) => Err(ResolutionFailure::SafetyBound {
                passes,
                changing_families: reference_families(&changing),
                pending_references: pending,
            }),
        }
    }

    fn solve_once(
        &self,
        previous: Option<
            &BTreeMap<AuthoredReferenceId, WorkingOutcome<AuthoredReferenceId, ResolutionOutcome>>,
        >,
    ) -> ResolutionState {
        let mut facts = Vec::new();
        let mut relationships = Vec::new();
        let nodes = self.graph.semantic_nodes();

        // Structural builders record authored targets as facts only. Candidate discovery starts
        // here, against that stable input, and never reads relationship edges or mutates the
        // graph. A single deterministic pass is sufficient for the currently owned families;
        // recursive inherited-member traversal has its own cycle guard below.
        for node in nodes {
            for (kind, targets) in authored_relationships(&node.declared_facts.relationships) {
                let relationship_kind = kind.relationship_kind();
                for (ordinal, authored) in targets.into_iter().enumerate() {
                    let reference = AuthoredReferenceId {
                        source: node.id.clone(),
                        kind,
                        authored_ordinal: ordinal as u32,
                    };
                    let blocked = previous.is_some_and(|state| {
                        reference_dependencies(self.graph, &reference)
                            .iter()
                            .any(|dependency| {
                                !matches!(
                                    state.get(dependency),
                                    Some(WorkingOutcome::Final(ResolutionOutcome::Resolved { .. }))
                                )
                            })
                    });
                    let outcome = if blocked {
                        ResolutionOutcome::Unresolved
                    } else {
                        let mut candidates =
                            explicit_name_candidates(self.graph, &node, &authored.reference);
                        candidates.sort_by(node_id_order);
                        candidates.dedup();
                        match candidates.as_slice() {
                            [target] => ResolutionOutcome::Resolved {
                                target: target.clone(),
                            },
                            [] => ResolutionOutcome::Unresolved,
                            candidates => ResolutionOutcome::Ambiguous {
                                candidates: candidates.to_vec(),
                            },
                        }
                    };
                    if let Some(target) = outcome.resolved_target() {
                        let relationship_kind =
                            relationship_kind.clone().expect("relationship kind");
                        relationships.push(ResolvedRelationship {
                            source: node.id.clone(),
                            target: target.clone(),
                            kind: relationship_kind,
                            provenance: ResolutionProvenance::Authored,
                            authored_reference: Some(reference.clone()),
                            expression: None,
                        });
                    }
                    facts.push(ResolutionFact {
                        reference,
                        authored_target: authored.reference,
                        authored_range: authored.range,
                        outcome,
                        import: None,
                    });
                }
            }
            if let Some(membership) = &node.declared_facts.membership {
                if let Some(import) = &membership.import {
                    let kind = match import.shape {
                        crate::semantic::model::ImportShape::Namespace
                        | crate::semantic::model::ImportShape::FilteredNamespace => {
                            ReferenceKind::NamespaceImport
                        }
                        crate::semantic::model::ImportShape::Membership => {
                            ReferenceKind::MembershipImport
                        }
                    };
                    let outcome = match crate::semantic::import_resolution::resolve_import_target(
                        self.graph,
                        &node,
                    ) {
                        crate::semantic::import_resolution::ImportTargetResolution::NotApplicable => {
                            ResolutionOutcome::Unresolved
                        }
                        crate::semantic::import_resolution::ImportTargetResolution::Resolved {
                            target,
                        } => ResolutionOutcome::Resolved { target },
                        crate::semantic::import_resolution::ImportTargetResolution::Unresolved => {
                            ResolutionOutcome::Unresolved
                        }
                        crate::semantic::import_resolution::ImportTargetResolution::Ambiguous {
                            mut candidates,
                        } => {
                            candidates.sort_by(node_id_order);
                            candidates.dedup();
                            ResolutionOutcome::Ambiguous { candidates }
                        }
                        crate::semantic::import_resolution::ImportTargetResolution::UnsupportedFiltered => {
                            ResolutionOutcome::UnsupportedFiltered
                        }
                    };
                    let resolved_kind = outcome
                        .resolved_target()
                        .and_then(|target| self.graph.get_node(target))
                        .map(|node| &node.element_kind);
                    let conformance = import_conformance(import, &outcome, resolved_kind);
                    facts.push(ResolutionFact {
                        reference: AuthoredReferenceId {
                            source: node.id.clone(),
                            kind,
                            authored_ordinal: 0,
                        },
                        authored_target: import.target.reference.clone(),
                        authored_range: import.target.range,
                        outcome,
                        import: Some(ResolutionImportFact {
                            origin: import.origin,
                            shape: import.shape,
                            recursive: import.recursive,
                            conformance,
                        }),
                    });
                }
            }
        }
        let mut expression_relationships = self
            .graph
            .declared_expression_relationships
            .iter()
            .collect::<Vec<_>>();
        expression_relationships.sort_by(|left, right| {
            left.owner
                .cmp(&right.owner)
                .then_with(|| left.authored_ordinal.cmp(&right.authored_ordinal))
                .then_with(|| {
                    expression_range_order(Some(&left.relationship))
                        .cmp(&expression_range_order(Some(&right.relationship)))
                })
        });
        for record in expression_relationships {
            resolve_expression_relationship(self.graph, record, &mut facts, &mut relationships);
        }
        derive_implied_relationships(self.graph, &mut relationships);
        derive_case_subject_relationships(self.graph, &facts, &mut relationships);
        let inherited_features = derive_inherited_feature_diagnostics(self.graph, &facts);

        facts.sort_by(|left, right| left.reference.cmp(&right.reference));
        relationships.sort_by(|left, right| {
            (&left.source, &left.kind, &left.target)
                .cmp(&(&right.source, &right.kind, &right.target))
                .then_with(|| left.authored_reference.cmp(&right.authored_reference))
                .then_with(|| {
                    provenance_order(left.provenance).cmp(&provenance_order(right.provenance))
                })
                .then_with(|| {
                    expression_range_order(left.expression.as_ref())
                        .cmp(&expression_range_order(right.expression.as_ref()))
                })
        });
        relationships.dedup();
        ResolutionState {
            facts,
            relationships,
            inherited_features,
        }
    }
}

fn derive_inherited_feature_diagnostics(
    graph: &SemanticGraph,
    facts: &[ResolutionFact],
) -> Vec<StructuralDiagnosticFact> {
    let resolved_targets = |source: &NodeId, kind: ReferenceKind| {
        facts
            .iter()
            .filter(|fact| fact.reference.source == *source && fact.reference.kind == kind)
            .filter_map(|fact| fact.outcome.resolved_target().cloned())
            .collect::<Vec<_>>()
    };

    let nodes = graph.semantic_nodes();
    let mut output = Vec::new();
    for node in &nodes {
        if node.element_kind == ElementKind::Ref
            || !node.declared_facts.relationships.redefinition.is_empty()
        {
            continue;
        }
        let Some(authored_value) = node.expression_text.value.clone() else {
            continue;
        };
        let Some(owner_id) = node.parent_id.as_ref() else {
            continue;
        };
        let Some(owner) = graph.get_node(owner_id) else {
            continue;
        };
        if owner.element_kind == ElementKind::MetadataUsage || node.name.trim().is_empty() {
            continue;
        }

        let mut pending = resolved_targets(&owner.id, ReferenceKind::FeatureTyping);
        if pending.is_empty() {
            pending.push(owner.id.clone());
        }
        let mut visited = std::collections::BTreeSet::new();
        let mut inherited = None;
        while let Some(type_id) = pending.pop() {
            if !visited.insert(type_id.clone()) {
                continue;
            }
            if let Some(candidate) = nodes.iter().find(|candidate| {
                candidate.parent_id.as_ref() == Some(&type_id) && candidate.name == node.name
            }) {
                inherited = Some(candidate);
                break;
            }
            pending.extend(resolved_targets(&type_id, ReferenceKind::Specialization));
        }
        let Some(inherited) = inherited else {
            continue;
        };
        let inherited_type = inherited
            .declared_facts
            .relationships
            .typing
            .first()
            .map(|target| target.reference.clone());
        let inherited_is_enum = resolved_targets(&inherited.id, ReferenceKind::FeatureTyping)
            .into_iter()
            .filter_map(|target| graph.get_node(&target))
            .any(|target| target.element_kind == ElementKind::EnumDef);
        output.push(StructuralDiagnosticFact {
            feature: node.id.clone(),
            inherited_feature: inherited.id.clone(),
            feature_name: node.name.clone(),
            feature_kind: node.element_kind.clone(),
            range: node.range,
            inherited_range: inherited.range,
            inherited_feature_name: inherited.name.clone(),
            inherited_feature_kind: inherited.element_kind.clone(),
            inherited_type,
            inherited_is_enum,
            authored_value: Some(authored_value),
        });
    }
    output.sort_by(|left, right| left.feature.cmp(&right.feature));
    output
}

fn resolve_expression_relationship(
    graph: &SemanticGraph,
    record: &DeclaredExpressionRelationshipRecord,
    facts: &mut Vec<ResolutionFact>,
    relationships: &mut Vec<ResolvedRelationship>,
) {
    let owner = graph.get_node(&record.owner);
    let expression = &record.relationship;
    let (source_kind, target_kind) = match expression.kind {
        RelationshipKind::Connection => (
            ReferenceKind::ConnectionSource,
            ReferenceKind::ConnectionTarget,
        ),
        RelationshipKind::Bind => (ReferenceKind::BindSource, ReferenceKind::BindTarget),
        RelationshipKind::Satisfy => (ReferenceKind::SatisfySource, ReferenceKind::SatisfyTarget),
        RelationshipKind::Allocate => {
            (ReferenceKind::AllocateSource, ReferenceKind::AllocateTarget)
        }
        RelationshipKind::Flow => (ReferenceKind::FlowSource, ReferenceKind::FlowTarget),
        RelationshipKind::SuccessionFlow => (
            ReferenceKind::SuccessionFlowSource,
            ReferenceKind::SuccessionFlowTarget,
        ),
        RelationshipKind::Perform => (ReferenceKind::PerformSource, ReferenceKind::PerformTarget),
        RelationshipKind::Transition => (
            ReferenceKind::TransitionSource,
            ReferenceKind::TransitionTarget,
        ),
        RelationshipKind::InitialState => (
            ReferenceKind::InitialStateSource,
            ReferenceKind::InitialStateTarget,
        ),
        RelationshipKind::Reference => (
            ReferenceKind::ReferenceSource,
            ReferenceKind::ReferenceTarget,
        ),
        RelationshipKind::Dependency => (
            ReferenceKind::DependencySource,
            ReferenceKind::DependencyTarget,
        ),
        RelationshipKind::Derivation => (
            ReferenceKind::DerivationSource,
            ReferenceKind::DerivationTarget,
        ),
        _ => return,
    };
    let source_outcome =
        resolve_expression_endpoint_outcome(graph, owner, &expression.source_expression);
    let target_outcome =
        resolve_expression_endpoint_outcome(graph, owner, &expression.target_expression);
    facts.push(ResolutionFact {
        reference: AuthoredReferenceId {
            source: record.owner.clone(),
            kind: source_kind,
            authored_ordinal: record.authored_ordinal,
        },
        authored_target: expression.source_expression.clone(),
        authored_range: Some(expression.source_range),
        outcome: source_outcome.clone(),
        import: None,
    });
    facts.push(ResolutionFact {
        reference: AuthoredReferenceId {
            source: record.owner.clone(),
            kind: target_kind,
            authored_ordinal: record.authored_ordinal,
        },
        authored_target: expression.target_expression.clone(),
        authored_range: expression.target_range,
        outcome: target_outcome.clone(),
        import: None,
    });
    if let (
        ResolutionOutcome::Resolved { target: source },
        ResolutionOutcome::Resolved { target },
    ) = (source_outcome, target_outcome)
    {
        relationships.push(ResolvedRelationship {
            source,
            target,
            kind: expression.kind.clone(),
            provenance: ResolutionProvenance::Authored,
            authored_reference: Some(AuthoredReferenceId {
                source: record.owner.clone(),
                kind: source_kind,
                authored_ordinal: record.authored_ordinal,
            }),
            expression: Some(expression.clone()),
        });
    }
}

fn resolve_expression_endpoint_outcome(
    graph: &SemanticGraph,
    owner: Option<&SemanticNode>,
    authored: &str,
) -> ResolutionOutcome {
    // Endpoint expressions are authored by the containing scope itself, whereas a feature
    // typing reference is authored by the child node. Start lookup at the scope owner so
    // `connect left to right` can see sibling members without falling back to a root search.
    let Some(owner) = owner else {
        return ResolutionOutcome::Unresolved;
    };
    let mut context = owner.clone();
    context.parent_id = Some(owner.id.clone());
    let mut candidates = explicit_name_candidates(graph, &context, authored);
    candidates.sort_by(node_id_order);
    candidates.dedup();
    match candidates.as_slice() {
        [target] => ResolutionOutcome::Resolved {
            target: target.clone(),
        },
        [] => ResolutionOutcome::Unresolved,
        candidates => ResolutionOutcome::Ambiguous {
            candidates: candidates.to_vec(),
        },
    }
}

fn derive_implied_relationships(
    graph: &SemanticGraph,
    relationships: &mut Vec<ResolvedRelationship>,
) {
    let mut nodes = graph.semantic_nodes();
    nodes.sort_by(|left, right| node_id_order(&left.id, &right.id));
    for source in nodes {
        let Some(specification) = source
            .element_kind
            .universal_standard_library_relationship()
        else {
            continue;
        };
        let mut candidates = graph
            .node_ids_for_qualified_name(specification.target.qualified_name())
            .unwrap_or_default()
            .iter()
            .filter(|candidate| graph.standard_library_uris.contains(&candidate.uri))
            .filter(|candidate| **candidate != source.id)
            .cloned()
            .collect::<Vec<_>>();
        candidates.sort_by(node_id_order);
        candidates.dedup();
        let [target] = candidates.as_slice() else {
            continue;
        };
        let authored_equivalent_exists = relationships.iter().any(|relationship| {
            relationship.source == source.id
                && relationship.target == *target
                && relationship.kind == specification.kind
                && relationship.provenance == ResolutionProvenance::Authored
        });
        if !authored_equivalent_exists {
            relationships.push(ResolvedRelationship {
                source: source.id,
                target: target.clone(),
                kind: specification.kind.clone(),
                provenance: ResolutionProvenance::Implied(
                    ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
                ),
                authored_reference: None,
                expression: None,
            });
        }
    }
}

fn derive_case_subject_relationships(
    graph: &SemanticGraph,
    facts: &[ResolutionFact],
    relationships: &mut Vec<ResolvedRelationship>,
) {
    for case in graph.semantic_nodes().into_iter().filter(is_case_kind) {
        let mut subject_nodes = graph
            .children_of(&case)
            .into_iter()
            .filter(|child| child.element_kind == ElementKind::Subject)
            .collect::<Vec<_>>();
        for child in graph.children_of(&case) {
            if child.element_kind == ElementKind::Objective {
                subject_nodes.extend(
                    graph
                        .children_of(child)
                        .into_iter()
                        .filter(|nested| nested.element_kind == ElementKind::VerifiedRequirement),
                );
            }
        }
        for subject in subject_nodes {
            let (kind, references) = if subject.element_kind == ElementKind::Subject {
                (
                    ReferenceKind::FeatureTyping,
                    &subject.declared_facts.relationships.typing,
                )
            } else {
                (
                    ReferenceKind::ReferenceSubsetting,
                    &subject.declared_facts.relationships.subject,
                )
            };
            for (ordinal, _) in references.iter().enumerate() {
                let reference = AuthoredReferenceId {
                    source: subject.id.clone(),
                    kind,
                    authored_ordinal: ordinal as u32,
                };
                if let Some(ResolutionOutcome::Resolved { target }) = facts
                    .iter()
                    .find(|fact| fact.reference == reference)
                    .map(|fact| &fact.outcome)
                {
                    relationships.push(ResolvedRelationship {
                        source: case.id.clone(),
                        target: target.clone(),
                        kind: RelationshipKind::Subject,
                        provenance: ResolutionProvenance::Derived(
                            DerivedRelationshipRule::CaseSubjectFromTypedSubject,
                        ),
                        authored_reference: None,
                        expression: None,
                    });
                }
            }
        }
    }
}

fn is_case_kind(node: &SemanticNode) -> bool {
    matches!(
        node.element_kind,
        ElementKind::AnalysisDef
            | ElementKind::Analysis
            | ElementKind::VerificationDef
            | ElementKind::Verification
            | ElementKind::UseCaseDef
            | ElementKind::UseCase
            | ElementKind::ConcernDef
            | ElementKind::Concern
            | ElementKind::RequirementDef
            | ElementKind::Requirement
    )
}

fn authored_reference_ids(graph: &SemanticGraph) -> Vec<AuthoredReferenceId> {
    let mut references = Vec::new();
    for node in graph.semantic_nodes() {
        for (kind, targets) in authored_relationships(&node.declared_facts.relationships) {
            references.extend(targets.into_iter().enumerate().map(|(ordinal, _)| {
                AuthoredReferenceId {
                    source: node.id.clone(),
                    kind,
                    authored_ordinal: ordinal as u32,
                }
            }));
        }
        if let Some(import) = node
            .declared_facts
            .membership
            .as_ref()
            .and_then(|membership| membership.import.as_ref())
        {
            let kind = match import.shape {
                crate::semantic::model::ImportShape::Membership => ReferenceKind::MembershipImport,
                crate::semantic::model::ImportShape::Namespace
                | crate::semantic::model::ImportShape::FilteredNamespace => {
                    ReferenceKind::NamespaceImport
                }
            };
            references.push(AuthoredReferenceId {
                source: node.id.clone(),
                kind,
                authored_ordinal: 0,
            });
        }
    }
    for record in &graph.declared_expression_relationships {
        if let Some((source_kind, target_kind)) =
            expression_reference_kinds(record.relationship.kind.clone())
        {
            references.push(AuthoredReferenceId {
                source: record.owner.clone(),
                kind: source_kind,
                authored_ordinal: record.authored_ordinal,
            });
            references.push(AuthoredReferenceId {
                source: record.owner.clone(),
                kind: target_kind,
                authored_ordinal: record.authored_ordinal,
            });
        }
    }
    references.sort();
    references.dedup();
    references
}

fn expression_reference_kinds(kind: RelationshipKind) -> Option<(ReferenceKind, ReferenceKind)> {
    Some(match kind {
        RelationshipKind::Connection => (
            ReferenceKind::ConnectionSource,
            ReferenceKind::ConnectionTarget,
        ),
        RelationshipKind::Bind => (ReferenceKind::BindSource, ReferenceKind::BindTarget),
        RelationshipKind::Satisfy => (ReferenceKind::SatisfySource, ReferenceKind::SatisfyTarget),
        RelationshipKind::Allocate => {
            (ReferenceKind::AllocateSource, ReferenceKind::AllocateTarget)
        }
        RelationshipKind::Flow => (ReferenceKind::FlowSource, ReferenceKind::FlowTarget),
        RelationshipKind::SuccessionFlow => (
            ReferenceKind::SuccessionFlowSource,
            ReferenceKind::SuccessionFlowTarget,
        ),
        RelationshipKind::Perform => (ReferenceKind::PerformSource, ReferenceKind::PerformTarget),
        RelationshipKind::Transition => (
            ReferenceKind::TransitionSource,
            ReferenceKind::TransitionTarget,
        ),
        RelationshipKind::InitialState => (
            ReferenceKind::InitialStateSource,
            ReferenceKind::InitialStateTarget,
        ),
        RelationshipKind::Reference => (
            ReferenceKind::ReferenceSource,
            ReferenceKind::ReferenceTarget,
        ),
        RelationshipKind::Dependency => (
            ReferenceKind::DependencySource,
            ReferenceKind::DependencyTarget,
        ),
        RelationshipKind::Derivation => (
            ReferenceKind::DerivationSource,
            ReferenceKind::DerivationTarget,
        ),
        _ => return None,
    })
}

fn reference_dependencies(
    graph: &SemanticGraph,
    reference: &AuthoredReferenceId,
) -> Vec<AuthoredReferenceId> {
    let Some(node) = graph.get_node(&reference.source) else {
        return Vec::new();
    };
    let owner = node
        .parent_id
        .as_ref()
        .and_then(|parent| graph.get_node(parent))
        .unwrap_or(node);
    if matches!(
        reference.kind,
        ReferenceKind::FeatureTyping
            | ReferenceKind::ConnectionSource
            | ReferenceKind::ConnectionTarget
            | ReferenceKind::BindSource
            | ReferenceKind::BindTarget
            | ReferenceKind::FlowSource
            | ReferenceKind::FlowTarget
    ) {
        return owner
            .declared_facts
            .relationships
            .specializes
            .iter()
            .enumerate()
            .map(|(ordinal, _)| AuthoredReferenceId {
                source: owner.id.clone(),
                kind: ReferenceKind::Specialization,
                authored_ordinal: ordinal as u32,
            })
            .collect();
    }
    Vec::new()
}

fn reference_families(references: &[AuthoredReferenceId]) -> Vec<String> {
    let mut families = references
        .iter()
        .map(|reference| format!("{:?}", reference.kind))
        .collect::<Vec<_>>();
    families.sort();
    families.dedup();
    families
}

fn explicit_name_candidates(
    graph: &SemanticGraph,
    source: &SemanticNode,
    authored: &str,
) -> Vec<NodeId> {
    let normalized = authored
        .trim()
        .trim_start_matches('~')
        .trim_matches(['\'', '"'])
        .replace('.', "::");
    if normalized.contains("::") {
        let mut parent = source.parent_id.clone();
        while let Some(current) = parent {
            let qualified = format!("{}::{normalized}", current.qualified_name);
            if let Some(ids) = graph.node_ids_for_qualified_name(&qualified) {
                return ids.to_vec();
            }
            parent = graph
                .get_node(&current)
                .and_then(|node| node.parent_id.clone());
        }
        return graph
            .node_ids_for_qualified_name(&normalized)
            .unwrap_or(&[])
            .to_vec();
    }
    let mut children = BTreeMap::<Option<NodeId>, Vec<NodeId>>::new();
    for node in graph.semantic_nodes() {
        if simple_name_matches(&node, &normalized) {
            children
                .entry(node.parent_id.clone())
                .or_default()
                .push(node.id.clone());
        }
    }
    let mut parent = source.parent_id.clone();
    let mut candidates = Vec::new();
    while let Some(current) = parent {
        if let Some(local) = children.get(&Some(current.clone())) {
            candidates.extend(local.iter().cloned());
            if !candidates.is_empty() {
                return candidates;
            }
        }
        let mut visited = BTreeMap::new();
        let inherited = inherited_members_named(graph, &current, &normalized, &mut visited);
        if !inherited.is_empty() {
            return inherited;
        }
        parent = graph
            .get_node(&current)
            .and_then(|node| node.parent_id.clone());
    }
    // Imports are consulted only after lexical scope has no binding. The import resolver is the
    // owning implementation for visibility, recursive exports, and cycle guards.
    candidates.extend(
        crate::semantic::import_resolution::resolve_imported_node_ids_for_simple_name(
            graph,
            source,
            &normalized,
        ),
    );
    candidates.extend(children.get(&None).into_iter().flatten().cloned());
    candidates
}

fn simple_name_matches(node: &SemanticNode, name: &str) -> bool {
    node.name == name
        || node.declared_name.as_deref() == Some(name)
        || node
            .attributes
            .get("shortName")
            .and_then(serde_json::Value::as_str)
            == Some(name)
}

/// Collect members inherited through specialization. The visited set is keyed by semantic owner
/// identity, so recursive specialization cycles terminate and a diamond is deduplicated.
fn inherited_members_named(
    graph: &SemanticGraph,
    owner_id: &NodeId,
    member_name: &str,
    visited: &mut BTreeMap<NodeId, ()>,
) -> Vec<NodeId> {
    if visited.insert(owner_id.clone(), ()).is_some() {
        return Vec::new();
    }
    let Some(owner) = graph.get_node(owner_id) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for specialization in &owner.declared_facts.relationships.specializes {
        let bases = explicit_name_candidates(graph, owner, &specialization.reference);
        for base_id in bases {
            for node in graph.semantic_nodes() {
                if node.parent_id.as_ref() == Some(&base_id)
                    && simple_name_matches(&node, member_name)
                {
                    out.push(node.id.clone());
                }
            }
            out.extend(inherited_members_named(
                graph,
                &base_id,
                member_name,
                visited,
            ));
        }
    }
    out.sort_by(node_id_order);
    out.dedup();
    out
}

fn authored_relationships(
    facts: &DeclaredRelationshipFacts,
) -> Vec<(ReferenceKind, Vec<DeclaredRelationshipTarget>)> {
    let mut reference_subsetting = facts.reference_subsetting.clone();
    reference_subsetting.extend(facts.subject.clone());
    vec![
        (ReferenceKind::FeatureTyping, facts.typing.clone()),
        (ReferenceKind::Specialization, facts.specializes.clone()),
        (ReferenceKind::Subsetting, facts.subsetting.clone()),
        (ReferenceKind::Redefinition, facts.redefinition.clone()),
        (ReferenceKind::ReferenceSubsetting, reference_subsetting),
        (
            ReferenceKind::CrossSubsetting,
            facts.cross_subsetting.clone(),
        ),
        (ReferenceKind::ConnectionSource, facts.connection.clone()),
        (ReferenceKind::BindSource, facts.bind.clone()),
        (ReferenceKind::SatisfySource, facts.satisfy.clone()),
        (ReferenceKind::AllocateSource, facts.allocate.clone()),
        (ReferenceKind::FlowSource, facts.flow.clone()),
        (
            ReferenceKind::SuccessionFlowSource,
            facts.succession_flow.clone(),
        ),
        (ReferenceKind::PerformSource, facts.perform.clone()),
        (ReferenceKind::TransitionSource, facts.transition.clone()),
        (
            ReferenceKind::InitialStateSource,
            facts.initial_state.clone(),
        ),
        (ReferenceKind::ReferenceSource, facts.reference.clone()),
        (ReferenceKind::DependencySource, facts.dependency.clone()),
        (ReferenceKind::DerivationSource, facts.derivation.clone()),
    ]
}

fn node_id_order(left: &NodeId, right: &NodeId) -> std::cmp::Ordering {
    left.uri
        .as_str()
        .cmp(right.uri.as_str())
        .then_with(|| left.qualified_name.cmp(&right.qualified_name))
}

fn provenance_order(provenance: ResolutionProvenance) -> u8 {
    match provenance {
        ResolutionProvenance::Authored => 0,
        ResolutionProvenance::Implied(_) => 1,
        ResolutionProvenance::Derived(_) => 2,
    }
}

fn expression_range_order(
    expression: Option<&DeclaredExpressionRelationship>,
) -> Option<(u32, u32, u32, u32)> {
    expression.map(|expression| {
        (
            expression.source_range.start.line,
            expression.source_range.start.character,
            expression.source_range.end.line,
            expression.source_range.end.character,
        )
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticModelPhase {
    Resolved,
    Evaluated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticModelCompleteness {
    Complete,
    EditorRecovery,
}

#[derive(Debug, Clone)]
pub(crate) struct EvaluationState {
    facts: HashMap<NodeId, NodeEvaluationFacts>,
}

/// Read-only semantic publication.
#[derive(Debug, Clone)]
pub struct SemanticModel {
    identity: SemanticModelIdentity,
    structural_graph: SemanticGraph,
    resolution: ResolutionState,
    evaluation: Option<EvaluationState>,
    phase: SemanticModelPhase,
    completeness: SemanticModelCompleteness,
    indexes: SemanticQueryIndexes,
    navigation_index: BTreeMap<url::Url, NavigationIntervalIndex>,
}

#[derive(Debug, Clone)]
struct NavigationIntervalNode {
    range: TextRange,
    fact_index: usize,
    max_end: TextPosition,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug, Clone, Default)]
struct NavigationIntervalIndex {
    nodes: Vec<NavigationIntervalNode>,
    root: Option<usize>,
}

impl NavigationIntervalIndex {
    fn from_entries(mut entries: Vec<(TextRange, usize)>) -> Self {
        entries.sort_by(|(left_range, left_index), (right_range, right_index)| {
            navigation_interval_order(left_range)
                .cmp(&navigation_interval_order(right_range))
                .then(left_index.cmp(right_index))
        });
        let mut nodes = Vec::with_capacity(entries.len());
        let root = Self::build_tree(&entries, &mut nodes);
        Self { nodes, root }
    }

    fn build_tree(
        entries: &[(TextRange, usize)],
        nodes: &mut Vec<NavigationIntervalNode>,
    ) -> Option<usize> {
        if entries.is_empty() {
            return None;
        }
        let middle = entries.len() / 2;
        let node_index = nodes.len();
        let (range, fact_index) = entries[middle];
        nodes.push(NavigationIntervalNode {
            range,
            fact_index,
            max_end: range.end,
            left: None,
            right: None,
        });
        let left = Self::build_tree(&entries[..middle], nodes);
        let right = Self::build_tree(&entries[middle + 1..], nodes);
        let mut max_end = range.end;
        if let Some(left) = left {
            max_end = max_position(max_end, nodes[left].max_end);
        }
        if let Some(right) = right {
            max_end = max_position(max_end, nodes[right].max_end);
        }
        nodes[node_index] = NavigationIntervalNode {
            range,
            fact_index,
            max_end,
            left,
            right,
        };
        Some(node_index)
    }

    fn matching_fact_indices(&self, position: TextPosition) -> Vec<usize> {
        let mut matches = Vec::new();
        self.visit(self.root, position, &mut matches);
        matches
    }

    fn visit(&self, node_index: Option<usize>, position: TextPosition, matches: &mut Vec<usize>) {
        let Some(node_index) = node_index else {
            return;
        };
        let node = &self.nodes[node_index];
        if node
            .left
            .is_some_and(|left| position_at_or_before(position, self.nodes[left].max_end))
        {
            self.visit(node.left, position, matches);
        }
        if position_at_or_after(position, node.range.start)
            && position_at_or_before(position, node.range.end)
        {
            matches.push(node.fact_index);
        }
        if position_at_or_after(position, node.range.start)
            && node
                .right
                .is_some_and(|right| position_at_or_before(position, self.nodes[right].max_end))
        {
            self.visit(node.right, position, matches);
        }
    }
}

fn position_key(position: TextPosition) -> (u32, u32) {
    (position.line, position.character)
}

fn position_at_or_before(left: TextPosition, right: TextPosition) -> bool {
    position_key(left) <= position_key(right)
}

fn position_at_or_after(left: TextPosition, right: TextPosition) -> bool {
    position_key(left) >= position_key(right)
}

fn max_position(left: TextPosition, right: TextPosition) -> TextPosition {
    if position_at_or_after(left, right) {
        left
    } else {
        right
    }
}

fn navigation_range_order(range: &TextRange) -> (u32, u32, u32, u32, u32, u32) {
    let line_span = range.end.line.saturating_sub(range.start.line);
    let character_span = if line_span == 0 {
        range.end.character.saturating_sub(range.start.character)
    } else {
        range.end.character
    };
    (
        line_span,
        character_span,
        range.start.line,
        range.start.character,
        range.end.line,
        range.end.character,
    )
}

fn navigation_interval_order(range: &TextRange) -> (u32, u32, u32, u32) {
    (
        range.start.line,
        range.start.character,
        range.end.line,
        range.end.character,
    )
}

/// A source reference outcome owned by the resolution phase for diagnostics.
///
/// This is intentionally not a clone of [`ResolutionFact`]: diagnostic consumers receive the
/// source range and classification they need, but cannot inspect resolver storage or rebuild a
/// graph from a collection of general-purpose facts.
#[derive(Debug, Clone)]
pub struct ResolutionDiagnosticReference {
    pub source: NodeId,
    pub source_kind: ElementKind,
    pub source_range: TextRange,
    pub authored_target: String,
    pub authored_range: Option<TextRange>,
    pub kind: ReferenceKind,
    pub authored_ordinal: u32,
    pub outcome: ResolutionOutcome,
    pub provenance: ResolutionProvenance,
    pub candidates: Vec<ResolutionDiagnosticCandidate>,
    /// Authored import semantics projected by the resolver. `None` for non-import references.
    pub import: Option<ResolutionImportFact>,
}

/// A candidate attached to an ambiguous authored reference. The resolver owns both the
/// canonical candidate order and the source location needed by diagnostics; consumers do not
/// rediscover candidates from names or graph indexes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionDiagnosticCandidate {
    pub target: NodeId,
    pub kind: ElementKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct ResolutionDiagnosticInput {
    references: Vec<ResolutionDiagnosticReference>,
}

impl ResolutionDiagnosticInput {
    pub fn references(&self) -> &[ResolutionDiagnosticReference] {
        &self.references
    }
}

/// The structural facts needed by inherited-feature diagnostics. The resolver computes the
/// inherited comparison before publication; diagnostics therefore do not walk nodes, indexes,
/// or relationship adjacency themselves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralDiagnosticFact {
    pub feature: NodeId,
    pub inherited_feature: NodeId,
    pub feature_name: String,
    pub feature_kind: ElementKind,
    pub range: TextRange,
    pub inherited_range: TextRange,
    pub inherited_feature_name: String,
    pub inherited_feature_kind: ElementKind,
    pub inherited_type: Option<String>,
    pub inherited_is_enum: bool,
    pub authored_value: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StructuralDiagnosticInput {
    facts: Vec<StructuralDiagnosticFact>,
}

impl StructuralDiagnosticInput {
    pub fn inherited_features(&self) -> &[StructuralDiagnosticFact] {
        &self.facts
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionDiagnosticRelationship {
    pub source: NodeId,
    pub target: NodeId,
    pub source_kind: ElementKind,
    pub target_kind: ElementKind,
    pub range: TextRange,
    pub source_expression: Option<String>,
    pub target_expression: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionDiagnosticInput {
    relationships: Vec<ConnectionDiagnosticRelationship>,
}

impl ConnectionDiagnosticInput {
    pub fn relationships(&self) -> &[ConnectionDiagnosticRelationship] {
        &self.relationships
    }
}

#[derive(Debug, Clone)]
pub struct BehaviorDiagnosticRelationship {
    pub source: NodeId,
    pub target: NodeId,
    pub source_kind: ElementKind,
    pub target_kind: ElementKind,
    pub kind: RelationshipKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct BehaviorDiagnosticInput {
    relationships: Vec<BehaviorDiagnosticRelationship>,
}

impl BehaviorDiagnosticInput {
    pub fn relationships(&self) -> &[BehaviorDiagnosticRelationship] {
        &self.relationships
    }
}

#[derive(Debug, Clone)]
pub struct RequirementCaseDiagnosticRelationship {
    pub source: NodeId,
    pub target: NodeId,
    pub source_kind: ElementKind,
    pub target_kind: ElementKind,
    pub kind: RelationshipKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct RequirementCaseDiagnosticInput {
    relationships: Vec<RequirementCaseDiagnosticRelationship>,
}

impl RequirementCaseDiagnosticInput {
    pub fn relationships(&self) -> &[RequirementCaseDiagnosticRelationship] {
        &self.relationships
    }
}

#[derive(Debug, Clone)]
pub struct ViewDiagnosticRelationship {
    pub source: NodeId,
    pub target: NodeId,
    pub source_kind: ElementKind,
    pub target_kind: ElementKind,
    pub kind: RelationshipKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, Default)]
pub struct ViewDiagnosticInput {
    relationships: Vec<ViewDiagnosticRelationship>,
}

impl ViewDiagnosticInput {
    pub fn relationships(&self) -> &[ViewDiagnosticRelationship] {
        &self.relationships
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionDiagnosticFact {
    pub owner: NodeId,
    pub range: TextRange,
    pub status: Option<EvaluationStatus>,
    pub value: Option<EvaluatedValue>,
    pub unit: Option<String>,
    pub error: Option<String>,
    pub analysis_passed: Option<bool>,
    pub analysis_status: Option<EvaluationStatus>,
    pub analysis_error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExpressionDiagnosticInput {
    facts: Vec<ExpressionDiagnosticFact>,
}

impl ExpressionDiagnosticInput {
    pub fn facts(&self) -> &[ExpressionDiagnosticFact] {
        &self.facts
    }
}

/// Unit diagnostics consume the evaluator's typed unit result, rather than parsing expression
/// text or inspecting attributes.
#[derive(Debug, Clone)]
pub struct UnitDiagnosticFact {
    pub owner: NodeId,
    pub range: TextRange,
    pub status: Option<EvaluationStatus>,
    pub unit: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UnitDiagnosticInput {
    facts: Vec<UnitDiagnosticFact>,
}

impl UnitDiagnosticInput {
    pub fn facts(&self) -> &[UnitDiagnosticFact] {
        &self.facts
    }
}

/// Builder/endpoint diagnostics consume only endpoint reference outcomes. This deliberately
/// shares the resolution owner's typed record shape through a category-specific view.
#[derive(Debug, Clone, Default)]
pub struct BuilderDiagnosticInput {
    references: Vec<BuilderDiagnosticReference>,
}

#[derive(Debug, Clone)]
pub struct BuilderDiagnosticReference {
    pub source: NodeId,
    pub source_range: TextRange,
    pub authored_target: String,
    pub authored_range: Option<TextRange>,
    pub kind: ReferenceKind,
    pub authored_ordinal: u32,
    pub outcome: ResolutionOutcome,
}

impl BuilderDiagnosticInput {
    pub fn references(&self) -> &[BuilderDiagnosticReference] {
        &self.references
    }
}

/// A complete source-fidelity navigation result owned by the settled semantic model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationReference {
    pub source: NodeId,
    pub range: TextRange,
    pub kind: ReferenceKind,
    pub authored_ordinal: u32,
    pub authored_target: String,
    pub outcome: NavigationOutcome,
}

/// Exhaustive navigation outcome. Unsupported and unresolved references are preserved rather
/// than being represented as a missing target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationOutcome {
    Resolved(NavigationTarget),
    Unresolved,
    Ambiguous(Vec<NavigationTarget>),
    UnsupportedFiltered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTarget {
    pub id: NodeId,
    pub range: TextRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationQueryError {
    MissingTarget(NodeId),
    MissingAuthoredRange(NodeId),
}

impl fmt::Display for NavigationQueryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTarget(id) => write!(
                formatter,
                "resolved navigation target is not present in the published model: {}::{}",
                id.uri, id.qualified_name
            ),
            Self::MissingAuthoredRange(id) => write!(
                formatter,
                "navigation reference has no authored range: {}::{}",
                id.uri, id.qualified_name
            ),
        }
    }
}

fn quote_navigation(value: &str) -> String {
    serde_json::to_string(value).expect("navigation strings are serializable")
}

fn render_navigation_range(range: TextRange) -> String {
    format!(
        "(start {} {}) (end {} {})",
        range.start.line, range.start.character, range.end.line, range.end.character
    )
}

fn write_navigation_reference(
    output: &mut dyn fmt::Write,
    reference: &NavigationReference,
    document_ids: &BTreeMap<url::Url, String>,
) -> fmt::Result {
    writeln!(output, "      (reference")?;
    writeln!(
        output,
        "        (source (document {}) (qualified-name {}))",
        quote_navigation(
            document_ids
                .get(&reference.source.uri)
                .expect("source document identity")
        ),
        quote_navigation(&reference.source.qualified_name)
    )?;
    writeln!(
        output,
        "        (kind {}) (ordinal {}) (authored-target {})",
        reference_kind_name(reference.kind),
        reference.authored_ordinal,
        quote_navigation(&reference.authored_target)
    )?;
    writeln!(
        output,
        "        (range {})",
        render_navigation_range(reference.range)
    )?;
    match &reference.outcome {
        NavigationOutcome::Resolved(target) => {
            writeln!(output, "        (outcome (status resolved)")?;
            write_navigation_target(output, "          ", target, document_ids)?;
            writeln!(output, "        )")?;
        }
        NavigationOutcome::Unresolved => {
            writeln!(output, "        (outcome (status unresolved))")?;
        }
        NavigationOutcome::UnsupportedFiltered => {
            writeln!(output, "        (outcome (status unsupported-filtered))")?;
        }
        NavigationOutcome::Ambiguous(targets) => {
            writeln!(output, "        (outcome (status ambiguous)")?;
            for target in targets {
                write_navigation_target(output, "          ", target, document_ids)?;
            }
            writeln!(output, "        )")?;
        }
    }
    writeln!(output, "      )")
}

fn write_navigation_target(
    output: &mut dyn fmt::Write,
    indent: &str,
    target: &NavigationTarget,
    document_ids: &BTreeMap<url::Url, String>,
) -> fmt::Result {
    writeln!(
        output,
        "{indent}(target (document {}) (qualified-name {}) (range {}))",
        quote_navigation(
            document_ids
                .get(&target.id.uri)
                .expect("target document identity")
        ),
        quote_navigation(&target.id.qualified_name),
        render_navigation_range(target.range)
    )
}

fn reference_kind_name(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::FeatureTyping => "featureTyping",
        ReferenceKind::Specialization => "specialization",
        ReferenceKind::Subsetting => "subsetting",
        ReferenceKind::Redefinition => "redefinition",
        ReferenceKind::ReferenceSubsetting => "referenceSubsetting",
        ReferenceKind::CrossSubsetting => "crossSubsetting",
        ReferenceKind::ConnectionSource => "connectionSource",
        ReferenceKind::ConnectionTarget => "connectionTarget",
        ReferenceKind::BindSource => "bindSource",
        ReferenceKind::BindTarget => "bindTarget",
        ReferenceKind::SatisfySource => "satisfySource",
        ReferenceKind::SatisfyTarget => "satisfyTarget",
        ReferenceKind::AllocateSource => "allocateSource",
        ReferenceKind::AllocateTarget => "allocateTarget",
        ReferenceKind::FlowSource => "flowSource",
        ReferenceKind::FlowTarget => "flowTarget",
        ReferenceKind::SuccessionFlowSource => "successionFlowSource",
        ReferenceKind::SuccessionFlowTarget => "successionFlowTarget",
        ReferenceKind::PerformSource => "performSource",
        ReferenceKind::PerformTarget => "performTarget",
        ReferenceKind::TransitionSource => "transitionSource",
        ReferenceKind::TransitionTarget => "transitionTarget",
        ReferenceKind::InitialStateSource => "initialStateSource",
        ReferenceKind::InitialStateTarget => "initialStateTarget",
        ReferenceKind::ReferenceSource => "referenceSource",
        ReferenceKind::ReferenceTarget => "referenceTarget",
        ReferenceKind::DependencySource => "dependencySource",
        ReferenceKind::DependencyTarget => "dependencyTarget",
        ReferenceKind::DerivationSource => "derivationSource",
        ReferenceKind::DerivationTarget => "derivationTarget",
        ReferenceKind::NamespaceImport => "namespaceImport",
        ReferenceKind::MembershipImport => "membershipImport",
    }
}

impl SemanticModel {
    /// Streams canonical navigation probes and their owner-computed query results.
    pub fn write_navigation_debug_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        let document_ids = self.navigation_document_ids();
        let mut probes = self
            .resolution
            .facts
            .iter()
            .filter_map(|fact| {
                fact.authored_range
                    .map(|range| (fact.reference.source.uri.clone(), range))
            })
            .collect::<Vec<_>>();
        probes.sort_by(|(left_uri, left_range), (right_uri, right_range)| {
            left_uri
                .cmp(right_uri)
                .then(navigation_range_order(left_range).cmp(&navigation_range_order(right_range)))
        });
        probes.dedup();

        writeln!(output, "(navigation")?;
        let mut current_uri = None;
        for (uri, range) in probes {
            if current_uri.as_ref() != Some(&uri) {
                if current_uri.is_some() {
                    writeln!(output, "  )")?;
                }
                writeln!(
                    output,
                    "  (document {}",
                    quote_navigation(document_ids.get(&uri).expect("probe document identity"))
                )?;
                current_uri = Some(uri.clone());
            }
            let matches = self
                .view()
                .navigation_references_at_position(&uri, range.start)
                .map_err(|_| fmt::Error)?;
            writeln!(
                output,
                "    (query (range {}) (probe (position {} {}))",
                render_navigation_range(range),
                range.start.line,
                range.start.character
            )?;
            for reference in matches {
                write_navigation_reference(output, &reference, &document_ids)?;
            }
            writeln!(output, "    )")?;
        }
        if current_uri.is_some() {
            writeln!(output, "  )")?;
        }
        write!(output, ")")
    }

    fn navigation_document_ids(&self) -> BTreeMap<url::Url, String> {
        let mut uris = BTreeMap::new();
        for fact in &self.resolution.facts {
            uris.insert(fact.reference.source.uri.clone(), ());
            match &fact.outcome {
                ResolutionOutcome::Resolved { target } => {
                    uris.insert(target.uri.clone(), ());
                }
                ResolutionOutcome::Ambiguous { candidates } => {
                    for candidate in candidates {
                        uris.insert(candidate.uri.clone(), ());
                    }
                }
                ResolutionOutcome::Unresolved | ResolutionOutcome::UnsupportedFiltered => {}
            }
        }
        uris.into_keys()
            .enumerate()
            .map(|(index, uri)| (uri, format!("d{index}")))
            .collect()
    }

    pub fn identity(&self) -> &SemanticModelIdentity {
        &self.identity
    }

    pub fn phase(&self) -> SemanticModelPhase {
        self.phase
    }

    pub fn completeness(&self) -> SemanticModelCompleteness {
        self.completeness
    }

    pub(crate) fn resolution(&self) -> &ResolutionState {
        &self.resolution
    }

    pub(crate) fn structural_nodes_for_debug(&self) -> Vec<&SemanticNode> {
        self.structural_graph.semantic_node_refs()
    }

    pub fn has_evaluation(&self) -> bool {
        self.evaluation.is_some()
    }

    pub(crate) fn evaluation_facts(&self) -> Option<&HashMap<NodeId, NodeEvaluationFacts>> {
        self.evaluation.as_ref().map(|state| &state.facts)
    }

    pub(crate) fn view(&self) -> ResolutionView<'_> {
        ResolutionView { model: self }
    }

    /// Source-position navigation is an implementation seam for `sysml_query`. It returns a
    /// cohesive typed answer and cannot expose graph nodes, resolver facts, or index storage.
    pub fn navigation_references_at_position(
        &self,
        uri: &url::Url,
        position: TextPosition,
    ) -> Result<Vec<NavigationReference>, NavigationQueryError> {
        self.view().navigation_references_at_position(uri, position)
    }

    /// Canonical outcome for one authored reference identity.
    pub fn authored_reference_outcome(
        &self,
        reference: &AuthoredReferenceId,
    ) -> Option<&ResolutionOutcome> {
        self.view().outcome(reference)
    }

    /// Resolved targets from the eager outgoing adjacency index.
    pub fn resolved_outgoing(&self, source: &NodeId, kind: RelationshipKind) -> &[NodeId] {
        self.view().outgoing(source, kind)
    }

    /// Resolved sources from the eager incoming adjacency index.
    pub fn resolved_incoming(&self, target: &NodeId, kind: RelationshipKind) -> &[NodeId] {
        self.view().incoming(target, kind)
    }

    /// Returns resolution-owned reference outcomes for diagnostics.
    pub fn resolution_diagnostics(&self) -> ResolutionDiagnosticInput {
        let mut references: Vec<ResolutionDiagnosticReference> = self
            .resolution
            .facts
            .iter()
            .filter_map(|fact| {
                let source = self.structural_graph.get_node(&fact.reference.source)?;
                Some(ResolutionDiagnosticReference {
                    source: fact.reference.source.clone(),
                    source_kind: source.element_kind.clone(),
                    source_range: source.range,
                    authored_target: fact.authored_target.clone(),
                    authored_range: fact.authored_range,
                    kind: fact.reference.kind,
                    authored_ordinal: fact.reference.authored_ordinal,
                    outcome: fact.outcome.clone(),
                    provenance: ResolutionProvenance::Authored,
                    candidates: diagnostic_candidates(&self.structural_graph, &fact.outcome),
                    import: fact.import.clone(),
                })
            })
            .collect();
        references.sort_by(|left, right| {
            left.source
                .cmp(&right.source)
                .then(left.kind.cmp(&right.kind))
                .then(left.authored_ordinal.cmp(&right.authored_ordinal))
                .then(
                    range_order(left.authored_range.or(Some(left.source_range))).cmp(&range_order(
                        right.authored_range.or(Some(right.source_range)),
                    )),
                )
        });
        ResolutionDiagnosticInput { references }
    }

    /// Returns structural inherited-value comparisons computed at the publication barrier.
    pub fn structural_diagnostics(&self) -> StructuralDiagnosticInput {
        let facts = self.resolution.inherited_features().to_vec();
        StructuralDiagnosticInput { facts }
    }

    fn relationship_range(&self, relationship: &ResolvedRelationship) -> TextRange {
        relationship
            .expression
            .as_ref()
            .map(|expression| expression.source_range)
            .or_else(|| {
                self.structural_graph
                    .get_node(&relationship.source)
                    .map(|node| node.range)
            })
            .unwrap_or(TextRange {
                start: TextPosition::new(0, 0),
                end: TextPosition::new(0, 0),
            })
    }

    /// Returns only connection and binding endpoint facts, including the resolved endpoint kinds.
    pub fn connection_diagnostics(&self) -> ConnectionDiagnosticInput {
        let mut relationships: Vec<ConnectionDiagnosticRelationship> = self
            .resolution
            .relationships
            .iter()
            .filter(|relationship| {
                matches!(
                    relationship.kind,
                    RelationshipKind::Connection | RelationshipKind::Bind
                )
            })
            .filter_map(|relationship| {
                let source = self.structural_graph.get_node(&relationship.source)?;
                let target = self.structural_graph.get_node(&relationship.target)?;
                let (source_expression, target_expression) = relationship
                    .expression
                    .as_ref()
                    .map(|expression| {
                        (
                            Some(expression.source_expression.clone()),
                            Some(expression.target_expression.clone()),
                        )
                    })
                    .unwrap_or((None, None));
                Some(ConnectionDiagnosticRelationship {
                    source: relationship.source.clone(),
                    target: relationship.target.clone(),
                    source_kind: source.element_kind.clone(),
                    target_kind: target.element_kind.clone(),
                    range: self.relationship_range(relationship),
                    source_expression,
                    target_expression,
                })
            })
            .collect();
        relationships.sort_by(|left, right| {
            left.source
                .cmp(&right.source)
                .then(left.target.cmp(&right.target))
                .then(range_order(Some(left.range)).cmp(&range_order(Some(right.range))))
        });
        ConnectionDiagnosticInput { relationships }
    }

    pub fn behavior_diagnostics(&self) -> BehaviorDiagnosticInput {
        let mut relationships: Vec<BehaviorDiagnosticRelationship> = self
            .resolution
            .relationships
            .iter()
            .filter(|relationship| {
                matches!(
                    relationship.kind,
                    RelationshipKind::Flow
                        | RelationshipKind::SuccessionFlow
                        | RelationshipKind::Perform
                        | RelationshipKind::Transition
                        | RelationshipKind::InitialState
                )
            })
            .filter_map(|relationship| {
                let source = self.structural_graph.get_node(&relationship.source)?;
                let target = self.structural_graph.get_node(&relationship.target)?;
                Some(BehaviorDiagnosticRelationship {
                    source: relationship.source.clone(),
                    target: relationship.target.clone(),
                    source_kind: source.element_kind.clone(),
                    target_kind: target.element_kind.clone(),
                    kind: relationship.kind.clone(),
                    range: self.relationship_range(relationship),
                })
            })
            .collect();
        relationships.sort_by(|left, right| {
            left.source
                .cmp(&right.source)
                .then(left.kind.cmp(&right.kind))
                .then(left.target.cmp(&right.target))
        });
        BehaviorDiagnosticInput { relationships }
    }

    pub fn requirement_case_diagnostics(&self) -> RequirementCaseDiagnosticInput {
        let mut relationships: Vec<RequirementCaseDiagnosticRelationship> = self
            .resolution
            .relationships
            .iter()
            .filter(|relationship| {
                matches!(
                    relationship.kind,
                    RelationshipKind::Satisfy
                        | RelationshipKind::Subject
                        | RelationshipKind::Derivation
                        | RelationshipKind::Dependency
                )
            })
            .filter_map(|relationship| {
                let source = self.structural_graph.get_node(&relationship.source)?;
                let target = self.structural_graph.get_node(&relationship.target)?;
                Some(RequirementCaseDiagnosticRelationship {
                    source: relationship.source.clone(),
                    target: relationship.target.clone(),
                    source_kind: source.element_kind.clone(),
                    target_kind: target.element_kind.clone(),
                    kind: relationship.kind.clone(),
                    range: self.relationship_range(relationship),
                })
            })
            .collect();
        relationships.sort_by(|left, right| {
            left.source
                .cmp(&right.source)
                .then(left.kind.cmp(&right.kind))
                .then(left.target.cmp(&right.target))
        });
        RequirementCaseDiagnosticInput { relationships }
    }

    pub fn view_diagnostics(&self) -> ViewDiagnosticInput {
        let mut relationships: Vec<ViewDiagnosticRelationship> = self
            .resolution
            .relationships
            .iter()
            .filter(|relationship| {
                matches!(
                    relationship.kind,
                    RelationshipKind::Annotation | RelationshipKind::Satisfy
                )
            })
            .filter_map(|relationship| {
                let source = self.structural_graph.get_node(&relationship.source)?;
                let target = self.structural_graph.get_node(&relationship.target)?;
                Some(ViewDiagnosticRelationship {
                    source: relationship.source.clone(),
                    target: relationship.target.clone(),
                    source_kind: source.element_kind.clone(),
                    target_kind: target.element_kind.clone(),
                    kind: relationship.kind.clone(),
                    range: self.relationship_range(relationship),
                })
            })
            .collect();
        relationships.sort_by(|left, right| {
            left.source
                .cmp(&right.source)
                .then(left.kind.cmp(&right.kind))
                .then(left.target.cmp(&right.target))
        });
        ViewDiagnosticInput { relationships }
    }

    pub fn expression_diagnostics(&self) -> ExpressionDiagnosticInput {
        let mut facts: Vec<ExpressionDiagnosticFact> = self
            .evaluation
            .as_ref()
            .into_iter()
            .flat_map(|state| state.facts.iter())
            .filter_map(|(owner, facts)| {
                let node = self.structural_graph.get_node(owner)?;
                Some(ExpressionDiagnosticFact {
                    owner: owner.clone(),
                    range: node.range,
                    status: facts
                        .expression
                        .as_ref()
                        .map(|expression| expression.status),
                    value: facts
                        .expression
                        .as_ref()
                        .and_then(|expression| expression.value.clone()),
                    unit: facts
                        .expression
                        .as_ref()
                        .and_then(|expression| expression.unit.clone()),
                    error: facts
                        .expression
                        .as_ref()
                        .and_then(|expression| expression.error.clone()),
                    analysis_passed: facts.analysis.as_ref().and_then(|analysis| analysis.passed),
                    analysis_status: facts
                        .analysis
                        .as_ref()
                        .map(|analysis| analysis.expression.status),
                    analysis_error: facts
                        .analysis
                        .as_ref()
                        .and_then(|analysis| analysis.expression.error.clone()),
                })
            })
            .collect();
        facts.sort_by(|left, right| left.owner.cmp(&right.owner));
        ExpressionDiagnosticInput { facts }
    }

    pub fn unit_diagnostics(&self) -> UnitDiagnosticInput {
        let facts = self
            .expression_diagnostics()
            .facts
            .into_iter()
            .map(|fact| UnitDiagnosticFact {
                owner: fact.owner,
                range: fact.range,
                status: fact.status,
                unit: fact.unit,
                error: fact.error,
            })
            .collect();
        UnitDiagnosticInput { facts }
    }

    pub fn builder_diagnostics(&self) -> BuilderDiagnosticInput {
        let references = self
            .resolution_diagnostics()
            .references
            .into_iter()
            .filter(|reference| {
                matches!(
                    reference.kind,
                    ReferenceKind::ConnectionSource
                        | ReferenceKind::ConnectionTarget
                        | ReferenceKind::BindSource
                        | ReferenceKind::BindTarget
                        | ReferenceKind::SatisfySource
                        | ReferenceKind::SatisfyTarget
                        | ReferenceKind::AllocateSource
                        | ReferenceKind::AllocateTarget
                        | ReferenceKind::DerivationSource
                        | ReferenceKind::DerivationTarget
                )
            })
            .map(|reference| BuilderDiagnosticReference {
                source: reference.source,
                source_range: reference.source_range,
                authored_target: reference.authored_target,
                authored_range: reference.authored_range,
                kind: reference.kind,
                authored_ordinal: reference.authored_ordinal,
                outcome: reference.outcome,
            })
            .collect();
        BuilderDiagnosticInput { references }
    }
}

fn diagnostic_candidates(
    graph: &SemanticGraph,
    outcome: &ResolutionOutcome,
) -> Vec<ResolutionDiagnosticCandidate> {
    let ResolutionOutcome::Ambiguous { candidates } = outcome else {
        return Vec::new();
    };
    let mut candidates = candidates
        .iter()
        .filter_map(|target| {
            graph
                .get_node(target)
                .map(|node| ResolutionDiagnosticCandidate {
                    target: target.clone(),
                    kind: node.element_kind.clone(),
                    range: node.range,
                })
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.target
            .cmp(&right.target)
            .then(range_order(Some(left.range)).cmp(&range_order(Some(right.range))))
    });
    candidates.dedup_by(|left, right| left.target == right.target);
    candidates
}

fn import_conformance(
    import: &crate::semantic::model::DeclaredImportFacts,
    outcome: &ResolutionOutcome,
    resolved_kind: Option<&ElementKind>,
) -> ImportConformanceOutcome {
    if import.target.presence == ImportTargetPresence::Missing {
        return ImportConformanceOutcome::MissingTarget;
    }
    let target_kind = match outcome {
        ResolutionOutcome::Resolved { .. } => resolved_kind,
        ResolutionOutcome::Unresolved => return ImportConformanceOutcome::NotCheckedUnresolved,
        ResolutionOutcome::Ambiguous { .. } => {
            return ImportConformanceOutcome::NotCheckedAmbiguous
        }
        ResolutionOutcome::UnsupportedFiltered => {
            return ImportConformanceOutcome::NotCheckedUnsupportedFiltered
        }
    };
    let Some(target_kind) = target_kind else {
        return ImportConformanceOutcome::NotCheckedUnresolved;
    };
    if import.shape == ImportShape::Namespace && !crate::semantic::kinds::is_namespace(target_kind)
    {
        return ImportConformanceOutcome::NamespaceKindMismatch {
            actual: target_kind.clone(),
        };
    }
    if import.recursive && !crate::semantic::kinds::is_namespace(target_kind) {
        return ImportConformanceOutcome::RecursiveNonNamespace {
            actual: target_kind.clone(),
        };
    }
    ImportConformanceOutcome::Valid
}

fn range_order(range: Option<TextRange>) -> (u32, u32, u32, u32) {
    let range = range.unwrap_or(TextRange {
        start: TextPosition::new(u32::MAX, u32::MAX),
        end: TextPosition::new(u32::MAX, u32::MAX),
    });
    (
        range.start.line,
        range.start.character,
        range.end.line,
        range.end.character,
    )
}

#[derive(Debug, Clone, Default)]
struct SemanticQueryIndexes {
    outgoing: BTreeMap<(NodeId, RelationshipKind), Vec<NodeId>>,
    incoming: BTreeMap<(NodeId, RelationshipKind), Vec<NodeId>>,
}

impl SemanticQueryIndexes {
    fn from_state(state: &ResolutionState) -> Self {
        let mut indexes = Self::default();
        for relationship in &state.relationships {
            indexes
                .outgoing
                .entry((relationship.source.clone(), relationship.kind.clone()))
                .or_default()
                .push(relationship.target.clone());
            indexes
                .incoming
                .entry((relationship.target.clone(), relationship.kind.clone()))
                .or_default()
                .push(relationship.source.clone());
        }
        indexes
    }
}

/// Read-only query surface over a settled semantic model.
pub(crate) struct ResolutionView<'a> {
    model: &'a SemanticModel,
}

impl<'a> ResolutionView<'a> {
    /// Returns all authored references containing a source position in canonical narrowest-first
    /// order. The interval index is immutable after publication and returns exhaustive typed
    /// outcomes without exposing resolver facts or graph storage.
    pub(crate) fn navigation_references_at_position(
        &self,
        uri: &url::Url,
        position: TextPosition,
    ) -> Result<Vec<NavigationReference>, NavigationQueryError> {
        let Some(index) = self.model.navigation_index.get(uri) else {
            return Ok(Vec::new());
        };
        let mut references = index
            .matching_fact_indices(position)
            .into_iter()
            .map(|fact| {
                let fact = &self.model.resolution.facts[fact];
                Ok(NavigationReference {
                    source: fact.reference.source.clone(),
                    range: fact.authored_range.ok_or_else(|| {
                        NavigationQueryError::MissingAuthoredRange(fact.reference.source.clone())
                    })?,
                    kind: fact.reference.kind,
                    authored_ordinal: fact.reference.authored_ordinal,
                    authored_target: fact.authored_target.clone(),
                    outcome: self.navigation_outcome(&fact.outcome)?,
                })
            })
            .collect::<Result<Vec<_>, NavigationQueryError>>()?;
        references.sort_by(|left, right| {
            navigation_range_order(&left.range)
                .cmp(&navigation_range_order(&right.range))
                .then(left.source.cmp(&right.source))
                .then(left.kind.cmp(&right.kind))
                .then(left.authored_ordinal.cmp(&right.authored_ordinal))
        });
        Ok(references)
    }

    fn navigation_outcome(
        &self,
        outcome: &ResolutionOutcome,
    ) -> Result<NavigationOutcome, NavigationQueryError> {
        match outcome {
            ResolutionOutcome::Resolved { target } => {
                Ok(NavigationOutcome::Resolved(self.navigation_target(target)?))
            }
            ResolutionOutcome::Unresolved => Ok(NavigationOutcome::Unresolved),
            ResolutionOutcome::UnsupportedFiltered => Ok(NavigationOutcome::UnsupportedFiltered),
            ResolutionOutcome::Ambiguous { candidates } => Ok(NavigationOutcome::Ambiguous(
                candidates
                    .iter()
                    .map(|candidate| self.navigation_target(candidate))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
        }
    }

    fn navigation_target(&self, id: &NodeId) -> Result<NavigationTarget, NavigationQueryError> {
        let Some(node) = self.model.structural_graph.get_node(id) else {
            return Err(NavigationQueryError::MissingTarget(id.clone()));
        };
        Ok(NavigationTarget {
            id: id.clone(),
            range: node.range,
        })
    }

    pub(crate) fn outcome(&self, reference: &AuthoredReferenceId) -> Option<&'a ResolutionOutcome> {
        self.model.resolution.outcome(reference)
    }

    pub(crate) fn outgoing(&self, source: &NodeId, kind: RelationshipKind) -> &'a [NodeId] {
        self.model
            .indexes
            .outgoing
            .get(&(source.clone(), kind))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(crate) fn incoming(&self, target: &NodeId, kind: RelationshipKind) -> &'a [NodeId] {
        self.model
            .indexes
            .incoming
            .get(&(target.clone(), kind))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

/// Build one immutable semantic publication.  Construction strategy affects only parsing and
/// authored graph construction; both strategies enter the same resolution publication barrier.
pub fn build_semantic_model(
    request: SemanticBuildRequest,
) -> Result<SemanticModel, SemanticBuildFailure> {
    build_prepared_semantic_model(request.prepare())
}

/// Builds from the exact semantic inputs bound to a precomputed publication identity.
pub fn build_prepared_semantic_model(
    request: PreparedSemanticBuildRequest,
) -> Result<SemanticModel, SemanticBuildFailure> {
    build_prepared_semantic_model_with_max_passes(request, 1_000)
}

#[cfg(test)]
pub(crate) fn build_semantic_model_with_max_passes(
    request: SemanticBuildRequest,
    max_passes: usize,
) -> Result<SemanticModel, SemanticBuildFailure> {
    build_prepared_semantic_model_with_max_passes(request.prepare(), max_passes)
}

fn build_prepared_semantic_model_with_max_passes(
    prepared: PreparedSemanticBuildRequest,
    max_passes: usize,
) -> Result<SemanticModel, SemanticBuildFailure> {
    let PreparedSemanticBuildRequest { request, identity } = prepared;
    let documents = request.sources.documents();
    let (graph, _, completeness) = build_structural_graph(documents, request.construction);
    let resolution = ResolutionDb::new(&graph)
        .solve_with_max_passes(max_passes)
        .map_err(SemanticBuildFailure::Resolution)?;
    let structural_graph = graph;
    let evaluation = if matches!(request.evaluation, EvaluationPolicy::Evaluate) {
        // Evaluation is an explicitly later phase. It receives a private working graph with
        // resolved relationships installed; the published structural graph remains immutable
        // authored input and never becomes a second resolution authority.
        let mut evaluation_graph = structural_graph.clone();
        resolution.install_relationship_projection(&mut evaluation_graph);
        crate::semantic::analysis_typing::prepare_analysis_evaluation_context(
            &mut evaluation_graph,
        );
        crate::semantic::evaluation::evaluate_expressions(&mut evaluation_graph);
        Some(EvaluationState {
            facts: evaluation_graph.evaluation_facts_by_node_id.clone(),
        })
    } else {
        None
    };
    let indexes = SemanticQueryIndexes::from_state(&resolution);
    let navigation_index = build_navigation_indexes(&resolution);
    Ok(SemanticModel {
        identity,
        structural_graph,
        resolution,
        evaluation,
        phase: if matches!(request.evaluation, EvaluationPolicy::Evaluate) {
            SemanticModelPhase::Evaluated
        } else {
            SemanticModelPhase::Resolved
        },
        completeness,
        indexes,
        navigation_index,
    })
}

fn build_navigation_indexes(
    resolution: &ResolutionState,
) -> BTreeMap<url::Url, NavigationIntervalIndex> {
    let mut entries_by_uri = BTreeMap::<url::Url, Vec<(TextRange, usize)>>::new();
    for (fact_index, fact) in resolution.facts.iter().enumerate() {
        if let Some(range) = fact.authored_range {
            entries_by_uri
                .entry(fact.reference.source.uri.clone())
                .or_default()
                .push((range, fact_index));
        }
    }
    entries_by_uri
        .into_iter()
        .map(|(uri, entries)| (uri, NavigationIntervalIndex::from_entries(entries)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::model::{DeclaredImportFacts, DeclaredImportTarget};
    use url::Url;

    fn document(uri: &str, source: &str) -> SysmlDocument {
        SysmlDocument::from_uri(
            uri,
            source.to_string(),
            None,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("test URI")
    }

    #[test]
    fn interval_index_returns_overlaps_without_scanning_unrelated_subtrees() {
        let index = NavigationIntervalIndex::from_entries(vec![
            (
                TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 2)),
                0,
            ),
            (
                TextRange::new(TextPosition::new(0, 1), TextPosition::new(0, 8)),
                1,
            ),
            (
                TextRange::new(TextPosition::new(2, 0), TextPosition::new(2, 2)),
                2,
            ),
        ]);
        assert_eq!(
            index.matching_fact_indices(TextPosition::new(0, 1)),
            vec![0, 1]
        );
        assert!(index
            .matching_fact_indices(TextPosition::new(1, 0))
            .is_empty());
    }

    #[test]
    fn source_snapshot_rejects_duplicate_uris() {
        let result = ImmutableSourceSnapshot::new(vec![
            document("memory://test/a.sysml", "package A {}"),
            document("memory://test/a.sysml", "package A {}"),
        ]);
        assert!(matches!(result, Err(SemanticBuildFailure::InvalidInput(_))));
    }

    #[test]
    fn recovered_parse_publishes_explicit_editor_completeness() {
        let snapshot = ImmutableSourceSnapshot::new(vec![document(
            "memory://test/recovery.sysml",
            "package A { part p : ;",
        )])
        .unwrap();
        let model = build_semantic_model(SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        })
        .expect("recovery model");
        assert_eq!(
            model.completeness(),
            SemanticModelCompleteness::EditorRecovery
        );
    }

    #[test]
    fn identity_changes_when_content_changes() {
        let first =
            ImmutableSourceSnapshot::new(vec![document("memory://test/a.sysml", "package A {}")])
                .unwrap();
        let second =
            ImmutableSourceSnapshot::new(vec![document("memory://test/a.sysml", "package B {}")])
                .unwrap();
        let config = SemanticConfiguration::default();
        let first_request = SemanticBuildRequest {
            sources: first,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: config.clone(),
        };
        let second_request = SemanticBuildRequest {
            sources: second,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: config,
        };
        assert_ne!(first_request.identity(), second_request.identity());
    }

    #[test]
    fn identity_retains_phase_affecting_policies() {
        let snapshot =
            ImmutableSourceSnapshot::new(vec![document("memory://test/a.sysml", "package A {}")])
                .unwrap();
        let resolved_only = SemanticBuildRequest {
            sources: snapshot.clone(),
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        };
        let evaluated = SemanticBuildRequest {
            sources: snapshot.clone(),
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::Evaluate,
            configuration: SemanticConfiguration::default(),
        };
        let parallel = SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Parallel,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        };
        assert_ne!(resolved_only.identity(), evaluated.identity());
        assert_ne!(resolved_only.identity(), parallel.identity());
        assert_eq!(
            resolved_only.identity().source_digest,
            parallel.identity().source_digest
        );
        assert_eq!(
            resolved_only.identity().evaluation,
            EvaluationPolicy::ResolvedOnly
        );
        assert_eq!(evaluated.identity().evaluation, EvaluationPolicy::Evaluate);
    }

    #[test]
    fn model_publishes_indexed_relationships() {
        let model = build("package A { part def P {} part p : P; }");
        assert_eq!(model.phase(), SemanticModelPhase::Resolved);
        assert!(!model.has_evaluation());
        assert_eq!(model.completeness(), SemanticModelCompleteness::Complete);
        assert!(model
            .structural_graph
            .semantic_edges()
            .into_iter()
            .all(|(_, _, edge)| !matches!(
                edge.kind,
                RelationshipKind::Typing
                    | RelationshipKind::Specializes
                    | RelationshipKind::Subsetting
                    | RelationshipKind::Redefinition
                    | RelationshipKind::ReferenceSubsetting
                    | RelationshipKind::CrossSubsetting
                    | RelationshipKind::Subject
            )));
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.kind == ReferenceKind::FeatureTyping
                && matches!(fact.outcome, ResolutionOutcome::Resolved { .. })
        }));
    }

    #[test]
    fn sequential_and_parallel_publications_have_equal_resolution_facts() {
        let snapshot = ImmutableSourceSnapshot::new(vec![document(
            "memory://test/a.sysml",
            "package A { part def P {} part p : P; }
                package B { part def Q :> A::P {} }",
        )])
        .unwrap();
        let configuration = SemanticConfiguration::default();
        let sequential = build_semantic_model(SemanticBuildRequest {
            sources: snapshot.clone(),
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: configuration.clone(),
        })
        .expect("sequential semantic model");
        let parallel = build_semantic_model(SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Parallel,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration,
        })
        .expect("parallel semantic model");
        assert_eq!(sequential.resolution(), parallel.resolution());
    }

    #[test]
    fn structural_input_has_authored_facts_without_semantic_endpoint_edges() {
        let source = document(
            "memory://test/structural.sysml",
            "package M { action def A { action one; action two; first one then two; } part p; part q; connect p to q; }",
        );
        let snapshot = ImmutableSourceSnapshot::new(vec![source]).unwrap();
        let (graph, _, _) =
            build_structural_graph(snapshot.documents(), ConstructionStrategy::Sequential);
        let edges = graph.semantic_edges();
        assert!(!edges.iter().any(|(_, _, edge)| {
            matches!(
                edge.kind,
                RelationshipKind::Connection
                    | RelationshipKind::Flow
                    | RelationshipKind::SuccessionFlow
                    | RelationshipKind::Perform
                    | RelationshipKind::Transition
                    | RelationshipKind::InitialState
            )
        }));
        assert!(!graph.declared_expression_relationships.is_empty());
    }

    #[test]
    fn multi_document_resolution_is_independent_of_source_order() {
        let first = document("memory://test/a.sysml", "package A { part def T; }");
        let second = document(
            "memory://test/b.sysml",
            "package B { import A::*; part p : T; }",
        );
        let forward = ImmutableSourceSnapshot::new(vec![first.clone(), second.clone()]).unwrap();
        let reverse = ImmutableSourceSnapshot::new(vec![second, first]).unwrap();
        let configuration = SemanticConfiguration::default();
        let make = |sources| {
            build_semantic_model(SemanticBuildRequest {
                sources,
                construction: ConstructionStrategy::Parallel,
                evaluation: EvaluationPolicy::ResolvedOnly,
                configuration: configuration.clone(),
            })
            .expect("canonical multi-document model")
        };
        assert_eq!(make(forward).resolution(), make(reverse).resolution());
    }

    #[test]
    fn cyclic_public_reexports_do_not_create_a_candidate_or_hang() {
        let model = build(
            "package A { public import B::*; }
             package B { public import A::*; }
             package C { import A::*; part p : Missing; }",
        );
        let fact = model
            .resolution()
            .facts()
            .iter()
            .find(|fact| fact.reference.kind == ReferenceKind::FeatureTyping)
            .expect("typing fact");
        assert!(matches!(fact.outcome, ResolutionOutcome::Unresolved));
    }

    #[test]
    fn standard_library_relationships_keep_implied_provenance() {
        let snapshot = ImmutableSourceSnapshot::new(vec![
            SysmlDocument::from_uri(
                "memory://stdlib/parts.sysml",
                "package Parts { part def Part; }".to_string(),
                None,
                SysmlDocumentSourceKind::StandardLibrary,
                None,
                None,
            )
            .expect("stdlib URI"),
            document("memory://test/model.sysml", "package M { part def P; }"),
        ])
        .unwrap();
        let model = build_semantic_model(SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        })
        .expect("semantic model");
        assert!(model
            .resolution()
            .relationships()
            .iter()
            .any(|relationship| {
                relationship.source.qualified_name == "M::P"
                    && relationship.target.qualified_name == "Parts::Part"
                    && relationship.kind == RelationshipKind::Specializes
                    && relationship.provenance
                        == ResolutionProvenance::Implied(
                            ImpliedRelationshipRule::UniversalStandardLibraryRelationship,
                        )
            }));
    }

    #[test]
    fn queries_are_read_only_over_the_published_resolution() {
        let model = build("package M { part def P; part p : P; }");
        let fact = model
            .resolution()
            .facts()
            .iter()
            .find(|fact| fact.reference.kind == ReferenceKind::FeatureTyping)
            .expect("typing fact");
        let before = model.resolution().clone();
        let target = fact
            .outcome
            .resolved_target()
            .expect("resolved target")
            .clone();
        let source = fact.reference.source.clone();
        let view = model.view();
        assert_eq!(view.outcome(&fact.reference), Some(&fact.outcome));
        assert_eq!(view.outgoing(&source, RelationshipKind::Typing), &[target]);
        assert!(view.incoming(&source, RelationshipKind::Typing).is_empty());
        assert_eq!(model.resolution(), &before);
    }

    #[test]
    fn evaluate_policy_runs_after_resolution_before_publication() {
        let snapshot = ImmutableSourceSnapshot::new(vec![document(
            "memory://test/a.sysml",
            "package A { part def P {} part p : P; attribute x = 1; }",
        )])
        .unwrap();
        let model = build_semantic_model(SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::Evaluate,
            configuration: SemanticConfiguration::default(),
        })
        .expect("evaluated semantic model");
        assert_eq!(model.phase(), SemanticModelPhase::Evaluated);
        assert!(model.has_evaluation());
        assert!(model
            .evaluation_facts()
            .is_some_and(|facts| !facts.is_empty()));
        assert!(model
            .structural_graph
            .semantic_edges()
            .into_iter()
            .all(|(_, _, edge)| !matches!(edge.kind, RelationshipKind::Typing)));
    }

    #[test]
    fn resolution_bound_reports_failure_without_publishing() {
        let snapshot = ImmutableSourceSnapshot::new(vec![document(
            "memory://test/a.sysml",
            "package A { part p : Missing; }",
        )])
        .unwrap();
        let request = SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        };
        let failure = build_semantic_model_with_max_passes(request, 0)
            .expect_err("zero pass bound must fail explicitly");
        assert!(matches!(
            failure,
            SemanticBuildFailure::Resolution(ResolutionFailure::SafetyBound {
                passes: 0,
                pending_references: ref pending,
                ..
            }) if !pending.is_empty()
        ));
    }

    #[test]
    fn fixed_point_resolves_pending_dependencies_from_previous_pass() {
        let initial = BTreeMap::from([(
            "inner",
            WorkingOutcome::Pending {
                dependencies: vec!["outer"],
            },
        )]);
        let result = solve_fixed_point(initial, 4, |previous| {
            let mut next = previous.clone();
            next.insert("inner", WorkingOutcome::Final(7));
            next
        })
        .expect("pending outcome should settle");
        assert_eq!(result["inner"], WorkingOutcome::Final(7));
    }

    #[test]
    fn fixed_point_does_not_read_new_results_within_the_same_pass() {
        let initial = BTreeMap::from([(
            "a",
            WorkingOutcome::Pending {
                dependencies: vec![],
            },
        )]);
        let result = solve_fixed_point(initial, 4, |previous| {
            let mut next = previous.clone();
            if matches!(previous["a"], WorkingOutcome::Pending { .. }) {
                next.insert("a", WorkingOutcome::Final(1));
            }
            next
        })
        .expect("previous-pass result should settle");
        assert_eq!(result["a"], WorkingOutcome::Final(1));
    }

    #[test]
    fn fixed_point_reports_dependency_deadlock_and_safety_bound() {
        let deadlock: BTreeMap<&str, WorkingOutcome<&str, i32>> = BTreeMap::from([
            (
                "a",
                WorkingOutcome::Pending {
                    dependencies: vec!["b"],
                },
            ),
            (
                "b",
                WorkingOutcome::Pending {
                    dependencies: vec!["a"],
                },
            ),
        ]);
        let failure = solve_fixed_point(deadlock, 4, |previous| previous.clone())
            .expect_err("cyclic pending dependencies must fail");
        assert!(matches!(
            failure,
            FixedPointFailure::DependencyDeadlock { passes: 1, .. }
        ));

        let bounded = BTreeMap::from([(
            "a",
            WorkingOutcome::Pending {
                dependencies: vec![],
            },
        )]);
        let failure = solve_fixed_point(bounded, 1, |_| {
            BTreeMap::from([("a", WorkingOutcome::Final(1))])
        })
        .expect_err("a one-pass bound cannot publish an unverified result");
        assert!(matches!(
            failure,
            FixedPointFailure::SafetyBound { passes: 1, .. }
        ));
    }

    #[test]
    fn fixed_point_reports_non_adjacent_oscillation_and_is_order_independent() {
        let initial = BTreeMap::from([("a", WorkingOutcome::Final(false))]);
        let failure = solve_fixed_point(initial, 8, |previous| {
            let value = match previous["a"] {
                WorkingOutcome::Final(value) => !value,
                WorkingOutcome::Pending { .. } => false,
            };
            BTreeMap::from([("a", WorkingOutcome::Final(value))])
        })
        .expect_err("repeated state must be treated as oscillation");
        assert!(matches!(failure, FixedPointFailure::Oscillation { .. }));

        let ordered = BTreeMap::from([
            ("a".to_string(), WorkingOutcome::Final(1)),
            ("b".to_string(), WorkingOutcome::Final(2)),
        ]);
        let reversed = BTreeMap::from([
            ("b".to_string(), WorkingOutcome::Final(2)),
            ("a".to_string(), WorkingOutcome::Final(1)),
        ]);
        let step = |state: &BTreeMap<String, WorkingOutcome<String, i32>>| state.clone();
        assert_eq!(
            solve_fixed_point(ordered, 2, step),
            solve_fixed_point(reversed, 2, step)
        );
    }

    #[test]
    fn import_conformance_records_resolution_state_without_fallback() {
        let target = NodeId::new(&Url::parse("memory://test").unwrap(), "Target");
        let import = |presence, shape, recursive| DeclaredImportFacts {
            target: DeclaredImportTarget {
                reference: "Target".to_string(),
                presence,
                range: None,
            },
            origin: ImportOrigin::Import,
            shape,
            recursive,
        };

        assert_eq!(
            import_conformance(
                &import(ImportTargetPresence::Present, ImportShape::Namespace, false),
                &ResolutionOutcome::Unresolved,
                None,
            ),
            ImportConformanceOutcome::NotCheckedUnresolved
        );
        assert_eq!(
            import_conformance(
                &import(ImportTargetPresence::Present, ImportShape::Namespace, false),
                &ResolutionOutcome::Ambiguous {
                    candidates: vec![target.clone()],
                },
                None,
            ),
            ImportConformanceOutcome::NotCheckedAmbiguous
        );
        assert_eq!(
            import_conformance(
                &import(
                    ImportTargetPresence::Present,
                    ImportShape::FilteredNamespace,
                    false
                ),
                &ResolutionOutcome::UnsupportedFiltered,
                None,
            ),
            ImportConformanceOutcome::NotCheckedUnsupportedFiltered
        );
        assert_eq!(
            import_conformance(
                &import(ImportTargetPresence::Missing, ImportShape::Namespace, false),
                &ResolutionOutcome::Unresolved,
                None,
            ),
            ImportConformanceOutcome::MissingTarget
        );
        assert_eq!(
            import_conformance(
                &import(ImportTargetPresence::Present, ImportShape::Namespace, false),
                &ResolutionOutcome::Resolved {
                    target: target.clone(),
                },
                Some(&ElementKind::Package),
            ),
            ImportConformanceOutcome::Valid
        );
        assert_eq!(
            import_conformance(
                &import(ImportTargetPresence::Present, ImportShape::Namespace, false),
                &ResolutionOutcome::Resolved {
                    target: target.clone(),
                },
                Some(&ElementKind::PartDef),
            ),
            ImportConformanceOutcome::NamespaceKindMismatch {
                actual: ElementKind::PartDef,
            }
        );
        assert_eq!(
            import_conformance(
                &import(ImportTargetPresence::Present, ImportShape::Membership, true),
                &ResolutionOutcome::Resolved { target },
                Some(&ElementKind::PartDef),
            ),
            ImportConformanceOutcome::RecursiveNonNamespace {
                actual: ElementKind::PartDef,
            }
        );
    }

    fn build(source: &str) -> SemanticModel {
        let snapshot =
            ImmutableSourceSnapshot::new(vec![document("memory://test/a.sysml", source)]).unwrap();
        build_semantic_model(SemanticBuildRequest {
            sources: snapshot,
            construction: ConstructionStrategy::Sequential,
            evaluation: EvaluationPolicy::ResolvedOnly,
            configuration: SemanticConfiguration::default(),
        })
        .expect("semantic model")
    }
}

// --- The graph's own publication contract (the `SemanticPublication` contract;
// `planning/UNIFY_CACHE_PLAN.md` §4.3). ---
//
// A decoded or in-memory [`SemanticGraph`] must be able to prove, without inspecting attributes
// or guessing from shape: which exact source root produced it, whether parsing used strict
// success or editor recovery, how far construction got (parsed / structurally linked /
// settled+evaluated), and whether that construction was complete, degraded, or did not finish.
// [`SemanticPublication`] is that proof. It intentionally does not duplicate
// [`EvaluationPublicationState`](crate::semantic::model::EvaluationPublicationState):
// [`SemanticPhase::SettledEvaluated`] is only ever reached by the pipeline barrier that also
// sets `evaluation_publication` to `Complete` (see `semantic::pipeline`), and
// [`SemanticGraph::is_storage_eligible`] checks both, so the two can never disagree in practice
// -- evaluation state remains the single owner of per-expression evaluation facts, while
// `SemanticPublication` owns the graph-wide phase/completeness/identity envelope around it.
//
// Distinct from the `SemanticModel` build API's [`SemanticModelPhase`]/
// [`SemanticModelCompleteness`] above, which describe one immutable build request's outcome;
// these types are carried by every `SemanticGraph`.

/// Repository-owned semantic algorithm contract version.
///
/// Deliberately not `CARGO_PKG_VERSION` (`planning/UNIFY_CACHE_PLAN.md` §6.1): the crate version changes
/// for reasons unrelated to the semantic construction algorithm (docs, unrelated modules,
/// dependency bumps), while this constant changes only when parsing, linking, effective-fact
/// construction, pending resolution, or evaluation semantics change in a way that could make a
/// previously published graph an unsafe substitute for a fresh build. Bump it whenever such a
/// change lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SemanticContractVersion(pub u32);

/// The current semantic algorithm contract. Increment when construction, linking, or evaluation
/// semantics change in a way that would make an older publication an unsafe substitute for a
/// fresh build.
pub const CURRENT_SEMANTIC_CONTRACT_VERSION: SemanticContractVersion = SemanticContractVersion(1);

/// How far a graph's construction has progressed, in strict barrier order.
///
/// Ordered so `SemanticPhase` comparisons (`<`, `>=`, ...) express "has this barrier been
/// crossed" directly, and so [`SemanticPublication::advance_phase`] can enforce monotonicity by
/// construction rather than by convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemanticPhase {
    /// Documents have been parsed into ASTs and merged into node/edge state, but workspace
    /// relationship linking has not (yet) run over the merged graph.
    Parsed,
    /// Workspace relationship linking, effective-fact construction, and pending-relationship
    /// resolution have crossed their barrier (`finalize_workspace_graph` /
    /// `link_parsed_documents_parallel_from`'s structural phase). Expression evaluation has not
    /// necessarily run, or ran against a now-stale structural state.
    StructurallyLinked,
    /// Expression evaluation has crossed its barrier against the current structural state
    /// (`evaluate_expressions`, gated the same way `evaluation_publication` is). The only phase
    /// eligible for persistent storage, and only when paired with
    /// [`SemanticCompleteness::Complete`].
    SettledEvaluated,
}

/// Whether a graph's construction input and process were complete, and if not, how it degraded.
///
/// Distinguishes "the graph is settled and its facts are exactly what a fresh build would
/// produce" from every other way construction can legitimately end. None of the non-`Complete`
/// variants are evidence of a bug: they are the explicit, typed alternative to guessing,
/// substituting, or silently treating degraded input as success (`AGENTS.md`, "Keep unresolved,
/// ambiguous, unsupported, partial, cancelled, and failed states explicit").
///
/// This is orthogonal to [`SemanticPhase`]: a `SettledEvaluated` graph can still be
/// `EditorRecovery` (parsed with recovered syntax errors but linked/evaluated to completion), and
/// pending unresolved/ambiguous *semantic* relationships never make a build `Partial` — see the
/// module-level warning and `planning/UNIFY_CACHE_PLAN.md` §4.3:
/// an explicit unresolved or ambiguous outcome is itself a complete, correctly settled fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SemanticCompleteness {
    /// Every admitted source parsed with strict success and construction ran to its declared
    /// phase with no cancellation, no unsupported input, and no failure. Explicit unresolved or
    /// ambiguous *semantic* facts (pending relationships, evaluation outcomes) do not disqualify
    /// this — they are settled, typed facts, not missing ones.
    Complete,
    /// At least one admitted source required editor/parser recovery (non-empty parse
    /// diagnostics) rather than strict parse success. Construction otherwise ran to its declared
    /// phase.
    EditorRecovery,
    /// Construction encountered input or a configuration it does not have a defined semantic
    /// mapping for (e.g. an unsupported language construct) and explicitly excluded it rather
    /// than guessing.
    Unsupported,
    /// Construction covers less than the complete admitted source set — e.g. a single-document
    /// live-edit patch, or a scoped frontier relink — rather than every admitted source.
    Partial,
    /// Construction was cancelled before reaching its declared phase (e.g. superseded by a newer
    /// revision).
    Cancelled,
    /// Construction attempted its declared phase and did not complete it (e.g. an internal
    /// error, or a safety bound in iterative enrichment was hit without convergence).
    Failed,
}

/// The graph's own publication identity, phase, and completeness — `SemanticPublication` from
/// `planning/UNIFY_CACHE_PLAN.md` §4.3 and the required resolution for the `SemanticPublication` contract.
///
/// Every [`SemanticGraph`](crate::semantic::graph::SemanticGraph) carries one. It answers, without
/// inspecting graph content: which exact source root produced this graph
/// ([`Self::root_digest`]), how far construction progressed ([`Self::phase`]), and whether that
/// construction was complete ([`Self::completeness`]). [`Self::is_storage_eligible`] is the single
/// typed predicate for whether a graph may be accepted into persistent cache storage — no other
/// code should reimplement that check from the individual fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticPublication {
    root_digest: RootDigest,
    phase: SemanticPhase,
    completeness: SemanticCompleteness,
    semantic_contract: SemanticContractVersion,
}

impl SemanticPublication {
    /// Starts a new publication at [`SemanticPhase::Parsed`] for the given source root and
    /// completeness, stamped with the [`CURRENT_SEMANTIC_CONTRACT_VERSION`].
    ///
    /// `completeness` is the caller's honest assessment of *this* construction (e.g. whether any
    /// admitted source needed editor recovery). It is never inferred from graph shape.
    pub fn new(root_digest: RootDigest, completeness: SemanticCompleteness) -> Self {
        Self {
            root_digest,
            phase: SemanticPhase::Parsed,
            completeness,
            semantic_contract: CURRENT_SEMANTIC_CONTRACT_VERSION,
        }
    }

    /// A publication for a graph that has not (yet) had a real source root or completeness
    /// established — e.g. a freshly constructed empty graph before any documents are known. Its
    /// [`SemanticCompleteness::Partial`] and [`SemanticPhase::Parsed`] make it correctly
    /// ineligible for storage until a real build supplies a real root digest and completeness via
    /// [`Self::new`].
    pub fn unpublished() -> Self {
        Self::new(
            RootDigest::from_digest(Blake3Digest::from_bytes([0u8; 32])),
            SemanticCompleteness::Partial,
        )
    }

    pub fn root_digest(&self) -> RootDigest {
        self.root_digest
    }

    pub fn phase(&self) -> SemanticPhase {
        self.phase
    }

    pub fn completeness(&self) -> SemanticCompleteness {
        self.completeness
    }

    pub fn semantic_contract(&self) -> SemanticContractVersion {
        self.semantic_contract
    }

    /// Replaces the source-root identity and completeness while leaving `phase` untouched.
    ///
    /// Used by the pipeline entry points that own document enumeration (`build_and_link_graph`,
    /// `build_and_link_graph_parallel`) to stamp the real root digest and completeness onto a
    /// freshly constructed graph before phase transitions begin. Not a phase transition itself —
    /// callers that also know the graph is further along must call [`Self::advance_phase`]
    /// separately, in barrier order, the same way pipeline functions already do.
    pub fn set_identity(&mut self, root_digest: RootDigest, completeness: SemanticCompleteness) {
        self.root_digest = root_digest;
        self.completeness = completeness;
    }

    /// Overrides completeness only, leaving root digest and phase untouched. Used when
    /// construction degrades (cancelled, failed, unsupported) partway through a phase that had
    /// already started with `Complete` or `EditorRecovery`.
    pub fn set_completeness(&mut self, completeness: SemanticCompleteness) {
        self.completeness = completeness;
    }

    /// Advances `phase` to `phase`, or leaves it unchanged if `phase` is not further along than
    /// the current phase.
    ///
    /// This is the only way `phase` ever changes after construction, and it can only move
    /// forward: regression is structurally impossible to express here rather than merely
    /// discouraged, satisfying `AGENTS.md`'s "Publish coherent model states atomically" and
    /// the `SemanticPublication` contract's "phase transitions explicit and monotonic". Taking the
    /// max (rather than asserting `phase >= self.phase`) is deliberate, not merely lenient: a
    /// pipeline barrier function such as `finalize_and_evaluate_frontier` unconditionally
    /// advances to `StructurallyLinked` before deciding whether to evaluate, and is itself a valid
    /// re-entry point on an already-`SettledEvaluated` graph (e.g. re-finalizing after a second,
    /// independent frontier refresh) -- that call is a legitimate no-op here, not a bug to flag.
    pub fn advance_phase(&mut self, phase: SemanticPhase) {
        if phase > self.phase {
            self.phase = phase;
        }
    }

    /// The single typed predicate for whether this publication may be accepted into persistent
    /// graph storage (the `SemanticPublication` contract, `planning/UNIFY_CACHE_PLAN.md` §4.3: "Only a
    /// complete settled/evaluated publication is eligible for a persistent semantic-graph
    /// entry.").
    ///
    /// Deliberately does **not** consult pending/unresolved/ambiguous relationship or evaluation
    /// outcomes — those are settled, explicit facts once [`SemanticPhase::SettledEvaluated`] is
    /// reached, not evidence of incompleteness. Getting this backwards (treating an explicit
    /// unresolved/ambiguous result as "incomplete") is called out as the easiest mistake to make
    /// in both design documents; this predicate is the one place that decision is made, so no
    /// other code should reimplement it from the individual fields.
    pub fn is_storage_eligible(&self) -> bool {
        self.phase == SemanticPhase::SettledEvaluated
            && self.completeness == SemanticCompleteness::Complete
    }
}

impl Default for SemanticPublication {
    fn default() -> Self {
        Self::unpublished()
    }
}

#[cfg(test)]
mod publication_contract_tests {
    use super::*;

    fn digest(byte: u8) -> RootDigest {
        RootDigest::from_digest(Blake3Digest::from_bytes([byte; 32]))
    }

    #[test]
    fn advance_phase_moves_forward() {
        let mut publication = SemanticPublication::new(digest(1), SemanticCompleteness::Complete);
        assert_eq!(publication.phase(), SemanticPhase::Parsed);
        publication.advance_phase(SemanticPhase::StructurallyLinked);
        assert_eq!(publication.phase(), SemanticPhase::StructurallyLinked);
        publication.advance_phase(SemanticPhase::SettledEvaluated);
        assert_eq!(publication.phase(), SemanticPhase::SettledEvaluated);
    }

    #[test]
    fn advance_phase_never_regresses() {
        let mut publication = SemanticPublication::new(digest(1), SemanticCompleteness::Complete);
        publication.advance_phase(SemanticPhase::SettledEvaluated);
        // Attempting to move backward is a no-op, not an error and not applied — regression is
        // impossible to observe even though the call itself does not panic in release builds.
        publication.advance_phase(SemanticPhase::Parsed);
        assert_eq!(publication.phase(), SemanticPhase::SettledEvaluated);
        publication.advance_phase(SemanticPhase::StructurallyLinked);
        assert_eq!(publication.phase(), SemanticPhase::SettledEvaluated);
    }

    #[test]
    fn settled_complete_is_storage_eligible() {
        let mut publication = SemanticPublication::new(digest(1), SemanticCompleteness::Complete);
        publication.advance_phase(SemanticPhase::StructurallyLinked);
        assert!(!publication.is_storage_eligible());
        publication.advance_phase(SemanticPhase::SettledEvaluated);
        assert!(publication.is_storage_eligible());
    }

    #[test]
    fn structurally_linked_is_never_storage_eligible() {
        let mut publication = SemanticPublication::new(digest(1), SemanticCompleteness::Complete);
        publication.advance_phase(SemanticPhase::StructurallyLinked);
        assert!(!publication.is_storage_eligible());
    }

    #[test]
    fn non_complete_completeness_is_never_storage_eligible() {
        for completeness in [
            SemanticCompleteness::EditorRecovery,
            SemanticCompleteness::Unsupported,
            SemanticCompleteness::Partial,
            SemanticCompleteness::Cancelled,
            SemanticCompleteness::Failed,
        ] {
            let mut publication = SemanticPublication::new(digest(1), completeness);
            publication.advance_phase(SemanticPhase::SettledEvaluated);
            assert!(
                !publication.is_storage_eligible(),
                "{completeness:?} must not be storage-eligible even when settled/evaluated"
            );
        }
    }

    #[test]
    fn unpublished_default_is_not_storage_eligible() {
        assert!(!SemanticPublication::unpublished().is_storage_eligible());
        assert!(!SemanticPublication::default().is_storage_eligible());
    }

    #[test]
    fn set_identity_replaces_root_and_completeness_not_phase() {
        let mut publication = SemanticPublication::new(digest(1), SemanticCompleteness::Complete);
        publication.advance_phase(SemanticPhase::StructurallyLinked);
        publication.set_identity(digest(2), SemanticCompleteness::EditorRecovery);
        assert_eq!(publication.root_digest(), digest(2));
        assert_eq!(
            publication.completeness(),
            SemanticCompleteness::EditorRecovery
        );
        assert_eq!(publication.phase(), SemanticPhase::StructurallyLinked);
    }
}
