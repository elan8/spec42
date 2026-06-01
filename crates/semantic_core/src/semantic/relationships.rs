//! Relationship edge logic: typing, specializes, connection, bind, workspace relationship linking.

use petgraph::visit::EdgeRef;
use sysml_v2_parser::ast::{PackageBody, PackageBodyElement};
use sysml_v2_parser::RootNamespace;

use url::Url;

use crate::semantic::ast_util::identification_name;
use crate::semantic::graph::{PendingExpressionRelationship, SemanticGraph};
use crate::semantic::import_resolution::resolve_type_reference_targets;
use crate::semantic::model::{
    ConnectStatementDetail, NodeId, RelationshipKind, SemanticEdge, SemanticNode,
};
use crate::semantic::reference_resolution::{resolve_expression_endpoint_strict, ResolveResult};
pub use crate::semantic::resolution::naming::{
    normalize_for_lookup, type_ref_candidates, type_ref_candidates_with_kind,
};
use crate::semantic::root_element::root_element_body;

const TYPING_TARGET_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "requirement",
    "use case def",
    "concern def",
    "enum def",
    "alias",
    // KerML modeled declarations (`datatype`, `class`, ...) from `.kerml` / library sources.
    "kermlDecl",
];

/// Definitional kinds that may appear as the target of `:>` / `specializes` on definitions.
/// Aligns with typing/RULE7 definitional kinds plus definition-only element kinds from the graph builder.
const SPECIALIZES_TARGET_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "enum def",
    "alias",
    "kermlDecl",
    "individual def",
    "connection def",
    "metadata def",
    "constraint def",
    "calc def",
    "case def",
    "analysis def",
    "verification def",
    "view def",
    "viewpoint def",
    "rendering def",
];
pub const TYPE_REFERENCE_ATTR_KEYS: &[&str] = &[
    "partType",
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
    "subjectType",
    "analysisType",
    "verificationType",
];

/// Canonical set of #kind suffixes that `qualified_name_for_node` may append.
/// Note: these are suffix spellings, not element_kind strings.
const DISAMBIGUATION_SUFFIX_KINDS: &[&str] = &[
    "part_def",
    "port_def",
    "action_def",
    "state_def",
    "flow_def",
    "allocation_def",
    "requirement_def",
    "use_case_def",
    "attribute_def",
    "enum_def",
    "item_def",
    "actor_def",
    "occurrence_def",
    "interface",
    "concern_def",
    "alias",
    "kermlDecl",
];

fn normalize_declared_type_ref(type_ref: &str) -> String {
    type_ref
        .trim()
        .strip_prefix('~')
        .map(str::trim)
        .unwrap_or(type_ref.trim())
        .to_string()
}

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

fn element_kind_allowed(element_kind: &str, allowed_kinds: &[&str]) -> bool {
    allowed_kinds.contains(&element_kind)
}

pub fn resolve_type_target_in_workspace(
    g: &SemanticGraph,
    context_node: &SemanticNode,
    type_ref: &str,
    allowed_target_kinds: &[&str],
) -> Option<NodeId> {
    let normalized_type_ref = normalize_declared_type_ref(type_ref);
    if normalized_type_ref.is_empty() {
        return None;
    }
    resolve_type_reference_targets(g, context_node, &normalized_type_ref, allowed_target_kinds)
        .into_iter()
        .next()
}

/// Returns true if the edge was added.
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
    target_kinds: Option<&[&str]>,
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
                target_kinds: target_kinds
                    .map(|kinds| kinds.iter().map(|kind| kind.to_string()).collect()),
            });
        return false;
    };
    if let Some(kinds) = target_kinds {
        let ek = tgt_node.element_kind.as_str();
        if !kinds.contains(&ek) {
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

pub(crate) fn add_pending_expression_relationship(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    source_expression: &str,
    target_expression: &str,
    kind: RelationshipKind,
    source_range: crate::semantic::text_span::TextRange,
) {
    g.pending_expression_relationships
        .push(PendingExpressionRelationship {
            uri: uri.clone(),
            source_expression: source_expression.to_string(),
            target_expression: target_expression.to_string(),
            kind,
            container_prefix: container_prefix.map(ToString::to_string),
            source_range,
        });
}

/// Re-resolve pending relationship queues for every document in the merged graph.
///
/// Must run after [`link_workspace_relationships`] so typing edges needed for member-chain
/// connection endpoints are available.
pub fn resolve_workspace_pending_relationships(g: &mut SemanticGraph) {
    const MAX_PASSES: usize = 8;
    for _ in 0..MAX_PASSES {
        let pending_before = g.pending_relationships.len() + g.pending_expression_relationships.len();
        if pending_before == 0 {
            break;
        }
        let uris: Vec<Url> = g.nodes_by_uri.keys().cloned().collect();
        for uri in uris {
            resolve_pending_relationships_for_uri(g, &uri);
        }
        let pending_after = g.pending_relationships.len() + g.pending_expression_relationships.len();
        if pending_after == pending_before {
            break;
        }
    }
}

pub fn resolve_pending_relationships_for_uri(g: &mut SemanticGraph, uri: &Url) {
    resolve_pending_expression_relationships_for_uri(g, uri);

    let pending = std::mem::take(&mut g.pending_relationships);
    for pending_edge in pending {
        if &pending_edge.uri != uri {
            g.pending_relationships.push(pending_edge);
            continue;
        }
        let source_id = NodeId::new(uri, &pending_edge.source_qualified);
        let target_id = NodeId::new(uri, &pending_edge.target_qualified);
        let (Some(_), Some(tgt_node), Some(_)) = (
            g.node_index_by_id.get(&source_id),
            g.get_node(&target_id),
            g.node_index_by_id.get(&target_id),
        ) else {
            g.pending_relationships.push(pending_edge);
            continue;
        };
        if let Some(ref target_kinds) = pending_edge.target_kinds {
            if !target_kinds
                .iter()
                .any(|kind| kind == &tgt_node.element_kind)
            {
                continue;
            }
        }
        add_semantic_edge_once(
            g,
            &source_id,
            &target_id,
            SemanticEdge::plain(pending_edge.kind.clone()),
        );
    }
}

fn resolve_pending_expression_relationships_for_uri(g: &mut SemanticGraph, uri: &Url) {
    let pending = std::mem::take(&mut g.pending_expression_relationships);
    for pending_edge in pending {
        if &pending_edge.uri != uri {
            g.pending_expression_relationships.push(pending_edge);
            continue;
        }
        let source_id = match resolve_expression_endpoint_strict(
            g,
            uri,
            pending_edge.container_prefix.as_deref(),
            &pending_edge.source_expression,
        ) {
            ResolveResult::Resolved(id) => id,
            ResolveResult::Ambiguous | ResolveResult::Unresolved => {
                g.pending_expression_relationships.push(pending_edge);
                continue;
            }
        };
        let target_id = match resolve_expression_endpoint_strict(
            g,
            uri,
            pending_edge.container_prefix.as_deref(),
            &pending_edge.target_expression,
        ) {
            ResolveResult::Resolved(id) => id,
            ResolveResult::Ambiguous | ResolveResult::Unresolved => {
                g.pending_expression_relationships.push(pending_edge);
                continue;
            }
        };
        if pending_edge.kind == RelationshipKind::Connection {
            add_semantic_edge_once(
                g,
                &source_id,
                &target_id,
                SemanticEdge::connection_with_connect(ConnectStatementDetail {
                    declaring_uri: uri.clone(),
                    range: pending_edge.source_range,
                    source_expression: pending_edge.source_expression,
                    target_expression: pending_edge.target_expression,
                    container_prefix: pending_edge.container_prefix.clone(),
                }),
            );
        } else {
            add_semantic_edge_once(
                g,
                &source_id,
                &target_id,
                SemanticEdge::plain(pending_edge.kind.clone()),
            );
        }
    }
}

/// Links `#derivation connection` ends (`#original`, `#derive`) to requirement elements.
pub(crate) fn try_wire_derivation_connection(
    g: &mut SemanticGraph,
    uri: &Url,
    connection_node_id: &NodeId,
) {
    let Some(connection) = g.get_node(connection_node_id) else {
        return;
    };
    if connection
        .attributes
        .get("connectionAnnotation")
        .and_then(|value| value.as_str())
        != Some("derivation")
    {
        return;
    }
    let scope_prefix = connection
        .parent_id
        .as_ref()
        .and_then(|parent_id| g.get_node(parent_id))
        .map(|parent| parent.id.qualified_name.as_str());

    let Some(original_id) =
        resolve_derivation_end_target(g, uri, scope_prefix, connection_node_id, "#original")
    else {
        return;
    };
    let Some(derived_id) =
        resolve_derivation_end_target(g, uri, scope_prefix, connection_node_id, "#derive")
    else {
        return;
    };

    add_semantic_edge_once(
        g,
        &original_id,
        &derived_id,
        SemanticEdge::plain(RelationshipKind::Derivation),
    );
    if let Some(connection) = g.get_node_mut(connection_node_id) {
        connection.attributes.insert(
            "derivationOriginal".to_string(),
            serde_json::json!(normalize_for_lookup(&original_id.qualified_name)),
        );
        connection.attributes.insert(
            "derivationDerived".to_string(),
            serde_json::json!(normalize_for_lookup(&derived_id.qualified_name)),
        );
    }
}

fn resolve_derivation_end_target(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    connection_node_id: &NodeId,
    end_name: &str,
) -> Option<NodeId> {
    let end = g
        .child_named(connection_node_id, end_name)
        .into_iter()
        .next()?;
    if let Some(target) = g
        .outgoing_targets_by_kind(&end, RelationshipKind::Typing)
        .into_iter()
        .next()
    {
        return Some(target.id.clone());
    }
    let type_ref = end.attributes.get("endType")?.as_str()?;
    match resolve_expression_endpoint_strict(g, uri, container_prefix, type_ref) {
        ResolveResult::Resolved(id) => Some(id),
        ResolveResult::Ambiguous | ResolveResult::Unresolved => {
            resolve_type_target_in_workspace(g, end, type_ref, TYPING_TARGET_KINDS)
        }
    }
}

/// Result of attempting to add a semantic edge between two nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddSemanticEdgeResult {
    Added,
    SkippedSameKind,
    DuplicateConnect,
}

/// Adds an edge when no same-kind edge exists between the pair.
/// For `Connection` edges with connect metadata, returns [`AddSemanticEdgeResult::DuplicateConnect`]
/// when a connection edge already exists (keeps the first edge's metadata).
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
    for existing in g.graph.edges_connecting(src_idx, tgt_idx) {
        if existing.weight().kind != edge.kind {
            continue;
        }
        if edge.kind == RelationshipKind::Connection {
            if let Some(connect) = edge.connect.clone() {
                if existing.weight().connect.is_none() {
                    if let Some(weight) = g.graph.edge_weight_mut(existing.id()) {
                        weight.connect = Some(connect);
                    }
                    return AddSemanticEdgeResult::Added;
                }
                if let Some(existing_connect) = &existing.weight().connect {
                    if existing_connect.source_expression == connect.source_expression
                        && existing_connect.target_expression == connect.target_expression
                        && existing_connect.container_prefix == connect.container_prefix
                    {
                        return AddSemanticEdgeResult::DuplicateConnect;
                    }
                }
                return AddSemanticEdgeResult::SkippedSameKind;
            }
        }
        return AddSemanticEdgeResult::SkippedSameKind;
    }
    g.graph.add_edge(src_idx, tgt_idx, edge);
    AddSemanticEdgeResult::Added
}

/// Adds a typing edge if source exists and target can be resolved. Tries type_ref as-is,
/// then qualified with package prefixes, then #kind-suffixed variants for disambiguated nodes.
/// Only matches targets that are actual types (part def, port def, interface, requirement def) to avoid
/// matching a package that shares the same name.
pub fn add_typing_edge_if_exists(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    type_ref: &str,
    container_prefix: Option<&str>,
) {
    let source_id = NodeId::new(uri, normalize_for_lookup(source_qualified));
    if g.get_node(&source_id).is_none() {
        return;
    }
    let _ = container_prefix;
    add_typing_edge_for_node(g, &source_id, type_ref);
}

/// Adds a specializes edge if source exists and target can be resolved. Same resolution as typing:
/// specializes target may be unqualified (e.g. "SurveillanceQuadrotorDrone") while the node
/// has qualified name (e.g. "SurveillanceDrone::SurveillanceQuadrotorDrone").
/// Only matches definition targets (part def, requirement def, …) to avoid matching a package.
pub fn add_specializes_edge_if_exists(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    specializes_ref: &str,
    container_prefix: Option<&str>,
) {
    let source_id = NodeId::new(uri, normalize_for_lookup(source_qualified));
    if g.get_node(&source_id).is_none() {
        return;
    }
    let _ = container_prefix;
    add_specializes_edges_for_node(g, &source_id, specializes_ref);
}

pub fn add_typing_edge_for_node(g: &mut SemanticGraph, source_id: &NodeId, type_ref: &str) {
    let Some(source_node) = g.get_node(source_id).cloned() else {
        return;
    };
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
        let Some(target_id) =
            resolve_type_target_in_workspace(g, &source_node, &normalized, SPECIALIZES_TARGET_KINDS)
        else {
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
    }

    // Per-document graph build cannot see imported elements from other files; re-wire after merge.
    let connection_ids: Vec<NodeId> = g
        .node_index_by_id
        .keys()
        .filter(|node_id| {
            g.get_node(node_id)
                .map(|node| node.element_kind == "derivation connection")
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    for connection_id in connection_ids {
        try_wire_derivation_connection(g, &connection_id.uri, &connection_id);
    }
}

/// Finds a PartDef in the root by qualified name by walking PackageBodyElements.
pub fn find_part_def_in_root<'a>(
    root: &'a RootNamespace,
    qualified: &str,
) -> Option<(&'a sysml_v2_parser::Node<sysml_v2_parser::PartDef>, String)> {
    let mut prefix = String::new();
    for node in &root.elements {
        let elements = match root_element_body(&node.value) {
            Some((elements, _, _, _)) => elements,
            None => continue,
        };
        if let Some(found) = find_part_def_in_elements(elements, &mut prefix, qualified) {
            return Some(found);
        }
    }
    None
}

pub fn find_part_def_in_elements<'a>(
    elements: &'a [sysml_v2_parser::Node<PackageBodyElement>],
    prefix: &mut String,
    target: &str,
) -> Option<(&'a sysml_v2_parser::Node<sysml_v2_parser::PartDef>, String)> {
    for node in elements {
        match &node.value {
            PackageBodyElement::Package(pkg) => {
                let name = identification_name(&pkg.identification);
                let prev = std::mem::take(prefix);
                *prefix = if prev.is_empty() {
                    name.clone()
                } else {
                    format!("{}::{}", prev, name)
                };
                if let PackageBody::Brace { elements: inner } = &pkg.body {
                    if let Some(found) = find_part_def_in_elements(inner, prefix, target) {
                        return Some(found);
                    }
                }
                *prefix = prev;
            }
            PackageBodyElement::PartDef(pd) => {
                let name = identification_name(&pd.identification);
                let q = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{}::{}", prefix, name)
                };
                if q == target {
                    return Some((pd, q));
                }
            }
            _ => {}
        }
    }
    None
}

/// Legacy URI-scoped wrapper for workspace relationship linking.
/// Adds typing/specializes edges from nodes in the given URI.
/// Kept for compatibility with incremental kernel update paths.
pub fn add_cross_document_edges_for_uri(g: &mut SemanticGraph, uri: &Url) {
    let edges = resolve_cross_document_edges_for_uri(g, uri);
    for (src_id, tgt_id, kind) in edges {
        if let (Some(&src_idx), Some(&tgt_idx)) = (
            g.node_index_by_id.get(&src_id),
            g.node_index_by_id.get(&tgt_id),
        ) {
            g.graph
                .add_edge(src_idx, tgt_idx, SemanticEdge::plain(kind));
        }
    }
}

/// Legacy URI-scoped resolver for workspace relationship linking.
/// Returns a list of (source NodeId, target NodeId, relationship kind) for resolved edges.
/// This function is thread-safe and can be called in parallel across different URIs.
pub fn resolve_cross_document_edges_for_uri(
    g: &SemanticGraph,
    uri: &Url,
) -> Vec<(NodeId, NodeId, RelationshipKind)> {
    let node_ids: Vec<NodeId> = g.nodes_by_uri.get(uri).cloned().unwrap_or_default();
    let mut resolved_edges = Vec::new();
    let mut seen_edges = std::collections::HashSet::new();

    for node_id in &node_ids {
        let Some(node) = g.get_node(node_id) else {
            continue;
        };
        let prefix: Option<String> = node
            .parent_id
            .as_ref()
            .and_then(|pid| g.get_node(pid))
            .map(|p| p.id.qualified_name.clone());

        // Typing relationships
        for key in TYPE_REFERENCE_ATTR_KEYS {
            if let Some(type_ref) = node.attributes.get(*key).and_then(|v| v.as_str()) {
                if let Some(target_id) = resolve_typing_edge_cross_document_inner(
                    g,
                    node,
                    type_ref,
                    prefix.as_deref(),
                    RelationshipKind::Typing,
                ) {
                    let dedupe_key = (node_id.clone(), target_id.clone(), "typing");
                    if seen_edges.insert(dedupe_key) {
                        resolved_edges.push((node_id.clone(), target_id, RelationshipKind::Typing));
                    }
                }
            }
        }

        // Specializes relationships
        let specializes_refs = node
            .attributes
            .get("specializes")
            .map(specializes_refs_from_value)
            .unwrap_or_default();
        for specializes_ref in specializes_refs {
            if let Some(target_id) = resolve_typing_edge_cross_document_inner(
                g,
                node,
                &specializes_ref,
                prefix.as_deref(),
                RelationshipKind::Specializes,
            ) {
                let dedupe_key = (node_id.clone(), target_id.clone(), "specializes");
                if seen_edges.insert(dedupe_key) {
                    resolved_edges.push((
                        node_id.clone(),
                        target_id,
                        RelationshipKind::Specializes,
                    ));
                }
            }
        }
    }
    resolved_edges
}

fn resolve_typing_edge_cross_document_inner(
    g: &SemanticGraph,
    src_node: &SemanticNode,
    type_ref: &str,
    container_prefix: Option<&str>,
    kind: RelationshipKind,
) -> Option<NodeId> {
    let normalized_type_ref = normalize_declared_type_ref(type_ref);
    if normalized_type_ref.is_empty() {
        return None;
    }
    let target_element_kinds: &[&str] = match kind {
        RelationshipKind::Typing => TYPING_TARGET_KINDS,
        RelationshipKind::Specializes => SPECIALIZES_TARGET_KINDS,
        _ => return None,
    };

    let mut targets =
        resolve_type_reference_targets(g, src_node, &normalized_type_ref, target_element_kinds);
    if let Some(prefix) = container_prefix {
        for suffix_kind in DISAMBIGUATION_SUFFIX_KINDS {
            for candidate in
                type_ref_candidates_with_kind(Some(prefix), &normalized_type_ref, suffix_kind)
            {
                let tgt_qualified = normalize_for_lookup(&candidate);
                if let Some(target_ids) = g.node_ids_for_qualified_name(&tgt_qualified) {
                    for target_id in target_ids {
                        if let Some(target) = g.get_node(target_id) {
                            if element_kind_allowed(
                                target.element_kind.as_str(),
                                target_element_kinds,
                            ) {
                                targets.push(target_id.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    targets.into_iter().next()
}
