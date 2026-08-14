//! Canonical ordering for diagnostics and their related locations.

use std::cmp::Ordering;

use crate::{DiagnosticRelatedInfo, DiagnosticSeverity, SemanticDiagnostic};

/// Canonicalizes diagnostic order before a diagnostic result is published.
pub fn canonicalize_diagnostics(diagnostics: &mut [SemanticDiagnostic]) {
    diagnostics.sort_by(compare_diagnostics);
}

/// Returns related locations in their canonical order.
pub(crate) fn canonical_related_information(
    related_information: &[DiagnosticRelatedInfo],
) -> Vec<&DiagnosticRelatedInfo> {
    let mut related = related_information.iter().collect::<Vec<_>>();
    related.sort_by(|left, right| compare_related_information_item(left, right));
    related
}

fn compare_diagnostics(left: &SemanticDiagnostic, right: &SemanticDiagnostic) -> Ordering {
    (
        left.uri.as_str(),
        left.range.start.line,
        left.range.start.character,
        left.range.end.line,
        left.range.end.character,
        severity_rank(left.severity),
        left.source.as_str(),
        left.code.as_str(),
    )
        .cmp(&(
            right.uri.as_str(),
            right.range.start.line,
            right.range.start.character,
            right.range.end.line,
            right.range.end.character,
            severity_rank(right.severity),
            right.source.as_str(),
            right.code.as_str(),
        ))
        .then_with(|| compare_related_information(left, right))
}

fn compare_related_information(left: &SemanticDiagnostic, right: &SemanticDiagnostic) -> Ordering {
    canonical_related_information(&left.related_information)
        .iter()
        .map(|related| related_sort_key(related))
        .cmp(
            canonical_related_information(&right.related_information)
                .iter()
                .map(|related| related_sort_key(related)),
        )
}

fn compare_related_information_item(
    left: &DiagnosticRelatedInfo,
    right: &DiagnosticRelatedInfo,
) -> Ordering {
    related_sort_key(left).cmp(&related_sort_key(right))
}

fn related_sort_key(related: &DiagnosticRelatedInfo) -> (&str, u32, u32, u32, u32) {
    (
        related.uri.as_str(),
        related.range.start.line,
        related.range.start.character,
        related.range.end.line,
        related.range.end.character,
    )
}

fn severity_rank(severity: DiagnosticSeverity) -> u8 {
    match severity {
        DiagnosticSeverity::Error => 0,
        DiagnosticSeverity::Warning => 1,
        DiagnosticSeverity::Information => 2,
        DiagnosticSeverity::Hint => 3,
    }
}
