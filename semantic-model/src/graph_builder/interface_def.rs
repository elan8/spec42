//! Interface and connection definition bodies: `end`, `ref`, `connect`.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ConnectStmt, ConnectionDefBodyElement, EndDecl, InterfaceDefBodyElement, RefDecl,
};
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::add_typing_edge_if_exists;

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
        E::EndDecl(w) => add_end_decl(g, uri, container_prefix, parent_id, w),
        E::RefDecl(w) => add_ref_decl(g, uri, container_prefix, parent_id, w),
        E::ConnectStmt(w) => add_connect_stmt(g, uri, container_prefix, w),
    }
}
