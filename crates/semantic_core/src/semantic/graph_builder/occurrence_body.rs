//! Occurrence-level body members inside `DefinitionBody` and nested occurrence usages.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    AssertConstraintMember, ConstraintDefBody, ConstraintDefBodyElement, OccurrenceBodyElement,
    OccurrenceUsageBody, PartUsageBody,
};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;

use super::expressions::expression_to_debug_string;
use super::part_usage;
use super::{add_node_and_recurse, qualified_name_for_node};

fn constraint_body_expression(body: &ConstraintDefBody) -> Option<String> {
    let ConstraintDefBody::Brace { elements } = body else {
        return None;
    };
    let mut fragments = Vec::new();
    for element in elements {
        if let ConstraintDefBodyElement::Expression(expr) = &element.value {
            let rendered = expression_to_debug_string(expr);
            if !rendered.is_empty() {
                fragments.push(rendered);
            }
        }
    }
    let expression = fragments.join(" ").trim().to_string();
    if expression.is_empty() {
        None
    } else {
        Some(expression)
    }
}

pub(super) fn build_from_occurrence_body_element(
    node: &sysml_v2_parser::Node<OccurrenceBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use OccurrenceBodyElement as OBE;
    match &node.value {
        OBE::AttributeUsage(attribute) => {
            let value = &attribute.value;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &value.name, "attribute");
            let mut attrs = HashMap::new();
            if let Some(ref typing) = value.typing {
                attrs.insert("attributeType".to_string(), serde_json::json!(typing));
            }
            if let Some(ref r) = value.redefines {
                attrs.insert("redefines".to_string(), serde_json::json!(r));
            }
            if let Some(expr_node) = &value.value {
                attrs.insert(
                    "value".to_string(),
                    serde_json::json!(expression_to_debug_string(expr_node)),
                );
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute",
                value.name.clone(),
                span_to_range(&attribute.span),
                attrs,
                Some(parent_id),
            );
            if let Some(ref typing) = value.typing {
                add_typing_edge_if_exists(g, uri, &qualified, typing, container_prefix);
            }
        }
        OBE::PartUsage(part_usage_node) => {
            let part = part_usage_node.as_ref();
            let name = &part.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
            let mut attrs = HashMap::new();
            attrs.insert("partType".to_string(), serde_json::json!(&part.type_name));
            if let Some(ref m) = part.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "part",
                name.clone(),
                span_to_range(&part.span),
                attrs,
                Some(parent_id),
            );
            let node_id = NodeId::new(uri, &qualified);
            add_typing_edge_if_exists(g, uri, &qualified, &part.type_name, container_prefix);
            if let PartUsageBody::Brace { elements } = &part.body {
                for child in elements {
                    part_usage::build_from_part_usage_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        g,
                    );
                }
            }
        }
        OBE::OccurrenceUsage(occurrence_usage_node) => {
            let occurrence = occurrence_usage_node.as_ref();
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &occurrence.name, "occurrence");
            let mut attrs = HashMap::new();
            if let Some(ref t) = occurrence.type_name {
                attrs.insert("occurrenceType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "occurrence",
                occurrence.name.clone(),
                span_to_range(&occurrence.span),
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = occurrence.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            let node_id = NodeId::new(uri, &qualified);
            if let OccurrenceUsageBody::Brace { elements } = &occurrence.body {
                for child in elements {
                    build_from_occurrence_body_element(child, uri, Some(&qualified), &node_id, g);
                }
            }
        }
        OBE::AssertConstraint(assert_node) => {
            add_assert_constraint_member(g, uri, parent_id, assert_node);
        }
        OBE::Doc(_)
        | OBE::Error(_)
        | OBE::Annotation(_)
        | OBE::Other(_) => {}
    }
}

fn add_assert_constraint_member(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    assert_node: &sysml_v2_parser::Node<AssertConstraintMember>,
) {
    let constraint_index = g
        .get_node(parent_id)
        .map(|parent| {
            g.children_of(parent)
                .iter()
                .filter(|child| child.element_kind == "assert constraint")
                .count()
        })
        .unwrap_or(0);
    let name = format!("_assertConstraint_{constraint_index}");
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &name,
        "assert constraint",
    );
    let mut attrs = HashMap::new();
    attrs.insert("kind".to_string(), serde_json::json!("assert_constraint"));
    if let Some(expression) = constraint_body_expression(&assert_node.value.body) {
        attrs.insert("expression".to_string(), serde_json::json!(expression));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "assert constraint",
        name,
        span_to_range(&assert_node.span),
        attrs,
        Some(parent_id),
    );
    let assert_id = NodeId::new(uri, &qualified);
    super::metadata_def::wire_constraint_body_metadata(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &assert_id,
        &assert_node.value.body,
    );
}
