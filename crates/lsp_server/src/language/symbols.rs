//! Document symbols, definition ranges, folding ranges, and symbol table helpers.
#![allow(deprecated)] // DocumentSymbol/SymbolInformation.deprecated; use tags in future

use crate::common::text_span::to_lsp_range;
use language_service::{
    document_symbols as ls_document_symbols, folding_ranges as ls_folding_ranges, OutlineSymbol,
};
use sysml_query::resolved_slice::ElementKind;
use sysml_query::syntax::{ParsedSource, SyntaxOutlineKind};
use tower_lsp::lsp_types::{DocumentSymbol, FoldingRange, FoldingRangeKind, SymbolKind};

/// The one label a host prints for an LSP [`SymbolKind`], the inverse of the table above.
///
/// It lives beside `outline_kind_to_lsp` so the crate's kind vocabulary has one home: the library
/// browser labels an outline kind by composing the two, rather than keeping a third table.
pub(crate) fn symbol_kind_label(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::FILE => "file",
        SymbolKind::MODULE => "module",
        SymbolKind::NAMESPACE => "namespace",
        SymbolKind::PACKAGE => "package",
        SymbolKind::CLASS => "class",
        SymbolKind::METHOD => "method",
        SymbolKind::PROPERTY => "property",
        SymbolKind::FIELD => "field",
        SymbolKind::CONSTRUCTOR => "constructor",
        SymbolKind::ENUM => "enum",
        SymbolKind::INTERFACE => "interface",
        SymbolKind::FUNCTION => "function",
        SymbolKind::VARIABLE => "variable",
        SymbolKind::CONSTANT => "constant",
        SymbolKind::STRING => "string",
        SymbolKind::NUMBER => "number",
        SymbolKind::BOOLEAN => "boolean",
        SymbolKind::ARRAY => "array",
        SymbolKind::OBJECT => "object",
        SymbolKind::KEY => "key",
        SymbolKind::NULL => "null",
        SymbolKind::ENUM_MEMBER => "enumMember",
        SymbolKind::STRUCT => "struct",
        SymbolKind::EVENT => "event",
        SymbolKind::OPERATOR => "operator",
        SymbolKind::TYPE_PARAMETER => "typeParameter",
        _ => "symbol",
    }
}

/// The single outline-kind -> LSP [`SymbolKind`] table for this crate.
///
/// It matches on the published [`SyntaxOutlineKind`] rather than on the authored keyword, so a
/// new declaration form the grammar publishes is a compile error here instead of silently
/// classifying as a variable; document symbols and workspace symbols share this one table.
fn outline_kind_to_lsp(kind: SyntaxOutlineKind) -> SymbolKind {
    match kind {
        SyntaxOutlineKind::Package
        | SyntaxOutlineKind::Namespace
        | SyntaxOutlineKind::LibraryPackage => SymbolKind::MODULE,
        SyntaxOutlineKind::PartDef | SyntaxOutlineKind::ClassifierDecl => SymbolKind::CLASS,
        SyntaxOutlineKind::PortDef
        | SyntaxOutlineKind::InterfaceDef
        | SyntaxOutlineKind::PortUsage => SymbolKind::INTERFACE,
        SyntaxOutlineKind::AttributeDef
        | SyntaxOutlineKind::AttributeUsage
        | SyntaxOutlineKind::FeatureDecl
        | SyntaxOutlineKind::Ref => SymbolKind::PROPERTY,
        SyntaxOutlineKind::ActionDef => SymbolKind::FUNCTION,
        SyntaxOutlineKind::PartUsage => SymbolKind::OBJECT,
        SyntaxOutlineKind::ActionUsage => SymbolKind::EVENT,
        SyntaxOutlineKind::ViewDef
        | SyntaxOutlineKind::ViewpointDef
        | SyntaxOutlineKind::RenderingDef
        | SyntaxOutlineKind::ViewUsage
        | SyntaxOutlineKind::ViewpointUsage
        | SyntaxOutlineKind::RenderingUsage => SymbolKind::NAMESPACE,
    }
}

/// The LSP [`SymbolKind`] for a published semantic element kind.
///
/// Workspace symbols and the library browser carry an [`ElementKind`] metaclass name, not an
/// outline keyword: they used to be passed through the outline table, where no metaclass name
/// matched and every symbol arrived as a variable. Classifying the parsed kind is the fix.
fn element_kind_to_lsp(kind: Option<ElementKind>) -> SymbolKind {
    let Some(kind) = kind else {
        return SymbolKind::VARIABLE;
    };
    match kind {
        ElementKind::Namespace | ElementKind::Package | ElementKind::LibraryPackage => {
            SymbolKind::MODULE
        }
        ElementKind::PortDefinition | ElementKind::InterfaceDefinition | ElementKind::PortUsage => {
            SymbolKind::INTERFACE
        }
        ElementKind::AttributeDefinition
        | ElementKind::AttributeUsage
        | ElementKind::ReferenceUsage => SymbolKind::PROPERTY,
        ElementKind::ActionDefinition | ElementKind::CalculationDefinition => SymbolKind::FUNCTION,
        ElementKind::ActionUsage | ElementKind::CalculationUsage => SymbolKind::EVENT,
        ElementKind::PartUsage | ElementKind::ItemUsage => SymbolKind::OBJECT,
        ElementKind::ViewDefinition
        | ElementKind::ViewpointDefinition
        | ElementKind::RenderingDefinition
        | ElementKind::ViewUsage
        | ElementKind::ViewpointUsage
        | ElementKind::RenderingUsage => SymbolKind::NAMESPACE,
        other if other.as_str().ends_with("Definition") => SymbolKind::CLASS,
        _ => SymbolKind::VARIABLE,
    }
}

/// The LSP [`SymbolKind`] for a metaclass name a published symbol entry carries.
pub(crate) fn element_kind_label_to_lsp(label: Option<&str>) -> SymbolKind {
    element_kind_to_lsp(label.and_then(ElementKind::parse))
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
        detail: Some(symbol.kind.keyword().to_string()),
        kind: outline_kind_to_lsp(symbol.kind),
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
            out.push((
                node.name.clone(),
                format!("{} '{}'", node.kind.keyword(), node.name),
            ));
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
