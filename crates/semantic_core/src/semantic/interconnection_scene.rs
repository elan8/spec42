use serde::{Deserialize, Serialize};

use crate::semantic::ibd::{
    enrich_connector_endpoint_refs, qualified_name_to_dot, resolve_owner_part_qn_for_endpoint,
    resolve_port_id_for_endpoint, IbdConnectorDto, IbdContainerGroupDto, IbdDataDto, IbdPartDto, IbdPortDto,
};
use crate::semantic::visualization_workspace::IbdScopeTrace;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionSceneViewDto {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub view_type: String,
    pub root_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionNodeDto {
    pub id: String,
    pub semantic_id: String,
    pub qualified_name: String,
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionPortDto {
    pub id: String,
    pub semantic_id: String,
    pub owner_node_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub side_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionEdgeDto {
    pub id: String,
    pub kind: String,
    pub source_port_id: String,
    pub target_port_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionContainerDto {
    pub id: String,
    pub label: String,
    pub parent_id: Option<String>,
    pub member_node_ids: Vec<String>,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionSceneDiagnosticDto {
    pub severity: String,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterconnectionSceneDto {
    pub schema_version: u32,
    pub view: InterconnectionSceneViewDto,
    pub nodes: Vec<InterconnectionNodeDto>,
    pub ports: Vec<InterconnectionPortDto>,
    pub edges: Vec<InterconnectionEdgeDto>,
    pub containers: Vec<InterconnectionContainerDto>,
    pub diagnostics: Vec<InterconnectionSceneDiagnosticDto>,
}

pub fn scene_node_id(qualified_name: &str) -> String {
    format!("node:{}", qualified_name_to_dot(qualified_name))
}

pub fn scene_port_id(port_id: &str) -> String {
    format!("port:{}", qualified_name_to_dot(port_id))
}

pub fn scene_container_id(qualified_name: &str) -> String {
    format!("container:{}", qualified_name_to_dot(qualified_name))
}

fn port_side_hint(port: &IbdPortDto) -> String {
    match port.port_side.as_deref() {
        Some("left") | Some("west") => "west".to_string(),
        Some("right") | Some("east") => "east".to_string(),
        _ => match port.direction.as_deref() {
            Some("in") | Some("input") => "west".to_string(),
            Some("out") | Some("output") => "east".to_string(),
            _ => "auto".to_string(),
        },
    }
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

fn owner_node_id_for_port(port_id: &str, ports: &[IbdPortDto], parts: &[IbdPartDto]) -> Option<String> {
    let port_dot = qualified_name_to_dot(port_id);
    ports
        .iter()
        .find(|port| {
            qualified_name_to_dot(&port.port_id) == port_dot
                || qualified_name_to_dot(&port.id) == port_dot
        })
        .map(|port| scene_node_id(&port.parent_id))
        .or_else(|| {
            resolve_owner_part_qn_for_endpoint(port_id, parts)
                .map(|owner| scene_node_id(&owner))
        })
}

pub fn validate_connector_invariants(
    connectors: &[IbdConnectorDto],
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
) -> Vec<InterconnectionSceneDiagnosticDto> {
    let mut diagnostics = Vec::new();
    for connector in connectors {
        let connector_id = format!(
            "{}->{}",
            qualified_name_to_dot(&connector.source_id),
            qualified_name_to_dot(&connector.target_id)
        );
        for (endpoint, part_id, port_id, role) in [
            (
                &connector.source_id,
                connector.source_part_id.as_deref(),
                connector.source_port_id.as_deref(),
                "source",
            ),
            (
                &connector.target_id,
                connector.target_part_id.as_deref(),
                connector.target_port_id.as_deref(),
                "target",
            ),
        ] {
            let resolved_owner = resolve_owner_part_qn_for_endpoint(endpoint, parts);
            let resolved_port = resolve_port_id_for_endpoint(endpoint, ports);
            if let Some(owner) = resolved_owner.as_deref() {
                if let Some(explicit_part) = part_id {
                    if qualified_name_to_dot(explicit_part) != qualified_name_to_dot(owner) {
                        diagnostics.push(InterconnectionSceneDiagnosticDto {
                            severity: "error".to_string(),
                            code: "connector_owner_mismatch".to_string(),
                            message: format!(
                                "{role}PartId {explicit_part} does not match resolved owner {owner} for endpoint {endpoint}"
                            ),
                            connector_id: Some(connector_id.clone()),
                        });
                    }
                }
            } else if part_id.is_some() {
                diagnostics.push(InterconnectionSceneDiagnosticDto {
                    severity: "warning".to_string(),
                    code: "connector_owner_unresolved".to_string(),
                    message: format!("Could not resolve {role} owner for endpoint {endpoint}"),
                    connector_id: Some(connector_id.clone()),
                });
            }
            if resolved_port.is_some() && port_id.is_none() {
                diagnostics.push(InterconnectionSceneDiagnosticDto {
                    severity: "warning".to_string(),
                    code: "connector_port_id_missing".to_string(),
                    message: format!("Endpoint {endpoint} resolves to a port but {role}PortId is missing"),
                    connector_id: Some(connector_id.clone()),
                });
            }
            if let (Some(port), Some(part)) = (port_id, part_id) {
                let expected_owner = owner_node_id_for_port(port, ports, parts);
                if let Some(expected) = expected_owner {
                    let actual = scene_node_id(part);
                    if expected != actual {
                        diagnostics.push(InterconnectionSceneDiagnosticDto {
                            severity: "error".to_string(),
                            code: "connector_port_owner_mismatch".to_string(),
                            message: format!(
                                "{role} port {port} owner {expected} does not match {role}PartId {actual}"
                            ),
                            connector_id: Some(connector_id.clone()),
                        });
                    }
                }
            }
        }
    }
    diagnostics
}

fn push_scope_trace_diagnostics(
    diagnostics: &mut Vec<InterconnectionSceneDiagnosticDto>,
    scope_trace: Option<&IbdScopeTrace>,
) {
    let Some(trace) = scope_trace else {
        return;
    };
    diagnostics.push(InterconnectionSceneDiagnosticDto {
        severity: "info".to_string(),
        code: "scope_strategy".to_string(),
        message: format!(
            "scope={} full={}/{} visible={}/{} root_scoped={}/{}",
            trace.chosen,
            trace.full_parts,
            trace.full_connectors,
            trace.visible_parts,
            trace.visible_connectors,
            trace.root_scoped_parts,
            trace.root_scoped_connectors
        ),
        connector_id: None,
    });
}

fn map_containers(ibd: &IbdDataDto) -> Vec<InterconnectionContainerDto> {
    let mut containers = Vec::new();
    for group in &ibd.container_groups {
        containers.push(map_container_group(group));
    }
    for group in &ibd.package_container_groups {
        containers.push(InterconnectionContainerDto {
            id: scene_container_id(&group.qualified_package),
            label: group.label.clone(),
            parent_id: group
                .parent_id
                .as_ref()
                .map(|parent| scene_container_id(parent)),
            member_node_ids: group
                .member_part_ids
                .iter()
                .map(|member| scene_node_id(member))
                .collect(),
            depth: 0,
        });
    }
    containers
}

fn map_container_group(group: &IbdContainerGroupDto) -> InterconnectionContainerDto {
    InterconnectionContainerDto {
        id: scene_container_id(&group.qualified_name),
        label: group.label.clone(),
        parent_id: group
            .parent_id
            .as_ref()
            .map(|parent| scene_container_id(parent)),
        member_node_ids: group
            .member_part_ids
            .iter()
            .map(|member| scene_node_id(member))
            .collect(),
        depth: group.depth,
    }
}

pub fn build_interconnection_scene(
    ibd: &IbdDataDto,
    view_id: &str,
    view_name: &str,
    root_ids: &[String],
    scope_trace: Option<&IbdScopeTrace>,
) -> InterconnectionSceneDto {
    let mut connectors = ibd.connectors.clone();
    enrich_connector_endpoint_refs(&mut connectors, &ibd.parts, &ibd.ports);
    let mut diagnostics = validate_connector_invariants(&connectors, &ibd.parts, &ibd.ports);
    push_scope_trace_diagnostics(&mut diagnostics, scope_trace);

    let part_scene_ids: std::collections::HashMap<String, String> = ibd
        .parts
        .iter()
        .map(|part| (part.qualified_name.clone(), scene_node_id(&part.qualified_name)))
        .collect();

    let nodes = ibd
        .parts
        .iter()
        .map(|part| {
            let type_name = part
                .attributes
                .get("partType")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            InterconnectionNodeDto {
                id: scene_node_id(&part.qualified_name),
                semantic_id: part.node_id.clone(),
                qualified_name: part.qualified_name.clone(),
                name: part.name.clone(),
                kind: if part.element_type.to_lowercase().contains("ref") {
                    "ref".to_string()
                } else {
                    "part".to_string()
                },
                type_name,
                parent_id: part
                    .container_id
                    .as_ref()
                    .and_then(|container| part_scene_ids.get(container).cloned()),
            }
        })
        .collect();

    let ports = ibd
        .ports
        .iter()
        .map(|port| InterconnectionPortDto {
            id: scene_port_id(&port.port_id),
            semantic_id: port.port_id.clone(),
            owner_node_id: scene_node_id(&port.parent_id),
            name: port.name.clone(),
            type_name: port.port_type.clone(),
            direction: port.direction.clone(),
            side_hint: port_side_hint(port),
        })
        .collect();

    let edges = connectors
        .iter()
        .enumerate()
        .filter_map(|(index, connector)| {
            let source_port = connector
                .source_port_id
                .clone()
                .or_else(|| resolve_port_id_for_endpoint(&connector.source_id, &ibd.ports))?;
            let target_port = connector
                .target_port_id
                .clone()
                .or_else(|| resolve_port_id_for_endpoint(&connector.target_id, &ibd.ports))?;
            let source_owner = connector
                .source_part_id
                .clone()
                .or_else(|| resolve_owner_part_qn_for_endpoint(&connector.source_id, &ibd.parts))?;
            let target_owner = connector
                .target_part_id
                .clone()
                .or_else(|| resolve_owner_part_qn_for_endpoint(&connector.target_id, &ibd.parts))?;
            Some(InterconnectionEdgeDto {
                id: format!(
                    "edge:{}->{}:{}",
                    qualified_name_to_dot(&source_port),
                    qualified_name_to_dot(&target_port),
                    index
                ),
                kind: connector_kind(&connector.rel_type),
                source_port_id: scene_port_id(&source_port),
                target_port_id: scene_port_id(&target_port),
                source_node_id: scene_node_id(&source_owner),
                target_node_id: scene_node_id(&target_owner),
                semantic_id: Some(format!(
                    "{}->{}",
                    qualified_name_to_dot(&connector.source_id),
                    qualified_name_to_dot(&connector.target_id)
                )),
                label: None,
            })
        })
        .collect();

    for connector in &connectors {
        let connector_id = format!(
            "{}->{}",
            qualified_name_to_dot(&connector.source_id),
            qualified_name_to_dot(&connector.target_id)
        );
        if resolve_port_id_for_endpoint(&connector.source_id, &ibd.ports).is_none()
            || resolve_port_id_for_endpoint(&connector.target_id, &ibd.ports).is_none()
        {
            diagnostics.push(InterconnectionSceneDiagnosticDto {
                severity: "error".to_string(),
                code: "connector_endpoint_unresolved".to_string(),
                message: "Connector endpoint does not resolve to a visible port".to_string(),
                connector_id: Some(connector_id),
            });
        }
    }

    InterconnectionSceneDto {
        schema_version: 1,
        view: InterconnectionSceneViewDto {
            id: view_id.to_string(),
            name: view_name.to_string(),
            view_type: "InterconnectionView".to_string(),
            root_ids: root_ids.to_vec(),
        },
        nodes,
        ports,
        edges,
        containers: map_containers(ibd),
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_part(name: &str, qn: &str, container: Option<&str>) -> IbdPartDto {
        IbdPartDto {
            id: qn.replace('.', "::"),
            node_id: qn.to_string(),
            name: name.to_string(),
            qualified_name: qn.to_string(),
            uri: None,
            container_id: container.map(str::to_string),
            element_type: "part".to_string(),
            attributes: HashMap::new(),
        }
    }

    fn test_port(id: &str, name: &str, parent: &str) -> IbdPortDto {
        IbdPortDto {
            id: id.to_string(),
            port_id: id.to_string(),
            name: name.to_string(),
            parent_id: parent.to_string(),
            direction: None,
            port_type: None,
            port_side: None,
        }
    }

    #[test]
    fn scene_uses_nested_port_owner_for_ring_segment() {
        let ibd = IbdDataDto {
            parts: vec![
                test_part("northSouthRing", "Grid.northSouthRing", None),
                test_part(
                    "ringSegmentBtoC",
                    "Grid.northSouthRing.ringSegmentBtoC",
                    Some("Grid.northSouthRing"),
                ),
                test_part("txStationB", "Grid.txStationB", None),
            ],
            ports: vec![
                test_port(
                    "Grid.northSouthRing.ringSegmentBtoC.a",
                    "a",
                    "Grid.northSouthRing.ringSegmentBtoC",
                ),
                test_port("Grid.txStationB.mvConnection", "mvConnection", "Grid.txStationB"),
            ],
            connectors: vec![IbdConnectorDto {
                source: "Grid.txStationB.mvConnection".to_string(),
                target: "Grid.northSouthRing.ringSegmentBtoC.a".to_string(),
                source_id: "Grid.txStationB.mvConnection".to_string(),
                target_id: "Grid.northSouthRing.ringSegmentBtoC.a".to_string(),
                source_part_id: Some("Grid.txStationB".to_string()),
                target_part_id: Some("Grid.northSouthRing".to_string()),
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            }],
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: vec!["Grid.northSouthRing".to_string()],
            default_root: None,
            root_views: HashMap::new(),
        };

        let scene = build_interconnection_scene(&ibd, "view-1", "systemContext", &["Grid.northSouthRing".to_string()], None);
        assert_eq!(scene.schema_version, 1);
        assert_eq!(scene.edges.len(), 1);
        assert_eq!(
            scene.edges[0].target_node_id,
            scene_node_id("Grid.northSouthRing.ringSegmentBtoC")
        );
        assert!(!scene
            .diagnostics
            .iter()
            .any(|diag| diag.code == "connector_owner_mismatch"));
    }
}
