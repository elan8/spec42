use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::semantic::dto::RangeDto;
use crate::semantic::ibd::{qualified_name_to_dot, IbdContainerGroupDto, IbdDataDto, IbdPortDto};
use crate::semantic::interconnection_projection::{
    build_interconnection_projection, occurrence_id_for_qualified_name, ProjectedFeature,
    ProjectionDiagnostic,
};
use crate::semantic::visualization_workspace::IbdScopeTrace;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionSceneViewDTO")]
pub struct InterconnectionSceneViewDto {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub view_type: String,
    pub root_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionSceneNodeDTO")]
pub struct InterconnectionNodeDto {
    pub id: String,
    pub semantic_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub definition_id: Option<String>,
    pub qualified_name: String,
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub type_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub parent_id: Option<String>,
    /// Source document URI, for click-to-source navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub uri: Option<String>,
    /// Source location of the declaring element, for click-to-source navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionScenePortDTO")]
pub struct InterconnectionPortDto {
    pub id: String,
    pub semantic_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub definition_id: Option<String>,
    pub owner_node_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub type_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub direction: Option<String>,
    pub side_hint: String,
    /// Source document URI, for click-to-source navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub uri: Option<String>,
    /// Source location of the declaring port, for click-to-source navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionSceneEdgeDTO")]
pub struct InterconnectionEdgeDto {
    pub id: String,
    pub kind: String,
    pub source_port_id: String,
    pub target_port_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub semantic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub source_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub target_expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionSceneContainerDTO")]
pub struct InterconnectionContainerDto {
    pub id: String,
    pub label: String,
    pub parent_id: Option<String>,
    pub member_node_ids: Vec<String>,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionSceneDiagnosticDTO")]
pub struct InterconnectionSceneDiagnosticDto {
    pub severity: String,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "InterconnectionSceneDTO")]
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

fn projection_diagnostic_to_scene(
    diagnostic: ProjectionDiagnostic,
) -> InterconnectionSceneDiagnosticDto {
    InterconnectionSceneDiagnosticDto {
        severity: diagnostic.severity,
        code: diagnostic.code,
        message: diagnostic.message,
        connector_id: diagnostic.connector_id,
    }
}

fn map_containers(ibd: &IbdDataDto) -> Vec<InterconnectionContainerDto> {
    let mut containers = Vec::new();
    for group in &ibd.container_groups {
        containers.push(map_container_group(group));
    }
    for group in &ibd.package_container_groups {
        containers.push(InterconnectionContainerDto {
            id: occurrence_id_for_qualified_name(&group.qualified_package),
            label: group.label.clone(),
            parent_id: group
                .parent_id
                .as_ref()
                .map(|parent| occurrence_id_for_qualified_name(parent)),
            member_node_ids: group
                .member_part_ids
                .iter()
                .map(|member| occurrence_id_for_qualified_name(member))
                .collect(),
            depth: 0,
        });
    }
    containers
}

fn map_container_group(group: &IbdContainerGroupDto) -> InterconnectionContainerDto {
    InterconnectionContainerDto {
        id: occurrence_id_for_qualified_name(&group.qualified_name),
        label: group.label.clone(),
        parent_id: group
            .parent_id
            .as_ref()
            .map(|parent| occurrence_id_for_qualified_name(parent)),
        member_node_ids: group
            .member_part_ids
            .iter()
            .map(|member| occurrence_id_for_qualified_name(member))
            .collect(),
        depth: group.depth,
    }
}

fn side_hint_for_projected_port(port: &ProjectedFeature, ibd_ports: &[IbdPortDto]) -> String {
    let matched = ibd_ports.iter().find(|candidate| {
        occurrence_id_for_qualified_name(&candidate.port_id) == port.occurrence_id
    });
    match matched.and_then(|port| port.port_side.as_deref()) {
        Some("left") | Some("west") => return "west".to_string(),
        Some("right") | Some("east") => return "east".to_string(),
        _ => {}
    }
    match matched.and_then(|port| port.direction.as_deref()) {
        Some("in") | Some("input") => "west".to_string(),
        Some("out") | Some("output") => "east".to_string(),
        _ => "auto".to_string(),
    }
}

pub fn build_interconnection_scene(
    ibd: &IbdDataDto,
    view_id: &str,
    view_name: &str,
    root_ids: &[String],
    scope_trace: Option<&IbdScopeTrace>,
) -> InterconnectionSceneDto {
    let projection = build_interconnection_projection(ibd);
    let mut diagnostics: Vec<InterconnectionSceneDiagnosticDto> = projection
        .diagnostics
        .into_iter()
        .map(projection_diagnostic_to_scene)
        .collect();
    push_scope_trace_diagnostics(&mut diagnostics, scope_trace);

    let parts_by_qn: std::collections::HashMap<&str, &crate::semantic::ibd::IbdPartDto> = ibd
        .parts
        .iter()
        .map(|part| (part.qualified_name.as_str(), part))
        .collect();
    let ports_by_occurrence_id: std::collections::HashMap<String, &IbdPortDto> = ibd
        .ports
        .iter()
        .map(|port| (occurrence_id_for_qualified_name(&port.port_id), port))
        .collect();

    let nodes = projection
        .features
        .iter()
        .filter(|feature| !feature.is_boundary)
        .map(|feature| {
            let source_part = parts_by_qn.get(feature.qualified_name.as_str()).copied();
            InterconnectionNodeDto {
                id: feature.occurrence_id.clone(),
                semantic_id: feature.semantic_id.clone(),
                definition_id: feature.definition_id.clone(),
                qualified_name: feature.qualified_name.clone(),
                name: feature.name.clone(),
                kind: feature.kind.clone(),
                type_name: feature.definition_id.clone(),
                parent_id: feature.owner_occurrence_id.clone(),
                uri: source_part.and_then(|part| part.uri.clone()),
                range: source_part.and_then(|part| part.range.clone()),
            }
        })
        .collect::<Vec<_>>();

    let ports = projection
        .features
        .iter()
        .filter(|feature| feature.is_boundary)
        .map(|feature| {
            let source_port = ports_by_occurrence_id.get(&feature.occurrence_id).copied();
            InterconnectionPortDto {
                id: feature.occurrence_id.clone(),
                semantic_id: feature.semantic_id.clone(),
                definition_id: feature.definition_id.clone(),
                owner_node_id: feature.owner_occurrence_id.clone().unwrap_or_default(),
                name: feature.name.clone(),
                type_name: feature.definition_id.clone(),
                direction: source_port.and_then(|port| port.direction.clone()),
                side_hint: side_hint_for_projected_port(feature, &ibd.ports),
                uri: source_port.and_then(|port| port.uri.clone()),
                range: source_port.and_then(|port| port.range.clone()),
            }
        })
        .collect::<Vec<_>>();

    let edges: Vec<InterconnectionEdgeDto> = projection
        .connections
        .iter()
        .filter_map(|connection| {
            let source_port = connection.endpoint_feature_ids.first()?.clone();
            let target_port = connection.endpoint_feature_ids.get(1)?.clone();
            let source_owner = connection.endpoint_owner_ids.first()?.clone();
            let target_owner = connection.endpoint_owner_ids.get(1)?.clone();
            Some(InterconnectionEdgeDto {
                id: connection.connection_id.clone(),
                kind: connection.kind.clone(),
                source_port_id: source_port,
                target_port_id: target_port,
                source_node_id: source_owner,
                target_node_id: target_owner,
                semantic_id: connection.semantic_id.clone(),
                label: None,
                source_expression: connection.source_expression.clone(),
                target_expression: connection.target_expression.clone(),
            })
        })
        .collect();

    InterconnectionSceneDto {
        schema_version: 2,
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
    use crate::semantic::ibd::{IbdConnectorDto, IbdPartDto};
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
            range: None,
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
            uri: None,
            range: None,
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
                test_port(
                    "Grid.txStationB.mvConnection",
                    "mvConnection",
                    "Grid.txStationB",
                ),
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
            def_instance_mappings: Vec::new(),
        };

        let scene = build_interconnection_scene(
            &ibd,
            "view-1",
            "systemContext",
            &["Grid.northSouthRing".to_string()],
            None,
        );
        assert_eq!(scene.schema_version, 2);
        assert_eq!(scene.edges.len(), 1);
        assert_eq!(
            scene.edges[0].target_node_id,
            occurrence_id_for_qualified_name("Grid.northSouthRing.ringSegmentBtoC")
        );
        assert!(!scene
            .diagnostics
            .iter()
            .any(|diag| diag.code == "connector_owner_mismatch"));
    }

    #[test]
    fn scene_nodes_and_ports_carry_source_location_for_click_to_source() {
        let range = RangeDto {
            start: crate::semantic::dto::PositionDto { line: 4, character: 2 },
            end: crate::semantic::dto::PositionDto { line: 4, character: 10 },
        };
        let mut part = test_part("mainElectronics", "Grid.mainElectronics", None);
        part.uri = Some("file:///model.sysml".to_string());
        part.range = Some(range.clone());
        let mut port = test_port(
            "Grid.mainElectronics.leftMotorPhaseOut",
            "leftMotorPhaseOut",
            "Grid.mainElectronics",
        );
        port.uri = Some("file:///model.sysml".to_string());
        port.range = Some(range.clone());

        let ibd = IbdDataDto {
            parts: vec![part],
            ports: vec![port],
            connectors: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: vec!["Grid.mainElectronics".to_string()],
            default_root: None,
            root_views: HashMap::new(),
            def_instance_mappings: Vec::new(),
        };

        let scene = build_interconnection_scene(
            &ibd,
            "view-1",
            "mainElectronics",
            &["Grid.mainElectronics".to_string()],
            None,
        );

        let node = scene
            .nodes
            .iter()
            .find(|n| n.qualified_name == "Grid.mainElectronics")
            .expect("expected mainElectronics node");
        assert_eq!(node.uri.as_deref(), Some("file:///model.sysml"));
        assert_eq!(node.range, Some(range.clone()));

        let port = scene
            .ports
            .iter()
            .find(|p| p.name == "leftMotorPhaseOut")
            .expect("expected leftMotorPhaseOut port");
        assert_eq!(port.uri.as_deref(), Some("file:///model.sysml"));
        assert_eq!(port.range, Some(range));
    }
}
