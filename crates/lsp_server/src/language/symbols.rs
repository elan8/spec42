//! Document symbols, definition ranges, folding ranges, and symbol table helpers.
#![allow(deprecated)] // DocumentSymbol/SymbolInformation.deprecated; use tags in future

use crate::common::text_span::to_lsp_range;
use language_service::{
    document_symbols as ls_document_symbols, folding_ranges as ls_folding_ranges, OutlineSymbol,
};
use sysml_query::syntax::ParsedSource;
use tower_lsp::lsp_types::{DocumentSymbol, FoldingRange, FoldingRangeKind, Range, SymbolKind};

/// Returns all LSP ranges in `source` where `name` appears as a whole word (word boundaries).
pub fn find_reference_ranges(source: &str, name: &str) -> Vec<Range> {
    use crate::common::text_span::to_lsp_range;

    language_service::find_reference_ranges(source, name)
        .into_iter()
        .map(to_lsp_range)
        .collect()
}

/// The single `&str` outline-kind -> LSP [`SymbolKind`] table for this crate.
///
/// The strings are the published outline kind vocabulary (`language_service`); document symbols
/// and workspace symbols must classify them identically.
pub(crate) fn outline_kind_to_lsp(kind: &str) -> SymbolKind {
    match kind {
        "package" | "namespace" | "library package" => SymbolKind::MODULE,
        "part def" | "classifier decl" => SymbolKind::CLASS,
        "port def" | "interface" | "port" => SymbolKind::INTERFACE,
        "attribute def" | "attribute" | "feature decl" | "ref" => SymbolKind::PROPERTY,
        "action def" => SymbolKind::FUNCTION,
        "part" => SymbolKind::OBJECT,
        "action" => SymbolKind::EVENT,
        "view def" | "viewpoint def" | "rendering def" | "view" | "viewpoint" | "rendering" => {
            SymbolKind::NAMESPACE
        }
        _ => SymbolKind::VARIABLE,
    }
}

fn map_outline_symbol(symbol: OutlineSymbol) -> DocumentSymbol {
    let range = to_lsp_range(symbol.range);
    let selection_range = to_lsp_range(symbol.selection_range);
    let children = symbol
        .children
        .into_iter()
        .map(map_outline_symbol)
        .collect::<Vec<_>>();
    DocumentSymbol {
        name: symbol.name,
        detail: Some(symbol.kind.clone()),
        kind: outline_kind_to_lsp(&symbol.kind),
        tags: None,
        deprecated: None,
        range,
        selection_range,
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    }
}

/// Collects document symbols (outline) from the AST.
pub fn collect_document_symbols(root: &ParsedSource) -> Vec<DocumentSymbol> {
    ls_document_symbols(root)
        .into_iter()
        .map(map_outline_symbol)
        .collect()
}

/// Collects folding ranges from the AST.
pub fn collect_folding_ranges(root: &ParsedSource) -> Vec<FoldingRange> {
    ls_folding_ranges(root)
        .into_iter()
        .map(|range| FoldingRange {
            start_line: range.start_line,
            start_character: None,
            end_line: range.end_line,
            end_character: None,
            kind: range.kind.map(|kind| match kind {
                language_service::FoldingRangeKindDto::Region => FoldingRangeKind::Region,
                language_service::FoldingRangeKindDto::Imports => FoldingRangeKind::Imports,
                language_service::FoldingRangeKindDto::Comment => FoldingRangeKind::Comment,
            }),
            collapsed_text: None,
        })
        .collect()
}

/// Collects all named elements from the document for hover/completion: (name, short_description).
/// Every named element in the document, flattened, with a short description.
///
/// Built from the published outline rather than a private AST walk: the outline already names
/// each declaration and its authored keyword, which is exactly what this reported.
#[cfg(test)]
pub fn collect_named_elements(document: &ParsedSource) -> Vec<(String, String)> {
    fn push(node: &language_service::OutlineSymbol, out: &mut Vec<(String, String)>) {
        if !node.name.is_empty() {
            out.push((node.name.clone(), format!("{} '{}'", node.kind, node.name)));
        }
        for child in &node.children {
            push(child, out);
        }
    }
    let mut out = Vec::new();
    for node in ls_document_symbols(document) {
        push(&node, &mut out);
    }
    out
}
