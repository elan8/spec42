//! Spec-driven view projection: effective view type determines node scope and edge filtering.
//!
//! Per SysML v2 §7.26.2 the pipeline is expose → filter → render. Projection runs after
//! expose/filter evaluation and before renderer-specific layout.
//!
//! Per §9.2.20.2.3, `GeneralView` remains generic: exact exposure controls its scope and its
//! declared filters select both nodes and relationships. Standard view definitions with explicit
//! traversal semantics (for example `BrowserView`) apply those semantics separately.

use std::collections::{HashMap, HashSet};

use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic::element_kind_classify::is_action_like;
use crate::semantic::explicit_views::{node_matches_all_filters, EvaluatedView, FilterExpr};
use crate::semantic::kinds::is_part_like_str as is_part_like;
use crate::semantic::standard_view_defaults::grid_subtype_for_filters;

/// Which relationship edges belong in the projected graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgePredicate {
    /// Keep all edges whose endpoints are in the projected node set.
    All,
    /// Keep relationships selected by the view's own filter expressions.
    MatchingFilters(Vec<FilterExpr>),
}

/// Presentation hints for standard-view renderers (grid layout, browser tree, geometry params).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionHints {
    pub grid_layout: Option<String>,
    pub grid_subtype: Option<String>,
    pub browser_layout: Option<String>,
    pub tree_roots: Vec<String>,
    pub geometry_mode: Option<String>,
    pub geometry_projection: Option<String>,
}

/// Result of projecting an evaluated view onto the semantic graph.
#[derive(Debug, Clone)]
pub struct ProjectedView {
    pub node_ids: HashSet<String>,
    /// Node ids after standard-view scope handling but before the view's filters are applied.
    /// For `GeneralView`, this is exactly the exposed scope; other standard views may define
    /// traversal semantics such as hierarchical membership or behavioral descendants.
    pub pre_filter_node_ids: HashSet<String>,
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

    let strategy = projection_strategy(&view_type);
    let expanded_ids = match strategy.scope {
        ScopeStrategy::Exposed => evaluated.exposed_ids.clone(),
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

    let pre_filter_node_ids = expanded_ids.clone();

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
        pre_filter_node_ids,
        edge_predicate: if strategy.edges_match_filters && !evaluated.filters.is_empty() {
            EdgePredicate::MatchingFilters(evaluated.filters.clone())
        } else {
            EdgePredicate::All
        },
        hints: ProjectionHints {
            grid_layout: strategy.grid_layout.map(str::to_string),
            grid_subtype: grid_subtype_for_filters(&evaluated.filters)
                .map(str::to_string)
                .or_else(|| strategy.grid_subtype.map(str::to_string)),
            browser_layout: strategy.browser_layout.map(str::to_string),
            tree_roots: evaluated.exposed_ids.iter().cloned().collect(),
            geometry_mode: strategy.geometry_mode.map(str::to_string),
            geometry_projection: strategy.geometry_projection.map(str::to_string),
        },
    }
}

pub fn project_ids_for_renderer(
    evaluated: &EvaluatedView,
    graph: &SysmlGraphDto,
) -> HashSet<String> {
    project_view(evaluated, graph).node_ids
}

pub fn apply_edge_predicate(graph: &SysmlGraphDto, predicate: &EdgePredicate) -> SysmlGraphDto {
    let edges: Vec<GraphEdgeDto> = match predicate {
        EdgePredicate::All => graph.edges.clone(),
        EdgePredicate::MatchingFilters(filters) => graph
            .edges
            .iter()
            .filter(|edge| edge_matches_all_filters(&edge.rel_type, filters))
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
    edges_match_filters: bool,
    grid_layout: Option<&'static str>,
    grid_subtype: Option<&'static str>,
    browser_layout: Option<&'static str>,
    geometry_mode: Option<&'static str>,
    geometry_projection: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScopeStrategy {
    Exposed,
    Structural,
    Descendants,
}

fn projection_strategy(normalized_view_type: &str) -> ProjectionStrategy {
    match normalized_view_type {
        "browserview" => ProjectionStrategy {
            // §9.2.20.2.2 defines a membership tree starting at the exposed root.
            // Following FeatureTyping here incorrectly splices the contents of a
            // definition into the usage's membership hierarchy.
            scope: ScopeStrategy::Descendants,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edges_match_filters: false,
            grid_layout: None,
            grid_subtype: None,
            browser_layout: Some("hierarchy"),
            geometry_mode: None,
            geometry_projection: None,
        },
        "actionflowview" | "sequenceview" | "statetransitionview" => ProjectionStrategy {
            scope: ScopeStrategy::Descendants,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edges_match_filters: false,
            grid_layout: None,
            grid_subtype: None,
            browser_layout: None,
            geometry_mode: None,
            geometry_projection: None,
        },
        "gridview" => ProjectionStrategy {
            // §9.2.20.2.5 says that the exposed elements and relationships are
            // arranged in a grid; it does not define implicit structural closure.
            scope: ScopeStrategy::Exposed,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edges_match_filters: true,
            grid_layout: None,
            grid_subtype: Some("element_table"),
            browser_layout: None,
            geometry_mode: None,
            geometry_projection: None,
        },
        "geometryview" => ProjectionStrategy {
            // GeometryView visualizes exposed spatial items. Spatial containment
            // and typing are model semantics, not permission to add unexposed nodes.
            scope: ScopeStrategy::Exposed,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edges_match_filters: true,
            grid_layout: None,
            grid_subtype: None,
            browser_layout: None,
            geometry_mode: Some("2d"),
            geometry_projection: Some("orthographic"),
        },
        "interconnectionview" => ProjectionStrategy {
            scope: ScopeStrategy::Structural,
            apply_filters_after_expansion: true,
            include_ancestors: true,
            edges_match_filters: false,
            grid_layout: None,
            grid_subtype: None,
            browser_layout: None,
            geometry_mode: None,
            geometry_projection: None,
        },
        _ => ProjectionStrategy {
            scope: ScopeStrategy::Exposed,
            apply_filters_after_expansion: true,
            include_ancestors: false,
            edges_match_filters: true,
            grid_layout: None,
            grid_subtype: None,
            browser_layout: None,
            geometry_mode: None,
            geometry_projection: None,
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

fn edge_matches_all_filters(rel_type: &str, filters: &[FilterExpr]) -> bool {
    filters
        .iter()
        .all(|filter| edge_matches_filter(filter, rel_type))
}

fn edge_matches_filter(filter: &FilterExpr, rel_type: &str) -> bool {
    match filter {
        FilterExpr::Matches(qualified) => edge_matches_kind(rel_type, qualified),
        FilterExpr::Not(inner) => !edge_matches_filter(inner, rel_type),
        FilterExpr::And(left, right) => {
            edge_matches_filter(left, rel_type) && edge_matches_filter(right, rel_type)
        }
        FilterExpr::Or(left, right) => {
            edge_matches_filter(left, rel_type) || edge_matches_filter(right, rel_type)
        }
        FilterExpr::Unsupported(_) => false,
    }
}

fn edge_matches_kind(rel_type: &str, qualified: &str) -> bool {
    let wanted = qualified
        .split("::")
        .last()
        .unwrap_or(qualified)
        .replace([' ', '_'], "")
        .to_lowercase();
    let actual = rel_type.replace([' ', '_'], "").to_lowercase();
    match actual.as_str() {
        "contains" => matches!(
            wanted.as_str(),
            "owningmembership" | "membership" | "containment" | "packagecontainment"
        ),
        "typing" => matches!(wanted.as_str(), "featuretyping" | "typing"),
        "specializes" | "specialization" => matches!(
            wanted.as_str(),
            "specialization" | "subclassification" | "subsetting" | "redefinition"
        ),
        "connection" => matches!(wanted.as_str(), "connection" | "connectionusage"),
        "allocate" | "allocation" => {
            matches!(wanted.as_str(), "allocation" | "allocationusage")
        }
        "derivation" => matches!(wanted.as_str(), "derivation" | "dependency"),
        "satisfy" => matches!(wanted.as_str(), "satisfy" | "satisfyrequirementusage"),
        "verify" => matches!(
            wanted.as_str(),
            "verify" | "verificationusage" | "verifyrequirementusage"
        ),
        "subject" => matches!(wanted.as_str(), "subject" | "subjectmembership"),
        _ => wanted == actual || wanted == "relationship",
    }
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

        let follows_typing = node_by_id.get(current.as_str()).is_some_and(|node| {
            is_part_like(&node.element_type) || is_action_like(&node.element_type)
        });
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
                if rel_type == "typing" {
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
    fn standard_view_scope_strategies_follow_the_spec_descriptions() {
        assert_eq!(
            projection_strategy("browserview").scope,
            ScopeStrategy::Descendants
        );
        assert_eq!(
            projection_strategy("sequenceview").scope,
            ScopeStrategy::Descendants
        );
        assert_eq!(
            projection_strategy("gridview").scope,
            ScopeStrategy::Exposed
        );
        assert_eq!(
            projection_strategy("geometryview").scope,
            ScopeStrategy::Exposed
        );
        assert_eq!(
            projection_strategy("interconnectionview").scope,
            ScopeStrategy::Structural
        );
    }

    #[test]
    fn general_view_does_not_infer_traceability_closure() {
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
        assert!(!projected.node_ids.contains("Pkg::req"));
        assert!(!projected.node_ids.contains("Pkg::part"));
        assert_eq!(
            projected.edge_predicate,
            EdgePredicate::MatchingFilters(evaluated.filters.clone())
        );
        assert!(projected.hints.grid_layout.is_none());
    }

    #[test]
    fn general_view_uses_exposed_scope_and_its_declared_filters() {
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
        assert_eq!(
            projected.edge_predicate,
            EdgePredicate::MatchingFilters(evaluated.filters.clone())
        );
        assert!(projected.hints.grid_layout.is_none());
    }

    #[test]
    fn part_structure_projection_keeps_explicit_parts_shallow() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::robot".to_string(),
                    element_type: "part".to_string(),
                    name: "robot".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::robot::mass".to_string(),
                    element_type: "attribute".to_string(),
                    name: "mass".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::robot".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![],
        };
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

        let projected = project_view(&evaluated, &graph);
        assert!(projected.node_ids.contains("Pkg::robot"));
        assert!(
            !projected.node_ids.contains("Pkg::robot::mass"),
            "attribute should not become its own diagram box under a PartUsage filter"
        );
        assert!(
            !projected.pre_filter_node_ids.contains("Pkg::robot::mass"),
            "a focused part structure should not populate attribute compartments recursively"
        );
        assert!(projected.pre_filter_node_ids.contains("Pkg::robot"));
    }

    #[test]
    fn general_view_contains_only_explicitly_exposed_definitions() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::robot".to_string(),
                    element_type: "part".to_string(),
                    name: "robot".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::Robot".to_string(),
                    element_type: "part def".to_string(),
                    name: "Robot".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::RequirementSubject".to_string(),
                    element_type: "part def".to_string(),
                    name: "RequirementSubject".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "Pkg::robot".to_string(),
                    target: "Pkg::Robot".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "Pkg::Robot".to_string(),
                    target: "Pkg::RequirementSubject".to_string(),
                    rel_type: "specializes".to_string(),
                    name: None,
                },
            ],
        };
        let evaluated = EvaluatedView {
            id: "Pkg::structure".to_string(),
            name: "structure".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::from(["Pkg::robot".to_string(), "Pkg::Robot".to_string()]),
            conforms_to: Vec::new(),
            filters: vec![FilterExpr::Or(
                Box::new(FilterExpr::Matches("@SysML::PartUsage".to_string())),
                Box::new(FilterExpr::Matches("@SysML::PartDefinition".to_string())),
            )],
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_view(&evaluated, &graph);
        assert!(projected.node_ids.contains("Pkg::robot"));
        assert!(projected.node_ids.contains("Pkg::Robot"));
        assert!(
            !projected.node_ids.contains("Pkg::RequirementSubject"),
            "a supertype provides semantics but is not a decomposition child"
        );
    }

    #[test]
    fn edge_predicate_matches_relationship_kinds_declared_by_filters() {
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
                GraphEdgeDto {
                    source: "a".to_string(),
                    target: "d".to_string(),
                    rel_type: "allocate".to_string(),
                    name: None,
                },
            ],
        };
        let predicate = EdgePredicate::MatchingFilters(vec![FilterExpr::Or(
            Box::new(FilterExpr::Matches(
                "@SysML::SatisfyRequirementUsage".to_string(),
            )),
            Box::new(FilterExpr::Matches("@SysML::AllocationUsage".to_string())),
        )]);
        let filtered = apply_edge_predicate(&graph, &predicate);
        assert_eq!(filtered.edges.len(), 2);
        assert_eq!(filtered.edges[0].rel_type, "satisfy");
        assert_eq!(filtered.edges[1].rel_type, "allocate");
    }

    #[test]
    fn product_structure_filters_keep_membership_and_typing_only() {
        let graph = SysmlGraphDto {
            nodes: vec![],
            edges: vec![
                GraphEdgeDto {
                    source: "robot".to_string(),
                    target: "base".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "base".to_string(),
                    target: "Base".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "base".to_string(),
                    target: "top".to_string(),
                    rel_type: "connection".to_string(),
                    name: None,
                },
            ],
        };
        let predicate = EdgePredicate::MatchingFilters(vec![FilterExpr::Or(
            Box::new(FilterExpr::Matches("@KerML::OwningMembership".to_string())),
            Box::new(FilterExpr::Matches("@KerML::FeatureTyping".to_string())),
        )]);

        let filtered = apply_edge_predicate(&graph, &predicate);
        assert_eq!(filtered.edges.len(), 2);
        assert_eq!(filtered.edges[0].rel_type, "contains");
        assert_eq!(filtered.edges[1].rel_type, "typing");
    }
}
