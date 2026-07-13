use super::*;

/// Drops unrelated assemblies, but keeps every part nested under a composite that
/// already contains a connector endpoint (including unconnected siblings).
pub(crate) fn prune_ibd_payload_to_connected_scope(
    parts: Vec<IbdPartDto>,
    ports: Vec<IbdPortDto>,
    connectors: Vec<IbdConnectorDto>,
) -> (Vec<IbdPartDto>, Vec<IbdPortDto>, Vec<IbdConnectorDto>) {
    if parts.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new());
    }
    if connectors.is_empty() {
        // Keep structural parts/ports even when no connectors resolved yet.
        return (parts, ports, connectors);
    }

    let part_by_qn: std::collections::HashMap<String, IbdPartDto> = parts
        .iter()
        .cloned()
        .map(|part| (part.qualified_name.clone(), part))
        .collect();
    let mut keep_part_qn: std::collections::HashSet<String> = std::collections::HashSet::new();

    for connector in &connectors {
        for endpoint in [&connector.source_id, &connector.target_id] {
            let Some(owner_qn) = resolve_owner_part_qn_for_endpoint(endpoint, &parts) else {
                continue;
            };
            let mut current_qn = Some(owner_qn);
            while let Some(qn) = current_qn {
                if !keep_part_qn.insert(qn.clone()) {
                    break;
                }
                current_qn = part_by_qn
                    .get(&qn)
                    .and_then(|part| part.container_id.clone())
                    .filter(|container| !container.is_empty());
            }
        }
    }

    // Parts that sit under the same composite as a connector-attached part (siblings,
    // or nested parts under those siblings) should still appear in the diagram.
    let mut subtree_expanded = true;
    while subtree_expanded {
        subtree_expanded = false;
        for part in &parts {
            if keep_part_qn.contains(&part.qualified_name) {
                continue;
            }
            let Some(parent) = part.container_id.as_ref().filter(|p| !p.is_empty()) else {
                continue;
            };
            if keep_part_qn.contains(parent) && keep_part_qn.insert(part.qualified_name.clone()) {
                subtree_expanded = true;
            }
        }
    }

    if keep_part_qn.is_empty() {
        return (parts, ports, connectors);
    }

    let parts: Vec<IbdPartDto> = parts
        .into_iter()
        .filter(|part| keep_part_qn.contains(&part.qualified_name))
        .collect();
    let part_qn: std::collections::HashSet<String> = parts
        .iter()
        .map(|part| part.qualified_name.clone())
        .collect();
    let ports: Vec<IbdPortDto> = ports
        .into_iter()
        .filter(|port| part_qn.contains(&port.parent_id))
        .collect();
    let connectors: Vec<IbdConnectorDto> = connectors
        .into_iter()
        .filter(|connector| {
            resolve_owner_part_qn_for_endpoint(&connector.source_id, &parts).is_some()
                && resolve_owner_part_qn_for_endpoint(&connector.target_id, &parts).is_some()
        })
        .collect();
    (parts, ports, connectors)
}

pub(crate) fn resolve_endpoint_anchor_node<'a>(
    graph: &'a SemanticGraph,
    uri: &Url,
    endpoint_id: &str,
) -> Option<&'a SemanticNode> {
    let mut candidate = endpoint_id.replace('.', "::");
    loop {
        let node_id = NodeId::new(uri, &candidate);
        if let Some(node) = graph.get_node(&node_id) {
            if is_port_like(node.element_kind.as_str()) {
                if let Some(parent_id) = &node.parent_id {
                    if let Some(parent) = graph.get_node(parent_id) {
                        return Some(parent);
                    }
                }
            }
            return Some(node);
        }
        if let Some((prefix, _)) = candidate.rsplit_once("::") {
            candidate = prefix.to_string();
        } else {
            break;
        }
    }
    None
}

pub(crate) fn ensure_endpoint_parts_present(
    parts: &mut Vec<IbdPartDto>,
    connectors: &[IbdConnectorDto],
    graph: &SemanticGraph,
    uri: &Url,
) {
    let mut existing_part_qn: std::collections::HashSet<String> = parts
        .iter()
        .map(|part| part.qualified_name.clone())
        .collect();

    for endpoint in connectors
        .iter()
        .flat_map(|connector| [&connector.source_id, &connector.target_id])
    {
        let Some(node) = resolve_endpoint_anchor_node(graph, uri, endpoint) else {
            continue;
        };
        let qualified_name = qualified_name_to_dot(&node.id.qualified_name);
        if !existing_part_qn.insert(qualified_name.clone()) {
            continue;
        }
        let container_id = node
            .parent_id
            .as_ref()
            .map(|parent| qualified_name_to_dot(&parent.qualified_name));
        if is_definition_element_kind(node.element_kind.as_str()) {
            continue;
        }
        let element_type = normalize_ibd_element_type(node.element_kind.as_str());
        let mut attributes = node.attributes.clone();
        decorate_ibd_part_attributes(&element_type, &mut attributes);
        parts.push(IbdPartDto {
            id: node.id.qualified_name.clone(),
            node_id: qualified_name.clone(),
            name: node.name.clone(),
            qualified_name,
            uri: Some(node.id.uri.as_str().to_string()),
            container_id,
            element_type,
            attributes,
            range: Some(crate::semantic::dto::range_to_dto(node.range)),
        });
    }
}

pub(crate) fn materialized_subtree_metrics(
    root_prefix: &str,
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
    connectors: &[IbdConnectorDto],
) -> (usize, usize, usize) {
    let part_count = parts
        .iter()
        .filter(|part| endpoint_matches_root(&part.qualified_name, root_prefix))
        .count();
    let port_count = ports
        .iter()
        .filter(|port| endpoint_matches_root(&port.parent_id, root_prefix))
        .count();
    let connector_count = connectors
        .iter()
        .filter(|connector| {
            endpoint_matches_root(&connector.source_id, root_prefix)
                && endpoint_matches_root(&connector.target_id, root_prefix)
        })
        .count();
    (part_count, port_count, connector_count)
}

pub(crate) fn is_top_level_part(part: &IbdPartDto, parts: &[IbdPartDto]) -> bool {
    match part.container_id.as_deref() {
        None | Some("") => true,
        Some(container_id) => !parts
            .iter()
            .any(|candidate| candidate.qualified_name == container_id),
    }
}
