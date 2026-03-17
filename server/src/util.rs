//! URI, config, and document helpers.

use tower_lsp::lsp_types::{Range, Url};

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

#[cfg(test)]
mod tests {
    use super::apply_incremental_change;
    use tower_lsp::lsp_types::{Position, Range};

    #[test]
    fn apply_incremental_change_handles_ascii_edit() {
        let text = "package Demo {\n  part def Engine;\n}\n";
        let range = Range::new(Position::new(1, 17), Position::new(1, 18));
        let updated = apply_incremental_change(text, &range, "").expect("edit applies");
        assert_eq!(updated, "package Demo {\n  part def Engine\n}\n");
    }

    #[test]
    fn apply_incremental_change_handles_utf16_positions() {
        let text = "package Demo {\n  // ok \u{1F600} here\n}\n";
        let range = Range::new(Position::new(1, 5), Position::new(1, 8));
        let updated = apply_incremental_change(text, &range, "fine ").expect("edit applies");
        assert_eq!(updated, "package Demo {\n  // fine \u{1F600} here\n}\n");
    }

    #[test]
    fn apply_incremental_change_rejects_mid_surrogate_position() {
        let text = "package Demo {\n  // ok \u{1F600} here\n}\n";
        let range = Range::new(Position::new(1, 9), Position::new(1, 10));
        assert!(apply_incremental_change(text, &range, "x").is_none());
    }

    #[test]
    fn apply_incremental_change_rejects_reversed_range() {
        let text = "package Demo {\n  part def Engine;\n}\n";
        let range = Range::new(Position::new(1, 18), Position::new(1, 17));
        assert!(apply_incremental_change(text, &range, "").is_none());
    }

    #[test]
    fn apply_incremental_change_rejects_out_of_bounds_line() {
        let text = "package Demo {\n  part def Engine;\n}\n";
        let range = Range::new(Position::new(99, 0), Position::new(99, 1));
        assert!(apply_incremental_change(text, &range, "x").is_none());
    }

    #[test]
    fn apply_incremental_change_handles_zero_width_insert_after_emoji() {
        let text = "package Demo {\n  // ok \u{1F600} here\n}\n";
        let range = Range::new(Position::new(1, 10), Position::new(1, 10));
        let updated = apply_incremental_change(text, &range, "still ").expect("insert applies");
        assert_eq!(updated, "package Demo {\n  // ok \u{1F600}still  here\n}\n");
    }

    #[test]
    fn apply_incremental_change_rejects_zero_width_insert_inside_surrogate_pair() {
        let text = "package Demo {\n  // ok \u{1F600} here\n}\n";
        let range = Range::new(Position::new(1, 9), Position::new(1, 9));
        assert!(apply_incremental_change(text, &range, "x").is_none());
    }
}

/// Normalize file URIs so that file:///C:/... and file:///c%3A/... (from client) match in the index.
/// Uses lowercase drive letter and decoded path so both server (from_file_path) and client URIs align.
pub fn normalize_file_uri(uri: &Url) -> Url {
    if uri.scheme() != "file" {
        return uri.clone();
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

/// Returns true if `uri` is under any of the library path roots (path prefix check).
pub fn uri_under_any_library(uri: &Url, library_paths: &[Url]) -> bool {
    let uri_path = match uri.to_file_path() {
        Ok(p) => p,
        Err(_) => return false,
    };
    for lib in library_paths {
        if let Ok(lib_path) = lib.to_file_path() {
            if uri_path.starts_with(&lib_path) {
                return true;
            }
        }
    }
    false
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
                    Url::from_file_path(path).ok()
                })
                .collect()
        })
        .unwrap_or_default()
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
