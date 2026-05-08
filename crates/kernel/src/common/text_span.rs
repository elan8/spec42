use semantic_core::{TextPosition, TextRange};
use tower_lsp::lsp_types::{Position, Range};

pub fn to_core_position(position: Position) -> TextPosition {
    TextPosition::new(position.line, position.character)
}

pub fn to_core_range(range: Range) -> TextRange {
    TextRange::new(
        to_core_position(range.start),
        to_core_position(range.end),
    )
}

pub fn to_lsp_range(range: TextRange) -> Range {
    Range::new(
        Position::new(range.start.line, range.start.character),
        Position::new(range.end.line, range.end.character),
    )
}
