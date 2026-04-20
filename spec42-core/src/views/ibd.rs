//! Builds IBD (Internal Block Diagram) / Interconnection View data from the semantic graph.
//! Used by sysml/model to return a ready-to-render structure for the client.

use serde::Serialize;
use tower_lsp::lsp_types::Url;

use crate::semantic_model::{NodeId, RelationshipKind, SemanticGraph, SemanticNode};

fn is_part_like(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k.contains("part def") || k == "part" || (k.contains("part") && !k.contains("def"))
}

fn is_part_instance_kind(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k == "part" || k.contains("part usage")
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
    #[serde(rename = "type")]
    pub rel_type: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdDataDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
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
}

/// Qualified name with "::" converted to "." for client path matching (e.g. "pkg::A::b" -> "A.b" when root is "A").
pub fn qualified_name_to_dot(qn: &str) -> String {
    qn.replace("::", ".")
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
    let mut existing_part_qn: std::collections::HashSet<String> =
        parts.iter().map(|part| part.qualified_name.clone()).collect();

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
        let container_id = node.parent_id.as_ref().map(|parent| qualified_name_to_dot(&parent.qualified_name));
        parts.push(IbdPartDto {
            id: node.id.qualified_name.clone(),
            name: node.name.clone(),
            qualified_name,
            uri: Some(node.id.uri.as_str().to_string()),
            container_id,
            element_type: node.element_kind.clone(),
            attributes: node.attributes.clone(),
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

fn endpoint_under_definition_prefix(endpoint: &str, def_prefix: &str) -> bool {
    endpoint == def_prefix || endpoint.starts_with(&format!("{def_prefix}::"))
}

fn map_definition_endpoint_to_usage(
    endpoint: &str,
    def_prefix: &str,
    usage_prefix_dot: &str,
) -> Option<String> {
    if endpoint == def_prefix {
        return Some(usage_prefix_dot.to_string());
    }
    let prefixed = format!("{def_prefix}::");
    if let Some(remainder) = endpoint.strip_prefix(&prefixed) {
        if remainder.is_empty() {
            return Some(usage_prefix_dot.to_string());
        }
        return Some(format!(
            "{usage_prefix_dot}.{}",
            remainder.replace("::", ".")
        ));
    }
    None
}

/// Builds IBD data for the given URI from the semantic graph.
pub fn build_ibd_for_uri(graph: &SemanticGraph, uri: &Url) -> IbdDataDto {
    let nodes = graph.nodes_for_uri(uri);

    let mut parts = Vec::new();
    let mut ports = Vec::new();

    for node in &nodes {
        let qn = node.id.qualified_name.clone();
        let parent_qualified = node.parent_id.as_ref().map(|p| p.qualified_name.clone());

        if is_part_like(&node.element_kind) {
            let container_id = node.parent_id.as_ref().and_then(|pid| {
                graph.get_node(pid).and_then(|p| {
                    if is_part_like(&p.element_kind) {
                        Some(qualified_name_to_dot(&pid.qualified_name))
                    } else {
                        None
                    }
                })
            });
            parts.push(IbdPartDto {
                id: qn.clone(),
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
                let port_side =
                    infer_port_side(&child.name, direction.as_deref(), port_type.as_deref());
                ports_out.push(IbdPortDto {
                    id: format!("{parent_dot}.{}", child.name),
                    name: child.name.clone(),
                    parent_id: parent_dot.to_string(),
                    direction,
                    port_type,
                    port_side,
                });
            }
        };

    fn first_typed_part_shape<'a>(
        graph: &'a SemanticGraph,
        node: &'a SemanticNode,
    ) -> Option<&'a SemanticNode> {
        graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .find(|t| {
                if !is_part_like(&t.element_kind) {
                    return false;
                }
                let children = graph.children_of(t);
                children
                    .iter()
                    .any(|c| is_part_like(&c.element_kind) || is_port_like(&c.element_kind))
            })
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
        for port_child in graph.children_of(def_node) {
            if !is_port_like(&port_child.element_kind) {
                continue;
            }
            let key = (parent_dot.to_string(), port_child.name.clone());
            if existing_ports.contains(&key) {
                continue;
            }
            existing_ports.insert(key);
            let direction = port_child
                .attributes
                .get("direction")
                .and_then(|v| v.as_str())
                .map(String::from);
            let port_type = port_child
                .attributes
                .get("portType")
                .and_then(|v| v.as_str())
                .map(String::from);
            let port_side =
                infer_port_side(&port_child.name, direction.as_deref(), port_type.as_deref());
            ports_out.push(IbdPortDto {
                id: format!("{parent_dot}.{}", port_child.name),
                name: port_child.name.clone(),
                parent_id: parent_dot.to_string(),
                direction,
                port_type,
                port_side,
            });
        }
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
                name: part_child.name.clone(),
                qualified_name: expanded_dot.clone(),
                uri: Some(part_child.id.uri.as_str().to_string()),
                container_id: Some(parent_dot.to_string()),
                element_type: part_child.element_kind.clone(),
                attributes: part_child.attributes.clone(),
            });
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
        if !p.id.contains("::") {
            continue;
        }
        let node_id = NodeId::new(uri, &p.id);
        let Some(node) = graph.get_node(&node_id) else {
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

    let edges = graph.edges_for_uri_as_strings(uri);
    let mut connectors = Vec::new();
    for (src, tgt, kind, _name) in &edges {
        if *kind == RelationshipKind::Connection {
            // Use full qualified path in dot form for frontend findPartPos resolution
            let source_id = src.replace("::", ".");
            let target_id = tgt.replace("::", ".");
            connectors.push(IbdConnectorDto {
                source: src.clone(),
                target: tgt.clone(),
                source_id,
                target_id,
                rel_type: "connection".to_string(),
            });
        }
    }

    // Mirror definition-level connectors into usage-instance paths so interconnection
    // rendering stays instance-centric for selected roots.
    let mut connector_keys: std::collections::HashSet<(String, String, String)> = connectors
        .iter()
        .map(|c| (c.source_id.clone(), c.target_id.clone(), c.rel_type.clone()))
        .collect();
    for p in &parts_snapshot {
        if !p.id.contains("::") {
            continue;
        }
        let node_id = NodeId::new(uri, &p.id);
        let Some(node) = graph.get_node(&node_id) else {
            continue;
        };
        let Some(def_node) = first_typed_part_shape(graph, node) else {
            continue;
        };
        let def_prefix = def_node.id.qualified_name.as_str();
        let usage_prefix_dot = p.qualified_name.as_str();
        for (src, tgt, kind, _name) in &edges {
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
                rel_type: "connection".to_string(),
            });
        }
    }

    ensure_endpoint_parts_present(&mut parts, &connectors, graph, uri);

    let top_level_parts: Vec<_> = parts
        .iter()
        .filter(|p| {
            p.container_id.is_none()
                || p.container_id
                    .as_ref()
                    .and_then(|container_id| {
                        graph.get_node(&NodeId::new(uri, container_id.replace('.', "::")))
                    })
                    .map(|n| !is_part_like(&n.element_kind))
                    .unwrap_or(true)
        })
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
        let a_score = a.3 * 100 + a.2 * 10 + a.1;
        let b_score = b.3 * 100 + b.2 * 10 + b.1;
        b_score.cmp(&a_score).then_with(|| a.0.name.cmp(&b.0.name))
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
        root_views.insert(
            p.name.clone(),
            IbdRootViewDto {
                parts: focused_parts,
                ports: focused_ports,
                connectors: focused_connectors,
            },
        );
    }

    IbdDataDto {
        parts,
        ports,
        connectors,
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
        for root in ibd.root_candidates {
            root_candidates.insert(root);
        }
        for (name, view) in ibd.root_views {
            let merged = root_views.entry(name).or_insert_with(|| IbdRootViewDto {
                parts: Vec::new(),
                ports: Vec::new(),
                connectors: Vec::new(),
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
        }
    }

    IbdDataDto {
        parts: parts_by_id.into_values().collect(),
        ports: ports_by_key.into_values().collect(),
        connectors: connectors_by_key.into_values().collect(),
        root_candidates: root_candidates.into_iter().collect(),
        default_root: None,
        root_views,
    }
}

#[cfg(test)]
mod tests {
    use super::infer_port_side;

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
}
