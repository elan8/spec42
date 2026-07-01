//! AST-driven semantic token ranges: collects (SourceRange, type_index) from parsed AST.

use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody, ActionUsageBodyElement,
    AttributeBody, AttributeBodyElement, CalcDefBody, ConnectionDefBody, ConnectionDefBodyElement,
    ConstraintDefBodyElement, DefinitionBody, DefinitionBodyElement, FinalState, InterfaceDefBody,
    InterfaceDefBodyElement, MetadataAnnotation, MetadataKeywordUsage, OccurrenceBodyElement,
    OccurrenceUsageBody, PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement,
    PartUsageBody, PartUsageBodyElement, PayloadClause, PortBody, PortBodyElement, PortDefBody,
    PortDefBodyElement, RequireConstraintBody, RequirementDefBody, RequirementDefBodyElement,
    RootElement, StateDefBody, StateDefBodyElement, StateUsage, ThenStmt, Transition,
    TransitionAccept,
};
use sysml_v2_parser::RootNamespace;

use crate::ast_util::{
    identification_name, push_ident_definition_spans, push_usage_name_type_spans,
    span_to_source_range, SourceRange,
};
use crate::types::*;

struct RangeCtx<'a> {
    source: &'a str,
}

/// Build (SourceRange, token_type_index) from AST for semantic_tokens_full/range.
pub fn ast_semantic_ranges(root: &RootNamespace, source: &str) -> Vec<(SourceRange, u32)> {
    let ctx = RangeCtx { source };
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
            collect_semantic_ranges_package_body_element(&ctx, el, &mut out);
        }
    }
    out
}

fn collect_semantic_ranges_package_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<PackageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(pkg_node) => {
            let name = identification_name(&pkg_node.identification);
            if !name.is_empty() {
                push_ident_definition_spans(&pkg_node.span, None, TYPE_NAMESPACE, out);
            }
            match &pkg_node.body {
                PackageBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_package_body_element(ctx, n, out);
                    }
                }
                PackageBody::Semicolon => {}
            }
        }
        PBE::Import(imp_node) => {
            // Use the precise target span so only the qualified name is highlighted,
            // not the leading `import` keyword or trailing `::*` suffix.
            out.push((
                span_to_source_range(&imp_node.value.target_span),
                TYPE_NAMESPACE,
            ));
        }
        PBE::PartDef(pd_node) => {
            push_ident_definition_spans(
                &pd_node.span,
                pd_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            match &pd_node.body {
                PartDefBody::Brace { elements } => {
                    for n in elements {
                        collect_semantic_ranges_part_def_body_element(ctx, n, out);
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
                        collect_semantic_ranges_part_usage_body_element(ctx, n, out);
                    }
                }
                PartUsageBody::Semicolon => {}
            }
        }
        PBE::PortDef(pd_node) => {
            push_ident_definition_spans(&pd_node.span, None, TYPE_TYPE, out);
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
            push_ident_definition_spans(&id_node.span, None, TYPE_INTERFACE, out);
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
            push_usage_name_type_spans(
                ctx.source,
                &ad_node.span,
                &ad_node.value.name,
                ad_node.value.typing.as_deref(),
                ad_node.value.name_span.as_ref(),
                ad_node.value.typing_span.as_ref(),
                out,
            );
        }
        PBE::ActionDef(ad_node) => {
            push_ident_definition_spans(
                &ad_node.span,
                ad_node.value.specializes_span.as_ref(),
                TYPE_FUNCTION,
                out,
            );
            match &ad_node.body {
                ActionDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_action_def_body_element(ctx, element, out);
                    }
                }
                ActionDefBody::Semicolon => {}
            }
        }
        PBE::RequirementDef(rd_node) => {
            push_ident_definition_spans(
                &rd_node.span,
                rd_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            match &rd_node.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        PBE::RequirementUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ru_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            match &ru_node.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
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
                        collect_semantic_ranges_action_usage_body_element(ctx, n, out);
                    }
                }
                ActionUsageBody::Semicolon => {}
            }
        }
        PBE::AliasDef(ad_node) => {
            push_ident_definition_spans(&ad_node.span, None, TYPE_NAMESPACE, out);
        }
        PBE::ViewDef(vd_node) => {
            push_ident_definition_spans(&vd_node.span, None, TYPE_NAMESPACE, out);
        }
        PBE::ViewpointDef(vpd_node) => {
            push_ident_definition_spans(&vpd_node.span, None, TYPE_NAMESPACE, out);
        }
        PBE::RenderingDef(rd_node) => {
            push_ident_definition_spans(&rd_node.span, None, TYPE_NAMESPACE, out);
        }
        PBE::ViewUsage(vu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &vu_node.span,
                &vu_node.value.name,
                vu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
        }
        PBE::ViewpointUsage(vpu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &vpu_node.span,
                &vpu_node.value.name,
                Some(&vpu_node.value.type_name),
                None,
                None,
                out,
            );
        }
        PBE::RenderingUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ru_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
        }
        PBE::ItemDef(id_node) => {
            push_ident_definition_spans(
                &id_node.span,
                id_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &id_node.value.body, out);
        }
        PBE::IndividualDef(id_node) => {
            push_ident_definition_spans(
                &id_node.span,
                id_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &id_node.value.body, out);
        }
        PBE::MetadataDef(md_node) => {
            push_ident_definition_spans(
                &md_node.span,
                md_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &md_node.value.body, out);
        }
        PBE::OccurrenceDef(occ_node) => {
            push_ident_definition_spans(
                &occ_node.span,
                occ_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            collect_semantic_ranges_definition_body(ctx, &occ_node.value.body, out);
        }
        PBE::FlowDef(flow_node) => {
            push_ident_definition_spans(
                &flow_node.span,
                flow_node.value.specializes_span.as_ref(),
                TYPE_INTERFACE,
                out,
            );
            collect_semantic_ranges_definition_body(ctx, &flow_node.value.body, out);
        }
        PBE::FlowUsage(flow_node) => {
            if let Some(ref name) = flow_node.value.name {
                push_usage_name_type_spans(
                    ctx.source,
                    &flow_node.span,
                    name,
                    flow_node.value.type_name.as_deref(),
                    None,
                    None,
                    out,
                );
            }
            collect_semantic_ranges_definition_body(ctx, &flow_node.value.body, out);
        }
        PBE::AllocationDef(alloc_node) => {
            push_ident_definition_spans(
                &alloc_node.span,
                alloc_node.value.specializes_span.as_ref(),
                TYPE_INTERFACE,
                out,
            );
            collect_semantic_ranges_definition_body(ctx, &alloc_node.value.body, out);
        }
        PBE::StateDef(sd_node) => {
            push_ident_definition_spans(
                &sd_node.span,
                sd_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            if let StateDefBody::Brace { elements } = &sd_node.body {
                for element in elements {
                    collect_semantic_ranges_state_def_body_element(ctx, element, out);
                }
            }
        }
        PBE::StateUsage(su_node) => {
            collect_semantic_ranges_state_usage(ctx, su_node, out);
        }
        PBE::ConnectionDef(conn_node) => {
            push_ident_definition_spans(&conn_node.span, None, TYPE_INTERFACE, out);
            if let ConnectionDefBody::Brace { elements } = &conn_node.body {
                for element in elements {
                    collect_semantic_ranges_connection_def_body_element(element, out);
                }
            }
        }
        PBE::ConstraintDef(cd_node) => {
            push_ident_definition_spans(
                &cd_node.span,
                cd_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
        }
        PBE::CalcDef(calc_node) => {
            push_ident_definition_spans(&calc_node.span, None, TYPE_FUNCTION, out);
        }
        PBE::EnumDef(enum_node) => {
            push_ident_definition_spans(
                &enum_node.span,
                enum_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
        }
        PBE::UseCaseDef(uc_node) => {
            push_ident_definition_spans(
                &uc_node.span,
                uc_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
        }
        PBE::VerificationCaseDef(vc_node) => {
            push_ident_definition_spans(
                &vc_node.span,
                vc_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
        }
        PBE::CaseDef(case_node) => {
            push_ident_definition_spans(
                &case_node.span,
                case_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
        }
        PBE::AnalysisCaseDef(ac_node) => {
            push_ident_definition_spans(
                &ac_node.span,
                ac_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
        }
        PBE::MetadataUsage(mu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &mu_node.span,
                &mu_node.value.name,
                mu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &mu_node.value.body, out);
        }
        PBE::OccurrenceUsage(ou_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ou_node.span,
                &ou_node.value.name,
                ou_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            if let OccurrenceUsageBody::Brace { elements } = &ou_node.value.body {
                for element in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, element, out);
                }
            }
        }
        PBE::AllocationUsage(au_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &au_node.span,
                &au_node.value.name,
                au_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            collect_semantic_ranges_definition_body(ctx, &au_node.value.body, out);
        }
        PBE::ConcernUsage(cu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &cu_node.span,
                &cu_node.value.name,
                cu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            match &cu_node.value.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        PBE::UseCaseUsage(ucu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ucu_node.span,
                &ucu_node.value.name,
                ucu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
        }
        PBE::VerificationCaseUsage(vcu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &vcu_node.span,
                &vcu_node.value.name,
                vcu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
        }
        PBE::CaseUsage(cu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &cu_node.span,
                &cu_node.value.name,
                cu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
        }
        PBE::AnalysisCaseUsage(acu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &acu_node.span,
                &acu_node.value.name,
                acu_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
        }
        PBE::Actor(actor_node) => {
            let name = identification_name(&actor_node.value.identification);
            if !name.is_empty() {
                push_ident_definition_spans(&actor_node.span, None, TYPE_PROPERTY, out);
            }
        }
        _ => {}
    }
}

fn collect_semantic_ranges_definition_body(
    ctx: &RangeCtx<'_>,
    body: &DefinitionBody,
    out: &mut Vec<(SourceRange, u32)>,
) {
    let DefinitionBody::Brace { elements } = body else {
        return;
    };
    for node in elements {
        match &node.value {
            DefinitionBodyElement::OccurrenceMember(member) => {
                collect_semantic_ranges_occurrence_body_element(ctx, member, out);
            }
            DefinitionBodyElement::Doc(_)
            | DefinitionBodyElement::Error(_)
            | DefinitionBodyElement::Other(_) => {}
        }
    }
}

fn collect_semantic_ranges_attribute_body(
    ctx: &RangeCtx<'_>,
    body: &AttributeBody,
    out: &mut Vec<(SourceRange, u32)>,
) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };
    for node in elements {
        match &node.value {
            AttributeBodyElement::AttributeDef(attribute) => {
                push_usage_name_type_spans(
                    ctx.source,
                    &attribute.span,
                    &attribute.value.name,
                    attribute.value.typing.as_deref(),
                    attribute.value.name_span.as_ref(),
                    attribute.value.typing_span.as_ref(),
                    out,
                );
            }
            AttributeBodyElement::AttributeUsage(attribute) => {
                push_usage_name_type_spans(
                    ctx.source,
                    &attribute.span,
                    &attribute.value.name,
                    attribute.value.typing.as_deref(),
                    attribute.value.name_span.as_ref(),
                    attribute.value.typing_span.as_ref(),
                    out,
                );
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
    out.push((
        span_to_source_range(&node.value.keyword_span),
        TYPE_PROPERTY,
    ));
    if let Some(ref span) = node.value.type_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
}

fn collect_semantic_ranges_metadata_annotation(
    node: &sysml_v2_parser::Node<MetadataAnnotation>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    if let Some(ref span) = node.value.head_span {
        out.push((span_to_source_range(span), TYPE_PROPERTY));
    }
    if let Some(ref span) = node.value.type_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
}

fn collect_semantic_ranges_payload_clause(
    clause: &PayloadClause,
    out: &mut Vec<(SourceRange, u32)>,
) {
    out.push((span_to_source_range(&clause.name_span), TYPE_PROPERTY));
    if let Some(ref span) = clause.type_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
}

fn collect_semantic_ranges_transition_accept(
    accept: &TransitionAccept,
    out: &mut Vec<(SourceRange, u32)>,
) {
    match accept {
        TransitionAccept::Payload(clause) => collect_semantic_ranges_payload_clause(clause, out),
        TransitionAccept::Shorthand(expr) => {
            out.push((span_to_source_range(&expr.span), TYPE_PROPERTY));
        }
    }
}

fn collect_semantic_ranges_then_stmt(then_stmt: &ThenStmt, out: &mut Vec<(SourceRange, u32)>) {
    if let Some(ref span) = then_stmt.name_span {
        out.push((span_to_source_range(span), TYPE_PROPERTY));
    }
}

fn collect_semantic_ranges_final_state(
    final_state: &FinalState,
    out: &mut Vec<(SourceRange, u32)>,
) {
    out.push((span_to_source_range(&final_state.name_span), TYPE_PROPERTY));
}

fn collect_semantic_ranges_transition(
    transition: &sysml_v2_parser::Node<Transition>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    out.push((span_to_source_range(&transition.span), TYPE_PROPERTY));
    let value = &transition.value;
    if let Some(ref accept) = value.accept {
        collect_semantic_ranges_transition_accept(accept, out);
    }
    out.push((span_to_source_range(&value.target.span), TYPE_PROPERTY));
}

fn collect_semantic_ranges_state_def_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<StateDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use StateDefBodyElement as SDBE;
    match &node.value {
        SDBE::StateUsage(state_usage) => {
            collect_semantic_ranges_state_usage(ctx, state_usage, out)
        }
        SDBE::Transition(transition) => collect_semantic_ranges_transition(transition, out),
        SDBE::Then(then_stmt) => collect_semantic_ranges_then_stmt(&then_stmt.value, out),
        SDBE::FinalState(final_state) => {
            collect_semantic_ranges_final_state(&final_state.value, out)
        }
        SDBE::Ref(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        SDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(mk_node, out);
        }
        SDBE::MetadataAnnotation(meta) => collect_semantic_ranges_metadata_annotation(meta, out),
        SDBE::RequirementUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ru_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            if let RequirementDefBody::Brace { elements } = &ru_node.body {
                for element in elements {
                    collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                }
            }
        }
        SDBE::Entry(_)
        | SDBE::Do(_)
        | SDBE::Exit(_)
        | SDBE::Doc(_)
        | SDBE::Error(_)
        | SDBE::Annotation(_)
        | SDBE::Other(_) => {}
    }
}

fn collect_semantic_ranges_occurrence_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<OccurrenceBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use OccurrenceBodyElement as OBE;
    match &node.value {
        OBE::AttributeUsage(attribute) => {
            push_usage_name_type_spans(
                ctx.source,
                &attribute.span,
                &attribute.value.name,
                attribute.value.typing.as_deref(),
                attribute.value.name_span.as_ref(),
                attribute.value.typing_span.as_ref(),
                out,
            );
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
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        OBE::OccurrenceUsage(occurrence_usage) => {
            out.push((span_to_source_range(&occurrence_usage.span), TYPE_PROPERTY));
            if let OccurrenceUsageBody::Brace { elements } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, child, out);
                }
            }
        }
        OBE::FlowUsage(flow) => {
            out.push((span_to_source_range(&flow.span), TYPE_PROPERTY));
            collect_semantic_ranges_definition_body(ctx, &flow.value.body, out);
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
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => {
            push_usage_name_type_spans(
                ctx.source,
                &n.span,
                &n.value.name,
                n.value.typing.as_deref(),
                n.value.name_span.as_ref(),
                n.value.typing_span.as_ref(),
                out,
            );
        }
        PDBE::AttributeUsage(n) => {
            push_usage_name_type_spans(
                ctx.source,
                &n.span,
                &n.value.name,
                n.value.typing.as_deref(),
                n.value.name_span.as_ref(),
                n.value.typing_span.as_ref(),
                out,
            );
        }
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
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        PDBE::Ref(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        PDBE::ItemDef(id_node) => {
            push_ident_definition_spans(
                &id_node.span,
                id_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &id_node.value.body, out);
        }
        PDBE::ItemUsage(item_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &item_node.span,
                &item_node.value.name,
                item_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &item_node.body, out);
        }
        PDBE::PartDef(pd_node) => {
            push_ident_definition_spans(
                &pd_node.span,
                pd_node.value.specializes_span.as_ref(),
                TYPE_CLASS,
                out,
            );
            if let PartDefBody::Brace { elements } = &pd_node.body {
                for element in elements {
                    collect_semantic_ranges_part_def_body_element(ctx, element, out);
                }
            }
        }
        PDBE::OccurrenceUsage(occurrence_usage) => {
            out.push((span_to_source_range(&occurrence_usage.span), TYPE_PROPERTY));
            if let OccurrenceUsageBody::Brace { elements } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, child, out);
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
        PDBE::ExhibitState(exhibit) => {
            push_usage_name_type_spans(
                ctx.source,
                &exhibit.span,
                &exhibit.value.name,
                exhibit.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            if let StateDefBody::Brace { elements } = &exhibit.value.body {
                for element in elements {
                    collect_semantic_ranges_state_def_body_element(ctx, element, out);
                }
            }
        }
        PDBE::CalcUsage(calc_node) => {
            out.push((span_to_source_range(&calc_node.span), TYPE_FUNCTION));
            if let CalcDefBody::Brace { .. } = &calc_node.value.body {}
        }
        PDBE::EnumerationUsage(enum_node) => {
            out.push((span_to_source_range(&enum_node.span), TYPE_PROPERTY));
            collect_semantic_ranges_attribute_body(ctx, &enum_node.body, out);
        }
        PDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(mk_node, out);
        }
        PDBE::MetadataAnnotation(meta) => collect_semantic_ranges_metadata_annotation(meta, out),
        PDBE::RequirementUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ru_node.value.type_name.as_deref(),
                None,
                None,
                out,
            );
            match &ru_node.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        PDBE::FlowUsage(flow) => {
            out.push((span_to_source_range(&flow.span), TYPE_PROPERTY));
            collect_semantic_ranges_definition_body(ctx, &flow.value.body, out);
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
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<PartUsageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => {
            push_usage_name_type_spans(
                ctx.source,
                &n.span,
                &n.value.name,
                n.value.typing.as_deref(),
                n.value.name_span.as_ref(),
                n.value.typing_span.as_ref(),
                out,
            );
        }
        PUBE::PartUsage(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), TYPE_PROPERTY));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), TYPE_TYPE));
            }
            if let PartUsageBody::Brace { elements } = &n.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
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

fn collect_semantic_ranges_action_usage(
    ctx: &RangeCtx<'_>,
    usage: &ActionUsage,
    out: &mut Vec<(SourceRange, u32)>,
) {
    if let Some(ref span) = usage.name_span {
        out.push((span_to_source_range(span), TYPE_PROPERTY));
    }
    if let Some(ref span) = usage.type_ref_span {
        out.push((span_to_source_range(span), TYPE_TYPE));
    }
    if let Some(ref accept) = usage.accept {
        collect_semantic_ranges_payload_clause(accept, out);
    }
    if let Some(ref send) = usage.send {
        collect_semantic_ranges_payload_clause(send, out);
    }
    if let ActionUsageBody::Brace { elements } = &usage.body {
        for element in elements {
            collect_semantic_ranges_action_usage_body_element(ctx, element, out);
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
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<StateUsage>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    push_usage_name_type_spans(
        ctx.source,
        &node.span,
        &node.value.name,
        node.value.type_name.as_deref(),
        None,
        None,
        out,
    );
    if let StateDefBody::Brace { elements } = &node.value.body {
        for element in elements {
            collect_semantic_ranges_state_def_body_element(ctx, element, out);
        }
    }
}

fn collect_semantic_ranges_requirement_def_body_element(
    ctx: &RangeCtx<'_>,
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
                span_to_source_range(&stakeholder.value.name_span),
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
            push_usage_name_type_spans(
                ctx.source,
                &attribute.span,
                &attribute.value.name,
                attribute.value.typing.as_deref(),
                attribute.value.name_span.as_ref(),
                attribute.value.typing_span.as_ref(),
                out,
            );
        }
        RDBE::AttributeUsage(attribute) => {
            push_usage_name_type_spans(
                ctx.source,
                &attribute.span,
                &attribute.value.name,
                attribute.value.typing.as_deref(),
                attribute.value.name_span.as_ref(),
                attribute.value.typing_span.as_ref(),
                out,
            );
        }
        RDBE::VerifyRequirement(verify) => {
            if let Some(requirement) = &verify.value.requirement {
                out.push((span_to_source_range(&requirement.span), TYPE_PROPERTY));
            }
        }
        RDBE::RequireConstraint(constraint) => {
            if let RequireConstraintBody::Brace { elements } = &constraint.value.body {
                for element in elements {
                    match &element.value {
                        ConstraintDefBodyElement::InOutDecl(param) => {
                            out.push((span_to_source_range(&param.span), TYPE_PROPERTY));
                        }
                        ConstraintDefBodyElement::MetadataAnnotation(meta) => {
                            collect_semantic_ranges_metadata_annotation(meta, out);
                        }
                        _ => {}
                    }
                }
            }
        }
        RDBE::Frame(frame) => {
            out.push((span_to_source_range(&frame.span), TYPE_NAMESPACE));
            match &frame.value.body {
                RequirementDefBody::Brace { elements } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon => {}
            }
        }
        RDBE::Import(import) => {
            out.push((span_to_source_range(&import.value.target_span), TYPE_NAMESPACE))
        }
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
        RDBE::Doc(_) | RDBE::Error(_) | RDBE::Other(_) | RDBE::Annotation(_) => {}
        RDBE::MetadataAnnotation(meta) => collect_semantic_ranges_metadata_annotation(meta, out),
    }
}

fn collect_semantic_ranges_action_def_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<ActionDefBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use ActionDefBodyElement as ADBE;
    match &node.value {
        ADBE::InOutDecl(in_out) => out.push((span_to_source_range(&in_out.span), TYPE_PROPERTY)),
        ADBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(ctx, usage.as_ref(), out),
        ADBE::ThenAction(then_action) => {
            collect_semantic_ranges_action_usage(ctx, &then_action.value.action.value, out);
        }
        ADBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        ADBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        ADBE::Perform(perform) => out.push((span_to_source_range(&perform.span), TYPE_FUNCTION)),
        ADBE::Assign(assign) => out.push((span_to_source_range(&assign.span), TYPE_PROPERTY)),
        ADBE::ForLoop(for_loop) => out.push((span_to_source_range(&for_loop.span), TYPE_PROPERTY)),
        ADBE::Bind(_)
        | ADBE::FlowUsage(_)
        | ADBE::FirstStmt(_)
        | ADBE::MergeStmt(_)
        | ADBE::DecisionStmt(_)
        | ADBE::JoinStmt(_)
        | ADBE::ForkStmt(_)
        | ADBE::Decl(_)
        | ADBE::Error(_)
        | ADBE::Doc(_)
        | ADBE::Annotation(_) => {}
        ADBE::MetadataAnnotation(meta) => collect_semantic_ranges_metadata_annotation(meta, out),
        ADBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(mk_node, out);
        }
    }
}

fn collect_semantic_ranges_action_usage_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<ActionUsageBodyElement>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    use sysml_v2_parser::ast::ActionUsageBodyElement as AUBE;
    match &node.value {
        AUBE::InOutDecl(in_out) => out.push((span_to_source_range(&in_out.span), TYPE_PROPERTY)),
        AUBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(ctx, usage.as_ref(), out),
        AUBE::ThenAction(then_action) => {
            collect_semantic_ranges_action_usage(ctx, &then_action.value.action.value, out);
        }
        AUBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        AUBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        AUBE::Assign(assign) => out.push((span_to_source_range(&assign.span), TYPE_PROPERTY)),
        AUBE::ForLoop(for_loop) => out.push((span_to_source_range(&for_loop.span), TYPE_PROPERTY)),
        AUBE::Bind(_)
        | AUBE::FlowUsage(_)
        | AUBE::FirstStmt(_)
        | AUBE::MergeStmt(_)
        | AUBE::DecisionStmt(_)
        | AUBE::JoinStmt(_)
        | AUBE::ForkStmt(_)
        | AUBE::Error(_)
        | AUBE::Doc(_)
        | AUBE::Annotation(_)
        | AUBE::Decl(_) => {}
        AUBE::MetadataAnnotation(meta) => collect_semantic_ranges_metadata_annotation(meta, out),
        AUBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(mk_node, out);
        }
    }
}
