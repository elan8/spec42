use std::collections::HashMap;

use sysml_v2_parser::ast::{
    RequirementDefBody, RequirementDefBodyElement, UseCaseDefBody, UseCaseDefBodyElement,
    VerifyRequirementMember,
};
use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::requirement_body::resolve_subject_type_target_qualified;
use super::{add_node_and_recurse, qualified_name_for_node};

fn verify_requirement_target(member: &VerifyRequirementMember) -> Option<String> {
    if let Some(requirement) = member.requirement.as_ref() {
        return Some(requirement.value.name.clone());
    }
    member.target.clone().and_then(|target| {
        let normalized = target.trim();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized.to_string())
        }
    })
}

fn add_verified_requirement_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    requirement_ref: &str,
    explicit_requirement_keyword: bool,
    span: tower_lsp::lsp_types::Range,
) {
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        requirement_ref,
        "verified requirement",
    );
    let mut attrs = HashMap::new();
    attrs.insert(
        "verifiedRequirement".to_string(),
        serde_json::json!(requirement_ref),
    );
    attrs.insert(
        "explicitRequirementKeyword".to_string(),
        serde_json::json!(explicit_requirement_keyword),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "verified requirement",
        requirement_ref.to_string(),
        span,
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(
        g,
        uri,
        &qualified,
        requirement_ref,
        container_prefix,
    );
}

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

    let mut previous_then_action: Option<String> = None;

    for node in elements {
        match &node.value {
            UseCaseDefBodyElement::SubjectDecl(sd) => {
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
                if let Some(target_qualified) = resolve_subject_type_target_qualified(
                    g,
                    uri,
                    container_prefix,
                    sd.value.type_name.as_str(),
                ) {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        &parent_id.qualified_name,
                        &target_qualified,
                        RelationshipKind::Subject,
                    );
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
                let mut objective_attrs = HashMap::new();
                if let Some(type_name) = objective.value.requirement.value.type_name.as_ref() {
                    objective_attrs.insert("objectiveType".to_string(), serde_json::json!(type_name));
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
                        if let Some(requirement_ref) =
                            verify_requirement_target(&verify_node.value)
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
                if let Some(previous_action) = previous_then_action.as_ref() {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        previous_action,
                        &action_qualified,
                        RelationshipKind::Flow,
                    );
                }
                previous_then_action = Some(action_qualified);
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
            UseCaseDefBodyElement::ThenDone(done) => {
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    "_verdict",
                    "verdict",
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "verdict",
                    "done".to_string(),
                    span_to_range(&done.span),
                    HashMap::new(),
                    Some(parent_id),
                );
            }
            UseCaseDefBodyElement::ReturnRef(return_ref) => {
                let value = &return_ref.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &value.name,
                    "verdict",
                );
                let mut attrs = HashMap::new();
                attrs.insert("returnBody".to_string(), serde_json::json!(value.body.as_str()));
                if let Some(multiplicity) = value.multiplicity.as_deref() {
                    attrs.insert("multiplicity".to_string(), serde_json::json!(multiplicity));
                }
                if let Some(verdict_token) = extract_verdict_kind_token(value.body.as_str()) {
                    attrs.insert(
                        "rawVerdictToken".to_string(),
                        serde_json::json!(verdict_token),
                    );
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "verdict",
                    value.name.clone(),
                    span_to_range(&return_ref.span),
                    attrs,
                    Some(parent_id),
                );
            }
            UseCaseDefBodyElement::Error(_)
            | UseCaseDefBodyElement::Doc(_)
            | UseCaseDefBodyElement::Other(_)
            | UseCaseDefBodyElement::SubjectRef(_)
            | UseCaseDefBodyElement::ActorUsage(_)
            | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
            | UseCaseDefBodyElement::FirstSuccession(_)
            | UseCaseDefBodyElement::ThenIncludeUseCase(_)
            | UseCaseDefBodyElement::ThenUseCaseUsage(_)
            | UseCaseDefBodyElement::IncludeUseCase(_)
            | UseCaseDefBodyElement::RefRedefinition(_)
            | UseCaseDefBodyElement::ForLoop(_) => {}
        }
    }
}
