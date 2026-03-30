use std::collections::HashMap;

use sysml_parser::ast::PortDefBodyElement;
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::NodeId;
use crate::relationships::add_typing_edge_if_exists;

use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_port_def_body_element(
    node: &sysml_parser::Node<PortDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PortDefBodyElement as PDBE;
    if let PDBE::PortUsage(n) = &node.value {
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
}
