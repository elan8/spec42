//! Instance-path normalization for scoped IBD payloads.

use super::connectors::enrich_connector_endpoint_refs;
use super::dto::{IbdDataDto, IbdPartDto, IbdPortDto};

/// Definition-to-instance root mappings, sourced from real typing-edge-derived data recorded
/// during extraction (`build_instance_def_mappings`, `ibd/connectors.rs`) — accurate for any
/// package naming convention, not just a specific "architecture"/"Architecture" segment pattern.
/// `build_ibd_for_uri` always populates `ibd.def_instance_mappings` in production; only hand-built
/// test fixtures ever leave it empty.
pub(crate) fn infer_def_instance_scope_mappings_for_ibd(ibd: &IbdDataDto) -> Vec<(String, String)> {
    let mut mappings: Vec<(String, String)> = ibd
        .def_instance_mappings
        .iter()
        .map(|mapping| (mapping.def_root.clone(), mapping.instance_root.clone()))
        .collect();
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
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
