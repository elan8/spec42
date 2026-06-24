use crate::common::text_span::{to_core_position, to_lsp_range};
use crate::workspace::{snapshot::ServerStateSnapshot, ServerState};
use language_service::WorkspaceSnapshot;
use tower_lsp::lsp_types::{Location, Position, Url};

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
