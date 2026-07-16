//! Map canonical interconnection scenes to render-ready prepared views.

use serde_json::{json, Value};

use crate::semantic::dto::SysmlVisualizationResultDto;
use crate::semantic::interconnection_scene::InterconnectionSceneDto;
use crate::semantic::prepared_view::dto::{PreparedEdgeDto, PreparedNodeDto, PreparedViewDto};
use crate::semantic::prepared_view::graph_norm::{
    is_definition_kind, is_reference_kind, normalize_edge_kind,
};

pub fn prepare_interconnection_prepared_view(
    response: &SysmlVisualizationResultDto,
) -> Result<PreparedViewDto, String> {
    let scene = response
        .interconnection_scene
        .as_ref()
        .ok_or_else(|| "interconnection-view requires interconnectionScene".to_string())?;
    Ok(prepare_interconnection_scene(scene, response))
}

pub fn prepare_interconnection_scene(
    scene: &InterconnectionSceneDto,
    response: &SysmlVisualizationResultDto,
) -> PreparedViewDto {
    let mut node_ids: std::collections::HashSet<String> =
        scene.nodes.iter().map(|node| node.id.clone()).collect();
    let mut nodes: Vec<PreparedNodeDto> = scene
        .nodes
        .iter()
        .map(|node| {
            let port_details: Vec<Value> = scene
                .ports
                .iter()
                .filter(|port| port.owner_node_id == node.id)
                .map(|port| {
                    json!({
                        "id": port.id,
                        "name": port.name,
                        "direction": port.direction,
                        "portType": port.type_name,
                        "portSide": match port.side_hint.as_str() {
                            "west" => json!("left"),
                            "east" => json!("right"),
                            _ => Value::Null,
                        },
                        "uri": port.uri,
                        "range": port.range,
                        "attributes": {
                            "parentId": port.owner_node_id,
                            "scenePortId": port.id,
                            "sideHint": port.side_hint,
                        }
                    })
                })
                .collect();
            PreparedNodeDto {
                id: node.id.clone(),
                label: node.name.clone(),
                kind: "part".to_string(),
                source_path: None,
                uri: node.uri.clone(),
                range: node.range.clone(),
                attributes: Some(json!({
                    "containerId": node.parent_id,
                    "qualifiedName": node.qualified_name,
                    "semanticId": node.semantic_id,
                    "definitionId": node.definition_id,
                    "partType": node.type_name,
                    "ports": port_details.iter().filter_map(|p| p.get("name").and_then(|v| v.as_str())).collect::<Vec<_>>(),
                    "portDetails": port_details,
                    "isDefinition": is_definition_kind(&node.kind),
                    "isReference": is_reference_kind(&node.kind) || node.kind == "ref",
                    "sceneNodeId": node.id,
                })),
            }
        })
        .collect();

    for container in &scene.containers {
        if node_ids.contains(&container.id) {
            continue;
        }
        nodes.push(PreparedNodeDto {
            id: container.id.clone(),
            label: container.label.clone(),
            kind: "package".to_string(),
            source_path: None,
            uri: None,
            range: None,
            attributes: Some(json!({
                "isSyntheticContainer": true,
                "containerId": container.parent_id,
                "qualifiedName": container.label,
                "memberNodeIds": container.member_node_ids,
                "layoutDepth": container.depth,
            })),
        });
        node_ids.insert(container.id.clone());
    }

    let edges: Vec<PreparedEdgeDto> = scene
        .edges
        .iter()
        .filter(|edge| {
            node_ids.contains(&edge.source_node_id) && node_ids.contains(&edge.target_node_id)
        })
        .map(|edge| PreparedEdgeDto {
            id: edge.id.clone(),
            source: edge.source_node_id.clone(),
            target: edge.target_node_id.clone(),
            label: edge.label.clone().unwrap_or_else(|| edge.kind.clone()),
            edge_kind: Some(normalize_edge_kind(&edge.kind)),
            attributes: Some(json!({
                "sourceId": edge.source_port_id,
                "targetId": edge.target_port_id,
                "sourcePortId": edge.source_port_id,
                "targetPortId": edge.target_port_id,
                "sourceNodeId": edge.source_node_id,
                "targetNodeId": edge.target_node_id,
                "semanticId": edge.semantic_id,
                "sourceExpression": edge.source_expression,
                "targetExpression": edge.target_expression,
                "relationType": edge.kind,
                "canonicalScene": true,
            })),
        })
        .collect();

    let title = if !scene.view.name.is_empty() {
        scene.view.name.clone()
    } else {
        response
            .selected_view_name
            .clone()
            .unwrap_or_else(|| "Interconnection View".to_string())
    };

    PreparedViewDto {
        title,
        view: "interconnection-view".to_string(),
        nodes,
        edges,
        meta: Some(json!({
            "canonicalScene": true,
            "schemaVersion": scene.schema_version,
            "selectedRoot": scene.view.root_ids.first(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::semantic::dto::{PositionDto, RangeDto, SysmlVisualizationResultDto};
    use crate::semantic::ibd::{IbdDataDto, IbdPartDto, IbdPortDto};
    use crate::semantic::interconnection_scene::build_interconnection_scene;

    use super::prepare_interconnection_scene;

    fn test_part(name: &str, qn: &str) -> IbdPartDto {
        IbdPartDto {
            id: qn.replace('.', "::"),
            node_id: qn.to_string(),
            name: name.to_string(),
            qualified_name: qn.to_string(),
            uri: Some("file:///model.sysml".to_string()),
            container_id: None,
            element_type: "part".to_string(),
            attributes: HashMap::new(),
            range: Some(RangeDto {
                start: PositionDto {
                    line: 4,
                    character: 2,
                },
                end: PositionDto {
                    line: 4,
                    character: 10,
                },
            }),
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
            uri: Some("file:///model.sysml".to_string()),
            range: Some(RangeDto {
                start: PositionDto {
                    line: 4,
                    character: 2,
                },
                end: PositionDto {
                    line: 4,
                    character: 10,
                },
            }),
        }
    }

    /// Regression test for a real bug found against a real workspace (the
    /// sysml-robot-vacuum-cleaner `firmwareDeployment` view): `IbdPartDto`/`IbdPortDto` and
    /// `InterconnectionNodeDto`/`InterconnectionPortDto` carried correct `uri`/`range` end to
    /// end (verified by `scene_nodes_and_ports_carry_source_location_for_click_to_source` in
    /// `interconnection_scene.rs`), but `prepare_interconnection_prepared_view` — the final step
    /// that actually produces what the renderer/webview consumes — hardcoded `uri: None,
    /// range: None` on every `PreparedNodeDto` regardless of what the scene node carried,
    /// silently breaking click-to-source for every real interconnection-view export despite the
    /// scene-level test passing. Assert all the way through to `PreparedViewDto` so this can't
    /// regress at this specific hand-off point again.
    #[test]
    fn prepared_view_nodes_and_port_details_carry_source_location_for_click_to_source() {
        let ibd = IbdDataDto {
            parts: vec![test_part("mainElectronics", "Grid.mainElectronics")],
            ports: vec![test_port(
                "Grid.mainElectronics.leftMotorPhaseOut",
                "leftMotorPhaseOut",
                "Grid.mainElectronics",
            )],
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
        let response = SysmlVisualizationResultDto {
            version: 0,
            model_ready: true,
            view: "interconnection-view".to_string(),
            workspace_root_uri: "file:///fixture".to_string(),
            view_candidates: Vec::new(),
            selected_view: None,
            selected_view_name: Some("mainElectronics".to_string()),
            empty_state_message: None,
            package_groups: None,
            graph: None,
            general_view_graph: None,
            workspace_model: None,
            activity_diagrams: None,
            activity_diagram_candidates: None,
            sequence_diagrams: None,
            sequence_diagram_candidates: None,
            state_machines: None,
            state_machine_candidates: None,
            ibd: None,
            interconnection_scene: None,
            stats: None,
            projection_hints: None,
            prepared_view: None,
        };

        let prepared = prepare_interconnection_scene(&scene, &response);

        let node = prepared
            .nodes
            .iter()
            .find(|n| n.id == scene.nodes[0].id)
            .expect("expected mainElectronics prepared node");
        assert_eq!(node.uri.as_deref(), Some("file:///model.sysml"));
        assert!(node.range.is_some());

        let port_details = node
            .attributes
            .as_ref()
            .and_then(|attrs| attrs.get("portDetails"))
            .and_then(|value| value.as_array())
            .expect("expected portDetails array");
        let port = port_details
            .iter()
            .find(|p| p.get("name").and_then(|v| v.as_str()) == Some("leftMotorPhaseOut"))
            .expect("expected leftMotorPhaseOut port detail");
        assert_eq!(
            port.get("uri").and_then(|v| v.as_str()),
            Some("file:///model.sysml")
        );
        assert!(port.get("range").is_some_and(|value| !value.is_null()));
    }
}
