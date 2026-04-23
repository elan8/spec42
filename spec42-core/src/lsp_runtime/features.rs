use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tracing::{debug, info};

use crate::common::util;
use crate::language::{
    collect_document_symbols, collect_folding_ranges, completion_prefix, find_reference_ranges,
    format_document, is_reserved_keyword, keyword_doc, keyword_hover_markdown,
    line_prefix_at_position, suggest_create_matching_part_def_quick_fix,
    suggest_explicit_redefinition_quick_fix, suggest_manage_custom_libraries_quick_fix,
    suggest_wrap_in_package, sysml_keywords, word_at_position,
};
use crate::semantic_model::{self, ResolveResult};
use crate::semantic_tokens::{ast_semantic_ranges, semantic_tokens_full, semantic_tokens_range};
use crate::workspace::ServerState;

use super::lookup_helpers::{collect_symbol_matches_for_lookup, debug_qualified_lookup_context};
use super::{hierarchy, navigation, references_resolver, symbols};

static CODE_LENS_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
static SEMANTIC_TOKENS_FULL_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
static SEMANTIC_TOKENS_RANGE_REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

const TYPE_LOOKUP_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "kermlDecl",
];

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

#[derive(Debug, Clone)]
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
        detect_type_reference_context(before_token, &raw_token)
    {
        return CompletionContext::TypeReference {
            prefix,
            qualifier,
            expected_kinds,
        };
    }

    if let Some((keyword, prefix)) = detect_declaration_modifier_context(before_token, &raw_token) {
        return CompletionContext::DeclarationModifier { prefix, keyword };
    }

    if let Some(prefix) = detect_declaration_name_context(before_token, &raw_token) {
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

fn typed_declaration_keyword(before_colon: &str) -> Option<&'static str> {
    let words: Vec<&str> = before_colon.split_whitespace().collect();
    for idx in (0..words.len()).rev() {
        match words[idx] {
            "attribute" => return Some("attribute"),
            "port" => return Some("port"),
            "part" => return Some("part"),
            _ => {}
        }
    }
    None
}

fn detect_declaration_name_context(before_token: &str, raw_token: &str) -> Option<String> {
    if raw_token.contains(':') || raw_token.contains('.') {
        return None;
    }

    let before_trimmed = before_token.trim_end();
    let keyword = before_trimmed
        .split_whitespace()
        .last()
        .filter(|word| matches!(*word, "package" | "part" | "port" | "attribute" | "action"))?;
    if keyword.is_empty() {
        return None;
    }

    Some(raw_token.to_string())
}

fn detect_declaration_modifier_context(
    before_token: &str,
    raw_token: &str,
) -> Option<(String, String)> {
    if raw_token.contains(':') || raw_token.contains('.') || raw_token.contains("::") {
        return None;
    }

    let before_trimmed = before_token.trim_end();
    let keyword = before_trimmed
        .split_whitespace()
        .last()
        .filter(|word| matches!(*word, "package" | "part" | "port" | "attribute" | "action"))?;

    if raw_token.is_empty() && !(before_token.ends_with(' ') || before_token.ends_with('\t')) {
        return None;
    }

    Some((keyword.to_string(), raw_token.to_string()))
}

fn detect_trailing_keyword_modifier_context(line_prefix: &str) -> Option<String> {
    if !(line_prefix.ends_with(' ') || line_prefix.ends_with('\t')) {
        return None;
    }

    let trimmed = line_prefix.trim_end();
    let keyword = trimmed
        .split_whitespace()
        .last()
        .filter(|word| matches!(*word, "package" | "part" | "port" | "attribute" | "action"))?;

    Some(keyword.to_string())
}

fn completion_semantic_hints(
    state: &ServerState,
    uri: &Url,
    pos: Position,
    context: &CompletionContext,
) -> CompletionSemanticHints {
    let context_node = state.semantic_graph.find_deepest_node_at_position(uri, pos);
    let mut hints = CompletionSemanticHints {
        same_file_uri: Some(uri.clone()),
        ..Default::default()
    };

    if let Some(node) = context_node {
        hints.container_names.insert(node.name.clone());
        for ancestor in state.semantic_graph.ancestors_of(node) {
            hints.container_names.insert(ancestor.name.clone());
        }
    }

    match context {
        CompletionContext::QualifiedReference { qualifier, .. } => {
            if let Some(target) = resolve_hover_reference_target(state, uri, pos, qualifier) {
                hints.preferred_names = direct_child_names(state, target);
            }
        }
        CompletionContext::MemberReference { receiver, .. } => {
            if let Some(target) = resolve_hover_reference_target(state, uri, pos, receiver) {
                hints.preferred_names = member_candidate_names(state, target);
            }
        }
        CompletionContext::TypeReference { qualifier, .. } => {
            if let Some(qualifier) = qualifier.as_deref() {
                if let Some(target) = resolve_hover_reference_target(state, uri, pos, qualifier) {
                    hints.preferred_names = direct_child_names(state, target);
                }
            }
        }
        CompletionContext::DeclarationModifier { .. } => {}
        CompletionContext::TopLevelKeyword { .. }
        | CompletionContext::General { .. }
        | CompletionContext::DeclarationName { .. }
        | CompletionContext::BodyStatement { .. } => {}
    }

    hints
}

fn refine_completion_context(
    state: &ServerState,
    uri: &Url,
    pos: Position,
    context: CompletionContext,
) -> CompletionContext {
    let Some(node) = state.semantic_graph.find_deepest_node_at_position(uri, pos) else {
        return context;
    };

    match context {
        CompletionContext::TopLevelKeyword { prefix }
            if body_like_context_node(&node.element_kind) =>
        {
            CompletionContext::BodyStatement { prefix }
        }
        other => other,
    }
}

fn body_like_context_node(kind: &str) -> bool {
    matches!(
        kind,
        "package" | "part def" | "part" | "port def" | "interface"
    )
}

fn direct_child_names(
    state: &ServerState,
    node: &crate::semantic_model::SemanticNode,
) -> HashSet<String> {
    state
        .semantic_graph
        .children_of(node)
        .into_iter()
        .map(|child| child.name.clone())
        .collect()
}

fn member_candidate_names(
    state: &ServerState,
    node: &crate::semantic_model::SemanticNode,
) -> HashSet<String> {
    fn visit_typed_members(
        graph: &crate::semantic_model::SemanticGraph,
        node: &crate::semantic_model::SemanticNode,
        visited: &mut HashSet<crate::semantic_model::NodeId>,
        out: &mut HashSet<String>,
    ) {
        for target in graph.outgoing_typing_or_specializes_targets(node) {
            if !visited.insert(target.id.clone()) {
                continue;
            }
            for child in graph.children_of(target) {
                out.insert(child.name.clone());
            }
            visit_typed_members(graph, target, visited, out);
        }
    }

    let mut out = direct_child_names(state, node);
    let mut visited = HashSet::new();
    visit_typed_members(&state.semantic_graph, node, &mut visited, &mut out);
    out
}

fn completion_edit_shape(pos: Position, prefix: &str) -> CompletionEditShape {
    let prefix_len = prefix.chars().count() as u32;
    let start_character = pos.character.saturating_sub(prefix_len);
    CompletionEditShape {
        replace_range: Range::new(
            Position::new(pos.line, start_character),
            Position::new(pos.line, pos.character),
        ),
    }
}

fn symbol_completion_kind(kind: SymbolKind) -> CompletionItemKind {
    match kind {
        SymbolKind::CLASS => CompletionItemKind::CLASS,
        SymbolKind::INTERFACE => CompletionItemKind::INTERFACE,
        SymbolKind::FUNCTION => CompletionItemKind::FUNCTION,
        SymbolKind::PROPERTY => CompletionItemKind::PROPERTY,
        SymbolKind::MODULE => CompletionItemKind::MODULE,
        SymbolKind::OBJECT | SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
        SymbolKind::CONSTANT => CompletionItemKind::CONSTANT,
        SymbolKind::ENUM => CompletionItemKind::ENUM,
        SymbolKind::EVENT => CompletionItemKind::EVENT,
        _ => CompletionItemKind::REFERENCE,
    }
}

fn completion_item_with_edit(
    label: String,
    kind: CompletionItemKind,
    new_text: String,
    edit_shape: &CompletionEditShape,
) -> CompletionItem {
    CompletionItem {
        label,
        kind: Some(kind),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
            range: edit_shape.replace_range,
            new_text,
        })),
        ..Default::default()
    }
}

fn apply_completion_metadata(
    item: &mut CompletionItem,
    edit_shape: &CompletionEditShape,
    filter_text: Option<String>,
    resolve_data: CompletionResolveData,
) {
    item.filter_text = filter_text;
    item.data = Some(serde_json::json!({
        COMPLETION_RESOLVE_DATA_KEY: {
            "detail": resolve_data.detail,
            "documentation": resolve_data.documentation,
        }
    }));
    if item.text_edit.is_none() {
        item.text_edit = Some(CompletionTextEdit::Edit(TextEdit {
            range: edit_shape.replace_range,
            new_text: item
                .insert_text
                .clone()
                .unwrap_or_else(|| item.label.to_string()),
        }));
    }
}

fn snippet_candidate(
    label: &str,
    detail: &str,
    snippet: &str,
    filter_text: &str,
    edit_shape: &CompletionEditShape,
    tier: i32,
    score: i32,
) -> CompletionCandidate {
    let mut item = completion_item_with_edit(
        label.to_string(),
        CompletionItemKind::SNIPPET,
        snippet.to_string(),
        edit_shape,
    );
    item.insert_text_format = Some(InsertTextFormat::SNIPPET);
    item.detail = Some(detail.to_string());
    apply_completion_metadata(
        &mut item,
        edit_shape,
        Some(filter_text.to_string()),
        CompletionResolveData {
            detail: Some(detail.to_string()),
            documentation: Some(detail.to_string()),
        },
    );
    CompletionCandidate {
        label: label.to_string(),
        item,
        tier,
        score,
    }
}

fn completion_keywords_for_context(context: &CompletionContext) -> &'static [&'static str] {
    match context {
        CompletionContext::QualifiedReference { .. }
        | CompletionContext::MemberReference { .. } => &[],
        CompletionContext::DeclarationModifier { .. } => DECLARATION_MODIFIER_KEYWORDS,
        CompletionContext::BodyStatement { .. } => BODY_CONTEXT_KEYWORDS,
        _ => sysml_keywords(),
    }
}

fn snippet_candidates_for_context(
    context: &CompletionContext,
    prefix: &str,
    edit_shape: &CompletionEditShape,
) -> Vec<CompletionCandidate> {
    let prefix_matches = |label: &str| prefix.is_empty() || label.starts_with(prefix);
    let mut out = Vec::new();

    match context {
        CompletionContext::DeclarationModifier { keyword, .. } => {
            if matches!(keyword.as_str(), "part" | "port" | "attribute" | "action")
                && prefix_matches("def")
            {
                out.push(snippet_candidate(
                    "def",
                    "Complete a definition after the declaration keyword",
                    "def ${1:Name} {\n    $0\n}",
                    "def",
                    edit_shape,
                    TIER_CONTEXTUAL_SNIPPET,
                    100,
                ));
            }
        }
        CompletionContext::TopLevelKeyword { .. } | CompletionContext::BodyStatement { .. } => {
            if prefix_matches("part def") {
                out.push(snippet_candidate(
                    "part def",
                    "Define a part",
                    "part def ${1:Name} {\n    $0\n}",
                    "part def",
                    edit_shape,
                    TIER_CONTEXTUAL_SNIPPET,
                    100,
                ));
            }
            if prefix_matches("part") {
                out.push(snippet_candidate(
                    "part usage",
                    "Declare a typed part usage",
                    "part ${1:name}: ${2:Type};",
                    "part",
                    edit_shape,
                    TIER_CONTEXTUAL_SNIPPET,
                    90,
                ));
            }
            if prefix_matches("port") {
                out.push(snippet_candidate(
                    "port usage",
                    "Declare a typed port usage",
                    "port ${1:name}: ${2:Type};",
                    "port",
                    edit_shape,
                    TIER_CONTEXTUAL_SNIPPET,
                    80,
                ));
            }
            if prefix_matches("attribute") {
                out.push(snippet_candidate(
                    "attribute",
                    "Declare a typed attribute",
                    "attribute ${1:name}: ${2:Type};",
                    "attribute",
                    edit_shape,
                    TIER_CONTEXTUAL_SNIPPET,
                    70,
                ));
            }
            if matches!(context, CompletionContext::TopLevelKeyword { .. })
                && prefix_matches("package")
            {
                out.push(snippet_candidate(
                    "package",
                    "Define a package",
                    "package ${1:Name} {\n    $0\n}",
                    "package",
                    edit_shape,
                    TIER_CONTEXTUAL_SNIPPET,
                    60,
                ));
            }
        }
        _ => {}
    }

    out
}

fn collect_completion_candidates(
    state: &ServerState,
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    edit_shape: &CompletionEditShape,
) -> Vec<CompletionCandidate> {
    let prefix = context.prefix();
    let mut candidates = Vec::new();

    candidates.extend(snippet_candidates_for_context(context, prefix, edit_shape));

    for kw in completion_keywords_for_context(context) {
        if prefix.is_empty() || kw.starts_with(prefix) {
            let mut item = completion_item_with_edit(
                (*kw).to_string(),
                CompletionItemKind::KEYWORD,
                (*kw).to_string(),
                edit_shape,
            );
            item.detail = keyword_doc(kw).map(String::from);
            apply_completion_metadata(
                &mut item,
                edit_shape,
                Some((*kw).to_string()),
                CompletionResolveData {
                    detail: keyword_doc(kw).map(String::from),
                    documentation: keyword_doc(kw).map(String::from),
                },
            );
            let (tier, score) = keyword_score(context, prefix, kw);
            candidates.push(CompletionCandidate {
                label: (*kw).to_string(),
                item,
                tier,
                score,
            });
        }
    }

    let mut seen = HashSet::<String>::new();
    for entry in &state.symbol_table {
        if (!prefix.is_empty() && !entry.name.starts_with(prefix))
            || !seen.insert(entry.name.clone())
        {
            continue;
        }
        let mut item = completion_item_with_edit(
            entry.name.clone(),
            symbol_completion_kind(entry.kind),
            entry.name.clone(),
            edit_shape,
        );
        item.detail = entry.detail.clone();
        let documentation = entry
            .signature
            .clone()
            .or_else(|| entry.description.clone())
            .map(Documentation::String);
        apply_completion_metadata(
            &mut item,
            edit_shape,
            Some(entry.name.clone()),
            CompletionResolveData {
                detail: entry.description.clone().or_else(|| entry.detail.clone()),
                documentation: documentation.and_then(|doc| match doc {
                    Documentation::String(text) => Some(text),
                    _ => None,
                }),
            },
        );
        let (tier, score) = symbol_score(context, prefix, hints, entry);
        candidates.push(CompletionCandidate {
            label: entry.name.clone(),
            item,
            tier,
            score,
        });
    }

    candidates
}

fn keyword_score(context: &CompletionContext, prefix: &str, keyword: &str) -> (i32, i32) {
    let mut score = 100;
    let mut tier = TIER_KEYWORD_FALLBACK;
    if prefix.is_empty() {
        score += 10;
    } else if keyword == prefix {
        score += 120;
    } else if keyword.starts_with(prefix) {
        score += 70 - prefix.len() as i32;
    }

    match context {
        CompletionContext::DeclarationName { .. } => score += 40,
        CompletionContext::DeclarationModifier { .. } => {
            if keyword == "def" {
                tier = TIER_CONTEXTUAL_SNIPPET;
                score += 220;
            }
        }
        CompletionContext::BodyStatement { .. } => {
            if BODY_CONTEXT_KEYWORDS.contains(&keyword) {
                score += 120;
            }
        }
        CompletionContext::TopLevelKeyword { .. } => {
            if BODY_CONTEXT_KEYWORDS.contains(&keyword) || keyword == "package" {
                score += 60;
            }
        }
        CompletionContext::General { .. } => {}
        CompletionContext::TypeReference { .. }
        | CompletionContext::QualifiedReference { .. }
        | CompletionContext::MemberReference { .. } => {}
    }

    (tier, score)
}

fn symbol_score(
    context: &CompletionContext,
    prefix: &str,
    hints: &CompletionSemanticHints,
    entry: &crate::language::SymbolEntry,
) -> (i32, i32) {
    let mut score = 100;
    let mut tier = TIER_GENERIC_SYMBOL;
    let kind_matches_context = matches!(
        context,
        CompletionContext::TypeReference {
            expected_kinds,
            ..
        } if entry_kind_matches(entry.detail.as_deref(), expected_kinds)
    );
    if prefix.is_empty() {
        score += 5;
    } else if entry.name == prefix {
        score += 140;
    } else {
        score += 80 - prefix.len() as i32;
    }

    if hints.same_file_uri.as_ref() == Some(&entry.uri) {
        score += 30;
    }
    if entry
        .container_name
        .as_ref()
        .map(|name| hints.container_names.contains(name))
        .unwrap_or(false)
    {
        score += 45;
    }
    if hints.preferred_names.contains(&entry.name) {
        tier = TIER_EXACT_SEMANTIC;
        score += 220;
    }

    match context {
        CompletionContext::TypeReference {
            expected_kinds,
            qualifier,
            ..
        } => {
            if entry_kind_matches(entry.detail.as_deref(), expected_kinds) {
                tier = if entry
                    .container_name
                    .as_ref()
                    .map(|name| hints.container_names.contains(name))
                    .unwrap_or(false)
                {
                    TIER_CONTEXT_COMPATIBLE_SAME_SCOPE
                } else if hints.same_file_uri.as_ref() == Some(&entry.uri) {
                    TIER_SAME_FILE_COMPATIBLE
                } else {
                    TIER_WORKSPACE_COMPATIBLE
                };
                score += 260;
            }
            if qualifier.is_some() && hints.preferred_names.contains(&entry.name) {
                score += 80;
            }
        }
        CompletionContext::QualifiedReference { .. } => {
            if hints.preferred_names.contains(&entry.name) {
                tier = TIER_EXACT_SEMANTIC;
                score += 80;
            }
        }
        CompletionContext::MemberReference { .. } => {
            if hints.preferred_names.contains(&entry.name) {
                tier = TIER_EXACT_SEMANTIC;
                score += 120;
            }
        }
        CompletionContext::DeclarationModifier { .. } => {
            score -= 140;
        }
        CompletionContext::DeclarationName { .. } => {
            score -= 40;
        }
        CompletionContext::BodyStatement { .. } => {
            score -= 20;
        }
        CompletionContext::TopLevelKeyword { .. } => {}
        CompletionContext::General { .. } => {}
    }

    if kind_matches_context && tier == TIER_GENERIC_SYMBOL {
        tier = TIER_WORKSPACE_COMPATIBLE;
    }

    (tier, score)
}

fn entry_kind_matches(detail: Option<&str>, expected_kinds: &[&str]) -> bool {
    detail
        .map(|detail| expected_kinds.iter().any(|kind| *kind == detail))
        .unwrap_or(false)
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

fn resolve_hover_type_reference_target<'a>(
    state: &'a ServerState,
    node: &crate::semantic_model::SemanticNode,
    word: &str,
    lookup_name: &str,
) -> Option<&'a crate::semantic_model::SemanticNode> {
    let mut candidates = Vec::<String>::new();
    let mut push_candidate = |candidate: String| {
        if !candidate.is_empty() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };

    push_candidate(word.to_string());
    if lookup_name != word {
        push_candidate(lookup_name.to_string());
    }

    if word.contains("::") {
        for ancestor in state.semantic_graph.ancestors_of(node) {
            push_candidate(format!("{}::{}", ancestor.id.qualified_name, word));
        }
    }

    for candidate in candidates {
        if let Some(target_id) = semantic_model::resolve_type_reference_targets(
            &state.semantic_graph,
            node,
            &candidate,
            TYPE_LOOKUP_KINDS,
        )
        .into_iter()
        .next()
        {
            if let Some(target) = state.semantic_graph.get_node(&target_id) {
                return Some(target);
            }
        }
    }

    None
}

fn resolve_hover_reference_target<'a>(
    state: &'a ServerState,
    uri: &Url,
    pos: Position,
    word: &str,
) -> Option<&'a crate::semantic_model::SemanticNode> {
    let context_node = state
        .semantic_graph
        .find_deepest_node_at_position(uri, pos)
        .or_else(|| {
            state
                .semantic_graph
                .nodes_for_uri(uri)
                .into_iter()
                .find(|n| n.name == word)
        });

    let context_node = context_node?;

    let mut prefixes = Vec::<Option<String>>::new();
    prefixes.push(Some(context_node.id.qualified_name.clone()));
    if let Some(parent_id) = &context_node.parent_id {
        prefixes.push(Some(parent_id.qualified_name.clone()));
    }
    for ancestor in state.semantic_graph.ancestors_of(context_node) {
        prefixes.push(Some(ancestor.id.qualified_name.clone()));
    }
    prefixes.push(None);

    for prefix in prefixes {
        let resolved = semantic_model::resolve_expression_endpoint_strict(
            &state.semantic_graph,
            uri,
            prefix.as_deref(),
            word,
        );
        if let ResolveResult::Resolved(target_id) = resolved {
            if let Some(target) = state.semantic_graph.get_node(&target_id) {
                return Some(target);
            }
        }
    }

    None
}

pub(crate) fn hover(state: &ServerState, uri: Url, pos: Position) -> Result<Option<Hover>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (line, char_start, char_end, word) = match word_at_position(&text, pos.line, pos.character)
    {
        Some(parts) => parts,
        None => return Ok(None),
    };
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    let range = Range::new(
        Position::new(line, char_start),
        Position::new(line, char_end),
    );

    if let Some(md) = keyword_hover_markdown(&lookup_name.to_lowercase()) {
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: md,
            }),
            range: Some(range),
        });
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::features",
                event = "feature:hover",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                elapsed_ms,
                "hover resolved via keyword docs"
            );
        }
        return Ok(response);
    }

    if let Some(node) = state
        .semantic_graph
        .find_deepest_node_at_position(&uri_norm, pos)
    {
        let target_match = state
            .semantic_graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .find(|target| {
                target.name == lookup_name
                    || target
                        .id
                        .qualified_name
                        .ends_with(&format!("::{}", lookup_name))
            });
        let markdown = if let Some(target) = target_match.as_ref() {
            semantic_model::hover_markdown_for_node(
                &state.semantic_graph,
                target,
                target.id.uri != uri_norm,
            )
        } else {
            semantic_model::hover_markdown_for_node(
                &state.semantic_graph,
                node,
                node.id.uri != uri_norm,
            )
        };
        let markdown = if target_match.is_none() && word != node.name {
            resolve_hover_type_reference_target(state, node, &word, &lookup_name)
                .map(|target| {
                    semantic_model::hover_markdown_for_node(
                        &state.semantic_graph,
                        target,
                        target.id.uri != uri_norm,
                    )
                })
                .unwrap_or(markdown)
        } else {
            markdown
        };
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown,
            }),
            range: Some(range),
        });
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::features",
                event = "feature:hover",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                elapsed_ms,
                "hover resolved via semantic graph"
            );
        }
        return Ok(response);
    }

    if let Some(target) = resolve_hover_reference_target(state, &uri_norm, pos, &word) {
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: semantic_model::hover_markdown_for_node(
                    &state.semantic_graph,
                    target,
                    target.id.uri != uri_norm,
                ),
            }),
            range: Some(range),
        });
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::features",
                event = "feature:hover",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                elapsed_ms,
                "hover resolved via context-aware reference lookup"
            );
        }
        return Ok(response);
    }

    let (same_file, other_files) =
        collect_symbol_matches_for_lookup(state, &uri_norm, &lookup_name, qualifier.as_deref());
    let all_matches = if same_file.is_empty() {
        &other_files
    } else {
        &same_file
    };
    if let Some(entry) = all_matches.first() {
        let value = if all_matches.len() > 1 {
            let mut md = format!(
                "**{}** - {} definitions (use Go to Definition to choose):\n\n",
                lookup_name,
                all_matches.len()
            );
            for entry in all_matches.iter() {
                let kind = entry.detail.as_deref().unwrap_or("element");
                let container = entry.container_name.as_deref().unwrap_or("(top level)");
                md.push_str(&format!("- `{}` in `{}`\n", kind, container));
            }
            md.push('\n');
            md.push_str(&util::symbol_hover_markdown(entry, entry.uri != uri_norm));
            md
        } else {
            util::symbol_hover_markdown(entry, entry.uri != uri_norm)
        };
        let response = Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: Some(range),
        });
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::features",
                event = "feature:hover",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                same_file_matches = same_file.len(),
                other_file_matches = other_files.len(),
                elapsed_ms,
                "hover resolved via symbol lookup"
            );
        }
        return Ok(response);
    }

    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "spec42_core::lsp_runtime::features",
            event = "feature:hover",
            uri = %uri_norm,
            line = pos.line,
            character = pos.character,
            lookup_name = %lookup_name,
            elapsed_ms,
            "hover completed with no result"
        );
    }
    Ok(None)
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

pub(crate) fn signature_help(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<SignatureHelp>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let line = text.lines().nth(pos.line as usize).unwrap_or("");
    let cursor_prefix = line
        .chars()
        .take(pos.character as usize)
        .collect::<String>();
    let active_param = cursor_prefix.matches(',').count() as u32;
    let label = if line.contains("part def") {
        "part def <Name> : <Type>"
    } else if line.contains("port def") || line.contains("port ") {
        "port <name> : <PortType>"
    } else if line.contains("attribute") {
        "attribute <name> : <AttributeType>"
    } else {
        "name : Type"
    };
    Ok(Some(SignatureHelp {
        signatures: vec![SignatureInformation {
            label: label.to_string(),
            documentation: Some(Documentation::String(
                "Basic SysML declaration shape".to_string(),
            )),
            parameters: None,
            active_parameter: Some(active_param),
        }],
        active_signature: Some(0),
        active_parameter: Some(active_param),
    }))
}

pub(crate) fn goto_definition(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<GotoDefinitionResponse>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    debug!(
        uri = %uri_norm,
        line = pos.line,
        character = pos.character,
        word = %word,
        lookup_name = %lookup_name,
        qualifier = ?qualifier,
        "goto_definition tokenized input"
    );

    if is_reserved_keyword(&word) || is_reserved_keyword(&lookup_name) {
        return Ok(None);
    }

    if let Some(node) = state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        for target in state
            .semantic_graph
            .outgoing_typing_or_specializes_targets(node)
        {
            if target.name == lookup_name
                || target
                    .id
                    .qualified_name
                    .ends_with(&format!("::{}", lookup_name))
            {
                let response = Some(GotoDefinitionResponse::Scalar(Location {
                    uri: target.id.uri.clone(),
                    range: target.range,
                }));
                let elapsed_ms = started_at.elapsed().as_millis();
                if elapsed_ms >= 10 {
                    info!(
                        target: "spec42_core::lsp_runtime::features",
                        event = "feature:gotoDefinition",
                        uri = %uri_norm,
                        line = pos.line,
                        character = pos.character,
                        lookup_name = %lookup_name,
                        elapsed_ms,
                        "goto definition resolved via semantic graph"
                    );
                }
                return Ok(response);
            }
        }
        if word != node.name {
            if let Some(target) = semantic_model::resolve_type_reference_targets(
                &state.semantic_graph,
                node,
                &word,
                TYPE_LOOKUP_KINDS,
            )
            .into_iter()
            .find_map(|target_id| state.semantic_graph.get_node(&target_id))
            {
                let response = Some(GotoDefinitionResponse::Scalar(Location {
                    uri: target.id.uri.clone(),
                    range: target.range,
                }));
                let elapsed_ms = started_at.elapsed().as_millis();
                if elapsed_ms >= 10 {
                    info!(
                        target: "spec42_core::lsp_runtime::features",
                        event = "feature:gotoDefinition",
                        uri = %uri_norm,
                        line = pos.line,
                        character = pos.character,
                        lookup_name = %lookup_name,
                        elapsed_ms,
                        "goto definition resolved via import-aware semantic graph"
                    );
                }
                return Ok(response);
            }
        }
    }

    let (same_file_matches, other_file_matches) =
        collect_symbol_matches_for_lookup(state, &uri_norm, &lookup_name, qualifier.as_deref());
    let same_file_match_count = same_file_matches.len();
    let other_file_match_count = other_file_matches.len();
    let same_file: Vec<Location> = same_file_matches
        .into_iter()
        .map(|entry| Location {
            uri: entry.uri.clone(),
            range: entry.range,
        })
        .collect();
    let other_files: Vec<Location> = other_file_matches
        .into_iter()
        .map(|entry| Location {
            uri: entry.uri.clone(),
            range: entry.range,
        })
        .collect();
    let locations = if same_file.is_empty() {
        other_files
    } else {
        same_file
    };
    if let [location] = locations.as_slice() {
        let response = Some(GotoDefinitionResponse::Scalar(location.clone()));
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::features",
                event = "feature:gotoDefinition",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                same_file_matches = same_file_match_count,
                other_file_matches = other_file_match_count,
                locations = 1,
                elapsed_ms,
                "goto definition resolved to single location"
            );
        }
        return Ok(response);
    }
    if !locations.is_empty() {
        let location_count = locations.len();
        let response = Some(GotoDefinitionResponse::Array(locations));
        let elapsed_ms = started_at.elapsed().as_millis();
        if elapsed_ms >= 10 {
            info!(
                target: "spec42_core::lsp_runtime::features",
                event = "feature:gotoDefinition",
                uri = %uri_norm,
                line = pos.line,
                character = pos.character,
                lookup_name = %lookup_name,
                same_file_matches = same_file_match_count,
                other_file_matches = other_file_match_count,
                locations = location_count,
                elapsed_ms,
                "goto definition resolved to multiple locations"
            );
        }
        return Ok(response);
    }
    if let Some(qualifier) = qualifier.as_deref() {
        debug_qualified_lookup_context(state, &lookup_name, qualifier, &uri_norm);
    }
    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "spec42_core::lsp_runtime::features",
            event = "feature:gotoDefinition",
            uri = %uri_norm,
            line = pos.line,
            character = pos.character,
            lookup_name = %lookup_name,
            same_file_matches = same_file_match_count,
            other_file_matches = other_file_match_count,
            elapsed_ms,
            "goto definition completed with no result"
        );
    }
    Ok(None)
}

pub(crate) fn references(
    state: &ServerState,
    uri: Url,
    pos: Position,
    include_declaration: bool,
) -> Result<Option<Vec<Location>>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let locations = references_resolver::resolved_references_at_position(
        state,
        &uri_norm,
        pos,
        include_declaration,
    );
    let elapsed_ms = started_at.elapsed().as_millis();
    if elapsed_ms >= 10 {
        info!(
            target: "spec42_core::lsp_runtime::features",
            event = "feature:references",
            uri = %uri_norm,
            line = pos.line,
            character = pos.character,
            include_declaration,
            locations = locations.as_ref().map(|items| items.len()).unwrap_or(0),
            elapsed_ms,
            "references request completed"
        );
    }
    Ok(locations)
}

pub(crate) fn document_link(state: &ServerState, uri: Url) -> Result<Option<Vec<DocumentLink>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let links = navigation::collect_document_links(text, |import_name| {
        state
            .symbol_table
            .iter()
            .find(|entry| entry.name == import_name)
            .map(|entry| entry.uri.clone())
    });
    Ok(Some(links))
}

pub(crate) fn document_highlight(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<DocumentHighlight>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if is_reserved_keyword(&word) {
        return Ok(None);
    }
    let highlights = find_reference_ranges(&text, &word)
        .into_iter()
        .map(|range| DocumentHighlight {
            range,
            kind: Some(DocumentHighlightKind::TEXT),
        })
        .collect();
    Ok(Some(highlights))
}

pub(crate) fn selection_range(
    state: &ServerState,
    uri: Url,
    positions: Vec<Position>,
) -> Result<Option<Vec<SelectionRange>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    Ok(Some(navigation::selection_ranges_for_positions(
        text,
        &positions,
        word_at_position,
    )))
}

pub(crate) fn prepare_rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<PrepareRenameResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (line, char_start, char_end, word) = match word_at_position(&text, pos.line, pos.character)
    {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if is_reserved_keyword(&word) {
        return Ok(None);
    }
    Ok(Some(PrepareRenameResponse::Range(Range::new(
        Position::new(line, char_start),
        Position::new(line, char_end),
    ))))
}

pub(crate) fn rename(
    state: &ServerState,
    uri: Url,
    pos: Position,
    new_name: String,
) -> Result<Option<WorkspaceEdit>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if is_reserved_keyword(&word) {
        return Ok(None);
    }

    let mut locations = Vec::new();
    for (uri, entry) in &state.index {
        for range in find_reference_ranges(&entry.content, &word) {
            locations.push(Location {
                uri: uri.clone(),
                range,
            });
        }
    }
    if locations.is_empty() {
        return Ok(Some(WorkspaceEdit::default()));
    }

    let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
        std::collections::HashMap::new();
    for location in locations {
        changes
            .entry(location.uri.clone())
            .or_default()
            .push(TextEdit {
                range: location.range,
                new_text: new_name.clone(),
            });
    }
    Ok(Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    }))
}

pub(crate) fn document_symbol(
    state: &ServerState,
    uri: Url,
) -> Result<Option<DocumentSymbolResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let entry = match state.index.get(&uri_norm) {
        Some(entry) => entry,
        None => return Ok(None),
    };
    let doc = match &entry.parsed {
        Some(doc) => doc,
        None => return Ok(None),
    };
    Ok(Some(DocumentSymbolResponse::Nested(
        collect_document_symbols(doc),
    )))
}

pub(crate) fn folding_range(state: &ServerState, uri: Url) -> Result<Option<Vec<FoldingRange>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let entry = match state.index.get(&uri_norm) {
        Some(entry) => entry,
        None => return Ok(None),
    };
    let doc = match &entry.parsed {
        Some(doc) => doc,
        None => return Ok(None),
    };
    Ok(Some(collect_folding_ranges(doc)))
}

#[allow(deprecated)]
pub(crate) fn workspace_symbol(
    state: &ServerState,
    query: String,
) -> Result<Option<Vec<SymbolInformation>>> {
    let query = query.to_lowercase();
    let out = state
        .symbol_table
        .iter()
        .filter(|entry| query.is_empty() || entry.name.to_lowercase().contains(&query))
        .map(|entry| SymbolInformation {
            name: entry.name.clone(),
            kind: entry.kind,
            tags: None,
            deprecated: None,
            location: Location {
                uri: entry.uri.clone(),
                range: entry.range,
            },
            container_name: entry.container_name.clone(),
        })
        .collect();
    Ok(Some(out))
}

pub(crate) fn code_action(
    state: &ServerState,
    uri: Url,
    diagnostics: &[Diagnostic],
) -> Result<Option<CodeActionResponse>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let mut actions = Vec::new();
    if let Some(action) = suggest_wrap_in_package(&text, &uri) {
        actions.push(CodeActionOrCommand::CodeAction(action));
    }
    for diagnostic in diagnostics {
        let is_untyped_part_usage = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code)) if code == "untyped_part_usage"
        );
        if is_untyped_part_usage {
            if let Some(action) =
                suggest_create_matching_part_def_quick_fix(&text, &uri, diagnostic)
            {
                actions.push(CodeActionOrCommand::CodeAction(action));
            }
        }
        let is_implicit_redefinition_without_operator = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code)) if code == "implicit_redefinition_without_operator"
        );
        if is_implicit_redefinition_without_operator {
            if let Some(action) = suggest_explicit_redefinition_quick_fix(&text, &uri, diagnostic) {
                actions.push(CodeActionOrCommand::CodeAction(action));
            }
        }
        let is_missing_library_context = matches!(
            diagnostic.code.as_ref(),
            Some(NumberOrString::String(code)) if code == "missing_library_context"
        );
        if is_missing_library_context {
            actions.push(CodeActionOrCommand::CodeAction(
                suggest_manage_custom_libraries_quick_fix(diagnostic),
            ));
        }
    }
    Ok(Some(actions))
}

pub(crate) fn code_lens(state: &ServerState, uri: Url) -> Result<Option<Vec<CodeLens>>> {
    if !state.code_lens_enabled {
        return Ok(None);
    }
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let lenses = symbols::build_code_lens(state, &uri_norm);
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = CODE_LENS_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if state.perf_logging_enabled {
        info!(
            target: "spec42_core::lsp_runtime::features",
            event = "feature:codeLens",
            uri = %uri_norm,
            lenses = lenses.len(),
            elapsed_ms,
            request_count,
            "code lens request completed"
        );
    }
    Ok(Some(lenses))
}

pub(crate) fn inlay_hint(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<InlayHint>>> {
    let _ = (state, uri, range);
    Ok(Some(Vec::new()))
}

pub(crate) fn formatting(
    state: &ServerState,
    uri: Url,
    options: FormattingOptions,
) -> Result<Option<Vec<TextEdit>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.clone())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    Ok(Some(format_document(&text, &options)))
}

pub(crate) fn semantic_tokens_full_request(
    state: &ServerState,
    uri: Url,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => (
            entry.content.clone(),
            entry.parsed.as_ref().map(ast_semantic_ranges),
        ),
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_full(&text, ast_ranges.as_deref());
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = SEMANTIC_TOKENS_FULL_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        target: "spec42_core::lsp_runtime::features",
        event = "feature:semanticTokensFull",
        uri = %uri_norm,
        token_count = tokens.data.len(),
        log_count = logs.len(),
        elapsed_ms,
        request_count,
        "semantic tokens full request completed"
    );
    Ok(Some((tokens, logs)))
}

pub(crate) fn semantic_tokens_range_request(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<(SemanticTokens, Vec<String>)>> {
    let started_at = Instant::now();
    let uri_norm = util::normalize_file_uri(&uri);
    let (text, ast_ranges) = match state.index.get(&uri_norm) {
        Some(entry) => (
            entry.content.clone(),
            entry.parsed.as_ref().map(ast_semantic_ranges),
        ),
        None => return Ok(None),
    };
    let (tokens, logs) = semantic_tokens_range(
        &text,
        range.start.line,
        range.start.character,
        range.end.line,
        range.end.character,
        ast_ranges.as_deref(),
    );
    let elapsed_ms = started_at.elapsed().as_millis();
    let request_count = SEMANTIC_TOKENS_RANGE_REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        target: "spec42_core::lsp_runtime::features",
        event = "feature:semanticTokensRange",
        uri = %uri_norm,
        start_line = range.start.line,
        end_line = range.end.line,
        token_count = tokens.data.len(),
        log_count = logs.len(),
        elapsed_ms,
        request_count,
        "semantic tokens range request completed"
    );
    Ok(Some((tokens, logs)))
}

pub(crate) fn linked_editing_range(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<LinkedEditingRanges>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let text = match state
        .index
        .get(&uri_norm)
        .map(|entry| entry.content.as_str())
    {
        Some(text) => text,
        None => return Ok(None),
    };
    let (line, _, _, word) = match word_at_position(text, pos.line, pos.character) {
        Some(parts) => parts,
        None => return Ok(None),
    };
    if is_reserved_keyword(&word) {
        return Ok(None);
    }
    let line_text = text.lines().nth(line as usize).unwrap_or("");
    let declaration_like = line_text.contains(" def ")
        || line_text.trim_start().starts_with("part ")
        || line_text.trim_start().starts_with("port ")
        || line_text.trim_start().starts_with("attribute ")
        || line_text.trim_start().starts_with("action ");
    if !declaration_like {
        return Ok(None);
    }
    let ranges: Vec<_> = find_reference_ranges(text, &word)
        .into_iter()
        .filter(|range| range.start.line == line)
        .collect();
    if ranges.is_empty() {
        return Ok(None);
    }
    Ok(Some(LinkedEditingRanges {
        ranges,
        word_pattern: None,
    }))
}

pub(crate) fn moniker(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<Moniker>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        Some(node) => node,
        None => return Ok(None),
    };
    Ok(Some(vec![hierarchy::moniker_for_node(node)]))
}

pub(crate) fn prepare_type_hierarchy(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        Some(node) => node,
        None => return Ok(None),
    };
    Ok(Some(vec![hierarchy::type_hierarchy_item_for_node(node)]))
}

pub(crate) fn supertypes(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let items = state
        .semantic_graph
        .outgoing_typing_or_specializes_targets(node)
        .into_iter()
        .map(hierarchy::type_hierarchy_item_for_node)
        .collect();
    Ok(Some(items))
}

pub(crate) fn subtypes(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<TypeHierarchyItem>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let items = state
        .semantic_graph
        .incoming_typing_or_specializes_sources(node)
        .into_iter()
        .map(hierarchy::type_hierarchy_item_for_node)
        .collect();
    Ok(Some(items))
}

pub(crate) fn prepare_call_hierarchy(
    state: &ServerState,
    uri: Url,
    pos: Position,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    let uri_norm = util::normalize_file_uri(&uri);
    let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
        Some(node) => node,
        None => return Ok(None),
    };
    Ok(Some(vec![hierarchy::call_hierarchy_item_for_node(node)]))
}

pub(crate) fn incoming_calls(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let from_ranges = vec![range];
    let calls = state
        .semantic_graph
        .incoming_perform_sources(node)
        .into_iter()
        .map(|src| CallHierarchyIncomingCall {
            from: hierarchy::call_hierarchy_item_for_node(src),
            from_ranges: from_ranges.clone(),
        })
        .collect();
    Ok(Some(calls))
}

pub(crate) fn outgoing_calls(
    state: &ServerState,
    uri: Url,
    range: Range,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    let node = match state
        .semantic_graph
        .find_node_at_position(&uri, range.start)
    {
        Some(node) => node,
        None => return Ok(None),
    };
    let from_ranges = vec![range];
    let calls = state
        .semantic_graph
        .outgoing_perform_targets(node)
        .into_iter()
        .map(|target| CallHierarchyOutgoingCall {
            to: hierarchy::call_hierarchy_item_for_node(target),
            from_ranges: from_ranges.clone(),
        })
        .collect();
    Ok(Some(calls))
}
