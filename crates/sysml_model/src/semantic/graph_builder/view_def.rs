//! View/viewpoint/rendering definitions and usages, plus shared filter/rendering-usage helpers.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    FilterMember, RenderingDef, RenderingDefBody, RenderingDefBodyElement, RequirementDefBody,
    ViewBody, ViewBodyElement, ViewDef, ViewDefBody, ViewDefBodyElement, ViewRenderingUsage,
    ViewUsage, ViewpointDef, ViewpointUsage,
};
use sysml_v2_parser::Node;
use url::Url;

use super::requirement_body::walk_requirement_def_body;
use super::{
    add_node_and_recurse, insert_def_specialization_attr, qualified_name_for_node,
    wire_def_specialization_edge,
};
use crate::semantic::ast_util::{attach_short_name_attribute, identification_name, span_to_range};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::expressions;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::add_typing_edge_if_exists;

pub(super) fn add_view_filter_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    filter: &Node<FilterMember>,
    filter_owner_kind: &str,
) {
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        "_filter",
        "filter",
    );
    let mut attrs = HashMap::new();
    attrs.insert(
        "condition".to_string(),
        serde_json::json!(expressions::expression_to_debug_string(
            &filter.value.condition
        )),
    );
    attrs.insert(
        "conditionIsBoolean".to_string(),
        serde_json::json!(expressions::expression_is_boolean_valued(
            &filter.value.condition
        )),
    );
    attrs.insert(
        "exprClass".to_string(),
        serde_json::json!(expressions::classify_expression(&filter.value.condition).as_str()),
    );
    attrs.insert(
        "filterOwnerKind".to_string(),
        serde_json::json!(filter_owner_kind),
    );
    if let Some(vis) = &filter.value.visibility {
        attrs.insert(
            "visibility".to_string(),
            serde_json::json!(format!("{vis:?}")),
        );
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "filter",
        "_filter".to_string(),
        span_to_range(&filter.span),
        attrs,
        Some(parent_id),
    );
}

pub(super) fn add_view_rendering_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    rendering: &Node<ViewRenderingUsage>,
) {
    let vr = &rendering.value;
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &vr.name,
        "view rendering",
    );
    let mut attrs = HashMap::new();
    if let Some(ref rendering_type) = vr.type_name {
        attrs.insert(
            "renderingType".to_string(),
            serde_json::json!(rendering_type),
        );
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "view rendering",
        vr.name.clone(),
        span_to_range(&rendering.span),
        attrs,
        Some(parent_id),
    );
    if let Some(ref rendering_type) = vr.type_name {
        add_typing_edge_if_exists(
            g,
            uri,
            &qualified,
            rendering_type,
            Some(parent_id.qualified_name.as_str()),
        );
    }
}

fn annotate_rendering_def_body(
    g: &mut SemanticGraph,
    rendering_def_id: &NodeId,
    body: &RenderingDefBody,
    uri: &Url,
) {
    let RenderingDefBody::Brace { elements } = body else {
        return;
    };
    for element in elements {
        match &element.value {
            RenderingDefBodyElement::Filter(filter) => {
                add_view_filter_node(g, uri, rendering_def_id, filter, "rendering def");
            }
            RenderingDefBodyElement::ViewRendering(rendering) => {
                add_view_rendering_node(g, uri, rendering_def_id, rendering);
            }
            RenderingDefBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, rendering_def_id, &doc.value.text);
            }
            RenderingDefBodyElement::Error(_) | RenderingDefBodyElement::Other(_) => {}
        }
    }
}

fn annotate_view_usage_body(g: &mut SemanticGraph, view_id: &NodeId, body: &ViewBody, uri: &Url) {
    let ViewBody::Brace { elements } = body else {
        return;
    };
    if let Some(view_node) = g.get_node_mut(view_id) {
        view_node
            .attributes
            .insert("hasViewBody".to_string(), serde_json::json!(true));
    }
    let mut has_expose = false;
    let mut expose_targets = Vec::new();
    for element in elements {
        match &element.value {
            ViewBodyElement::Expose(expose) => {
                has_expose = true;
                expose_targets.push(serde_json::json!({
                    "target": expose.target,
                    "range": crate::semantic::ast_util::text_range_to_json(
                        crate::semantic::ast_util::span_to_range(&element.span),
                    ),
                }));
            }
            ViewBodyElement::ViewRendering(rendering) => {
                add_view_rendering_node(g, uri, view_id, rendering);
            }
            ViewBodyElement::Filter(filter) => {
                add_view_filter_node(g, uri, view_id, filter, "view");
            }
            ViewBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, view_id, &doc.value.text);
            }
            ViewBodyElement::Error(_) | ViewBodyElement::Other(_) | ViewBodyElement::Satisfy(_) => {
            }
        }
    }
    if has_expose {
        if let Some(view_node) = g.get_node_mut(view_id) {
            view_node
                .attributes
                .insert("hasExpose".to_string(), serde_json::json!(true));
            view_node.attributes.insert(
                "exposeTargets".to_string(),
                serde_json::json!(expose_targets),
            );
        }
    }
}

pub(super) fn build_view_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    vd_node: &Node<ViewDef>,
) {
    let name = identification_name(&vd_node.value.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "view def");
    let range = span_to_range(&vd_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &vd_node.value.identification);
    insert_def_specialization_attr(&mut attrs, vd_node.value.specializes.as_deref());
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "view def",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    let view_def_id = NodeId::new(uri, &qualified);
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        vd_node.value.specializes.as_deref(),
    );
    if let ViewDefBody::Brace { elements } = &vd_node.value.body {
        for element in elements {
            match &element.value {
                ViewDefBodyElement::Filter(filter) => {
                    add_view_filter_node(g, uri, &view_def_id, filter, "view def");
                }
                ViewDefBodyElement::ViewRendering(rendering) => {
                    add_view_rendering_node(g, uri, &view_def_id, rendering);
                }
                ViewDefBodyElement::Doc(doc) => {
                    super::attach_doc_comment(g, &view_def_id, &doc.value.text);
                }
                ViewDefBodyElement::MetadataAnnotation(meta) => {
                    super::metadata_def::add_metadata_annotation_node(
                        g,
                        uri,
                        container_prefix,
                        &view_def_id,
                        &meta.value,
                        &meta.span,
                    );
                }
                ViewDefBodyElement::Error(_) | ViewDefBodyElement::Other(_) => {}
            }
        }
    }
}

pub(super) fn build_viewpoint_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    vpd_node: &Node<ViewpointDef>,
) {
    let name = identification_name(&vpd_node.value.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "viewpoint def");
    let range = span_to_range(&vpd_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &vpd_node.value.identification);
    insert_def_specialization_attr(&mut attrs, vpd_node.value.specializes.as_deref());
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "viewpoint def",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    let viewpoint_def_id = NodeId::new(uri, &qualified);
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        vpd_node.value.specializes.as_deref(),
    );
    if let RequirementDefBody::Brace { .. } = &vpd_node.value.body {
        walk_requirement_def_body(
            g,
            uri,
            container_prefix,
            &qualified,
            &viewpoint_def_id,
            &vpd_node.value.body,
        );
    }
}

pub(super) fn build_rendering_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    rd_node: &Node<RenderingDef>,
) {
    let name = identification_name(&rd_node.value.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "rendering def");
    let range = span_to_range(&rd_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &rd_node.value.identification);
    insert_def_specialization_attr(&mut attrs, rd_node.value.specializes.as_deref());
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "rendering def",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        rd_node.value.specializes.as_deref(),
    );
    let rendering_def_id = NodeId::new(uri, &qualified);
    annotate_rendering_def_body(g, &rendering_def_id, &rd_node.value.body, uri);
}

pub(super) fn build_view_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    vu_node: &Node<ViewUsage>,
) {
    let name = &vu_node.value.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "view");
    let range = span_to_range(&vu_node.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = vu_node.value.type_name {
        attrs.insert("viewType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "view",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    let view_id = NodeId::new(uri, &qualified);
    if let Some(ref t) = vu_node.value.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    annotate_view_usage_body(g, &view_id, &vu_node.value.body, uri);
}

pub(super) fn build_viewpoint_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    vpu_node: &Node<ViewpointUsage>,
) {
    let name = &vpu_node.value.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "viewpoint");
    let range = span_to_range(&vpu_node.span);
    let mut attrs = HashMap::new();
    attrs.insert(
        "viewpointType".to_string(),
        serde_json::json!(vpu_node.value.type_name.as_str()),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "viewpoint",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    add_typing_edge_if_exists(
        g,
        uri,
        &qualified,
        vpu_node.value.type_name.as_str(),
        container_prefix,
    );
    let viewpoint_id = NodeId::new(uri, &qualified);
    walk_requirement_def_body(
        g,
        uri,
        container_prefix,
        &qualified,
        &viewpoint_id,
        &vpu_node.value.body,
    );
}

pub(super) fn build_rendering_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    ru_node: &Node<sysml_v2_parser::ast::RenderingUsage>,
) {
    let name = &ru_node.value.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "rendering");
    let range = span_to_range(&ru_node.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = ru_node.value.type_name {
        attrs.insert("renderingType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "rendering",
        name.clone(),
        range,
        attrs,
        parent_id,
    );
    if let Some(ref t) = ru_node.value.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
}

pub(super) fn build_filter_member(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    f: &Node<FilterMember>,
) {
    let Some(pid) = parent_id else {
        return;
    };
    let qualified = qualified_name_for_node(g, uri, container_prefix, "_filter", "filter");
    let mut attrs = HashMap::new();
    attrs.insert(
        "condition".to_string(),
        serde_json::json!(expressions::expression_to_debug_string(&f.value.condition)),
    );
    attrs.insert(
        "conditionIsBoolean".to_string(),
        serde_json::json!(expressions::expression_is_boolean_valued(
            &f.value.condition
        )),
    );
    attrs.insert(
        "exprClass".to_string(),
        serde_json::json!(expressions::classify_expression(&f.value.condition).as_str()),
    );
    if let Some(vis) = &f.value.visibility {
        attrs.insert(
            "visibility".to_string(),
            serde_json::json!(format!("{vis:?}")),
        );
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "filter",
        "_filter".to_string(),
        span_to_range(&f.span),
        attrs,
        Some(pid),
    );
}
