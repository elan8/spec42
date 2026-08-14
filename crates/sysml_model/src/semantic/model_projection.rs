use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};
use url::Url;

use crate::semantic::dto::{range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic::element_kind_classify::{is_attribute_like, is_parameter_like};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::kinds::is_port_like_str as is_port_like;

/// Projects [`crate::semantic::model::DeclaredExpressionText`] and the analysis-case-level
/// `objectiveBoundTo`/`analysisExpression` facts onto a boundary DTO's legacy `attributes` JSON
/// map, at the transport boundary only (see `AGENTS.md` "Boundary DTO modules remain explicitly
/// allowed"). `SemanticNode` itself no longer carries these as JSON (`planning/UNIFY_CACHE_PROGRESS.md`
/// chunk E); this keeps presentation consumers that read the projected DTO's `attributes` (e.g.
/// `general_view_fold::detail_value_text`, `lsp_server`'s hover/symbol projections) unchanged.
pub fn project_expression_text_attributes(
    attributes: &mut HashMap<String, Value>,
    node: &crate::semantic::model::SemanticNode,
) {
    let text = &node.expression_text;
    if let Some(value) = &text.value {
        attributes.insert("value".to_string(), json!(value));
    }
    if let Some(value) = &text.default_value {
        attributes.insert("defaultValue".to_string(), json!(value));
    }
    if let Some(value) = &text.lhs {
        attributes.insert("lhs".to_string(), json!(value));
    }
    if let Some(value) = &text.rhs {
        attributes.insert("rhs".to_string(), json!(value));
    }
    if let Some(value) = &text.condition {
        attributes.insert("condition".to_string(), json!(value));
    }
    if let Some(value) = &text.is_then {
        attributes.insert("isThen".to_string(), json!(value));
    }
    if let Some(facts) = &node.declared_facts.analysis_case {
        if let Some(expression) = &facts.expression {
            attributes.insert("analysisExpression".to_string(), json!(expression));
        }
        if let Some(bound_to) = &facts.objective_bound_to {
            attributes.insert("objectiveBoundTo".to_string(), json!(bound_to));
        }
    }
}

/// Projects [`crate::semantic::model::SourceTextFacts`] onto a boundary DTO's legacy `attributes`
/// JSON map, at the transport boundary only (see `AGENTS.md` "Boundary DTO modules remain
/// explicitly allowed"). `SemanticNode` itself no longer carries `doc`/`text`/`language`/`keyword`
/// as JSON (`planning/UNIFY_CACHE_PROGRESS.md` B9); this keeps presentation consumers that read the
/// projected DTO's `attributes` map (e.g. `general_view_fold`, `lsp_server`'s symbol projections)
/// unchanged. `keyword` here is the hover-only spelling; the separate semantic
/// `DeclaredSemanticFacts::modeled_keyword` fact is never projected through this map.
pub fn project_source_text_attributes(
    attributes: &mut HashMap<String, Value>,
    node: &crate::semantic::model::SemanticNode,
) {
    let text = &node.source_text;
    if let Some(value) = &text.doc {
        attributes.insert("doc".to_string(), json!(value));
    }
    if let Some(value) = &text.body {
        attributes.insert("body".to_string(), json!(value));
    }
    if let Some(value) = &text.text {
        attributes.insert("text".to_string(), json!(value));
    }
    if let Some(value) = &text.language {
        attributes.insert("language".to_string(), json!(value));
    }
    if let Some(value) = &text.keyword {
        attributes.insert("keyword".to_string(), json!(value));
    }
}

/// Projects the authored `redefines`/`subsetsFeature` display spelling from
/// [`crate::semantic::model::DeclaredRelationshipFacts::redefinition`]/`subsetting` onto a
/// boundary DTO's legacy `attributes` JSON map, at the transport boundary only (see `AGENTS.md`
/// "Boundary DTO modules remain explicitly allowed"). `general_view_fold` reads these through the
/// projected DTO rather than `SemanticNode.attributes` (`planning/UNIFY_CACHE_PROGRESS.md` B9); the
/// other relationship-target keys in the same family (`referencesFeature`, `crossesFeature`,
/// `specializes`) have no attribute-map reader left and are not projected here.
pub fn project_relationship_target_attributes(
    attributes: &mut HashMap<String, Value>,
    node: &crate::semantic::model::SemanticNode,
) {
    if let Some(target) = node.declared_facts.relationships.redefinition.first() {
        attributes.insert("redefines".to_string(), json!(target.reference));
    }
    if let Some(target) = node.declared_facts.relationships.subsetting.first() {
        attributes.insert("subsetsFeature".to_string(), json!(target.reference));
    }
}

/// Projects authored type-reference display text onto a boundary DTO's legacy `attributes` JSON
/// map, at the transport boundary only (see `AGENTS.md` "Boundary DTO modules remain explicitly
/// allowed"). `SemanticNode` itself no longer carries `attributeType`/`dataType`/`type`/`partType`/
/// `portType`/`refType`/`parameterType` as JSON: every one of those was a pure duplicate of the
/// first authored [`crate::semantic::model::DeclaredRelationshipFacts::typing`] target
/// (`planning/UNIFY_CACHE_PROGRESS.md` B9) -- populating all of the legacy key names with
/// that single value preserves every existing per-element-kind DTO reader
/// (`general_view_fold::detail_type_name`, `lsp_server`'s symbol projections) unchanged.
/// `payloadType`/`acceptType` are genuinely separate facts
/// ([`crate::semantic::model::DeclaredSemanticFacts::payload_type_reference`]/
/// `accept_type_reference`), not duplicates of `relationships.typing`.
pub fn project_type_reference_attributes(
    attributes: &mut HashMap<String, Value>,
    node: &crate::semantic::model::SemanticNode,
) {
    // An interface/connection end's declared type (`end name : Type;`) is kept off
    // `relationships.typing` entirely (see `DeclaredSemanticFacts::interface_end_type`'s doc
    // comment), so it is checked first and takes precedence over any (absent) typing target.
    let typing = node
        .declared_facts
        .declared_end_reference()
        .or_else(|| node.declared_facts.relationships.typing_display());
    if let Some(typing) = typing {
        attributes.insert("attributeType".to_string(), json!(typing));
        attributes.insert("partType".to_string(), json!(typing));
        attributes.insert("portType".to_string(), json!(typing));
        attributes.insert("refType".to_string(), json!(typing));
        attributes.insert("parameterType".to_string(), json!(typing));
    }
    // A named flow's own `FlowPayload` child feature has no separate
    // `payload_type_reference` fact -- its authored type is a pure duplicate of its `Typing`
    // edge/declared fact (unlike an action/transition `accept`/`send` clause's payload type,
    // which never gets a `Typing` edge), so it falls back to the same `typing` display text.
    if let Some(payload_type) = node
        .declared_facts
        .payload_type_reference
        .as_deref()
        .or(typing)
    {
        attributes.insert("payloadType".to_string(), json!(payload_type));
    }
    if let Some(accept_type) = &node.declared_facts.accept_type_reference {
        attributes.insert("acceptType".to_string(), json!(accept_type));
    }
}

pub fn canonical_general_view_graph(
    graph: &SysmlGraphDto,
    _include_all_roots: bool,
) -> SysmlGraphDto {
    let filtered_graph = fold_general_view_leaf_details_into_owners(graph);

    let mut node_by_id: HashMap<String, GraphNodeDto> = HashMap::new();
    for node in &filtered_graph.nodes {
        node_by_id
            .entry(node.id.clone())
            .or_insert_with(|| node.clone());
    }

    let mut edge_keys: HashSet<(String, String, String, Option<String>)> = HashSet::new();
    let mut out_edges: Vec<GraphEdgeDto> = Vec::new();
    for edge in &filtered_graph.edges {
        let key = (
            edge.source.clone(),
            edge.target.clone(),
            edge.rel_type.to_lowercase(),
            edge.name.clone(),
        );
        if edge_keys.insert(key) {
            out_edges.push(edge.clone());
        }
    }

    let mut out_nodes: Vec<GraphNodeDto> = node_by_id.into_values().collect();
    out_nodes.sort_by(|a, b| a.id.cmp(&b.id));
    out_edges.sort_by(|a, b| {
        (
            a.source.as_str(),
            a.target.as_str(),
            a.rel_type.to_lowercase(),
            a.name.as_deref().unwrap_or(""),
        )
            .cmp(&(
                b.source.as_str(),
                b.target.as_str(),
                b.rel_type.to_lowercase(),
                b.name.as_deref().unwrap_or(""),
            ))
    });
    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

mod general_view_fold;
mod workspace_dto;
pub(crate) use general_view_fold::*;
pub use workspace_dto::*;

#[cfg(test)]
mod tests;
