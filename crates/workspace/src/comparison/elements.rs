//! Element diff from host semantic projections.

use std::collections::BTreeMap;

use crate::error::{WorkspaceError, WorkspaceResult};
use crate::snapshot::{HostSemanticModelNode, HostSemanticProjection};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct HostElementIdentity {
    /// Opaque, projection-owned identity used to pair live snapshots.
    #[serde(default)]
    pub semantic_id: String,
    /// Readable context retained for reports and schema-1 compatibility.
    pub uri: String,
    pub qualified_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostElementFieldChange {
    pub field: String,
    /// Canonical JSON representation of a typed projection fact. Existing
    /// string-valued schema-1 fields deserialize as JSON strings.
    pub previous: serde_json::Value,
    pub next: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostElementChange {
    pub identity: HostElementIdentity,
    pub fields: Vec<HostElementFieldChange>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostElementComparison {
    pub added: Vec<HostSemanticModelNode>,
    pub removed: Vec<HostSemanticModelNode>,
    pub changed: Vec<HostElementChange>,
}

pub(crate) fn compare_elements(
    previous: &HostSemanticProjection,
    next: &HostSemanticProjection,
) -> WorkspaceResult<HostElementComparison> {
    let previous_map = nodes_by_semantic_id(&previous.nodes)?;
    let next_map = nodes_by_semantic_id(&next.nodes)?;

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();
    for (semantic_id, node) in &next_map {
        match previous_map.get(semantic_id) {
            None => added.push(node.clone()),
            Some(previous_node) => {
                if let Some(field_changes) = diff_node_fields(previous_node, node) {
                    changed.push(HostElementChange {
                        identity: element_identity(node),
                        fields: field_changes,
                    });
                }
            }
        }
    }
    for (semantic_id, node) in &previous_map {
        if !next_map.contains_key(semantic_id) {
            removed.push(node.clone());
        }
    }

    added.sort_by(node_sort_key);
    removed.sort_by(node_sort_key);
    changed.sort_by(|left, right| left.identity.cmp(&right.identity));
    Ok(HostElementComparison {
        added,
        removed,
        changed,
    })
}

fn nodes_by_semantic_id(
    nodes: &[HostSemanticModelNode],
) -> WorkspaceResult<BTreeMap<String, HostSemanticModelNode>> {
    let mut result = BTreeMap::new();
    for node in nodes {
        if node.semantic_id.is_empty() {
            return Err(WorkspaceError::missing_comparison_identity("element"));
        }
        if result
            .insert(node.semantic_id.clone(), node.clone())
            .is_some()
        {
            return Err(WorkspaceError::duplicate_comparison_identity(
                "element",
                &node.semantic_id,
            ));
        }
    }
    Ok(result)
}

fn element_identity(node: &HostSemanticModelNode) -> HostElementIdentity {
    HostElementIdentity {
        semantic_id: node.semantic_id.clone(),
        uri: node.uri.clone(),
        qualified_name: node.qualified_name.clone(),
    }
}

fn node_sort_key(
    left: &HostSemanticModelNode,
    right: &HostSemanticModelNode,
) -> std::cmp::Ordering {
    left.semantic_id
        .cmp(&right.semantic_id)
        .then_with(|| left.uri.cmp(&right.uri))
        .then_with(|| left.qualified_name.cmp(&right.qualified_name))
}

fn diff_node_fields(
    previous: &HostSemanticModelNode,
    next: &HostSemanticModelNode,
) -> Option<Vec<HostElementFieldChange>> {
    let mut fields = Vec::new();
    push_change(&mut fields, "uri", &previous.uri, &next.uri);
    push_change(
        &mut fields,
        "qualified_name",
        &previous.qualified_name,
        &next.qualified_name,
    );
    push_change(
        &mut fields,
        "element_kind",
        &previous.element_kind,
        &next.element_kind,
    );
    push_change(&mut fields, "name", &previous.name, &next.name);
    push_change(&mut fields, "parent", &previous.parent, &next.parent);
    push_change(&mut fields, "range", &previous.range, &next.range);
    push_change(&mut fields, "facts", &previous.facts, &next.facts);
    if fields.is_empty() {
        None
    } else {
        fields.sort_by(|left, right| left.field.cmp(&right.field));
        Some(fields)
    }
}

fn push_change<T: serde::Serialize + PartialEq>(
    fields: &mut Vec<HostElementFieldChange>,
    field: &str,
    previous: &T,
    next: &T,
) {
    if previous != next {
        fields.push(HostElementFieldChange {
            field: field.to_string(),
            previous: serde_json::to_value(previous)
                .expect("host semantic element facts must serialize"),
            next: serde_json::to_value(next).expect("host semantic element facts must serialize"),
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::snapshot::HostElementFacts;
    use sysml_model::{ElementKind, TextPosition, TextRange};

    fn node() -> HostSemanticModelNode {
        HostSemanticModelNode {
            semantic_id: "s42e:item".to_string(),
            uri: "file:///demo.sysml".to_string(),
            qualified_name: "Demo::item".to_string(),
            name: "item".to_string(),
            element_kind: ElementKind::Part,
            range: TextRange::new(TextPosition::new(1, 4), TextPosition::new(1, 21)),
            parent: Some("Demo".to_string()),
            attributes: HashMap::from([("typeRef".to_string(), serde_json::json!("Thing"))]),
            facts: HostElementFacts {
                declared_name: Some("item".to_string()),
                effective_name: "item".to_string(),
                ..HostElementFacts::default()
            },
        }
    }

    #[test]
    fn compares_authored_and_effective_node_facts_as_typed_values() {
        let previous_node = node();
        let mut next_node = previous_node.clone();
        next_node.facts.declared_name = Some("authoredItem".to_string());
        next_node.facts.effective_name = "effectiveItem".to_string();
        let comparison = compare_elements(
            &HostSemanticProjection {
                nodes: vec![previous_node],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection {
                nodes: vec![next_node],
                ..HostSemanticProjection::default()
            },
        )
        .expect("compare");
        assert_eq!(comparison.changed.len(), 1);
        assert_eq!(comparison.changed[0].fields[0].field, "facts");
        assert!(comparison.changed[0].fields[0].previous.is_object());
    }

    #[test]
    fn presentation_attributes_do_not_create_semantic_element_changes() {
        let previous_node = node();
        let mut next_node = previous_node.clone();
        next_node
            .attributes
            .insert("displayOnly".to_string(), serde_json::json!("new label"));
        let comparison = compare_elements(
            &HostSemanticProjection {
                nodes: vec![previous_node],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection {
                nodes: vec![next_node],
                ..HostSemanticProjection::default()
            },
        )
        .expect("compare");
        assert_eq!(comparison, HostElementComparison::default());
    }

    #[test]
    fn reports_added_removed_and_deterministically_ordered_nodes() {
        let mut alpha = node();
        alpha.semantic_id = "s42e:alpha".to_string();
        let mut beta = node();
        beta.semantic_id = "s42e:beta".to_string();
        let empty = HostSemanticProjection::default();
        let with_nodes = HostSemanticProjection {
            nodes: vec![beta.clone(), alpha.clone()],
            ..HostSemanticProjection::default()
        };
        let added = compare_elements(&empty, &with_nodes).expect("compare");
        let removed = compare_elements(&with_nodes, &empty).expect("compare");
        assert_eq!(
            added
                .added
                .iter()
                .map(|node| node.semantic_id.as_str())
                .collect::<Vec<_>>(),
            vec!["s42e:alpha", "s42e:beta"]
        );
        assert_eq!(removed.removed, added.added);
    }

    #[test]
    fn rejects_missing_or_duplicate_element_identity() {
        let mut missing = node();
        missing.semantic_id.clear();
        let missing_error = compare_elements(
            &HostSemanticProjection {
                nodes: vec![missing],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection::default(),
        )
        .expect_err("missing identity must fail");
        assert_eq!(missing_error.code(), "missing_comparison_identity");

        let duplicate = node();
        let duplicate_error = compare_elements(
            &HostSemanticProjection {
                nodes: vec![duplicate.clone(), duplicate],
                ..HostSemanticProjection::default()
            },
            &HostSemanticProjection::default(),
        )
        .expect_err("duplicate identity must fail");
        assert_eq!(duplicate_error.code(), "duplicate_comparison_identity");
    }
}
