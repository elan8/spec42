//! Shared walker for `AttributeBody` members (metadata, item def, individual def).

use std::collections::HashMap;

use sysml_v2_parser::ast::{AttributeBody, AttributeBodyElement};
use url::Url;

use super::{add_node_and_recurse, expressions, qualified_name_for_node, unit_metadata};
use crate::semantic::ast_util::{span_to_range, subsetting_target, typing_targets};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::kinds::METADATA_RESTRICTION_FEATURE_NAMES;
use crate::semantic::model::{ElementKind, NodeId};
use crate::semantic::relationships::add_typing_edge_if_exists;

/// Attaches any `doc` comments written inside a nested attribute def/usage's own
/// braces (e.g. a unit-catalog definition's `symbol`/`conversionFactor` block) to
/// that attribute's node. `build_from_attribute_body` only walks sibling elements,
/// so this body is otherwise never visited for graph materialization.
fn attach_nested_doc_comments(g: &mut SemanticGraph, node_id: &NodeId, body: &AttributeBody) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };
    for node in elements {
        if let AttributeBodyElement::Doc(doc) = &node.value {
            super::attach_doc_comment(g, node_id, &doc.value.text);
        }
    }
}

pub(super) fn build_from_attribute_body(
    body: &AttributeBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };

    for node in elements {
        match &node.value {
            AttributeBodyElement::AttributeDef(attribute) => {
                let value = &attribute.value;
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &value.name, "attribute def");
                let mut attrs = HashMap::new();
                let targets = typing_targets(value.typing.as_deref());
                if !targets.is_empty() {
                    attrs.insert(
                        "attributeType".to_string(),
                        serde_json::json!(targets.join(", ")),
                    );
                }
                unit_metadata::project_attribute_def_unit_metadata(&mut attrs, value);
                if let Some(expr_node) = &value.value {
                    let rendered =
                        expressions::expression_to_debug_string(&expr_node.value.expression);
                    attrs.insert("value".to_string(), serde_json::json!(rendered));
                    attrs.insert("defaultValue".to_string(), serde_json::json!(rendered));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute def",
                    value.name.clone(),
                    span_to_range(&attribute.span),
                    attrs,
                    Some(parent_id),
                );
                for target in typing_targets(value.typing.as_deref()) {
                    add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
                }
                attach_nested_doc_comments(g, &NodeId::new(uri, &qualified), &value.body);
            }
            AttributeBodyElement::AttributeUsage(attribute) => {
                let value = &attribute.value;
                let name = super::effective_usage_name(&value.name, value.redefines.as_deref());
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, name, "attribute");
                let mut attrs = HashMap::new();
                let targets = typing_targets(value.typing.as_deref());
                if !targets.is_empty() {
                    attrs.insert(
                        "attributeType".to_string(),
                        serde_json::json!(targets.join(", ")),
                    );
                }
                unit_metadata::project_attribute_usage_unit_metadata(&mut attrs, value);
                if let Some(s) = subsetting_target(value.subsets.as_deref()) {
                    attrs.insert("subsetsFeature".to_string(), serde_json::json!(s));
                }
                if let Some(r) = subsetting_target(value.redefines.as_deref()) {
                    attrs.insert("redefines".to_string(), serde_json::json!(r));
                    if g.get_node(parent_id).is_some_and(|parent| {
                        parent.element_kind == ElementKind::MetadataDef
                            && METADATA_RESTRICTION_FEATURE_NAMES.contains(&r)
                    }) {
                        attrs.insert("subsetsFeature".to_string(), serde_json::json!(r));
                    }
                }
                if let Some(expr_node) = &value.value {
                    attrs.insert(
                        "value".to_string(),
                        serde_json::json!(expressions::expression_to_debug_string(
                            &expr_node.value.expression
                        )),
                    );
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute",
                    name.to_string(),
                    span_to_range(&attribute.span),
                    attrs,
                    Some(parent_id),
                );
                for target in typing_targets(value.typing.as_deref()) {
                    add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
                }
                attach_nested_doc_comments(g, &NodeId::new(uri, &qualified), &value.body);
            }
            AttributeBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            AttributeBodyElement::Error(_) | AttributeBodyElement::Other(_) => {}
        }
    }
}
