//! URI, config, and document helpers.

use tower_lsp::lsp_types::{Position, Range, Url};

use crate::language::{position_to_byte_offset, SymbolEntry};

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

/// Normalize file URIs so that file:///C:/... and file:///c%3A/... (from client) match in the index.
/// Uses lowercase drive letter and decoded path so both server (from_file_path) and client URIs align.
pub fn normalize_file_uri(uri: &Url) -> Url {
    if uri.scheme() != "file" {
        return uri.clone();
    }
    // Prefer filesystem roundtrip: decodes percent-encoding (e.g. c%3A -> c:)
    // and yields consistent file URI formatting across client/server.
    if let Ok(path) = uri.to_file_path() {
        if let Ok(mut normalized) = Url::from_file_path(path) {
            let p = normalized.path();
            if p.len() >= 3 {
                let mut chars: Vec<char> = p.chars().collect();
                if chars[0] == '/' && chars[1].is_ascii_alphabetic() && chars.get(2) == Some(&':') {
                    chars[1] = chars[1].to_ascii_lowercase();
                    let new_path: String = chars.into_iter().collect();
                    if let Ok(u) = Url::parse(&format!("file://{}", new_path)) {
                        normalized = u;
                    }
                }
            }
            return normalized;
        }
    }
    let path = uri.path();
    if path.len() >= 3 {
        let mut chars: Vec<char> = path.chars().collect();
        if chars[0] == '/' && chars[1].is_ascii_alphabetic() && chars.get(2) == Some(&':') {
            chars[1] = chars[1].to_ascii_lowercase();
            let new_path: String = chars.into_iter().collect();
            if let Ok(u) = Url::parse(&format!("file://{}", new_path)) {
                return u;
            }
        }
    }
    uri.clone()
}

/// When parse fails, get diagnostic messages from parse_with_diagnostics for logging.
pub fn parse_failure_diagnostics(content: &str, max_errors: usize) -> Vec<String> {
    let result = sysml_parser::parse_with_diagnostics(content);
    result
        .errors
        .iter()
        .take(max_errors)
        .map(|e| {
            let loc = e
                .to_lsp_range()
                .map(|(sl, sc, _, _)| format!("{}:{}", sl, sc))
                .unwrap_or_else(|| format!("{:?}:{:?}", e.line, e.column));
            format!("{} {}", loc, e.message)
        })
        .collect()
}

/// Editor-oriented parse: returns a (possibly partial) AST plus diagnostics.
///
/// `sysml-parser` currently exposes this behavior as `parse_with_diagnostics`.
pub fn parse_for_editor(text: &str) -> sysml_parser::ParseResult {
    sysml_parser::parse_with_diagnostics(text)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntypedPartUsage {
    pub name: String,
    pub range: Range,
}

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

pub fn untyped_part_usage_diagnostics(content: &str) -> Vec<UntypedPartUsage> {
    let mut out = Vec::new();
    for (line_idx, raw_line) in content.lines().enumerate() {
        let Some(name) = parse_untyped_part_usage_line(raw_line) else {
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

/// Lightweight fallback syntax checks for cases where parser diagnostics are empty.
/// Currently flags likely statement lines missing a trailing semicolon.
pub fn missing_semicolon_ranges(content: &str) -> Vec<Range> {
    const KEYWORDS_REQUIRING_SEMICOLON: &[&str] = &[
        "part",
        "port",
        "attribute",
        "item",
        "import",
        "alias",
        "connection",
        "bind",
        "allocate",
        "ref",
    ];

    let mut ranges = Vec::new();
    for (line_idx, raw_line) in content.lines().enumerate() {
        let code_only = strip_line_comment(raw_line);
        let trimmed = code_only.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.ends_with(';') || trimmed.ends_with('{') || trimmed.ends_with('}') {
            continue;
        }
        let first = match trimmed.split_whitespace().next() {
            Some(token) => token,
            None => continue,
        };
        if !KEYWORDS_REQUIRING_SEMICOLON.contains(&first) {
            continue;
        }
        if trimmed.starts_with("part def")
            || trimmed.starts_with("port def")
            || trimmed.starts_with("attribute def")
            || trimmed.starts_with("item def")
            || trimmed.starts_with("connection def")
            || trimmed.starts_with("allocation def")
        {
            continue;
        }

        let start_char = utf16_len(raw_line) - utf16_len(raw_line.trim_start());
        let end_char = utf16_len(raw_line);
        ranges.push(Range {
            start: tower_lsp::lsp_types::Position::new(line_idx as u32, start_char),
            end: tower_lsp::lsp_types::Position::new(line_idx as u32, end_char),
        });
    }

    ranges
}

fn strip_line_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut idx = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    while idx + 1 < bytes.len() {
        let ch = bytes[idx];
        if escaped {
            escaped = false;
            idx += 1;
            continue;
        }
        match ch {
            b'\\' if in_string => escaped = true,
            b'"' => in_string = !in_string,
            b'/' if !in_string && bytes[idx + 1] == b'/' => return &line[..idx],
            _ => {}
        }
        idx += 1;
    }
    line
}

pub fn import_statement_ranges(content: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    for (line_idx, raw_line) in content.lines().enumerate() {
        let code_only = raw_line.split("//").next().unwrap_or("");
        let trimmed = code_only.trim();
        if !trimmed.starts_with("import ") {
            continue;
        }

        let start_char = utf16_len(raw_line) - utf16_len(raw_line.trim_start());
        let end_char = start_char + utf16_len(trimmed);
        ranges.push(Range {
            start: Position::new(line_idx as u32, start_char),
            end: Position::new(line_idx as u32, end_char),
        });
    }
    ranges
}

/// Returns true if `uri` is under any of the library path roots (path prefix check).
pub fn uri_under_any_library(uri: &Url, library_paths: &[Url]) -> bool {
    semantic_model_crate::uri_under_any_library(uri, library_paths)
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

pub fn parse_startup_trace_id_from_value(value: Option<&serde_json::Value>) -> Option<String> {
    value
        .and_then(|opts| opts.get("startupTraceId"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToOwned::to_owned)
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

/// Builds Markdown for symbol hover: title (kind + name), code block with signature or description, container, optional location.
pub fn symbol_hover_markdown(entry: &SymbolEntry, show_location: bool) -> String {
    let kind = entry.detail.as_deref().unwrap_or("symbol");
    let name = &entry.name;
    let mut md = format!("**{}** `{}`\n\n", kind, name);
    let code_block = entry
        .signature
        .as_deref()
        .or(entry.description.as_deref())
        .unwrap_or(name.as_str());
    md.push_str("```sysml\n");
    md.push_str(code_block);
    md.push_str("\n```\n\n");
    if let Some(ref pkg) = entry.container_name {
        if pkg != "(top level)" {
            md.push_str(&format!("*Package:* `{}`\n\n", pkg));
        }
    }
    if show_location {
        md.push_str(&format!("*Defined in:* {}", entry.uri.path()));
    }
    md
}

#[cfg(test)]
mod tests {
    use super::{
        apply_incremental_change, import_statement_ranges, missing_semicolon_ranges,
        untyped_part_usage_diagnostics,
    };
    use tower_lsp::lsp_types::{Position, Range};

    #[test]
    fn apply_incremental_change_handles_ascii_edit() {
        let text = "package Demo {\n  part def Engine;\n}\n";
        let range = Range::new(Position::new(1, 17), Position::new(1, 18));
        let updated = apply_incremental_change(text, &range, "").expect("edit applies");
        assert_eq!(updated, "package Demo {\n  part def Engine\n}\n");
    }

    #[test]
    fn import_statement_ranges_detects_import_lines() {
        let content = "package P {\n  import ScalarValues::Real;\n  // import Ignored::Type;\n}\n";
        let ranges = import_statement_ranges(content);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start.line, 1);
        assert_eq!(ranges[0].start.character, 2);
    }

    #[test]
    fn missing_semicolon_ranges_detects_unterminated_part_usage() {
        let text = "package test {\n  part def Laptop {\n    part motherboard\n  }\n}\n";
        let ranges = missing_semicolon_ranges(text);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start.line, 2);
    }

    #[test]
    fn missing_semicolon_ranges_ignores_terminated_lines() {
        let text = "package test {\n  part def Laptop {\n    part motherboard;\n  }\n}\n";
        let ranges = missing_semicolon_ranges(text);
        assert!(ranges.is_empty());
    }

    #[test]
    fn missing_semicolon_ranges_ignores_url_like_strings() {
        let text = "package test {\n  part def Repo {\n    attribute repositoryUrl = \"https://git.example.com/orders-service\";\n    attribute vaultUri = \"vault://orders/config\";\n  }\n}\n";
        let ranges = missing_semicolon_ranges(text);
        assert!(ranges.is_empty());
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
}
