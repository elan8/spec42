use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::semantic::ibd::{
    enrich_connector_endpoint_refs, qualified_name_to_dot, resolve_owner_part_qn_for_endpoint,
    resolve_port_id_for_endpoint, IbdDataDto,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedFeature {
    pub occurrence_id: String,
    pub semantic_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_occurrence_id: Option<String>,
    pub qualified_name: String,
    pub name: String,
    pub kind: String,
    pub feature_chain: Vec<String>,
    pub is_boundary: bool,
    pub source: ProjectionSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedConnection {
    pub connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<String>,
    pub kind: String,
    pub endpoint_feature_ids: Vec<String>,
    pub endpoint_owner_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaring_context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProjectionSource {
    Explicit,
    Owned,
    Inherited,
    Redefined,
    Subsetted,
    EndpointClosure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterconnectionProjection {
    pub features: Vec<ProjectedFeature>,
    pub connections: Vec<ProjectedConnection>,
    pub diagnostics: Vec<ProjectionDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionDiagnostic {
    pub severity: String,
    pub code: String,
    pub message: String,
    pub connector_id: Option<String>,
}

pub fn occurrence_id_for_qualified_name(qualified_name: &str) -> String {
    format!("occ:{}", qualified_name_to_dot(qualified_name))
}

fn feature_chain_for_qualified_name(qualified_name: &str) -> Vec<String> {
    qualified_name_to_dot(qualified_name)
        .split('.')
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect()
}

fn connector_kind(rel_type: &str) -> String {
    let lower = rel_type.to_lowercase();
    if lower.contains("binding") {
        "binding".to_string()
    } else if lower.contains("reference") {
        "reference".to_string()
    } else if lower.contains("interface") {
        "interface".to_string()
    } else if lower.contains("flow") {
        "flow".to_string()
    } else {
        "connection".to_string()
    }
}

pub fn build_interconnection_projection(ibd: &IbdDataDto) -> InterconnectionProjection {
    let mut connectors = ibd.connectors.clone();
    enrich_connector_endpoint_refs(&mut connectors, &ibd.parts, &ibd.ports);

    let mut features = Vec::with_capacity(ibd.parts.len() + ibd.ports.len());
    for part in &ibd.parts {
        let occurrence_id = occurrence_id_for_qualified_name(&part.qualified_name);
        features.push(ProjectedFeature {
            occurrence_id,
            semantic_id: part.node_id.clone(),
            definition_id: part
                .attributes
                .get("partType")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            owner_occurrence_id: part
                .container_id
                .as_ref()
                .map(|parent| occurrence_id_for_qualified_name(parent)),
            qualified_name: qualified_name_to_dot(&part.qualified_name),
            name: part.name.clone(),
            kind: if part.element_type.to_lowercase().contains("ref") {
                "ref".to_string()
            } else {
                "part".to_string()
            },
            feature_chain: feature_chain_for_qualified_name(&part.qualified_name),
            is_boundary: false,
            source: ProjectionSource::Explicit,
        });
    }

    for port in &ibd.ports {
        features.push(ProjectedFeature {
            occurrence_id: occurrence_id_for_qualified_name(&port.port_id),
            semantic_id: qualified_name_to_dot(&port.port_id),
            definition_id: port.port_type.clone(),
            owner_occurrence_id: Some(occurrence_id_for_qualified_name(&port.parent_id)),
            qualified_name: qualified_name_to_dot(&port.port_id),
            name: port.name.clone(),
            kind: "port".to_string(),
            feature_chain: feature_chain_for_qualified_name(&port.port_id),
            is_boundary: true,
            source: ProjectionSource::Owned,
        });
    }

    let feature_by_occurrence: HashMap<String, &ProjectedFeature> = features
        .iter()
        .map(|feature| (feature.occurrence_id.clone(), feature))
        .collect();
    let mut diagnostics = Vec::new();
    let mut connections = Vec::new();

    for (index, connector) in connectors.iter().enumerate() {
        let connector_id = format!(
            "conn:{}->{}:{}",
            qualified_name_to_dot(&connector.source_id),
            qualified_name_to_dot(&connector.target_id),
            index
        );
        let source_port = connector
            .source_port_id
            .clone()
            .or_else(|| resolve_port_id_for_endpoint(&connector.source_id, &ibd.ports));
        let target_port = connector
            .target_port_id
            .clone()
            .or_else(|| resolve_port_id_for_endpoint(&connector.target_id, &ibd.ports));
        let source_owner = resolve_owner_part_qn_for_endpoint(&connector.source_id, &ibd.parts)
            .or_else(|| connector.source_part_id.clone());
        let target_owner = resolve_owner_part_qn_for_endpoint(&connector.target_id, &ibd.parts)
            .or_else(|| connector.target_part_id.clone());

        let Some(source_port) = source_port else {
            diagnostics.push(ProjectionDiagnostic {
                severity: "error".to_string(),
                code: "connector_source_endpoint_unresolved".to_string(),
                message: format!(
                    "Source endpoint '{}' did not resolve to a projected boundary feature",
                    connector.source_id
                ),
                connector_id: Some(connector_id),
            });
            continue;
        };
        let Some(target_port) = target_port else {
            diagnostics.push(ProjectionDiagnostic {
                severity: "error".to_string(),
                code: "connector_target_endpoint_unresolved".to_string(),
                message: format!(
                    "Target endpoint '{}' did not resolve to a projected boundary feature",
                    connector.target_id
                ),
                connector_id: Some(connector_id),
            });
            continue;
        };
        let Some(source_owner) = source_owner else {
            diagnostics.push(ProjectionDiagnostic {
                severity: "error".to_string(),
                code: "connector_source_owner_unresolved".to_string(),
                message: format!(
                    "Source endpoint '{}' did not resolve to a projected owner feature",
                    connector.source_id
                ),
                connector_id: Some(connector_id),
            });
            continue;
        };
        let Some(target_owner) = target_owner else {
            diagnostics.push(ProjectionDiagnostic {
                severity: "error".to_string(),
                code: "connector_target_owner_unresolved".to_string(),
                message: format!(
                    "Target endpoint '{}' did not resolve to a projected owner feature",
                    connector.target_id
                ),
                connector_id: Some(connector_id),
            });
            continue;
        };

        let endpoint_feature_ids = vec![
            occurrence_id_for_qualified_name(&source_port),
            occurrence_id_for_qualified_name(&target_port),
        ];
        let endpoint_owner_ids = vec![
            occurrence_id_for_qualified_name(&source_owner),
            occurrence_id_for_qualified_name(&target_owner),
        ];

        if endpoint_feature_ids
            .iter()
            .chain(endpoint_owner_ids.iter())
            .any(|id| !feature_by_occurrence.contains_key(id))
        {
            diagnostics.push(ProjectionDiagnostic {
                severity: "error".to_string(),
                code: "connector_endpoint_outside_projection".to_string(),
                message: "Connector endpoint resolved outside the projected occurrence set"
                    .to_string(),
                connector_id: Some(connector_id),
            });
            continue;
        }

        connections.push(ProjectedConnection {
            connection_id: connector_id,
            semantic_id: Some(format!(
                "{}->{}",
                qualified_name_to_dot(&connector.source_id),
                qualified_name_to_dot(&connector.target_id)
            )),
            kind: connector_kind(&connector.rel_type),
            endpoint_feature_ids,
            endpoint_owner_ids,
            declaring_context_id: None,
            source_expression: Some(connector.source.clone()),
            target_expression: Some(connector.target.clone()),
        });
    }

    InterconnectionProjection {
        features,
        connections,
        diagnostics,
    }
}
