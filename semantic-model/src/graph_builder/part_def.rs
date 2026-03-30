use std::collections::HashMap;

use sysml_parser::ast::{PartDefBodyElement, PartUsageBody};
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::part_usage;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_part_def_body_element(
    node: &sysml_parser::Node<PartDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => {
            let name = &n.name;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name, "attribute def");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = n.typing {
                attrs.insert("attributeType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute def",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = n.typing {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PDBE::PortUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "port");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = n.type_name {
                attrs.insert("portType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "port",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = n.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PDBE::PartUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            attrs.insert("partType".to_string(), serde_json::json!(&n.type_name));
            if let Some(ref m) = n.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "part",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            let node_id = NodeId::new(uri, &qualified);
            add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
            if let PartUsageBody::Brace { elements } = &n.body {
                for child in elements {
                    part_usage::build_from_part_usage_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        root,
                        g,
                    );
                }
            }
            part_usage::expand_typed_part_usage(
                root,
                uri,
                &qualified,
                &n.type_name,
                container_prefix,
                &node_id,
                g,
            );
        }
        PDBE::Connect(c) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &c.from,
                &c.to,
                RelationshipKind::Connection,
            );
        }
        PDBE::InterfaceUsage(interface_usage) => {
            use sysml_parser::ast::InterfaceUsage;
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
        PDBE::Perform(perform_node) => {
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
        PDBE::Allocate(allocate_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &allocate_node.source,
                &allocate_node.target,
                RelationshipKind::Allocate,
            );
        }
        _ => {}
    }
}
