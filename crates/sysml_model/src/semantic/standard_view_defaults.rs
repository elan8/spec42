//! Default filter expressions for standard view types when a view usage omits explicit filters.
//!
//! Per §9.2.20, `StandardViewDefinitions` in the Systems Model Library may supply filter
//! conditions; OMG issue SYSML2-25 leaves those incomplete. Spec42 tries stdlib introspection
//! first and otherwise preserves the usage's unfiltered exposed set. Descriptive lists in
//! §9.2.20.2 are not executable default filter expressions.

use crate::semantic::explicit_views::FilterExpr;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::ElementKind;
use crate::semantic::standard_views::normalize_view_type_name;

/// Merge usage-level filters with defaults for standard view types.
///
/// Defaults apply only when the view usage itself has no `filter` members (definition-level
/// and expose filters are merged separately in [`evaluate_views`](super::explicit_views::evaluate_views)).
pub fn merge_usage_default_filters(
    effective_view_type: &str,
    usage_filters: &[FilterExpr],
    semantic_graph: Option<&SemanticGraph>,
) -> Vec<FilterExpr> {
    if !usage_filters.is_empty() {
        return usage_filters.to_vec();
    }

    let normalized = normalize_view_type_name(effective_view_type);
    if let Some(graph) = semantic_graph {
        if let Some(filters) = filters_from_stdlib_view_def(graph, &normalized) {
            return filters;
        }
    }
    fallback_default_filters(&normalized)
}

fn filters_from_stdlib_view_def(
    graph: &SemanticGraph,
    normalized_view_type: &str,
) -> Option<Vec<FilterExpr>> {
    let target_name = match normalized_view_type {
        "browserview" => "BrowserView",
        "gridview" => "GridView",
        "geometryview" => "GeometryView",
        _ => return None,
    };

    for node in graph.nodes_named(target_name) {
        if node.element_kind != ElementKind::ViewDef {
            continue;
        }
        if !node.id.qualified_name.contains("StandardViewDefinitions") {
            continue;
        }
        if let Some(filters) = parse_filter_attributes(&node.attributes) {
            if !filters.is_empty() {
                return Some(filters);
            }
        }
    }
    None
}

fn parse_filter_attributes(
    attributes: &std::collections::HashMap<String, serde_json::Value>,
) -> Option<Vec<FilterExpr>> {
    let raw = attributes.get("filters")?.as_array()?;
    let mut filters = Vec::new();
    for entry in raw {
        if let Some(text) = entry.as_str() {
            filters.push(crate::semantic::explicit_views::parse_filter_text(text));
        }
    }
    if filters.is_empty() {
        None
    } else {
        Some(filters)
    }
}

fn fallback_default_filters(normalized_view_type: &str) -> Vec<FilterExpr> {
    match normalized_view_type {
        // The standard definitions currently provide no normative executable
        // fallback filters for these views.
        "browserview" | "gridview" | "geometryview" => Vec::new(),
        _ => Vec::new(),
    }
}

pub(crate) fn grid_subtype_for_filters(filters: &[FilterExpr]) -> Option<&'static str> {
    if filters.iter().any(filter_expr_targets_relationship_matrix) {
        return Some("relationship_matrix");
    }
    None
}

fn filter_expr_targets_relationship_matrix(filter: &FilterExpr) -> bool {
    match filter {
        FilterExpr::Matches(qualified) => is_relationship_matrix_kind(qualified),
        FilterExpr::Not(inner) => filter_expr_targets_relationship_matrix(inner),
        FilterExpr::And(left, right) | FilterExpr::Or(left, right) => {
            filter_expr_targets_relationship_matrix(left)
                || filter_expr_targets_relationship_matrix(right)
        }
        FilterExpr::Unsupported(_) => false,
    }
}

fn is_relationship_matrix_kind(qualified: &str) -> bool {
    let kind = qualified
        .split("::")
        .last()
        .unwrap_or(qualified)
        .replace([' ', '_'], "")
        .to_lowercase();
    matches!(
        kind.as_str(),
        "relationship"
            | "connectionusage"
            | "connectiondefinition"
            | "allocationusage"
            | "allocationdefinition"
            | "satisfyrequirementusage"
            | "dependency"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_view_does_not_invent_a_spatial_kind_filter() {
        assert!(fallback_default_filters("geometryview").is_empty());
    }

    #[test]
    fn browser_view_has_no_default_kind_filter() {
        assert!(fallback_default_filters("browserview").is_empty());
    }

    #[test]
    fn relationship_filter_selects_matrix_subtype() {
        let filters = vec![FilterExpr::Matches("@SysML::ConnectionUsage".to_string())];
        assert_eq!(
            grid_subtype_for_filters(&filters),
            Some("relationship_matrix")
        );
    }
}
