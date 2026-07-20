//! Diagnostic introduced/resolved diff from host validation reports.

use std::collections::{BTreeMap, BTreeSet};

use sysml_diagnostics::{DiagnosticSeverity, SemanticDiagnostic};

use crate::snapshot::HostValidationReport;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct HostDiagnosticIdentity {
    pub uri: String,
    pub code: String,
    pub severity: String,
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
) -> HostDiagnosticComparison {
    let previous_by_uri = diagnostics_by_uri(previous);
    let next_by_uri = diagnostics_by_uri(next);

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

    HostDiagnosticComparison { by_document }
}

fn diagnostics_by_uri(
    report: &HostValidationReport,
) -> BTreeMap<String, BTreeSet<HostDiagnosticIdentity>> {
    let mut by_uri = BTreeMap::new();
    for document in &report.documents {
        let mut identities = BTreeSet::new();
        for diagnostic in &document.diagnostics {
            identities.insert(diagnostic_identity(diagnostic));
        }
        by_uri.insert(document.uri.clone(), identities);
    }
    by_uri
}

fn diagnostic_identity(diagnostic: &SemanticDiagnostic) -> HostDiagnosticIdentity {
    HostDiagnosticIdentity {
        uri: diagnostic.uri.to_string(),
        code: diagnostic.code.clone(),
        severity: severity_label(diagnostic.severity).to_string(),
        message: diagnostic.message.clone(),
    }
}

fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Information => "information",
        DiagnosticSeverity::Hint => "hint",
    }
}
