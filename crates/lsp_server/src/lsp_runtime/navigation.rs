use sysml_query::syntax::SyntaxImport;
use tower_lsp::lsp_types::{DocumentLink, Position, Range, Url};

/// Document links for the imports the syntax service found, in source order.
///
/// Every link is anchored to an import's own published range. The line scan this replaced found
/// the keyword anywhere on a line -- including inside a comment or a string -- and could not tell
/// a two-line import from two imports.
pub(crate) fn collect_document_links(
    text: &str,
    imports: &[SyntaxImport],
    symbol_uri_for_import_name: impl Fn(&str) -> Option<Url>,
) -> Vec<DocumentLink> {
    let mut links = Vec::new();
    for import in imports {
        let range = Range::new(
            Position::new(import.range.start_line, import.range.start_character),
            Position::new(import.range.end_line, import.range.end_character),
        );
        // A `file://` target is a literal link the grammar keeps as part of the name, so it is
        // read from the import's own line and anchored inside the import's range.
        let line = text
            .lines()
            .nth(import.range.start_line as usize)
            .unwrap_or("");
        if let Some(file_idx) = line.find("file://") {
            let target_text = line[file_idx..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches('"')
                .trim_matches('\'');
            if let Ok(target) = Url::parse(target_text) {
                let start = line[..file_idx].chars().count() as u32;
                links.push(DocumentLink {
                    range: Range::new(
                        Position::new(import.range.start_line, start),
                        Position::new(
                            import.range.start_line,
                            start + target_text.chars().count() as u32,
                        ),
                    ),
                    target: Some(target),
                    tooltip: Some("Open import target".to_string()),
                    data: None,
                });
                continue;
            }
        }
        if let Some(uri) = symbol_uri_for_import_name(import.target) {
            links.push(DocumentLink {
                range,
                target: Some(uri),
                tooltip: Some("Open imported symbol".to_string()),
                data: None,
            });
        }
    }
    links
}

pub(crate) fn selection_ranges_for_positions(
    text: &str,
    parsed: &sysml_query::syntax::ParsedSource,
    positions: &[Position],
) -> Vec<tower_lsp::lsp_types::SelectionRange> {
    let mut out = Vec::new();
    for pos in positions {
        let mut ranges = Vec::<Range>::new();
        // The innermost selection is the token the syntax service finds under the cursor.
        if let Some(token) = parsed.token_at(pos.line, pos.character) {
            ranges.push(Range::new(
                Position::new(token.range.start_line, token.range.start_character),
                Position::new(token.range.end_line, token.range.end_character),
            ));
        }
        let line_len = text
            .lines()
            .nth(pos.line as usize)
            .map(|l| l.chars().count() as u32)
            .unwrap_or(0);
        ranges.push(Range::new(
            Position::new(pos.line, 0),
            Position::new(pos.line, line_len),
        ));
        ranges.push(Range::new(
            Position::new(0, 0),
            Position::new(text.lines().count().saturating_sub(1) as u32, 0),
        ));
        let mut current: Option<tower_lsp::lsp_types::SelectionRange> = None;
        for r in ranges.into_iter().rev() {
            current = Some(tower_lsp::lsp_types::SelectionRange {
                range: r,
                parent: current.map(Box::new),
            });
        }
        if let Some(sel) = current {
            out.push(sel);
        }
    }
    out
}
