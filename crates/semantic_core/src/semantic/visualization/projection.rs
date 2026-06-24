//! View projection, package groups, and workspace model shaping.

use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::dto::{
    range_to_dto, GraphEdgeDto, GraphNodeDto, RelationshipDto,
    SysmlElementDto, SysmlGraphDto, SysmlVisualizationGroupDto, SysmlVisualizationPackageCandidateDto,
    WorkspaceFileModelDto, WorkspaceModelDto, WorkspaceModelSummaryDto,
};
use crate::semantic::extracted_model::{
    extract_activity_diagrams, ActivityDiagramDto,
};
use crate::semantic::ibd::{IbdDataDto, IbdPackageContainerGroupDto, IbdPartDto};
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use crate::SemanticGraph;

pub fn build_workspace_graph_dto_for_uris(
    semantic_graph: &SemanticGraph,
    workspace_uris: &[Url],
) -> SysmlGraphDto {
    let workspace_uri_set: HashSet<Url> = workspace_uris.iter().cloned().collect();
    let mut nodes = Vec::new();
    let mut node_ids = HashSet::new();
    for workspace_uri in workspace_uris {
        for node in semantic_graph
            .nodes_for_uri(workspace_uri)
            .into_iter()
            .filter(|n| n.element_kind != "diagnostic")
        {
            node_ids.insert(node.id.qualified_name.clone());
            nodes.push(GraphNodeDto {
                id: node.id.qualified_name.clone(),
                element_type: node.element_kind.clone(),
                name: node.name.clone(),
                uri: Some(node.id.uri.as_str().to_string()),
                parent_id: node
                    .parent_id
                    .as_ref()
                    .map(|parent| parent.qualified_name.clone()),
                range: range_to_dto(node.range),
                attributes: node.attributes.clone(),
            });
        }
    }

    let mut edge_keys = HashSet::new();
    let mut edges = Vec::new();
    for workspace_uri in workspace_uris {
        for (source, target, kind, name) in semantic_graph.edges_for_uri_as_strings(workspace_uri) {
            let key = (
                source.clone(),
                target.clone(),
                kind.as_str().to_string(),
                name.clone(),
            );
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

pub(crate) fn project_graph_by_ids(graph: &SysmlGraphDto, visible_ids: &HashSet<String>) -> SysmlGraphDto {
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

// --- Activity diagrams (kernel `visualization/activity_views.rs`) ---

pub(crate) fn normalize_package_path(value: &str) -> String {
    value.replace('.', "::").trim().to_string()
}

pub(crate) fn diagram_matches_package_filter(
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

pub fn build_workspace_activity_diagrams(
    documents: &[WorkspaceParsedDocument],
    workspace_uris: &[Url],
    package_filter: Option<(&str, Option<&str>)>,
) -> Vec<ActivityDiagramDto> {
    let mut diagrams = Vec::new();
    for workspace_uri in workspace_uris {
        let Some(doc) = documents.iter().find(|d| &d.uri == workspace_uri) else {
            continue;
        };
        let source_uri = workspace_uri.as_str().to_string();
        let mut extracted = extract_activity_diagrams(&doc.parsed);
        for diagram in &mut extracted {
            if diagram.uri.is_none() {
                diagram.uri = Some(source_uri.clone());
            }
            for action in &mut diagram.actions {
                if action.uri.is_none() {
                    action.uri = Some(source_uri.clone());
                }
            }
        }
        diagrams.extend(extracted);
    }

    if let Some((package_ref, package_name)) = package_filter {
        diagrams
            .retain(|diagram| diagram_matches_package_filter(diagram, package_ref, package_name));
    }

    diagrams
}

pub fn top_level_package_for_node_id(node_id: &str) -> String {
    normalize_package_path(node_id)
        .split("::")
        .next()
        .unwrap_or("")
        .to_string()
}

pub fn filter_activity_diagrams_by_graph(
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
pub fn build_package_groups_from_graph(graph: &SysmlGraphDto) -> Vec<SysmlVisualizationGroupDto> {
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
        if package_ids.contains(edge.source.as_str()) && package_ids.contains(edge.target.as_str())
        {
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
        let node_ids: Vec<String> =
            collect_non_package_descendants(package_id, &package_ids, &children_by_parent)
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

// --- Workspace model + IBD package groups (kernel `visualization.rs`) ---

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

pub(crate) fn graph_to_element_tree(graph: &SysmlGraphDto, uri: &Url) -> Vec<SysmlElementDto> {
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
    let mut outgoing_relationships: HashMap<&str, Vec<RelationshipDto>> = HashMap::new();

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
                .push(RelationshipDto {
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
        outgoing_relationships: &HashMap<&str, Vec<RelationshipDto>>,
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
            uri: Some(node.uri.clone().unwrap_or_else(|| uri.as_str().to_string())),
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

pub(crate) fn build_workspace_model_dto_from_graph(
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
                    graph.nodes.iter().any(|node| {
                        node.id == edge.source
                            && node.uri.as_deref() == Some(workspace_uri.as_str())
                    }) || graph.nodes.iter().any(|node| {
                        node.id == edge.target
                            && node.uri.as_deref() == Some(workspace_uri.as_str())
                    })
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

pub(crate) fn merge_namespace_elements(elements: &[SysmlElementDto]) -> Vec<SysmlElementDto> {
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

pub(crate) fn collect_package_candidates(
    elements: &[SysmlElementDto],
    seen: &mut HashSet<String>,
    out: &mut Vec<SysmlVisualizationPackageCandidateDto>,
) {
    for element in elements {
        if element.element_type.to_lowercase().contains("package") {
            let id = element.id.clone().unwrap_or_else(|| element.name.clone());
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

pub(crate) fn renderer_empty_state_message(view: &str) -> String {
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
        "sequence-view" => {
            "Define a SysML view typed by SequenceView to display a software interaction sequence in this visualizer panel."
                .to_string()
        }
        "state-transition-view" => {
            "Define a SysML view typed by StateTransitionView to display something in this visualizer panel."
                .to_string()
        }
        _ => "Define a SysML view to display something in this visualizer panel.".to_string(),
    }
}

pub(crate) fn no_defined_views_message() -> String {
    "Define a SysML view with expose (and optional filter) to use the visualizer.".to_string()
}

pub(crate) fn unsupported_view_type_message(view_type: Option<&str>) -> String {
    match view_type.filter(|value| !value.trim().is_empty()) {
        Some(view_type) => {
            format!("This SysML view is typed by {view_type}, which Spec42 does not support yet.")
        }
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

fn within_package_prefix(value: &str, package_prefix: &str, dot_prefix: &str) -> bool {
    value == package_prefix
        || value.starts_with(&format!("{package_prefix}::"))
        || value == dot_prefix
        || value.starts_with(&format!("{dot_prefix}."))
}

pub fn build_ibd_package_container_groups(
    parts: &[IbdPartDto],
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
    let candidate_ids: Vec<&str> = selected_candidates
        .iter()
        .map(|candidate| candidate.id.as_str())
        .collect();

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

pub fn attach_ibd_package_container_groups(
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

pub(crate) fn workspace_parsed_documents_for_uris(
    documents: &[WorkspaceParsedDocument],
    workspace_uris: &[Url],
) -> Vec<WorkspaceParsedDocument> {
    let set: HashSet<Url> = workspace_uris.iter().cloned().collect();
    documents
        .iter()
        .filter(|d| set.contains(&d.uri))
        .cloned()
        .collect()
}
