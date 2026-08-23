//! Phase 2 lowering — structure: parts, attributes, items, enumerations, occurrences, parameters.

use crate::lower::facts::definition_prefix_modifiers;
use crate::lower::facts::definition_prefix_node_modifiers;
use crate::lower::facts::direction_fact;
use crate::lower::facts::direction_node_fact;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::occurrence_prefix_modifiers;
use crate::lower::facts::portion_kind_node_fact;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::ParameterDirection;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::SemanticModelBuilder;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use sysml_v2_parser::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, CaseReturnDecl,
    DefaultReferenceUsage, DefinitionBody, DefinitionBodyElement, DefinitionPrefix, EnumDef,
    EnumerationBody, EnumerationBodyElement, EnumerationUsage as ParserEnumerationUsage,
    FeatureValue, InOut, InOutDecl, ItemDef, ItemUsage as ParserItemUsage,
    MembershipKind as ParserMembershipKind, Node, OccurrenceBodyElement, OccurrenceDef,
    OccurrenceUsage as ParserOccurrenceUsage, OccurrenceUsageBody, PartDef, PartDefBody,
    PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement, RefDecl, ReturnDecl,
};

impl SemanticModelBuilder {
    pub(crate) fn lower_part_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PartDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let PartDefBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    PartDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PartDefBodyElement::Package(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::PartDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    PartDefBodyElement::LibraryPackage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::PartDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    PartDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    PartDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PartDefBodyElement::PartUsage(part) => {
                        self.lower_part_usage(document, Some(declaration), part)?;
                    }
                    PartDefBodyElement::PartDef(part) => {
                        self.lower_part_def(document, Some(declaration), part)?;
                    }
                    PartDefBodyElement::Import(import) => {
                        self.lower_import(document, Some(declaration), import)?;
                    }
                    PartDefBodyElement::EnumDef(enum_def) => {
                        self.lower_enum_def(document, Some(declaration), enum_def)?;
                    }
                    PartDefBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PartDefBodyElement::RequirementDef(requirement_def) => {
                        self.lower_requirement_def(document, Some(declaration), requirement_def)?;
                    }
                    PartDefBodyElement::AnalysisCaseDef(analysis_case_def) => {
                        self.lower_analysis_case_def(
                            document,
                            Some(declaration),
                            analysis_case_def,
                        )?;
                    }
                    PartDefBodyElement::CaseDef(case_def) => {
                        self.lower_case_def(document, Some(declaration), case_def)?;
                    }
                    PartDefBodyElement::CaseUsage(case_usage) => {
                        self.lower_case_usage(document, Some(declaration), case_usage)?;
                    }
                    PartDefBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                        self.lower_analysis_case_usage(
                            document,
                            Some(declaration),
                            analysis_case_usage,
                        )?;
                    }
                    PartDefBodyElement::VerificationCaseDef(verification_case_def) => {
                        self.lower_verification_case_def(
                            document,
                            Some(declaration),
                            verification_case_def,
                        )?;
                    }
                    PartDefBodyElement::UseCaseDef(use_case_def) => {
                        self.lower_use_case_def(document, Some(declaration), use_case_def)?;
                    }
                    PartDefBodyElement::RequirementUsage(requirement_usage) => {
                        self.lower_requirement_usage(
                            document,
                            Some(declaration),
                            requirement_usage,
                        )?;
                    }
                    PartDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    PartDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PartDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PartDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PartDefBodyElement::MetadataDef(metadata_def) => {
                        self.lower_metadata_def(document, Some(declaration), metadata_def)?;
                    }
                    PartDefBodyElement::MetadataUsage(metadata_usage) => {
                        self.lower_metadata_usage(document, Some(declaration), metadata_usage)?;
                    }
                    PartDefBodyElement::ActionDef(action_def) => {
                        self.lower_action_def(document, Some(declaration), action_def)?;
                    }
                    PartDefBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(declaration), action_usage)?;
                    }
                    PartDefBodyElement::StateDef(state_def) => {
                        self.lower_state_def(document, Some(declaration), state_def)?;
                    }
                    PartDefBodyElement::StateUsage(state_usage) => {
                        self.lower_state_usage(document, Some(declaration), state_usage)?;
                    }
                    PartDefBodyElement::ConnectionDef(connection_def) => {
                        self.lower_connection_def(document, Some(declaration), connection_def)?;
                    }
                    PartDefBodyElement::InterfaceDef(interface_def) => {
                        self.lower_interface_def(document, Some(declaration), interface_def)?;
                    }
                    PartDefBodyElement::ViewDef(view_def) => {
                        self.lower_view_def(document, Some(declaration), view_def)?;
                    }
                    PartDefBodyElement::ViewpointDef(viewpoint_def) => {
                        self.lower_viewpoint_def(document, Some(declaration), viewpoint_def)?;
                    }
                    PartDefBodyElement::RenderingDef(rendering_def) => {
                        self.lower_rendering_def(document, Some(declaration), rendering_def)?;
                    }
                    PartDefBodyElement::AllocationDef(allocation_def) => {
                        self.lower_allocation_def(document, Some(declaration), allocation_def)?;
                    }
                    PartDefBodyElement::FlowDef(flow_def) => {
                        self.lower_flow_def(document, Some(declaration), flow_def)?;
                    }
                    PartDefBodyElement::Connection(connection_usage) => {
                        self.lower_connection_usage(document, Some(declaration), connection_usage)?;
                    }
                    PartDefBodyElement::OccurrenceDef(occurrence_def) => {
                        self.lower_occurrence_def(document, Some(declaration), occurrence_def)?;
                    }
                    PartDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    PartDefBodyElement::InterfaceUsage(interface_usage) => {
                        self.lower_interface_usage(document, Some(declaration), interface_usage)?;
                    }
                    PartDefBodyElement::ViewUsage(view_usage) => {
                        self.lower_view_usage(document, Some(declaration), view_usage)?;
                    }
                    PartDefBodyElement::RenderingUsage(node) => {
                        self.lower_rendering_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::UseCaseUsage(node) => {
                        self.lower_use_case_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::VerificationCaseUsage(node) => {
                        self.lower_verification_case_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::ConstraintDef(constraint_def) => {
                        self.lower_constraint_def(document, Some(declaration), constraint_def)?;
                    }
                    PartDefBodyElement::ConstraintUsage(constraint_usage) => {
                        self.lower_constraint_usage(document, Some(declaration), constraint_usage)?;
                    }
                    PartDefBodyElement::CalcDef(calc_def) => {
                        self.lower_calc_def(document, Some(declaration), calc_def)?;
                    }
                    PartDefBodyElement::CalcUsage(calc_usage) => {
                        self.lower_calc_usage(document, Some(declaration), calc_usage)?;
                    }
                    PartDefBodyElement::AliasDef(alias_def) => {
                        self.lower_alias_def(document, Some(declaration), alias_def)?;
                    }
                    PartDefBodyElement::Perform(perform) => {
                        self.lower_perform(document, Some(declaration), perform)?;
                    }
                    PartDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PartDefinitionMember,
                            member,
                        )?;
                    }
                    PartDefBodyElement::Satisfy(node) => {
                        self.lower_satisfy(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::Allocate(node) => {
                        self.lower_allocate(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::Bind(node) => {
                        self.lower_bind(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::FirstStmt(first_stmt) => {
                        self.lower_first_stmt(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            first_stmt,
                        )?;
                    }
                    PartDefBodyElement::VariantUsage(node) => {
                        self.lower_variant_usage(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::AssertConstraint(node) => self
                        .lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?,
                    PartDefBodyElement::Ref(node) => {
                        self.lower_ref_decl(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::DefaultReferenceUsage(node) => {
                        self.lower_default_reference_usage(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::Dependency(node) => {
                        self.lower_dependency(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::Connect(node) => {
                        self.lower_bare_connect(
                            document,
                            declaration,
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::ViewpointUsage(node) => {
                        self.lower_viewpoint_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::KermlClassifier(node) => {
                        self.lower_kerml_classifier_decl(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::ExhibitState(node) => {
                        self.lower_exhibit_state(
                            document,
                            Some(declaration),
                            UnsupportedFamily::PartDefinitionMember,
                            node,
                        )?;
                    }
                    PartDefBodyElement::AllocationUsage(node) => {
                        self.lower_allocation_usage(document, Some(declaration), node)?;
                    }
                    PartDefBodyElement::MetadataKeywordUsage(_)
                    | PartDefBodyElement::FlowUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PartDefinitionMember,
                        element.span.clone(),
                    ),
                    PartDefBodyElement::UnsupportedMember(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    pub(crate) fn lower_part_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PartUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let facts = DeclarationFacts {
            short_name,
            modifiers: DeclarationModifiers {
                ordered: node.value.multiplicity_modifiers.is_ordered(),
                nonunique: !node.value.multiplicity_modifiers.is_unique(),
                ..occurrence_prefix_modifiers(&node.value.prefix)
            },
            direction: direction_node_fact(node.value.prefix.basic.ref_prefix.direction.as_ref()),
            multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
            ..DeclarationFacts::none()
        };
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartUsage,
            name,
            node.span.clone(),
            facts,
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship_impl(
                document,
                declaration,
                relationship,
                matches!(
                    node.value
                        .prefix
                        .basic
                        .ref_prefix
                        .variance
                        .as_ref()
                        .map(|prefix| prefix.value),
                    Some(DefinitionPrefix::Variation)
                ),
                None,
            )?;
        }
        if let Some((relationship, _)) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let PartUsageBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                self.lower_part_usage_body_element(
                    document,
                    declaration,
                    UnsupportedFamily::PartUsageMember,
                    element,
                )?;
            }
        }
        Ok(())
    }

    /// Lowers one `PartUsageBodyElement` found inside a `part` usage/def body, and reused
    /// verbatim by every statement form whose body is `UsageBody = DefinitionBody` and which
    /// mints an anonymous declaration to own it -- `bind`, `allocate`, a bare `connect`, and a
    /// keyword-less binding connector all hold this same `PartUsageBody` member set -- and by
    /// `ref { ... }` bodies, which hold this same member set upstream (`RefBody =
    /// Body<PartUsageBodyElement>`). See `lower_part_usage`'s own doc comment for the per-arm
    /// recognized/unsupported shape.
    ///
    /// `family` names the owning body in the unsupported facts this dispatch produces, so a `ref`
    /// body's unmodeled members stay distinguishable from a `part` usage body's.
    pub(crate) fn lower_part_usage_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        element: &Node<PartUsageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            PartUsageBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            PartUsageBodyElement::AttributeUsage(attribute) => {
                self.lower_attribute_usage(document, Some(owner), attribute)?;
            }
            PartUsageBodyElement::PartUsage(part) => {
                self.lower_part_usage(document, Some(owner), part)?;
            }
            PartUsageBodyElement::Import(import) => {
                self.lower_import(document, Some(owner), import)?;
            }
            PartUsageBodyElement::EnumDef(enum_def) => {
                self.lower_enum_def(document, Some(owner), enum_def)?;
            }
            PartUsageBodyElement::EnumerationUsage(enum_usage) => {
                self.lower_enum_usage(document, Some(owner), enum_usage)?;
            }
            PartUsageBodyElement::RequirementDef(requirement_def) => {
                self.lower_requirement_def(document, Some(owner), requirement_def)?;
            }
            PartUsageBodyElement::AnalysisCaseDef(analysis_case_def) => {
                self.lower_analysis_case_def(document, Some(owner), analysis_case_def)?;
            }
            PartUsageBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                self.lower_analysis_case_usage(document, Some(owner), analysis_case_usage)?;
            }
            PartUsageBodyElement::RequirementUsage(requirement_usage) => {
                self.lower_requirement_usage(document, Some(owner), requirement_usage)?;
            }
            PartUsageBodyElement::PortDef(port_def) => {
                self.lower_port_def(document, Some(owner), port_def)?;
            }
            PartUsageBodyElement::PortUsage(port_usage) => {
                self.lower_port_usage(document, Some(owner), port_usage)?;
            }
            PartUsageBodyElement::ItemDef(item_def) => {
                self.lower_item_def(document, Some(owner), item_def)?;
            }
            PartUsageBodyElement::ItemUsage(item_usage) => {
                self.lower_item_usage(document, Some(owner), item_usage)?;
            }
            PartUsageBodyElement::MetadataDef(metadata_def) => {
                self.lower_metadata_def(document, Some(owner), metadata_def)?;
            }
            PartUsageBodyElement::MetadataUsage(metadata_usage) => {
                self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
            }
            PartUsageBodyElement::ActionUsage(action_usage) => {
                self.lower_action_usage(document, Some(owner), action_usage)?;
            }
            PartUsageBodyElement::StateDef(state_def) => {
                self.lower_state_def(document, Some(owner), state_def)?;
            }
            PartUsageBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            PartUsageBodyElement::ConnectionDef(connection_def) => {
                self.lower_connection_def(document, Some(owner), connection_def)?;
            }
            PartUsageBodyElement::Connection(connection_usage) => {
                self.lower_connection_usage(document, Some(owner), connection_usage)?;
            }
            PartUsageBodyElement::OccurrenceDef(occurrence_def) => {
                self.lower_occurrence_def(document, Some(owner), occurrence_def)?;
            }
            PartUsageBodyElement::OccurrenceUsage(occurrence_usage) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
            }
            PartUsageBodyElement::FlowDef(flow_def) => {
                self.lower_flow_def(document, Some(owner), flow_def)?;
            }
            PartUsageBodyElement::InterfaceUsage(interface_usage) => {
                self.lower_interface_usage(document, Some(owner), interface_usage)?;
            }
            PartUsageBodyElement::ConstraintDef(constraint_def) => {
                self.lower_constraint_def(document, Some(owner), constraint_def)?;
            }
            PartUsageBodyElement::ConstraintUsage(constraint_usage) => {
                self.lower_constraint_usage(document, Some(owner), constraint_usage)?;
            }
            PartUsageBodyElement::CalcDef(calc_def) => {
                self.lower_calc_def(document, Some(owner), calc_def)?;
            }
            PartUsageBodyElement::CalcUsage(calc_usage) => {
                self.lower_calc_usage(document, Some(owner), calc_usage)?;
            }
            PartUsageBodyElement::AliasDef(alias_def) => {
                self.lower_alias_def(document, Some(owner), alias_def)?;
            }
            PartUsageBodyElement::Perform(perform) => {
                self.lower_perform(document, Some(owner), perform)?;
            }
            PartUsageBodyElement::Annotating(member) => {
                self.lower_annotating_member(document, Some(owner), family, member)?;
            }
            PartUsageBodyElement::KermlClassifier(node) => {
                self.lower_kerml_classifier_decl(document, Some(owner), node)?;
            }
            PartUsageBodyElement::Satisfy(node) => {
                self.lower_satisfy(document, owner, family, node)?;
            }
            PartUsageBodyElement::VariantUsage(node) => {
                self.lower_variant_usage(document, owner, family, node)?;
            }
            PartUsageBodyElement::Allocate(node) => {
                self.lower_allocate(document, owner, family, node)?;
            }
            PartUsageBodyElement::Bind(node) => {
                self.lower_bind(document, owner, family, node)?;
            }
            PartUsageBodyElement::AssertConstraint(node) => {
                self.lower_assert_constraint_member(document, owner, family, node)?
            }
            PartUsageBodyElement::Ref(node) => {
                self.lower_ref_decl(document, Some(owner), node)?;
            }
            PartUsageBodyElement::EndDecl(node) => {
                self.lower_end_decl(document, owner, node)?;
            }
            PartUsageBodyElement::InOutDecl(node) => {
                self.lower_parameter_declaration(document, Some(owner), family, node)?;
            }
            PartUsageBodyElement::DefaultReferenceUsage(node) => {
                self.lower_default_reference_usage(document, Some(owner), family, node)?;
            }
            PartUsageBodyElement::Connect(node) => {
                self.lower_bare_connect(document, owner, family, node)?;
            }
            PartUsageBodyElement::UseCaseUsage(node) => {
                self.lower_use_case_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::VerificationCaseUsage(node) => {
                self.lower_verification_case_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::ViewDef(node) => {
                self.lower_view_def(document, Some(owner), node)?;
            }
            PartUsageBodyElement::ViewUsage(node) => {
                self.lower_view_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::ViewpointDef(node) => {
                self.lower_viewpoint_def(document, Some(owner), node)?;
            }
            PartUsageBodyElement::ViewpointUsage(node) => {
                self.lower_viewpoint_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::RenderingDef(node) => {
                self.lower_rendering_def(document, Some(owner), node)?;
            }
            PartUsageBodyElement::RenderingUsage(node) => {
                self.lower_rendering_usage(document, Some(owner), node)?;
            }
            PartUsageBodyElement::FlowUsage(_)
            | PartUsageBodyElement::SuccessionUsage(_)
            | PartUsageBodyElement::MetadataKeywordUsage(_)
            | PartUsageBodyElement::IncludeUseCase(_) => {
                self.push_unsupported(document, family, element.span.clone())
            }
        }
        Ok(())
    }

    /// Lowers a `ref <name>: <Type>;` non-owning referential feature (BNF `ReferenceUsage`,
    /// `ast::RefDecl`), reused verbatim across part/attribute/action/state/connection/interface/
    /// package bodies. Mirrors `lower_part_usage`'s ownership/typing/redefines/subsets shape (`ref`
    /// is a `FeatureMembership` like any other usage; see `ast::connector::ref_decl`'s
    /// `Membership::feature` construction), since `RefDecl` carries the same structured
    /// `typing`/`redefines`/`subsets` clauses as `PartUsage`/`AttributeUsage`. Its body is the
    /// general usage-member set (`RefBody = Body<PartUsageBodyElement>`, `UsageBody =
    /// DefinitionBody` per SysML 8.2.2.6.2) whatever declaration owns it, so it walks through the
    /// shared `lower_part_usage_body_element` dispatcher under
    /// `UnsupportedFamily::ReferenceUsageMember`.
    pub(crate) fn lower_ref_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RefDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.usage_prefix.as_ref());
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ReferenceUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    derived: node.value.is_derived,
                    constant: node.value.is_constant,
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    // The `ref` keyword is this declaration's own form, not a prefix modifier on
                    // some other usage, so `reference` stays false here.
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        for element in node.value.body.members() {
            self.lower_part_usage_body_element(
                document,
                declaration,
                UnsupportedFamily::ReferenceUsageMember,
                element,
            )?;
        }
        Ok(())
    }

    pub(crate) fn lower_attribute_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_modifiers(node.value.usage_prefix.as_ref());
        let facts = DeclarationFacts {
            short_name,
            modifiers: DeclarationModifiers {
                is_abstract,
                variation,
                derived: node.value.is_derived,
                end: node.value.is_end,
                reference: node.value.is_reference,
                constant: node.value.is_constant,
                ordered: node.value.multiplicity_modifiers.is_ordered(),
                nonunique: !node.value.multiplicity_modifiers.is_unique(),
                ..DeclarationModifiers::default()
            },
            direction: direction_fact(node.value.direction.as_ref()),
            multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
            ..DeclarationFacts::none()
        };
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeUsage,
            name,
            node.span.clone(),
            facts,
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_default_value(document, declaration, node.value.value.as_ref())?;
        self.lower_attribute_body(document, declaration, &node.value.body)?;
        Ok(())
    }

    /// Value-assignment lowering for a `name = <Expression>;` shape, shared by every context that
    /// carries one: an attribute def/usage default value AND explicit redefinition value
    /// (`attribute mass = 5;` / `attribute :>> status = RequirementStatusKind::approved;`, both
    /// typed as `FeatureValue` -- the parser does not distinguish "default" from "redefinition
    /// value" syntactically, both are just an attribute's own optional `=` clause), a metadata
    /// annotation body override (`@Safety{isMandatory = true;}`, also `FeatureValue` on a nested
    /// `AttributeUsage`), and a parameter default value (`out v_out : SpeedValue = vel.v;`, also
    /// `FeatureValue`). All four contexts share the exact same typed-AST shape (a name, an `=`,
    /// and an `Expression`), so this one helper handles all of them: reuses the full
    /// `classify_expression`/`lower_constraint_expression` machinery (literal,
    /// feature-ref, member-access, arithmetic/comparison/logical `BinaryOp`, invocation) rather
    /// than a bespoke literal-only walk, so `attribute mass = length * width;`, `attribute :>>
    /// status = RequirementStatusKind::approved;`, and `attribute f = other.value;` all resolve
    /// their operands and, where possible, evaluate to a genuine constant via `compute_evaluation`
    /// -- a value that is resolved but not itself constant publishes `NonConstant`, never a
    /// fabricated value (see `EvaluatedValue`). `family` selects which diagnostic an unsupported
    /// expression shape falls through to.
    pub(crate) fn lower_value_assignment(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        family: UnsupportedFamily,
        value: Option<&Node<FeatureValue>>,
    ) -> Result<(), ConstructionError> {
        let Some(feature_value) = value else {
            return Ok(());
        };
        self.record_feature_value(declaration, feature_value)?;
        let expression = &feature_value.value.expression;
        self.push_evaluation_fact(
            declaration,
            self.constraint_expression_site(document, &expression.value),
        );
        self.lower_constraint_expression(document, declaration, family, expression)
    }

    pub(crate) fn lower_attribute_default_value(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        value: Option<&Node<FeatureValue>>,
    ) -> Result<(), ConstructionError> {
        self.lower_value_assignment(
            document,
            declaration,
            UnsupportedFamily::AttributeMember,
            value,
        )
    }

    pub(crate) fn lower_attribute_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_default_value(document, declaration, node.value.value.as_ref())?;
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    pub(crate) fn lower_attribute_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &AttributeBody,
    ) -> Result<(), ConstructionError> {
        let AttributeBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                AttributeBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                AttributeBodyElement::DefaultReferenceUsage(node) => {
                    // New upstream member kind: kept visible as unsupported rather than dropped.
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::AttributeMember,
                        node.span.clone(),
                    );
                }
                AttributeBodyElement::VariantUsage(node) => {
                    // New upstream member kind: kept visible as unsupported rather than dropped.
                    self.push_unsupported(
                        document,
                        UnsupportedFamily::AttributeMember,
                        node.span.clone(),
                    );
                }
                AttributeBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        Some(owner),
                        UnsupportedFamily::AttributeMember,
                        member,
                    )?;
                }
                AttributeBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                AttributeBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                AttributeBodyElement::PartUsage(part) => {
                    self.lower_part_usage(document, Some(owner), part)?;
                }
                AttributeBodyElement::OccurrenceUsage(occurrence_usage) => {
                    self.lower_occurrence_usage(document, Some(owner), occurrence_usage)?;
                }
                AttributeBodyElement::ItemUsage(item_usage) => {
                    self.lower_item_usage(document, Some(owner), item_usage)?;
                }
                AttributeBodyElement::AssertConstraint(node) => self
                    .lower_assert_constraint_member(
                        document,
                        owner,
                        UnsupportedFamily::AttributeMember,
                        node,
                    )?,
                AttributeBodyElement::RefDecl(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                AttributeBodyElement::Connect(node) => {
                    self.lower_bare_connect(
                        document,
                        owner,
                        UnsupportedFamily::AttributeMember,
                        node,
                    )?;
                }
                AttributeBodyElement::Bind(node) => {
                    self.lower_bind(document, owner, UnsupportedFamily::AttributeMember, node)?;
                }
                AttributeBodyElement::Connection(node) => {
                    self.lower_connection_usage(document, Some(owner), node)?;
                }
                AttributeBodyElement::ConstraintUsage(node) => {
                    self.lower_constraint_usage(document, Some(owner), node)?;
                }
                AttributeBodyElement::CalcDef(node) => {
                    self.lower_calc_def(document, Some(owner), node)?;
                }
                AttributeBodyElement::CalcUsage(node) => {
                    self.lower_calc_usage(document, Some(owner), node)?;
                }
                AttributeBodyElement::KermlClassifier(node) => {
                    self.lower_kerml_classifier_decl(document, Some(owner), node)?;
                }
                AttributeBodyElement::KermlConnector(node) => {
                    self.lower_kerml_connector_member(document, owner, node)?;
                }
                AttributeBodyElement::KermlFeature(node) => self.lower_kerml_feature_member(
                    document,
                    Some(owner),
                    UnsupportedFamily::AttributeMember,
                    node,
                )?,
                AttributeBodyElement::Invariant(node) => {
                    self.lower_kerml_invariant_member(document, Some(owner), node)?;
                }
                AttributeBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
                AttributeBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::AttributeMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers an `enum def` (BNF EnumerationDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>`/`:` specialization relationship (an enum def may specialize
    /// another enum def or an attribute def), and each owned enumeration literal as its own typed
    /// declaration.
    pub(crate) fn lower_enum_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<EnumDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let EnumerationBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                match &element.value {
                    EnumerationBodyElement::Value(value) => {
                        self.lower_enumerated_value(document, declaration, value)?;
                    }
                    // `EnumerationBody` names its own membership -- the annotating production plus
                    // enumerated values, and nothing else -- so `enum def` shares the attribute
                    // family it specializes rather than owning a family of its own.
                    EnumerationBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::AttributeMember,
                            member,
                        )?;
                    }
                    EnumerationBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                }
            }
        }
        Ok(())
    }

    /// Lowers one `enum <name>;` value owned by an `enum def` body (BNF EnumeratedValue) into its
    /// own declaration. An enumerated value is a full SysML `Usage`, so it carries an
    /// identification, an optional `= expr` initializer, and a `PartUsageBody` of its own -- the
    /// same body shape `lower_part_usage` walks, so its owned members go through the same
    /// `lower_part_usage_body_element`.
    pub(crate) fn lower_enumerated_value(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<sysml_v2_parser::ast::EnumeratedValue>,
    ) -> Result<(), ConstructionError> {
        let name = match node.value.identification.name.as_deref() {
            Some(name) => self.intern_declared_name(name)?,
            None => None,
        };
        let short_name = self.intern_short_name(node.value.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::EnumerationLiteral,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let PartUsageBody::Brace { elements, .. } = &node.value.body {
            for element in elements {
                self.lower_part_usage_body_element(
                    document,
                    declaration,
                    UnsupportedFamily::AttributeMember,
                    element,
                )?;
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `enum` feature member (BNF EnumerationUsage), e.g.
    /// `enum color : ColorKind;`, mirroring `lower_attribute_usage`. `type_name` is a bare
    /// `QualifiedReferenceId` (not a `TypingRelationship` node), so its `FeatureTyping` reference
    /// is pushed directly rather than through `lower_typing_relationship`.
    pub(crate) fn lower_enum_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserEnumerationUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    end: node.value.is_end,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers an `item def` (BNF ItemDefinition), mirroring `lower_part_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `ItemDef`'s body is a plain
    /// `AttributeBody` (shared with `AttributeDef`/`AttributeUsage`), not a `PartDefBody`, so its
    /// owned members are lowered through the existing `lower_attribute_body` rather than a
    /// dedicated item-specific body walker.
    pub(crate) fn lower_item_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ItemDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a keyword-less `<name> = <expr>;` / `<name> : <Type>;` binding
    /// (`ast::structure::DefaultReferenceUsage`, BNF §8.2.2.6 / Spec §7.6.4), e.g. `baseType =
    /// Atom meta KerML::Classifier;` (KerML `metaclass` body member), mirroring
    /// `lower_kerml_feature_member`/`lower_ref_decl`: ownership, membership, an optional `:`
    /// typing target, `subsets`/`redefines` relationships, and an optional `=` value expression
    /// resolved through the shared `classify_expression`/`lower_calc_expression` pipeline
    /// (the same machinery `lower_kerml_feature_member`'s `value` clause uses, which already
    /// covers the `MetaCast` reflective-operator shape from `ea6eb632`). `family` selects which
    /// diagnostic an unsupported value-expression shape falls through to; multiplicity and the
    /// `has_feature_keyword`/`body` fields are intentionally left unmodeled (see
    /// `DeclarationKind::DefaultReferenceUsage`).
    pub(crate) fn lower_default_reference_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        family: UnsupportedFamily,
        node: &Node<DefaultReferenceUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::DefaultReferenceUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(
                declaration,
                self.calc_expression_site(document, &expression.value),
            );
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `item` feature member (BNF ItemUsage), e.g.
    /// `item i : SomeItem;`, mirroring `lower_part_usage`. `type_name` is a bare
    /// `QualifiedReferenceId` (not a `TypingRelationship` node, like `ItemUsage::type_name`'s
    /// `lower_enum_usage` counterpart), so its `FeatureTyping` reference is pushed directly rather
    /// than through `lower_typing_relationship`. `ItemUsage`'s body is a plain `AttributeBody`
    /// (see `lower_item_def`), so owned members are lowered through `lower_attribute_body`.
    pub(crate) fn lower_item_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserItemUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..occurrence_prefix_modifiers(&node.value.prefix)
                },
                direction: direction_node_fact(
                    node.value.prefix.basic.ref_prefix.direction.as_ref(),
                ),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    variation: matches!(
                        node.value
                            .prefix
                            .basic
                            .ref_prefix
                            .variance
                            .as_ref()
                            .map(|prefix| prefix.value),
                        Some(DefinitionPrefix::Variation)
                    ),
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a directed `in`/`out`/`inout` parameter declaration (BNF `InOutDecl`) found in a
    /// `calc def`/`constraint def`/`action def` body, e.g. `in partMasses : MassValue[0..*];`,
    /// mirroring `lower_item_usage`: ownership, membership, and (when a type is present) a
    /// `FeatureTyping` reference to the declared type, carrying an explicit
    /// `RelationshipFlags::direction` fact mirroring the `conjugated` flag precedent set by
    /// `PortUsage`. Untyped parameters (`type_name` is `None`, e.g. a bare `in :>> target = expr;`
    /// redefinition form, or `in seq[1..*] nonunique ordered;` with only a multiplicity/collection
    /// modifiers) still get the declaration/membership shell lowered -- only the `FeatureTyping`
    /// reference (and hence the direction fact) is skipped when there is no type to reference. The
    /// `:>` subsets clause (`ast::InOutDecl::subsets`, e.g. `in value :> seq;`) and the `:>>`
    /// redefinition clause (`ast::InOutDecl::redefines`) are each lowered via
    /// `lower_subsetting_relationship` regardless of whether a type is present, reusing the exact
    /// same helper `AttributeUsage`/`ItemUsage` already call. The two spellings are separate
    /// authored clauses upstream -- `:>` was previously folded into `type_name`, which reported a
    /// subsetting as a typing. The declared name
    /// may be empty for the anonymous redefinition shape; `intern_declared_name` already treats an
    /// empty name as anonymous (see its callers for `EnumerationLiteral` etc.). Multiplicity and
    /// collection modifiers (`nonunique`/`ordered`) remain out of scope, matching every other
    /// declaration kind in this codebase.
    pub(crate) fn lower_parameter_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        family: UnsupportedFamily,
        node: &Node<InOutDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    reference: node.value.is_reference,
                    var: node.value.is_var,
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(Some(&node.value.direction)),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            let direction = Some(match node.value.direction {
                InOut::In => ParameterDirection::In,
                InOut::Out => ParameterDirection::Out,
                InOut::InOut => ParameterDirection::InOut,
            });
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    direction,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        // Widened value-assignment handling (see `lower_value_assignment`/`lower_return_decl`'s
        // own `= expr` handling, which this mirrors exactly): a parameter default value
        // (`out v_out : SpeedValue = vel.v;`) is a bare `Node<Expression>` on `InOutDecl::value`
        // -- the same shape `ReturnDecl::value` already has classify_expression/
        // lower_calc_expression wiring for, so this reuses that identical pipeline rather than
        // introducing new logic. Previously deferred (`494b0ba6`) pending value-assignment
        // machinery existing at all.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(
                declaration,
                self.calc_expression_site(document, &expression.value),
            );
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        Ok(())
    }

    /// Lowers a calc-body `return` declaration (BNF `ReturnDecl`, e.g. `return : Type = a + b;`
    /// or `return result : Type = a + b;`), reusing `lower_parameter_declaration`'s shape
    /// (ownership, membership, a `FeatureTyping` reference to the declared type) since a return
    /// declaration is itself a parameter-like feature -- SysML models a calc's `result` as an
    /// implicit output parameter. `name` is empty for the common anonymous `return : Type = expr;`
    /// form (validation `10c`/`10d`); `intern_declared_name` folds that to `None`, exactly like
    /// `lower_subject_decl`'s bare `subject;` form. Unlike `InOutDecl::type_name`, `ReturnDecl::
    /// type_name` is never optional (the grammar requires a type), so the `FeatureTyping`
    /// reference is unconditional here.
    ///
    /// When a `= expr` value is present, its expression is classified/lowered through the exact
    /// same `classify_expression`/`lower_calc_expression` machinery slices 1-4 already built
    /// for a bare `CalcDefBodyElement::Expression` body -- this is the "distinct ReturnDecl shape"
    /// `bd50fccd` deferred: most real-corpus calc arithmetic (e.g. `return : Type = a + b * c;`)
    /// lives here, not in a bare `Expression` body-element, and it is the exact same `Expression`
    /// enum/`BinaryOp`/`FeatureRef` leaf shapes slices 1-4 already handle, so this is pure wiring
    /// into the same pipeline -- no new evaluation logic.
    ///
    /// `is_redefine` (`return :>> name = expr;`) and `is_subsetting` (`return name :> Type = expr;`)
    /// spelling variants are not modeled as distinct relationship kinds here (mirrors
    /// `lower_parameter_declaration`'s own `InOutDecl::redefines`-shaped field being out of
    /// scope); both spellings still get the same `FeatureTyping` reference and evaluation fact.
    pub(crate) fn lower_return_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ReturnDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(
                declaration,
                self.calc_expression_site(document, &expression.value),
            );
            self.lower_calc_expression(
                document,
                declaration,
                UnsupportedFamily::CalcDefinitionMember,
                &expression,
            )?;
        }
        Ok(())
    }

    /// Lowers a case-family `return` declaration (BNF `CaseReturnDecl`) found in an analysis/
    /// verification/use-case/case def or usage body, e.g. `return calculatedFuelEconomy :
    /// DistancePerVolumeValue;`, `return part :>> selectedAlternative : Engine;`, `return
    /// simulatedRange = vehicle.vehicleBehavior.output.distance;`, or the bare shorthand `return
    /// :>> target;`. Mirrors `lower_parameter_declaration`'s shape (reusing the same
    /// `DeclarationKind::ParameterUsage` -- a case return is itself an output-parameter-like
    /// feature, same as a calc's own `ReturnDecl`): ownership, membership, a `FeatureTyping` (`:`)
    /// or `Subsetting` (`:>`, `is_subsetting`) reference to the declared type, an authored
    /// `Redefinition` reference for the `:>>`-shorthand `target` (mirrors `VerifyRequirementMember::
    /// redefines`'s identical bare-`QualifiedReferenceId` handling), and a bound `=`/`:=` value
    /// through the same `classify_expression`/`lower_calc_expression` pipeline `lower_return_
    /// decl` uses. `declaration_name` is empty for the common anonymous `return : Type = expr;`
    /// form; `intern_declared_name` folds that to `None`. The `part`/`attribute` `feature_kind`
    /// prefix and `multiplicity` are not modeled as distinct facts, mirroring `lower_parameter_
    /// declaration`'s own out-of-scope fields.
    pub(crate) fn lower_case_return_decl(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<CaseReturnDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.declaration_name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ParameterUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: if node.value.is_subsetting {
                    ReferenceKind::Subsetting
                } else {
                    ReferenceKind::FeatureTyping
                },
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(target) = node.value.target {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::Redefinition,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        // The trailing `:>>` clause on a *named* return declaration (`return verdict :
        // VerdictKind :>> result;`). `target` above is the leading anonymous form, where the
        // redefinition target stands in for the declaration name; both spellings lower to the
        // same `Redefinition` relationship.
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(
                declaration,
                self.calc_expression_site(document, &expression.value),
            );
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        Ok(())
    }

    /// Lowers an `occurrence def` (BNF OccurrenceDefinition), mirroring `lower_port_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/item/part/nested-occurrence declarations. Occurrence-specific semantics
    /// (individual/portion-of-life, time-slicing, snapshot facts, `exhibit`/`succession`/
    /// `satisfy`/`allocate`/connector-end body constructs) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_occurrence_definition_member` via
    /// `lower_occurrence_body_element`. `OccurrenceDef.body` is the generic `DefinitionBody`
    /// (shared with e.g. `ItemDef`), which wraps the same `OccurrenceBodyElement` that
    /// `OccurrenceUsage.body` (`OccurrenceUsageBody`) holds directly -- both def and usage publish
    /// under one `UnsupportedFamily::OccurrenceDefinitionMember`.
    pub(crate) fn lower_occurrence_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<OccurrenceDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let short_name = self.intern_short_name(node.identification.short_name.as_ref())?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
                    individual: node.value.is_individual,
                    ..DeclarationModifiers::default()
                },
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        let DefinitionBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers one `OccurrenceBodyElement`, shared by `occurrence def`'s body (wrapped in
    /// `DefinitionBodyElement::OccurrenceMember`), an `occurrence` usage's own owned members
    /// (`OccurrenceUsageBody` holds `OccurrenceBodyElement` directly), and `allocation def`/
    /// `flow def` bodies (also `DefinitionBodyElement::OccurrenceMember`): recognized owned
    /// members are attribute/part/item/nested-occurrence usages plus `end` declarations (lowered
    /// as connector-end references through the same `lower_end_decl`/`ReferenceKind::ConnectorEnd`
    /// machinery `connection def`/`interface def` use), plus `assert constraint` members
    /// (`lower_assert_constraint_member`); everything else -- flow usages, succession usages,
    /// `satisfy`, `allocate`, `exhibit` state usages -- falls through to
    /// `unsupported_occurrence_definition_member`.
    pub(crate) fn lower_occurrence_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<OccurrenceBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            OccurrenceBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            OccurrenceBodyElement::Bind(node) => {
                // New upstream member kind: kept visible as unsupported rather than dropped.
                self.push_unsupported(
                    document,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    node.span.clone(),
                );
            }
            OccurrenceBodyElement::Annotating(member) => {
                self.lower_annotating_member(
                    document,
                    Some(owner),
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    member,
                )?;
            }
            OccurrenceBodyElement::AttributeUsage(attribute) => {
                self.lower_attribute_usage(document, Some(owner), attribute)?;
            }
            OccurrenceBodyElement::PartUsage(part) => {
                self.lower_part_usage(document, Some(owner), part)?;
            }
            OccurrenceBodyElement::ItemUsage(item) => {
                self.lower_item_usage(document, Some(owner), item)?;
            }
            OccurrenceBodyElement::OccurrenceUsage(occurrence) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence)?;
            }
            OccurrenceBodyElement::EndDecl(end_decl) => {
                self.lower_end_decl(document, owner, end_decl)?;
            }
            OccurrenceBodyElement::RefDecl(ref_decl) => {
                self.lower_ref_decl(document, Some(owner), ref_decl)?;
            }
            OccurrenceBodyElement::ConnectionUsage(connection_usage) => {
                self.lower_connection_usage(document, Some(owner), connection_usage)?;
            }
            OccurrenceBodyElement::StateUsage(state_usage) => {
                self.lower_state_usage(document, Some(owner), state_usage)?;
            }
            OccurrenceBodyElement::Satisfy(node) => {
                self.lower_satisfy(
                    document,
                    owner,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    node,
                )?;
            }
            OccurrenceBodyElement::Allocate(node) => {
                self.lower_allocate(
                    document,
                    owner,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    node,
                )?;
            }
            OccurrenceBodyElement::AssertConstraint(node) => self.lower_assert_constraint_member(
                document,
                owner,
                UnsupportedFamily::OccurrenceDefinitionMember,
                node,
            )?,
            OccurrenceBodyElement::MetadataKeywordUsage(_)
            | OccurrenceBodyElement::FlowUsage(_)
            | OccurrenceBodyElement::SuccessionUsage(_) => self.push_unsupported(
                document,
                UnsupportedFamily::OccurrenceDefinitionMember,
                element.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `occurrence` feature member (BNF OccurrenceUsage),
    /// e.g. `occurrence o;` or `occurrence o : SomeOccurrence;`, mirroring `lower_port_usage`.
    /// `type_name` is a bare `QualifiedReferenceId` (like `ItemUsage`/`MetadataUsage`), not a
    /// structured `TypingRelationship`, but does carry an independent `type_is_conjugated` flag
    /// (mirrored as an explicit `RelationshipFlags::conjugated` fact on the pushed `FeatureTyping`
    /// reference, the same convention `lower_typing_relationship` uses for `PortUsage`). Individual/
    /// event/portion-of-life prefixes (`individual`/`then`/`event`/`ref`/`abstract`/`constant`,
    /// `portion_kind`) and the `event path` occurrence-reference shorthand are explicitly out of
    /// scope -- only the ordinary declaration/typing/subsetting shape is lowered. Owned members
    /// lower through the shared `lower_occurrence_body_element` (both def and usage share
    /// `OccurrenceBodyElement`).
    pub(crate) fn lower_occurrence_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserOccurrenceUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    event: node.value.is_event,
                    ..occurrence_prefix_modifiers(&node.value.prefix)
                },
                portion_kind: portion_kind_node_fact(node.value.prefix.portion.as_ref()),
                direction: direction_node_fact(
                    node.value.prefix.basic.ref_prefix.direction.as_ref(),
                ),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    conjugated: node.value.type_is_conjugated,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        let OccurrenceUsageBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            self.lower_occurrence_body_element(document, declaration, element)?;
        }
        Ok(())
    }
}
