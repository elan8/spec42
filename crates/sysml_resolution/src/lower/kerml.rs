//! Phase 2 lowering — KerML-scope declarations: classifiers, features, connectors, invariants.

use crate::lower::facts::basic_feature_prefix_modifiers;
use crate::lower::facts::direction_node_fact;
use crate::lower::facts::kerml_classifier_kind;
use crate::lower::facts::kerml_feature_kind;
use crate::lower::facts::kerml_feature_prefix_modifiers;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
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
    FeaturePrefixHead, FeatureRelationshipPart, KermlBindingMember, KermlClassifierDecl,
    KermlConnectorEnd, KermlConnectorMember, KermlFeature, KermlInvariantMember,
    KermlSuccessionMember, KermlTypeRelationship, KermlTypeRelationshipKeyword,
    MembershipKind as ParserMembershipKind, Node, OwnedCrossFeature,
};

impl SemanticModelBuilder {
    /// Lowers a bodied KerML classifier declaration (`KermlClassifierDecl`), mirroring
    /// `lower_class_def`: ownership, an optional `specializes` relationship, and owned-member
    /// structure. Its body shares the `CalcDefBody` grammar (parameters, `return` results,
    /// feature members, invariants, expressions, documentation), the same shape `calc def`
    /// bodies use, so it is walked through the existing `lower_calc_def_body` rather than
    /// `lower_attribute_body`. `is_abstract`/`is_all`/`multiplicity`/`type_relationships` and the
    /// specific `KermlClassifierKeyword` spelling are not modeled as distinct facts here (see
    /// `DeclarationKind::KermlClassifier`).
    /// Lowers the KerML type-relationship clauses on a classifier or feature header --
    /// `unions`, `intersects`, `differences`, `disjoint from` (BNF `TypeRelationshipPart`).
    ///
    /// KerML models these as four distinct metaclasses, each a direct kind of `Relationship` and
    /// none of them a kind of `Specialization`: `Unioning` relates `typeUnioned` to `unioningType`,
    /// and `Intersecting`, `Differencing` and `Disjoining` follow the same source-to-target shape.
    /// They are therefore lowered as their own reference kinds rather than folded into the
    /// specialization edges, which would state a generalization the author did not write and would
    /// put union operands into `supertypes`.
    ///
    /// One reference per authored target, in authored order across clauses. The per-`(source,
    /// kind)` ordinal is what carries that order, and it is load-bearing for `differences`, whose
    /// first target is the type being reduced and whose remaining targets are the exclusions --
    /// including across a second `differences` clause, which continues the same list.
    ///
    /// Shared by the classifier and feature owners so the two cannot drift; the parser gives both
    /// the same `Vec<Node<KermlTypeRelationship>>`.
    /// Lowers the `FeatureRelationshipPart` list a KerML feature declaration carries.
    ///
    /// `unions`/`intersects`/`disjoint from`/`differences` reuse the existing type-relationship
    /// lowering. `chains` and `featured by` lower to their own canonical relationship kinds;
    /// `inverse of` remains explicitly unsupported because it needs a separate inverse-fact owner.
    pub(crate) fn lower_kerml_feature_relationship_parts(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        family: UnsupportedFamily,
        parts: &[Node<FeatureRelationshipPart>],
    ) -> Result<(), ConstructionError> {
        for part in parts {
            match &part.value {
                FeatureRelationshipPart::TypeRelationship(relationship) => {
                    self.lower_kerml_type_relationships(
                        document,
                        source,
                        std::slice::from_ref(relationship),
                    )?;
                }
                FeatureRelationshipPart::Chaining { target } => {
                    let span = self.documents[document.index()]
                        .parsed
                        .qualified_reference(*target)
                        .ok_or(ConstructionError::InvalidParserReference)?
                        .metadata
                        .span
                        .clone();
                    self.push_reference(PendingReference {
                        source,
                        kind: ReferenceKind::FeatureChaining,
                        document,
                        local: *target,
                        flags: RelationshipFlags::default(),
                        span,
                        import: None,
                    })?;
                }
                FeatureRelationshipPart::TypeFeaturing(featuring) => {
                    for target in featuring.value.targets.iter().copied() {
                        let span = self.documents[document.index()]
                            .parsed
                            .qualified_reference(target)
                            .ok_or(ConstructionError::InvalidParserReference)?
                            .metadata
                            .span
                            .clone();
                        self.push_reference(PendingReference {
                            source,
                            kind: ReferenceKind::TypeFeaturing,
                            document,
                            local: target,
                            flags: RelationshipFlags::default(),
                            span,
                            import: None,
                        })?;
                    }
                }
                FeatureRelationshipPart::Inverting { .. } => {
                    self.push_unsupported(document, family, part.span.clone());
                }
            }
        }
        Ok(())
    }

    pub(crate) fn lower_kerml_type_relationships(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        relationships: &[Node<KermlTypeRelationship>],
    ) -> Result<(), ConstructionError> {
        for relationship in relationships {
            let kind = match relationship.value.keyword {
                KermlTypeRelationshipKeyword::Unions => ReferenceKind::Unioning,
                KermlTypeRelationshipKeyword::Intersects => ReferenceKind::Intersecting,
                KermlTypeRelationshipKeyword::Differences => ReferenceKind::Differencing,
                KermlTypeRelationshipKeyword::DisjointFrom => ReferenceKind::Disjoining,
            };
            for target in relationship.value.targets.iter().copied() {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source,
                    kind,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        Ok(())
    }

    pub(crate) fn lower_kerml_classifier_decl(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<KermlClassifierDecl>,
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
            kerml_classifier_kind(&node.value.keyword),
            name,
            node.span.clone(),
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    all: node.value.is_all,
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
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_kerml_type_relationships(document, declaration, &node.value.type_relationships)?;
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a bare/bodied KerML feature member (`KermlFeature`, gap #14: previously an
    /// opaque `FeatureDecl { keyword, text }` raw-text fallback, now a fully typed shape),
    /// mirroring `lower_ref_decl`: ownership, membership, an optional `:` typing target, and
    /// `subsets`/`redefines` relationships. Its `= expr` value, when present, is classified and
    /// lowered through the same `classify_calc_expression`/`lower_calc_expression` pipeline
    /// `lower_parameter_declaration`/`lower_return_decl` use. Its body shares the `CalcDefBody`
    /// grammar, so owned members are walked through the existing `lower_calc_def_body`. See
    /// `DeclarationKind::KermlFeature` for the facts intentionally left unmodeled.
    ///
    /// This is now also the entry point for the two nodes upstream folded into it: the directed
    /// kinded parameter (`in expr p : Boolean = a;`, formerly `TypedParameterMember`), whose
    /// direction is the `BasicFeaturePrefix` slot read below, and the association end with an
    /// owned cross feature (`end happensDuring [1..*] subsets ... feature thatOccurrence : ...;`,
    /// formerly `KermlEndMember`), whose cross feature the grammar owns from the `EndFeaturePrefix`
    /// alternative -- so it is lowered here as an owned child through
    /// `lower_kerml_owned_cross_feature` rather than as this feature's owner.
    pub(crate) fn lower_kerml_feature_member(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        family: UnsupportedFamily,
        node: &Node<KermlFeature>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            kerml_feature_kind(node.value.kind.as_ref()),
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    all: node.value.is_all,
                    member: node.value.is_member,
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..kerml_feature_prefix_modifiers(&node.value.prefix)
                },
                direction: direction_node_fact(node.value.prefix.direction()),
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
        if let FeaturePrefixHead::End {
            cross: Some(cross), ..
        } = &node.value.prefix.head
        {
            self.lower_kerml_owned_cross_feature(document, declaration, cross)?;
        }
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship_impl(
                document,
                declaration,
                relationship,
                false,
                direction_node_fact(node.value.prefix.direction()),
            )?;
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
        self.lower_kerml_feature_relationship_parts(
            document,
            declaration,
            family,
            &node.value.relationship_parts,
        )?;
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(declaration, feature_value)?;
            let expression = feature_value.value.expression.clone();
            self.push_evaluation_fact(
                declaration,
                self.calc_evaluation_shape(document, &expression.value),
            );
            self.lower_calc_expression(document, declaration, family, &expression)?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML connector member (`KermlConnectorMember`), e.g. `connector fixWheel :
    /// BikeWheelFixed from [1] rollsOn to [1] holdsWheel;` (KerML Spec Annex A-3-3, gap: this
    /// construct was previously entirely unlowered -- see `DeclarationKind::KermlConnector`).
    /// Mirrors `lower_connection_def`: ownership, membership, an optional `:` typing target, and
    /// `from`/`to` ends resolved through `lower_kerml_connector_end` (the same
    /// `ReferenceKind::ConnectorEnd` reference kind `connection def`/`interface def` use). `is_all`
    /// and body content beyond the shared `lower_calc_def_body` walk are not modeled as distinct
    /// facts here.
    pub(crate) fn lower_kerml_connector_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlConnectorMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::KermlConnector,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    all: node.value.is_all,
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
        if let Some(type_name) = node.value.typing {
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
        if let Some(end) = &node.value.from {
            self.lower_kerml_connector_end(
                document,
                declaration,
                ReferenceKind::ConnectorEnd,
                end,
            )?;
        }
        if let Some(end) = &node.value.to {
            self.lower_kerml_connector_end(
                document,
                declaration,
                ReferenceKind::ConnectorEnd,
                end,
            )?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a KerML binding connector member (`KermlBindingMember`), e.g. `binding [1]
    /// startShot = [1] endShot;` (KerML Spec §8.2.4, gap: previously entirely unlowered -- see
    /// `DeclarationKind::KermlBinding`). Structurally the keyword-full sibling of
    /// `BindingConnectorUsage`/`Bind` -- mirrors `lower_binding_connector_usage`'s two-reference
    /// shape, resolving `left`/`right` as `ReferenceKind::BindSource`/`BindTarget` references
    /// through `lower_kerml_connector_end`'s target rather than a bare `QualifiedReferenceId`
    /// (each end additionally carries an optional multiplicity/`references` chain, both out of
    /// scope here, same as `KermlConnectorMember`'s ends).
    pub(crate) fn lower_kerml_binding_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlBindingMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::KermlBinding,
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
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::BindSource,
            &node.value.left,
        )?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::BindTarget,
            &node.value.right,
        )?;
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers one `KermlConnectorEnd` -- the connector-end shape shared by KerML connector,
    /// binding and succession members and by a `flow`/`allocation` usage's `from`/`to` clauses --
    /// as an authored reference of `kind`, mirroring `lower_binding_connector_operand` but
    /// operating on `KermlConnectorEnd.target` rather than a general expression. Allocation ends
    /// preserve their directional kind while dotted paths use the canonical type-directed member
    /// resolver; other KerML end kinds retain their established qualified lookup. The end's own
    /// `multiplicity` and `references` chain are not modeled as distinct facts here.
    pub(crate) fn lower_kerml_connector_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        kind: ReferenceKind,
        end: &Node<KermlConnectorEnd>,
    ) -> Result<(), ConstructionError> {
        if matches!(
            kind,
            ReferenceKind::AllocateSource | ReferenceKind::AllocateTarget
        ) {
            return self.push_satisfy_reference(document, owner, kind, end.value.target);
        }
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(end.value.target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: owner,
            kind,
            document,
            local: end.value.target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
        Ok(())
    }

    /// Lowers a KerML succession member (`KermlSuccessionMember`), e.g. `succession p_before_d
    /// first [1] paint then [1] dry;` (Kernel Semantic Library `ControlPerformances.kerml`, KerML
    /// Spec Annex A-3-6-Sequences). Structurally the keyword-full sibling of `KermlBindingMember`
    /// (same `KermlConnectorEnd`-shaped `first`/`then` operands, same absent `body`/`membership`
    /// shape difference from `KermlConnectorMember`) -- reuses `lower_kerml_connector_end`
    /// verbatim for both ends, tagged `ReferenceKind::Succession` (the same kind
    /// `lower_first_stmt`'s `FirstStmt` uses for its own `first`/`then` operands) rather than
    /// `BindSource`/`BindTarget`, since this is a succession relationship, not a binding. `is_all`
    /// (`all` sufficiency) and the succession's own `multiplicity` are not modeled as distinct
    /// facts here, mirroring `KermlConnectorMember`/`KermlBindingMember`'s own unmodeled
    /// end-level `multiplicity`/`references`.
    pub(crate) fn lower_kerml_succession_member(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<KermlSuccessionMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::Succession,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    all: node.value.is_all,
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
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::Succession,
            &node.value.first,
        )?;
        self.lower_kerml_connector_end(
            document,
            declaration,
            ReferenceKind::Succession,
            &node.value.then,
        )?;
        Ok(())
    }

    /// Lowers a KerML invariant member (`KermlInvariantMember`), e.g. `inv unitBound { -1.0 <=
    /// that & that <= 1.0 }` or the anonymous `inv { isClosed == true }` (KerML Spec §8.2.7, gap:
    /// previously entirely unlowered -- see `DeclarationKind::KermlInvariant`). Its body shares
    /// the `CalcDefBody` grammar (not `ConstraintDefBody`, unlike `AssertConstraintMember`), so it
    /// is walked through the existing `lower_calc_def_body` -- the same
    /// `classify_calc_expression`/`lower_calc_expression` pipeline already used for
    /// `KermlFeatureMember` values applies unchanged to its boolean expression(s). Its typed
    /// `is_negated` parser field is published as the canonical declaration polarity fact; the
    /// evaluator may still report an unrelated unsupported expression shape explicitly.
    pub(crate) fn lower_kerml_invariant_member(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<KermlInvariantMember>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::KermlInvariant,
            name,
            node.span.clone(),
            DeclarationFacts {
                negated: Some(node.value.is_negated),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the cross feature an `end`-prefixed KerML feature owns (`OwnedCrossFeature`, KerML
    /// BNF 595), e.g. the `happensDuring [1..*] subsets timeCoincidentOccurrences` in `end
    /// happensDuring [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence
    /// redefines longerOccurrence;` (KerML Spec Annex A-3, association-end form).
    ///
    /// Upstream folded `KermlEndMember` into `FeaturePrefix`'s own `OwnedCrossFeatureMember`, which
    /// inverts the ownership this used to publish: the cross feature is owned *by* the end-prefixed
    /// feature, as `FeaturePrefix` spells it, not the other way round. It keeps
    /// `DeclarationKind::KermlEnd`, and its `subsets` clause resolves through the same
    /// `SubsettingKind`-dispatched machinery every sibling clause uses. `OwnedCrossFeature` carries
    /// only the slots the corpus authors in cross position, so there is no typing, value or body to
    /// walk here.
    pub(crate) fn lower_kerml_owned_cross_feature(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<OwnedCrossFeature>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::KermlEnd,
            name,
            node.span.clone(),
            DeclarationFacts {
                modifiers: DeclarationModifiers {
                    ordered: node.value.multiplicity_modifiers.is_ordered(),
                    nonunique: !node.value.multiplicity_modifiers.is_unique(),
                    ..basic_feature_prefix_modifiers(&node.value.prefix)
                },
                direction: direction_node_fact(node.value.prefix.direction.as_ref()),
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
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        Ok(())
    }
}
