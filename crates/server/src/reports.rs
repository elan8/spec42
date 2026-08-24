//! CLI rendering of one [`HostValidationReport`].
//!
//! The report itself is `workspace`'s: this module owns only the projections the CLI publishes —
//! text, the historical LSP-shaped JSON, SARIF and JUnit — plus baseline filtering over the JSON
//! signature. Nothing here decides what a diagnostic means.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Serialize;
use sysml_diagnostics::{DiagnosticSeverity, SemanticDiagnostic};
use workspace::{HostValidatedDocument, HostValidationReport, HostValidationSummary};

use crate::{cli::OutputFormat, diagnostic_catalog};

pub fn emit_validation_report(
    report: &HostValidationReport,
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
                serde_json::to_string_pretty(&json_report(report))
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

// -- JSON projection ---------------------------------------------------------------------
//
// `spec42 check --format json` has always published the LSP diagnostic shape, and baselines on
// disk are keyed by it. The report crossed the graph as `tower-lsp` values before; now it
// crosses as neutral ones and the transport spelling is applied here, where the other three
// output formats are also produced.

#[derive(Serialize)]
struct JsonReport<'a> {
    workspace_root: &'a Option<String>,
    resolved_library_paths: &'a [String],
    documents: Vec<JsonDocument<'a>>,
    summary: &'a HostValidationSummary,
    advice: &'a [String],
}

#[derive(Serialize)]
struct JsonDocument<'a> {
    uri: &'a str,
    diagnostics: Vec<JsonDiagnostic<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonDiagnostic<'a> {
    range: JsonRange,
    severity: i32,
    code: &'a str,
    source: &'a str,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_information: Option<Vec<JsonRelatedInformation<'a>>>,
}

#[derive(Serialize)]
struct JsonRelatedInformation<'a> {
    location: JsonLocation,
    message: &'a str,
}

#[derive(Serialize)]
struct JsonLocation {
    uri: String,
    range: JsonRange,
}

#[derive(Serialize)]
struct JsonRange {
    start: JsonPosition,
    end: JsonPosition,
}

#[derive(Serialize)]
struct JsonPosition {
    line: u32,
    character: u32,
}

fn json_report(report: &HostValidationReport) -> JsonReport<'_> {
    JsonReport {
        workspace_root: &report.workspace_root,
        resolved_library_paths: &report.resolved_library_paths,
        documents: report
            .documents
            .iter()
            .map(|document| JsonDocument {
                uri: &document.uri,
                diagnostics: document.diagnostics.iter().map(json_diagnostic).collect(),
            })
            .collect(),
        summary: &report.summary,
        advice: &report.advice,
    }
}

fn json_diagnostic(diagnostic: &SemanticDiagnostic) -> JsonDiagnostic<'_> {
    JsonDiagnostic {
        range: json_range(&diagnostic.range),
        severity: severity_number(diagnostic.severity),
        code: &diagnostic.code,
        source: &diagnostic.source,
        message: &diagnostic.message,
        related_information: (!diagnostic.related_information.is_empty()).then(|| {
            diagnostic
                .related_information
                .iter()
                .map(|info| JsonRelatedInformation {
                    location: JsonLocation {
                        uri: info.uri.to_string(),
                        range: json_range(&info.range),
                    },
                    message: &info.message,
                })
                .collect()
        }),
    }
}

fn json_range(range: &sysml_query::resolved_slice::TextRange) -> JsonRange {
    JsonRange {
        start: JsonPosition {
            line: range.start.line,
            character: range.start.character,
        },
        end: JsonPosition {
            line: range.end.line,
            character: range.end.character,
        },
    }
}

// -- baseline ----------------------------------------------------------------------------

pub fn apply_baseline(
    report: &HostValidationReport,
    baseline_path: &Path,
) -> Result<HostValidationReport, String> {
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
        .map(|document| HostValidatedDocument {
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

    Ok(HostValidationReport {
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

fn diagnostic_signature(uri: &str, diagnostic: &SemanticDiagnostic) -> String {
    format!(
        "{}|{}:{}-{}:{}|{}|{}|{}",
        uri,
        diagnostic.range.start.line,
        diagnostic.range.start.character,
        diagnostic.range.end.line,
        diagnostic.range.end.character,
        severity_number(diagnostic.severity),
        diagnostic.code,
        diagnostic.message
    )
}

fn summarize(documents: &[HostValidatedDocument]) -> HostValidationSummary {
    let mut summary = HostValidationSummary {
        document_count: documents.len(),
        ..HostValidationSummary::default()
    };
    for diagnostic in documents.iter().flat_map(|document| &document.diagnostics) {
        match diagnostic.severity {
            DiagnosticSeverity::Error => summary.error_count += 1,
            DiagnosticSeverity::Warning => summary.warning_count += 1,
            DiagnosticSeverity::Information => summary.information_count += 1,
        }
    }
    summary
}

// -- text / SARIF / JUnit ------------------------------------------------------------------

fn print_text_report(report: &HostValidationReport) {
    for document in &report.documents {
        for diagnostic in &document.diagnostics {
            let severity = sysml_diagnostics::severity_label(diagnostic.severity);
            let code = if diagnostic.code.is_empty() {
                String::new()
            } else {
                format!("[{}] ", diagnostic.code)
            };
            println!(
                "{}:{}:{}: {severity} {code}{}",
                document.uri,
                diagnostic.range.start.line + 1,
                diagnostic.range.start.character + 1,
                diagnostic.message
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

fn sarif_report(report: &HostValidationReport) -> serde_json::Value {
    let rule_ids = report
        .documents
        .iter()
        .flat_map(|document| document.diagnostics.iter())
        .map(|diagnostic| diagnostic.code.clone())
        .collect::<BTreeSet<_>>();
    let rules = rule_ids
        .iter()
        .filter_map(|rule_id| diagnostic_catalog::lookup(rule_id))
        .map(sarif_rule)
        .collect::<Vec<_>>();
    let results = report
        .documents
        .iter()
        .flat_map(|document| {
            document.diagnostics.iter().map(move |diagnostic| {
                let rule_id = if diagnostic.code.is_empty() {
                    "spec42"
                } else {
                    diagnostic.code.as_str()
                };
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
                    "informationUri": "https://github.com/elan8/spec42",
                    "rules": rules
                }
            },
            "results": results
        }]
    })
}

fn sarif_rule(entry: &diagnostic_catalog::DiagnosticCatalogEntry) -> serde_json::Value {
    serde_json::json!({
        "id": entry.code,
        "name": entry.code,
        "shortDescription": { "text": entry.meaning },
        "help": { "text": entry.typical_fix },
        "defaultConfiguration": { "level": sarif_catalog_level(entry.severity) },
        "properties": { "spec42Alignment": diagnostic_catalog::alignment(entry.code) }
    })
}

fn sarif_catalog_level(severity: &str) -> &'static str {
    match severity {
        "error" => "error",
        "warning" => "warning",
        "information" | "hint" => "note",
        _ => "warning",
    }
}

fn junit_report(report: &HostValidationReport) -> String {
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
            let severity = sysml_diagnostics::severity_label(diagnostic.severity);
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

fn sarif_level(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Information => "note",
    }
}

/// The LSP severity number the JSON projection and baseline signatures are keyed by.
fn severity_number(severity: DiagnosticSeverity) -> i32 {
    match severity {
        DiagnosticSeverity::Error => 1,
        DiagnosticSeverity::Warning => 2,
        DiagnosticSeverity::Information => 3,
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
    use sysml_query::resolved_slice::{TextPosition, TextRange};

    fn sample_report() -> HostValidationReport {
        HostValidationReport {
            workspace_root: None,
            resolved_library_paths: Vec::new(),
            documents: vec![HostValidatedDocument {
                uri: "file:///model.sysml".to_string(),
                diagnostics: vec![SemanticDiagnostic {
                    uri: url::Url::parse("file:///model.sysml").expect("uri"),
                    range: TextRange::new(TextPosition::new(1, 2), TextPosition::new(1, 8)),
                    severity: DiagnosticSeverity::Warning,
                    source: "spec42".to_string(),
                    code: "demo_rule".to_string(),
                    message: "Demo warning".to_string(),
                    related_information: Vec::new(),
                }],
            }],
            summary: HostValidationSummary {
                document_count: 1,
                error_count: 0,
                warning_count: 1,
                information_count: 0,
            },
            advice: Vec::new(),
        }
    }

    #[test]
    fn json_keeps_the_published_lsp_diagnostic_shape() {
        let value = serde_json::to_value(json_report(&sample_report())).expect("json");
        let diagnostic = &value["documents"][0]["diagnostics"][0];
        assert_eq!(diagnostic["range"]["start"]["line"], 1);
        assert_eq!(diagnostic["range"]["start"]["character"], 2);
        assert_eq!(diagnostic["severity"], 2);
        assert_eq!(diagnostic["code"], "demo_rule");
        assert_eq!(diagnostic["source"], "spec42");
        assert_eq!(diagnostic["message"], "Demo warning");
        assert!(diagnostic.get("relatedInformation").is_none());
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
    fn sarif_includes_metadata_for_cataloged_rules() {
        let mut report = sample_report();
        report.documents[0].diagnostics[0].code = "unresolved_type_reference".to_string();

        let sarif = sarif_report(&report);
        let rule = &sarif["runs"][0]["tool"]["driver"]["rules"][0];
        let entry = diagnostic_catalog::lookup("unresolved_type_reference").expect("catalog");
        assert_eq!(rule["id"], entry.code);
        assert_eq!(rule["shortDescription"]["text"], entry.meaning);
        assert_eq!(rule["help"]["text"], entry.typical_fix);
        assert_eq!(rule["defaultConfiguration"]["level"], "warning");
        assert_eq!(rule["properties"]["spec42Alignment"], "spec_constraint");
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
            serde_json::to_string(&json_report(&sample_report())).expect("serialize report"),
        )
        .expect("write baseline");
        let filtered = apply_baseline(&sample_report(), baseline.as_path()).expect("baseline");
        assert_eq!(filtered.summary.warning_count, 0);
        assert!(filtered.documents[0].diagnostics.is_empty());
    }
}
