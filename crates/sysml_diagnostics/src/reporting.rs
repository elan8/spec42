//! Projecting one publication's diagnostics for a host, under an explicit reporting policy.

use sysml_query::resolved_slice::{
    Diagnostic, DiagnosticOrigin, DiagnosticSeverity as PublishedSeverity, PublishedModel,
};
use url::Url;

use crate::types::{
    DiagnosticRelatedInfo, DiagnosticSeverity, SemanticDiagnostic, PARSER_SOURCE, SEMANTIC_SOURCE,
};

/// What a host chooses to report, over diagnostics it does not decide.
///
/// The one policy Spec42's hosts have: batch validation may report *only* what the parser rejected
/// when a document does not parse, because a document that failed to parse produces semantic
/// answers about a model the author did not write. Interactive editing reports everything, because
/// an editor's user is mid-keystroke and the semantic answers are still the best available.
///
/// This is a filter over settled values and nothing else. It never suppresses an unresolved state
/// in favour of a resolved one, never re-runs a check, and never changes a severity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReportingPolicy {
    /// Report only parser-owned errors for a document the parser rejected.
    pub parse_errors_only_when_source_is_invalid: bool,
}

impl ReportingPolicy {
    /// The policy batch validation uses.
    pub fn strict(strict: bool) -> Self {
        Self {
            parse_errors_only_when_source_is_invalid: strict,
        }
    }
}

/// The diagnostics one document reports, in the publication's canonical order.
///
/// `uri` is both the document identity to query and the identity the host addresses results by, so
/// a caller cannot ask about one document and receive another's.
pub fn document_diagnostics(
    model: &PublishedModel,
    uri: &Url,
    policy: ReportingPolicy,
) -> Vec<SemanticDiagnostic> {
    let published = model.diagnostics().for_document(uri.as_str());
    let has_parse_error = published.iter().any(is_parse_error);
    published
        .iter()
        .filter(|diagnostic| {
            !(policy.parse_errors_only_when_source_is_invalid && has_parse_error)
                || is_parse_error(diagnostic)
        })
        .map(|diagnostic| project(diagnostic, uri))
        .collect()
}

fn is_parse_error(diagnostic: &Diagnostic) -> bool {
    diagnostic.origin() == DiagnosticOrigin::Parser && diagnostic.severity() == PublishedSeverity::Error
}

fn project(diagnostic: &Diagnostic, uri: &Url) -> SemanticDiagnostic {
    SemanticDiagnostic {
        uri: uri.clone(),
        range: diagnostic.location().range(),
        severity: severity(diagnostic.severity()),
        source: match diagnostic.origin() {
            DiagnosticOrigin::Parser => PARSER_SOURCE,
            DiagnosticOrigin::Semantic => SEMANTIC_SOURCE,
        }
        .to_string(),
        code: diagnostic.code().as_str().to_string(),
        message: diagnostic.message().to_string(),
        related_information: diagnostic
            .related()
            .filter_map(|related| {
                // A related site the host cannot address is dropped rather than pointed at the
                // reporting document: the diagnostic itself is still correct without it, and
                // substituting a URI would send a reader to the wrong file.
                Some(DiagnosticRelatedInfo {
                    uri: Url::parse(related.location().document()).ok()?,
                    range: related.location().range(),
                    message: related.message().to_string(),
                })
            })
            .collect(),
    }
}

fn severity(severity: PublishedSeverity) -> DiagnosticSeverity {
    match severity {
        PublishedSeverity::Error => DiagnosticSeverity::Error,
        PublishedSeverity::Warning => DiagnosticSeverity::Warning,
        PublishedSeverity::Information => DiagnosticSeverity::Information,
    }
}

/// The one label a host prints for a severity.
///
/// Reporting policy, and therefore this crate's to decide: the CLI text report, the JUnit report,
/// and the comparison harness all name the same three severities and must name them identically.
pub fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Information => "info",
    }
}
