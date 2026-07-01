use std::collections::HashSet;

use sysml_model::{ElementKind, TextPosition, TextRange};
use url::Url;

use crate::dto::{
    CompletionEditShape, CompletionItemDto, CompletionItemKindDto,
    CompletionItemLabelDetailsDto, CompletionResult, CompletionTextEditDto,
};
use crate::keywords::keyword_doc;
use crate::presentation_hover::hover_markdown_for_node;
use crate::references::TYPE_LOOKUP_KINDS;
use crate::text::{completion_prefix, line_prefix_at_position};
use crate::workspace::WorkspaceSnapshot;

pub const PART_TYPE_LOOKUP_KINDS: &[&str] = &["part def"];
pub const PORT_TYPE_LOOKUP_KINDS: &[&str] = &["port def", "interface"];
pub const ATTRIBUTE_TYPE_LOOKUP_KINDS: &[&str] = &[
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

/// Completion context detected from the line prefix at the cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionContext {
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
    item: CompletionItemDto,
    tier: i32,
    score: i32,
}

#[derive(Debug, Default)]
struct CompletionSemanticHints {
    same_file_uri: Option<Url>,
    preferred_names: HashSet<String>,
    container_names: HashSet<String>,
}

/// Compute protocol-neutral completion items for a document path and cursor position.
pub fn complete(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
) -> Option<CompletionResult> {
    let uri = workspace.resolve_uri_for_path(document_path)?;
    let text = workspace.document_text(&uri)?;
    let line_prefix = line_prefix_at_position(text, position.line, position.character);
    let context = refine_completion_context(workspace, &uri, position, detect_completion_context(&line_prefix));
    let hints = completion_semantic_hints(workspace, &uri, position, &context);
    let edit_shape = completion_edit_shape(position, context.prefix());
    let items = rank_completion_candidates(collect_completion_candidates(
        workspace,
        &uri,
        &context,
        &hints,
        &edit_shape,
    ));
    Some(CompletionResult {
        items,
        is_incomplete: false,
    })
}

pub fn detect_completion_context(line_prefix: &str) -> CompletionContext {
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

pub fn completion_edit_shape(pos: TextPosition, prefix: &str) -> CompletionEditShape {
    CompletionEditShape {
        replace_range: TextRange::new(
            TextPosition::new(
                pos.line,
                pos.character.saturating_sub(prefix.chars().count() as u32),
            ),
            pos,
        ),
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
    workspace: &impl WorkspaceSnapshot,
    uri: &Url,
    pos: TextPosition,
    context: CompletionContext,
) -> CompletionContext {
    match context {
        CompletionContext::TopLevelKeyword { prefix }
        | CompletionContext::General { prefix }
        | CompletionContext::BodyStatement { prefix }
            if prefix.is_empty() =>
        {
            if let Some(node) = workspace
                .semantic_graph()
                .find_deepest_node_at_position(uri, pos)
            {
                if node.element_kind == ElementKind::Package
                    || node.element_kind.as_str().ends_with(" def")
                {
                    return CompletionContext::BodyStatement { prefix };
                }
            }
            CompletionContext::TopLevelKeyword { prefix }
        }
        other => other,
    }
}

fn completion_semantic_hints(
    workspace: &impl WorkspaceSnapshot,
    uri: &Url,
    pos: TextPosition,
    context: &CompletionContext,
) -> CompletionSemanticHints {
    if !workspace.supports_semantic_queries() {
        return CompletionSemanticHints::default();
    }

    let mut hints = CompletionSemanticHints {
        same_file_uri: Some(workspace.normalize_uri(uri)),
        ..CompletionSemanticHints::default()
    };

    let graph = workspace.semantic_graph();
    if let Some(node) = graph.find_deepest_node_at_position(uri, pos) {
        hints.preferred_names.insert(node.name.clone());
        if let Some(parent_id) = &node.parent_id {
            hints
                .container_names
                .insert(parent_id.qualified_name.clone());
        }
        for ancestor in graph.ancestors_of(node) {
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
    workspace: &impl WorkspaceSnapshot,
    uri: &Url,
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    edit_shape: &CompletionEditShape,
) -> Vec<CompletionCandidate> {
    let mut candidates = Vec::new();
    collect_snippet_candidates(context, edit_shape, &mut candidates);
    collect_keyword_candidates(context, edit_shape, &mut candidates);
    collect_symbol_candidates(workspace, uri, context, hints, edit_shape, &mut candidates);
    rank_candidates_in_place(context, hints, &mut candidates);
    dedupe_completion_candidates(candidates)
}

fn collect_snippet_candidates(
    context: &CompletionContext,
    edit_shape: &CompletionEditShape,
    out: &mut Vec<CompletionCandidate>,
) {
    match context {
        CompletionContext::DeclarationModifier { keyword, .. }
            if DECLARATION_MODIFIER_KEYWORDS.contains(&"def") =>
        {
            out.push(snippet_candidate(
                "def",
                format!("{keyword} definition"),
                "Declare a reusable definition",
                "def ${1:Name} {\n\t$0\n}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
        }
        CompletionContext::DeclarationModifier { .. } => {}
        CompletionContext::TopLevelKeyword { .. } | CompletionContext::BodyStatement { .. } => {
            let mut part_def_candidate = snippet_candidate(
                "part def",
                "part definition",
                "Declare a reusable part definition",
                "part def ${1:Name} {\n\t$0\n}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            );
            part_def_candidate.score = 40;
            out.push(part_def_candidate);
            out.push(snippet_candidate(
                "part",
                "part usage",
                "Declare a typed part usage",
                "part ${1:name} : ${2:Type}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "port",
                "port usage",
                "Declare a typed port usage",
                "port ${1:name} : ${2:PortType}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "attribute",
                "attribute usage",
                "Declare a typed attribute usage",
                "attribute ${1:name} : ${2:AttributeType}",
                CompletionItemKindDto::Snippet,
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
    kind: CompletionItemKindDto,
    edit_shape: &CompletionEditShape,
) -> CompletionCandidate {
    let detail = detail.into();
    let documentation = documentation.into();
    CompletionCandidate {
        label: label.to_string(),
        item: CompletionItemDto {
            label: label.to_string(),
            kind: Some(kind),
            detail: Some(detail.clone()),
            documentation: Some(documentation.clone()),
            documentation_is_markdown: false,
            label_details: None,
            filter_text: Some(label.to_string()),
            text_edit: Some(CompletionTextEditDto {
                range: edit_shape.replace_range,
                new_text: snippet.to_string(),
            }),
            insert_text_format_snippet: true,
            sort_text: None,
            preselect: false,
            deprecated: false,
            resolve_detail: Some(detail),
            resolve_documentation: Some(documentation),
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
            item: CompletionItemDto {
                label: (*keyword).to_string(),
                kind: Some(CompletionItemKindDto::Keyword),
                detail: Some("keyword".to_string()),
                documentation: keyword_doc(keyword).map(str::to_string),
                documentation_is_markdown: false,
                label_details: None,
                filter_text: Some((*keyword).to_string()),
                text_edit: Some(CompletionTextEditDto {
                    range: edit_shape.replace_range,
                    new_text: (*keyword).to_string(),
                }),
                insert_text_format_snippet: false,
                sort_text: None,
                preselect: false,
                deprecated: false,
                resolve_detail: None,
                resolve_documentation: None,
            },
            tier: TIER_KEYWORD_FALLBACK,
            score: 0,
        });
    }
}

fn collect_symbol_candidates(
    workspace: &impl WorkspaceSnapshot,
    _current_uri: &Url,
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    edit_shape: &CompletionEditShape,
    out: &mut Vec<CompletionCandidate>,
) {
    let prefix = context.prefix().to_lowercase();
    let graph = workspace.semantic_graph();

    for entry in workspace.symbol_table() {
        if !prefix.is_empty() && !entry.name.to_lowercase().contains(&prefix) {
            continue;
        }
        let detail = entry.detail.clone();
        let node = graph.nodes_for_uri(&entry.uri).into_iter().find(|node| {
            node.name == entry.name
                && detail
                    .as_deref()
                    .is_none_or(|detail| detail == node.element_kind)
        });
        let documentation = node
            .map(|node| hover_markdown_for_node(graph, node, false))
            .or_else(|| entry.description.clone());
        let label_details = entry.container_name.as_ref().map(|container| {
            CompletionItemLabelDetailsDto {
                detail: Some(format!(
                    " - {}",
                    entry.detail.as_deref().unwrap_or("symbol")
                )),
                description: Some(container.clone()),
            }
        });
        let kind = entry
            .detail
            .as_deref()
            .map(element_kind_to_completion_kind);
        out.push(CompletionCandidate {
            label: entry.name.clone(),
            item: CompletionItemDto {
                label: entry.name.clone(),
                kind,
                detail: detail.clone(),
                documentation: documentation.clone(),
                documentation_is_markdown: documentation
                    .as_ref()
                    .is_some_and(|doc| doc.contains("```")),
                label_details,
                filter_text: Some(entry.name.clone()),
                text_edit: Some(CompletionTextEditDto {
                    range: edit_shape.replace_range,
                    new_text: entry.name.clone(),
                }),
                insert_text_format_snippet: false,
                sort_text: None,
                preselect: false,
                deprecated: false,
                resolve_detail: detail,
                resolve_documentation: documentation,
            },
            tier: if hints.same_file_uri.as_ref() == Some(&entry.uri) {
                TIER_SAME_FILE_COMPATIBLE
            } else {
                TIER_GENERIC_SYMBOL
            },
            score: 0,
        });
    }
}

fn element_kind_to_completion_kind(kind: &str) -> CompletionItemKindDto {
    match kind {
        "package" => CompletionItemKindDto::Module,
        "part def" => CompletionItemKindDto::Class,
        "port def" | "interface" => CompletionItemKindDto::Interface,
        "action def" => CompletionItemKindDto::Function,
        "attribute def" | "attribute" => CompletionItemKindDto::Property,
        "part" | "item" => CompletionItemKindDto::Variable,
        "requirement def" | "case def" | "analysis def" => CompletionItemKindDto::Event,
        _ => CompletionItemKindDto::Reference,
    }
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
                (true, _) => 420,
                (_, true) => 45,
                _ => 0,
            };

        let kind_matches_context = match context {
            CompletionContext::TypeReference { expected_kinds, .. } => {
                entry_kind_matches(detail, expected_kinds)
            }
            CompletionContext::DeclarationModifier { .. } => candidate.label == "def",
            CompletionContext::BodyStatement { .. } | CompletionContext::TopLevelKeyword { .. } => {
                matches!(
                    candidate.item.kind,
                    Some(CompletionItemKindDto::Keyword) | Some(CompletionItemKindDto::Snippet)
                )
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
            CompletionContext::DeclarationModifier { .. } => score -= 140,
            CompletionContext::DeclarationName { .. } => score -= 40,
            CompletionContext::BodyStatement { .. } => score -= 20,
            CompletionContext::TopLevelKeyword { .. }
            | CompletionContext::General { .. }
            | CompletionContext::TypeReference { .. }
            | CompletionContext::QualifiedReference { .. }
            | CompletionContext::MemberReference { .. } => {}
        }

        if kind_matches_context && tier == TIER_GENERIC_SYMBOL {
            tier = TIER_WORKSPACE_COMPATIBLE;
        }
        if matches!(context, CompletionContext::TypeReference { .. }) && !kind_matches_context {
            tier = tier.min(TIER_GENERIC_SYMBOL);
            score -= 250;
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

fn dedupe_completion_candidates(candidates: Vec<CompletionCandidate>) -> Vec<CompletionCandidate> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for candidate in candidates {
        let detail = candidate.item.detail.as_deref().unwrap_or("");
        let description = candidate
            .item
            .label_details
            .as_ref()
            .and_then(|details| details.description.as_deref())
            .unwrap_or("");
        let key = format!("{}|{}|{}", candidate.label, detail, description);
        if seen.insert(key) {
            deduped.push(candidate);
        }
    }
    deduped
}

fn rank_completion_candidates(mut candidates: Vec<CompletionCandidate>) -> Vec<CompletionItemDto> {
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
            item.preselect = idx == 0;
            item
        })
        .collect()
}
