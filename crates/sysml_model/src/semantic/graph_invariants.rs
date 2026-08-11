//! Cache-import invariant validator for [`crate::semantic::graph::SemanticGraphData`]
//! (`ROUNDTRIP_SEMGRAPH_PREREQS.md` B7).
//!
//! This module is the single owner of graph-invariant validation. Nothing else in the crate
//! should hand-roll a second ad hoc consistency check over the semantic graph: a new invariant
//! belongs here, as a new [`GraphInvariantError`] variant plus a check in
//! [`validate_graph_invariants`].
//!
//! # Why this exists
//!
//! A decoded cache payload is local but not trusted semantic input (`ROUNDTRIP_SEMGRAPH_PREREQS.md`
//! B10): corruption, a crash between commit and enforcement, partial writes, and another local
//! process can all produce a structurally-decodable-but-incoherent graph. Before B7, deserialization
//! trusted graph content and simply populated maps from it (`SemanticGraphData::rebuild_derived_indexes`
//! in `graph.rs`). This module is what a cache decoder runs *before* any of that content is treated
//! as authoritative or published to a consumer.
//!
//! # Cache miss, not a model diagnostic
//!
//! A [`GraphInvariantError`] must never be surfaced as though the user's source model were at
//! fault. It is deliberately not a `sysml_diagnostics` diagnostic and carries no source-facing
//! severity, code, or message contract; the only sanctioned use is `Err` propagation into a cache
//! layer that maps it to a typed miss (`workspace::cache::api::CacheMissReason::InvariantFailure`)
//! and falls back to the canonical uncached build path. See the `separates_from_user_diagnostics`
//! test below.
//!
//! # Artifact-specific containment
//!
//! A deliberately extracted library subgraph may legitimately omit a workspace parent (its root
//! nodes have `parent_id: None` even though, in a full workspace build, an ancestor package would
//! exist). [`GraphArtifactKind`] parameterizes the validator so this is expressed as an explicit
//! per-artifact containment policy rather than a blanket relaxation of the full-workspace rule.
//!
//! # Iterative expression walking
//!
//! `DeclaredExpression` can be arbitrarily deep and already carries a custom iterative `Drop` to
//! avoid stack overflow on adversarial nesting (`model.rs`). [`expression_depth_and_count`] mirrors
//! that iterative worklist pattern: it walks with an explicit heap-allocated stack rather than
//! recursion, so a validator run over hostile input cannot itself reintroduce the stack-overflow
//! risk `Drop` was written to avoid.

// `GraphInvariantError` deliberately stays a flat, directly-matchable enum (no internal boxing)
// so every call site and test can `matches!`/destructure a variant without an extra deref. It is
// returned only from this module's validation entry points -- never threaded through a hot loop
// -- so the larger `Result::Err` outweighs the ergonomic cost of boxing here.
#![allow(clippy::result_large_err)]

use std::collections::{HashMap, HashSet};

use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use url::Url;

use crate::semantic::graph::SemanticGraphData;
use crate::semantic::model::{
    DeclaredExpression, DerivedRelationshipResolution, EvaluatedValue, EvaluationPublicationState,
    NodeId,
};
use source_identity::SourceRole;

/// Which containment/publication policy applies to the graph being validated.
///
/// `ROUNDTRIP_SEMGRAPH_PREREQS.md` B7 requires artifact-specific containment rules rather than a
/// global relaxation: a deliberately extracted library subgraph may omit a workspace parent, but a
/// full workspace graph may not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphArtifactKind {
    /// A complete workspace semantic graph. Every node must resolve a `parent_id` to another node
    /// present in the same graph; there is no legitimate reason for a workspace build to leave a
    /// node parentless or reference a parent that was not also materialized.
    Workspace,
    /// A library subgraph deliberately extracted for reuse as a cached base graph. Root-level
    /// library nodes may have no parent in this artifact even though a full workspace build would
    /// place them under a package; only *cycles* and *dangling non-root* parent references remain
    /// rejected.
    LibrarySubgraph,
}

/// Explicit, injectable resource bounds for [`validate_graph_invariants`]
/// (`ROUNDTRIP_SEMGRAPH_PREREQS.md` B10). Production callers use [`GraphInvariantLimits::default`];
/// tests inject small values to exercise the boundary cases without building huge fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphInvariantLimits {
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_effective_facts: usize,
    pub max_evaluation_facts: usize,
    pub max_derived_relationship_resolutions: usize,
    pub max_pending_relationships: usize,
    pub max_pending_expression_relationships: usize,
    /// Bound on any single `String` field considered by this validator (names, qualified names),
    /// in UTF-8 bytes.
    pub max_string_bytes: usize,
    /// Bound on the number of `DeclaredExpression` nodes reachable from any one root expression.
    pub max_expression_nodes: usize,
    /// Bound on `DeclaredExpression` nesting depth from any one root expression.
    pub max_expression_depth: usize,
}

impl Default for GraphInvariantLimits {
    fn default() -> Self {
        Self {
            max_nodes: 2_000_000,
            max_edges: 8_000_000,
            max_effective_facts: 2_000_000,
            max_evaluation_facts: 2_000_000,
            max_derived_relationship_resolutions: 2_000_000,
            max_pending_relationships: 500_000,
            max_pending_expression_relationships: 500_000,
            max_string_bytes: 1 << 20,
            max_expression_nodes: 100_000,
            max_expression_depth: 4_096,
        }
    }
}

/// A named collection whose length is bounded by [`GraphInvariantLimits`], used to identify which
/// bound a [`GraphInvariantError::TooManyRecords`] violated without a stringly-typed field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedCollection {
    Nodes,
    Edges,
    EffectiveFacts,
    EvaluationFacts,
    DerivedRelationshipResolutions,
    PendingRelationships,
    PendingExpressionRelationships,
}

/// Which typed record a dangling reference was found in, for
/// [`GraphInvariantError::DanglingReference`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceSite {
    EffectiveFactsKey,
    EffectiveFactsFeaturingType,
    EffectiveFactsExpressionOwner,
    EvaluationFactsKey,
    DerivedRelationshipResolutionKey,
    DerivedRelationshipResolutionTarget,
    EdgeEndpoint,
}

/// Every rejection this validator can produce. One variant per `ROUNDTRIP_SEMGRAPH_PREREQS.md` B7
/// category. This type carries no source range, diagnostic code, or severity: it is a cache-import
/// failure, never a user-facing model diagnostic (see the module doc comment).
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum GraphInvariantError {
    #[error("duplicate node id: {0:?}")]
    DuplicateNodeId(NodeId),

    #[error("reference at {site:?} points to absent node {node:?}")]
    DanglingReference { site: ReferenceSite, node: NodeId },

    #[error("node {node:?} declares parent {parent:?}, which is not present in this artifact")]
    InvalidParent { node: NodeId, parent: NodeId },

    #[error("node {node:?} is a workspace root with no parent, which artifact kind Workspace does not permit")]
    MissingWorkspaceParent { node: NodeId },

    #[error("containment cycle detected reaching node {0:?} again")]
    ContainmentCycle(NodeId),

    #[error("node {node:?} has uri {node_uri} but no matching source_origins entry")]
    SourceOriginMissingForNodeUri { node: NodeId, node_uri: Url },

    #[error("standard_library_uris contains {0} which is absent from source_origins, or is not classified StandardLibrary/Library there")]
    UnadmittedStandardLibraryUri(Url),

    #[error("build-local pending declared membership facts present at publication for node {0:?}")]
    PendingMembershipFactAtPublication(NodeId),

    #[error("evaluation facts are non-empty but evaluation_publication is {0:?}")]
    EvaluationFactsInconsistentWithPublication(EvaluationPublicationState),

    #[error("non-finite evaluated real ({value}) for node {node:?}")]
    NonFiniteEvaluatedReal { node: NodeId, value: f64 },

    #[error("invalid range for node {0:?}: start position is after end position")]
    InvalidRange(NodeId),

    #[error("declaring_uri {declaring_uri} on an edge from {edge_source:?} does not match the source node's own uri {source_uri}")]
    EdgeDeclaringUriMismatch {
        edge_source: NodeId,
        source_uri: Url,
        declaring_uri: Url,
    },

    #[error("declared expression rooted at node {node:?} exceeds the maximum nesting depth ({depth} > {limit})")]
    ExpressionTooDeep {
        node: NodeId,
        depth: usize,
        limit: usize,
    },

    #[error("declared expression rooted at node {node:?} exceeds the maximum node count ({count} > {limit})")]
    ExpressionTooLarge {
        node: NodeId,
        count: usize,
        limit: usize,
    },

    #[error("string field on node {node:?} exceeds the maximum byte length ({len} > {limit})")]
    StringTooLong {
        node: NodeId,
        len: usize,
        limit: usize,
    },

    #[error("{collection:?} has {count} entries, exceeding the active host limit of {limit}")]
    TooManyRecords {
        collection: BoundedCollection,
        count: usize,
        limit: usize,
    },
}

/// The single cache-import validator owned by the semantic layer
/// (`ROUNDTRIP_SEMGRAPH_PREREQS.md` B7, `UNIFY_CACHE_PLAN.md` §7.5). Rejects the graph on the
/// first invariant violation found; callers treat any `Err` as
/// `workspace::cache::api::CacheMissReason::InvariantFailure`, never as a model diagnostic.
///
/// This validates the *runtime* graph (`SemanticGraphData`) directly, independent of any cache
/// record encoding, so it can run both immediately after a fresh full/incremental build and again
/// after a future `SemanticGraphRecordV1` decoder reconstructs a runtime graph from bytes
/// (`ROUNDTRIP_SEMGRAPH_PREREQS.md` §5 import flow steps 3 and 6 both call this same function).
pub fn validate_graph_invariants(
    data: &SemanticGraphData,
    artifact: GraphArtifactKind,
    limits: &GraphInvariantLimits,
) -> Result<(), GraphInvariantError> {
    // --- Resource limits first: cheap counts, and refuses to do further O(n) work over an
    // oversized structure (ROUNDTRIP_SEMGRAPH_PREREQS.md B10). ---
    check_count(
        data.graph.node_count(),
        limits.max_nodes,
        BoundedCollection::Nodes,
    )?;
    check_count(
        data.graph.edge_count(),
        limits.max_edges,
        BoundedCollection::Edges,
    )?;
    check_count(
        data.effective_facts_by_node_id.len(),
        limits.max_effective_facts,
        BoundedCollection::EffectiveFacts,
    )?;
    check_count(
        data.evaluation_facts_by_node_id.len(),
        limits.max_evaluation_facts,
        BoundedCollection::EvaluationFacts,
    )?;
    check_count(
        data.derived_relationship_resolution_by_source_id.len(),
        limits.max_derived_relationship_resolutions,
        BoundedCollection::DerivedRelationshipResolutions,
    )?;
    check_count(
        data.pending_relationships.len(),
        limits.max_pending_relationships,
        BoundedCollection::PendingRelationships,
    )?;
    check_count(
        data.pending_expression_relationships.len(),
        limits.max_pending_expression_relationships,
        BoundedCollection::PendingExpressionRelationships,
    )?;

    // --- Build-local handoff must never survive to publication. ---
    if let Some((node, _)) = data.pending_declared_membership_facts.iter().next() {
        return Err(GraphInvariantError::PendingMembershipFactAtPublication(
            node.clone(),
        ));
    }

    // --- Duplicate node IDs, per-node string/range checks, and the id set used by every
    // reference check below. ---
    let mut node_ids: HashSet<&NodeId> = HashSet::with_capacity(data.graph.node_count());
    for node in data.graph.node_weights() {
        if !node_ids.insert(&node.id) {
            return Err(GraphInvariantError::DuplicateNodeId(node.id.clone()));
        }
        // `TextPosition` derives no `Ord`; compare its (line, character) fields directly rather
        // than adding an ordering contract to a source-fidelity type this blocker does not own.
        let start = (node.range.start.line, node.range.start.character);
        let end = (node.range.end.line, node.range.end.character);
        if end < start {
            return Err(GraphInvariantError::InvalidRange(node.id.clone()));
        }
        check_string(&node.id.qualified_name, node, limits)?;
        check_string(&node.name, node, limits)?;
        if let Some(declared_name) = &node.declared_name {
            check_string(declared_name, node, limits)?;
        }

        // Iterative expression bounds (ROUNDTRIP_SEMGRAPH_PREREQS.md B10): every DeclaredExpression
        // root reachable from this node is walked with an explicit stack, never recursion.
        if let Some(value) = &node.declared_facts.feature_value {
            check_expression(&node.id, &value.expression, limits)?;
        }
        if let Some(own_expression) = &node.declared_facts.own_expression {
            check_expression(&node.id, own_expression, limits)?;
        }
        if let Some(multiplicity) = &node.declared_facts.multiplicity {
            if let Some(lower) = &multiplicity.lower {
                check_expression(&node.id, lower, limits)?;
            }
            if let Some(upper) = &multiplicity.upper {
                check_expression(&node.id, upper, limits)?;
            }
        }
    }

    // --- Parent relationships: presence, and (for Workspace artifacts) requiredness. ---
    for node in data.graph.node_weights() {
        match &node.parent_id {
            Some(parent_id) => {
                if !node_ids.contains(parent_id) {
                    return Err(GraphInvariantError::InvalidParent {
                        node: node.id.clone(),
                        parent: parent_id.clone(),
                    });
                }
            }
            None => {
                // A top-level element (its qualified name carries no `::` package separator) is
                // legitimately parentless in every artifact kind -- it is a workspace/library
                // root by construction, not an omission. A *nested* qualified name with no parent
                // is only legitimate for a deliberately extracted `LibrarySubgraph`: a full
                // workspace build always resolves a nested element's containing package.
                let is_nested = node.id.qualified_name.contains("::");
                if is_nested && artifact == GraphArtifactKind::Workspace {
                    return Err(GraphInvariantError::MissingWorkspaceParent {
                        node: node.id.clone(),
                    });
                }
            }
        }
    }
    detect_containment_cycles(data, &node_ids)?;

    // --- Source-origin classification agrees with node URIs, and standard-library URIs are all
    // admitted through source_origins. ---
    for node in data.graph.node_weights() {
        if !data.source_origins.contains_key(&node.id.uri) {
            return Err(GraphInvariantError::SourceOriginMissingForNodeUri {
                node: node.id.clone(),
                node_uri: node.id.uri.clone(),
            });
        }
    }
    for uri in &data.standard_library_uris {
        match data.source_origins.get(uri) {
            Some(SourceRole::StandardLibrary) | Some(SourceRole::Library) => {}
            _ => {
                return Err(GraphInvariantError::UnadmittedStandardLibraryUri(
                    uri.clone(),
                ))
            }
        }
    }

    // --- Edges: endpoints, and detail declaring_uri normalization. ---
    for edge_ref in data.graph.edge_references() {
        let Some(source_node) = data.graph.node_weight(edge_ref.source()) else {
            continue;
        };
        let Some(target_node) = data.graph.node_weight(edge_ref.target()) else {
            continue;
        };
        if !node_ids.contains(&source_node.id) {
            return Err(GraphInvariantError::DanglingReference {
                site: ReferenceSite::EdgeEndpoint,
                node: source_node.id.clone(),
            });
        }
        if !node_ids.contains(&target_node.id) {
            return Err(GraphInvariantError::DanglingReference {
                site: ReferenceSite::EdgeEndpoint,
                node: target_node.id.clone(),
            });
        }
        let edge = edge_ref.weight();
        if let Some(connect) = &edge.connect {
            if connect.declaring_uri != source_node.id.uri {
                return Err(GraphInvariantError::EdgeDeclaringUriMismatch {
                    edge_source: source_node.id.clone(),
                    source_uri: source_node.id.uri.clone(),
                    declaring_uri: connect.declaring_uri.clone(),
                });
            }
        }
        if let Some(flow) = &edge.flow {
            if flow.declaring_uri != source_node.id.uri {
                return Err(GraphInvariantError::EdgeDeclaringUriMismatch {
                    edge_source: source_node.id.clone(),
                    source_uri: source_node.id.uri.clone(),
                    declaring_uri: flow.declaring_uri.clone(),
                });
            }
        }
    }

    // --- Effective facts: keys and internal NodeId references resolve. ---
    for (node_id, facts) in &data.effective_facts_by_node_id {
        if !node_ids.contains(node_id) {
            return Err(GraphInvariantError::DanglingReference {
                site: ReferenceSite::EffectiveFactsKey,
                node: node_id.clone(),
            });
        }
        if let Some(featuring_type) = &facts.featuring_type {
            if !node_ids.contains(featuring_type) {
                return Err(GraphInvariantError::DanglingReference {
                    site: ReferenceSite::EffectiveFactsFeaturingType,
                    node: featuring_type.clone(),
                });
            }
        }
        if let Some(binding) = &facts.implied_feature_value_binding {
            if !node_ids.contains(&binding.expression_result.owner_id) {
                return Err(GraphInvariantError::DanglingReference {
                    site: ReferenceSite::EffectiveFactsExpressionOwner,
                    node: binding.expression_result.owner_id.clone(),
                });
            }
        }
    }

    // --- Evaluation facts: keys resolve, non-finite reals rejected, publication consistency. ---
    for (node_id, facts) in &data.evaluation_facts_by_node_id {
        if !node_ids.contains(node_id) {
            return Err(GraphInvariantError::DanglingReference {
                site: ReferenceSite::EvaluationFactsKey,
                node: node_id.clone(),
            });
        }
        check_evaluation_facts_finite(node_id, facts)?;
    }
    if !data.evaluation_facts_by_node_id.is_empty()
        && data.evaluation_publication == EvaluationPublicationState::NotRun
    {
        return Err(
            GraphInvariantError::EvaluationFactsInconsistentWithPublication(
                data.evaluation_publication,
            ),
        );
    }

    // --- Derived relationship resolutions: keys and any target/candidate NodeIds resolve. ---
    for (node_id, resolution) in &data.derived_relationship_resolution_by_source_id {
        if !node_ids.contains(node_id) {
            return Err(GraphInvariantError::DanglingReference {
                site: ReferenceSite::DerivedRelationshipResolutionKey,
                node: node_id.clone(),
            });
        }
        let targets: Vec<&NodeId> = match resolution {
            DerivedRelationshipResolution::Resolved { target }
            | DerivedRelationshipResolution::SelfTargetSuppressed { target } => vec![target],
            DerivedRelationshipResolution::Ambiguous { candidates } => candidates.iter().collect(),
            DerivedRelationshipResolution::NotRun
            | DerivedRelationshipResolution::NotApplicable
            | DerivedRelationshipResolution::MissingPrerequisite { .. } => Vec::new(),
        };
        for target in targets {
            if !node_ids.contains(target) {
                return Err(GraphInvariantError::DanglingReference {
                    site: ReferenceSite::DerivedRelationshipResolutionTarget,
                    node: target.clone(),
                });
            }
        }
    }

    Ok(())
}

fn check_count(
    count: usize,
    limit: usize,
    collection: BoundedCollection,
) -> Result<(), GraphInvariantError> {
    if count > limit {
        return Err(GraphInvariantError::TooManyRecords {
            collection,
            count,
            limit,
        });
    }
    Ok(())
}

fn check_string(
    value: &str,
    node: &crate::semantic::model::SemanticNode,
    limits: &GraphInvariantLimits,
) -> Result<(), GraphInvariantError> {
    if value.len() > limits.max_string_bytes {
        return Err(GraphInvariantError::StringTooLong {
            node: node.id.clone(),
            len: value.len(),
            limit: limits.max_string_bytes,
        });
    }
    Ok(())
}

fn check_evaluation_facts_finite(
    node_id: &NodeId,
    facts: &crate::semantic::model::NodeEvaluationFacts,
) -> Result<(), GraphInvariantError> {
    let check = |value: &Option<EvaluatedValue>| -> Result<(), GraphInvariantError> {
        if let Some(EvaluatedValue::Real(real)) = value {
            if !real.is_finite() {
                return Err(GraphInvariantError::NonFiniteEvaluatedReal {
                    node: node_id.clone(),
                    value: *real,
                });
            }
        }
        Ok(())
    };
    if let Some(expression) = &facts.expression {
        check(&expression.value)?;
    }
    if let Some(analysis) = &facts.analysis {
        check(&analysis.expression.value)?;
        check(&analysis.computed_value)?;
    }
    Ok(())
}

/// Walks a `DeclaredExpression` tree with an explicit heap stack (never recursion), mirroring
/// `DeclaredExpression`'s own custom iterative `Drop` (`ROUNDTRIP_SEMGRAPH_PREREQS.md` B10): a
/// recursive validator over adversarially deep input would reintroduce exactly the stack-overflow
/// risk that `Drop` impl exists to avoid. Returns as soon as either bound is exceeded.
fn check_expression(
    node_id: &NodeId,
    root: &DeclaredExpression,
    limits: &GraphInvariantLimits,
) -> Result<(), GraphInvariantError> {
    let (depth, count) = expression_depth_and_count(root, limits.max_expression_depth);
    if depth > limits.max_expression_depth {
        return Err(GraphInvariantError::ExpressionTooDeep {
            node: node_id.clone(),
            depth,
            limit: limits.max_expression_depth,
        });
    }
    if count > limits.max_expression_nodes {
        return Err(GraphInvariantError::ExpressionTooLarge {
            node: node_id.clone(),
            count,
            limit: limits.max_expression_nodes,
        });
    }
    Ok(())
}

/// Iterative (stack-free) depth/node-count walk of a `DeclaredExpression` tree. Once `depth_limit`
/// is exceeded on a branch, that branch's descendants stop being pushed (so hostile input cannot
/// force unbounded work once rejection is already certain), but the reported `depth` still
/// reflects how deep the walk got before bailing, so the caller can report the exact bound
/// violated.
fn expression_depth_and_count(root: &DeclaredExpression, depth_limit: usize) -> (usize, usize) {
    // Explicit worklist of (node, depth) pairs, heap-allocated rather than recursive call frames.
    let mut stack: Vec<(&DeclaredExpression, usize)> = vec![(root, 1)];
    let mut max_depth = 0usize;
    let mut count = 0usize;
    while let Some((node, depth)) = stack.pop() {
        count += 1;
        max_depth = max_depth.max(depth);
        if depth > depth_limit {
            continue;
        }
        for child in &node.children {
            stack.push((child, depth + 1));
        }
        for argument in &node.arguments {
            stack.push((&argument.value, depth + 1));
        }
    }
    (max_depth, count)
}

/// Detects containment cycles by walking each node's `parent_id` chain with an explicit visited
/// set, bounded by the total node count so a cycle cannot cause unbounded iteration.
fn detect_containment_cycles(
    data: &SemanticGraphData,
    node_ids: &HashSet<&NodeId>,
) -> Result<(), GraphInvariantError> {
    let parent_of: HashMap<&NodeId, &NodeId> = data
        .graph
        .node_weights()
        .filter_map(|node| node.parent_id.as_ref().map(|parent| (&node.id, parent)))
        .collect();

    for node in data.graph.node_weights() {
        let mut visited: HashSet<&NodeId> = HashSet::new();
        let mut current = &node.id;
        visited.insert(current);
        while let Some(parent) = parent_of.get(current) {
            if !node_ids.contains(*parent) {
                // Dangling parent is reported by the separate InvalidParent check; stop walking
                // this chain rather than double-reporting.
                break;
            }
            if !visited.insert(parent) {
                return Err(GraphInvariantError::ContainmentCycle(node.id.clone()));
            }
            current = parent;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::graph::{insert_canonical, SemanticGraphData};
    use crate::semantic::model::{
        ConstructionOwner, DeclaredExpressionKind, DeclaredFeatureValue, DeclaredFeatureValueKind,
        DeclaredSemanticFacts, ElementKind, EvaluatedValue, EvaluationStatus, ExpressionEvaluation,
        ExpressionResultId, ExpressionResultRole, ImpliedFeatureValueBinding, NodeEvaluationFacts,
        RelationshipKind, SemanticEdge, SemanticNode,
    };
    use crate::semantic::text_span::{TextPosition, TextRange};
    use std::collections::HashMap;

    fn uri(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    fn range() -> TextRange {
        TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1))
    }

    fn leaf_expression() -> DeclaredExpression {
        DeclaredExpression {
            kind: DeclaredExpressionKind::IntegerLiteral,
            range: range(),
            literal: Some(crate::semantic::model::DeclaredLiteral::Integer(1)),
            reference: None,
            operator: None,
            children: Vec::new(),
            arguments: Vec::new(),
        }
    }

    /// Builds a chain of `depth` nested `Parenthesized` expressions, used to construct input that
    /// would overflow a recursive validator.
    fn nested_expression(depth: usize) -> DeclaredExpression {
        let mut current = leaf_expression();
        for _ in 0..depth {
            current = DeclaredExpression {
                kind: DeclaredExpressionKind::Parenthesized,
                range: range(),
                literal: None,
                reference: None,
                operator: None,
                children: vec![current],
                arguments: Vec::new(),
            };
        }
        current
    }

    fn plain_node(id: NodeId, parent_id: Option<NodeId>) -> SemanticNode {
        SemanticNode {
            id,
            element_kind: ElementKind::Package,
            declared_name: None,
            name: "n".to_string(),
            range: range(),
            attributes: HashMap::new(),
            declared_facts: DeclaredSemanticFacts::default(),
            source_text: Default::default(),
            expression_text: Default::default(),
            parent_id,
        }
    }

    fn add_node(data: &mut SemanticGraphData, node: SemanticNode) -> NodeId {
        let id = node.id.clone();
        let node_uri = id.uri.clone();
        let index = data.graph.add_node(node);
        data.node_index_by_id.insert(id.clone(), index);
        data.nodes_by_uri
            .entry(node_uri)
            .or_default()
            .push(id.clone());
        insert_canonical(
            data.node_ids_by_qualified_name
                .entry(id.qualified_name.clone())
                .or_default(),
            id.clone(),
        );
        id
    }

    /// A single-document workspace graph with a root package and one child feature, admitted as
    /// a `Workspace` source. Every invariant check should pass against this fixture.
    fn valid_workspace_graph() -> (SemanticGraphData, NodeId, NodeId) {
        let mut data = SemanticGraphData::new();
        let doc = uri("file:///ws/a.sysml");
        data.source_origins
            .insert(doc.clone(), SourceRole::Workspace);
        let root_id = NodeId::new(&doc, "Root");
        add_node(&mut data, plain_node(root_id.clone(), None));
        let mut child = plain_node(NodeId::new(&doc, "Root::Child"), Some(root_id.clone()));
        child.declared_facts.feature_value = Some(DeclaredFeatureValue {
            kind: DeclaredFeatureValueKind::Bound,
            expression: leaf_expression(),
            range: range(),
        });
        let child_id = add_node(&mut data, child);
        let source_idx = data.node_index_by_id[&root_id];
        let target_idx = data.node_index_by_id[&child_id];
        data.graph.add_edge(
            source_idx,
            target_idx,
            SemanticEdge::plain(
                RelationshipKind::Typing,
                ConstructionOwner::DocumentConstruction,
            ),
        );
        (data, root_id, child_id)
    }

    fn assert_ok(data: &SemanticGraphData) {
        assert_eq!(
            validate_graph_invariants(
                data,
                GraphArtifactKind::Workspace,
                &GraphInvariantLimits::default(),
            ),
            Ok(())
        );
    }

    #[test]
    fn valid_graph_passes() {
        let (data, _root, _child) = valid_workspace_graph();
        assert_ok(&data);
    }

    #[test]
    fn rejects_duplicate_node_id() {
        let (mut data, root_id, _child) = valid_workspace_graph();
        // Insert a second node weight carrying the same NodeId without going through add_node's
        // bookkeeping, simulating a corrupted decode that produced two nodes with one identity.
        data.graph.add_node(plain_node(root_id, None));
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(err, GraphInvariantError::DuplicateNodeId(_)));
    }

    #[test]
    fn rejects_dangling_reference_from_stale_effective_facts() {
        let (mut data, _root_id, child_id) = valid_workspace_graph();
        // Simulate the realistic corruption shape: a fact surviving deletion of the node it
        // referenced, rather than an edge (petgraph structurally cannot hold an edge whose
        // endpoint index does not exist).
        let doc = child_id.uri.clone();
        let ghost_id = NodeId::new(&doc, "Root::Ghost");
        data.effective_facts_by_node_id.insert(
            ghost_id,
            crate::semantic::model::EffectiveSemanticFacts::default(),
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::DanglingReference {
                site: ReferenceSite::EffectiveFactsKey,
                ..
            }
        ));
    }

    #[test]
    fn rejects_invalid_parent() {
        let (mut data, _root_id, _child_id) = valid_workspace_graph();
        let doc = uri("file:///ws/a.sysml");
        let missing_parent = NodeId::new(&doc, "Root::Nowhere");
        add_node(
            &mut data,
            plain_node(NodeId::new(&doc, "Root::Orphan"), Some(missing_parent)),
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(err, GraphInvariantError::InvalidParent { .. }));
    }

    #[test]
    fn rejects_containment_cycle() {
        let mut data = SemanticGraphData::new();
        let doc = uri("file:///ws/a.sysml");
        data.source_origins
            .insert(doc.clone(), SourceRole::Workspace);
        let a_id = NodeId::new(&doc, "A");
        let b_id = NodeId::new(&doc, "B");
        // A's parent is B, B's parent is A: a two-node cycle.
        add_node(&mut data, plain_node(a_id.clone(), Some(b_id.clone())));
        add_node(&mut data, plain_node(b_id, Some(a_id)));
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(err, GraphInvariantError::ContainmentCycle(_)));
    }

    #[test]
    fn rejects_source_origin_disagreeing_with_node_uri() {
        let (mut data, _root, _child) = valid_workspace_graph();
        let other_doc = uri("file:///ws/b.sysml");
        add_node(
            &mut data,
            plain_node(NodeId::new(&other_doc, "Stray"), None),
        );
        // `other_doc` was never added to source_origins, so this node's uri disagrees with the
        // admitted source set.
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::LibrarySubgraph,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::SourceOriginMissingForNodeUri { .. }
        ));
    }

    #[test]
    fn rejects_standard_library_uri_absent_from_admitted_sources() {
        let (mut data, _root, _child) = valid_workspace_graph();
        let stdlib_doc = uri("file:///stdlib/Base.sysml");
        // Declared as a standard-library source without a matching admitted source_origins entry.
        data.standard_library_uris.insert(stdlib_doc);
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::UnadmittedStandardLibraryUri(_)
        ));
    }

    #[test]
    fn rejects_dangling_evaluation_fact_key() {
        let (mut data, _root, child_id) = valid_workspace_graph();
        let doc = child_id.uri.clone();
        let ghost = NodeId::new(&doc, "Root::NeverAdded");
        data.evaluation_facts_by_node_id
            .insert(ghost, NodeEvaluationFacts::default());
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::DanglingReference {
                site: ReferenceSite::EvaluationFactsKey,
                ..
            }
        ));
    }

    #[test]
    fn rejects_dangling_derived_resolution_target() {
        let (mut data, root_id, child_id) = valid_workspace_graph();
        let doc = child_id.uri.clone();
        let ghost = NodeId::new(&doc, "Root::NeverAdded");
        data.derived_relationship_resolution_by_source_id.insert(
            root_id,
            DerivedRelationshipResolution::Resolved { target: ghost },
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::DanglingReference {
                site: ReferenceSite::DerivedRelationshipResolutionTarget,
                ..
            }
        ));
    }

    #[test]
    fn rejects_dangling_effective_fact_featuring_type() {
        let (mut data, root_id, child_id) = valid_workspace_graph();
        let doc = child_id.uri.clone();
        let ghost = NodeId::new(&doc, "Root::NeverAdded");
        let facts = crate::semantic::model::EffectiveSemanticFacts {
            featuring_type: Some(ghost),
            ..Default::default()
        };
        data.effective_facts_by_node_id.insert(root_id, facts);
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::DanglingReference {
                site: ReferenceSite::EffectiveFactsFeaturingType,
                ..
            }
        ));
    }

    #[test]
    fn rejects_dangling_effective_fact_expression_owner() {
        let (mut data, root_id, child_id) = valid_workspace_graph();
        let doc = child_id.uri.clone();
        let ghost = NodeId::new(&doc, "Root::NeverAdded");
        let facts = crate::semantic::model::EffectiveSemanticFacts {
            implied_feature_value_binding: Some(ImpliedFeatureValueBinding {
                expression_result: ExpressionResultId {
                    owner_id: ghost,
                    role: ExpressionResultRole::FeatureValue,
                },
            }),
            ..Default::default()
        };
        data.effective_facts_by_node_id.insert(root_id, facts);
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::DanglingReference {
                site: ReferenceSite::EffectiveFactsExpressionOwner,
                ..
            }
        ));
    }

    #[test]
    fn rejects_pending_membership_fact_at_publication() {
        let (mut data, root_id, _child) = valid_workspace_graph();
        data.pending_declared_membership_facts.insert(
            root_id,
            crate::semantic::model::DeclaredMembershipFacts::default(),
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::PendingMembershipFactAtPublication(_)
        ));
    }

    #[test]
    fn rejects_evaluation_facts_inconsistent_with_publication() {
        let (mut data, root_id, _child) = valid_workspace_graph();
        // Facts present, but publication left at its default NotRun.
        data.evaluation_facts_by_node_id
            .insert(root_id, NodeEvaluationFacts::default());
        assert_eq!(
            data.evaluation_publication,
            EvaluationPublicationState::NotRun
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::EvaluationFactsInconsistentWithPublication(_)
        ));
    }

    #[test]
    fn rejects_non_finite_evaluated_real() {
        let (mut data, root_id, _child) = valid_workspace_graph();
        data.evaluation_publication = EvaluationPublicationState::Complete;
        data.evaluation_facts_by_node_id.insert(
            root_id,
            NodeEvaluationFacts {
                expression: Some(ExpressionEvaluation {
                    status: EvaluationStatus::Ok,
                    value: Some(EvaluatedValue::Real(f64::NAN)),
                    unit: None,
                    error: None,
                }),
                analysis: None,
            },
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::NonFiniteEvaluatedReal { .. }
        ));
    }

    #[test]
    fn rejects_invalid_range() {
        let mut data = SemanticGraphData::new();
        let doc = uri("file:///ws/a.sysml");
        data.source_origins
            .insert(doc.clone(), SourceRole::Workspace);
        let mut node = plain_node(NodeId::new(&doc, "Backwards"), None);
        node.range = TextRange::new(TextPosition::new(5, 0), TextPosition::new(1, 0));
        add_node(&mut data, node);
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(err, GraphInvariantError::InvalidRange(_)));
    }

    #[test]
    fn rejects_edge_declaring_uri_mismatch() {
        let (mut data, root_id, child_id) = valid_workspace_graph();
        let other_doc = uri("file:///ws/other.sysml");
        let root_idx = data.node_index_by_id[&root_id];
        let child_idx = data.node_index_by_id[&child_id];
        let detail = crate::semantic::model::ConnectStatementDetail {
            declaring_uri: other_doc,
            range: range(),
            source_expression: "a".to_string(),
            target_expression: "b".to_string(),
            container_prefix: None,
            is_interface_usage: false,
            interface_type: None,
        };
        data.graph.add_edge(
            root_idx,
            child_idx,
            SemanticEdge::connection_with_connect(detail, ConstructionOwner::DocumentConstruction),
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::EdgeDeclaringUriMismatch { .. }
        ));
    }

    #[test]
    fn library_subgraph_permits_absent_workspace_parent_but_workspace_rejects_it() {
        let mut data = SemanticGraphData::new();
        let doc = uri("file:///lib/Base.sysml");
        data.source_origins.insert(doc.clone(), SourceRole::Library);
        // A library root deliberately has no parent: it was extracted without its would-be
        // workspace ancestor package.
        add_node(
            &mut data,
            plain_node(NodeId::new(&doc, "Base::LibRoot"), None),
        );

        assert_eq!(
            validate_graph_invariants(
                &data,
                GraphArtifactKind::LibrarySubgraph,
                &GraphInvariantLimits::default(),
            ),
            Ok(())
        );
        let err = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::MissingWorkspaceParent { .. }
        ));
    }

    #[test]
    fn rejects_too_many_nodes() {
        let (data, _root, _child) = valid_workspace_graph();
        let tiny_limits = GraphInvariantLimits {
            max_nodes: 1,
            ..GraphInvariantLimits::default()
        };
        let err = validate_graph_invariants(&data, GraphArtifactKind::Workspace, &tiny_limits)
            .unwrap_err();
        assert!(matches!(
            err,
            GraphInvariantError::TooManyRecords {
                collection: BoundedCollection::Nodes,
                ..
            }
        ));
    }

    #[test]
    fn rejects_string_too_long() {
        let mut data = SemanticGraphData::new();
        let doc = uri("file:///ws/a.sysml");
        data.source_origins
            .insert(doc.clone(), SourceRole::Workspace);
        let mut node = plain_node(NodeId::new(&doc, "Root"), None);
        node.name = "x".repeat(64);
        add_node(&mut data, node);
        let tiny_limits = GraphInvariantLimits {
            max_string_bytes: 8,
            ..GraphInvariantLimits::default()
        };
        let err = validate_graph_invariants(&data, GraphArtifactKind::Workspace, &tiny_limits)
            .unwrap_err();
        assert!(matches!(err, GraphInvariantError::StringTooLong { .. }));
    }

    /// A deeply-nested expression (well beyond typical native stack recursion limits) is rejected
    /// by the depth bound rather than crashing the validator, proving `check_expression`'s
    /// iterative walk never recurses.
    #[test]
    fn rejects_deeply_nested_expression_without_stack_overflow() {
        let mut data = SemanticGraphData::new();
        let doc = uri("file:///ws/a.sysml");
        data.source_origins
            .insert(doc.clone(), SourceRole::Workspace);
        let mut node = plain_node(NodeId::new(&doc, "Deep"), None);
        node.declared_facts.feature_value = Some(DeclaredFeatureValue {
            kind: DeclaredFeatureValueKind::Bound,
            expression: nested_expression(200_000),
            range: range(),
        });
        add_node(&mut data, node);
        let tiny_limits = GraphInvariantLimits {
            max_expression_depth: 4_096,
            ..GraphInvariantLimits::default()
        };
        let err = validate_graph_invariants(&data, GraphArtifactKind::Workspace, &tiny_limits)
            .unwrap_err();
        assert!(matches!(err, GraphInvariantError::ExpressionTooDeep { .. }));
    }

    /// A rejected graph never produces a `sysml_diagnostics`-shaped value: `GraphInvariantError`
    /// carries no diagnostic code, severity, or source range, and is only ever surfaced through
    /// `Result::Err`, never appended to a diagnostics collection. This asserts the type-level
    /// separation the module doc comment describes: nothing about this error can be mistaken for
    /// (or silently converted into) a user-facing model diagnostic.
    #[test]
    fn separates_from_user_diagnostics() {
        let (mut data, root_id, _child) = valid_workspace_graph();
        data.graph.add_node(plain_node(root_id, None));
        let result = validate_graph_invariants(
            &data,
            GraphArtifactKind::Workspace,
            &GraphInvariantLimits::default(),
        );
        match result {
            Err(GraphInvariantError::DuplicateNodeId(_)) => {}
            other => panic!("expected DuplicateNodeId, got {other:?}"),
        }
        // `GraphInvariantError` has no `code()`/`severity()`/`range()` accessor and does not
        // implement any `sysml_diagnostics` diagnostic trait -- there is no path from this type
        // into a diagnostics collection. The compile-time absence of such members is the
        // assertion; this test documents that intent alongside the typed-variant checks above.
    }
}
