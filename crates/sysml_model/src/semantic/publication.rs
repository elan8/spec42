//! Immutable semantic publication and canonical relationship-resolution results.
//!
//! The graph builder is deliberately an implementation detail of this module's build service.
//! A [`SemanticModel`] is the only settled semantic state that new consumers should retain.  The
//! existing graph is used while migrating the older builders, then is frozen behind this value;
//! no resolver or query method is allowed to mutate it after publication.

use std::collections::BTreeMap;
use std::fmt;

use sha2::{Digest, Sha256};

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{
    DeclaredRelationshipFacts, DeclaredRelationshipTarget, NodeId, RelationshipKind, SemanticNode,
};
use crate::semantic::pipeline::{build_and_link_graph, build_and_link_graph_parallel};
use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};

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
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionOutcome {
    Resolved { target: NodeId },
    Unresolved,
    Ambiguous { candidates: Vec<NodeId> },
}

impl ResolutionOutcome {
    pub fn resolved_target(&self) -> Option<&NodeId> {
        match self {
            Self::Resolved { target } => Some(target),
            Self::Unresolved | Self::Ambiguous { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionFact {
    pub reference: AuthoredReferenceId,
    pub authored_target: String,
    pub outcome: ResolutionOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionProvenance {
    Authored,
    Implied,
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRelationship {
    pub source: NodeId,
    pub target: NodeId,
    pub kind: RelationshipKind,
    pub provenance: ResolutionProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionFailure {
    DidNotConverge {
        passes: usize,
        changing_families: Vec<String>,
        pending_references: Vec<AuthoredReferenceId>,
    },
}

impl fmt::Display for ResolutionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DidNotConverge {
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

    pub(crate) fn solve(self) -> Result<ResolutionState, ResolutionFailure> {
        let mut facts = Vec::new();
        let mut relationships = Vec::new();
        let edges = self.graph.semantic_edges();
        let nodes = self.graph.semantic_nodes();

        // The graph linker has already materialized the structural relationship pass.  We
        // project its result into typed facts in one deterministic pass.  No graph mutation or
        // first-candidate selection is performed here.
        for node in nodes {
            for (kind, targets) in authored_relationships(&node.declared_facts.relationships) {
                let relationship_kind = kind.relationship_kind();
                for (ordinal, authored) in targets.into_iter().enumerate() {
                    let mut candidates = relationship_kind
                        .as_ref()
                        .map(|relationship_kind| {
                            edges
                                .iter()
                                .filter(|(source, target, edge)| {
                                    *source == node.id
                                        && edge.kind == *relationship_kind
                                        && authored_target_matches(target, &authored.reference)
                                })
                                .map(|(_, target, _)| target.clone())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    // Qualified-name duplicates are a genuine ambiguity, even where the graph
                    // linker happened to retain one edge for an older consumer.
                    candidates.extend(explicit_name_candidates(
                        self.graph,
                        &node,
                        &authored.reference,
                    ));
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
                        });
                    }
                    facts.push(ResolutionFact {
                        reference,
                        authored_target: authored.reference,
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
                    let candidates =
                        explicit_name_candidates(self.graph, &node, &import.target.reference);
                    let outcome = match candidates.as_slice() {
                        [target] => ResolutionOutcome::Resolved {
                            target: target.clone(),
                        },
                        [] => ResolutionOutcome::Unresolved,
                        candidates => ResolutionOutcome::Ambiguous {
                            candidates: candidates.to_vec(),
                        },
                    };
                    facts.push(ResolutionFact {
                        reference: AuthoredReferenceId {
                            source: node.id.clone(),
                            kind,
                            authored_ordinal: 0,
                        },
                        authored_target: import.target.reference.clone(),
                        outcome,
                    });
                }
            }
        }

        facts.sort_by(|left, right| left.reference.cmp(&right.reference));
        relationships.sort_by(|left, right| {
            (&left.source, &left.kind, &left.target).cmp(&(
                &right.source,
                &right.kind,
                &right.target,
            ))
        });
        relationships.dedup();
        Ok(ResolutionState {
            facts,
            relationships,
        })
    }
}

fn authored_target_matches(target: &NodeId, authored: &str) -> bool {
    let normalized = authored
        .trim()
        .trim_start_matches('~')
        .trim_matches(['\'', '"'])
        .replace('.', "::");
    target.qualified_name == normalized
        || target.qualified_name.ends_with(&format!("::{normalized}"))
        || target.qualified_name.rsplit("::").next() == Some(normalized.as_str())
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
        return graph
            .node_ids_for_qualified_name(&normalized)
            .unwrap_or(&[])
            .to_vec();
    }
    let nodes = graph.semantic_nodes();
    let mut children = BTreeMap::<Option<NodeId>, Vec<NodeId>>::new();
    for node in nodes {
        if node.name == normalized
            || node.declared_name.as_deref() == Some(normalized.as_str())
            || node
                .attributes
                .get("shortName")
                .and_then(serde_json::Value::as_str)
                == Some(normalized.as_str())
        {
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
        parent = graph
            .get_node(&current)
            .and_then(|node| node.parent_id.clone());
    }
    candidates.extend(children.get(&None).into_iter().flatten().cloned());
    candidates
}

fn authored_relationships(
    facts: &DeclaredRelationshipFacts,
) -> Vec<(ReferenceKind, Vec<DeclaredRelationshipTarget>)> {
    vec![
        (ReferenceKind::FeatureTyping, facts.typing.clone()),
        (ReferenceKind::Specialization, facts.specializes.clone()),
        (ReferenceKind::Subsetting, facts.subsetting.clone()),
        (ReferenceKind::Redefinition, facts.redefinition.clone()),
        (
            ReferenceKind::ReferenceSubsetting,
            facts.reference_subsetting.clone(),
        ),
        (
            ReferenceKind::CrossSubsetting,
            facts.cross_subsetting.clone(),
        ),
    ]
}

fn node_id_order(left: &NodeId, right: &NodeId) -> std::cmp::Ordering {
    left.uri
        .as_str()
        .cmp(right.uri.as_str())
        .then_with(|| left.qualified_name.cmp(&right.qualified_name))
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationState;

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
    let identity = SemanticModelIdentity::for_request(&request.sources, &request.configuration);
    let documents = request.sources.documents();
    let (graph, _) = match request.construction {
        ConstructionStrategy::Sequential => {
            build_and_link_graph(documents).map_err(SemanticBuildFailure::InvalidInput)?
        }
        ConstructionStrategy::Parallel => build_and_link_graph_parallel(documents),
    };
    let resolution = ResolutionDb::new(&graph)
        .solve()
        .map_err(SemanticBuildFailure::Resolution)?;
    let mut structural_graph = graph;
    structural_graph.remove_resolution_edges();
    if matches!(request.evaluation, EvaluationPolicy::ResolvedOnly) {
        structural_graph.clear_evaluation_state();
    }
    let indexes = SemanticQueryIndexes::from_state(&resolution);
    Ok(SemanticModel {
        identity,
        structural_graph,
        resolution,
        evaluation: matches!(request.evaluation, EvaluationPolicy::Evaluate)
            .then_some(EvaluationState),
        phase: if matches!(request.evaluation, EvaluationPolicy::Evaluate) {
            SemanticPhase::Evaluated
        } else {
            SemanticPhase::Resolved
        },
        completeness: SemanticCompleteness::Complete,
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
    fn missing_targets_remain_explicitly_unresolved() {
        let model = build("package A { part p : Missing; }");
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.kind == ReferenceKind::FeatureTyping
                && matches!(fact.outcome, ResolutionOutcome::Unresolved)
        }));
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
