use super::*;

pub(crate) fn infer_port_side(
    name: &str,
    direction: Option<&str>,
    _port_type: Option<&str>,
) -> Option<String> {
    let normalized_name = name.trim().to_lowercase();
    let normalized_direction = direction.unwrap_or("").trim().to_lowercase();

    match normalized_direction.as_str() {
        "in" | "input" => return Some("left".to_string()),
        "out" | "output" => return Some("right".to_string()),
        _ => {}
    }

    let tokens: Vec<&str> = normalized_name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    let trailing = tokens.last().copied().unwrap_or("");

    if trailing == "in"
        || trailing == "input"
        || normalized_name.ends_with("input")
        || normalized_name.ends_with("in")
    {
        return Some("left".to_string());
    }
    if trailing == "out"
        || trailing == "output"
        || normalized_name.ends_with("output")
        || normalized_name.ends_with("out")
    {
        return Some("right".to_string());
    }

    // Generic semantic hints for producer/provider style names.
    if normalized_name.contains("provide")
        || normalized_name.contains("supply")
        || normalized_name.contains("source")
        || normalized_name.starts_with("regulated")
    {
        return Some("right".to_string());
    }
    if normalized_name.contains("consume")
        || normalized_name.contains("demand")
        || normalized_name.contains("sink")
    {
        return Some("left".to_string());
    }

    None
}

pub(crate) fn endpoint_matches_root(endpoint: &str, root_prefix: &str) -> bool {
    endpoint == root_prefix || endpoint.starts_with(&format!("{root_prefix}."))
}

pub(crate) fn endpoint_matches_part(endpoint: &str, part_qn_dot: &str) -> bool {
    endpoint == part_qn_dot || endpoint.starts_with(&format!("{part_qn_dot}."))
}

pub(crate) fn resolve_owner_part_qn_for_endpoint(
    endpoint: &str,
    parts: &[IbdPartDto],
) -> Option<String> {
    let endpoint = qualified_name_to_dot(endpoint);
    parts
        .iter()
        .filter(|part| endpoint_matches_part(&endpoint, &part.qualified_name))
        .max_by_key(|part| part.qualified_name.len())
        .map(|part| part.qualified_name.clone())
}

pub(crate) fn resolve_port_id_for_endpoint(endpoint: &str, ports: &[IbdPortDto]) -> Option<String> {
    let endpoint_dot = qualified_name_to_dot(endpoint);
    ports
        .iter()
        .find(|port| {
            endpoint_dot == qualified_name_to_dot(&port.id)
                || endpoint_dot == qualified_name_to_dot(&port.port_id)
                || endpoint_dot == canonical_port_id(&port.parent_id, &port.name)
        })
        .map(|port| port.port_id.clone())
}
