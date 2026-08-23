//! URI, config, and document helpers.

use std::collections::BTreeSet;
use std::path::PathBuf;

use tower_lsp::lsp_types::{Position, Range, Url};

use crate::language::{position_to_byte_offset, SymbolEntry};
use language_service::utf16_len;

/// Applies an incremental content change (range + new text) to the document.
/// Uses LSP UTF-16 positions and only slices on validated UTF-8 byte boundaries.
pub fn apply_incremental_change(text: &str, range: &Range, new_text: &str) -> Option<String> {
    let start_byte = position_to_byte_offset(text, range.start.line, range.start.character)?;
    let end_byte = position_to_byte_offset(text, range.end.line, range.end.character)?;
    if start_byte > text.len() || end_byte > text.len() || start_byte > end_byte {
        return None;
    }
    let mut out = String::with_capacity(text.len() - (end_byte - start_byte) + new_text.len());
    out.push_str(&text[..start_byte]);
    out.push_str(new_text);
    out.push_str(&text[end_byte..]);
    Some(out)
}

/// Reuse the repository-owned URI identity policy at the LSP admission boundary.
pub fn normalize_file_uri(uri: &Url) -> Url {
    sysml_query::source::normalize_uri(uri)
}

/// The first `max_errors` parser diagnostics of a parsed document, formatted for a log line.
pub fn parse_failure_diagnostics(
    parsed: &sysml_query::syntax::ParsedSource,
    max_errors: usize,
) -> Vec<String> {
    parsed
        .diagnostics()
        .iter()
        .take(max_errors)
        .map(|e| {
            let loc = e
                .range()
                .map(|range| format!("{}:{}", range.start_line, range.start_character))
                .unwrap_or_else(|| format!("{:?}:{:?}", e.line, e.column));
            format!("{} {}", loc, e.message)
        })
        .collect()
}

/// Editor-oriented parse through the syntax service: always a document, diagnostics additive.
pub fn parse_for_editor(text: &str) -> sysml_query::syntax::ParsedSource {
    sysml_query::syntax::SyntaxService::new().parse_text(text)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntypedPartUsage {
    pub name: String,
    pub range: Range,
}

pub fn untyped_part_usage_diagnostics(content: &str) -> Vec<UntypedPartUsage> {
    let mut out = Vec::new();
    for (line_idx, raw_line) in content.lines().enumerate() {
        let Some(name) = language_service::parse_untyped_part_usage_name(raw_line) else {
            continue;
        };
        let start_char = utf16_len(raw_line) - utf16_len(raw_line.trim_start());
        let end_char = utf16_len(raw_line);
        out.push(UntypedPartUsage {
            name,
            range: Range {
                start: Position::new(line_idx as u32, start_char),
                end: Position::new(line_idx as u32, end_char),
            },
        });
    }
    out
}

/// Returns true if `uri` is under any of the library path roots (path prefix check).
pub fn uri_under_any_library(uri: &Url, library_paths: &[Url]) -> bool {
    sysml_query::source::uri_under_any(uri, library_paths)
}

/// Parse library paths from LSP config (initialization_options or didChangeConfiguration settings).
pub fn parse_library_paths_from_value(value: Option<&serde_json::Value>) -> Vec<Url> {
    value
        .and_then(|opts| opts.get("libraryPaths"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|s| s.as_str())
                .filter_map(|path_str| {
                    let path = std::path::PathBuf::from(path_str);
                    Url::from_file_path(path)
                        .ok()
                        .map(|u| normalize_file_uri(&u))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Prepend host default library directories and merge with client `libraryPaths` (deduplicated).
pub fn merge_host_and_client_library_paths(
    host_defaults: &[PathBuf],
    client: Vec<Url>,
) -> Vec<Url> {
    let mut seen = BTreeSet::<String>::new();
    let mut out = Vec::new();
    for p in host_defaults {
        if let Ok(u) = Url::from_file_path(p) {
            let n = normalize_file_uri(&u);
            if seen.insert(n.as_str().to_string()) {
                out.push(n);
            }
        }
    }
    for u in client {
        let n = normalize_file_uri(&u);
        if seen.insert(n.as_str().to_string()) {
            out.push(n);
        }
    }
    out
}

pub fn parse_startup_trace_id_from_value(value: Option<&serde_json::Value>) -> Option<String> {
    value
        .and_then(|opts| opts.get("startupTraceId"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToOwned::to_owned)
}

pub fn parse_code_lens_enabled_from_value(
    value: Option<&serde_json::Value>,
    default_enabled: bool,
) -> bool {
    value
        .and_then(|opts| opts.get("codeLens"))
        .and_then(|code_lens| code_lens.get("enabled"))
        .and_then(|enabled| enabled.as_bool())
        .unwrap_or(default_enabled)
}

pub fn parse_perf_logging_enabled_from_value(
    value: Option<&serde_json::Value>,
    default_enabled: bool,
) -> bool {
    value
        .and_then(|opts| opts.get("performanceLogging"))
        .and_then(|perf| perf.get("enabled"))
        .and_then(|enabled| enabled.as_bool())
        .unwrap_or(default_enabled)
}

/// Development-only override: include library paths in the debounced workspace-wide
/// diagnostics sweep (normally excluded — see `publish_workspace_diagnostics`'s comment
/// and `docs/engineering/PERFORMANCE-GUARDRAILS.md`). Maps to the VS Code setting
/// `spec42.development.diagnoseLibraryPaths`.
pub fn parse_diagnose_library_paths_from_value(
    value: Option<&serde_json::Value>,
    default_enabled: bool,
) -> bool {
    value
        .and_then(|opts| opts.get("diagnostics"))
        .and_then(|diagnostics| diagnostics.get("includeLibraryPaths"))
        .and_then(|enabled| enabled.as_bool())
        .unwrap_or(default_enabled)
}

pub fn env_flag_enabled(name: &str, default_enabled: bool) -> bool {
    let Ok(raw_value) = std::env::var(name) else {
        return default_enabled;
    };
    let normalized = raw_value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return default_enabled;
    }
    match normalized.as_str() {
        "1" | "true" | "yes" | "on" => true,
        "0" | "false" | "no" | "off" => false,
        _ => default_enabled,
    }
}

pub fn env_usize(name: &str, default_value: usize) -> usize {
    let Ok(raw_value) = std::env::var(name) else {
        return default_value;
    };
    raw_value
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

/// Builds Markdown for symbol hover. Presentation is owned by `language_service`.
pub fn symbol_hover_markdown(entry: &SymbolEntry, show_location: bool) -> String {
    language_service::symbol_hover_markdown(entry, show_location)
}

/// Whether the host was asked to admit whole library trees instead of the import closure.
///
/// Host configuration, read once here: the closure service decides what a workspace needs, and
/// this only says whether the host asks it at all.
pub(crate) fn library_full_scan_enabled() -> bool {
    env_flag_enabled("SPEC42_LIBRARY_FULL_SCAN", false)
}

#[cfg(test)]
mod tests {
    use super::{
        apply_incremental_change, normalize_file_uri,
        parse_diagnose_library_paths_from_value, untyped_part_usage_diagnostics,
    };
    use tower_lsp::lsp_types::{Position, Range};

    #[cfg(unix)]
    #[test]
    fn file_uri_admission_collapses_filesystem_aliases() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().expect("tempdir");
        let real = temp.path().join("real");
        let alias = temp.path().join("alias");
        std::fs::create_dir(&real).expect("real directory");
        symlink(&real, &alias).expect("directory alias");
        let real_file = real.join("Model.sysml");
        std::fs::write(&real_file, "package Model;").expect("model");

        let real_uri = tower_lsp::lsp_types::Url::from_file_path(real_file).expect("real URI");
        let alias_uri = tower_lsp::lsp_types::Url::from_file_path(alias.join("Model.sysml"))
            .expect("alias URI");
        assert_eq!(
            normalize_file_uri(&real_uri),
            normalize_file_uri(&alias_uri)
        );
    }

    #[test]
    fn apply_incremental_change_handles_ascii_edit() {
        let text = "package Demo {\n  part def Engine;\n}\n";
        let range = Range::new(Position::new(1, 17), Position::new(1, 18));
        let updated = apply_incremental_change(text, &range, "").expect("edit applies");
        assert_eq!(updated, "package Demo {\n  part def Engine\n}\n");
    }


    #[test]
    fn untyped_part_usage_diagnostics_detects_part_usage_without_type() {
        let text = "package P {\n  part def Laptop {\n    part display;\n  }\n}\n";
        let diagnostics = untyped_part_usage_diagnostics(text);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].name, "display");
        assert_eq!(diagnostics[0].range.start.line, 2);
    }

    #[test]
    fn untyped_part_usage_diagnostics_ignores_typed_usage() {
        let text = "package P {\n  part def Laptop {\n    part display : Display;\n  }\n}\n";
        let diagnostics = untyped_part_usage_diagnostics(text);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn diagnose_library_paths_defaults_when_absent() {
        assert!(!parse_diagnose_library_paths_from_value(None, false));
        assert!(parse_diagnose_library_paths_from_value(None, true));
    }

    #[test]
    fn diagnose_library_paths_reads_nested_flag() {
        let value = serde_json::json!({ "diagnostics": { "includeLibraryPaths": true } });
        assert!(parse_diagnose_library_paths_from_value(Some(&value), false));

        let value = serde_json::json!({ "diagnostics": { "includeLibraryPaths": false } });
        assert!(!parse_diagnose_library_paths_from_value(Some(&value), true));
    }
}
