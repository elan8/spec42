//! Immutable semantic publication and canonical relationship-resolution results.
//!
//! The graph builder is deliberately an implementation detail of this module's build service.
//! A [`SemanticModel`] is the only settled semantic state that new consumers should retain.  The
//! existing graph is used while migrating the older builders, then is frozen behind this value;
//! no resolver or query method is allowed to mutate it after publication.

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use sha2::{Digest, Sha256};

use crate::semantic::graph::{DeclaredExpressionRelationshipRecord, SemanticGraph};
pub use crate::semantic::model::DerivedRelationshipRule;
use crate::semantic::model::{
    DeclaredExpressionRelationship, DeclaredRelationshipFacts, DeclaredRelationshipTarget,
    ElementKind, ImpliedRelationshipRule, NodeEvaluationFacts, NodeId, RelationshipKind,
    SemanticEdge, SemanticNode,
};
use crate::semantic::pipeline::build_structural_graph;
use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
use crate::semantic::text_span::TextRange;

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

/// Content-complete identity for an immutable semantic input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticModelIdentity {
    pub source_digest: String,
    pub semantic_contract_version: String,
}

impl SemanticModelIdentity {
    fn for_request(
        snapshot: &ImmutableSourceSnapshot,
        configuration: &SemanticConfiguration,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"spec42-semantic-input\0");
        hasher.update(configuration.semantic_contract_version.as_bytes());
        hasher.update([0]);
        for document in snapshot.documents() {
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
            semantic_contract_version: configuration.semantic_contract_version.clone(),
        }
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
pub struct ResolutionFact {
    pub reference: AuthoredReferenceId,
    pub authored_target: String,
    pub authored_range: Option<TextRange>,
    pub outcome: ResolutionOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionProvenance {
    Authored,
    Implied(ImpliedRelationshipRule),
    Derived(DerivedRelationshipRule),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRelationship {
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
pub struct ResolutionState {
    facts: Vec<ResolutionFact>,
    relationships: Vec<ResolvedRelationship>,
}

impl ResolutionState {
    pub fn facts(&self) -> &[ResolutionFact] {
        &self.facts
    }

    pub fn outcome(&self, reference: &AuthoredReferenceId) -> Option<&ResolutionOutcome> {
        self.facts
            .binary_search_by(|fact| fact.reference.cmp(reference))
            .ok()
            .map(|index| &self.facts[index].outcome)
    }

    pub fn relationships(&self) -> &[ResolvedRelationship] {
        &self.relationships
    }

    pub(crate) fn install_relationship_projection(&self, graph: &mut SemanticGraph) {
        for relationship in &self.relationships {
            crate::semantic::relationships::add_semantic_edge_once(
                graph,
                &relationship.source,
                &relationship.target,
                match relationship.provenance {
                    ResolutionProvenance::Authored => {
                        SemanticEdge::plain(relationship.kind.clone())
                    }
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
            let solved = self.solve_once();
            // Keep unresolved slots in the working state.  Omitting a pending key would make
            // the generic fixed-point driver mistake a partial pass for convergence.
            let mut next = previous.clone();
            for fact in solved.facts() {
                let ready = previous
                    .get(&fact.reference)
                    .is_some_and(|outcome| match outcome {
                        WorkingOutcome::Final(_) => true,
                        WorkingOutcome::Pending { dependencies } => {
                            dependencies.iter().all(|dependency| {
                                matches!(previous.get(dependency), Some(WorkingOutcome::Final(_)))
                            })
                        }
                    });
                if ready {
                    next.insert(
                        fact.reference.clone(),
                        WorkingOutcome::Final(fact.outcome.clone()),
                    );
                }
            }
            next
        });
        match result {
            Ok(_) => Ok(self.solve_once()),
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

    fn solve_once(&self) -> ResolutionState {
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
                    let mut candidates =
                        explicit_name_candidates(self.graph, &node, &authored.reference);
                    candidates.sort_by(node_id_order);
                    candidates.dedup();
                    let reference = AuthoredReferenceId {
                        source: node.id.clone(),
                        kind,
                        authored_ordinal: ordinal as u32,
                    };
                    let outcome = match candidates.as_slice() {
                        [target] => ResolutionOutcome::Resolved {
                            target: target.clone(),
                        },
                        [] => ResolutionOutcome::Unresolved,
                        candidates => ResolutionOutcome::Ambiguous {
                            candidates: candidates.to_vec(),
                        },
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
                            candidates,
                        } => ResolutionOutcome::Ambiguous { candidates },
                        crate::semantic::import_resolution::ImportTargetResolution::UnsupportedFiltered => {
                            ResolutionOutcome::UnsupportedFiltered
                        }
                    };
                    facts.push(ResolutionFact {
                        reference: AuthoredReferenceId {
                            source: node.id.clone(),
                            kind,
                            authored_ordinal: 0,
                        },
                        authored_target: import.target.reference.clone(),
                        authored_range: import.target.range,
                        outcome,
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
        }
    }
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
        RelationshipKind::Transition | RelationshipKind::InitialState => (
            ReferenceKind::TransitionSource,
            ReferenceKind::TransitionTarget,
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
        RelationshipKind::Transition | RelationshipKind::InitialState => (
            ReferenceKind::TransitionSource,
            ReferenceKind::TransitionTarget,
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
        (ReferenceKind::TransitionSource, facts.initial_state.clone()),
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
pub enum SemanticPhase {
    Resolved,
    Evaluated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticCompleteness {
    Complete,
    EditorRecovery,
}

#[derive(Debug, Clone)]
pub struct EvaluationState {
    facts: HashMap<NodeId, NodeEvaluationFacts>,
}

/// Read-only semantic publication.
#[derive(Debug, Clone)]
pub struct SemanticModel {
    identity: SemanticModelIdentity,
    structural_graph: SemanticGraph,
    resolution: ResolutionState,
    evaluation: Option<EvaluationState>,
    phase: SemanticPhase,
    completeness: SemanticCompleteness,
    indexes: SemanticQueryIndexes,
}

impl SemanticModel {
    pub fn identity(&self) -> &SemanticModelIdentity {
        &self.identity
    }

    pub fn phase(&self) -> SemanticPhase {
        self.phase
    }

    pub fn completeness(&self) -> SemanticCompleteness {
        self.completeness
    }

    pub fn resolution(&self) -> &ResolutionState {
        &self.resolution
    }

    pub fn has_evaluation(&self) -> bool {
        self.evaluation.is_some()
    }

    pub fn evaluation_facts(&self) -> Option<&HashMap<NodeId, NodeEvaluationFacts>> {
        self.evaluation.as_ref().map(|state| &state.facts)
    }

    pub fn view(&self) -> ResolutionView<'_> {
        ResolutionView { model: self }
    }
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
pub struct ResolutionView<'a> {
    model: &'a SemanticModel,
}

impl<'a> ResolutionView<'a> {
    pub fn outcome(&self, reference: &AuthoredReferenceId) -> Option<&'a ResolutionOutcome> {
        self.model.resolution.outcome(reference)
    }

    pub fn outgoing(&self, source: &NodeId, kind: RelationshipKind) -> &'a [NodeId] {
        self.model
            .indexes
            .outgoing
            .get(&(source.clone(), kind))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn incoming(&self, target: &NodeId, kind: RelationshipKind) -> &'a [NodeId] {
        self.model
            .indexes
            .incoming
            .get(&(target.clone(), kind))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn node(&self, id: &NodeId) -> Option<&'a SemanticNode> {
        self.model.structural_graph.get_node(id)
    }
}

/// Build one immutable semantic publication.  Construction strategy affects only parsing and
/// authored graph construction; both strategies enter the same resolution publication barrier.
pub fn build_semantic_model(
    request: SemanticBuildRequest,
) -> Result<SemanticModel, SemanticBuildFailure> {
    build_semantic_model_with_max_passes(request, 1_000)
}

pub(crate) fn build_semantic_model_with_max_passes(
    request: SemanticBuildRequest,
    max_passes: usize,
) -> Result<SemanticModel, SemanticBuildFailure> {
    let identity = SemanticModelIdentity::for_request(&request.sources, &request.configuration);
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
    Ok(SemanticModel {
        identity,
        structural_graph,
        resolution,
        evaluation,
        phase: if matches!(request.evaluation, EvaluationPolicy::Evaluate) {
            SemanticPhase::Evaluated
        } else {
            SemanticPhase::Resolved
        },
        completeness,
        indexes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(model.completeness(), SemanticCompleteness::EditorRecovery);
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
        assert_ne!(
            SemanticModelIdentity::for_request(&first, &config),
            SemanticModelIdentity::for_request(&second, &config)
        );
    }

    #[test]
    fn model_publishes_indexed_relationships() {
        let model = build("package A { part def P {} part p : P; }");
        assert_eq!(model.phase(), SemanticPhase::Resolved);
        assert!(!model.has_evaluation());
        assert_eq!(model.completeness(), SemanticCompleteness::Complete);
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
    fn expression_relationships_are_recorded_and_resolved_at_publication() {
        let model = build("package M { part def System { part a; part b; connect a to b; } }");
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.kind == ReferenceKind::ConnectionSource
                && matches!(fact.outcome, ResolutionOutcome::Resolved { .. })
        }));
        let connection = model
            .resolution()
            .relationships()
            .iter()
            .find(|relationship| relationship.kind == RelationshipKind::Connection)
            .expect("canonical connection relationship");
        assert_eq!(connection.source.qualified_name, "M::System::a");
        assert_eq!(connection.target.qualified_name, "M::System::b");
        assert!(connection.expression.is_some());
    }

    #[test]
    fn generic_flow_builder_targets_are_resolved_canonically() {
        let model = build(
            "package P { action def ExecuteMission { action validateRoute; action startMission; first validateRoute then startMission; } }",
        );
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.kind == ReferenceKind::FlowSource
                && matches!(fact.outcome, ResolutionOutcome::Resolved { .. })
        }));
        assert!(model
            .resolution()
            .relationships()
            .iter()
            .any(|relationship| relationship.kind == RelationshipKind::Flow));
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
    fn missing_targets_remain_explicitly_unresolved() {
        let model = build("package A { part p : Missing; }");
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.kind == ReferenceKind::FeatureTyping
                && matches!(fact.outcome, ResolutionOutcome::Unresolved)
        }));
    }

    #[test]
    fn imported_candidates_are_ambiguous_in_canonical_order() {
        let model = build(
            "package A { part def T; }
             package B { part def T; }
             package C {
                 import A::*;
                 import B::*;
                 part p : T;
             }",
        );
        let fact = model
            .resolution()
            .facts()
            .iter()
            .find(|fact| fact.reference.kind == ReferenceKind::FeatureTyping)
            .expect("typing fact");
        let ResolutionOutcome::Ambiguous { candidates } = &fact.outcome else {
            panic!(
                "expected imported duplicate to remain ambiguous: {:?}",
                fact.outcome
            );
        };
        assert_eq!(candidates.len(), 2);
        assert!(candidates[0].qualified_name < candidates[1].qualified_name);
    }

    #[test]
    fn filtered_imports_publish_an_explicit_unsupported_outcome() {
        let model =
            build("package Source { part def Item; } package Client { import Source [ 1 ]; }");
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.kind == ReferenceKind::NamespaceImport
                && matches!(fact.outcome, ResolutionOutcome::UnsupportedFiltered)
        }));
    }

    #[test]
    fn inner_lexical_binding_shadows_outer_import_even_when_incompatible() {
        let model = build(
            "package A { part def T; }
             package C {
                 import A::*;
                 part T;
                 part p : T;
             }",
        );
        let fact = model
            .resolution()
            .facts()
            .iter()
            .find(|fact| fact.reference.kind == ReferenceKind::FeatureTyping)
            .expect("typing fact");
        let ResolutionOutcome::Resolved { target } = &fact.outcome else {
            panic!("inner lexical binding must be retained: {:?}", fact.outcome);
        };
        assert_eq!(target.qualified_name, "C::T");
    }

    #[test]
    fn qualified_segments_are_resolved_from_the_innermost_namespace() {
        let model = build(
            "package A { part def T; }
             package C {
                 package A { part def T; }
                 part p : A::T;
             }",
        );
        let fact = model
            .resolution()
            .facts()
            .iter()
            .find(|fact| fact.reference.kind == ReferenceKind::FeatureTyping)
            .expect("typing fact");
        let ResolutionOutcome::Resolved { target } = &fact.outcome else {
            panic!("qualified target should resolve: {:?}", fact.outcome);
        };
        assert_eq!(target.qualified_name, "C::A::T");
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
    fn inherited_members_are_resolved_and_deduplicated_across_a_diamond() {
        let model = build(
            "package M {
                 part def Base { part def Member; }
                 part def Left :> Base;
                 part def Right :> Base;
                 part def Diamond :> Left, Right { part p : Member; }
             }",
        );
        let fact = model
            .resolution()
            .facts()
            .iter()
            .find(|fact| fact.reference.kind == ReferenceKind::FeatureTyping)
            .expect("typing fact");
        let ResolutionOutcome::Resolved { target } = &fact.outcome else {
            panic!("inherited member should resolve: {:?}", fact.outcome);
        };
        assert_eq!(target.qualified_name, "M::Base::Member");
    }

    #[test]
    fn case_subject_is_a_derived_relationship_with_explicit_provenance() {
        let model = build("package M { part def P; analysis def A { subject s : P; } }");
        let subject = model
            .resolution()
            .relationships()
            .iter()
            .find(|relationship| relationship.kind == RelationshipKind::Subject)
            .expect("derived subject relationship");
        assert_eq!(subject.source.qualified_name, "M::A");
        assert_eq!(subject.target.qualified_name, "M::P");
        assert_eq!(
            subject.provenance,
            ResolutionProvenance::Derived(DerivedRelationshipRule::CaseSubjectFromTypedSubject)
        );
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
        assert_eq!(model.phase(), SemanticPhase::Evaluated);
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
