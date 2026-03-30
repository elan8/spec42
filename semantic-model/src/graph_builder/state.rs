use std::collections::HashMap;

use sysml_parser::ast::{StateDefBody, StateDefBodyElement};
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_state_body(
    elements: &[sysml_parser::Node<StateDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::StateDefBodyElement as SDBE;
    for node in elements {
        match &node.value {
            SDBE::StateUsage(state_node) => {
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
                    build_from_state_body(elements, uri, Some(&qualified), &state_id, g);
                }
            }
            SDBE::Transition(transition_node) => {
                if let Some(src_expr) = &transition_node.source {
                    let src_rel = expressions::expr_node_to_qualified_string(src_expr);
                    let tgt_rel =
                        expressions::expr_node_to_qualified_string(&transition_node.target);
                    if !src_rel.is_empty() && !tgt_rel.is_empty() {
                        let (src, tgt) = if let Some(prefix) = container_prefix {
                            (
                                format!("{}::{}", prefix, src_rel),
                                format!("{}::{}", prefix, tgt_rel),
                            )
                        } else {
                            (src_rel, tgt_rel)
                        };
                        add_edge_if_both_exist(g, uri, &src, &tgt, RelationshipKind::Transition);
                    }
                }
            }
            _ => {}
        }
    }
}
