use sysml_model::TextPosition;

use crate::dto::SourceLocation;
use crate::references::{find_references_at_position, resolve_symbol_target_at_position};
use crate::workspace::WorkspaceSnapshot;

/// Returns the identifier range that would be renamed at the cursor.
pub fn prepare_rename(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
) -> Option<sysml_model::TextRange> {
    let uri = workspace.resolve_uri_for_path(document_path)?;
    resolve_symbol_target_at_position(workspace, &uri, position)
        .map(|target| target.identifier_range)
}

/// Produces neutral text edits to rename a symbol and all references.
pub fn apply_rename(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
    new_name: &str,
) -> Vec<crate::dto::TextEditDto> {
    let references = find_references_at_position(workspace, document_path, position, true);
    references
        .locations
        .into_iter()
        .map(|location| crate::dto::TextEditDto {
            path: location.path,
            range: location.range,
            replacement: new_name.to_string(),
        })
        .collect()
}

/// Rename target metadata for hosts that need declaration + reference sites.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameTarget {
    pub name: String,
    pub definition: SourceLocation,
    pub references: Vec<SourceLocation>,
}

pub fn rename_target(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
) -> Option<RenameTarget> {
    let uri = workspace.resolve_uri_for_path(document_path)?;
    let target = resolve_symbol_target_at_position(workspace, &uri, position)?;
    let references = find_references_at_position(workspace, document_path, position, true);
    Some(RenameTarget {
        name: target.name,
        definition: target.definition_location,
        references: references.locations,
    })
}
