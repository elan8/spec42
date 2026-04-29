use std::collections::HashMap;

use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::requirement_body::resolve_subject_type_target_qualified;
use super::{add_node_and_recurse, qualified_name_for_node};

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
                let target = resolve_subject_type_target_qualified(
                    g,
                    uri,
                    container_prefix,
                    sd.value.type_name.as_str(),
                );
                if let Some(target_qualified) = target {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        &parent_id.qualified_name,
                        &target_qualified,
                        RelationshipKind::Subject,
                    );
                }
            }
            UCBE::Objective(obj) => {
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    "_objective",
                    "objective",
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "objective",
                    "objective".to_string(),
                    span_to_range(&obj.span),
                    HashMap::new(),
                    Some(parent_id),
                );
            }
            UCBE::Error(_) | UCBE::Doc(_) => {}
            _ => {}
        }
    }
}
