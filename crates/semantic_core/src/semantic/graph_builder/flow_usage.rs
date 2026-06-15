//! Shared FlowUsage materialization for all structure-usage body contexts.

use std::collections::HashMap;

use sysml_v2_parser::ast::{FlowUsage, FlowUsageKind, Node};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::add_typing_edge_if_exists;

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

    if let (Some(from), Some(to)) = (&flow.from, &flow.to) {
        expressions::add_expression_edge_if_both_exist(
            g,
            uri,
            container_prefix,
            from,
            to,
            RelationshipKind::Flow,
        );
    }
}
