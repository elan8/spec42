use std::collections::HashMap;

use sysml_v2_parser::ast::StateDefBody;
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::port_def::materialize_port_usage;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_part_usage_body_element(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => {
            super::usage_builders::materialize_attribute_usage(
                n,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        PUBE::PartUsage(n) => {
            super::usage_builders::materialize_part_usage(
                n,
                uri,
                container_prefix,
                Some(parent_id),
                g,
            );
        }
        PUBE::PortUsage(n) => {
            materialize_port_usage(n, uri, container_prefix, parent_id, g);
        }
        PUBE::Connect(c) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &c.from,
                &c.to,
                RelationshipKind::Connection,
            );
        }
        PUBE::FlowUsage(flow) => {
            super::flow_usage::materialize_flow_usage(flow, uri, container_prefix, parent_id, g);
        }
        PUBE::Bind(b) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &b.left,
                &b.right,
                RelationshipKind::Bind,
            );
        }
        PUBE::InterfaceUsage(interface_usage) => {
            use sysml_v2_parser::ast::InterfaceUsage;
            match &interface_usage.value {
                InterfaceUsage::TypedConnect { from, to, .. }
                | InterfaceUsage::Connection { from, to, .. } => {
                    expressions::add_expression_edge_if_both_exist(
                        g,
                        uri,
                        container_prefix,
                        from,
                        to,
                        RelationshipKind::Connection,
                    );
                }
            }
        }
        PUBE::Perform(perform_node) => {
            let perform_qualified = expressions::add_perform_usage_node(
                g,
                uri,
                container_prefix,
                parent_id,
                &perform_node.value.action_name,
                perform_node.value.type_name.as_deref(),
                span_to_range(&perform_node.span),
            );
            add_edge_if_both_exist(
                g,
                uri,
                &parent_id.qualified_name,
                &perform_qualified,
                RelationshipKind::Perform,
            );
        }
        PUBE::Allocate(allocate_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &allocate_node.source,
                &allocate_node.target,
                RelationshipKind::Allocate,
            );
        }
        PUBE::Satisfy(satisfy_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &satisfy_node.source,
                &satisfy_node.target,
                RelationshipKind::Satisfy,
            );
            if let Some(elements) = &satisfy_node.body_elements {
                super::requirement_body::walk_satisfy_constraint_elements(
                    elements,
                    uri,
                    container_prefix,
                    g,
                );
            }
        }
        PUBE::Ref(r) => {
            let n = &r.value;
            let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "ref");
            let range = span_to_range(&r.span);
            let mut attrs = HashMap::new();
            attrs.insert("refType".to_string(), serde_json::json!(&n.type_name));
            let value_expression = n
                .value
                .as_ref()
                .map(expressions::expression_to_debug_string);
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
            add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
            if let Some(value_expression) = value_expression.as_deref() {
                if let Some(target) = expressions::resolve_expression_endpoint_legacy(
                    g,
                    uri,
                    container_prefix,
                    value_expression,
                ) {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        &qualified,
                        &target,
                        RelationshipKind::Reference,
                    );
                }
            }
        }
        PUBE::StateUsage(state_node) => {
            let name = &state_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "state");
            let range = span_to_range(&state_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = state_node.type_name {
                attrs.insert("stateType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "state",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            let state_id = NodeId::new(uri, &qualified);
            if let Some(ref t) = state_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let StateDefBody::Brace { elements } = &state_node.body {
                super::state::build_from_state_body(elements, uri, Some(&qualified), &state_id, g);
            }
        }
        PUBE::MetadataAnnotation(meta) => {
            super::metadata_def::add_metadata_annotation_node(
                g,
                uri,
                container_prefix,
                parent_id,
                &meta.value,
                &meta.span,
            );
        }
        PUBE::OccurrenceUsage(occ_node) => {
            super::usage_builders::materialize_occurrence_usage(
                occ_node,
                uri,
                container_prefix,
                Some(parent_id),
                g,
            );
        }
        PUBE::MetadataKeywordUsage(mk_node) => {
            super::metadata_keyword::add_metadata_keyword_node(
                g,
                uri,
                parent_id,
                &mk_node.value,
                &mk_node.span,
            );
        }
        PUBE::VariantUsage(variant) => {
            super::usage_builders::materialize_variant_usage(
                variant,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        PUBE::Doc(doc) => {
            super::attach_doc_comment(g, parent_id, &doc.value.text);
        }
        PUBE::EnumerationUsage(_) | PUBE::Annotation(_) | PUBE::Error(_) => {}
    }
}
