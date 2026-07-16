//! Builds IBD (Internal Block Diagram) / Interconnection View data from the semantic graph.
//! Used by sysml/model to return a ready-to-render structure for the client.

use std::collections::{HashMap, HashSet};

use url::Url;

use super::connectors::{
    build_instance_def_mappings, dedupe_connectors, endpoint_under_definition_prefix,
    enrich_connector_endpoint_refs, map_definition_endpoint_to_usage,
    mirror_connectors_from_definition_document, remap_connectors_to_typed_instances,
    IbdConnectorSink,
};
use super::dto::{
    DefInstanceMappingDto, IbdConnectorDto, IbdContainerGroupDto, IbdDataDto, IbdPartDto,
    IbdPortDto, IbdRootViewDto,
};
use crate::{ElementKind, NodeId, RelationshipKind, SemanticGraph, SemanticNode};

mod container_groups;
mod endpoint_expansion;
mod endpoint_resolution;
mod kind_classify;
mod scope_pruning;
pub(crate) use container_groups::*;
pub(crate) use endpoint_expansion::*;
pub(crate) use endpoint_resolution::*;
pub(crate) use kind_classify::*;
pub(crate) use scope_pruning::*;

/// Interconnection view (BNF): `interconnection-element = part | part-ref` — no definitions on canvas.
pub(crate) fn prune_interconnection_definition_parts(
    parts: Vec<IbdPartDto>,
    ports: Vec<IbdPortDto>,
    connectors: Vec<IbdConnectorDto>,
) -> (Vec<IbdPartDto>, Vec<IbdPortDto>, Vec<IbdConnectorDto>) {
    let mut normalized_parts: Vec<IbdPartDto> = Vec::with_capacity(parts.len());
    for mut part in parts {
        if is_definition_element_kind(&part.element_type) {
            continue;
        }
        part.element_type = normalize_ibd_element_type(&part.element_type);
        decorate_ibd_part_attributes(&part.element_type, &mut part.attributes);
        normalized_parts.push(part);
    }
    let remaining: std::collections::HashSet<String> = normalized_parts
        .iter()
        .map(|part| part.qualified_name.clone())
        .collect();
    let ports = ports
        .into_iter()
        .filter(|port| remaining.contains(&port.parent_id))
        .collect();
    let connectors = connectors
        .into_iter()
        .filter(|connector| {
            resolve_owner_part_qn_for_endpoint(&connector.source_id, &normalized_parts).is_some()
                && resolve_owner_part_qn_for_endpoint(&connector.target_id, &normalized_parts)
                    .is_some()
        })
        .collect();
    (normalized_parts, ports, connectors)
}

/// True if the element kind represents a port (port def or port usage). Public for semantic_checks.
pub fn is_port_like(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k.contains("port def") || k == "port"
}

/// Count of part nodes in the subtree (direct + recursive). Uses typing to follow part def structure.
fn part_tree_size(graph: &SemanticGraph, node: &SemanticNode, _uri: &Url) -> usize {
    let mut visiting_defs: std::collections::HashSet<String> = std::collections::HashSet::new();
    part_tree_size_inner(graph, node, _uri, &mut visiting_defs)
}

fn part_tree_size_inner(
    graph: &SemanticGraph,
    node: &SemanticNode,
    _uri: &Url,
    visiting_defs: &mut std::collections::HashSet<String>,
) -> usize {
    let children = graph.children_of(node);
    let part_children: Vec<_> = children
        .iter()
        .filter(|c| is_part_like(c.element_kind.as_str()))
        .collect();
    part_children
        .iter()
        .map(|c| {
            let typed = graph.outgoing_typing_or_specializes_targets(c);
            let def = typed.into_iter().next();
            if let Some(def_node) = def {
                if is_part_like(def_node.element_kind.as_str()) {
                    let def_key = def_node.id.qualified_name.clone();
                    if !visiting_defs.insert(def_key.clone()) {
                        // Break recursive type cycles (A -> B -> A).
                        return 1;
                    }
                    let size = 1 + part_tree_size_inner(graph, def_node, _uri, visiting_defs);
                    visiting_defs.remove(&def_key);
                    return size;
                }
            }
            1 + part_tree_size_inner(graph, c, _uri, visiting_defs)
        })
        .sum()
}

/// Qualified name with "::" converted to "." for client path matching (e.g. "pkg::A::b" -> "A.b" when root is "A").
pub fn qualified_name_to_dot(qn: &str) -> String {
    qn.replace("::", ".")
}

fn canonical_port_id(parent_id: &str, port_name: &str) -> String {
    let parent = qualified_name_to_dot(parent_id);
    if parent.is_empty() {
        port_name.to_string()
    } else {
        format!("{parent}.{port_name}")
    }
}

pub(crate) fn graph_node_for_ibd_part<'a>(
    graph: &'a SemanticGraph,
    uri: &Url,
    part: &IbdPartDto,
) -> Option<&'a SemanticNode> {
    if let Some(node) = graph.get_node(&NodeId::new(uri, &part.id)) {
        return Some(node);
    }
    let colon_id = part.id.replace('.', "::");
    if colon_id != part.id {
        if let Some(node) = graph.get_node(&NodeId::new(uri, &colon_id)) {
            return Some(node);
        }
    }
    let colon_qn = part.qualified_name.replace('.', "::");
    graph
        .node_ids_for_qualified_name(&colon_qn)
        .and_then(|ids| {
            ids.iter()
                .find(|id| id.uri == *uri)
                .or_else(|| ids.first())
                .cloned()
        })
        .and_then(|id| graph.get_node(&id))
}

/// Builds IBD data for the given URI from the semantic graph.
pub fn build_ibd_for_uri(graph: &SemanticGraph, uri: &Url) -> IbdDataDto {
    let nodes = graph.nodes_for_uri(uri);

    let mut parts = Vec::new();
    let mut ports = Vec::new();

    for node in &nodes {
        let qn = node.id.qualified_name.clone();
        let parent_qualified = node.parent_id.as_ref().map(|p| p.qualified_name.clone());

        if is_interconnection_element_kind(node.element_kind.as_str()) {
            let container_id = node.parent_id.as_ref().and_then(|pid| {
                graph.get_node(pid).and_then(|p| {
                    if is_interconnection_element_kind(p.element_kind.as_str()) {
                        Some(qualified_name_to_dot(&pid.qualified_name))
                    } else {
                        None
                    }
                })
            });
            parts.push(IbdPartDto {
                id: qn.clone(),
                node_id: qualified_name_to_dot(&qn),
                name: node.name.clone(),
                qualified_name: qualified_name_to_dot(&qn),
                uri: Some(node.id.uri.as_str().to_string()),
                container_id: container_id.map(|s| qualified_name_to_dot(&s)),
                element_type: node.element_kind.as_str().to_string(),
                attributes: node.attributes.clone(),
                range: Some(crate::semantic::dto::range_to_dto(node.range)),
            });
        } else if is_port_like(node.element_kind.as_str()) {
            let parent_id = parent_qualified
                .as_ref()
                .map(|pq| qualified_name_to_dot(pq))
                .unwrap_or_else(|| node.name.clone());
            let direction = node
                .attributes
                .get("direction")
                .and_then(|v| v.as_str())
                .map(String::from);
            let port_type = node
                .attributes
                .get("portType")
                .and_then(|v| v.as_str())
                .map(String::from);
            let port_side = infer_port_side(&node.name, direction.as_deref(), port_type.as_deref());
            ports.push(IbdPortDto {
                id: node.id.qualified_name.clone(),
                port_id: canonical_port_id(&parent_id, &node.name),
                name: node.name.clone(),
                parent_id,
                direction,
                port_type,
                port_side,
                uri: Some(node.id.uri.as_str().to_string()),
                range: Some(crate::semantic::dto::range_to_dto(node.range)),
            });
        }
    }

    // Interconnection view: expand typed part trees via component_view (shared with other consumers).
    let mut existing_part_qn_dot: std::collections::HashSet<String> =
        parts.iter().map(|p| p.qualified_name.clone()).collect();
    let mut existing_ports: std::collections::HashSet<(String, String)> = ports
        .iter()
        .map(|p| (p.parent_id.clone(), p.name.clone()))
        .collect();

    // Collect (part_qualified_name, def_node_id) for connector mirroring below.
    let mut typed_roots: Vec<(String, NodeId)> = Vec::new();

    let parts_snapshot = parts.clone();
    for p in &parts_snapshot {
        let Some(node) = graph_node_for_ibd_part(graph, uri, p) else {
            continue;
        };
        let Some(def_node) =
            crate::semantic::component_view::first_typed_definition_with_shape(graph, node)
        else {
            continue;
        };
        typed_roots.push((p.qualified_name.clone(), def_node.id.clone()));
        let parent_dot = p.qualified_name.as_str();
        let expanded = crate::semantic::component_view::expand_part_definition(
            graph, def_node, parent_dot, None,
        );
        for ep in &expanded {
            if !existing_part_qn_dot.insert(ep.path.clone()) {
                continue;
            }
            parts.push(IbdPartDto {
                id: ep.path.clone(),
                node_id: ep.path.clone(),
                name: ep.name.clone(),
                qualified_name: ep.path.clone(),
                uri: ep.uri.as_ref().map(|u| u.as_str().to_string()),
                container_id: ep.parent_path.clone(),
                element_type: ep.element_kind.clone(),
                attributes: ep.attributes.clone(),
                range: graph
                    .get_node(&ep.node_id)
                    .map(|node| crate::semantic::dto::range_to_dto(node.range)),
            });
            for port in &ep.ports {
                let key = (port.parent_path.clone(), port.name.clone());
                if existing_ports.insert(key) {
                    ports.push(expanded_port_to_ibd_dto(graph, port));
                }
            }
        }
        // Also add ports for the root part itself from the definition.
        for port in crate::semantic::component_view::inherited_ports(graph, def_node, parent_dot) {
            let key = (port.parent_path.clone(), port.name.clone());
            if existing_ports.insert(key) {
                ports.push(expanded_port_to_ibd_dto(graph, &port));
            }
        }
    }

    let def_container_prefixes: Vec<String> = nodes
        .iter()
        .filter(|node| {
            node.element_kind
                .as_str()
                .to_lowercase()
                .contains("part def")
        })
        .map(|node| node.id.qualified_name.clone())
        .collect();

    let mut connectors = Vec::new();
    for (src_id, tgt_id, edge) in graph.connection_edges_touching_uri(uri) {
        if edge.kind != RelationshipKind::Connection {
            continue;
        }
        let source = src_id.qualified_name.clone();
        let target = tgt_id.qualified_name.clone();
        let (source_id, target_id) = if let Some(connect) = &edge.connect {
            let source_id = if connect.container_prefix.is_some() {
                qualify_pending_connection_endpoint(
                    connect.container_prefix.as_deref(),
                    &connect.source_expression,
                )
            } else {
                qualify_occurrence_endpoint(&connect.source_expression, &def_container_prefixes)
            };
            let target_id = if connect.container_prefix.is_some() {
                qualify_pending_connection_endpoint(
                    connect.container_prefix.as_deref(),
                    &connect.target_expression,
                )
            } else {
                qualify_occurrence_endpoint(&connect.target_expression, &def_container_prefixes)
            };
            (source_id, target_id)
        } else {
            (source.replace("::", "."), target.replace("::", "."))
        };
        connectors.push(IbdConnectorDto {
            source: source.clone(),
            target: target.clone(),
            source_id,
            target_id,
            source_part_id: None,
            target_part_id: None,
            source_port_id: None,
            target_port_id: None,
            rel_type: "connection".to_string(),
        });
    }
    for pending in graph
        .pending_expression_relationships
        .iter()
        .filter(|pending| pending.kind == RelationshipKind::Connection && &pending.uri == uri)
    {
        let source_id = qualify_pending_connection_endpoint(
            pending.container_prefix.as_deref(),
            &pending.source_expression,
        );
        let target_id = qualify_pending_connection_endpoint(
            pending.container_prefix.as_deref(),
            &pending.target_expression,
        );
        if source_id.is_empty() || target_id.is_empty() {
            continue;
        }
        connectors.push(IbdConnectorDto {
            source: source_id.clone(),
            target: target_id.clone(),
            source_id,
            target_id,
            source_part_id: None,
            target_part_id: None,
            source_port_id: None,
            target_port_id: None,
            rel_type: "connection".to_string(),
        });
    }

    let instance_def_mappings = build_instance_def_mappings(graph, uri, &parts);

    // Mirror definition-level connectors into usage-instance paths so interconnection
    // rendering stays instance-centric for selected roots.
    let mut connector_keys: std::collections::HashSet<(String, String, String)> = connectors
        .iter()
        .map(|c| (c.source_id.clone(), c.target_id.clone(), c.rel_type.clone()))
        .collect();
    for (usage_prefix_dot, def_id) in &typed_roots {
        let def_prefix = def_id.qualified_name.as_str();
        let usage_prefix_dot = usage_prefix_dot.as_str();
        if def_id.uri != *uri {
            mirror_connectors_from_definition_document(
                graph,
                &def_id.uri,
                def_prefix,
                usage_prefix_dot,
                &parts,
                &ports,
                &mut IbdConnectorSink {
                    connectors: &mut connectors,
                    keys: &mut connector_keys,
                },
            );
            continue;
        }
        let def_edges = graph.edges_for_uri_as_strings(&def_id.uri);
        for (src, tgt, kind, _name) in &def_edges {
            if *kind != RelationshipKind::Connection {
                continue;
            }
            if !endpoint_under_definition_prefix(src, def_prefix)
                || !endpoint_under_definition_prefix(tgt, def_prefix)
            {
                continue;
            }
            let Some(source_id) =
                map_definition_endpoint_to_usage(src, def_prefix, usage_prefix_dot)
            else {
                continue;
            };
            let Some(target_id) =
                map_definition_endpoint_to_usage(tgt, def_prefix, usage_prefix_dot)
            else {
                continue;
            };
            let key = (
                source_id.clone(),
                target_id.clone(),
                "connection".to_string(),
            );
            if !connector_keys.insert(key) {
                continue;
            }
            connectors.push(IbdConnectorDto {
                source: source_id.clone(),
                target: target_id.clone(),
                source_id,
                target_id,
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            });
        }
    }

    connectors = remap_connectors_to_typed_instances(connectors, &instance_def_mappings);
    let port_names_by_parent = build_port_names_by_parent(&ports);
    for connector in &mut connectors {
        connector.source_id = expand_relative_endpoint_with_port_index(
            &connector.source_id,
            &parts,
            &port_names_by_parent,
        );
        connector.target_id = expand_relative_endpoint_with_port_index(
            &connector.target_id,
            &parts,
            &port_names_by_parent,
        );
        if connector.source == connector.source_id {
            connector.source = connector.source_id.clone();
        }
        if connector.target == connector.target_id {
            connector.target = connector.target_id.clone();
        }
    }
    let connectors = dedupe_connectors(connectors);

    ensure_endpoint_parts_present(&mut parts, &connectors, graph, uri);

    let (parts, ports, connectors) =
        prune_interconnection_definition_parts(parts, ports, connectors);
    let (parts, ports, connectors) = prune_ibd_payload_to_connected_scope(parts, ports, connectors);
    let mut connectors = connectors;
    enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);
    let container_groups = build_container_groups(&parts, &|prefix| {
        let colon = prefix.replace('.', "::");
        graph
            .get_node(&NodeId::new(uri, &colon))
            .map(|node| !is_part_like(node.element_kind.as_str()))
            .unwrap_or(false)
    });

    let top_level_parts: Vec<_> = parts
        .iter()
        .filter(|p| is_top_level_part(p, &parts))
        .collect();

    let mut roots_with_metrics: Vec<(&IbdPartDto, usize, usize, usize)> = top_level_parts
        .iter()
        .filter(|p| is_part_instance_kind(&p.element_type))
        .filter_map(|p| {
            let root_prefix = p.qualified_name.as_str();
            let (part_count, port_count, connector_count) =
                materialized_subtree_metrics(root_prefix, &parts, &ports, &connectors);
            let has_materialized_structure =
                part_count > 1 || port_count > 0 || connector_count > 0;
            if has_materialized_structure {
                let tree_size = graph
                    .get_node(&NodeId::new(uri, &p.id))
                    .map(|node| part_tree_size(graph, node, uri))
                    .unwrap_or(part_count.saturating_sub(1));
                Some((
                    *p,
                    tree_size.max(part_count.saturating_sub(1)),
                    port_count,
                    connector_count,
                ))
            } else {
                None
            }
        })
        .collect();

    // Fallback: if no explicit instances are available, keep previous behavior.
    if roots_with_metrics.is_empty() {
        roots_with_metrics = top_level_parts
            .iter()
            .filter_map(|p| {
                let root_prefix = p.qualified_name.as_str();
                let (part_count, port_count, connector_count) =
                    materialized_subtree_metrics(root_prefix, &parts, &ports, &connectors);
                let has_materialized_structure =
                    part_count > 1 || port_count > 0 || connector_count > 0;
                if has_materialized_structure {
                    let tree_size = graph
                        .get_node(&NodeId::new(uri, &p.id))
                        .map(|node| part_tree_size(graph, node, uri))
                        .unwrap_or(part_count.saturating_sub(1));
                    Some((
                        *p,
                        tree_size.max(part_count.saturating_sub(1)),
                        port_count,
                        connector_count,
                    ))
                } else {
                    None
                }
            })
            .collect();
    }

    roots_with_metrics.sort_by(|a, b| {
        let a_instance = is_part_instance_kind(&a.0.element_type);
        let b_instance = is_part_instance_kind(&b.0.element_type);
        match b_instance.cmp(&a_instance) {
            std::cmp::Ordering::Equal => {
                let a_score = a.3 * 1000 + a.2 * 10 + a.1;
                let b_score = b.3 * 1000 + b.2 * 10 + b.1;
                b_score.cmp(&a_score).then_with(|| a.0.name.cmp(&b.0.name))
            }
            other => other,
        }
    });

    let root_candidates: Vec<String> = roots_with_metrics
        .iter()
        .map(|(p, _, _, _)| p.name.clone())
        .collect();
    let default_root = root_candidates.first().cloned();
    let mut root_views: std::collections::HashMap<String, IbdRootViewDto> =
        std::collections::HashMap::new();
    for (p, _, _, _) in &roots_with_metrics {
        let root_prefix = p.qualified_name.as_str();
        let focused_connectors: Vec<IbdConnectorDto> = connectors
            .iter()
            .filter(|connector| {
                endpoint_matches_root(&connector.source_id, root_prefix)
                    || endpoint_matches_root(&connector.target_id, root_prefix)
            })
            .cloned()
            .collect();
        let mut focused_part_ids: std::collections::HashSet<String> = parts
            .iter()
            .filter(|part| endpoint_matches_root(&part.qualified_name, root_prefix))
            .map(|part| part.qualified_name.clone())
            .collect();
        for connector in &focused_connectors {
            for endpoint in [&connector.source_id, &connector.target_id] {
                if let Some(part) = parts
                    .iter()
                    .filter(|part| endpoint_matches_part(endpoint, &part.qualified_name))
                    .max_by_key(|part| part.qualified_name.len())
                {
                    focused_part_ids.insert(part.qualified_name.clone());
                }
            }
        }
        let focused_parts: Vec<IbdPartDto> = parts
            .iter()
            .filter(|part| focused_part_ids.contains(&part.qualified_name))
            .cloned()
            .collect();
        let focused_ports: Vec<IbdPortDto> = ports
            .iter()
            .filter(|port| focused_part_ids.contains(&port.parent_id))
            .cloned()
            .collect();
        let focused_container_groups: Vec<IbdContainerGroupDto> = container_groups
            .iter()
            .filter(|group| {
                focused_parts
                    .iter()
                    .any(|part| group.member_part_ids.contains(&part.id))
            })
            .cloned()
            .collect();
        root_views.insert(
            p.name.clone(),
            IbdRootViewDto {
                parts: focused_parts,
                ports: focused_ports,
                connectors: focused_connectors,
                container_groups: focused_container_groups,
                package_container_groups: Vec::new(),
            },
        );
    }

    let def_instance_mappings = instance_def_mappings
        .into_iter()
        .map(|(def_root, instance_root)| DefInstanceMappingDto {
            def_root,
            instance_root,
        })
        .collect();

    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        default_root,
        root_views,
        def_instance_mappings,
    }
}

#[cfg(test)]
mod tests;
