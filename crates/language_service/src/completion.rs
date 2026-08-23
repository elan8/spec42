use std::collections::HashSet;

use sysml_query::resolved_slice::ElementKind;
use sysml_query::resolved_slice::{TextPosition, TextRange};
use url::Url;

use crate::dto::{
    CompletionEditShape, CompletionItemDto, CompletionItemKindDto, CompletionItemLabelDetailsDto,
    CompletionResult, CompletionTextEditDto,
};
use crate::keywords::{keyword_doc, sysml_keywords};
use crate::text::{completion_prefix, line_prefix_at_position};
use crate::workspace::WorkspaceSnapshot;

/// The definition kinds offered when a typed declaration gives no narrower expectation.
pub const DEFAULT_TYPE_LOOKUP_KINDS: &[ElementKind] = &[
    ElementKind::PartDefinition,
    ElementKind::PortDefinition,
    ElementKind::InterfaceDefinition,
    ElementKind::ItemDefinition,
    ElementKind::AttributeDefinition,
    ElementKind::ActionDefinition,
    ElementKind::OccurrenceDefinition,
    ElementKind::FlowConnectionDefinition,
    ElementKind::AllocationDefinition,
    ElementKind::StateDefinition,
    ElementKind::RequirementDefinition,
    ElementKind::UseCaseDefinition,
    ElementKind::ConcernDefinition,
    ElementKind::Classifier,
    ElementKind::Class,
    ElementKind::Structure,
    ElementKind::DataType,
];
pub const PART_TYPE_LOOKUP_KINDS: &[ElementKind] = &[ElementKind::PartDefinition];
pub const PORT_TYPE_LOOKUP_KINDS: &[ElementKind] = &[
    ElementKind::PortDefinition,
    ElementKind::InterfaceDefinition,
];
pub const ATTRIBUTE_TYPE_LOOKUP_KINDS: &[ElementKind] = &[
    ElementKind::AttributeDefinition,
    ElementKind::ItemDefinition,
    ElementKind::EnumerationDefinition,
    ElementKind::OccurrenceDefinition,
    // What the former `"kermlDecl"` string stood for. The KerML declaration kinds are now
    // distinct metaclasses, so name the ones that can actually type an attribute.
    ElementKind::DataType,
    ElementKind::Classifier,
    ElementKind::Class,
    ElementKind::Structure,
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
        expected_kinds: &'static [ElementKind],
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
    /// The candidate's published element kind, where it came from a model member.
    ///
    /// Ranking matches on this rather than on `item.detail`, which is an English label built for
    /// display. Comparing display text was how the kind filter silently stopped matching.
    element_kind: Option<ElementKind>,
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
    let context = refine_completion_context(
        workspace,
        &uri,
        position,
        detect_completion_context(&line_prefix),
    );
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
) -> Option<(&'static [ElementKind], Option<String>, String)> {
    let before_trimmed = before_token.trim_end();
    if !before_trimmed.ends_with(':') || before_trimmed.ends_with("::") {
        return None;
    }

    let declaration_prefix = before_trimmed.strip_suffix(':')?.trim_end();
    let expected_kinds = match typed_declaration_keyword(declaration_prefix)? {
        "part" => PART_TYPE_LOOKUP_KINDS,
        "port" => PORT_TYPE_LOOKUP_KINDS,
        "attribute" => ATTRIBUTE_TYPE_LOOKUP_KINDS,
        _ => DEFAULT_TYPE_LOOKUP_KINDS,
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
    _workspace: &impl WorkspaceSnapshot,
    _uri: &Url,
    _pos: TextPosition,
    context: CompletionContext,
) -> CompletionContext {
    match context {
        CompletionContext::TopLevelKeyword { prefix }
        | CompletionContext::General { prefix }
        | CompletionContext::BodyStatement { prefix }
            if prefix.is_empty() =>
        {
            CompletionContext::TopLevelKeyword { prefix }
        }
        other => other,
    }
}

fn completion_semantic_hints(
    workspace: &impl WorkspaceSnapshot,
    uri: &Url,
    _pos: TextPosition,
    context: &CompletionContext,
) -> CompletionSemanticHints {
    if !workspace.supports_semantic_queries() {
        return CompletionSemanticHints::default();
    }

    let mut hints = CompletionSemanticHints {
        same_file_uri: Some(workspace.normalize_uri(uri)),
        ..CompletionSemanticHints::default()
    };

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
            out.push(snippet_candidate(
                "requirement def",
                "requirement definition",
                "Declare a reusable requirement definition",
                "requirement def ${1:Name} {\n\tdoc /* $2 */\n\t$0\n}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "interface def",
                "interface definition",
                "Declare a reusable interface definition",
                "interface def ${1:Name} {\n\t$0\n}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "action def",
                "action definition",
                "Declare a reusable action definition",
                "action def ${1:Name} {\n\t$0\n}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "connect",
                "connection usage",
                "Connect two ports or parts",
                "connect ${1:source} to ${2:target}",
                CompletionItemKindDto::Snippet,
                edit_shape,
            ));
            out.push(snippet_candidate(
                "import",
                "import statement",
                "Import a package or its members",
                "import ${1:Package::*}",
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
        element_kind: None,
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
        CompletionContext::TopLevelKeyword { .. } | CompletionContext::BodyStatement { .. } => {
            sysml_keywords()
        }
        CompletionContext::DeclarationModifier { .. } => DECLARATION_MODIFIER_KEYWORDS,
        _ => &[],
    };

    for keyword in keywords {
        out.push(CompletionCandidate {
            label: (*keyword).to_string(),
            element_kind: None,
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
    current_uri: &Url,
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    edit_shape: &CompletionEditShape,
    out: &mut Vec<CompletionCandidate>,
) {
    // Declaring a new name (or right after `def`) is not a symbol-reference position.
    if matches!(
        context,
        CompletionContext::DeclarationName { .. } | CompletionContext::DeclarationModifier { .. }
    ) {
        return;
    }

    let prefix = context.prefix().to_lowercase();

    // Without a typed prefix, positions that also offer keyword/snippet completions
    // would otherwise dump the entire workspace symbol table.
    if prefix.is_empty()
        && matches!(
            context,
            CompletionContext::TopLevelKeyword { .. }
                | CompletionContext::BodyStatement { .. }
                | CompletionContext::General { .. }
        )
    {
        return;
    }

    let qualifier = match context {
        CompletionContext::TypeReference {
            qualifier: Some(qualifier),
            ..
        }
        | CompletionContext::QualifiedReference { qualifier, .. } => Some(qualifier.as_str()),
        CompletionContext::MemberReference { receiver, .. } => Some(receiver.as_str()),
        _ => None,
    };

    let query_position = edit_shape.replace_range.end;
    let Some(model) = workspace.published_model() else {
        return;
    };
    let outcome = model.completion().visible_members(
        current_uri.as_str(),
        sysml_query::resolved_slice::TextPosition {
            line: query_position.line,
            character: query_position.character,
        },
        qualifier.map(|value| value.trim_end_matches(':')),
    );
    let mut members = match outcome {
        sysml_query::resolved_slice::QueryOutcome::Resolved(members)
        | sysml_query::resolved_slice::QueryOutcome::Recovered(members)
        | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(members) => members.into_vec(),
        _ => return,
    };
    if qualifier.is_some() {
        match model.completion().visible_members(
            current_uri.as_str(),
            sysml_query::resolved_slice::TextPosition {
                line: query_position.line,
                character: query_position.character,
            },
            None,
        ) {
            sysml_query::resolved_slice::QueryOutcome::Resolved(extra)
            | sysml_query::resolved_slice::QueryOutcome::Recovered(extra)
            | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(extra) => {
                members.extend(extra.into_vec())
            }
            _ => {}
        }
    }
    members.sort_by_key(|a| a.symbol);
    members.dedup_by(|a, b| a.symbol == b.symbol);
    for member in members {
        let name = member.name.to_string();
        if !prefix.is_empty() && !name.to_lowercase().contains(&prefix) {
            continue;
        }
        let detail = Some(query_kind_label(member.kind).to_string());
        let documentation = Some(format!(
            "**{}**\n\nQualified name: `{}`",
            name, member.qualified_name
        ));
        let label_details = Some(CompletionItemLabelDetailsDto {
            detail: Some(format!(" - {}", member.kind)),
            description: member
                .container_name
                .as_ref()
                .map(ToString::to_string)
                .or_else(|| Some(member.declaring_document.to_string())),
        });
        let kind = Some(element_kind_to_completion_kind(member.kind));
        out.push(CompletionCandidate {
            label: name.clone(),
            element_kind: Some(member.kind),
            item: CompletionItemDto {
                label: name.clone(),
                kind,
                detail: detail.clone(),
                documentation: documentation.clone(),
                documentation_is_markdown: documentation
                    .as_ref()
                    .is_some_and(|doc| doc.contains("```")),
                label_details,
                filter_text: Some(name.clone()),
                text_edit: Some(CompletionTextEditDto {
                    range: edit_shape.replace_range,
                    new_text: name.clone(),
                }),
                insert_text_format_snippet: false,
                sort_text: None,
                preselect: false,
                deprecated: false,
                resolve_detail: detail,
                resolve_documentation: documentation,
            },
            tier: if qualifier
                .is_some_and(|qualifier| member.declaring_document.contains(qualifier))
            {
                TIER_CONTEXT_COMPATIBLE_SAME_SCOPE
            } else if hints
                .same_file_uri
                .as_ref()
                .is_some_and(|uri| uri.as_str() == member.declaring_document.as_ref())
            {
                TIER_SAME_FILE_COMPATIBLE
            } else {
                TIER_GENERIC_SYMBOL
            },
            score: 0,
        });
    }
}

/// The editor icon for a published element kind.
///
/// Takes `ElementKind` rather than a string: this function used to match display labels such as
/// `"part def"` while being handed the raw debug spelling `"PartDefinition"`, so no arm ever fired
/// and every completion item was rendered as `Reference`. With the enum the mismatch cannot be
/// expressed.
fn element_kind_to_completion_kind(kind: ElementKind) -> CompletionItemKindDto {
    match kind {
        ElementKind::Package | ElementKind::LibraryPackage | ElementKind::Namespace => {
            CompletionItemKindDto::Module
        }
        ElementKind::PartDefinition
        | ElementKind::ItemDefinition
        | ElementKind::OccurrenceDefinition
        | ElementKind::IndividualDefinition
        | ElementKind::Definition
        | ElementKind::Class
        | ElementKind::Classifier
        | ElementKind::Structure
        | ElementKind::Association
        | ElementKind::AssociationStructure
        | ElementKind::DataType
        | ElementKind::Metaclass => CompletionItemKindDto::Class,
        ElementKind::PortDefinition
        | ElementKind::InterfaceDefinition
        | ElementKind::InterfaceUsage => CompletionItemKindDto::Interface,
        ElementKind::ActionDefinition
        | ElementKind::ActionUsage
        | ElementKind::CalculationDefinition
        | ElementKind::CalculationUsage
        | ElementKind::Behavior
        | ElementKind::Function
        | ElementKind::Predicate
        | ElementKind::Interaction
        | ElementKind::Step
        | ElementKind::Expression
        | ElementKind::BooleanExpression => CompletionItemKindDto::Function,
        ElementKind::AttributeDefinition | ElementKind::AttributeUsage | ElementKind::Feature => {
            CompletionItemKindDto::Property
        }
        ElementKind::PartUsage
        | ElementKind::ItemUsage
        | ElementKind::OccurrenceUsage
        | ElementKind::PortUsage
        | ElementKind::ReferenceUsage
        | ElementKind::ForLoopVariable => CompletionItemKindDto::Variable,
        ElementKind::RequirementDefinition
        | ElementKind::CaseDefinition
        | ElementKind::AnalysisCaseDefinition
        | ElementKind::UseCaseDefinition
        | ElementKind::VerificationCaseDefinition
        | ElementKind::ConcernDefinition
        | ElementKind::ConstraintDefinition => CompletionItemKindDto::Event,
        // `CompletionItemKindDto` has no `Enum` variant; an enumeration definition is a
        // classifier, so `Class` is the closest available icon.
        ElementKind::EnumerationDefinition => CompletionItemKindDto::Class,
        _ => CompletionItemKindDto::Reference,
    }
}

/// The surface-syntax label shown in a completion item's detail text.
fn query_kind_label(kind: ElementKind) -> &'static str {
    match kind {
        ElementKind::PartDefinition => "part def",
        ElementKind::PortDefinition => "port def",
        ElementKind::InterfaceDefinition => "interface def",
        ElementKind::ItemDefinition => "item def",
        ElementKind::AttributeDefinition => "attribute def",
        ElementKind::ActionDefinition => "action def",
        ElementKind::OccurrenceDefinition => "occurrence def",
        ElementKind::FlowConnectionDefinition => "flow def",
        ElementKind::AllocationDefinition => "allocation def",
        ElementKind::StateDefinition => "state def",
        ElementKind::RequirementDefinition => "requirement def",
        ElementKind::UseCaseDefinition => "use case def",
        ElementKind::ConcernDefinition => "concern def",
        ElementKind::EnumerationDefinition => "enum def",
        ElementKind::Package | ElementKind::LibraryPackage => "package",
        ElementKind::PartUsage => "part",
        ElementKind::PortUsage => "port",
        ElementKind::ItemUsage => "item",
        ElementKind::AttributeUsage => "attribute",
        ElementKind::ActionUsage => "action",
        other => other.as_str(),
    }
}

fn rank_candidates_in_place(
    context: &CompletionContext,
    hints: &CompletionSemanticHints,
    candidates: &mut [CompletionCandidate],
) {
    for candidate in candidates {
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
                entry_kind_matches(candidate.element_kind, expected_kinds)
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

fn entry_kind_matches(kind: Option<ElementKind>, expected_kinds: &[ElementKind]) -> bool {
    kind.is_some_and(|kind| expected_kinds.contains(&kind))
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
