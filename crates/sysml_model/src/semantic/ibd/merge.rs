//! Merge per-URI IBD payloads into a workspace-scoped payload.

use std::collections::HashSet;

use super::connectors::enrich_connector_endpoint_refs;
use super::dto::{
    DefInstanceMappingDto, IbdConnectorDto, IbdContainerGroupDto, IbdDataDto,
    IbdPackageContainerGroupDto, IbdPartDto, IbdPortDto, IbdRootViewDto,
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
    let mut root_views: std::collections::BTreeMap<String, IbdRootViewDto> =
        std::collections::BTreeMap::new();
    let mut container_groups_by_id: std::collections::HashMap<String, IbdContainerGroupDto> =
        std::collections::HashMap::new();
    let mut package_container_groups_by_id: std::collections::HashMap<
        String,
        IbdPackageContainerGroupDto,
    > = std::collections::HashMap::new();
    let mut def_instance_mappings_by_key: std::collections::HashMap<
        (String, String),
        DefInstanceMappingDto,
    > = std::collections::HashMap::new();

    for ibd in ibds {
        for mapping in ibd.def_instance_mappings {
            def_instance_mappings_by_key
                .entry((mapping.def_root.clone(), mapping.instance_root.clone()))
                .or_insert(mapping);
        }
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

    // O-6: `.into_values()` on a `HashMap` yields entries in that map's internal bucket order,
    // which depends on Rust's per-process-randomized default hasher -- stable within one run, but
    // different across separate `spec42 diagrams export` invocations even for byte-identical
    // input. That randomized order silently propagates into `finalize_merged_ibd_connectors`'s
    // instance/def remap (which iterates `parts`/`connectors` to build its mapping list) and from
    // there into which of several structurally-equal-but-differently-worded connector
    // representations survives dedup, and ultimately into ELK's node/edge order -- producing
    // different (but individually valid) layouts run to run for the same model. Sorting by each
    // DTO's own natural key right after the HashMap collapses back to deterministic id/name order
    // regardless of hash seed.
    let mut parts: Vec<IbdPartDto> = parts_by_id.into_values().collect();
    parts.sort_by(|a, b| a.id.cmp(&b.id));
    let mut ports: Vec<IbdPortDto> = ports_by_key.into_values().collect();
    ports.sort_by(|a, b| (&a.parent_id, &a.name).cmp(&(&b.parent_id, &b.name)));
    let mut connectors: Vec<IbdConnectorDto> = connectors_by_key.into_values().collect();
    connectors.sort_by(|a, b| {
        (&a.source_id, &a.target_id, &a.rel_type).cmp(&(&b.source_id, &b.target_id, &b.rel_type))
    });
    let (parts, ports, connectors) =
        prune_interconnection_definition_parts(parts, ports, connectors);
    let mut connectors = connectors;
    if enrich_connectors {
        enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);
    }
    for view in root_views.values_mut() {
        view.parts.sort_by(|a, b| a.id.cmp(&b.id));
        view.ports
            .sort_by(|a, b| (&a.parent_id, &a.name).cmp(&(&b.parent_id, &b.name)));
        view.connectors.sort_by(|a, b| {
            (&a.source_id, &a.target_id, &a.rel_type).cmp(&(
                &b.source_id,
                &b.target_id,
                &b.rel_type,
            ))
        });
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

    let mut container_groups: Vec<IbdContainerGroupDto> =
        container_groups_by_id.into_values().collect();
    container_groups.sort_by(|a, b| a.id.cmp(&b.id));
    let mut package_container_groups: Vec<IbdPackageContainerGroupDto> =
        package_container_groups_by_id.into_values().collect();
    package_container_groups.sort_by(|a, b| a.id.cmp(&b.id));
    let mut def_instance_mappings: Vec<DefInstanceMappingDto> =
        def_instance_mappings_by_key.into_values().collect();
    def_instance_mappings
        .sort_by(|a, b| (&a.def_root, &a.instance_root).cmp(&(&b.def_root, &b.instance_root)));

    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups,
        root_candidates: root_candidates.into_iter().collect(),
        default_root,
        root_views,
        def_instance_mappings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn part(id: &str) -> IbdPartDto {
        IbdPartDto {
            id: id.to_string(),
            node_id: id.replace("::", "."),
            name: id.rsplit("::").next().unwrap_or(id).to_string(),
            qualified_name: id.replace("::", "."),
            uri: None,
            container_id: None,
            element_type: "part".to_string(),
            attributes: HashMap::new(),
            range: None,
        }
    }

    /// `prune_interconnection_definition_parts` matches `port.parent_id` against the owning
    /// part's `qualified_name` verbatim (no `::`/`.` normalization at that step, unlike connector
    /// endpoint resolution) -- so `parent` must be the *part's own* `qualified_name`, not its `id`.
    fn port(parent: &IbdPartDto, name: &str) -> IbdPortDto {
        IbdPortDto {
            id: format!("{}::{name}", parent.id),
            port_id: format!("{}.{name}", parent.qualified_name),
            name: name.to_string(),
            parent_id: parent.qualified_name.clone(),
            direction: None,
            port_type: None,
            multiplicity: None,
            port_side: None,
            uri: None,
            range: None,
        }
    }

    fn connector(source_id: &str, target_id: &str) -> IbdConnectorDto {
        IbdConnectorDto {
            source: source_id.to_string(),
            target: target_id.to_string(),
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            source_part_id: None,
            target_part_id: None,
            source_port_id: None,
            target_port_id: None,
            rel_type: "connection".to_string(),
        }
    }

    fn ibd(
        parts: Vec<IbdPartDto>,
        ports: Vec<IbdPortDto>,
        connectors: Vec<IbdConnectorDto>,
    ) -> IbdDataDto {
        IbdDataDto {
            parts,
            ports,
            connectors,
            def_instance_mappings: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            default_root: None,
            root_views: Default::default(),
        }
    }

    /// O-6 regression: `merge_ibd_payloads_inner` dedups parts/ports/connectors through a
    /// `HashMap` before collecting them into the final `Vec`s. `HashMap`'s default hasher is
    /// randomly seeded per process, so `.into_values().collect()` alone yields a different (but
    /// content-equal) order every time the *process* is restarted -- invisible to a same-process
    /// test that merges once and asserts a single result, since the seed (and therefore the
    /// order) is identical across calls within one process. What a same-process test *can* pin
    /// down is the actual fix: the merge output must be sorted by each DTO's own natural key,
    /// regardless of which order (or how many duplicate/overlapping per-URI payloads) fed into
    /// it. Feeds the per-URI payloads in deliberately reversed/interleaved order relative to the
    /// expected sorted output to make sure a sort is actually happening, not just coincidentally
    /// already-ordered input.
    #[test]
    fn merge_output_is_sorted_by_natural_key_regardless_of_input_order() {
        let part_a = part("Pkg::A");
        let part_b = part("Pkg::B");
        let part_c = part("Pkg::C");
        let port_a_in = port(&part_a, "in");
        let port_b_mid = port(&part_b, "mid");
        let port_c_out = port(&part_c, "out");

        let ibd_c = ibd(
            vec![part_c.clone()],
            vec![port_c_out.clone()],
            vec![connector(&port_c_out.port_id, &port_a_in.port_id)],
        );
        let ibd_a = ibd(
            vec![part_a.clone(), part_a.clone()], // duplicate part across "documents"
            vec![port_a_in.clone()],
            vec![],
        );
        let ibd_b = ibd(vec![part_b.clone()], vec![port_b_mid.clone()], vec![]);

        let merged = merge_ibd_payloads_for_workspace_finalize(vec![ibd_c, ibd_a, ibd_b]);

        let part_ids: Vec<&str> = merged.parts.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(
            part_ids,
            vec!["Pkg::A", "Pkg::B", "Pkg::C"],
            "parts must be sorted by id"
        );

        let port_keys: Vec<(&str, &str)> = merged
            .ports
            .iter()
            .map(|p| (p.parent_id.as_str(), p.name.as_str()))
            .collect();
        assert_eq!(
            port_keys,
            vec![("Pkg.A", "in"), ("Pkg.B", "mid"), ("Pkg.C", "out")],
            "ports must be sorted by (parent_id, name)"
        );

        let connector_keys: Vec<(&str, &str)> = merged
            .connectors
            .iter()
            .map(|c| (c.source_id.as_str(), c.target_id.as_str()))
            .collect();
        assert_eq!(
            connector_keys,
            vec![(port_c_out.port_id.as_str(), port_a_in.port_id.as_str())],
            "connectors must be sorted by (source_id, target_id, rel_type)"
        );
    }
}
