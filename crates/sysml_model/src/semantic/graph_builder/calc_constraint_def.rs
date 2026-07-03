//! Top-level `constraint def` and `calc def` package-body members.

use std::collections::HashMap;
use std::fs;

use sysml_v2_parser::ast::{CalcDef, CalcDefBody, CalcDefBodyElement, ConstraintDef, ConstraintDefBody, ConstraintDefBodyElement, InOut};
use sysml_v2_parser::Node;
use url::Url;

use super::{
    add_node_and_recurse, insert_def_specialization_attr, qualified_name_for_node,
    wire_def_specialization_edge,
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
                ConstraintDefBodyElement::Error(_)
                | ConstraintDefBodyElement::Doc(_)
                | ConstraintDefBodyElement::MetadataAnnotation(_)
                | ConstraintDefBodyElement::Other(_) => {}
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
                CalcDefBodyElement::Error(_)
                | CalcDefBodyElement::Doc(_)
                | CalcDefBodyElement::MetadataAnnotation(_) => {}
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
    let name = identification_name(&c_node.value.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "constraint def");
    let (params, expression) = extract_constraint_metadata(uri, &c_node.value.body);
    let mut attrs = HashMap::new();
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

pub(super) fn build_calc_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<CalcDef>,
) {
    let name = identification_name(&c_node.value.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "calc def");
    let (params, return_decl, expression) = extract_calc_metadata(uri, &c_node.value.body);
    let mut attrs = HashMap::new();
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
    // Wire InOutDecl / ReturnDecl as typed child graph nodes.
    if let CalcDefBody::Brace { elements } = &c_node.value.body {
        let calc_id = NodeId::new(uri, &qualified);
        for element in elements {
            match &element.value {
                CalcDefBodyElement::InOutDecl(in_out) => {
                    super::action::add_in_out_decl(g, uri, container_prefix, &calc_id, in_out);
                }
                CalcDefBodyElement::ReturnDecl(ret) => {
                    let ret_qualified = qualified_name_for_node(
                        g,
                        uri,
                        container_prefix,
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
                        Some(&calc_id),
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
                    super::attach_doc_comment(g, &calc_id, &doc.value.text);
                }
                CalcDefBodyElement::MetadataAnnotation(meta) => {
                    super::metadata_def::add_metadata_annotation_node(
                        g,
                        uri,
                        container_prefix,
                        &calc_id,
                        &meta.value,
                        &meta.span,
                    );
                }
                CalcDefBodyElement::Expression(_)
                | CalcDefBodyElement::Other(_)
                | CalcDefBodyElement::Error(_) => {}
            }
        }
    }
}
