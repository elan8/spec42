//! Builds semantic graph from parsed AST (packages, parts, ports, connections, etc.).

use std::collections::HashMap;

use crate::semantic::text_span::TextRange;
use sysml_v2_parser::ast::{RootElement, SubsettingRelationship, TypingRelationship};
use sysml_v2_parser::RootNamespace;
use url::Url;

use crate::semantic::ast_util::{
    declared_subsetting_targets, declared_typing_targets, span_to_range, subsetting_target,
    typing_targets,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{
    ConstructionOwner, DeclaredFeatureProperties, DeclaredRelationshipTarget, ElementKind, NodeId,
    RelationshipKind, SemanticEdge, SemanticNode,
};
use crate::semantic::relationships::add_semantic_edge_once;

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
    build_graph_from_doc_mode(root, uri, false)
}

/// Builds an authored-only graph fragment for the canonical publication resolver.
pub(crate) fn build_structural_graph_from_doc(root: &RootNamespace, uri: &Url) -> SemanticGraph {
    build_graph_from_doc_mode(root, uri, true)
}

fn build_graph_from_doc_mode(
    root: &RootNamespace,
    uri: &Url,
    structural_input_only: bool,
) -> SemanticGraph {
    let mut g = SemanticGraph::new();
    g.set_structural_input_only(structural_input_only);
    for node in &root.elements {
        match &node.value {
            // A RootNamespace is a sequence of PackageBodyElements. Packages, namespaces, and
            // library packages own an explicit body; materialize that body beneath the declared
            // namespace node as before.
            RootElement::Package(_)
            | RootElement::Namespace(_)
            | RootElement::LibraryPackage(_) => {
                build_root_container(&mut g, root, uri, &node.value);
            }
            // The parser preserves legal package-body members at file scope as `Member` rather
            // than inventing an implicit package. The root dispatcher admits only members with
            // parser-published semantic fields; it leaves opaque fallback declarations explicit
            // rather than deriving facts from source text. Keeping the absent parent preserves
            // authored root scope and avoids a synthetic namespace becoming a second authority.
            RootElement::Member(member) => {
                package_body::build_from_root_member(member, uri, root, &mut g);
            }
            // Imports are dedicated root variants, not `Member`s. They are still real authored
            // semantic elements and use the same materializer as package-body imports.
            RootElement::Import(import) => {
                package_body::materialize_import(&mut g, uri, None, None, import);
            }
        }
    }
    g.assert_no_pending_declared_membership_facts();
    g.assert_no_pending_declared_short_names();
    g
}

fn build_root_container(
    g: &mut SemanticGraph,
    root: &RootNamespace,
    uri: &Url,
    root_element: &RootElement,
) {
    let (elements, pkg_qualified, pkg_name_display, pkg_span) =
        crate::root_element_body(root_element)
            .expect("package, namespace, and library package have a root body");
    let pkg_qualified_disambiguated = qualified_name_for_node(
        g,
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
        root_element,
        RootElement::LibraryPackage(lp) if lp.is_standard
    );
    let mut root_attrs = HashMap::new();
    if is_standard_library {
        root_attrs.insert("isStandardLibrary".to_string(), serde_json::json!(true));
    }
    add_node_and_recurse(
        g,
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
            g,
        );
    }
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

/// Kind-tagged synthetic base name for anonymous elements (`"item def"` → `"_itemDef"`).
///
/// Used with [`qualified_name_for_node`], which appends `#kind` / `#kindN` on sibling collisions.
pub(super) fn anonymous_element_base_name(kind: &str) -> String {
    let mut camel = String::from("_");
    for (i, part) in kind.split_whitespace().enumerate() {
        if i == 0 {
            camel.push_str(part);
            continue;
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            camel.extend(first.to_uppercase());
            camel.push_str(chars.as_str());
        }
    }
    if camel == "_" {
        "_element".to_string()
    } else {
        camel
    }
}

/// Prefer a declared identification name; when empty, return a kind-tagged synthetic name and
/// mark `attrs["isAnonymous"] = true` so anonymous-but-legal defs/usages stay addressable
/// ([#32](https://github.com/elan8/spec42/issues/32)).
pub(super) fn resolve_addressable_name(
    declared: &str,
    kind: &str,
    attrs: &mut HashMap<String, serde_json::Value>,
) -> String {
    let declared = declared.trim();
    if !declared.is_empty() {
        return declared.to_string();
    }
    attrs.insert("isAnonymous".to_string(), serde_json::json!(true));
    anonymous_element_base_name(kind)
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
    let declared_membership = g.take_declared_membership_facts(&node_id);
    let declared_short_name = g.take_declared_short_name(&node_id);
    let is_anonymous = attrs
        .get("isAnonymous")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let node = SemanticNode {
        id: node_id.clone(),
        element_kind: ElementKind::from(kind),
        declared_name: (!is_anonymous && !name.is_empty()).then(|| name.clone()),
        name,
        range,
        attributes: attrs,
        declared_facts: crate::semantic::model::DeclaredSemanticFacts {
            membership: declared_membership,
            short_name: declared_short_name,
            ..Default::default()
        },
        source_text: crate::semantic::model::SourceTextFacts::default(),
        expression_text: crate::semantic::model::DeclaredExpressionText::default(),
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

/// Records the AST-authored name separately from the node's effective name.
///
/// Anonymous redefinitions inherit an effective name for identity and
/// resolution, but must remain observably anonymous to consumers that need
/// authored-vs-derived provenance.
pub(super) fn attach_declared_name(g: &mut SemanticGraph, node_id: &NodeId, declared: &str) {
    if let Some(node) = g.get_node_mut(node_id) {
        node.declared_name = (!declared.trim().is_empty()).then(|| declared.to_string());
    }
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

/// Records parser-authored relationship targets on the node that owns the
/// declaration. Linking consumes these facts; `attributes` only projects them.
pub(super) fn attach_declared_relationship_targets<'a>(
    g: &mut SemanticGraph,
    node_id: &NodeId,
    kind: RelationshipKind,
    targets: impl IntoIterator<Item = &'a str>,
) {
    let Some(node) = g.get_node_mut(node_id) else {
        return;
    };
    for target in targets {
        node.declared_facts
            .relationships
            .record_reference(&kind, target);
    }
}

fn attach_declared_relationship_target_facts(
    g: &mut SemanticGraph,
    node_id: &NodeId,
    kind: RelationshipKind,
    targets: impl IntoIterator<Item = DeclaredRelationshipTarget>,
) {
    let Some(node) = g.get_node_mut(node_id) else {
        return;
    };
    for target in targets {
        node.declared_facts
            .relationships
            .record_target(&kind, target);
    }
}

pub(super) fn attach_declared_typing_relationship(
    g: &mut SemanticGraph,
    node_id: &NodeId,
    typing: Option<&TypingRelationship>,
) {
    attach_declared_relationship_target_facts(
        g,
        node_id,
        RelationshipKind::Typing,
        declared_typing_targets(typing),
    );
}

/// Records all four parser-owned subsetting-family clauses for a declaration.
pub(super) fn attach_declared_subsetting_family(
    g: &mut SemanticGraph,
    node_id: &NodeId,
    subsets: Option<&SubsettingRelationship>,
    redefines: Option<&SubsettingRelationship>,
    references: Option<&SubsettingRelationship>,
    crosses: Option<&SubsettingRelationship>,
) {
    for (kind, relationship) in [
        (RelationshipKind::Subsetting, subsets),
        (RelationshipKind::Redefinition, redefines),
        (RelationshipKind::ReferenceSubsetting, references),
        (RelationshipKind::CrossSubsetting, crosses),
    ] {
        attach_declared_relationship_target_facts(
            g,
            node_id,
            kind,
            declared_subsetting_targets(relationship),
        );
    }
}

/// Attaches a `doc /* ... */` comment as an addressable Documentation child of the
/// annotated element, wires an Annotation edge, and keeps the convenience `doc`
/// attribute text on the annotated node (multiple docs join with a blank line).
pub(super) fn attach_doc_comment(g: &mut SemanticGraph, node_id: &NodeId, text: &str) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }
    let Some(annotated) = g.get_node(node_id).cloned() else {
        return;
    };
    let combined = match annotated.source_text.doc.as_deref() {
        Some(existing) if !existing.is_empty() => format!("{existing}\n\n{text}"),
        _ => text.to_string(),
    };
    if let Some(node) = g.get_node_mut(node_id) {
        // The `doc` attribute is kept in the legacy display map for consumers not yet migrated
        // off it (see `UNIFY_CACHE_PROGRESS.md` chunk G); `source_text.doc` is the canonical typed
        // fact and the only value hover presentation reads.
        node.attributes
            .insert("doc".to_string(), serde_json::json!(combined));
        node.source_text.doc = Some(combined.clone());
    }

    let uri = &node_id.uri;
    let container_prefix = Some(node_id.qualified_name.as_str());
    let qualified =
        qualified_name_for_node(g, uri, container_prefix, "_documentation", "documentation");
    let mut attrs = HashMap::new();
    attrs.insert("body".to_string(), serde_json::json!(text));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "documentation",
        String::new(),
        annotated.range,
        attrs,
        Some(node_id),
    );
    let doc_id = NodeId::new(uri, &qualified);
    if let Some(doc_node) = g.get_node_mut(&doc_id) {
        doc_node.source_text.body = Some(text.to_string());
    }
    add_semantic_edge_once(
        g,
        &doc_id,
        node_id,
        SemanticEdge::plain(
            RelationshipKind::Annotation,
            ConstructionOwner::DocumentConstruction,
        ),
    );
}

/// Inserts a `specializes` attribute on a def-kind node's attribute map, if present. SysML v2
/// allows a comma-separated multi-target clause (`specializes A, B;`), so this joins every
/// declared target for display -- the real per-target `Specializes` edges are wired separately by
/// [`wire_def_specialization_edge`].
pub(super) fn insert_def_specialization_attr(
    attrs: &mut HashMap<String, serde_json::Value>,
    specializes: Option<&TypingRelationship>,
) {
    let targets = typing_targets(specializes);
    if !targets.is_empty() {
        attrs.insert(
            "specializes".to_string(),
            serde_json::json!(targets.join(", ")),
        );
    }
}

/// Wires a `Specializes` edge for a def-kind node for every declared `specializes` target, not
/// just the first -- `specializes A, B;` is two independent `Subclassification` relationships
/// (SysML v2 comma-separated multi-target clause), not one.
pub(super) fn wire_def_specialization_edge(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    container_prefix: Option<&str>,
    specializes: Option<&TypingRelationship>,
) {
    let node_id = NodeId::new(uri, qualified);
    attach_declared_relationship_target_facts(
        g,
        &node_id,
        RelationshipKind::Specializes,
        declared_typing_targets(specializes),
    );
    for target in typing_targets(specializes) {
        crate::semantic::relationships::add_specializes_edge_if_exists(
            g,
            uri,
            qualified,
            target,
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

#[cfg(test)]
mod anonymous_name_tests {
    use super::{anonymous_element_base_name, resolve_addressable_name};
    use std::collections::HashMap;

    #[test]
    fn kind_tagged_base_names() {
        assert_eq!(anonymous_element_base_name("item def"), "_itemDef");
        assert_eq!(
            anonymous_element_base_name("constraint def"),
            "_constraintDef"
        );
        assert_eq!(anonymous_element_base_name("calc def"), "_calcDef");
        assert_eq!(anonymous_element_base_name("constraint"), "_constraint");
        assert_eq!(anonymous_element_base_name(""), "_element");
    }

    #[test]
    fn resolve_marks_anonymous_and_preserves_declared() {
        let mut attrs = HashMap::new();
        assert_eq!(
            resolve_addressable_name("Widget", "item def", &mut attrs),
            "Widget"
        );
        assert!(attrs.is_empty());

        assert_eq!(
            resolve_addressable_name("", "item def", &mut attrs),
            "_itemDef"
        );
        assert_eq!(attrs.get("isAnonymous"), Some(&serde_json::json!(true)));
    }
}

#[cfg(test)]
mod root_namespace_tests {
    use url::Url;

    use super::build_graph_from_doc;
    use crate::semantic::graph::SemanticGraph;
    use crate::semantic::model::{ElementKind, NodeId};
    use crate::semantic::pipeline::{build_and_link_graph, patch_graph_for_document};
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};

    fn parse(source: &str) -> sysml_v2_parser::RootNamespace {
        sysml_v2_parser::parse(source).expect("root namespace parses")
    }

    #[test]
    fn root_members_use_the_existing_typed_package_body_materializers() {
        let source = "import Catalog::*; part def Base; part item : Base;";
        let uri = Url::parse("file:///root-members.sysml").expect("URI");
        let graph = build_graph_from_doc(&parse(source), &uri);

        let base_id = NodeId::new(&uri, "Base");
        let base = graph.get_node(&base_id).expect("root part definition");
        assert_eq!(base.element_kind, ElementKind::PartDef);
        assert_eq!(
            base.parent_id, None,
            "root member keeps authored root scope"
        );

        let item_id = NodeId::new(&uri, "item");
        let item = graph.get_node(&item_id).expect("root part usage");
        assert_eq!(item.element_kind, ElementKind::Part);
        assert_eq!(
            item.parent_id, None,
            "root member keeps authored root scope"
        );
        assert!(item
            .declared_facts
            .relationships
            .typing
            .iter()
            .any(|target| target.reference == "Base"));

        let import = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.element_kind == ElementKind::Import)
            .expect("root import");
        assert_eq!(
            import.parent_id, None,
            "root import keeps authored root scope"
        );
        assert_eq!(
            import
                .declared_facts
                .membership
                .as_ref()
                .and_then(|membership| membership.import.as_ref())
                .map(|facts| facts.target.reference.as_str()),
            Some("Catalog::*")
        );
    }

    #[test]
    fn opaque_root_declarations_do_not_invent_semantic_facts_from_source_text() {
        let uri = Url::parse("file:///root-kerml.sysml").expect("URI");
        let graph = build_graph_from_doc(
            &parse("class Camera { feature focusedState : Camera; } feature state : Camera;"),
            &uri,
        );
        assert_eq!(graph.graph.node_count(), 0);
        assert_eq!(graph.graph.edge_count(), 0);

        let behavior_graph = build_graph_from_doc(
            &parse("behavior TakePicture { succession first then second; }"),
            &uri,
        );
        assert_eq!(behavior_graph.graph.node_count(), 0);
        assert_eq!(behavior_graph.graph.edge_count(), 0);
    }

    #[test]
    fn root_comments_do_not_materialize_semantic_facts() {
        let uri = Url::parse("file:///root-comment.sysml").expect("URI");
        let graph = build_graph_from_doc(&parse("// source comment only\n"), &uri);
        assert_eq!(graph.graph.node_count(), 0);
        assert_eq!(graph.graph.edge_count(), 0);
    }

    #[test]
    fn root_member_full_and_incremental_construction_match() {
        let source = "import Catalog::*; part def Base; part item : Base;";
        let document = SysmlDocument::from_memory_path(
            "workspace",
            "root-member-parity.sysml",
            source.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let uri = document.uri.clone();
        let parsed = parse(source);
        let (full, _) = build_and_link_graph(&[document]).expect("full graph");

        let mut incremental = SemanticGraph::new();
        patch_graph_for_document(&mut incremental, &uri, Some(&parsed), true);

        assert_eq!(full.to_semantic_sexpr(), incremental.to_semantic_sexpr());
    }
}
