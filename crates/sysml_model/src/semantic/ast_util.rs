//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use std::collections::HashMap;

use crate::semantic::text_span::{TextPosition, TextRange};
use sysml_v2_parser::ast::Identification;
use sysml_v2_parser::Span;

/// Converts sysml-v2-parser Span (1-based line/column) to 0-based TextRange.
pub fn span_to_range(span: &Span) -> TextRange {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    TextRange::new(
        TextPosition::new(start_line, start_char),
        TextPosition::new(end_line, end_char),
    )
}

pub fn text_range_to_json(range: TextRange) -> serde_json::Value {
    serde_json::json!({
        "start": {
            "line": range.start.line,
            "character": range.start.character,
        },
        "end": {
            "line": range.end.line,
            "character": range.end.character,
        },
    })
}

/// Returns the display name from Identification (name, or short_name, or empty string).
pub fn identification_name(ident: &Identification) -> String {
    ident
        .name
        .as_deref()
        .or(ident.short_name.as_deref())
        .unwrap_or("")
        .to_string()
}

/// Stashes `identification.short_name` as a `"shortName"` attribute when both a short name
/// and a regular name are present. When short_name is the *only* name, `identification_name`
/// already uses it as `SemanticNode.name`, so there's nothing extra to capture — without this,
/// a short name declared alongside a regular name (e.g. `part def <'CB'> ControlBoard;`) was
/// silently dropped: nothing outside the raw source text ever knew `CB` refers to
/// `ControlBoard`, so references to `CB` failed to resolve entirely.
pub fn attach_short_name_attribute(
    attrs: &mut HashMap<String, serde_json::Value>,
    identification: &Identification,
) {
    if identification.name.is_none() {
        return;
    }
    if let Some(short) = identification
        .short_name
        .as_deref()
        .filter(|s| !s.is_empty())
    {
        attrs.insert("shortName".to_string(), serde_json::json!(short));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identification(name: Option<&str>, short_name: Option<&str>) -> Identification {
        Identification {
            name: name.map(str::to_string),
            short_name: short_name.map(str::to_string),
        }
    }

    #[test]
    fn attaches_short_name_when_both_name_and_short_name_present() {
        let ident = identification(Some("ControlBoard"), Some("CB"));
        let mut attrs = HashMap::new();
        attach_short_name_attribute(&mut attrs, &ident);
        assert_eq!(
            attrs.get("shortName").and_then(|v| v.as_str()),
            Some("CB")
        );
    }

    #[test]
    fn does_not_attach_short_name_when_only_short_name_present() {
        // identification_name already uses short_name as the node's primary name in this case,
        // so there is nothing extra to capture.
        let ident = identification(None, Some("CB"));
        let mut attrs = HashMap::new();
        attach_short_name_attribute(&mut attrs, &ident);
        assert!(!attrs.contains_key("shortName"));
    }

    #[test]
    fn does_not_attach_short_name_when_absent() {
        let ident = identification(Some("ControlBoard"), None);
        let mut attrs = HashMap::new();
        attach_short_name_attribute(&mut attrs, &ident);
        assert!(!attrs.contains_key("shortName"));
    }
}
