//! Generic part/port expansion for semantic queries, analyses, and views.
//!
//! This module provides domain-neutral expansion of SysML part hierarchies.
//! It is the shared foundation used by the IBD pipeline, Model Explorer, and
//! external consumers (e.g. Babel42 component queries).
//!
//! Unlike the IBD pipeline, output types here carry no rendering hints
//! (no `port_side`, no dot-notation ID remapping, no connector remapping).

use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::ibd::is_part_like;
use crate::semantic::model::{NodeId, SemanticNode};

// Re-export the str-based port predicate for consumers of this module.
pub use crate::semantic::ibd::is_port_like;

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

/// A recursively expanded part instance — domain neutral, no rendering hints.
#[derive(Debug, Clone)]
pub struct ExpandedPart {
    pub node_id: NodeId,
    pub name: String,
    pub element_kind: String,
    /// Dot-notation instance path (e.g. `"Vehicle.engine"` or `"Vehicle.engine.piston"`).
    pub path: String,
    pub parent_path: Option<String>,
    pub ports: Vec<ExpandedPort>,
    pub children: Vec<ExpandedPart>,
    pub attributes: HashMap<String, serde_json::Value>,
    pub uri: Option<Url>,
}

/// A port resolved from a part definition — domain neutral, no rendering hints.
#[derive(Debug, Clone)]
pub struct ExpandedPort {
    pub node_id: NodeId,
    pub name: String,
    pub direction: Option<String>,
    pub port_type: Option<String>,
    /// Dot-notation path of the owning part.
    pub parent_path: String,
}

/// Source location of a node, for client-side "open source" navigation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedSourceLocation {
    pub uri: String,
    pub line: u32,
}

/// Canonical reference to the definition a usage-like node is typed/specialized by.
///
/// See `typed_by_reference` — resolved via `typing`/`specializes` edges, independent
/// of whether the target has any materialized shape.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedByRef {
    pub id: String,
    pub qualified_name: String,
    pub name: String,
    pub kind: String,
    pub source: ResolvedSourceLocation,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Returns `true` when the part definition has any directly owned or inherited
/// parts or ports (i.e. it has "materialized shape" worth expanding).
///
/// Uses the graph's workspace-level shape cache when available.
pub fn has_materialized_shape(graph: &SemanticGraph, def_node: &SemanticNode) -> bool {
    if let Some(cached) = graph.get_cached_shape(def_node) {
        return cached;
    }
    let mut visiting = HashSet::new();
    compute_has_materialized_shape(graph, def_node, &mut visiting)
}

/// Returns ports inherited from the part definition's generalization hierarchy,
/// combined with directly-owned ports, deduplicating by `(parent_path, name)`.
///
/// Cycle-safe via internal visited set.
pub fn inherited_ports(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    parent_path: &str,
) -> Vec<ExpandedPort> {
    let mut out = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut visiting = HashSet::new();
    collect_inherited_ports(
        graph,
        def_node,
        parent_path,
        &mut out,
        &mut seen,
        &mut visiting,
    );
    out
}

/// Recursively expands a PartDef into its full part/port structure.
///
/// Returns a flat list of [`ExpandedPart`] at all depths, each carrying its own
/// `children` list (for tree traversal) and a flat `ports` list.
///
/// - `parent_path`: dot-notation path of the owning instance (prefix for children).
/// - `max_depth`: `None` = unlimited. `Some(0)` = no children, just the def's own ports.
pub fn expand_part_definition(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    parent_path: &str,
    max_depth: Option<usize>,
) -> Vec<ExpandedPart> {
    let mut out = Vec::new();
    let mut visiting_defs = HashSet::new();
    let mut existing_paths = HashSet::new();
    expand_def_subtree(
        graph,
        def_node,
        parent_path,
        max_depth,
        0,
        &mut out,
        &mut visiting_defs,
        &mut existing_paths,
    );
    out
}

/// Finds the typed PartDef for a PartUsage and expands it.
///
/// Returns `None` when the usage has no typed definition with materialized shape.
pub fn expand_part_usage(
    graph: &SemanticGraph,
    usage_node: &SemanticNode,
    max_depth: Option<usize>,
) -> Option<Vec<ExpandedPart>> {
    let def_node = first_typed_definition_with_shape(graph, usage_node)?;
    let parent_path = usage_node.name.as_str();
    Some(expand_part_definition(
        graph,
        def_node,
        parent_path,
        max_depth,
    ))
}

/// Resolves the canonical type/specialization target for a usage-like node.
///
/// Unlike [`first_typed_definition_with_shape`], this does not require the
/// target to have materialized shape or be part-like: `typedBy` should point
/// at whatever the usage is typed by (a `part def`, `port def`, `item def`,
/// `attribute def`, `action def`, ...) even if that definition has no members.
pub fn typed_by_reference(graph: &SemanticGraph, usage_node: &SemanticNode) -> Option<TypedByRef> {
    first_typing_or_specializes_target(graph, usage_node).map(node_to_typed_by_ref)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Returns the first typing/specializes target of any kind (no part-like or
/// materialized-shape restriction — see [`typed_by_reference`]).
fn first_typing_or_specializes_target<'a>(
    graph: &'a SemanticGraph,
    node: &SemanticNode,
) -> Option<&'a SemanticNode> {
    graph
        .outgoing_typing_or_specializes_targets(node)
        .into_iter()
        .next()
}

fn node_source_location(node: &SemanticNode) -> ResolvedSourceLocation {
    ResolvedSourceLocation {
        uri: node.id.uri.to_string(),
        line: node.range.start.line,
    }
}

fn node_to_typed_by_ref(node: &SemanticNode) -> TypedByRef {
    TypedByRef {
        id: node.id.qualified_name.clone(),
        qualified_name: node.id.qualified_name.clone(),
        name: node.name.clone(),
        kind: node.element_kind.as_str().to_string(),
        source: node_source_location(node),
    }
}

fn compute_has_materialized_shape(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    visiting: &mut HashSet<NodeId>,
) -> bool {
    if let Some(cached) = graph.get_cached_shape(def_node) {
        return cached;
    }
    if !visiting.insert(def_node.id.clone()) {
        return false;
    }
    let has_direct = graph.children_of(def_node).iter().any(|child| {
        is_part_like(child.element_kind.as_str()) || is_port_like(child.element_kind.as_str())
    });
    let result = has_direct
        || graph
            .outgoing_typing_or_specializes_targets(def_node)
            .into_iter()
            .filter(|g| is_part_like(g.element_kind.as_str()))
            .any(|generalization| compute_has_materialized_shape(graph, generalization, visiting));
    visiting.remove(&def_node.id);
    graph.set_cached_shape(&def_node.id, result);
    result
}

/// Returns the first typing/specializes target that is a PartDef with materialized shape.
pub(crate) fn first_typed_definition_with_shape<'a>(
    graph: &'a SemanticGraph,
    node: &SemanticNode,
) -> Option<&'a SemanticNode> {
    graph
        .outgoing_typing_or_specializes_targets(node)
        .into_iter()
        .find(|def| is_part_like(def.element_kind.as_str()) && has_materialized_shape(graph, def))
}

fn collect_inherited_ports(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    parent_path: &str,
    out: &mut Vec<ExpandedPort>,
    seen: &mut HashSet<(String, String)>,
    visiting: &mut HashSet<NodeId>,
) {
    if !visiting.insert(def_node.id.clone()) {
        return;
    }
    // Recurse into generalizations first so more-specific ports win on dedup.
    for generalization in graph.outgoing_typing_or_specializes_targets(def_node) {
        if is_part_like(generalization.element_kind.as_str()) {
            collect_inherited_ports(graph, generalization, parent_path, out, seen, visiting);
        }
    }
    // Direct ports of this definition.
    for child in graph.children_of(def_node) {
        if !is_port_like(child.element_kind.as_str()) {
            continue;
        }
        let key = (parent_path.to_string(), child.name.clone());
        if seen.insert(key) {
            out.push(ExpandedPort {
                node_id: child.id.clone(),
                name: child.name.clone(),
                direction: child
                    .attributes
                    .get("direction")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                port_type: child
                    .attributes
                    .get("portType")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                parent_path: parent_path.to_string(),
            });
        }
    }
    visiting.remove(&def_node.id);
}

#[allow(clippy::too_many_arguments)]
fn expand_def_subtree(
    graph: &SemanticGraph,
    def_node: &SemanticNode,
    parent_path: &str,
    max_depth: Option<usize>,
    current_depth: usize,
    out: &mut Vec<ExpandedPart>,
    visiting_defs: &mut HashSet<NodeId>,
    existing_paths: &mut HashSet<String>,
) {
    if !visiting_defs.insert(def_node.id.clone()) {
        return;
    }

    let can_recurse = max_depth.is_none_or(|max| current_depth < max);

    for part_child in graph.children_of(def_node) {
        if !is_part_like(part_child.element_kind.as_str()) {
            continue;
        }
        let child_path = format!("{parent_path}.{}", part_child.name);
        if !existing_paths.insert(child_path.clone()) {
            continue;
        }

        // Find the child's typed definition once — used for both port lookup and recursive expansion.
        let grand_def = first_typed_definition_with_shape(graph, part_child);

        let mut child_part = ExpandedPart {
            node_id: part_child.id.clone(),
            name: part_child.name.clone(),
            element_kind: part_child.element_kind.as_str().to_string(),
            path: child_path.clone(),
            parent_path: Some(parent_path.to_string()),
            ports: Vec::new(),
            children: Vec::new(),
            attributes: part_child.attributes.clone(),
            uri: Some(part_child.id.uri.clone()),
        };

        // Ports come from the child's typed definition, not the parent def.
        child_part.ports = if let Some(gd) = grand_def {
            inherited_ports(graph, gd, &child_path)
        } else {
            Vec::new()
        };

        if can_recurse {
            let mut sub_out = Vec::new();
            // Expand usage-owned children first.
            expand_usage_children(
                graph,
                part_child,
                &child_path,
                max_depth,
                current_depth + 1,
                &mut sub_out,
                visiting_defs,
                existing_paths,
            );
            // Then expand from the typed definition.
            if let Some(gd) = grand_def {
                expand_def_subtree(
                    graph,
                    gd,
                    &child_path,
                    max_depth,
                    current_depth + 1,
                    &mut sub_out,
                    visiting_defs,
                    existing_paths,
                );
            }
            child_part.children = sub_out.clone();
            out.push(child_part);
            out.extend(sub_out);
        } else {
            out.push(child_part);
        }
    }

    visiting_defs.remove(&def_node.id);
}

#[allow(clippy::too_many_arguments)]
fn expand_usage_children(
    graph: &SemanticGraph,
    usage_node: &SemanticNode,
    parent_path: &str,
    max_depth: Option<usize>,
    current_depth: usize,
    out: &mut Vec<ExpandedPart>,
    visiting_defs: &mut HashSet<NodeId>,
    existing_paths: &mut HashSet<String>,
) {
    for part_child in graph.children_of(usage_node) {
        if !is_part_like(part_child.element_kind.as_str()) {
            continue;
        }
        let child_path = format!("{parent_path}.{}", part_child.name);
        if !existing_paths.insert(child_path.clone()) {
            continue;
        }
        let can_recurse = max_depth.is_none_or(|max| current_depth < max);
        let mut sub_out = Vec::new();
        if can_recurse {
            expand_usage_children(
                graph,
                part_child,
                &child_path,
                max_depth,
                current_depth + 1,
                &mut sub_out,
                visiting_defs,
                existing_paths,
            );
            if let Some(grand_def) = first_typed_definition_with_shape(graph, part_child) {
                expand_def_subtree(
                    graph,
                    grand_def,
                    &child_path,
                    max_depth,
                    current_depth + 1,
                    &mut sub_out,
                    visiting_defs,
                    existing_paths,
                );
            }
        }
        out.push(ExpandedPart {
            node_id: part_child.id.clone(),
            name: part_child.name.clone(),
            element_kind: part_child.element_kind.as_str().to_string(),
            path: child_path.clone(),
            parent_path: Some(parent_path.to_string()),
            ports: inherited_ports(graph, part_child, &child_path),
            children: sub_out.clone(),
            attributes: part_child.attributes.clone(),
            uri: Some(part_child.id.uri.clone()),
        });
        out.extend(sub_out);
    }
}

#[cfg(test)]
mod tests {
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
    use crate::semantic::workspace_graph::build_semantic_graph_from_documents;

    use super::typed_by_reference;

    fn build_graph(source: &str) -> crate::semantic::graph::SemanticGraph {
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            source.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
        graph
    }

    fn node_by_qualified_name<'a>(
        graph: &'a crate::semantic::graph::SemanticGraph,
        qualified_name: &str,
    ) -> &'a crate::semantic::model::SemanticNode {
        let ids = graph
            .node_ids_for_qualified_name(qualified_name)
            .unwrap_or_else(|| panic!("no node for {qualified_name}"));
        graph
            .get_node(&ids[0])
            .unwrap_or_else(|| panic!("dangling node id for {qualified_name}"))
    }

    const CLEANING_HEAD_MODEL: &str = r#"package Demo {
  part def PowerPort;
  part def BrushMotor;
  part def CleaningHead {
    part brushMotor : BrushMotor;
    port powerIn : PowerPort;
  }
  part def Robot {
    part cleaningHead : CleaningHead;
  }
}"#;

    #[test]
    fn typed_by_reference_resolves_usage_to_its_definition() {
        let graph = build_graph(CLEANING_HEAD_MODEL);
        let usage = node_by_qualified_name(&graph, "Demo::Robot::cleaningHead");
        let typed_by = typed_by_reference(&graph, usage).expect("typedBy should resolve");
        assert_eq!(typed_by.qualified_name, "Demo::CleaningHead");
        assert_eq!(typed_by.name, "CleaningHead");
    }

    #[test]
    fn typed_by_reference_is_none_for_untyped_usage() {
        let graph = build_graph("package Demo { part def Robot { part loose; } }");
        let usage = node_by_qualified_name(&graph, "Demo::Robot::loose");
        assert!(typed_by_reference(&graph, usage).is_none());
    }
}
