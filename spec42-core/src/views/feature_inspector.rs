//! sysml/featureInspector request parsing and response building.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Position, Url};

use crate::common::util;
use crate::semantic_model::{RelationshipKind, SemanticGraph, SemanticNode};
use crate::views::dto::{
    range_to_dto, PositionDto, SysmlFeatureInspectorElementDto, SysmlFeatureInspectorElementRefDto,
    SysmlFeatureInspectorParamsDto, SysmlFeatureInspectorRelationshipDto,
    SysmlFeatureInspectorResolutionDto, SysmlFeatureInspectorResultDto,
};

const TYPING_ATTRIBUTE_KEYS: &[&str] = &[
    "partType",
    "attributeType",
    "portType",
    "actionType",
    "actorType",
    "itemType",
    "occurrenceType",
    "flowType",
    "allocationType",
    "stateType",
    "requirementType",
    "useCaseType",
    "concernType",
    "endType",
    "refType",
    "parameterType",
];

pub fn parse_sysml_feature_inspector_params(v: &serde_json::Value) -> Result<(Url, Position)> {
    let params: SysmlFeatureInspectorParamsDto = serde_json::from_value(v.clone())
        .map_err(|error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()))?;
    let uri = Url::parse(&params.text_document.uri).map_err(|_| {
        tower_lsp::jsonrpc::Error::invalid_params("sysml/featureInspector: invalid URI")
    })?;
    let uri = util::normalize_file_uri(&uri);
    let position = Position::new(params.position.line, params.position.character);
    Ok((uri, position))
}

pub fn empty_feature_inspector_response(
    uri: &Url,
    position: Position,
) -> SysmlFeatureInspectorResultDto {
    SysmlFeatureInspectorResultDto {
        version: 0,
        source_uri: uri.to_string(),
        requested_position: PositionDto {
            line: position.line,
            character: position.character,
        },
        element: None,
    }
}

fn element_ref(node: &SemanticNode) -> SysmlFeatureInspectorElementRefDto {
    SysmlFeatureInspectorElementRefDto {
        id: node.id.qualified_name.clone(),
        name: node.name.clone(),
        qualified_name: node.id.qualified_name.clone(),
        element_type: node.element_kind.clone(),
        uri: node.id.uri.to_string(),
        range: range_to_dto(node.range),
    }
}

fn has_typing_intent(node: &SemanticNode) -> bool {
    TYPING_ATTRIBUTE_KEYS.iter().any(|key| {
        node.attributes
            .get(*key)
            .and_then(|value| value.as_str())
            .is_some()
    })
}

fn has_specialization_intent(node: &SemanticNode) -> bool {
    node.attributes
        .get("specializes")
        .and_then(|value| value.as_str())
        .is_some()
}

fn resolution(has_intent: bool, targets: Vec<&SemanticNode>) -> SysmlFeatureInspectorResolutionDto {
    let status = if !has_intent {
        "notApplicable"
    } else if targets.is_empty() {
        "unresolved"
    } else {
        "resolved"
    };
    SysmlFeatureInspectorResolutionDto {
        status: status.to_string(),
        targets: targets.into_iter().map(element_ref).collect(),
    }
}

fn outgoing_relationships(
    semantic_graph: &SemanticGraph,
    node: &SemanticNode,
) -> Vec<SysmlFeatureInspectorRelationshipDto> {
    semantic_graph
        .outgoing_relationships(node)
        .into_iter()
        .map(|(peer, kind)| SysmlFeatureInspectorRelationshipDto {
            rel_type: kind.as_str().to_string(),
            peer: element_ref(peer),
            name: None,
        })
        .collect()
}

fn incoming_relationships(
    semantic_graph: &SemanticGraph,
    node: &SemanticNode,
) -> Vec<SysmlFeatureInspectorRelationshipDto> {
    semantic_graph
        .incoming_relationships(node)
        .into_iter()
        .map(|(peer, kind)| SysmlFeatureInspectorRelationshipDto {
            rel_type: kind.as_str().to_string(),
            peer: element_ref(peer),
            name: None,
        })
        .collect()
}

pub fn build_sysml_feature_inspector_response(
    semantic_graph: &SemanticGraph,
    uri: &Url,
    position: Position,
) -> SysmlFeatureInspectorResultDto {
    let requested_position = PositionDto {
        line: position.line,
        character: position.character,
    };
    let element = semantic_graph
        .find_deepest_node_at_position(uri, position)
        .filter(|node| node.id.uri == *uri)
        .map(|node| {
            let parent = node
                .parent_id
                .as_ref()
                .and_then(|parent_id| semantic_graph.get_node(parent_id))
                .map(element_ref);
            let typing_targets =
                semantic_graph.outgoing_targets_by_kind(node, RelationshipKind::Typing);
            let specialization_targets =
                semantic_graph.outgoing_targets_by_kind(node, RelationshipKind::Specializes);

            SysmlFeatureInspectorElementDto {
                id: node.id.qualified_name.clone(),
                name: node.name.clone(),
                qualified_name: node.id.qualified_name.clone(),
                element_type: node.element_kind.clone(),
                uri: node.id.uri.to_string(),
                range: range_to_dto(node.range),
                parent,
                attributes: node.attributes.clone(),
                typing: resolution(has_typing_intent(node), typing_targets),
                specialization: resolution(has_specialization_intent(node), specialization_targets),
                incoming_relationships: incoming_relationships(semantic_graph, node),
                outgoing_relationships: outgoing_relationships(semantic_graph, node),
            }
        });

    SysmlFeatureInspectorResultDto {
        version: 0,
        source_uri: uri.to_string(),
        requested_position,
        element,
    }
}
