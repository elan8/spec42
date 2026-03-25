use tower_lsp::lsp_types::{DocumentLink, Position, Range, Url};

pub(crate) fn collect_document_links(
    text: &str,
    symbol_uri_for_import_name: impl Fn(&str) -> Option<Url>,
) -> Vec<DocumentLink> {
    let mut links = Vec::new();
    for (line_idx, line) in text.lines().enumerate() {
        if let Some(import_idx) = line.find("import ") {
            if let Some(file_idx) = line.find("file://") {
                let target_start = file_idx as u32;
                let target_text = &line[file_idx..].split_whitespace().next().unwrap_or("");
                if let Ok(target) = Url::parse(target_text) {
                    links.push(DocumentLink {
                        range: Range::new(
                            Position::new(line_idx as u32, target_start),
                            Position::new(
                                line_idx as u32,
                                target_start + target_text.chars().count() as u32,
                            ),
                        ),
                        target: Some(target),
                        tooltip: Some("Open import target".to_string()),
                        data: None,
                    });
                }
            } else {
                let import_name = line[(import_idx + "import ".len())..].trim();
                if let Some(uri) = symbol_uri_for_import_name(import_name) {
                    links.push(DocumentLink {
                        range: Range::new(
                            Position::new(line_idx as u32, import_idx as u32),
                            Position::new(line_idx as u32, line.chars().count() as u32),
                        ),
                        target: Some(uri),
                        tooltip: Some("Open imported symbol".to_string()),
                        data: None,
                    });
                }
            }
        }
    }
    links
}

pub(crate) fn selection_ranges_for_positions(
    text: &str,
    positions: &[Position],
    word_at: impl Fn(&str, u32, u32) -> Option<(u32, u32, u32, String)>,
) -> Vec<tower_lsp::lsp_types::SelectionRange> {
    let mut out = Vec::new();
    for pos in positions {
        let mut ranges = Vec::<Range>::new();
        if let Some((line, start, end, _)) = word_at(text, pos.line, pos.character) {
            ranges.push(Range::new(
                Position::new(line, start),
                Position::new(line, end),
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
