use std::collections::BTreeSet;

use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Url,
};

#[derive(Debug, Clone, Copy)]
pub struct DiagnosticsPostprocessOptions {
    pub suppress_semantic_after_parse_error: bool,
}

impl Default for DiagnosticsPostprocessOptions {
    fn default() -> Self {
        Self {
            suppress_semantic_after_parse_error: true,
        }
    }
}

pub fn postprocess_document_diagnostics(
    uri: &Url,
    diagnostics: Vec<Diagnostic>,
    options: DiagnosticsPostprocessOptions,
) -> Vec<Diagnostic> {
    let mut diagnostics = deduplicate_diagnostics(diagnostics);
    diagnostics = collapse_duplicate_unresolved_semantic_diagnostics(diagnostics);
    if options.suppress_semantic_after_parse_error {
        diagnostics = suppress_semantic_shadowed_by_parse_errors(diagnostics);
    }
    diagnostics = attach_cascade_related_information(uri, diagnostics);
    collapse_cascade_parse_diagnostics(diagnostics)
}

pub fn diagnostics_dominated_by_cascades(diagnostics: &[Diagnostic]) -> bool {
    let cascade_codes = diagnostics.iter().filter(|d| is_cascade_code(d)).count();
    let parse_errors = diagnostics
        .iter()
        .filter(|d| d.source.as_deref() == Some("sysml") && is_parse_error_severity(d))
        .count();
    cascade_codes + parse_errors >= 3 && cascade_codes * 2 >= parse_errors.max(1)
}

fn deduplicate_diagnostics(diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut seen = BTreeSet::new();
    let mut output = Vec::new();
    for diagnostic in diagnostics {
        let code = diagnostic_code_str(&diagnostic).unwrap_or_default();
        let key = (
            diagnostic.range.start.line,
            diagnostic.range.start.character,
            diagnostic.range.end.line,
            diagnostic.range.end.character,
            format!("{:?}", diagnostic.severity),
            code,
            diagnostic.message.clone(),
        );
        if seen.insert(key) {
            output.push(diagnostic);
        }
    }
    output
}

fn collapse_duplicate_unresolved_semantic_diagnostics(
    diagnostics: Vec<Diagnostic>,
) -> Vec<Diagnostic> {
    let mut seen_unresolved = BTreeSet::new();
    let mut output = Vec::new();
    for diagnostic in diagnostics {
        if diagnostic.source.as_deref() == Some("semantic")
            && diagnostic_code_str(&diagnostic)
                .as_deref()
                .is_some_and(is_unresolved_semantic_code)
        {
            let code = diagnostic_code_str(&diagnostic).unwrap_or_default();
            let symbol = first_single_quoted_value(&diagnostic.message).unwrap_or_default();
            let key = (
                code,
                diagnostic.range.start.line,
                diagnostic.range.start.character,
                diagnostic.range.end.line,
                diagnostic.range.end.character,
                symbol,
            );
            if !seen_unresolved.insert(key) {
                continue;
            }
        }
        output.push(diagnostic);
    }
    output
}

fn is_unresolved_semantic_code(code: &str) -> bool {
    matches!(
        code,
        "unresolved_type_reference"
            | "unresolved_ref_type_reference"
            | "unresolved_import_target"
            | "unresolved_specializes_reference"
            | "unresolved_pending_relationship"
            | "unresolved_pending_expression_relationship"
            | "unresolved_allocate_source"
            | "unresolved_allocate_target"
            | "unresolved_satisfy_source"
            | "unresolved_satisfy_target"
            | "unresolved_viewpoint_conformance_target"
    )
}

fn first_single_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

fn suppress_semantic_shadowed_by_parse_errors(diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let earliest_parse_error_line = diagnostics
        .iter()
        .filter(|d| d.source.as_deref() == Some("sysml") && is_parse_error_severity(d))
        .map(|d| d.range.start.line)
        .min();
    let Some(cutoff_line) = earliest_parse_error_line else {
        return diagnostics;
    };
    diagnostics
        .into_iter()
        .filter(|d| {
            if d.source.as_deref() != Some("semantic") {
                return true;
            }
            if !is_shadowable_semantic_code(d) {
                return true;
            }
            d.range.start.line >= cutoff_line
        })
        .collect()
}

fn is_shadowable_semantic_code(diagnostic: &Diagnostic) -> bool {
    matches!(
        diagnostic_code_str(diagnostic).as_deref(),
        Some(
            "unresolved_type_reference"
                | "unresolved_import_target"
                | "unresolved_specializes_reference"
                | "unresolved_ref_type_reference"
                | "unresolved_pending_relationship"
                | "unresolved_pending_expression_relationship"
                | "unresolved_allocate_source"
                | "unresolved_allocate_target"
                | "unresolved_satisfy_source"
                | "unresolved_satisfy_target"
                | "unresolved_viewpoint_conformance_target"
        )
    )
}

fn attach_cascade_related_information(uri: &Url, diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let primary_index = diagnostics
        .iter()
        .enumerate()
        .filter(|(_, d)| d.source.as_deref() == Some("sysml") && is_parse_error_severity(d))
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
        related.push(DiagnosticRelatedInformation {
            location: Location {
                uri: uri.clone(),
                range: diagnostic.range,
            },
            message: format!(
                "{} [{}]: {}",
                diagnostic
                    .code
                    .as_ref()
                    .and_then(|c| match c {
                        NumberOrString::String(s) => Some(s.as_str()),
                        _ => None,
                    })
                    .unwrap_or("cascade"),
                diagnostic
                    .severity
                    .map(|s| format!("{s:?}"))
                    .unwrap_or_default(),
                diagnostic.message
            ),
        });
    }

    if related.is_empty() {
        return output;
    }

    related.insert(
        0,
        DiagnosticRelatedInformation {
            location: Location {
                uri: uri.clone(),
                range: primary_range,
            },
            message: format!("Primary issue: {primary_message}"),
        },
    );

    output[primary_index].related_information = Some(related);
    output
}

fn collapse_cascade_parse_diagnostics(diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut primary_parse: Option<Diagnostic> = None;
    let mut other = Vec::new();

    for diagnostic in diagnostics {
        if diagnostic.source.as_deref() == Some("sysml") && is_parse_error_severity(&diagnostic) {
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
        if diagnostic.source.as_deref() == Some("semantic")
            && matches!(
                diagnostic.severity,
                Some(DiagnosticSeverity::WARNING) | Some(DiagnosticSeverity::ERROR)
            )
            && !is_shadowable_semantic_code(&diagnostic)
        {
            other.push(diagnostic);
            continue;
        }
        if diagnostic.source.as_deref() == Some("sysml")
            && diagnostic_code_str(&diagnostic).as_deref() == Some("recovery_cascade_suppressed")
        {
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

fn diagnostic_priority(diagnostic: &Diagnostic) -> u8 {
    match diagnostic_code_str(diagnostic).as_deref() {
        Some("illegal_top_level_definition") => 0,
        Some("unexpected_keyword_in_scope")
        | Some("invalid_requirement_short_name_syntax")
        | Some("bare_feature_declaration_in_part_def") => 1,
        Some("unexpected_closing_brace") | Some("missing_closing_brace") => 2,
        Some("missing_member_name") => 3,
        Some("recovered_root_body") => 4,
        Some(code) if code.starts_with("recovered_") => 6,
        Some("missing_body_or_semicolon") | Some("missing_semicolon") => 7,
        Some("recovery_cascade_suppressed") => 8,
        _ if diagnostic.source.as_deref() == Some("sysml") => 5,
        _ => 10,
    }
}

fn is_cascade_code(diagnostic: &Diagnostic) -> bool {
    matches!(
        diagnostic_code_str(diagnostic).as_deref(),
        Some("missing_semicolon")
            | Some("missing_body_or_semicolon")
            | Some("recovery_cascade_suppressed")
    ) || diagnostic_code_str(diagnostic)
        .as_deref()
        .is_some_and(|code| code.starts_with("recovered_"))
}

fn is_parse_error_severity(diagnostic: &Diagnostic) -> bool {
    matches!(diagnostic.severity, Some(DiagnosticSeverity::ERROR) | None)
}

fn diagnostic_code_str(diagnostic: &Diagnostic) -> Option<String> {
    diagnostic.code.as_ref().map(|code| match code {
        NumberOrString::String(s) => s.clone(),
        NumberOrString::Number(n) => n.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::{Position, Range};

    fn sample_parse_error(line: u32) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position::new(line, 0),
                end: Position::new(line, 1),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String(
                "recovered_part_def_body_element".to_string(),
            )),
            code_description: None,
            source: Some("sysml".to_string()),
            message: "recovered".to_string(),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    fn sample_semantic_warning(line: u32, code: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position::new(line, 0),
                end: Position::new(line, 1),
            },
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String(code.to_string())),
            code_description: None,
            source: Some("semantic".to_string()),
            message: code.to_string(),
            related_information: None,
            tags: None,
            data: None,
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
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let diagnostics = vec![
            sample_parse_error(5),
            sample_parse_error(6),
            Diagnostic {
                range: Range {
                    start: Position::new(1, 0),
                    end: Position::new(1, 5),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String(
                    "invalid_requirement_short_name_syntax".to_string(),
                )),
                code_description: None,
                source: Some("sysml".to_string()),
                message: "bad requirement id".to_string(),
                related_information: None,
                tags: None,
                data: None,
            },
        ];
        let out = postprocess_document_diagnostics(
            &uri,
            diagnostics,
            DiagnosticsPostprocessOptions {
                suppress_semantic_after_parse_error: false,
            },
        );
        assert_eq!(out.len(), 1);
        assert_eq!(
            out[0].code.as_ref(),
            Some(&NumberOrString::String(
                "invalid_requirement_short_name_syntax".to_string()
            ))
        );
    }

    #[test]
    fn suppresses_unresolved_relationship_cascades_after_parse_error() {
        let diagnostics = vec![
            sample_parse_error(5),
            sample_semantic_warning(3, "unresolved_allocate_source"),
            sample_semantic_warning(4, "unresolved_satisfy_source"),
            sample_semantic_warning(4, "unresolved_viewpoint_conformance_target"),
            sample_semantic_warning(6, "unresolved_allocate_target"),
        ];

        let filtered = suppress_semantic_shadowed_by_parse_errors(diagnostics);
        let codes: Vec<_> = filtered.iter().filter_map(diagnostic_code_str).collect();

        assert!(!codes.contains(&"unresolved_allocate_source".to_string()));
        assert!(!codes.contains(&"unresolved_satisfy_source".to_string()));
        assert!(!codes.contains(&"unresolved_viewpoint_conformance_target".to_string()));
        assert!(codes.contains(&"unresolved_allocate_target".to_string()));
    }

    #[test]
    fn collapses_duplicate_unresolved_semantic_diagnostics_by_code_range_and_symbol() {
        let diagnostics = vec![
            sample_semantic_warning(2, "unresolved_type_reference"),
            Diagnostic {
                message: "Type reference 'MissingType' for 'vehicle' could not be resolved."
                    .to_string(),
                ..sample_semantic_warning(2, "unresolved_type_reference")
            },
            Diagnostic {
                message: "Type reference 'OtherType' for 'vehicle' could not be resolved."
                    .to_string(),
                ..sample_semantic_warning(2, "unresolved_type_reference")
            },
        ];

        let collapsed = collapse_duplicate_unresolved_semantic_diagnostics(diagnostics);
        assert_eq!(
            collapsed.len(),
            3,
            "blank-symbol fixture plus two distinct symbols should remain"
        );

        let duplicate_messages = vec![
            Diagnostic {
                message: "Type reference 'MissingType' for 'vehicle' could not be resolved."
                    .to_string(),
                ..sample_semantic_warning(4, "unresolved_type_reference")
            },
            Diagnostic {
                message:
                    "Type reference 'MissingType' for 'vehicle' could not resolve via imports."
                        .to_string(),
                ..sample_semantic_warning(4, "unresolved_type_reference")
            },
        ];
        let collapsed = collapse_duplicate_unresolved_semantic_diagnostics(duplicate_messages);
        assert_eq!(collapsed.len(), 1);
    }
}
