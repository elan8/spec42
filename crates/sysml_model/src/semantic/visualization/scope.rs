//! IBD build scoping for incremental visualization.

use std::collections::HashSet;

use url::Url;

use crate::semantic::reference_resolution::{resolve_inherited_member_via_type, ResolveResult};
use crate::SemanticGraph;

/// Resolve a `redefines`/`subsetsFeature` attribute value to the target node(s) it
/// redefines/subsets. The value is a name that's either:
/// - A qualified path to an *unrelated* feature (`subsets` doesn't require any typing/specializes
///   relationship between the two owners) — resolved via a direct qualified-name lookup.
/// - A bare name resolved relative to `owner`'s own typing/specialization chain (the common
///   `redefines` case, matching a member inherited from a supertype) — mirroring how the
///   `unresolved_redefines_target`/`incompatible_subset_redefine_kind` diagnostics already resolve
///   the same attribute (`diagnostics/checks/kind_compatibility.rs`).
/// Both are tried since either shape is legal SysML and the attribute alone doesn't say which.
fn resolve_redefines_or_subsets_targets<'a>(
    semantic_graph: &'a SemanticGraph,
    owner: &crate::SemanticNode,
    attribute_value: &str,
) -> Vec<&'a crate::SemanticNode> {
    let qualified = attribute_value.replace('.', "::");
    if let Some(node_ids) = semantic_graph.node_ids_by_qualified_name.get(&qualified) {
        let targets: Vec<&crate::SemanticNode> = node_ids
            .iter()
            .filter_map(|id| semantic_graph.get_node(id))
            .collect();
        if !targets.is_empty() {
            return targets;
        }
    }
    let member = attribute_value
        .split("::")
        .last()
        .unwrap_or(attribute_value);
    match resolve_inherited_member_via_type(semantic_graph, owner, member) {
        ResolveResult::Resolved(target_id) => {
            semantic_graph.get_node(&target_id).into_iter().collect()
        }
        _ => Vec::new(),
    }
}

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

/// Follows containment (children), typing/specializes edges, and `redefines`/`subsets` targets
/// transitively from `node`, collecting every document URI touched. An exposed usage's nested
/// structure and connectors are frequently declared on its *type definition* (possibly in another
/// document) rather than on the usage itself, so the exposed-id qualified-name prefix alone isn't
/// enough to find every document needed to reconstruct the interconnection view (e.g. connectors
/// mirrored from a part def in a sibling file would otherwise be silently excluded from the scoped
/// build). Likewise, a `redefines`/`subsets` target can live in yet another file whose connectors
/// are needed for correct mirroring — without following it, the scoped build can end up resolving
/// against a different (but individually valid) root than the full-workspace build would.
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
    if let Some(owner_id) = node.parent_id.as_ref() {
        if let Some(owner) = semantic_graph.get_node(owner_id) {
            for attribute_key in ["redefines", "subsetsFeature"] {
                let Some(attribute_value) = node
                    .attributes
                    .get(attribute_key)
                    .and_then(|value| value.as_str())
                else {
                    continue;
                };
                for target in
                    resolve_redefines_or_subsets_targets(semantic_graph, owner, attribute_value)
                {
                    collect_definition_uris_for_subtree(semantic_graph, target, uris, visited);
                }
            }
        }
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

    #[test]
    fn ibd_uri_closure_follows_subsets_target_into_a_sibling_document() {
        // Regression test for O-1: a `subsets` target can reference a completely unrelated part
        // (not necessarily a supertype/base-type member reachable via typing/specializes), so the
        // closure walk must resolve it explicitly rather than relying on the typing-chain walk to
        // happen to pass through the same file.
        let base_doc = doc(
            "Base.sysml",
            r#"package Base {
    part def Rig {
        part sensor;
    }
}"#,
        );
        let base_uri = base_doc.uri.clone();
        let exposed_doc = doc(
            "Exposed.sysml",
            r#"package Exposed {
    part def Vehicle {
        part sensorSubset subsets Base::Rig::sensor;
    }
}"#,
        );
        let (graph, _) =
            build_semantic_graph_from_documents(&[base_doc, exposed_doc]).expect("graph");
        let mut exposed = HashSet::new();
        exposed.insert("Exposed::Vehicle".to_string());
        let closure = ibd_uri_closure_for_exposed_ids(&graph, &exposed);
        assert!(
            closure.iter().any(|candidate| candidate == &base_uri),
            "expected the subsets target's document to be included in the closure, got {:?}",
            closure
        );
    }
}
