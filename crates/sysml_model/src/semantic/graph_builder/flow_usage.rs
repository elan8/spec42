//! Shared FlowUsage materialization for all structure-usage body contexts.

use std::collections::HashMap;

use sysml_v2_parser::ast::{FlowUsage, FlowUsageKind, Node, PayloadFeature};
use url::Url;

use crate::semantic::ast_util::{declared_multiplicity, span_to_range};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::kinds::TYPING_TARGET_KINDS;
use crate::semantic::model::{FlowStatementDetail, NodeId, RelationshipKind, SemanticEdge};
use crate::semantic::relationships::{
    add_semantic_edge_once, add_typing_edge_if_exists, resolve_type_target_in_workspace,
};

use super::definition_body;
use super::expressions::{self, expression_to_debug_string};
use super::{add_node_and_recurse, qualified_name_for_node};

/// Readable summary of a `PayloadFeature` for the `payloadExpression` display attribute --
/// e.g. `"qty : Payload"` or just `"Payload"` for the bare-type-reference form. The structured
/// facts (name/type/multiplicity) live on the real `ElementKind::FlowPayload` child node
/// (`materialize_flow_payload`) and the resolved `payload_type_id` on the edge
/// (`add_flow_edge_if_both_exist`); this is text only, matching `flowType`'s existing role.
fn payload_feature_debug_string(payload: &PayloadFeature) -> String {
    match (&payload.name, &payload.type_name) {
        (Some(name), Some(type_name)) => format!("{name} : {type_name}"),
        (Some(name), None) => name.clone(),
        (None, Some(type_name)) => type_name.clone(),
        (None, None) => String::new(),
    }
}

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
            serde_json::json!(payload_feature_debug_string(&payload.value)),
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
        if let Some(ref payload) = flow.payload {
            materialize_flow_payload(g, uri, Some(qualified.as_str()), &node_id, payload);
        }
        definition_body::build_from_definition_body(&flow.body, uri, Some(&qualified), &node_id, g);
    }

    add_flow_edge_if_both_exist(g, uri, container_prefix, parent_id, flow);
}

/// SysML v2 8.2.2.16: `of X` on a named flow materializes as a real, addressable
/// `ElementKind::FlowPayload` child feature of the flow -- name (if given, else a synthetic
/// name matching this codebase's `"_assign"`/`"_terminate"` convention for unnamed control-flow
/// children), a typing edge to the resolved type (if given), and multiplicity (if given). Per
/// spec, `Identification?` only makes the *name* optional -- the feature itself always exists,
/// so this always creates a node when `flow.payload` is `Some`, additive alongside (not a
/// replacement for) the flat `payload_type_id` already resolved onto the edge in
/// `add_flow_edge_if_both_exist`. Unnamed flows have no node to own this feature and keep only
/// the edge-level `payload_type_id` (deliberate scope limit, not a regression).
fn materialize_flow_payload(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    flow_id: &NodeId,
    payload: &Node<PayloadFeature>,
) {
    let name = payload
        .value
        .name
        .clone()
        .unwrap_or_else(|| "_payload".to_string());
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "flow payload");
    let mut attrs = HashMap::new();
    if let Some(ref type_name) = payload.value.type_name {
        attrs.insert("payloadType".to_string(), serde_json::json!(type_name));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "flow payload",
        name,
        span_to_range(&payload.span),
        attrs,
        Some(flow_id),
    );
    if let Some(ref type_name) = payload.value.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
    }
    if let Some(ref multiplicity) = payload.value.multiplicity {
        let node_id = NodeId::new(uri, &qualified);
        if let Some(node) = g.get_node_mut(&node_id) {
            node.declared_facts.multiplicity = Some(declared_multiplicity(multiplicity, false));
        }
    }
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
    parent_id: &NodeId,
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
    let Some(src) =
        expressions::resolve_expression_endpoint_legacy(g, uri, container_prefix, &from_str)
    else {
        return;
    };
    let Some(tgt) =
        expressions::resolve_expression_endpoint_legacy(g, uri, container_prefix, &to_str)
    else {
        return;
    };
    // `of Payload` names a type, not a feature path, so it resolves through the same
    // workspace type-target lookup `add_typing_edge_if_exists` uses rather than the
    // feature-path resolver used for `from`/`to` above. This slot carries the resolved node's
    // qualified name at the graph-builder layer; `workspace`'s projection step (which owns the
    // semantic-ID hashing scheme) translates it into the final `payload_type_id` semantic ID
    // exposed to the API, the same two-step handoff `source_id`/`target_id` already use.
    let payload_type_id = flow.payload.as_ref().and_then(|payload| {
        let type_name = payload.value.type_name.as_deref()?;
        let context_node = g.get_node(parent_id)?;
        let target_id =
            resolve_type_target_in_workspace(g, context_node, type_name, TYPING_TARGET_KINDS)?;
        Some(target_id.qualified_name.clone())
    });
    let detail = FlowStatementDetail {
        declaring_uri: uri.clone(),
        range: span_to_range(&from.span),
        payload_expression: flow
            .payload
            .as_ref()
            .map(|payload| payload_feature_debug_string(&payload.value)),
        source_expression: Some(from_str),
        target_expression: Some(to_str),
        payload_type_id,
    };
    add_semantic_edge_once(
        g,
        &NodeId::new(uri, &src),
        &NodeId::new(uri, &tgt),
        SemanticEdge::flow_with_detail(relationship_kind_for_flow(flow.kind), detail),
    );
}
