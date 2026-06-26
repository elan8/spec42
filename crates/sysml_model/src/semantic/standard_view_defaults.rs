//! Default filter expressions for standard view types when a view usage omits explicit filters.
//!
//! Per §9.2.20, `StandardViewDefinitions` in the Systems Model Library may supply filter
//! conditions; OMG issue SYSML2-25 leaves those incomplete. Spec42 tries stdlib introspection
//! first, then applies documented fallbacks from §9.2.20.2.2–2.5.

use crate::semantic::explicit_views::FilterExpr;
use crate::semantic::graph::SemanticGraph;
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
        if node.element_kind != "view def" {
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
        // §9.2.20.2.4 — spatial items, coordinate frames, quantity features on spatial items.
        "geometryview" => geometry_spatial_default_filter(),
        // §9.2.20.2.2 / 2.5 — membership tree / element table: no kind restriction by default.
        "browserview" | "gridview" => Vec::new(),
        _ => Vec::new(),
    }
}

fn geometry_spatial_default_filter() -> Vec<FilterExpr> {
    let kinds = [
        "@SysML::PartUsage",
        "@SysML::PortUsage",
        "@SysML::ItemUsage",
        "@SysML::ConnectionUsage",
        "@SysML::Shape",
        "@SysML::CoordinateFrame",
    ];
    kinds
        .iter()
        .rev()
        .fold(None, |acc: Option<FilterExpr>, kind| {
            let expr = FilterExpr::Matches((*kind).to_string());
            match acc {
                None => Some(expr),
                Some(right) => Some(FilterExpr::Or(Box::new(expr), Box::new(right))),
            }
        })
        .into_iter()
        .collect()
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
    fn geometry_view_gets_spatial_default_filter() {
        let filters = fallback_default_filters("geometryview");
        assert_eq!(filters.len(), 1);
        assert!(matches!(&filters[0], FilterExpr::Or(_, _)));
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
