use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::workspace::{snapshot::ServerStateSnapshot, ServerState};
use language_service::WorkspaceSnapshot;
use tower_lsp::lsp_types::{Location, Position, Range, Url};

#[derive(Debug, Clone)]
pub(crate) struct ResolvedSymbolTarget {
    #[allow(dead_code)]
    pub(crate) target_id: semantic_core::NodeId,
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) definition_location: Location,
    pub(crate) identifier_range: Range,
    pub(crate) is_renameable: bool,
}

pub(crate) fn resolved_references_at_position(
    state: &ServerState,
    uri_norm: &Url,
    pos: Position,
    include_declaration: bool,
) -> Option<Vec<Location>> {
    let snapshot = ServerStateSnapshot::new(state);
    let path = snapshot.path_for_uri(uri_norm);
    let result = language_service::find_references(
        &snapshot,
        &path,
        to_core_position(pos),
        include_declaration,
    );
    Some(
        result
            .locations
            .into_iter()
            .filter_map(|loc| {
                snapshot.resolve_uri_for_path(&loc.path).map(|uri| Location {
                    uri,
                    range: to_lsp_range(loc.range),
                })
            })
            .collect(),
    )
}

pub(crate) fn resolve_symbol_target_at_position(
    state: &ServerState,
    uri_norm: &Url,
    pos: Position,
) -> Option<ResolvedSymbolTarget> {
    let snapshot = ServerStateSnapshot::new(state);
    let target = language_service::references::resolve_symbol_target_at_position(
        &snapshot,
        uri_norm,
        to_core_position(pos),
    )?;
    let definition_uri = snapshot
        .resolve_uri_for_path(&target.definition_location.path)
        .unwrap_or_else(|| uri_norm.clone());
    Some(ResolvedSymbolTarget {
        target_id: target.target_id,
        name: target.name,
        definition_location: Location {
            uri: definition_uri,
            range: to_lsp_range(target.definition_location.range),
        },
        identifier_range: to_lsp_range(target.identifier_range),
        is_renameable: true,
    })
}
