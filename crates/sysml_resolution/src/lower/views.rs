//! Phase 2 lowering — views and renderings.

use crate::lower::facts::definition_prefix_node_modifiers;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::FilterForm;
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
use sysml_v2_parser::ast::{
    MembershipKind as ParserMembershipKind, Node, RenderingDef, RenderingDefBody,
    RenderingDefBodyElement, RenderingUsage as ParserRenderingUsage, RenderingUsageBody,
    RenderingUsageBodyElement, ViewBody, ViewBodyElement, ViewDef, ViewDefBody, ViewDefBodyElement,
    ViewUsage as ParserViewUsage,
};

impl SemanticModelBuilder {
    /// Lowers a `view def` (BNF ViewDefinition), mirroring `lower_interface_def`: ownership,
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// Subclassification/FeatureTyping `DeclarationDomain::Type` fixed point). View-specific
    /// body members (`render`, `filter`) are out of scope -- see `DeclarationKind::ViewDefinition`'s
    /// doc comment and planning/UPSTREAM_PARSER_GAPS.md #8.
    pub(crate) fn lower_view_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ViewDef>,
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
            DeclarationKind::ViewDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
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
        self.lower_view_def_body(document, declaration, &node.value.body)
    }

    /// Body walker for `view def` bodies (`ViewDefBody`/`ViewDefBodyElement`). `filter`/`render`
    /// members are out of scope for this slice and fall through to
    /// `unsupported_view_definition_member`.
    pub(crate) fn lower_view_def_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ViewDefBody,
    ) -> Result<(), ConstructionError> {
        if let ViewDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ViewDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ViewDefBodyElement::AliasDef(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::ViewDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    ViewDefBodyElement::RenderingUsage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::ViewDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    ViewDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ViewDefinitionMember,
                        element.span.clone(),
                    ),
                    ViewDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ViewDefinitionMember,
                            member,
                        )?;
                    }
                    ViewDefBodyElement::RefDecl(ref_decl) => {
                        self.lower_ref_decl(document, Some(declaration), ref_decl)?;
                    }
                    ViewDefBodyElement::ViewpointUsage(viewpoint_usage) => {
                        self.lower_viewpoint_usage(document, Some(declaration), viewpoint_usage)?;
                    }
                    ViewDefBodyElement::Satisfy(node) => {
                        self.lower_satisfy(
                            document,
                            declaration,
                            UnsupportedFamily::ViewDefinitionMember,
                            node,
                        )?;
                    }
                    ViewDefBodyElement::Filter(filter) => {
                        self.lower_filter_condition(
                            document,
                            declaration,
                            FilterForm::View,
                            &filter.value.condition,
                        )?;
                    }
                    ViewDefBodyElement::Unsupported(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                    ViewDefBodyElement::ViewRendering(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ViewDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `view` feature member (BNF ViewUsage), mirroring
    /// `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #8): `ViewUsage` previously had no `subsets` field. Multiplicity
    /// and view-specific body members (`render`/`filter`) are out of scope for this slice.
    pub(crate) fn lower_view_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserViewUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let short_name = self.intern_short_name(node.value.short_name.as_ref())?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewUsage,
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
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_view_usage_body(document, declaration, &node.value.body)
    }

    /// Body walker for `view` usage bodies (`ViewBody`/`ViewBodyElement`), mirroring
    /// `lower_view_def_body`. `filter`/`render` members are out of scope for this slice and fall
    /// through to `unsupported_view_definition_member`.
    pub(crate) fn lower_view_usage_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ViewBody,
    ) -> Result<(), ConstructionError> {
        if let ViewBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ViewBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ViewBodyElement::AliasDef(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::ViewDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    ViewBodyElement::RenderingUsage(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::ViewDefinitionMember,
                            node.span.clone(),
                        );
                    }
                    ViewBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ViewDefinitionMember,
                            member,
                        )?;
                    }
                    ViewBodyElement::Satisfy(node) => {
                        self.lower_satisfy(
                            document,
                            declaration,
                            UnsupportedFamily::ViewDefinitionMember,
                            node,
                        )?;
                    }
                    ViewBodyElement::RefDecl(ref_decl) => {
                        self.lower_ref_decl(document, Some(declaration), ref_decl)?;
                    }
                    ViewBodyElement::Filter(filter) => {
                        self.lower_filter_condition(
                            document,
                            declaration,
                            FilterForm::View,
                            &filter.value.condition,
                        )?;
                    }
                    ViewBodyElement::Expose(node) => {
                        self.lower_expose(document, declaration, node)?;
                    }
                    ViewBodyElement::ViewRendering(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ViewDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `rendering` feature member (BNF RenderingUsage),
    /// mirroring `lower_view_usage`: ownership, membership, a `:` typing target, and
    /// `subsets`/`redefines` subsetting relationships. `ast::RenderingUsage` now carries full
    /// field parity with `ViewUsage` (planning/UPSTREAM_PARSER_GAPS.md #26, resolved upstream in
    /// `cb026cd`) -- `is_abstract`/`multiplicity`/`ordered`/`nonunique`/`value` are not modeled as
    /// distinct facts here (see `DeclarationKind::RenderingUsage`).
    pub(crate) fn lower_rendering_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserRenderingUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RenderingUsage,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
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
                flags: RelationshipFlags::default(),
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
        self.lower_rendering_usage_body(document, declaration, &node.value.body)
    }

    /// Body walker for `rendering` usage bodies (`RenderingUsageBody`/
    /// `RenderingUsageBodyElement`). Nested `view`/`rendering` usage members recurse through
    /// `lower_view_usage`/`lower_rendering_usage` themselves (the same shape a package-level
    /// `view`/`rendering` member uses); anything else falls through to
    /// `UnsupportedFamily::PackageMember`, matching the top-level dispatch this body's owner was
    /// itself lowered from.
    pub(crate) fn lower_rendering_usage_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &RenderingUsageBody,
    ) -> Result<(), ConstructionError> {
        if let RenderingUsageBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    RenderingUsageBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    RenderingUsageBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::RenderingDefinitionMember,
                            member,
                        )?;
                    }
                    RenderingUsageBodyElement::ViewUsage(node) => {
                        self.lower_view_usage(document, Some(declaration), node)?;
                    }
                    RenderingUsageBodyElement::Rendering(node) => {
                        self.lower_rendering_usage(document, Some(declaration), node)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn lower_rendering_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RenderingDef>,
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
            DeclarationKind::RenderingDefinition,
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract,
                    variation,
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
        let RenderingDefBody::Brace { elements, .. } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RenderingDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                RenderingDefBodyElement::Annotating(member) => {
                    self.lower_annotating_member(
                        document,
                        Some(declaration),
                        UnsupportedFamily::RenderingDefinitionMember,
                        member,
                    )?;
                }
                RenderingDefBodyElement::Filter(filter) => {
                    self.lower_filter_condition(
                        document,
                        declaration,
                        FilterForm::Rendering,
                        &filter.value.condition,
                    )?;
                }
                RenderingDefBodyElement::RefDecl(ref_decl) => {
                    self.lower_ref_decl(document, Some(declaration), ref_decl)?;
                }
                RenderingDefBodyElement::Unsupported(node) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                ),
                RenderingDefBodyElement::ViewRendering(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::RenderingDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }
}
