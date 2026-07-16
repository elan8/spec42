//! Requirement (and concern) bodies: subject edges, frames, constraints, imports.

use std::collections::HashMap;
use std::fs;

use sysml_v2_parser::ast::{
    ConstraintDefBodyElement, InOut, InOutDecl, RequireConstraintBody, RequirementDefBody,
    RequirementDefBodyElement, VerifyRequirementMember,
};
use url::Url;

use crate::semantic::ast_util::{span_to_range, text_range_to_json};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::import_resolution::resolve_type_reference_targets;
use crate::semantic::kinds::VERIFIED_REQUIREMENT_TARGET_KINDS;
use crate::semantic::model::{ElementKind, NodeId, RelationshipKind};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};
use crate::semantic::text_span::TextRange;

use super::expressions::expression_to_debug_string;
use super::metadata_keyword::add_metadata_keyword_node;
use super::{add_node_and_recurse, qualified_name_for_node};
use crate::semantic::ast_util::identification_name;

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
    let end = span.offset.checked_add(span.len)?;
    if let Some(snippet) = content.get(span.offset..end) {
        return Some(compact_whitespace(snippet));
    }

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
                    ConstraintDefBodyElement::MetadataAnnotation(_) => {}
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

fn require_constraint_structured(
    uri: &Url,
    body: &RequireConstraintBody,
) -> Option<serde_json::Value> {
    let RequireConstraintBody::Brace { elements } = body else {
        return None;
    };
    let mut params = Vec::new();
    let mut expression_fragments: Vec<String> = Vec::new();
    let mut doc_fragments: Vec<String> = Vec::new();
    let mut metadata_names: Vec<String> = Vec::new();
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
            ConstraintDefBodyElement::Doc(doc) => {
                let text = compact_whitespace(&doc.value.text);
                if !text.is_empty() {
                    doc_fragments.push(text);
                }
            }
            ConstraintDefBodyElement::MetadataAnnotation(meta) => {
                metadata_names.push(meta.value.name.clone());
            }
            ConstraintDefBodyElement::Error(_) | ConstraintDefBodyElement::Other(_) => {}
        }
    }
    let expression = compact_whitespace(&expression_fragments.join(" "));
    if expression.is_empty() {
        return None;
    }
    let doc = doc_fragments.join("\n\n");
    Some({
        serde_json::json!({
            "kind": "require_constraint",
            "params": params,
            "expression": expression,
            "doc": doc,
            "metadata": metadata_names,
        })
    })
}

fn fallback_verified_requirement_target(parent_id: &NodeId, requirement_ref: &str) -> String {
    if requirement_ref.contains("::") {
        requirement_ref.to_string()
    } else {
        parent_id
            .qualified_name
            .rsplit_once("::")
            .map(|(owner, _)| format!("{owner}::{requirement_ref}"))
            .unwrap_or_else(|| requirement_ref.to_string())
    }
}

pub(super) fn verify_requirement_target(member: &VerifyRequirementMember) -> Option<String> {
    if let Some(requirement) = member.requirement.as_ref() {
        if let Some(type_name) = requirement.value.type_name.as_deref() {
            let normalized = type_name.trim();
            if !normalized.is_empty() {
                return Some(normalized.to_string());
            }
        }
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

pub(super) fn add_verified_requirement_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    requirement_ref: &str,
    explicit_requirement_keyword: bool,
    span: TextRange,
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
    add_typing_edge_if_exists(g, uri, &qualified, requirement_ref, container_prefix);
    let requirement_target = if let Some(parent) = g.get_node(parent_id) {
        resolve_type_reference_targets(
            g,
            parent,
            requirement_ref,
            VERIFIED_REQUIREMENT_TARGET_KINDS,
        )
        .into_iter()
        .next()
        .map(|id| id.qualified_name.clone())
        .unwrap_or_else(|| fallback_verified_requirement_target(parent_id, requirement_ref))
    } else {
        fallback_verified_requirement_target(parent_id, requirement_ref)
    };
    add_edge_if_both_exist(
        g,
        uri,
        &parent_id.qualified_name,
        &requirement_target,
        RelationshipKind::Subject,
    );
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
    _subject_edge_source_qualified: &str,
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
                add_edge_if_both_exist(
                    g,
                    uri,
                    &parent_id.qualified_name,
                    &qualified,
                    RelationshipKind::Subject,
                );
            }
            RequirementDefBodyElement::RequirementActorDecl(ad) => {
                let name = ad.value.name.clone();
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &name,
                    "actor",
                );
                let mut attrs = HashMap::new();
                attrs.insert(
                    "actorType".to_string(),
                    serde_json::json!(ad.value.type_name.as_str()),
                );
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "actor",
                    name,
                    span_to_range(&ad.span),
                    attrs,
                    Some(parent_id),
                );
                add_typing_edge_if_exists(
                    g,
                    uri,
                    &qualified,
                    ad.value.type_name.as_str(),
                    type_resolution_prefix,
                );
            }
            RequirementDefBodyElement::RequireConstraint(rc) => {
                for line in require_constraint_display_lines(&rc.value.body) {
                    append_string_list_attribute(g, parent_id, REQUIREMENT_CONSTRAINTS_ATTR, line);
                }
                let structured = require_constraint_structured(uri, &rc.value.body);
                if let Some(ref constraint) = structured {
                    append_json_list_attribute(
                        g,
                        parent_id,
                        ANALYSIS_CONSTRAINTS_ATTR,
                        constraint.clone(),
                    );
                }
                let constraint_index = g
                    .get_node(parent_id)
                    .map(|parent| {
                        g.children_of(parent)
                            .iter()
                            .filter(|child| child.element_kind == ElementKind::RequireConstraint)
                            .count()
                    })
                    .unwrap_or(0);
                let constraint_name = format!("_requireConstraint_{constraint_index}");
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &constraint_name,
                    "require constraint",
                );
                let mut attrs = HashMap::new();
                if let Some(constraint) = structured {
                    if let Some(obj) = constraint.as_object() {
                        for (key, value) in obj {
                            attrs.insert(key.clone(), value.clone());
                        }
                    }
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "require constraint",
                    constraint_name,
                    span_to_range(&rc.span),
                    attrs,
                    Some(parent_id),
                );
                let constraint_id = NodeId::new(uri, &qualified);
                super::metadata_def::wire_require_constraint_body_metadata(
                    g,
                    uri,
                    type_resolution_prefix,
                    &constraint_id,
                    &rc.value.body,
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
                if let Some(vis) = &v.membership.visibility {
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
                if let Some(value_expr) = &attr_def.value.value {
                    let rendered = expression_to_debug_string(&value_expr.value.expression);
                    if !rendered.is_empty() {
                        attrs.insert("value".to_string(), serde_json::json!(rendered));
                        attrs.insert("defaultValue".to_string(), serde_json::json!(rendered));
                    }
                } else if let Some(initializer) =
                    extract_attribute_initializer_from_span(uri, &attr_def.span)
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
                    add_typing_edge_if_exists(g, uri, &qualified, typing, type_resolution_prefix);
                }
            }
            RequirementDefBodyElement::AttributeUsage(attr_usage) => {
                let name = super::effective_usage_name(
                    &attr_usage.value.name,
                    attr_usage.value.redefines.as_deref(),
                );
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    name,
                    "attribute",
                );
                let mut attrs = HashMap::new();
                if let Some(typing) =
                    crate::semantic::ast_util::typing_target(attr_usage.value.typing.as_deref())
                {
                    attrs.insert("attributeType".to_string(), serde_json::json!(typing));
                }
                if let Some(redefines) = crate::semantic::ast_util::subsetting_target(
                    attr_usage.value.redefines.as_deref(),
                ) {
                    attrs.insert("redefines".to_string(), serde_json::json!(redefines));
                }
                if let Some(ref value) = attr_usage.value.value {
                    attrs.insert(
                        "value".to_string(),
                        serde_json::json!(expression_to_debug_string(&value.value.expression)),
                    );
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute",
                    name.to_string(),
                    span_to_range(&attr_usage.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(typing) =
                    crate::semantic::ast_util::typing_target(attr_usage.value.typing.as_deref())
                {
                    add_typing_edge_if_exists(g, uri, &qualified, typing, type_resolution_prefix);
                }
            }
            RequirementDefBodyElement::MetadataKeywordUsage(mk_node) => {
                add_metadata_keyword_node(g, uri, parent_id, &mk_node.value, &mk_node.span);
            }
            RequirementDefBodyElement::Stakeholder(stakeholder) => {
                let s = &stakeholder.value;
                if let Some(ref type_name) = s.type_name {
                    let qualified = qualified_name_for_node(
                        g,
                        uri,
                        Some(parent_id.qualified_name.as_str()),
                        &s.name,
                        "stakeholder",
                    );
                    let mut attrs = HashMap::new();
                    attrs.insert("stakeholderType".to_string(), serde_json::json!(type_name));
                    add_node_and_recurse(
                        g,
                        uri,
                        &qualified,
                        "stakeholder",
                        s.name.clone(),
                        span_to_range(&stakeholder.span),
                        attrs,
                        Some(parent_id),
                    );
                    add_typing_edge_if_exists(
                        g,
                        uri,
                        &qualified,
                        type_name,
                        type_resolution_prefix,
                    );
                } else {
                    let qualified = qualified_name_for_node(
                        g,
                        uri,
                        Some(parent_id.qualified_name.as_str()),
                        &format!("_stakeholder_{}", s.name),
                        "stakeholder",
                    );
                    let mut attrs = HashMap::new();
                    attrs.insert("refTarget".to_string(), serde_json::json!(&s.name));
                    add_node_and_recurse(
                        g,
                        uri,
                        &qualified,
                        "stakeholder",
                        s.name.clone(),
                        span_to_range(&stakeholder.span),
                        attrs,
                        Some(parent_id),
                    );
                }
            }
            RequirementDefBodyElement::Purpose(purpose) => {
                let p = &purpose.value;
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &format!("_purpose_{}", p.target),
                    "purpose",
                );
                let mut attrs = HashMap::new();
                attrs.insert("refTarget".to_string(), serde_json::json!(&p.target));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "purpose",
                    p.target.clone(),
                    span_to_range(&purpose.span),
                    attrs,
                    Some(parent_id),
                );
            }
            RequirementDefBodyElement::TextualRep(t) => {
                let tr = &t.value;
                let name = tr
                    .rep_identification
                    .as_ref()
                    .map(identification_name)
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "_textualRep".to_string());
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    Some(parent_id.qualified_name.as_str()),
                    &name,
                    "textualRep",
                );
                let mut attrs = HashMap::new();
                attrs.insert("language".to_string(), serde_json::json!(&tr.language));
                attrs.insert("text".to_string(), serde_json::json!(&tr.text));
                if let Some(ref language_span) = tr.language_span {
                    attrs.insert(
                        "languageSpan".to_string(),
                        text_range_to_json(span_to_range(language_span)),
                    );
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "textualRep",
                    name,
                    span_to_range(&t.span),
                    attrs,
                    Some(parent_id),
                );
            }
            RequirementDefBodyElement::VerifyRequirement(verify_node) => {
                if let Some(requirement_ref) = verify_requirement_target(&verify_node.value) {
                    add_verified_requirement_node(
                        g,
                        uri,
                        type_resolution_prefix,
                        parent_id,
                        &requirement_ref,
                        verify_node.value.explicit_requirement_keyword,
                        span_to_range(&verify_node.span),
                    );
                }
            }
            RequirementDefBodyElement::MetadataAnnotation(meta) => {
                super::metadata_def::add_metadata_annotation_node(
                    g,
                    uri,
                    type_resolution_prefix,
                    parent_id,
                    &meta.value,
                    &meta.span,
                );
            }
            RequirementDefBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            RequirementDefBodyElement::Annotation(_)
            | RequirementDefBodyElement::Error(_)
            | RequirementDefBodyElement::Other(_) => {}
        }
    }
}

/// Walk constraint body elements from a braced `satisfy` body (parser 0.27.0+).
///
/// `satisfy … { … }` bodies expose structured constraint members. Since a satisfy
/// statement maps to a graph edge (not a node), we cannot attach child nodes here.
/// This function is a placeholder that acknowledges the structured data without
/// dropping it silently — full wiring (parameter nodes, expression diagnostics)
/// can be added once satisfy gains a dedicated graph node.
pub(super) fn walk_satisfy_constraint_elements(
    _elements: &[sysml_v2_parser::Node<ConstraintDefBodyElement>],
    _uri: &Url,
    _container_prefix: Option<&str>,
    _g: &mut SemanticGraph,
) {
}

fn extract_attribute_initializer_from_span(
    uri: &Url,
    span: &sysml_v2_parser::Span,
) -> Option<String> {
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
