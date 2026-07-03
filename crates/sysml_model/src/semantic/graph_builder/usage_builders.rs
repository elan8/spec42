//! Shared materializers for AST usage constructs that can legally appear in more than one
//! containing body (top-level package body, `part def { ... }` body, `part <usage> { ... }`
//! body, nested occurrence/state bodies, ...). Centralizing these avoids independent call
//! sites re-deriving the same node attributes/edges and drifting apart — see the doc comments
//! on each `materialize_*` function below for the specific drift each one fixed.

use std::collections::HashMap;

use sysml_v2_parser::ast::{DefinitionPrefix, PartUsageBody};
use sysml_v2_parser::Node;
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId};
use crate::semantic::reference_resolution::{resolve_member_via_type, ResolveResult};
use crate::semantic::relationships::add_typing_edge_if_exists;

use super::expressions;
use super::occurrence_body;
use super::part_usage;
use super::requirement_body::walk_requirement_def_body;
use super::{add_node_and_recurse, qualified_name_for_node};

/// Builds the `part`-usage node (and recurses into its body), wiring the typing edge. Used by
/// the top-level package body, `part def` bodies, and `part` usage bodies alike.
pub(super) fn materialize_part_usage(
    n: &Node<sysml_v2_parser::ast::PartUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    g: &mut SemanticGraph,
) -> NodeId {
    let name = &n.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
    let range = span_to_range(&n.span);
    let mut attrs = HashMap::new();
    if let Some(ref prefix) = n.usage_prefix {
        attrs.insert(
            "usagePrefix".to_string(),
            serde_json::json!(match prefix {
                DefinitionPrefix::Abstract => "abstract",
                DefinitionPrefix::Variation => "variation",
            }),
        );
    }
    attrs.insert("partType".to_string(), serde_json::json!(&n.type_name));
    if let Some(ref m) = n.multiplicity {
        attrs.insert("multiplicity".to_string(), serde_json::json!(m));
    }
    attrs.insert("ordered".to_string(), serde_json::json!(n.ordered));
    if let Some((ref feat, ref val)) = n.subsets {
        attrs.insert("subsetsFeature".to_string(), serde_json::json!(feat));
        if let Some(v) = val {
            attrs.insert(
                "subsetsValue".to_string(),
                serde_json::json!(expressions::expression_to_debug_string(v)),
            );
        }
    }
    if let Some(ref r) = n.redefines {
        attrs.insert("redefines".to_string(), serde_json::json!(r));
    }
    if let Some(ref v) = n.value.value {
        attrs.insert(
            "value".to_string(),
            serde_json::json!(expressions::expression_to_debug_string(v)),
        );
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "part",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    let node_id = NodeId::new(uri, &qualified);
    add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
    if let PartUsageBody::Brace { elements } = &n.body {
        for child in elements {
            part_usage::build_from_part_usage_body_element(child, uri, Some(&qualified), &node_id, g);
        }
    }
    node_id
}

/// Builds an `attribute`-usage node, wiring the typing edge. Used by `part def` bodies and
/// `part` usage bodies. `parent_id` drives [`infer_attribute_usage_kind`] so an attribute that
/// redefines a port (`attribute redefines somePort;`) is classified as [`ElementKind::Port`]
/// rather than a plain attribute — this refinement must run consistently regardless of which
/// body the attribute usage is nested in.
pub(super) fn materialize_attribute_usage(
    n: &Node<sysml_v2_parser::ast::AttributeUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) -> NodeId {
    let name = &n.name;
    let kind = infer_attribute_usage_kind(g, parent_id, n.redefines.as_deref());
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, kind);
    let range = span_to_range(&n.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = n.typing {
        attrs.insert("attributeType".to_string(), serde_json::json!(t));
    }
    if let Some(ref s) = n.subsets {
        attrs.insert("subsetsFeature".to_string(), serde_json::json!(s));
    }
    if let Some(ref r) = n.references {
        attrs.insert("referencesFeature".to_string(), serde_json::json!(r));
    }
    if let Some(ref c) = n.crosses {
        attrs.insert("crossesFeature".to_string(), serde_json::json!(c));
    }
    if let Some(ref r) = n.redefines {
        attrs.insert("redefines".to_string(), serde_json::json!(r));
    }
    if let Some(ref v) = n.value.value {
        attrs.insert(
            "value".to_string(),
            serde_json::json!(expressions::expression_to_debug_string(v)),
        );
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        kind,
        name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    if let Some(ref t) = n.typing {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    NodeId::new(uri, &qualified)
}

/// Builds an `occurrence`-usage node, wiring the typing edge and recursing into its body.
/// Used by the top-level package body, `part def`/`part` usage bodies, and nested occurrence
/// bodies alike (four independent call sites previously; one of them — the `part` usage body
/// copy — silently dropped the body recursion, so any children of a nested `occurrence { ... }`
/// usage were missing from the graph).
pub(super) fn materialize_occurrence_usage(
    n: &Node<sysml_v2_parser::ast::OccurrenceUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    g: &mut SemanticGraph,
) -> NodeId {
    let name = &n.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "occurrence");
    let range = span_to_range(&n.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = n.type_name {
        attrs.insert("occurrenceType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "occurrence",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    if let Some(ref t) = n.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    let node_id = NodeId::new(uri, &qualified);
    if let sysml_v2_parser::ast::OccurrenceUsageBody::Brace { elements } = &n.body {
        for child in elements {
            occurrence_body::build_from_occurrence_body_element(
                child,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
    }
    node_id
}

/// Builds a `requirement`-usage node, wiring the typing edge and walking its body. Used by the
/// top-level package body, `part def` bodies, and nested state bodies alike (three independent
/// call sites previously; the top-level package-body copy silently dropped the
/// `subsetsFeature` attribute the other two set for `requirement ... subsets ...;`).
pub(super) fn materialize_requirement_usage(
    n: &Node<sysml_v2_parser::ast::RequirementUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    g: &mut SemanticGraph,
) -> NodeId {
    let name = &n.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "requirement");
    let range = span_to_range(&n.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = n.type_name {
        attrs.insert("requirementType".to_string(), serde_json::json!(t));
    }
    if let Some(ref subsets) = n.subsets {
        attrs.insert("subsetsFeature".to_string(), serde_json::json!(subsets));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "requirement",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    if let Some(ref t) = n.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    let node_id = NodeId::new(uri, &qualified);
    walk_requirement_def_body(g, uri, container_prefix, &qualified, &node_id, &n.body);
    node_id
}

fn infer_attribute_usage_kind(
    g: &SemanticGraph,
    parent_id: &NodeId,
    redefines: Option<&str>,
) -> &'static str {
    let Some(owner) = g.get_node(parent_id) else {
        return "attribute";
    };
    let Some(redefined_name) = redefines
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
    else {
        return "attribute";
    };
    match resolve_member_via_type(g, owner, redefined_name) {
        ResolveResult::Resolved(target_id) => g
            .get_node(&target_id)
            .map(|target| {
                if target.element_kind == ElementKind::Port {
                    "port"
                } else {
                    "attribute"
                }
            })
            .unwrap_or("attribute"),
        ResolveResult::Ambiguous | ResolveResult::Unresolved => "attribute",
    }
}
