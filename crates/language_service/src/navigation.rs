use sysml_query::resolved_slice::{
    AuthoredUnit, ElementEvaluation, ElementInspection, PublishedModel, QueryOutcome, ReferenceAt,
    SymbolId, UnitResolution,
};
use sysml_query::resolved_slice::{TextPosition, TextRange};

use crate::dto::HoverResult;
use crate::keywords::keyword_hover_markdown;
use crate::workspace::WorkspaceSnapshot;

pub fn hover_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<HoverResult> {
    let uri = workspace.resolve_uri_for_path(path)?;
    let uri_norm = workspace.normalize_uri(&uri);
    // The token under the cursor, and its simple name, from the syntax service: which characters
    // continue an identifier and where a qualified name divides are the grammar's rules, not a
    // rule each service restates.
    // The handle is held for as long as the token, which borrows its source text.
    let parsed = workspace.parsed(&uri_norm)?;
    let token = parsed.token_at(position.line, position.character)?;
    let lookup_name = token.simple_name().to_string();
    let range = Some(TextRange {
        start: TextPosition {
            line: token.range.start_line,
            character: token.range.start_character,
        },
        end: TextPosition {
            line: token.range.end_line,
            character: token.range.end_character,
        },
    });

    if let Some(md) = unit_literal_hover_markdown(workspace, &uri_norm, position) {
        return Some(HoverResult {
            contents: md,
            range,
            semantic_status: workspace.semantic_status(),
        });
    }

    if let Some(model) = workspace.published_model() {
        if let Some(at) = resolved(model.inspection().inspect_at(uri_norm.as_str(), position)) {
            let inspected = match at.referenced {
                ReferenceAt::Resolved(target) => Some(*target),
                ReferenceAt::Unresolved => {
                    return Some(HoverResult {
                        contents: format!(
                            "**Unresolved reference** `{lookup_name}`\n\nSpec42 could not resolve this name in the current immutable publication."
                        ),
                        range,
                        semantic_status: workspace.semantic_status(),
                    });
                }
                _ => at.containing,
            };
            if let Some(element) = inspected {
                return Some(HoverResult {
                    contents: inspection_hover_markdown(
                        model,
                        &element,
                        model.document_identity(element.location.document)
                            != Some(uri_norm.as_str()),
                    ),
                    range,
                    semantic_status: workspace.semantic_status(),
                });
            }
        }
    }

    if let Some(md) = unit_literal_hover_markdown(workspace, &uri_norm, position) {
        return Some(HoverResult {
            contents: md,
            range,
            semantic_status: workspace.semantic_status(),
        });
    }

    // Last resort: if nothing resolved the word as a declared symbol, reference, or literal,
    // check whether it's a reserved keyword. Deliberately not tried earlier — checking by text
    // alone (with no position/grammar context) would otherwise hijack hover for any identifier
    // that happens to share spelling with a keyword (e.g. a part usage literally named `frame`).
    if let Some(md) = keyword_hover_markdown(&lookup_name) {
        return Some(HoverResult {
            contents: md,
            range,
            semantic_status: workspace.semantic_status(),
        });
    }

    Some(HoverResult {
        contents: format!(
            "**Unresolved reference** `{}`\n\nSpec42 could not resolve this name in the current scope, imports, or indexed workspace symbols.",
            lookup_name
        ),
        range,
        semantic_status: workspace.semantic_status(),
    })
}

fn inspection_hover_markdown(
    model: &PublishedModel,
    element: &ElementInspection,
    show_location: bool,
) -> String {
    let name = element.name.as_deref().unwrap_or("(anonymous)");
    let kind = human_kind(element.kind.as_str());
    let qualified_name = model.qualified_name(element.identity).unwrap_or_default();
    let mut markdown = format!("**{}** `{name}`\n\n", kind);
    markdown.push_str("```sysml\n");
    markdown.push_str(qualified_name);
    markdown.push_str("\n```\n\n");
    markdown.push_str(&format!("**Qualified name:** `{qualified_name}`\n\n"));
    if let Some(role) = element.role {
        markdown.push_str(&format!("**Role:** `{}`\n\n", role.as_str()));
    }
    if let Some((container, _)) = qualified_name.rsplit_once("::") {
        markdown.push_str(&format!("**Container:** `{container}`\n\n"));
    }
    if let Some(types) = resolved(model.types().direct_types(element.identity)) {
        let names = types
            .iter()
            .filter_map(|typing| model.qualified_name(typing.symbol))
            .collect::<Vec<_>>();
        if !names.is_empty() {
            markdown.push_str(&format!("**Declared type:** `{}`\n\n", names.join("`, `")));
        }
    }
    for documentation in element.documentation.iter() {
        markdown.push_str(model.text(documentation.text).unwrap_or_default());
        markdown.push_str("\n\n");
    }
    if show_location {
        markdown.push_str(&format!(
            "*Defined in:* {}",
            model
                .document_identity(element.location.document)
                .unwrap_or_default()
        ));
    }
    markdown
}

fn human_kind(kind: &str) -> String {
    let mut words = String::new();
    for (index, character) in kind.chars().enumerate() {
        if index > 0 && character.is_ascii_uppercase() {
            words.push(' ');
        }
        words.push(character.to_ascii_lowercase());
    }
    words
}

/// The unit token under the cursor, rendered from what the publication settled for it.
///
/// The publication owns the token: its authored spelling, its exact range, and whether it names a
/// unit, names several, names none, or is a unit expression this engine does not decompose. Hover
/// selects the token covering the cursor and formats the outcome; it does not look a symbol up in
/// a catalog of its own, and every state it can show is one the owner published.
fn unit_literal_hover_markdown(
    workspace: &impl WorkspaceSnapshot,
    uri: &url::Url,
    position: TextPosition,
) -> Option<String> {
    let unit = element_evaluation_at(workspace, uri, position)?
        .units
        .iter()
        .find(|unit| range_covers(unit.location.range, position))
        .cloned()?;
    Some(unit_hover_markdown(workspace.published_model(), &unit))
}

/// Formats one published unit outcome.
///
/// The named units are element handles, so the text a reader sees is asked of the publication.
/// Without one -- a syntax-only snapshot -- the outcome's shape is still shown; only the names
/// are missing, which is what the publication actually knows.
fn unit_hover_markdown(model: Option<&PublishedModel>, unit: &AuthoredUnit) -> String {
    let name = |symbol| {
        model
            .and_then(|model| model.qualified_name(symbol))
            .unwrap_or_default()
    };
    let mut lines = vec![
        format!(
            "**Unit literal** `[{}]`",
            model
                .and_then(|model| model.text(unit.authored))
                .unwrap_or_default()
        ),
        String::new(),
    ];
    match &unit.resolution {
        UnitResolution::Resolved(resolved) => {
            lines.push(format!("*{}*", name(resolved.unit)));
            for dimension in resolved.dimensions.iter() {
                lines.push(format!("Measured in `{}`", name(*dimension)));
            }
        }
        UnitResolution::UnknownSymbol => lines.push(
            "No unit with this symbol is declared in the admitted measurement catalog.".to_string(),
        ),
        UnitResolution::Ambiguous(candidates) => {
            lines.push("Several admitted units carry this symbol:".to_string());
            for candidate in candidates.iter() {
                lines.push(format!("- `{}`", name(*candidate)));
            }
        }
        UnitResolution::UnsupportedExpression => lines.push(
            "This is a unit expression rather than a single unit symbol, which Spec42 does not \
             decompose."
                .to_string(),
        ),
        UnitResolution::CatalogUnavailable => lines.push(
            "No measurement catalog is admitted to this workspace, so unit symbols cannot be \
             resolved."
                .to_string(),
        ),
    }
    lines.join("\n")
}

/// The settled evaluation of the element a cursor position identifies.
///
/// Prefers what a reference under the cursor points at, and falls back to the declaration the
/// cursor is inside, mirroring what hover renders for the same position.
fn element_evaluation_at(
    workspace: &impl WorkspaceSnapshot,
    uri: &url::Url,
    position: TextPosition,
) -> Option<ElementEvaluation> {
    let model = workspace.published_model()?;
    let at = resolved(model.inspection().inspect_at(uri.as_str(), position))?;
    let symbol = match &at.referenced {
        ReferenceAt::Resolved(inspection) => Some(inspection.identity),
        _ => at.containing.as_ref().map(|containing| containing.identity),
    }?;
    element_evaluation(model, symbol)
}

fn element_evaluation(model: &PublishedModel, symbol: SymbolId) -> Option<ElementEvaluation> {
    resolved(model.evaluation().evaluate(symbol))
}

/// The value of an outcome that carried one, whatever completeness it was published under.
fn resolved<T>(outcome: QueryOutcome<T>) -> Option<T> {
    match outcome.answer {
        sysml_query::resolved_slice::QueryAnswer::Resolved(value) => Some(value),
        _ => None,
    }
}

fn range_covers(range: sysml_query::resolved_slice::TextRange, position: TextPosition) -> bool {
    let after_start =
        (range.start.line, range.start.character) <= (position.line, position.character);
    let before_end = (position.line, position.character) <= (range.end.line, range.end.character);
    after_start && before_end
}

pub fn hover(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<HoverResult> {
    hover_at_position(workspace, path, position)
}

pub fn goto_definition(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> crate::dto::DefinitionResult {
    crate::references::goto_definition_at_position(workspace, path, position)
}

pub fn find_references(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
    include_declaration: bool,
) -> crate::dto::ReferencesResult {
    crate::references::find_references_at_position(workspace, path, position, include_declaration)
}
