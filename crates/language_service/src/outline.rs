//! Editor outline DTOs for the syntax outline the parser authority publishes.
//!
//! The traversal lives behind the syntax service; this maps its nodes onto the DTO shape hosts
//! consume. The two are deliberately separate types: the authority names the authored declaration
//! keyword, and nothing in it should have to know that an editor wants a `TextRange`.

use sysml_query::resolved_slice::{TextPosition, TextRange};
use sysml_query::syntax::{
    ParsedSource, SyntaxFoldingKind, SyntaxFoldingRegion, SyntaxOutlineNode, SyntaxRange,
};

use crate::dto::{FoldingRangeDto, FoldingRangeKindDto, OutlineSymbol};

fn text_range(range: SyntaxRange) -> TextRange {
    TextRange::new(
        TextPosition::new(range.start_line, range.start_character),
        TextPosition::new(range.end_line, range.end_character),
    )
}

fn outline_symbol(node: SyntaxOutlineNode) -> OutlineSymbol {
    OutlineSymbol {
        name: node.name,
        kind: node.kind,
        range: text_range(node.range),
        selection_range: text_range(node.selection_range),
        children: node.children.into_iter().map(outline_symbol).collect(),
    }
}

/// Document outline symbols for an already-parsed document.
pub fn document_symbols(document: &ParsedSource) -> Vec<OutlineSymbol> {
    document.outline().into_iter().map(outline_symbol).collect()
}

/// Folding ranges for an already-parsed document.
pub fn folding_ranges(document: &ParsedSource) -> Vec<FoldingRangeDto> {
    document
        .folding_regions()
        .into_iter()
        .map(|region: SyntaxFoldingRegion| FoldingRangeDto {
            start_line: region.start_line,
            end_line: region.end_line,
            kind: region.kind.map(|kind| match kind {
                SyntaxFoldingKind::Region => FoldingRangeKindDto::Region,
                SyntaxFoldingKind::Comment => FoldingRangeKindDto::Comment,
                SyntaxFoldingKind::Imports => FoldingRangeKindDto::Imports,
            }),
        })
        .collect()
}
