use super::*;

pub(crate) fn node_matches_all_filters(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
    filters: &[FilterExpr],
) -> bool {
    node_by_id
        .get(node_id)
        .is_some_and(|node| element_type_matches_all_filters(&node.element_type, filters))
}

pub(crate) fn node_matches_expose_filter(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
    filter: Option<&FilterExpr>,
) -> bool {
    filter.is_none_or(|expr| {
        node_by_id
            .get(node_id)
            .is_some_and(|node| element_type_matches_filter(&node.element_type, expr))
    })
}

/// Whether `element_type` (a semantic `element_kind` string) survives every filter.
pub fn element_type_matches_all_filters(element_type: &str, filters: &[FilterExpr]) -> bool {
    filters
        .iter()
        .all(|filter| element_type_matches_filter(element_type, filter))
}

pub(crate) fn element_type_matches_filter(element_type: &str, filter: &FilterExpr) -> bool {
    match filter {
        FilterExpr::Matches(qualified) => element_type_matches_kind(element_type, qualified),
        FilterExpr::Not(inner) => !element_type_matches_filter(element_type, inner),
        FilterExpr::And(left, right) => {
            element_type_matches_filter(element_type, left)
                && element_type_matches_filter(element_type, right)
        }
        FilterExpr::Or(left, right) => {
            element_type_matches_filter(element_type, left)
                || element_type_matches_filter(element_type, right)
        }
        FilterExpr::Unsupported(_) => false,
    }
}

pub(crate) fn element_type_matches_kind(element_type: &str, qualified: &str) -> bool {
    let wanted = normalize_kind_name(qualified);
    let actual = element_type.to_lowercase();
    actual == wanted
        || actual.contains(&wanted)
        || wanted.contains(actual.as_str())
        || actual == map_sysml_kind_alias(&wanted)
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
