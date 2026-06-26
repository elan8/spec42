//! Relationship diff from host semantic projections.

use std::collections::BTreeSet;

use crate::snapshot::{HostSemanticModelRelationship, HostSemanticProjection};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct HostRelationshipIdentity {
    pub source: String,
    pub target: String,
    pub kind: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostRelationshipComparison {
    pub added: Vec<HostRelationshipIdentity>,
    pub removed: Vec<HostRelationshipIdentity>,
}

pub(crate) fn compare_relationships(
    previous: &HostSemanticProjection,
    next: &HostSemanticProjection,
) -> HostRelationshipComparison {
    let previous_set = relationship_set(&previous.relationships);
    let next_set = relationship_set(&next.relationships);

    let added: Vec<_> = next_set.difference(&previous_set).cloned().collect();
    let removed: Vec<_> = previous_set.difference(&next_set).cloned().collect();

    HostRelationshipComparison { added, removed }
}

fn relationship_set(
    relationships: &[HostSemanticModelRelationship],
) -> BTreeSet<HostRelationshipIdentity> {
    relationships
        .iter()
        .map(|relationship| HostRelationshipIdentity {
            source: relationship.source.clone(),
            target: relationship.target.clone(),
            kind: relationship.kind.as_str().to_string(),
        })
        .collect()
}
