use semantic_core::{TextPosition, TextRange};
use tower_lsp::lsp_types::{Position, Range};

pub fn to_core_position(position: Position) -> TextPosition {
    TextPosition::new(position.line, position.character)
}

pub fn to_core_range(range: Range) -> TextRange {
    TextRange::new(to_core_position(range.start), to_core_position(range.end))
}

pub fn to_lsp_range(range: TextRange) -> Range {
    Range::new(
        Position::new(range.start.line, range.start.character),
        Position::new(range.end.line, range.end.character),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_position_conversion() {
        let lsp = Position::new(12, 34);
        let core = to_core_position(lsp);
        assert_eq!(core.line, 12);
        assert_eq!(core.character, 34);
    }

    #[test]
    fn roundtrip_range_conversion() {
        let lsp = Range::new(Position::new(1, 2), Position::new(3, 4));
        let core = to_core_range(lsp);
        let back = to_lsp_range(core);
        assert_eq!(back.start.line, 1);
        assert_eq!(back.start.character, 2);
        assert_eq!(back.end.line, 3);
        assert_eq!(back.end.character, 4);
    }

    #[test]
    fn no_local_converter_duplicate_in_references_resolver() {
        let path = format!(
            "{}/src/lsp_runtime/references_resolver.rs",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = std::fs::read_to_string(path).expect("read references_resolver");
        assert!(
            !content.contains("fn to_core_range("),
            "local converter duplicate found in references_resolver"
        );
    }
}
