//! sysml/featureInspector request parsing and response building.

use std::collections::{HashSet, VecDeque};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Position, Url};

use crate::common::text_span::to_core_position;
use crate::common::util;
use crate::semantic::{RelationshipKind, SemanticGraph, SemanticNode};
use crate::views::dto::{
    SysmlFeatureInspectorElementDto, SysmlFeatureInspectorElementRefDto,
    SysmlFeatureInspectorEvaluationDto, SysmlFeatureInspectorEvaluationStatusDto,
    SysmlFeatureInspectorInheritedFeatureDto, SysmlFeatureInspectorParamsDto,
    SysmlFeatureInspectorRelationshipDto, SysmlFeatureInspectorResolutionDto,
    SysmlFeatureInspectorResultDto, SysmlFeatureInspectorSelectionDto,
};
use sysml_model::{range_to_dto, ElementKind, ExpressionEvaluationQuery, PositionDto};

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
    "viewType",
    "viewpointType",
    "renderingType",
    "subjectType",
    "analysisType",
    "verificationType",
    "connectionType",
    "metadataType",
    "keywordType",
];

/// The inspector is a JSON transport boundary. Semantic evaluation keeps this closed scalar
/// representation in `sysml_model`; conversion happens only while assembling this DTO.
fn evaluated_value_to_json(value: &sysml_model::EvaluatedValue) -> serde_json::Value {
    match value {
        sysml_model::EvaluatedValue::Integer(value) => serde_json::Value::Number((*value).into()),
        sysml_model::EvaluatedValue::Real(value) => serde_json::Number::from_f64(*value)
            .map(serde_json::Value::Number)
            // Preserve an invalid externally-constructed scalar visibly instead of presenting it
            // as a successful JSON number. The evaluator itself only publishes finite values.
            .unwrap_or_else(|| serde_json::Value::String(value.to_string())),
        sysml_model::EvaluatedValue::Boolean(value) => serde_json::Value::Bool(*value),
        sysml_model::EvaluatedValue::String(value) => serde_json::Value::String(value.clone()),
    }
}

fn evaluation_status(
    status: sysml_model::EvaluationStatus,
) -> SysmlFeatureInspectorEvaluationStatusDto {
    match status {
        sysml_model::EvaluationStatus::Ok => SysmlFeatureInspectorEvaluationStatusDto::Ok,
        sysml_model::EvaluationStatus::Unresolved => {
            SysmlFeatureInspectorEvaluationStatusDto::Unresolved
        }
        sysml_model::EvaluationStatus::Ambiguous => {
            SysmlFeatureInspectorEvaluationStatusDto::Ambiguous
        }
        sysml_model::EvaluationStatus::Malformed => {
            SysmlFeatureInspectorEvaluationStatusDto::Malformed
        }
        sysml_model::EvaluationStatus::Incomplete => {
            SysmlFeatureInspectorEvaluationStatusDto::Incomplete
        }
        sysml_model::EvaluationStatus::TypeError => {
            SysmlFeatureInspectorEvaluationStatusDto::TypeError
        }
        sysml_model::EvaluationStatus::DivByZero => {
            SysmlFeatureInspectorEvaluationStatusDto::DivByZero
        }
        sysml_model::EvaluationStatus::Unsupported => {
            SysmlFeatureInspectorEvaluationStatusDto::Unsupported
        }
        sysml_model::EvaluationStatus::Cycle => SysmlFeatureInspectorEvaluationStatusDto::Cycle,
    }
}

fn evaluation(
    semantic_graph: &SemanticGraph,
    node: &SemanticNode,
) -> SysmlFeatureInspectorEvaluationDto {
    match semantic_graph.expression_evaluation_for(node) {
        ExpressionEvaluationQuery::NotRun => SysmlFeatureInspectorEvaluationDto::NotRun,
        ExpressionEvaluationQuery::NotApplicable => {
            SysmlFeatureInspectorEvaluationDto::NotApplicable
        }
        ExpressionEvaluationQuery::Result(result) => {
            let analysis = semantic_graph
                .evaluation_facts_for(node)
                .and_then(|facts| facts.analysis.as_ref());
            SysmlFeatureInspectorEvaluationDto::Result {
                status: evaluation_status(result.status),
                value: result.value.as_ref().map(evaluated_value_to_json),
                unit: result.unit.clone(),
                error: result.error.clone(),
                passed: analysis.and_then(|value| value.passed),
                computed_value: analysis
                    .and_then(|value| value.computed_value.as_ref())
                    .map(evaluated_value_to_json),
                computed_unit: analysis.and_then(|value| value.computed_unit.clone()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_model::{
        build_semantic_graph_from_documents, evaluate_expressions, SysmlDocument,
        SysmlDocumentSourceKind,
    };

    fn graph() -> (SemanticGraph, sysml_model::NodeId) {
        let document = SysmlDocument::from_memory_path(
            "inspector-evaluation",
            "model.sysml",
            "package Demo { attribute value = 1; part def Empty; }".into(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");
        let id = graph
            .node_ids_by_qualified_name
            .get("Demo::value")
            .and_then(|ids| ids.first())
            .cloned()
            .expect("value");
        (graph, id)
    }

    #[test]
    fn inspector_evaluation_is_not_run_before_phase() {
        let (mut graph, id) = graph();
        graph.invalidate_evaluation_facts();
        let dto = feature_inspector_element(&graph, graph.get_node(&id).expect("node"));
        assert!(matches!(
            dto.evaluation,
            SysmlFeatureInspectorEvaluationDto::NotRun
        ));
    }

    #[test]
    fn inspector_evaluation_is_not_applicable_after_phase() {
        let (mut graph, _) = graph();
        evaluate_expressions(&mut graph);
        let id = graph
            .node_ids_by_qualified_name
            .get("Demo::Empty")
            .and_then(|ids| ids.first())
            .cloned()
            .expect("empty");
        let dto = feature_inspector_element(&graph, graph.get_node(&id).expect("node"));
        assert!(matches!(
            dto.evaluation,
            SysmlFeatureInspectorEvaluationDto::NotApplicable
        ));
    }

    #[test]
    fn inspector_evaluation_projects_completed_typed_result_at_transport_boundary() {
        let (mut graph, id) = graph();
        evaluate_expressions(&mut graph);
        let dto = feature_inspector_element(&graph, graph.get_node(&id).expect("node"));
        assert!(matches!(
            dto.evaluation,
            SysmlFeatureInspectorEvaluationDto::Result {
                status: SysmlFeatureInspectorEvaluationStatusDto::Ok,
                value: Some(serde_json::Value::Number(ref value)),
                unit: None,
                ..
            } if value.as_i64() == Some(1)
        ));
    }
}

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
        version: 2,
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

fn has_relationship_intent(node: &SemanticNode, attribute_keys: &[&str]) -> bool {
    attribute_keys.iter().any(|key| {
        node.attributes
            .get(*key)
            .is_some_and(|value| !value.is_null())
    })
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
        .filter(|(peer, kind)| {
            *kind != RelationshipKind::Annotation || peer.element_kind != ElementKind::Documentation
        })
        .map(|(peer, kind)| SysmlFeatureInspectorRelationshipDto {
            rel_type: kind.as_str().to_string(),
            peer: element_ref(peer),
            name: None,
        })
        .collect()
}

fn distinct_nodes<'a>(nodes: impl IntoIterator<Item = &'a SemanticNode>) -> Vec<&'a SemanticNode> {
    let mut seen = HashSet::new();
    nodes
        .into_iter()
        .filter(|node| seen.insert(node.id.clone()))
        .collect()
}

fn declared_target_candidates<'a>(
    semantic_graph: &'a SemanticGraph,
    node: &SemanticNode,
    raw: &str,
) -> Vec<&'a SemanticNode> {
    let normalized = raw.trim().trim_start_matches('~').replace('.', "::");
    if normalized.is_empty() {
        return Vec::new();
    }
    let mut qualified_candidates = vec![normalized.clone()];
    if !normalized.contains("::") {
        if let Some(parent) = node
            .parent_id
            .as_ref()
            .and_then(|parent_id| semantic_graph.get_node(parent_id))
        {
            qualified_candidates.push(format!("{}::{normalized}", parent.id.qualified_name));
        }
    }
    let exact = distinct_nodes(
        qualified_candidates
            .iter()
            .filter_map(|qualified| semantic_graph.node_ids_by_qualified_name.get(qualified))
            .flatten()
            .filter_map(|id| semantic_graph.get_node(id))
            .filter(|candidate| candidate.id != node.id),
    );
    if !exact.is_empty() {
        return exact;
    }

    let simple = normalized.split("::").last().unwrap_or(normalized.as_str());
    if let Some(owner) = node
        .parent_id
        .as_ref()
        .and_then(|parent_id| semantic_graph.get_node(parent_id))
    {
        if let sysml_model::ResolveResult::Resolved(target_id) =
            sysml_model::resolve_inherited_member_via_type(semantic_graph, owner, simple)
        {
            if let Some(target) = semantic_graph.get_node(&target_id) {
                return vec![target];
            }
        }
    }
    let matches: Vec<_> = semantic_graph
        .nodes_named(simple)
        .into_iter()
        .filter(|candidate| candidate.id != node.id)
        .collect();
    if matches.len() == 1 {
        matches
    } else {
        Vec::new()
    }
}

fn relationship_targets_with_fallback<'a>(
    semantic_graph: &'a SemanticGraph,
    node: &'a SemanticNode,
    kind: RelationshipKind,
    attribute_keys: &[&str],
) -> Vec<&'a SemanticNode> {
    let direct = semantic_graph.outgoing_targets_by_kind(node, kind);
    if !direct.is_empty() {
        return distinct_nodes(direct);
    }
    distinct_nodes(attribute_keys.iter().flat_map(|key| {
        node.attributes
            .get(*key)
            .into_iter()
            .flat_map(|value| match value {
                serde_json::Value::String(raw) => raw
                    .split(',')
                    .flat_map(|target| declared_target_candidates(semantic_graph, node, target))
                    .collect(),
                serde_json::Value::Array(values) => values
                    .iter()
                    .filter_map(|value| value.as_str())
                    .flat_map(|target| declared_target_candidates(semantic_graph, node, target))
                    .collect(),
                _ => Vec::new(),
            })
    }))
}

fn subsetting_targets<'a>(
    semantic_graph: &'a SemanticGraph,
    node: &'a SemanticNode,
) -> Vec<&'a SemanticNode> {
    distinct_nodes(
        [
            (RelationshipKind::Subsetting, "subsetsFeature"),
            (RelationshipKind::ReferenceSubsetting, "referencesFeature"),
            (RelationshipKind::CrossSubsetting, "crossesFeature"),
        ]
        .into_iter()
        .flat_map(|(kind, key)| {
            relationship_targets_with_fallback(semantic_graph, node, kind, &[key])
        }),
    )
}

fn effective_typing_targets<'a>(
    semantic_graph: &'a SemanticGraph,
    node: &'a SemanticNode,
) -> Vec<&'a SemanticNode> {
    let direct = relationship_targets_with_fallback(
        semantic_graph,
        node,
        RelationshipKind::Typing,
        TYPING_ATTRIBUTE_KEYS,
    );
    if !direct.is_empty() {
        return distinct_nodes(direct);
    }

    let mut visited = HashSet::new();
    let mut queue: VecDeque<&SemanticNode> = [
        (RelationshipKind::Redefinition, "redefines"),
        (RelationshipKind::Subsetting, "subsetsFeature"),
        (RelationshipKind::ReferenceSubsetting, "referencesFeature"),
        (RelationshipKind::CrossSubsetting, "crossesFeature"),
    ]
    .into_iter()
    .flat_map(|(kind, key)| relationship_targets_with_fallback(semantic_graph, node, kind, &[key]))
    .collect();
    while let Some(candidate) = queue.pop_front() {
        if !visited.insert(candidate.id.clone()) {
            continue;
        }
        let typing = relationship_targets_with_fallback(
            semantic_graph,
            candidate,
            RelationshipKind::Typing,
            TYPING_ATTRIBUTE_KEYS,
        );
        if !typing.is_empty() {
            return distinct_nodes(typing);
        }
        for (kind, key) in [
            (RelationshipKind::Redefinition, "redefines"),
            (RelationshipKind::Subsetting, "subsetsFeature"),
            (RelationshipKind::ReferenceSubsetting, "referencesFeature"),
            (RelationshipKind::CrossSubsetting, "crossesFeature"),
        ] {
            queue.extend(relationship_targets_with_fallback(
                semantic_graph,
                candidate,
                kind,
                &[key],
            ));
        }
    }
    Vec::new()
}

fn feature_modifiers(semantic_graph: &SemanticGraph, node: &SemanticNode) -> Vec<String> {
    let Some(properties) = node.declared_facts.feature_properties.as_ref() else {
        return Vec::new();
    };
    let mut modifiers = Vec::new();
    for (enabled, label) in [
        (properties.is_abstract, "abstract"),
        (properties.is_variation, "variation"),
        (properties.is_individual, "individual"),
        (properties.is_derived, "derived"),
        (properties.is_constant, "constant"),
        (properties.is_end, "end"),
        (properties.is_conjugated, "conjugated"),
        (properties.is_portion, "portion"),
    ] {
        if enabled {
            modifiers.push(label.to_string());
        }
    }
    if let Some(ownership) = semantic_graph.effective_feature_ownership_for(node) {
        if ownership.is_reference {
            modifiers.push("reference".to_string());
        } else if ownership.is_composite {
            modifiers.push("composite".to_string());
        }
    }
    if properties.is_ordered == Some(true) {
        modifiers.push("ordered".to_string());
    }
    if properties.is_unique == Some(false) {
        modifiers.push("nonunique".to_string());
    }
    if let Some(portion_kind) = properties.portion_kind.as_deref() {
        if !modifiers.iter().any(|item| item == portion_kind) {
            modifiers.push(portion_kind.to_string());
        }
    }
    modifiers
}

fn declared_expression_text(expression: &sysml_model::DeclaredExpression) -> Option<String> {
    if let Some(literal) = expression.literal.as_ref() {
        return match literal {
            sysml_model::DeclaredLiteral::Integer(value) => Some(value.to_string()),
            sysml_model::DeclaredLiteral::Real(value)
            | sysml_model::DeclaredLiteral::String(value) => Some(value.clone()),
            sysml_model::DeclaredLiteral::Boolean(value) => Some(value.to_string()),
        };
    }
    expression.reference.clone()
}

fn multiplicity_text(node: &SemanticNode) -> Option<String> {
    if let Some(text) = node
        .attributes
        .get("multiplicity")
        .and_then(|value| value.as_str())
    {
        return Some(text.to_string());
    }
    let multiplicity = node.declared_facts.multiplicity.as_ref()?;
    let lower = multiplicity
        .lower
        .as_ref()
        .and_then(declared_expression_text);
    let upper = multiplicity
        .upper
        .as_ref()
        .and_then(declared_expression_text);
    match (lower, upper) {
        (Some(lower), Some(upper)) if lower == upper => Some(lower),
        (Some(lower), Some(upper)) => Some(format!("{lower}..{upper}")),
        // The parser represents the unlimited upper bound in `[lower..*]` as `None`.
        (Some(lower), None) => Some(format!("{lower}..*")),
        (None, Some(upper)) => Some(upper),
        (None, None) => None,
    }
}

fn metadata_refs(
    semantic_graph: &SemanticGraph,
    node: &SemanticNode,
) -> Vec<SysmlFeatureInspectorElementRefDto> {
    distinct_nodes(
        semantic_graph
            .incoming_relationships(node)
            .into_iter()
            .filter_map(|(peer, kind)| {
                (kind == RelationshipKind::Annotation
                    && peer.element_kind != ElementKind::Documentation)
                    .then_some(peer)
            }),
    )
    .into_iter()
    .map(element_ref)
    .collect()
}

fn inherited_features(
    semantic_graph: &SemanticGraph,
    node: &SemanticNode,
    effective_typing: &[&SemanticNode],
) -> Vec<SysmlFeatureInspectorInheritedFeatureDto> {
    let mut queue: VecDeque<&SemanticNode> = if node.element_kind.is_definition() {
        relationship_targets_with_fallback(
            semantic_graph,
            node,
            RelationshipKind::Specializes,
            &["specializes"],
        )
        .into()
    } else {
        effective_typing.iter().copied().collect()
    };
    let direct_names: HashSet<String> = semantic_graph
        .children_of(node)
        .into_iter()
        .map(|child| child.name.to_ascii_lowercase())
        .collect();
    let mut seen_owners = HashSet::new();
    let mut seen_features = direct_names;
    let mut inherited = Vec::new();

    while let Some(owner) = queue.pop_front() {
        if !seen_owners.insert(owner.id.clone()) {
            continue;
        }
        for child in semantic_graph.children_of(owner) {
            if semantic_role(&child.element_kind) != "usage" || child.name.trim().is_empty() {
                continue;
            }
            if !seen_features.insert(child.name.to_ascii_lowercase()) {
                continue;
            }
            inherited.push(SysmlFeatureInspectorInheritedFeatureDto {
                feature: element_ref(child),
                declared_in: element_ref(owner),
            });
        }
        queue.extend(relationship_targets_with_fallback(
            semantic_graph,
            owner,
            RelationshipKind::Specializes,
            &["specializes"],
        ));
    }
    inherited
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
    let typing_targets = relationship_targets_with_fallback(
        semantic_graph,
        node,
        RelationshipKind::Typing,
        TYPING_ATTRIBUTE_KEYS,
    );
    let effective_typing_targets = effective_typing_targets(semantic_graph, node);
    let specialization_targets = relationship_targets_with_fallback(
        semantic_graph,
        node,
        RelationshipKind::Specializes,
        &["specializes"],
    );
    let subsetting_targets = subsetting_targets(semantic_graph, node);
    let redefinition_targets = relationship_targets_with_fallback(
        semantic_graph,
        node,
        RelationshipKind::Redefinition,
        &["redefines"],
    );
    let inherited_features = inherited_features(semantic_graph, node, &effective_typing_targets);
    let documentation = node
        .attributes
        .get("doc")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let multiplicity = multiplicity_text(node);
    let direction = node
        .declared_facts
        .feature_properties
        .as_ref()
        .and_then(|properties| properties.direction.clone())
        .or_else(|| {
            node.attributes
                .get("direction")
                .and_then(|value| value.as_str())
                .map(str::to_string)
        });

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
        documentation,
        multiplicity,
        direction,
        modifiers: feature_modifiers(semantic_graph, node),
        attributes: node.attributes.clone(),
        evaluation: evaluation(semantic_graph, node),
        typing: resolution(has_typing_intent(node), typing_targets),
        effective_typing: resolution(
            has_typing_intent(node)
                || has_relationship_intent(
                    node,
                    &[
                        "redefines",
                        "subsetsFeature",
                        "referencesFeature",
                        "crossesFeature",
                    ],
                ),
            effective_typing_targets,
        ),
        specialization: resolution(has_specialization_intent(node), specialization_targets),
        subsetting: resolution(
            has_relationship_intent(
                node,
                &["subsetsFeature", "referencesFeature", "crossesFeature"],
            ),
            subsetting_targets,
        ),
        redefinition: resolution(
            has_relationship_intent(node, &["redefines"]),
            redefinition_targets,
        ),
        inherited_features,
        metadata: metadata_refs(semantic_graph, node),
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
        version: 2,
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
