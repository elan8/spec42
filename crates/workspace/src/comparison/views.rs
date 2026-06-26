//! View catalog and supported-view payload identity diff.

use std::collections::BTreeMap;

use sysml_model::{SysmlVisualizationResultDto, SysmlVisualizationViewCandidateDto};
use sha2::{Digest, Sha256};

use crate::error::HostResult;
use crate::snapshot::HostWorkspaceSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostViewCatalogEntry {
    pub id: String,
    pub name: String,
    pub supported: bool,
    pub renderer_view: Option<String>,
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostViewCatalogFieldChange {
    pub field: String,
    pub previous: String,
    pub next: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostViewCatalogChange {
    pub view_id: String,
    pub fields: Vec<HostViewCatalogFieldChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostViewPayloadChange {
    pub view_id: String,
    pub previous_hash: String,
    pub next_hash: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HostViewComparison {
    pub catalog_added: Vec<HostViewCatalogEntry>,
    pub catalog_removed: Vec<HostViewCatalogEntry>,
    pub catalog_changed: Vec<HostViewCatalogChange>,
    pub changed_view_payloads: Vec<HostViewPayloadChange>,
}

pub(crate) fn compare_views(
    previous: &HostWorkspaceSnapshot,
    next: &HostWorkspaceSnapshot,
) -> HostResult<HostViewComparison> {
    let previous_catalog = catalog_map(previous);
    let next_catalog = catalog_map(next);

    let mut catalog_added = Vec::new();
    let mut catalog_removed = Vec::new();
    let mut catalog_changed = Vec::new();

    for (id, entry) in &next_catalog {
        match previous_catalog.get(id) {
            None => catalog_added.push(entry.clone()),
            Some(previous_entry) => {
                if let Some(fields) = diff_catalog_entry(previous_entry, entry) {
                    catalog_changed.push(HostViewCatalogChange {
                        view_id: id.clone(),
                        fields,
                    });
                }
            }
        }
    }

    for (id, entry) in &previous_catalog {
        if !next_catalog.contains_key(id) {
            catalog_removed.push(entry.clone());
        }
    }

    catalog_added.sort_by(|a, b| a.id.cmp(&b.id));
    catalog_removed.sort_by(|a, b| a.id.cmp(&b.id));
    catalog_changed.sort_by(|a, b| a.view_id.cmp(&b.view_id));

    let mut changed_view_payloads = Vec::new();
    for (id, next_entry) in &next_catalog {
        let Some(previous_entry) = previous_catalog.get(id) else {
            continue;
        };
        if !previous_entry.supported || !next_entry.supported {
            continue;
        }
        let Some(renderer_view) = next_entry.renderer_view.as_deref() else {
            continue;
        };
        let previous_payload = previous.prepare_view(renderer_view, Some(&next_entry.name))?;
        let next_payload = next.prepare_view(renderer_view, Some(&next_entry.name))?;
        let previous_hash = view_payload_fingerprint(&previous_payload);
        let next_hash = view_payload_fingerprint(&next_payload);
        if previous_hash != next_hash {
            changed_view_payloads.push(HostViewPayloadChange {
                view_id: id.clone(),
                previous_hash,
                next_hash,
            });
        }
    }

    changed_view_payloads.sort_by(|a, b| a.view_id.cmp(&b.view_id));

    Ok(HostViewComparison {
        catalog_added,
        catalog_removed,
        catalog_changed,
        changed_view_payloads,
    })
}

fn catalog_map(snapshot: &HostWorkspaceSnapshot) -> BTreeMap<String, HostViewCatalogEntry> {
    snapshot
        .view_catalog()
        .view_index
        .view_candidates
        .iter()
        .map(|candidate| (candidate.id.clone(), catalog_entry(candidate)))
        .collect()
}

fn catalog_entry(candidate: &SysmlVisualizationViewCandidateDto) -> HostViewCatalogEntry {
    HostViewCatalogEntry {
        id: candidate.id.clone(),
        name: candidate.name.clone(),
        supported: candidate.supported,
        renderer_view: candidate.renderer_view.clone(),
        view_type: candidate.view_type.clone(),
    }
}

fn diff_catalog_entry(
    previous: &HostViewCatalogEntry,
    next: &HostViewCatalogEntry,
) -> Option<Vec<HostViewCatalogFieldChange>> {
    let mut fields = Vec::new();
    if previous.name != next.name {
        fields.push(HostViewCatalogFieldChange {
            field: "name".to_string(),
            previous: previous.name.clone(),
            next: next.name.clone(),
        });
    }
    if previous.supported != next.supported {
        fields.push(HostViewCatalogFieldChange {
            field: "supported".to_string(),
            previous: previous.supported.to_string(),
            next: next.supported.to_string(),
        });
    }
    if previous.renderer_view != next.renderer_view {
        fields.push(HostViewCatalogFieldChange {
            field: "renderer_view".to_string(),
            previous: previous.renderer_view.clone().unwrap_or_default(),
            next: next.renderer_view.clone().unwrap_or_default(),
        });
    }
    if previous.view_type != next.view_type {
        fields.push(HostViewCatalogFieldChange {
            field: "view_type".to_string(),
            previous: previous.view_type.clone().unwrap_or_default(),
            next: next.view_type.clone().unwrap_or_default(),
        });
    }
    if fields.is_empty() {
        None
    } else {
        fields.sort_by(|a, b| a.field.cmp(&b.field));
        Some(fields)
    }
}

fn view_payload_fingerprint(result: &SysmlVisualizationResultDto) -> String {
    let mut candidate_ids: Vec<String> = result
        .view_candidates
        .iter()
        .map(|candidate| candidate.id.clone())
        .collect();
    candidate_ids.sort();

    let graph = result
        .general_view_graph
        .as_ref()
        .or(result.graph.as_ref());
    let node_count = graph.map(|graph| graph.nodes.len()).unwrap_or(0);
    let edge_count = graph.map(|graph| graph.edges.len()).unwrap_or(0);
    let prepared_view_key = result
        .prepared_view
        .as_ref()
        .map(|prepared| format!("{}:{}", prepared.view, prepared.title));

    let fingerprint = serde_json::json!({
        "candidate_ids": candidate_ids,
        "empty_state_message": result.empty_state_message,
        "node_count": node_count,
        "edge_count": edge_count,
        "prepared_view": prepared_view_key,
        "selected_view": result.selected_view,
        "selected_view_name": result.selected_view_name,
        "view": result.view,
    });

    let canonical = serde_json::to_string(&fingerprint).unwrap_or_default();
    let digest = Sha256::digest(canonical.as_bytes());
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
