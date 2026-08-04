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
    mappings.sort_by(|left, right| {
        right
            .0
            .len()
            .cmp(&left.0.len())
            .then_with(|| {
                mapping_scope_relevance(ibd, &right.1).cmp(&mapping_scope_relevance(ibd, &left.1))
            })
            .then_with(|| left.1.cmp(&right.1))
    });
    mappings
}

fn mapping_scope_relevance(ibd: &IbdDataDto, instance_root: &str) -> u8 {
    let matches_instance =
        |value: &str| value == instance_root || value.starts_with(&format!("{instance_root}."));
    if ibd.connectors.iter().any(|connector| {
        matches_instance(&connector.source_id) || matches_instance(&connector.target_id)
    }) {
        return 2;
    }
    if ibd
        .parts
        .iter()
        .any(|part| matches_instance(&part.qualified_name))
        || ibd
            .ports
            .iter()
            .any(|port| matches_instance(&port.parent_id) || matches_instance(&port.port_id))
    {
        return 1;
    }
    0
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
    // O-6: `.into_values()` yields entries in the HashMap's hash-randomized bucket order --
    // re-sort by the DTO's own natural key so this doesn't undo merge.rs's determinism fix.
    ibd.parts = parts_by_qn.into_values().collect();
    ibd.parts.sort_by(|a, b| a.id.cmp(&b.id));

    let mut ports_by_id: std::collections::HashMap<String, IbdPortDto> =
        std::collections::HashMap::new();
    for mut port in ibd.ports.drain(..) {
        port.parent_id = remap_qualified_name_with_mappings(&port.parent_id, &mappings);
        port.port_id = remap_qualified_name_with_mappings(&port.port_id, &mappings);
        port.id = port.port_id.replace('.', "::");
        ports_by_id.insert(port.port_id.clone(), port);
    }
    ibd.ports = ports_by_id.into_values().collect();
    ibd.ports
        .sort_by(|a, b| (&a.parent_id, &a.name).cmp(&(&b.parent_id, &b.name)));

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

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use super::normalize_ibd_to_instance_paths;
    use crate::semantic::ibd::{
        DefInstanceMappingDto, IbdConnectorDto, IbdDataDto, IbdPartDto, IbdPortDto,
    };

    #[test]
    fn normalization_prefers_instance_mapping_present_in_scoped_connectors() {
        let definition_root = "Architecture.TransmitterFirstOpticalTransceiver";
        let definition_part = format!("{definition_root}.transmitter");
        let architecture_root = "Architecture.transceiverSystem";
        let architecture_part = format!("{architecture_root}.transmitter");
        let context_root = "Context.operationalContext.transceiver";
        let mut ibd = IbdDataDto {
            parts: vec![IbdPartDto {
                id: definition_part.replace('.', "::"),
                node_id: definition_part.clone(),
                name: "transmitter".to_string(),
                qualified_name: definition_part.clone(),
                uri: None,
                container_id: Some(definition_root.to_string()),
                element_type: "part".to_string(),
                attributes: HashMap::new(),
                range: None,
            }],
            ports: vec![IbdPortDto {
                id: format!("{definition_part}.output").replace('.', "::"),
                port_id: format!("{definition_part}.output"),
                name: "output".to_string(),
                parent_id: definition_part.clone(),
                direction: None,
                port_type: None,
                multiplicity: Some("[1]".to_string()),
                port_side: None,
                uri: None,
                range: None,
            }],
            connectors: vec![IbdConnectorDto {
                source: format!("{architecture_part}.driver.output"),
                target: format!("{architecture_part}.modulator.input"),
                source_id: format!("{architecture_part}.driver.output"),
                target_id: format!("{architecture_part}.modulator.input"),
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            }],
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            root_views: BTreeMap::new(),
            default_root: None,
            def_instance_mappings: vec![
                DefInstanceMappingDto {
                    def_root: definition_root.to_string(),
                    instance_root: context_root.to_string(),
                },
                DefInstanceMappingDto {
                    def_root: definition_root.to_string(),
                    instance_root: architecture_root.to_string(),
                },
            ],
        };

        normalize_ibd_to_instance_paths(&mut ibd);

        assert_eq!(ibd.parts[0].qualified_name, architecture_part);
        assert_eq!(ibd.ports[0].parent_id, architecture_part);
    }

    fn part_with_id(qualified_name: &str) -> IbdPartDto {
        IbdPartDto {
            id: qualified_name.replace('.', "::"),
            node_id: qualified_name.to_string(),
            name: qualified_name
                .rsplit('.')
                .next()
                .unwrap_or(qualified_name)
                .to_string(),
            qualified_name: qualified_name.to_string(),
            uri: None,
            container_id: None,
            element_type: "part".to_string(),
            attributes: HashMap::new(),
            range: None,
        }
    }

    fn port_with_parent(qualified_parent: &str, name: &str) -> IbdPortDto {
        IbdPortDto {
            id: format!("{qualified_parent}.{name}").replace('.', "::"),
            port_id: format!("{qualified_parent}.{name}"),
            name: name.to_string(),
            parent_id: qualified_parent.to_string(),
            direction: None,
            port_type: None,
            multiplicity: None,
            port_side: None,
            uri: None,
            range: None,
        }
    }

    /// O-6 regression: like `merge_ibd_payloads_inner`, `normalize_ibd_to_instance_paths` dedups
    /// remapped parts/ports through a `HashMap` before collecting into the final `Vec`s
    /// (`parts_by_qn.into_values()`/`ports_by_id.into_values()`), which would otherwise leak that
    /// HashMap's hash-randomized bucket order into the output. Asserts the actual fix -- sorted
    /// output -- rather than trying to reproduce cross-process hash randomization (which a
    /// same-process test structurally cannot do; see the equivalent comment on
    /// `merge_output_is_sorted_by_natural_key_regardless_of_input_order` in `merge.rs`).
    #[test]
    fn normalized_parts_and_ports_are_sorted_by_natural_key() {
        let definition_root = "Architecture.Widget";
        let instance_root = "Context.system.widget";
        let mut ibd = IbdDataDto {
            parts: vec![
                part_with_id(&format!("{definition_root}.zebra")),
                part_with_id(&format!("{definition_root}.apple")),
                part_with_id(&format!("{definition_root}.mango")),
            ],
            ports: vec![
                port_with_parent(&format!("{definition_root}.zebra"), "out"),
                port_with_parent(&format!("{definition_root}.apple"), "out"),
            ],
            connectors: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            root_views: BTreeMap::new(),
            default_root: None,
            def_instance_mappings: vec![DefInstanceMappingDto {
                def_root: definition_root.to_string(),
                instance_root: instance_root.to_string(),
            }],
        };

        normalize_ibd_to_instance_paths(&mut ibd);

        let part_names: Vec<String> = ibd.parts.iter().map(|p| p.qualified_name.clone()).collect();
        assert_eq!(
            part_names,
            vec![
                format!("{instance_root}.apple"),
                format!("{instance_root}.mango"),
                format!("{instance_root}.zebra"),
            ],
            "remapped parts must be sorted by qualified_name (id)"
        );

        let port_parents: Vec<String> = ibd.ports.iter().map(|p| p.parent_id.clone()).collect();
        assert_eq!(
            port_parents,
            vec![
                format!("{instance_root}.apple"),
                format!("{instance_root}.zebra"),
            ],
            "remapped ports must be sorted by (parent_id, name)"
        );
    }
}
