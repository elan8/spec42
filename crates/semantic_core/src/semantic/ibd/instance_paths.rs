//! Instance-path normalization for scoped IBD payloads.

use super::connectors::enrich_connector_endpoint_refs;
use super::dto::{IbdDataDto, IbdPartDto, IbdPortDto};

fn split_architecture_scope_root(qualified_name: &str) -> Option<(&str, &str)> {
    if let Some(pos) = qualified_name.find(".architecture.") {
        return Some((
            &qualified_name[..pos + ".architecture".len()],
            &qualified_name[pos + ".architecture.".len()..],
        ));
    }
    if let Some(pos) = qualified_name.find(".Architecture.") {
        let start = pos + ".Architecture.".len();
        let tail = &qualified_name[start..];
        let def_len = tail.find('.').unwrap_or(tail.len());
        if def_len > 0 {
            let end = start + def_len;
            let remainder = qualified_name[end..].strip_prefix('.').unwrap_or_default();
            return Some((&qualified_name[..end], remainder));
        }
    }
    if let Some(pos) = qualified_name.rfind(".RegionalGridArchitecture") {
        let end = pos + ".RegionalGridArchitecture".len();
        let tail = qualified_name[end..].strip_prefix('.').unwrap_or_default();
        return Some((&qualified_name[..end], tail));
    }
    None
}

fn architecture_package_prefix(qualified_name: &str) -> Option<&str> {
    if let Some(pos) = qualified_name.find(".architecture") {
        return qualified_name[..pos]
            .rsplit_once('.')
            .map(|(prefix, _)| prefix)
            .or(Some(&qualified_name[..pos]));
    }
    if let Some(pos) = qualified_name.find(".Architecture.") {
        return Some(&qualified_name[..pos]);
    }
    None
}

pub(crate) fn infer_def_instance_scope_mappings_for_ibd(ibd: &IbdDataDto) -> Vec<(String, String)> {
    infer_def_instance_scope_mappings(&ibd.parts)
}

fn infer_def_instance_scope_mappings(parts: &[IbdPartDto]) -> Vec<(String, String)> {
    let mut definition_roots: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut instance_roots: std::collections::HashSet<String> = std::collections::HashSet::new();

    for part in parts {
        let Some((root, _)) = split_architecture_scope_root(&part.qualified_name) else {
            continue;
        };
        if root.ends_with(".architecture") {
            instance_roots.insert(root.to_string());
        } else if root.contains(".Architecture.") {
            definition_roots.insert(root.to_string());
        }
    }

    let mut mappings: Vec<(String, String)> = Vec::new();
    for def_root in definition_roots {
        let Some(def_package) = architecture_package_prefix(&def_root) else {
            continue;
        };
        for instance_root in &instance_roots {
            let Some(instance_package) = architecture_package_prefix(instance_root) else {
                continue;
            };
            if def_package != instance_package {
                continue;
            }
            mappings.push((def_root.clone(), instance_root.clone()));
        }
    }
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
    mappings.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
    mappings
}

fn remap_qualified_name_with_mappings(value: &str, mappings: &[(String, String)]) -> String {
    for (def_prefix, instance_prefix) in mappings {
        if value == def_prefix {
            return instance_prefix.clone();
        }
        let prefixed = format!("{def_prefix}.");
        if let Some(remainder) = value.strip_prefix(&prefixed) {
            return format!("{instance_prefix}.{remainder}");
        }
    }
    value.to_string()
}

/// Align scoped IBD parts/ports with instance-centric connector endpoints.
pub fn normalize_ibd_to_instance_paths(ibd: &mut IbdDataDto) {
    let mappings = infer_def_instance_scope_mappings_for_ibd(ibd);
    if mappings.is_empty() {
        return;
    }

    let mut parts_by_qn: std::collections::HashMap<String, IbdPartDto> =
        std::collections::HashMap::new();
    for mut part in ibd.parts.drain(..) {
        part.qualified_name = remap_qualified_name_with_mappings(&part.qualified_name, &mappings);
        part.node_id = part.qualified_name.clone();
        part.id = part.qualified_name.replace('.', "::");
        if let Some(container_id) = part.container_id.as_mut() {
            *container_id = remap_qualified_name_with_mappings(container_id, &mappings);
        }
        parts_by_qn.insert(part.qualified_name.clone(), part);
    }
    ibd.parts = parts_by_qn.into_values().collect();

    let mut ports_by_id: std::collections::HashMap<String, IbdPortDto> =
        std::collections::HashMap::new();
    for mut port in ibd.ports.drain(..) {
        port.parent_id = remap_qualified_name_with_mappings(&port.parent_id, &mappings);
        port.port_id = remap_qualified_name_with_mappings(&port.port_id, &mappings);
        port.id = port.port_id.replace('.', "::");
        ports_by_id.insert(port.port_id.clone(), port);
    }
    ibd.ports = ports_by_id.into_values().collect();

    for group in &mut ibd.container_groups {
        group.member_part_ids = group
            .member_part_ids
            .iter()
            .map(|member| remap_qualified_name_with_mappings(member, &mappings))
            .collect();
        if let Some(parent) = group.parent_id.as_mut() {
            *parent = remap_qualified_name_with_mappings(parent, &mappings);
        }
        group.qualified_name = remap_qualified_name_with_mappings(&group.qualified_name, &mappings);
    }

    enrich_connector_endpoint_refs(&mut ibd.connectors, &ibd.parts, &ibd.ports);
}
