//! Top-level `constraint def` and `calc def` package-body members.

use std::collections::HashMap;
use std::fs;

use sysml_v2_parser::ast::{
    CalcDef, CalcDefBody, CalcDefBodyElement, ConstraintDef, ConstraintDefBody,
    ConstraintDefBodyElement, ConstraintUsage, InOut,
};
use sysml_v2_parser::Node;
use url::Url;

use super::{
    add_node_and_recurse, insert_def_specialization_attr, qualified_name_for_node,
    resolve_addressable_name, wire_def_specialization_edge,
};
use crate::semantic::ast_util::{attach_short_name_attribute, identification_name, span_to_range};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::expressions;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;

fn direction_to_str(direction: &InOut) -> &'static str {
    match direction {
        InOut::In => "in",
        InOut::Out => "out",
        InOut::InOut => "inout",
    }
}

fn compact_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn expression_text_from_span(uri: &Url, span: &sysml_v2_parser::Span, fallback: &str) -> String {
    let Some(path) = uri.to_file_path().ok() else {
        return fallback.to_string();
    };
    let Ok(content) = fs::read_to_string(path) else {
        return fallback.to_string();
    };
    let range = span_to_range(span);
    let start = range.start.line as usize;
    let end = range.end.line as usize;
    let lines: Vec<&str> = content.lines().collect();
    if start >= lines.len() || end >= lines.len() || start > end {
        return fallback.to_string();
    }
    compact_whitespace(&lines[start..=end].join(" "))
}

fn extract_constraint_metadata(
    uri: &Url,
    body: &ConstraintDefBody,
) -> (Vec<serde_json::Value>, Option<String>) {
    let mut params = Vec::new();
    let mut expression: Option<String> = None;
    if let ConstraintDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                ConstraintDefBodyElement::InOutDecl(param) => params.push(serde_json::json!({
                    "direction": direction_to_str(&param.value.direction),
                    "name": param.value.name,
                    "type": param.value.type_name,
                })),
                ConstraintDefBodyElement::Expression(expr) => {
                    let rendered = expression_text_from_span(
                        uri,
                        &expr.span,
                        &expressions::expression_to_debug_string(expr),
                    );
                    if !rendered.trim().is_empty() {
                        expression = Some(rendered);
                    }
                }
                // Nested constraint/attribute content isn't summarized here -- this function
                // only extracts the flat params/expression pair for the `analysis*` attrs below.
                ConstraintDefBodyElement::Error(_)
                | ConstraintDefBodyElement::Doc(_)
                | ConstraintDefBodyElement::MetadataAnnotation(_)
                | ConstraintDefBodyElement::Other(_)
                | ConstraintDefBodyElement::Constraint(_)
                | ConstraintDefBodyElement::AttributeUsage(_) => {}
            }
        }
    }
    (params, expression)
}

fn strip_calc_return_expression(text: &str) -> String {
    text.trim()
        .strip_prefix("return")
        .map(str::trim)
        .unwrap_or(text.trim())
        .trim_end_matches(';')
        .trim()
        .to_string()
}

fn extract_calc_metadata(
    uri: &Url,
    body: &CalcDefBody,
) -> (
    Vec<serde_json::Value>,
    Option<serde_json::Value>,
    Option<String>,
) {
    let mut params = Vec::new();
    let mut return_decl: Option<serde_json::Value> = None;
    let mut expression: Option<String> = None;
    if let CalcDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                CalcDefBodyElement::InOutDecl(param) => params.push(serde_json::json!({
                    "direction": direction_to_str(&param.value.direction),
                    "name": param.value.name,
                    "type": param.value.type_name,
                })),
                CalcDefBodyElement::ReturnDecl(ret) => {
                    return_decl = Some(serde_json::json!({
                        "name": ret.value.name,
                        "type": ret.value.type_name,
                    }));
                }
                CalcDefBodyElement::Expression(expr) => {
                    let rendered = expression_text_from_span(
                        uri,
                        &expr.span,
                        &expressions::expression_to_debug_string(expr),
                    );
                    let rendered = strip_calc_return_expression(&rendered);
                    if !rendered.is_empty() {
                        expression = Some(rendered);
                    }
                }
                CalcDefBodyElement::Other(preview) => {
                    let rendered = strip_calc_return_expression(preview);
                    if expression.is_none() && !rendered.is_empty() {
                        expression = Some(rendered);
                    }
                }
                // Nested calc/part content isn't summarized here -- this function only extracts
                // the flat params/return/expression triple for the `analysis*` attrs below.
                CalcDefBodyElement::Error(_)
                | CalcDefBodyElement::Doc(_)
                | CalcDefBodyElement::MetadataAnnotation(_)
                | CalcDefBodyElement::CalcUsage(_)
                | CalcDefBodyElement::CalcDef(_)
                | CalcDefBodyElement::PartUsage(_) => {}
            }
        }
    }
    (params, return_decl, expression)
}

pub(super) fn build_constraint_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<ConstraintDef>,
) {
    let mut attrs = HashMap::new();
    let name = resolve_addressable_name(
        &identification_name(&c_node.value.identification),
        "constraint def",
        &mut attrs,
    );
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "constraint def");
    let (params, expression) = extract_constraint_metadata(uri, &c_node.value.body);
    attrs.insert(
        "analysisKind".to_string(),
        serde_json::json!("constraint_def"),
    );
    attrs.insert(
        "analysisParams".to_string(),
        serde_json::Value::Array(params),
    );
    if let Some(expr) = expression {
        attrs.insert("analysisExpression".to_string(), serde_json::json!(expr));
    }
    insert_def_specialization_attr(&mut attrs, c_node.value.specializes.as_deref());
    attach_short_name_attribute(&mut attrs, &c_node.value.identification);
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&c_node.value.membership),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "constraint def",
        name,
        span_to_range(&c_node.span),
        attrs,
        parent_id,
    );
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        c_node.value.specializes.as_deref(),
    );
    let constraint_id = NodeId::new(uri, &qualified);
    super::metadata_def::wire_constraint_body_metadata(
        g,
        uri,
        container_prefix,
        &constraint_id,
        &c_node.value.body,
    );
}

/// `constraint` usage: package-level only (`sysml-v2-parser` 0.40.0 added the `ConstraintUsage`
/// AST node; see its doc comment for the real-library forms it covers). Mirrors
/// [`build_constraint_def`]'s metadata extraction -- same `ConstraintDefBody` type -- but reads
/// `ConstraintUsage`'s plain `name`/`type_name` fields (no `Identification`/short name, matching
/// the parser struct) and wires a typing edge when `type_name` is present.
pub(super) fn build_constraint_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<ConstraintUsage>,
) {
    let mut attrs = HashMap::new();
    let name = resolve_addressable_name(&c_node.value.name, "constraint", &mut attrs);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "constraint");
    let (params, expression) = extract_constraint_metadata(uri, &c_node.value.body);
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&c_node.value.membership),
    );
    attrs.insert(
        "analysisKind".to_string(),
        serde_json::json!("constraint_usage"),
    );
    attrs.insert(
        "analysisParams".to_string(),
        serde_json::Value::Array(params),
    );
    if let Some(expr) = expression {
        attrs.insert("analysisExpression".to_string(), serde_json::json!(expr));
    }
    if let Some(ref t) = c_node.value.type_name {
        attrs.insert("constraintType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "constraint",
        name,
        span_to_range(&c_node.span),
        attrs,
        parent_id,
    );
    if let Some(ref t) = c_node.value.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    let constraint_id = NodeId::new(uri, &qualified);
    super::metadata_def::wire_constraint_body_metadata(
        g,
        uri,
        container_prefix,
        &constraint_id,
        &c_node.value.body,
    );
}

pub(super) fn build_calc_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<CalcDef>,
) {
    let mut attrs = HashMap::new();
    let name = resolve_addressable_name(
        &identification_name(&c_node.value.identification),
        "calc def",
        &mut attrs,
    );
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "calc def");
    let (params, return_decl, expression) = extract_calc_metadata(uri, &c_node.value.body);
    attrs.insert("analysisKind".to_string(), serde_json::json!("calc_def"));
    let params_json = serde_json::Value::Array(params.clone());
    attrs.insert("analysisParams".to_string(), params_json.clone());
    attrs.insert("parameters".to_string(), params_json);
    if let Some(ret) = return_decl {
        attrs.insert("analysisReturn".to_string(), ret);
    }
    if let Some(expr) = expression {
        attrs.insert("analysisExpression".to_string(), serde_json::json!(expr));
    }
    attach_short_name_attribute(&mut attrs, &c_node.value.identification);
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&c_node.value.membership),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "calc def",
        name,
        span_to_range(&c_node.span),
        attrs,
        parent_id,
    );
    let calc_id = NodeId::new(uri, &qualified);
    build_calc_def_body_elements(g, uri, container_prefix, &calc_id, &c_node.value.body);
}

/// Shared child-element walker for a `calc`/`calc def`'s own body: `in`/`out`/`return`
/// parameters, doc/metadata annotations, nested `part`/`calc`/`calc def` content. Used by both
/// [`build_calc_def`] (top-level `calc def`) and `part_def.rs`'s `PDBE::CalcUsage` arm (a `calc`
/// usage nested inside a `part def` body) so the two don't hand-roll the same loop twice.
pub(super) fn build_calc_def_body_elements(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    calc_id: &NodeId,
    body: &CalcDefBody,
) {
    let CalcDefBody::Brace { elements } = body else {
        return;
    };
    for element in elements {
        match &element.value {
            CalcDefBodyElement::InOutDecl(in_out) => {
                super::action::add_in_out_decl(g, uri, container_prefix, calc_id, in_out);
            }
            CalcDefBodyElement::ReturnDecl(ret) => {
                let ret_qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(calc_id.qualified_name.as_str()),
                    &ret.value.name,
                    "return parameter",
                );
                let mut ret_attrs = HashMap::new();
                ret_attrs.insert("direction".to_string(), serde_json::json!("return"));
                ret_attrs.insert(
                    "parameterType".to_string(),
                    serde_json::json!(&ret.value.type_name),
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &ret_qualified,
                    "return parameter",
                    ret.value.name.clone(),
                    span_to_range(&ret.span),
                    ret_attrs,
                    Some(calc_id),
                );
                add_typing_edge_if_exists(
                    g,
                    uri,
                    &ret_qualified,
                    &ret.value.type_name,
                    container_prefix,
                );
            }
            CalcDefBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, calc_id, &doc.value.text);
            }
            CalcDefBodyElement::MetadataAnnotation(meta) => {
                super::metadata_def::add_metadata_annotation_node(
                    g,
                    uri,
                    container_prefix,
                    calc_id,
                    &meta.value,
                    &meta.span,
                );
            }
            // Directed `in part …` parameter (validation `10b`) -- materialize the same way
            // every other calc/action/attribute body already does.
            CalcDefBodyElement::PartUsage(part) => {
                super::usage_builders::materialize_part_usage(
                    part,
                    uri,
                    container_prefix,
                    Some(calc_id),
                    g,
                );
            }
            // Nested `calc` usage inside a calc body (validation `10b` rollups).
            CalcDefBodyElement::CalcUsage(nested) => {
                materialize_calc_usage(g, uri, container_prefix, calc_id, nested);
            }
            // Nested `calc def` inside a calc body.
            CalcDefBodyElement::CalcDef(nested) => {
                build_calc_def(g, uri, container_prefix, Some(calc_id), nested);
            }
            CalcDefBodyElement::Expression(_)
            | CalcDefBodyElement::Other(_)
            | CalcDefBodyElement::Error(_) => {}
        }
    }
}

/// Materializes a `calc` usage (named, optionally typed, with its own body) as a child of
/// `parent_id`: a "calc" node plus recursion into its body via [`build_calc_def_body_elements`].
/// Shared by every body kind a `calc` usage can nest in directly -- `part_def.rs`'s top-level
/// `PDBE::CalcUsage`, this module's own nested-`calc`-inside-`calc` rollups (validation `10b`),
/// and analysis/verification case bodies' `UseCaseDefBodyElement::CalcUsage`.
pub(super) fn materialize_calc_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    calc_node: &Node<sysml_v2_parser::ast::CalcUsage>,
) {
    let name = identification_name(&calc_node.value.identification);
    let qualified = qualified_name_for_node(g, uri, Some(&parent_id.qualified_name), &name, "calc");
    let range = span_to_range(&calc_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &calc_node.value.identification);
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&calc_node.value.membership),
    );
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
    let calc_id = NodeId::new(uri, &qualified);
    build_calc_def_body_elements(g, uri, container_prefix, &calc_id, &calc_node.value.body);
}
