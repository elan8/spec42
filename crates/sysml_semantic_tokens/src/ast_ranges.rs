//! AST-driven semantic token ranges: collects (SourceRange, type_index) from parsed AST.

use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody, ActionUsageBodyElement,
    AttributeBody, AttributeBodyElement, CalcDefBody, ConnectionDefBody, ConnectionDefBodyElement,
    ConstraintDefBodyElement, DefinitionBody, DefinitionBodyElement, InterfaceDefBody,
    InterfaceDefBodyElement, MetadataKeywordUsage, OccurrenceBodyElement, OccurrenceUsageBody,
    PackageBody, PackageBodyElement, PartDefBody,
    PartDefBodyElement, PartUsageBody, PartUsageBodyElement, PortBody, PortBodyElement,
    PortDefBody, PortDefBodyElement, RequireConstraintBody, RequirementDefBody,
    RequirementDefBodyElement, RootElement,
};
use sysml_v2_parser::RootNamespace;

use crate::ast_util::{identification_name, span_to_source_range, SourceRange};
use crate::types::*;

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
            let name = identification_name(&pkg_node.identification);
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
            if let Some(ref s) = ad_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            match &ad_node.body {
                ActionDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_action_def_body_element(element, out);
                    }
                }
                ActionDefBody::Semicolon => {}
            }
        }
        PBE::RequirementDef(rd_node) => {
            out.push((span_to_source_range(&rd_node.span), TYPE_CLASS));
            if let Some(ref s) = rd_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            match &rd_node.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        PBE::RequirementUsage(ru_node) => {
            out.push((span_to_source_range(&ru_node.span), TYPE_PROPERTY));
            match &ru_node.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
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
        PBE::ItemDef(id_node) => {
            out.push((span_to_source_range(&id_node.span), TYPE_CLASS));
            if let Some(ref s) = id_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            collect_semantic_ranges_attribute_body(&id_node.value.body, out);
        }
        PBE::IndividualDef(id_node) => {
            out.push((span_to_source_range(&id_node.span), TYPE_CLASS));
            if let Some(ref s) = id_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            collect_semantic_ranges_attribute_body(&id_node.value.body, out);
        }
        PBE::MetadataDef(md_node) => {
            out.push((span_to_source_range(&md_node.span), TYPE_CLASS));
            if let Some(ref s) = md_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            collect_semantic_ranges_attribute_body(&md_node.value.body, out);
        }
        PBE::OccurrenceDef(occ_node) => {
            out.push((span_to_source_range(&occ_node.span), TYPE_CLASS));
            if let Some(ref s) = occ_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            collect_semantic_ranges_definition_body(&occ_node.value.body, out);
        }
        PBE::FlowDef(flow_node) => {
            out.push((span_to_source_range(&flow_node.span), TYPE_INTERFACE));
            if let Some(ref s) = flow_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            collect_semantic_ranges_definition_body(&flow_node.value.body, out);
        }
        PBE::FlowUsage(flow_node) => {
            out.push((span_to_source_range(&flow_node.span), TYPE_PROPERTY));
            collect_semantic_ranges_definition_body(&flow_node.value.body, out);
        }
        PBE::AllocationDef(alloc_node) => {
            out.push((span_to_source_range(&alloc_node.span), TYPE_INTERFACE));
            if let Some(ref s) = alloc_node.value.specializes_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            collect_semantic_ranges_definition_body(&alloc_node.value.body, out);
        }
        _ => {}
    }
}

fn collect_semantic_ranges_definition_body(body: &DefinitionBody, out: &mut Vec<(SourceRange, u32)>) {
    let DefinitionBody::Brace { elements } = body else {
        return;
    };
    for node in elements {
        match &node.value {
            DefinitionBodyElement::OccurrenceMember(member) => {
                collect_semantic_ranges_occurrence_body_element(member, out);
            }
            DefinitionBodyElement::Doc(_)
            | DefinitionBodyElement::Error(_)
            | DefinitionBodyElement::Other(_) => {}
        }
    }
}

fn collect_semantic_ranges_attribute_body(body: &AttributeBody, out: &mut Vec<(SourceRange, u32)>) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };
    for node in elements {
        match &node.value {
            AttributeBodyElement::AttributeDef(attribute) => {
                out.push((span_to_source_range(&attribute.span), TYPE_PROPERTY));
            }
            AttributeBodyElement::AttributeUsage(attribute) => {
                out.push((span_to_source_range(&attribute.span), TYPE_PROPERTY));
            }
            AttributeBodyElement::Doc(_)
            | AttributeBodyElement::Error(_)
            | AttributeBodyElement::Other(_) => {}
        }
    }
}

fn collect_semantic_ranges_metadata_keyword_usage(
    node: &sysml_v2_parser::Node<MetadataKeywordUsage>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    out.push((span_to_source_range(&node.value.keyword_span), TYPE_PROPERTY));
    if let Some(ref span) = node.value.type_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
}

fn collect_semantic_ranges_occurrence_body_element(
    node: &sysml_v2_parser::Node<OccurrenceBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use OccurrenceBodyElement as OBE;
    match &node.value {
        OBE::AttributeUsage(attribute) => {
            out.push((span_to_source_range(&attribute.span), TYPE_PROPERTY));
        }
        OBE::PartUsage(part_usage) => {
            if let Some(ref span) = part_usage.value.name_span {
                out.push((span_to_source_range(span), TYPE_PROPERTY));
            }
            if let Some(ref span) = part_usage.value.type_ref_span {
                out.push((span_to_source_range(span), TYPE_TYPE));
            }
            if let PartUsageBody::Brace { elements } = &part_usage.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(child, out);
                }
            }
        }
        OBE::OccurrenceUsage(occurrence_usage) => {
            out.push((span_to_source_range(&occurrence_usage.span), TYPE_PROPERTY));
            if let OccurrenceUsageBody::Brace { elements } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(child, out);
                }
            }
        }
        OBE::Doc(_)
        | OBE::Error(_)
        | OBE::Annotation(_)
        | OBE::AssertConstraint(_)
        | OBE::Other(_) => {}
    }
}

fn collect_semantic_ranges_connection_def_body_element(
    node: &sysml_v2_parser::Node<ConnectionDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use ConnectionDefBodyElement as CDBE;
    match &node.value {
        CDBE::EndDecl(end_decl) => {
            if let Some(ref span) = end_decl.name_span {
                out.push((span_to_source_range(span), TYPE_PROPERTY));
            }
            if let Some(ref span) = end_decl.type_ref_span {
                out.push((span_to_source_range(span), TYPE_TYPE));
            }
        }
        CDBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        CDBE::ConnectStmt(_) => {}
    }
}

fn collect_semantic_ranges_part_def_body_element(
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => out.push((span_to_source_range(&n.span), TYPE_PROPERTY)),
        PDBE::AttributeUsage(n) => out.push((span_to_source_range(&n.span), TYPE_PROPERTY)),
        PDBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PDBE::PartUsage(pu_node) => {
            if let Some(ref span) = pu_node.value.name_span {
                out.push((span_to_source_range(span), TYPE_PROPERTY));
            }
            if let Some(ref span) = pu_node.value.type_ref_span {
                out.push((span_to_source_range(span), TYPE_TYPE));
            }
            if let PartUsageBody::Brace { elements } = &pu_node.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(child, out);
                }
            }
        }
        PDBE::Ref(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        PDBE::ItemUsage(item_node) => {
            out.push((span_to_source_range(&item_node.span), TYPE_PROPERTY));
            collect_semantic_ranges_attribute_body(&item_node.body, out);
        }
        PDBE::OccurrenceUsage(occurrence_usage) => {
            out.push((span_to_source_range(&occurrence_usage.span), TYPE_PROPERTY));
            if let OccurrenceUsageBody::Brace { elements } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(child, out);
                }
            }
        }
        PDBE::InterfaceDef(id_node) => {
            out.push((span_to_source_range(&id_node.span), TYPE_INTERFACE));
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                for element in elements {
                    collect_semantic_ranges_interface_def_body_element(element, out);
                }
            }
        }
        PDBE::Connection(connection_usage) => {
            out.push((span_to_source_range(&connection_usage.span), TYPE_PROPERTY));
            if let ConnectionDefBody::Brace { elements } = &connection_usage.value.body {
                for element in elements {
                    collect_semantic_ranges_connection_def_body_element(element, out);
                }
            }
        }
        PDBE::Perform(perform) => out.push((span_to_source_range(&perform.span), TYPE_FUNCTION)),
        PDBE::ExhibitState(state_usage) => {
            out.push((span_to_source_range(&state_usage.span), TYPE_PROPERTY));
        }
        PDBE::CalcUsage(calc_node) => {
            out.push((span_to_source_range(&calc_node.span), TYPE_FUNCTION));
            if let CalcDefBody::Brace { .. } = &calc_node.value.body {}
        }
        PDBE::EnumerationUsage(enum_node) => {
            out.push((span_to_source_range(&enum_node.span), TYPE_PROPERTY));
            collect_semantic_ranges_attribute_body(&enum_node.body, out);
        }
        PDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(mk_node, out);
        }
        PDBE::RequirementUsage(ru_node) => {
            out.push((span_to_source_range(&ru_node.span), TYPE_PROPERTY));
            match &ru_node.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        PDBE::Connect(_)
        | PDBE::InterfaceUsage(_)
        | PDBE::Allocate(_)
        | PDBE::OpaqueMember(_)
        | PDBE::Annotation(_)
        | PDBE::Error(_)
        | PDBE::Doc(_)
        | PDBE::Comment(_)
        | PDBE::Other(_) => {}
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
        PUBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PUBE::Ref(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            } else {
                out.push((span_to_source_range(&n.span), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
        }
        _ => {}
    }
}

fn collect_semantic_ranges_port_usage(
    n: &sysml_v2_parser::Node<sysml_v2_parser::ast::PortUsage>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    if let Some(ref s) = n.value.name_span {
        out.push((span_to_source_range(s), TYPE_PROPERTY));
    }
    if let Some(ref s) = n.value.type_ref_span {
        out.push((span_to_source_range(s), TYPE_TYPE));
    }
    if let PortBody::Brace { elements } = &n.body {
        for child in elements {
            collect_semantic_ranges_port_body_element(child, out);
        }
    }
}

fn collect_semantic_ranges_port_body_element(
    node: &sysml_v2_parser::Node<PortBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use PortBodyElement as PBE;
    match &node.value {
        PBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PBE::InOutDecl(w) => {
            out.push((span_to_source_range(&w.span), TYPE_PROPERTY));
        }
        PBE::Error(_) | PBE::Other(_) => {}
    }
}

fn collect_semantic_ranges_port_def_body_element(
    node: &sysml_v2_parser::Node<PortDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PortDefBodyElement as PDBE;
    match &node.value {
        PDBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PDBE::InOutDecl(w) => {
            out.push((span_to_source_range(&w.span), TYPE_PROPERTY));
        }
        _ => {}
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

fn collect_semantic_ranges_action_usage(usage: &ActionUsage, out: &mut Vec<(SourceRange, u32)>) {
    if let Some(ref span) = usage.name_span {
        out.push((span_to_source_range(span), TYPE_PROPERTY));
    }
    if let Some(ref span) = usage.type_ref_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
    if let ActionUsageBody::Brace { elements } = &usage.body {
        for element in elements {
            collect_semantic_ranges_action_usage_body_element(element, out);
        }
    }
}

fn collect_semantic_ranges_ref_decl(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::RefDecl>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    let value = &node.value;
    if let Some(ref span) = value.name_span {
        out.push((span_to_source_range(span), TYPE_PROPERTY));
    } else {
        out.push((span_to_source_range(&node.span), TYPE_PROPERTY));
    }
    if let Some(ref span) = value.type_ref_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
}

fn collect_semantic_ranges_state_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::StateUsage>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    out.push((span_to_source_range(&node.span), TYPE_PROPERTY));
}

fn collect_semantic_ranges_requirement_def_body_element(
    node: &sysml_v2_parser::Node<RequirementDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use RequirementDefBodyElement as RDBE;
    match &node.value {
        RDBE::SubjectDecl(subject) => {
            out.push((span_to_source_range(&subject.span), TYPE_PROPERTY));
        }
        RDBE::Stakeholder(stakeholder) => {
            out.push((
                span_to_source_range(&stakeholder.value.target_span),
                TYPE_PROPERTY,
            ));
        }
        RDBE::Purpose(purpose) => {
            out.push((
                span_to_source_range(&purpose.value.target_span),
                TYPE_PROPERTY,
            ));
        }
        RDBE::AttributeDef(attribute) => {
            out.push((span_to_source_range(&attribute.span), TYPE_PROPERTY));
        }
        RDBE::AttributeUsage(attribute) => {
            out.push((span_to_source_range(&attribute.span), TYPE_PROPERTY));
        }
        RDBE::VerifyRequirement(verify) => {
            if let Some(requirement) = &verify.value.requirement {
                out.push((span_to_source_range(&requirement.span), TYPE_PROPERTY));
            }
        }
        RDBE::RequireConstraint(constraint) => {
            if let RequireConstraintBody::Brace { elements } = &constraint.value.body {
                for element in elements {
                    if let ConstraintDefBodyElement::InOutDecl(param) = &element.value {
                        out.push((span_to_source_range(&param.span), TYPE_PROPERTY));
                    }
                }
            }
        }
        RDBE::Frame(frame) => {
            out.push((span_to_source_range(&frame.span), TYPE_NAMESPACE));
            match &frame.value.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        RDBE::Import(import) => out.push((span_to_source_range(&import.span), TYPE_NAMESPACE)),
        RDBE::TextualRep(textual) => {
            if let Some(ref span) = textual.value.language_span {
                out.push((span_to_source_range(span), TYPE_STRING));
            }
            out.push((span_to_source_range(&textual.span), TYPE_PROPERTY));
        }
        RDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(mk_node, out);
        }
        RDBE::RequirementActorDecl(actor) => {
            out.push((span_to_source_range(&actor.span), TYPE_PROPERTY));
        }
        RDBE::Doc(_)
        | RDBE::Error(_)
        | RDBE::Other(_)
        | RDBE::Annotation(_)
        | RDBE::MetadataAnnotation(_) => {}
    }
}

fn collect_semantic_ranges_action_def_body_element(
    node: &sysml_v2_parser::Node<ActionDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use ActionDefBodyElement as ADBE;
    match &node.value {
        ADBE::InOutDecl(in_out) => out.push((span_to_source_range(&in_out.span), TYPE_PROPERTY)),
        ADBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(usage.as_ref(), out),
        ADBE::ThenAction(then_action) => {
            collect_semantic_ranges_action_usage(&then_action.value.action.value, out);
        }
        ADBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        ADBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(state_usage, out),
        ADBE::Perform(perform) => out.push((span_to_source_range(&perform.span), TYPE_FUNCTION)),
        ADBE::Assign(assign) => out.push((span_to_source_range(&assign.span), TYPE_PROPERTY)),
        ADBE::ForLoop(for_loop) => out.push((span_to_source_range(&for_loop.span), TYPE_PROPERTY)),
        ADBE::Bind(_)
        | ADBE::Flow(_)
        | ADBE::FirstStmt(_)
        | ADBE::MergeStmt(_)
        | ADBE::Decl(_)
        | ADBE::Error(_)
        | ADBE::Doc(_)
        | ADBE::Annotation(_) => {}
    }
}

fn collect_semantic_ranges_action_usage_body_element(
    node: &sysml_v2_parser::Node<ActionUsageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::ActionUsageBodyElement as AUBE;
    match &node.value {
        AUBE::InOutDecl(in_out) => out.push((span_to_source_range(&in_out.span), TYPE_PROPERTY)),
        AUBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(usage.as_ref(), out),
        AUBE::ThenAction(then_action) => {
            collect_semantic_ranges_action_usage(&then_action.value.action.value, out);
        }
        AUBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        AUBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(state_usage, out),
        AUBE::Assign(assign) => out.push((span_to_source_range(&assign.span), TYPE_PROPERTY)),
        AUBE::ForLoop(for_loop) => out.push((span_to_source_range(&for_loop.span), TYPE_PROPERTY)),
        AUBE::Bind(_)
        | AUBE::Flow(_)
        | AUBE::FirstStmt(_)
        | AUBE::MergeStmt(_)
        | AUBE::Error(_)
        | AUBE::Doc(_)
        | AUBE::Annotation(_)
        | AUBE::Decl(_) => {}
    }
}
