//! AST-driven syntax roles: what each span *is*, collected from the parsed document.
//!
//! Moved here from `sysml_tokens` because it walks the AST, which only the parser authority may
//! do. It emits [`SyntaxRole`]s rather than editor token indices: identifying a span as a type
//! reference is the grammar's business, deciding what colour that gets is not.

use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody, ActionUsageBodyElement,
    AttributeBody, AttributeBodyElement, CalcDefBody, ConnectionDefBody, ConnectionDefBodyElement,
    ConstraintDefBody, ConstraintDefBodyElement, DefinitionBody, DefinitionBodyElement,
    EndIdentity, EnumerationBody, FinalState, InterfaceDefBody, InterfaceDefBodyElement,
    MetadataAnnotation, MetadataKeywordUsage, OccurrenceBodyElement, OccurrenceUsageBody,
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBody,
    PartUsageBodyElement, PayloadClause, PortBody, PortBodyElement, PortDefBody,
    PortDefBodyElement, RequirementDefBody, RequirementDefBodyElement, RootElement, StateDefBody,
    StateDefBodyElement, StateUsage, ThenStmt, Transition, TransitionAccept,
};
use sysml_v2_parser::{ParsedDocument, QualifiedReferenceId};

use super::token_util::{
    identification_name, modeled_decl_name, push_ident_definition_spans,
    push_usage_name_type_spans, push_word_token, qualified_identification_name,
    span_to_source_range,
};
use super::{SyntaxRange, SyntaxRole};
use sysml_v2_parser::ast::{Dependency, SatisfyRequirementUsage as Satisfy};

/// The four annotating members the grammar folds into one `AnnotatingMember` production.
///
/// `Doc`, `Comment` and `TextualRep` classify as ordinary comment trivia and contribute no
/// semantic-token ranges; only a metadata annotation names elements worth highlighting.
fn collect_semantic_ranges_annotating(
    member: &sysml_v2_parser::ast::AnnotatingMember,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::AnnotatingMember as AM;
    match member {
        AM::MetadataAnnotation(meta) => collect_semantic_ranges_metadata_annotation(meta, out),
        AM::Doc(_) | AM::Comment(_) | AM::TextualRep(_) => {}
    }
}

/// `SendPayload` is the send half of the payload grammar: either a typed parameter or an
/// expression. The accept half keeps its own `PayloadClause` collector.
fn collect_semantic_ranges_send_payload(
    ctx: &RangeCtx<'_>,
    payload: &sysml_v2_parser::ast::SendPayload,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::SendPayload as SP;
    match payload {
        SP::Typed(clause) => collect_semantic_ranges_payload_clause(clause, out),
        SP::Expression(expression) => {
            push_word_token(ctx.source, &expression.span, "", SyntaxRole::Property, out);
        }
    }
}

struct RangeCtx<'a> {
    source: &'a str,
    document: &'a ParsedDocument,
}

impl<'a> RangeCtx<'a> {
    /// Authored text of an arena-backed type reference.
    ///
    /// Type names are `QualifiedReferenceId`s now, not owned strings: the document owns the
    /// segments, separators and spans, so the only way back to what the author wrote is through
    /// its arena. Returning `&'a str` keeps the borrow tied to the document rather than copying
    /// every type name on every token pass.
    /// Authored text of a typing relationship's first target.
    ///
    /// `TypingRelationship.target` is a `Vec<QualifiedReferenceId>` now, so the relationship
    /// cannot answer "what type is this" on its own -- it needs the document that owns the arena.
    fn typing_text(
        &self,
        relationship: Option<&sysml_v2_parser::ast::TypingRelationship>,
    ) -> Option<&'a str> {
        self.type_text(relationship?.target.first().copied())
    }

    fn type_text(&self, reference: Option<QualifiedReferenceId>) -> Option<&'a str> {
        self.document
            .qualified_reference(reference?)
            .map(|view| view.authored_text())
    }
}

/// Collect every span the grammar gives a role, in source order.
pub fn semantic_token_roles(
    document: &super::SyntaxDocument,
    source: &str,
) -> Vec<(SyntaxRange, SyntaxRole)> {
    let document = document.inner();
    let ctx = RangeCtx { source, document };
    let mut out = Vec::new();
    for node in &document.elements {
        let elements = match &node.value {
            RootElement::Package(p) => match &p.body {
                PackageBody::Brace { elements, .. } => elements,
                _ => continue,
            },
            RootElement::Namespace(n) => match &n.body {
                PackageBody::Brace { elements, .. } => elements,
                _ => continue,
            },
            RootElement::LibraryPackage(lp) => match &lp.body {
                PackageBody::Brace { elements, .. } => elements,
                _ => continue,
            },
            RootElement::Import(_) => continue,
            RootElement::Member(_) => continue,
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
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(pkg_node) => {
            let name = qualified_identification_name(ctx.document, &pkg_node.identification);
            if !name.is_empty() {
                push_ident_definition_spans(&pkg_node.span, None, SyntaxRole::Namespace, out);
            }
            match &pkg_node.body {
                PackageBody::Brace { elements, .. } => {
                    for n in elements {
                        collect_semantic_ranges_package_body_element(ctx, n, out);
                    }
                }
                PackageBody::Semicolon { .. } => {}
            }
        }
        PBE::Import(imp_node) => {
            // Use the precise target span so only the qualified name is highlighted,
            // not the leading `import` keyword or trailing `::*` suffix.
            // The target is an arena identity now, so its precise span -- the qualified name
            // without the leading `import` keyword or a trailing `::*` -- comes from the document
            // rather than a span field on the node.
            out.push((
                span_to_source_range(&imp_node.value.target.span),
                SyntaxRole::Namespace,
            ));
        }
        PBE::PartDef(pd_node) => {
            push_ident_definition_spans(
                &pd_node.span,
                pd_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            match &pd_node.body {
                PartDefBody::Brace { elements, .. } => {
                    for n in elements {
                        collect_semantic_ranges_part_def_body_element(ctx, n, out);
                    }
                }
                PartDefBody::Semicolon { .. } => {}
            }
        }
        PBE::PartUsage(pu_node) => {
            if let Some(ref s) = pu_node.value.name_span {
                out.push((span_to_source_range(s), SyntaxRole::Property));
            }
            if let Some(ref s) = pu_node.value.type_ref_span {
                out.push((span_to_source_range(s), SyntaxRole::Type));
            }
            match &pu_node.body {
                PartUsageBody::Brace { elements, .. } => {
                    for n in elements {
                        collect_semantic_ranges_part_usage_body_element(ctx, n, out);
                    }
                }
                PartUsageBody::Semicolon { .. } => {}
            }
        }
        PBE::PortDef(pd_node) => {
            push_ident_definition_spans(&pd_node.span, None, SyntaxRole::Type, out);
            match &pd_node.body {
                PortDefBody::Brace { elements, .. } => {
                    for n in elements {
                        collect_semantic_ranges_port_def_body_element(n, out);
                    }
                }
                PortDefBody::Semicolon { .. } => {}
            }
        }
        PBE::InterfaceDef(id_node) => {
            push_ident_definition_spans(&id_node.span, None, SyntaxRole::Interface, out);
            match &id_node.body {
                InterfaceDefBody::Brace { elements, .. } => {
                    for n in elements {
                        collect_semantic_ranges_interface_def_body_element(n, out);
                    }
                }
                InterfaceDefBody::Semicolon { .. } => {}
            }
        }
        PBE::AttributeDef(ad_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ad_node.span,
                &ad_node.value.name,
                ctx.typing_text(ad_node.value.typing.as_deref()),
                ad_node.value.name_span.as_ref(),
                ad_node.value.typing_span.as_ref(),
                out,
            );
        }
        PBE::ActionDef(ad_node) => {
            push_ident_definition_spans(
                &ad_node.span,
                ad_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Function,
                out,
            );
            match &ad_node.body {
                ActionDefBody::Brace { elements, .. } => {
                    for element in elements {
                        collect_semantic_ranges_action_def_body_element(ctx, element, out);
                    }
                }
                ActionDefBody::Semicolon { .. } => {}
            }
        }
        PBE::RequirementDef(rd_node) => {
            push_ident_definition_spans(
                &rd_node.span,
                rd_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            match &rd_node.body {
                RequirementDefBody::Brace { elements, .. } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon { .. } => {}
            }
        }
        PBE::RequirementUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ctx.type_text(ru_node.value.type_name),
                None,
                None,
                out,
            );
            match &ru_node.body {
                RequirementDefBody::Brace { elements, .. } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon { .. } => {}
            }
        }
        PBE::ActionUsage(au_node) => {
            if let Some(ref s) = au_node.value.name_span {
                out.push((span_to_source_range(s), SyntaxRole::Property));
            }
            if let Some(ref s) = au_node.value.type_ref_span {
                out.push((span_to_source_range(s), SyntaxRole::Type));
            }
            if let Some(ActionUsageBody::Brace { elements, .. }) = &au_node.body {
                for n in elements {
                    collect_semantic_ranges_action_usage_body_element(ctx, n, out);
                }
            }
        }
        PBE::AliasDef(ad_node) => {
            push_ident_definition_spans(&ad_node.span, None, SyntaxRole::Namespace, out);
        }
        PBE::ViewDef(vd_node) => {
            push_ident_definition_spans(&vd_node.span, None, SyntaxRole::Namespace, out);
        }
        PBE::ViewpointDef(vpd_node) => {
            push_ident_definition_spans(&vpd_node.span, None, SyntaxRole::Namespace, out);
        }
        PBE::RenderingDef(rd_node) => {
            push_ident_definition_spans(&rd_node.span, None, SyntaxRole::Namespace, out);
        }
        PBE::ViewUsage(vu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &vu_node.span,
                &vu_node.value.name,
                ctx.type_text(vu_node.value.type_name),
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
                ctx.type_text(vpu_node.value.type_name),
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
                ctx.type_text(ru_node.value.type_name),
                None,
                None,
                out,
            );
        }
        PBE::ItemDef(id_node) => {
            push_ident_definition_spans(
                &id_node.span,
                id_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &id_node.value.body, out);
        }
        PBE::IndividualDef(id_node) => {
            push_ident_definition_spans(
                &id_node.span,
                id_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &id_node.value.body, out);
        }
        PBE::MetadataDef(md_node) => {
            push_ident_definition_spans(
                &md_node.span,
                md_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &md_node.value.body, out);
        }
        PBE::OccurrenceDef(occ_node) => {
            push_ident_definition_spans(
                &occ_node.span,
                occ_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            collect_semantic_ranges_definition_body(ctx, &occ_node.value.body, out);
        }
        PBE::FlowDef(flow_node) => {
            push_ident_definition_spans(
                &flow_node.span,
                flow_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Interface,
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
                    ctx.type_text(flow_node.value.type_name),
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
                alloc_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Interface,
                out,
            );
            collect_semantic_ranges_definition_body(ctx, &alloc_node.value.body, out);
        }
        PBE::StateDef(sd_node) => {
            push_ident_definition_spans(
                &sd_node.span,
                sd_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            if let StateDefBody::Brace { elements, .. } = &sd_node.body {
                for element in elements {
                    collect_semantic_ranges_state_def_body_element(ctx, element, out);
                }
            }
        }
        PBE::StateUsage(su_node) => {
            collect_semantic_ranges_state_usage(ctx, su_node, out);
        }
        PBE::ConnectionDef(conn_node) => {
            push_ident_definition_spans(&conn_node.span, None, SyntaxRole::Interface, out);
            if let ConnectionDefBody::Brace { elements, .. } = &conn_node.body {
                for element in elements {
                    collect_semantic_ranges_connection_def_body_element(element, out);
                }
            }
        }
        PBE::ConstraintDef(cd_node) => {
            push_ident_definition_spans(
                &cd_node.span,
                cd_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
        }
        PBE::CalcDef(calc_node) => {
            push_ident_definition_spans(&calc_node.span, None, SyntaxRole::Function, out);
        }
        PBE::EnumDef(enum_node) => {
            push_ident_definition_spans(
                &enum_node.span,
                enum_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            // The enum's own name-vs-body narrowing (see `refine_declaration_ranges`) only
            // stops the class color from bleeding onto the body; it doesn't give the literal
            // members (`entry;`, `standard;`, ...) any color of their own. They read like named
            // constants, same as `EnumerationUsage`'s own name elsewhere in this file, so give
            // them the matching token instead of leaving them accidentally uncolored.
            if let EnumerationBody::Brace { elements, .. } = &enum_node.value.body {
                use sysml_v2_parser::ast::EnumerationBodyElement as EBE;
                for element in elements {
                    match &element.value {
                        EBE::Value(value) => {
                            out.push((span_to_source_range(&value.span), SyntaxRole::Property));
                        }
                        EBE::Annotating(member) => {
                            collect_semantic_ranges_annotating(member, out);
                        }
                        EBE::Error(_) => {}
                    }
                }
            }
        }
        PBE::UseCaseDef(uc_node) => {
            push_ident_definition_spans(
                &uc_node.span,
                uc_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
        }
        PBE::VerificationCaseDef(vc_node) => {
            push_ident_definition_spans(
                &vc_node.span,
                vc_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
        }
        PBE::CaseDef(case_node) => {
            push_ident_definition_spans(
                &case_node.span,
                case_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
        }
        PBE::AnalysisCaseDef(ac_node) => {
            push_ident_definition_spans(
                &ac_node.span,
                ac_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
        }
        PBE::MetadataUsage(mu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &mu_node.span,
                &mu_node.value.name,
                ctx.type_text(mu_node.value.type_reference),
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
                ctx.type_text(ou_node.value.type_name),
                None,
                None,
                out,
            );
            if let OccurrenceUsageBody::Brace { elements, .. } = &ou_node.value.body {
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
                ctx.type_text(au_node.value.type_name),
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
                ctx.type_text(cu_node.value.type_name),
                None,
                None,
                out,
            );
            match &cu_node.value.body {
                RequirementDefBody::Brace { elements, .. } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon { .. } => {}
            }
        }
        PBE::UseCaseUsage(ucu_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ucu_node.span,
                &ucu_node.value.name,
                ctx.type_text(ucu_node.value.type_name),
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
                ctx.type_text(vcu_node.value.type_name),
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
                ctx.type_text(cu_node.value.type_name),
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
                ctx.type_text(acu_node.value.type_name),
                None,
                None,
                out,
            );
        }
        PBE::Actor(actor_node) => {
            let name = identification_name(&actor_node.value.identification);
            if !name.is_empty() {
                push_ident_definition_spans(&actor_node.span, None, SyntaxRole::Property, out);
            }
        }
        PBE::Satisfy(satisfy_node) => collect_semantic_ranges_satisfy(ctx, satisfy_node, out),
        PBE::Dependency(dep_node) => collect_semantic_ranges_dependency(ctx, dep_node, out),
        PBE::FeatureDecl(feature_node) => {
            collect_semantic_ranges_modeled_decl(
                ctx,
                &feature_node.span,
                &feature_node.value.keyword,
                &feature_node.value.text,
                SyntaxRole::Property,
                out,
            );
        }
        PBE::ClassifierDecl(classifier_node) => {
            collect_semantic_ranges_modeled_decl(
                ctx,
                &classifier_node.span,
                &classifier_node.value.keyword,
                &classifier_node.value.text,
                SyntaxRole::Class,
                out,
            );
        }
        PBE::KermlFeatureDecl(decl) => {
            collect_semantic_ranges_modeled_decl(
                ctx,
                &decl.span,
                &decl.value.bnf_production,
                &decl.value.text,
                SyntaxRole::Property,
                out,
            );
        }
        PBE::KermlSemanticDecl(decl) => {
            collect_semantic_ranges_modeled_decl(
                ctx,
                &decl.span,
                &decl.value.bnf_production,
                &decl.value.text,
                SyntaxRole::Class,
                out,
            );
        }
        // `datatype Magnitude;`, `struct S { ... }`, `feature baseType;` -- these arrive as typed
        // nodes now, where they used to be opaque `KermlSemanticDecl`/`KermlFeatureDecl` raw text
        // whose declared name had to be recovered by re-scanning the source. Classify them from
        // the node instead, which is both cheaper and exact.
        PBE::KermlClassifier(decl) => {
            if let Some(name) = decl.value.identification.name.as_deref() {
                push_word_token(ctx.source, &decl.span, name, SyntaxRole::Class, out);
            }
        }
        PBE::KermlFeature(decl) => {
            if !decl.value.name.is_empty() {
                push_word_token(
                    ctx.source,
                    &decl.span,
                    &decl.value.name,
                    SyntaxRole::Property,
                    out,
                );
            }
        }
        PBE::ExtendedLibraryDecl(decl) => {
            collect_semantic_ranges_modeled_decl(
                ctx,
                &decl.span,
                &decl.value.bnf_production,
                &decl.value.text,
                SyntaxRole::Class,
                out,
            );
        }
        _ => {}
    }
}

fn collect_semantic_ranges_satisfy(
    ctx: &RangeCtx<'_>,
    satisfy: &sysml_v2_parser::Node<Satisfy>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    // The production's two mutually exclusive requirement clauses: a reference to an existing
    // requirement, or an inline declaration of one. The `by` subject is a separate optional clause.
    use sysml_v2_parser::ast::SatisfiedRequirement;
    match &satisfy.value.requirement {
        SatisfiedRequirement::Reference { reference } => {
            if let Some(view) = ctx.document.qualified_reference(*reference) {
                out.push((
                    span_to_source_range(&view.metadata.span),
                    SyntaxRole::Property,
                ));
            }
        }
        SatisfiedRequirement::Declaration(declaration) => {
            if let Some(name) = declaration.value.identification.name.as_deref() {
                push_word_token(ctx.source, &satisfy.span, name, SyntaxRole::Property, out);
            }
        }
    }
    if let Some(subject) = &satisfy.value.subject {
        if let Some(view) = ctx.document.qualified_reference(subject.value.reference) {
            out.push((
                span_to_source_range(&view.metadata.span),
                SyntaxRole::Property,
            ));
        }
    }
    if let Some(relationship) = &satisfy.value.typing {
        if let Some(target) = relationship.value.target.first() {
            if let Some(view) = ctx.document.qualified_reference(*target) {
                out.push((span_to_source_range(&view.metadata.span), SyntaxRole::Type));
            }
        }
    }
}

fn collect_semantic_ranges_dependency(
    ctx: &RangeCtx<'_>,
    dependency: &sysml_v2_parser::Node<Dependency>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    if let Some(ident) = &dependency.value.identification {
        let name = identification_name(ident);
        push_word_token(
            ctx.source,
            &dependency.span,
            &name,
            SyntaxRole::Property,
            out,
        );
    }
    for reference in dependency
        .value
        .clients
        .iter()
        .chain(dependency.value.suppliers.iter())
    {
        if let Some(view) = ctx.document.qualified_reference(*reference) {
            out.push((
                span_to_source_range(&view.metadata.span),
                SyntaxRole::Property,
            ));
        }
    }
}

fn collect_semantic_ranges_modeled_decl(
    ctx: &RangeCtx<'_>,
    span: &sysml_v2_parser::Span,
    keyword: &str,
    text: &str,
    role: SyntaxRole,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    let Some(name) = modeled_decl_name(keyword, text) else {
        return;
    };
    push_word_token(ctx.source, span, &name, role, out);
}

fn collect_semantic_ranges_definition_body(
    ctx: &RangeCtx<'_>,
    body: &DefinitionBody,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    let DefinitionBody::Brace { elements, .. } = body else {
        return;
    };
    for node in elements {
        match &node.value {
            DefinitionBodyElement::OccurrenceMember(member) => {
                collect_semantic_ranges_occurrence_body_element(ctx, member, out);
            }
            DefinitionBodyElement::Error(_) => {}
            // Member kinds this collector assigns no token of their own.
            DefinitionBodyElement::Unsupported(_) => {}
        }
    }
}

fn collect_semantic_ranges_attribute_body(
    ctx: &RangeCtx<'_>,
    body: &AttributeBody,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    let AttributeBody::Brace { elements, .. } = body else {
        return;
    };
    for node in elements {
        match &node.value {
            AttributeBodyElement::AttributeDef(attribute) => {
                push_usage_name_type_spans(
                    ctx.source,
                    &attribute.span,
                    &attribute.value.name,
                    ctx.typing_text(attribute.value.typing.as_deref()),
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
                    ctx.typing_text(attribute.value.typing.as_deref()),
                    attribute.value.name_span.as_ref(),
                    attribute.value.typing_span.as_ref(),
                    out,
                );
            }
            AttributeBodyElement::MetadataKeywordUsage(mk_node) => {
                collect_semantic_ranges_metadata_keyword_usage(ctx, mk_node, out);
            }
            // §6 G27: this body is shared with `item def`/`item` usage bodies. `Connect` has no
            // dedicated highlighting elsewhere in this file either (see e.g. `PDBE::Connect`).
            AttributeBodyElement::OccurrenceUsage(_) | AttributeBodyElement::Connect(_) => {}
            AttributeBodyElement::Error(_) => {}
            // `ref`/`ref part` members (validation `15_11`/`15_19`/`17a`/`17b`) -- same
            // collector every other body kind's `RefDecl` arm already uses.
            AttributeBodyElement::RefDecl(ref_decl) => {
                collect_semantic_ranges_ref_decl(ref_decl, out);
            }
            // Nested `part` usage inside an item/attribute body (validation `3e`/`14c`) -- same
            // shape `OBE::PartUsage` above already highlights.
            AttributeBodyElement::PartUsage(part_usage) => {
                if let Some(ref span) = part_usage.value.name_span {
                    out.push((span_to_source_range(span), SyntaxRole::Property));
                }
                if let Some(ref span) = part_usage.value.type_ref_span {
                    out.push((span_to_source_range(span), SyntaxRole::Type));
                }
                if let PartUsageBody::Brace { elements, .. } = &part_usage.value.body {
                    for child in elements {
                        collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                    }
                }
            }
            // No dedicated highlighting yet (mirrors every other body kind's `AssertConstraint`
            // arm in this file).
            AttributeBodyElement::AssertConstraint(_) => {}
            // Member kinds this collector assigns no token of their own.
            AttributeBodyElement::Unsupported(_)
            | AttributeBodyElement::Annotating(_)
            | AttributeBodyElement::ItemUsage(_)
            | AttributeBodyElement::KermlFeature(_)
            | AttributeBodyElement::Invariant(_)
            | AttributeBodyElement::KermlConnector(_)
            | AttributeBodyElement::KermlClassifier(_)
            | AttributeBodyElement::Bind(_)
            | AttributeBodyElement::Connection(_)
            | AttributeBodyElement::CalcDef(_)
            | AttributeBodyElement::CalcUsage(_)
            | AttributeBodyElement::ConstraintUsage(_)
            | AttributeBodyElement::DefaultReferenceUsage(_)
            | AttributeBodyElement::VariantUsage(_) => {}
        }
    }
}

fn collect_semantic_ranges_metadata_keyword_usage(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<MetadataKeywordUsage>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    out.push((
        span_to_source_range(&node.value.hash_span),
        SyntaxRole::Property,
    ));
    if let Some(view) = ctx.document.qualified_reference(node.value.reference) {
        out.push((span_to_source_range(&view.metadata.span), SyntaxRole::Type));
    }
}

fn collect_semantic_ranges_metadata_annotation(
    node: &sysml_v2_parser::Node<MetadataAnnotation>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::MetadataFeatureIntroducer as MFI;
    let introducer = match &node.value.introducer {
        MFI::At { span } | MFI::Metadata { span } => span,
    };
    out.push((span_to_source_range(introducer), SyntaxRole::Property));
    out.push((
        span_to_source_range(&node.value.type_span),
        SyntaxRole::Type,
    ));
}

fn collect_semantic_ranges_payload_clause(
    clause: &PayloadClause,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    out.push((
        span_to_source_range(&clause.name_span),
        SyntaxRole::Property,
    ));
    if let Some(ref span) = clause.type_span {
        out.push((span_to_source_range(span), SyntaxRole::Type));
    }
}

fn collect_semantic_ranges_transition_accept(
    accept: &TransitionAccept,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    match accept {
        TransitionAccept::Payload(clause, _via) => {
            collect_semantic_ranges_payload_clause(clause, out)
        }
        TransitionAccept::Shorthand(expr, _via) => {
            out.push((span_to_source_range(&expr.span), SyntaxRole::Property));
        }
        TransitionAccept::TimeTrigger(_kind, expr) => {
            out.push((span_to_source_range(&expr.span), SyntaxRole::Property));
        }
    }
}

fn collect_semantic_ranges_then_stmt(
    ctx: &RangeCtx<'_>,
    then_stmt: &ThenStmt,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    if let Some(view) = ctx.document.qualified_reference(then_stmt.state_reference) {
        out.push((
            span_to_source_range(&view.metadata.span),
            SyntaxRole::Property,
        ));
    }
}

fn collect_semantic_ranges_final_state(
    final_state: &FinalState,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    out.push((
        span_to_source_range(&final_state.name_span),
        SyntaxRole::Property,
    ));
}

fn collect_semantic_ranges_transition(
    transition: &sysml_v2_parser::Node<Transition>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    out.push((span_to_source_range(&transition.span), SyntaxRole::Property));
    let value = &transition.value;
    if let Some(ref accept) = value.accept {
        collect_semantic_ranges_transition_accept(accept, out);
    }
    out.push((
        span_to_source_range(&value.target.span),
        SyntaxRole::Property,
    ));
}

fn collect_semantic_ranges_state_def_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<StateDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use StateDefBodyElement as SDBE;
    match &node.value {
        SDBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        SDBE::Transition(transition) => collect_semantic_ranges_transition(transition, out),
        SDBE::Then(then_stmt) => collect_semantic_ranges_then_stmt(ctx, &then_stmt.value, out),
        SDBE::FinalState(final_state) => {
            collect_semantic_ranges_final_state(&final_state.value, out)
        }
        SDBE::Ref(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        SDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(ctx, mk_node, out);
        }
        SDBE::Annotating(member) => collect_semantic_ranges_annotating(member, out),
        SDBE::RequirementUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ctx.type_text(ru_node.value.type_name),
                None,
                None,
                out,
            );
            if let RequirementDefBody::Brace { elements, .. } = &ru_node.body {
                for element in elements {
                    collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                }
            }
        }
        SDBE::InOutDecl(in_out) => {
            out.push((span_to_source_range(&in_out.span), SyntaxRole::Property))
        }
        SDBE::Entry(_) | SDBE::Do(_) | SDBE::Exit(_) | SDBE::Error(_) => {}
        // Member kinds this collector assigns no token of their own.
        SDBE::AttributeUsage(_)
        | SDBE::ActionUsage(_)
        | SDBE::SuccessionUsage(_)
        | SDBE::PartUsage(_)
        | SDBE::ConstraintUsage(_)
        | SDBE::AssertConstraint(_) => {}
    }
}

fn collect_semantic_ranges_occurrence_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<OccurrenceBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use OccurrenceBodyElement as OBE;
    match &node.value {
        OBE::AttributeUsage(attribute) => {
            push_usage_name_type_spans(
                ctx.source,
                &attribute.span,
                &attribute.value.name,
                ctx.typing_text(attribute.value.typing.as_deref()),
                attribute.value.name_span.as_ref(),
                attribute.value.typing_span.as_ref(),
                out,
            );
        }
        OBE::PartUsage(part_usage) => {
            if let Some(ref span) = part_usage.value.name_span {
                out.push((span_to_source_range(span), SyntaxRole::Property));
            }
            if let Some(ref span) = part_usage.value.type_ref_span {
                out.push((span_to_source_range(span), SyntaxRole::Type));
            }
            if let PartUsageBody::Brace { elements, .. } = &part_usage.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        OBE::OccurrenceUsage(occurrence_usage) => {
            out.push((
                span_to_source_range(&occurrence_usage.span),
                SyntaxRole::Property,
            ));
            if let OccurrenceUsageBody::Brace { elements, .. } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, child, out);
                }
            }
        }
        OBE::FlowUsage(flow) => {
            out.push((span_to_source_range(&flow.span), SyntaxRole::Property));
            collect_semantic_ranges_definition_body(ctx, &flow.value.body, out);
        }
        OBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        // `end name : Type;` (or nested forms) inside allocation/connection-like definition
        // bodies -- same highlighting `CDBE::EndDecl`/`IDBE::EndDecl` already use.
        OBE::EndDecl(end_decl) => {
            if let EndIdentity::Declaration(label) = &end_decl.value.identity {
                out.push((span_to_source_range(&label.span), SyntaxRole::Property));
            }
            if let Some(ref span) = end_decl.type_ref_span {
                out.push((span_to_source_range(span), SyntaxRole::Type));
            }
        }
        OBE::Satisfy(satisfy) => collect_semantic_ranges_satisfy(ctx, satisfy, out),
        OBE::Error(_)
        | OBE::AssertConstraint(_)
        | OBE::Allocate(_)
        | OBE::Bind(_)
        | OBE::SuccessionUsage(_) => {}
        // Member kinds this collector assigns no token of their own.
        OBE::Annotating(_)
        | OBE::MetadataKeywordUsage(_)
        | OBE::ItemUsage(_)
        | OBE::RefDecl(_)
        | OBE::ConnectionUsage(_) => {}
    }
}

fn collect_semantic_ranges_connection_def_body_element(
    node: &sysml_v2_parser::Node<ConnectionDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use ConnectionDefBodyElement as CDBE;
    match &node.value {
        CDBE::EndDecl(end_decl) => {
            if let EndIdentity::Declaration(label) = &end_decl.value.identity {
                out.push((span_to_source_range(&label.span), SyntaxRole::Property));
            }
            if let Some(ref span) = end_decl.type_ref_span {
                out.push((span_to_source_range(span), SyntaxRole::Type));
            }
        }
        CDBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        CDBE::ConnectStmt(_) | CDBE::Error(_) => {}
        _ => {}
    }
}

fn collect_semantic_ranges_part_def_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => {
            push_usage_name_type_spans(
                ctx.source,
                &n.span,
                &n.value.name,
                ctx.typing_text(n.value.typing.as_deref()),
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
                ctx.typing_text(n.value.typing.as_deref()),
                n.value.name_span.as_ref(),
                n.value.typing_span.as_ref(),
                out,
            );
        }
        PDBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PDBE::PartUsage(pu_node) => {
            if let Some(ref span) = pu_node.value.name_span {
                out.push((span_to_source_range(span), SyntaxRole::Property));
            }
            if let Some(ref span) = pu_node.value.type_ref_span {
                out.push((span_to_source_range(span), SyntaxRole::Type));
            }
            if let PartUsageBody::Brace { elements, .. } = &pu_node.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        PDBE::Ref(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        PDBE::ItemDef(id_node) => {
            push_ident_definition_spans(
                &id_node.span,
                id_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &id_node.value.body, out);
        }
        PDBE::ItemUsage(item_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &item_node.span,
                &item_node.value.name,
                ctx.type_text(item_node.value.type_name),
                None,
                None,
                out,
            );
            collect_semantic_ranges_attribute_body(ctx, &item_node.body, out);
        }
        PDBE::PartDef(pd_node) => {
            push_ident_definition_spans(
                &pd_node.span,
                pd_node
                    .value
                    .specializes
                    .as_ref()
                    .map(|relationship| &relationship.value.span),
                SyntaxRole::Class,
                out,
            );
            if let PartDefBody::Brace { elements, .. } = &pd_node.body {
                for element in elements {
                    collect_semantic_ranges_part_def_body_element(ctx, element, out);
                }
            }
        }
        PDBE::OccurrenceUsage(occurrence_usage) => {
            out.push((
                span_to_source_range(&occurrence_usage.span),
                SyntaxRole::Property,
            ));
            if let OccurrenceUsageBody::Brace { elements, .. } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, child, out);
                }
            }
        }
        PDBE::InterfaceDef(id_node) => {
            out.push((span_to_source_range(&id_node.span), SyntaxRole::Interface));
            if let InterfaceDefBody::Brace { elements, .. } = &id_node.body {
                for element in elements {
                    collect_semantic_ranges_interface_def_body_element(element, out);
                }
            }
        }
        PDBE::Connection(connection_usage) => {
            out.push((
                span_to_source_range(&connection_usage.span),
                SyntaxRole::Property,
            ));
            if let ConnectionDefBody::Brace { elements, .. } = &connection_usage.value.body {
                for element in elements {
                    collect_semantic_ranges_connection_def_body_element(element, out);
                }
            }
        }
        PDBE::Perform(perform) => {
            out.push((span_to_source_range(&perform.span), SyntaxRole::Function))
        }
        PDBE::ExhibitState(exhibit) => {
            push_usage_name_type_spans(
                ctx.source,
                &exhibit.span,
                &exhibit.value.name,
                exhibit
                    .value
                    .typing
                    .as_ref()
                    .and_then(|relationship| relationship.value.target.first().copied())
                    .and_then(|target| ctx.type_text(Some(target))),
                None,
                None,
                out,
            );
            if let StateDefBody::Brace { elements, .. } = &exhibit.value.body {
                for element in elements {
                    collect_semantic_ranges_state_def_body_element(ctx, element, out);
                }
            }
        }
        PDBE::CalcUsage(calc_node) => {
            out.push((span_to_source_range(&calc_node.span), SyntaxRole::Function));
            if let CalcDefBody::Brace { .. } = &calc_node.value.body {}
        }
        PDBE::EnumerationUsage(enum_node) => {
            out.push((span_to_source_range(&enum_node.span), SyntaxRole::Property));
            collect_semantic_ranges_attribute_body(ctx, &enum_node.body, out);
        }
        PDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(ctx, mk_node, out);
        }
        PDBE::Annotating(member) => collect_semantic_ranges_annotating(member, out),
        PDBE::RequirementUsage(ru_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &ru_node.span,
                &ru_node.value.name,
                ctx.type_text(ru_node.value.type_name),
                None,
                None,
                out,
            );
            match &ru_node.body {
                RequirementDefBody::Brace { elements, .. } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon { .. } => {}
            }
        }
        PDBE::FlowUsage(flow) => {
            out.push((span_to_source_range(&flow.span), SyntaxRole::Property));
            collect_semantic_ranges_definition_body(ctx, &flow.value.body, out);
        }
        PDBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(ctx, usage.as_ref(), out),
        PDBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        PDBE::Satisfy(satisfy) => collect_semantic_ranges_satisfy(ctx, satisfy, out),
        PDBE::Dependency(dep) => collect_semantic_ranges_dependency(ctx, dep, out),
        PDBE::Connect(_)
        | PDBE::InterfaceUsage(_)
        | PDBE::Allocate(_)
        | PDBE::Bind(_)
        | PDBE::Error(_)
        | PDBE::AssertConstraint(_) => {}
        PDBE::VariantUsage(n) => {
            out.push((span_to_source_range(&n.span), SyntaxRole::Property));
        }
        _ => {}
    }
}

fn collect_semantic_ranges_part_usage_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<PartUsageBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => {
            push_usage_name_type_spans(
                ctx.source,
                &n.span,
                &n.value.name,
                ctx.typing_text(n.value.typing.as_deref()),
                n.value.name_span.as_ref(),
                n.value.typing_span.as_ref(),
                out,
            );
        }
        PUBE::PartUsage(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), SyntaxRole::Property));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), SyntaxRole::Type));
            }
            if let PartUsageBody::Brace { elements, .. } = &n.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        PUBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PUBE::Ref(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), SyntaxRole::Property));
            } else {
                out.push((span_to_source_range(&n.span), SyntaxRole::Property));
            }
            if let Some(ref s) = n.value.type_ref_span {
                out.push((span_to_source_range(s), SyntaxRole::Type));
            }
        }
        PUBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(ctx, usage.as_ref(), out),
        PUBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        PUBE::Satisfy(satisfy) => collect_semantic_ranges_satisfy(ctx, satisfy, out),
        _ => {}
    }
}

fn collect_semantic_ranges_port_usage(
    n: &sysml_v2_parser::Node<sysml_v2_parser::ast::PortUsage>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    if let Some(ref s) = n.value.name_span {
        out.push((span_to_source_range(s), SyntaxRole::Property));
    }
    if let Some(ref s) = n.value.type_ref_span {
        out.push((span_to_source_range(s), SyntaxRole::Type));
    }
    if let PortBody::Brace { elements, .. } = &n.body {
        for child in elements {
            collect_semantic_ranges_port_body_element(child, out);
        }
    }
}

fn collect_semantic_ranges_port_body_element(
    node: &sysml_v2_parser::Node<PortBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use PortBodyElement as PBE;
    match &node.value {
        PBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PBE::InOutDecl(w) => {
            out.push((span_to_source_range(&w.span), SyntaxRole::Property));
        }
        PBE::Error(_) => {}
        _ => {}
    }
}

fn collect_semantic_ranges_port_def_body_element(
    node: &sysml_v2_parser::Node<PortDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::PortDefBodyElement as PDBE;
    match &node.value {
        PDBE::PortUsage(n) => collect_semantic_ranges_port_usage(n, out),
        PDBE::InOutDecl(w) => {
            out.push((span_to_source_range(&w.span), SyntaxRole::Property));
        }
        _ => {}
    }
}

fn collect_semantic_ranges_interface_def_body_element(
    node: &sysml_v2_parser::Node<InterfaceDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::InterfaceDefBodyElement as IDBE;
    match &node.value {
        IDBE::EndDecl(n) => {
            if let EndIdentity::Declaration(label) = &n.value.identity {
                out.push((span_to_source_range(&label.span), SyntaxRole::Property));
            }
            if let Some(ref s) = n.type_ref_span {
                out.push((span_to_source_range(s), SyntaxRole::Type));
            }
        }
        IDBE::RefDecl(n) => {
            if let Some(ref s) = n.value.name_span {
                out.push((span_to_source_range(s), SyntaxRole::Property));
            }
            if let Some(ref s) = n.type_ref_span {
                out.push((span_to_source_range(s), SyntaxRole::Type));
            }
        }
        IDBE::ConnectStmt(_) => {}
        _ => {}
    }
}

fn collect_semantic_ranges_action_usage(
    ctx: &RangeCtx<'_>,
    usage: &ActionUsage,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    if let Some(ref span) = usage.name_span {
        out.push((span_to_source_range(span), SyntaxRole::Property));
    }
    if let Some(ref span) = usage.type_ref_span {
        out.push((span_to_source_range(span), SyntaxRole::Type));
    }
    if let Some(ref accept) = usage.accept {
        collect_semantic_ranges_payload_clause(accept, out);
    }
    if let Some(ref send) = usage.send {
        collect_semantic_ranges_send_payload(ctx, send, out);
    }
    if let Some(ActionUsageBody::Brace { elements, .. }) = &usage.body {
        for element in elements {
            collect_semantic_ranges_action_usage_body_element(ctx, element, out);
        }
    }
}

fn collect_semantic_ranges_ref_decl(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::RefDecl>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    let value = &node.value;
    if let Some(ref span) = value.name_span {
        out.push((span_to_source_range(span), SyntaxRole::Property));
    } else {
        out.push((span_to_source_range(&node.span), SyntaxRole::Property));
    }
    if let Some(ref span) = value.type_ref_span {
        out.push((span_to_source_range(span), SyntaxRole::Type));
    }
}

fn collect_semantic_ranges_state_usage(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<StateUsage>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    push_usage_name_type_spans(
        ctx.source,
        &node.span,
        &node.value.name,
        ctx.type_text(node.value.type_name),
        None,
        None,
        out,
    );
    if let StateDefBody::Brace { elements, .. } = &node.value.body {
        for element in elements {
            collect_semantic_ranges_state_def_body_element(ctx, element, out);
        }
    }
}

fn collect_semantic_ranges_requirement_def_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<RequirementDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use RequirementDefBodyElement as RDBE;
    match &node.value {
        RDBE::SubjectDecl(subject) => {
            out.push((span_to_source_range(&subject.span), SyntaxRole::Property));
        }
        RDBE::Stakeholder(stakeholder) => {
            out.push((
                span_to_source_range(&stakeholder.span),
                SyntaxRole::Property,
            ));
        }
        RDBE::Purpose(purpose) => {
            if let Some(view) = ctx.document.qualified_reference(purpose.value.target) {
                out.push((
                    span_to_source_range(&view.metadata.span),
                    SyntaxRole::Property,
                ));
            }
        }
        RDBE::AttributeDef(attribute) => {
            push_usage_name_type_spans(
                ctx.source,
                &attribute.span,
                &attribute.value.name,
                ctx.typing_text(attribute.value.typing.as_deref()),
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
                ctx.typing_text(attribute.value.typing.as_deref()),
                attribute.value.name_span.as_ref(),
                attribute.value.typing_span.as_ref(),
                out,
            );
        }
        RDBE::VerifyRequirement(verify) => {
            if let Some(requirement) = &verify.value.requirement {
                out.push((
                    span_to_source_range(&requirement.span),
                    SyntaxRole::Property,
                ));
            }
        }
        RDBE::RequireConstraint(constraint) => {
            if let ConstraintDefBody::Brace { elements, .. } = &constraint.value.body {
                for element in elements {
                    match &element.value {
                        ConstraintDefBodyElement::InOutDecl(param) => {
                            out.push((span_to_source_range(&param.span), SyntaxRole::Property));
                        }
                        ConstraintDefBodyElement::Annotating(member) => {
                            collect_semantic_ranges_annotating(&member.clone(), out);
                        }
                        _ => {}
                    }
                }
            }
        }
        RDBE::Frame(frame) => {
            out.push((span_to_source_range(&frame.span), SyntaxRole::Namespace));
            match &frame.value.body {
                RequirementDefBody::Brace { elements, .. } => {
                    for element in elements {
                        collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                    }
                }
                RequirementDefBody::Semicolon { .. } => {}
            }
        }
        RDBE::Import(import) => out.push((
            span_to_source_range(&import.value.target.span),
            SyntaxRole::Namespace,
        )),
        RDBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(ctx, mk_node, out);
        }
        RDBE::RequirementActorDecl(actor) => {
            out.push((span_to_source_range(&actor.span), SyntaxRole::Property));
        }
        RDBE::RequirementUsage(requirement) => {
            push_usage_name_type_spans(
                ctx.source,
                &requirement.span,
                &requirement.value.name,
                ctx.type_text(requirement.value.type_name),
                None,
                None,
                out,
            );
            if let RequirementDefBody::Brace { elements, .. } = &requirement.value.body {
                for element in elements {
                    collect_semantic_ranges_requirement_def_body_element(ctx, element, out);
                }
            }
        }
        RDBE::Error(_) => {}
        RDBE::Annotating(member) => collect_semantic_ranges_annotating(member, out),
        // `subject;` shorthand (concern/viewpoint bodies, validation `11a`) -- mirrors
        // `RDBE::SubjectDecl`'s simple whole-span highlight (no separate name to point at).
        RDBE::SubjectRef(subject_ref) => {
            out.push((
                span_to_source_range(&subject_ref.span),
                SyntaxRole::Property,
            ));
        }
        // `variant name;` inside a `variation requirement` body (validation `7b`) -- same
        // whole-span highlight `PDBE::VariantUsage` already uses.
        RDBE::VariantUsage(variant) => {
            out.push((span_to_source_range(&variant.span), SyntaxRole::Property));
        }
        // Bare `constraint` member nested in a requirement def body.
        RDBE::Constraint(constraint) => {
            out.push((span_to_source_range(&constraint.span), SyntaxRole::Property));
        }
        // Member kinds this collector assigns no token of their own.
        RDBE::Dependency(_)
        | RDBE::RequirementDef(_)
        | RDBE::RefDecl(_)
        | RDBE::ConcernUsage(_)
        | RDBE::CalcUsage(_)
        | RDBE::PortUsage(_)
        | RDBE::AllocationUsage(_)
        | RDBE::Satisfy(_)
        | RDBE::ActionUsage(_)
        | RDBE::SuccessionUsage(_)
        | RDBE::Perform(_)
        | RDBE::StateUsage(_)
        | RDBE::ItemUsage(_)
        | RDBE::PartUsage(_)
        | RDBE::Connect(_)
        | RDBE::ConnectionUsage(_) => {}
    }
}

fn collect_semantic_ranges_action_def_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<ActionDefBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use ActionDefBodyElement as ADBE;
    match &node.value {
        ADBE::InOutDecl(in_out) => {
            out.push((span_to_source_range(&in_out.span), SyntaxRole::Property))
        }
        ADBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(ctx, usage.as_ref(), out),
        ADBE::ThenAction(then_action) => {
            // `then merge <name>;`/`then <name>;` (§6 G23) aren't action-usage declarations --
            // nothing to highlight beyond what's already emitted for the enclosing statement.
            if let sysml_v2_parser::ast::ThenTarget::Action(action_node) = &then_action.value.target
            {
                collect_semantic_ranges_action_usage(ctx, &action_node.value, out);
            }
        }
        ADBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        ADBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        ADBE::Perform(perform) => {
            out.push((span_to_source_range(&perform.span), SyntaxRole::Function))
        }
        ADBE::Assign(assign) => {
            out.push((span_to_source_range(&assign.span), SyntaxRole::Property))
        }
        ADBE::ForLoop(for_loop) => {
            out.push((span_to_source_range(&for_loop.span), SyntaxRole::Property))
        }
        ADBE::DefaultReferenceUsage(default_ref) => {
            push_usage_name_type_spans(
                ctx.source,
                &default_ref.span,
                &default_ref.value.name,
                ctx.typing_text(default_ref.value.typing.as_deref()),
                default_ref.value.name_span.as_ref(),
                default_ref.value.typing_span.as_ref(),
                out,
            );
        }
        ADBE::Bind(_)
        | ADBE::Import(_)
        | ADBE::VariantUsage(_)
        | ADBE::FlowUsage(_)
        | ADBE::FirstStmt(_)
        | ADBE::MergeStmt(_)
        | ADBE::DecisionStmt(_)
        | ADBE::JoinStmt(_)
        | ADBE::ForkStmt(_)
        | ADBE::Error(_)
        | ADBE::TerminateStmt(_)
        | ADBE::WhileStmt(_)
        | ADBE::LoopStmt(_)
        | ADBE::IfStmt(_) => {}
        ADBE::Annotating(member) => collect_semantic_ranges_annotating(member, out),
        ADBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(ctx, mk_node, out);
        }
        ADBE::PartUsage(pu_node) => {
            if let Some(ref span) = pu_node.value.name_span {
                out.push((span_to_source_range(span), SyntaxRole::Property));
            }
            if let Some(ref span) = pu_node.value.type_ref_span {
                out.push((span_to_source_range(span), SyntaxRole::Type));
            }
            if let PartUsageBody::Brace { elements, .. } = &pu_node.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        ADBE::ItemUsage(item_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &item_node.span,
                &item_node.value.name,
                ctx.type_text(item_node.value.type_name),
                None,
                None,
                out,
            );
        }
        ADBE::OccurrenceUsage(occurrence_usage) => {
            out.push((
                span_to_source_range(&occurrence_usage.span),
                SyntaxRole::Property,
            ));
            if let OccurrenceUsageBody::Brace { elements, .. } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, child, out);
                }
            }
        }
        ADBE::AssertConstraint(_) => {}
        // Member kinds this collector assigns no token of their own.
        ADBE::Dependency(_)
        | ADBE::MetadataUsage(_)
        | ADBE::AttributeUsage(_)
        | ADBE::CalcUsage(_)
        | ADBE::ActionDef(_) => {}
    }
}

fn collect_semantic_ranges_action_usage_body_element(
    ctx: &RangeCtx<'_>,
    node: &sysml_v2_parser::Node<ActionUsageBodyElement>,
    out: &mut Vec<(SyntaxRange, SyntaxRole)>,
) {
    use sysml_v2_parser::ast::ActionUsageBodyElement as AUBE;
    match &node.value {
        AUBE::InOutDecl(in_out) => {
            out.push((span_to_source_range(&in_out.span), SyntaxRole::Property))
        }
        AUBE::ActionUsage(usage) => collect_semantic_ranges_action_usage(ctx, usage.as_ref(), out),
        AUBE::ThenAction(then_action) => {
            // See ADBE::ThenAction above.
            if let sysml_v2_parser::ast::ThenTarget::Action(action_node) = &then_action.value.target
            {
                collect_semantic_ranges_action_usage(ctx, &action_node.value, out);
            }
        }
        AUBE::RefDecl(ref_decl) => collect_semantic_ranges_ref_decl(ref_decl, out),
        AUBE::StateUsage(state_usage) => collect_semantic_ranges_state_usage(ctx, state_usage, out),
        AUBE::Assign(assign) => {
            out.push((span_to_source_range(&assign.span), SyntaxRole::Property))
        }
        AUBE::ForLoop(for_loop) => {
            out.push((span_to_source_range(&for_loop.span), SyntaxRole::Property))
        }
        AUBE::DefaultReferenceUsage(default_ref) => {
            push_usage_name_type_spans(
                ctx.source,
                &default_ref.span,
                &default_ref.value.name,
                ctx.typing_text(default_ref.value.typing.as_deref()),
                default_ref.value.name_span.as_ref(),
                default_ref.value.typing_span.as_ref(),
                out,
            );
        }
        AUBE::Bind(_)
        | AUBE::Import(_)
        | AUBE::FlowUsage(_)
        | AUBE::FirstStmt(_)
        | AUBE::MergeStmt(_)
        | AUBE::DecisionStmt(_)
        | AUBE::JoinStmt(_)
        | AUBE::ForkStmt(_)
        | AUBE::Error(_)
        | AUBE::TerminateStmt(_)
        | AUBE::WhileStmt(_)
        | AUBE::LoopStmt(_)
        | AUBE::IfStmt(_) => {}
        AUBE::Annotating(member) => collect_semantic_ranges_annotating(member, out),
        AUBE::MetadataKeywordUsage(mk_node) => {
            collect_semantic_ranges_metadata_keyword_usage(ctx, mk_node, out);
        }
        AUBE::PartUsage(pu_node) => {
            if let Some(ref span) = pu_node.value.name_span {
                out.push((span_to_source_range(span), SyntaxRole::Property));
            }
            if let Some(ref span) = pu_node.value.type_ref_span {
                out.push((span_to_source_range(span), SyntaxRole::Type));
            }
            if let PartUsageBody::Brace { elements, .. } = &pu_node.body {
                for child in elements {
                    collect_semantic_ranges_part_usage_body_element(ctx, child, out);
                }
            }
        }
        AUBE::ItemUsage(item_node) => {
            push_usage_name_type_spans(
                ctx.source,
                &item_node.span,
                &item_node.value.name,
                ctx.type_text(item_node.value.type_name),
                None,
                None,
                out,
            );
        }
        AUBE::OccurrenceUsage(occurrence_usage) => {
            out.push((
                span_to_source_range(&occurrence_usage.span),
                SyntaxRole::Property,
            ));
            if let OccurrenceUsageBody::Brace { elements, .. } = &occurrence_usage.body {
                for child in elements {
                    collect_semantic_ranges_occurrence_body_element(ctx, child, out);
                }
            }
        }
        AUBE::AssertConstraint(_) => {}
        // Member kinds this collector assigns no token of their own.
        AUBE::Dependency(_)
        | AUBE::MetadataUsage(_)
        | AUBE::AttributeUsage(_)
        | AUBE::CalcUsage(_)
        | AUBE::ActionDef(_)
        | AUBE::VariantUsage(_) => {}
    }
}
