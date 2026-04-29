use std::collections::{HashMap, HashSet};

use crate::views::dto::{GraphNodeDto, SysmlGraphDto, SysmlVisualizationGroupDto};

pub(super) fn build_package_groups_from_graph(
    graph: &SysmlGraphDto,
) -> Vec<SysmlVisualizationGroupDto> {
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
