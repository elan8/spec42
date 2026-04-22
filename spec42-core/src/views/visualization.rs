use std::collections::{HashMap, HashSet};
use std::time::Instant;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Url;

use crate::semantic_model;
use crate::views::extracted_model::{extract_activity_diagrams, ActivityDiagramDto};
use crate::views::model_projection;
use crate::views::dto::{
    range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlElementDto, SysmlGraphDto,
    SysmlModelStatsDto, SysmlVisualizationGroupDto, SysmlVisualizationPackageCandidateDto,
    SysmlVisualizationParamsDto,
    SysmlVisualizationResultDto, WorkspaceFileModelDto,
    WorkspaceModelDto, WorkspaceModelSummaryDto,
};
use crate::views::ibd::{self, IbdDataDto, IbdPackageContainerGroupDto};

#[path = "explicit_views.rs"]
mod explicit_views;

fn normalize_package_path(value: &str) -> String {
    value.replace('.', "::").trim().to_string()
}

fn diagram_matches_package_filter(
    diagram: &ActivityDiagramDto,
    package_ref: &str,
    package_name: Option<&str>,
) -> bool {
    let diagram_path = normalize_package_path(&diagram.package_path);
    let normalized_ref = normalize_package_path(package_ref);
    let normalized_name = package_name.map(normalize_package_path);

    if !normalized_ref.is_empty()
        && (diagram_path == normalized_ref
            || diagram_path.starts_with(&format!("{normalized_ref}::")))
    {
        return true;
    }

    if let Some(name) = normalized_name {
        if !name.is_empty()
            && (diagram_path == name || diagram_path.starts_with(&format!("{name}::")))
        {
            return true;
        }
    }

    false
}

fn build_workspace_activity_diagrams(
    index: &std::collections::HashMap<Url, crate::workspace::state::IndexEntry>,
    workspace_uris: &[Url],
    package_filter: Option<(&str, Option<&str>)>,
) -> Vec<ActivityDiagramDto> {
    let mut diagrams = Vec::new();
    for workspace_uri in workspace_uris {
        let Some(entry) = index.get(workspace_uri) else {
            continue;
        };
        let Some(parsed) = entry.parsed.as_ref() else {
            continue;
        };
        diagrams.extend(extract_activity_diagrams(parsed));
    }

    if let Some((package_ref, package_name)) = package_filter {
        diagrams.retain(|diagram| diagram_matches_package_filter(diagram, package_ref, package_name));
    }

    diagrams
}

pub(crate) fn parse_sysml_visualization_params(
    v: &serde_json::Value,
) -> Result<(Url, String, Option<String>)> {
    let params = if let Ok(params) = serde_json::from_value::<SysmlVisualizationParamsDto>(v.clone())
    {
        params
    } else if let Some(arr) = v.as_array() {
        let first = arr.first().ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/visualization params array must have at least one element",
            )
        })?;
        if let Some(obj) = first.as_object() {
            SysmlVisualizationParamsDto {
                workspace_root_uri: obj
                    .get("workspaceRootUri")
                    .and_then(|value| value.as_str())
                    .unwrap_or_default()
                    .to_string(),
                view: obj
                    .get("view")
                    .and_then(|value| value.as_str())
                    .or_else(|| arr.get(1).and_then(|value| value.as_str()))
                    .unwrap_or_default()
                    .to_string(),
                selected_view: obj
                    .get("selectedView")
                    .and_then(|value| value.as_str())
                    .or_else(|| arr.get(2).and_then(|value| value.as_str()))
                    .map(ToString::to_string),
            }
        } else {
            SysmlVisualizationParamsDto {
                workspace_root_uri: first.as_str().unwrap_or_default().to_string(),
                view: arr
                    .get(1)
                    .and_then(|value| value.as_str())
                    .unwrap_or_default()
                    .to_string(),
                selected_view: arr
                    .get(2)
                    .and_then(|value| value.as_str())
                    .map(ToString::to_string),
            }
        }
    } else {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/visualization params must include workspaceRootUri and view",
        ));
    };

    if params.workspace_root_uri.trim().is_empty() || params.view.trim().is_empty() {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/visualization params must include workspaceRootUri and view",
        ));
    }

    let workspace_root_uri = Url::parse(&params.workspace_root_uri).map_err(|_| {
        tower_lsp::jsonrpc::Error::invalid_params("sysml/visualization: invalid workspaceRootUri")
    })?;

    Ok((
        crate::common::util::normalize_file_uri(&workspace_root_uri),
        params.view,
        params.selected_view,
    ))
}

fn clone_element(element: &SysmlElementDto) -> SysmlElementDto {
    SysmlElementDto {
        id: element.id.clone(),
        element_type: element.element_type.clone(),
        name: element.name.clone(),
        uri: element.uri.clone(),
        range: element.range.clone(),
        children: element.children.iter().map(clone_element).collect(),
        attributes: element.attributes.clone(),
        relationships: element.relationships.clone(),
        errors: element.errors.clone(),
    }
}

fn uri_under_root(uri: &Url, workspace_root_uri: &Url) -> bool {
    match (uri.to_file_path(), workspace_root_uri.to_file_path()) {
        (Ok(uri_path), Ok(root_path)) => uri_path.starts_with(root_path),
        _ => {
            let root = workspace_root_uri.as_str().trim_end_matches('/');
            uri.as_str() == root || uri.as_str().starts_with(&format!("{root}/"))
        }
    }
}

fn workspace_uris_for_root(
    semantic_graph: &semantic_model::SemanticGraph,
    library_paths: &[Url],
    workspace_root_uri: &Url,
) -> Vec<Url> {
    let mut uris: Vec<Url> = semantic_graph
        .workspace_uris_excluding_libraries(library_paths)
        .into_iter()
        .filter(|uri| uri_under_root(uri, workspace_root_uri))
        .collect();
    uris.sort_by(|left, right| left.as_str().cmp(right.as_str()));
    uris
}

fn build_workspace_graph_dto_for_uris(
    semantic_graph: &semantic_model::SemanticGraph,
    workspace_uris: &[Url],
) -> SysmlGraphDto {
    let workspace_uri_set: HashSet<Url> = workspace_uris.iter().cloned().collect();
    let mut nodes = Vec::new();
    let mut node_ids = HashSet::new();
    for workspace_uri in workspace_uris {
        for node in semantic_graph.nodes_for_uri(workspace_uri) {
            node_ids.insert(node.id.qualified_name.clone());
            nodes.push(GraphNodeDto {
                id: node.id.qualified_name.clone(),
                element_type: node.element_kind.clone(),
                name: node.name.clone(),
                uri: Some(node.id.uri.as_str().to_string()),
                parent_id: node.parent_id.as_ref().map(|parent| parent.qualified_name.clone()),
                range: range_to_dto(node.range),
                attributes: node.attributes.clone(),
            });
        }
    }

    let mut edge_keys = HashSet::new();
    let mut edges = Vec::new();
    for workspace_uri in workspace_uris {
        for (source, target, kind, name) in semantic_graph.edges_for_uri_as_strings(workspace_uri) {
            let key = (source.clone(), target.clone(), kind.as_str().to_string(), name.clone());
            if edge_keys.insert(key) {
                edges.push(GraphEdgeDto {
                    source,
                    target,
                    rel_type: kind.as_str().to_string(),
                    name,
                });
            }
        }
    }

    for workspace_uri in workspace_uris {
        for node in semantic_graph.nodes_for_uri(workspace_uri) {
            if let Some(parent_id) = &node.parent_id {
                if workspace_uri_set.contains(&parent_id.uri)
                    && node_ids.contains(&parent_id.qualified_name)
                    && node_ids.contains(&node.id.qualified_name)
                {
                    let key = (
                        parent_id.qualified_name.clone(),
                        node.id.qualified_name.clone(),
                        "contains".to_string(),
                        None::<String>,
                    );
                    if edge_keys.insert(key) {
                        edges.push(GraphEdgeDto {
                            source: parent_id.qualified_name.clone(),
                            target: node.id.qualified_name.clone(),
                            rel_type: "contains".to_string(),
                            name: None,
                        });
                    }
                }
            }
        }
    }

    SysmlGraphDto { nodes, edges }
}

fn graph_to_element_tree(graph: &SysmlGraphDto, uri: &Url) -> Vec<SysmlElementDto> {
    let contains_targets: HashSet<&str> = graph
        .edges
        .iter()
        .filter(|edge| edge.rel_type.eq_ignore_ascii_case("contains"))
        .map(|edge| edge.target.as_str())
        .collect();

    let nodes_by_id: HashMap<&str, &GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let mut child_ids_by_parent: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut outgoing_relationships: HashMap<&str, Vec<crate::views::dto::RelationshipDto>> =
        HashMap::new();

    for edge in &graph.edges {
        if edge.rel_type.eq_ignore_ascii_case("contains") {
            child_ids_by_parent
                .entry(edge.source.as_str())
                .or_default()
                .push(edge.target.as_str());
        } else {
            outgoing_relationships
                .entry(edge.source.as_str())
                .or_default()
                .push(crate::views::dto::RelationshipDto {
                    rel_type: edge.rel_type.clone(),
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    name: edge.name.clone(),
                });
        }
    }

    fn build_element(
        node_id: &str,
        uri: &Url,
        nodes_by_id: &HashMap<&str, &GraphNodeDto>,
        child_ids_by_parent: &HashMap<&str, Vec<&str>>,
        outgoing_relationships: &HashMap<&str, Vec<crate::views::dto::RelationshipDto>>,
    ) -> Option<SysmlElementDto> {
        let node = nodes_by_id.get(node_id)?;
        let children = child_ids_by_parent
            .get(node_id)
            .into_iter()
            .flatten()
            .filter_map(|child_id| {
                build_element(
                    child_id,
                    uri,
                    nodes_by_id,
                    child_ids_by_parent,
                    outgoing_relationships,
                )
            })
            .collect();
        Some(SysmlElementDto {
            id: Some(node.id.clone()),
            element_type: node.element_type.clone(),
            name: node.name.clone(),
            uri: Some(
                node.uri
                    .clone()
                    .unwrap_or_else(|| uri.as_str().to_string()),
            ),
            range: node.range.clone(),
            children,
            attributes: node.attributes.clone(),
            relationships: outgoing_relationships
                .get(node_id)
                .cloned()
                .unwrap_or_default(),
            errors: None,
        })
    }

    graph
        .nodes
        .iter()
        .filter(|node| !contains_targets.contains(node.id.as_str()))
        .filter_map(|node| {
            build_element(
                node.id.as_str(),
                uri,
                &nodes_by_id,
                &child_ids_by_parent,
                &outgoing_relationships,
            )
        })
        .collect()
}

fn project_graph_by_ids(graph: &SysmlGraphDto, visible_ids: &HashSet<String>) -> SysmlGraphDto {
    let nodes: Vec<_> = graph
        .nodes
        .iter()
        .filter(|node| visible_ids.contains(&node.id))
        .cloned()
        .collect();
    let edges: Vec<_> = graph
        .edges
        .iter()
        .filter(|edge| visible_ids.contains(&edge.source) && visible_ids.contains(&edge.target))
        .cloned()
        .collect();
    SysmlGraphDto { nodes, edges }
}

fn build_workspace_model_dto_from_graph(
    graph: &SysmlGraphDto,
    workspace_uris: &[Url],
) -> WorkspaceModelDto {
    let mut files = Vec::with_capacity(workspace_uris.len());
    let mut all_elements = Vec::new();
    for workspace_uri in workspace_uris {
        let uri_graph = SysmlGraphDto {
            nodes: graph
                .nodes
                .iter()
                .filter(|node| node.uri.as_deref() == Some(workspace_uri.as_str()))
                .cloned()
                .collect(),
            edges: graph
                .edges
                .iter()
                .filter(|edge| {
                    graph.nodes.iter().any(|node| node.id == edge.source && node.uri.as_deref() == Some(workspace_uri.as_str()))
                        || graph.nodes.iter().any(|node| node.id == edge.target && node.uri.as_deref() == Some(workspace_uri.as_str()))
                })
                .cloned()
                .collect(),
        };
        let elements = graph_to_element_tree(&uri_graph, workspace_uri);
        all_elements.extend(elements.iter().map(clone_element));
        if !elements.is_empty() {
            files.push(WorkspaceFileModelDto {
                uri: workspace_uri.as_str().to_string(),
                elements,
            });
        }
    }
    WorkspaceModelDto {
        summary: WorkspaceModelSummaryDto {
            scanned_files: files.len(),
            loaded_files: files.len(),
            failures: 0,
            truncated: false,
        },
        semantic: merge_namespace_elements(&all_elements),
        files,
    }
}

fn filter_activity_diagrams_by_graph(
    diagrams: &[ActivityDiagramDto],
    graph: &SysmlGraphDto,
) -> Vec<ActivityDiagramDto> {
    let mut action_keys = HashSet::new();
    for node in &graph.nodes {
        let kind = node.element_type.to_lowercase();
        if kind.contains("action") || kind.contains("perform") {
            action_keys.insert((node.name.clone(), top_level_package_for_node_id(&node.id)));
        }
    }

    diagrams
        .iter()
        .filter(|diagram| {
            let package = normalize_package_path(&diagram.package_path)
                .split("::")
                .next()
                .unwrap_or("")
                .to_string();
            action_keys.contains(&(diagram.name.clone(), package))
        })
        .cloned()
        .collect()
}

fn top_level_package_for_node_id(node_id: &str) -> String {
    normalize_package_path(node_id)
        .split("::")
        .next()
        .unwrap_or("")
        .to_string()
}

fn build_package_groups_from_graph(graph: &SysmlGraphDto) -> Vec<SysmlVisualizationGroupDto> {
    let contains_edges: Vec<_> = graph
        .edges
        .iter()
        .filter(|edge| edge.rel_type.eq_ignore_ascii_case("contains"))
        .collect();
    if contains_edges.is_empty() {
        return Vec::new();
    }

    let node_by_id: HashMap<&str, &GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let package_ids: HashSet<&str> = graph
        .nodes
        .iter()
        .filter(|node| node.element_type.to_lowercase().contains("package"))
        .map(|node| node.id.as_str())
        .collect();
    let mut children_by_parent: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut package_parent: HashMap<&str, &str> = HashMap::new();
    for edge in contains_edges {
        children_by_parent
            .entry(edge.source.as_str())
            .or_default()
            .push(edge.target.as_str());
        if package_ids.contains(edge.source.as_str()) && package_ids.contains(edge.target.as_str()) {
            package_parent.insert(edge.target.as_str(), edge.source.as_str());
        }
    }

    fn collect_non_package_descendants<'a>(
        package_id: &'a str,
        package_ids: &HashSet<&'a str>,
        children_by_parent: &HashMap<&'a str, Vec<&'a str>>,
    ) -> Vec<&'a str> {
        let mut out = Vec::new();
        let mut stack: Vec<&str> = children_by_parent
            .get(package_id)
            .cloned()
            .unwrap_or_default();
        let mut visited: HashSet<&str> = HashSet::new();
        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }
            if !package_ids.contains(current) {
                out.push(current);
            }
            if let Some(children) = children_by_parent.get(current) {
                stack.extend(children.iter().copied());
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    let mut groups = Vec::new();
    for package_id in &package_ids {
        let Some(package_node) = node_by_id.get(package_id) else {
            continue;
        };
        let node_ids: Vec<String> = collect_non_package_descendants(
            package_id,
            &package_ids,
            &children_by_parent,
        )
        .into_iter()
        .map(String::from)
        .collect();
        if node_ids.is_empty() {
            continue;
        }
        let mut depth = 1usize;
        let mut parent = package_parent.get(package_id).copied();
        while let Some(parent_id) = parent {
            depth += 1;
            parent = package_parent.get(parent_id).copied();
        }
        groups.push(SysmlVisualizationGroupDto {
            id: (*package_id).to_string(),
            label: package_node.name.clone(),
            depth,
            parent_id: package_parent
                .get(package_id)
                .map(|value| value.to_string()),
            node_ids,
        });
    }
    groups.sort_by(|left, right| {
        left.depth
            .cmp(&right.depth)
            .then_with(|| left.label.cmp(&right.label))
            .then_with(|| left.id.cmp(&right.id))
    });
    groups
}

fn merge_namespace_elements(elements: &[SysmlElementDto]) -> Vec<SysmlElementDto> {
    let namespace_types = ["package"];
    let mut merged_by_key: HashMap<String, usize> = HashMap::new();
    let mut merged = Vec::new();

    for element in elements {
        let key = format!("{}::{}", element.element_type, element.name);
        if namespace_types.contains(&element.element_type.as_str()) {
            if let Some(existing_index) = merged_by_key.get(&key).copied() {
                let next = merge_two_elements(&merged[existing_index], element);
                merged[existing_index] = next;
            } else {
                merged_by_key.insert(key, merged.len());
                merged.push(clone_element(element));
            }
        } else {
            merged.push(clone_element(element));
        }
    }

    merged
}

fn merge_two_elements(a: &SysmlElementDto, b: &SysmlElementDto) -> SysmlElementDto {
    let namespace_types = ["package"];
    let mut child_by_key: HashMap<String, SysmlElementDto> = a
        .children
        .iter()
        .map(|child| {
            (
                format!("{}::{}", child.element_type, child.name),
                clone_element(child),
            )
        })
        .collect();

    for child in &b.children {
        let key = format!("{}::{}", child.element_type, child.name);
        if namespace_types.contains(&child.element_type.as_str()) {
            if let Some(existing_child) = child_by_key.get(&key).cloned() {
                child_by_key.insert(key, merge_two_elements(&existing_child, child));
            } else {
                child_by_key.insert(key, clone_element(child));
            }
        } else {
            child_by_key
                .entry(key)
                .or_insert_with(|| clone_element(child));
        }
    }

    let mut relationship_keys: HashSet<String> = a
        .relationships
        .iter()
        .map(|rel| format!("{}::{}::{}", rel.rel_type, rel.source, rel.target))
        .collect();
    let mut relationships = a.relationships.clone();
    for relationship in &b.relationships {
        let key = format!(
            "{}::{}::{}",
            relationship.rel_type, relationship.source, relationship.target
        );
        if relationship_keys.insert(key) {
            relationships.push(relationship.clone());
        }
    }

    let mut attributes = a.attributes.clone();
    attributes.extend(b.attributes.clone());

    SysmlElementDto {
        id: a.id.clone().or_else(|| b.id.clone()),
        element_type: a.element_type.clone(),
        name: a.name.clone(),
        uri: a.uri.clone().or_else(|| b.uri.clone()),
        range: a.range.clone(),
        children: child_by_key.into_values().collect(),
        attributes,
        relationships,
        errors: a.errors.clone().or_else(|| b.errors.clone()),
    }
}

#[allow(dead_code)]
fn build_workspace_model_dto_for_uris(
    semantic_graph: &semantic_model::SemanticGraph,
    workspace_uris: &[Url],
) -> WorkspaceModelDto {
    let mut files = Vec::with_capacity(workspace_uris.len());
    let mut all_elements = Vec::new();

    for workspace_uri in workspace_uris {
        let graph = model_projection::strip_synthetic_nodes(&build_workspace_graph_dto_for_uris(
            semantic_graph,
            std::slice::from_ref(workspace_uri),
        ));
        let elements = graph_to_element_tree(&graph, workspace_uri);
        all_elements.extend(elements.iter().map(clone_element));
        files.push(WorkspaceFileModelDto {
            uri: workspace_uri.as_str().to_string(),
            elements,
        });
    }

    WorkspaceModelDto {
        summary: WorkspaceModelSummaryDto {
            scanned_files: files.len(),
            loaded_files: files.len(),
            failures: 0,
            truncated: false,
        },
        semantic: merge_namespace_elements(&all_elements),
        files,
    }
}

fn collect_package_candidates(
    elements: &[SysmlElementDto],
    seen: &mut HashSet<String>,
    out: &mut Vec<SysmlVisualizationPackageCandidateDto>,
) {
    for element in elements {
        if element.element_type.to_lowercase().contains("package") {
            let id = element
                .id
                .clone()
                .unwrap_or_else(|| element.name.clone());
            if seen.insert(id.clone()) {
                out.push(SysmlVisualizationPackageCandidateDto {
                    id,
                    name: element.name.clone(),
                });
            }
        }
        collect_package_candidates(&element.children, seen, out);
    }
}

#[allow(dead_code)]
fn find_package_element<'a>(
    elements: &'a [SysmlElementDto],
    package_ref: &str,
) -> Option<&'a SysmlElementDto> {
    for element in elements {
        if element.element_type.to_lowercase().contains("package")
            && (element.id.as_deref() == Some(package_ref) || element.name == package_ref)
        {
            return Some(element);
        }
        if let Some(found) = find_package_element(&element.children, package_ref) {
            return Some(found);
        }
    }
    None
}

#[allow(dead_code)]
fn collect_subtree_ids(element: &SysmlElementDto, ids: &mut HashSet<String>) {
    if let Some(id) = &element.id {
        ids.insert(id.clone());
    }
    for child in &element.children {
        collect_subtree_ids(child, ids);
    }
}

#[allow(dead_code)]
fn filter_workspace_model_files(
    files: &[WorkspaceFileModelDto],
    package_ref: &str,
) -> Vec<WorkspaceFileModelDto> {
    files.iter()
        .filter_map(|file| {
            let matched = find_package_element(&file.elements, package_ref)?;
            Some(WorkspaceFileModelDto {
                uri: file.uri.clone(),
                elements: vec![clone_element(matched)],
            })
        })
        .collect()
}

fn within_package_prefix(value: &str, package_prefix: &str, dot_prefix: &str) -> bool {
    value == package_prefix
        || value.starts_with(&format!("{package_prefix}::"))
        || value == dot_prefix
        || value.starts_with(&format!("{dot_prefix}."))
}

#[allow(dead_code)]
fn filter_ibd_by_package(ibd: &IbdDataDto, package_ref: &str) -> IbdDataDto {
    let dot_prefix = package_ref.replace("::", ".");
    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| {
            within_package_prefix(&part.id, package_ref, &dot_prefix)
                || within_package_prefix(&part.qualified_name, package_ref, &dot_prefix)
        })
        .cloned()
        .collect();
    let part_ids: HashSet<String> = parts.iter().map(|part| part.qualified_name.clone()).collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| part_ids.contains(&port.parent_id))
        .cloned()
        .collect();
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            within_package_prefix(&connector.source_id, package_ref, &dot_prefix)
                && within_package_prefix(&connector.target_id, package_ref, &dot_prefix)
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group.member_part_ids.iter().any(|part_id| {
                parts
                    .iter()
                    .any(|part| &part.id == part_id || &part.qualified_name == part_id)
            })
        })
        .cloned()
        .collect();

    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| {
                within_package_prefix(&part.id, package_ref, &dot_prefix)
                    || within_package_prefix(&part.qualified_name, package_ref, &dot_prefix)
            })
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<String> =
            filtered_parts.iter().map(|part| part.qualified_name.clone()).collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| filtered_part_ids.contains(&port.parent_id))
            .cloned()
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                within_package_prefix(&connector.source_id, package_ref, &dot_prefix)
                    && within_package_prefix(&connector.target_id, package_ref, &dot_prefix)
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group.member_part_ids.iter().any(|part_id| {
                    filtered_parts
                        .iter()
                        .any(|part| &part.id == part_id || &part.qualified_name == part_id)
                })
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                ibd::IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }

    let root_candidates: Vec<String> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = root_candidates.first().cloned();

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

fn filter_ibd_by_visible_ids(ibd: &IbdDataDto, visible_ids: &HashSet<String>) -> IbdDataDto {
    let visible_dot_ids: HashSet<String> =
        visible_ids.iter().map(|id| id.replace("::", ".")).collect();
    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| {
            visible_ids.contains(&part.id)
                || visible_dot_ids.contains(&part.qualified_name)
        })
        .cloned()
        .collect();
    let part_ids: HashSet<_> = parts.iter().map(|part| part.id.as_str()).collect();
    let part_qualified_names: HashSet<_> =
        parts.iter().map(|part| part.qualified_name.as_str()).collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| {
            visible_ids.contains(&port.id)
                || visible_dot_ids.contains(&port.id.replace("::", "."))
                || part_ids.contains(port.parent_id.as_str())
                || part_qualified_names.contains(port.parent_id.as_str())
        })
        .cloned()
        .collect();
    let port_ids: HashSet<_> = ports.iter().map(|port| port.id.as_str()).collect();
    let port_dot_ids: HashSet<String> = ports
        .iter()
        .map(|port| port.id.replace("::", "."))
        .collect();
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            port_ids.contains(connector.source.as_str())
                || port_ids.contains(connector.target.as_str())
                || port_dot_ids.contains(&connector.source_id)
                || port_dot_ids.contains(&connector.target_id)
                || part_ids.contains(connector.source_id.as_str())
                || part_ids.contains(connector.target_id.as_str())
                || part_qualified_names.contains(connector.source_id.as_str())
                || part_qualified_names.contains(connector.target_id.as_str())
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group.member_part_ids
                .iter()
                .any(|member_id| part_ids.contains(member_id.as_str()))
        })
        .cloned()
        .collect();
    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| {
                visible_ids.contains(&part.id)
                    || visible_dot_ids.contains(&part.qualified_name)
            })
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<_> =
            filtered_parts.iter().map(|part| part.id.as_str()).collect();
        let filtered_part_qualified_names: HashSet<_> = filtered_parts
            .iter()
            .map(|part| part.qualified_name.as_str())
            .collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| {
                visible_ids.contains(&port.id)
                    || visible_dot_ids.contains(&port.id.replace("::", "."))
                    || filtered_part_ids.contains(port.parent_id.as_str())
                    || filtered_part_qualified_names.contains(port.parent_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_port_ids: HashSet<_> =
            filtered_ports.iter().map(|port| port.id.as_str()).collect();
        let filtered_port_dot_ids: HashSet<String> = filtered_ports
            .iter()
            .map(|port| port.id.replace("::", "."))
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                filtered_port_ids.contains(connector.source.as_str())
                    || filtered_port_ids.contains(connector.target.as_str())
                    || filtered_port_dot_ids.contains(&connector.source_id)
                    || filtered_port_dot_ids.contains(&connector.target_id)
                    || filtered_part_ids.contains(connector.source_id.as_str())
                    || filtered_part_ids.contains(connector.target_id.as_str())
                    || filtered_part_qualified_names.contains(connector.source_id.as_str())
                    || filtered_part_qualified_names.contains(connector.target_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group.member_part_ids
                    .iter()
                    .any(|member_id| filtered_part_ids.contains(member_id.as_str()))
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                ibd::IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }
    let root_candidates: Vec<_> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = ibd
        .default_root
        .as_ref()
        .filter(|root| root_views.contains_key(root.as_str()))
        .cloned()
        .or_else(|| root_candidates.first().cloned());
    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        root_views,
        default_root,
    }
}

fn endpoint_matches_root_prefix(endpoint: &str, root_prefix: &str) -> bool {
    endpoint == root_prefix || endpoint.starts_with(&format!("{root_prefix}."))
}

fn filter_ibd_by_root_prefixes(ibd: &IbdDataDto, root_prefixes: &HashSet<String>) -> IbdDataDto {
    let matches_any_root =
        |endpoint: &str| root_prefixes.iter().any(|prefix| endpoint_matches_root_prefix(endpoint, prefix));

    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| matches_any_root(&part.qualified_name))
        .cloned()
        .collect();
    let part_ids: HashSet<_> = parts.iter().map(|part| part.id.as_str()).collect();
    let part_qualified_names: HashSet<_> =
        parts.iter().map(|part| part.qualified_name.as_str()).collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| {
            matches_any_root(&port.parent_id)
                || part_ids.contains(port.parent_id.as_str())
                || part_qualified_names.contains(port.parent_id.as_str())
        })
        .cloned()
        .collect();
    let port_ids: HashSet<_> = ports.iter().map(|port| port.id.as_str()).collect();
    let port_dot_ids: HashSet<String> = ports
        .iter()
        .map(|port| port.id.replace("::", "."))
        .collect();
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            matches_any_root(&connector.source_id)
                || matches_any_root(&connector.target_id)
                || port_ids.contains(connector.source.as_str())
                || port_ids.contains(connector.target.as_str())
                || port_dot_ids.contains(&connector.source_id)
                || port_dot_ids.contains(&connector.target_id)
                || part_qualified_names.contains(connector.source_id.as_str())
                || part_qualified_names.contains(connector.target_id.as_str())
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group.member_part_ids
                .iter()
                .any(|member_id| part_ids.contains(member_id.as_str()))
        })
        .cloned()
        .collect();

    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| matches_any_root(&part.qualified_name))
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<_> =
            filtered_parts.iter().map(|part| part.id.as_str()).collect();
        let filtered_part_qualified_names: HashSet<_> = filtered_parts
            .iter()
            .map(|part| part.qualified_name.as_str())
            .collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| {
                matches_any_root(&port.parent_id)
                    || filtered_part_ids.contains(port.parent_id.as_str())
                    || filtered_part_qualified_names.contains(port.parent_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_port_ids: HashSet<_> =
            filtered_ports.iter().map(|port| port.id.as_str()).collect();
        let filtered_port_dot_ids: HashSet<String> = filtered_ports
            .iter()
            .map(|port| port.id.replace("::", "."))
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                matches_any_root(&connector.source_id)
                    || matches_any_root(&connector.target_id)
                    || filtered_port_ids.contains(connector.source.as_str())
                    || filtered_port_ids.contains(connector.target.as_str())
                    || filtered_port_dot_ids.contains(&connector.source_id)
                    || filtered_port_dot_ids.contains(&connector.target_id)
                    || filtered_part_qualified_names.contains(connector.source_id.as_str())
                    || filtered_part_qualified_names.contains(connector.target_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group.member_part_ids
                    .iter()
                    .any(|member_id| filtered_part_ids.contains(member_id.as_str()))
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                ibd::IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }

    let root_candidates: Vec<_> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = ibd
        .default_root
        .as_ref()
        .filter(|root| root_views.contains_key(root.as_str()))
        .cloned()
        .or_else(|| root_candidates.first().cloned());
    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        root_views,
        default_root,
    }
}

fn renderer_empty_state_message(view: &str) -> String {
    match view {
        "general-view" => {
            "Define a SysML view typed by GeneralView to display something in this visualizer panel."
                .to_string()
        }
        "interconnection-view" => {
            "Define a SysML view typed by InterconnectionView to display something in this visualizer panel."
                .to_string()
        }
        "action-flow-view" => {
            "Define a SysML view typed by ActionFlowView to display something in this visualizer panel."
                .to_string()
        }
        "state-transition-view" => {
            "Define a SysML view typed by StateTransitionView to display something in this visualizer panel."
                .to_string()
        }
        _ => "Define a SysML view to display something in this visualizer panel.".to_string(),
    }
}

fn no_defined_views_message() -> String {
    "Define a SysML view with expose (and optional filter) to use the visualizer.".to_string()
}

fn unsupported_view_type_message(view_type: Option<&str>) -> String {
    match view_type.filter(|value| !value.trim().is_empty()) {
        Some(view_type) => format!(
            "This SysML view is typed by {view_type}, which Spec42 does not support yet."
        ),
        None => "This SysML view uses a view type that Spec42 does not support yet.".to_string(),
    }
}

fn package_group_id(package_ref: &str) -> String {
    format!("package:{}", package_ref.replace("::", "."))
}

fn package_group_label(package_ref: &str, fallback_name: Option<&str>) -> String {
    fallback_name
        .map(String::from)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| {
            package_ref
                .rsplit("::")
                .next()
                .map(String::from)
                .unwrap_or_else(|| package_ref.to_string())
        })
}

fn package_group_parent_id<'a>(
    package_ref: &str,
    candidate_ids: impl Iterator<Item = &'a str>,
) -> Option<String> {
    let mut best_parent: Option<&str> = None;
    for candidate_id in candidate_ids {
        if candidate_id == package_ref {
            continue;
        }
        if package_ref.starts_with(&format!("{candidate_id}::")) {
            match best_parent {
                Some(current) if current.len() >= candidate_id.len() => {}
                _ => best_parent = Some(candidate_id),
            }
        }
    }
    best_parent.map(package_group_id)
}

fn build_ibd_package_container_groups(
    parts: &[ibd::IbdPartDto],
    package_candidates: &[SysmlVisualizationPackageCandidateDto],
    selected_package: Option<(&str, Option<&str>)>,
) -> Vec<IbdPackageContainerGroupDto> {
    let selected_candidates: Vec<SysmlVisualizationPackageCandidateDto> = match selected_package {
        Some((package_ref, package_name)) => vec![SysmlVisualizationPackageCandidateDto {
            id: package_ref.to_string(),
            name: package_group_label(package_ref, package_name),
        }],
        None => package_candidates.to_vec(),
    };
    let candidate_ids: Vec<&str> = selected_candidates.iter().map(|candidate| candidate.id.as_str()).collect();

    let mut groups = Vec::new();
    for candidate in &selected_candidates {
        let dot_prefix = candidate.id.replace("::", ".");
        let member_part_ids: Vec<String> = parts
            .iter()
            .filter(|part| {
                within_package_prefix(&part.id, &candidate.id, &dot_prefix)
                    || within_package_prefix(&part.qualified_name, &candidate.id, &dot_prefix)
            })
            .map(|part| part.id.clone())
            .collect();
        if member_part_ids.is_empty() {
            continue;
        }
        groups.push(IbdPackageContainerGroupDto {
            id: package_group_id(&candidate.id),
            label: package_group_label(&candidate.id, Some(candidate.name.as_str())),
            qualified_package: candidate.id.clone(),
            parent_id: package_group_parent_id(&candidate.id, candidate_ids.iter().copied()),
            member_part_ids,
        });
    }
    groups.sort_by(|left, right| {
        left.qualified_package
            .matches("::")
            .count()
            .cmp(&right.qualified_package.matches("::").count())
            .then_with(|| left.label.cmp(&right.label))
            .then_with(|| left.qualified_package.cmp(&right.qualified_package))
    });
    groups
}

fn attach_ibd_package_container_groups(
    mut ibd: IbdDataDto,
    package_candidates: &[SysmlVisualizationPackageCandidateDto],
    selected_package: Option<(&str, Option<&str>)>,
) -> IbdDataDto {
    ibd.package_container_groups =
        build_ibd_package_container_groups(&ibd.parts, package_candidates, selected_package);

    for root_view in ibd.root_views.values_mut() {
        root_view.package_container_groups = build_ibd_package_container_groups(
            &root_view.parts,
            package_candidates,
            selected_package,
        );
    }

    ibd
}

pub(crate) fn build_sysml_visualization_response(
    semantic_graph: &semantic_model::SemanticGraph,
    index: &std::collections::HashMap<Url, crate::workspace::state::IndexEntry>,
    workspace_root_uri: &Url,
    library_paths: &[Url],
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
) -> SysmlVisualizationResultDto {
    let workspace_uris = workspace_uris_for_root(semantic_graph, library_paths, workspace_root_uri);
    let graph = model_projection::strip_synthetic_nodes(&build_workspace_graph_dto_for_uris(
        semantic_graph,
        &workspace_uris,
    ));
    let empty_graph = SysmlGraphDto {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    let full_ibd = ibd::merge_ibd_payloads(
        workspace_uris
            .iter()
            .map(|workspace_uri| ibd::build_ibd_for_uri(semantic_graph, workspace_uri))
            .collect(),
    );
    let full_activity_diagrams = build_workspace_activity_diagrams(index, &workspace_uris, None);
    let catalog = explicit_views::build_view_catalog(index, &workspace_uris);

    if catalog.usages.is_empty() {
        return SysmlVisualizationResultDto {
            version: 0,
            view: view.to_string(),
            workspace_root_uri: workspace_root_uri.as_str().to_string(),
            view_candidates: Vec::new(),
            selected_view: None,
            selected_view_name: None,
            empty_state_message: Some(no_defined_views_message()),
            package_groups: Some(Vec::new()),
            graph: Some(empty_graph.clone()),
            general_view_graph: Some(empty_graph.clone()),
            workspace_model: Some(build_workspace_model_dto_from_graph(&empty_graph, &workspace_uris)),
            activity_diagrams: Some(Vec::new()),
            ibd: Some(filter_ibd_by_visible_ids(&full_ibd, &HashSet::new())),
            stats: Some(SysmlModelStatsDto {
                total_elements: 0,
                resolved_elements: 0,
                unresolved_elements: 0,
                parse_time_ms: 0,
                model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                parse_cached: false,
            }),
        };
    }

    let evaluated_views = explicit_views::evaluate_views(&catalog, &graph);
    let mut projected_graphs: HashMap<&str, SysmlGraphDto> = HashMap::new();
    let mut projected_activity_diagrams: HashMap<&str, Vec<ActivityDiagramDto>> = HashMap::new();
    for evaluated in &evaluated_views {
        let projected_ids = explicit_views::renderer_view_for_view_type(
            evaluated.effective_view_type.as_deref(),
        )
        .map(|renderer_view| explicit_views::project_ids_for_renderer(evaluated, &graph, renderer_view))
        .unwrap_or_default();
        let projected_graph = project_graph_by_ids(&graph, &projected_ids);
        let diagrams = filter_activity_diagrams_by_graph(&full_activity_diagrams, &projected_graph);
        projected_graphs.insert(evaluated.id.as_str(), projected_graph);
        projected_activity_diagrams.insert(evaluated.id.as_str(), diagrams);
    }

    let view_candidates = explicit_views::build_view_candidates(
        &evaluated_views,
        &projected_activity_diagrams,
        &projected_graphs,
    );
    let selected_candidate = selected_view
        .and_then(|selected| view_candidates.iter().find(|candidate| candidate.id == selected))
        .or_else(|| {
            view_candidates
                .iter()
                .find(|candidate| candidate.supported && candidate.renderer_view.as_deref() == Some(view))
        })
        .or_else(|| view_candidates.iter().find(|candidate| candidate.supported))
        .or_else(|| view_candidates.first());

    let Some(selected_candidate) = selected_candidate else {
        return SysmlVisualizationResultDto {
            version: 0,
            view: view.to_string(),
            workspace_root_uri: workspace_root_uri.as_str().to_string(),
            view_candidates,
            selected_view: None,
            selected_view_name: None,
            empty_state_message: Some(renderer_empty_state_message(view)),
            package_groups: Some(Vec::new()),
            graph: Some(empty_graph.clone()),
            general_view_graph: Some(empty_graph.clone()),
            workspace_model: Some(build_workspace_model_dto_from_graph(&empty_graph, &workspace_uris)),
            activity_diagrams: Some(Vec::new()),
            ibd: Some(filter_ibd_by_visible_ids(&full_ibd, &HashSet::new())),
            stats: Some(SysmlModelStatsDto {
                total_elements: 0,
                resolved_elements: 0,
                unresolved_elements: 0,
                parse_time_ms: 0,
                model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                parse_cached: false,
            }),
        };
    };
    let selected_view_id = selected_candidate.id.clone();
    let selected_view_name = Some(selected_candidate.name.clone());
    let selected_view_type = selected_candidate.view_type.clone();

    if !selected_candidate.supported {
        return SysmlVisualizationResultDto {
            version: 0,
            view: view.to_string(),
            workspace_root_uri: workspace_root_uri.as_str().to_string(),
            view_candidates,
            selected_view: Some(selected_view_id),
            selected_view_name,
            empty_state_message: Some(unsupported_view_type_message(
                selected_view_type.as_deref(),
            )),
            package_groups: Some(Vec::new()),
            graph: Some(empty_graph.clone()),
            general_view_graph: Some(empty_graph.clone()),
            workspace_model: Some(build_workspace_model_dto_from_graph(&empty_graph, &workspace_uris)),
            activity_diagrams: Some(Vec::new()),
            ibd: Some(filter_ibd_by_visible_ids(&full_ibd, &HashSet::new())),
            stats: Some(SysmlModelStatsDto {
                total_elements: 0,
                resolved_elements: 0,
                unresolved_elements: 0,
                parse_time_ms: 0,
                model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
                parse_cached: false,
            }),
        };
    }

    let resolved_view = selected_candidate
        .renderer_view
        .clone()
        .unwrap_or_else(|| view.to_string());

    let selected_graph = projected_graphs
        .get(selected_view_id.as_str())
        .cloned()
        .unwrap_or_else(|| empty_graph.clone());
    let general_view_graph =
        model_projection::canonical_general_view_graph(&selected_graph, true);
    let package_groups = Some(build_package_groups_from_graph(&general_view_graph));
    let workspace_model = build_workspace_model_dto_from_graph(&selected_graph, &workspace_uris);
    let mut package_candidates = Vec::new();
    let mut seen_packages = HashSet::new();
    collect_package_candidates(
        &workspace_model.semantic,
        &mut seen_packages,
        &mut package_candidates,
    );
    package_candidates.sort_by(|left, right| left.name.cmp(&right.name));
    let selected_ids: HashSet<String> = selected_graph
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .collect();
    let selected_evaluated = evaluated_views
        .iter()
        .find(|evaluated| evaluated.id == selected_view_id);
    let filtered_ibd = attach_ibd_package_container_groups(
        if resolved_view == "interconnection-view" {
            let root_prefixes: HashSet<String> = selected_evaluated
                .map(|evaluated| {
                    evaluated
                        .exposed_ids
                        .iter()
                        .map(|id| id.replace("::", "."))
                        .collect()
                })
                .unwrap_or_default();
            if root_prefixes.is_empty() {
                filter_ibd_by_visible_ids(&full_ibd, &selected_ids)
            } else {
                filter_ibd_by_root_prefixes(&full_ibd, &root_prefixes)
            }
        } else {
            filter_ibd_by_visible_ids(&full_ibd, &selected_ids)
        },
        &package_candidates,
        None,
    );
    let activity_diagrams = projected_activity_diagrams
        .remove(selected_view_id.as_str())
        .unwrap_or_default();

    SysmlVisualizationResultDto {
        version: 0,
        view: resolved_view,
        workspace_root_uri: workspace_root_uri.as_str().to_string(),
        view_candidates,
        selected_view: Some(selected_view_id),
        selected_view_name,
        empty_state_message: None,
        package_groups,
        graph: Some(selected_graph.clone()),
        general_view_graph: Some(general_view_graph),
        workspace_model: Some(workspace_model),
        activity_diagrams: Some(activity_diagrams),
        ibd: Some(filtered_ibd),
        stats: Some(SysmlModelStatsDto {
            total_elements: selected_graph.nodes.len() as u32,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
            parse_cached: false,
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        attach_ibd_package_container_groups, build_package_groups_from_graph,
        build_ibd_package_container_groups, build_workspace_activity_diagrams,
        parse_sysml_visualization_params,
    };
    use crate::views::dto::{GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto, SysmlGraphDto};
    use crate::views::ibd::{IbdDataDto, IbdPartDto, IbdRootViewDto};
    use crate::workspace::state::{IndexEntry, ParseMetadata};
    use sysml_v2_parser::parse;
    use tower_lsp::lsp_types::Url;

    fn zero_range() -> RangeDto {
        RangeDto {
            start: PositionDto {
                line: 0,
                character: 0,
            },
            end: PositionDto {
                line: 0,
                character: 0,
            },
        }
    }

    #[test]
    fn parse_visualization_params_accepts_workspace_root_and_selected_view() {
        let params = serde_json::json!({
            "workspaceRootUri": "file:///C:/demo",
            "view": "general-view",
            "selectedView": "Demo::Pkg::VehicleView"
        });

        let (workspace_root_uri, view, selected_view) =
            parse_sysml_visualization_params(&params).expect("parse visualization params");
        assert_eq!(workspace_root_uri.as_str(), "file:///c:/demo");
        assert_eq!(view, "general-view");
        assert_eq!(selected_view.as_deref(), Some("Demo::Pkg::VehicleView"));
    }

    #[test]
    fn parse_visualization_params_accepts_array_shape() {
        let params = serde_json::json!([
            {
                "workspaceRootUri": "file:///C:/demo",
                "view": "interconnection-view",
                "selectedView": "Demo::VehicleConnections"
            }
        ]);

        let (workspace_root_uri, view, selected_view) =
            parse_sysml_visualization_params(&params).expect("parse visualization params");
        assert_eq!(workspace_root_uri.as_str(), "file:///c:/demo");
        assert_eq!(view, "interconnection-view");
        assert_eq!(selected_view.as_deref(), Some("Demo::VehicleConnections"));
    }

    #[test]
    fn workspace_activity_diagrams_include_performer_contexts_and_support_package_filtering() {
        let uri_a = Url::parse("file:///C:/demo/Logical.sysml").expect("uri a");
        let uri_b = Url::parse("file:///C:/demo/Function.sysml").expect("uri b");
        let parsed_a = parse(
            r#"
                package LogicalComponentsPackage {
                    part def LaunchSystem {
                        perform action provideStage1Thrust : ProvideStage1Thrust;
                        perform action provideStage2Thrust : ProvideStage2Thrust;
                    }
                }
            "#,
        )
        .expect("parse logical");
        let parsed_b = parse(
            r#"
                package FunctionsPackage {
                    action def LaunchToOrbit {
                        action countdown: ExecuteTerminalCountdown;
                        action provideThrust1: ProvideStage1Thrust;
                    }
                }
            "#,
        )
        .expect("parse function");

        let index = HashMap::from([
            (
                uri_a.clone(),
                IndexEntry {
                    content: String::new(),
                    parsed: Some(parsed_a),
                    parse_metadata: ParseMetadata::default(),
                },
            ),
            (
                uri_b.clone(),
                IndexEntry {
                    content: String::new(),
                    parsed: Some(parsed_b),
                    parse_metadata: ParseMetadata::default(),
                },
            ),
        ]);

        let all_diagrams =
            build_workspace_activity_diagrams(&index, &[uri_a.clone(), uri_b.clone()], None);
        assert!(
            all_diagrams
                .iter()
                .any(|diagram| diagram.name == "LaunchSystem" && diagram.source_kind == "performer"),
            "expected performer diagram to be aggregated from workspace files"
        );
        assert!(
            all_diagrams
                .iter()
                .any(|diagram| diagram.name == "LaunchToOrbit" && diagram.source_kind == "actionDef"),
            "expected action-def diagram to be aggregated from workspace files"
        );

        let filtered_diagrams = build_workspace_activity_diagrams(
            &index,
            &[uri_a, uri_b],
            Some(("LogicalComponentsPackage", Some("LogicalComponentsPackage"))),
        );
        assert_eq!(filtered_diagrams.len(), 1);
        assert_eq!(filtered_diagrams[0].name, "LaunchSystem");
        assert_eq!(filtered_diagrams[0].source_kind, "performer");
    }

    #[test]
    fn package_groups_are_built_from_contains_hierarchy() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "P".to_string(),
                    element_type: "package".to_string(),
                    name: "P".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "P::Inner".to_string(),
                    element_type: "package".to_string(),
                    name: "Inner".to_string(),
                    uri: None,
                    parent_id: Some("P".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "P::Inner::x".to_string(),
                    element_type: "part".to_string(),
                    name: "x".to_string(),
                    uri: None,
                    parent_id: Some("P::Inner".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "P".to_string(),
                    target: "P::Inner".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "P::Inner".to_string(),
                    target: "P::Inner::x".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
            ],
        };

        let groups = build_package_groups_from_graph(&graph);
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().any(|group| group.id == "P"));
        assert!(groups
            .iter()
            .any(|group| group.id == "P::Inner" && group.parent_id.as_deref() == Some("P")));
        assert!(groups
            .iter()
            .any(|group| group.node_ids.iter().any(|node_id| node_id == "P::Inner::x")));
    }

    #[test]
    fn ibd_package_container_groups_follow_package_membership() {
        let parts = vec![
            IbdPartDto {
                id: "Drone::Vehicle".to_string(),
                name: "Vehicle".to_string(),
                qualified_name: "Drone.Vehicle".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            },
            IbdPartDto {
                id: "Timer::TimerSystem".to_string(),
                name: "TimerSystem".to_string(),
                qualified_name: "Timer.TimerSystem".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            },
        ];
        let groups = build_ibd_package_container_groups(
            &parts,
            &[
                crate::views::dto::SysmlVisualizationPackageCandidateDto {
                    id: "Drone".to_string(),
                    name: "Drone".to_string(),
                },
                crate::views::dto::SysmlVisualizationPackageCandidateDto {
                    id: "Timer".to_string(),
                    name: "Timer".to_string(),
                },
            ],
            None,
        );
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().any(|group| {
            group.id == "package:Drone"
                && group.member_part_ids == vec!["Drone::Vehicle".to_string()]
        }));
        assert!(groups.iter().any(|group| {
            group.id == "package:Timer"
                && group.member_part_ids == vec!["Timer::TimerSystem".to_string()]
        }));
    }

    #[test]
    fn attach_ibd_package_container_groups_populates_root_views_for_selected_package() {
        let payload = IbdDataDto {
            parts: vec![IbdPartDto {
                id: "Drone::Vehicle".to_string(),
                name: "Vehicle".to_string(),
                qualified_name: "Drone.Vehicle".to_string(),
                uri: None,
                container_id: None,
                element_type: "part def".to_string(),
                attributes: HashMap::new(),
            }],
            ports: Vec::new(),
            connectors: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: vec!["Vehicle".to_string()],
            default_root: Some("Vehicle".to_string()),
            root_views: HashMap::from([(
                "Vehicle".to_string(),
                IbdRootViewDto {
                    parts: vec![IbdPartDto {
                        id: "Drone::Vehicle".to_string(),
                        name: "Vehicle".to_string(),
                        qualified_name: "Drone.Vehicle".to_string(),
                        uri: None,
                        container_id: None,
                        element_type: "part def".to_string(),
                        attributes: HashMap::new(),
                    }],
                    ports: Vec::new(),
                    connectors: Vec::new(),
                    container_groups: Vec::new(),
                    package_container_groups: Vec::new(),
                },
            )]),
        };

        let attached = attach_ibd_package_container_groups(
            payload,
            &[crate::views::dto::SysmlVisualizationPackageCandidateDto {
                id: "Drone".to_string(),
                name: "Drone".to_string(),
            }],
            Some(("Drone", Some("Drone"))),
        );

        assert_eq!(attached.package_container_groups.len(), 1);
        assert_eq!(attached.package_container_groups[0].id, "package:Drone");
        assert_eq!(
            attached
                .root_views
                .get("Vehicle")
                .expect("root view")
                .package_container_groups
                .len(),
            1
        );
    }
}
