//! Feature variability, portion and individuality rules.
//!
//! Each rule consumes one canonical owner's fact: `isVariable` from
//! [`crate::index::types::TypeIndex::feature_is_variable`], the owning type from
//! [`crate::index::types::TypeIndex::owning_type`], `Occurrences::Occurrence` conformance from the
//! source-role-verified library anchor, and a
//! Usage's types from the canonical effective-type collection. A prerequisite that is unresolved
//! or ambiguous (a missing library anchor, for instance) leaves the rule unanswered rather than
//! reported.

use crate::index::types::TypeIndex;
use crate::index::types::UsageTimeVariationOutcome;
use crate::lower::facts::FeatureValueKind;
use crate::model::element_kind::element_kind;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DocumentIdx;
use crate::resolve::results::ResolutionError;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticSeverity;
use sysml_contract::ElementKind;

/// Which side of the SysML occurrence family a metaclass is on, or `None` when it is neither an
/// `OccurrenceDefinition` nor an `OccurrenceUsage` (directly or through a subclass), read from the
/// canonical metaclass hierarchy [`ElementKind::conforms_to`].
pub(crate) fn occurrence_metaclass_role(kind: ElementKind) -> Option<OccurrenceRole> {
    if kind.conforms_to(ElementKind::OccurrenceDefinition) {
        Some(OccurrenceRole::Definition)
    } else if kind.conforms_to(ElementKind::OccurrenceUsage) {
        Some(OccurrenceRole::Usage)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OccurrenceRole {
    Definition,
    Usage,
}

impl<D> SemanticModel<D> {
    /// Appends every feature variability, portion and individuality diagnostic for
    /// the declarations and feature values authored in `document`.
    pub(crate) fn collect_feature_rules(
        &self,
        document: DocumentIdx,
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
            let is_variable = self.types.feature_is_variable(&self.storage, id);

            // KerML 8.3.3.3.4 `validateFeatureIsVariable`: `isVariable implies owningType <> null
            // and owningType.specializes('Occurrences::Occurrence')`. A Usage's `isVariable` is
            // `mayTimeVary`, whose derivation already requires exactly that owning type, so only
            // a KerML Feature's authored variability can violate it.
            if !crate::resolve::is_usage_declaration(declaration.kind)
                && matches!(is_variable, Some(UsageTimeVariationOutcome::Resolved(true)))
            {
                let owner_is_occurrence = match TypeIndex::owning_type(&self.storage, id) {
                    None => Some(false),
                    Some(owner) => match self.types.specializes_occurrence(owner) {
                        UsageTimeVariationOutcome::Resolved(value) => Some(value),
                        UsageTimeVariationOutcome::Unresolved
                        | UsageTimeVariationOutcome::Ambiguous => None,
                    },
                };
                if owner_is_occurrence == Some(false) {
                    diagnostics.push(self.declaration_diagnostic(
                        id,
                        DiagnosticCode::VariableFeatureOwnerNotOccurrence,
                        DiagnosticSeverity::Warning,
                    )?);
                }
            }

            // KerML 8.3.3.3.4 `validateFeaturePortionNotVariable`: `isPortion implies not
            // isVariable`. Only KerML spells `portion` together with `var`/`const`; a SysML
            // snapshot or timeslice is never time-varying by the `mayTimeVary` derivation.
            if facts.modifiers.portion
                && matches!(is_variable, Some(UsageTimeVariationOutcome::Resolved(true)))
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::PortionFeatureIsVariable,
                    DiagnosticSeverity::Warning,
                )?);
            }

            // SysML 8.3.9.4 `validateOccurrenceUsagePortionKind`: `portionKind <> null implies
            // owningType <> null and (owningType.oclIsKindOf(OccurrenceDefinition) or
            // owningType.oclIsKindOf(OccurrenceUsage))`. This is a metaclass test, not a
            // library-specialization one.
            if facts.portion_kind.is_some()
                && !TypeIndex::owning_type(&self.storage, id).is_some_and(|owner| {
                    self.storage.declaration(owner).is_some_and(|owner| {
                        occurrence_metaclass_role(element_kind(owner.kind)).is_some()
                    })
                })
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::PortionOwnerNotOccurrence,
                    DiagnosticSeverity::Warning,
                )?);
            }

            self.collect_individual_definitions(id, declaration.kind, facts, diagnostics)?;
            self.collect_owned_subsetting_counts(id, diagnostics)?;
        }
        self.collect_initial_feature_values(document, diagnostics)?;
        self.collect_feature_value_overriding(document, diagnostics)?;
        self.collect_feature_chaining_rules(document, diagnostics)
    }

    /// KerML 8.3.4.10.2 `validateFeatureValueOverriding`: every feature directly or indirectly
    /// redefined by a FeatureValue's `featureWithValue` has only default FeatureValues.
    ///
    /// The redefined features are the settled specialization closure restricted to Redefinition
    /// edges (authored and implied alike, as the Pilot's `getAllRedefinedFeaturesOf` is). The
    /// violation is reported at the overriding feature, with the overridden feature related.
    fn collect_feature_value_overriding(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        use crate::index::types::ScopeBits;
        let valuations = self
            .storage
            .feature_values
            .iter()
            .map(|value| (value.declaration, value))
            .collect::<std::collections::BTreeMap<_, _>>();
        for value in self.storage.feature_values.iter() {
            let feature = self
                .storage
                .declaration(value.declaration)
                .ok_or(ResolutionError::InvalidStorage)?;
            if feature.document != document {
                continue;
            }
            let mut overridden = self
                .types
                .specialization()
                .scoped_ancestors(value.declaration)
                .map(|(ancestor, _)| ancestor)
                .filter(|ancestor| {
                    self.types.specialization().reaches(
                        value.declaration,
                        *ancestor,
                        ScopeBits::Redefinition,
                    )
                })
                .filter(|ancestor| {
                    valuations
                        .get(ancestor)
                        .is_some_and(|redefined| !redefined.is_default)
                })
                .collect::<Vec<_>>();
            overridden.sort_by_key(|ancestor| self.declaration_range(*ancestor));
            let Some(first) = overridden.first() else {
                continue;
            };
            let mut diagnostic = self.declaration_diagnostic(
                value.declaration,
                DiagnosticCode::FeatureValueOverridesNonDefault,
                DiagnosticSeverity::Warning,
            )?;
            diagnostic.related = Box::from([
                self.related_declaration(*first, crate::check::conformance::RELATED_DECLARED)?
            ]);
            diagnostics.push(diagnostic);
        }
        Ok(())
    }

    /// KerML 8.3.3.3.4 `validateFeatureChainingFeatureNotOne` and
    /// `validateFeatureChainingFeaturesNotSelf`, over each authored `chains` reference.
    ///
    /// The chaining features of one `chains` clause are its `.`-separated hops: the parser's
    /// typed separators set the reference's `dotted` fact, so a reference without it names
    /// exactly one chaining feature, and a feature has exactly one chaining feature when that is
    /// its only `chains` reference. Each hop's settled target is the canonical per-hop
    /// member-access outcome; a hop that is not settled is not compared.
    fn collect_feature_chaining_rules(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        use crate::model::AuthoredReferenceId;
        use crate::model::ReferenceKind;
        use crate::resolve::results::ResolutionStatus;
        for (index, reference) in self.storage.references.iter().enumerate() {
            if reference.kind != ReferenceKind::FeatureChaining {
                continue;
            }
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            if source.document != document {
                continue;
            }
            // `ownedFeatureChaining` spans every `chains` clause of the feature, so exactly one
            // chaining feature means one clause whose target has no `.` hop.
            if !reference.flags.dotted
                && self
                    .storage
                    .references
                    .iter()
                    .filter(|other| {
                        other.source == reference.source
                            && other.kind == ReferenceKind::FeatureChaining
                    })
                    .count()
                    == 1
            {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    DiagnosticCode::FeatureChainingSingleOperand,
                    DiagnosticSeverity::Warning,
                    None,
                )?);
            }
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let includes_self = match self.resolution.member_access_paths.get(&id) {
                Some(path) => path.iter().any(|hop| {
                    matches!(hop, ResolutionStatus::Resolved(target) if *target == reference.source)
                }),
                None => matches!(
                    self.resolution.outcome(id),
                    Some(ResolutionStatus::Resolved(target)) if target == reference.source
                ),
            };
            if includes_self {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    DiagnosticCode::FeatureChainingIncludesSelf,
                    DiagnosticSeverity::Warning,
                    None,
                )?);
            }
            // `validateFeatureChainingFeatureConformance`: each hop after the first is featured
            // within the hop before it. Only a chain whose every hop settled is decided, and only
            // a decided `false` is reported.
            if let Some(path) = self.resolution.member_access_paths.get(&id) {
                let hops = path
                    .iter()
                    .map(|hop| match hop {
                        ResolutionStatus::Resolved(target) => Some(*target),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>();
                if let Some(hops) = hops {
                    if hops
                        .windows(2)
                        .any(|pair| self.is_featured_within(pair[1], pair[0]) == Some(false))
                    {
                        diagnostics.push(self.reference_diagnostic(
                            reference,
                            DiagnosticCode::FeatureChainingNotFeaturedWithinPrevious,
                            DiagnosticSeverity::Warning,
                            None,
                        )?);
                    }
                }
            }
        }
        Ok(())
    }

    /// KerML `Feature::isFeaturedWithin(type)` for a non-null `type`: every featuring type of
    /// `feature` is one `type` is compatible with, or `feature` is variable and `type`
    /// specializes its owning type. `None` when an input is not settled -- a featuring row that
    /// still needs the owner's `snapshots` feature, an undecidable compatibility, or an
    /// unresolved variability -- so the question stays unanswered rather than guessed.
    fn is_featured_within(&self, feature: DeclarationId, ty: DeclarationId) -> Option<bool> {
        use crate::index::connector_context::is_compatible;
        use crate::index::types::ScopeBits;
        use crate::index::types::UsageTimeVariationOutcome;
        if self.types.featuring_requires_snapshots(feature) {
            return None;
        }
        let mut featured = true;
        for (featuring, _) in self.types.featuring_types(feature) {
            if !is_compatible(&self.types, ty, *featuring)? {
                featured = false;
            }
        }
        if featured {
            return Some(true);
        }
        match self.types.feature_is_variable(&self.storage, feature) {
            Some(UsageTimeVariationOutcome::Resolved(false)) | None => Some(false),
            Some(UsageTimeVariationOutcome::Resolved(true)) => {
                let owner = self.storage.declaration(feature)?.owner?;
                Some(
                    ty == owner
                        || self.types.specialization().reaches(
                            ty,
                            owner,
                            ScopeBits::AnySpecialization,
                        ),
                )
            }
            Some(_) => None,
        }
    }

    /// Whether `declaration` is an `OccurrenceDefinition` with `isIndividual = true`.
    fn is_individual_occurrence_definition(&self, declaration: DeclarationId) -> bool {
        let (Some(record), Some(facts)) = (
            self.storage.declaration(declaration),
            self.storage.declaration_facts(declaration),
        ) else {
            return false;
        };
        occurrence_metaclass_role(element_kind(record.kind)) == Some(OccurrenceRole::Definition)
            && facts.modifiers.individual
    }

    /// KerML 8.3.3.3.4 `validateFeatureOwnedReferenceSubsetting` and
    /// `validateFeatureOwnedCrossSubsetting`: a Feature owns at most one ReferenceSubsetting and at
    /// most one CrossSubsetting.
    ///
    /// Every `references`/`::>` and `crosses`/`=>` clause owns exactly one such relationship (the
    /// grammar admits no target list in either), so the authored references of each kind are
    /// the owned relationships. As the Pilot does, each one after the first is reported.
    fn collect_owned_subsetting_counts(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        use crate::model::ReferenceKind;
        for (kind, code) in [
            (
                ReferenceKind::References,
                DiagnosticCode::FeatureMultipleReferenceSubsettings,
            ),
            (
                ReferenceKind::Crosses,
                DiagnosticCode::FeatureMultipleCrossSubsettings,
            ),
        ] {
            let mut owned = self.authored_references(id, &[kind]);
            owned.sort_by_key(|(_, reference)| reference.ordinal);
            for (_, reference) in owned.iter().skip(1) {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    code.clone(),
                    DiagnosticSeverity::Warning,
                    None,
                )?);
            }
        }
        Ok(())
    }

    /// SysML 8.3.9.4 `validateOccurrenceUsageIndividualDefinition`
    /// (`occurrenceDefinition->select(isIndividual)->size() <= 1`) and
    /// `validateOccurrenceUsageIndividualUsage` (`isIndividual implies individualDefinition <>
    /// null`), over the canonical effective types (`Feature::type`, which includes the types a
    /// usage inherits through subsetting and redefinition).
    ///
    /// Two individual definitions are a violation whatever else is unknown, since further types
    /// can only add to the count. The absence of one is only settled when the usage's whole
    /// specialization hierarchy is resolved and its implied `Occurrences::Occurrence` lineage is
    /// anchored, so a missing library or an unresolved typing leaves the rule unanswered.
    fn collect_individual_definitions(
        &self,
        id: DeclarationId,
        kind: crate::model::DeclarationKind,
        facts: &crate::lower::facts::DeclarationFacts,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        if occurrence_metaclass_role(element_kind(kind)) != Some(OccurrenceRole::Usage) {
            return Ok(());
        }
        // One effective type may be both direct and inherited; it is one occurrenceDefinition.
        let individual_definitions = self
            .types
            .effective_types(id)
            .iter()
            .map(|(definition, _)| *definition)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .filter(|definition| self.is_individual_occurrence_definition(*definition))
            .count();
        if individual_definitions > 1 {
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::OccurrenceMultipleIndividualDefinitions,
                DiagnosticSeverity::Warning,
            )?);
        } else if facts.modifiers.individual
            && individual_definitions == 0
            && matches!(
                self.types.specializes_occurrence(id),
                UsageTimeVariationOutcome::Resolved(true)
            )
            && !self.specialization_hierarchy_is_unsettled(id)
        {
            diagnostics.push(self.declaration_diagnostic(
                id,
                DiagnosticCode::IndividualUsageWithoutIndividualDefinition,
                DiagnosticSeverity::Warning,
            )?);
        }
        Ok(())
    }

    /// KerML 8.3.4.10.2 `validateFeatureValueIsInitial`: `isInitial implies
    /// featureWithValue.isVariable`, reported at the feature that owns the value. `:=` is the
    /// initial form whether or not it is also a default.
    fn collect_initial_feature_values(
        &self,
        document: DocumentIdx,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        for value in self.storage.feature_values.iter() {
            if value.kind != FeatureValueKind::Assign {
                continue;
            }
            let feature = self
                .storage
                .declaration(value.declaration)
                .ok_or(ResolutionError::InvalidStorage)?;
            if feature.document != document {
                continue;
            }
            if !matches!(
                self.types
                    .feature_is_variable(&self.storage, value.declaration),
                Some(UsageTimeVariationOutcome::Resolved(false))
            ) {
                continue;
            }
            diagnostics.push(self.declaration_diagnostic(
                value.declaration,
                DiagnosticCode::InitialValueFeatureNotVariable,
                DiagnosticSeverity::Warning,
            )?);
        }
        Ok(())
    }
}
