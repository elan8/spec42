use std::collections::HashMap;

use sysml_v2_parser::ast::{CalcDefBody, InterfaceDefBody, PartDefBodyElement, PartUsageBody};
use url::Url;

use crate::semantic::ast_util::{identification_name, span_to_range};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::interface_def;
use super::part_usage;
use super::port_def::materialize_port_usage;
use super::requirement_body::walk_requirement_def_body;
use super::state;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_part_def_body_element(
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
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
            if let Some(ref v) = n.value.value {
                let rendered = expressions::expression_to_debug_string(v);
                attrs.insert("value".to_string(), serde_json::json!(rendered));
                attrs.insert("defaultValue".to_string(), serde_json::json!(rendered));
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
            if let Some(ref s) = n.subsets {
                attrs.insert("subsetsFeature".to_string(), serde_json::json!(s));
            }
            if let Some(ref r) = n.references {
                attrs.insert("referencesFeature".to_string(), serde_json::json!(r));
            }
            if let Some(ref c) = n.crosses {
                attrs.insert("crossesFeature".to_string(), serde_json::json!(c));
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
            let exhibit_state_id = NodeId::new(uri, &qualified);
            if let sysml_v2_parser::ast::StateDefBody::Brace { elements } = &es.body {
                state::build_from_state_body(elements, uri, Some(&qualified), &exhibit_state_id, g);
            }
        }
        PDBE::PortUsage(n) => {
            materialize_port_usage(n, uri, container_prefix, parent_id, g);
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
                        g,
                    );
                }
            }
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
        PDBE::ItemUsage(item_node) => {
            let name = &item_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "item");
            let range = span_to_range(&item_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = item_node.type_name {
                attrs.insert("itemType".to_string(), serde_json::json!(t));
            }
            if let Some(ref m) = item_node.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "item",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = item_node.type_name {
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
        PDBE::InterfaceDef(id_node) => {
            let name = identification_name(&id_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "interface");
            let range = span_to_range(&id_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "interface",
                name.clone(),
                range,
                HashMap::new(),
                Some(parent_id),
            );
            let iface_id = NodeId::new(uri, &qualified);
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                for el in elements {
                    interface_def::build_from_interface_def_body_element(
                        el,
                        uri,
                        Some(&qualified),
                        &iface_id,
                        g,
                    );
                }
            }
        }
        PDBE::Connection(connection_usage) => {
            let connection = &connection_usage.value;
            let name = connection
                .name
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("_connection");
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "connection");
            let mut attrs = HashMap::new();
            if let Some(ref subsets) = connection.subsets {
                attrs.insert("subsetsFeature".to_string(), serde_json::json!(subsets));
            }
            if let Some(ref redefines) = connection.redefines {
                attrs.insert("redefines".to_string(), serde_json::json!(redefines));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "connection",
                name.to_string(),
                span_to_range(&connection_usage.span),
                attrs,
                Some(parent_id),
            );
            if let Some(ref type_name) = connection.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
            }
            let connection_node_id = NodeId::new(uri, &qualified);
            if let sysml_v2_parser::ast::ConnectionDefBody::Brace { elements } = &connection.body {
                super::interface_def::build_from_connection_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &connection_node_id,
                    g,
                );
            }
        }
        PDBE::CalcUsage(calc_node) => {
            let name = identification_name(&calc_node.value.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "calc");
            let range = span_to_range(&calc_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = calc_node.value.type_name {
                attrs.insert("calcType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "calc",
                name,
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = calc_node.value.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let CalcDefBody::Brace { .. } = &calc_node.value.body {
                // Calc body members (parameters, return) are not expanded into the graph yet.
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
        PDBE::Ref(r) => {
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
        // Compatibility-only members introduced by newer parser versions are intentionally ignored.
        PDBE::EnumerationUsage(_)
        | PDBE::Annotation(_)
        | PDBE::Error(_)
        | PDBE::Doc(_)
        | PDBE::Comment(_)
        | PDBE::Other(_)
        | PDBE::OpaqueMember(_) => {}
    }
}
