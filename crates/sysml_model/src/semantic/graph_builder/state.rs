use std::collections::HashMap;

use sysml_v2_parser::ast::{StateDefBody, StateDefBodyElement, TransitionEffect};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::metadata_keyword::add_metadata_keyword_node;
use super::payload::insert_transition_accept_attrs;
use super::{add_node_and_recurse, qualified_name_for_node};

fn transition_target_is_done(target: &str) -> bool {
    target
        .rsplit("::")
        .next()
        .unwrap_or(target)
        .eq_ignore_ascii_case("done")
}

/// Renders a structured `do` transition effect (`TransitionEffect::{Perform,Accept,Send,Assign,
/// Expression}`) as a readable debug string for the `effectExpression` attribute.
fn transition_effect_to_debug_string(effect: &TransitionEffect) -> String {
    match effect {
        TransitionEffect::Perform { name, type_name } => {
            let name_part = name.as_deref().unwrap_or_default();
            match type_name {
                Some(t) => format!("action {name_part} : {t}"),
                None => format!("action {name_part}"),
            }
        }
        TransitionEffect::Accept {
            payload,
            type_name,
            via,
        } => {
            let mut s = format!("accept {}", expressions::expression_to_debug_string(payload));
            if let Some(t) = type_name {
                s.push_str(&format!(" : {t}"));
            }
            if let Some(via) = via {
                s.push_str(&format!(
                    " via {}",
                    expressions::expression_to_debug_string(via)
                ));
            }
            s
        }
        TransitionEffect::Send {
            payload,
            type_name,
            via,
            to,
        } => {
            let mut s = format!("send {}", expressions::expression_to_debug_string(payload));
            if let Some(t) = type_name {
                s.push_str(&format!(" : {t}"));
            }
            if let Some(via) = via {
                s.push_str(&format!(
                    " via {}",
                    expressions::expression_to_debug_string(via)
                ));
            }
            if let Some(to) = to {
                s.push_str(&format!(
                    " to {}",
                    expressions::expression_to_debug_string(to)
                ));
            }
            s
        }
        TransitionEffect::Assign { lhs, rhs } => format!(
            "assign {} := {}",
            expressions::expression_to_debug_string(lhs),
            expressions::expression_to_debug_string(rhs)
        ),
        TransitionEffect::Expression(expr) => expressions::expression_to_debug_string(expr),
    }
}

fn increment_state_def_counter(g: &mut SemanticGraph, parent_id: &NodeId, attribute: &str) {
    if let Some(state_def_node) = g.get_node_mut(parent_id) {
        let count = state_def_node
            .attributes
            .get(attribute)
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            .saturating_add(1);
        state_def_node
            .attributes
            .insert(attribute.to_string(), serde_json::json!(count));
    }
}

pub(super) fn build_from_state_body(
    elements: &[sysml_v2_parser::Node<StateDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::StateDefBodyElement as SDBE;
    let has_explicit_then = elements
        .iter()
        .any(|node| matches!(node.value, SDBE::Then(_)));
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
                let target_is_done = transition_target_is_done(&tgt_rel);
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
                if let Some(guard) = &t.guard {
                    attrs.insert(
                        "guardExpression".to_string(),
                        serde_json::json!(expressions::expression_to_debug_string(guard)),
                    );
                    attrs.insert(
                        "conditionIsBoolean".to_string(),
                        serde_json::json!(expressions::expression_is_boolean_valued(guard)),
                    );
                    attrs.insert(
                        "exprClass".to_string(),
                        serde_json::json!(expressions::classify_expression(guard).as_str()),
                    );
                }
                if let Some(effect) = &t.effect {
                    attrs.insert(
                        "effectExpression".to_string(),
                        serde_json::json!(transition_effect_to_debug_string(effect)),
                    );
                }
                if let Some(ref accept) = t.accept {
                    insert_transition_accept_attrs(&mut attrs, accept);
                }
                if t.is_initial {
                    attrs.insert("isInitial".to_string(), serde_json::json!(true));
                }
                if target_is_done {
                    attrs.insert("targetIsDone".to_string(), serde_json::json!(true));
                    increment_state_def_counter(g, parent_id, "doneTransitionCount");
                }
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
                let is_unnamed = t.name.as_deref().unwrap_or("").trim().is_empty();
                if t.is_initial && t.guard.is_none() && !has_explicit_then && is_unnamed {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        &parent_id.qualified_name,
                        &src,
                        RelationshipKind::InitialState,
                    );
                }
            }
            SDBE::FinalState(final_node) => {
                let fs = &final_node.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &format!("final_{}", fs.state_name),
                    "final state",
                );
                let mut attrs = HashMap::new();
                attrs.insert("stateName".to_string(), serde_json::json!(&fs.state_name));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "final state",
                    fs.state_name.clone(),
                    span_to_range(&final_node.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(state_def_node) = g.get_node_mut(parent_id) {
                    let count = state_def_node
                        .attributes
                        .get("finalStateCount")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0)
                        .saturating_add(1);
                    state_def_node
                        .attributes
                        .insert("finalStateCount".to_string(), serde_json::json!(count));
                }
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
                    "action",
                );
                let mut attrs = HashMap::new();
                attrs.insert("compartment".to_string(), serde_json::json!("entry"));
                if let Some(ref an) = en.action_name {
                    attrs.insert("actionName".to_string(), serde_json::json!(an));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "action",
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
            SDBE::Do(do_node) => {
                let d = &do_node.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    "_do",
                    "action",
                );
                let mut attrs = HashMap::new();
                attrs.insert("compartment".to_string(), serde_json::json!("do"));
                if let Some(ref an) = d.action_name {
                    attrs.insert("actionName".to_string(), serde_json::json!(an));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "action",
                    "do".to_string(),
                    span_to_range(&do_node.span),
                    attrs,
                    Some(parent_id),
                );
                let do_id = NodeId::new(uri, &qualified);
                if let StateDefBody::Brace { elements } = &d.body {
                    build_from_state_body(elements, uri, Some(&qualified), &do_id, g);
                }
            }
            SDBE::Exit(exit_node) => {
                let ex = &exit_node.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    "_exit",
                    "action",
                );
                let mut attrs = HashMap::new();
                attrs.insert("compartment".to_string(), serde_json::json!("exit"));
                if let Some(ref an) = ex.action_name {
                    attrs.insert("actionName".to_string(), serde_json::json!(an));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "action",
                    "exit".to_string(),
                    span_to_range(&exit_node.span),
                    attrs,
                    Some(parent_id),
                );
                let exit_id = NodeId::new(uri, &qualified);
                if let StateDefBody::Brace { elements } = &ex.body {
                    build_from_state_body(elements, uri, Some(&qualified), &exit_id, g);
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
                super::usage_builders::materialize_requirement_usage(
                    ru_node,
                    uri,
                    container_prefix,
                    Some(parent_id),
                    g,
                );
            }
            SDBE::MetadataKeywordUsage(mk_node) => {
                add_metadata_keyword_node(g, uri, parent_id, &mk_node.value, &mk_node.span);
            }
            SDBE::MetadataAnnotation(meta) => {
                super::metadata_def::add_metadata_annotation_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &meta.value,
                    &meta.span,
                );
            }
            SDBE::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            SDBE::Error(_) | SDBE::Annotation(_) | SDBE::Other(_) => {}
        }
    }
}
