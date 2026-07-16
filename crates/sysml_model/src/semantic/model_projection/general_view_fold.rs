use super::*;

pub(crate) fn fold_general_view_leaf_details_into_owners(graph: &SysmlGraphDto) -> SysmlGraphDto {
    let node_by_id: HashMap<&str, &GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let children_by_parent: HashMap<&str, Vec<&GraphNodeDto>> = {
        let mut map: HashMap<&str, Vec<&GraphNodeDto>> = HashMap::new();
        for node in &graph.nodes {
            if let Some(parent_id) = node.parent_id.as_deref() {
                map.entry(parent_id).or_default().push(node);
            }
        }
        map
    };
    let typing_targets: HashMap<&str, &str> = graph
        .edges
        .iter()
        .filter(|edge| edge.rel_type.eq_ignore_ascii_case("typing"))
        .map(|edge| (edge.source.as_str(), edge.target.as_str()))
        .collect();
    let specialization_targets: HashMap<&str, Vec<&str>> = {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for edge in &graph.edges {
            if edge.rel_type.eq_ignore_ascii_case("specializes") {
                map.entry(edge.source.as_str())
                    .or_default()
                    .push(edge.target.as_str());
            }
        }
        map
    };

    let detail_ids: HashSet<&str> = graph
        .nodes
        .iter()
        .filter(|node| is_general_view_inline_detail(node))
        .map(|node| node.id.as_str())
        .collect();

    let has_requirement_inline_content = graph.nodes.iter().any(|node| {
        node.attributes
            .get("requirementConstraints")
            .and_then(|value| value.as_array())
            .is_some_and(|lines| !lines.is_empty())
    });

    if detail_ids.is_empty() && !has_requirement_inline_content {
        return graph.clone();
    }

    let mut out_nodes: Vec<GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| !detail_ids.contains(node.id.as_str()))
        .cloned()
        .collect();
    for node in &mut out_nodes {
        let direct_details = collect_direct_general_view_details(
            node.id.as_str(),
            &children_by_parent,
            &node_by_id,
            &typing_targets,
            node.attributes
                .get("requirementConstraints")
                .and_then(|value| value.as_array()),
        );
        let inherited_details = collect_inherited_general_view_details(
            node.id.as_str(),
            &children_by_parent,
            &node_by_id,
            &typing_targets,
            &specialization_targets,
            &direct_details,
        );

        insert_detail_items(
            &mut node.attributes,
            "generalViewDirectAttributes",
            direct_details.attributes,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewDirectParts",
            direct_details.parts,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewDirectPorts",
            direct_details.ports,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewInheritedAttributes",
            inherited_details.attributes,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewInheritedParts",
            inherited_details.parts,
        );
    }

    let out_edges: Vec<GraphEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| {
            !detail_ids.contains(edge.source.as_str()) && !detail_ids.contains(edge.target.as_str())
        })
        .cloned()
        .collect();

    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

pub(crate) fn is_general_view_inline_detail(node: &GraphNodeDto) -> bool {
    let lower = node.element_type.to_lowercase();
    is_port_like(&lower)
        || is_attribute_like(&lower)
        || is_parameter_like(&lower)
        || lower == "require constraint"
        // Anonymous redefinition stubs like `part :>> engines[5] = (...)` should not
        // surface as standalone structure nodes in General View.
        || is_anonymous_redefinition_stub(node)
}

pub(crate) fn is_anonymous_redefinition_stub(node: &GraphNodeDto) -> bool {
    node.name.trim().is_empty() && node.attributes.contains_key("redefines")
}

#[derive(Default)]
pub(crate) struct GeneralViewDetails {
    attributes: Vec<Value>,
    parts: Vec<Value>,
    ports: Vec<Value>,
    attribute_names: HashSet<String>,
    part_names: HashSet<String>,
}

pub(crate) fn collect_direct_general_view_details(
    owner_id: &str,
    children_by_parent: &HashMap<&str, Vec<&GraphNodeDto>>,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
    requirement_constraints: Option<&Vec<Value>>,
) -> GeneralViewDetails {
    let mut details = GeneralViewDetails::default();

    if let Some(children) = children_by_parent.get(owner_id) {
        for child in children {
            if is_port_like(&child.element_type) {
                if let Some(item) =
                    build_general_view_detail_item(child, node_by_id, typing_targets, None)
                {
                    details.ports.push(item);
                }
                continue;
            }
            if is_attribute_like(&child.element_type) {
                if let Some(item) =
                    build_general_view_detail_item(child, node_by_id, typing_targets, None)
                {
                    if let Some(name) = item.get("name").and_then(|value| value.as_str()) {
                        details.attribute_names.insert(name.to_lowercase());
                    }
                    details.attributes.push(item);
                }
                continue;
            }
            if is_part_compartment_item(child) {
                if let Some(item) =
                    build_general_view_detail_item(child, node_by_id, typing_targets, None)
                {
                    if let Some(name) = item.get("name").and_then(|value| value.as_str()) {
                        details.part_names.insert(name.to_lowercase());
                    }
                    details.parts.push(item);
                }
            }
        }
    }

    if let Some(lines) = requirement_constraints {
        for line in lines {
            if let Some(display_text) = line.as_str().map(|value| value.trim().to_string()) {
                if display_text.is_empty() {
                    continue;
                }
                details.attributes.push(json!({
                    "name": display_text,
                    "displayText": display_text,
                }));
            }
        }
    }

    details
}

pub(crate) fn collect_inherited_general_view_details(
    owner_id: &str,
    children_by_parent: &HashMap<&str, Vec<&GraphNodeDto>>,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
    specialization_targets: &HashMap<&str, Vec<&str>>,
    direct_details: &GeneralViewDetails,
) -> GeneralViewDetails {
    let mut details = GeneralViewDetails::default();
    let mut seen_ancestors: HashSet<&str> = HashSet::new();
    let mut queue: Vec<&str> = specialization_targets
        .get(owner_id)
        .cloned()
        .unwrap_or_default();

    while let Some(ancestor_id) = queue.first().copied() {
        queue.remove(0);
        if !seen_ancestors.insert(ancestor_id) {
            continue;
        }
        if let Some(next_targets) = specialization_targets.get(ancestor_id) {
            queue.extend(next_targets.iter().copied());
        }
        let declared_in = node_by_id
            .get(ancestor_id)
            .map(|node| node.name.clone())
            .filter(|name| !name.trim().is_empty());
        let Some(children) = children_by_parent.get(ancestor_id) else {
            continue;
        };
        for child in children {
            if is_attribute_like(&child.element_type) {
                if let Some(item) = build_general_view_detail_item(
                    child,
                    node_by_id,
                    typing_targets,
                    declared_in.clone(),
                ) {
                    let Some(name) = item.get("name").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    let normalized = name.to_lowercase();
                    if direct_details.attribute_names.contains(&normalized)
                        || !details.attribute_names.insert(normalized)
                    {
                        continue;
                    }
                    details.attributes.push(item);
                }
                continue;
            }
            if is_part_compartment_item(child) {
                if let Some(item) = build_general_view_detail_item(
                    child,
                    node_by_id,
                    typing_targets,
                    declared_in.clone(),
                ) {
                    let Some(name) = item.get("name").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    let normalized = name.to_lowercase();
                    if direct_details.part_names.contains(&normalized)
                        || !details.part_names.insert(normalized)
                    {
                        continue;
                    }
                    details.parts.push(item);
                }
            }
        }
    }

    details
}

pub(crate) fn is_part_compartment_item(node: &GraphNodeDto) -> bool {
    let lower = node.element_type.to_lowercase();
    if !lower.contains("part") || is_port_like(&lower) {
        return false;
    }
    if node.name.trim().is_empty() {
        return node.attributes.contains_key("redefines");
    }
    true
}

pub(crate) fn build_general_view_detail_item(
    detail: &GraphNodeDto,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
    declared_in: Option<String>,
) -> Option<Value> {
    let name = general_view_detail_name(detail)?;
    let type_name = detail_type_name(detail, node_by_id, typing_targets);
    let value_text = detail_value_text(detail);
    let multiplicity = detail_multiplicity(detail);
    let direction = detail_direction(detail);
    let redefines = detail_redefines_short_name(detail);
    let subsets = detail_subsets_short_name(detail);
    let display_text = format_general_view_detail_display_text(
        &name,
        type_name.as_deref(),
        value_text.as_deref(),
        multiplicity.as_deref(),
        direction.as_deref(),
        redefines.as_deref(),
        subsets.as_deref(),
    );

    Some(json!({
        "name": name,
        "typeName": type_name,
        "valueText": value_text,
        "declaredIn": declared_in,
        "displayText": display_text,
    }))
}

pub(crate) fn detail_multiplicity(detail: &GraphNodeDto) -> Option<String> {
    detail
        .attributes
        .get("multiplicity")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(crate) fn detail_direction(detail: &GraphNodeDto) -> Option<String> {
    if !is_port_like(&detail.element_type) {
        return None;
    }
    detail
        .attributes
        .get("direction")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(crate) fn short_name_of_qualified_attribute(
    detail: &GraphNodeDto,
    key: &str,
) -> Option<String> {
    detail
        .attributes
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.split("::").last().unwrap_or(value).to_string())
        .filter(|value| !value.trim().is_empty())
}

pub(crate) fn detail_redefines_short_name(detail: &GraphNodeDto) -> Option<String> {
    // Anonymous stub nodes (no name of their own) already use `redefines` as their display
    // name via `general_view_detail_name`'s fallback — appending it again here would be
    // redundant (e.g. "engine redefines engine"). Only annotate named features.
    if detail.name.trim().is_empty() {
        return None;
    }
    short_name_of_qualified_attribute(detail, "redefines")
}

pub(crate) fn detail_subsets_short_name(detail: &GraphNodeDto) -> Option<String> {
    if detail.name.trim().is_empty() {
        return None;
    }
    short_name_of_qualified_attribute(detail, "subsetsFeature")
}

pub(crate) fn general_view_detail_name(detail: &GraphNodeDto) -> Option<String> {
    let explicit_name = detail.name.trim();
    if !explicit_name.is_empty() {
        return Some(explicit_name.to_string());
    }
    if is_attribute_like(&detail.element_type) || is_part_compartment_item(detail) {
        return detail
            .attributes
            .get("redefines")
            .and_then(|value| value.as_str())
            .map(|value| value.split("::").last().unwrap_or(value).to_string());
    }
    None
}

pub(crate) fn detail_type_name(
    detail: &GraphNodeDto,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
) -> Option<String> {
    typing_targets
        .get(detail.id.as_str())
        .and_then(|target_id| node_by_id.get(target_id))
        .map(|target| target.name.as_str())
        .or_else(|| {
            if is_port_like(&detail.element_type) {
                detail
                    .attributes
                    .get("portType")
                    .and_then(|value| value.as_str())
            } else if is_part_compartment_item(detail) {
                detail
                    .attributes
                    .get("partType")
                    .and_then(|value| value.as_str())
                    .or_else(|| {
                        detail
                            .attributes
                            .get("type")
                            .and_then(|value| value.as_str())
                    })
            } else {
                detail
                    .attributes
                    .get("refType")
                    .and_then(|value| value.as_str())
                    .or_else(|| {
                        detail
                            .attributes
                            .get("dataType")
                            .and_then(|value| value.as_str())
                    })
                    .or_else(|| {
                        detail
                            .attributes
                            .get("attributeType")
                            .and_then(|value| value.as_str())
                    })
                    .or_else(|| {
                        detail
                            .attributes
                            .get("type")
                            .and_then(|value| value.as_str())
                    })
                    .or_else(|| {
                        detail
                            .attributes
                            .get("parameterType")
                            .and_then(|value| value.as_str())
                    })
            }
        })
        .map(|type_name| {
            type_name
                .split("::")
                .last()
                .unwrap_or(type_name)
                .to_string()
        })
        .filter(|type_name| !type_name.trim().is_empty())
}

pub(crate) fn detail_value_text(detail: &GraphNodeDto) -> Option<String> {
    for key in ["value", "defaultValue", "valueText", "literal"] {
        let Some(value) = detail.attributes.get(key) else {
            continue;
        };
        if let Some(raw) = value.as_str() {
            if !raw.trim().is_empty() {
                return Some(raw.trim().to_string());
            }
        } else if value.is_number() || value.is_boolean() {
            return Some(value.to_string());
        }
    }
    None
}

pub(crate) fn format_general_view_detail_display_text(
    name: &str,
    type_name: Option<&str>,
    value_text: Option<&str>,
    multiplicity: Option<&str>,
    direction: Option<&str>,
    redefines: Option<&str>,
    subsets: Option<&str>,
) -> String {
    let mut text = String::new();
    if let Some(direction) = direction.filter(|d| !d.is_empty()) {
        text.push_str(direction);
        text.push(' ');
    }
    text.push_str(name);
    if let Some(multiplicity) = multiplicity.filter(|m| !m.is_empty()) {
        text.push_str(" [");
        text.push_str(multiplicity);
        text.push(']');
    }
    match (type_name, value_text) {
        (Some(type_name), Some(value_text)) if !type_name.is_empty() && !value_text.is_empty() => {
            text.push_str(" : ");
            text.push_str(type_name);
            text.push_str(" = ");
            text.push_str(value_text);
        }
        (Some(type_name), _) if !type_name.is_empty() => {
            text.push_str(" : ");
            text.push_str(type_name);
        }
        (_, Some(value_text)) if !value_text.is_empty() => {
            text.push_str(" = ");
            text.push_str(value_text);
        }
        _ => {}
    }
    if let Some(redefines) = redefines.filter(|r| !r.is_empty()) {
        text.push_str(" redefines ");
        text.push_str(redefines);
    }
    if let Some(subsets) = subsets.filter(|s| !s.is_empty()) {
        text.push_str(" subsets ");
        text.push_str(subsets);
    }
    text
}

pub(crate) fn insert_detail_items(
    attributes: &mut HashMap<String, Value>,
    key: &str,
    items: Vec<Value>,
) {
    if items.is_empty() {
        attributes.remove(key);
    } else {
        attributes.insert(key.to_string(), Value::Array(items));
    }
}
