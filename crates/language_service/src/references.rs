use sysml_query::resolved_slice::{TextPosition, TextRange};

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

#[derive(Debug, Clone)]
pub struct ResolvedSymbolTarget {
    pub name: String,
    pub definition_location: SourceLocation,
    pub identifier_range: TextRange,
}

/// A published location, projected onto the host's path vocabulary.
///
/// The location names its document by handle, so the identity is materialised here from the
/// model that answered the query -- once per location a host is about to show a person.
fn location(
    workspace: &impl WorkspaceSnapshot,
    model: &sysml_query::resolved_slice::PublishedModel,
    value: sysml_query::resolved_slice::SourceLocation,
) -> SourceLocation {
    let identity = model.document_identity(value.document).unwrap_or_default();
    let path = workspace
        .resolve_uri_for_path(identity)
        .map(|uri| workspace.path_for_uri(&uri))
        .unwrap_or_else(|| identity.to_owned());
    SourceLocation {
        path,
        range: value.range,
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
            semantic_status: workspace.semantic_status(),
        };
    };
    let Some(model) = workspace.published_model() else {
        return DefinitionResult {
            locations: Vec::new(),
            semantic_status: workspace.semantic_status(),
        };
    };
    let outcome = model.navigation().target_at(uri.as_str(), position);
    let targets: Vec<_> = match outcome.answer {
        sysml_query::resolved_slice::QueryAnswer::Resolved(target) => vec![target],
        sysml_query::resolved_slice::QueryAnswer::Ambiguous(targets) => targets.into_vec(),
        _ => Vec::new(),
    };
    DefinitionResult {
        locations: targets
            .into_iter()
            .map(|target| location(workspace, model, target.location))
            .collect(),
        semantic_status: workspace.semantic_status(),
    }
}

pub fn resolve_symbol_target_at_position(
    workspace: &impl WorkspaceSnapshot,
    uri: &url::Url,
    position: TextPosition,
) -> Option<ResolvedSymbolTarget> {
    let model = workspace.published_model()?;
    let outcome = model.navigation().target_at(uri.as_str(), position);
    let target = match outcome.answer {
        sysml_query::resolved_slice::QueryAnswer::Resolved(target) => target,
        _ => return None,
    };
    Some(ResolvedSymbolTarget {
        name: model
            .symbol_name(target.symbol)
            .unwrap_or_default()
            .to_owned(),
        definition_location: location(workspace, model, target.location),
        identifier_range: target.location.range,
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
            semantic_status: workspace.semantic_status(),
        };
    };
    let Some(model) = workspace.published_model() else {
        return ReferencesResult {
            locations: Vec::new(),
            semantic_status: workspace.semantic_status(),
        };
    };
    let target_outcome = model.navigation().target_at(uri.as_str(), position);
    let target = match target_outcome.answer {
        sysml_query::resolved_slice::QueryAnswer::Resolved(target) => target,
        _ => {
            return ReferencesResult {
                locations: Vec::new(),
                semantic_status: workspace.semantic_status(),
            }
        }
    };
    let locations_outcome = model
        .navigation()
        .references(target.symbol, include_declaration);
    let locations = match locations_outcome.answer {
        sysml_query::resolved_slice::QueryAnswer::Resolved(locations) => locations,
        _ => {
            return ReferencesResult {
                locations: Vec::new(),
                semantic_status: workspace.semantic_status(),
            }
        }
    };
    ReferencesResult {
        locations: locations
            .into_vec()
            .into_iter()
            .map(|value| location(workspace, model, value))
            .collect(),
        semantic_status: workspace.semantic_status(),
    }
}
