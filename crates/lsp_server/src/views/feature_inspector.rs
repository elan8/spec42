//! sysml/featureInspector request parsing and response building.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Position, Url};

use crate::common::text_span::to_core_position;
use crate::common::util;
use crate::semantic::{RelationshipKind, SemanticGraph, SemanticNode};
use crate::views::dto::{
    SysmlFeatureInspectorElementDto, SysmlFeatureInspectorElementRefDto,
    SysmlFeatureInspectorParamsDto, SysmlFeatureInspectorRelationshipDto,
    SysmlFeatureInspectorResolutionDto, SysmlFeatureInspectorResultDto,
    SysmlFeatureInspectorSelectionDto,
};
use sysml_model::{range_to_dto, ElementKind, PositionDto};

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
    // vscode-jsonrpc versions can encode `sendRequest(method, params, undefined)`
    // as `[params, null]`. Accept that transition artifact at the protocol boundary
    // while clients migrate to omitting the absent cancellation-token argument.
    let normalized = match v.as_array().map(Vec::as_slice) {
        Some([params]) if params.is_object() => params,
        Some([params, trailing]) if params.is_object() && trailing.is_null() => params,
        _ => v,
    };
    let params: SysmlFeatureInspectorParamsDto = serde_json::from_value(normalized.clone())
        .map_err(|error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()))?;
    let uri_text = params
        .text_document
        .map(|document| document.uri)
        .or(params.uri)
        .ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/featureInspector: expected textDocument.uri",
            )
        })?;
    let uri = Url::parse(&uri_text).map_err(|_| {
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
        version: 1,
        source_uri: uri.to_string(),
        requested_position: PositionDto {
            line: position.line,
            character: position.character,
        },
        selection: SysmlFeatureInspectorSelectionDto {
            kind: "other".to_string(),
            text: None,
            range: None,
        },
        language_help: None,
        containing_element: None,
        referenced_element: None,
    }
}

fn element_ref(node: &SemanticNode) -> SysmlFeatureInspectorElementRefDto {
    SysmlFeatureInspectorElementRefDto {
        id: node.id.qualified_name.clone(),
        name: node.name.clone(),
        qualified_name: node.id.qualified_name.clone(),
        element_type: node.element_kind.as_str().to_string(),
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

fn semantic_role(kind: &ElementKind) -> &'static str {
    if kind.is_definition() {
        "definition"
    } else {
        match kind {
            ElementKind::Package => "namespace",
            ElementKind::Interface
            | ElementKind::Flow
            | ElementKind::Allocation
            | ElementKind::Connection
            | ElementKind::Binding
            | ElementKind::DerivationConnection
            | ElementKind::Import
            | ElementKind::Dependency => "relationship",
            ElementKind::Unknown(_) => "other",
            _ => "usage",
        }
    }
}

pub(crate) fn feature_inspector_element(
    semantic_graph: &SemanticGraph,
    node: &SemanticNode,
) -> SysmlFeatureInspectorElementDto {
    let parent = node
        .parent_id
        .as_ref()
        .and_then(|parent_id| semantic_graph.get_node(parent_id))
        .map(element_ref);
    let typing_targets = semantic_graph.outgoing_targets_by_kind(node, RelationshipKind::Typing);
    let specialization_targets =
        semantic_graph.outgoing_targets_by_kind(node, RelationshipKind::Specializes);

    SysmlFeatureInspectorElementDto {
        id: node.id.qualified_name.clone(),
        name: node.name.clone(),
        qualified_name: node.id.qualified_name.clone(),
        element_type: node.element_kind.as_str().to_string(),
        role: semantic_role(&node.element_kind).to_string(),
        declaration: language_service::signature_from_node(node)
            .unwrap_or_else(|| format!("{} {};", node.element_kind, node.name)),
        uri: node.id.uri.to_string(),
        range: range_to_dto(node.range),
        parent,
        attributes: node.attributes.clone(),
        typing: resolution(has_typing_intent(node), typing_targets),
        specialization: resolution(has_specialization_intent(node), specialization_targets),
        incoming_relationships: incoming_relationships(semantic_graph, node),
        outgoing_relationships: outgoing_relationships(semantic_graph, node),
    }
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
    let containing_element = semantic_graph
        .find_deepest_node_at_position(uri, to_core_position(position))
        .filter(|node| node.id.uri == *uri)
        .map(|node| feature_inspector_element(semantic_graph, node));

    SysmlFeatureInspectorResultDto {
        version: 1,
        source_uri: uri.to_string(),
        requested_position,
        selection: SysmlFeatureInspectorSelectionDto {
            kind: "other".to_string(),
            text: None,
            range: None,
        },
        language_help: None,
        containing_element,
        referenced_element: None,
    }
}
