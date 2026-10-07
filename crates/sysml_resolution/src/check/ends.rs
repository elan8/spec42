//! KerML end-feature, Association, Connector and Flow validations, settled at the publication
//! barrier.
//!
//! Every rule reads owned facts and nothing else: the canonical owned end collection
//! ([`crate::lower::storage::SemanticModelStorage::owned_end_features`]), the derived KerML
//! `Type::endFeature` and `Feature::type` ([`crate::index::types::TypeIndex`]), the
//! `relatedFeature` member each end contributes ([`SemanticModel::end_related_feature`]) and the
//! authored multiplicity of an end feature, and the PayloadFeature count of a Flow.
//!
//! # Scope and unanswered cases
//!
//! The Association and Connector rules run for the KerML metaclass kinds whose owned end
//! collection is complete (KerML `assoc`, `assoc struct`, `connector`, `binding`). SysML connector
//! usages keep their bare ends as references only, so their end counts are not stated here.
//!
//! A rule whose operands depend on an unresolved or ambiguous specialization -- an end whose
//! typing did not settle, an association whose inherited ends are unknown, or a publication
//! without a single `Links::BinaryLink` -- is left unanswered rather than answered from a guess.
//!
//! `validateFeatureEndMultiplicity` reads the end feature's own authored multiplicity. An end
//! with none authored, or with an expression bound, is unanswered: the multiplicity it inherits
//! is not a derived fact yet. A positional connector end (`end [1] a`, `connect [1] a to b`)
//! authors its multiplicity in the cross-multiplicity position of the grammar, which is not the
//! end feature's own multiplicity, so only `end`-prefixed features are judged.

use std::collections::BTreeSet;

use crate::index::expressions::conforms;
use crate::lower::facts::OwnedEndFeature;
use crate::model::query::ends::EndRelatedFeature;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::resolve::end_features::cross_feature_of;
use crate::resolve::end_features::CrossFeature;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::Diagnostic;
use crate::DiagnosticCode;
use crate::DiagnosticSeverity;

impl<D> SemanticModel<D> {
    /// Appends every end-feature, Association and Connector diagnostic for `declared`.
    pub(crate) fn collect_end_feature_rules(
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

            // KerML 8.3.3.3.4 `validateFeatureEndMultiplicity`.
            if facts.modifiers.end
                && facts
                    .multiplicity
                    .as_ref()
                    .and_then(Self::literal_bounds)
                    .is_some_and(|bounds| bounds != (1, Some(1)))
            {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::EndFeatureMultiplicityNotOne,
                    DiagnosticSeverity::Warning,
                )?);
            }

            // KerML 8.3.4.9.2 `validateFlowPayloadFeature`.
            if facts.payload_feature_count.is_some_and(|count| count > 1) {
                diagnostics.push(self.declaration_diagnostic(
                    id,
                    DiagnosticCode::FlowMultiplePayloadFeatures,
                    DiagnosticSeverity::Warning,
                )?);
            }

            self.collect_cross_subsetting_rules(id, declaration.owner, diagnostics)?;
            self.collect_cross_feature_rules(id, diagnostics)?;

            let is_abstract = facts.modifiers.effectively_abstract(declaration.kind);
            // A recovered member means the parser could not read part of the body, so the ends
            // it authored are unknown rather than few.
            match declaration.kind {
                DeclarationKind::KermlAssociation | DeclarationKind::KermlAssociationStructure
                    if !self.contains_recovery(id)? =>
                {
                    self.collect_association_rules(id, is_abstract, diagnostics)?;
                }
                DeclarationKind::KermlConnector | DeclarationKind::KermlBinding
                    if !self.contains_recovery(id)? =>
                {
                    self.collect_connector_rules(id, declaration.kind, is_abstract, diagnostics)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// KerML 8.3.3.3.2 `validateCrossSubsettingCrossingFeature` and
    /// `validateCrossSubsettingCrossedFeature`, as the Pilot's `checkCrossSubsetting` evaluates
    /// them over each authored CrossSubsetting (`crosses`):
    ///
    /// - the crossing feature is an end feature of an owning Type with at least two end features;
    /// - when it is an end feature of an owning Type, the crossed feature is a feature chain of
    ///   exactly two chaining features and, for a Type with exactly two end features, the first
    ///   chaining feature is the other end.
    ///
    /// The chaining features are the canonical per-hop outcomes at the last segment of each
    /// `.`-separated chaining feature of the dotted reference; a chain
    /// hop or other end that did not settle to a declaration leaves the rule unanswered.
    fn collect_cross_subsetting_rules(
        &self,
        id: DeclarationId,
        owner: Option<DeclarationId>,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let crossings = self.authored_references(id, &[ReferenceKind::Crosses]);
        if crossings.is_empty() {
            return Ok(());
        }
        let is_end = self.is_end_feature(id);
        let ends = owner
            .map(|owner| self.types.end_features(owner).collect::<Vec<_>>())
            .unwrap_or_default();
        for (reference_id, reference) in crossings {
            if !is_end || owner.is_none() || ends.len() < 2 {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    DiagnosticCode::CrossSubsettingCrossingFeatureInvalid,
                    DiagnosticSeverity::Warning,
                    None,
                )?);
            }
            if !is_end || owner.is_none() {
                continue;
            }
            let chain = if reference.flags.dotted {
                let Some(path) = self.resolution.member_access_paths.get(&reference_id) else {
                    continue;
                };
                reference
                    .chaining_feature_ends
                    .iter()
                    .map(|end| {
                        path.get(*end as usize)
                            .copied()
                            .unwrap_or(ResolutionStatus::Unresolved)
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let violated = if chain.len() != 2 {
                true
            } else if ends.len() == 2 {
                let other = ends
                    .iter()
                    .find(|end| **end != OwnedEndFeature::Declared(id))
                    .copied();
                match (chain.first(), other) {
                    (
                        Some(ResolutionStatus::Resolved(first)),
                        Some(OwnedEndFeature::Declared(other)),
                    ) => *first != other,
                    _ => continue,
                }
            } else {
                false
            };
            if violated {
                diagnostics.push(self.reference_diagnostic(
                    reference,
                    DiagnosticCode::CrossSubsettingCrossedFeatureInvalid,
                    DiagnosticSeverity::Warning,
                    None,
                )?);
            }
        }
        Ok(())
    }

    /// KerML 8.3.3.3.4 `validateFeatureCrossFeatureType` and
    /// `validateFeatureCrossFeatureSpecialization` over the canonical `Feature::crossFeature`
    /// ([`cross_feature_of`]), as the Pilot's `KerMLValidator.checkFeature` evaluates them:
    ///
    /// - the cross feature has the same `type` set as the feature;
    /// - the cross feature specializes the cross feature of every Feature the feature redefines,
    ///   authored or implied (such as the positional end redefinitions).
    ///
    /// Reported at the owned CrossSubsetting's crossed feature when the feature authors one, and
    /// otherwise at its owned cross feature. A cross feature or redefined cross feature that did
    /// not settle, or an operand whose specialization hierarchy did not settle, leaves the rule
    /// unanswered.
    fn collect_cross_feature_rules(
        &self,
        id: DeclarationId,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let CrossFeature::Resolved(cross) =
            cross_feature_of(&self.storage, &self.resolution.member_access_paths, id)?
        else {
            return Ok(());
        };
        let crossing = self
            .authored_references(id, &[ReferenceKind::Crosses])
            .into_iter()
            .next()
            .map(|(_, reference)| reference);
        let report = |code: DiagnosticCode| match crossing {
            Some(reference) => {
                self.reference_diagnostic(reference, code, DiagnosticSeverity::Warning, None)
            }
            None => self.declaration_diagnostic(cross, code, DiagnosticSeverity::Warning),
        };

        let mut redefined = BTreeSet::new();
        self.collect_redefined_members(id, &mut redefined);
        let mut specializes_all = Some(true);
        for redefined in redefined {
            match cross_feature_of(
                &self.storage,
                &self.resolution.member_access_paths,
                redefined,
            )? {
                CrossFeature::Resolved(general) => {
                    if self.specialization_hierarchy_is_unsettled(cross) {
                        specializes_all = None;
                    } else if !conforms(&self.types, cross, general) {
                        specializes_all = specializes_all.map(|_| false);
                    }
                }
                CrossFeature::Absent => {}
                CrossFeature::Unpublished => specializes_all = None,
            }
        }
        if specializes_all == Some(false) {
            diagnostics.push(report(
                DiagnosticCode::CrossFeatureSpecializationIncompatible,
            )?);
        }

        if !self.specialization_hierarchy_is_unsettled(id)
            && !self.specialization_hierarchy_is_unsettled(cross)
            && self.types.feature_types(cross) != self.types.feature_types(id)
        {
            diagnostics.push(report(DiagnosticCode::CrossFeatureTypeMismatch)?);
        }
        Ok(())
    }

    fn collect_association_rules(
        &self,
        association: DeclarationId,
        is_abstract: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        let owned = self.storage.owned_end_features(association);

        // 8.3.4.4.2 `validateAssociationEndTypes`: every owned end feature has exactly one type.
        for record in owned {
            let OwnedEndFeature::Declared(end) = record.end else {
                continue;
            };
            if self
                .settled_end_type_count(association, end)
                .is_some_and(|count| count != 1)
            {
                diagnostics.push(self.declaration_diagnostic(
                    end,
                    DiagnosticCode::AssociationEndTypeNotOne,
                    DiagnosticSeverity::Warning,
                )?);
            }
        }

        // 8.3.4.4.2 `validateAssociationRelatedTypes`: `relatedType = associationEnd.type`, over
        // every end feature the association has, owned or inherited.
        if !is_abstract && !self.specialization_hierarchy_is_unsettled(association) {
            if let Some(related) = self.end_feature_type_count(association) {
                if related < 2 {
                    diagnostics.push(self.declaration_diagnostic(
                        association,
                        DiagnosticCode::AssociationRelatedTypesInsufficient,
                        DiagnosticSeverity::Warning,
                    )?);
                }
            }
        }

        // 8.3.4.4.2 `validateAssociationBinarySpecialization`. Owned ends suffice: they redefine
        // any inherited ones positionally. Reported at each end past the second, as the Pilot does.
        if owned.len() > 2 && self.types.specializes_binary_link(association) == Some(true) {
            for record in &owned[2..] {
                diagnostics.push(
                    self.end_diagnostic(record.end, DiagnosticCode::BinaryAssociationEndCount)?,
                );
            }
        }
        Ok(())
    }

    fn collect_connector_rules(
        &self,
        connector: DeclarationId,
        kind: DeclarationKind,
        is_abstract: bool,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<(), ResolutionError> {
        // `connectorEnd` is the connector's end features, owned or inherited. When the connector's
        // own specialization did not settle, the inherited ones are unknown.
        let settled = !self.specialization_hierarchy_is_unsettled(connector);
        let related = self
            .types
            .end_features(connector)
            .map(|end| self.end_related_feature(end))
            .filter(|related| related.is_present())
            .count();

        // 8.3.4.5.3 `validateConnectorRelatedFeatures`.
        if settled && !is_abstract && related < 2 {
            diagnostics.push(self.declaration_diagnostic(
                connector,
                DiagnosticCode::ConnectorRelatedFeaturesInsufficient,
                DiagnosticSeverity::Warning,
            )?);
        }

        // 8.3.4.5.2 `validateBindingConnectorIsBinary`.
        if settled && kind == DeclarationKind::KermlBinding && related != 2 {
            diagnostics.push(self.declaration_diagnostic(
                connector,
                DiagnosticCode::BindingConnectorNotBinary,
                DiagnosticSeverity::Warning,
            )?);
        }

        // 8.3.4.5.3 `validateConnectorBinarySpecialization`. As in the Pilot, the diagnostic is
        // placed on each owned end past the second when there are such ends, and on the connector
        // when the excess ends are all inherited.
        if settled
            && self.types.end_features(connector).count() > 2
            && self.types.specializes_binary_link(connector) == Some(true)
        {
            let owned = self.storage.owned_end_features(connector);
            if owned.len() <= 2 {
                diagnostics.push(self.declaration_diagnostic(
                    connector,
                    DiagnosticCode::BinaryConnectorEndCount,
                    DiagnosticSeverity::Warning,
                )?);
            } else {
                for record in &owned[2..] {
                    diagnostics.push(
                        self.end_diagnostic(record.end, DiagnosticCode::BinaryConnectorEndCount)?,
                    );
                }
            }
        }
        Ok(())
    }

    /// The size of `relatedType = associationEnd.type` for `association`, or `None` when an end's
    /// types are not settled.
    fn end_feature_type_count(&self, association: DeclarationId) -> Option<usize> {
        let mut count = 0usize;
        for end in self.types.end_features(association) {
            let feature = match end {
                OwnedEndFeature::Declared(feature) => feature,
                // A bare end's implicit end feature reference-subsets its target, so it has the
                // target's types.
                OwnedEndFeature::Bare(_) | OwnedEndFeature::Flow { .. } => {
                    match self.end_related_feature(end) {
                        EndRelatedFeature::Resolved(target) => target,
                        EndRelatedFeature::Unsettled | EndRelatedFeature::Absent => return None,
                    }
                }
            };
            count += self.settled_end_type_count(association, feature)?;
        }
        Some(count)
    }

    /// How many types one end feature of `association` has, or `None` when they are not settled.
    ///
    /// An end with no authored type gets its types from the library: the association's implied
    /// specialization of `Links::Link` / `Links::BinaryLink` makes the end redefine `participant`
    /// / `source` / `target`. When this publication has no resolved link anchor those implied
    /// types are missing rather than absent, so an empty type set is not an answer.
    fn settled_end_type_count(
        &self,
        association: DeclarationId,
        end: DeclarationId,
    ) -> Option<usize> {
        if self.specialization_hierarchy_is_unsettled(end) {
            return None;
        }
        let count = self.types.feature_types(end).len();
        if count == 0 && self.types.specializes_binary_link(association).is_none() {
            return None;
        }
        Some(count)
    }

    /// A diagnostic placed on one owned end: its declaration, or for a bare end the reference
    /// that represents it.
    fn end_diagnostic(
        &self,
        end: OwnedEndFeature,
        code: DiagnosticCode,
    ) -> Result<Diagnostic, ResolutionError> {
        match end {
            OwnedEndFeature::Declared(declaration) => {
                self.declaration_diagnostic(declaration, code, DiagnosticSeverity::Warning)
            }
            OwnedEndFeature::Bare(reference) | OwnedEndFeature::Flow { reference, .. } => self
                .reference_diagnostic(
                    self.storage
                        .references
                        .get(reference.index())
                        .ok_or(ResolutionError::InvalidStorage)?,
                    code,
                    DiagnosticSeverity::Warning,
                    None,
                ),
        }
    }
}
