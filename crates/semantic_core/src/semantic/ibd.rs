//! Builds IBD (Internal Block Diagram) / Interconnection View data from the semantic graph.
//! Used by sysml/model to return a ready-to-render structure for the client.

use serde::Serialize;
use url::Url;

use crate::{NodeId, RelationshipKind, SemanticGraph, SemanticNode};

pub(crate) fn is_part_like(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k.contains("part def") || k == "part" || (k.contains("part") && !k.contains("def"))
}

/// BNF interconnection-element: part usage or part-ref (not definitions).
fn is_interconnection_element_kind(kind: &str) -> bool {
    is_part_like(kind) || is_reference_element_kind(kind)
}

fn is_part_instance_kind(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k == "part" || k.contains("part usage")
}

/// True when the element kind is a SysML definition (not valid as interconnection-element).
fn is_definition_element_kind(kind: &str) -> bool {
    let k = kind.trim().to_lowercase();
    k.contains(" def") || k.ends_with(" def") || k.contains("_def") || k.contains("definition")
}

fn is_reference_element_kind(kind: &str) -> bool {
    let k = kind.trim().to_lowercase();
    if k == "ref" {
        return true;
    }
    if k.ends_with("-ref") || k.ends_with(" ref") {
        return true;
    }
    k.split_whitespace().any(|token| token == "ref")
}

/// Normalize graph element_kind to interconnection-view `type` for the diagram renderer.
fn normalize_ibd_element_type(kind: &str) -> String {
    if is_reference_element_kind(kind) {
        return "ref".to_string();
    }
    if is_part_instance_kind(kind) {
        return "part".to_string();
    }
    kind.trim().to_string()
}

fn decorate_ibd_part_attributes(
    element_type: &str,
    attributes: &mut std::collections::HashMap<String, serde_json::Value>,
) {
    attributes.insert(
        "isReference".to_string(),
        serde_json::json!(is_reference_element_kind(element_type)),
    );
    attributes.insert(
        "isDefinition".to_string(),
        serde_json::json!(is_definition_element_kind(element_type)),
    );
}

/// Interconnection view (BNF): `interconnection-element = part | part-ref` — no definitions on canvas.
fn prune_interconnection_definition_parts(
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
        .filter(|c| is_part_like(&c.element_kind))
        .collect();
    part_children
        .iter()
        .map(|c| {
            let typed = graph.outgoing_typing_or_specializes_targets(c);
            let def = typed.into_iter().next();
            if let Some(def_node) = def {
                if is_part_like(&def_node.element_kind) {
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdPartDto {
    pub id: String,
    pub node_id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: String,
    pub attributes: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdPortDto {
    pub id: String,
    pub port_id: String,
    pub name: String,
    pub parent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_side: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdConnectorDto {
    pub source: String,
    pub target: String,
    pub source_id: String,
    pub target_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_part_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_part_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port_id: Option<String>,
    #[serde(rename = "type")]
    pub rel_type: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdContainerGroupDto {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub member_part_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdPackageContainerGroupDto {
    pub id: String,
    pub label: String,
    pub qualified_package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub member_part_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdDataDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
    pub container_groups: Vec<IbdContainerGroupDto>,
    pub package_container_groups: Vec<IbdPackageContainerGroupDto>,
    pub root_candidates: Vec<String>,
    pub default_root: Option<String>,
    pub root_views: std::collections::HashMap<String, IbdRootViewDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdRootViewDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
    pub container_groups: Vec<IbdContainerGroupDto>,
    pub package_container_groups: Vec<IbdPackageContainerGroupDto>,
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

fn graph_node_for_ibd_part<'a>(
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

fn push_inherited_ports_from_definition(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    parent_dot: &str,
    ports_out: &mut Vec<IbdPortDto>,
    existing_ports: &mut std::collections::HashSet<(String, String)>,
    visiting: &mut std::collections::HashSet<String>,
) {
    let def_key = def_node.id.qualified_name.clone();
    if !visiting.insert(def_key) {
        return;
    }
    for generalization in graph.outgoing_typing_or_specializes_targets(def_node) {
        if is_part_like(&generalization.element_kind) {
            push_inherited_ports_from_definition(
                graph,
                generalization,
                parent_dot,
                ports_out,
                existing_ports,
                visiting,
            );
        }
    }
    for child in graph.children_of(def_node) {
        if !is_port_like(&child.element_kind) {
            continue;
        }
        let key = (parent_dot.to_string(), child.name.clone());
        if existing_ports.contains(&key) {
            continue;
        }
        existing_ports.insert(key);
        let direction = child
            .attributes
            .get("direction")
            .and_then(|v| v.as_str())
            .map(String::from);
        let port_type = child
            .attributes
            .get("portType")
            .and_then(|v| v.as_str())
            .map(String::from);
        let port_side = infer_port_side(&child.name, direction.as_deref(), port_type.as_deref());
        ports_out.push(IbdPortDto {
            id: canonical_port_id(parent_dot, &child.name),
            port_id: canonical_port_id(parent_dot, &child.name),
            name: child.name.clone(),
            parent_id: parent_dot.to_string(),
            direction,
            port_type,
            port_side,
        });
    }
    visiting.remove(&def_node.id.qualified_name);
}

fn add_inherited_ports_from_definition(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    parent_dot: &str,
    ports_out: &mut Vec<IbdPortDto>,
    existing_ports: &mut std::collections::HashSet<(String, String)>,
) {
    let mut visiting = std::collections::HashSet::new();
    push_inherited_ports_from_definition(
        graph,
        def_node,
        parent_dot,
        ports_out,
        existing_ports,
        &mut visiting,
    );
}

fn infer_port_side(
    name: &str,
    direction: Option<&str>,
    _port_type: Option<&str>,
) -> Option<String> {
    let normalized_name = name.trim().to_lowercase();
    let normalized_direction = direction.unwrap_or("").trim().to_lowercase();

    match normalized_direction.as_str() {
        "in" | "input" => return Some("left".to_string()),
        "out" | "output" => return Some("right".to_string()),
        _ => {}
    }

    let tokens: Vec<&str> = normalized_name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    let trailing = tokens.last().copied().unwrap_or("");

    if trailing == "in"
        || trailing == "input"
        || normalized_name.ends_with("input")
        || normalized_name.ends_with("in")
    {
        return Some("left".to_string());
    }
    if trailing == "out"
        || trailing == "output"
        || normalized_name.ends_with("output")
        || normalized_name.ends_with("out")
    {
        return Some("right".to_string());
    }

    // Generic semantic hints for producer/provider style names.
    if normalized_name.contains("provide")
        || normalized_name.contains("supply")
        || normalized_name.contains("source")
        || normalized_name.starts_with("regulated")
    {
        return Some("right".to_string());
    }
    if normalized_name.contains("consume")
        || normalized_name.contains("demand")
        || normalized_name.contains("sink")
    {
        return Some("left".to_string());
    }

    None
}

fn endpoint_matches_root(endpoint: &str, root_prefix: &str) -> bool {
    endpoint == root_prefix || endpoint.starts_with(&format!("{root_prefix}."))
}

fn endpoint_matches_part(endpoint: &str, part_qn_dot: &str) -> bool {
    endpoint == part_qn_dot || endpoint.starts_with(&format!("{part_qn_dot}."))
}

pub(crate) fn resolve_owner_part_qn_for_endpoint(
    endpoint: &str,
    parts: &[IbdPartDto],
) -> Option<String> {
    let endpoint = qualified_name_to_dot(endpoint);
    parts
        .iter()
        .filter(|part| endpoint_matches_part(&endpoint, &part.qualified_name))
        .max_by_key(|part| part.qualified_name.len())
        .map(|part| part.qualified_name.clone())
}

pub(crate) fn resolve_port_id_for_endpoint(endpoint: &str, ports: &[IbdPortDto]) -> Option<String> {
    let endpoint_dot = qualified_name_to_dot(endpoint);
    ports
        .iter()
        .find(|port| {
            endpoint_dot == qualified_name_to_dot(&port.id)
                || endpoint_dot == qualified_name_to_dot(&port.port_id)
                || endpoint_dot == canonical_port_id(&port.parent_id, &port.name)
        })
        .map(|port| port.port_id.clone())
}

/// Drops unrelated assemblies, but keeps every part nested under a composite that
/// already contains a connector endpoint (including unconnected siblings).
fn prune_ibd_payload_to_connected_scope(
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

fn resolve_endpoint_anchor_node<'a>(
    graph: &'a SemanticGraph,
    uri: &Url,
    endpoint_id: &str,
) -> Option<&'a SemanticNode> {
    let mut candidate = endpoint_id.replace('.', "::");
    loop {
        let node_id = NodeId::new(uri, &candidate);
        if let Some(node) = graph.get_node(&node_id) {
            if is_port_like(&node.element_kind) {
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

fn ensure_endpoint_parts_present(
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
        if is_definition_element_kind(&node.element_kind) {
            continue;
        }
        let element_type = normalize_ibd_element_type(&node.element_kind);
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
        });
    }
}

fn materialized_subtree_metrics(
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

fn attribute_text(part: &IbdPartDto, key: &str) -> Option<String> {
    part.attributes
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.trim().trim_start_matches('~').to_string())
        .filter(|value| !value.is_empty())
}

fn last_type_segment(value: &str) -> String {
    value
        .rsplit("::")
        .next()
        .unwrap_or(value)
        .rsplit('.')
        .next()
        .unwrap_or(value)
        .to_string()
}

fn typed_by_name(part: &IbdPartDto) -> Option<String> {
    attribute_text(part, "partType")
        .or_else(|| attribute_text(part, "type"))
        .or_else(|| attribute_text(part, "typedBy"))
        .map(|value| last_type_segment(&value))
}

fn is_top_level_part(part: &IbdPartDto, parts: &[IbdPartDto]) -> bool {
    match part.container_id.as_deref() {
        None | Some("") => true,
        Some(container_id) => !parts
            .iter()
            .any(|candidate| candidate.qualified_name == container_id),
    }
}

#[allow(dead_code)]
fn prune_redundant_top_level_roots(
    parts: Vec<IbdPartDto>,
    ports: Vec<IbdPortDto>,
    connectors: Vec<IbdConnectorDto>,
    uri: &Url,
) -> (Vec<IbdPartDto>, Vec<IbdPortDto>, Vec<IbdConnectorDto>) {
    let top_level_parts: Vec<&IbdPartDto> = parts
        .iter()
        .filter(|part| is_top_level_part(part, &parts))
        .collect();

    let root_metrics: std::collections::HashMap<String, (usize, usize, usize)> = top_level_parts
        .iter()
        .map(|part| {
            (
                part.qualified_name.clone(),
                materialized_subtree_metrics(&part.qualified_name, &parts, &ports, &connectors),
            )
        })
        .collect();
    let top_level_defs_by_name: std::collections::HashMap<String, &IbdPartDto> = top_level_parts
        .iter()
        .filter(|part| part.element_type.to_lowercase().contains("part def"))
        .map(|part| (part.name.clone(), *part))
        .collect();

    let mut redundant_roots: std::collections::HashSet<String> = std::collections::HashSet::new();
    for root in &top_level_parts {
        if is_part_instance_kind(&root.element_type) {
            if let Some(typed_name) = typed_by_name(root) {
                if let Some(def_root) = top_level_defs_by_name.get(&typed_name) {
                    let root_score = root_metrics
                        .get(&root.qualified_name)
                        .copied()
                        .unwrap_or((0, 0, 0));
                    let def_score = root_metrics
                        .get(&def_root.qualified_name)
                        .copied()
                        .unwrap_or((0, 0, 0));
                    if def_score.0 >= root_score.0
                        && def_score.1 >= root_score.1
                        && def_score.2 >= root_score.2
                    {
                        redundant_roots.insert(root.qualified_name.clone());
                    }
                }
            }
            continue;
        }

        let root_score = root_metrics
            .get(&root.qualified_name)
            .copied()
            .unwrap_or((0, 0, 0));
        let represented_elsewhere = top_level_parts.iter().any(|other_root| {
            if other_root.qualified_name == root.qualified_name {
                return false;
            }
            parts
                .iter()
                .filter(|candidate| {
                    candidate.qualified_name != root.qualified_name
                        && endpoint_matches_root(
                            &candidate.qualified_name,
                            &other_root.qualified_name,
                        )
                })
                .filter(|candidate| typed_by_name(candidate).as_deref() == Some(root.name.as_str()))
                .any(|candidate| {
                    let nested_score = materialized_subtree_metrics(
                        &candidate.qualified_name,
                        &parts,
                        &ports,
                        &connectors,
                    );
                    nested_score.0 >= root_score.0
                        && nested_score.1 >= root_score.1
                        && nested_score.2 >= root_score.2
                })
        });
        if represented_elsewhere {
            redundant_roots.insert(root.qualified_name.clone());
        }
    }

    if redundant_roots.is_empty() {
        return (parts, ports, connectors);
    }

    let parts: Vec<IbdPartDto> = parts
        .into_iter()
        .filter(|part| {
            !redundant_roots
                .iter()
                .any(|root_prefix| endpoint_matches_root(&part.qualified_name, root_prefix))
        })
        .collect();
    let remaining_part_ids: std::collections::HashSet<String> = parts
        .iter()
        .map(|part| part.qualified_name.clone())
        .collect();
    let ports: Vec<IbdPortDto> = ports
        .into_iter()
        .filter(|port| remaining_part_ids.contains(&port.parent_id))
        .collect();
    let connectors: Vec<IbdConnectorDto> = connectors
        .into_iter()
        .filter(|connector| {
            !redundant_roots.iter().any(|root_prefix| {
                endpoint_matches_root(&connector.source_id, root_prefix)
                    || endpoint_matches_root(&connector.target_id, root_prefix)
            })
        })
        .collect();

    #[cfg(debug_assertions)]
    {
        if !redundant_roots.is_empty() {
            eprintln!(
                "[IBD] pruned redundant top-level roots for {}: {}",
                uri,
                redundant_roots
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    (parts, ports, connectors)
}

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

fn build_instance_def_mappings(
    graph: &SemanticGraph,
    uri: &Url,
    parts: &[IbdPartDto],
) -> Vec<(String, String)> {
    let mut mappings: Vec<(String, String)> = Vec::new();
    for part in parts {
        if let Some(mapping) = instance_def_mapping_for_part(graph, uri, part) {
            mappings.push(mapping);
        }
    }
    for node in graph.nodes_for_uri(uri) {
        if !is_part_like(&node.element_kind)
            || node.element_kind.to_lowercase().contains("part def")
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
            .find(|target| is_part_like(&target.element_kind))
        {
            let def_dot = qualified_name_to_dot(&def_node.id.qualified_name);
            mappings.push((def_dot, usage_dot));
        }
    }
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
    mappings.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
    extend_instance_def_mappings_with_specializations(graph, &mut mappings);
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
        .find(|target| is_part_like(&target.element_kind))?;
    Some((
        qualified_name_to_dot(&def_node.id.qualified_name),
        part.qualified_name.clone(),
    ))
}

fn extend_instance_def_mappings_with_specializations(
    graph: &SemanticGraph,
    mappings: &mut Vec<(String, String)>,
) {
    let seed = mappings.clone();
    for (def_dot, instance_dot) in seed {
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
                if !is_part_like(&source.element_kind)
                    || !source.element_kind.to_lowercase().contains("part def")
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

fn remap_connectors_to_typed_instances(
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

fn dedupe_connectors(connectors: Vec<IbdConnectorDto>) -> Vec<IbdConnectorDto> {
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

fn endpoint_under_definition_prefix(endpoint: &str, def_prefix: &str) -> bool {
    let endpoint_dot = qualified_name_to_dot(endpoint);
    let def_dot = qualified_name_to_dot(def_prefix);
    endpoint_dot == def_dot || endpoint_dot.starts_with(&format!("{def_dot}."))
}

fn map_definition_endpoint_to_usage(
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
        .filter(|node| node.element_kind.to_lowercase().contains("part def"))
        .map(|node| node.id.qualified_name.clone())
        .collect()
}

struct IbdConnectorSink<'a> {
    connectors: &'a mut Vec<IbdConnectorDto>,
    keys: &'a mut std::collections::HashSet<(String, String, String)>,
}

/// Copy connectors declared on a part definition's document onto a typed instance path.
fn mirror_connectors_from_definition_document(
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

fn qualify_pending_connection_endpoint(container_prefix: Option<&str>, endpoint: &str) -> String {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.contains("::") {
        return trimmed.replace("::", ".");
    }
    let Some(prefix) = container_prefix
        .map(str::trim)
        .filter(|prefix| !prefix.is_empty())
    else {
        return trimmed.to_string();
    };
    let prefix_dot = prefix.replace("::", ".");
    if trimmed == prefix_dot || trimmed.starts_with(&format!("{prefix_dot}.")) {
        trimmed.to_string()
    } else {
        format!("{prefix_dot}.{trimmed}")
    }
}

fn qualify_occurrence_endpoint(endpoint: &str, def_container_prefixes: &[String]) -> String {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.contains("::") {
        return trimmed.replace("::", ".");
    }
    if def_container_prefixes.is_empty() {
        return trimmed.to_string();
    }
    let mut qualified: Vec<String> = def_container_prefixes
        .iter()
        .map(|prefix| qualify_pending_connection_endpoint(Some(prefix.as_str()), trimmed))
        .filter(|candidate| !candidate.is_empty())
        .collect();
    qualified.sort_by_key(|candidate| std::cmp::Reverse(candidate.len()));
    qualified
        .into_iter()
        .next()
        .unwrap_or_else(|| trimmed.to_string())
}

fn expand_relative_endpoint_to_part_path(
    endpoint: &str,
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
) -> String {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() || trimmed.contains("::") {
        return trimmed.replace("::", ".");
    }
    let segments: Vec<&str> = trimmed.split('.').collect();
    if segments.len() < 2 {
        return trimmed.to_string();
    }
    let port_name = segments[segments.len() - 1];
    let path_without_port = segments[..segments.len() - 1].join(".");

    let mut best_match: Option<(String, bool, usize)> = None;
    for part in parts {
        if !ports
            .iter()
            .any(|port| port.parent_id == part.qualified_name && port.name == port_name)
        {
            continue;
        }
        if path_without_port == part.qualified_name
            || path_without_port.ends_with(&format!(".{}", part.qualified_name))
            || part
                .qualified_name
                .ends_with(&format!(".{path_without_port}"))
        {
            let candidate = format!("{}.{}", part.qualified_name, port_name);
            let is_instance = is_part_instance_kind(&part.element_type);
            let is_better = best_match
                .as_ref()
                .is_none_or(|(_, best_instance, best_len)| {
                    if is_instance && !*best_instance {
                        true
                    } else if is_instance == *best_instance {
                        if is_instance {
                            candidate.len() < *best_len
                        } else {
                            candidate.len() > *best_len
                        }
                    } else {
                        false
                    }
                });
            if is_better {
                let candidate_len = candidate.len();
                best_match = Some((candidate, is_instance, candidate_len));
            }
        }
    }
    best_match
        .map(|(path, _, _)| path)
        .unwrap_or_else(|| trimmed.to_string())
}

fn build_container_groups(parts: &[IbdPartDto]) -> Vec<IbdContainerGroupDto> {
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

    let mut groups: Vec<IbdContainerGroupDto> = groups_by_qn
        .into_values()
        .filter(|group| !group.member_part_ids.is_empty())
        .collect();
    groups.sort_by(|left, right| {
        left.depth
            .cmp(&right.depth)
            .then_with(|| left.qualified_name.cmp(&right.qualified_name))
    });
    groups
}

/// Builds IBD data for the given URI from the semantic graph.
pub fn build_ibd_for_uri(graph: &SemanticGraph, uri: &Url) -> IbdDataDto {
    let nodes = graph.nodes_for_uri(uri);

    let mut parts = Vec::new();
    let mut ports = Vec::new();

    for node in &nodes {
        let qn = node.id.qualified_name.clone();
        let parent_qualified = node.parent_id.as_ref().map(|p| p.qualified_name.clone());

        if is_interconnection_element_kind(&node.element_kind) {
            let container_id = node.parent_id.as_ref().and_then(|pid| {
                graph.get_node(pid).and_then(|p| {
                    if is_interconnection_element_kind(&p.element_kind) {
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
                element_type: node.element_kind.clone(),
                attributes: node.attributes.clone(),
            });
        } else if is_port_like(&node.element_kind) {
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
            });
        }
    }

    // Interconnection view requires instance-centric expansion of typed part trees.
    let mut existing_part_qn_dot: std::collections::HashSet<String> =
        parts.iter().map(|p| p.qualified_name.clone()).collect();
    let mut existing_ports: std::collections::HashSet<(String, String)> = ports
        .iter()
        .map(|p| (p.parent_id.clone(), p.name.clone()))
        .collect();

    let add_ports_from_def =
        |def_node: &SemanticNode,
         parent_dot: &str,
         ports_out: &mut Vec<IbdPortDto>,
         existing_ports: &mut std::collections::HashSet<(String, String)>| {
            add_inherited_ports_from_definition(
                graph,
                def_node,
                parent_dot,
                ports_out,
                existing_ports,
            );
        };

    fn part_def_has_materialized_shape(
        graph: &SemanticGraph,
        def_node: &SemanticNode,
        visiting: &mut std::collections::HashSet<String>,
    ) -> bool {
        let def_key = def_node.id.qualified_name.clone();
        if !visiting.insert(def_key) {
            return false;
        }
        let direct = graph
            .children_of(def_node)
            .iter()
            .any(|child| is_part_like(&child.element_kind) || is_port_like(&child.element_kind));
        let inherited = graph
            .outgoing_typing_or_specializes_targets(def_node)
            .into_iter()
            .any(|generalization| {
                is_part_like(&generalization.element_kind)
                    && part_def_has_materialized_shape(graph, generalization, visiting)
            });
        visiting.remove(&def_node.id.qualified_name);
        direct || inherited
    }

    fn first_typed_part_shape<'a>(
        graph: &'a SemanticGraph,
        node: &'a SemanticNode,
    ) -> Option<&'a SemanticNode> {
        graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .find(|typed_def| {
                is_part_like(&typed_def.element_kind)
                    && part_def_has_materialized_shape(
                        graph,
                        typed_def,
                        &mut std::collections::HashSet::new(),
                    )
            })
    }

    #[allow(clippy::too_many_arguments)]
    fn expand_part_usage_subtree(
        graph: &SemanticGraph,
        usage_node: &SemanticNode,
        parent_dot: &str,
        parts_out: &mut Vec<IbdPartDto>,
        ports_out: &mut Vec<IbdPortDto>,
        existing_part_qn_dot: &mut std::collections::HashSet<String>,
        existing_ports: &mut std::collections::HashSet<(String, String)>,
        visiting_defs: &mut std::collections::HashSet<String>,
    ) {
        for part_child in graph.children_of(usage_node) {
            if !is_part_like(&part_child.element_kind) {
                continue;
            }
            let expanded_dot = format!("{parent_dot}.{}", part_child.name);
            if existing_part_qn_dot.insert(expanded_dot.clone()) {
                parts_out.push(IbdPartDto {
                    id: expanded_dot.clone(),
                    node_id: expanded_dot.clone(),
                    name: part_child.name.clone(),
                    qualified_name: expanded_dot.clone(),
                    uri: Some(part_child.id.uri.as_str().to_string()),
                    container_id: Some(parent_dot.to_string()),
                    element_type: part_child.element_kind.clone(),
                    attributes: part_child.attributes.clone(),
                });
            }
            expand_part_usage_subtree(
                graph,
                part_child,
                &expanded_dot,
                parts_out,
                ports_out,
                existing_part_qn_dot,
                existing_ports,
                visiting_defs,
            );
            if let Some(grand_def) = first_typed_part_shape(graph, part_child) {
                expand_def_subtree(
                    graph,
                    grand_def,
                    &expanded_dot,
                    parts_out,
                    ports_out,
                    existing_part_qn_dot,
                    existing_ports,
                    visiting_defs,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn expand_def_subtree(
        graph: &SemanticGraph,
        def_node: &SemanticNode,
        parent_dot: &str,
        parts_out: &mut Vec<IbdPartDto>,
        ports_out: &mut Vec<IbdPortDto>,
        existing_part_qn_dot: &mut std::collections::HashSet<String>,
        existing_ports: &mut std::collections::HashSet<(String, String)>,
        visiting_defs: &mut std::collections::HashSet<String>,
    ) {
        let def_key = def_node.id.qualified_name.clone();
        if !visiting_defs.insert(def_key.clone()) {
            return;
        }
        add_inherited_ports_from_definition(graph, def_node, parent_dot, ports_out, existing_ports);
        for part_child in graph.children_of(def_node) {
            if !is_part_like(&part_child.element_kind) {
                continue;
            }
            let expanded_dot = format!("{parent_dot}.{}", part_child.name);
            if existing_part_qn_dot.contains(&expanded_dot) {
                continue;
            }
            existing_part_qn_dot.insert(expanded_dot.clone());
            parts_out.push(IbdPartDto {
                id: expanded_dot.clone(),
                node_id: expanded_dot.clone(),
                name: part_child.name.clone(),
                qualified_name: expanded_dot.clone(),
                uri: Some(part_child.id.uri.as_str().to_string()),
                container_id: Some(parent_dot.to_string()),
                element_type: part_child.element_kind.clone(),
                attributes: part_child.attributes.clone(),
            });
            expand_part_usage_subtree(
                graph,
                part_child,
                &expanded_dot,
                parts_out,
                ports_out,
                existing_part_qn_dot,
                existing_ports,
                visiting_defs,
            );
            if let Some(grand_def) = first_typed_part_shape(graph, part_child) {
                expand_def_subtree(
                    graph,
                    grand_def,
                    &expanded_dot,
                    parts_out,
                    ports_out,
                    existing_part_qn_dot,
                    existing_ports,
                    visiting_defs,
                );
            }
        }
        visiting_defs.remove(&def_key);
    }

    let parts_snapshot = parts.clone();
    for p in &parts_snapshot {
        let Some(node) = graph_node_for_ibd_part(graph, uri, p) else {
            continue;
        };
        let Some(def_node) = first_typed_part_shape(graph, node) else {
            continue;
        };
        let parent_dot = p.qualified_name.as_str();
        add_ports_from_def(def_node, parent_dot, &mut ports, &mut existing_ports);
        let mut visiting_defs: std::collections::HashSet<String> = std::collections::HashSet::new();
        expand_def_subtree(
            graph,
            def_node,
            parent_dot,
            &mut parts,
            &mut ports,
            &mut existing_part_qn_dot,
            &mut existing_ports,
            &mut visiting_defs,
        );
    }

    let def_container_prefixes: Vec<String> = graph
        .nodes_for_uri(uri)
        .iter()
        .filter(|node| node.element_kind.to_lowercase().contains("part def"))
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
    connectors = remap_connectors_to_typed_instances(connectors, &instance_def_mappings);

    for connector in &mut connectors {
        connector.source_id =
            expand_relative_endpoint_to_part_path(&connector.source_id, &parts, &ports);
        connector.target_id =
            expand_relative_endpoint_to_part_path(&connector.target_id, &parts, &ports);
        if connector.source == connector.source_id {
            connector.source = connector.source_id.clone();
        }
        if connector.target == connector.target_id {
            connector.target = connector.target_id.clone();
        }
    }

    // Mirror definition-level connectors into usage-instance paths so interconnection
    // rendering stays instance-centric for selected roots.
    let mut connector_keys: std::collections::HashSet<(String, String, String)> = connectors
        .iter()
        .map(|c| (c.source_id.clone(), c.target_id.clone(), c.rel_type.clone()))
        .collect();
    for p in &parts_snapshot {
        let Some(node) = graph_node_for_ibd_part(graph, uri, p) else {
            continue;
        };
        let Some(def_node) = first_typed_part_shape(graph, node) else {
            continue;
        };
        let def_prefix = def_node.id.qualified_name.as_str();
        let usage_prefix_dot = p.qualified_name.as_str();
        if def_node.id.uri != *uri {
            mirror_connectors_from_definition_document(
                graph,
                &def_node.id.uri,
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
        let def_edges = graph.edges_for_uri_as_strings(&def_node.id.uri);
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
    for connector in &mut connectors {
        connector.source_id =
            expand_relative_endpoint_to_part_path(&connector.source_id, &parts, &ports);
        connector.target_id =
            expand_relative_endpoint_to_part_path(&connector.target_id, &parts, &ports);
        if connector.source == connector.source_id {
            connector.source = connector.source_id.clone();
        }
        if connector.target == connector.target_id {
            connector.target = connector.target_id.clone();
        }
    }
    connectors = remap_connectors_to_typed_instances(connectors, &instance_def_mappings);
    let mut connectors = dedupe_connectors(connectors);
    enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);

    ensure_endpoint_parts_present(&mut parts, &connectors, graph, uri);

    let (parts, ports, connectors) =
        prune_interconnection_definition_parts(parts, ports, connectors);
    let (parts, ports, connectors) = prune_ibd_payload_to_connected_scope(parts, ports, connectors);
    let mut connectors = connectors;
    enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);
    let container_groups = build_container_groups(&parts);

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

    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        default_root,
        root_views,
    }
}

/// Merge multiple per-URI IBD payloads into one workspace-scoped payload.
pub fn merge_ibd_payloads(ibds: Vec<IbdDataDto>) -> IbdDataDto {
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
                    let mut members: std::collections::HashSet<String> =
                        existing.member_part_ids.iter().cloned().collect();
                    for part_id in &group.member_part_ids {
                        if members.insert(part_id.clone()) {
                            existing.member_part_ids.push(part_id.clone());
                        }
                    }
                })
                .or_insert(group);
        }
        for group in ibd.package_container_groups {
            package_container_groups_by_id
                .entry(group.id.clone())
                .and_modify(|existing| {
                    let mut members: std::collections::HashSet<String> =
                        existing.member_part_ids.iter().cloned().collect();
                    for part_id in &group.member_part_ids {
                        if members.insert(part_id.clone()) {
                            existing.member_part_ids.push(part_id.clone());
                        }
                    }
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
    enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);
    for view in root_views.values_mut() {
        let (view_parts, view_ports, view_connectors) = prune_interconnection_definition_parts(
            std::mem::take(&mut view.parts),
            std::mem::take(&mut view.ports),
            std::mem::take(&mut view.connectors),
        );
        let mut view_connectors = view_connectors;
        enrich_connector_endpoint_refs(&mut view_connectors, &view_parts, &view_ports);
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

/// After merging per-document IBD payloads, remap definition-path connectors onto typed
/// instance paths using workspace-wide part typings.
fn split_architecture_scope_root(qualified_name: &str) -> Option<(&str, &str)> {
    if let Some(pos) = qualified_name.find(".architecture.") {
        return Some((
            &qualified_name[..pos + ".architecture".len()],
            &qualified_name[pos + ".architecture.".len()..],
        ));
    }
    if let Some(pos) = qualified_name.rfind(".RegionalGridArchitecture") {
        let end = pos + ".RegionalGridArchitecture".len();
        let tail = qualified_name[end..].strip_prefix('.').unwrap_or_default();
        return Some((&qualified_name[..end], tail));
    }
    None
}

fn architecture_package_prefix(qualified_name: &str) -> Option<&str> {
    if let Some(pos) = qualified_name.find(".regionalExpansionProject.architecture") {
        return Some(&qualified_name[..pos]);
    }
    if let Some(pos) = qualified_name.find(".Architecture.RegionalGridArchitecture") {
        return Some(&qualified_name[..pos]);
    }
    if let Some(pos) = qualified_name.rfind(".RegionalGridArchitecture") {
        return qualified_name[..pos]
            .rsplit_once('.')
            .map(|(prefix, _)| prefix);
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
        if root.contains(".regionalExpansionProject.architecture") {
            instance_roots.insert(root.to_string());
        } else if root.contains("RegionalGridArchitecture") {
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

pub fn finalize_merged_ibd_connectors(
    graph: &SemanticGraph,
    workspace_uris: &[Url],
    ibd: &mut IbdDataDto,
) {
    let mut mappings = Vec::new();
    for uri in workspace_uris {
        mappings.extend(build_instance_def_mappings(graph, uri, &ibd.parts));
    }
    mappings.sort_by_key(|mapping| std::cmp::Reverse(mapping.0.len()));
    mappings.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);

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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use url::Url;

    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;

    use super::{
        build_container_groups, infer_port_side, prune_ibd_payload_to_connected_scope,
        prune_interconnection_definition_parts, prune_redundant_top_level_roots, IbdConnectorDto,
        IbdPartDto, IbdPortDto,
    };

    fn test_part(
        id: &str,
        name: &str,
        qualified_name: &str,
        container_id: Option<&str>,
        element_type: &str,
    ) -> IbdPartDto {
        IbdPartDto {
            id: id.to_string(),
            node_id: qualified_name.to_string(),
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            uri: None,
            container_id: container_id.map(String::from),
            element_type: element_type.to_string(),
            attributes: HashMap::new(),
        }
    }

    fn test_port(id: &str, name: &str, parent_id: &str) -> IbdPortDto {
        IbdPortDto {
            id: id.to_string(),
            port_id: format!("{parent_id}.{name}"),
            name: name.to_string(),
            parent_id: parent_id.to_string(),
            direction: None,
            port_type: None,
            port_side: None,
        }
    }

    fn test_connector(source_id: &str, target_id: &str) -> IbdConnectorDto {
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

    #[test]
    fn infer_port_side_prefers_direction() {
        assert_eq!(
            infer_port_side("power_out", Some("in"), Some("PowerPort")),
            Some("left".to_string())
        );
        assert_eq!(
            infer_port_side("sensor_in", Some("out"), Some("SensorPort")),
            Some("right".to_string())
        );
    }

    #[test]
    fn infer_port_side_uses_generic_name_hints() {
        assert_eq!(
            infer_port_side("camera_input", None, None),
            Some("left".to_string())
        );
        assert_eq!(
            infer_port_side("telemetryOutput", None, None),
            Some("right".to_string())
        );
        assert_eq!(
            infer_port_side("fuel_in", None, None),
            Some("left".to_string())
        );
        assert_eq!(
            infer_port_side("payload_out", None, None),
            Some("right".to_string())
        );
    }

    #[test]
    fn infer_port_side_does_not_use_model_specific_type_names() {
        assert_eq!(infer_port_side("status", None, Some("PowerPort")), None);
        assert_eq!(
            infer_port_side("status", None, Some("~TelemetryPort")),
            None
        );
    }

    #[test]
    fn prune_ibd_keeps_unconnected_parts_under_same_composite() {
        let parts = vec![
            test_part("O::Desk", "desk", "O.Desk", None, "part"),
            test_part(
                "O::Desk::connected",
                "connected",
                "O.Desk.connected",
                Some("O.Desk"),
                "part",
            ),
            test_part(
                "O::Desk::orphan",
                "orphan",
                "O.Desk.orphan",
                Some("O.Desk"),
                "part",
            ),
            test_part(
                "O::Desk::orphan::nested",
                "nested",
                "O.Desk.orphan.nested",
                Some("O.Desk.orphan"),
                "part",
            ),
        ];
        let ports = vec![
            test_port("O.Desk.connected.p1", "p1", "O.Desk.connected"),
            test_port("O.Desk.connected.p2", "p2", "O.Desk.connected"),
        ];
        let connectors = vec![test_connector("O.Desk.connected.p1", "O.Desk.connected.p2")];

        let (parts, _ports, _connectors) =
            prune_ibd_payload_to_connected_scope(parts, ports, connectors);

        let qns: Vec<&str> = parts.iter().map(|p| p.qualified_name.as_str()).collect();
        assert!(qns.contains(&"O.Desk"));
        assert!(qns.contains(&"O.Desk.connected"));
        assert!(
            qns.contains(&"O.Desk.orphan"),
            "sibling part with no connectors should remain in IBD payload"
        );
        assert!(
            qns.contains(&"O.Desk.orphan.nested"),
            "nested parts under an unconnected sibling should remain"
        );
    }

    #[test]
    fn container_groups_are_derived_from_part_qualified_names() {
        let parts = vec![
            test_part("P::Inner::a", "a", "P.Inner.a", None, "part"),
            test_part("P::Inner::b", "b", "P.Inner.b", None, "part"),
        ];
        let groups = build_container_groups(&parts);
        assert!(groups
            .iter()
            .any(|group| group.qualified_name == "P" && group.member_part_ids.len() == 2));
        assert!(groups
            .iter()
            .any(|group| group.qualified_name == "P.Inner" && group.member_part_ids.len() == 2));
    }

    #[test]
    fn connector_endpoint_refs_use_nested_port_owner() {
        let parts = vec![
            test_part(
                "Grid::northSouthRing",
                "northSouthRing",
                "Grid.northSouthRing",
                None,
                "part",
            ),
            test_part(
                "Grid::northSouthRing::ringSegmentBtoC",
                "ringSegmentBtoC",
                "Grid.northSouthRing.ringSegmentBtoC",
                Some("Grid.northSouthRing"),
                "part",
            ),
            test_part(
                "Grid::txStationB",
                "txStationB",
                "Grid.txStationB",
                None,
                "part",
            ),
        ];
        let ports = vec![
            test_port(
                "Grid.northSouthRing.ringSegmentBtoC.a",
                "a",
                "Grid.northSouthRing.ringSegmentBtoC",
            ),
            test_port(
                "Grid.txStationB.mvConnection",
                "mvConnection",
                "Grid.txStationB",
            ),
        ];
        let mut connectors = vec![IbdConnectorDto {
            source: "Grid.txStationB.mvConnection".to_string(),
            target: "Grid.northSouthRing.ringSegmentBtoC.a".to_string(),
            source_id: "Grid.txStationB.mvConnection".to_string(),
            target_id: "Grid.northSouthRing.ringSegmentBtoC.a".to_string(),
            source_part_id: Some("Grid.txStationB".to_string()),
            target_part_id: Some("Grid.northSouthRing".to_string()),
            source_port_id: None,
            target_port_id: None,
            rel_type: "connection".to_string(),
        }];

        super::enrich_connector_endpoint_refs(&mut connectors, &parts, &ports);

        assert_eq!(
            connectors[0].target_part_id.as_deref(),
            Some("Grid.northSouthRing.ringSegmentBtoC")
        );
        assert_eq!(
            connectors[0].target_port_id.as_deref(),
            Some("Grid.northSouthRing.ringSegmentBtoC.a")
        );
        assert_eq!(
            connectors[0].source_port_id.as_deref(),
            Some("Grid.txStationB.mvConnection")
        );
    }

    #[test]
    fn redundant_top_level_roots_are_pruned_when_already_represented() {
        let parts = vec![
            IbdPartDto {
                id: "Pkg::Vehicle".to_string(),
                node_id: "Pkg.Vehicle".to_string(),
                name: "Vehicle".to_string(),
                qualified_name: "Pkg.Vehicle".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            },
            IbdPartDto {
                id: "Pkg::Vehicle::controller".to_string(),
                node_id: "Pkg.Vehicle.controller".to_string(),
                name: "controller".to_string(),
                qualified_name: "Pkg.Vehicle.controller".to_string(),
                uri: None,
                container_id: Some("Pkg.Vehicle".to_string()),
                element_type: "part".to_string(),
                attributes: HashMap::from([(
                    "partType".to_string(),
                    serde_json::Value::String("Controller".to_string()),
                )]),
            },
            IbdPartDto {
                id: "Pkg::Controller".to_string(),
                node_id: "Pkg.Controller".to_string(),
                name: "Controller".to_string(),
                qualified_name: "Pkg.Controller".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            },
            IbdPartDto {
                id: "Pkg::Controller::sensor".to_string(),
                node_id: "Pkg.Controller.sensor".to_string(),
                name: "sensor".to_string(),
                qualified_name: "Pkg.Controller.sensor".to_string(),
                uri: None,
                container_id: Some("Pkg.Controller".to_string()),
                element_type: "part".to_string(),
                attributes: HashMap::new(),
            },
            IbdPartDto {
                id: "Pkg::Vehicle::controller::sensor".to_string(),
                node_id: "Pkg.Vehicle.controller.sensor".to_string(),
                name: "sensor".to_string(),
                qualified_name: "Pkg.Vehicle.controller.sensor".to_string(),
                uri: None,
                container_id: Some("Pkg.Vehicle.controller".to_string()),
                element_type: "part".to_string(),
                attributes: HashMap::new(),
            },
            IbdPartDto {
                id: "Pkg::VehicleInst".to_string(),
                node_id: "Pkg.vehicleInst".to_string(),
                name: "vehicleInst".to_string(),
                qualified_name: "Pkg.vehicleInst".to_string(),
                uri: None,
                container_id: None,
                element_type: "part".to_string(),
                attributes: HashMap::from([(
                    "partType".to_string(),
                    serde_json::Value::String("Vehicle".to_string()),
                )]),
            },
        ];
        let ports = vec![
            IbdPortDto {
                id: "Pkg.Vehicle.controller.out".to_string(),
                port_id: "Pkg.Vehicle.controller.out".to_string(),
                name: "out".to_string(),
                parent_id: "Pkg.Vehicle.controller".to_string(),
                direction: None,
                port_type: None,
                port_side: None,
            },
            IbdPortDto {
                id: "Pkg.Vehicle.controller.sensor.in".to_string(),
                port_id: "Pkg.Vehicle.controller.sensor.in".to_string(),
                name: "in".to_string(),
                parent_id: "Pkg.Vehicle.controller.sensor".to_string(),
                direction: None,
                port_type: None,
                port_side: None,
            },
            IbdPortDto {
                id: "Pkg.Controller.sensor.in".to_string(),
                port_id: "Pkg.Controller.sensor.in".to_string(),
                name: "in".to_string(),
                parent_id: "Pkg.Controller.sensor".to_string(),
                direction: None,
                port_type: None,
                port_side: None,
            },
            IbdPortDto {
                id: "Pkg.vehicleInst.out".to_string(),
                port_id: "Pkg.vehicleInst.out".to_string(),
                name: "out".to_string(),
                parent_id: "Pkg.vehicleInst".to_string(),
                direction: None,
                port_type: None,
                port_side: None,
            },
        ];
        let connectors = vec![
            IbdConnectorDto {
                source: "Pkg.Vehicle.controller.out".to_string(),
                target: "Pkg.Vehicle.controller.sensor.in".to_string(),
                source_id: "Pkg.Vehicle.controller.out".to_string(),
                target_id: "Pkg.Vehicle.controller.sensor.in".to_string(),
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            },
            IbdConnectorDto {
                source: "Pkg.Controller.sensor.in".to_string(),
                target: "Pkg.Controller.sensor.out".to_string(),
                source_id: "Pkg.Controller.sensor.in".to_string(),
                target_id: "Pkg.Controller.sensor.out".to_string(),
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            },
            IbdConnectorDto {
                source: "Pkg.vehicleInst.out".to_string(),
                target: "Pkg.vehicleInst.in".to_string(),
                source_id: "Pkg.vehicleInst.out".to_string(),
                target_id: "Pkg.vehicleInst.in".to_string(),
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            },
        ];

        let (parts, ports, connectors) = prune_redundant_top_level_roots(
            parts,
            ports,
            connectors,
            &Url::parse("file:///test.sysml").expect("url"),
        );

        assert!(parts
            .iter()
            .any(|part| part.qualified_name == "Pkg.Vehicle"));
        assert!(!parts
            .iter()
            .any(|part| part.qualified_name == "Pkg.Controller"));
        assert!(!parts
            .iter()
            .any(|part| part.qualified_name == "Pkg.vehicleInst"));
        assert!(ports
            .iter()
            .all(|port| !port.parent_id.starts_with("Pkg.Controller")));
        assert!(connectors
            .iter()
            .all(|connector| !connector.source_id.starts_with("Pkg.Controller")));
    }

    #[test]
    fn build_ibd_mirrors_definition_connections_onto_cross_file_instance() {
        let architecture = r#"
            package WebShopArchitecture {
                part def Storefront {
                    port checkoutApiOut;
                }
                part def ApiGateway {
                    port publicCheckoutIn;
                }
                part def WebShopSystem {
                    part storefront : Storefront;
                    part apiGateway : ApiGateway;
                    connect storefront.checkoutApiOut to apiGateway.publicCheckoutIn;
                }
            }
        "#;
        let instance = r#"
            package WebShopExample {
                import WebShopArchitecture::*;
                part webshopSystem : WebShopSystem;
            }
        "#;

        let arch_doc = SysmlDocument::from_memory_path(
            "workspace",
            "WebShopArchitecture.sysml",
            architecture.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("arch doc");
        let instance_doc = SysmlDocument::from_memory_path(
            "workspace",
            "webshop.sysml",
            instance.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("instance doc");
        let architecture_uri = arch_doc.uri.clone();
        let instance_uri = instance_doc.uri.clone();

        let (graph, _parsed) =
            build_semantic_graph_from_documents(&[arch_doc, instance_doc]).expect("graph");

        let merged = super::merge_ibd_payloads(vec![
            super::build_ibd_for_uri(&graph, &architecture_uri),
            super::build_ibd_for_uri(&graph, &instance_uri),
        ]);

        let root_view = merged
            .root_views
            .get("webshopSystem")
            .expect("webshopSystem root view");
        assert!(
            root_view.connectors.iter().any(|connector| {
                connector.source_id.contains("webshopSystem.storefront")
                    && connector.source_id.contains("checkoutApiOut")
                    && connector.target_id.contains("webshopSystem.apiGateway")
                    && connector.target_id.contains("publicCheckoutIn")
            }),
            "expected mirrored storefront→gateway connector, got {:?}",
            root_view.connectors
        );
    }

    #[test]
    fn finalize_merged_ibd_remapped_definition_connects_to_typed_instance() {
        let architecture = SysmlDocument::from_memory_path(
            "powersystems",
            "Architecture.sysml",
            r#"package RegionalGridExpansion::Architecture {
    part def RegionalGridArchitecture {
        part feederNorth { port outgoing; }
        part cable01 { port a; port b; }
        connect feederNorth.outgoing to cable01.a;
    }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture uri");
        let project = SysmlDocument::from_memory_path(
            "powersystems",
            "Project.sysml",
            r#"package RegionalGridExpansion {
    public import RegionalGridExpansion::Architecture::*;
    part regionalExpansionProject {
        part architecture : RegionalGridArchitecture;
    }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("project uri");
        let uris = [architecture.uri.clone(), project.uri.clone()];
        let (graph, _) =
            build_semantic_graph_from_documents(&[architecture, project]).expect("graph");
        let mut merged = super::merge_ibd_payloads(
            uris.iter()
                .map(|uri| super::build_ibd_for_uri(&graph, uri))
                .collect(),
        );
        super::finalize_merged_ibd_connectors(&graph, &uris, &mut merged);
        assert!(
            merged.connectors.iter().any(|connector| {
                connector
                    .source_id
                    .contains("regionalExpansionProject.architecture.feederNorth")
                    && connector
                        .target_id
                        .contains("regionalExpansionProject.architecture.cable01")
            }),
            "expected definition-level connect mirrored to project architecture instance, got {:?}",
            merged.connectors
        );
    }

    #[test]
    fn build_ibd_materializes_inline_children_of_typed_part_usage_on_instance() {
        let architecture = SysmlDocument::from_memory_path(
            "workspace",
            "Architecture.sysml",
            r#"package GridArchitecture {
    part def Segment {
        port a;
        port b;
    }
    part def TiePoint {
        port incoming;
        port outgoing;
    }
    part def Ring;
    part def System {
        part ring : Ring {
            part segment : Segment;
            part tie : TiePoint;
        }
        connect ring.segment.b to ring.tie.incoming;
    }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture uri");
        let project = SysmlDocument::from_memory_path(
            "workspace",
            "Project.sysml",
            r#"package Project {
    import GridArchitecture::*;
    part system : System;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("project uri");
        let uris = [architecture.uri.clone(), project.uri.clone()];
        let (graph, _) =
            build_semantic_graph_from_documents(&[architecture, project]).expect("graph");
        let mut merged = super::merge_ibd_payloads(
            uris.iter()
                .map(|uri| super::build_ibd_for_uri(&graph, uri))
                .collect(),
        );
        super::finalize_merged_ibd_connectors(&graph, &uris, &mut merged);

        assert!(
            merged
                .parts
                .iter()
                .any(|part| part.qualified_name == "Project.system.ring.segment"),
            "expected inline nested segment materialized under instance path, got {:?}",
            merged.parts
        );
        assert!(
            merged
                .parts
                .iter()
                .any(|part| part.qualified_name == "Project.system.ring.tie"),
            "expected inline nested tie materialized under instance path, got {:?}",
            merged.parts
        );
        assert!(
            merged.ports.iter().any(|port| {
                port.parent_id == "Project.system.ring.segment" && port.name == "b"
            }),
            "expected segment ports under instance path, got {:?}",
            merged.ports
        );
        assert!(
            merged.connectors.iter().any(|connector| {
                connector.source_id == "Project.system.ring.segment.b"
                    && connector.target_id == "Project.system.ring.tie.incoming"
            }),
            "expected connector to instance inline nested ports, got {:?}",
            merged.connectors
        );
    }

    #[test]
    fn build_ibd_includes_ports_inherited_from_generalized_part_def() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package Grid {
    part def MediumVoltageFeeder {
        port source;
        port outgoing;
    }
    part def DutchMVFeeder :> MediumVoltageFeeder;
    part def System {
        part feederNorth : DutchMVFeeder;
    }
    part system : System;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri");
        let uri = doc.uri.clone();
        let (graph, _) =
            build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");
        let ibd = super::build_ibd_for_uri(&graph, &uri);
        let feeder_ports: Vec<_> = ibd
            .ports
            .iter()
            .filter(|port| port.parent_id.contains("feederNorth"))
            .map(|port| port.name.as_str())
            .collect();
        assert!(
            feeder_ports.contains(&"source") && feeder_ports.contains(&"outgoing"),
            "expected inherited feeder ports on instance path, got {:?}",
            feeder_ports
        );
    }

    #[test]
    fn build_ibd_materializes_pending_connection_endpoints_for_untyped_connects() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            r#"package Architecture {
  part def PowerSubsystem {
    port powerOut;
  }
  part def ControlSoftware {
    port powerIn;
  }
  part AutonomousFloorCleaningRobot {
    part power : PowerSubsystem;
    part control : ControlSoftware;
    connect power.powerOut to control.powerIn;
  }
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/model.sysml").expect("uri");
        let ibd = super::build_ibd_for_uri(&graph, &uri);
        assert!(
            ibd.connectors.iter().any(|connector| {
                connector.source_id == "Architecture.AutonomousFloorCleaningRobot.power.powerOut"
                    && connector.target_id
                        == "Architecture.AutonomousFloorCleaningRobot.control.powerIn"
            }),
            "expected pending connect endpoints to materialize as IBD connector: {:?}",
            ibd.connectors
        );
    }

    #[test]
    fn build_ibd_surveillance_drone_instance_has_nested_parts_and_connectors() {
        let fixture = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../kernel/tests/fixtures/surveillance_drone_full.sysml"),
        )
        .expect("read surveillance drone fixture");
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "surveillance_drone_full.sysml",
            fixture,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/surveillance_drone_full.sysml").expect("uri");
        let ibd = super::build_ibd_for_uri(&graph, &uri);

        assert_eq!(
            ibd.default_root.as_deref(),
            Some("droneInstance"),
            "expected drone instance as default root, got {:?}",
            ibd.default_root
        );
        assert!(
            ibd.connectors.len() >= 14,
            "expected drone connector set, got {:?}",
            ibd.connectors.len()
        );
        assert!(
            ibd.parts
                .iter()
                .any(|part| { part.qualified_name.ends_with("propulsion.propulsionUnit4") }),
            "expected expanded propulsion unit in IBD, got {:?}",
            ibd.parts
                .iter()
                .map(|p| &p.qualified_name)
                .collect::<Vec<_>>()
        );
        assert!(
            ibd.connectors.iter().any(|connector| {
                connector.source_id.ends_with("flightController.motorCmd")
                    && connector.target_id.ends_with("propulsionUnit1.cmd")
            }),
            "expected motor command connector under drone instance, got {:?}",
            ibd.connectors
        );
        for unit in ["propulsionUnit2", "propulsionUnit3", "propulsionUnit4"] {
            assert!(
                ibd.connectors.iter().any(|connector| {
                    connector.source_id.ends_with("flightController.motorCmd")
                        && connector.target_id.ends_with(&format!("{unit}.cmd"))
                }),
                "expected motor command connector to {unit}, got {:?}",
                ibd.connectors
            );
            assert!(
                ibd.connectors
                    .iter()
                    .any(|connector| { connector.target_id.ends_with(&format!("{unit}.pwr")) }),
                "expected power connector to {unit}, got {:?}",
                ibd.connectors
            );
        }

        let default_root = ibd.default_root.as_deref().expect("default root");
        let root_view = ibd.root_views.get(default_root).expect("default root view");
        assert!(
            root_view.connectors.len() >= 14,
            "expected default root view to include connector set, got {} in {:?}: {:?}",
            root_view.connectors.len(),
            default_root,
            root_view.connectors
        );
        for part in &ibd.parts {
            assert!(
                !part.element_type.to_lowercase().contains(" def"),
                "IBD parts must not include definitions: {:?}",
                part
            );
        }
        for part in &root_view.parts {
            assert!(
                !part.element_type.to_lowercase().contains(" def"),
                "scoped IBD parts must not include definitions: {:?}",
                part
            );
        }
    }

    #[test]
    fn ibd_payload_excludes_definitions_from_connected_scope() {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "parts_tree.sysml",
            r#"package PartsTree {
  part def Tree {
    part branch;
  }
  part def Vehicle {
    part tree : Tree;
  }
  part vehicle : Vehicle;
}"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        let uri = Url::parse("memory://workspace/parts_tree.sysml").expect("uri");
        let ibd = super::build_ibd_for_uri(&graph, &uri);

        assert!(
            ibd.parts
                .iter()
                .all(|part| !part.element_type.to_lowercase().contains(" def")),
            "expected no part definitions in IBD payload, got {:?}",
            ibd.parts
                .iter()
                .map(|part| (&part.name, &part.element_type))
                .collect::<Vec<_>>()
        );
        for view in ibd.root_views.values() {
            assert!(
                view.parts
                    .iter()
                    .all(|part| !part.element_type.to_lowercase().contains(" def")),
                "scoped IBD must not include definitions: {:?}",
                view.parts
            );
        }
    }

    #[test]
    fn prune_interconnection_definition_parts_normalizes_reference_metadata() {
        let parts = vec![IbdPartDto {
            id: "PartsTree::sharedBranch".to_string(),
            node_id: "PartsTree.sharedBranch".to_string(),
            name: "sharedBranch".to_string(),
            qualified_name: "PartsTree.sharedBranch".to_string(),
            uri: None,
            container_id: Some("PartsTree.tree".to_string()),
            element_type: "ref".to_string(),
            attributes: HashMap::new(),
        }];
        let (parts, ports, connectors) =
            prune_interconnection_definition_parts(parts, Vec::new(), Vec::new());
        assert!(ports.is_empty());
        assert!(connectors.is_empty());
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].element_type, "ref");
        assert_eq!(
            parts[0].attributes.get("isReference"),
            Some(&serde_json::json!(true))
        );
        assert_eq!(
            parts[0].attributes.get("isDefinition"),
            Some(&serde_json::json!(false))
        );
    }
}
