use std::collections::HashMap;

use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::NodeId;
use crate::relationships::add_typing_edge_if_exists;

use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_use_case_body(
    elements: &[sysml_parser::Node<sysml_parser::ast::UseCaseDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::UseCaseDefBodyElement as UCBE;
    for node in elements {
        if let UCBE::ActorUsage(actor_node) = &node.value {
            let name = &actor_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "actor");
            let range = span_to_range(&actor_node.span);
            let mut attrs = HashMap::new();
            attrs.insert(
                "actorType".to_string(),
                serde_json::json!(&actor_node.type_name),
            );
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "actor",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            add_typing_edge_if_exists(g, uri, &qualified, &actor_node.type_name, container_prefix);
        }
    }
}
