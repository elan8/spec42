//! Shared materialization for `ref` declarations (`RefDecl`).

use std::collections::HashMap;

use sysml_v2_parser::ast::RefDecl;
use sysml_v2_parser::Node;
use url::Url;

use crate::semantic::ast_util::{
    declared_feature_value, ref_decl_feature_properties, span_to_range, subsetting_target,
    typing_targets,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::{add_node_and_recurse, attach_feature_properties, qualified_name_for_node};

/// Options for context-specific follow-up after the shared ref node is created.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RefDeclOptions {
    /// Wire a `Reference` edge from the optional binding expression (part def/usage bodies).
    pub wire_value_reference: bool,
}

/// Materializes a `ref name : Type [= value] …` declaration as `ElementKind::Ref`
/// with reference ownership facts and optional feature value.
pub(super) fn materialize_ref_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    wrap: &Node<RefDecl>,
    options: RefDeclOptions,
) -> NodeId {
    let n = &wrap.value;
    let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "ref");
    let range = span_to_range(&wrap.span);
    let mut attrs = HashMap::new();
    attrs.insert("refType".to_string(), serde_json::json!(&n.type_name));
    if let Some(vis) = &n.membership.visibility {
        attrs.insert(
            "visibility".to_string(),
            serde_json::json!(format!("{vis:?}")),
        );
    }
    if let Some(r) = subsetting_target(n.redefines.as_deref()) {
        attrs.insert("redefines".to_string(), serde_json::json!(r));
    }
    let value_expression = n
        .value
        .as_ref()
        .map(|value| expressions::expression_to_debug_string(&value.value.expression));
    if let Some(ref v) = value_expression {
        attrs.insert("value".to_string(), serde_json::json!(v));
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
    let node_id = NodeId::new(uri, &qualified);
    attach_feature_properties(g, &node_id, ref_decl_feature_properties());
    if let Some(value) = &n.value {
        if let Some(node) = g.get_node_mut(&node_id) {
            node.declared_facts.feature_value = Some(declared_feature_value(value));
        }
    }
    for target in typing_targets(n.typing.as_deref()) {
        add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
    }
    if options.wire_value_reference {
        if let Some(value_expression) = value_expression.as_deref() {
            if let Some(target) = expressions::resolve_expression_endpoint_legacy(
                g,
                uri,
                container_prefix,
                value_expression,
            ) {
                add_edge_if_both_exist(g, uri, &qualified, &target, RelationshipKind::Reference);
            }
        }
    }
    node_id
}
