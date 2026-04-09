use std::collections::HashMap;

use sysml_v2_parser::ast::{PartDefBody, PartUsageBody, StateDefBody};
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{
    add_edge_if_both_exist, add_typing_edge_if_exists, find_part_def_in_root, type_ref_candidates,
};

use super::expressions;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_part_usage_body_element(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "attribute");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
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
                "attribute",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
        }
        PUBE::PartUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
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
                Some(parent_id),
            );
            let node_id = NodeId::new(uri, &qualified);
            add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
            if let PartUsageBody::Brace { elements } = &n.body {
                for child in elements {
                    build_from_part_usage_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        root,
                        g,
                    );
                }
            }
            expand_typed_part_usage(
                root,
                uri,
                &qualified,
                &n.type_name,
                container_prefix,
                &node_id,
                g,
            );
        }
        PUBE::PortUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "port");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = n.type_name {
                attrs.insert("portType".to_string(), serde_json::json!(t));
            }
            if let Some(ref m) = n.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
            }
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
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "port",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = n.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PUBE::Connect(c) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &c.from,
                &c.to,
                RelationshipKind::Connection,
            );
        }
        PUBE::Bind(b) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &b.left,
                &b.right,
                RelationshipKind::Bind,
            );
        }
        PUBE::InterfaceUsage(interface_usage) => {
            use sysml_v2_parser::ast::InterfaceUsage;
            match &interface_usage.value {
                InterfaceUsage::TypedConnect { from, to, .. }
                | InterfaceUsage::Connection { from, to, .. } => {
                    expressions::add_expression_edge_if_both_exist(
                        g,
                        uri,
                        container_prefix,
                        from,
                        to,
                        RelationshipKind::Connection,
                    );
                }
            }
        }
        PUBE::Perform(perform_node) => {
            let perform_qualified = expressions::add_perform_usage_node(
                g,
                uri,
                container_prefix,
                parent_id,
                &perform_node.value.action_name,
                perform_node.value.type_name.as_deref(),
                span_to_range(&perform_node.span),
            );
            add_edge_if_both_exist(
                g,
                uri,
                &parent_id.qualified_name,
                &perform_qualified,
                RelationshipKind::Perform,
            );
        }
        PUBE::Allocate(allocate_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &allocate_node.source,
                &allocate_node.target,
                RelationshipKind::Allocate,
            );
        }
        PUBE::Satisfy(satisfy_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &satisfy_node.source,
                &satisfy_node.target,
                RelationshipKind::Satisfy,
            );
        }
        PUBE::Ref(r) => {
            let n = &r.value;
            let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "ref");
            let range = span_to_range(&r.span);
            let mut attrs = HashMap::new();
            attrs.insert("refType".to_string(), serde_json::json!(&n.type_name));
            if let Some(ref v) = n.value {
                attrs.insert(
                    "value".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(v)),
                );
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "ref",
                n.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
        }
        PUBE::StateUsage(state_node) => {
            let name = &state_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "state");
            let range = span_to_range(&state_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = state_node.type_name {
                attrs.insert("stateType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "state",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            let state_id = NodeId::new(uri, &qualified);
            if let Some(ref t) = state_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let StateDefBody::Brace { elements } = &state_node.body {
                super::state::build_from_state_body(elements, uri, Some(&qualified), &state_id, g);
            }
        }
        PUBE::MetadataAnnotation(meta) => {
            let m = &meta.value;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &m.name, "metadata usage");
            let range = span_to_range(&meta.span);
            let mut attrs = HashMap::new();
            attrs.insert("annotationName".to_string(), serde_json::json!(&m.name));
            if let Some(ref t) = m.type_name {
                attrs.insert("metadataType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "metadata usage",
                m.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = m.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PUBE::Error(_) | PUBE::Doc(_) => {}
    }
}

/// Expands a typed PartUsage by adding nodes for the type's nested parts and ports.
pub(super) fn expand_typed_part_usage(
    root: &RootNamespace,
    uri: &Url,
    usage_qualified: &str,
    type_ref: &str,
    _container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let pkg_prefix = usage_qualified
        .split("::")
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    let candidates = type_ref_candidates(Some(pkg_prefix), type_ref);
    if let Some((part_def_node, part_def_qualified)) = candidates
        .iter()
        .find_map(|c| find_part_def_in_root(root, c))
    {
        let mut expansion_stack = vec![part_def_qualified];
        expand_part_def_members(
            root,
            uri,
            usage_qualified,
            part_def_node,
            parent_id,
            g,
            pkg_prefix,
            &mut expansion_stack,
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn expand_part_def_members(
    root: &RootNamespace,
    uri: &Url,
    container_qualified: &str,
    part_def: &sysml_v2_parser::Node<sysml_v2_parser::PartDef>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
    pkg_prefix: &str,
    expansion_stack: &mut Vec<String>,
) {
    if let PartDefBody::Brace { elements } = &part_def.body {
        for node in elements {
            use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
            match &node.value {
                PDBE::AttributeDef(n) => {
                    let qualified = qualified_name_for_node(
                        g,
                        uri,
                        Some(container_qualified),
                        &n.name,
                        "attribute def",
                    );
                    let mut attrs = HashMap::new();
                    if let Some(ref t) = n.typing {
                        attrs.insert("attributeType".to_string(), serde_json::json!(t));
                    }
                    super::add_node_if_not_exists(
                        g,
                        uri,
                        &qualified,
                        "attribute def",
                        n.name.clone(),
                        parent_id,
                        span_to_range(&n.span),
                        attrs,
                    );
                    if let Some(ref t) = n.typing {
                        add_typing_edge_if_exists(g, uri, &qualified, t, Some(container_qualified));
                    }
                }
                PDBE::PortUsage(n) => {
                    let qualified =
                        qualified_name_for_node(g, uri, Some(container_qualified), &n.name, "port");
                    let mut attrs = HashMap::new();
                    if let Some(ref t) = n.type_name {
                        attrs.insert("portType".to_string(), serde_json::json!(t));
                    }
                    super::add_node_if_not_exists(
                        g,
                        uri,
                        &qualified,
                        "port",
                        n.name.clone(),
                        parent_id,
                        span_to_range(&n.span),
                        attrs,
                    );
                    if let Some(ref t) = n.type_name {
                        add_typing_edge_if_exists(g, uri, &qualified, t, Some(container_qualified));
                    }
                }
                PDBE::PartUsage(n) => {
                    let qualified =
                        qualified_name_for_node(g, uri, Some(container_qualified), &n.name, "part");
                    let mut attrs = HashMap::new();
                    attrs.insert("partType".to_string(), serde_json::json!(&n.type_name));
                    if let Some(ref m) = n.multiplicity {
                        attrs.insert("multiplicity".to_string(), serde_json::json!(m));
                    }
                    super::add_node_if_not_exists(
                        g,
                        uri,
                        &qualified,
                        "part",
                        n.name.clone(),
                        parent_id,
                        span_to_range(&n.span),
                        attrs,
                    );
                    let node_id = NodeId::new(uri, &qualified);
                    add_typing_edge_if_exists(
                        g,
                        uri,
                        &qualified,
                        &n.type_name,
                        Some(container_qualified),
                    );

                    if let PartUsageBody::Brace { elements } = &n.body {
                        for child in elements {
                            build_from_part_usage_body_element(
                                child,
                                uri,
                                Some(&qualified),
                                &node_id,
                                root,
                                g,
                            );
                        }
                    }

                    let nested_candidates = type_ref_candidates(Some(pkg_prefix), &n.type_name);
                    if let Some((nested_def, nested_def_qualified)) = nested_candidates
                        .iter()
                        .find_map(|c| find_part_def_in_root(root, c))
                    {
                        if expansion_stack
                            .iter()
                            .any(|visited| visited == &nested_def_qualified)
                        {
                            continue;
                        }
                        expansion_stack.push(nested_def_qualified);
                        expand_part_def_members(
                            root,
                            uri,
                            &qualified,
                            nested_def,
                            &node_id,
                            g,
                            pkg_prefix,
                            expansion_stack,
                        );
                        expansion_stack.pop();
                    }
                }
                _ => {}
            }
        }
    }
}
