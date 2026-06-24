//! Merge per-URI IBD payloads into a workspace-scoped payload.

use std::collections::HashSet;

use super::connectors::enrich_connector_endpoint_refs;
use super::dto::{
    IbdConnectorDto, IbdContainerGroupDto, IbdDataDto, IbdPackageContainerGroupDto, IbdPartDto,
    IbdPortDto, IbdRootViewDto,
};
use super::extract_impl::{is_part_instance_kind, prune_interconnection_definition_parts};

/// Merge multiple per-URI IBD payloads into one workspace-scoped payload.
pub fn merge_ibd_payloads(ibds: Vec<IbdDataDto>) -> IbdDataDto {
    merge_ibd_payloads_inner(ibds, true)
}

/// Merge payloads before [`super::connectors::finalize_merged_ibd_connectors`], which re-enriches connectors.
pub fn merge_ibd_payloads_for_workspace_finalize(ibds: Vec<IbdDataDto>) -> IbdDataDto {
    merge_ibd_payloads_inner(ibds, false)
}

fn merge_member_part_ids(existing: &mut Vec<String>, incoming: &[String]) {
    if incoming.is_empty() {
        return;
    }
    if existing.is_empty() {
        existing.extend_from_slice(incoming);
        return;
    }
    let mut seen: HashSet<String> = existing.iter().cloned().collect();
    for part_id in incoming {
        if seen.insert(part_id.clone()) {
            existing.push(part_id.clone());
        }
    }
}

fn merge_ibd_payloads_inner(ibds: Vec<IbdDataDto>, enrich_connectors: bool) -> IbdDataDto {
    let mut parts_by_id: std::collections::HashMap<String, IbdPartDto> =
        std::collections::HashMap::new();
    let mut ports_by_key: std::collections::HashMap<(String, String), IbdPortDto> =
        std::collections::HashMap::new();
    let mut connectors_by_key: std::collections::HashMap<
        (String, String, String),
        IbdConnectorDto,
    > = std::collections::HashMap::new();
    let mut root_candidates: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut root_views: std::collections::HashMap<String, IbdRootViewDto> =
        std::collections::HashMap::new();
    let mut container_groups_by_id: std::collections::HashMap<String, IbdContainerGroupDto> =
        std::collections::HashMap::new();
    let mut package_container_groups_by_id: std::collections::HashMap<
        String,
        IbdPackageContainerGroupDto,
    > = std::collections::HashMap::new();

    for ibd in ibds {
        for p in ibd.parts {
            parts_by_id.entry(p.id.clone()).or_insert(p);
        }
        for p in ibd.ports {
            ports_by_key
                .entry((p.parent_id.clone(), p.name.clone()))
                .or_insert(p);
        }
        for c in ibd.connectors {
            connectors_by_key
                .entry((c.source_id.clone(), c.target_id.clone(), c.rel_type.clone()))
                .or_insert(c);
        }
        for group in ibd.container_groups {
            container_groups_by_id
                .entry(group.id.clone())
                .and_modify(|existing| {
                    merge_member_part_ids(&mut existing.member_part_ids, &group.member_part_ids);
                })
                .or_insert(group);
        }
        for group in ibd.package_container_groups {
            package_container_groups_by_id
                .entry(group.id.clone())
                .and_modify(|existing| {
                    merge_member_part_ids(&mut existing.member_part_ids, &group.member_part_ids);
                })
                .or_insert(group);
        }
        for root in ibd.root_candidates {
            root_candidates.insert(root);
        }
        for (name, view) in ibd.root_views {
            let merged = root_views.entry(name).or_insert_with(|| IbdRootViewDto {
                parts: Vec::new(),
                ports: Vec::new(),
                connectors: Vec::new(),
                container_groups: Vec::new(),
                package_container_groups: Vec::new(),
            });
            let mut part_ids: std::collections::HashSet<String> =
                merged.parts.iter().map(|p| p.id.clone()).collect();
            for p in view.parts {
                if part_ids.insert(p.id.clone()) {
                    merged.parts.push(p);
                }
            }
            let mut port_keys: std::collections::HashSet<(String, String)> = merged
                .ports
                .iter()
                .map(|p| (p.parent_id.clone(), p.name.clone()))
                .collect();
            for p in view.ports {
                let key = (p.parent_id.clone(), p.name.clone());
                if port_keys.insert(key) {
                    merged.ports.push(p);
                }
            }
            let mut connector_keys: std::collections::HashSet<(String, String, String)> = merged
                .connectors
                .iter()
                .map(|c| (c.source_id.clone(), c.target_id.clone(), c.rel_type.clone()))
                .collect();
            for c in view.connectors {
                let key = (c.source_id.clone(), c.target_id.clone(), c.rel_type.clone());
                if connector_keys.insert(key) {
                    merged.connectors.push(c);
                }
            }
            let mut group_ids: std::collections::HashSet<String> = merged
                .container_groups
                .iter()
                .map(|group| group.id.clone())
                .collect();
            for group in view.container_groups {
                if group_ids.insert(group.id.clone()) {
                    merged.container_groups.push(group);
                }
            }
            let mut package_group_ids: std::collections::HashSet<String> = merged
                .package_container_groups
                .iter()
                .map(|group| group.id.clone())
                .collect();
            for group in view.package_container_groups {
                if package_group_ids.insert(group.id.clone()) {
                    merged.package_container_groups.push(group);
                }
            }
        }
    }

    let parts: Vec<IbdPartDto> = parts_by_id.into_values().collect();
    let ports: Vec<IbdPortDto> = ports_by_key.into_values().collect();
    let connectors: Vec<IbdConnectorDto> = connectors_by_key.into_values().collect();
    let (parts, ports, connectors) =
        prune_interconnection_definition_parts(parts, ports, connectors);
    let mut connectors = connectors;
    if enrich_connectors {
        enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);
    }
    for view in root_views.values_mut() {
        let (view_parts, view_ports, view_connectors) = prune_interconnection_definition_parts(
            std::mem::take(&mut view.parts),
            std::mem::take(&mut view.ports),
            std::mem::take(&mut view.connectors),
        );
        let mut view_connectors = view_connectors;
        if enrich_connectors {
            enrich_connector_endpoint_refs(&mut view_connectors, &view_parts, &view_ports);
        }
        view.parts = view_parts;
        view.ports = view_ports;
        view.connectors = view_connectors;
    }

    let default_root = root_candidates
        .iter()
        .filter(|name| root_views.contains_key(name.as_str()))
        .max_by_key(|name| {
            let view = root_views.get(*name).expect("root view");
            let connector_count = view.connectors.len();
            let part_count = view.parts.len();
            let is_instance = view
                .parts
                .iter()
                .find(|part| part.name == **name)
                .map(|part| is_part_instance_kind(&part.element_type))
                .unwrap_or(false);
            let instance_bonus = if is_instance { 1usize } else { 0usize };
            (connector_count, instance_bonus, part_count)
        })
        .cloned();

    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups: container_groups_by_id.into_values().collect(),
        package_container_groups: package_container_groups_by_id.into_values().collect(),
        root_candidates: root_candidates.into_iter().collect(),
        default_root,
        root_views,
    }
}
