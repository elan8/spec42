use url::Url;

use crate::semantic::ast_util::{connection_end_expression, span_to_range};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::add_edge_if_both_exist;

use super::expressions;
use super::port_def::materialize_port_usage;

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
                connection_end_expression(&c.from),
                connection_end_expression(&c.to),
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
            super::ref_decl::materialize_ref_decl(
                g,
                uri,
                container_prefix,
                parent_id,
                r,
                super::ref_decl::RefDeclOptions {
                    wire_value_reference: true,
                },
            );
        }
        PUBE::ActionUsage(au) => {
            super::action::materialize_top_level_action_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                au.as_ref(),
            );
        }
        PUBE::StateUsage(state_node) => {
            super::package_body::materialize_state_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                state_node,
            );
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
        _ => {}
    }
}
