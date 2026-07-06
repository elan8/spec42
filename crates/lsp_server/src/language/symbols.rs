//! Document symbols, definition ranges, folding ranges, and symbol table helpers.
#![allow(deprecated)] // DocumentSymbol/SymbolInformation.deprecated; use tags in future

use crate::common::text_span::to_lsp_range;
#[cfg(test)]
use crate::syntax::ast_util::identification_name;
use language_service::{
    document_symbols as ls_document_symbols, folding_ranges as ls_folding_ranges, OutlineSymbol,
};
#[cfg(test)]
use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBody,
    PartUsageBodyElement, RootElement,
};
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::{
    DocumentSymbol, FoldingRange, FoldingRangeKind, Range, SymbolKind, Url,
};

#[cfg(test)]
fn modeled_decl_name(keyword: &str, text: &str, fallback: &str) -> String {
    let t = text.trim().trim_end_matches(';').trim();
    let tokens: Vec<String> = t
        .split_whitespace()
        .map(|s| {
            s.trim_end_matches(';')
                .trim_end_matches(',')
                .trim_end_matches(')')
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect();
    let kw = keyword.trim();
    if let Some(pos) = tokens.iter().position(|tok| tok.eq_ignore_ascii_case(kw)) {
        if pos + 1 < tokens.len() {
            let name = sanitize_identifier(&tokens[pos + 1]);
            if !name.is_empty() && !name.eq_ignore_ascii_case("specializes") {
                return name;
            }
        }
    }
    for tok in &tokens {
        let name = sanitize_identifier(tok);
        if !name.is_empty() {
            return name;
        }
    }
    fallback.to_string()
}

#[cfg(test)]
fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Returns all LSP ranges in `source` where `name` appears as a whole word (word boundaries).
pub fn find_reference_ranges(source: &str, name: &str) -> Vec<Range> {
    use crate::common::text_span::to_lsp_range;

    language_service::find_reference_ranges(source, name)
        .into_iter()
        .map(to_lsp_range)
        .collect()
}

fn outline_kind_to_lsp(kind: &str) -> SymbolKind {
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
    let children = symbol.children.into_iter().map(map_outline_symbol).collect::<Vec<_>>();
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
pub fn collect_document_symbols(root: &RootNamespace) -> Vec<DocumentSymbol> {
    ls_document_symbols(root)
        .into_iter()
        .map(map_outline_symbol)
        .collect()
}

/// Collects folding ranges from the AST.
pub fn collect_folding_ranges(root: &RootNamespace) -> Vec<FoldingRange> {
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

/// Workspace-wide symbol entry: one definable name with location and semantic info.
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub uri: Url,
    pub range: Range,
    pub kind: SymbolKind,
    pub container_name: Option<String>,
    pub detail: Option<String>,
    pub description: Option<String>,
    /// One-line signature for hover code block (e.g. "part def Vehicle : Car;").
    pub signature: Option<String>,
}

/// Collects a flat list of symbol entries from a parsed document for the symbol table.
#[cfg(test)]
pub fn collect_symbol_entries(_root: &RootNamespace, _uri: &Url) -> Vec<SymbolEntry> {
    vec![]
}


/// Collects all named elements from the document for hover/completion: (name, short_description).
#[cfg(test)]
pub fn collect_named_elements(root: &RootNamespace) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for node in &root.elements {
        let (name, elements) = match &node.value {
            RootElement::Package(p) => {
                let name = identification_name(&p.identification);
                let elements = match &p.body {
                    PackageBody::Brace { elements } => elements,
                    _ => continue,
                };
                (name, elements)
            }
            RootElement::Namespace(n) => {
                let name = identification_name(&n.identification);
                let elements = match &n.body {
                    PackageBody::Brace { elements } => elements,
                    _ => continue,
                };
                (name, elements)
            }
            RootElement::LibraryPackage(lp) => {
                let name = identification_name(&lp.identification);
                let elements = match &lp.body {
                    PackageBody::Brace { elements } => elements,
                    _ => continue,
                };
                (name, elements)
            }
            RootElement::Import(_) => continue,
        };
        if !name.is_empty() {
            out.push((name.clone(), format!("package '{}'", name)));
        }
        for el in elements {
            collect_named_from_element(el, &mut out);
        }
    }
    out
}

#[cfg(test)]
fn collect_named_from_element(
    node: &sysml_v2_parser::Node<PackageBodyElement>,
    out: &mut Vec<(String, String)>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("package '{}'", name)));
            }
            if let PackageBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_named_from_element(child, out);
                }
            }
        }
        PBE::PartDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("part def '{}'", name)));
            }
            if let PartDefBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_named_from_part_def_body(child, out);
                }
            }
        }
        PBE::PartUsage(p) => {
            out.push((p.name.clone(), format!("part usage '{}'", p.name)));
            if let PartUsageBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_named_from_part_usage_body(child, out);
                }
            }
        }
        PBE::PortDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("port def '{}'", name)));
            }
        }
        PBE::InterfaceDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("interface def '{}'", name)));
            }
        }
        PBE::AttributeDef(p) => out.push((p.name.clone(), format!("attribute def '{}'", p.name))),
        PBE::FeatureDecl(p) => {
            let name = modeled_decl_name(&p.keyword, &p.text, "_feature");
            if !name.is_empty() {
                out.push((name.clone(), format!("feature decl '{}'", name)));
            }
        }
        PBE::ClassifierDecl(p) => {
            let name = modeled_decl_name(&p.keyword, &p.text, "_classifier");
            if !name.is_empty() {
                out.push((name.clone(), format!("classifier decl '{}'", name)));
            }
        }
        PBE::ActionDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("action def '{}'", name)));
            }
        }
        PBE::ActionUsage(p) => out.push((p.name.clone(), format!("action usage '{}'", p.name))),
        PBE::ViewDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("view def '{}'", name)));
            }
        }
        PBE::ViewpointDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("viewpoint def '{}'", name)));
            }
        }
        PBE::RenderingDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), format!("rendering def '{}'", name)));
            }
        }
        PBE::ViewUsage(p) => out.push((p.name.clone(), format!("view usage '{}'", p.name))),
        PBE::ViewpointUsage(p) => {
            out.push((p.name.clone(), format!("viewpoint usage '{}'", p.name)))
        }
        PBE::RenderingUsage(p) => {
            out.push((p.name.clone(), format!("rendering usage '{}'", p.name)))
        }
        PBE::Import(_) | PBE::AliasDef(_) => {}
        _ => {}
    }
}

#[cfg(test)]
fn collect_named_from_part_def_body(
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(String, String)>,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => out.push((n.name.clone(), format!("attribute def '{}'", n.name))),
        PDBE::PortUsage(n) => out.push((n.name.clone(), format!("port usage '{}'", n.name))),
        _ => {}
    }
}

#[cfg(test)]
fn collect_named_from_part_usage_body(
    node: &sysml_v2_parser::Node<PartUsageBodyElement>,
    out: &mut Vec<(String, String)>,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => out.push((n.name.clone(), format!("attribute '{}'", n.name))),
        PUBE::PartUsage(n) => {
            out.push((n.name.clone(), format!("part usage '{}'", n.name)));
            if let PartUsageBody::Brace { elements } = &n.body {
                for child in elements {
                    collect_named_from_part_usage_body(child, out);
                }
            }
        }
        PUBE::PortUsage(n) => out.push((n.name.clone(), format!("port '{}'", n.name))),
        PUBE::Ref(n) => out.push((n.value.name.clone(), format!("ref '{}'", n.value.name))),
        _ => {}
    }
}
