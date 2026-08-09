//! Relationship diff from host semantic projections.

use std::collections::BTreeMap;

use crate::error::{WorkspaceError, WorkspaceResult};
use crate::snapshot::{HostSemanticModelRelationship, HostSemanticProjection};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct HostRelationshipIdentity {
    /// Opaque, projection-owned relationship identity. `source`, `target`, and
    /// `kind` remain readable context for reports and v1 compatibility.
    #[serde(default)]
    pub semantic_id: String,
    pub source: String,
    pub target: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostRelationshipFieldChange {
    pub field: String,
    pub previous: serde_json::Value,
    pub next: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostRelationshipChange {
    pub identity: HostRelationshipIdentity,
    pub fields: Vec<HostRelationshipFieldChange>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostRelationshipComparison {
    pub added: Vec<HostRelationshipIdentity>,
    pub removed: Vec<HostRelationshipIdentity>,
    #[serde(default)]
    pub changed: Vec<HostRelationshipChange>,
}

pub(crate) fn compare_relationships(
    previous: &HostSemanticProjection,
    next: &HostSemanticProjection,
) -> WorkspaceResult<HostRelationshipComparison> {
    let previous_map = relationships_by_semantic_id(&previous.relationships)?;
    let next_map = relationships_by_semantic_id(&next.relationships)?;

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();
    for (key, relationship) in &next_map {
        match previous_map.get(key) {
            None => added.push(relationship_identity(relationship)),
            Some(previous_relationship) => {
                if let Some(fields) = diff_relationship_fields(previous_relationship, relationship)
                {
                    changed.push(HostRelationshipChange {
                        identity: relationship_identity(relationship),
                        fields,
                    });
                }
            }
        }
    }
    for (key, relationship) in &previous_map {
        if !next_map.contains_key(key) {
            removed.push(relationship_identity(relationship));
        }
    }

    Ok(HostRelationshipComparison {
        added,
        removed,
        changed,
    })
}

fn relationships_by_semantic_id(
    relationships: &[HostSemanticModelRelationship],
) -> WorkspaceResult<BTreeMap<String, HostSemanticModelRelationship>> {
    let mut result = BTreeMap::new();
    for relationship in relationships {
        if relationship.semantic_id.is_empty() {
            return Err(WorkspaceError::missing_comparison_identity("relationship"));
        }
        if result
            .insert(relationship.semantic_id.clone(), relationship.clone())
            .is_some()
        {
            return Err(WorkspaceError::duplicate_comparison_identity(
                "relationship",
                &relationship.semantic_id,
            ));
        }
    }
    Ok(result)
}

fn relationship_identity(relationship: &HostSemanticModelRelationship) -> HostRelationshipIdentity {
    HostRelationshipIdentity {
        semantic_id: relationship.semantic_id.clone(),
        source: relationship.source.clone(),
        target: relationship.target.clone(),
        kind: relationship.kind.as_str().to_string(),
    }
}

fn diff_relationship_fields(
    previous: &HostSemanticModelRelationship,
    next: &HostSemanticModelRelationship,
) -> Option<Vec<HostRelationshipFieldChange>> {
    let mut fields = Vec::new();
    push_change(&mut fields, "source", &previous.source, &next.source);
    push_change(&mut fields, "target", &previous.target, &next.target);
    push_change(&mut fields, "kind", &previous.kind, &next.kind);
    push_change(
        &mut fields,
        "source_id",
        &previous.source_id,
        &next.source_id,
    );
    push_change(
        &mut fields,
        "target_id",
        &previous.target_id,
        &next.target_id,
    );
    push_change(&mut fields, "owner_id", &previous.owner_id, &next.owner_id);
    push_change(
        &mut fields,
        "related_element_ids",
        &previous.related_element_ids,
        &next.related_element_ids,
    );
    push_change(&mut fields, "range", &previous.range, &next.range);
    push_change(
        &mut fields,
        "is_implied",
        &previous.is_implied,
        &next.is_implied,
    );
    push_change(
        &mut fields,
        "metaclass",
        &previous.metaclass,
        &next.metaclass,
    );
    push_change(
        &mut fields,
        "membership_kind",
        &previous.membership_kind,
        &next.membership_kind,
    );
    push_change(
        &mut fields,
        "visibility",
        &previous.visibility,
        &next.visibility,
    );
    push_change(&mut fields, "connect", &previous.connect, &next.connect);
    push_change(&mut fields, "flow", &previous.flow, &next.flow);
    if fields.is_empty() {
        None
    } else {
        fields.sort_by(|a, b| a.field.cmp(&b.field));
        Some(fields)
    }
}

fn push_change<T: serde::Serialize + PartialEq>(
    fields: &mut Vec<HostRelationshipFieldChange>,
    field: &str,
    previous: &T,
    next: &T,
) {
    if previous != next {
        fields.push(HostRelationshipFieldChange {
            field: field.to_string(),
            previous: serde_json::to_value(previous)
                .expect("host semantic relationship facts must serialize"),
            next: serde_json::to_value(next)
                .expect("host semantic relationship facts must serialize"),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::{HostRelationshipMetaclass, HostSemanticProjection};
    use sysml_model::RelationshipKind;

    fn relationship() -> HostSemanticModelRelationship {
        HostSemanticModelRelationship {
            semantic_id: "s42r:relationship".to_string(),
            source_id: "s42e:source".to_string(),
            target_id: "s42e:target".to_string(),
            owner_id: Some("s42e:source".to_string()),
            related_element_ids: vec!["s42e:source".to_string(), "s42e:target".to_string()],
            range: None,
            is_implied: false,
            metaclass: HostRelationshipMetaclass::FeatureTyping,
            membership_kind: None,
            visibility: None,
            source: "Demo::item".to_string(),
            target: "Demo::Thing".to_string(),
            kind: RelationshipKind::Typing,
            connect: None,
            flow: None,
        }
    }

    #[test]
    fn compares_relationship_provenance_and_typed_contract_facts() {
        let previous = relationship();
        let mut next = previous.clone();
        next.is_implied = true;
        next.visibility = Some("public".to_string());

        let comparison = compare_relationships(
            &HostSemanticProjection {
                relationships: vec![previous],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection {
                relationships: vec![next],
                ..HostSemanticProjection::default()
            },
        )
        .expect("compare");

        assert!(comparison.added.is_empty());
        assert!(comparison.removed.is_empty());
        assert_eq!(comparison.changed.len(), 1);
        assert_eq!(
            comparison.changed[0]
                .fields
                .iter()
                .map(|change| change.field.as_str())
                .collect::<Vec<_>>(),
            vec!["is_implied", "visibility"]
        );
    }

    #[test]
    fn stable_relationship_identity_pairs_endpoint_changes() {
        let previous = relationship();
        let mut next = previous.clone();
        next.source = "Demo::renamedItem".to_string();
        next.target = "Demo::Other".to_string();
        next.kind = RelationshipKind::Specializes;

        let comparison = compare_relationships(
            &HostSemanticProjection {
                relationships: vec![previous],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection {
                relationships: vec![next],
                ..HostSemanticProjection::default()
            },
        )
        .expect("compare");

        assert!(comparison.added.is_empty());
        assert!(comparison.removed.is_empty());
        assert_eq!(
            comparison.changed[0]
                .fields
                .iter()
                .map(|change| change.field.as_str())
                .collect::<Vec<_>>(),
            vec!["kind", "source", "target"]
        );
    }

    #[test]
    fn reports_added_and_removed_relationships() {
        let relationship = relationship();
        let empty = HostSemanticProjection::default();
        let with_relationship = HostSemanticProjection {
            relationships: vec![relationship.clone()],
            ..HostSemanticProjection::default()
        };

        let added = compare_relationships(&empty, &with_relationship).expect("compare");
        let removed = compare_relationships(&with_relationship, &empty).expect("compare");
        assert_eq!(added.added, vec![relationship_identity(&relationship)]);
        assert_eq!(removed.removed, vec![relationship_identity(&relationship)]);
    }

    #[test]
    fn rejects_duplicate_relationship_identity() {
        let duplicate = relationship();
        let error = compare_relationships(
            &HostSemanticProjection {
                relationships: vec![duplicate.clone(), duplicate],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection::default(),
        )
        .expect_err("duplicate relationship must not be overwritten");
        assert_eq!(error.code(), "duplicate_comparison_identity");
    }

    #[test]
    fn rejects_missing_relationship_identity() {
        let mut missing = relationship();
        missing.semantic_id.clear();
        let error = compare_relationships(
            &HostSemanticProjection {
                relationships: vec![missing],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection::default(),
        )
        .expect_err("missing relationship identity must fail");
        assert_eq!(error.code(), "missing_comparison_identity");
    }

    #[test]
    fn relationship_order_is_independent_of_projection_order() {
        let mut first = relationship();
        first.semantic_id = "s42r:first".to_string();
        let mut second = relationship();
        second.semantic_id = "s42r:second".to_string();
        second.target = "Demo::Other".to_string();
        let comparison = compare_relationships(
            &HostSemanticProjection {
                relationships: vec![second.clone(), first.clone()],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection {
                relationships: vec![first, second],
                ..HostSemanticProjection::default()
            },
        )
        .expect("compare");

        assert_eq!(comparison, HostRelationshipComparison::default());
    }
}
