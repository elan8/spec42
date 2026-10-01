//! KerML `Connector::relatedFeature`, derived once from the canonical owned end collection.

use crate::lower::facts::OwnedEndFeature;
use crate::model::resolver::SemanticModel;
pub(crate) use crate::resolve::end_features::EndRelatedFeature;

impl<D> SemanticModel<D> {
    /// The `relatedFeature` member one owned connector end contributes.
    ///
    /// Delegates to the single derivation in [`crate::resolve::end_features::end_related_feature`],
    /// supplying the published outgoing-reference index.
    pub(crate) fn end_related_feature(&self, end: OwnedEndFeature) -> EndRelatedFeature {
        crate::resolve::end_features::end_related_feature(
            &self.storage,
            &self.resolution,
            end,
            |declaration| self.outgoing_reference_ids(declaration).iter().copied(),
        )
    }
}
