//! Builds semantic graph from parsed AST (packages, parts, ports, connections, etc.).

use std::collections::HashMap;

use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::{Range, Url};

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, SemanticNode};

mod expressions;
mod interface_def;
mod modeled_kerml_name;
mod package_body;
mod package_packages;
mod part_def;
mod part_usage;
mod port_def;
mod requirement_body;
mod state;
mod stubs;
mod use_case;
mod verification;

/// Builds a semantic graph from a parsed RootNamespace (sysml-v2-parser AST).
/// Adds the root package/namespace as a node and sets parent_id on its direct children
/// so that contains edges are emitted for the General View.
pub fn build_graph_from_doc(root: &RootNamespace, uri: &Url) -> SemanticGraph {
    let mut g = SemanticGraph::new();
    for node in &root.elements {
        let (elements, pkg_qualified, pkg_name_display, pkg_span) =
            match crate::root_element_body(&node.value) {
                Some(t) => t,
                None => continue,
            };
        let pkg_qualified_disambiguated = qualified_name_for_node(
            &g,
            uri,
            None,
            if pkg_name_display == "(top level)" {
                ""
            } else {
                &pkg_name_display
            },
            "package",
        );
        let pkg_qualified_final = if pkg_qualified_disambiguated.is_empty() {
            pkg_qualified.clone()
        } else {
            pkg_qualified_disambiguated
        };
        add_node_and_recurse(
            &mut g,
            uri,
            &pkg_qualified_final,
            "package",
            pkg_name_display,
            span_to_range(pkg_span),
            HashMap::new(),
            None,
        );
        let package_node_id = NodeId::new(uri, &pkg_qualified_final);
        let child_prefix = if pkg_qualified == "(top level)" || pkg_qualified.is_empty() {
            None
        } else {
            Some(pkg_qualified_final.as_str())
        };
        for el in elements {
            package_body::build_from_package_body_element(
                el,
                uri,
                child_prefix,
                Some(&package_node_id),
                root,
                &mut g,
            );
        }
    }
    crate::semantic::relationships::resolve_pending_relationships_for_uri(&mut g, uri);
    g
}

pub(super) fn qualified_name(container_prefix: Option<&str>, name: &str) -> String {
    match container_prefix {
        Some(p) if !p.is_empty() => format!("{}::{}", p, name),
        _ => name.to_string(),
    }
}

/// Returns a qualified name that is unique among siblings. When a node with the same
/// base qualified name already exists (e.g. package and part def with same name), appends
/// #kind to disambiguate.
pub(super) fn qualified_name_for_node(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    name: &str,
    kind: &str,
) -> String {
    let base = qualified_name(container_prefix, name);
    let kind_suffix = kind.replace(' ', "_");
    let mut candidate = base.clone();
    let mut ordinal = 0usize;
    loop {
        let node_id = NodeId::new(uri, &candidate);
        if !g.node_index_by_id.contains_key(&node_id) {
            return candidate;
        }
        ordinal += 1;
        candidate = if ordinal == 1 {
            format!("{}#{}", base, kind_suffix)
        } else {
            format!("{}#{}{}", base, kind_suffix, ordinal)
        };
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn add_node_and_recurse(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    kind: &str,
    name: String,
    range: Range,
    attrs: HashMap<String, serde_json::Value>,
    parent_id: Option<&NodeId>,
) {
    let node_id = NodeId::new(uri, qualified);
    let node = SemanticNode {
        id: node_id.clone(),
        element_kind: kind.to_string(),
        name,
        range,
        attributes: attrs,
        parent_id: parent_id.cloned(),
    };
    let idx = g.graph.add_node(node);
    g.node_index_by_id.insert(node_id.clone(), idx);
    g.nodes_by_uri.entry(uri.clone()).or_default().push(node_id);
    g.node_ids_by_qualified_name
        .entry(qualified.to_string())
        .or_default()
        .push(NodeId::new(uri, qualified));
}

#[allow(clippy::too_many_arguments)]
pub(super) fn add_node_if_not_exists(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    kind: &str,
    name: String,
    parent_id: &NodeId,
    source_range: Range,
    attrs: HashMap<String, serde_json::Value>,
) {
    let node_id = NodeId::new(uri, qualified);
    if g.node_index_by_id.contains_key(&node_id) {
        return;
    }
    let mut attrs = attrs;
    attrs.insert("synthetic".to_string(), serde_json::json!(true));
    attrs.insert(
        "originRange".to_string(),
        serde_json::json!({
            "start": {"line": source_range.start.line, "character": source_range.start.character},
            "end": {"line": source_range.end.line, "character": source_range.end.character}
        }),
    );
    let parent_range = g
        .get_node(parent_id)
        .map(|node| node.range)
        .unwrap_or(source_range);
    let node = SemanticNode {
        id: node_id.clone(),
        element_kind: kind.to_string(),
        name,
        range: parent_range,
        attributes: attrs,
        parent_id: Some(parent_id.clone()),
    };
    let idx = g.graph.add_node(node);
    g.node_index_by_id.insert(node_id.clone(), idx);
    g.nodes_by_uri.entry(uri.clone()).or_default().push(node_id);
    g.node_ids_by_qualified_name
        .entry(qualified.to_string())
        .or_default()
        .push(NodeId::new(uri, qualified));
}
