//! Structural feature-conformance decisions, settled at the publication barrier.
//!
//! The KerML feature rules and the SysML variation and connection-end rules, over the typed facts
//! the publication owns: authored modifiers, direction, uniqueness, effective positional ends,
//! featuring types, and settled reference outcomes.
//!
//! # What changed relative to the legacy check
//!
//! The legacy versions of these rules were written against a graph that could not represent an
//! effective end closure, an effective featuring type, or uniqueness for every feature kind, and
//! each rule fell silent wherever its prerequisite was missing. Where this publication can state
//! the fact, the rule now answers:
//!
//! - the end-count rules read the derived effective end count, so a declaration whose ends are
//!   entirely inherited is checked rather than skipped;
//! - the uniqueness rule reads uniqueness with its provenance, so KerML's default applies where no
//!   `nonunique` was authored rather than the question being abandoned.
//!
//! Where the fact genuinely is not available the rule still declines to answer, and says so in a
//! comment naming what is missing.

use crate::check::conformance;
use crate::index::expressions::conforms;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::ParameterDirection;
use crate::model::render as writer;
use crate::model::resolver::SemanticModel;
use crate::model::span::document_range;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::ReferenceKind;
use crate::resolve::is_feature_declaration;
use crate::resolve::is_usage_declaration;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::type_query::Conformance;
use crate::type_query::SpecializationScope;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticLocation;
use crate::DiagnosticOrigin;
use crate::DiagnosticSeverity;

/// Whether a declaration is a connection-like definition: one whose members include connector
/// ends.
pub(crate) fn is_connection_like(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::ConnectionDefinition
            | DeclarationKind::InterfaceDefinition
            | DeclarationKind::FlowDefinition
            | DeclarationKind::AllocationDefinition
    )
}

/// Whether a connection-like definition is binary, so its end count is fixed at two.
///
/// SysML flow and allocation definitions relate exactly two ends. A connection or interface
/// definition may be n-ary, so only the incomplete-pair rule constrains it.
pub(crate) fn requires_exactly_two_ends(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::FlowDefinition | DeclarationKind::AllocationDefinition
    )
}

/// Whether the declaration's KerML metaclass conforms to `Association` or `Connector` for
/// `validateRedefinitionEndConformance`.
///
/// This is a static metamodel question, not model typing: Pilot's validator guards the diagnostic
/// with `owningType instanceof Association || owningType instanceof Connector`. The represented
/// concrete subtypes are included here so every caller reads the same typed predicate.
pub(crate) const fn is_association_or_connector(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::KermlAssociation
            | DeclarationKind::KermlAssociationStructure
            | DeclarationKind::KermlConnector
            | DeclarationKind::KermlBinding
    )
}

/// Whether a redefining feature's direction is the redefined feature's or narrows it.
///
/// KerML lets a redefinition restrict direction, so this is a conformance test rather than
/// equality: `inout` permits values in both directions, and a redefinition narrowing it to `in` or
/// `out` removes one rather than contradicting it. The Kernel Semantic Library relies on exactly
/// that -- `out feature afterValues redefines values;` narrows an `inout` parameter -- and an
/// equality test, which is what the legacy check used, reported it as a mismatch.
pub(crate) fn direction_conforms(actual: ParameterDirection, expected: ParameterDirection) -> bool {
    match expected {
        // Already the widest, so any direction narrows it.
        ParameterDirection::InOut => true,
        ParameterDirection::In => actual == ParameterDirection::In,
        ParameterDirection::Out => actual == ParameterDirection::Out,
    }
}

/// Whether uniqueness was authored or is KerML's default.
///
/// The pinned parser cannot express `unique` at all -- it consumes the keyword without recording it
/// (`planning/UPSTREAM_PARSER_GAPS.md`, Gap 52) -- so an authored `unique` and an unwritten one are
/// indistinguishable. `nonunique` *is* a field, which is what makes the pair decidable: a feature
/// is non-unique exactly when `nonunique` was authored, and unique otherwise by KerML's default.
/// The provenance is published so the two are never confused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Uniqueness {
    /// `nonunique` was written.
    NonUniqueAuthored,
    /// Nothing was written, so KerML's default applies.
    UniqueByDefault,
}

impl<D> SemanticModel<D> {
    pub(crate) fn uniqueness(&self, declaration: DeclarationId) -> Uniqueness {
        match self
            .storage
            .declaration_facts(declaration)
            .is_some_and(|facts| facts.modifiers.nonunique)
        {
            true => Uniqueness::NonUniqueAuthored,
            false => Uniqueness::UniqueByDefault,
        }
    }

    /// Whether any parser-recovery span falls inside `declaration`'s own span.
    ///
    /// Recovery is a published fact, so a rule whose operands could have been swallowed by it can
    /// ask. This is barrier work over one document's recovery records, not a query-time scan.
    pub(crate) fn contains_recovery(
        &self,
        declaration: DeclarationId,
    ) -> Result<bool, ResolutionError> {
        let record = self
            .storage
            .declaration(declaration)
            .ok_or(ResolutionError::InvalidStorage)?;
        if self.storage.recovery.is_empty() {
            return Ok(false);
        }
        let span = document_range(&self.storage, record.document, &record.span)?;
        for recovery in self.storage.recovery.iter() {
            if recovery.document != record.document {
                continue;
            }
            let recovered = document_range(&self.storage, recovery.document, &recovery.span)?;
            if recovered.start >= span.start && recovered.end <= span.end {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Whether a declaration is a KerML `Feature` by metamodel category, independent of the
    /// membership that owns it (a body `metadata` usage is a Feature owned through an
    /// OwningMembership).
    pub(crate) fn is_feature(&self, declaration: DeclarationId) -> bool {
        self.storage
            .declaration(declaration)
            .is_some_and(|value| crate::resolve::is_feature_declaration(value.kind))
    }

    /// Whether a declaration is an end feature.
    ///
    /// Three authored forms mean it, and all three count: the `end` modifier prefix on a feature,
    /// an `end` member of a connection-like body (which carries a positional-end fact rather than
    /// the prefix), and a KerML `end` association member.
    pub(crate) fn is_end_feature(&self, declaration: DeclarationId) -> bool {
        let Some(facts) = self.storage.declaration_facts(declaration) else {
            return false;
        };
        facts.modifiers.end
            || facts.positional_end.is_some()
            || self
                .storage
                .declaration(declaration)
                .is_some_and(|value| value.kind == DeclarationKind::KermlEnd)
    }

    /// KerML 8.3.4.12.3 `validateMetadataFeatureBody`: every feature owned by a metadata feature
    /// redefines a feature of the metaclass the metadata feature is typed by. As in the Pilot's
    /// `checkMetadataBodyFeature`, a body member with no redefinition at all fails outright;
    /// when the metadata feature's own typing is settled, a redefinition whose target's owner the
    /// metaclass does not specialize fails too. The model-level-evaluability half of the clause
    /// is not decided here.
    fn collect_metadata_body_features(
        &self,
        id: DeclarationId,
        kind: DeclarationKind,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        if kind != DeclarationKind::MetadataUsage {
            return Ok(());
        }
        let metaclass = self
            .settled_targets(id, &[ReferenceKind::FeatureTyping])
            .first()
            .copied();
        for member in self.child_declarations(id).iter().copied() {
            let Some(member_kind) = self.kind_of(member) else {
                continue;
            };
            // `ownedFeature`: a Feature owned through a FeatureMembership. A multiplicity is an
            // ownedMember under an OwningMembership, not a body feature.
            if !is_feature_declaration(member_kind)
                || self.memberships.get(member).is_none_or(|membership| {
                    membership.kind != crate::model::MembershipKind::Feature
                })
            {
                continue;
            }
            let redefined = self.settled_targets(member, &[ReferenceKind::Redefinition]);
            let has_redefinition = !self
                .authored_references(member, &[ReferenceKind::Redefinition])
                .is_empty();
            // An authored redefinition that did not settle is reported by resolution; deciding
            // the owner half over an unsettled target would report the same defect twice.
            let redefines_metaclass_feature = has_redefinition
                && match metaclass {
                    None => true,
                    Some(metaclass) => {
                        redefined.is_empty()
                            || redefined.iter().any(|target| {
                                self.storage
                                    .declaration(*target)
                                    .and_then(|target| target.owner)
                                    .is_some_and(|owner| conforms(&self.types, metaclass, owner))
                            })
                    }
                };
            if !redefines_metaclass_feature {
                diagnostics.push(self.declaration_diagnostic(
                    member,
                    DiagnosticCode::MetadataBodyFeatureInvalid,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        Ok(())
    }

    /// SysML 8.3.12.5 `validatePortDefinitionOwnedUsagesNotComposite` and 8.3.12.6
    /// `validatePortUsageNestedUsagesNotComposite`: every non-port usage a port definition owns
    /// or a port usage nests is referential.
    fn collect_port_member_composition(
        &self,
        id: DeclarationId,
        kind: DeclarationKind,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let code = match kind {
            DeclarationKind::PortDefinition => DiagnosticCode::PortOwnedUsageComposite,
            DeclarationKind::PortUsage => DiagnosticCode::PortNestedUsageComposite,
            _ => return Ok(()),
        };
        for member in self.child_declarations(id).iter().copied() {
            let Some(member_kind) = self.kind_of(member) else {
                continue;
            };
            if member_kind == DeclarationKind::PortUsage || !self.usage_is_composite(member) {
                continue;
            }
            diagnostics.push(self.declaration_diagnostic(
                member,
                code.clone(),
                DiagnosticSeverity::Warning,
            )?);
        }
        Ok(())
    }

    /// SysML 8.3.18.5 `validateStateDefinitionParallelSubactions` and 8.3.18.6
    /// `validateStateUsageParallelSubactions`: a parallel state owns no transition or succession.
    /// Reported at each offending member, as the Pilot's `checkTransitionUsage` does.
    fn collect_parallel_state_subactions(
        &self,
        id: DeclarationId,
        kind: DeclarationKind,
        facts: &DeclarationFacts,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        if !matches!(
            kind,
            DeclarationKind::StateDefinition
                | DeclarationKind::StateUsage
                | DeclarationKind::ExhibitStateUsage
        ) || !facts.modifiers.parallel
        {
            return Ok(());
        }
        for member in self.child_declarations(id).iter().copied() {
            if matches!(
                self.kind_of(member),
                Some(DeclarationKind::Transition | DeclarationKind::Succession)
            ) {
                diagnostics.push(self.declaration_diagnostic(
                    member,
                    DiagnosticCode::ParallelStateSubstateTransition,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        Ok(())
    }

    /// `Feature::isComposite` of one lowered declaration.
    ///
    /// A SysML usage's value is the complement of the canonical effective
    /// [`crate::index::types::TypeIndex::usage_is_reference`] fact (authored `ref`, a
    /// never-composite metaclass, a direction, an end, or no expected featuring type all make it
    /// referential). KerML features are composite only when authored `composite`.
    pub(crate) fn usage_is_composite(&self, declaration: DeclarationId) -> bool {
        let Some(kind) = self.kind_of(declaration) else {
            return false;
        };
        if !is_usage_declaration(kind) {
            return self
                .storage
                .declaration_facts(declaration)
                .is_some_and(|facts| facts.modifiers.composite);
        }
        self.types
            .usage_is_reference(&self.storage, declaration)
            .is_some_and(|is_reference| !is_reference)
    }

    /// Appends every structural feature-conformance diagnostic authored in `document`.
    pub(crate) fn collect_structural_conformance(
        &self,
        document: DocumentIdx,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        self.collect_declaration_structure(declared, diagnostics)?;
        self.collect_structural_reference_rules(document, diagnostics)?;
        self.collect_specialization_specific_not_conjugated(document, diagnostics)?;
        self.collect_implied_structural_rules(document, diagnostics)?;
        self.collect_control_node_succession_rules(document, diagnostics)?;
        self.collect_end_feature_rules(declared, diagnostics)?;
        self.collect_feature_rules(document, declared, diagnostics)?;
        Ok(())
    }

    /// The rules whose whole operand set is one declaration's own facts.
    pub(crate) fn collect_declaration_structure(
        &self,
        declared: &[DeclarationId],
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for id in declared.iter().copied() {
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            let facts = self
                .storage
                .declaration_facts(id)
                .ok_or(ResolutionError::InvalidStorage)?;

            // KerML 8.3.3.3.4 `validateFeatureEndNotDerivedAbstractCompositeOrPortion`. KerML's
            // own `EndFeaturePrefix` spells only `const? end`, so the textual spelling that
            // reaches this rule is SysML's `DefaultReferenceUsage` (`end derived x : T;`),
            // lowered by `lower_end_decl` with the same modifier facts.
            if self.is_end_feature(id)
                && (facts.modifiers.derived
                    || facts.modifiers.effectively_abstract(declaration.kind)
                    || facts.modifiers.composite
                    || facts.modifiers.portion)
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::EndFeatureInvalidRestrictions,
                    DiagnosticSeverity::Warning,
                )?);
            }
            // KerML 8.3.3.3.4 `validateFeatureEndNoDirection`, reachable through the same SysML
            // spelling (`end in x : T;`).
            if self.is_end_feature(id) && facts.direction.is_some() {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::EndFeatureHasDirection,
                    DiagnosticSeverity::Warning,
                )?);
            }
            // KerML 8.3.2.4.2 `validateImportTopLevelVisibility`: an Import whose owning
            // namespace is a root namespace (no owner) must be private. The effective visibility
            // is the canonical membership fact, which defaults an Import to private.
            if declaration.kind == DeclarationKind::Import
                && declaration.owner.is_none()
                && self.memberships.get(id).is_some_and(|membership| {
                    membership.visibility != crate::resolve::names::EffectiveVisibility::Private
                })
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::TopLevelImportNotPrivate,
                    DiagnosticSeverity::Warning,
                )?);
            }
            self.collect_metadata_body_features(id, declaration.kind, diagnostics)?;
            self.collect_port_member_composition(id, declaration.kind, diagnostics)?;
            self.collect_parallel_state_subactions(id, declaration.kind, facts, diagnostics)?;
            self.collect_variation_owned_features(id, diagnostics)?;
            // SysML 8.3.6.5 `validateVariantMembershipOwningNamespace`: every definition and
            // usage body admits `variant` members, so the owner is checked here.
            if self.effective_membership_role(id) == Some(crate::MembershipRole::Variant)
                && !declaration
                    .owner
                    .is_some_and(|owner| self.is_variation(owner))
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::VariantOutsideVariation,
                    DiagnosticSeverity::Warning,
                )?);
            }

            self.collect_owner_rules(id, declaration.kind, declaration.owner, diagnostics)?;

            // An abstract declaration is deliberately incomplete, so its end count states nothing.
            if !is_connection_like(declaration.kind)
                || facts.modifiers.effectively_abstract(declaration.kind)
            {
                continue;
            }
            // The abstract guard above now fires for all four connection-like kinds:
            // `ConnectionDef`, `FlowDef`, `AllocationDef` and `InterfaceDef` each carry a
            // `definition_prefix`, and `lower_connection_def` and its siblings publish it as the
            // `is_abstract` modifier. An `abstract connection def C { end a; }` is deliberately
            // incomplete and is no longer reported.
            //
            // A recovered member inside the declaration means the parser could not read part of its
            // body, so the ends it authored are unknown rather than few. `connection def AB { end
            // [1] item a : A { @M; } ... }` recovers its first end and would otherwise be reported
            // as having one end when it authors two.
            if self.contains_recovery(id)? {
                continue;
            }
            let ends = self.types.effective_ends(id);
            // One end is an incomplete pair whatever the arity: a connector relates at least two
            // things. Zero ends is a declaration that states no connection structure at all, which
            // is a different thing from an incomplete one.
            if ends == 1 {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::IncompleteConnectionLikeEndPair,
                    DiagnosticSeverity::Warning,
                )?);
            }
            if requires_exactly_two_ends(declaration.kind) && ends > 2 {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::InvalidBinaryConnectionLikeEndCount,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }
        Ok(())
    }

    /// The rules whose operands are an authored reference and its settled target.
    /// KerML 8.3.3.1.8 `validateSpecializationSpecificNotConjugated`: the specific Type of every
    /// Specialization -- an owned specialization clause or a standalone KerML relationship
    /// declaration of any Specialization subkind -- is not conjugated, that is, owns no
    /// Conjugation. A conjugation or specialization whose endpoints do not settle answers
    /// nothing. Implied library specializations are not Specializations a source authors.
    pub(crate) fn collect_specialization_specific_not_conjugated(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let mut conjugated = std::collections::BTreeSet::new();
        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.kind != ReferenceKind::Conjugation {
                continue;
            }
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            if matches!(
                self.resolution.outcome(id),
                Some(ResolutionStatus::Resolved(_))
            ) {
                conjugated.insert(reference.source);
            }
        }
        conjugated.extend(
            self.resolution
                .authored_relationships
                .iter()
                .filter(|relationship| relationship.kind == ReferenceKind::Conjugation)
                .map(|relationship| relationship.source),
        );
        if conjugated.is_empty() {
            return Ok(());
        }
        for (index, reference) in self.storage.references.iter().enumerate() {
            if !reference.kind.is_specialization() || !conjugated.contains(&reference.source) {
                continue;
            }
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            if source.document != document
                || !matches!(
                    self.resolution.outcome(id),
                    Some(ResolutionStatus::Resolved(_))
                )
            {
                continue;
            }
            diagnostics.push(self.reference_diagnostic(
                reference,
                DiagnosticCode::SpecializationSpecificConjugated,
                DiagnosticSeverity::Error,
                None,
            )?);
        }
        for relationship in self.resolution.authored_relationships.iter() {
            if !relationship.kind.is_specialization() || !conjugated.contains(&relationship.source)
            {
                continue;
            }
            let Some(declaration) = self
                .storage
                .relationship_declarations
                .iter()
                .find(|declaration| declaration.source == relationship.declaration)
            else {
                return Err(ResolutionError::InvalidStorage);
            };
            let endpoint = self
                .storage
                .references
                .get(declaration.source.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            let owner = self
                .storage
                .declaration(endpoint.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if owner.document != document {
                continue;
            }
            diagnostics.push(Diagnostic {
                payload: None,
                message: DiagnosticCode::SpecializationSpecificConjugated
                    .describe()
                    .into(),
                code: DiagnosticCode::SpecializationSpecificConjugated,
                severity: DiagnosticSeverity::Error,
                origin: DiagnosticOrigin::Semantic,
                subject: self.symbol_id(relationship.source),
                location: DiagnosticLocation {
                    document: writer::document_identity(self, owner.document).into(),
                    range: document_range(&self.storage, owner.document, &declaration.span)?,
                },
                related: Box::default(),
            });
        }
        Ok(())
    }

    pub(crate) fn collect_structural_reference_rules(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for reference in self.storage.references.iter() {
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            let index = self.reference_index(reference)?;
            let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(index) else {
                continue;
            };
            // SysML 8.3.6.2/8.3.6.4 `validate{Definition,Usage}VariationSpecialization`. The
            // normative OCL reads `ownedSpecialization.specific`, which is always the variation
            // itself; the Pilot's validator checks `general`, the only reading that is not
            // vacuous, and so does this rule. Every authored Specialization subkind applies:
            // `edge_scopes` is the canonical table of them.
            if crate::index::types::edge_scopes(reference.kind).is_some()
                && self.is_variation(reference.source)
                && self.is_variation(target)
            {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    DiagnosticCode::VariationSpecializesVariation,
                    DiagnosticSeverity::Warning,
                    Some(target),
                )?);
            }
            // A dotted target (`subsets a.b`) subsets the implicit feature chain, whose featuring
            // types are its first chaining feature's; the settled target is only the chain's last
            // feature, so it is not the subsetted feature this rule compares.
            if matches!(
                reference.kind,
                ReferenceKind::Subsetting | ReferenceKind::References
            ) && !reference.flags.dotted
                && self.subsetting_target_not_accessible(reference.source, target)
            {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    DiagnosticCode::SubsettingTargetNotAccessible,
                    DiagnosticSeverity::Warning,
                    Some(target),
                )?);
            }
            match reference.kind {
                // SysML 8.4.4: a flow payload carries an occurrence, not a value. The occurrence
                // families are exactly those descending from `Occurrence` in the metamodel, so this
                // asks the same hierarchy the kind rules do rather than naming library types.
                ReferenceKind::FlowPayloadType => {
                    let Some((family, _)) = self.declaration_family(target) else {
                        continue;
                    };
                    if !conformance::descends_from_occurrence(family) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            DiagnosticCode::FlowPayloadTypeNotOccurrence,
                            DiagnosticSeverity::Error,
                            Some(target),
                        )?);
                    }
                }
                // SysML 8.4.3: a reference-form `variant` member must target a usage compatible
                // with the owning variation's kind. The reference source is the anonymous
                // ReferenceUsage owned by the VariantMembership, so applicability comes from its
                // canonical owner rather than from the proxy usage's own metaclass.
                ReferenceKind::Subsetting
                    if self.effective_membership_role(reference.source)
                        == Some(crate::MembershipRole::Variant) =>
                {
                    let Some(variation_source) = self
                        .storage
                        .declaration(reference.source)
                        .and_then(|source| source.owner)
                    else {
                        continue;
                    };
                    if !self.is_variation(variation_source) {
                        continue;
                    }
                    let (Some((variation, _)), Some((variant, _))) = (
                        self.declaration_family(variation_source),
                        self.declaration_family(target),
                    ) else {
                        continue;
                    };
                    if !conformance::families_are_comparable(variation, variant) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            DiagnosticCode::InvalidVariationMemberKind,
                            DiagnosticSeverity::Warning,
                            Some(target),
                        )?);
                    }
                }
                ReferenceKind::Redefinition => {
                    if self.subsetting_constant_mismatch(reference.source, target) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            DiagnosticCode::SubsettingConstantMismatch,
                            DiagnosticSeverity::Warning,
                            Some(target),
                        )?);
                    }
                    for code in self.redefinition_structure(reference.source, target) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            code,
                            DiagnosticSeverity::Warning,
                            Some(target),
                        )?);
                    }
                    if let Some(code) = self.featuring_structure(reference.source, target) {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            code,
                            DiagnosticSeverity::Error,
                            Some(target),
                        )?);
                    }
                }
                ReferenceKind::Subsetting | ReferenceKind::References | ReferenceKind::Crosses
                    if self.subsetting_constant_mismatch(reference.source, target) =>
                {
                    diagnostics.push(self.reference_diagnostic(
                        reference,
                        DiagnosticCode::SubsettingConstantMismatch,
                        DiagnosticSeverity::Warning,
                        Some(target),
                    )?);
                }
                // KerML 8.3.3.3.10: a non-unique feature cannot subset a unique one, since it would
                // admit repeated values the subsetted feature excludes.
                ReferenceKind::Subsetting
                    if self.uniqueness(reference.source) == Uniqueness::NonUniqueAuthored
                        && self.uniqueness(target) == Uniqueness::UniqueByDefault =>
                {
                    diagnostics.push(self.reference_diagnostic(
                        reference,
                        DiagnosticCode::SubsettingUniquenessMismatch,
                        DiagnosticSeverity::Warning,
                        Some(target),
                    )?);
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// The redefinition rules, over the redefinitions the resolver derived.
    ///
    /// Reported at the redefining declaration, since an implied relationship has no authored range.
    pub(crate) fn collect_implied_structural_rules(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for relationship in self.resolution.implied_relationships.iter() {
            if !matches!(
                relationship.kind,
                ReferenceKind::Redefinition | ReferenceKind::Subsetting
            ) {
                continue;
            }
            if self
                .storage
                .declaration(relationship.source)
                .ok_or(ResolutionError::InvalidStorage)?
                .document
                != document
            {
                continue;
            }
            if self.subsetting_constant_mismatch(relationship.source, relationship.target) {
                diagnostics.push(self.declaration_diagnostic(
                    relationship.source,
                    DiagnosticCode::SubsettingConstantMismatch,
                    DiagnosticSeverity::Warning,
                )?);
            }
            if relationship.kind != ReferenceKind::Redefinition {
                continue;
            }
            for code in self.redefinition_structure(relationship.source, relationship.target) {
                diagnostics.push(self.declaration_diagnostic(
                    relationship.source,
                    code,
                    DiagnosticSeverity::Warning,
                )?);
            }
            if let Some(code) = self.featuring_structure(relationship.source, relationship.target) {
                diagnostics.push(self.declaration_diagnostic(
                    relationship.source,
                    code,
                    DiagnosticSeverity::Error,
                )?);
            }
        }
        Ok(())
    }

    /// KerML 8.3.3.3.10 `validateSubsettingConstantConformance`: `subsettedFeature.isConstant
    /// and subsettingFeature.isVariable implies subsettingFeature.isConstant`. A Redefinition is
    /// a Subsetting, so it applies to both. `isVariable` is the canonical
    /// [`crate::index::types::TypeIndex::feature_is_variable`] fact; an unsettled answer is not
    /// a violation.
    pub(crate) fn subsetting_constant_mismatch(
        &self,
        subsetting: DeclarationId,
        subsetted: DeclarationId,
    ) -> bool {
        let is_constant = |declaration: DeclarationId| {
            self.storage
                .declaration_facts(declaration)
                .is_some_and(|facts| facts.modifiers.constant)
        };
        // An end feature is excluded: validateFeatureEndIsConstant already requires a variable
        // end to be constant, and the corpus treats a variable non-constant end as having no
        // spelling, yet `Usage::mayTimeVary` answers true for SysML connection ends (the
        // standard library's `end occurrence theCause :> causes :>> source` subsets a constant
        // feature). Reporting it here would restate that unresolved end-variability question.
        is_constant(subsetted)
            && !is_constant(subsetting)
            && !self.is_end_feature(subsetting)
            && matches!(
                self.types.feature_is_variable(&self.storage, subsetting),
                Some(crate::index::types::UsageTimeVariationOutcome::Resolved(
                    true
                ))
            )
    }

    /// Whether `feature`'s canonical effective featuring-type row is complete enough to compare.
    ///
    /// A variable feature is featured by its owner's snapshots and a feature nested in another is
    /// featured through that enclosing feature; neither effective featuring type is a published
    /// fact yet, and an unresolved `featured by` or `chains` target leaves the row incomplete, so
    /// such (possibly empty) rows are not settled.
    pub(crate) fn featuring_row_is_settled(&self, feature: DeclarationId) -> bool {
        !self.types.featuring_requires_snapshots(feature)
            && self
                .outgoing_reference_ids(feature)
                .iter()
                .all(|reference_id| {
                    !matches!(
                        self.storage.references[reference_id.index()].kind,
                        ReferenceKind::TypeFeaturing | ReferenceKind::FeatureChaining
                    ) || matches!(
                        self.resolution.outcome(*reference_id),
                        Some(ResolutionStatus::Resolved(_))
                    )
                })
            && !self
                .types
                .featuring_types(feature)
                .iter()
                .any(|(featuring_type, _)| self.is_feature(*featuring_type))
    }

    /// KerML 8.3.3.3.10 `validateSubsettingFeaturingTypes`: a subsetted feature with featuring
    /// types must be accessible by the subsetting feature (the Pilot's `FeatureUtil.canAccess`).
    ///
    /// Over settled type-valued featuring rows this is: some featuring type of the subsetting
    /// feature specializes every featuring type of the subsetted feature, or the subsetted feature
    /// is variable and that featuring type specializes its owning type. A feature-valued
    /// featuring type (the recursive `canAccess` case) is not a published row, so such a pair is
    /// left unanswered, as is any indeterminate conformance.
    pub(crate) fn subsetting_target_not_accessible(
        &self,
        subsetting: DeclarationId,
        subsetted: DeclarationId,
    ) -> bool {
        if !self.featuring_row_is_settled(subsetting) || !self.featuring_row_is_settled(subsetted) {
            return false;
        }
        let subsetted_featuring = self.types.featuring_types(subsetted);
        let subsetting_featuring = self.types.featuring_types(subsetting);
        if subsetted_featuring.is_empty() || subsetting_featuring.is_empty() {
            return false;
        }
        let subsetted_is_variable = matches!(
            self.types.feature_is_variable(&self.storage, subsetted),
            Some(crate::index::types::UsageTimeVariationOutcome::Resolved(
                true
            ))
        );
        let subsetted_owner = crate::index::types::TypeIndex::owning_type(&self.storage, subsetted);
        let mut indeterminate = false;
        for (featuring_type, _) in subsetting_featuring {
            let mut within = true;
            for (required, _) in subsetted_featuring {
                match self.conformance(
                    *featuring_type,
                    *required,
                    SpecializationScope::AnySpecialization,
                ) {
                    Conformance::Conforms => {}
                    Conformance::DoesNotConform => within = false,
                    Conformance::Indeterminate(_) => {
                        indeterminate = true;
                        within = false;
                    }
                }
            }
            if within {
                return false;
            }
            if subsetted_is_variable {
                if let Some(owner) = subsetted_owner {
                    match self.conformance(
                        *featuring_type,
                        owner,
                        SpecializationScope::AnySpecialization,
                    ) {
                        Conformance::Conforms => return false,
                        Conformance::DoesNotConform => {}
                        Conformance::Indeterminate(_) => indeterminate = true,
                    }
                }
            }
        }
        !indeterminate
    }

    /// The end and direction rules of one redefinition.
    pub(crate) fn redefinition_structure(
        &self,
        source: DeclarationId,
        target: DeclarationId,
    ) -> Vec<DiagnosticCode> {
        let mut codes = Vec::new();
        // KerML 8.3.3.3.8: when the redefining feature is owned by an Association or Connector,
        // redefining an end requires it to remain an end. Other Type owners are deliberately
        // exempt; StatePerformances in the normative library relies on the Behavior case.
        let source_owner_is_association_or_connector = self
            .storage
            .declaration(source)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| self.storage.declaration(owner))
            .is_some_and(|owner| is_association_or_connector(owner.kind));
        if source_owner_is_association_or_connector
            && self.is_end_feature(target)
            && !self.is_end_feature(source)
        {
            codes.push(DiagnosticCode::RedefinitionEndMismatch);
        }
        // Only authored directions are compared. Effective direction under port conjugation is not
        // a published fact, so a feature that inherits its direction rather than declaring one
        // leaves this question unanswered rather than answered from a missing operand.
        let direction = |declaration: DeclarationId| {
            self.storage
                .declaration_facts(declaration)
                .and_then(|facts| facts.direction)
        };
        if let (Some(actual), Some(expected)) = (direction(source), direction(target)) {
            if !direction_conforms(actual, expected) {
                codes.push(DiagnosticCode::RedefinitionDirectionMismatch);
            }
        }
        codes
    }

    /// KerML 8.3.3.3: a redefinition is introduced by the featuring type of the redefined feature,
    /// or by one that specializes it.
    ///
    /// A feature owned only by namespaces has no featuring type, so the rule has nothing to compare
    /// and cannot be violated. An indeterminate conformance answer -- a cyclic hierarchy -- also
    /// reports nothing: the obstacle is already its own diagnostic.
    ///
    /// Both sides must be featured by a *type*. A feature nested in another feature -- `snapshot s
    /// { var feature :>> x = 0; }` -- is featured by that enclosing feature, and its effective
    /// featuring type is the enclosing feature's type, which is a further derivation this
    /// publication does not own. Comparing the enclosing feature against a type instead would fail
    /// for every such declaration; the corpus authors 33 of them across the KerML time-varying
    /// fixtures alone, none of them a modelling error.
    pub(crate) fn featuring_structure(
        &self,
        source: DeclarationId,
        target: DeclarationId,
    ) -> Option<DiagnosticCode> {
        // KerML 8.3.3.3.8 `validateRedefinitionFeaturingTypes`: the redefining feature must have
        // a featuring type the redefined feature does not, so a redefinition between features
        // with the same effective featuring types (including none at all) redefines nothing it
        // could narrow. Compared as sets, as the Pilot's `checkRedefinition` does.
        let featuring = |feature: DeclarationId| {
            self.types
                .featuring_types(feature)
                .iter()
                .map(|(featuring_type, _)| *featuring_type)
                .collect::<std::collections::BTreeSet<_>>()
        };
        if self.featuring_row_is_settled(source)
            && self.featuring_row_is_settled(target)
            && featuring(source) == featuring(target)
        {
            return Some(DiagnosticCode::RedefinitionFeaturingTypeIncompatible);
        }
        let redefining = self.types.featuring_type(source)?;
        let redefined = self.types.featuring_type(target)?;
        if self.is_feature(redefining) || self.is_feature(redefined) {
            return None;
        }
        match self.conformance(
            redefining,
            redefined,
            SpecializationScope::AnySpecialization,
        ) {
            Conformance::DoesNotConform => {
                Some(DiagnosticCode::RedefinitionFeaturingTypeIncompatible)
            }
            Conformance::Conforms | Conformance::Indeterminate(_) => None,
        }
    }

    /// The effective `isVariation` of a declaration (authored, or implied by its metaclass).
    pub(crate) fn is_variation(&self, id: DeclarationId) -> bool {
        let (Some(declaration), Some(facts)) = (
            self.storage.declaration(id),
            self.storage.declaration_facts(id),
        ) else {
            return false;
        };
        facts.modifiers.effectively_variation(declaration.kind)
    }

    /// SysML 8.3.6.2/8.3.6.4 `validate{Definition,Usage}VariationOwnedFeatureMembership`:
    /// `isVariation implies ownedFeatureMembership->isEmpty()`. A variant is owned through a
    /// VariantMembership, which is not a FeatureMembership, so every other canonical
    /// Feature-membership member of a variation is reported at that member.
    fn collect_variation_owned_features(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        if !self.is_variation(id) {
            return Ok(());
        }
        for member in self.owned_feature_members(id) {
            if self.effective_membership_role(member) == Some(crate::MembershipRole::Variant) {
                continue;
            }
            diagnostics.push(self.declaration_diagnostic(
                member,
                DiagnosticCode::VariationOwnsFeatureMembership,
                DiagnosticSeverity::Warning,
            )?);
        }
        Ok(())
    }
}
