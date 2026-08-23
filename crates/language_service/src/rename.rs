use sysml_query::resolved_slice::{TextPosition, TextRange};

use crate::dto::SourceLocation;
use crate::workspace::WorkspaceSnapshot;

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
    match workspace
        .published_model()?
        .edits()
        .prepare_rename(uri.as_str(), position, None)
    {
        sysml_query::resolved_slice::RenameOutcome::Ready { range, .. } => Some(range),
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
        .prepare_rename(uri.as_str(), position, Some(new_name))
    else {
        return None;
    };
    Some(
        occurrences
            .into_vec()
            .into_iter()
            .map(|location| crate::dto::TextEditDto {
                path: path_for_document(
                    workspace,
                    model
                        .document_identity(location.document)
                        .unwrap_or_default(),
                ),
                range: location.range,
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
    let model = workspace.published_model()?;
    let sysml_query::resolved_slice::RenameOutcome::Ready {
        name, occurrences, ..
    } = model.edits().prepare_rename(uri.as_str(), position, None)
    else {
        return None;
    };
    let references = occurrences
        .into_vec()
        .into_iter()
        .map(|location| SourceLocation {
            path: path_for_document(
                workspace,
                model
                    .document_identity(location.document)
                    .unwrap_or_default(),
            ),
            range: location.range,
        })
        .collect::<Vec<_>>();
    let definition = references.first()?.clone();
    Some(RenameTarget {
        name: name.to_string(),
        definition,
        references,
    })
}
