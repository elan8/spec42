//! Builds IBD (Internal Block Diagram) / Interconnection View data from the semantic graph.
//! Used by sysml/model to return a ready-to-render structure for the client.

use serde::Serialize;
use tower_lsp::lsp_types::Url;

use crate::semantic_model::{NodeId, RelationshipKind, SemanticGraph, SemanticNode};

fn is_part_like(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k.contains("part def") || k == "part" || (k.contains("part") && !k.contains("def"))
}

/// True if the element kind represents a port (port def or port usage). Public for semantic_checks.
pub fn is_port_like(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k.contains("port def") || k == "port"
}

/// Count of part nodes in the subtree (direct + recursive). Uses typing to follow part def structure.
fn part_tree_size(graph: &SemanticGraph, node: &SemanticNode, _uri: &Url) -> usize {
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
                    return 1 + part_tree_size(graph, def_node, _uri);
                }
            }
            1 + part_tree_size(graph, c, _uri)
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

fn infer_port_side(name: &str, direction: Option<&str>, port_type: Option<&str>) -> Option<String> {
    let normalized_name = name.trim().to_lowercase();
    let normalized_direction = direction.unwrap_or("").trim().to_lowercase();
    let normalized_type = port_type.unwrap_or("").trim().to_lowercase();

    match normalized_direction.as_str() {
        "in" => return Some("left".to_string()),
        "out" => return Some("right".to_string()),
        _ => {}
    }

    if normalized_name.ends_with("in") || normalized_name.contains("input") {
        return Some("left".to_string());
    }
    if normalized_name.ends_with("out") || normalized_name.contains("output") {
        return Some("right".to_string());
    }

    let conjugated = normalized_type.starts_with('~');
    let stripped_type = normalized_type.trim_start_matches('~');
    if conjugated {
        if stripped_type.contains("powerport")
            || stripped_type.contains("telemetryport")
            || stripped_type.contains("sensordataport")
            || stripped_type.contains("cameracontrolport")
            || stripped_type.contains("gimbalcommandport")
            || stripped_type.contains("rccommandport")
        {
            return Some("left".to_string());
        }
    } else if stripped_type.contains("powerport")
        || stripped_type.contains("telemetryport")
        || stripped_type.contains("sensordataport")
        || stripped_type.contains("videostreamport")
    {
        return Some("right".to_string());
    }

    if normalized_name.contains("sensor")
        || normalized_name.contains("telemetry")
        || normalized_name.contains("video")
        || normalized_name.contains("command")
        || normalized_name.contains("power")
        || normalized_name.contains("payload")
        || normalized_name.contains("c2")
        || normalized_name.contains("rc")
    {
        return if normalized_name.contains("in") {
            Some("left".to_string())
        } else if normalized_name.contains("out")
            || normalized_name.contains("regulated")
            || normalized_name.contains("supply")
        {
            Some("right".to_string())
        } else {
            None
        };
    }

    None
}

fn endpoint_matches_root(endpoint: &str, root_prefix: &str, root_name: &str) -> bool {
    endpoint == root_prefix
        || endpoint.starts_with(&format!("{root_prefix}."))
        || endpoint == root_name
        || endpoint.starts_with(&format!("{root_name}."))
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
    let mut part_qualified_by_id: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

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
            part_qualified_by_id.insert(qn.clone(), qn.clone());
            parts.push(IbdPartDto {
                id: qn.clone(),
                name: node.name.clone(),
                qualified_name: qualified_name_to_dot(&qn),
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

    // Expand typed part defs into the part-usage hierarchy so the client can render the full nested tree.
    // Example: if `root.propulsion` is typed by `Propulsion`, then `Propulsion`'s internal parts become
    // `root.propulsion.<child>` (with containerId = `root.propulsion`).
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
                // Prefer targets that actually contribute structure (ports/parts).
                let children = graph.children_of(t);
                children
                    .iter()
                    .any(|c| is_part_like(&c.element_kind) || is_port_like(&c.element_kind))
            })
    }

    fn expand_def_subtree(
        graph: &SemanticGraph,
        def_node: &SemanticNode,
        parent_dot: &str,
        parts_out: &mut Vec<IbdPartDto>,
        ports_out: &mut Vec<IbdPortDto>,
        existing_part_qn_dot: &mut std::collections::HashSet<String>,
        existing_ports: &mut std::collections::HashSet<(String, String)>,
    ) {
        // First, inherit ports from the definition onto the parent usage node.
        // (The closure below is duplicated as a small helper for borrow reasons.)
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
                container_id: Some(parent_dot.to_string()),
                element_type: part_child.element_kind.clone(),
                attributes: part_child.attributes.clone(),
            });

            // Recursively expand if this part usage is typed by a part definition.
            if let Some(grand_def) = first_typed_part_shape(graph, part_child) {
                expand_def_subtree(
                    graph,
                    grand_def,
                    &expanded_dot,
                    parts_out,
                    ports_out,
                    existing_part_qn_dot,
                    existing_ports,
                );
            }
        }
    }

    // Iterate over a snapshot of current parts to avoid infinite growth during iteration.
    let parts_snapshot = parts.clone();
    for p in &parts_snapshot {
        // Only expand for parts that correspond to real semantic nodes (base nodes), not synthetic expanded ones.
        // Base parts use ids with "::" qualified names.
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

        // Expand the definition subtree under this usage part's dot-qualified name.
        let parent_dot = p.qualified_name.as_str();
        // inherit ports (in case the def has ports) onto the usage itself
        add_ports_from_def(def_node, parent_dot, &mut ports, &mut existing_ports);
        expand_def_subtree(
            graph,
            def_node,
            parent_dot,
            &mut parts,
            &mut ports,
            &mut existing_part_qn_dot,
            &mut existing_ports,
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

    // Mirror internal definition-level connections onto each typed part usage instance.
    // This keeps Interconnection View faithful for nested usages like
    // `...flightControl.<internal parts/ports>`.
    let mut connector_keys: std::collections::HashSet<(String, String, String)> = connectors
        .iter()
        .map(|c| (c.source_id.clone(), c.target_id.clone(), c.rel_type.clone()))
        .collect();
    for p in &parts_snapshot {
        // Base parts use semantic ids with "::"; synthetic expanded parts are dot-only ids.
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
            let Some(source_id) = map_definition_endpoint_to_usage(src, def_prefix, usage_prefix_dot)
            else {
                continue;
            };
            let Some(target_id) = map_definition_endpoint_to_usage(tgt, def_prefix, usage_prefix_dot)
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
        .filter_map(|p| {
            graph.get_node(&NodeId::new(uri, &p.id)).and_then(|node| {
                if graph
                    .children_of(node)
                    .iter()
                    .any(|c| is_part_like(&c.element_kind))
                {
                    let root_prefix = p.qualified_name.as_str();
                    let port_count = ports
                        .iter()
                        .filter(|port| endpoint_matches_root(&port.parent_id, root_prefix, &p.name))
                        .count();
                    let connector_count = connectors
                        .iter()
                        .filter(|connector| {
                            endpoint_matches_root(&connector.source_id, root_prefix, &p.name)
                                && endpoint_matches_root(&connector.target_id, root_prefix, &p.name)
                        })
                        .count();
                    Some((
                        *p,
                        part_tree_size(graph, node, uri),
                        port_count,
                        connector_count,
                    ))
                } else {
                    None
                }
            })
        })
        .collect();

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
        let focused_parts: Vec<IbdPartDto> = parts
            .iter()
            .filter(|part| endpoint_matches_root(&part.qualified_name, root_prefix, &p.name))
            .cloned()
            .collect();
        let focused_ports: Vec<IbdPortDto> = ports
            .iter()
            .filter(|port| endpoint_matches_root(&port.parent_id, root_prefix, &p.name))
            .cloned()
            .collect();
        let focused_connectors: Vec<IbdConnectorDto> = connectors
            .iter()
            .filter(|connector| {
                endpoint_matches_root(&connector.source_id, root_prefix, &p.name)
                    && endpoint_matches_root(&connector.target_id, root_prefix, &p.name)
            })
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
