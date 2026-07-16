//! Builds semantic graph from parsed AST (packages, parts, ports, connections, etc.).

use std::collections::HashMap;

use crate::semantic::text_span::TextRange;
use sysml_v2_parser::ast::{RootElement, SubsettingRelationship, TypingRelationship};
use sysml_v2_parser::RootNamespace;
use url::Url;

use crate::semantic::ast_util::{span_to_range, subsetting_target, typing_target};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{DeclaredFeatureProperties, ElementKind, NodeId, SemanticNode};

mod action;
mod analysis_case;
mod attribute_body;
mod calc_constraint_def;
mod definition_body;
pub(crate) mod expressions;
mod flow_usage;
mod interface_def;
mod kerml_library;
mod metadata_def;
mod metadata_keyword;
mod modeled_kerml_name;
mod occurrence_body;
mod package_body;
mod package_packages;
mod part_def;
mod part_usage;
mod payload;
mod port_def;
mod ref_decl;
mod requirement_body;
mod state;
pub(crate) mod unit_metadata;
pub(crate) mod unit_type_promotion;
mod usage_builders;
mod use_case;
mod verification;
mod view_def;

pub struct MaterializeContext<'a> {
    pub uri: &'a Url,
    pub ast: &'a RootNamespace,
    pub graph: &'a mut SemanticGraph,
}

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
        let is_standard_library = matches!(
            &node.value,
            RootElement::LibraryPackage(lp) if lp.is_standard
        );
        let mut root_attrs = HashMap::new();
        if is_standard_library {
            root_attrs.insert("isStandardLibrary".to_string(), serde_json::json!(true));
        }
        add_node_and_recurse(
            &mut g,
            uri,
            &pkg_qualified_final,
            "package",
            pkg_name_display,
            span_to_range(pkg_span),
            root_attrs,
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

pub(crate) fn qualified_name(container_prefix: Option<&str>, name: &str) -> String {
    match container_prefix {
        Some(p) if !p.is_empty() => format!("{}::{}", p, name),
        _ => name.to_string(),
    }
}

/// Resolves a usage's *effective name* per SysML v2 §7.6.5 "Effective Names": if a name is
/// declared, use it; otherwise, for a usage with an owned redefinition (`redefines`), fall back
/// to the (simple, last-segment) name of the feature it redefines. Spec example: `part redefines
/// cylinders[4];` has no declared name, but its effective name is `"cylinders"`.
pub(super) fn effective_usage_name<'a>(
    declared: &'a str,
    redefines: Option<&'a SubsettingRelationship>,
) -> &'a str {
    if !declared.is_empty() {
        return declared;
    }
    match subsetting_target(redefines)
        .map(str::trim)
        .filter(|r| !r.is_empty())
    {
        Some(r) => r.rsplit("::").next().unwrap_or(r),
        None => declared,
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
    range: TextRange,
    attrs: HashMap<String, serde_json::Value>,
    parent_id: Option<&NodeId>,
) {
    let node_id = NodeId::new(uri, qualified);
    let node = SemanticNode {
        id: node_id.clone(),
        element_kind: ElementKind::from(kind),
        name,
        range,
        attributes: attrs,
        declared_facts: Default::default(),
        parent_id: parent_id.cloned(),
    };
    // Also index the node under its short-name-qualified variant (if any), so
    // typing/specializes references by short name (e.g. `part x : CB;` when the def is
    // `part def <'CB'> ControlBoard;`) resolve to the same node. A real element that happens
    // to share a name with this alias collides naturally via the existing Vec<NodeId>-per-key
    // + ambiguity-detection mechanism (see resolve_name's `len() > 1 => Ambiguous`) — no extra
    // handling needed here. Must run before `node` moves into `add_node` below.
    g.register_short_name_alias(&node_id, &node);
    let idx = g.graph.add_node(node);
    g.node_index_by_id.insert(node_id.clone(), idx);
    g.nodes_by_uri
        .entry(uri.clone())
        .or_default()
        .push(node_id.clone());
    g.node_ids_by_qualified_name
        .entry(qualified.to_string())
        .or_default()
        .push(NodeId::new(uri, qualified));
    if let Some(pid) = parent_id {
        g.children_by_parent_id
            .entry(pid.clone())
            .or_default()
            .push(node_id);
    }
    g.invalidate_query_indexes();
}

/// Records typed declaration modifiers on a semantic node.
pub(super) fn attach_feature_properties(
    g: &mut SemanticGraph,
    node_id: &NodeId,
    properties: DeclaredFeatureProperties,
) {
    if let Some(node) = g.get_node_mut(node_id) {
        node.declared_facts.feature_properties = Some(properties);
    }
}

/// Attaches a `doc /* ... */` comment's text to the `doc` attribute of the node it
/// annotates. Multiple doc blocks on the same node are joined with a blank line.
pub(super) fn attach_doc_comment(g: &mut SemanticGraph, node_id: &NodeId, text: &str) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }
    if let Some(node) = g.get_node_mut(node_id) {
        let combined = match node.attributes.get("doc").and_then(|v| v.as_str()) {
            Some(existing) if !existing.is_empty() => format!("{existing}\n\n{text}"),
            _ => text.to_string(),
        };
        node.attributes
            .insert("doc".to_string(), serde_json::json!(combined));
    }
}

/// Inserts a `specializes` attribute on a def-kind node's attribute map, if present.
pub(super) fn insert_def_specialization_attr(
    attrs: &mut HashMap<String, serde_json::Value>,
    specializes: Option<&TypingRelationship>,
) {
    if let Some(s) = typing_target(specializes) {
        attrs.insert("specializes".to_string(), serde_json::json!(s));
    }
}

/// Wires the `Specializes` edge for a def-kind node, if it declares a `specializes` target.
pub(super) fn wire_def_specialization_edge(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    container_prefix: Option<&str>,
    specializes: Option<&TypingRelationship>,
) {
    if let Some(s) = typing_target(specializes) {
        crate::semantic::relationships::add_specializes_edge_if_exists(
            g,
            uri,
            qualified,
            s,
            container_prefix,
        );
    }
}

#[cfg(test)]
mod short_name_tests {
    use url::Url;

    use crate::semantic::graph::SemanticGraph;
    use crate::semantic::pipeline::patch_graph_for_document;

    fn build(content: &str) -> (SemanticGraph, Url) {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let parsed = sysml_v2_parser::parse(content).expect("parse");
        let mut graph = SemanticGraph::new();
        patch_graph_for_document(&mut graph, &uri, Some(&parsed), true);
        (graph, uri)
    }

    #[test]
    fn typing_resolves_by_short_name_when_declared_alongside_a_name() {
        let (graph, uri) = build("package Demo { part def <'CB'> ControlBoard; part x : CB; }");
        let usage = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.name == "x")
            .expect("usage node present");
        let targets = graph.outgoing_typing_or_specializes_targets(usage);
        assert!(
            targets.iter().any(|target| target.name == "ControlBoard"),
            "expected `x` to resolve its type through short name `CB`, got {targets:#?}"
        );
    }

    #[test]
    fn nested_member_resolves_by_short_name() {
        let (graph, uri) =
            build("package Demo { part def Robot { part def <'CB'> ControlBoard; } }");
        let robot = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.name == "Robot")
            .expect("Robot node present");
        let matches = graph.child_named(&robot.id, "CB");
        assert!(
            matches.iter().any(|node| node.name == "ControlBoard"),
            "expected child_named(\"CB\") to find ControlBoard, got {matches:#?}"
        );
    }

    #[test]
    fn short_name_alias_does_not_shadow_a_real_element_of_the_same_name() {
        let (graph, uri) = build("package Demo { part def <'CB'> ControlBoard; part def CB; }");
        let matching_ids = graph
            .node_ids_by_qualified_name
            .get("Demo::CB")
            .cloned()
            .unwrap_or_default();
        assert_eq!(
            matching_ids.len(),
            2,
            "expected both the real `CB` element and the `ControlBoard` alias under \
             \"Demo::CB\", got {matching_ids:#?}"
        );
        let _ = uri;
    }

    #[test]
    fn removing_a_document_deregisters_its_short_name_alias() {
        let (mut graph, uri) = build("package Demo { part def <'CB'> ControlBoard; part x : CB; }");
        assert!(graph.node_ids_by_qualified_name.contains_key("Demo::CB"));

        patch_graph_for_document(&mut graph, &uri, None, true);

        assert!(
            !graph.node_ids_by_qualified_name.contains_key("Demo::CB"),
            "expected the short-name alias to be cleaned up once ControlBoard's document is \
             removed, got {:#?}",
            graph.node_ids_by_qualified_name.get("Demo::CB")
        );
    }
}
