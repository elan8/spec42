use std::collections::BTreeSet;
use std::path::Path;

use lsp_server::{ValidatedDocument, ValidationReport, ValidationSummary};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};

use crate::cli::OutputFormat;

pub fn emit_validation_report(
    report: &ValidationReport,
    format: OutputFormat,
) -> Result<(), String> {
    match format {
        OutputFormat::Text => {
            print_text_report(report);
            Ok(())
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report)
                    .map_err(|err| format!("Failed to serialize report as JSON: {err}"))?
            );
            Ok(())
        }
        OutputFormat::Sarif => {
            println!(
                "{}",
                serde_json::to_string_pretty(&sarif_report(report))
                    .map_err(|err| format!("Failed to serialize SARIF report: {err}"))?
            );
            Ok(())
        }
        OutputFormat::Junit => {
            println!("{}", junit_report(report));
            Ok(())
        }
    }
}

pub fn apply_baseline(
    report: &ValidationReport,
    baseline_path: &Path,
) -> Result<ValidationReport, String> {
    let raw = std::fs::read_to_string(baseline_path)
        .map_err(|err| format!("Failed to read baseline {}: {err}", baseline_path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&raw).map_err(|err| {
        format!(
            "Failed to parse baseline {} as JSON: {err}",
            baseline_path.display()
        )
    })?;
    let baseline = collect_baseline_signatures(&value);

    let documents = report
        .documents
        .iter()
        .map(|document| ValidatedDocument {
            uri: document.uri.clone(),
            diagnostics: document
                .diagnostics
                .iter()
                .filter(|diagnostic| {
                    !baseline.contains(&diagnostic_signature(&document.uri, diagnostic))
                })
                .cloned()
                .collect(),
        })
        .collect::<Vec<_>>();

    Ok(ValidationReport {
        workspace_root: report.workspace_root.clone(),
        resolved_library_paths: report.resolved_library_paths.clone(),
        summary: summarize(&documents),
        advice: report.advice.clone(),
        documents,
    })
}

fn collect_baseline_signatures(value: &serde_json::Value) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Some(documents) = value.get("documents").and_then(|value| value.as_array()) else {
        return out;
    };
    for document in documents {
        let Some(uri) = document.get("uri").and_then(|value| value.as_str()) else {
            continue;
        };
        let Some(diagnostics) = document
            .get("diagnostics")
            .and_then(|value| value.as_array())
        else {
            continue;
        };
        for diagnostic in diagnostics {
            if let Some(signature) = diagnostic_signature_from_json(uri, diagnostic) {
                out.insert(signature);
            }
        }
    }
    out
}

fn diagnostic_signature_from_json(uri: &str, diagnostic: &serde_json::Value) -> Option<String> {
    let range = diagnostic.get("range")?;
    let start = range.get("start")?;
    let end = range.get("end")?;
    let code = diagnostic
        .get("code")
        .map(code_value_label)
        .unwrap_or_default();
    Some(format!(
        "{}|{}:{}-{}:{}|{}|{}|{}",
        uri,
        start.get("line")?.as_u64()?,
        start.get("character")?.as_u64()?,
        end.get("line")?.as_u64()?,
        end.get("character")?.as_u64()?,
        diagnostic
            .get("severity")
            .and_then(|value| value.as_u64())
            .unwrap_or(1),
        code,
        diagnostic.get("message")?.as_str()?
    ))
}

fn code_value_label(value: &serde_json::Value) -> String {
    if let Some(text) = value.as_str() {
        return text.to_string();
    }
    if let Some(number) = value.as_i64() {
        return number.to_string();
    }
    value
        .get("String")
        .and_then(|value| value.as_str())
        .or_else(|| value.get("string").and_then(|value| value.as_str()))
        .map(ToString::to_string)
        .unwrap_or_default()
}

fn diagnostic_signature(uri: &str, diagnostic: &Diagnostic) -> String {
    format!(
        "{}|{}:{}-{}:{}|{}|{}|{}",
        uri,
        diagnostic.range.start.line,
        diagnostic.range.start.character,
        diagnostic.range.end.line,
        diagnostic.range.end.character,
        diagnostic.severity.map(severity_number).unwrap_or(1),
        diagnostic
            .code
            .as_ref()
            .map(number_or_string_label)
            .unwrap_or_default(),
        diagnostic.message
    )
}

fn summarize(documents: &[ValidatedDocument]) -> ValidationSummary {
    let mut summary = ValidationSummary {
        document_count: documents.len(),
        ..ValidationSummary::default()
    };
    for document in documents {
        for diagnostic in &document.diagnostics {
            match diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR) {
                DiagnosticSeverity::ERROR => summary.error_count += 1,
                DiagnosticSeverity::WARNING => summary.warning_count += 1,
                DiagnosticSeverity::INFORMATION | DiagnosticSeverity::HINT => {
                    summary.information_count += 1
                }
                _ => summary.error_count += 1,
            }
        }
    }
    summary
}

fn print_text_report(report: &ValidationReport) {
    for document in &report.documents {
        for diagnostic in &document.diagnostics {
            let severity = diagnostic.severity.map(severity_label).unwrap_or("error");
            let code = diagnostic
                .code
                .as_ref()
                .map(number_or_string_label)
                .unwrap_or_default();
            println!(
                "{}:{}:{}: {}{}{}",
                document.uri,
                diagnostic.range.start.line + 1,
                diagnostic.range.start.character + 1,
                severity,
                if code.is_empty() { "" } else { " [" },
                if code.is_empty() {
                    diagnostic.message.clone()
                } else {
                    format!("{code}] {}", diagnostic.message)
                }
            );
        }
    }
    println!(
        "Checked {} document(s): {} error(s), {} warning(s), {} info(s)",
        report.summary.document_count,
        report.summary.error_count,
        report.summary.warning_count,
        report.summary.information_count
    );
    for advice in &report.advice {
        println!("Advice: {advice}");
    }
}

fn sarif_report(report: &ValidationReport) -> serde_json::Value {
    let results = report
        .documents
        .iter()
        .flat_map(|document| {
            document.diagnostics.iter().map(move |diagnostic| {
                let rule_id = diagnostic
                    .code
                    .as_ref()
                    .map(number_or_string_label)
                    .unwrap_or_else(|| "spec42".to_string());
                serde_json::json!({
                    "ruleId": rule_id,
                    "level": sarif_level(diagnostic.severity),
                    "message": { "text": diagnostic.message },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": { "uri": document.uri },
                            "region": {
                                "startLine": diagnostic.range.start.line + 1,
                                "startColumn": diagnostic.range.start.character + 1,
                                "endLine": diagnostic.range.end.line + 1,
                                "endColumn": diagnostic.range.end.character + 1
                            }
                        }
                    }]
                })
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "spec42",
                    "informationUri": "https://github.com/elan8/spec42"
                }
            },
            "results": results
        }]
    })
}

fn junit_report(report: &ValidationReport) -> String {
    let tests = report
        .documents
        .iter()
        .map(|document| document.diagnostics.len().max(1))
        .sum::<usize>();
    let failures = report.summary.error_count + report.summary.warning_count;
    let mut out = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><testsuite name="spec42" tests="{}" failures="{}">"#,
        tests, failures
    );
    for document in &report.documents {
        if document.diagnostics.is_empty() {
            out.push_str(&format!(
                r#"<testcase classname="spec42" name="{}"/>"#,
                xml_escape(&document.uri)
            ));
            continue;
        }
        for diagnostic in &document.diagnostics {
            let name = format!(
                "{}:{}:{}",
                document.uri,
                diagnostic.range.start.line + 1,
                diagnostic.range.start.character + 1
            );
            let severity = diagnostic.severity.map(severity_label).unwrap_or("error");
            out.push_str(&format!(
                r#"<testcase classname="spec42" name="{}"><failure type="{}" message="{}">{}</failure></testcase>"#,
                xml_escape(&name),
                xml_escape(severity),
                xml_escape(&diagnostic.message),
                xml_escape(&format!(
                    "{}:{}:{}: {}",
                    document.uri,
                    diagnostic.range.start.line + 1,
                    diagnostic.range.start.character + 1,
                    diagnostic.message
                ))
            ));
        }
    }
    out.push_str("</testsuite>");
    out
}

fn sarif_level(severity: Option<DiagnosticSeverity>) -> &'static str {
    match severity.unwrap_or(DiagnosticSeverity::ERROR) {
        DiagnosticSeverity::ERROR => "error",
        DiagnosticSeverity::WARNING => "warning",
        DiagnosticSeverity::INFORMATION | DiagnosticSeverity::HINT => "note",
        _ => "error",
    }
}

fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::ERROR => "error",
        DiagnosticSeverity::WARNING => "warning",
        DiagnosticSeverity::INFORMATION => "info",
        DiagnosticSeverity::HINT => "hint",
        _ => "unknown",
    }
}

fn severity_number(severity: DiagnosticSeverity) -> i32 {
    match severity {
        DiagnosticSeverity::ERROR => 1,
        DiagnosticSeverity::WARNING => 2,
        DiagnosticSeverity::INFORMATION => 3,
        DiagnosticSeverity::HINT => 4,
        _ => 1,
    }
}

fn number_or_string_label(value: &NumberOrString) -> String {
    match value {
        NumberOrString::String(value) => value.clone(),
        NumberOrString::Number(value) => value.to_string(),
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

    fn sample_report() -> ValidationReport {
        ValidationReport {
            workspace_root: None,
            resolved_library_paths: Vec::new(),
            documents: vec![ValidatedDocument {
                uri: "file:///model.sysml".to_string(),
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 2,
                        },
                        end: Position {
                            line: 1,
                            character: 8,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("demo_rule".to_string())),
                    source: Some("spec42".to_string()),
                    message: "Demo warning".to_string(),
                    ..Diagnostic::default()
                }],
            }],
            summary: ValidationSummary {
                document_count: 1,
                error_count: 0,
                warning_count: 1,
                information_count: 0,
            },
            advice: Vec::new(),
        }
    }

    #[test]
    fn sarif_contains_diagnostic_location_and_rule() {
        let sarif = sarif_report(&sample_report());
        let result = &sarif["runs"][0]["results"][0];
        assert_eq!(result["ruleId"], "demo_rule");
        assert_eq!(result["level"], "warning");
        assert_eq!(
            result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
            "file:///model.sysml"
        );
    }

    #[test]
    fn junit_contains_failure() {
        let junit = junit_report(&sample_report());
        assert!(junit.contains("Demo warning"));
        assert!(junit.contains("failures=\"1\""));
    }

    #[test]
    fn baseline_filters_matching_diagnostics() {
        let temp = tempfile::tempdir().expect("temp dir");
        let baseline = temp.path().join("baseline.json");
        std::fs::write(
            &baseline,
            serde_json::to_string(&sample_report()).expect("serialize report"),
        )
        .expect("write baseline");
        let filtered = apply_baseline(&sample_report(), baseline.as_path()).expect("baseline");
        assert_eq!(filtered.summary.warning_count, 0);
        assert!(filtered.documents[0].diagnostics.is_empty());
    }
}
