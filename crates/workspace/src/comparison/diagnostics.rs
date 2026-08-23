//! Diagnostic introduced/resolved diff from host validation reports.

use std::collections::{BTreeMap, BTreeSet};

use sysml_diagnostics::{DiagnosticRelatedInfo, SemanticDiagnostic};
use sysml_query::resolved_slice::{TextPosition, TextRange};

use crate::error::{WorkspaceError, WorkspaceResult};
use crate::snapshot::HostValidationReport;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostDiagnosticIdentity {
    pub uri: String,
    pub code: String,
    pub severity: String,
    pub message: String,
    /// Primary source evidence. `None` represents an older persisted report
    /// whose schema intentionally omitted this field.
    #[serde(default)]
    pub range: Option<TextRange>,
    /// The diagnostic producer is semantic provenance, distinct from its
    /// human-readable message.
    #[serde(default)]
    pub source: String,
    /// Related diagnostic provenance is part of the typed diagnostic contract.
    #[serde(default)]
    pub related_information: Vec<HostDiagnosticRelatedInformation>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostDiagnosticRelatedInformation {
    pub uri: String,
    pub range: TextRange,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostDocumentDiagnosticComparison {
    pub introduced: Vec<HostDiagnosticIdentity>,
    pub resolved: Vec<HostDiagnosticIdentity>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostDiagnosticComparison {
    pub by_document: BTreeMap<String, HostDocumentDiagnosticComparison>,
}

pub(crate) fn compare_diagnostics(
    previous: &HostValidationReport,
    next: &HostValidationReport,
) -> WorkspaceResult<HostDiagnosticComparison> {
    let previous_by_uri = diagnostics_by_uri(previous)?;
    let next_by_uri = diagnostics_by_uri(next)?;

    let mut all_uris: BTreeMap<String, ()> = BTreeMap::new();
    for uri in previous_by_uri.keys() {
        all_uris.insert(uri.clone(), ());
    }
    for uri in next_by_uri.keys() {
        all_uris.insert(uri.clone(), ());
    }

    let mut by_document = BTreeMap::new();
    for uri in all_uris.keys() {
        let previous_set = previous_by_uri.get(uri).cloned().unwrap_or_default();
        let next_set = next_by_uri.get(uri).cloned().unwrap_or_default();

        let introduced: Vec<_> = next_set.difference(&previous_set).cloned().collect();
        let resolved: Vec<_> = previous_set.difference(&next_set).cloned().collect();

        if !introduced.is_empty() || !resolved.is_empty() {
            by_document.insert(
                uri.clone(),
                HostDocumentDiagnosticComparison {
                    introduced,
                    resolved,
                },
            );
        }
    }

    Ok(HostDiagnosticComparison { by_document })
}

fn diagnostics_by_uri(
    report: &HostValidationReport,
) -> WorkspaceResult<BTreeMap<String, BTreeSet<HostDiagnosticIdentity>>> {
    let mut by_uri = BTreeMap::new();
    for document in &report.documents {
        let mut identities = BTreeSet::new();
        for diagnostic in &document.diagnostics {
            let identity = diagnostic_identity(diagnostic);
            if !identities.insert(identity.clone()) {
                return Err(WorkspaceError::duplicate_comparison_identity(
                    "diagnostic",
                    format!(
                        "{}:{}:{}:{}",
                        identity.uri, identity.code, identity.severity, identity.message
                    ),
                ));
            }
        }
        if by_uri.insert(document.uri.clone(), identities).is_some() {
            return Err(WorkspaceError::duplicate_comparison_identity(
                "diagnostic_document",
                &document.uri,
            ));
        }
    }
    Ok(by_uri)
}

/// The comparison DTO's own range type, copied coordinate for coordinate.
///
/// A published diagnostic carries the resolution owner's range; the comparison contract predates
/// it and carries the graph's. The two are the same two positions, and this is the boundary that
/// says so once rather than in every field.
fn comparison_range(range: sysml_query::resolved_slice::TextRange) -> TextRange {
    TextRange::new(
        TextPosition::new(range.start.line, range.start.character),
        TextPosition::new(range.end.line, range.end.character),
    )
}

fn diagnostic_identity(diagnostic: &SemanticDiagnostic) -> HostDiagnosticIdentity {
    HostDiagnosticIdentity {
        uri: diagnostic.uri.to_string(),
        code: diagnostic.code.clone(),
        severity: sysml_diagnostics::severity_label(diagnostic.severity).to_string(),
        message: diagnostic.message.clone(),
        range: Some(comparison_range(diagnostic.range)),
        source: diagnostic.source.clone(),
        related_information: canonical_related_information(&diagnostic.related_information),
    }
}

fn canonical_related_information(
    related: &[DiagnosticRelatedInfo],
) -> Vec<HostDiagnosticRelatedInformation> {
    let mut result = related
        .iter()
        .map(|information| HostDiagnosticRelatedInformation {
            uri: information.uri.to_string(),
            range: comparison_range(information.range),
            message: information.message.clone(),
        })
        .collect::<Vec<_>>();
    result.sort_by(compare_related_information);
    result
}

fn compare_related_information(
    left: &HostDiagnosticRelatedInformation,
    right: &HostDiagnosticRelatedInformation,
) -> std::cmp::Ordering {
    left.uri
        .cmp(&right.uri)
        .then_with(|| left.range.start.line.cmp(&right.range.start.line))
        .then_with(|| left.range.start.character.cmp(&right.range.start.character))
        .then_with(|| left.range.end.line.cmp(&right.range.end.line))
        .then_with(|| left.range.end.character.cmp(&right.range.end.character))
        .then_with(|| left.message.cmp(&right.message))
}

impl PartialOrd for HostDiagnosticIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HostDiagnosticIdentity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uri
            .cmp(&other.uri)
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.severity.cmp(&other.severity))
            .then_with(|| self.message.cmp(&other.message))
            .then_with(|| compare_ranges(&self.range, &other.range))
            .then_with(|| self.source.cmp(&other.source))
            .then_with(|| {
                compare_related_lists(&self.related_information, &other.related_information)
            })
    }
}

fn compare_ranges(left: &Option<TextRange>, right: &Option<TextRange>) -> std::cmp::Ordering {
    match (left, right) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(left), Some(right)) => left
            .start
            .line
            .cmp(&right.start.line)
            .then_with(|| left.start.character.cmp(&right.start.character))
            .then_with(|| left.end.line.cmp(&right.end.line))
            .then_with(|| left.end.character.cmp(&right.end.character)),
    }
}

fn compare_related_lists(
    left: &[HostDiagnosticRelatedInformation],
    right: &[HostDiagnosticRelatedInformation],
) -> std::cmp::Ordering {
    for (left, right) in left.iter().zip(right) {
        let ordering = compare_related_information(left, right);
        if ordering != std::cmp::Ordering::Equal {
            return ordering;
        }
    }
    left.len().cmp(&right.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{HostValidatedDocument, HostValidationReport, HostValidationSummary};
    use sysml_diagnostics::DiagnosticSeverity;
    use sysml_query::resolved_slice::{TextPosition, TextRange};
    use url::Url;

    fn report(diagnostic: SemanticDiagnostic) -> HostValidationReport {
        HostValidationReport {
            workspace_root: None,
            resolved_library_paths: Vec::new(),
            documents: vec![HostValidatedDocument {
                uri: diagnostic.uri.to_string(),
                diagnostics: vec![diagnostic],
            }],
            summary: HostValidationSummary::default(),
        }
    }

    fn diagnostic(source: &str) -> SemanticDiagnostic {
        SemanticDiagnostic {
            uri: Url::parse("file:///demo.sysml").expect("uri"),
            range: TextRange {
                start: TextPosition {
                    line: 1,
                    character: 0,
                },
                end: TextPosition {
                    line: 1,
                    character: 1,
                },
            },
            severity: DiagnosticSeverity::Warning,
            source: source.to_string(),
            code: "semantic.example".to_string(),
            message: "example".to_string(),
            related_information: Vec::new(),
        }
    }

    #[test]
    fn diagnostic_provenance_and_primary_range_are_compared() {
        let previous = report(diagnostic("parser"));
        let mut shifted = diagnostic("parser");
        shifted.range = TextRange {
            start: TextPosition {
                line: 9,
                character: 0,
            },
            end: TextPosition {
                line: 9,
                character: 1,
            },
        };
        let shifted_comparison = compare_diagnostics(&previous, &report(shifted)).expect("compare");
        assert_eq!(
            shifted_comparison.by_document["file:///demo.sysml"]
                .introduced
                .len(),
            1
        );
        assert_eq!(
            shifted_comparison.by_document["file:///demo.sysml"]
                .resolved
                .len(),
            1
        );

        let comparison =
            compare_diagnostics(&previous, &report(diagnostic("semantic"))).expect("compare");
        let document = comparison
            .by_document
            .get("file:///demo.sysml")
            .expect("changed document");
        assert_eq!(document.introduced.len(), 1);
        assert_eq!(document.resolved.len(), 1);
    }

    #[test]
    fn rejects_duplicate_diagnostic_identity() {
        let duplicate = diagnostic("semantic");
        let error = compare_diagnostics(
            &report(duplicate.clone()),
            &HostValidationReport {
                workspace_root: None,
                resolved_library_paths: Vec::new(),
                documents: vec![HostValidatedDocument {
                    uri: duplicate.uri.to_string(),
                    diagnostics: vec![duplicate.clone(), duplicate],
                }],
                summary: HostValidationSummary::default(),
            },
        )
        .expect_err("duplicate diagnostics must not be collapsed");
        assert_eq!(error.code(), "duplicate_comparison_identity");
    }
}
