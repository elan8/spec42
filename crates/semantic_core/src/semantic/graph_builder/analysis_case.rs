use std::collections::{HashMap, HashSet};

use sysml_v2_parser::ast::{UseCaseDefBody, UseCaseDefBodyElement};
use url::Url;

use super::{add_node_and_recurse, expressions, qualified_name_for_node};
use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::{
    add_typing_edge_if_exists, resolve_type_target_in_workspace, SPECIALIZES_TARGET_KINDS,
};

pub(super) fn build_from_analysis_body(
    body: &UseCaseDefBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let UseCaseDefBody::Brace { elements } = body else {
        return;
    };

    let mut analysis_result_qualified: Option<String> = None;
    let mut objective_node_ids: Vec<NodeId> = Vec::new();
    let mut has_subject = false;

    for node in elements {
        match &node.value {
            UseCaseDefBodyElement::SubjectDecl(sd) => {
                has_subject = true;
                let name = sd.value.name.clone();
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &name,
                    "subject",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "subjectType".to_string(),
                    serde_json::json!(sd.value.type_name.as_str()),
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "subject",
                    name,
                    span_to_range(&sd.span),
                    attrs,
                    Some(parent_id),
                );
                add_typing_edge_if_exists(
                    g,
                    uri,
                    &qualified,
                    sd.value.type_name.as_str(),
                    container_prefix,
                );
            }
            UseCaseDefBodyElement::ReturnRef(return_ref) => {
                let value = &return_ref.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &value.name,
                    "analysis result",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "returnBody".to_string(),
                    serde_json::json!(value.body.as_str()),
                );
                if let Some(multiplicity) = value.multiplicity.as_deref() {
                    attrs.insert("multiplicity".to_string(), serde_json::json!(multiplicity));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "analysis result",
                    value.name.clone(),
                    span_to_range(&return_ref.span),
                    attrs,
                    Some(parent_id),
                );
                if analysis_result_qualified.is_none() {
                    analysis_result_qualified = Some(qualified);
                    let expression = strip_analysis_return_body(value.body.as_str());
                    if !expression.is_empty() {
                        if let Some(parent_node) = g.get_node_mut(parent_id) {
                            parent_node.attributes.insert(
                                "analysisExpression".to_string(),
                                serde_json::json!(expression),
                            );
                        }
                    }
                }
            }
            UseCaseDefBodyElement::Objective(objective) => {
                let objective_name = &objective.value.requirement.value.name;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    objective_name,
                    "objective",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "objectiveBindingKind".to_string(),
                    serde_json::json!("analysis_result"),
                );
                if let Some(bound_to) = analysis_result_qualified.as_ref() {
                    attrs.insert("objectiveBoundTo".to_string(), serde_json::json!(bound_to));
                }
                if let Some(type_name) = objective.value.requirement.value.type_name.as_ref() {
                    attrs.insert("objectiveType".to_string(), serde_json::json!(type_name));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "objective",
                    objective_name.clone(),
                    span_to_range(&objective.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(type_name) = objective.value.requirement.value.type_name.as_ref() {
                    add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
                }
                objective_node_ids.push(NodeId::new(uri, &qualified));
            }
            UseCaseDefBodyElement::Assign(assign) => {
                let value = &assign.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    "_verify",
                    "verify",
                );
                let mut attrs = HashMap::new();
                attrs.insert("lhs".to_string(), serde_json::json!(value.lhs.as_str()));
                attrs.insert("rhs".to_string(), serde_json::json!(value.rhs.as_str()));
                attrs.insert("isThen".to_string(), serde_json::json!(value.is_then));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "verify",
                    "verify".to_string(),
                    span_to_range(&assign.span),
                    attrs,
                    Some(parent_id),
                );
            }
            UseCaseDefBodyElement::AttributeDef(attribute) => {
                let value = &attribute.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &value.name,
                    "attribute def",
                );
                let mut attrs = HashMap::new();
                if let Some(ref typing) = value.typing {
                    attrs.insert("attributeType".to_string(), serde_json::json!(typing));
                }
                if let Some(expr_node) = &value.value {
                    let rendered = expressions::expression_to_debug_string(expr_node);
                    attrs.insert("value".to_string(), serde_json::json!(rendered));
                    attrs.insert("defaultValue".to_string(), serde_json::json!(rendered));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute def",
                    value.name.clone(),
                    span_to_range(&attribute.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(ref typing) = value.typing {
                    add_typing_edge_if_exists(g, uri, &qualified, typing, container_prefix);
                }
            }
            UseCaseDefBodyElement::Other(text) => {
                for parsed in parse_analysis_attributes_from_other(text) {
                    let qualified = qualified_name_for_node(
                        g,
                        uri,
                        Some(parent_id.qualified_name.as_str()),
                        &parsed.name,
                        parsed.kind,
                    );
                    let mut attrs = HashMap::new();
                    if let Some(typing) = parsed.typing.as_ref() {
                        attrs.insert("attributeType".to_string(), serde_json::json!(typing));
                    }
                    if let Some(value) = parsed.value {
                        attrs.insert(
                            if parsed.kind == "attribute def" {
                                "defaultValue".to_string()
                            } else {
                                "value".to_string()
                            },
                            serde_json::json!(value),
                        );
                    }
                    add_node_and_recurse(
                        g,
                        uri,
                        &qualified,
                        parsed.kind,
                        parsed.name,
                        span_to_range(&node.span),
                        attrs,
                        Some(parent_id),
                    );
                    if let Some(typing) = parsed.typing.as_ref() {
                        add_typing_edge_if_exists(g, uri, &qualified, typing, container_prefix);
                    }
                }
            }
            UseCaseDefBodyElement::Error(_)
            | UseCaseDefBodyElement::Doc(_)
            | UseCaseDefBodyElement::SubjectRef(_)
            | UseCaseDefBodyElement::ActorUsage(_)
            | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
            | UseCaseDefBodyElement::FirstSuccession(_)
            | UseCaseDefBodyElement::ThenDone(_)
            | UseCaseDefBodyElement::ThenIncludeUseCase(_)
            | UseCaseDefBodyElement::ThenUseCaseUsage(_)
            | UseCaseDefBodyElement::IncludeUseCase(_)
            | UseCaseDefBodyElement::RefRedefinition(_)
            | UseCaseDefBodyElement::ForLoop(_)
            | UseCaseDefBodyElement::ThenAction(_) => {}
        }
    }

    let objective_count = objective_node_ids.len();
    let had_local_result = analysis_result_qualified.is_some();
    let bound_to =
        analysis_result_qualified.or_else(|| inherited_analysis_result_qualified(g, parent_id));
    if let Some(bound_to) = bound_to.as_ref() {
        for objective_id in &objective_node_ids {
            if let Some(objective_node) = g.get_node_mut(objective_id) {
                objective_node
                    .attributes
                    .insert("objectiveBoundTo".to_string(), serde_json::json!(bound_to));
            }
        }
    }
    if !had_local_result {
        if let Some(inherited_expression) =
            inherited_analysis_expression(g, parent_id, bound_to.as_deref())
        {
            if let Some(parent_node) = g.get_node_mut(parent_id) {
                parent_node.attributes.insert(
                    "analysisExpression".to_string(),
                    serde_json::json!(inherited_expression),
                );
            }
        }
    }

    let analysis_result_count = g
        .get_node(parent_id)
        .map(|parent| {
            g.children_of(parent)
                .into_iter()
                .filter(|child| child.element_kind == "analysis result")
                .count()
        })
        .unwrap_or(0);
    if let Some(parent_node) = g.get_node_mut(parent_id) {
        parent_node.attributes.insert(
            "hasSubject".to_string(),
            serde_json::json!(has_subject),
        );
        parent_node.attributes.insert(
            "objectiveCount".to_string(),
            serde_json::json!(objective_count),
        );
        parent_node.attributes.insert(
            "analysisResultCount".to_string(),
            serde_json::json!(analysis_result_count),
        );
    }
}

/// Walks `:>` on analysis definitions and returns the first inherited `analysis result`.
fn inherited_analysis_result_qualified(
    g: &SemanticGraph,
    analysis_def_id: &NodeId,
) -> Option<String> {
    let mut current_id = analysis_def_id.clone();
    let mut seen = HashSet::new();
    loop {
        let specializes_ref = g
            .get_node(&current_id)?
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())?;
        if !seen.insert(current_id.clone()) {
            return None;
        }
        let current = g.get_node(&current_id)?.clone();
        let target_id = resolve_type_target_in_workspace(
            g,
            &current,
            specializes_ref,
            SPECIALIZES_TARGET_KINDS,
        )?;
        let target = g.get_node(&target_id)?;
        if target.element_kind != "analysis def" {
            return None;
        }
        for child in g.children_of(target) {
            if child.element_kind == "analysis result" {
                return Some(child.id.qualified_name.clone());
            }
        }
        current_id = target_id;
    }
}

fn inherited_analysis_expression(
    g: &SemanticGraph,
    analysis_def_id: &NodeId,
    inherited_result_qualified: Option<&str>,
) -> Option<String> {
    if let Some(result_qualified) = inherited_result_qualified {
        let result_id = NodeId::new(&analysis_def_id.uri, result_qualified);
        if let Some(result_node) = g.get_node(&result_id) {
            if let Some(body) = result_node
                .attributes
                .get("returnBody")
                .and_then(|value| value.as_str())
            {
                let expression = strip_analysis_return_body(body);
                if !expression.is_empty() {
                    return Some(expression);
                }
            }
        }
    }
    let mut current_id = analysis_def_id.clone();
    let mut seen = HashSet::new();
    loop {
        let specializes_ref = g
            .get_node(&current_id)?
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())?;
        if !seen.insert(current_id.clone()) {
            return None;
        }
        let current = g.get_node(&current_id)?.clone();
        let target_id = resolve_type_target_in_workspace(
            g,
            &current,
            specializes_ref,
            SPECIALIZES_TARGET_KINDS,
        )?;
        let target = g.get_node(&target_id)?;
        if target.element_kind != "analysis def" {
            return None;
        }
        if let Some(expression) = target
            .attributes
            .get("analysisExpression")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Some(expression.to_string());
        }
        current_id = target_id;
    }
}

fn strip_analysis_return_body(body: &str) -> String {
    let mut trimmed = body.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        trimmed = trimmed[1..trimmed.len() - 1].trim();
    }
    let without_return = trimmed
        .strip_prefix("return")
        .map(str::trim)
        .unwrap_or(trimmed);
    without_return.trim_end_matches(';').trim().to_string()
}

#[derive(Debug, Clone)]
struct ParsedAnalysisAttribute {
    name: String,
    kind: &'static str,
    typing: Option<String>,
    value: Option<String>,
}

fn parse_analysis_attributes_from_other(text: &str) -> Vec<ParsedAnalysisAttribute> {
    text.lines()
        .filter_map(parse_analysis_attribute_line)
        .collect()
}

fn parse_analysis_attribute_line(line: &str) -> Option<ParsedAnalysisAttribute> {
    let line = line.trim();
    if !line.starts_with("attribute ") {
        return None;
    }
    let body = line
        .trim_start_matches("attribute ")
        .trim_end_matches(';')
        .trim();
    if body.is_empty() {
        return None;
    }
    let (lhs, rhs) = if let Some((lhs, rhs)) = body.split_once('=') {
        (lhs.trim(), Some(rhs.trim().to_string()))
    } else {
        (body, None)
    };
    let (name, typing, kind) = if let Some((name, ty)) = lhs.split_once(':') {
        (
            name.trim().to_string(),
            Some(ty.trim().to_string()),
            "attribute def",
        )
    } else {
        (lhs.trim().to_string(), None, "attribute")
    };
    if name.is_empty() {
        return None;
    }
    Some(ParsedAnalysisAttribute {
        name,
        kind,
        typing,
        value: rhs,
    })
}
