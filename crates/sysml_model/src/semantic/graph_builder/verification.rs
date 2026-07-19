use std::collections::HashMap;

use sysml_v2_parser::ast::{
    Expression, RequirementDefBody, RequirementDefBodyElement, UseCaseDefBody,
    UseCaseDefBodyElement,
};
use url::Url;

use super::requirement_body::{add_verified_requirement_node, verify_requirement_target};
use super::use_case::{self, add_include_use_case_node};
use super::{add_node_and_recurse, expressions, qualified_name_for_node};
use crate::semantic::analysis_typing::strip_analysis_return_body;
use crate::semantic::ast_util::{span_to_range, typing_targets};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;
fn extract_verdict_kind_token(body_text: &str) -> Option<String> {
    let marker = "VerdictKind::";
    let start = body_text.find(marker)?;
    let token = body_text[start + marker.len()..]
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .next()
        .unwrap_or_default()
        .trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_ascii_lowercase())
    }
}

fn extract_verdict_kind_from_expression(expr: &Expression) -> Option<String> {
    match expr {
        Expression::MemberAccess(base, member) => {
            if let Expression::FeatureRef(prefix) = &base.value {
                if prefix == "VerdictKind" {
                    return Some(member.to_ascii_lowercase());
                }
            }
            None
        }
        Expression::FeatureRef(name) => {
            let marker = "VerdictKind::";
            let rest = name.strip_prefix(marker)?;
            let token = rest
                .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
                .next()
                .unwrap_or_default()
                .trim();
            if token.is_empty() {
                None
            } else {
                Some(token.to_ascii_lowercase())
            }
        }
        _ => None,
    }
}

fn extract_verdict_kind_from_return_ref(
    body: &str,
    return_expression: Option<&Expression>,
) -> Option<String> {
    return_expression
        .and_then(extract_verdict_kind_from_expression)
        .or_else(|| extract_verdict_kind_token(body))
}

pub(super) fn build_from_verification_body(
    body: &UseCaseDefBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let UseCaseDefBody::Brace { elements } = body else {
        return;
    };

    let mut chain = use_case::CaseSuccessionChain::new();
    let mut case_subject_qualified: Option<String> = None;
    let mut objective_node_ids: Vec<NodeId> = Vec::new();
    let mut has_subject = false;
    let mut verdict_count = 0usize;
    let mut then_action_count = 0usize;

    for node in elements {
        if use_case::wire_extended_case_body_element(
            g,
            uri,
            parent_id,
            node,
            container_prefix,
            Some(&mut chain),
        ) {
            has_subject = true;
            continue;
        }
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
                if case_subject_qualified.is_none() {
                    case_subject_qualified = Some(qualified.clone());
                }
                add_typing_edge_if_exists(
                    g,
                    uri,
                    &qualified,
                    sd.value.type_name.as_str(),
                    container_prefix,
                );
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
                let mut objective_attrs = HashMap::new();
                objective_attrs.insert(
                    "objectiveBindingKind".to_string(),
                    serde_json::json!("verification_subject"),
                );
                if let Some(bound_to) = case_subject_qualified.as_ref() {
                    objective_attrs
                        .insert("objectiveBoundTo".to_string(), serde_json::json!(bound_to));
                }
                if let Some(type_name) = objective.value.requirement.value.type_name.as_ref() {
                    objective_attrs
                        .insert("objectiveType".to_string(), serde_json::json!(type_name));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "objective",
                    objective_name.clone(),
                    span_to_range(&objective.span),
                    objective_attrs,
                    Some(parent_id),
                );
                objective_node_ids.push(NodeId::new(uri, &qualified));
                if let Some(type_name) = objective.value.requirement.value.type_name.as_ref() {
                    add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
                }
                let RequirementDefBody::Brace { elements } =
                    &objective.value.requirement.value.body
                else {
                    continue;
                };
                for body_element in elements {
                    if let RequirementDefBodyElement::VerifyRequirement(verify_node) =
                        &body_element.value
                    {
                        if let Some(requirement_ref) = verify_requirement_target(&verify_node.value)
                        {
                            add_verified_requirement_node(
                                g,
                                uri,
                                container_prefix,
                                parent_id,
                                &requirement_ref,
                                verify_node.value.explicit_requirement_keyword,
                                span_to_range(&verify_node.span),
                            );
                        }
                    }
                }
            }
            UseCaseDefBodyElement::ThenAction(then_action) => {
                then_action_count += 1;
                chain.chain_then_action(g, uri, container_prefix, parent_id, then_action);
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
                attrs.insert(
                    "lhs".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(&value.lhs)),
                );
                let rhs_text = expressions::expression_to_debug_string(&value.rhs);
                attrs.insert("rhs".to_string(), serde_json::json!(rhs_text));
                attrs.insert("isThen".to_string(), serde_json::json!(value.is_then));
                let rhs_trimmed = rhs_text.trim();
                attrs.insert(
                    "rhsIsBoolean".to_string(),
                    serde_json::json!(matches!(rhs_trimmed, "true" | "false")),
                );
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
            UseCaseDefBodyElement::ThenDone(done) => {
                verdict_count += 1;
                chain.chain_then_done(g, uri, parent_id, done);
            }
            UseCaseDefBodyElement::ReturnRef(return_ref) => {
                verdict_count += 1;
                let value = &return_ref.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &value.name,
                    "verdict",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "returnBody".to_string(),
                    serde_json::json!(value.body.as_str()),
                );
                if let Some(multiplicity) = value.multiplicity.as_deref() {
                    attrs.insert("multiplicity".to_string(), serde_json::json!(multiplicity));
                }
                let return_expr = value.return_expression.as_ref().map(|node| &node.value);
                if let Some(verdict_token) =
                    extract_verdict_kind_from_return_ref(value.body.as_str(), return_expr)
                {
                    attrs.insert(
                        "rawVerdictToken".to_string(),
                        serde_json::json!(verdict_token),
                    );
                }
                let verdict_range = value
                    .return_expression
                    .as_ref()
                    .map(|expr| span_to_range(&expr.span))
                    .unwrap_or_else(|| span_to_range(&return_ref.span));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "verdict",
                    value.name.clone(),
                    verdict_range,
                    attrs,
                    Some(parent_id),
                );
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
            UseCaseDefBodyElement::ThenIncludeUseCase(then_include) => {
                add_include_use_case_node(
                    g,
                    uri,
                    parent_id,
                    &then_include.value.include.value,
                    span_to_range(&then_include.span),
                    container_prefix,
                );
            }
            UseCaseDefBodyElement::IncludeUseCase(include_node) => {
                add_include_use_case_node(
                    g,
                    uri,
                    parent_id,
                    &include_node.value,
                    span_to_range(&include_node.span),
                    container_prefix,
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
                let typed_by = typing_targets(value.typing.as_deref());
                if !typed_by.is_empty() {
                    attrs.insert(
                        "attributeType".to_string(),
                        serde_json::json!(typed_by.join(", ")),
                    );
                }
                if let Some(expr_node) = &value.value {
                    let rendered =
                        super::expressions::expression_to_debug_string(&expr_node.value.expression);
                    attrs.insert("value".to_string(), serde_json::json!(rendered));
                    attrs.insert("defaultValue".to_string(), serde_json::json!(rendered));
                    attrs.insert(
                        "valueIsBoolean".to_string(),
                        serde_json::json!(super::expressions::expression_is_boolean_valued(
                            &expr_node.value.expression
                        )),
                    );
                }
                if let Some(span) = value.value_span.as_ref() {
                    attrs.insert(
                        "valueSpan".to_string(),
                        serde_json::json!(crate::semantic::ast_util::span_to_range(span)),
                    );
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
                for target in typing_targets(value.typing.as_deref()) {
                    add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
                }
            }
            UseCaseDefBodyElement::ActorUsage(_)
            | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
            | UseCaseDefBodyElement::FirstSuccession(_)
            | UseCaseDefBodyElement::ThenUseCaseUsage(_)
            | UseCaseDefBodyElement::RefRedefinition(_)
            | UseCaseDefBodyElement::SubjectRef(_) => {}
            UseCaseDefBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            UseCaseDefBodyElement::MetadataAnnotation(meta) => {
                super::metadata_def::add_metadata_annotation_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    &meta.value,
                    &meta.span,
                );
            }
            UseCaseDefBodyElement::Error(_)
            | UseCaseDefBodyElement::Other(_)
            | UseCaseDefBodyElement::ForLoop(_)
            | UseCaseDefBodyElement::Annotation(_)
            | UseCaseDefBodyElement::CaseReturnDecl(_) => {}
            UseCaseDefBodyElement::FlowUsage(flow) => {
                super::flow_usage::materialize_flow_usage(
                    flow,
                    uri,
                    container_prefix,
                    parent_id,
                    g,
                );
            }
            UseCaseDefBodyElement::MetadataKeywordUsage(mk_node) => {
                super::metadata_keyword::add_metadata_keyword_node(
                    g,
                    uri,
                    parent_id,
                    &mk_node.value,
                    &mk_node.span,
                );
            }
        }
    }

    let objective_count = objective_node_ids.len();
    if let Some(bound_to) = case_subject_qualified.as_ref() {
        for objective_id in &objective_node_ids {
            if let Some(objective_node) = g.get_node_mut(objective_id) {
                objective_node
                    .attributes
                    .insert("objectiveBoundTo".to_string(), serde_json::json!(bound_to));
            }
        }
    }

    if let Some(parent_node) = g.get_node_mut(parent_id) {
        parent_node
            .attributes
            .insert("hasSubject".to_string(), serde_json::json!(has_subject));
        parent_node.attributes.insert(
            "objectiveCount".to_string(),
            serde_json::json!(objective_count),
        );
        parent_node
            .attributes
            .insert("verdictCount".to_string(), serde_json::json!(verdict_count));
        parent_node.attributes.insert(
            "thenActionCount".to_string(),
            serde_json::json!(then_action_count),
        );
    }
}
