use std::collections::HashSet;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::common::util;
use crate::language::{completion_prefix, keyword_doc, line_prefix_at_position};
use crate::workspace::ServerState;

use super::shared::TYPE_LOOKUP_KINDS;

const PART_TYPE_LOOKUP_KINDS: &[&str] = &["part def"];
const PORT_TYPE_LOOKUP_KINDS: &[&str] = &["port def", "interface"];
const ATTRIBUTE_TYPE_LOOKUP_KINDS: &[&str] = &[
    "attribute def",
    "item def",
    "enum def",
    "occurrence def",
    "kermlDecl",
];
const BODY_CONTEXT_KEYWORDS: &[&str] = &[
    "package",
    "part",
    "port",
    "attribute",
    "action",
    "requirement",
    "interface",
    "item",
];
const DECLARATION_MODIFIER_KEYWORDS: &[&str] = &["def"];

const TIER_CONTEXTUAL_SNIPPET: i32 = 7000;
const TIER_EXACT_SEMANTIC: i32 = 6000;
const TIER_CONTEXT_COMPATIBLE_SAME_SCOPE: i32 = 5000;
const TIER_SAME_FILE_COMPATIBLE: i32 = 4000;
const TIER_WORKSPACE_COMPATIBLE: i32 = 3000;
const TIER_KEYWORD_FALLBACK: i32 = 2000;
const TIER_GENERIC_SYMBOL: i32 = 1000;

const COMPLETION_RESOLVE_DATA_KEY: &str = "spec42Completion";

#[derive(Debug, Clone, PartialEq, Eq)]
enum CompletionContext {
    TopLevelKeyword {
        prefix: String,
    },
    TypeReference {
        prefix: String,
        qualifier: Option<String>,
        expected_kinds: &'static [&'static str],
    },
    QualifiedReference {
        prefix: String,
        qualifier: String,
    },
    MemberReference {
        prefix: String,
        receiver: String,
    },
    DeclarationName {
        prefix: String,
    },
    DeclarationModifier {
        prefix: String,
        keyword: String,
    },
    BodyStatement {
        prefix: String,
    },
    General {
        prefix: String,
    },
}

impl CompletionContext {
    fn prefix(&self) -> &str {
        match self {
            CompletionContext::TopLevelKeyword { prefix }
            | CompletionContext::TypeReference { prefix, .. }
            | CompletionContext::QualifiedReference { prefix, .. }
            | CompletionContext::MemberReference { prefix, .. }
            | CompletionContext::DeclarationName { prefix }
            | CompletionContext::DeclarationModifier { prefix, .. }
            | CompletionContext::BodyStatement { prefix }
            | CompletionContext::General { prefix } => prefix,
        }
    }
}

#[derive(Debug, Clone)]
struct CompletionCandidate {
    label: String,
    item: CompletionItem,
    tier: i32,
    score: i32,
}

#[derive(Debug, Default)]
struct CompletionSemanticHints {
    same_file_uri: Option<Url>,
    preferred_names: HashSet<String>,
    container_names: HashSet<String>,
}

#[derive(Debug, Clone)]
struct CompletionEditShape {
    replace_range: Range,
}

#[derive(Debug, Clone, serde::Serialize)]
struct CompletionResolveData {
    detail: Option<String>,
    documentation: Option<String>,
}

fn detect_completion_context(line_prefix: &str) -> CompletionContext {
    if let Some(keyword) = detect_trailing_keyword_modifier_context(line_prefix) {
        return CompletionContext::DeclarationModifier {
            prefix: String::new(),
            keyword,
        };
    }

    let trimmed = line_prefix.trim_end();
    if trimmed.is_empty() {
        return CompletionContext::TopLevelKeyword {
            prefix: String::new(),
        };
    }
    if trimmed.trim_start().starts_with("//") {
        return CompletionContext::General {
            prefix: completion_prefix(trimmed).to_string(),
        };
    }

    let (token_start, raw_token) = completion_token(trimmed);
    let before_token = &trimmed[..token_start];

    if let Some((expected_kinds, qualifier, prefix)) =
        detect_type_reference_context(before_token, raw_token)
    {
        return CompletionContext::TypeReference {
            prefix,
            qualifier,
            expected_kinds,
        };
    }

    if let Some((keyword, prefix)) = detect_declaration_modifier_context(before_token, raw_token) {
        return CompletionContext::DeclarationModifier { prefix, keyword };
    }

    if let Some(prefix) = detect_declaration_name_context(before_token, raw_token) {
        return CompletionContext::DeclarationName { prefix };
    }

    if let Some((receiver, prefix)) = raw_token.rsplit_once('.') {
        if !receiver.is_empty() {
            return CompletionContext::MemberReference {
                prefix: prefix.to_string(),
                receiver: receiver.to_string(),
            };
        }
    }

    if let Some((qualifier, prefix)) = raw_token.rsplit_once("::") {
        if !qualifier.is_empty() {
            return CompletionContext::QualifiedReference {
                prefix: prefix.to_string(),
                qualifier: qualifier.to_string(),
            };
        }
    }

    if before_token.trim().is_empty() {
        return CompletionContext::TopLevelKeyword {
            prefix: raw_token.to_string(),
        };
    }

    if before_token.trim_end().ends_with('{') {
        return CompletionContext::BodyStatement {
            prefix: raw_token.to_string(),
        };
    }

    CompletionContext::General {
        prefix: raw_token.to_string(),
    }
}

fn completion_token(trimmed_line_prefix: &str) -> (usize, &str) {
    fn is_completion_token_char(c: char) -> bool {
        c.is_alphanumeric() || matches!(c, '_' | ':' | '.' | '>')
    }

    let mut start = trimmed_line_prefix.len();
    for (idx, ch) in trimmed_line_prefix.char_indices().rev() {
        if is_completion_token_char(ch) {
            start = idx;
        } else {
            break;
        }
    }
    (start, trimmed_line_prefix.get(start..).unwrap_or(""))
}

fn detect_type_reference_context(
    before_token: &str,
    raw_token: &str,
) -> Option<(&'static [&'static str], Option<String>, String)> {
    let before_trimmed = before_token.trim_end();
    if !before_trimmed.ends_with(':') || before_trimmed.ends_with("::") {
        return None;
    }

    let declaration_prefix = before_trimmed.strip_suffix(':')?.trim_end();
    let expected_kinds = match typed_declaration_keyword(declaration_prefix)? {
        "part" => PART_TYPE_LOOKUP_KINDS,
        "port" => PORT_TYPE_LOOKUP_KINDS,
        "attribute" => ATTRIBUTE_TYPE_LOOKUP_KINDS,
        _ => TYPE_LOOKUP_KINDS,
    };

    let (qualifier, prefix) = if let Some((qualifier, prefix)) = raw_token.rsplit_once("::") {
        (Some(qualifier.to_string()), prefix.to_string())
    } else {
        (None, raw_token.to_string())
    };

    Some((expected_kinds, qualifier, prefix))
}

fn detect_declaration_modifier_context(
    before_token: &str,
    raw_token: &str,
) -> Option<(String, String)> {
    let keyword = declaration_keyword(before_token)?;
    Some((keyword.to_string(), raw_token.to_string()))
}

fn detect_trailing_keyword_modifier_context(line_prefix: &str) -> Option<String> {
    let trimmed = line_prefix.trim_end_matches([' ', '\t']);
    if trimmed.len() == line_prefix.len() {
        return None;
    }
    declaration_keyword(trimmed).map(str::to_string)
}

fn detect_declaration_name_context(before_token: &str, raw_token: &str) -> Option<String> {
    let keyword = declaration_keyword(before_token)?;
    if keyword == "def" {
        return None;
    }
    if raw_token.is_empty() {
        return None;
    }
    Some(raw_token.to_string())
}

fn declaration_keyword(prefix: &str) -> Option<&str> {
    let trimmed = prefix.trim();
    matches!(
        trimmed,
        "package" | "part" | "port" | "attribute" | "action" | "def"
    )
    .then_some(trimmed)
}

fn typed_declaration_keyword(prefix: &str) -> Option<&str> {
    prefix
        .split_whitespace()
        .next()
        .filter(|keyword| matches!(*keyword, "part" | "port" | "attribute"))
}

fn refine_completion_context(
    state: &ServerState,
    uri: &Url,
    pos: Position,
    context: CompletionContext,
) -> CompletionContext {
    match context {
        CompletionContext::TopLevelKeyword { prefix }
        | CompletionContext::General { prefix }
        | CompletionContext::BodyStatement { prefix }
            if prefix.is_empty() =>
        {
            if let Some(node) = state.semantic_graph.find_deepest_node_at_position(uri, pos) {
                if node.element_kind == "package" || node.element_kind.ends_with(" def") {
                    return CompletionContext::BodyStatement { prefix };
                }
            }
            CompletionContext::TopLevelKeyword { prefix }
        }
        other => other,
    }
}

fn completion_semantic_hints(
    state: &ServerState,
    uri: &Url,
    pos: Position,
    context: &CompletionContext,
) -> CompletionSemanticHints {
    if !state.semantic_lifecycle.supports_semantic_queries() {
        return CompletionSemanticHints::default();
    }

    let mut hints = CompletionSemanticHints {
        same_file_uri: Some(uri.clone()),
        ..CompletionSemanticHints::default()
    };

    let context_node = state.semantic_graph.find_deepest_node_at_position(uri, pos);
    if let Some(node) = context_node {
        hints.preferred_names.insert(node.name.clone());
        if let Some(parent_id) = &node.parent_id {
            hints
                .container_names
                .insert(parent_id.qualified_name.clone());
        }
        for ancestor in state.semantic_graph.ancestors_of(node) {
            hints
                .container_names
                .insert(ancestor.id.qualified_name.clone());
        }
    }

    match context {
        CompletionContext::TypeReference {
            qualifier: Some(qualifier),
            ..
        }
        | CompletionContext::QualifiedReference { qualifier, .. } => {
            hints.preferred_names.insert(qualifier.clone());
        }
        CompletionContext::MemberReference { receiver, .. } => {
            hints.preferred_names.insert(receiver.clone());
        }
        _ => {}
    }

    hints
}

fn collect_completion_candidates(
    state: &ServerState,
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    edit_shape: &CompletionEditShape,
) -> Vec<CompletionCandidate> {
    let mut candidates = Vec::new();
    collect_snippet_candidates(context, edit_shape, &mut candidates);
    collect_keyword_candidates(context, edit_shape, &mut candidates);
    collect_symbol_candidates(state, context, hints, edit_shape, &mut candidates);
    rank_candidates_in_place(context, hints, &mut candidates);
    dedupe_completion_candidates(candidates)
}

fn collect_snippet_candidates(
    context: &CompletionContext,
    edit_shape: &CompletionEditShape,
    out: &mut Vec<CompletionCandidate>,
) {
    match context {
        CompletionContext::DeclarationModifier { keyword, .. } => {
            if DECLARATION_MODIFIER_KEYWORDS.contains(&"def") {
                out.push(snippet_candidate(
                    "def",
                    format!("{keyword} definition"),
                    "Declare a reusable definition",
                    "def ${1:Name} {\n\t$0\n}",
                    CompletionItemKind::SNIPPET,
                    edit_shape,
                ));
            }
        }
        CompletionContext::TopLevelKeyword { .. } | CompletionContext::BodyStatement { .. } => {
            let mut part_def_candidate = snippet_candidate(
                "part def",
                "part definition",
                "Declare a reusable part definition",
                "part def ${1:Name} {\n\t$0\n}",
                CompletionItemKind::SNIPPET,
                edit_shape,
            );
            part_def_candidate.score = 40;
            out.push(part_def_candidate);
            out.push(snippet_candidate(
                "part",
                "part usage",
                "Declare a typed part usage",
                "part ${1:name} : ${2:Type}",
                CompletionItemKind::SNIPPET,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "port",
                "port usage",
                "Declare a typed port usage",
                "port ${1:name} : ${2:PortType}",
                CompletionItemKind::SNIPPET,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "attribute",
                "attribute usage",
                "Declare a typed attribute usage",
                "attribute ${1:name} : ${2:AttributeType}",
                CompletionItemKind::SNIPPET,
                edit_shape,
            ));
        }
        _ => {}
    }
}

fn snippet_candidate(
    label: &str,
    detail: impl Into<String>,
    documentation: impl Into<String>,
    snippet: &str,
    kind: CompletionItemKind,
    edit_shape: &CompletionEditShape,
) -> CompletionCandidate {
    let detail = detail.into();
    let documentation = documentation.into();
    CompletionCandidate {
        label: label.to_string(),
        item: CompletionItem {
            label: label.to_string(),
            kind: Some(kind),
            detail: Some(detail.clone()),
            documentation: Some(Documentation::String(documentation.clone())),
            filter_text: Some(label.to_string()),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range: edit_shape.replace_range,
                new_text: snippet.to_string(),
            })),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            data: Some(serde_json::json!({
                COMPLETION_RESOLVE_DATA_KEY: CompletionResolveData {
                    detail: Some(detail),
                    documentation: Some(documentation),
                }
            })),
            ..CompletionItem::default()
        },
        tier: TIER_CONTEXTUAL_SNIPPET,
        score: 0,
    }
}

fn collect_keyword_candidates(
    context: &CompletionContext,
    edit_shape: &CompletionEditShape,
    out: &mut Vec<CompletionCandidate>,
) {
    let keywords: &[&str] = match context {
        CompletionContext::TopLevelKeyword { .. } => BODY_CONTEXT_KEYWORDS,
        CompletionContext::DeclarationModifier { .. } => DECLARATION_MODIFIER_KEYWORDS,
        CompletionContext::BodyStatement { .. } => BODY_CONTEXT_KEYWORDS,
        _ => &[],
    };

    for keyword in keywords {
        out.push(CompletionCandidate {
            label: (*keyword).to_string(),
            item: CompletionItem {
                label: (*keyword).to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("keyword".to_string()),
                documentation: keyword_doc(keyword)
                    .map(|doc| Documentation::String(doc.to_string())),
                filter_text: Some((*keyword).to_string()),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_shape.replace_range,
                    new_text: (*keyword).to_string(),
                })),
                ..CompletionItem::default()
            },
            tier: TIER_KEYWORD_FALLBACK,
            score: 0,
        });
    }
}

fn collect_symbol_candidates(
    state: &ServerState,
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    edit_shape: &CompletionEditShape,
    out: &mut Vec<CompletionCandidate>,
) {
    let prefix = context.prefix().to_lowercase();
    for entry in &state.symbol_table {
        if !prefix.is_empty() && !entry.name.to_lowercase().contains(&prefix) {
            continue;
        }
        let detail = entry.detail.clone();
        out.push(CompletionCandidate {
            label: entry.name.clone(),
            item: CompletionItem {
                label: entry.name.clone(),
                kind: Some(symbol_kind_to_completion_item_kind(entry.kind)),
                detail: detail.clone(),
                documentation: entry
                    .description
                    .as_ref()
                    .map(|doc| Documentation::String(doc.clone())),
                filter_text: Some(entry.name.clone()),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: edit_shape.replace_range,
                    new_text: entry.name.clone(),
                })),
                data: Some(serde_json::json!({
                    COMPLETION_RESOLVE_DATA_KEY: CompletionResolveData {
                        detail,
                        documentation: entry.description.clone(),
                    }
                })),
                ..CompletionItem::default()
            },
            tier: if hints.same_file_uri.as_ref() == Some(&entry.uri) {
                TIER_SAME_FILE_COMPATIBLE
            } else {
                TIER_GENERIC_SYMBOL
            },
            score: 0,
        });
    }

    let _ = state;
}

fn rank_candidates_in_place(
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    candidates: &mut [CompletionCandidate],
) {
    for candidate in candidates {
        let detail = candidate.item.detail.as_deref();
        let prefix = context.prefix().to_lowercase();
        let label = candidate.label.to_lowercase();
        let starts_with_prefix = !prefix.is_empty() && label.starts_with(&prefix);
        let contains_prefix = !prefix.is_empty() && label.contains(&prefix);
        let mut tier = candidate.tier;
        let mut score = candidate.score
            + match (starts_with_prefix, contains_prefix) {
                (true, _) => 200,
                (_, true) => 80,
                _ => 0,
            };

        let kind_matches_context = match context {
            CompletionContext::TypeReference { expected_kinds, .. } => {
                entry_kind_matches(detail, expected_kinds)
            }
            CompletionContext::DeclarationModifier { .. } => candidate.label == "def",
            CompletionContext::BodyStatement { .. } | CompletionContext::TopLevelKeyword { .. } => {
                candidate.item.kind == Some(CompletionItemKind::KEYWORD)
                    || candidate.item.kind == Some(CompletionItemKind::SNIPPET)
            }
            _ => false,
        };

        if kind_matches_context {
            tier = tier.max(TIER_CONTEXT_COMPATIBLE_SAME_SCOPE);
            score += 300;
        }
        if hints
            .preferred_names
            .iter()
            .any(|name| candidate.label == *name)
        {
            tier = tier.max(TIER_EXACT_SEMANTIC);
            score += 500;
        }
        if let Some(container_name) = hints.container_names.iter().find(|container_name| {
            candidate
                .item
                .detail
                .as_deref()
                .is_some_and(|detail| detail.contains(container_name.as_str()))
        }) {
            let _ = container_name;
            score += 120;
        }

        match context {
            CompletionContext::DeclarationModifier { .. } => {
                score -= 140;
            }
            CompletionContext::DeclarationName { .. } => {
                score -= 40;
            }
            CompletionContext::BodyStatement { .. } => {
                score -= 20;
            }
            CompletionContext::TopLevelKeyword { .. }
            | CompletionContext::General { .. }
            | CompletionContext::TypeReference { .. }
            | CompletionContext::QualifiedReference { .. }
            | CompletionContext::MemberReference { .. } => {}
        }

        if kind_matches_context && tier == TIER_GENERIC_SYMBOL {
            tier = TIER_WORKSPACE_COMPATIBLE;
        }

        candidate.tier = tier;
        candidate.score = score;
    }
}

fn entry_kind_matches(detail: Option<&str>, expected_kinds: &[&str]) -> bool {
    detail
        .map(|detail| expected_kinds.contains(&detail))
        .unwrap_or(false)
}

fn symbol_kind_to_completion_item_kind(kind: SymbolKind) -> CompletionItemKind {
    match kind {
        SymbolKind::MODULE | SymbolKind::NAMESPACE => CompletionItemKind::MODULE,
        SymbolKind::CLASS => CompletionItemKind::CLASS,
        SymbolKind::INTERFACE => CompletionItemKind::INTERFACE,
        SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
        SymbolKind::PROPERTY => CompletionItemKind::PROPERTY,
        SymbolKind::OBJECT => CompletionItemKind::VARIABLE,
        SymbolKind::EVENT => CompletionItemKind::EVENT,
        _ => CompletionItemKind::REFERENCE,
    }
}

fn dedupe_completion_candidates(candidates: Vec<CompletionCandidate>) -> Vec<CompletionCandidate> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for candidate in candidates {
        if seen.insert(candidate.label.clone()) {
            deduped.push(candidate);
        }
    }
    deduped
}

fn rank_completion_candidates(mut candidates: Vec<CompletionCandidate>) -> Vec<CompletionItem> {
    candidates.sort_by(|left, right| {
        right
            .tier
            .cmp(&left.tier)
            .then_with(|| right.score.cmp(&left.score))
            .then_with(|| left.label.cmp(&right.label))
    });

    let total = candidates.len();
    candidates
        .into_iter()
        .enumerate()
        .map(|(idx, candidate)| {
            let mut item = candidate.item;
            item.sort_text = Some(format!(
                "{:04}_{:06}_{}",
                9999_i32.saturating_sub(candidate.tier),
                total.saturating_sub(idx),
                candidate.label
            ));
            item.preselect = Some(idx == 0);
            item
        })
        .collect()
}

pub(crate) fn completion_resolve(
    _state: &ServerState,
    mut item: CompletionItem,
) -> Result<CompletionItem> {
    let Some(data) = item.data.as_ref() else {
        return Ok(item);
    };
    let Some(payload) = data.get(COMPLETION_RESOLVE_DATA_KEY) else {
        return Ok(item);
    };
    item.detail = item.detail.or_else(|| {
        payload
            .get("detail")
            .and_then(|value| value.as_str())
            .map(str::to_string)
    });
    if item.documentation.is_none() {
        item.documentation = payload
            .get("documentation")
            .and_then(|value| value.as_str())
            .map(|value| Documentation::String(value.to_string()));
    }
    Ok(item)
}

fn completion_edit_shape(pos: Position, prefix: &str) -> CompletionEditShape {
    CompletionEditShape {
        replace_range: Range::new(
            Position::new(
                pos.line,
                pos.character.saturating_sub(prefix.chars().count() as u32),
            ),
            pos,
        ),
    }
}

pub(crate) fn completion(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<CompletionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let line_prefix = line_prefix_at_position(text, pos.line, pos.character);
    let context = refine_completion_context(
        state,
        &uri_norm,
        pos,
        detect_completion_context(&line_prefix),
    );
    let hints = completion_semantic_hints(state, &uri_norm, pos, &context);
    let edit_shape = completion_edit_shape(pos, context.prefix());
    let items = rank_completion_candidates(collect_completion_candidates(
        state,
        &context,
        &hints,
        &edit_shape,
    ));

    Ok(Some(CompletionResponse::List(CompletionList {
        is_incomplete: false,
        items,
    })))
}

#[cfg(test)]
mod tests {
    use super::{
        completion_edit_shape, detect_completion_context, CompletionContext,
        ATTRIBUTE_TYPE_LOOKUP_KINDS, PART_TYPE_LOOKUP_KINDS, PORT_TYPE_LOOKUP_KINDS,
    };
    use tower_lsp::lsp_types::Position;

    #[test]
    fn detects_part_type_reference_context() {
        let context = detect_completion_context("    part laptop: La");
        assert_eq!(
            context,
            CompletionContext::TypeReference {
                prefix: "La".to_string(),
                qualifier: None,
                expected_kinds: PART_TYPE_LOOKUP_KINDS,
            }
        );
    }

    #[test]
    fn detects_port_type_reference_context() {
        let context = detect_completion_context("    port control: C");
        assert_eq!(
            context,
            CompletionContext::TypeReference {
                prefix: "C".to_string(),
                qualifier: None,
                expected_kinds: PORT_TYPE_LOOKUP_KINDS,
            }
        );
    }

    #[test]
    fn detects_attribute_type_reference_context() {
        let context = detect_completion_context("    attribute mass: M");
        assert_eq!(
            context,
            CompletionContext::TypeReference {
                prefix: "M".to_string(),
                qualifier: None,
                expected_kinds: ATTRIBUTE_TYPE_LOOKUP_KINDS,
            }
        );
    }

    #[test]
    fn detects_qualified_reference_context() {
        let context = detect_completion_context("    part laptop: Pkg::La");
        assert_eq!(
            context,
            CompletionContext::TypeReference {
                prefix: "La".to_string(),
                qualifier: Some("Pkg".to_string()),
                expected_kinds: PART_TYPE_LOOKUP_KINDS,
            }
        );
    }

    #[test]
    fn detects_member_reference_context() {
        let context = detect_completion_context("    vehicle.eng");
        assert_eq!(
            context,
            CompletionContext::MemberReference {
                prefix: "eng".to_string(),
                receiver: "vehicle".to_string(),
            }
        );
    }

    #[test]
    fn detects_declaration_modifier_context() {
        let context = detect_completion_context("    part ");
        assert_eq!(
            context,
            CompletionContext::DeclarationModifier {
                prefix: String::new(),
                keyword: "part".to_string(),
            }
        );
    }

    #[test]
    fn does_not_treat_comments_as_type_context() {
        let context = detect_completion_context("// part laptop: La");
        assert_eq!(
            context,
            CompletionContext::General {
                prefix: "La".to_string(),
            }
        );
    }

    #[test]
    fn detects_top_level_keyword_context() {
        let context = detect_completion_context("    pa");
        assert_eq!(
            context,
            CompletionContext::TopLevelKeyword {
                prefix: "pa".to_string(),
            }
        );
    }

    #[test]
    fn replacement_range_uses_only_member_prefix() {
        let shape = completion_edit_shape(Position::new(0, 11), "eng");
        assert_eq!(shape.replace_range.start, Position::new(0, 8));
        assert_eq!(shape.replace_range.end, Position::new(0, 11));
    }

    #[test]
    fn replacement_range_uses_only_qualified_suffix() {
        let shape = completion_edit_shape(Position::new(0, 7), "Fo");
        assert_eq!(shape.replace_range.start, Position::new(0, 5));
        assert_eq!(shape.replace_range.end, Position::new(0, 7));
    }
}
