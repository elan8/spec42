use std::collections::HashMap;

use sysml_v2_parser::ast::{
    AttributeBody, ConstraintDefBody, ConstraintDefBodyElement, MetadataAnnotation, MetadataUsage,
    Node,
};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::{add_typing_edge_if_exists, wire_metadata_annotated_elements};

use super::attribute_body::build_from_attribute_body;
use super::{add_node_and_recurse, qualified_name_for_node};

fn insert_metadata_usage_attrs(
    attrs: &mut HashMap<String, serde_json::Value>,
    name: &str,
    type_name: Option<&str>,
    about_targets: &[String],
) {
    attrs.insert("annotationName".to_string(), serde_json::json!(name));
    if let Some(t) = type_name.filter(|value| !value.trim().is_empty()) {
        attrs.insert("metadataType".to_string(), serde_json::json!(t));
    }
    if !about_targets.is_empty() {
        attrs.insert("aboutTargets".to_string(), serde_json::json!(about_targets));
    }
}

fn project_metadata_usage_body(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    qualified: &str,
    body: &AttributeBody,
) {
    let node_id = NodeId::new(uri, qualified);
    build_from_metadata_attribute_body(body, uri, Some(qualified), &node_id, g);
    let _ = container_prefix;
}

pub(super) fn add_package_metadata_usage_node(
    g: &mut SemanticGraph,
    uri: &Url,
    type_resolution_prefix: Option<&str>,
    parent_id: &NodeId,
    mu: &MetadataUsage,
    span: &sysml_v2_parser::Span,
) {
    // Ownership QN follows the owning namespace/element; typing may resolve from an outer prefix.
    let ownership_prefix = Some(parent_id.qualified_name.as_str());
    let qualified = qualified_name_for_node(g, uri, ownership_prefix, &mu.name, "metadata usage");
    let mut attrs = HashMap::new();
    insert_metadata_usage_attrs(
        &mut attrs,
        &mu.name,
        mu.type_name.as_deref(),
        &mu.about_targets,
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "metadata usage",
        mu.name.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
    if let Some(ref t) = mu.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, type_resolution_prefix);
    }
    project_metadata_usage_body(g, uri, ownership_prefix, &qualified, &mu.body);
    let metadata_id = NodeId::new(uri, &qualified);
    wire_metadata_annotated_elements(g, uri, &metadata_id, parent_id, &mu.about_targets);
}

/// Materializes a `@Name` / `#keyword`-style metadata annotation as a `metadata usage` child of
/// `parent_id`. Qualified names always nest under the owner (matching subject/actor/attribute
/// nesting), while `type_resolution_prefix` is only used for typing edges — important for
/// requirement bodies, which keep the enclosing package as the type-resolution prefix.
pub(super) fn add_metadata_annotation_node(
    g: &mut SemanticGraph,
    uri: &Url,
    type_resolution_prefix: Option<&str>,
    parent_id: &NodeId,
    meta: &MetadataAnnotation,
    span: &sysml_v2_parser::Span,
) {
    let ownership_prefix = Some(parent_id.qualified_name.as_str());
    let qualified = qualified_name_for_node(g, uri, ownership_prefix, &meta.name, "metadata usage");
    let mut attrs = HashMap::new();
    insert_metadata_usage_attrs(
        &mut attrs,
        &meta.name,
        meta.type_name.as_deref(),
        &meta.about_targets,
    );
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
        add_typing_edge_if_exists(g, uri, &qualified, t, type_resolution_prefix);
    }
    project_metadata_usage_body(g, uri, ownership_prefix, &qualified, &meta.body);
    let metadata_id = NodeId::new(uri, &qualified);
    wire_metadata_annotated_elements(g, uri, &metadata_id, parent_id, &meta.about_targets);
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
        match &element.value {
            ConstraintDefBodyElement::MetadataAnnotation(meta) => {
                add_metadata_annotation_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &meta.value,
                    &meta.span,
                );
            }
            ConstraintDefBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            _ => {}
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
