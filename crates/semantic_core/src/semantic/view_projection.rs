//! Spec-driven view projection: effective view type determines node scope and edge filtering.
//!
//! Per SysML v2 §7.26.2 the pipeline is expose → filter → render. Projection runs after
//! expose/filter evaluation and before renderer-specific layout.
//!
//! Per §9.2.20.2.3, requirement traceability is not a separate standard view type: it is a
//! `GeneralView` specialization expressed through filters (RequirementUsage, VerificationCase,
//! SatisfyRequirementUsage, …). Traceability closure applies only for those filtered views.

use std::collections::{HashMap, HashSet};

use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic::explicit_views::{node_matches_all_filters, EvaluatedView, FilterExpr};

/// Which relationship edges belong in the projected graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgePredicate {
    /// Keep all edges whose endpoints are in the projected node set.
    All,
    /// Requirement/traceability views: derivation, satisfy, verify, subject.
    TraceabilityOnly,
}

/// Presentation hints for provisional standard-view renderers (grid layout, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionHints {
    pub grid_layout: Option<String>,
}

/// Result of projecting an evaluated view onto the semantic graph.
#[derive(Debug, Clone)]
pub struct ProjectedView {
    pub node_ids: HashSet<String>,
    pub edge_predicate: EdgePredicate,
    pub hints: ProjectionHints,
}

pub fn project_view(evaluated: &EvaluatedView, graph: &SysmlGraphDto) -> ProjectedView {
    let indexes = GraphIndexes::build(graph);
    let view_type = evaluated
        .effective_view_type
        .as_deref()
        .map(normalize_view_type)
        .unwrap_or_else(|| "generalview".to_string());

    let strategy = resolve_projection_strategy(&view_type, &evaluated.filters);
    let expanded_ids = match strategy.scope {
        ScopeStrategy::TraceabilityClosure => expand_traceability_scope(
            &evaluated.exposed_ids,
            graph,
            &evaluated.filters,
            &indexes.node_by_id,
        ),
        ScopeStrategy::Structural => expand_structural_scope(
            &evaluated.exposed_ids,
            &indexes.children_by_parent,
            &indexes.typing_targets,
            &indexes.node_by_id,
        ),
        ScopeStrategy::Descendants => {
            expand_descendants(&evaluated.exposed_ids, &indexes.children_by_parent)
        }
    };

    let filtered_ids: HashSet<String> = if strategy.apply_filters_after_expansion {
        expanded_ids
            .iter()
            .filter(|node_id| {
                node_matches_all_filters(node_id, &indexes.node_by_id, &evaluated.filters)
            })
            .cloned()
            .collect()
    } else {
        expanded_ids
    };

    let node_ids = if strategy.include_ancestors {
        with_ancestors(filtered_ids, &indexes.parent_by_id)
    } else {
        filtered_ids
    };

    ProjectedView {
        node_ids,
        edge_predicate: strategy.edge_predicate,
        hints: ProjectionHints {
            grid_layout: strategy.grid_layout.map(str::to_string),
        },
    }
}

pub fn project_ids_for_renderer(
    evaluated: &EvaluatedView,
    graph: &SysmlGraphDto,
) -> HashSet<String> {
    project_view(evaluated, graph).node_ids
}

pub fn apply_edge_predicate(graph: &SysmlGraphDto, predicate: EdgePredicate) -> SysmlGraphDto {
    let edges: Vec<GraphEdgeDto> = match predicate {
        EdgePredicate::All => graph.edges.clone(),
        EdgePredicate::TraceabilityOnly => graph
            .edges
            .iter()
            .filter(|edge| is_traceability_rel_type(&edge.rel_type))
            .cloned()
            .collect(),
    };
    SysmlGraphDto {
        nodes: graph.nodes.clone(),
        edges,
    }
}

#[derive(Debug, Clone, Copy)]
struct ProjectionStrategy {
    scope: ScopeStrategy,
    apply_filters_after_expansion: bool,
    include_ancestors: bool,
    edge_predicate: EdgePredicate,
    grid_layout: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
enum ScopeStrategy {
    TraceabilityClosure,
    Structural,
    Descendants,
}

fn resolve_projection_strategy(
    normalized_view_type: &str,
    filters: &[FilterExpr],
) -> ProjectionStrategy {
    if is_requirement_traceability_general_view(normalized_view_type, filters) {
        return traceability_projection_strategy();
    }
    projection_strategy(normalized_view_type)
}

fn traceability_projection_strategy() -> ProjectionStrategy {
    ProjectionStrategy {
        scope: ScopeStrategy::TraceabilityClosure,
        apply_filters_after_expansion: false,
        include_ancestors: false,
        edge_predicate: EdgePredicate::TraceabilityOnly,
        grid_layout: Some("traceability"),
    }
}

/// §9.2.20.2.3: requirement traceability is a filtered `GeneralView`, not a standard view type.
fn is_requirement_traceability_general_view(
    normalized_view_type: &str,
    filters: &[FilterExpr],
) -> bool {
    normalized_view_type == "generalview"
        && filters
            .iter()
            .any(filter_expr_targets_requirement_traceability)
}

fn filter_expr_targets_requirement_traceability(filter: &FilterExpr) -> bool {
    match filter {
        FilterExpr::Matches(qualified) => is_requirement_traceability_kind(qualified),
        FilterExpr::Not(inner) => filter_expr_targets_requirement_traceability(inner),
        FilterExpr::And(left, right) | FilterExpr::Or(left, right) => {
            filter_expr_targets_requirement_traceability(left)
                || filter_expr_targets_requirement_traceability(right)
        }
        FilterExpr::Unsupported(_) => false,
    }
}

fn is_requirement_traceability_kind(qualified: &str) -> bool {
    let kind = qualified
        .split("::")
        .last()
        .unwrap_or(qualified)
        .replace([' ', '_'], "")
        .to_lowercase();
    matches!(
        kind.as_str(),
        "requirementusage"
            | "requirementdefinition"
            | "requirementdef"
            | "verificationcase"
            | "verificationusage"
            | "satisfyrequirementusage"
            | "allocationusage"
            | "allocationdefinition"
    )
}

fn projection_strategy(normalized_view_type: &str) -> ProjectionStrategy {
    match normalized_view_type {
        "browserview" => ProjectionStrategy {
            scope: ScopeStrategy::Structural,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edge_predicate: EdgePredicate::All,
            grid_layout: None,
        },
        "actionflowview" | "statetransitionview" => ProjectionStrategy {
            scope: ScopeStrategy::Descendants,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edge_predicate: EdgePredicate::All,
            grid_layout: None,
        },
        "gridview" => ProjectionStrategy {
            scope: ScopeStrategy::Structural,
            apply_filters_after_expansion: true,
            include_ancestors: true,
            edge_predicate: EdgePredicate::All,
            grid_layout: None,
        },
        "geometryview" => ProjectionStrategy {
            scope: ScopeStrategy::Structural,
            apply_filters_after_expansion: true,
            include_ancestors: true,
            edge_predicate: EdgePredicate::All,
            grid_layout: None,
        },
        "interconnectionview" => ProjectionStrategy {
            scope: ScopeStrategy::Structural,
            apply_filters_after_expansion: true,
            include_ancestors: true,
            edge_predicate: EdgePredicate::All,
            grid_layout: None,
        },
        _ => ProjectionStrategy {
            scope: ScopeStrategy::Structural,
            apply_filters_after_expansion: true,
            include_ancestors: true,
            edge_predicate: EdgePredicate::All,
            grid_layout: None,
        },
    }
}

fn normalize_view_type(view_type: &str) -> String {
    view_type
        .split("::")
        .last()
        .unwrap_or(view_type)
        .replace([' ', '_'], "")
        .to_lowercase()
}

fn is_traceability_rel_type(rel_type: &str) -> bool {
    matches!(
        rel_type.to_lowercase().as_str(),
        "derivation" | "satisfy" | "verify" | "subject"
    )
}

fn expand_traceability_scope(
    seed_ids: &HashSet<String>,
    graph: &SysmlGraphDto,
    filters: &[FilterExpr],
    node_by_id: &HashMap<&str, &GraphNodeDto>,
) -> HashSet<String> {
    let mut visible: HashSet<String> = seed_ids
        .iter()
        .filter(|node_id| node_matches_all_filters(node_id, node_by_id, filters))
        .cloned()
        .collect();

    loop {
        let mut changed = false;
        for edge in &graph.edges {
            if !is_traceability_rel_type(&edge.rel_type) {
                continue;
            }
            if visible.contains(&edge.source)
                && node_matches_all_filters(edge.target.as_str(), node_by_id, filters)
                && visible.insert(edge.target.clone())
            {
                changed = true;
            }
            if visible.contains(&edge.target)
                && node_matches_all_filters(edge.source.as_str(), node_by_id, filters)
                && visible.insert(edge.source.clone())
            {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    visible
}

fn expand_descendants(
    root_ids: &HashSet<String>,
    children_by_parent: &HashMap<&str, Vec<&str>>,
) -> HashSet<String> {
    let mut expanded = root_ids.clone();
    let mut stack: Vec<String> = root_ids.iter().cloned().collect();
    while let Some(current) = stack.pop() {
        if let Some(children) = children_by_parent.get(current.as_str()) {
            for child in children {
                let child_string = (*child).to_string();
                if expanded.insert(child_string.clone()) {
                    stack.push(child_string);
                }
            }
        }
    }
    expanded
}

fn expand_structural_scope(
    root_ids: &HashSet<String>,
    children_by_parent: &HashMap<&str, Vec<&str>>,
    typing_targets: &HashMap<&str, Vec<&str>>,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
) -> HashSet<String> {
    let mut expanded = HashSet::new();
    let mut stack: Vec<String> = root_ids.iter().cloned().collect();

    while let Some(current) = stack.pop() {
        if !expanded.insert(current.clone()) {
            continue;
        }

        if let Some(children) = children_by_parent.get(current.as_str()) {
            for child in children {
                stack.push((*child).to_string());
            }
        }

        let follows_typing = node_by_id
            .get(current.as_str())
            .is_some_and(|node| is_part_like(&node.element_type) || is_action_like(&node.element_type));
        if follows_typing {
            if let Some(targets) = typing_targets.get(current.as_str()) {
                for target in targets {
                    stack.push((*target).to_string());
                }
            }
        }
    }

    expanded
}

fn is_part_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("part")
}

fn is_action_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("action")
}

fn with_ancestors(
    mut visible_ids: HashSet<String>,
    parent_by_id: &HashMap<&str, &str>,
) -> HashSet<String> {
    let mut stack: Vec<String> = visible_ids.iter().cloned().collect();
    while let Some(current) = stack.pop() {
        if let Some(parent) = parent_by_id.get(current.as_str()) {
            let parent_string = (*parent).to_string();
            if visible_ids.insert(parent_string.clone()) {
                stack.push(parent_string);
            }
        }
    }
    visible_ids
}

struct GraphIndexes<'a> {
    node_by_id: HashMap<&'a str, &'a GraphNodeDto>,
    parent_by_id: HashMap<&'a str, &'a str>,
    children_by_parent: HashMap<&'a str, Vec<&'a str>>,
    typing_targets: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> GraphIndexes<'a> {
    fn build(graph: &'a SysmlGraphDto) -> Self {
        let node_by_id: HashMap<&str, &GraphNodeDto> = graph
            .nodes
            .iter()
            .map(|node| (node.id.as_str(), node))
            .collect();
        let parent_by_id: HashMap<&str, &str> = graph
            .nodes
            .iter()
            .filter_map(|node| {
                node.parent_id
                    .as_deref()
                    .map(|parent| (node.id.as_str(), parent))
            })
            .collect();
        let children_by_parent: HashMap<&str, Vec<&str>> = {
            let mut map = HashMap::new();
            for node in &graph.nodes {
                if let Some(parent_id) = node.parent_id.as_deref() {
                    map.entry(parent_id)
                        .or_insert_with(Vec::new)
                        .push(node.id.as_str());
                }
            }
            map
        };
        let typing_targets: HashMap<&str, Vec<&str>> = {
            let mut map = HashMap::new();
            for edge in &graph.edges {
                let rel_type = edge.rel_type.to_lowercase();
                if rel_type == "typing" || rel_type == "specializes" {
                    map.entry(edge.source.as_str())
                        .or_insert_with(Vec::new)
                        .push(edge.target.as_str());
                }
            }
            map
        };
        Self {
            node_by_id,
            parent_by_id,
            children_by_parent,
            typing_targets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::dto::{PositionDto, RangeDto};
    use std::collections::HashMap;

    fn zero_range() -> RangeDto {
        RangeDto {
            start: PositionDto {
                line: 0,
                character: 0,
            },
            end: PositionDto {
                line: 0,
                character: 0,
            },
        }
    }

    #[test]
    fn traceability_closure_is_stable_and_respects_filters() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::need".to_string(),
                    element_type: "requirement".to_string(),
                    name: "need".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::req".to_string(),
                    element_type: "requirement".to_string(),
                    name: "req".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::part".to_string(),
                    element_type: "part".to_string(),
                    name: "part".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "Pkg::need".to_string(),
                    target: "Pkg::req".to_string(),
                    rel_type: "derivation".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "Pkg::part".to_string(),
                    target: "Pkg::req".to_string(),
                    rel_type: "satisfy".to_string(),
                    name: None,
                },
            ],
        };
        let evaluated = EvaluatedView {
            id: "Pkg::trace".to_string(),
            name: "trace".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::from(["Pkg::need".to_string()]),
            conforms_to: Vec::new(),
            filters: vec![FilterExpr::Matches("@SysML::RequirementUsage".to_string())],
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_view(&evaluated, &graph);
        assert!(projected.node_ids.contains("Pkg::need"));
        assert!(projected.node_ids.contains("Pkg::req"));
        assert!(!projected.node_ids.contains("Pkg::part"));
        assert_eq!(projected.edge_predicate, EdgePredicate::TraceabilityOnly);
        assert_eq!(
            projected.hints.grid_layout.as_deref(),
            Some("traceability")
        );
    }

    #[test]
    fn part_usage_general_view_uses_structural_projection() {
        let evaluated = EvaluatedView {
            id: "Pkg::structure".to_string(),
            name: "structure".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::from(["Pkg::robot".to_string()]),
            conforms_to: Vec::new(),
            filters: vec![FilterExpr::Matches("@SysML::PartUsage".to_string())],
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };
        let graph = SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        };
        let projected = project_view(&evaluated, &graph);
        assert_eq!(projected.edge_predicate, EdgePredicate::All);
        assert!(projected.hints.grid_layout.is_none());
    }

    #[test]
    fn edge_predicate_filters_non_traceability_edges() {
        let graph = SysmlGraphDto {
            nodes: vec![],
            edges: vec![
                GraphEdgeDto {
                    source: "a".to_string(),
                    target: "b".to_string(),
                    rel_type: "satisfy".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "a".to_string(),
                    target: "c".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
            ],
        };
        let filtered = apply_edge_predicate(&graph, EdgePredicate::TraceabilityOnly);
        assert_eq!(filtered.edges.len(), 1);
        assert_eq!(filtered.edges[0].rel_type, "satisfy");
    }
}
