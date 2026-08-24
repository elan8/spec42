//! Phase 2 lowering — requirements, cases, viewpoints, concerns, and their satisfaction members.

use crate::evaluate::classify::flatten_member_access_chain;
use crate::lower::facts::definition_prefix_node_modifiers;
use crate::lower::facts::direction_fact;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::occurrence_prefix_modifiers;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::SemanticModelBuilder;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use std::sync::Arc;
use sysml_v2_parser::ast::{
    ActorUsage, AnalysisCaseDef, AnalysisCaseUsage as ParserAnalysisCaseUsage, CaseDef,
    CaseUsage as ParserCaseUsage, ConcernUsage as ParserConcernUsage, Expression, FrameMember,
    IncludeUseCase, MembershipKind as ParserMembershipKind, Node, PurposeMember,
    QualifiedReferenceId, ReferenceSeparator, RequirementActorDecl, RequirementDef,
    RequirementDefBody, RequirementDefBodyElement, RequirementUsage as ParserRequirementUsage,
    SatisfiedRequirement, SatisfyRequirementUsage, StakeholderMember, SubjectDecl, UseCaseDef,
    UseCaseDefBody, UseCaseDefBodyElement, UseCaseUsage as ParserUseCaseUsage, VerificationCaseDef,
    VerificationCaseUsage as ParserVerificationCaseUsage, VerifyRequirementMember, ViewpointDef,
    ViewpointUsage as ParserViewpointUsage,
};

impl SemanticModelBuilder {
    /// Lowers a `subject` declaration (BNF `SubjectDecl`) found in a requirement/concern/case-
    /// family def or usage body, e.g. `subject vehicle : Vehicle;`, mirroring
    /// `lower_parameter_declaration`'s shape: ownership, membership, and (when a type is present)
    /// a `FeatureTyping` reference to the declared type. No direction fact applies here.
    /// Multiplicity, the bound `= expr` value, and the bare `subject = expr;`/`subject;`
    /// shorthand forms (`ast::SubjectRef`, handled separately) are out of scope.
    pub(crate) fn lower_subject_decl(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<SubjectDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let short_name = self.intern_short_name(document, node.value.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::SubjectUsage,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        // Records the authored value spelling (`=`/`:=`/`default`) for this declaration. The
        // value expression itself is not lowered here -- expression coverage for this usage
        // family is unchanged by this fact family.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        Ok(())
    }

    /// Lowers a `stakeholder` member found in a requirement/viewpoint def body (BNF
    /// `StakeholderMember`), mirroring `lower_subject_decl`'s typed-declaration shape (ownership,
    /// membership, an optional `FeatureTyping` reference) plus the concern-reference/redefinition
    /// operand: when `target` is present, it is lowered as an authored `ReferenceKind::
    /// Redefinition` reference (for the `:>>` spelling, `is_redefinition == true`) or
    /// `ReferenceKind::StakeholderTarget` reference (the bare `stakeholder Concern;` spelling)
    /// sourced at the same declaration. `declaration_name` may be empty for either reference form
    /// (`intern_declared_name` already folds that to an anonymous declaration, matching
    /// `SubjectUsage`'s own bare-form handling).
    pub(crate) fn lower_stakeholder_member(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<StakeholderMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.declaration_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StakeholderUsage,
            name,
            node.span,
            // `ast::StakeholderMember` carries no modifier, multiplicity, direction, or short name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        if let Some(target) = node.value.target {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            let kind = if node.value.is_redefinition {
                ReferenceKind::Redefinition
            } else {
                ReferenceKind::StakeholderTarget
            };
            self.push_reference(PendingReference {
                source: declaration,
                kind,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a viewpoint `purpose` member (BNF `PurposeMember`), an always-present concern
    /// reference (`PurposeMember.target`, no plain-declaration/redefinition alternatives the way
    /// `StakeholderMember` has), resolved as an authored `ReferenceKind::PurposeTarget` reference
    /// sourced directly at the enclosing `owner` declaration, mirroring `Variant`'s
    /// single-operand, no-nested-declaration shape.
    pub(crate) fn lower_purpose_member(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<PurposeMember>,
    ) -> Result<(), ConstructionError> {
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(node.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: owner,
            kind: ReferenceKind::PurposeTarget,
            document,
            local: node.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a typed `actor` parameter declaration found in a requirement def body (BNF
    /// `RequirementActorDecl`), mirroring `lower_subject_decl`'s shape (ownership, membership,
    /// a `FeatureTyping` reference to the declared type), except `type_name` is unconditional here
    /// (never optional, unlike `SubjectDecl::type_name`).
    pub(crate) fn lower_requirement_actor_decl(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<RequirementActorDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementActor,
            name,
            node.span,
            DeclarationFacts {
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        let type_name = node.value.type_name;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(type_name)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span;
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::FeatureTyping,
            document,
            local: type_name,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers an `actor` member found in a use-case-family def/usage body (BNF `ActorUsage`,
    /// e.g. `actor driver : Person;`, `actor passengers : Person[0..4];`), mirroring
    /// `lower_requirement_actor_decl`'s shape (ownership, membership, a `FeatureTyping` reference
    /// to the declared type) but reading visibility off `ActorUsage::membership` (kind
    /// `ActorMembership`) instead. The bare untyped form (`actor environment;`) authors no type,
    /// so it contributes the declaration and its membership and no typing reference. The optional
    /// trailing multiplicity is not modeled as a distinct fact, mirroring `lower_subject_decl`'s
    /// own out-of-scope multiplicity.
    pub(crate) fn lower_actor_usage(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        node: &Node<ActorUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::CaseActor,
            name,
            node.span,
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
                ParserMembershipKind::ActorMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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

    /// Lowers a named `frame` member found in a requirement def body (BNF `FrameMember`) as a
    /// purely syntactic named grouping: ownership, membership, and its nested `RequirementDefBody`
    /// content dispatched back through the same shared `lower_requirement_shaped_body` walker used
    /// by `requirement def`/`requirement` usage/`viewpoint def` bodies, sharing the caller-supplied
    /// `unsupported` family so a member unrecognized inside a frame reports under the same
    /// diagnostic family as one outside it.
    pub(crate) fn lower_frame_member(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        unsupported: UnsupportedFamily,
        node: &Node<FrameMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Frame,
            name,
            node.span,
            // A `frame` member is a purely syntactic named grouping; `ast::FrameMember` carries no
            // modifier, multiplicity, direction, or short name.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        self.lower_requirement_shaped_body(document, declaration, &node.value.body, unsupported)
    }

    /// Lowers a requirement/objective-body `verify <requirement>;` shorthand body element (BNF
    /// `VerifyRequirementMember`, `explicit_requirement_keyword == false`) as an anonymous feature
    /// owned by the enclosing declaration, mirroring `Satisfy`'s nested-declaration shape: the
    /// shorthand `target` is lowered as an authored `ReferenceKind::VerifyRequirementTarget`
    /// reference, and an optional `:>>` `redefines` target is lowered as an authored
    /// `ReferenceKind::Redefinition` reference, both sourced at this new declaration. The fuller
    /// `verify requirement <name> : <Type> { ... }` form (`explicit_requirement_keyword == true`,
    /// which defines a new anonymous requirement usage inline rather than referencing an existing
    /// one) is out of scope and reported as an explicit `family` unsupported diagnostic.
    pub(crate) fn lower_verify_requirement_member(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<VerifyRequirementMember>,
    ) -> Result<(), ConstructionError> {
        // `verify requirement <name> : <Type> { ... }` declares an inline requirement usage
        // rather than referencing an existing one. It is the same `RequirementUsage` production
        // an ordinary `requirement` member spells, so it lowers through the shared walker under
        // the `VerifyRequirement` kind that carries the `RequirementVerificationMembership` role.
        if node.value.explicit_requirement_keyword {
            let Some(requirement) = &node.value.requirement else {
                self.push_unsupported(document, family, node.span);
                return Ok(());
            };
            return self.lower_requirement_usage_as(
                document,
                Some(owner),
                DeclarationKind::VerifyRequirement,
                requirement,
            );
        }
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::VerifyRequirement,
            None,
            node.span,
            // `ast::VerifyRequirementMember` carries only its redefinition target.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        if let Some(target) = node.value.target {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::VerifyRequirementTarget,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(redefines) = node.value.redefines {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(redefines)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::Redefinition,
                document,
                local: redefines,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a `SatisfyRequirementUsage` body element (`ast::SatisfyRequirementUsage`) as an
    /// anonymous `DeclarationKind::Satisfy` feature owned by the enclosing `owner` declaration,
    /// mirroring `lower_transition`'s nested-declaration shape.
    ///
    /// There is one satisfy production, and every scope that accepts a satisfy usage -- package,
    /// part def, part usage, occurrence, requirement, view def, and view usage bodies -- reaches
    /// it the same way, so all of them lower through here. The `by` clause's
    /// `SatisfactionSubjectMember` and the reference alternative's `OwnedReferenceSubsetting` are
    /// both source-backed `QualifiedReferenceId`s rather than expressions, so each resolves
    /// directly as an authored `SatisfySource`/`SatisfyTarget` reference through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point `Succession`/`TransitionSource` use.
    ///
    /// `by` is optional in the production, so a satisfy usage written without one carries no
    /// `SatisfyTarget` reference at all -- the satisfied requirement is never copied over to
    /// fabricate a subject. The `assert` prefix and the `not` negation (`negated`) do not
    /// change how the references resolve.
    ///
    /// Out of scope, left as an explicit `family` unsupported diagnostic: the
    /// `'requirement' UsageDeclaration` alternative (`SatisfiedRequirement::Declaration`, which
    /// declares a new requirement inline rather than referencing an existing one -- a meaningfully
    /// different construct, not merely an unresolved reference) and the members of the
    /// `RequirementBody` the usage owns.
    pub(crate) fn lower_satisfy(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<SatisfyRequirementUsage>,
    ) -> Result<(), ConstructionError> {
        let SatisfiedRequirement::Reference { reference } = node.value.requirement else {
            self.push_unsupported(document, family, node.span);
            return Ok(());
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Satisfy,
            None,
            node.span,
            // Negation is a satisfaction-polarity fact rather than a declaration modifier.
            DeclarationFacts {
                negated: Some(node.value.not_span.is_some()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        self.push_satisfy_reference(
            document,
            declaration,
            ReferenceKind::SatisfySource,
            reference,
        )?;
        if let Some(subject) = &node.value.subject {
            self.push_satisfy_reference(
                document,
                declaration,
                ReferenceKind::SatisfyTarget,
                subject.value.reference,
            )?;
        }
        for element in node.value.body.members() {
            self.push_unsupported(document, family, element.span);
        }
        Ok(())
    }

    /// Pushes one of a satisfy usage's two source-backed operands at its anonymous satisfy
    /// declaration. The parser preserves each segment separator: a dotted feature path is routed
    /// through the canonical type-directed member-access resolver, while a `::` qualified name
    /// keeps ordinary namespace lookup.
    pub(crate) fn push_satisfy_reference(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        kind: ReferenceKind,
        reference: QualifiedReferenceId,
    ) -> Result<(), ConstructionError> {
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let parsed_reference = parsed
            .qualified_reference(reference)
            .ok_or(ConstructionError::InvalidParserReference)?;
        let span = parsed_reference.metadata.span;
        if parsed_reference
            .segments
            .iter()
            .any(|segment| segment.separator_before == Some(ReferenceSeparator::Dot))
        {
            if matches!(
                kind,
                ReferenceKind::AllocateSource | ReferenceKind::AllocateTarget
            ) {
                self.push_member_access_reference_with_kind(
                    declaration,
                    document,
                    kind,
                    &[reference],
                    span,
                )?;
            } else {
                self.push_member_access_reference(declaration, document, &[reference], span)?;
            }
            return Ok(());
        }
        self.push_reference(PendingReference {
            source: declaration,
            kind,
            document,
            local: reference,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers one `Satisfy` operand (`source`/`target`), mirroring `lower_transition_end`: its
    /// path expression is a structured `Expression`, so a simple/qualified name
    /// (`Expression::FeatureRef`) resolves as an authored reference of `kind` through the shared
    /// `DeclarationDomain::Any` lexical lookup. A dotted feature-chain path
    /// (`Expression::MemberAccess`/`Expression::FeatureChainRef`, e.g. `f.a`) resolves as a
    /// `ReferenceKind::MemberAccessOperand` reference instead, through the same
    /// `flatten_member_access_chain`/`push_member_access_reference` path `lower_connector_end`
    /// uses -- this is also `Bind`'s (`lower_bind`) operand path, since it shares this helper, so
    /// `bind f.a = a.g;` resolves both dotted operands the same way `connect f.a to a.g;` does.
    /// Also supports `Expression::Invocation`/`Expression::Constructor` (reference resolution
    /// only, via `lower_invocation_callee`/`ReferenceKind::InvocationCallee`, recursing arguments
    /// back into this same function with `kind` unchanged). Any other expression shape falls
    /// through to an explicit `family` unsupported diagnostic.
    pub(crate) fn lower_satisfy_operand(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        family: UnsupportedFamily,
        kind: ReferenceKind,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: owner,
                    kind,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            Expression::MemberAccess { .. } | Expression::FeatureChainRef(_) => {
                if let Some(chain) = flatten_member_access_chain(node) {
                    self.push_member_access_reference(owner, document, &chain, node.span)?;
                } else {
                    self.push_unsupported(document, family, node.span);
                }
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, owner, callee, args.len(), node.span)?;
                for arg in args {
                    self.lower_satisfy_operand(document, owner, family, kind, &arg.value)?;
                }
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, owner, *type_name)?;
                for arg in args {
                    self.lower_satisfy_operand(document, owner, family, kind, &arg.value)?;
                }
            }
            _ => self.push_unsupported(document, family, node.span),
        }
        Ok(())
    }

    /// Lowers an `include <includedUseCase>;` body element inside a `use case def`/`use case`
    /// usage body (BNF `UseCaseDefBodyElement::IncludeUseCase`, `ast::IncludeUseCase`) -- see
    /// `ReferenceKind::IncludeUseCase`'s doc comment: a single-operand reference to an existing
    /// use case, sourced directly at the enclosing use case declaration (no anonymous
    /// nested-declaration scope shift), mirroring `lower_variant_usage`'s shape.
    pub(crate) fn lower_include_use_case(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        node: &Node<IncludeUseCase>,
    ) -> Result<(), ConstructionError> {
        match node.value.target {
            // `include <ref>;`: a reference to an existing use case, sourced at the owner.
            Some(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::IncludeUseCase,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            // `include use case <name> [: Type] ...;` (`IncludeUseCaseUsage`'s second
            // alternative, SysML BNF 2300-2306): declares the included use case inline as a
            // `UseCaseUsage` owned by the including case, with `IncludeUseCase` as its membership
            // role -- the same shape `lower_satisfy` gives an inline requirement declaration.
            None => {
                let name = self.intern_short_name(document, node.value.name)?;
                let included = self.push_typed_declaration(
                    document,
                    Some(declaration),
                    DeclarationKind::UseCaseUsage,
                    name,
                    node.span,
                    DeclarationFacts {
                        multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                        ..DeclarationFacts::none()
                    },
                )?;
                self.push_membership(
                    included,
                    MembershipKind::Feature,
                    Visibility::Default,
                    node.span,
                )?;
                if let Some(relationship) = &node.value.typing {
                    self.lower_typing_relationship(document, included, relationship)?;
                }
                self.lower_case_family_def_body(
                    document,
                    included,
                    &node.value.body,
                    UnsupportedFamily::UseCaseDefinitionMember,
                )?;
            }
        }
        Ok(())
    }

    /// Lowers a `requirement def` (BNF RequirementDefinition), mirroring `lower_part_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/requirement members. Requirement-specific semantics (subject binding,
    /// assumption/constraint facts) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_requirement_definition_member`.
    pub(crate) fn lower_requirement_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<RequirementDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.value.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementDefinition,
            name,
            node.span,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `requirement` feature member (BNF
    /// RequirementUsage), mirroring `lower_part_usage`: ownership, membership, an optional
    /// `:`/`:>` typing reference, `subsets`/`references` subsetting relationships, and owned
    /// attribute/requirement members.
    pub(crate) fn lower_requirement_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserRequirementUsage>,
    ) -> Result<(), ConstructionError> {
        self.lower_requirement_usage_as(document, owner, DeclarationKind::RequirementUsage, node)
    }

    /// The `RequirementUsage` lowering, parameterized by the declaration kind the owning
    /// membership gives it. An ordinary `requirement r : R;` is a `RequirementUsage`; the same
    /// production owned by a `RequirementVerificationMembership` (`verify requirement limit :
    /// Limit;`, BNF `VerifyRequirementMember` with `explicit_requirement_keyword == true`) is a
    /// `VerifyRequirement`, because the kind is what `membership_role` reads to derive
    /// `MembershipRole::RequirementVerification` -- and that role is the prerequisite of the
    /// generated `checkRequirementUsageRequirementVerificationSpecialization` library
    /// specialization. Everything else about the declaration is identical, so the two forms share
    /// one walker rather than a copy that could drift.
    pub(crate) fn lower_requirement_usage_as(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        kind: DeclarationKind,
        node: &Node<ParserRequirementUsage>,
    ) -> Result<(), ConstructionError> {
        self.lower_requirement_usage_as_with_implicit_name(document, owner, kind, node, None)
    }

    /// Lowers the shared requirement-usage payload while keeping a grammar-owned implicit role
    /// name separate from an authored declaration name. `objective { ... }` has no authored NAME,
    /// but the ObjectiveMembership canonically names its owned requirement `objective`; parser AST
    /// v240 correctly stopped fabricating that spelling in the syntax tree.
    fn lower_requirement_usage_as_with_implicit_name(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        kind: DeclarationKind,
        node: &Node<ParserRequirementUsage>,
        implicit_name: Option<&str>,
    ) -> Result<(), ConstructionError> {
        let name = match node.value.name {
            Some(name) => self.intern_declaration_name(document, Some(name))?,
            None => implicit_name
                .map(|name| self.intern_name(name))
                .transpose()?,
        };
        let short_name = self.intern_short_name(document, node.value.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            kind,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    variation: node.value.is_variation,
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
            node.value.membership.span,
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
                .span;
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
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the shared `RequirementDefBody` used by both `requirement def` and `requirement`
    /// usage bodies: recognized owned members are attribute def/usage and nested requirement
    /// usages; everything else falls through to `unsupported_requirement_definition_member` via
    /// the single `RequirementDefinitionMember` family (both def and usage bodies share the same
    /// grammar production, `RequirementBody`, so there is no def/usage-specific distinction to
    /// make here).
    pub(crate) fn lower_requirement_def_body(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        body: &RequirementDefBody,
    ) -> Result<(), ConstructionError> {
        self.lower_requirement_shaped_body(
            document,
            owner,
            body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Shared body walker for grammar productions using `RequirementDefBody`/
    /// `RequirementDefBodyElement` (`requirement def`/`requirement` usage and `viewpoint def`),
    /// parameterized by the caller-supplied `unsupported` family so each kind's diagnostics stay
    /// distinct even though the typed AST body shape is identical.
    pub(crate) fn lower_requirement_shaped_body(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        body: &RequirementDefBody,
        unsupported: UnsupportedFamily,
    ) -> Result<(), ConstructionError> {
        let RequirementDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RequirementDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span);
                }
                RequirementDefBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                RequirementDefBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                RequirementDefBodyElement::RequirementUsage(requirement) => {
                    self.lower_requirement_usage(document, Some(owner), requirement)?;
                }
                RequirementDefBodyElement::Import(import) => {
                    self.lower_import(document, Some(owner), import)?;
                }
                RequirementDefBodyElement::SubjectDecl(subject) => {
                    self.lower_subject_decl(document, Some(owner), subject)?;
                }
                RequirementDefBodyElement::Constraint(constraint) => {
                    self.lower_constraint_usage(document, Some(owner), constraint)?;
                }
                RequirementDefBodyElement::Annotating(member) => {
                    self.lower_annotating_member(document, Some(owner), unsupported, member)?;
                }
                // `subject;` shorthand: an entirely empty AST node (`ast::requirement::SubjectRef`
                // has no fields at all) referencing the case-family subject already established
                // elsewhere -- nothing to lower, so it is recognized and silently ignored rather
                // than reported as an unsupported member, mirroring `Doc`/`TextualRep`'s inert
                // handling above.
                RequirementDefBodyElement::SubjectRef(_) => {}
                RequirementDefBodyElement::RequirementActorDecl(actor) => {
                    self.lower_requirement_actor_decl(document, Some(owner), actor)?;
                }
                RequirementDefBodyElement::Stakeholder(stakeholder) => {
                    self.lower_stakeholder_member(document, Some(owner), stakeholder)?;
                }
                RequirementDefBodyElement::Purpose(purpose) => {
                    self.lower_purpose_member(document, owner, purpose)?;
                }
                RequirementDefBodyElement::VerifyRequirement(verify) => {
                    self.lower_verify_requirement_member(document, owner, unsupported, verify)?;
                }
                RequirementDefBodyElement::Frame(frame) => {
                    self.lower_frame_member(document, owner, unsupported, frame)?;
                }
                RequirementDefBodyElement::VariantUsage(node) => {
                    self.lower_variant_usage(document, owner, unsupported, node)?;
                }
                RequirementDefBodyElement::RequireConstraint(node) => {
                    self.lower_require_constraint_member(document, owner, unsupported, node)?;
                }
                RequirementDefBodyElement::RefDecl(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                // The usage families a `requirement def` body inherits from the general member
                // grammar, admitted upstream in `ec47463` (planning/UPSTREAM_PARSER_GAPS.md gap 42).
                // Each dispatches to the lowering its package- or part-level spelling already uses;
                RequirementDefBodyElement::ActionUsage(node) => {
                    self.lower_action_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::Perform(node) => {
                    self.lower_perform(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::StateUsage(node) => {
                    self.lower_state_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::ItemUsage(node) => {
                    self.lower_item_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::PartUsage(node) => {
                    self.lower_part_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::ConnectionUsage(node) => {
                    self.lower_connection_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::Connect(node) => {
                    self.lower_bare_connect(document, owner, unsupported, node)?;
                }
                RequirementDefBodyElement::SuccessionUsage(node) => {
                    self.lower_succession_usage(document, owner, unsupported, node)?;
                }
                // The three member families upstream added to close the `requirement def` half of
                // planning/UPSTREAM_PARSER_GAPS.md gap 42: a nested definition of the body's own
                // kind, and the `port`/`allocate` members the SysML v2 spec annex authors. Each
                // dispatches to the lowering its package-level spelling already uses.
                RequirementDefBodyElement::RequirementDef(node) => {
                    self.lower_requirement_def(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::PortUsage(node) => {
                    self.lower_port_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::AllocationUsage(node) => {
                    self.lower_allocation_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::ConcernUsage(node) => {
                    self.lower_concern_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::CalcUsage(node) => {
                    self.lower_calc_usage(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::Dependency(node) => {
                    self.lower_dependency(document, Some(owner), node)?;
                }
                RequirementDefBodyElement::Satisfy(node) => {
                    self.lower_satisfy(document, owner, unsupported, node)?;
                }
                RequirementDefBodyElement::MetadataKeywordUsage(_) => {
                    self.push_unsupported(document, unsupported, element.span)
                }
            }
        }
        Ok(())
    }

    /// Lowers a `viewpoint def` (BNF ViewpointDefinition), mirroring `lower_requirement_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/nested-requirement members via the shared `RequirementDefBody` walker.
    /// Stakeholder/concern-binding semantics are out of scope; unrecognized body elements fall
    /// through to `unsupported_requirement_definition_member` (the same family `requirement def`
    /// uses, since `ViewpointDef` shares its exact body shape). `viewpoint` usage lowering is
    /// deferred -- see `DeclarationKind::ViewpointDefinition`'s doc comment.
    pub(crate) fn lower_viewpoint_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ViewpointDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.value.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewpointDefinition,
            name,
            node.span,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_shaped_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `viewpoint` feature member (BNF ViewpointUsage),
    /// mirroring `lower_viewpoint_def`: ownership, membership, a `:` typing target, and owned
    /// members via the same shared `lower_requirement_shaped_body` walker, plus the header-level
    /// `:>`/`:>>` clauses through the shared `lower_subsetting_relationship`, exactly as
    /// `lower_concern_usage` handles its own pair.
    pub(crate) fn lower_viewpoint_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserViewpointUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ViewpointUsage,
            name,
            node.span,
            // No modifier, multiplicity, or short-name field on `ast::ViewpointUsage`; its
            // `subsets`/`redefines` clauses are relationships, pushed below rather than facts.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_requirement_shaped_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Lowers a package-level `concern` member (BNF ConcernUsage), dispatching on
    /// `is_definition` to either `concern def` (`DeclarationKind::ConcernDefinition`, Owning
    /// membership, mirroring `lower_viewpoint_def`'s owned-type shape) or a bare `concern` usage
    /// (`DeclarationKind::ConcernUsage`, Feature membership, mirroring `lower_requirement_usage`).
    /// Both forms share the same parsed fields -- `parser::requirement::concern_usage` calls the
    /// same `feature_usage_header` for both textual forms, so there is no separate `specializes:
    /// Node<TypingRelationship>` for the `def` form the way `RequirementDef`/`ViewpointDef` have;
    /// `type_name`/`subsets`/`redefines` are lowered identically (`FeatureTyping`/`Subsetting`/
    /// `Redefinition`) regardless of `is_definition`. The parser folds both textual forms into
    /// this single struct (see `ast::requirement::ConcernUsage`'s doc comment) rather than a
    /// distinct `ConcernDef` type. Genuinely new: previously blocked entirely
    /// (planning/UPSTREAM_PARSER_GAPS.md #9), resolved upstream in `0757de13`. Stakeholder/subject-binding
    /// semantics are out of scope, sharing `UnsupportedFamily::RequirementDefinitionMember` with
    /// `requirement def`/`viewpoint def`.
    pub(crate) fn lower_concern_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserConcernUsage>,
    ) -> Result<(), ConstructionError> {
        // `parser::requirement::concern_usage` always constructs `Membership::feature(...)`
        // regardless of `is_definition` (there is no distinct owning-membership constructor call
        // for the `def` textual form the way other `*Def`/`*Usage` pairs have), so
        // `member_visibility` is always checked against `FeatureMembership` here even though the
        // `def` form maps to our own `MembershipKind::Owning`.
        let (kind, membership_kind) = if node.value.is_definition {
            (DeclarationKind::ConcernDefinition, MembershipKind::Owning)
        } else {
            (DeclarationKind::ConcernUsage, MembershipKind::Feature)
        };
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        // `ast::ConcernUsage` carries no direction or short name.
        let declaration = self.push_typed_declaration(
            document,
            owner,
            kind,
            name,
            node.span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            membership_kind,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_requirement_shaped_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::RequirementDefinitionMember,
        )
    }

    /// Lowers an `analysis def` (BNF AnalysisCaseDefinition), mirroring `lower_requirement_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/nested members via the shared `UseCaseDefBody`. Analysis-case-specific semantics
    /// (subject binding, objective, result parameter binding) are explicitly out of scope;
    /// unrecognized body elements (including nested `analysis` usages -- see
    /// planning/UPSTREAM_PARSER_GAPS.md #5) fall through to `unsupported_analysis_case_definition_member`.
    /// `analysis` usage lowering itself is deferred entirely (same doc entry): `AnalysisCaseUsage`
    /// silently drops parsed `:>`/`:>>` clauses, unlike `AnalysisCaseDef`.
    pub(crate) fn lower_analysis_case_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<AnalysisCaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.value.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AnalysisCaseDefinition,
            name,
            node.span,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_analysis_case_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `UseCaseDefBody` owned by an `analysis def`: recognized owned members are
    /// attribute def/usage; everything else (subject/actor/objective/succession/nested
    /// action/analysis/calc/requirement/part usages, etc.) falls through to
    /// `unsupported_analysis_case_definition_member`.
    pub(crate) fn lower_analysis_case_def_body(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        body: &UseCaseDefBody,
    ) -> Result<(), ConstructionError> {
        self.lower_case_family_def_body(
            document,
            owner,
            body,
            UnsupportedFamily::AnalysisCaseDefinitionMember,
        )
    }

    /// Lowers a `case def` (BNF CaseDefinition), mirroring `lower_analysis_case_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned attribute/nested
    /// members via the shared `UseCaseDefBody`. Case-specific semantics (subject binding,
    /// objective, first-succession/return structure) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_case_definition_member`. `case` usage lowering
    /// is deferred entirely (planning/UPSTREAM_PARSER_GAPS.md #5): `CaseUsage` silently drops parsed
    /// `:>`/`:>>` clauses, unlike `CaseDef`.
    pub(crate) fn lower_case_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<CaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.value.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CaseDefinition,
            name,
            node.span,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::CaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `analysis` feature member (BNF
    /// AnalysisCaseUsage), mirroring `lower_requirement_usage`: ownership, membership, a `:`
    /// typing target (bare `QualifiedReferenceId`, pushed as a `FeatureTyping` reference), and
    /// `subsets`/`redefines` subsetting relationships. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #5): `AnalysisCaseUsage` previously had no typed field to lower
    /// these relationships from.
    pub(crate) fn lower_analysis_case_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserAnalysisCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AnalysisCaseUsage,
            name,
            node.span,
            DeclarationFacts {
                modifiers: occurrence_prefix_modifiers(&node.value.prefix),
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
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::AnalysisCaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `case` feature member (BNF CaseUsage), mirroring
    /// `lower_analysis_case_usage` (shares the same field shape). Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #5): `CaseUsage` previously had no typed field to lower
    /// `subsets`/`redefines` from.
    pub(crate) fn lower_case_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CaseUsage,
            name,
            node.span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
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
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::CaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `use case` feature member (BNF UseCaseUsage),
    /// mirroring `lower_case_usage`. `ast::UseCaseUsage` still has no `redefines` field (see
    /// `DeclarationKind::UseCaseUsage`), so `name`/`type_name`/`is_abstract`/`multiplicity`/
    /// `subsets` are lowered as facts; owned members are walked through the shared
    /// `lower_case_family_def_body`.
    pub(crate) fn lower_use_case_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserUseCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::UseCaseUsage,
            name,
            node.span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                // `ast::UseCaseUsage` has no `nonunique` field; see
                // planning/UPSTREAM_PARSER_GAPS.md Gap 53.
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
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::UseCaseDefinitionMember,
        )
    }

    /// Lowers a package/definition/usage-level `verification` feature member (BNF
    /// VerificationCaseUsage), mirroring `lower_use_case_usage` (shares the same field
    /// shape/limitation: no `redefines` field on `ast::VerificationCaseUsage`).
    pub(crate) fn lower_verification_case_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserVerificationCaseUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, Some(node.value.name))?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::VerificationCaseUsage,
            name,
            node.span,
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                // `ast::VerificationCaseUsage` has no `nonunique` field; see
                // planning/UPSTREAM_PARSER_GAPS.md Gap 53.
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
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
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
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::VerificationCaseDefinitionMember,
        )
    }

    /// Lowers a `verification def` (BNF VerificationCaseDefinition), mirroring `lower_case_def`.
    /// Verification-specific semantics are explicitly out of scope; unrecognized body elements
    /// fall through to `unsupported_verification_case_definition_member`. `verification` usage
    /// lowering (`DeclarationKind::VerificationCaseUsage`, `lower_verification_case_usage`): see
    /// its own doc comment for the remaining `redefines` gap.
    pub(crate) fn lower_verification_case_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<VerificationCaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.value.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::VerificationCaseDefinition,
            name,
            node.span,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::VerificationCaseDefinitionMember,
        )
    }

    /// Lowers a `use case def` (BNF UseCaseDefinition), mirroring `lower_case_def`. Use-case-
    /// specific semantics (actor/include structure) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_use_case_definition_member`. `use case` usage
    /// lowering is deferred entirely (planning/UPSTREAM_PARSER_GAPS.md #5): `UseCaseUsage` silently drops
    /// parsed `:>`/`:>>` clauses, unlike `UseCaseDef`.
    pub(crate) fn lower_use_case_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<UseCaseDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.value.identification.short_name)?;
        let (is_abstract, variation) =
            definition_prefix_node_modifiers(node.value.definition_prefix.as_ref());
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::UseCaseDefinition,
            name,
            node.span,
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
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_case_family_def_body(
            document,
            declaration,
            &node.value.body,
            UnsupportedFamily::UseCaseDefinitionMember,
        )
    }

    /// Shared body walker for the case-family def kinds (`analysis def`/`case def`/
    /// `verification def`/`use case def`), all of which share the same `UseCaseDefBody`/
    /// `UseCaseDefBodyElement` shape in the typed AST. Recognized owned members are attribute
    /// def/usage; everything else (subject/actor/objective/succession/nested
    /// action/analysis/calc/requirement/part usages, etc.) falls through to the caller-supplied
    /// `unsupported` family so each def kind's diagnostics stay distinct.
    pub(crate) fn lower_case_family_def_body(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        body: &UseCaseDefBody,
        unsupported: UnsupportedFamily,
    ) -> Result<(), ConstructionError> {
        let UseCaseDefBody::Brace { elements, .. } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                UseCaseDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span);
                }
                UseCaseDefBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                UseCaseDefBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                UseCaseDefBodyElement::AnalysisCaseUsage(analysis_case_usage) => {
                    self.lower_analysis_case_usage(document, Some(owner), analysis_case_usage)?;
                }
                UseCaseDefBodyElement::UseCaseUsage(use_case_usage) => {
                    self.lower_use_case_usage(document, Some(owner), use_case_usage)?;
                }
                UseCaseDefBodyElement::CaseUsage(case_usage) => {
                    self.lower_case_usage(document, Some(owner), case_usage)?;
                }
                UseCaseDefBodyElement::VerificationCaseUsage(verification_case_usage) => {
                    self.lower_verification_case_usage(
                        document,
                        Some(owner),
                        verification_case_usage,
                    )?;
                }
                UseCaseDefBodyElement::ActionUsage(action_usage) => {
                    self.lower_action_usage(document, Some(owner), action_usage)?;
                }
                UseCaseDefBodyElement::CalcUsage(calc_usage) => {
                    self.lower_calc_usage(document, Some(owner), calc_usage)?;
                }
                UseCaseDefBodyElement::RequirementUsage(requirement_usage) => {
                    self.lower_requirement_usage(document, Some(owner), requirement_usage)?;
                }
                UseCaseDefBodyElement::PartUsage(part_usage) => {
                    self.lower_part_usage(document, Some(owner), part_usage)?;
                }
                UseCaseDefBodyElement::SubjectDecl(subject) => {
                    self.lower_subject_decl(document, Some(owner), subject)?;
                }
                UseCaseDefBodyElement::Ref(node) => {
                    self.lower_ref_decl(document, Some(owner), node)?;
                }
                UseCaseDefBodyElement::InOutDecl(param) => {
                    self.lower_parameter_declaration(document, Some(owner), unsupported, param)?;
                }
                UseCaseDefBodyElement::Annotating(member) => {
                    self.lower_annotating_member(document, Some(owner), unsupported, member)?;
                }
                UseCaseDefBodyElement::AssertConstraint(node) => {
                    self.lower_assert_constraint_member(document, owner, unsupported, node)?
                }
                UseCaseDefBodyElement::IncludeUseCase(node) => {
                    self.lower_include_use_case(document, owner, node)?;
                }
                UseCaseDefBodyElement::ThenIncludeUseCase(node) => {
                    self.lower_include_use_case(document, owner, &node.value.include)?;
                }
                // `subject;` shorthand: see the identical-shape `RequirementDefBodyElement::
                // SubjectRef` handling in `lower_requirement_shaped_body` -- an entirely empty AST
                // node with nothing to lower, recognized and silently ignored.
                UseCaseDefBodyElement::SubjectRef(_) => {}
                UseCaseDefBodyElement::ActorUsage(node) => {
                    self.lower_actor_usage(document, owner, node)?;
                }
                // `objective { ... }`/`objective <name> [: Type] { ... }` wraps a fully typed
                // `RequirementUsage` (`Objective::requirement`) -- lower it through the exact same
                // `lower_requirement_usage` pipeline every other requirement-usage site uses.
                // `Objective::visibility` (an outer `private`/`protected`/`public` prefix consumed
                // separately by the parser, before the wrapped `RequirementUsage`'s own membership)
                // is not threaded through; the nested node's own membership visibility is used as
                // authored, mirroring other case-family wrapper nodes' out-of-scope facts.
                UseCaseDefBodyElement::Objective(node) => {
                    self.lower_requirement_usage_as_with_implicit_name(
                        document,
                        Some(owner),
                        DeclarationKind::RequirementUsage,
                        &node.value.requirement,
                        Some("objective"),
                    )?;
                }
                UseCaseDefBodyElement::CaseReturnDecl(node) => {
                    self.lower_case_return_decl(document, owner, unsupported, node)?;
                }
                UseCaseDefBodyElement::Assign(node) => {
                    self.lower_assign_stmt(document, owner, unsupported, node.span, &node.value)?;
                }
                UseCaseDefBodyElement::ForLoop(node) => {
                    self.lower_for_loop(document, owner, unsupported, node.span, &node.value)?;
                }
                UseCaseDefBodyElement::ThenAction(node) => {
                    self.lower_then_action(document, owner, unsupported, node)?;
                }
                UseCaseDefBodyElement::FlowUsage(node) => {
                    self.lower_flow_usage(document, owner, node)?;
                }
                // Bare result expression in an analysis/case body (validation `10a`: `vehicle.
                // mass`) -- mirrors `CalcDefBodyElement::Expression`'s identical shape: the
                // expression is the enclosing case-family declaration's own evaluated result, not
                // a new nested declaration, so it is classified/lowered directly at `owner` through
                // the same `classify_expression`/`lower_calc_expression` pipeline a calc def's
                // bare body expression uses.
                UseCaseDefBodyElement::Expression(expression) => {
                    self.push_evaluation_fact(
                        owner,
                        self.calc_expression_site(document, &expression.value),
                    );
                    self.lower_calc_expression(document, owner, unsupported, expression)?;
                }
                UseCaseDefBodyElement::MetadataKeywordUsage(_)
                | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
                | UseCaseDefBodyElement::FirstSuccession(_)
                | UseCaseDefBodyElement::ThenUseCaseUsage(_)
                | UseCaseDefBodyElement::ThenDone(_)
                | UseCaseDefBodyElement::RefRedefinition(_)
                | UseCaseDefBodyElement::ReturnRef(_) => {
                    self.push_unsupported(document, unsupported, element.span)
                }
            }
        }
        Ok(())
    }
}
