#![allow(dead_code)] // staged inherited-attribute lens helpers; wired fully in follow-up work

use crate::semantic::{RelationshipKind, SemanticNode};
use crate::workspace::ServerState;
use std::time::Instant;
use tower_lsp::lsp_types::{CodeLens, Url};
use tracing::info;

const MAX_INHERITED_ATTRIBUTE_LENSES: usize = 5;

pub(crate) fn build_code_lens(state: &ServerState, uri_norm: &Url) -> Vec<CodeLens> {
    let started_at = Instant::now();
    let indexed_symbols = state
        .symbol_table
        .iter()
        .filter(|s| s.uri == *uri_norm)
        .count();
    let elapsed_ms = started_at.elapsed().as_millis();
    if state.perf_logging_enabled && elapsed_ms >= 10 {
        info!(
            target: "kernel::lsp_runtime::symbols",
            event = "symbols:buildCodeLens",
            uri = %uri_norm,
            indexed_symbols,
            emitted_lenses = 0,
            elapsed_ms,
            "build_code_lens completed"
        );
    }
    Vec::new()
}

pub(crate) fn inherited_attributes_for_part_def<'a>(
    state: &'a ServerState,
    owner: &'a SemanticNode,
) -> Vec<InheritedAttributeLens> {
    if owner.element_kind != "part def" {
        return Vec::new();
    }
    let direct_attribute_names = direct_attribute_names(state, owner);
    let mut seen_names = std::collections::HashSet::<String>::new();
    let mut inherited = Vec::<InheritedAttributeLens>::new();
    let mut queue: Vec<_> = state
        .semantic_graph
        .outgoing_targets_by_kind(owner, RelationshipKind::Specializes)
        .into_iter()
        .map(|node| node.id.clone())
        .collect();
    let mut seen_ancestors = std::collections::HashSet::new();
    while let Some(ancestor_id) = queue.pop() {
        if !seen_ancestors.insert(ancestor_id.clone()) {
            continue;
        }
        let Some(ancestor) = state.semantic_graph.get_node(&ancestor_id) else {
            continue;
        };
        let next_targets = state
            .semantic_graph
            .outgoing_targets_by_kind(ancestor, RelationshipKind::Specializes);
        queue.extend(next_targets.into_iter().map(|node| node.id.clone()));
        for child in state.semantic_graph.children_of(ancestor) {
            if !is_attribute_like_kind(&child.element_kind) {
                continue;
            }
            let name = child.name.trim();
            if name.is_empty() {
                continue;
            }
            let normalized = name.to_lowercase();
            if direct_attribute_names.contains(&normalized) || !seen_names.insert(normalized) {
                continue;
            }
            inherited.push(InheritedAttributeLens {
                name: name.to_string(),
                type_name: attribute_type_name(state, child),
                declared_value: declared_value_text(child),
                effective_value: effective_value_text(child),
                declared_in: Some(ancestor.name.clone()).filter(|value| !value.trim().is_empty()),
            });
        }
    }
    inherited.sort_by(|left, right| {
        (
            left.declared_in.as_deref().unwrap_or(""),
            left.name.as_str(),
            left.type_name.as_deref().unwrap_or(""),
        )
            .cmp(&(
                right.declared_in.as_deref().unwrap_or(""),
                right.name.as_str(),
                right.type_name.as_deref().unwrap_or(""),
            ))
    });
    inherited
}

pub(crate) fn inherited_attribute_hint_lines(
    state: &ServerState,
    owner: &SemanticNode,
) -> Vec<String> {
    let inherited = inherited_attributes_for_part_def(state, owner);
    if inherited.is_empty() {
        return Vec::new();
    }
    let mut lines = inherited
        .iter()
        .take(MAX_INHERITED_ATTRIBUTE_LENSES)
        .map(format_inherited_attribute_hint)
        .collect::<Vec<_>>();
    if inherited.len() > MAX_INHERITED_ATTRIBUTE_LENSES {
        lines.push(format!(
            "inherited ... +{} more attribute(s)",
            inherited.len() - MAX_INHERITED_ATTRIBUTE_LENSES
        ));
    }
    lines
}

fn direct_attribute_names(
    state: &ServerState,
    owner: &SemanticNode,
) -> std::collections::HashSet<String> {
    state
        .semantic_graph
        .children_of(owner)
        .into_iter()
        .filter(|child| is_attribute_like_kind(&child.element_kind))
        .map(|child| child.name.to_lowercase())
        .collect()
}

fn is_attribute_like_kind(kind: &str) -> bool {
    let lower = kind.to_lowercase();
    lower.contains("attribute") || lower.contains("property")
}

fn attribute_type_name(state: &ServerState, node: &SemanticNode) -> Option<String> {
    state
        .semantic_graph
        .outgoing_targets_by_kind(node, RelationshipKind::Typing)
        .into_iter()
        .find_map(|target| Some(target.name.clone()).filter(|name| !name.trim().is_empty()))
        .or_else(|| {
            node.attributes
                .get("attributeType")
                .or_else(|| node.attributes.get("dataType"))
                .or_else(|| node.attributes.get("type"))
                .and_then(|value| value.as_str())
                .map(|raw| raw.split("::").last().unwrap_or(raw).to_string())
        })
}

fn declared_value_text(node: &SemanticNode) -> Option<String> {
    for key in ["value", "defaultValue", "literal"] {
        let Some(value) = node.attributes.get(key) else {
            continue;
        };
        if value.is_null() {
            continue;
        }
        return Some(value_to_display_text(value));
    }
    None
}

fn effective_value_text(node: &SemanticNode) -> Option<String> {
    let mut value = node
        .attributes
        .get("evaluatedValue")
        .or_else(|| node.attributes.get("value"))
        .or_else(|| node.attributes.get("defaultValue"))
        .or_else(|| node.attributes.get("literal"))
        .map(value_to_display_text)?;
    if let Some(unit) = node
        .attributes
        .get("evaluatedUnit")
        .and_then(|raw| raw.as_str())
        .map(str::trim)
        .filter(|unit| !unit.is_empty())
    {
        value.push_str(" [");
        value.push_str(unit);
        value.push(']');
    }
    Some(value)
}

fn value_to_display_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(v) => v.clone(),
        serde_json::Value::Number(v) => v.to_string(),
        serde_json::Value::Bool(v) => v.to_string(),
        serde_json::Value::Null => "null".to_string(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "<complex>".to_string()),
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InheritedAttributeLens {
    name: String,
    type_name: Option<String>,
    declared_value: Option<String>,
    effective_value: Option<String>,
    declared_in: Option<String>,
}

fn format_inherited_attribute_hint(attr: &InheritedAttributeLens) -> String {
    let mut text = format!("inherited {}", attr.name);
    let inferred_type = attr
        .type_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .or_else(|| infer_type_from_value(attr.effective_value.as_deref()))
        .or_else(|| infer_type_from_value(attr.declared_value.as_deref()));
    if let Some(type_name) = inferred_type.as_deref() {
        text.push_str(": ");
        text.push_str(type_name);
    }
    let declared = attr.declared_value.as_deref();
    let effective = attr
        .effective_value
        .as_deref()
        .or(declared)
        .unwrap_or("n/a");
    let declared_in = attr.declared_in.as_deref().unwrap_or("ancestor");
    text.push_str(" = ");
    text.push_str(effective);
    text.push_str(" (from ");
    text.push_str(declared_in);
    text.push(')');
    if let Some(declared) = declared.filter(|value| *value != effective) {
        text.push_str(" [declared ");
        text.push_str(declared);
        text.push(']');
    }
    truncate_hint_line(&text, 110)
}

fn infer_type_from_value(value: Option<&str>) -> Option<String> {
    let raw = value?;
    let start = raw.find('[')?;
    let end = raw[start + 1..].find(']')?;
    let symbol = raw[start + 1..start + 1 + end].trim();
    if symbol.is_empty() {
        return None;
    }
    Some(symbol.to_string())
}

fn truncate_hint_line(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut out = String::new();
    for ch in text.chars().take(max_chars.saturating_sub(1)) {
        out.push(ch);
    }
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::services::store_document_text;
    use crate::workspace::ServerState;
    use tower_lsp::lsp_types::Url;

    fn prepare_state(source: &str) -> (ServerState, Url) {
        let uri = Url::parse("file:///inherited-codelens-test.sysml").expect("uri");
        let mut state = ServerState::default();
        let warning = store_document_text(&mut state, &uri, source.to_string());
        assert!(warning.is_none(), "unexpected warning while storing source");
        (state, uri)
    }

    fn inherited_titles(state: &ServerState, uri: &Url) -> Vec<String> {
        let Some(owner) = state
            .semantic_graph
            .nodes_for_uri(uri)
            .into_iter()
            .find(|node| node.element_kind == "part def" && node.name == "Car")
        else {
            return Vec::new();
        };
        inherited_attribute_hint_lines(state, owner)
    }

    #[test]
    fn inherited_attribute_lenses_include_declared_effective_and_provenance() {
        let source = r#"
            package P {
                part def Base {
                    attribute mass = 1200 [kg];
                }
                part def Car :> Base;
            }
        "#;
        let (state, uri) = prepare_state(source);
        let titles = inherited_titles(&state, &uri);
        assert!(
            titles.iter().any(|title| {
                title.contains("inherited mass")
                    && title.contains("1200 [kg]")
                    && title.contains("from Base")
            }),
            "expected inherited mass lens with declared/effective/provenance, got {titles:#?}"
        );
    }

    #[test]
    fn direct_override_suppresses_inherited_duplicate_lens() {
        let source = r#"
            package P {
                part def Base {
                    attribute mass = 1200 [kg];
                }
                part def Car :> Base {
                    attribute mass = 1250 [kg];
                }
            }
        "#;
        let (state, uri) = prepare_state(source);
        let titles = inherited_titles(&state, &uri);
        assert!(
            titles.iter().all(|title| !title.contains("inherited mass")),
            "direct override should suppress inherited duplicate, got {titles:#?}"
        );
    }

    #[test]
    fn inherited_attribute_lenses_are_bounded_with_overflow_summary() {
        let source = r#"
            package P {
                part def Base {
                    attribute a1 = 1;
                    attribute a2 = 2;
                    attribute a3 = 3;
                    attribute a4 = 4;
                    attribute a5 = 5;
                    attribute a6 = 6;
                    attribute a7 = 7;
                }
                part def Car :> Base;
            }
        "#;
        let (state, uri) = prepare_state(source);
        let titles = inherited_titles(&state, &uri);
        let inherited_count = titles
            .iter()
            .filter(|title| title.starts_with("inherited a"))
            .count();
        assert_eq!(
            inherited_count, MAX_INHERITED_ATTRIBUTE_LENSES,
            "expected bounded inherited lens count"
        );
        assert!(
            titles
                .iter()
                .any(|title| title == "inherited ... +2 more attribute(s)"),
            "expected overflow summary lens, got {titles:#?}"
        );
    }
}
