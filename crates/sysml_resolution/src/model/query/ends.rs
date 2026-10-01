//! KerML `Connector::relatedFeature`, derived once from the canonical owned end collection.

use crate::lower::facts::OwnedEndFeature;
use crate::model::resolver::SemanticModel;
use crate::model::AuthoredReferenceId;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionStatus;

/// What one connector end contributes to its connector's `relatedFeature` collection.
///
/// KerML derives `relatedFeature` from each end's owned ReferenceSubsetting
/// (`ConnectorUtil.getRelatedFeaturesOf` in the Pilot). An end with no ReferenceSubsetting
/// contributes nothing; an end whose ReferenceSubsetting is authored but did not settle to one
/// target does relate a feature, just not one this publication can name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EndRelatedFeature {
    /// The end's ReferenceSubsetting settled to this feature.
    Resolved(crate::model::DeclarationId),
    /// The end authors a ReferenceSubsetting whose target is unresolved or ambiguous.
    Unsettled,
    /// The end authors no ReferenceSubsetting.
    Absent,
}

impl EndRelatedFeature {
    /// Whether the end contributes a member to `relatedFeature`, known or not.
    pub(crate) fn is_present(self) -> bool {
        !matches!(self, Self::Absent)
    }
}

/// The reference kinds that carry a connector end's ReferenceSubsetting: the `references` /
/// `::>` relationship of a declared end, and the connector-end kinds a bare or named KerML
/// connector end is lowered as.
fn is_reference_subsetting(kind: ReferenceKind) -> bool {
    matches!(
        kind,
        ReferenceKind::References
            | ReferenceKind::ConnectorEnd
            | ReferenceKind::BindSource
            | ReferenceKind::BindTarget
            | ReferenceKind::Succession
            | ReferenceKind::FlowSource
            | ReferenceKind::FlowTarget
            | ReferenceKind::MemberAccessOperand
    )
}

impl<D> SemanticModel<D> {
    /// The `relatedFeature` member one owned connector end contributes.
    pub(crate) fn end_related_feature(&self, end: OwnedEndFeature) -> EndRelatedFeature {
        let reference = match end {
            OwnedEndFeature::Bare(reference) => Some(reference),
            OwnedEndFeature::Declared(declaration) => self
                .outgoing_reference_ids(declaration)
                .iter()
                .copied()
                .find(|reference| {
                    self.reference_kind(*reference)
                        .is_some_and(is_reference_subsetting)
                }),
        };
        match reference.map(|reference| self.resolution.outcome(reference)) {
            None => EndRelatedFeature::Absent,
            Some(Some(ResolutionStatus::Resolved(target))) => EndRelatedFeature::Resolved(target),
            Some(_) => EndRelatedFeature::Unsettled,
        }
    }

    fn reference_kind(&self, reference: AuthoredReferenceId) -> Option<ReferenceKind> {
        self.storage
            .references
            .get(reference.index())
            .map(|reference| reference.kind)
    }
}
