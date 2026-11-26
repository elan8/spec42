//! KerML `Connector::relatedFeature`, derived once from the canonical owned end collection.

use crate::lower::facts::OwnedEndFeature;
use crate::model::resolver::SemanticModel;
use crate::model::{AuthoredReferenceId, DeclarationId, ReferenceKind};
use crate::resolve::end_features::end_reference;
pub(crate) use crate::resolve::end_features::EndRelatedFeature;

pub(crate) enum ConnectorEndSource {
    Bare(AuthoredReferenceId),
    Declared {
        declaration: DeclarationId,
        reference: Option<AuthoredReferenceId>,
    },
}

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

    /// Wired ends retain their authored reference order, followed by unwired declarations.
    pub(crate) fn connector_end_sources(
        &self,
        connector: DeclarationId,
    ) -> Vec<ConnectorEndSource> {
        let mut wired = Vec::new();
        // Bare SysML ends are published as references on the connector itself.
        for &id in self.outgoing_reference_ids(connector) {
            if self
                .storage
                .references
                .get(id.index())
                .is_some_and(|reference| {
                    matches!(
                        reference.kind,
                        ReferenceKind::ConnectorEnd | ReferenceKind::MemberAccessOperand
                    )
                })
            {
                wired.push((id, ConnectorEndSource::Bare(id)));
            }
        }
        let mut unwired = Vec::new();
        for record in self.storage.owned_end_features(connector) {
            let OwnedEndFeature::Declared(declaration) = record.end else {
                continue;
            };
            let reference = end_reference(&self.storage, record.end, |source| {
                self.outgoing_reference_ids(source).iter().copied()
            });
            let source = ConnectorEndSource::Declared {
                declaration,
                reference,
            };
            if let Some(id) = reference {
                wired.push((id, source));
            } else {
                unwired.push(source);
            }
        }
        wired.sort_by_key(|(id, _)| id.index());
        wired
            .into_iter()
            .map(|(_, source)| source)
            .chain(unwired)
            .collect()
    }
}
