//! Document outline and folding regions from the parsed document.
//!
//! Moved here from `language_service` because it walks the AST. It publishes what each node *is*
//! -- the authored declaration keyword -- and leaves the mapping to an editor symbol category to
//! the host adapter that knows about editors.

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBody,
    PartUsageBodyElement, PortDefBody, PortDefBodyElement, RootElement,
};

use super::token_util::{
    identification_name, qualified_identification_name, span_to_source_range as span_to_range,
};
use super::{SyntaxFoldingKind, SyntaxFoldingRegion, SyntaxOutlineNode};

pub(super) fn document_outline(
    document: &sysml_v2_parser::ParsedDocument,
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in &document.elements {
        let sym = match &node.value {
            RootElement::Package(p) => {
                let name = qualified_identification_name(document, &p.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = span_to_range(&p.span);
                let children = match &p.body {
                    PackageBody::Brace { elements, .. } => elements
                        .iter()
                        .filter_map(|element| outline_symbol_from_element(document, element))
                        .collect(),
                    _ => vec![],
                };
                Some(SyntaxOutlineNode {
                    name,
                    kind: "package".to_string(),
                    range,
                    selection_range: range,
                    children,
                })
            }
            RootElement::Namespace(n) => {
                let name = qualified_identification_name(document, &n.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = span_to_range(&n.span);
                let children = match &n.body {
                    PackageBody::Brace { elements, .. } => elements
                        .iter()
                        .filter_map(|element| outline_symbol_from_element(document, element))
                        .collect(),
                    _ => vec![],
                };
                Some(SyntaxOutlineNode {
                    name,
                    kind: "namespace".to_string(),
                    range,
                    selection_range: range,
                    children,
                })
            }
            RootElement::LibraryPackage(lp) => {
                let name = qualified_identification_name(document, &lp.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = span_to_range(&lp.span);
                let children = match &lp.body {
                    PackageBody::Brace { elements, .. } => elements
                        .iter()
                        .filter_map(|element| outline_symbol_from_element(document, element))
                        .collect(),
                    _ => vec![],
                };
                Some(SyntaxOutlineNode {
                    name,
                    kind: "library package".to_string(),
                    range,
                    selection_range: range,
                    children,
                })
            }
            RootElement::Import(_) => None,
            RootElement::Member(member) => outline_symbol_from_element(document, member),
        };
        if let Some(s) = sym {
            out.push(s);
        }
    }
    normalize_outline_symbols(&mut out);
    out
}

fn normalize_outline_symbols(symbols: &mut [SyntaxOutlineNode]) {
    for symbol in symbols {
        if symbol.name.trim().is_empty() {
            let fallback = if symbol.kind.trim().is_empty() {
                "(anonymous)".to_string()
            } else {
                format!("(anonymous {})", symbol.kind.trim())
            };
            symbol.name = fallback;
        }
        if !symbol.children.is_empty() {
            normalize_outline_symbols(&mut symbol.children);
        }
    }
}

/// Collects folding ranges from the AST. This reuses the document-symbol outline ranges and
/// produces one folding range per symbol whose extent spans multiple lines.
pub(super) fn folding_regions(
    document: &sysml_v2_parser::ParsedDocument,
) -> Vec<SyntaxFoldingRegion> {
    let symbols = document_outline(document);
    let mut out = Vec::new();

    fn push_symbol(symbol: &SyntaxOutlineNode, out: &mut Vec<SyntaxFoldingRegion>) {
        let start = symbol.range.start_line;
        let end = symbol.range.end_line;
        if end > start {
            out.push(SyntaxFoldingRegion {
                start_line: start,
                end_line: end,
                kind: Some(SyntaxFoldingKind::Region),
            });
        }
        for c in &symbol.children {
            push_symbol(c, out);
        }
    }

    for s in &symbols {
        push_symbol(s, &mut out);
    }

    out
}

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

fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

fn outline_symbol_from_element(
    document: &sysml_v2_parser::ParsedDocument,
    node: &sysml_v2_parser::Node<PackageBodyElement>,
) -> Option<SyntaxOutlineNode> {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    let range = span_to_range(&node.span);
    match &node.value {
        PBE::Package(p) => {
            let name = qualified_identification_name(document, &p.identification);
            let name = if name.is_empty() {
                "(top level)".to_string()
            } else {
                name
            };
            let children = match &p.body {
                PackageBody::Brace { elements, .. } => elements
                    .iter()
                    .filter_map(|element| outline_symbol_from_element(document, element))
                    .collect(),
                _ => vec![],
            };
            Some(SyntaxOutlineNode {
                name,
                kind: "package".to_string(),
                range,
                selection_range: range,
                children,
            })
        }
        PBE::PartDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            let children = match &p.body {
                PartDefBody::Brace { elements, .. } => outline_symbols_from_part_def_body(elements),
                _ => vec![],
            };
            Some(SyntaxOutlineNode {
                name,
                kind: "part def".to_string(),
                range,
                selection_range: range,
                children,
            })
        }
        PBE::PartUsage(p) => {
            let children = match &p.body {
                PartUsageBody::Brace { elements, .. } => {
                    outline_symbols_from_part_usage_body(elements)
                }
                _ => vec![],
            };
            Some(SyntaxOutlineNode {
                name: p.name.clone(),
                kind: "part".to_string(),
                range,
                selection_range: range,
                children,
            })
        }
        PBE::PortDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            let children = match &p.body {
                PortDefBody::Brace { elements, .. } => outline_symbols_from_port_def_body(elements),
                _ => vec![],
            };
            Some(SyntaxOutlineNode {
                name,
                kind: "port def".to_string(),
                range,
                selection_range: range,
                children,
            })
        }
        PBE::InterfaceDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "interface".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::AttributeDef(p) => Some(SyntaxOutlineNode {
            name: p.name.clone(),
            kind: "attribute def".to_string(),
            range,
            selection_range: range,
            children: vec![],
        }),
        // `feature myFeature : BaseFeature;` and `class VehicleClass;` arrive as typed nodes now,
        // where they used to be opaque `FeatureDecl`/`ClassifierDecl` raw text whose declared name
        // had to be recovered by re-scanning it. Same published symbol kinds, read from the node.
        PBE::KermlFeature(p) => {
            if p.value.name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name: p.value.name.clone(),
                kind: "feature decl".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::KermlClassifier(p) => {
            let name = identification_name(&p.value.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "classifier decl".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::FeatureDecl(p) => {
            let name = modeled_decl_name(&p.keyword, &p.text, "_feature");
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "feature decl".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::ClassifierDecl(p) => {
            let name = modeled_decl_name(&p.keyword, &p.text, "_classifier");
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "classifier decl".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::ActionDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "action def".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::ActionUsage(p) => Some(SyntaxOutlineNode {
            name: p.name.clone(),
            kind: "action".to_string(),
            range,
            selection_range: range,
            children: vec![],
        }),
        PBE::ViewDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "view def".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::ViewpointDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "viewpoint def".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::RenderingDef(p) => {
            let name = identification_name(&p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: "rendering def".to_string(),
                range,
                selection_range: range,
                children: vec![],
            })
        }
        PBE::ViewUsage(p) => Some(SyntaxOutlineNode {
            name: p.name.clone(),
            kind: "view".to_string(),
            range,
            selection_range: range,
            children: vec![],
        }),
        PBE::ViewpointUsage(p) => Some(SyntaxOutlineNode {
            name: p.name.clone(),
            kind: "viewpoint".to_string(),
            range,
            selection_range: range,
            children: vec![],
        }),
        PBE::RenderingUsage(p) => Some(SyntaxOutlineNode {
            name: p.name.clone(),
            kind: "rendering".to_string(),
            range,
            selection_range: range,
            children: vec![],
        }),
        PBE::Import(_) | PBE::AliasDef(_) => None,
        _ => None,
    }
}

fn outline_symbols_from_part_def_body(
    elements: &[sysml_v2_parser::Node<PartDefBodyElement>],
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
        let range = span_to_range(&node.span);
        match &node.value {
            PDBE::AttributeDef(n) => out.push(SyntaxOutlineNode {
                name: n.name.clone(),
                kind: "attribute def".to_string(),
                range,
                selection_range: range,
                children: vec![],
            }),
            PDBE::PortUsage(n) => out.push(SyntaxOutlineNode {
                name: n.name.clone(),
                kind: "port".to_string(),
                range,
                selection_range: range,
                children: vec![],
            }),
            _ => {}
        }
    }
    out
}

fn outline_symbols_from_part_usage_body(
    elements: &[sysml_v2_parser::Node<PartUsageBodyElement>],
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
        let range = span_to_range(&node.span);
        match &node.value {
            PUBE::AttributeUsage(n) => out.push(SyntaxOutlineNode {
                name: n.name.clone(),
                kind: "attribute".to_string(),
                range,
                selection_range: range,
                children: vec![],
            }),
            PUBE::PartUsage(n) => {
                let children = match &n.body {
                    PartUsageBody::Brace { elements, .. } => {
                        outline_symbols_from_part_usage_body(elements)
                    }
                    _ => vec![],
                };
                out.push(SyntaxOutlineNode {
                    name: n.name.clone(),
                    kind: "part".to_string(),
                    range,
                    selection_range: range,
                    children,
                });
            }
            PUBE::PortUsage(n) => out.push(SyntaxOutlineNode {
                name: n.name.clone(),
                kind: "port".to_string(),
                range,
                selection_range: range,
                children: vec![],
            }),
            PUBE::Ref(n) => out.push(SyntaxOutlineNode {
                name: n.value.name.clone(),
                kind: "ref".to_string(),
                range,
                selection_range: range,
                children: vec![],
            }),
            _ => {}
        }
    }
    out
}

fn outline_symbols_from_port_def_body(
    elements: &[sysml_v2_parser::Node<PortDefBodyElement>],
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_v2_parser::ast::PortDefBodyElement as PDBE;
        let range = span_to_range(&node.span);
        if let PDBE::PortUsage(n) = &node.value {
            out.push(SyntaxOutlineNode {
                name: n.name.clone(),
                kind: "port".to_string(),
                range,
                selection_range: range,
                children: vec![],
            });
        }
    }
    out
}
