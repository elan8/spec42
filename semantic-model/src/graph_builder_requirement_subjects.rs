use tower_lsp::lsp_types::Url;

use sysml_parser::ast::{RequirementDefBody, RequirementDefBodyElement};

use crate::relationships::{add_edge_if_both_exist, type_ref_candidates};
use crate::{NodeId, RelationshipKind, SemanticGraph};

pub(crate) fn add_requirement_subject_edges(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    requirement_qualified: &str,
    body: &RequirementDefBody,
) {
    let RequirementDefBody::Brace { elements } = body else {
        return;
    };
    for element in elements {
        let RequirementDefBodyElement::SubjectDecl(subject_decl) = &element.value else {
            continue;
        };
        let target = resolve_subject_type_target_qualified(
            g,
            uri,
            container_prefix,
            subject_decl.value.type_name.as_str(),
        );
        if let Some(target_qualified) = target {
            add_edge_if_both_exist(
                g,
                uri,
                requirement_qualified,
                &target_qualified,
                RelationshipKind::Subject,
            );
        }
    }
}

fn resolve_subject_type_target_qualified(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    type_ref: &str,
) -> Option<String> {
    let normalized = type_ref.trim();
    if normalized.is_empty() {
        return None;
    }
    let allowed_target_kinds: &[&str] = &[
        "part def",
        "port def",
        "interface",
        "item def",
        "attribute def",
        "requirement def",
        "action def",
        "actor def",
        "occurrence def",
        "flow def",
        "allocation def",
        "state def",
        "use case def",
        "concern def",
    ];
    let kind_suffixes: &[&str] = &[
        "part_def",
        "port_def",
        "requirement_def",
        "action_def",
        "actor_def",
        "occurrence_def",
        "flow_def",
        "allocation_def",
        "state_def",
        "use_case_def",
        "concern_def",
        "item_def",
        "attribute_def",
    ];
    let candidates = type_ref_candidates(container_prefix, normalized);
    for base in &candidates {
        let mut expanded = vec![base.clone()];
        expanded.extend(
            kind_suffixes
                .iter()
                .map(|suffix| format!("{base}#{suffix}")),
        );
        for candidate in expanded {
            let node_id = NodeId::new(uri, &candidate);
            let Some(node) = g.get_node(&node_id) else {
                continue;
            };
            if allowed_target_kinds.contains(&node.element_kind.as_str()) {
                return Some(candidate);
            }
        }
    }
    let suffixes: Vec<String> = std::iter::once(format!("::{}", normalized))
        .chain(
            kind_suffixes
                .iter()
                .map(|suffix| format!("::{}#{}", normalized, suffix)),
        )
        .collect();
    g.nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            node_id.qualified_name == normalized
                || suffixes
                    .iter()
                    .any(|suffix| node_id.qualified_name.ends_with(suffix))
        })
        .filter_map(|node_id| {
            let node = g.get_node(node_id)?;
            if allowed_target_kinds.contains(&node.element_kind.as_str()) {
                Some(node_id.qualified_name.clone())
            } else {
                None
            }
        })
        .min_by_key(|qualified| qualified.len())
}
