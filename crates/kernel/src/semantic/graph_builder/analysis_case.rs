use std::collections::HashMap;

use sysml_v2_parser::ast::{UseCaseDefBody, UseCaseDefBodyElement};
use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::requirement_body::resolve_subject_type_target_qualified;
use super::{add_node_and_recurse, qualified_name_for_node};

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
                attrs.insert("returnBody".to_string(), serde_json::json!(value.body.as_str()));
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
            UseCaseDefBodyElement::Error(_)
            | UseCaseDefBodyElement::Doc(_)
            | UseCaseDefBodyElement::Other(_)
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

    if let Some(bound_to) = analysis_result_qualified.as_ref() {
        for objective_id in objective_node_ids {
            if let Some(objective_node) = g.get_node_mut(&objective_id) {
                objective_node.attributes.insert(
                    "objectiveBoundTo".to_string(),
                    serde_json::json!(bound_to),
                );
            }
        }
    }
}
