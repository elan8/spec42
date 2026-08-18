use sysml_query::resolved_slice::{PublishedModel, QueryOutcome};
use sysml_query::resolved_slice::{TextPosition, TextRange};
use url::Url;

/// Neutral symbol table entry for editor lookup (no LSP types).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolEntry {
    pub name: String,
    pub uri: Url,
    pub range: TextRange,
    pub container_name: Option<String>,
    pub detail: Option<String>,
    pub description: Option<String>,
    pub signature: Option<String>,
}

/// Collects the immutable publication's symbols for one document.
pub fn symbol_entries_for_uri(model: &PublishedModel, uri: &Url) -> Vec<SymbolEntry> {
    let symbols = match model.inspection().document_symbols(uri.as_str()) {
        QueryOutcome::Resolved(value)
        | QueryOutcome::Recovered(value)
        | QueryOutcome::UnsupportedWith(value) => value,
        _ => return Vec::new(),
    };
    symbols
        .into_vec()
        .into_iter()
        .filter_map(|symbol| {
            let name = symbol.name?.into_string();
            let container_name = symbol
                .qualified_name
                .rsplit_once("::")
                .map(|(owner, _)| owner.to_string());
            let detail = symbol.kind.as_str().to_string();
            let range = TextRange::new(
                TextPosition::new(
                    symbol.location.range.start.line,
                    symbol.location.range.start.character,
                ),
                TextPosition::new(
                    symbol.location.range.end.line,
                    symbol.location.range.end.character,
                ),
            );
            Some(SymbolEntry {
                description: Some(format!("{detail} '{name}'")),
                signature: None,
                name,
                uri: Url::parse(&symbol.location.document).ok()?,
                range,
                container_name,
                detail: Some(detail),
            })
        })
        .collect()
}

/// Builds Markdown for symbol hover from a neutral symbol entry.
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

/// Returns all ranges in `source` where `name` appears as a whole word.
pub fn find_reference_ranges(source: &str, name: &str) -> Vec<TextRange> {
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '-'
    }
    if name.is_empty() {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    for (line_no, line) in source.lines().enumerate() {
        let mut search_start = 0;
        while let Some(off) = line[search_start..].find(name) {
            let start = search_start + off;
            let end = start + name.len();
            let before_ok =
                start == 0 || !line[..start].chars().next_back().is_some_and(is_ident_char);
            let after_ok =
                end >= line.len() || !line[end..].chars().next().is_some_and(is_ident_char);
            if before_ok && after_ok {
                let start_char = line[..start].chars().count() as u32;
                let end_char = start_char + name.chars().count() as u32;
                ranges.push(TextRange {
                    start: TextPosition {
                        line: line_no as u32,
                        character: start_char,
                    },
                    end: TextPosition {
                        line: line_no as u32,
                        character: end_char,
                    },
                });
            }
            search_start = end;
        }
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_query::resolved_slice::{
        BuildRequest, ConstructionStrategy, SourceDocument, SourceKind,
    };
    use url::Url;

    #[test]
    fn find_reference_ranges_finds_multiple_occurrences() {
        let ranges = find_reference_ranges("foo bar foo baz foo", "foo");
        assert_eq!(ranges.len(), 3);
    }

    #[test]
    fn symbol_entries_for_uri_includes_definitions() {
        let input = "package P { part def Engine { } }";
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let source =
            SourceDocument::from_uri(uri.as_str(), input.to_string(), SourceKind::Workspace)
                .unwrap();
        let request =
            BuildRequest::resolved(vec![source], ConstructionStrategy::Sequential).unwrap();
        let model = sysml_query::resolved_slice::build(request).unwrap();
        let symbols = symbol_entries_for_uri(&model, &uri);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"P"));
        assert!(names.contains(&"Engine"));
    }
}
