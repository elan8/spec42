use url::Url;

use crate::types::{DiagnosticRelatedInfo, DiagnosticSeverity, SemanticDiagnostic};
use sysml_model::semantic::text_span::{TextPosition, TextRange};

fn utf16_len(s: &str) -> u32 {
    s.encode_utf16().count() as u32
}

fn parse_untyped_part_usage_line(raw_line: &str) -> Option<String> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let trimmed = code_only.trim();
    if !trimmed.starts_with("part ") || trimmed.starts_with("part def") {
        return None;
    }
    if !trimmed.ends_with(';') || trimmed.contains(':') {
        return None;
    }
    let after_part = trimmed.strip_prefix("part ")?;
    let name = after_part.strip_suffix(';')?.trim();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    Some(name.to_string())
}

fn first_import_range(content: &str) -> Option<TextRange> {
    for (line_idx, raw_line) in content.lines().enumerate() {
        let code_only = raw_line.split("//").next().unwrap_or("");
        let trimmed = code_only.trim();
        if !trimmed.starts_with("import ") {
            continue;
        }
        let start_char = utf16_len(raw_line) - utf16_len(raw_line.trim_start());
        let end_char = start_char + utf16_len(trimmed);
        return Some(TextRange::new(
            TextPosition::new(line_idx as u32, start_char),
            TextPosition::new(line_idx as u32, end_char),
        ));
    }
    None
}

pub fn collect_untyped_part_usage_diagnostics(uri: &Url, content: &str) -> Vec<SemanticDiagnostic> {
    let mut out = Vec::new();
    for (line_idx, raw_line) in content.lines().enumerate() {
        let Some(name) = parse_untyped_part_usage_line(raw_line) else {
            continue;
        };
        let start_char = utf16_len(raw_line) - utf16_len(raw_line.trim_start());
        let end_char = utf16_len(raw_line);
        out.push(SemanticDiagnostic {
            uri: uri.clone(),
            range: TextRange::new(
                TextPosition::new(line_idx as u32, start_char),
                TextPosition::new(line_idx as u32, end_char),
            ),
            severity: DiagnosticSeverity::Information,
            source: "sysml".to_string(),
            code: "untyped_part_usage".to_string(),
            message: format!("Part '{name}' has no declared type."),
            related_information: Vec::<DiagnosticRelatedInfo>::new(),
        });
    }
    out
}

pub fn missing_library_context_diagnostic(
    uri: &Url,
    content: &str,
    has_unresolved_library_references: bool,
    has_library_paths: bool,
) -> Option<SemanticDiagnostic> {
    if !has_unresolved_library_references || has_library_paths {
        return None;
    }
    let range = first_import_range(content)?;
    Some(SemanticDiagnostic {
        uri: uri.clone(),
        range,
        severity: DiagnosticSeverity::Information,
        source: "semantic".to_string(),
        code: "missing_library_context".to_string(),
        message: "This document imports external library symbols, but no SysML library paths are configured or indexed. Install or configure a library if these references should resolve.".to_string(),
        related_information: Vec::<DiagnosticRelatedInfo>::new(),
    })
}
