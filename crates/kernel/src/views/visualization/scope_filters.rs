use std::collections::{HashMap, HashSet};

use crate::views::ibd::{self, IbdDataDto};

pub(super) fn filter_ibd_by_visible_ids(ibd: &IbdDataDto, visible_ids: &HashSet<String>) -> IbdDataDto {
    let visible_dot_ids: HashSet<String> =
        visible_ids.iter().map(|id| id.replace("::", ".")).collect();
    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| {
            visible_ids.contains(&part.id) || visible_dot_ids.contains(&part.qualified_name)
        })
        .cloned()
        .collect();
    let part_ids: HashSet<_> = parts.iter().map(|part| part.id.as_str()).collect();
    let part_qualified_names: HashSet<_> = parts
        .iter()
        .map(|part| part.qualified_name.as_str())
        .collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| {
            visible_ids.contains(&port.id)
                || visible_dot_ids.contains(&port.id.replace("::", "."))
                || part_ids.contains(port.parent_id.as_str())
                || part_qualified_names.contains(port.parent_id.as_str())
        })
        .cloned()
        .collect();
    let port_ids: HashSet<_> = ports.iter().map(|port| port.id.as_str()).collect();
    let port_dot_ids: HashSet<String> = ports
        .iter()
        .map(|port| port.id.replace("::", "."))
        .collect();
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            port_ids.contains(connector.source.as_str())
                || port_ids.contains(connector.target.as_str())
                || port_dot_ids.contains(&connector.source_id)
                || port_dot_ids.contains(&connector.target_id)
                || part_ids.contains(connector.source_id.as_str())
                || part_ids.contains(connector.target_id.as_str())
                || part_qualified_names.contains(connector.source_id.as_str())
                || part_qualified_names.contains(connector.target_id.as_str())
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group
                .member_part_ids
                .iter()
                .any(|member_id| part_ids.contains(member_id.as_str()))
        })
        .cloned()
        .collect();
    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| {
                visible_ids.contains(&part.id) || visible_dot_ids.contains(&part.qualified_name)
            })
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<_> =
            filtered_parts.iter().map(|part| part.id.as_str()).collect();
        let filtered_part_qualified_names: HashSet<_> = filtered_parts
            .iter()
            .map(|part| part.qualified_name.as_str())
            .collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| {
                visible_ids.contains(&port.id)
                    || visible_dot_ids.contains(&port.id.replace("::", "."))
                    || filtered_part_ids.contains(port.parent_id.as_str())
                    || filtered_part_qualified_names.contains(port.parent_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_port_ids: HashSet<_> =
            filtered_ports.iter().map(|port| port.id.as_str()).collect();
        let filtered_port_dot_ids: HashSet<String> = filtered_ports
            .iter()
            .map(|port| port.id.replace("::", "."))
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                filtered_port_ids.contains(connector.source.as_str())
                    || filtered_port_ids.contains(connector.target.as_str())
                    || filtered_port_dot_ids.contains(&connector.source_id)
                    || filtered_port_dot_ids.contains(&connector.target_id)
                    || filtered_part_ids.contains(connector.source_id.as_str())
                    || filtered_part_ids.contains(connector.target_id.as_str())
                    || filtered_part_qualified_names.contains(connector.source_id.as_str())
                    || filtered_part_qualified_names.contains(connector.target_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group
                    .member_part_ids
                    .iter()
                    .any(|member_id| filtered_part_ids.contains(member_id.as_str()))
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                ibd::IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }
    let root_candidates: Vec<_> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = ibd
        .default_root
        .as_ref()
        .filter(|root| root_views.contains_key(root.as_str()))
        .cloned()
        .or_else(|| root_candidates.first().cloned());
    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        root_views,
        default_root,
    }
}

fn endpoint_matches_root_prefix(endpoint: &str, root_prefix: &str) -> bool {
    endpoint == root_prefix || endpoint.starts_with(&format!("{root_prefix}."))
}

pub(super) fn filter_ibd_by_root_prefixes(ibd: &IbdDataDto, root_prefixes: &HashSet<String>) -> IbdDataDto {
    let matches_any_root = |endpoint: &str| {
        root_prefixes
            .iter()
            .any(|prefix| endpoint_matches_root_prefix(endpoint, prefix))
    };

    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| matches_any_root(&part.qualified_name))
        .cloned()
        .collect();
    let part_ids: HashSet<_> = parts.iter().map(|part| part.id.as_str()).collect();
    let part_qualified_names: HashSet<_> = parts
        .iter()
        .map(|part| part.qualified_name.as_str())
        .collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| {
            matches_any_root(&port.parent_id)
                || part_ids.contains(port.parent_id.as_str())
                || part_qualified_names.contains(port.parent_id.as_str())
        })
        .cloned()
        .collect();
    let port_ids: HashSet<_> = ports.iter().map(|port| port.id.as_str()).collect();
    let port_dot_ids: HashSet<String> = ports
        .iter()
        .map(|port| port.id.replace("::", "."))
        .collect();
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            matches_any_root(&connector.source_id)
                || matches_any_root(&connector.target_id)
                || port_ids.contains(connector.source.as_str())
                || port_ids.contains(connector.target.as_str())
                || port_dot_ids.contains(&connector.source_id)
                || port_dot_ids.contains(&connector.target_id)
                || part_qualified_names.contains(connector.source_id.as_str())
                || part_qualified_names.contains(connector.target_id.as_str())
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group
                .member_part_ids
                .iter()
                .any(|member_id| part_ids.contains(member_id.as_str()))
        })
        .cloned()
        .collect();

    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| matches_any_root(&part.qualified_name))
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<_> =
            filtered_parts.iter().map(|part| part.id.as_str()).collect();
        let filtered_part_qualified_names: HashSet<_> = filtered_parts
            .iter()
            .map(|part| part.qualified_name.as_str())
            .collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| {
                matches_any_root(&port.parent_id)
                    || filtered_part_ids.contains(port.parent_id.as_str())
                    || filtered_part_qualified_names.contains(port.parent_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_port_ids: HashSet<_> =
            filtered_ports.iter().map(|port| port.id.as_str()).collect();
        let filtered_port_dot_ids: HashSet<String> = filtered_ports
            .iter()
            .map(|port| port.id.replace("::", "."))
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                matches_any_root(&connector.source_id)
                    || matches_any_root(&connector.target_id)
                    || filtered_port_ids.contains(connector.source.as_str())
                    || filtered_port_ids.contains(connector.target.as_str())
                    || filtered_port_dot_ids.contains(&connector.source_id)
                    || filtered_port_dot_ids.contains(&connector.target_id)
                    || filtered_part_qualified_names.contains(connector.source_id.as_str())
                    || filtered_part_qualified_names.contains(connector.target_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group
                    .member_part_ids
                    .iter()
                    .any(|member_id| filtered_part_ids.contains(member_id.as_str()))
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                ibd::IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }

    let root_candidates: Vec<_> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = ibd
        .default_root
        .as_ref()
        .filter(|root| root_views.contains_key(root.as_str()))
        .cloned()
        .or_else(|| root_candidates.first().cloned());
    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        root_views,
        default_root,
    }
}

pub(super) fn select_interconnection_ibd_scope(
    full_ibd: &IbdDataDto,
    selected_ids: &HashSet<String>,
    selected_exposed_ids: Option<&HashSet<String>>,
) -> IbdDataDto {
    let visible_scope_ibd = filter_ibd_by_visible_ids(full_ibd, selected_ids);
    let root_prefixes: HashSet<String> = selected_exposed_ids
        .map(|exposed_ids| exposed_ids.iter().map(|id| id.replace("::", ".")).collect())
        .unwrap_or_default();
    if root_prefixes.is_empty() {
        return visible_scope_ibd;
    }
    let root_scoped_ibd = filter_ibd_by_root_prefixes(full_ibd, &root_prefixes);
    if root_scoped_ibd.parts.is_empty() && root_scoped_ibd.connectors.is_empty() {
        // Cross-file instance roots (usage in one file, connections in another)
        // can miss direct root-prefix matches; keep the selected visible scope instead.
        return visible_scope_ibd;
    }
    root_scoped_ibd
}
