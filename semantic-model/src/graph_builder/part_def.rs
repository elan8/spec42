use std::collections::HashMap;

use sysml_v2_parser::ast::{PartDefBodyElement, PartUsageBody};
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::part_usage;
use super::requirement_body::walk_requirement_def_body;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_part_def_body_element(
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
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
        PDBE::AttributeUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "attribute");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref r) = n.redefines {
                attrs.insert("redefines".to_string(), serde_json::json!(r));
            }
            if let Some(ref v) = n.value.value {
                attrs.insert(
                    "value".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(v)),
                );
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
        }
        PDBE::ExhibitState(es_node) => {
            let es = &es_node.value;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &es.name, "exhibit state");
            let range = span_to_range(&es_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref state_type) = es.type_name {
                attrs.insert("stateType".to_string(), serde_json::json!(state_type));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "exhibit state",
                es.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref state_type) = es.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, state_type, container_prefix);
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
            attrs.insert("ordered".to_string(), serde_json::json!(n.ordered));
            if let Some((ref feat, ref val)) = n.subsets {
                attrs.insert("subsetsFeature".to_string(), serde_json::json!(feat));
                if let Some(v) = val {
                    attrs.insert(
                        "subsetsValue".to_string(),
                        serde_json::json!(expressions::expression_to_debug_string(v)),
                    );
                }
            }
            if let Some(ref r) = n.redefines {
                attrs.insert("redefines".to_string(), serde_json::json!(r));
            }
            if let Some(ref v) = n.value.value {
                attrs.insert(
                    "value".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(v)),
                );
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
        PDBE::OccurrenceUsage(occ_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &occ_node.name, "occurrence");
            let range = span_to_range(&occ_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = occ_node.type_name {
                attrs.insert("occurrenceType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "occurrence",
                occ_node.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = occ_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PDBE::RequirementUsage(ru_node) => {
            let name = &ru_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "requirement");
            let range = span_to_range(&ru_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = ru_node.type_name {
                attrs.insert("requirementType".to_string(), serde_json::json!(t));
            }
            if let Some(ref subsets) = ru_node.subsets {
                attrs.insert("subsetsFeature".to_string(), serde_json::json!(subsets));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "requirement",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = ru_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            let node_id = NodeId::new(uri, &qualified);
            walk_requirement_def_body(
                g,
                uri,
                container_prefix,
                &qualified,
                &node_id,
                &ru_node.body,
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
        // Compatibility-only members introduced by newer parser versions are intentionally ignored.
        PDBE::Annotation(_)
        | PDBE::Error(_)
        | PDBE::Doc(_)
        | PDBE::Other(_)
        | PDBE::Ref(_)
        | PDBE::OpaqueMember(_) => {}
    }
}
