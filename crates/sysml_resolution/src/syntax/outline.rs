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
    declaration_name_text, identification_name, qualified_identification_name,
    span_to_source_range as span_to_range,
};

/// A node's true extent, resolved through the document's own position index.
///
/// The outline reports where a declaration *ends*, which is what makes "which declaration covers
/// this line" and "where does this body fold" answerable. The single-line projection the token
/// collector keeps (see [`span_to_source_range`](super::token_util::span_to_source_range)) puts
/// the end of a multi-line declaration on its opening line, which would make every such question
/// answer "no".
fn node_range(
    document: &sysml_v2_parser::ParsedDocument,
    span: &sysml_v2_parser::Span,
) -> SyntaxRange {
    match document.range(span) {
        Some(range) => SyntaxRange {
            start_line: range.start.line.saturating_sub(1),
            start_character: (range.start.column as u32).saturating_sub(1),
            end_line: range.end.line.saturating_sub(1),
            end_character: (range.end.column as u32).saturating_sub(1),
        },
        None => span_to_range(span),
    }
}
use super::{
    SyntaxFoldingKind, SyntaxFoldingRegion, SyntaxOutlineKind, SyntaxOutlineNode, SyntaxRange,
};

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
                let range = node_range(document, &p.span);
                let children = match &p.body {
                    PackageBody::Brace { elements, .. } => elements
                        .iter()
                        .filter_map(|element| outline_symbol_from_element(document, element))
                        .collect(),
                    _ => vec![],
                };
                let (head_range, body_range) = split_body(document, &p.body, range);
                Some(SyntaxOutlineNode {
                    name,
                    kind: SyntaxOutlineKind::Package,
                    range,
                    selection_range: range,
                    head_range,
                    body_range,
                    children,
                    ..SyntaxOutlineNode::bare(range)
                })
            }
            RootElement::Namespace(n) => {
                let name = qualified_identification_name(document, &n.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = node_range(document, &n.span);
                let children = match &n.body {
                    PackageBody::Brace { elements, .. } => elements
                        .iter()
                        .filter_map(|element| outline_symbol_from_element(document, element))
                        .collect(),
                    _ => vec![],
                };
                let (head_range, body_range) = split_body(document, &n.body, range);
                Some(SyntaxOutlineNode {
                    name,
                    kind: SyntaxOutlineKind::Namespace,
                    range,
                    selection_range: range,
                    head_range,
                    body_range,
                    children,
                    ..SyntaxOutlineNode::bare(range)
                })
            }
            RootElement::LibraryPackage(lp) => {
                let name = qualified_identification_name(document, &lp.identification);
                let name = if name.is_empty() {
                    "(top level)".to_string()
                } else {
                    name
                };
                let range = node_range(document, &lp.span);
                let children = match &lp.body {
                    PackageBody::Brace { elements, .. } => elements
                        .iter()
                        .filter_map(|element| outline_symbol_from_element(document, element))
                        .collect(),
                    _ => vec![],
                };
                let (head_range, body_range) = split_body(document, &lp.body, range);
                Some(SyntaxOutlineNode {
                    name,
                    kind: SyntaxOutlineKind::LibraryPackage,
                    range,
                    selection_range: range,
                    head_range,
                    body_range,
                    children,
                    ..SyntaxOutlineNode::bare(range)
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
            symbol.name = format!("(anonymous {})", symbol.kind.keyword());
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

/// Split a declaration's extent into its header and its braced body.
///
/// The grammar records the exact `{` and `}` tokens, so a host asking "where does this
/// declaration's signature end" never has to count braces in the text.
fn split_body<E>(
    document: &sysml_v2_parser::ParsedDocument,
    body: &sysml_v2_parser::ast::Body<E>,
    range: SyntaxRange,
) -> (SyntaxRange, Option<SyntaxRange>) {
    match body {
        sysml_v2_parser::ast::Body::Semicolon { .. } => (range, None),
        sysml_v2_parser::ast::Body::Brace {
            open_span,
            close_span,
            ..
        } => {
            let open = node_range(document, open_span);
            let close = node_range(document, close_span);
            let head = SyntaxRange {
                start_line: range.start_line,
                start_character: range.start_character,
                end_line: open.start_line,
                end_character: open.start_character,
            };
            let body_range = SyntaxRange {
                start_line: open.start_line,
                start_character: open.start_character,
                end_line: close.end_line,
                end_character: close.end_character,
            };
            (head, Some(body_range))
        }
    }
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
    let range = node_range(document, &node.span);
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
            let (head_range, body_range) = split_body(document, &p.body, range);
            Some(SyntaxOutlineNode {
                name,
                kind: SyntaxOutlineKind::Package,
                range,
                selection_range: range,
                head_range,
                body_range,
                children,
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::PartDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            let children = match &p.body {
                PartDefBody::Brace { elements, .. } => {
                    outline_symbols_from_part_def_body(document, elements)
                }
                _ => vec![],
            };
            let (head_range, body_range) = split_body(document, &p.body, range);
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::PartDef,
                typed_by: super::closure_targets::typing_target_display(
                    document,
                    p.specializes.as_deref(),
                ),
                range,
                selection_range: range,
                head_range,
                body_range,
                children,
            })
        }
        PBE::PartUsage(p) => {
            let children = match &p.body {
                PartUsageBody::Brace { elements, .. } => {
                    outline_symbols_from_part_usage_body(document, elements)
                }
                _ => vec![],
            };
            let (head_range, body_range) = split_body(document, &p.body, range);
            Some(SyntaxOutlineNode {
                name: declaration_name_text(document, p.name).unwrap_or_default(),
                short_name: declaration_name_text(document, p.short_name),
                kind: SyntaxOutlineKind::PartUsage,
                typed_by: super::closure_targets::typing_target_display(
                    document,
                    p.typing.as_deref(),
                ),
                range,
                selection_range: range,
                head_range,
                body_range,
                children,
            })
        }
        PBE::PortDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            let children = match &p.body {
                PortDefBody::Brace { elements, .. } => {
                    outline_symbols_from_port_def_body(document, elements)
                }
                _ => vec![],
            };
            let (head_range, body_range) = split_body(document, &p.body, range);
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::PortDef,
                range,
                selection_range: range,
                head_range,
                body_range,
                children,
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::InterfaceDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::InterfaceDef,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::AttributeDef(p) => Some(SyntaxOutlineNode {
            name: declaration_name_text(document, p.name).unwrap_or_default(),
            kind: SyntaxOutlineKind::AttributeDef,
            range,
            selection_range: range,
            children: vec![],
            ..SyntaxOutlineNode::bare(range)
        }),
        // `feature myFeature : BaseFeature;` and `class VehicleClass;` arrive as typed nodes now,
        // where they used to be opaque `FeatureDecl`/`ClassifierDecl` raw text whose declared name
        // had to be recovered by re-scanning it. Same published symbol kinds, read from the node.
        PBE::KermlFeature(p) => {
            let name = declaration_name_text(document, p.value.name)?;
            Some(SyntaxOutlineNode {
                name,
                kind: SyntaxOutlineKind::FeatureDecl,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::KermlClassifier(p) => {
            let name = identification_name(document, &p.value.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.value.identification.short_name),
                kind: SyntaxOutlineKind::ClassifierDecl,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::FeatureDecl(p) => {
            let name = modeled_decl_name(
                document.source.slice(&p.keyword_span).unwrap_or_default(),
                document.opaque_text(p.text).unwrap_or_default(),
                "_feature",
            );
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: SyntaxOutlineKind::FeatureDecl,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::ClassifierDecl(p) => {
            let name = modeled_decl_name(
                document.source.slice(&p.keyword_span).unwrap_or_default(),
                document.opaque_text(p.text).unwrap_or_default(),
                "_classifier",
            );
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                kind: SyntaxOutlineKind::ClassifierDecl,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::ActionDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::ActionDef,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::ActionUsage(p) => Some(SyntaxOutlineNode {
            name: declaration_name_text(document, p.name).unwrap_or_default(),
            kind: SyntaxOutlineKind::ActionUsage,
            range,
            selection_range: range,
            children: vec![],
            ..SyntaxOutlineNode::bare(range)
        }),
        PBE::ViewDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::ViewDef,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::ViewpointDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::ViewpointDef,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::RenderingDef(p) => {
            let name = identification_name(document, &p.identification);
            if name.is_empty() {
                return None;
            }
            Some(SyntaxOutlineNode {
                name,
                short_name: declaration_name_text(document, p.identification.short_name),
                kind: SyntaxOutlineKind::RenderingDef,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            })
        }
        PBE::ViewUsage(p) => Some(SyntaxOutlineNode {
            name: declaration_name_text(document, p.name).unwrap_or_default(),
            kind: SyntaxOutlineKind::ViewUsage,
            range,
            selection_range: range,
            children: vec![],
            ..SyntaxOutlineNode::bare(range)
        }),
        PBE::ViewpointUsage(p) => Some(SyntaxOutlineNode {
            name: declaration_name_text(document, Some(p.name)).unwrap_or_default(),
            kind: SyntaxOutlineKind::ViewpointUsage,
            range,
            selection_range: range,
            children: vec![],
            ..SyntaxOutlineNode::bare(range)
        }),
        PBE::RenderingUsage(p) => Some(SyntaxOutlineNode {
            name: declaration_name_text(document, p.name).unwrap_or_default(),
            kind: SyntaxOutlineKind::RenderingUsage,
            range,
            selection_range: range,
            children: vec![],
            ..SyntaxOutlineNode::bare(range)
        }),
        PBE::Import(_) | PBE::AliasDef(_) => None,
        _ => None,
    }
}

fn outline_symbols_from_part_def_body(
    document: &sysml_v2_parser::ParsedDocument,
    elements: &[sysml_v2_parser::Node<PartDefBodyElement>],
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
        let range = node_range(document, &node.span);
        match &node.value {
            PDBE::AttributeDef(n) => out.push(SyntaxOutlineNode {
                name: declaration_name_text(document, n.name).unwrap_or_default(),
                kind: SyntaxOutlineKind::AttributeDef,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            }),
            PDBE::PortUsage(n) => out.push(SyntaxOutlineNode {
                name: declaration_name_text(document, n.name).unwrap_or_default(),
                kind: SyntaxOutlineKind::PortUsage,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            }),
            _ => {}
        }
    }
    out
}

fn outline_symbols_from_part_usage_body(
    document: &sysml_v2_parser::ParsedDocument,
    elements: &[sysml_v2_parser::Node<PartUsageBodyElement>],
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
        let range = node_range(document, &node.span);
        match &node.value {
            PUBE::AttributeUsage(n) => out.push(SyntaxOutlineNode {
                name: declaration_name_text(document, n.name).unwrap_or_default(),
                kind: SyntaxOutlineKind::AttributeUsage,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            }),
            PUBE::PartUsage(n) => {
                let children = match &n.body {
                    PartUsageBody::Brace { elements, .. } => {
                        outline_symbols_from_part_usage_body(document, elements)
                    }
                    _ => vec![],
                };
                let (head_range, body_range) = split_body(document, &n.body, range);
                out.push(SyntaxOutlineNode {
                    name: declaration_name_text(document, n.name).unwrap_or_default(),
                    short_name: declaration_name_text(document, n.short_name),
                    kind: SyntaxOutlineKind::PartUsage,
                    range,
                    selection_range: range,
                    head_range,
                    body_range,
                    children,
                    ..SyntaxOutlineNode::bare(range)
                });
            }
            PUBE::PortUsage(n) => out.push(SyntaxOutlineNode {
                name: declaration_name_text(document, n.name).unwrap_or_default(),
                kind: SyntaxOutlineKind::PortUsage,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            }),
            PUBE::Ref(n) => out.push(SyntaxOutlineNode {
                name: declaration_name_text(document, n.value.name).unwrap_or_default(),
                kind: SyntaxOutlineKind::Ref,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            }),
            _ => {}
        }
    }
    out
}

fn outline_symbols_from_port_def_body(
    document: &sysml_v2_parser::ParsedDocument,
    elements: &[sysml_v2_parser::Node<PortDefBodyElement>],
) -> Vec<SyntaxOutlineNode> {
    let mut out = Vec::new();
    for node in elements {
        use sysml_v2_parser::ast::PortDefBodyElement as PDBE;
        let range = node_range(document, &node.span);
        if let PDBE::PortUsage(n) = &node.value {
            out.push(SyntaxOutlineNode {
                name: declaration_name_text(document, n.name).unwrap_or_default(),
                kind: SyntaxOutlineKind::PortUsage,
                range,
                selection_range: range,
                children: vec![],
                ..SyntaxOutlineNode::bare(range)
            });
        }
    }
    out
}
