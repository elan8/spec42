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

use crate::lower::facts::OwnedEndFeature;
use crate::model::query::ends::EndRelatedFeature;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::resolve::results::ResolutionError;
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
            if !self.specialization_hierarchy_is_unsettled(end)
                && self.types.feature_types(end).len() != 1
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
                OwnedEndFeature::Bare(_) => match self.end_related_feature(end) {
                    EndRelatedFeature::Resolved(target) => target,
                    EndRelatedFeature::Unsettled | EndRelatedFeature::Absent => return None,
                },
            };
            if self.specialization_hierarchy_is_unsettled(feature) {
                return None;
            }
            count += self.types.feature_types(feature).len();
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
            OwnedEndFeature::Bare(reference) => self.reference_diagnostic(
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
