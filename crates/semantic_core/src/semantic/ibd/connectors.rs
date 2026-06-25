//! Connector remapping, deduplication, and merge finalization.

use std::collections::HashMap;

use url::Url;

use crate::SemanticGraph;
use crate::semantic::model::{RelationshipKind, SemanticNode};

use super::dto::{IbdConnectorDto, IbdDataDto, IbdPartDto, IbdPortDto};
use super::extract_impl::{
    expand_relative_endpoint_to_part_path, graph_node_for_ibd_part, is_part_like,
    qualify_occurrence_endpoint, qualify_pending_connection_endpoint, qualified_name_to_dot,
    resolve_owner_part_qn_for_endpoint, resolve_port_id_for_endpoint,
};

fn map_container_endpoint_to_instance(
    endpoint: &str,
    container_def_dot: &str,
    instance_prefix_dot: &str,
) -> Option<String> {
    if endpoint == container_def_dot {
        return Some(instance_prefix_dot.to_string());
    }
    let prefixed = format!("{container_def_dot}.");
    endpoint
        .strip_prefix(&prefixed)
        .map(|remainder| format!("{instance_prefix_dot}.{remainder}"))
}

pub(crate) fn build_instance_def_mappings(
    graph: &SemanticGraph,
    uri: &Url,
    parts: &[IbdPartDto],
) -> Vec<(String, String)> {
    let mut mappings = Vec::new();
    collect_instance_def_mappings(graph, uri, parts, &mut mappings);
    finalize_instance_def_mappings(graph, &mut mappings);
    mappings
}

fn collect_instance_def_mappings(
    graph: &SemanticGraph,
    uri: &Url,
    parts: &[IbdPartDto],
    mappings: &mut Vec<(String, String)>,
) {
    for part in parts {
        if let Some(mapping) = instance_def_mapping_for_part(graph, uri, part) {
            mappings.push(mapping);
        }
    }
    for node in graph.nodes_for_uri(uri) {
        if !is_part_like(node.element_kind.as_str())
            || node.element_kind.as_str().to_lowercase().contains("part def")
        {
            continue;
        }
        let usage_dot = qualified_name_to_dot(&node.id.qualified_name);
        if mappings
            .iter()
            .any(|(_, instance_dot)| instance_dot == &usage_dot)
        {
            continue;
        }
        if let Some(def_node) = graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .find(|target| is_part_like(target.element_kind.as_str()))
        {
            let def_dot = qualified_name_to_dot(&def_node.id.qualified_name);
            mappings.push((def_dot, usage_dot));
        }
    }
}

fn finalize_instance_def_mappings(graph: &SemanticGraph, mappings: &mut Vec<(String, String)>) {
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
    mappings.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
    extend_instance_def_mappings_with_specializations(graph, mappings);
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
    mappings.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
}

fn build_workspace_instance_def_mappings(
    graph: &SemanticGraph,
    workspace_uris: &[Url],
    all_parts: &[IbdPartDto],
) -> Vec<(String, String)> {
    let mut parts_by_uri: HashMap<&str, Vec<usize>> = HashMap::new();
    for (index, part) in all_parts.iter().enumerate() {
        if let Some(uri) = part.uri.as_deref() {
            parts_by_uri.entry(uri).or_default().push(index);
        }
    }
    let mut mappings = Vec::new();
    for uri in workspace_uris {
        let empty = Vec::new();
        let indices = parts_by_uri.get(uri.as_str()).unwrap_or(&empty);
        for &index in indices {
            if let Some(mapping) =
                instance_def_mapping_for_part(graph, uri, &all_parts[index])
            {
                mappings.push(mapping);
            }
        }
        for node in graph.nodes_for_uri(uri) {
            if !is_part_like(node.element_kind.as_str())
                || node.element_kind.as_str().to_lowercase().contains("part def")
            {
                continue;
            }
            let usage_dot = qualified_name_to_dot(&node.id.qualified_name);
            if mappings
                .iter()
                .any(|(_, instance_dot)| instance_dot == &usage_dot)
            {
                continue;
            }
            if let Some(def_node) = graph
                .outgoing_typing_or_specializes_targets(node)
                .into_iter()
                .find(|target| is_part_like(target.element_kind.as_str()))
            {
                let def_dot = qualified_name_to_dot(&def_node.id.qualified_name);
                mappings.push((def_dot, usage_dot));
            }
        }
    }
    finalize_instance_def_mappings(graph, &mut mappings);
    mappings
}

fn instance_def_mapping_for_part(
    graph: &SemanticGraph,
    uri: &Url,
    part: &IbdPartDto,
) -> Option<(String, String)> {
    let node = graph_node_for_ibd_part(graph, uri, part)?;
    let def_node = graph
        .outgoing_typing_or_specializes_targets(node)
        .into_iter()
        .find(|target| is_part_like(target.element_kind.as_str()))?;
    Some((
        qualified_name_to_dot(&def_node.id.qualified_name),
        part.qualified_name.clone(),
    ))
}

fn extend_instance_def_mappings_with_specializations(
    graph: &SemanticGraph,
    mappings: &mut Vec<(String, String)>,
) {
    let initial_len = mappings.len();
    for index in 0..initial_len {
        let (def_dot, instance_dot) = mappings[index].clone();
        let def_qn = def_dot.replace('.', "::");
        let Some(def_ids) = graph.node_ids_for_qualified_name(&def_qn) else {
            continue;
        };
        for def_id in def_ids {
            let Some(def_node) = graph.get_node(def_id) else {
                continue;
            };
            let mut stack: Vec<&SemanticNode> =
                graph.incoming_typing_or_specializes_sources(def_node);
            let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
            while let Some(source) = stack.pop() {
                if !is_part_like(source.element_kind.as_str())
                    || !source.element_kind.as_str().to_lowercase().contains("part def")
                {
                    continue;
                }
                let source_dot = qualified_name_to_dot(&source.id.qualified_name);
                if !visited.insert(source_dot.clone()) {
                    continue;
                }
                mappings.push((source_dot, instance_dot.clone()));
                for next in graph.incoming_typing_or_specializes_sources(source) {
                    stack.push(next);
                }
            }
        }
    }
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
    mappings.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
}

fn remap_connector_via_mapping(
    connector: &IbdConnectorDto,
    def_dot: &str,
    instance_dot: &str,
) -> Option<IbdConnectorDto> {
    let source_id =
        map_container_endpoint_to_instance(&connector.source_id, def_dot, instance_dot)?;
    let target_id =
        map_container_endpoint_to_instance(&connector.target_id, def_dot, instance_dot)?;
    if source_id == connector.source_id && target_id == connector.target_id {
        return None;
    }
    let mut remapped = connector.clone();
    remapped.source_id = source_id.clone();
    remapped.target_id = target_id.clone();
    if remapped.source.replace("::", ".") == remapped.source_id
        || remapped.source == remapped.source_id
    {
        remapped.source = source_id;
    }
    if remapped.target.replace("::", ".") == remapped.target_id
        || remapped.target == remapped.target_id
    {
        remapped.target = target_id;
    }
    Some(remapped)
}

pub(crate) fn remap_connectors_to_typed_instances(
    connectors: Vec<IbdConnectorDto>,
    mappings: &[(String, String)],
) -> Vec<IbdConnectorDto> {
    if mappings.is_empty() {
        return connectors;
    }

    let mut expanded = Vec::with_capacity(connectors.len());
    for connector in &connectors {
        let mut remapped_any = false;
        for (def_dot, instance_dot) in mappings {
            if let Some(remapped) = remap_connector_via_mapping(connector, def_dot, instance_dot) {
                expanded.push(remapped);
                remapped_any = true;
            }
        }
        if !remapped_any {
            expanded.push(connector.clone());
        }
    }
    dedupe_connectors(expanded)
}

pub(crate) fn enrich_connector_endpoint_refs(
    connectors: &mut [IbdConnectorDto],
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
) {
    for connector in connectors.iter_mut() {
        connector.source_part_id = resolve_owner_part_qn_for_endpoint(&connector.source_id, parts);
        connector.target_part_id = resolve_owner_part_qn_for_endpoint(&connector.target_id, parts);
        connector.source_port_id = resolve_port_id_for_endpoint(&connector.source_id, ports);
        connector.target_port_id = resolve_port_id_for_endpoint(&connector.target_id, ports);
    }
}

pub(crate) fn dedupe_connectors(connectors: Vec<IbdConnectorDto>) -> Vec<IbdConnectorDto> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for connector in connectors {
        let key = (
            connector.source_id.clone(),
            connector.target_id.clone(),
            connector.rel_type.clone(),
        );
        if seen.insert(key) {
            out.push(connector);
        }
    }
    out
}

pub(crate) fn endpoint_under_definition_prefix(endpoint: &str, def_prefix: &str) -> bool {
    let endpoint_dot = qualified_name_to_dot(endpoint);
    let def_dot = qualified_name_to_dot(def_prefix);
    endpoint_dot == def_dot || endpoint_dot.starts_with(&format!("{def_dot}."))
}

pub(crate) fn map_definition_endpoint_to_usage(
    endpoint: &str,
    def_prefix: &str,
    usage_prefix_dot: &str,
) -> Option<String> {
    let endpoint_dot = qualified_name_to_dot(endpoint);
    let def_dot = qualified_name_to_dot(def_prefix);
    if endpoint_dot == def_dot {
        return Some(usage_prefix_dot.to_string());
    }
    let prefixed = format!("{def_dot}.");
    if let Some(remainder) = endpoint_dot.strip_prefix(&prefixed) {
        if remainder.is_empty() {
            return Some(usage_prefix_dot.to_string());
        }
        return Some(format!("{usage_prefix_dot}.{remainder}"));
    }
    None
}

fn def_container_prefixes_for_uri(graph: &SemanticGraph, uri: &Url) -> Vec<String> {
    graph
        .nodes_for_uri(uri)
        .iter()
        .filter(|node| node.element_kind.as_str().to_lowercase().contains("part def"))
        .map(|node| node.id.qualified_name.clone())
        .collect()
}

pub(crate) struct IbdConnectorSink<'a> {
    pub(crate) connectors: &'a mut Vec<IbdConnectorDto>,
    pub(crate) keys: &'a mut std::collections::HashSet<(String, String, String)>,
}

/// Copy connectors declared on a part definition's document onto a typed instance path.
pub(crate) fn mirror_connectors_from_definition_document(
    graph: &SemanticGraph,
    def_uri: &Url,
    def_prefix: &str,
    usage_prefix_dot: &str,
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
    sink: &mut IbdConnectorSink<'_>,
) {
    let def_prefix_dot = qualified_name_to_dot(def_prefix);
    let def_container_prefixes = def_container_prefixes_for_uri(graph, def_uri);

    let mut push_connector = |source_id: String, target_id: String| {
        let source_id = expand_relative_endpoint_to_part_path(&source_id, parts, ports);
        let target_id = expand_relative_endpoint_to_part_path(&target_id, parts, ports);
        let source_id =
            map_container_endpoint_to_instance(&source_id, &def_prefix_dot, usage_prefix_dot)
                .unwrap_or(source_id);
        let target_id =
            map_container_endpoint_to_instance(&target_id, &def_prefix_dot, usage_prefix_dot)
                .unwrap_or(target_id);
        let key = (
            source_id.clone(),
            target_id.clone(),
            "connection".to_string(),
        );
        if !sink.keys.insert(key) {
            return;
        }
        sink.connectors.push(IbdConnectorDto {
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
    };

    for (_src, _tgt, edge) in graph.connection_edges_touching_uri(def_uri) {
        if edge.kind != RelationshipKind::Connection {
            continue;
        }
        let Some(connect) = &edge.connect else {
            continue;
        };
        let source_id = qualify_pending_connection_endpoint(
            connect.container_prefix.as_deref(),
            &connect.source_expression,
        );
        let target_id = qualify_pending_connection_endpoint(
            connect.container_prefix.as_deref(),
            &connect.target_expression,
        );
        let (source_id, target_id) = if source_id.is_empty() || target_id.is_empty() {
            (
                qualify_occurrence_endpoint(&connect.source_expression, &def_container_prefixes),
                qualify_occurrence_endpoint(&connect.target_expression, &def_container_prefixes),
            )
        } else {
            (source_id, target_id)
        };
        push_connector(source_id, target_id);
    }

    for pending in graph
        .pending_expression_relationships
        .iter()
        .filter(|pending| pending.kind == RelationshipKind::Connection && pending.uri == *def_uri)
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
        push_connector(source_id, target_id);
    }

    for (src_id, tgt_id, edge) in graph.connection_edges_touching_uri(def_uri) {
        if edge.kind != RelationshipKind::Connection || edge.connect.is_some() {
            continue;
        }
        let src = src_id.qualified_name;
        let tgt = tgt_id.qualified_name;
        if !endpoint_under_definition_prefix(&src, def_prefix)
            && !endpoint_under_definition_prefix(&tgt, def_prefix)
        {
            continue;
        }
        let Some(source_id) = map_definition_endpoint_to_usage(&src, def_prefix, usage_prefix_dot)
        else {
            continue;
        };
        let Some(target_id) = map_definition_endpoint_to_usage(&tgt, def_prefix, usage_prefix_dot)
        else {
            continue;
        };
        push_connector(source_id, target_id);
    }
}
pub fn finalize_merged_ibd_connectors(
    graph: &SemanticGraph,
    workspace_uris: &[Url],
    ibd: &mut IbdDataDto,
) {
    let mappings = build_workspace_instance_def_mappings(graph, workspace_uris, &ibd.parts);
    ibd.connectors =
        remap_connectors_to_typed_instances(std::mem::take(&mut ibd.connectors), &mappings);
    for view in ibd.root_views.values_mut() {
        view.connectors =
            remap_connectors_to_typed_instances(std::mem::take(&mut view.connectors), &mappings);
    }

    ibd.connectors = dedupe_connectors(std::mem::take(&mut ibd.connectors));
    enrich_connector_endpoint_refs(&mut ibd.connectors, &ibd.parts, &ibd.ports);
    for view in ibd.root_views.values_mut() {
        view.connectors = dedupe_connectors(std::mem::take(&mut view.connectors));
        enrich_connector_endpoint_refs(&mut view.connectors, &view.parts, &view.ports);
    }
}
