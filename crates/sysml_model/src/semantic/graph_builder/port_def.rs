use std::collections::HashMap;

use sysml_v2_parser::ast::{
    InOut, InOutDecl, ItemUsage, PortBody, PortBodyElement, PortDefBodyElement, PortUsage,
};
use sysml_v2_parser::Node;
use url::Url;

use crate::semantic::ast_util::{span_to_range, subsetting_target, typing_target};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;

use super::attribute_body;
use super::expressions;
use super::{add_node_and_recurse, qualified_name_for_node};

fn build_in_out_decl(
    w: &Node<InOutDecl>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let d = &w.value;
    let qualified = qualified_name_for_node(g, uri, container_prefix, &d.name, "in out parameter");
    let range = span_to_range(&w.span);
    let mut attrs = HashMap::new();
    attrs.insert(
        "direction".to_string(),
        serde_json::json!(match d.direction {
            InOut::In => "in",
            InOut::Out => "out",
            InOut::InOut => "inout",
        }),
    );
    attrs.insert("parameterType".to_string(), serde_json::json!(&d.type_name));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "in out parameter",
        d.name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    add_typing_edge_if_exists(g, uri, &qualified, &d.type_name, container_prefix);
}

/// Materializes a port usage node and recurses into a structured `PortBody` when present.
pub(super) fn materialize_port_usage(
    n: &Node<PortUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) -> NodeId {
    let name = super::effective_usage_name(&n.name, n.redefines.as_deref());
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
    if let Some(ref r) = n.references {
        attrs.insert("referencesFeature".to_string(), serde_json::json!(r));
    }
    if let Some(ref c) = n.crosses {
        attrs.insert("crossesFeature".to_string(), serde_json::json!(c));
    }
    if let Some(ref r) = n.redefines {
        attrs.insert("redefines".to_string(), serde_json::json!(r));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "port",
        name.to_string(),
        range,
        attrs,
        Some(parent_id),
    );
    let node_id = NodeId::new(uri, &qualified);
    if let Some(ref t) = n.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    if let PortBody::Brace { elements } = &n.body {
        for child in elements {
            build_from_port_body_element(child, uri, Some(&qualified), &node_id, g);
        }
    }
    node_id
}

pub(super) fn build_from_port_body_element(
    node: &Node<PortBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use PortBodyElement as PBE;
    match &node.value {
        PBE::PortUsage(n) => {
            materialize_port_usage(n, uri, container_prefix, parent_id, g);
        }
        PBE::InOutDecl(w) => build_in_out_decl(w, uri, container_prefix, parent_id, g),
        PBE::AttributeUsage(attribute) => {
            super::usage_builders::materialize_attribute_usage(attribute, uri, container_prefix, parent_id, g);
        }
        PBE::ItemUsage(item) => materialize_port_def_item_usage(item, uri, container_prefix, parent_id, g),
        PBE::Error(_) => {}
        PBE::Doc(doc) => super::attach_doc_comment(g, parent_id, &doc.value.text),
    }
}

pub(super) fn build_from_port_def_body_element(
    node: &Node<PortDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use PortDefBodyElement as PDBE;
    match &node.value {
        PDBE::Doc(doc) => {
            super::attach_doc_comment(g, parent_id, &doc.value.text);
        }
        PDBE::InOutDecl(w) => build_in_out_decl(w, uri, container_prefix, parent_id, g),
        PDBE::AttributeDef(n) => {
            let name = &n.name;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name, "attribute def");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(t) = typing_target(n.typing.as_deref()) {
                attrs.insert("attributeType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute def",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(t) = typing_target(n.typing.as_deref()) {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PDBE::AttributeUsage(n) => {
            if let Some(direction) = n.direction {
                let name = super::effective_usage_name(&n.name, n.redefines.as_deref());
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, name, "in out parameter");
                let range = span_to_range(&n.span);
                let mut attrs = HashMap::new();
                attrs.insert(
                    "direction".to_string(),
                    serde_json::json!(match direction {
                        InOut::In => "in",
                        InOut::Out => "out",
                        InOut::InOut => "inout",
                    }),
                );
                let parameter_type = typing_target(n.typing.as_deref())
                    .or(subsetting_target(n.subsets.as_deref()))
                    .unwrap_or_default();
                if !parameter_type.is_empty() {
                    attrs.insert(
                        "parameterType".to_string(),
                        serde_json::json!(parameter_type),
                    );
                }
                if let Some(r) = subsetting_target(n.redefines.as_deref()) {
                    attrs.insert("redefines".to_string(), serde_json::json!(r));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "in out parameter",
                    name.to_string(),
                    range,
                    attrs,
                    Some(parent_id),
                );
                if !parameter_type.is_empty() {
                    add_typing_edge_if_exists(g, uri, &qualified, parameter_type, container_prefix);
                }
            } else {
                let name = super::effective_usage_name(&n.name, n.redefines.as_deref());
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, name, "attribute");
                let range = span_to_range(&n.span);
                let mut attrs = HashMap::new();
                if let Some(t) = typing_target(n.typing.as_deref()) {
                    attrs.insert("attributeType".to_string(), serde_json::json!(t));
                }
                if let Some(s) = subsetting_target(n.subsets.as_deref()) {
                    attrs.insert("subsetsFeature".to_string(), serde_json::json!(s));
                }
                if let Some(r) = subsetting_target(n.references.as_deref()) {
                    attrs.insert("referencesFeature".to_string(), serde_json::json!(r));
                }
                if let Some(c) = subsetting_target(n.crosses.as_deref()) {
                    attrs.insert("crossesFeature".to_string(), serde_json::json!(c));
                }
                if let Some(r) = subsetting_target(n.redefines.as_deref()) {
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
                    name.to_string(),
                    range,
                    attrs,
                    Some(parent_id),
                );
                if let Some(ref t) = n.typing {
                    add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
                }
            }
        }
        PDBE::ItemUsage(n) => {
            materialize_port_def_item_usage(n, uri, container_prefix, parent_id, g);
        }
        PDBE::ItemDef(item) => super::package_body::materialize_item_def(
            g, uri, container_prefix, Some(parent_id), item,
        ),
        PDBE::EnumerationUsage(_) => {}
        PDBE::PortUsage(n) => {
            materialize_port_usage(n, uri, container_prefix, parent_id, g);
        }
        PDBE::Error(_) | PDBE::Other(_) => {}
    }
}

fn materialize_port_def_item_usage(
    n: &Node<ItemUsage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let name = &n.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "item");
    let range = span_to_range(&n.span);
    let mut attrs = HashMap::new();
    if let Some(direction) = n.direction {
        attrs.insert(
            "direction".to_string(),
            serde_json::json!(match direction {
                InOut::In => "in",
                InOut::Out => "out",
                InOut::InOut => "inout",
            }),
        );
    }
    if let Some(ref t) = n.type_name {
        attrs.insert("itemType".to_string(), serde_json::json!(t));
    }
    if let Some(ref m) = n.multiplicity {
        attrs.insert("multiplicity".to_string(), serde_json::json!(m));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "item",
        name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    if let Some(ref t) = n.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    let node_id = NodeId::new(uri, &qualified);
    attribute_body::build_from_attribute_body(&n.body, uri, Some(&qualified), &node_id, g);
}
