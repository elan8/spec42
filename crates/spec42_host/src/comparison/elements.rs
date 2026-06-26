//! Element diff from host semantic projections.

use std::collections::BTreeMap;

use crate::snapshot::HostSemanticModelNode;
use crate::snapshot::HostSemanticProjection;
use sysml_model::TextRange;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct HostElementIdentity {
    pub uri: String,
    pub qualified_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostElementFieldChange {
    pub field: String,
    pub previous: String,
    pub next: String,
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
) -> HostElementComparison {
    let previous_map = nodes_by_identity(&previous.nodes);
    let next_map = nodes_by_identity(&next.nodes);

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (identity, node) in &next_map {
        match previous_map.get(identity) {
            None => added.push(node.clone()),
            Some(previous_node) => {
                if let Some(field_changes) = diff_node_fields(previous_node, node) {
                    changed.push(HostElementChange {
                        identity: identity.clone(),
                        fields: field_changes,
                    });
                }
            }
        }
    }

    for (identity, node) in &previous_map {
        if !next_map.contains_key(identity) {
            removed.push(node.clone());
        }
    }

    added.sort_by(|a, b| node_sort_key(a).cmp(&node_sort_key(b)));
    removed.sort_by(|a, b| node_sort_key(a).cmp(&node_sort_key(b)));
    changed.sort_by(|a, b| {
        a.identity
            .cmp(&b.identity)
            .then_with(|| a.fields.len().cmp(&b.fields.len()))
    });

    HostElementComparison {
        added,
        removed,
        changed,
    }
}

fn nodes_by_identity(nodes: &[HostSemanticModelNode]) -> BTreeMap<HostElementIdentity, HostSemanticModelNode> {
    let mut map = BTreeMap::new();
    for node in nodes {
        let identity = HostElementIdentity {
            uri: node.uri.clone(),
            qualified_name: node.qualified_name.clone(),
        };
        map.insert(identity, node.clone());
    }
    map
}

fn node_sort_key(node: &HostSemanticModelNode) -> (&str, &str) {
    (&node.uri, &node.qualified_name)
}

fn diff_node_fields(
    previous: &HostSemanticModelNode,
    next: &HostSemanticModelNode,
) -> Option<Vec<HostElementFieldChange>> {
    let mut fields = Vec::new();
    if previous.element_kind != next.element_kind {
        fields.push(HostElementFieldChange {
            field: "element_kind".to_string(),
            previous: previous.element_kind.as_str().to_string(),
            next: next.element_kind.as_str().to_string(),
        });
    }
    if previous.name != next.name {
        fields.push(HostElementFieldChange {
            field: "name".to_string(),
            previous: previous.name.clone(),
            next: next.name.clone(),
        });
    }
    if previous.parent != next.parent {
        fields.push(HostElementFieldChange {
            field: "parent".to_string(),
            previous: previous.parent.clone().unwrap_or_default(),
            next: next.parent.clone().unwrap_or_default(),
        });
    }
    if previous.range != next.range {
        fields.push(HostElementFieldChange {
            field: "range".to_string(),
            previous: format_range(previous.range),
            next: format_range(next.range),
        });
    }
    if fields.is_empty() {
        None
    } else {
        fields.sort_by(|a, b| a.field.cmp(&b.field));
        Some(fields)
    }
}

fn format_range(range: TextRange) -> String {
    format!(
        "{}:{}-{}:{}",
        range.start.line,
        range.start.character,
        range.end.line,
        range.end.character
    )
}
