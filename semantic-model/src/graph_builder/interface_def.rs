//! Interface and connection definition bodies: `end`, `ref`, `connect`.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ConnectStmt, ConnectionDefBodyElement, EndDecl, InterfaceDefBodyElement, RefDecl,
};
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{
    add_edge_if_both_exist, add_typing_edge_if_exists, normalize_for_lookup,
};

use super::expressions;
use super::{add_node_and_recurse, qualified_name_for_node};

fn add_end_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    wrap: &sysml_v2_parser::Node<EndDecl>,
) {
    let n = &wrap.value;
    let range = span_to_range(&wrap.span);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "interface end");
    let mut attrs = HashMap::new();
    attrs.insert("endType".to_string(), serde_json::json!(&n.type_name));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "interface end",
        n.name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
}

fn add_ref_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    wrap: &sysml_v2_parser::Node<RefDecl>,
) {
    let n = &wrap.value;
    let range = span_to_range(&wrap.span);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "ref");
    let mut attrs = HashMap::new();
    attrs.insert("refType".to_string(), serde_json::json!(&n.type_name));
    if let Some(ref v) = n.value {
        attrs.insert(
            "value".to_string(),
            serde_json::json!(expressions::expression_to_debug_string(v)),
        );
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "ref",
        n.name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
}

fn add_connect_stmt(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    wrap: &sysml_v2_parser::Node<ConnectStmt>,
) {
    let n = &wrap.value;
    expressions::add_expression_edge_if_both_exist(
        g,
        uri,
        container_prefix,
        &n.from,
        &n.to,
        RelationshipKind::Connection,
    );
}

fn maybe_add_derivation_edge(g: &mut SemanticGraph, uri: &Url, parent_id: &NodeId) {
    let Some(parent) = g.get_node(parent_id) else {
        return;
    };
    if parent
        .attributes
        .get("connectionAnnotation")
        .and_then(|value| value.as_str())
        != Some("derivation")
    {
        return;
    }
    let Some(original_target) = g
        .child_named(parent_id, "#original")
        .into_iter()
        .next()
        .and_then(|node| node.attributes.get("endType"))
        .and_then(|value| value.as_str())
    else {
        return;
    };
    let Some(derived_target) = g
        .child_named(parent_id, "#derive")
        .into_iter()
        .next()
        .and_then(|node| node.attributes.get("endType"))
        .and_then(|value| value.as_str())
    else {
        return;
    };
    let original_target = normalize_for_lookup(original_target);
    let derived_target = normalize_for_lookup(derived_target);
    let _ = add_edge_if_both_exist(
        g,
        uri,
        &original_target,
        &derived_target,
        RelationshipKind::Derivation,
    );
    if let Some(parent) = g.get_node_mut(parent_id) {
        parent.attributes.insert(
            "derivationOriginal".to_string(),
            serde_json::json!(original_target),
        );
        parent.attributes.insert(
            "derivationDerived".to_string(),
            serde_json::json!(derived_target),
        );
    }
}

fn add_connection_edges_from_end_typing(g: &mut SemanticGraph, uri: &Url, parent_id: &NodeId) {
    let Some(parent) = g.get_node(parent_id) else {
        return;
    };
    let scope_prefix = parent
        .parent_id
        .as_ref()
        .map(|id| id.qualified_name.as_str());
    let mut end_targets: Vec<String> = g
        .children_of(parent)
        .into_iter()
        .filter(|child| child.element_kind == "interface end")
        .filter_map(|child| {
            g.outgoing_targets_by_kind(child, RelationshipKind::Typing)
                .into_iter()
                .next()
                .map(|target| target.id.qualified_name.clone())
                .or_else(|| {
                    child
                        .attributes
                        .get("endType")
                        .and_then(|value| value.as_str())
                        .and_then(|end_type| {
                            expressions::resolve_expression_endpoint_legacy(
                                g,
                                uri,
                                scope_prefix,
                                end_type,
                            )
                        })
                })
        })
        .collect();
    if end_targets.len() < 2 {
        return;
    }
    let mut seen = std::collections::HashSet::new();
    end_targets.retain(|target| seen.insert(target.clone()));
    if end_targets.len() < 2 {
        return;
    }

    // Binary connections emit one edge; n-ary connections use the first end as hub.
    let source = end_targets[0].clone();
    for target in end_targets.into_iter().skip(1) {
        let _ = add_edge_if_both_exist(g, uri, &source, &target, RelationshipKind::Connection);
    }
}

pub(super) fn build_from_interface_def_body_element(
    node: &sysml_v2_parser::Node<InterfaceDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::InterfaceDefBodyElement as E;
    match &node.value {
        E::Doc(_) => {}
        E::EndDecl(w) => add_end_decl(g, uri, container_prefix, parent_id, w),
        E::RefDecl(w) => add_ref_decl(g, uri, container_prefix, parent_id, w),
        E::ConnectStmt(w) => add_connect_stmt(g, uri, container_prefix, w),
    }
}

pub(super) fn build_from_connection_def_body_element(
    node: &sysml_v2_parser::Node<ConnectionDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::ConnectionDefBodyElement as E;
    match &node.value {
        E::EndDecl(w) => {
            add_end_decl(g, uri, container_prefix, parent_id, w);
            maybe_add_derivation_edge(g, uri, parent_id);
        }
        E::RefDecl(w) => add_ref_decl(g, uri, container_prefix, parent_id, w),
        E::ConnectStmt(w) => add_connect_stmt(g, uri, container_prefix, w),
    }
}

pub(super) fn build_from_connection_def_body(
    elements: &[sysml_v2_parser::Node<ConnectionDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    for element in elements {
        build_from_connection_def_body_element(element, uri, container_prefix, parent_id, g);
    }
    add_connection_edges_from_end_typing(g, uri, parent_id);
}
