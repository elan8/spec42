//! Requirement (and concern) bodies: subject edges, frames, constraints, imports.

use std::collections::HashMap;

use sysml_parser::ast::{RequirementDefBody, RequirementDefBodyElement};
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{add_edge_if_both_exist, type_ref_candidates};

use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn import_member_label(target: &str) -> String {
    let t = target.trim();
    if t.is_empty() {
        return "import".to_string();
    }
    let base = t.rsplit("::").next().unwrap_or(t);
    let base = base.trim_end_matches("::*");
    if base.len() > 48 {
        format!("import_{}", &base[..48])
    } else {
        base.to_string()
    }
}

/// Walks a requirement-style body and adds structural nodes plus subject relationship edges.
pub(super) fn walk_requirement_def_body(
    g: &mut SemanticGraph,
    uri: &Url,
    type_resolution_prefix: Option<&str>,
    subject_edge_source_qualified: &str,
    parent_id: &NodeId,
    body: &RequirementDefBody,
) {
    let RequirementDefBody::Brace { elements } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        match &element.value {
            RequirementDefBodyElement::SubjectDecl(sd) => {
                let target = resolve_subject_type_target_qualified(
                    g,
                    uri,
                    type_resolution_prefix,
                    sd.value.type_name.as_str(),
                );
                if let Some(target_qualified) = target {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        subject_edge_source_qualified,
                        &target_qualified,
                        RelationshipKind::Subject,
                    );
                }
            }
            RequirementDefBodyElement::RequireConstraint(_) => {
                let name = format!("requireConstraint{}", i);
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &name,
                    "require constraint",
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "require constraint",
                    name,
                    span_to_range(&element.span),
                    HashMap::new(),
                    Some(parent_id),
                );
            }
            RequirementDefBodyElement::Frame(f) => {
                let frame = &f.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &frame.name,
                    "frame",
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "frame",
                    frame.name.clone(),
                    span_to_range(&f.span),
                    HashMap::new(),
                    Some(parent_id),
                );
                let frame_id = NodeId::new(uri, &qualified);
                walk_requirement_def_body(
                    g,
                    uri,
                    type_resolution_prefix,
                    &qualified,
                    &frame_id,
                    &frame.body,
                );
            }
            RequirementDefBodyElement::Import(imp) => {
                let v = &imp.value;
                let name = import_member_label(&v.target);
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &name,
                    "import",
                );
                let mut attrs = HashMap::new();
                attrs.insert("importTarget".to_string(), serde_json::json!(&v.target));
                attrs.insert("importAll".to_string(), serde_json::json!(v.is_import_all));
                if let Some(vis) = &v.visibility {
                    attrs.insert(
                        "visibility".to_string(),
                        serde_json::json!(format!("{vis:?}")),
                    );
                }
                attrs.insert("recursive".to_string(), serde_json::json!(v.is_recursive));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "import",
                    name,
                    span_to_range(&imp.span),
                    attrs,
                    Some(parent_id),
                );
            }
            RequirementDefBodyElement::Doc(_)
            | RequirementDefBodyElement::Error(_) => {}
        }
    }
}

pub(super) fn resolve_subject_type_target_qualified(
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
        .or_else(|| {
            candidates
                .iter()
                .rev()
                .find(|candidate| candidate.contains("::"))
                .cloned()
                .or_else(|| candidates.last().cloned())
        })
}
