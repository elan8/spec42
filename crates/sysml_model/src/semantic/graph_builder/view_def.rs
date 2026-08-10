//! View/viewpoint/rendering definitions and usages, plus shared filter/rendering-usage helpers.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ExposeMember, FilterMember, RenderingDef, RenderingDefBody, RenderingDefBodyElement,
    RenderingUsageBody, RenderingUsageBodyElement, RequirementDefBody, ViewBody, ViewBodyElement,
    ViewDef, ViewDefBody, ViewDefBodyElement, ViewRenderingUsage, ViewUsage, ViewpointDef,
    ViewpointUsage,
};
use sysml_v2_parser::Node;
use url::Url;

use super::requirement_body::{import_member_label, walk_requirement_def_body};
use super::{
    add_node_and_recurse, insert_def_specialization_attr, qualified_name_for_node,
    wire_def_specialization_edge,
};
use crate::semantic::ast_util::{
    attach_short_name_attribute, declared_expression, identification_name, span_to_range,
    subsetting_target,
};
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
    let node_id = NodeId::new(uri, &qualified);
    if let Some(node) = g.get_node_mut(&node_id) {
        node.declared_facts.own_expression = Some(declared_expression(&filter.value.condition));
    }
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&vr.membership),
    );
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
    let node_id = NodeId::new(uri, &qualified);
    walk_rendering_usage_body(g, uri, &node_id, &vr.body);
}

/// Walks a `render`/`rendering` usage body for nested `view` usage members -- most notably a
/// `columnView` redefinition of `asElementTable` (`view :>> columnView[N] { render ...; }`, the
/// Systems Library's GridView element-table column-configuration mechanism, `Views.sysml`'s
/// `view columnView[0..*] ordered { ... }`). Shared by both rendering-usage builders
/// ([`add_view_rendering_node`] for the inline `render` form,
/// [`build_rendering_usage`](super::build_rendering_usage) for the standalone `rendering` form)
/// since both AST nodes carry the same `RenderingUsageBody` shape.
fn walk_rendering_usage_body(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    body: &RenderingUsageBody,
) {
    let RenderingUsageBody::Brace { elements } = body else {
        return;
    };
    for element in elements {
        match &element.value {
            RenderingUsageBodyElement::ViewUsage(column) => {
                add_view_column_node(g, uri, parent_id, column);
            }
            RenderingUsageBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            RenderingUsageBodyElement::Error(_) => {}
        }
    }
}

/// Materializes a nested `view :>> columnView[N] { render <renderingName>; }` redefinition inside
/// a `render`/`rendering` usage body as a `view column` child node -- captures which feature it
/// redefines (`columnView`), its declared index, and the rendering it applies to each row element
/// (the nested `render` binding's name, e.g. `asTextualNotation`), in declaration order (the
/// stdlib defines `columnView[0..*] ordered`, so order is semantically meaningful -- this walks
/// `elements` in their parsed order, never re-sorted).
fn add_view_column_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    column: &Node<ViewUsage>,
) {
    let cv = &column.value;
    let redefines_name = subsetting_target(cv.redefines.as_deref());
    let base_label = redefines_name.unwrap_or("_columnView");
    // Multiple `columnView[N]` redefinitions in one table would otherwise all display the same
    // base label (the redefined feature name is always `columnView`, per §Views.sysml's stdlib
    // definition) -- the index disambiguates them for display, purely a label concern (qualified
    // name uniqueness is already handled independently by `qualified_name_for_node`'s own
    // collision suffix below).
    let index_text = cv.multiplicity.as_ref().and_then(|m| {
        m.lower
            .as_ref()
            .map(|lower| expressions::expression_to_debug_string(lower))
    });
    let label = match index_text {
        Some(index) => format!("{base_label}[{index}]"),
        None => base_label.to_string(),
    };
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &label,
        "view column",
    );
    let mut attrs = HashMap::new();
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&cv.membership),
    );
    if let Some(redefines) = redefines_name {
        attrs.insert("redefines".to_string(), serde_json::json!(redefines));
    }
    if let Some(ref multiplicity) = cv.multiplicity {
        attrs.insert("multiplicity".to_string(), serde_json::json!(multiplicity));
    }
    if let ViewBody::Brace { elements } = &cv.body {
        if let Some(rendering_name) = elements.iter().find_map(|element| match &element.value {
            ViewBodyElement::ViewRendering(rendering) => Some(rendering.value.name.clone()),
            _ => None,
        }) {
            attrs.insert(
                "renderingType".to_string(),
                serde_json::json!(rendering_name),
            );
        }
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "view column",
        label,
        span_to_range(&column.span),
        attrs,
        Some(parent_id),
    );
    super::attach_declared_subsetting_family(
        g,
        &NodeId::new(uri, &qualified),
        None,
        cv.redefines.as_deref(),
        None,
        None,
    );
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

/// Materializes an `expose` member as a real `import`-kind node, reusing the exact attribute
/// shape `materialize_import` uses for ordinary `import` statements (`importTarget`/`importAll`/
/// `recursive`) -- `expose` is normatively an Import per `ExposeMember`'s own BNF doc comment
/// (`MembershipImport = QualifiedName (::**)?`, `NamespaceImport = QualifiedName :: * (::**)?`).
/// Reusing the "import" kind string means the node automatically flows through the existing
/// `membership_kind`/`membership_relationship_metaclass` pipeline
/// (`HostMembershipKind::Import` -> `HostRelationshipMetaclass::NamespaceImport`/
/// `::MembershipImport` based on the `importAll` attribute) with no new classification logic.
pub(super) fn materialize_expose_member(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    expose: &Node<ExposeMember>,
) {
    let v = &expose.value;
    let name = import_member_label(&v.target);
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &name,
        "import",
    );
    let attrs = HashMap::new();
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_expose_membership_facts(expose),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "import",
        name,
        span_to_range(&expose.span),
        attrs,
        Some(parent_id),
    );
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
                    "target": expose.value.target,
                    "range": crate::semantic::ast_util::text_range_to_json(
                        crate::semantic::ast_util::span_to_range(&element.span),
                    ),
                }));
                materialize_expose_member(g, uri, view_id, expose);
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&vd_node.value.membership),
    );
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&vpd_node.value.membership),
    );
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&rd_node.value.membership),
    );
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&vu_node.value.membership),
    );
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&vpu_node.value.membership),
    );
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
    g.register_declared_membership_facts(
        NodeId::new(uri, &qualified),
        crate::semantic::ast_util::declared_membership_facts(&ru_node.value.membership),
    );
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
    let node_id = NodeId::new(uri, &qualified);
    walk_rendering_usage_body(g, uri, &node_id, &ru_node.value.body);
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
    let node_id = NodeId::new(uri, &qualified);
    if let Some(node) = g.get_node_mut(&node_id) {
        node.declared_facts.own_expression = Some(declared_expression(&f.value.condition));
    }
}
