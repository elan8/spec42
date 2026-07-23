//! Relationship edge logic: typing, specializes, connection, bind, workspace relationship linking.

use petgraph::visit::EdgeRef;
use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, TypingRelationship};
use sysml_v2_parser::{Node, RootNamespace};

use url::Url;

use crate::semantic::ast_util::identification_name;
use crate::semantic::graph::{PendingExpressionRelationship, SemanticGraph, SemanticGraphData};
use crate::semantic::import_resolution::{
    import_target, is_import_all, normalized_membership_target, normalized_namespace_target,
    resolve_type_reference_targets,
};
use crate::semantic::kinds::{
    self, element_kind_allowed, SUBJECT_TYPE_TARGET_KINDS, VERIFIED_REQUIREMENT_TARGET_KINDS,
};
pub use crate::semantic::kinds::{
    ANNOTATED_ELEMENT_TARGET_KINDS, SPECIALIZES_TARGET_KINDS, TYPING_TARGET_KINDS,
};
use crate::semantic::model::{
    ConnectStatementDetail, ElementKind, NodeId, RelationshipKind, SemanticEdge, SemanticNode,
};
use crate::semantic::reference_resolution::{
    resolve_expression_endpoint_strict, resolve_inherited_member_via_type, ResolveResult,
};
pub use crate::semantic::resolution::naming::{
    normalize_declared_type_ref, normalize_for_lookup, type_ref_candidates,
    type_ref_candidates_with_kind,
};
use crate::semantic::root_element::root_element_body;

/// Uniform view of an explicit parser relationship target or a legacy textual target.
/// The graph resolver consumes the target only at this boundary; builders retain typed AST facts.
pub trait TypeReferenceTarget {
    fn type_reference_target(&self) -> &str;
}

impl TypeReferenceTarget for str {
    fn type_reference_target(&self) -> &str {
        self
    }
}

impl TypeReferenceTarget for String {
    fn type_reference_target(&self) -> &str {
        self
    }
}

impl TypeReferenceTarget for TypingRelationship {
    fn type_reference_target(&self) -> &str {
        self.target
            .first()
            .and_then(|target| target.value.local_name())
            .unwrap_or_default()
    }
}

impl TypeReferenceTarget for Node<TypingRelationship> {
    fn type_reference_target(&self) -> &str {
        self.value
            .target
            .first()
            .and_then(|target| target.value.local_name())
            .unwrap_or_default()
    }
}

pub const TYPE_REFERENCE_ATTR_KEYS: &[&str] = &[
    "partType",
    "refType",
    "attributeType",
    "portType",
    "actionType",
    "actorType",
    "itemType",
    "occurrenceType",
    "flowType",
    "allocationType",
    "stateType",
    "requirementType",
    "useCaseType",
    "concernType",
    "viewType",
    "viewpointType",
    "renderingType",
    "subjectType",
    "analysisType",
    "verificationType",
    "connectionType",
    "metadataType",
    "keywordType",
];

fn split_specializes_refs(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(normalize_declared_type_ref)
        .filter(|item| !item.is_empty())
        .collect()
}

fn specializes_refs_from_value(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::String(raw) => split_specializes_refs(raw),
        serde_json::Value::Array(items) => items
            .iter()
            .filter_map(|item| item.as_str())
            .flat_map(split_specializes_refs)
            .collect(),
        _ => Vec::new(),
    }
}

/// Resolve a `subsetsFeature` / `redefines` attribute value to a target node id.
/// Prefers a direct qualified-name hit, then inherited-member resolution via the owner.
fn resolve_subsets_or_redefines_target(
    g: &SemanticGraph,
    owner: Option<&SemanticNode>,
    attribute_value: &str,
) -> Option<NodeId> {
    let qualified = attribute_value.replace('.', "::");
    if let Some(node_ids) = g.node_ids_by_qualified_name.get(&qualified) {
        if let Some(id) = node_ids.first() {
            return Some(id.clone());
        }
    }
    let owner = owner?;
    let member = attribute_value
        .split("::")
        .last()
        .unwrap_or(attribute_value);
    match resolve_inherited_member_via_type(g, owner, member) {
        ResolveResult::Resolved(target_id) => Some(target_id),
        _ => None,
    }
}

/// Wires every subsetting-family clause on a node to a real edge: `subsets`/`:>`, `redefines`/
/// `:>>`, and -- KerML 8.3.4.4/8.3.4.5 `ReferenceSubsetting`/`CrossSubsetting`, S42-002 --
/// `references`/`::>` and `crosses`/`=>`. All four resolve identically (same attribute-value ->
/// target lookup), only the resulting `RelationshipKind` differs.
fn link_subsetting_family_edges_for_node(g: &mut SemanticGraph, node_id: &NodeId) {
    let Some(node) = g.get_node(node_id).cloned() else {
        return;
    };
    let owner = node
        .parent_id
        .as_ref()
        .and_then(|pid| g.get_node(pid))
        .cloned();
    for (attribute_key, kind) in [
        ("subsetsFeature", RelationshipKind::Subsetting),
        ("redefines", RelationshipKind::Redefinition),
        ("referencesFeature", RelationshipKind::ReferenceSubsetting),
        ("crossesFeature", RelationshipKind::CrossSubsetting),
    ] {
        if let Some(attr) = node
            .attributes
            .get(attribute_key)
            .and_then(|value| value.as_str())
        {
            if let Some(target_id) = resolve_subsets_or_redefines_target(g, owner.as_ref(), attr) {
                add_semantic_edge_once(g, node_id, &target_id, SemanticEdge::plain(kind));
            }
        }
    }
}

mod cross_document;
mod derivation;
mod pending;
mod subject;
pub use cross_document::*;
pub use derivation::*;
pub use pending::*;
pub use subject::*;

pub fn add_edge_if_both_exist(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    target_qualified: &str,
    kind: RelationshipKind,
) -> bool {
    add_edge_if_both_exist_opt(g, uri, source_qualified, target_qualified, kind, None)
}

/// Like add_edge_if_both_exist but for typing/specializes: only adds when target is a type
/// (part def, port def, interface). Avoids matching a package that shares the same name.
fn add_edge_if_both_exist_opt(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    target_qualified: &str,
    kind: RelationshipKind,
    target_kinds: Option<&[ElementKind]>,
) -> bool {
    let src_key = normalize_for_lookup(source_qualified);
    let tgt_key = normalize_for_lookup(target_qualified);
    let src_id = NodeId::new(uri, &src_key);
    let tgt_id = NodeId::new(uri, &tgt_key);
    let (Some(&src_idx), Some(tgt_node)) = (g.node_index_by_id.get(&src_id), g.get_node(&tgt_id))
    else {
        g.pending_relationships
            .push(crate::semantic::graph::PendingRelationship {
                uri: uri.clone(),
                source_qualified: src_key,
                target_qualified: tgt_key,
                kind,
                target_kinds: target_kinds.map(|kinds| kinds.to_vec()),
            });
        return false;
    };
    if let Some(kinds) = target_kinds {
        if !kinds.contains(&tgt_node.element_kind) {
            return false;
        }
    }
    let Some(tgt_idx) = g.node_index_by_id.get(&tgt_id).copied() else {
        return false;
    };
    g.graph
        .add_edge(src_idx, tgt_idx, SemanticEdge::plain(kind));
    true
}

/// Adds an edge when no same-kind edge exists between the pair.
/// For `Connection` edges with connect metadata, returns [`AddSemanticEdgeResult::DuplicateConnect`]
/// when an identical connect statement already exists.
///
/// Fan-out/fan-in to the same resolved port node (for example four `connect` statements from
/// `flightController.motorCmd` to `propulsion.propulsionUnitN.cmd` that all resolve through
/// inherited definition ports) still records separate parallel connection edges so each
/// statement keeps its own endpoint expressions for interconnection rendering.
pub fn add_semantic_edge_once(
    g: &mut SemanticGraph,
    source_id: &NodeId,
    target_id: &NodeId,
    edge: SemanticEdge,
) -> AddSemanticEdgeResult {
    let (Some(&src_idx), Some(&tgt_idx)) = (
        g.node_index_by_id.get(source_id),
        g.node_index_by_id.get(target_id),
    ) else {
        return AddSemanticEdgeResult::SkippedSameKind;
    };
    if edge.kind == RelationshipKind::Connection {
        if let Some(connect) = edge.connect.clone() {
            // Collect edge data before mutating — borrow checker requires immutable scan
            // to complete before any mutable access to the same graph field.
            #[derive(Clone)]
            enum ExistingConnectState {
                NoConnect(petgraph::graph::EdgeIndex),
                Duplicate,
                Other,
            }
            let mut saw_connection = false;
            let mut action = None;
            for existing in g.graph.edges_connecting(src_idx, tgt_idx) {
                if existing.weight().kind != RelationshipKind::Connection {
                    continue;
                }
                saw_connection = true;
                if existing.weight().connect.is_none() {
                    action = Some(ExistingConnectState::NoConnect(existing.id()));
                    break;
                }
                if let Some(existing_connect) = &existing.weight().connect {
                    if existing_connect.source_expression == connect.source_expression
                        && existing_connect.target_expression == connect.target_expression
                        && existing_connect.container_prefix == connect.container_prefix
                    {
                        action = Some(ExistingConnectState::Duplicate);
                        break;
                    }
                }
                action = Some(ExistingConnectState::Other);
            }
            match action {
                Some(ExistingConnectState::NoConnect(eid)) => {
                    if let Some(weight) = g.graph.edge_weight_mut(eid) {
                        weight.connect = Some(connect);
                    }
                    return AddSemanticEdgeResult::Added;
                }
                Some(ExistingConnectState::Duplicate) => {
                    return AddSemanticEdgeResult::DuplicateConnect;
                }
                _ => {}
            }
            if saw_connection {
                g.graph.add_edge(src_idx, tgt_idx, edge);
                return AddSemanticEdgeResult::Added;
            }
            g.graph.add_edge(src_idx, tgt_idx, edge);
            return AddSemanticEdgeResult::Added;
        }
        for existing in g.graph.edges_connecting(src_idx, tgt_idx) {
            if existing.weight().kind == edge.kind {
                return AddSemanticEdgeResult::SkippedSameKind;
            }
        }
        g.graph.add_edge(src_idx, tgt_idx, edge);
        return AddSemanticEdgeResult::Added;
    }
    for existing in g.graph.edges_connecting(src_idx, tgt_idx) {
        if existing.weight().kind == edge.kind {
            return AddSemanticEdgeResult::SkippedSameKind;
        }
    }
    g.graph.add_edge(src_idx, tgt_idx, edge);
    AddSemanticEdgeResult::Added
}

/// Wire `annotatedElement` links from a metadata usage to its explicit `about` targets or owner.
pub fn wire_metadata_annotated_elements(
    g: &mut SemanticGraph,
    uri: &Url,
    metadata_id: &NodeId,
    owner_id: &NodeId,
    about_targets: &[String],
) {
    let Some(metadata_node) = g.get_node(metadata_id).cloned() else {
        return;
    };
    if about_targets.is_empty() {
        add_semantic_edge_once(
            g,
            metadata_id,
            owner_id,
            SemanticEdge::plain(RelationshipKind::Annotation),
        );
        let _ = uri;
        return;
    }
    for target_ref in about_targets {
        let Some(target_id) = resolve_type_target_in_workspace(
            g,
            &metadata_node,
            target_ref,
            ANNOTATED_ELEMENT_TARGET_KINDS,
        ) else {
            continue;
        };
        add_semantic_edge_once(
            g,
            metadata_id,
            &target_id,
            SemanticEdge::plain(RelationshipKind::Annotation),
        );
    }
    let _ = uri;
}

/// Adds a typing edge if source exists and target can be resolved. Tries type_ref as-is,
/// then qualified with package prefixes, then #kind-suffixed variants for disambiguated nodes.
/// Only matches targets that are actual types (part def, port def, interface, requirement def) to avoid
/// matching a package that shares the same name.
pub fn add_typing_edge_if_exists<T: TypeReferenceTarget + ?Sized>(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    type_ref: &T,
    container_prefix: Option<&str>,
) {
    let source_id = NodeId::new(uri, normalize_for_lookup(source_qualified));
    if g.get_node(&source_id).is_none() {
        return;
    }
    let _ = container_prefix;
    add_typing_edge_for_node(g, &source_id, type_ref.type_reference_target());
}

/// Adds a specializes edge if source exists and target can be resolved. Same resolution as typing:
/// specializes target may be unqualified (e.g. "SurveillanceQuadrotorDrone") while the node
/// has qualified name (e.g. "SurveillanceDrone::SurveillanceQuadrotorDrone").
/// Only matches definition targets (part def, requirement def, …) to avoid matching a package.
pub fn add_specializes_edge_if_exists<T: TypeReferenceTarget + ?Sized>(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    specializes_ref: &T,
    container_prefix: Option<&str>,
) {
    let source_id = NodeId::new(uri, normalize_for_lookup(source_qualified));
    if g.get_node(&source_id).is_none() {
        return;
    }
    let _ = container_prefix;
    add_specializes_edges_for_node(g, &source_id, specializes_ref.type_reference_target());
}

pub fn add_typing_edge_for_node(g: &mut SemanticGraph, source_id: &NodeId, type_ref: &str) {
    let Some(source_node) = g.get_node(source_id).cloned() else {
        return;
    };
    // `~P` (port conjugation, KerML 8.3.12.3 / SysML v2 8.4.8.2: `port p : ~Pd;` is equivalent
    // to `port p : Pd::'~Pd';`) resolves through the `ConjugatedPortDefinition` nested in `P`,
    // not `P` itself. Handled here, centrally, so both callers get it: the immediate
    // per-document resolution in `graph_builder/port_def.rs` AND the deferred cross-document
    // `link_workspace_relationships` pass (which re-resolves every node's `portType` attribute
    // through this same function) -- a bespoke fix at only one call site would have been
    // silently overwritten by the other.
    if let Some(base_ref) = type_ref.trim().strip_prefix('~') {
        let conjugate_target_id = resolve_type_target_in_workspace(
            g,
            &source_node,
            base_ref.trim(),
            &[ElementKind::PortDef],
        )
        .and_then(|id| g.get_node(&id).cloned())
        .and_then(|base_node| {
            g.children_of(&base_node)
                .into_iter()
                .find(|child| child.element_kind == ElementKind::ConjugatedPortDefinition)
                .map(|child| child.id.clone())
        });
        if let Some(target_id) = conjugate_target_id {
            add_semantic_edge_once(
                g,
                source_id,
                &target_id,
                SemanticEdge::plain(RelationshipKind::Typing),
            );
            return;
        }
        // Base not resolvable yet (e.g. cross-document forward reference) -- fall through to
        // the generic path below, matching pre-existing behavior for unresolvable references.
    }
    let Some(target_id) =
        resolve_type_target_in_workspace(g, &source_node, type_ref, TYPING_TARGET_KINDS)
    else {
        return;
    };
    add_semantic_edge_once(
        g,
        source_id,
        &target_id,
        SemanticEdge::plain(RelationshipKind::Typing),
    );
}

pub fn add_specializes_edges_for_node(
    g: &mut SemanticGraph,
    source_id: &NodeId,
    specializes_ref: &str,
) {
    let Some(source_node) = g.get_node(source_id).cloned() else {
        return;
    };
    for normalized in split_specializes_refs(specializes_ref) {
        let Some(target_id) = resolve_type_target_in_workspace(
            g,
            &source_node,
            &normalized,
            SPECIALIZES_TARGET_KINDS,
        ) else {
            continue;
        };
        add_semantic_edge_once(
            g,
            source_id,
            &target_id,
            SemanticEdge::plain(RelationshipKind::Specializes),
        );
    }
}

pub fn link_workspace_relationships(g: &mut SemanticGraph) {
    let node_ids: Vec<NodeId> = g.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        let Some(node) = g.get_node(&node_id).cloned() else {
            continue;
        };
        for key in TYPE_REFERENCE_ATTR_KEYS {
            if let Some(type_ref) = node.attributes.get(*key).and_then(|value| value.as_str()) {
                add_typing_edge_for_node(g, &node_id, type_ref);
            }
        }
        let specializes_refs = node
            .attributes
            .get("specializes")
            .map(specializes_refs_from_value)
            .unwrap_or_default();
        for specializes_ref in specializes_refs {
            add_specializes_edges_for_node(g, &node_id, &specializes_ref);
        }
        link_subsetting_family_edges_for_node(g, &node_id);
    }

    // Per-document graph build cannot see imported elements from other files; re-wire after merge.
    let connection_ids: Vec<NodeId> = g
        .node_index_by_id
        .keys()
        .filter(|node_id| {
            g.get_node(node_id)
                .map(|node| node.element_kind == ElementKind::DerivationConnection)
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    for connection_id in connection_ids {
        try_wire_derivation_connection(g, &connection_id.uri, &connection_id);
    }

    link_case_subject_relationships(g);
}

/// Wire derivation connections after a full parallel cross-document edge resolution.
///
/// When [`resolve_cross_document_edges_for_uri`] is run in parallel for every URI in
/// the workspace, it already resolves typing, specializes, and subject edges for all
/// nodes.  In that full-build path, calling [`link_workspace_relationships`] afterwards
/// redundantly re-resolves those same edges for all 1 681+ nodes.  Use this slimmer
/// variant at the full-build call sites to skip the redundant loops and only wire the
/// one thing the parallel phase does not cover: derivation connections.
///
/// The incremental update path (single-file change) still needs the full
/// [`link_workspace_relationships`] because only one URI's edges were refreshed.
pub fn link_workspace_derivations(g: &mut SemanticGraph) {
    let connection_ids: Vec<NodeId> = g
        .node_index_by_id
        .keys()
        .filter(|node_id| {
            g.get_node(node_id)
                .map(|node| node.element_kind == ElementKind::DerivationConnection)
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    for connection_id in connection_ids {
        try_wire_derivation_connection(g, &connection_id.uri, &connection_id);
    }
    // Full parallel builds resolve typing/specializes/subject in
    // `resolve_cross_document_edges_for_uri`; subsetting/redefinition still need
    // a whole-graph pass after merge (same shape as derivation rewiring).
    let node_ids: Vec<NodeId> = g.node_index_by_id.keys().cloned().collect();
    for node_id in node_ids {
        link_subsetting_family_edges_for_node(g, &node_id);
    }
}
