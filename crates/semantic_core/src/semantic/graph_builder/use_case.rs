use std::collections::HashMap;

use url::Url;

use super::{add_node_and_recurse, qualified_name_for_node};
use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;

pub(super) fn build_from_use_case_body(
    elements: &[sysml_v2_parser::Node<sysml_v2_parser::ast::UseCaseDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::UseCaseDefBodyElement as UCBE;
    for node in elements {
        match &node.value {
            UCBE::ActorUsage(actor_node) => {
                let name = &actor_node.name;
                let qualified = qualified_name_for_node(g, uri, container_prefix, name, "actor");
                let range = span_to_range(&actor_node.span);
                let mut attrs = HashMap::new();
                attrs.insert(
                    "actorType".to_string(),
                    serde_json::json!(&actor_node.type_name),
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "actor",
                    name.clone(),
                    range,
                    attrs,
                    Some(parent_id),
                );
                add_typing_edge_if_exists(
                    g,
                    uri,
                    &qualified,
                    &actor_node.type_name,
                    container_prefix,
                );
            }
            UCBE::SubjectDecl(sd) => {
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
            UCBE::Objective(obj) => {
                let objective_name = &obj.value.requirement.value.name;
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
                    serde_json::json!("case_result_default"),
                );
                if let Some(type_name) = obj.value.requirement.value.type_name.as_ref() {
                    attrs.insert("objectiveType".to_string(), serde_json::json!(type_name));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "objective",
                    objective_name.clone(),
                    span_to_range(&obj.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(type_name) = obj.value.requirement.value.type_name.as_ref() {
                    add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
                }
            }
            UCBE::Error(_) | UCBE::Doc(_) => {}
            _ => {}
        }
    }
}
