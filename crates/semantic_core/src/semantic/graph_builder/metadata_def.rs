use std::collections::HashMap;

use sysml_v2_parser::ast::{
    AttributeBody, ConstraintDefBody, ConstraintDefBodyElement, MetadataAnnotation, Node,
};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;

use super::attribute_body::build_from_attribute_body;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn add_metadata_annotation_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    meta: &MetadataAnnotation,
    span: &sysml_v2_parser::Span,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, &meta.name, "metadata usage");
    let mut attrs = HashMap::new();
    attrs.insert("annotationName".to_string(), serde_json::json!(&meta.name));
    if let Some(ref t) = meta.type_name {
        attrs.insert("metadataType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "metadata usage",
        meta.name.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
    if let Some(ref t) = meta.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
}

pub(super) fn wire_constraint_body_metadata(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    body: &ConstraintDefBody,
) {
    let ConstraintDefBody::Brace { elements } = body else {
        return;
    };
    wire_constraint_body_metadata_elements(g, uri, container_prefix, parent_id, elements);
}

pub(super) fn wire_require_constraint_body_metadata(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    body: &sysml_v2_parser::ast::RequireConstraintBody,
) {
    let sysml_v2_parser::ast::RequireConstraintBody::Brace { elements } = body else {
        return;
    };
    wire_constraint_body_metadata_elements(g, uri, container_prefix, parent_id, elements);
}

fn wire_constraint_body_metadata_elements(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    elements: &[Node<ConstraintDefBodyElement>],
) {
    for element in elements {
        if let ConstraintDefBodyElement::MetadataAnnotation(meta) = &element.value {
            add_metadata_annotation_node(
                g,
                uri,
                container_prefix,
                parent_id,
                &meta.value,
                &meta.span,
            );
        }
    }
}

pub(super) fn build_from_metadata_attribute_body(
    body: &AttributeBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    build_from_attribute_body(body, uri, container_prefix, parent_id, g);
}
