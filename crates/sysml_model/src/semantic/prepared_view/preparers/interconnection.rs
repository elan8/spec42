//! Map canonical interconnection scenes to render-ready prepared views.

use serde_json::{json, Value};

use crate::semantic::dto::SysmlVisualizationResultDto;
use crate::semantic::interconnection_scene::InterconnectionSceneDto;
use crate::semantic::prepared_view::dto::{PreparedEdgeDto, PreparedNodeDto, PreparedViewDto};
use crate::semantic::prepared_view::graph_norm::{is_definition_kind, is_reference_kind, normalize_edge_kind};

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
                uri: None,
                range: None,
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
        .filter(|edge| node_ids.contains(&edge.source_node_id) && node_ids.contains(&edge.target_node_id))
        .map(|edge| PreparedEdgeDto {
            id: edge.id.clone(),
            source: edge.source_node_id.clone(),
            target: edge.target_node_id.clone(),
            label: edge
                .label
                .clone()
                .unwrap_or_else(|| edge.kind.clone()),
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
