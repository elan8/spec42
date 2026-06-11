//! Action definition and usage bodies: parameters, perform, flow, nested actions.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ActionBodyDecl, ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody,
    ActionUsageBodyElement, AssignStmt, ForLoop, InOut, Perform, RefDecl, StateDefBody, StateUsage,
    ThenAction,
};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::payload::insert_action_payload_attrs;
use super::state;
use super::{add_node_and_recurse, qualified_name_for_node};

struct ThenActionChain {
    previous: Option<String>,
}

impl ThenActionChain {
    fn chain_then_action(
        &mut self,
        g: &mut SemanticGraph,
        uri: &Url,
        container_prefix: Option<&str>,
        parent_id: &NodeId,
        then_action: &sysml_v2_parser::Node<ThenAction>,
    ) {
        let action = &then_action.value.action.value;
        let action_qualified = qualified_name_for_node(
            g,
            uri,
            Some(parent_id.qualified_name.as_str()),
            &action.name,
            "action",
        );
        let mut attrs = HashMap::new();
        attrs.insert(
            "actionType".to_string(),
            serde_json::json!(action.type_name.as_str()),
        );
        insert_action_payload_attrs(&mut attrs, &then_action.value.action.value);
        add_node_and_recurse(
            g,
            uri,
            &action_qualified,
            "action",
            action.name.clone(),
            span_to_range(&then_action.span),
            attrs,
            Some(parent_id),
        );
        add_typing_edge_if_exists(
            g,
            uri,
            &action_qualified,
            action.type_name.as_str(),
            container_prefix,
        );
        add_edge_if_both_exist(
            g,
            uri,
            &parent_id.qualified_name,
            &action_qualified,
            RelationshipKind::Perform,
        );
        if let Some(previous_action) = self.previous.as_ref() {
            add_edge_if_both_exist(
                g,
                uri,
                previous_action,
                &action_qualified,
                RelationshipKind::Flow,
            );
        }
        if let ActionUsageBody::Brace { elements } = &action.body {
            build_from_action_usage_body(
                elements,
                uri,
                Some(action_qualified.as_str()),
                &NodeId::new(uri, &action_qualified),
                g,
            );
        }
        self.previous = Some(action_qualified);
    }
}

fn add_in_out_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    in_out: &sysml_v2_parser::Node<sysml_v2_parser::ast::InOutDecl>,
) {
    let parameter = &in_out.value;
    let child_qualified = qualified_name_for_node(
        g,
        uri,
        container_prefix,
        &parameter.name,
        "in out parameter",
    );
    let mut attrs = HashMap::new();
    attrs.insert(
        "direction".to_string(),
        serde_json::json!(match parameter.direction {
            InOut::In => "in",
            InOut::Out => "out",
            InOut::InOut => "inout",
        }),
    );
    attrs.insert(
        "parameterType".to_string(),
        serde_json::json!(&parameter.type_name),
    );
    add_node_and_recurse(
        g,
        uri,
        &child_qualified,
        "in out parameter",
        parameter.name.clone(),
        span_to_range(&in_out.span),
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(
        g,
        uri,
        &child_qualified,
        &parameter.type_name,
        container_prefix,
    );
}

fn add_perform_step(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    perform: &sysml_v2_parser::Node<Perform>,
) {
    let step_name = if perform.value.action_name.trim().is_empty() {
        perform
            .value
            .type_name
            .clone()
            .unwrap_or_else(|| "perform".to_string())
    } else {
        perform.value.action_name.clone()
    };
    let child_qualified = qualified_name_for_node(
        g,
        uri,
        container_prefix,
        &step_name,
        "perform",
    );
    let mut attrs = HashMap::new();
    if let Some(ref action_type) = perform.value.type_name {
        attrs.insert("actionType".to_string(), serde_json::json!(action_type));
    }
    add_node_and_recurse(
        g,
        uri,
        &child_qualified,
        "perform",
        step_name,
        span_to_range(&perform.span),
        attrs,
        Some(parent_id),
    );
    if let Some(ref action_type) = perform.value.type_name {
        add_typing_edge_if_exists(g, uri, &child_qualified, action_type, container_prefix);
    }
}

fn add_ref_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    wrap: &sysml_v2_parser::Node<RefDecl>,
) {
    let n = &wrap.value;
    let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "ref");
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
        span_to_range(&wrap.span),
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
}

fn add_assign_stmt(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    assign: &sysml_v2_parser::Node<AssignStmt>,
) {
    let value = &assign.value;
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        "_assign",
        "assign",
    );
    let mut attrs = HashMap::new();
    attrs.insert("lhs".to_string(), serde_json::json!(value.lhs.as_str()));
    attrs.insert("rhs".to_string(), serde_json::json!(value.rhs.as_str()));
    attrs.insert("isThen".to_string(), serde_json::json!(value.is_then));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "assign",
        "assign".to_string(),
        span_to_range(&assign.span),
        attrs,
        Some(parent_id),
    );
}

fn add_state_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    su_node: &sysml_v2_parser::Node<StateUsage>,
) {
    let name = &su_node.value.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "state");
    let mut attrs = HashMap::new();
    if let Some(ref t) = su_node.value.type_name {
        attrs.insert("stateType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "state",
        name.clone(),
        span_to_range(&su_node.span),
        attrs,
        Some(parent_id),
    );
    let state_id = NodeId::new(uri, &qualified);
    if let Some(ref t) = su_node.value.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    if let StateDefBody::Brace { elements } = &su_node.value.body {
        state::build_from_state_body(elements, uri, Some(&qualified), &state_id, g);
    }
}

fn add_for_loop(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    for_loop: &sysml_v2_parser::Node<ForLoop>,
) {
    let fl = &for_loop.value;
    let qualified = qualified_name_for_node(
        g,
        uri,
        container_prefix,
        &format!("for_{}", fl.var),
        "for loop",
    );
    let mut attrs = HashMap::new();
    attrs.insert("loopVar".to_string(), serde_json::json!(&fl.var));
    attrs.insert("loopRange".to_string(), serde_json::json!(&fl.range));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "for loop",
        fl.var.clone(),
        span_to_range(&for_loop.span),
        attrs,
        Some(parent_id),
    );
    let loop_id = NodeId::new(uri, &qualified);
    if let ActionDefBody::Brace { elements } = &fl.body {
        build_from_action_def_body(elements, uri, container_prefix, &loop_id, g);
    }
}

fn add_action_body_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    decl: &sysml_v2_parser::Node<ActionBodyDecl>,
) {
    let d = &decl.value;
    let name = if d.text.trim().is_empty() {
        d.keyword.clone()
    } else {
        d.text.clone()
    };
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "action body decl");
    let mut attrs = HashMap::new();
    attrs.insert("keyword".to_string(), serde_json::json!(&d.keyword));
    attrs.insert("text".to_string(), serde_json::json!(&d.text));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "action body decl",
        name,
        span_to_range(&decl.span),
        attrs,
        Some(parent_id),
    );
}

fn materialize_nested_action_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    au_node: &ActionUsage,
    span: &sysml_v2_parser::Span,
    link_perform_from_parent: bool,
) -> String {
    let name = &au_node.name;
    let child_qualified = qualified_name_for_node(g, uri, container_prefix, name, "action");
    let mut attrs = HashMap::new();
    attrs.insert("actionType".to_string(), serde_json::json!(&au_node.type_name));
    insert_action_payload_attrs(&mut attrs, au_node);
    add_node_and_recurse(
        g,
        uri,
        &child_qualified,
        "action",
        name.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(
        g,
        uri,
        &child_qualified,
        &au_node.type_name,
        container_prefix,
    );
    if link_perform_from_parent {
        add_edge_if_both_exist(
            g,
            uri,
            &parent_id.qualified_name,
            &child_qualified,
            RelationshipKind::Perform,
        );
    }
    if let ActionUsageBody::Brace { elements } = &au_node.body {
        build_from_action_usage_body(
            elements,
            uri,
            Some(child_qualified.as_str()),
            &NodeId::new(uri, &child_qualified),
            g,
        );
    }
    child_qualified
}

pub(super) fn build_from_action_def_body(
    elements: &[sysml_v2_parser::Node<ActionDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let mut then_chain = ThenActionChain { previous: None };
    for element in elements {
        match &element.value {
            ActionDefBodyElement::InOutDecl(in_out) => {
                add_in_out_decl(g, uri, container_prefix, parent_id, in_out);
            }
            ActionDefBodyElement::Perform(perform) => {
                add_perform_step(g, uri, container_prefix, parent_id, perform);
            }
            ActionDefBodyElement::Bind(bind) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    &bind.value.left,
                    &bind.value.right,
                    RelationshipKind::Bind,
                );
            }
            ActionDefBodyElement::Flow(flow) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    &flow.value.from,
                    &flow.value.to,
                    RelationshipKind::Flow,
                );
            }
            ActionDefBodyElement::FirstStmt(first) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    &first.value.first,
                    &first.value.then,
                    RelationshipKind::Flow,
                );
            }
            ActionDefBodyElement::MergeStmt(merge) => {
                let merge_target = expressions::expression_to_debug_string(&merge.value.merge);
                let child_qualified = qualified_name_for_node(
                    g,
                    uri,
                    container_prefix,
                    &merge_target,
                    "merge",
                );
                let mut attrs = HashMap::new();
                attrs.insert("mergeTarget".to_string(), serde_json::json!(merge_target));
                add_node_and_recurse(
                    g,
                    uri,
                    &child_qualified,
                    "merge",
                    "merge".to_string(),
                    span_to_range(&merge.span),
                    attrs,
                    Some(parent_id),
                );
            }
            ActionDefBodyElement::ActionUsage(action_usage) => {
                let au_node = action_usage.as_ref();
                materialize_nested_action_usage(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &au_node.value,
                    &au_node.span,
                    true,
                );
            }
            ActionDefBodyElement::ThenAction(then_action) => {
                then_chain.chain_then_action(g, uri, container_prefix, parent_id, then_action);
            }
            ActionDefBodyElement::Assign(assign) => {
                add_assign_stmt(g, uri, parent_id, assign);
            }
            ActionDefBodyElement::RefDecl(ref_decl) => {
                add_ref_decl(g, uri, container_prefix, parent_id, ref_decl);
            }
            ActionDefBodyElement::StateUsage(state_usage) => {
                add_state_usage(g, uri, container_prefix, parent_id, state_usage);
            }
            ActionDefBodyElement::ForLoop(for_loop) => {
                add_for_loop(g, uri, container_prefix, parent_id, for_loop);
            }
            ActionDefBodyElement::Decl(decl) => {
                add_action_body_decl(g, uri, container_prefix, parent_id, decl);
            }
            ActionDefBodyElement::MetadataAnnotation(meta) => {
                super::metadata_def::add_metadata_annotation_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &meta.value,
                    &meta.span,
                );
            }
            ActionDefBodyElement::MetadataKeywordUsage(mk_node) => {
                super::metadata_keyword::add_metadata_keyword_node(
                    g,
                    uri,
                    parent_id,
                    &mk_node.value,
                    &mk_node.span,
                );
            }
            ActionDefBodyElement::Doc(_)
            | ActionDefBodyElement::Error(_)
            | ActionDefBodyElement::Annotation(_) => {}
        }
    }
}

pub(super) fn build_from_action_usage_body(
    elements: &[sysml_v2_parser::Node<ActionUsageBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let mut then_chain = ThenActionChain { previous: None };
    for element in elements {
        match &element.value {
            ActionUsageBodyElement::InOutDecl(in_out) => {
                add_in_out_decl(g, uri, container_prefix, parent_id, in_out);
            }
            ActionUsageBodyElement::Bind(bind) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    &bind.value.left,
                    &bind.value.right,
                    RelationshipKind::Bind,
                );
            }
            ActionUsageBodyElement::Flow(flow) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    &flow.value.from,
                    &flow.value.to,
                    RelationshipKind::Flow,
                );
            }
            ActionUsageBodyElement::FirstStmt(first) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    &first.value.first,
                    &first.value.then,
                    RelationshipKind::Flow,
                );
            }
            ActionUsageBodyElement::MergeStmt(merge) => {
                let merge_target = expressions::expression_to_debug_string(&merge.value.merge);
                let child_qualified = qualified_name_for_node(
                    g,
                    uri,
                    container_prefix,
                    &merge_target,
                    "merge",
                );
                let mut attrs = HashMap::new();
                attrs.insert("mergeTarget".to_string(), serde_json::json!(merge_target));
                add_node_and_recurse(
                    g,
                    uri,
                    &child_qualified,
                    "merge",
                    "merge".to_string(),
                    span_to_range(&merge.span),
                    attrs,
                    Some(parent_id),
                );
            }
            ActionUsageBodyElement::ActionUsage(action_usage) => {
                let au_node = action_usage.as_ref();
                materialize_nested_action_usage(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &au_node.value,
                    &au_node.span,
                    true,
                );
            }
            ActionUsageBodyElement::ThenAction(then_action) => {
                then_chain.chain_then_action(g, uri, container_prefix, parent_id, then_action);
            }
            ActionUsageBodyElement::Assign(assign) => {
                add_assign_stmt(g, uri, parent_id, assign);
            }
            ActionUsageBodyElement::RefDecl(ref_decl) => {
                add_ref_decl(g, uri, container_prefix, parent_id, ref_decl);
            }
            ActionUsageBodyElement::StateUsage(state_usage) => {
                add_state_usage(g, uri, container_prefix, parent_id, state_usage);
            }
            ActionUsageBodyElement::ForLoop(for_loop) => {
                add_for_loop(g, uri, container_prefix, parent_id, for_loop);
            }
            ActionUsageBodyElement::Decl(decl) => {
                add_action_body_decl(g, uri, container_prefix, parent_id, decl);
            }
            ActionUsageBodyElement::MetadataAnnotation(meta) => {
                super::metadata_def::add_metadata_annotation_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &meta.value,
                    &meta.span,
                );
            }
            ActionUsageBodyElement::MetadataKeywordUsage(mk_node) => {
                super::metadata_keyword::add_metadata_keyword_node(
                    g,
                    uri,
                    parent_id,
                    &mk_node.value,
                    &mk_node.span,
                );
            }
            ActionUsageBodyElement::Doc(_)
            | ActionUsageBodyElement::Error(_)
            | ActionUsageBodyElement::Annotation(_) => {}
        }
    }
}

pub(super) fn materialize_top_level_action_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    au_node: &sysml_v2_parser::Node<ActionUsage>,
) -> String {
    let usage = &au_node.value;
    let name = &usage.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "action");
    let mut attrs = HashMap::new();
    attrs.insert(
        "actionType".to_string(),
        serde_json::json!(&usage.type_name),
    );
    insert_action_payload_attrs(&mut attrs, usage);
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "action",
        name.clone(),
        span_to_range(&au_node.span),
        attrs,
        parent_id,
    );
    add_typing_edge_if_exists(g, uri, &qualified, &usage.type_name, container_prefix);
    if let ActionUsageBody::Brace { elements } = &usage.body {
        let action_id = NodeId::new(uri, &qualified);
        build_from_action_usage_body(elements, uri, Some(&qualified), &action_id, g);
    }
    qualified
}

pub(super) fn materialize_action_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    ad_node: &sysml_v2_parser::Node<sysml_v2_parser::ast::ActionDef>,
    name: &str,
    specializes: Option<&str>,
) -> String {
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "action def");
    let action_id = NodeId::new(uri, &qualified);
    let mut attrs = HashMap::new();
    if let Some(spec) = specializes.filter(|s| !s.trim().is_empty()) {
        attrs.insert("specializes".to_string(), serde_json::json!(spec));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "action def",
        name.to_string(),
        span_to_range(&ad_node.span),
        attrs,
        parent_id,
    );
    if let ActionDefBody::Brace { elements } = &ad_node.body {
        build_from_action_def_body(
            elements,
            uri,
            Some(&qualified),
            &action_id,
            g,
        );
    }
    qualified
}
