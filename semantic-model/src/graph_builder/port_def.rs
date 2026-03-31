use std::collections::HashMap;

use sysml_parser::ast::{InOut, PortDefBodyElement};
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::NodeId;
use crate::relationships::add_typing_edge_if_exists;

use super::expressions;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_port_def_body_element(
    node: &sysml_parser::Node<PortDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PortDefBodyElement as PDBE;
    match &node.value {
        PDBE::Doc(_) => {}
        PDBE::InOutDecl(w) => {
            let d = &w.value;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &d.name, "in out parameter");
            let range = span_to_range(&w.span);
            let mut attrs = HashMap::new();
            attrs.insert(
                "direction".to_string(),
                serde_json::json!(match d.direction {
                    InOut::In => "in",
                    InOut::Out => "out",
                }),
            );
            attrs.insert("parameterType".to_string(), serde_json::json!(&d.type_name));
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "in out parameter",
                d.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            add_typing_edge_if_exists(g, uri, &qualified, &d.type_name, container_prefix);
        }
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
        PDBE::PortUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "port");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = n.type_name {
                attrs.insert("portType".to_string(), serde_json::json!(t));
            }
            if let Some(ref m) = n.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
            }
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
    }
}
