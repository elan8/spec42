//! Reporting policy over one document's settled diagnostics: dedup, shadowing, cascade collapse.
//!
//! Everything here is a decision about *what a reader is shown*, taken over values the publication
//! already settled. It removes exact duplicates, suppresses semantic answers a parse error already
//! explains, links the cascade sites to the primary parse error, and collapses a recovery cascade
//! down to the one diagnostic an author should fix first.
//!
//! It decides nothing semantic: no diagnostic's code, severity, range or message is changed, no
//! message is read as an input, and nothing suppressed here is replaced by an invented fact. A host
//! that asks for no policy receives the publication's own answer unaltered.

use std::collections::BTreeSet;

use crate::types::{
    DiagnosticRelatedInfo, DiagnosticSeverity, SemanticDiagnostic, PARSER_SOURCE, SEMANTIC_SOURCE,
};

/// Which parts of the collapse policy a host asked for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PostprocessPolicy {
    /// Drop the semantic answers that a parse error earlier in the document already explains.
    pub suppress_semantic_after_parse_error: bool,
}

/// Apply the reporting policy to one document's diagnostics.
///
/// `diagnostics` must all belong to the same document; the cascade links are addressed by the URI
/// each diagnostic already carries.
pub fn postprocess_document_diagnostics(
    diagnostics: Vec<SemanticDiagnostic>,
    policy: PostprocessPolicy,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = deduplicate_diagnostics(diagnostics);
    if policy.suppress_semantic_after_parse_error {
        diagnostics = suppress_semantic_shadowed_by_parse_errors(diagnostics);
    }
    diagnostics = attach_cascade_related_information(diagnostics);
    collapse_cascade_parse_diagnostics(diagnostics)
}

/// Whether a document's report is mostly recovery cascade rather than distinct problems.
///
/// A reporting question, not a semantic one: it decides whether a reader is advised to fix the
/// earliest error first, and changes no diagnostic.
pub fn diagnostics_dominated_by_cascades(diagnostics: &[SemanticDiagnostic]) -> bool {
    let cascade_codes = diagnostics.iter().filter(|d| is_cascade_code(d)).count();
    let parse_errors = diagnostics.iter().filter(|d| is_parse_error(d)).count();
    cascade_codes + parse_errors >= 3 && cascade_codes * 2 >= parse_errors.max(1)
}

fn deduplicate_diagnostics(diagnostics: Vec<SemanticDiagnostic>) -> Vec<SemanticDiagnostic> {
    let mut seen = BTreeSet::new();
    let mut output = Vec::new();
    for diagnostic in diagnostics {
        let key = (
            diagnostic.range.start.line,
            diagnostic.range.start.character,
            diagnostic.range.end.line,
            diagnostic.range.end.character,
            format!("{:?}", diagnostic.severity),
            diagnostic.code.clone(),
            diagnostic.message.clone(),
        );
        if seen.insert(key) {
            output.push(diagnostic);
        }
    }
    output
}

fn suppress_semantic_shadowed_by_parse_errors(
    diagnostics: Vec<SemanticDiagnostic>,
) -> Vec<SemanticDiagnostic> {
    let earliest_parse_error_line = diagnostics
        .iter()
        .filter(|d| is_parse_error(d))
        .map(|d| d.range.start.line)
        .min();
    let Some(cutoff_line) = earliest_parse_error_line else {
        return diagnostics;
    };
    diagnostics
        .into_iter()
        .filter(|d| {
            if d.source != SEMANTIC_SOURCE {
                return true;
            }
            if !is_shadowable_semantic_code(d) {
                return true;
            }
            d.range.start.line >= cutoff_line
        })
        .collect()
}

fn is_shadowable_semantic_code(diagnostic: &SemanticDiagnostic) -> bool {
    matches!(
        diagnostic.code.as_str(),
        "unresolved_type_reference"
            | "unresolved_import_target"
            | "unresolved_specializes_reference"
            | "unresolved_reference"
            | "ambiguous_reference"
            | "ambiguous_import_target"
    )
}

fn attach_cascade_related_information(
    diagnostics: Vec<SemanticDiagnostic>,
) -> Vec<SemanticDiagnostic> {
    let primary_index = diagnostics
        .iter()
        .enumerate()
        .filter(|(_, d)| is_parse_error(d))
        .min_by_key(|(_, d)| {
            (
                diagnostic_priority(d),
                d.range.start.line,
                d.range.start.character,
            )
        })
        .map(|(idx, _)| idx);

    let Some(primary_index) = primary_index else {
        return diagnostics;
    };

    let mut output = diagnostics;
    let primary_uri = output[primary_index].uri.clone();
    let primary_range = output[primary_index].range;
    let primary_message = output[primary_index].message.clone();
    let mut related = Vec::new();

    for (idx, diagnostic) in output.iter().enumerate() {
        if idx == primary_index {
            continue;
        }
        if !is_cascade_code(diagnostic) {
            continue;
        }
        related.push(DiagnosticRelatedInfo {
            uri: diagnostic.uri.clone(),
            range: diagnostic.range,
            message: format!(
                "{} [{}]: {}",
                diagnostic.code,
                severity_debug(diagnostic.severity),
                diagnostic.message
            ),
        });
    }

    if related.is_empty() {
        return output;
    }

    related.insert(
        0,
        DiagnosticRelatedInfo {
            uri: primary_uri,
            range: primary_range,
            message: format!("Primary issue: {primary_message}"),
        },
    );

    output[primary_index].related_information = related;
    output
}

fn collapse_cascade_parse_diagnostics(
    diagnostics: Vec<SemanticDiagnostic>,
) -> Vec<SemanticDiagnostic> {
    let mut primary_parse: Option<SemanticDiagnostic> = None;
    let mut other = Vec::new();

    for diagnostic in diagnostics {
        if is_parse_error(&diagnostic) {
            if primary_parse.as_ref().is_none_or(|existing| {
                diagnostic_priority(&diagnostic) < diagnostic_priority(existing)
            }) {
                primary_parse = Some(diagnostic);
            }
            continue;
        }
        if is_cascade_code(&diagnostic) {
            continue;
        }
        if diagnostic.source == SEMANTIC_SOURCE
            && matches!(
                diagnostic.severity,
                DiagnosticSeverity::Warning | DiagnosticSeverity::Error
            )
            && !is_shadowable_semantic_code(&diagnostic)
        {
            other.push(diagnostic);
            continue;
        }
        if diagnostic.source == PARSER_SOURCE && diagnostic.code == "recovery_cascade_suppressed" {
            if let Some(ref mut primary) = primary_parse {
                primary.related_information = diagnostic.related_information.clone();
            }
            continue;
        }
        if diagnostic_priority(&diagnostic) >= 4 {
            other.push(diagnostic);
        }
    }

    let mut output = Vec::new();
    if let Some(primary) = primary_parse {
        output.push(primary);
    }
    other.sort_by_key(|d| (d.range.start.line, d.range.start.character));
    output.extend(other);
    output
}

/// Which parse diagnostic an author should be shown first when several describe one failure.
///
/// A presentation order over stable codes the parser owns; it changes no diagnostic's meaning.
fn diagnostic_priority(diagnostic: &SemanticDiagnostic) -> u8 {
    match diagnostic.code.as_str() {
        "illegal_top_level_definition" => 0,
        "unexpected_keyword_in_scope"
        | "invalid_requirement_short_name_syntax"
        | "bare_feature_declaration_in_part_def" => 1,
        "unexpected_closing_brace" | "missing_closing_brace" => 2,
        "missing_member_name" => 3,
        "recovered_root_body" => 4,
        code if code.starts_with(RECOVERY_CODE_PREFIX) => 6,
        "missing_body_or_semicolon" | "missing_semicolon" => 7,
        "recovery_cascade_suppressed" => 8,
        _ if diagnostic.source == PARSER_SOURCE => 5,
        _ => 10,
    }
}

/// The prefix the parser gives every recovery-produced code.
const RECOVERY_CODE_PREFIX: &str = "recovered_";

fn is_cascade_code(diagnostic: &SemanticDiagnostic) -> bool {
    matches!(
        diagnostic.code.as_str(),
        "missing_semicolon" | "missing_body_or_semicolon" | "recovery_cascade_suppressed"
    ) || diagnostic.code.starts_with(RECOVERY_CODE_PREFIX)
}

fn is_parse_error(diagnostic: &SemanticDiagnostic) -> bool {
    diagnostic.source == PARSER_SOURCE && diagnostic.severity == DiagnosticSeverity::Error
}

fn severity_debug(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "ERROR",
        DiagnosticSeverity::Warning => "WARNING",
        DiagnosticSeverity::Information => "INFORMATION",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_query::resolved_slice::{TextPosition, TextRange};
    use url::Url;

    fn uri() -> Url {
        Url::parse("file:///test.sysml").expect("uri")
    }

    fn range(line: u32) -> TextRange {
        TextRange {
            start: TextPosition { line, character: 0 },
            end: TextPosition { line, character: 1 },
        }
    }

    fn sample_parse_error(line: u32) -> SemanticDiagnostic {
        SemanticDiagnostic {
            uri: uri(),
            range: range(line),
            severity: DiagnosticSeverity::Error,
            source: PARSER_SOURCE.to_string(),
            code: "recovered_part_def_body_element".to_string(),
            message: "recovered".to_string(),
            related_information: Vec::new(),
        }
    }

    fn sample_semantic_warning(line: u32, code: &str) -> SemanticDiagnostic {
        SemanticDiagnostic {
            uri: uri(),
            range: range(line),
            severity: DiagnosticSeverity::Warning,
            source: SEMANTIC_SOURCE.to_string(),
            code: code.to_string(),
            message: code.to_string(),
            related_information: Vec::new(),
        }
    }

    #[test]
    fn dedup_removes_identical_diagnostics() {
        let d = sample_parse_error(2);
        let out = deduplicate_diagnostics(vec![d.clone(), d]);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn collapse_keeps_highest_priority_parse_error() {
        let mut higher_priority = sample_parse_error(1);
        higher_priority.code = "invalid_requirement_short_name_syntax".to_string();
        higher_priority.message = "bad requirement id".to_string();

        let out = postprocess_document_diagnostics(
            vec![
                sample_parse_error(5),
                sample_parse_error(6),
                higher_priority,
            ],
            PostprocessPolicy {
                suppress_semantic_after_parse_error: false,
            },
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].code, "invalid_requirement_short_name_syntax");
    }

    #[test]
    fn suppresses_unresolved_relationship_cascades_after_parse_error() {
        let diagnostics = vec![
            sample_parse_error(5),
            sample_semantic_warning(3, "unresolved_reference"),
            sample_semantic_warning(4, "unresolved_type_reference"),
            sample_semantic_warning(4, "ambiguous_reference"),
            sample_semantic_warning(6, "unresolved_reference"),
        ];

        let filtered = suppress_semantic_shadowed_by_parse_errors(diagnostics);
        let codes: Vec<_> = filtered.iter().map(|d| d.code.clone()).collect();

        assert_eq!(
            codes,
            vec![
                "recovered_part_def_body_element".to_string(),
                "unresolved_reference".to_string()
            ],
            "only the unresolved codes above the first parse error are suppressed"
        );
    }
}
