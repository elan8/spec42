use sysml_model::{TextPosition, TextRange};

use crate::dto::SourceLocation;
use crate::workspace::WorkspaceSnapshot;

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

fn path_for_document(workspace: &impl WorkspaceSnapshot, document: &str) -> String {
    workspace
        .resolve_uri_for_path(document)
        .map(|uri| workspace.path_for_uri(&uri))
        .unwrap_or_else(|| document.to_string())
}

pub fn prepare_rename(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
) -> Option<TextRange> {
    let uri = workspace.resolve_uri_for_path(document_path)?;
    match workspace.published_model()?.edits().prepare_rename(
        uri.as_str(),
        query_position(position),
        None,
    ) {
        sysml_query::resolved_slice::RenameOutcome::Ready { range, .. } => Some(core_range(range)),
        _ => None,
    }
}

pub fn apply_rename(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
    new_name: &str,
) -> Option<Vec<crate::dto::TextEditDto>> {
    let uri = workspace.resolve_uri_for_path(document_path)?;
    let model = workspace.published_model()?;
    let sysml_query::resolved_slice::RenameOutcome::Ready { occurrences, .. } = model
        .edits()
        .prepare_rename(uri.as_str(), query_position(position), Some(new_name))
    else {
        return None;
    };
    Some(
        occurrences
            .into_vec()
            .into_iter()
            .map(|location| crate::dto::TextEditDto {
                path: path_for_document(workspace, &location.document),
                range: core_range(location.range),
                replacement: new_name.to_string(),
            })
            .collect(),
    )
}

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
    let sysml_query::resolved_slice::RenameOutcome::Ready {
        name, occurrences, ..
    } = workspace.published_model()?.edits().prepare_rename(
        uri.as_str(),
        query_position(position),
        None,
    )
    else {
        return None;
    };
    let references = occurrences
        .into_vec()
        .into_iter()
        .map(|location| SourceLocation {
            path: path_for_document(workspace, &location.document),
            range: core_range(location.range),
        })
        .collect::<Vec<_>>();
    let definition = references.first()?.clone();
    Some(RenameTarget {
        name: name.to_string(),
        definition,
        references,
    })
}
