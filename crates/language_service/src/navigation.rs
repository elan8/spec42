use sysml_query::resolved_slice::{
    AuthoredUnit, EffectiveTypeOrigin, ElementDetails, ElementEvaluation, PublishedModel,
    QueryAnswer, QueryOutcome, ReferenceAt, ReferencedDetails, RelationshipOutcome, SymbolId,
    UnitResolution,
};
use sysml_query::resolved_slice::{TextPosition, TextRange};

use crate::completion::element_kind_label;
use crate::dto::HoverResult;
use crate::keywords::keyword_help;
use crate::presentation_hover::{
    render_hover_markdown, HoverBlock, HoverLink, HoverRelation, HoverReport, HoverResolutionState,
    HoverUnitOutcome,
};
use crate::workspace::WorkspaceSnapshot;

pub fn hover_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<HoverResult> {
    let (report, range) = hover_report_and_range(workspace, path, position)?;
    Some(HoverResult {
        contents: render_hover_markdown(&report),
        range: Some(range),
        semantic_status: workspace.semantic_status(),
    })
}

/// Builds the structured hover report used by every presentation surface.
///
/// Elements, references, units, keywords, and unresolved names all pass through this function;
/// the editor boundary only renders its answer.
pub fn hover_report(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<HoverReport> {
    hover_report_and_range(workspace, path, position).map(|(report, _)| report)
}

fn hover_report_and_range(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<(HoverReport, TextRange)> {
    let uri = workspace.resolve_uri_for_path(path)?;
    let uri_norm = workspace.normalize_uri(&uri);
    let parsed = workspace.parsed(&uri_norm)?;
    let token = parsed.token_at(position.line, position.character)?;
    let range = TextRange::new(
        TextPosition::new(token.range.start_line, token.range.start_character),
        TextPosition::new(token.range.end_line, token.range.end_character),
    );
    if !workspace.supports_semantic_queries() {
        return Some((
            state_report(
                HoverResolutionState::Loading,
                "workspace semantics",
                token.simple_name(),
                Some("Spec42 is indexing this workspace; semantic hover will be available after the first publication is ready."),
            ),
            range,
        ));
    }
    if let Some(report) = unit_literal_hover_report(workspace, &uri_norm, position) {
        return Some((report, range));
    }
    if let Some(model) = workspace.published_model() {
        let outcome = model
            .inspection()
            .element_details_at(uri_norm.as_str(), position);
        if let Some(report) = contextual_hover_report(
            model,
            &outcome.answer,
            token.simple_name(),
            uri_norm.as_str(),
        ) {
            return Some((report, range));
        }
    }
    if let Some(help) = keyword_help(token.simple_name()) {
        return Some((
            HoverReport {
                blocks: vec![HoverBlock::Keyword {
                    keyword: token.simple_name().to_string(),
                    description: help.description.to_string(),
                    syntax: help.syntax.map(str::to_string),
                }],
                links: vec![],
            },
            range,
        ));
    }
    Some((state_report(HoverResolutionState::Unresolved, "reference", token.simple_name(), Some("Spec42 could not resolve this name in the current scope, imports, or indexed workspace symbols.")), range))
}

fn contextual_hover_report(
    model: &PublishedModel,
    answer: &QueryAnswer<sysml_query::resolved_slice::ElementDetailsAt>,
    token: &str,
    current_document: &str,
) -> Option<HoverReport> {
    let at = match answer {
        QueryAnswer::Resolved(at) => at,
        QueryAnswer::Unresolved => return None,
        QueryAnswer::Ambiguous(_) => return Some(state_report(HoverResolutionState::Ambiguous, "semantic result", token, None)),
        QueryAnswer::Unsupported => return Some(state_report(HoverResolutionState::Unsupported, "semantic result", token, None)),
        QueryAnswer::Recovery => return Some(state_report(HoverResolutionState::Recovery, "semantic result", token, Some("Spec42 recovered this occurrence from incomplete or invalid syntax; no settled semantic target is available."))),
        QueryAnswer::Incomplete => return Some(state_report(HoverResolutionState::Incomplete, "semantic result", token, Some("Model analysis did not converge, so no settled semantic target is available."))),
    };

    let relation = at.reference_kind.and_then(reference_context);
    let reference_subject = at
        .reference_kind
        .and_then(reference_state_subject)
        .unwrap_or("reference");
    match &at.referenced {
        ReferencedDetails::None => at
            .containing
            .as_ref()
            .map(|details| element_hover_report(model, details, None, current_document)),
        ReferencedDetails::Resolved(target) => Some(element_hover_report(
            model,
            target,
            relation.map(|relation| (relation, at.containing.as_ref())),
            current_document,
        )),
        ReferencedDetails::Ambiguous(candidates) => {
            let mut report = state_report(
                HoverResolutionState::Ambiguous,
                reference_subject,
                token,
                None,
            );
            if !candidates.is_empty() {
                let names = candidates
                    .iter()
                    .map(|candidate| {
                        let name = model
                            .qualified_name(candidate.inspection.identity)
                            .unwrap_or("(anonymous)");
                        push_hover_link(
                            model,
                            &mut report.links,
                            name,
                            candidate.inspection.location,
                        );
                        name.to_string()
                    })
                    .collect();
                report.blocks.push(HoverBlock::Candidates(names));
            }
            Some(report)
        }
        ReferencedDetails::Unresolved => Some(state_report(
            HoverResolutionState::Unresolved,
            reference_subject,
            token,
            Some("Spec42 could not resolve this name in the current scope and admitted imports."),
        )),
        ReferencedDetails::Unsupported => Some(state_report(
            HoverResolutionState::Unsupported,
            reference_subject,
            token,
            Some("Spec42 recognizes this reference, but does not yet resolve this form."),
        )),
        ReferencedDetails::Incomplete => Some(state_report(
            HoverResolutionState::Incomplete,
            reference_subject,
            token,
            Some("Model analysis did not converge, so this reference has no settled target."),
        )),
    }
}

fn element_hover_report(
    model: &PublishedModel,
    details: &ElementDetails,
    context: Option<(HoverRelation, Option<&ElementDetails>)>,
    current_document: &str,
) -> HoverReport {
    let element = &details.inspection;
    let name = element.name.as_deref().unwrap_or("(anonymous)");
    let kind = element_kind_label(element.kind).to_string();
    let direct_types = details
        .effective_typing
        .types
        .iter()
        .filter(|entry| entry.origin == EffectiveTypeOrigin::Direct)
        .filter_map(|entry| model.qualified_name(entry.element.identity))
        .collect::<Vec<_>>();
    let mut blocks = Vec::new();
    let mut links = Vec::new();
    if let Some((relation, containing)) = context {
        if let Some(containing) = containing {
            if containing.inspection.name.is_some() {
                let subject = model
                    .qualified_name(containing.inspection.identity)
                    .or(containing.inspection.name.as_deref());
                blocks.push(HoverBlock::Context {
                    relation,
                    subject: subject.map(str::to_string),
                });
                if let Some(subject) = subject {
                    push_hover_link(model, &mut links, subject, containing.inspection.location);
                }
            } else {
                blocks.push(HoverBlock::Context {
                    relation,
                    subject: None,
                });
            }
        } else {
            blocks.push(HoverBlock::Context {
                relation,
                subject: None,
            });
        }
    }
    blocks.push(HoverBlock::Identity {
        kind,
        role: element.role.map(|role| role.as_str().to_string()),
        name: name.to_string(),
        direct_types: direct_types
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    });
    push_hover_link(model, &mut links, name, element.location);
    for entry in details
        .effective_typing
        .types
        .iter()
        .filter(|entry| entry.origin == EffectiveTypeOrigin::Direct)
    {
        if let Some(type_name) = model.qualified_name(entry.element.identity) {
            push_hover_link(model, &mut links, type_name, entry.element.location);
        }
    }

    let qualified_name = model.qualified_name(element.identity);
    if let Some(qualified) = qualified_name {
        if qualified != name {
            blocks.push(HoverBlock::QualifiedName(qualified.to_string()));
            push_hover_link(model, &mut links, qualified, element.location);
        }
    }
    if let Some(owner) = details
        .owner
        .as_ref()
        .and_then(|owner| model.qualified_name(owner.identity))
    {
        let owner_is_qualified_prefix = qualified_name
            .and_then(|qualified| qualified.rsplit_once("::"))
            .is_some_and(|(qualified_owner, _)| qualified_owner == owner);
        if !owner_is_qualified_prefix {
            blocks.push(HoverBlock::Owner(owner.to_string()));
            if let Some(entry) = details.owner.as_ref() {
                push_hover_link(model, &mut links, owner, entry.location);
            }
        }
    }
    for effective in details.effective_typing.types.iter() {
        if let EffectiveTypeOrigin::Inherited(origin_symbol) = &effective.origin {
            let type_name = model
                .qualified_name(effective.element.identity)
                .unwrap_or("(anonymous)");
            let origin = model
                .qualified_name(*origin_symbol)
                .unwrap_or("(anonymous)");
            blocks.push(HoverBlock::InheritedType {
                type_name: type_name.to_string(),
                inherited_from: origin.to_string(),
            });
            push_hover_link(model, &mut links, type_name, effective.element.location);
            push_symbol_link(model, &mut links, origin, *origin_symbol);
        }
    }
    if details.effective_typing.outcome != RelationshipOutcome::Resolved
        && details.effective_typing.outcome != RelationshipOutcome::NotApplicable
    {
        blocks.push(HoverBlock::TypeResolution(
            details.effective_typing.outcome.as_str().to_string(),
        ));
    }
    for documentation in element.documentation.iter() {
        let text = model.text(documentation.text).unwrap_or_default();
        if !text.trim().is_empty() {
            blocks.push(HoverBlock::Documentation(text.trim().to_string()));
        }
    }
    if let Some(identity) = model.document_identity(element.location.document) {
        if identity != current_document {
            blocks.push(HoverBlock::Source {
                identity: identity.to_string(),
                line: element.location.range.start.line.saturating_add(1),
            });
        }
    }
    HoverReport { blocks, links }
}

fn push_hover_link(
    model: &PublishedModel,
    links: &mut Vec<HoverLink>,
    label: &str,
    location: sysml_query::resolved_slice::SourceLocation,
) {
    let Some(uri) = model.document_identity(location.document) else {
        return;
    };
    let link = HoverLink {
        labels: vec![label.to_string()],
        uri: uri.to_string(),
        line: location.range.start.line,
        character: location.range.start.character,
    };
    if let Some(existing) = links.iter_mut().find(|existing| {
        existing.uri == link.uri
            && existing.line == link.line
            && existing.character == link.character
    }) {
        if !existing.labels.iter().any(|existing| existing == label) {
            existing.labels.push(label.to_string());
        }
    } else {
        links.push(link);
    }
}

fn push_symbol_link(
    model: &PublishedModel,
    links: &mut Vec<HoverLink>,
    label: &str,
    symbol: SymbolId,
) {
    if let QueryAnswer::Resolved(element) = model.inspection().inspect(symbol).answer {
        push_hover_link(model, links, label, element.location);
    }
}

fn reference_context(kind: &str) -> Option<HoverRelation> {
    match kind {
        "featureTyping" => Some(HoverRelation::TypeOf),
        "redefinition" => Some(HoverRelation::Redefines),
        "subsetting" | "referenceSubsetting" | "crossSubsetting" => Some(HoverRelation::Subsets),
        "specialization" => Some(HoverRelation::Specializes),
        "namespaceImport" | "membershipImport" | "filterImport" => Some(HoverRelation::Imports),
        "aliasBinding" => Some(HoverRelation::Aliases),
        _ => None,
    }
}

fn reference_state_subject(kind: &str) -> Option<&'static str> {
    match kind {
        "featureTyping" => Some("type reference"),
        "redefinition" => Some("redefinition target"),
        "subsetting" | "referenceSubsetting" | "crossSubsetting" => Some("subsetting target"),
        "specialization" => Some("specialization target"),
        "namespaceImport" | "membershipImport" | "filterImport" => Some("import target"),
        "aliasBinding" => Some("alias target"),
        _ => None,
    }
}

fn state_report(
    state: HoverResolutionState,
    subject: &str,
    token: &str,
    explanation: Option<&str>,
) -> HoverReport {
    HoverReport {
        blocks: vec![HoverBlock::Resolution {
            state,
            subject: subject.to_string(),
            token: token.to_string(),
            explanation: explanation.map(str::to_string),
        }],
        links: vec![],
    }
}

/// The unit token under the cursor, rendered from what the publication settled for it.
///
/// The publication owns the token: its authored spelling, its exact range, and whether it names a
/// unit, names several, names none, or is a unit expression this engine does not decompose. Hover
/// selects the token covering the cursor and formats the outcome; it does not look a symbol up in
/// a catalog of its own, and every state it can show is one the owner published.
fn unit_literal_hover_report(
    workspace: &impl WorkspaceSnapshot,
    uri: &url::Url,
    position: TextPosition,
) -> Option<HoverReport> {
    let unit = element_evaluation_at(workspace, uri, position)?
        .units
        .iter()
        .find(|unit| range_covers(unit.location.range, position))
        .cloned()?;
    Some(unit_hover_report(workspace.published_model(), &unit))
}

/// Formats one published unit outcome.
///
/// The named units are element handles, so the text a reader sees is asked of the publication.
/// Without one -- a syntax-only snapshot -- the outcome's shape is still shown; only the names
/// are missing, which is what the publication actually knows.
fn unit_hover_report(model: Option<&PublishedModel>, unit: &AuthoredUnit) -> HoverReport {
    let name = |symbol| {
        model
            .and_then(|model| model.qualified_name(symbol))
            .unwrap_or_default()
    };
    let outcome = match &unit.resolution {
        UnitResolution::Resolved(resolved) => HoverUnitOutcome::Resolved {
            unit: name(resolved.unit).to_string(),
            dimensions: resolved
                .dimensions
                .iter()
                .map(|symbol| name(*symbol).to_string())
                .collect(),
        },
        UnitResolution::UnknownSymbol => HoverUnitOutcome::UnknownSymbol,
        UnitResolution::Ambiguous(candidates) => HoverUnitOutcome::Ambiguous(
            candidates
                .iter()
                .map(|symbol| name(*symbol).to_string())
                .collect(),
        ),
        UnitResolution::UnsupportedExpression => HoverUnitOutcome::UnsupportedExpression,
        UnitResolution::CatalogUnavailable => HoverUnitOutcome::CatalogUnavailable,
    };
    let mut report = HoverReport {
        blocks: vec![HoverBlock::UnitLiteral {
            authored: model
                .and_then(|model| model.text(unit.authored))
                .unwrap_or_default()
                .to_string(),
            outcome,
        }],
        links: vec![],
    };
    if let Some(model) = model {
        match &unit.resolution {
            UnitResolution::Resolved(resolved) => {
                if let Some(label) = model.qualified_name(resolved.unit) {
                    push_symbol_link(model, &mut report.links, label, resolved.unit);
                }
                for symbol in resolved.dimensions.iter().copied() {
                    if let Some(label) = model.qualified_name(symbol) {
                        push_symbol_link(model, &mut report.links, label, symbol);
                    }
                }
            }
            UnitResolution::Ambiguous(candidates) => {
                for symbol in candidates.iter().copied() {
                    if let Some(label) = model.qualified_name(symbol) {
                        push_symbol_link(model, &mut report.links, label, symbol);
                    }
                }
            }
            _ => {}
        }
    }
    report
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

#[cfg(test)]
mod tests {
    use super::{reference_context, reference_state_subject};
    use crate::presentation_hover::HoverRelation;

    #[test]
    fn reference_context_is_exhaustive_for_contextual_hover_families() {
        assert_eq!(
            reference_context("featureTyping"),
            Some(HoverRelation::TypeOf)
        );
        assert_eq!(
            reference_context("redefinition"),
            Some(HoverRelation::Redefines)
        );
        assert_eq!(
            reference_context("subsetting"),
            Some(HoverRelation::Subsets)
        );
        assert_eq!(
            reference_context("namespaceImport"),
            Some(HoverRelation::Imports)
        );
        assert_eq!(reference_context("expressionOperand"), None);
        assert_eq!(
            reference_state_subject("featureTyping"),
            Some("type reference")
        );
    }
}
