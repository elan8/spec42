use semantic_core::{SemanticGraph, SemanticNode, TextPosition, TextRange};
use url::Url;

use crate::presentation_hover::signature_from_node;

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

/// Collects symbol entries for a URI from the semantic graph.
pub fn symbol_entries_for_uri(graph: &SemanticGraph, uri: &Url) -> Vec<SymbolEntry> {
    let mut out = Vec::new();
    for node in graph.nodes_for_uri(uri) {
        let container_name = node
            .parent_id
            .as_ref()
            .and_then(|pid| graph.get_node(pid))
            .map(|p| p.name.clone());
        let description = format!("{} '{}'", node.element_kind, node.name);
        let signature = signature_from_node(node);
        out.push(SymbolEntry {
            name: node.name.clone(),
            uri: node.id.uri.clone(),
            range: node.range,
            container_name,
            detail: Some(node.element_kind.as_str().to_string()),
            description: Some(description),
            signature,
        });
    }
    out
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
            let before_ok = start == 0
                || !line[..start]
                    .chars()
                    .next_back()
                    .is_some_and(is_ident_char);
            let after_ok = end >= line.len() || !line[end..].chars().next().is_some_and(is_ident_char);
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

pub fn symbol_entry_node_id(
    graph: &SemanticGraph,
    entry: &SymbolEntry,
) -> Option<semantic_core::NodeId> {
    let entry_uri = crate::uri::normalize_uri(&entry.uri);
    graph
        .nodes_for_uri(&entry_uri)
        .into_iter()
        .find(|node| node.name == entry.name && node.range == entry.range)
        .map(|node| node.id.clone())
}

pub fn location_node_id(
    graph: &SemanticGraph,
    uri: &Url,
    lookup_name: &str,
    range: TextRange,
) -> Option<semantic_core::NodeId> {
    graph
        .nodes_for_uri(uri)
        .into_iter()
        .find(|node| node.name == lookup_name && node.range == range)
        .map(|node| node.id.clone())
}

pub fn node_to_source_location(path: &str, node: &SemanticNode) -> crate::dto::SourceLocation {
    crate::dto::SourceLocation {
        path: path.to_string(),
        range: node.range,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semantic_core::build_graph_from_doc;
    use sysml_v2_parser::parse;
    use url::Url;

    #[test]
    fn find_reference_ranges_finds_multiple_occurrences() {
        let ranges = find_reference_ranges("foo bar foo baz foo", "foo");
        assert_eq!(ranges.len(), 3);
    }

    #[test]
    fn symbol_entries_for_uri_includes_definitions() {
        let input = "package P { part def Engine { } }";
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let symbols = symbol_entries_for_uri(&graph, &uri);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"P"));
        assert!(names.contains(&"Engine"));
    }
}
