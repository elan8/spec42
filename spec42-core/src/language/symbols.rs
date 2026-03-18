//! Document symbols, definition ranges, folding ranges, and symbol table helpers.
#![allow(deprecated)] // DocumentSymbol/SymbolInformation.deprecated; use tags in future

use crate::ast_util::{identification_name, span_to_range};
use sysml_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBody,
    PartUsageBodyElement, PortDefBody, PortDefBodyElement, RootElement,
};
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::{
    DocumentSymbol, FoldingRange, FoldingRangeKind, Location, Position, Range, SymbolInformation,
    SymbolKind, Url,
};

/// Collects for each defined name in the document the LSP range of its definition.
pub fn collect_definition_ranges(root: &RootNamespace) -> Vec<(String, Range)> {
    let mut out = Vec::new();
    for node in &root.elements {
        let (name, range, elements) = match &node.value {
            RootElement::Package(p) => {
                let name = identification_name(&p.identification);
                let range = span_to_range(&p.span);
                let elements = match &p.body {
                    PackageBody::Brace { elements } => elements,
                    _ => continue,
                };
                (name, range, elements)
            }
            RootElement::Namespace(n) => {
                let name = identification_name(&n.identification);
                let range = span_to_range(&n.span);
                let elements = match &n.body {
                    PackageBody::Brace { elements } => elements,
                    _ => continue,
                };
                (name, range, elements)
            }
            RootElement::LibraryPackage(lp) => {
                let name = identification_name(&lp.identification);
                let range = span_to_range(&lp.span);
                let elements = match &lp.body {
                    PackageBody::Brace { elements } => elements,
                    _ => continue,
                };
                (name, range, elements)
            }
            RootElement::Import(_) => continue,
        };
        if !name.is_empty() {
            out.push((name, range));
        }
        for el in elements {
            collect_definition_ranges_from_element(el, &mut out);
        }
    }
    out
}

fn collect_definition_ranges_from_element(
    node: &sysml_parser::Node<PackageBodyElement>,
    out: &mut Vec<(String, Range)>,
) {
    use sysml_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name.clone(), span_to_range(&p.span)));
            }
            if let PackageBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_definition_ranges_from_element(child, out);
                }
            }
        }
        PBE::PartDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
            if let PartDefBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_definition_range_part_def_body(child, out);
                }
            }
        }
        PBE::PartUsage(p) => {
            out.push((p.name.clone(), span_to_range(&p.span)));
            if let PartUsageBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_definition_range_part_usage_body(child, out);
                }
            }
        }
        PBE::PortDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
            if let PortDefBody::Brace { elements } = &p.body {
                for child in elements {
                    collect_definition_range_port_def_body(child, out);
                }
            }
        }
        PBE::InterfaceDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
        }
        PBE::AttributeDef(p) => {
            out.push((p.name.clone(), span_to_range(&p.span)));
        }
        PBE::ActionDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
        }
        PBE::ActionUsage(p) => {
            out.push((p.name.clone(), span_to_range(&p.span)));
        }
        PBE::ViewDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
        }
        PBE::ViewpointDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
        }
        PBE::RenderingDef(p) => {
            let name = identification_name(&p.identification);
            if !name.is_empty() {
                out.push((name, span_to_range(&p.span)));
            }
        }
        PBE::ViewUsage(p) => out.push((p.name.clone(), span_to_range(&p.span))),
        PBE::ViewpointUsage(p) => out.push((p.name.clone(), span_to_range(&p.span))),
        PBE::RenderingUsage(p) => out.push((p.name.clone(), span_to_range(&p.span))),
        PBE::Import(_) | PBE::AliasDef(_) => {}
        _ => {}
    }
}

fn collect_definition_range_part_def_body(
    node: &sysml_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(String, Range)>,
) {
    use sysml_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => out.push((n.name.clone(), span_to_range(&n.span))),
        PDBE::PortUsage(n) => out.push((n.name.clone(), span_to_range(&n.span))),
        _ => {}
    }
}

fn collect_definition_range_part_usage_body(
    node: &sysml_parser::Node<PartUsageBodyElement>,
    out: &mut Vec<(String, Range)>,
) {
    use sysml_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => out.push((n.name.clone(), span_to_range(&n.span))),
        PUBE::PartUsage(n) => {
            out.push((n.name.clone(), span_to_range(&n.span)));
            if let PartUsageBody::Brace { elements } = &n.body {
                for child in elements {
                    collect_definition_range_part_usage_body(child, out);
                }
            }
        }
        PUBE::PortUsage(n) => out.push((n.name.clone(), span_to_range(&n.span))),
        _ => {}
    }
}

fn collect_definition_range_port_def_body(
    node: &sysml_parser::Node<PortDefBodyElement>,
    out: &mut Vec<(String, Range)>,
) {
    use sysml_parser::ast::PortDefBodyElement as PDBE;
    match &node.value {
        PDBE::PortUsage(n) => out.push((n.name.clone(), span_to_range(&n.span))),
        _ => {}
    }
}

/// Returns all LSP ranges in `source` where `name` appears as a whole word (word boundaries).
pub fn find_reference_ranges(source: &str, name: &str) -> Vec<Range> {
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '-'
    }
    if name.is_empty() {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    for (line_no, line) in source.lines().enumerate() {
        let line_utf8 = line;
        let mut search_start = 0;
        while let Some(off) = line_utf8[search_start..].find(name) {
            let start = search_start + off;
            let end = start + name.len();
            let before_ok = start == 0
                || !line_utf8[..start]
                    .chars()
                    .next_back()
                    .is_some_and(is_ident_char);
            let after_ok = end >= line_utf8.len()
                || !line_utf8[end..].chars().next().is_some_and(is_ident_char);
            if before_ok && after_ok {
                let start_char = line_utf8[..start].chars().count() as u32;
                let end_char = start_char + name.chars().count() as u32;
                ranges.push(Range::new(
                    Position::new(line_no as u32, start_char),
                    Position::new(line_no as u32, end_char),
                ));
            }
            search_start = end;
        }
    }
    ranges
}

/// Collects document symbols (outline) from the AST.
pub fn collect_document_symbols(root: &RootNamespace) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    for node in &root.elements {
        let sym = match &node.value {
            RootElement::Package(p) => {
                let name = identification_name(&p.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = span_to_range(&p.span);
                let children = match &p.body {
                    PackageBody::Brace { elements } => elements
                        .iter()
                        .filter_map(document_symbol_from_element)
                        .collect(),
                    _ => vec![],
                };
                Some(DocumentSymbol {
                    name,
                    detail: Some("package".to_string()),
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: Some(children),
                })
            }
            RootElement::Namespace(n) => {
                let name = identification_name(&n.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = span_to_range(&n.span);
                let children = match &n.body {
                    PackageBody::Brace { elements } => elements
                        .iter()
                        .filter_map(document_symbol_from_element)
                        .collect(),
                    _ => vec![],
                };
                Some(DocumentSymbol {
                    name,
                    detail: Some("namespace".to_string()),
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: Some(children),
                })
            }
            RootElement::LibraryPackage(lp) => {
                let name = identification_name(&lp.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = span_to_range(&lp.span);
                let children = match &lp.body {
                    PackageBody::Brace { elements } => elements
                        .iter()
                        .filter_map(document_symbol_from_element)
                        .collect(),
                    _ => vec![],
                };
                Some(DocumentSymbol {
                    name,
                    detail: Some("library package".to_string()),
                    kind: SymbolKind::MODULE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: Some(children),
                })
            }
            RootElement::Import(_) => None,
        };
        if let Some(s) = sym {
            out.push(s);
        }
    }
    out
}

/// Collects folding ranges from the AST. This reuses the document-symbol outline ranges and
/// produces one folding range per symbol whose extent spans multiple lines.
pub fn collect_folding_ranges(root: &RootNamespace) -> Vec<FoldingRange> {
    let symbols = collect_document_symbols(root);
    let mut out = Vec::new();

    fn push_symbol(symbol: &DocumentSymbol, out: &mut Vec<FoldingRange>) {
        let start = symbol.range.start.line;
        let end = symbol.range.end.line;
        if end > start {
            out.push(FoldingRange {
                start_line: start,
                start_character: None,
                end_line: end,
                end_character: None,
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        }
        if let Some(children) = symbol.children.as_ref() {
            for c in children {
                push_symbol(c, out);
            }
        }
    }

    for s in &symbols {
        push_symbol(s, &mut out);
    }

    out
}

/// Flattens document symbols into workspace symbol list with the given file URI.
#[allow(dead_code)]
pub fn document_symbols_to_workspace_symbols(
    uri: &Url,
    symbols: &[DocumentSymbol],
) -> Vec<SymbolInformation> {
    let mut out = Vec::new();
    fn flatten(
        uri: &Url,
        symbols: &[DocumentSymbol],
        container: Option<&str>,
        out: &mut Vec<SymbolInformation>,
    ) {
        for s in symbols {
            out.push(SymbolInformation {
                name: s.name.clone(),
                kind: s.kind,
                tags: s.tags.clone(),
                deprecated: s.deprecated,
                location: Location {
                    uri: uri.clone(),
                    range: s.range,
                },
                container_name: container.map(String::from),
            });
            if let Some(ref children) = s.children {
                flatten(uri, children, Some(&s.name), out);
            }
        }
    }
    flatten(uri, symbols, None, &mut out);
    out
}

/// Formats multiplicity for display (e.g. "[1..*]", "[*]", "[3]").
#[allow(dead_code)]
pub(crate) fn format_multiplicity(s: Option<&str>) -> String {
    s.unwrap_or("[*]").to_string()
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
#[allow(dead_code)]
pub fn collect_symbol_entries(_root: &RootNamespace, _uri: &Url) -> Vec<SymbolEntry> {
    vec![]
}

#[allow(dead_code)]
fn _symbol_entries_stub() {}

#[allow(dead_code)]
fn _symbol_entries_from_member_removed(
    _member: &std::marker::PhantomData<()>,
    _uri: &Url,
    _container: Option<&str>,
    _out: &mut Vec<SymbolEntry>,
) {
}

fn document_symbol_from_element(
    node: &sysml_parser::Node<PackageBodyElement>,
) -> Option<DocumentSymbol> {
    use sysml_parser::ast::PackageBodyElement as PBE;
    let range = span_to_range(&node.span);
    match &node.value {
        PBE::Package(p) => {
            let name = identification_name(&p.identification);
            let name = if name.is_empty() {
                "(top level)".to_string()
            } else {
                name
            };
            let children = match &p.body {
                PackageBody::Brace { elements } => elements
                    .iter()
                    .filter_map(document_symbol_from_element)
                    .collect(),
                _ => vec![],
            };
            Some(DocumentSymbol {
                name,
                detail: Some("package".to_string()),
                kind: SymbolKind::MODULE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
            })
        }
        PBE::PartDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            let children = match &p.body {
                PartDefBody::Brace { elements } => document_symbols_from_part_def_body(elements),
                _ => vec![],
            };
            Some(DocumentSymbol {
                name,
                detail: Some("part def".to_string()),
                kind: SymbolKind::CLASS,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
            })
        }
        PBE::PartUsage(p) => {
            let children = match &p.body {
                PartUsageBody::Brace { elements } => {
                    document_symbols_from_part_usage_body(elements)
                }
                _ => vec![],
            };
            Some(DocumentSymbol {
                name: p.name.clone(),
                detail: Some("part".to_string()),
                kind: SymbolKind::VARIABLE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
            })
        }
        PBE::PortDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            let children = match &p.body {
                PortDefBody::Brace { elements } => document_symbols_from_port_def_body(elements),
                _ => vec![],
            };
            Some(DocumentSymbol {
                name,
                detail: Some("port def".to_string()),
                kind: SymbolKind::INTERFACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
            })
        }
        PBE::InterfaceDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(DocumentSymbol {
                name,
                detail: Some("interface".to_string()),
                kind: SymbolKind::INTERFACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            })
        }
        PBE::AttributeDef(p) => Some(DocumentSymbol {
            name: p.name.clone(),
            detail: Some("attribute def".to_string()),
            kind: SymbolKind::PROPERTY,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }),
        PBE::ActionDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(DocumentSymbol {
                name,
                detail: Some("action def".to_string()),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            })
        }
        PBE::ActionUsage(p) => Some(DocumentSymbol {
            name: p.name.clone(),
            detail: Some("action".to_string()),
            kind: SymbolKind::FUNCTION,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }),
        PBE::ViewDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(DocumentSymbol {
                name,
                detail: Some("view def".to_string()),
                kind: SymbolKind::NAMESPACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            })
        }
        PBE::ViewpointDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(DocumentSymbol {
                name,
                detail: Some("viewpoint def".to_string()),
                kind: SymbolKind::NAMESPACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            })
        }
        PBE::RenderingDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(DocumentSymbol {
                name,
                detail: Some("rendering def".to_string()),
                kind: SymbolKind::NAMESPACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            })
        }
        PBE::ViewUsage(p) => Some(DocumentSymbol {
            name: p.name.clone(),
            detail: Some("view".to_string()),
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }),
        PBE::ViewpointUsage(p) => Some(DocumentSymbol {
            name: p.name.clone(),
            detail: Some("viewpoint".to_string()),
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }),
        PBE::RenderingUsage(p) => Some(DocumentSymbol {
            name: p.name.clone(),
            detail: Some("rendering".to_string()),
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }),
        PBE::Import(_) | PBE::AliasDef(_) => None,
        _ => None,
    }
}

fn document_symbols_from_part_def_body(
    elements: &[sysml_parser::Node<PartDefBodyElement>],
) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_parser::ast::PartDefBodyElement as PDBE;
        let range = span_to_range(&node.span);
        match &node.value {
            PDBE::AttributeDef(n) => out.push(DocumentSymbol {
                name: n.name.clone(),
                detail: Some("attribute def".to_string()),
                kind: SymbolKind::PROPERTY,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }),
            PDBE::PortUsage(n) => out.push(DocumentSymbol {
                name: n.name.clone(),
                detail: Some("port".to_string()),
                kind: SymbolKind::INTERFACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }),
            _ => {}
        }
    }
    out
}

fn document_symbols_from_part_usage_body(
    elements: &[sysml_parser::Node<PartUsageBodyElement>],
) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_parser::ast::PartUsageBodyElement as PUBE;
        let range = span_to_range(&node.span);
        match &node.value {
            PUBE::AttributeUsage(n) => out.push(DocumentSymbol {
                name: n.name.clone(),
                detail: Some("attribute".to_string()),
                kind: SymbolKind::PROPERTY,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }),
            PUBE::PartUsage(n) => {
                let children = match &n.body {
                    PartUsageBody::Brace { elements } => {
                        document_symbols_from_part_usage_body(elements)
                    }
                    _ => vec![],
                };
                out.push(DocumentSymbol {
                    name: n.name.clone(),
                    detail: Some("part".to_string()),
                    kind: SymbolKind::VARIABLE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: if children.is_empty() {
                        None
                    } else {
                        Some(children)
                    },
                });
            }
            PUBE::PortUsage(n) => out.push(DocumentSymbol {
                name: n.name.clone(),
                detail: Some("port".to_string()),
                kind: SymbolKind::INTERFACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }),
            _ => {}
        }
    }
    out
}

fn document_symbols_from_port_def_body(
    elements: &[sysml_parser::Node<PortDefBodyElement>],
) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_parser::ast::PortDefBodyElement as PDBE;
        let range = span_to_range(&node.span);
        match &node.value {
            PDBE::PortUsage(n) => out.push(DocumentSymbol {
                name: n.name.clone(),
                detail: Some("port".to_string()),
                kind: SymbolKind::INTERFACE,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }),
            _ => {}
        }
    }
    out
}

/// Collects all named elements from the document for hover/completion: (name, short_description).
#[allow(dead_code)]
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

fn collect_named_from_element(
    node: &sysml_parser::Node<PackageBodyElement>,
    out: &mut Vec<(String, String)>,
) {
    use sysml_parser::ast::PackageBodyElement as PBE;
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

fn collect_named_from_part_def_body(
    node: &sysml_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(String, String)>,
) {
    use sysml_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => out.push((n.name.clone(), format!("attribute def '{}'", n.name))),
        PDBE::PortUsage(n) => out.push((n.name.clone(), format!("port usage '{}'", n.name))),
        _ => {}
    }
}

fn collect_named_from_part_usage_body(
    node: &sysml_parser::Node<PartUsageBodyElement>,
    out: &mut Vec<(String, String)>,
) {
    use sysml_parser::ast::PartUsageBodyElement as PUBE;
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
        _ => {}
    }
}
