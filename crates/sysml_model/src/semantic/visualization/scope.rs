//! IBD build scoping for incremental visualization.

use std::collections::HashSet;

use url::Url;

use crate::SemanticGraph;

/// Whether a document URI lies under the workspace root URI.
pub fn uri_under_root(uri: &Url, workspace_root_uri: &Url) -> bool {
    match (uri.to_file_path(), workspace_root_uri.to_file_path()) {
        (Ok(uri_path), Ok(root_path)) => uri_path.starts_with(root_path),
        _ => {
            let root = workspace_root_uri.as_str().trim_end_matches('/');
            uri.as_str() == root || uri.as_str().starts_with(&format!("{root}/"))
        }
    }
}

/// Workspace document URIs under `workspace_root_uri`, excluding library paths.
pub fn workspace_uris_for_root(
    semantic_graph: &SemanticGraph,
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

/// Whether workspace visualization artifacts include a full merged IBD cache entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IbdArtifactMode {
    /// Build and cache merged IBD for all workspace URIs (Model Explorer path).
    #[default]
    FullWorkspace,
    /// Skip full-workspace IBD; scoped IBD is built on demand per visualization request.
    Deferred,
}

/// Controls how many workspace URIs participate in merged IBD construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IbdBuildScope {
    /// Build IBD for every workspace URI (Model Explorer,8 full cache).
    #[default]
    FullWorkspace,
    /// Build IBD only for URIs that contain nodes exposed by the selected view.
    ViewExposedPackages,
}

/// Follows containment (children) and typing/specializes edges transitively from `node`,
/// collecting every document URI touched. An exposed usage's nested structure and connectors
/// are frequently declared on its *type definition* (possibly in another document) rather than
/// on the usage itself, so the exposed-id qualified-name prefix alone isn't enough to find every
/// document needed to reconstruct the interconnection view (e.g. connectors mirrored from a
/// part def in a sibling file would otherwise be silently excluded from the scoped build).
fn collect_definition_uris_for_subtree(
    semantic_graph: &SemanticGraph,
    node: &crate::SemanticNode,
    uris: &mut HashSet<Url>,
    visited: &mut HashSet<String>,
) {
    if !visited.insert(node.id.qualified_name.clone()) {
        return;
    }
    uris.insert(node.id.uri.clone());
    for def_node in semantic_graph.outgoing_typing_or_specializes_targets(node) {
        collect_definition_uris_for_subtree(semantic_graph, def_node, uris, visited);
    }
    for child in semantic_graph.children_of(node) {
        collect_definition_uris_for_subtree(semantic_graph, child, uris, visited);
    }
}

/// Collect workspace URIs whose semantic nodes fall under `exposed_ids`.
pub fn ibd_uri_closure_for_exposed_ids(
    semantic_graph: &SemanticGraph,
    exposed_ids: &HashSet<String>,
) -> Vec<Url> {
    if exposed_ids.is_empty() {
        return Vec::new();
    }

    let mut uris = HashSet::new();
    let mut visited = HashSet::new();
    for exposed_id in exposed_ids {
        if let Some(node_ids) = semantic_graph.node_ids_by_qualified_name.get(exposed_id) {
            for node_id in node_ids {
                uris.insert(node_id.uri.clone());
                if let Some(node) = semantic_graph.get_node(node_id) {
                    collect_definition_uris_for_subtree(
                        semantic_graph,
                        node,
                        &mut uris,
                        &mut visited,
                    );
                }
            }
        }
        let dot_prefix = format!("{}.", exposed_id.replace("::", "."));
        let colon_prefix = format!("{exposed_id}::");
        for (qualified_name, node_ids) in &semantic_graph.node_ids_by_qualified_name {
            if qualified_name.starts_with(&colon_prefix)
                || qualified_name.starts_with(&dot_prefix)
                || qualified_name == exposed_id
            {
                for node_id in node_ids {
                    uris.insert(node_id.uri.clone());
                }
            }
        }
    }

    let mut ordered: Vec<Url> = uris.into_iter().collect();
    ordered.sort_by(|left, right| left.as_str().cmp(right.as_str()));
    ordered
}

/// Intersect workspace URIs with an optional exposed-id closure.
pub fn workspace_uris_for_ibd_scope(
    workspace_uris: &[Url],
    semantic_graph: &SemanticGraph,
    scope: IbdBuildScope,
    exposed_ids: &HashSet<String>,
) -> Vec<Url> {
    match scope {
        IbdBuildScope::FullWorkspace => workspace_uris.to_vec(),
        IbdBuildScope::ViewExposedPackages => {
            let scoped = ibd_uri_closure_for_exposed_ids(semantic_graph, exposed_ids);
            if scoped.is_empty() {
                return workspace_uris.to_vec();
            }
            let scoped_set: HashSet<Url> = scoped.into_iter().collect();
            workspace_uris
                .iter()
                .filter(|uri| scoped_set.contains(*uri))
                .cloned()
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

    fn doc(path: &str, content: &str) -> SysmlDocument {
        SysmlDocument::from_memory_path(
            "workspace",
            path,
            content.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("doc")
    }

    #[test]
    fn ibd_uri_closure_collects_document_for_exposed_package_member() {
        let content = r#"package P {
    part def Engine;
    part engine : Engine;
}"#;
        let document = doc("Engine.sysml", content);
        let uri = document.uri.clone();
        let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");
        let mut exposed = HashSet::new();
        exposed.insert("P".to_string());
        let closure = ibd_uri_closure_for_exposed_ids(&graph, &exposed);
        assert!(closure.iter().any(|candidate| candidate == &uri));
    }
}
