//! Requirement (and concern) bodies: subject edges, frames, constraints, imports.

use std::collections::HashMap;
use std::fs;

use sysml_v2_parser::ast::{
    ConstraintDefBodyElement, InOut, InOutDecl, RequireConstraintBody, RequirementDefBody,
    RequirementDefBodyElement,
};
use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{
    add_edge_if_both_exist, add_typing_edge_if_exists, type_ref_candidates,
};

use super::expressions::expression_to_debug_string;
use super::{add_node_and_recurse, qualified_name_for_node};

const REQUIREMENT_CONSTRAINTS_ATTR: &str = "requirementConstraints";
const ANALYSIS_CONSTRAINTS_ATTR: &str = "analysisConstraints";

fn append_string_list_attribute(g: &mut SemanticGraph, node_id: &NodeId, key: &str, line: String) {
    let Some(node) = g.get_node_mut(node_id) else {
        return;
    };
    let entry = node
        .attributes
        .entry(key.to_string())
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    if !entry.is_array() {
        *entry = serde_json::Value::Array(Vec::new());
    }
    if let serde_json::Value::Array(lines) = entry {
        if !lines
            .iter()
            .any(|existing| existing.as_str() == Some(line.as_str()))
        {
            lines.push(serde_json::Value::String(line));
        }
    }
}

fn append_json_list_attribute(
    g: &mut SemanticGraph,
    node_id: &NodeId,
    key: &str,
    value: serde_json::Value,
) {
    let Some(node) = g.get_node_mut(node_id) else {
        return;
    };
    let entry = node
        .attributes
        .entry(key.to_string())
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    if !entry.is_array() {
        *entry = serde_json::Value::Array(Vec::new());
    }
    if let serde_json::Value::Array(values) = entry {
        if !values.iter().any(|existing| existing == &value) {
            values.push(value);
        }
    }
}

fn compact_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn text_from_span(uri: &Url, span: &sysml_v2_parser::Span) -> Option<String> {
    let path = uri.to_file_path().ok()?;
    let content = fs::read_to_string(path).ok()?;
    let range = span_to_range(span);
    let start = range.start.line as usize;
    let end = range.end.line as usize;
    let lines: Vec<&str> = content.lines().collect();
    if start >= lines.len() || end >= lines.len() || start > end {
        return None;
    }
    Some(compact_whitespace(&lines[start..=end].join(" ")))
}

fn format_constraint_parameter_line(param: &InOutDecl) -> String {
    let direction = match param.direction {
        InOut::In => "in",
        InOut::Out => "out",
        InOut::InOut => "inout",
    };
    let type_name = param.type_name.trim();
    if type_name.is_empty() {
        format!("  {} {}", direction, param.name)
    } else {
        let short_type = type_name.split("::").last().unwrap_or(type_name);
        format!("  {} {} : {}", direction, param.name, short_type)
    }
}

fn require_constraint_display_lines(body: &RequireConstraintBody) -> Vec<String> {
    match body {
        RequireConstraintBody::Semicolon => vec!["  require constraint;".to_string()],
        RequireConstraintBody::Brace { elements } => {
            let mut lines = Vec::new();
            for element in elements {
                match &element.value {
                    ConstraintDefBodyElement::Doc(doc) => {
                        let text = compact_whitespace(&doc.value.text);
                        if !text.is_empty() {
                            lines.push(format!("  {}", text));
                        }
                    }
                    ConstraintDefBodyElement::Expression(expr) => {
                        let text = compact_whitespace(&expression_to_debug_string(expr));
                        if !text.is_empty() {
                            lines.push(format!("  {}", text));
                        }
                    }
                    ConstraintDefBodyElement::InOutDecl(param) => {
                        lines.push(format_constraint_parameter_line(&param.value));
                    }
                    ConstraintDefBodyElement::Error(_) | ConstraintDefBodyElement::Other(_) => {}
                }
            }
            if lines.is_empty() {
                vec!["  require constraint".to_string()]
            } else {
                lines
            }
        }
    }
}

fn require_constraint_structured(uri: &Url, body: &RequireConstraintBody) -> Option<serde_json::Value> {
    let RequireConstraintBody::Brace { elements } = body else {
        return None;
    };
    let mut params = Vec::new();
    let mut expression_fragments: Vec<String> = Vec::new();
    for element in elements {
        match &element.value {
            ConstraintDefBodyElement::InOutDecl(param) => {
                let direction = match param.value.direction {
                    InOut::In => "in",
                    InOut::Out => "out",
                    InOut::InOut => "inout",
                };
                params.push(serde_json::json!({
                    "direction": direction,
                    "name": param.value.name,
                    "type": param.value.type_name,
                }));
            }
            ConstraintDefBodyElement::Expression(expr) => {
                let rendered = text_from_span(uri, &expr.span)
                    .unwrap_or_else(|| compact_whitespace(&expression_to_debug_string(expr)));
                if !rendered.is_empty() {
                    expression_fragments.push(rendered);
                }
            }
            ConstraintDefBodyElement::Doc(_)
            | ConstraintDefBodyElement::Error(_)
            | ConstraintDefBodyElement::Other(_) => {}
        }
    }
    let expression = compact_whitespace(&expression_fragments.join(" "));
    if expression.is_empty() {
        return None;
    }
    Some({
        serde_json::json!({
            "kind": "require_constraint",
            "params": params,
            "expression": expression,
        })
    })
}

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
    for element in elements {
        match &element.value {
            RequirementDefBodyElement::SubjectDecl(sd) => {
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
                    type_resolution_prefix,
                );
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
            RequirementDefBodyElement::RequireConstraint(rc) => {
                for line in require_constraint_display_lines(&rc.value.body) {
                    append_string_list_attribute(g, parent_id, REQUIREMENT_CONSTRAINTS_ATTR, line);
                }
                if let Some(constraint) = require_constraint_structured(uri, &rc.value.body) {
                    append_json_list_attribute(g, parent_id, ANALYSIS_CONSTRAINTS_ATTR, constraint);
                }
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
            RequirementDefBodyElement::AttributeDef(attr_def) => {
                let name = &attr_def.value.name;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    name,
                    "attribute def",
                );
                let mut attrs = HashMap::new();
                if let Some(ref typing) = attr_def.value.typing {
                    attrs.insert("attributeType".to_string(), serde_json::json!(typing));
                }
                if let Some(initializer) = extract_attribute_initializer_from_span(uri, &attr_def.span)
                {
                    attrs.insert("defaultValue".to_string(), serde_json::json!(initializer));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute def",
                    name.clone(),
                    span_to_range(&attr_def.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(ref typing) = attr_def.value.typing {
                    add_typing_edge_if_exists(
                        g,
                        uri,
                        &qualified,
                        typing,
                        type_resolution_prefix,
                    );
                }
            }
            RequirementDefBodyElement::AttributeUsage(attr_usage) => {
                let name = &attr_usage.value.name;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    name,
                    "attribute",
                );
                let mut attrs = HashMap::new();
                if let Some(ref redefines) = attr_usage.value.redefines {
                    attrs.insert("redefines".to_string(), serde_json::json!(redefines));
                }
                if let Some(ref value) = attr_usage.value.value {
                    attrs.insert(
                        "value".to_string(),
                        serde_json::json!(expression_to_debug_string(value)),
                    );
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute",
                    name.clone(),
                    span_to_range(&attr_usage.span),
                    attrs,
                    Some(parent_id),
                );
            }
            RequirementDefBodyElement::Doc(_)
            | RequirementDefBodyElement::Annotation(_)
            | RequirementDefBodyElement::VerifyRequirement(_)
            | RequirementDefBodyElement::Error(_)
            | RequirementDefBodyElement::Other(_) => {}
        }
    }
}

fn extract_attribute_initializer_from_span(uri: &Url, span: &sysml_v2_parser::Span) -> Option<String> {
    let path = uri.to_file_path().ok()?;
    let content = fs::read_to_string(path).ok()?;
    let range = span_to_range(span);
    let start = range.start.line as usize;
    let end = range.end.line as usize;
    let lines: Vec<&str> = content.lines().collect();
    if start >= lines.len() || end >= lines.len() || start > end {
        return None;
    }
    let snippet = lines[start..=end].join("\n");
    let equals_idx = snippet.find('=')?;
    let semicolon_idx = snippet[equals_idx + 1..].rfind(';')? + equals_idx + 1;
    let initializer = snippet[equals_idx + 1..semicolon_idx].trim();
    (!initializer.is_empty()).then_some(initializer.to_string())
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
