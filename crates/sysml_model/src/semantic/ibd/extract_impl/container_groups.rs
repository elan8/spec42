use super::*;

/// Builds ancestor "container" boxes for the interconnection view by walking each part's
/// dotted qualified name. `is_non_part_container` lets callers exclude prefixes that resolve
/// to a package/namespace rather than an actual composing Part Definition/Usage (a package
/// segment isn't a diagrammable node and would otherwise render as an empty phantom box).
pub(crate) fn build_container_groups(
    parts: &[IbdPartDto],
    is_non_part_container: &dyn Fn(&str) -> bool,
) -> Vec<IbdContainerGroupDto> {
    let part_qn: std::collections::HashSet<String> = parts
        .iter()
        .map(|part| part.qualified_name.clone())
        .collect();
    let mut groups_by_qn: std::collections::HashMap<String, IbdContainerGroupDto> =
        std::collections::HashMap::new();

    for part in parts {
        let qn = part.qualified_name.as_str();
        let segments: Vec<&str> = qn
            .split('.')
            .filter(|segment| !segment.is_empty())
            .collect();
        if segments.len() < 2 {
            continue;
        }
        for depth in 1..segments.len() {
            let prefix = segments[..depth].join(".");
            if part_qn.contains(&prefix) {
                continue;
            }
            if is_non_part_container(&prefix) {
                continue;
            }
            let parent_qn = if depth > 1 {
                Some(segments[..depth - 1].join("."))
            } else {
                None
            };
            let id = format!("container:{prefix}");
            groups_by_qn
                .entry(prefix.clone())
                .or_insert_with(|| IbdContainerGroupDto {
                    id,
                    label: segments[depth - 1].to_string(),
                    depth,
                    qualified_name: prefix.clone(),
                    parent_id: parent_qn
                        .map(|value| format!("container:{value}"))
                        .filter(|value| !value.is_empty()),
                    member_part_ids: Vec::new(),
                });
        }
    }

    for part in parts {
        for group in groups_by_qn.values_mut() {
            if part.qualified_name == group.qualified_name
                || part
                    .qualified_name
                    .starts_with(&format!("{}.", group.qualified_name))
            {
                group.member_part_ids.push(part.id.clone());
            }
        }
    }

    let existing_group_ids: std::collections::HashSet<String> = groups_by_qn
        .values()
        .map(|group| group.id.clone())
        .collect();
    let mut groups: Vec<IbdContainerGroupDto> = groups_by_qn
        .into_values()
        .filter(|group| !group.member_part_ids.is_empty())
        .map(|mut group| {
            if let Some(parent_id) = &group.parent_id {
                if !existing_group_ids.contains(parent_id) {
                    group.parent_id = None;
                }
            }
            group
        })
        .collect();
    groups.sort_by(|left, right| {
        left.depth
            .cmp(&right.depth)
            .then_with(|| left.qualified_name.cmp(&right.qualified_name))
    });
    groups
}

pub(crate) fn expanded_port_to_ibd_dto(
    graph: &SemanticGraph,
    port: &crate::semantic::component_view::ExpandedPort,
) -> IbdPortDto {
    let port_side = infer_port_side(
        &port.name,
        port.direction.as_deref(),
        port.port_type.as_deref(),
    );
    let declaring_node = graph.get_node(&port.node_id);
    IbdPortDto {
        id: canonical_port_id(&port.parent_path, &port.name),
        port_id: canonical_port_id(&port.parent_path, &port.name),
        name: port.name.clone(),
        parent_id: port.parent_path.clone(),
        direction: port.direction.clone(),
        port_type: port.port_type.clone(),
        port_side,
        uri: Some(port.node_id.uri.as_str().to_string()),
        range: declaring_node.map(|node| crate::semantic::dto::range_to_dto(node.range)),
    }
}
