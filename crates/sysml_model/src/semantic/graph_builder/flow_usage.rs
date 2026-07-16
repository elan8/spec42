//! Shared FlowUsage materialization for all structure-usage body contexts.

use std::collections::HashMap;

use sysml_v2_parser::ast::{FlowUsage, FlowUsageKind, Node};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{FlowStatementDetail, NodeId, RelationshipKind, SemanticEdge};
use crate::semantic::relationships::{add_semantic_edge_once, add_typing_edge_if_exists};

use super::definition_body;
use super::expressions::{self, expression_to_debug_string};
use super::{add_node_and_recurse, qualified_name_for_node};

fn flow_kind_label(kind: FlowUsageKind) -> &'static str {
    match kind {
        FlowUsageKind::Flow => "flow",
        FlowUsageKind::Message => "message",
        FlowUsageKind::SuccessionFlow => "successionFlow",
    }
}

pub(super) fn materialize_flow_usage(
    flow_node: &Node<FlowUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let flow = &flow_node.value;
    let mut attrs = HashMap::new();
    attrs.insert(
        "flowKind".to_string(),
        serde_json::json!(flow_kind_label(flow.kind)),
    );
    if let Some(ref type_name) = flow.type_name {
        attrs.insert("flowType".to_string(), serde_json::json!(type_name));
    }
    if let Some(ref payload) = flow.payload {
        attrs.insert(
            "payloadExpression".to_string(),
            serde_json::json!(expression_to_debug_string(payload)),
        );
    }
    if let Some(ref from) = flow.from {
        attrs.insert(
            "sourceExpression".to_string(),
            serde_json::json!(expression_to_debug_string(from)),
        );
    }
    if let Some(ref to) = flow.to {
        attrs.insert(
            "targetExpression".to_string(),
            serde_json::json!(expression_to_debug_string(to)),
        );
    }

    if let Some(ref name) = flow.name {
        let qualified = qualified_name_for_node(g, uri, container_prefix, name, "flow");
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "flow",
            name.clone(),
            span_to_range(&flow_node.span),
            attrs,
            Some(parent_id),
        );
        if let Some(ref type_name) = flow.type_name {
            add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
        }
        let node_id = NodeId::new(uri, &qualified);
        definition_body::build_from_definition_body(&flow.body, uri, Some(&qualified), &node_id, g);
    }

    add_flow_edge_if_both_exist(g, uri, container_prefix, flow);
}

/// `RelationshipKind` for a resolved flow edge: `SuccessionFlow` gets its own kind (it implies an
/// ordering constraint, not just data/control flow); plain `Flow` and `Message` share the
/// existing generic `Flow` kind -- `flow.kind`'s full distinction is retained in
/// `FlowStatementDetail`/the `flowKind` display attribute above regardless.
fn relationship_kind_for_flow(kind: FlowUsageKind) -> RelationshipKind {
    match kind {
        FlowUsageKind::Flow | FlowUsageKind::Message => RelationshipKind::Flow,
        FlowUsageKind::SuccessionFlow => RelationshipKind::SuccessionFlow,
    }
}

/// Resolve `flow`'s `from`/`to` endpoints and add a `Flow`/`SuccessionFlow` edge carrying a
/// [`FlowStatementDetail`] (payload/source/target text), mirroring how `Connection` edges carry
/// [`crate::semantic::model::ConnectStatementDetail`]. Endpoint resolution is the existing
/// "legacy" (non-strict) path already used for `Flow` edges before this detail was added, not
/// the strict `Connection`-only resolver -- unresolved endpoints are silently skipped, matching
/// prior behavior for this edge kind.
fn add_flow_edge_if_both_exist(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    flow: &FlowUsage,
) {
    let (Some(from), Some(to)) = (&flow.from, &flow.to) else {
        return;
    };
    let from_str = expressions::expr_node_to_qualified_string(from);
    let to_str = expressions::expr_node_to_qualified_string(to);
    if from_str.is_empty() || to_str.is_empty() {
        return;
    }
    let Some(src) = expressions::resolve_expression_endpoint_legacy(g, uri, container_prefix, &from_str)
    else {
        return;
    };
    let Some(tgt) = expressions::resolve_expression_endpoint_legacy(g, uri, container_prefix, &to_str)
    else {
        return;
    };
    let detail = FlowStatementDetail {
        declaring_uri: uri.clone(),
        range: span_to_range(&from.span),
        payload_expression: flow.payload.as_ref().map(expression_to_debug_string),
        source_expression: Some(from_str),
        target_expression: Some(to_str),
    };
    add_semantic_edge_once(
        g,
        &NodeId::new(uri, &src),
        &NodeId::new(uri, &tgt),
        SemanticEdge::flow_with_detail(relationship_kind_for_flow(flow.kind), detail),
    );
}
