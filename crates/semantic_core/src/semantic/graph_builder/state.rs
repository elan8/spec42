use std::collections::HashMap;

use sysml_v2_parser::ast::{StateDefBody, StateDefBodyElement};
use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::requirement_body::walk_requirement_def_body;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_state_body(
    elements: &[sysml_v2_parser::Node<StateDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::StateDefBodyElement as SDBE;
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
                let t = &transition_node.value;
                let tgt_rel = expressions::expr_node_to_qualified_string(&t.target);
                if tgt_rel.is_empty() {
                    continue;
                }
                let tgt = if let Some(prefix) = container_prefix {
                    format!("{}::{}", prefix, tgt_rel)
                } else {
                    tgt_rel
                };
                let src = if let Some(src_expr) = &t.source {
                    let src_rel = expressions::expr_node_to_qualified_string(src_expr);
                    if src_rel.is_empty() {
                        continue;
                    }
                    if let Some(prefix) = container_prefix {
                        format!("{}::{}", prefix, src_rel)
                    } else {
                        src_rel
                    }
                } else {
                    parent_id.qualified_name.clone()
                };
                let transition_name = if t.name.as_deref().unwrap_or("").trim().is_empty() {
                    format!(
                        "transition_{}_to_{}",
                        src.rsplit("::").next().unwrap_or("source"),
                        tgt.rsplit("::").next().unwrap_or("target")
                    )
                } else {
                    t.name.clone().unwrap_or_default()
                };
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &transition_name,
                    "transition",
                );
                let mut attrs = HashMap::new();
                attrs.insert("source".to_string(), serde_json::json!(&src));
                attrs.insert("target".to_string(), serde_json::json!(&tgt));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "transition",
                    transition_name,
                    span_to_range(&transition_node.span),
                    attrs,
                    Some(parent_id),
                );
                add_edge_if_both_exist(g, uri, &src, &tgt, RelationshipKind::Transition);
            }
            SDBE::Then(then_node) => {
                let state_name = &then_node.value.state_name;
                let tgt = if let Some(prefix) = container_prefix {
                    format!("{}::{}", prefix, state_name)
                } else {
                    state_name.clone()
                };
                add_edge_if_both_exist(
                    g,
                    uri,
                    &parent_id.qualified_name,
                    &tgt,
                    RelationshipKind::InitialState,
                );
            }
            SDBE::Entry(entry_node) => {
                let en = &entry_node.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    "_entry",
                    "entry",
                );
                let mut attrs = HashMap::new();
                if let Some(ref an) = en.action_name {
                    attrs.insert("actionName".to_string(), serde_json::json!(an));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "entry",
                    "entry".to_string(),
                    span_to_range(&entry_node.span),
                    attrs,
                    Some(parent_id),
                );
                let entry_id = NodeId::new(uri, &qualified);
                if let StateDefBody::Brace { elements } = &en.body {
                    build_from_state_body(elements, uri, Some(&qualified), &entry_id, g);
                }
            }
            SDBE::Ref(r) => {
                let n = &r.value;
                let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "ref");
                let range = span_to_range(&r.span);
                let mut attrs = HashMap::new();
                attrs.insert("refType".to_string(), serde_json::json!(&n.type_name));
                if let Some(ref v) = n.value {
                    attrs.insert(
                        "value".to_string(),
                        serde_json::json!(expressions::expression_to_debug_string(v)),
                    );
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
            }
            SDBE::RequirementUsage(ru_node) => {
                let name = &ru_node.name;
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, name, "requirement");
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
            SDBE::Error(_) | SDBE::Doc(_) | SDBE::Annotation(_) | SDBE::Other(_) => {}
        }
    }
}
