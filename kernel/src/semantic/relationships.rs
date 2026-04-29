//! Relationship edge logic: typing, specializes, connection, bind, cross-document resolution.

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement};
use sysml_v2_parser::RootNamespace;

use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::identification_name;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::import_resolution::resolve_type_reference_targets;
use crate::semantic::model::{NodeId, RelationshipKind, SemanticNode};
pub(crate) use crate::semantic::resolution::naming::{
    normalize_for_lookup, type_ref_candidates, type_ref_candidates_with_kind,
};
use crate::root_element_body;

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
    "use case def",
    "concern def",
    "enum def",
    "alias",
    // KerML modeled declarations (`datatype`, `class`, â€¦) from `.kerml` / library sources.
    "kermlDecl",
];

const SPECIALIZES_TARGET_KINDS: &[&str] = &["part def"];

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

fn resolve_type_target_local(
    g: &SemanticGraph,
    uri: &Url,
    type_ref: &str,
    container_prefix: Option<&str>,
    allowed_target_kinds: &[&str],
) -> Option<NodeId> {
    // 1) Try deterministic candidate qualified names (plain + #kind variants).
    for suffix_kind in DISAMBIGUATION_SUFFIX_KINDS {
        for candidate in type_ref_candidates_with_kind(container_prefix, type_ref, suffix_kind) {
            let tgt_key = normalize_for_lookup(&candidate);
            let tgt_id = NodeId::new(uri, &tgt_key);
            if let Some(tgt) = g.get_node(&tgt_id) {
                if element_kind_allowed(tgt.element_kind.as_str(), allowed_target_kinds) {
                    return Some(tgt_id);
                }
            }
        }
    }

    // 2) Fallback: find by short name within the same document (best-effort).
    // This is intentionally conservative: only matches allowed definitional kinds,
    // and returns the shortest qualified name to prefer nearer scopes.
    let mut best: Option<NodeId> = None;
    for n in g.nodes_for_uri(uri) {
        if !element_kind_allowed(n.element_kind.as_str(), allowed_target_kinds) {
            continue;
        }
        if n.name != type_ref {
            continue;
        }
        let candidate = n.id.clone();
        let take = match &best {
            None => true,
            Some(current) => candidate.qualified_name.len() < current.qualified_name.len(),
        };
        if take {
            best = Some(candidate);
        }
    }
    best
}

/// Returns true if the edge was added.
pub(crate) fn add_edge_if_both_exist(
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
    g.graph.add_edge(src_idx, tgt_idx, kind);
    true
}

pub fn resolve_pending_relationships_for_uri(g: &mut SemanticGraph, uri: &Url) {
    let pending = std::mem::take(&mut g.pending_relationships);
    for pending_edge in pending {
        if &pending_edge.uri != uri {
            g.pending_relationships.push(pending_edge);
            continue;
        }
        let source_id = NodeId::new(uri, &pending_edge.source_qualified);
        let target_id = NodeId::new(uri, &pending_edge.target_qualified);
        let (Some(&src_idx), Some(tgt_node), Some(&tgt_idx)) = (
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
        g.graph.add_edge(src_idx, tgt_idx, pending_edge.kind);
    }
}

/// Adds a typing edge if source exists and target can be resolved. Tries type_ref as-is,
/// then qualified with package prefixes, then #kind-suffixed variants for disambiguated nodes.
/// Only matches targets that are actual types (part def, port def, interface, requirement def) to avoid
/// matching a package that shares the same name.
pub(crate) fn add_typing_edge_if_exists(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    type_ref: &str,
    container_prefix: Option<&str>,
) {
    let normalized_type_ref = normalize_declared_type_ref(type_ref);
    if normalized_type_ref.is_empty() {
        return;
    }
    if let Some(target_id) = resolve_type_target_local(
        g,
        uri,
        &normalized_type_ref,
        container_prefix,
        TYPING_TARGET_KINDS,
    ) {
        let target_qualified = target_id.qualified_name.clone();
        let _ = add_edge_if_both_exist_opt(
            g,
            uri,
            source_qualified,
            &target_qualified,
            RelationshipKind::Typing,
            Some(TYPING_TARGET_KINDS),
        );
    }
}

/// Adds a specializes edge if source exists and target can be resolved. Same resolution as typing:
/// specializes target may be unqualified (e.g. "SurveillanceQuadrotorDrone") while the node
/// has qualified name (e.g. "SurveillanceDrone::SurveillanceQuadrotorDrone").
/// Only matches PartDef targets to avoid matching a package.
pub(crate) fn add_specializes_edge_if_exists(
    g: &mut SemanticGraph,
    uri: &Url,
    source_qualified: &str,
    specializes_ref: &str,
    container_prefix: Option<&str>,
) {
    for normalized in split_specializes_refs(specializes_ref) {
        if let Some(target_id) = resolve_type_target_local(
            g,
            uri,
            &normalized,
            container_prefix,
            SPECIALIZES_TARGET_KINDS,
        ) {
            let target_qualified = target_id.qualified_name.clone();
            let _ = add_edge_if_both_exist_opt(
                g,
                uri,
                source_qualified,
                &target_qualified,
                RelationshipKind::Specializes,
                Some(SPECIALIZES_TARGET_KINDS),
            );
        }
    }
}

/// Finds a PartDef in the root by qualified name by walking PackageBodyElements.
pub(crate) fn find_part_def_in_root<'a>(
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

pub(crate) fn find_part_def_in_elements<'a>(
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

/// Adds typing/specializes edges from nodes in the given URI to targets that may be in other files.
/// Called after merge so the full graph contains nodes from all documents.
pub fn add_cross_document_edges_for_uri(g: &mut SemanticGraph, uri: &Url) {
    let edges = resolve_cross_document_edges_for_uri(g, uri);
    for (src_id, tgt_id, kind) in edges {
        if let (Some(&src_idx), Some(&tgt_idx)) = (
            g.node_index_by_id.get(&src_id),
            g.node_index_by_id.get(&tgt_id),
        ) {
            g.graph.add_edge(src_idx, tgt_idx, kind);
        }
    }
}

/// Resolves typing/specializes edges from nodes in the given URI to targets that may be in other files.
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
        for key in [
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
        ] {
            if let Some(type_ref) = node.attributes.get(key).and_then(|v| v.as_str()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::graph_builder::build_graph_from_doc;
    use sysml_v2_parser::parse;

    #[test]
    fn specializes_refs_from_value_supports_string_and_array() {
        let from_string = specializes_refs_from_value(&serde_json::json!("BaseA, BaseB"));
        assert_eq!(from_string, vec!["BaseA".to_string(), "BaseB".to_string()]);

        let from_array = specializes_refs_from_value(&serde_json::json!(["BaseA", "BaseB, BaseC"]));
        assert_eq!(
            from_array,
            vec![
                "BaseA".to_string(),
                "BaseB".to_string(),
                "BaseC".to_string()
            ]
        );
    }

    #[test]
    fn resolve_cross_document_specializes_handles_multi_base_array_and_dedupes() {
        let lib_src = r#"
            package Lib {
                part def BaseA {}
            }
        "#;
        let app_src = r#"
            package App {
                import Lib::*;
                part def Child :> BaseA {}
            }
        "#;

        let lib_uri = Url::parse("file:///lib.sysml").expect("lib uri");
        let app_uri = Url::parse("file:///app.sysml").expect("app uri");
        let lib_root = parse(lib_src).expect("parse lib");
        let app_root = parse(app_src).expect("parse app");
        let mut graph = build_graph_from_doc(&lib_root, &lib_uri);
        graph.merge(build_graph_from_doc(&app_root, &app_uri));

        let child_id = graph
            .nodes_for_uri(&app_uri)
            .into_iter()
            .find(|node| node.element_kind == "part def" && node.name == "Child")
            .map(|node| node.id.clone())
            .expect("child node id");
        graph
            .get_node_mut(&child_id)
            .expect("child node")
            .attributes
            .insert(
                "specializes".to_string(),
                serde_json::json!(["BaseA", "BaseA", "MissingBase"]),
            );

        let resolved_edges = resolve_cross_document_edges_for_uri(&graph, &app_uri);
        let specialize_edges: Vec<_> = resolved_edges
            .into_iter()
            .filter(|(_, _, kind)| *kind == RelationshipKind::Specializes)
            .collect();

        assert_eq!(
            specialize_edges.len(),
            1,
            "expected one deduped specializes edge to BaseA"
        );
        let (_, target_id, _) = &specialize_edges[0];
        assert!(
            target_id.qualified_name.ends_with("Lib::BaseA"),
            "expected specializes edge to Lib::BaseA, got {:?}",
            target_id
        );
    }
}
