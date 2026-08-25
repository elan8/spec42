use sysml_query::resolved_slice::PublishedModel;
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
    let symbols = match model.inspection().document_symbols(uri.as_str()).answer {
        sysml_query::resolved_slice::QueryAnswer::Resolved(value) => value,
        _ => return Vec::new(),
    };
    symbols
        .into_vec()
        .into_iter()
        .filter_map(|symbol| {
            let name = symbol.name?.into_string();
            let container_name = model
                .qualified_name(symbol.identity)
                .and_then(|qualified_name| qualified_name.rsplit_once("::"))
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
                uri: Url::parse(model.document_identity(symbol.location.document)?).ok()?,
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

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_query::{source::SourceKind, Services};
    use url::Url;

    #[test]
    fn symbol_entries_for_uri_includes_definitions() {
        let input = "package P { part def Engine { } }";
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let services = Services::new();
        let source = services
            .source
            .admit(uri.as_str(), input, SourceKind::Workspace)
            .unwrap();
        let model = services.publication.publish(&[source], []).unwrap();
        let symbols = symbol_entries_for_uri(&model, &uri);
        let names: Vec<_> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"P"));
        assert!(names.contains(&"Engine"));
    }
}
