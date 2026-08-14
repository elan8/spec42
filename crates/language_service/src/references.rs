use sysml_model::{TextPosition, TextRange};

use crate::dto::{DefinitionResult, ReferencesResult, SourceLocation};
use crate::workspace::WorkspaceSnapshot;

pub const TYPE_LOOKUP_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface def",
    "item def",
    "attribute def",
    "action def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "kermlDecl",
];

pub const TYPE_LOOKUP_ELEMENT_KINDS: &[sysml_model::ElementKind] = &[
    sysml_model::ElementKind::PartDef,
    sysml_model::ElementKind::PortDef,
    sysml_model::ElementKind::InterfaceDef,
    sysml_model::ElementKind::ItemDef,
    sysml_model::ElementKind::AttributeDef,
    sysml_model::ElementKind::ActionDef,
    sysml_model::ElementKind::OccurrenceDef,
    sysml_model::ElementKind::FlowDef,
    sysml_model::ElementKind::AllocationDef,
    sysml_model::ElementKind::StateDef,
    sysml_model::ElementKind::RequirementDef,
    sysml_model::ElementKind::UseCaseDef,
    sysml_model::ElementKind::ConcernDef,
    sysml_model::ElementKind::KermlDecl,
];

#[derive(Debug, Clone)]
pub struct ResolvedSymbolTarget {
    pub name: String,
    pub definition_location: SourceLocation,
    pub identifier_range: TextRange,
}

fn query_position(position: TextPosition) -> sysml_query::resolved_slice::TextPosition {
    sysml_query::resolved_slice::TextPosition {
        line: position.line,
        character: position.character,
    }
}

fn core_range(range: sysml_query::resolved_slice::TextRange) -> TextRange {
    TextRange::new(
        TextPosition::new(range.start.line, range.start.character),
        TextPosition::new(range.end.line, range.end.character),
    )
}

fn location(
    workspace: &impl WorkspaceSnapshot,
    value: sysml_query::resolved_slice::SourceLocation,
) -> SourceLocation {
    let path = workspace
        .resolve_uri_for_path(&value.document)
        .map(|uri| workspace.path_for_uri(&uri))
        .unwrap_or_else(|| value.document.to_string());
    SourceLocation {
        path,
        range: core_range(value.range),
    }
}

pub fn goto_definition_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> DefinitionResult {
    let Some(uri) = workspace.resolve_uri_for_path(path) else {
        return DefinitionResult {
            locations: Vec::new(),
        };
    };
    let Some(model) = workspace.published_model() else {
        return DefinitionResult {
            locations: Vec::new(),
        };
    };
    let outcome = model
        .navigation()
        .target_at(uri.as_str(), query_position(position));
    let targets: Vec<_> = match outcome {
        sysml_query::resolved_slice::QueryOutcome::Resolved(target)
        | sysml_query::resolved_slice::QueryOutcome::Recovered(target)
        | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(target) => vec![target],
        sysml_query::resolved_slice::QueryOutcome::Ambiguous(targets) => targets.into_vec(),
        _ => Vec::new(),
    };
    DefinitionResult {
        locations: targets
            .into_iter()
            .map(|target| location(workspace, target.location))
            .collect(),
    }
}

pub fn resolve_symbol_target_at_position(
    workspace: &impl WorkspaceSnapshot,
    uri: &url::Url,
    position: TextPosition,
) -> Option<ResolvedSymbolTarget> {
    let outcome = workspace
        .published_model()?
        .navigation()
        .target_at(uri.as_str(), query_position(position));
    let target = match outcome {
        sysml_query::resolved_slice::QueryOutcome::Resolved(target)
        | sysml_query::resolved_slice::QueryOutcome::Recovered(target)
        | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(target) => target,
        _ => return None,
    };
    Some(ResolvedSymbolTarget {
        name: target.name.to_string(),
        definition_location: location(workspace, target.location.clone()),
        identifier_range: core_range(target.location.range),
    })
}

pub fn find_references_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
    include_declaration: bool,
) -> ReferencesResult {
    let Some(uri) = workspace.resolve_uri_for_path(path) else {
        return ReferencesResult {
            locations: Vec::new(),
        };
    };
    let Some(model) = workspace.published_model() else {
        return ReferencesResult {
            locations: Vec::new(),
        };
    };
    let target_outcome = model
        .navigation()
        .target_at(uri.as_str(), query_position(position));
    let target = match target_outcome {
        sysml_query::resolved_slice::QueryOutcome::Resolved(target)
        | sysml_query::resolved_slice::QueryOutcome::Recovered(target)
        | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(target) => target,
        _ => {
            return ReferencesResult {
                locations: Vec::new(),
            }
        }
    };
    let locations_outcome = model
        .navigation()
        .references(&target.symbol, include_declaration);
    let locations = match locations_outcome {
        sysml_query::resolved_slice::QueryOutcome::Resolved(locations)
        | sysml_query::resolved_slice::QueryOutcome::Recovered(locations)
        | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(locations) => locations,
        _ => {
            return ReferencesResult {
                locations: Vec::new(),
            }
        }
    };
    ReferencesResult {
        locations: locations
            .into_vec()
            .into_iter()
            .map(|value| location(workspace, value))
            .collect(),
    }
}
