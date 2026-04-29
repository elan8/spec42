//! AST-driven semantic token ranges: collects (SourceRange, type_index) from parsed AST.

use crate::syntax::ast_util::{span_to_source_range, SourceRange};
use sysml_v2_parser::ast::{
    ActionDefBody, ActionUsageBody, ActionUsageBodyElement, InterfaceDefBody,
    InterfaceDefBodyElement, PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement,
    PartUsageBody, PartUsageBodyElement, PortDefBody, PortDefBodyElement, RootElement,
};
use sysml_v2_parser::RootNamespace;

use super::types::*;

/// Build (SourceRange, token_type_index) from AST for semantic_tokens_full/range.
pub fn ast_semantic_ranges(root: &RootNamespace) -> Vec<(SourceRange, u32)> {
    let mut out = Vec::new();
    for node in &root.elements {
        let elements = match &node.value {
            RootElement::Package(p) => match &p.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Namespace(n) => match &n.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::LibraryPackage(lp) => match &lp.body {
                PackageBody::Brace { elements } => elements,
                _ => continue,
            },
            RootElement::Import(_) => continue,
        };
        for el in elements {
            collect_semantic_ranges_package_body_element(el, &mut out);
        }
    }
    out
}

fn collect_semantic_ranges_package_body_element(
    node: &sysml_v2_parser::Node<PackageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(pkg_node) => {
            let name = crate::syntax::ast_util::identification_name(&pkg_node.identification);
            if !name.is_empty() {
                out.push((span_to_source_range(&pkg_node.span), TYPE_NAMESPACE));
            }
            match &pkg_node.body {
                PackageBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_package_body_element(n, out);
                    }
                }
                PackageBody::Semicolon => {}
            }
        }
        PBE::Import(imp_node) => {
            out.push((span_to_source_range(&imp_node.span), TYPE_NAMESPACE));
        }
        PBE::PartDef(pd_node) => {
            out.push((span_to_source_range(&pd_node.span), TYPE_CLASS));
            if let Some(ref s) = pd_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            match &pd_node.body {
                PartDefBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_part_def_body_element(n, out);
                    }
                }
                PartDefBody::Semicolon => {}
            }
        }
        PBE::PartUsage(pu_node) => {
            if let Some(ref s) = pu_node.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = pu_node.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            match &pu_node.body {
                PartUsageBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_part_usage_body_element(n, out);
                    }
                }
                PartUsageBody::Semicolon => {}
            }
        }
        PBE::PortDef(pd_node) => {
            out.push((span_to_source_range(&pd_node.span), TYPE_TYPE));
            match &pd_node.body {
                PortDefBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_port_def_body_element(n, out);
                    }
                }
                PortDefBody::Semicolon => {}
            }
        }
        PBE::InterfaceDef(id_node) => {
            out.push((span_to_source_range(&id_node.span), TYPE_INTERFACE));
            match &id_node.body {
                InterfaceDefBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_interface_def_body_element(n, out);
                    }
                }
                InterfaceDefBody::Semicolon => {}
            }
        }
        PBE::AttributeDef(ad_node) => {
            out.push((span_to_source_range(&ad_node.span), TYPE_PROPERTY));
        }
        PBE::ActionDef(ad_node) => {
            out.push((span_to_source_range(&ad_node.span), TYPE_FUNCTION));
            match &ad_node.body {
                ActionDefBody::Brace { elements: _ } => {}
                ActionDefBody::Semicolon => {}
            }
        }
        PBE::ActionUsage(au_node) => {
            if let Some(ref s) = au_node.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = au_node.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            match &au_node.body {
                ActionUsageBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_action_usage_body_element(n, out);
                    }
                }
                ActionUsageBody::Semicolon => {}
            }
        }
        PBE::AliasDef(ad_node) => {
            out.push((span_to_source_range(&ad_node.span), TYPE_NAMESPACE));
        }
        PBE::ViewDef(vd_node) => {
            out.push((span_to_source_range(&vd_node.span), TYPE_NAMESPACE));
        }
        PBE::ViewpointDef(vpd_node) => {
            out.push((span_to_source_range(&vpd_node.span), TYPE_NAMESPACE));
        }
        PBE::RenderingDef(rd_node) => {
            out.push((span_to_source_range(&rd_node.span), TYPE_NAMESPACE));
        }
        PBE::ViewUsage(vu_node) => {
            out.push((span_to_source_range(&vu_node.span), TYPE_PROPERTY));
        }
        PBE::ViewpointUsage(vpu_node) => {
            out.push((span_to_source_range(&vpu_node.span), TYPE_PROPERTY));
        }
        PBE::RenderingUsage(ru_node) => {
            out.push((span_to_source_range(&ru_node.span), TYPE_PROPERTY));
        }
        _ => {}
    }
}

fn collect_semantic_ranges_part_def_body_element(
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => out.push((span_to_source_range(&n.span), TYPE_PROPERTY)),
        PDBE::PortUsage(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
        }
        _ => {}
    }
}

fn collect_semantic_ranges_part_usage_body_element(
    node: &sysml_v2_parser::Node<PartUsageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => out.push((span_to_source_range(&n.span), TYPE_PROPERTY)),
        PUBE::PartUsage(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
        }
        PUBE::PortUsage(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
        }
        _ => {}
    }
}

fn collect_semantic_ranges_port_def_body_element(
    node: &sysml_v2_parser::Node<PortDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PortDefBodyElement as PDBE;
    if let PDBE::PortUsage(n) = &node.value {
        if let Some(ref s) = n.value.name_span {
            out.push((span_to_source_range(s), TYPE_PROPERTY));
        }
        if let Some(ref s) = n.value.type_ref_span {
            out.push((span_to_source_range(s), TYPE_TYPE));
        }
    }
}

fn collect_semantic_ranges_interface_def_body_element(
    node: &sysml_v2_parser::Node<InterfaceDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::InterfaceDefBodyElement as IDBE;
    match &node.value {
        IDBE::EndDecl(n) => {
            if let Some(ref s) = n.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
        }
        IDBE::RefDecl(n) => {
            if let Some(ref s) = n.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
        }
        IDBE::ConnectStmt(_) | IDBE::Doc(_) => {}
    }
}

fn collect_semantic_ranges_action_usage_body_element(
    node: &sysml_v2_parser::Node<ActionUsageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::ActionUsageBodyElement as AUBE;
    match &node.value {
        AUBE::InOutDecl(n) => out.push((span_to_source_range(&n.span), TYPE_PROPERTY)),
        AUBE::Bind(_)
        | AUBE::Flow(_)
        | AUBE::FirstStmt(_)
        | AUBE::MergeStmt(_)
        | AUBE::ActionUsage(_)
        | AUBE::Error(_)
        | AUBE::Doc(_)
        | AUBE::Annotation(_)
        | AUBE::RefDecl(_)
        | AUBE::StateUsage(_)
        | AUBE::Assign(_)
        | AUBE::ForLoop(_)
        | AUBE::ThenAction(_)
        | AUBE::Decl(_) => {}
    }
}
