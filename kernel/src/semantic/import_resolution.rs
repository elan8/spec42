use std::collections::HashSet;

use crate::graph::SemanticGraph;
use crate::model::{NodeId, SemanticNode};
use crate::relationships::{normalize_for_lookup, type_ref_candidates_with_kind};

const TYPE_DISAMBIGUATION_SUFFIX_KINDS: &[&str] = &[
    "part_def",
    "port_def",
    "action_def",
    "state_def",
    "flow_def",
    "allocation_def",
    "requirement_def",
    "use_case_def",
    "attribute_def",
    "enum_def",
    "item_def",
    "actor_def",
    "occurrence_def",
    "interface",
    "concern_def",
    "alias",
    "kermlDecl",
];

fn is_namespace_kind(kind: &str) -> bool {
    matches!(
        kind,
        "package"
            | "requirement def"
            | "requirement"
            | "use case def"
            | "use case"
            | "analysis def"
            | "analysis"
            | "verification def"
            | "verification"
            | "concern def"
            | "concern"
    )
}

fn normalize_declared_type_ref(type_ref: &str) -> String {
    type_ref
        .trim()
        .strip_prefix('~')
        .map(str::trim)
        .unwrap_or(type_ref.trim())
        .to_string()
}

fn element_kind_allowed(element_kind: &str, allowed_kinds: &[&str]) -> bool {
    allowed_kinds.contains(&element_kind)
}

fn import_visibility(import: &SemanticNode) -> String {
    import
        .attributes
        .get("visibility")
        .and_then(|value| value.as_str())
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "private".to_string())
}

fn import_target(import: &SemanticNode) -> Option<&str> {
    import
        .attributes
        .get("importTarget")
        .and_then(|value| value.as_str())
}

fn is_import_all(import: &SemanticNode) -> bool {
    import
        .attributes
        .get("importAll")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn is_recursive(import: &SemanticNode) -> bool {
    import
        .attributes
        .get("recursive")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn normalized_namespace_target(target: &str) -> String {
    target
        .trim()
        .trim_end_matches("::**")
        .trim_end_matches("::*")
        .trim()
        .to_string()
}

fn normalized_membership_target(target: &str) -> String {
    target.trim().trim_end_matches("::**").trim().to_string()
}

fn dedupe_node_ids(ids: Vec<NodeId>) -> Vec<NodeId> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for id in ids {
        if seen.insert(id.clone()) {
            out.push(id);
        }
    }
    out
}

fn unique_graph_wide_named_members(
    graph: &SemanticGraph,
    simple_name: &str,
    allowed_kinds: &[&str],
) -> Vec<NodeId> {
    let matches: Vec<NodeId> = graph
        .nodes_by_uri
        .values()
        .flatten()
        .filter_map(|id| {
            let node = graph.get_node(id)?;
            (node.name == simple_name && element_kind_allowed(&node.element_kind, allowed_kinds))
                .then(|| id.clone())
        })
        .collect();

    let qualified_names: HashSet<String> =
        matches.iter().map(|id| id.qualified_name.clone()).collect();
    if qualified_names.len() == 1 {
        matches
    } else {
        Vec::new()
    }
}

fn namespace_scope_chain(graph: &SemanticGraph, context_node: &SemanticNode) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut current = Some(context_node.id.clone());
    while let Some(node_id) = current {
        let Some(node) = graph.get_node(&node_id) else {
            break;
        };
        if is_namespace_kind(&node.element_kind) {
            out.push(node.id.clone());
        }
        current = node.parent_id.clone();
    }
    out
}

fn has_any_import_in_scope(graph: &SemanticGraph, context_node: &SemanticNode) -> bool {
    for namespace_id in namespace_scope_chain(graph, context_node) {
        if let Some(namespace) = graph.get_node(&namespace_id) {
            if graph
                .children_of(namespace)
                .into_iter()
                .any(|child| child.element_kind == "import")
            {
                return true;
            }
        }
    }
    false
}

fn namespace_node_ids_for_qualified_name(
    graph: &SemanticGraph,
    qualified_name: &str,
) -> Vec<NodeId> {
    graph
        .node_ids_for_qualified_name(&normalize_for_lookup(qualified_name))
        .into_iter()
        .flatten()
        .filter(|id| {
            graph
                .get_node(id)
                .map(|node| is_namespace_kind(&node.element_kind))
                .unwrap_or(false)
        })
        .cloned()
        .collect()
}

fn owned_namespace_children(graph: &SemanticGraph, namespace_id: &NodeId) -> Vec<NodeId> {
    let Some(namespace) = graph.get_node(namespace_id) else {
        return Vec::new();
    };
    graph
        .children_of(namespace)
        .into_iter()
        .filter(|child| child.element_kind != "import" && is_namespace_kind(&child.element_kind))
        .map(|child| child.id.clone())
        .collect()
}

fn exact_named_members(graph: &SemanticGraph, qualified_name: &str) -> Vec<NodeId> {
    graph
        .node_ids_for_qualified_name(&normalize_for_lookup(qualified_name))
        .into_iter()
        .flatten()
        .filter(|id| {
            graph
                .get_node(id)
                .map(|node| node.element_kind != "import")
                .unwrap_or(false)
        })
        .cloned()
        .collect()
}

fn allowed_exact_named_members(
    graph: &SemanticGraph,
    qualified_name: &str,
    allowed_kinds: &[&str],
) -> Vec<NodeId> {
    exact_named_members(graph, qualified_name)
        .into_iter()
        .filter(|id| {
            graph
                .get_node(id)
                .map(|node| element_kind_allowed(&node.element_kind, allowed_kinds))
                .unwrap_or(false)
        })
        .collect()
}

fn exported_members_named_from_namespace(
    graph: &SemanticGraph,
    namespace_id: &NodeId,
    simple_name: &str,
    exported_only: bool,
    stack: &mut HashSet<(NodeId, String, bool)>,
) -> Vec<NodeId> {
    let cache_key = (namespace_id.clone(), simple_name.to_string(), exported_only);
    if let Ok(cache) = graph.import_lookup_cache.lock() {
        if let Some(cached) = cache.get(&cache_key) {
            return cached.clone();
        }
    }
    if !stack.insert(cache_key.clone()) {
        return Vec::new();
    }

    let mut out = Vec::new();
    let Some(namespace) = graph.get_node(namespace_id) else {
        stack.remove(&cache_key);
        return Vec::new();
    };

    for child in graph.children_of(namespace) {
        if child.element_kind != "import" && child.name == simple_name {
            out.push(child.id.clone());
        }
    }

    for import in graph
        .children_of(namespace)
        .into_iter()
        .filter(|child| child.element_kind == "import")
    {
        if exported_only && import_visibility(import) != "public" {
            continue;
        }
        out.extend(resolve_import_targets_named(
            graph,
            import,
            simple_name,
            stack,
        ));
    }

    let out = dedupe_node_ids(out);
    stack.remove(&cache_key);
    if let Ok(mut cache) = graph.import_lookup_cache.lock() {
        cache.insert(cache_key, out.clone());
    }
    out
}

fn collect_recursive_namespace_members(
    graph: &SemanticGraph,
    namespace_id: &NodeId,
    simple_name: &str,
    stack: &mut HashSet<(NodeId, String, bool)>,
) -> Vec<NodeId> {
    let mut out =
        exported_members_named_from_namespace(graph, namespace_id, simple_name, true, stack);
    for child_namespace in owned_namespace_children(graph, namespace_id) {
        out.extend(collect_recursive_namespace_members(
            graph,
            &child_namespace,
            simple_name,
            stack,
        ));
    }
    dedupe_node_ids(out)
}

fn resolve_membership_import_named(
    graph: &SemanticGraph,
    target: &str,
    recursive: bool,
    simple_name: &str,
    stack: &mut HashSet<(NodeId, String, bool)>,
) -> Vec<NodeId> {
    let normalized_target = normalized_membership_target(target);
    let mut out = Vec::new();

    if let Some((parent_qualified, member_name)) = normalized_target.rsplit_once("::") {
        if member_name == simple_name {
            out.extend(exact_named_members(graph, &normalized_target));
            for namespace_id in namespace_node_ids_for_qualified_name(graph, parent_qualified) {
                out.extend(exported_members_named_from_namespace(
                    graph,
                    &namespace_id,
                    member_name,
                    true,
                    stack,
                ));
            }
        }
    } else if normalized_target == simple_name {
        out.extend(exact_named_members(graph, &normalized_target));
    }

    if recursive {
        for namespace_id in namespace_node_ids_for_qualified_name(graph, &normalized_target) {
            out.extend(collect_recursive_namespace_members(
                graph,
                &namespace_id,
                simple_name,
                stack,
            ));
        }
    }

    dedupe_node_ids(out)
}

fn resolve_namespace_import_named(
    graph: &SemanticGraph,
    target: &str,
    recursive: bool,
    simple_name: &str,
    stack: &mut HashSet<(NodeId, String, bool)>,
) -> Vec<NodeId> {
    let normalized_target = normalized_namespace_target(target);
    let mut out = Vec::new();
    for namespace_id in namespace_node_ids_for_qualified_name(graph, &normalized_target) {
        if recursive {
            out.extend(collect_recursive_namespace_members(
                graph,
                &namespace_id,
                simple_name,
                stack,
            ));
        } else {
            out.extend(exported_members_named_from_namespace(
                graph,
                &namespace_id,
                simple_name,
                true,
                stack,
            ));
        }
    }
    dedupe_node_ids(out)
}

fn resolve_import_targets_named(
    graph: &SemanticGraph,
    import: &SemanticNode,
    simple_name: &str,
    stack: &mut HashSet<(NodeId, String, bool)>,
) -> Vec<NodeId> {
    let Some(target) = import_target(import) else {
        return Vec::new();
    };
    if is_import_all(import) {
        resolve_namespace_import_named(graph, target, is_recursive(import), simple_name, stack)
    } else {
        resolve_membership_import_named(graph, target, is_recursive(import), simple_name, stack)
    }
}

pub fn resolve_imported_node_ids_for_simple_name(
    graph: &SemanticGraph,
    context_node: &SemanticNode,
    simple_name: &str,
) -> Vec<NodeId> {
    let mut out = Vec::new();
    let mut stack = HashSet::new();
    for namespace_id in namespace_scope_chain(graph, context_node) {
        out.extend(exported_members_named_from_namespace(
            graph,
            &namespace_id,
            simple_name,
            false,
            &mut stack,
        ));
    }
    dedupe_node_ids(out)
}

pub fn resolve_type_reference_targets(
    graph: &SemanticGraph,
    context_node: &SemanticNode,
    type_ref: &str,
    allowed_kinds: &[&str],
) -> Vec<NodeId> {
    let normalized_type_ref = normalize_declared_type_ref(type_ref);
    if normalized_type_ref.is_empty() {
        return Vec::new();
    }

    let container_prefix = context_node
        .parent_id
        .as_ref()
        .and_then(|parent_id| graph.get_node(parent_id))
        .map(|parent| parent.id.qualified_name.as_str());

    let mut out = Vec::new();
    for suffix_kind in TYPE_DISAMBIGUATION_SUFFIX_KINDS {
        for candidate in
            type_ref_candidates_with_kind(container_prefix, &normalized_type_ref, suffix_kind)
        {
            if let Some(target_ids) =
                graph.node_ids_for_qualified_name(&normalize_for_lookup(&candidate))
            {
                for target_id in target_ids {
                    if let Some(target) = graph.get_node(target_id) {
                        if element_kind_allowed(&target.element_kind, allowed_kinds) {
                            out.push(target_id.clone());
                        }
                    }
                }
            }
        }
    }

    if let Some((namespace_qualified, simple_name)) = normalized_type_ref.rsplit_once("::") {
        out.extend(allowed_exact_named_members(
            graph,
            &normalized_type_ref,
            allowed_kinds,
        ));
        let mut stack = HashSet::new();
        for namespace_id in namespace_node_ids_for_qualified_name(graph, namespace_qualified) {
            out.extend(
                exported_members_named_from_namespace(
                    graph,
                    &namespace_id,
                    simple_name,
                    true,
                    &mut stack,
                )
                .into_iter()
                .filter(|id| {
                    graph
                        .get_node(id)
                        .map(|node| element_kind_allowed(&node.element_kind, allowed_kinds))
                        .unwrap_or(false)
                }),
            );
        }
    }

    if !normalized_type_ref.contains("::") {
        for imported_target in
            resolve_imported_node_ids_for_simple_name(graph, context_node, &normalized_type_ref)
        {
            if let Some(target) = graph.get_node(&imported_target) {
                if element_kind_allowed(&target.element_kind, allowed_kinds) {
                    out.push(imported_target);
                }
            }
        }

        if let Some(context_uri_nodes) = graph.nodes_by_uri.get(&context_node.id.uri) {
            let local_suffixes: Vec<String> = TYPE_DISAMBIGUATION_SUFFIX_KINDS
                .iter()
                .map(|suffix| format!("::{}#{}", normalized_type_ref, suffix))
                .collect();
            let mut local_matches: Vec<NodeId> = context_uri_nodes
                .iter()
                .filter(|node_id| {
                    node_id.qualified_name == normalized_type_ref
                        || node_id
                            .qualified_name
                            .ends_with(&format!("::{}", normalized_type_ref))
                        || local_suffixes
                            .iter()
                            .any(|suffix| node_id.qualified_name.ends_with(suffix))
                })
                .filter_map(|node_id| {
                    let node = graph.get_node(node_id)?;
                    element_kind_allowed(&node.element_kind, allowed_kinds).then(|| node_id.clone())
                })
                .collect();
            local_matches.sort_by_key(|id| id.qualified_name.len());
            out.extend(local_matches);
        }

        if out.is_empty() && !has_any_import_in_scope(graph, context_node) {
            out.extend(unique_graph_wide_named_members(
                graph,
                &normalized_type_ref,
                allowed_kinds,
            ));
        }
    }

    dedupe_node_ids(out)
}
