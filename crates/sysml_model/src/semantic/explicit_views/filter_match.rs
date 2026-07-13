use super::*;

pub(crate) fn node_matches_all_filters(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
    filters: &[FilterExpr],
) -> bool {
    filters
        .iter()
        .all(|filter| match_filter_expr(filter, node_id, node_by_id))
}

pub(crate) fn node_matches_expose_filter(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
    filter: Option<&FilterExpr>,
) -> bool {
    filter.is_none_or(|expr| match_filter_expr(expr, node_id, node_by_id))
}

pub(crate) fn match_filter_expr(
    filter: &FilterExpr,
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
) -> bool {
    match filter {
        FilterExpr::Matches(qualified) => node_matches_kind(node_id, qualified, node_by_id),
        FilterExpr::Not(inner) => !match_filter_expr(inner, node_id, node_by_id),
        FilterExpr::And(left, right) => {
            match_filter_expr(left, node_id, node_by_id)
                && match_filter_expr(right, node_id, node_by_id)
        }
        FilterExpr::Or(left, right) => {
            match_filter_expr(left, node_id, node_by_id)
                || match_filter_expr(right, node_id, node_by_id)
        }
        FilterExpr::Unsupported(_) => false,
    }
}

pub(crate) fn node_matches_kind(
    node_id: &str,
    qualified: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
) -> bool {
    let wanted = normalize_kind_name(qualified);
    node_by_id.get(node_id).is_some_and(|node| {
        let actual = node.element_type.to_lowercase();
        actual == wanted
            || actual.contains(&wanted)
            || wanted.contains(actual.as_str())
            || actual == map_sysml_kind_alias(&wanted)
    })
}

pub(crate) fn map_sysml_kind_alias(wanted: &str) -> String {
    match wanted {
        "partusage" => "part".to_string(),
        "partdefinition" | "partdef" => "part def".to_string(),
        "connectionusage" => "connection".to_string(),
        "actionusage" => "action".to_string(),
        "actiondefinition" | "actiondef" => "action def".to_string(),
        "portusage" => "port".to_string(),
        "portdefinition" | "portdef" => "port def".to_string(),
        "connectiondefinition" | "connectiondef" => "connection def".to_string(),
        "stateusage" => "state".to_string(),
        "statedefinition" | "statedef" => "state def".to_string(),
        "metadatausage" => "metadata usage".to_string(),
        "requirementusage" => "requirement".to_string(),
        "verificationcase" => "verification".to_string(),
        "analysiscase" => "analysis".to_string(),
        "package" => "package".to_string(),
        other => other.to_string(),
    }
}

pub(crate) fn normalize_kind_name(value: &str) -> String {
    normalize_path(value)
        .split("::")
        .last()
        .unwrap_or(value)
        .replace([' ', '_'], "")
        .to_lowercase()
}

pub(crate) fn normalize_path(value: &str) -> String {
    value
        .replace('.', "::")
        .trim()
        .trim_matches('\'')
        .to_string()
}
