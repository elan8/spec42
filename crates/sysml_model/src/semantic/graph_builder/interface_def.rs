//! Interface and connection definition bodies: `end`, `ref`, `connect`.

use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ConnectStmt, ConnectionDefBodyElement, EndDecl, InterfaceDefBodyElement, RefDecl,
};
use url::Url;

use crate::semantic::ast_util::{
    connection_end_expression, declared_multiplicity, span_to_range, subsetting_target,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{DeclaredFeatureProperties, ElementKind, NodeId, RelationshipKind};
use crate::semantic::relationships::{
    add_edge_if_both_exist, add_typing_edge_if_exists, try_wire_derivation_connection,
};

use super::expressions;
use super::{
    add_node_and_recurse, attach_declared_subsetting_family, attach_feature_properties,
    qualified_name_for_node,
};
use crate::semantic::resolution::resolve_expression_endpoint_qualified;

pub(super) fn add_end_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    wrap: &sysml_v2_parser::Node<EndDecl>,
) {
    let n = &wrap.value;
    let range = span_to_range(&wrap.span);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &n.name, "interface end");
    let mut attrs = HashMap::new();
    if let Some(reference_target) = subsetting_target(n.references.as_deref()) {
        // spec42#1/#2: `::>`/`references` names a reference (KerML `ReferenceSubsetting`), not a
        // type. Previously this always also set `portType`, which feeds the generic
        // type-resolution diagnostics/edges (`unresolved_type_reference`,
        // `add_typing_edge_if_exists`) and misfired treating the referenced feature's name as an
        // unresolved type. Set `referencesFeature` instead, which
        // `link_subsetting_family_edges_for_node` already resolves into a proper
        // `ReferenceSubsetting` edge the same way attribute/occurrence usages' `references`
        // clauses do.
        attrs.insert(
            "referencesFeature".to_string(),
            serde_json::json!(reference_target),
        );
    } else {
        attrs.insert("portType".to_string(), serde_json::json!(&n.type_name));
    }
    if let Some(target) = subsetting_target(n.redefines.as_deref()) {
        attrs.insert("redefines".to_string(), serde_json::json!(target));
    }
    if let Some(target) = subsetting_target(n.crosses.as_deref()) {
        attrs.insert("crossesFeature".to_string(), serde_json::json!(target));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "interface end",
        n.name.clone(),
        range,
        attrs,
        Some(parent_id),
    );
    attach_feature_properties(
        g,
        &NodeId::new(uri, &qualified),
        DeclaredFeatureProperties {
            is_end: true,
            ..DeclaredFeatureProperties::default()
        },
    );
    attach_declared_subsetting_family(
        g,
        &NodeId::new(uri, &qualified),
        None,
        n.redefines.as_deref(),
        n.references.as_deref(),
        n.crosses.as_deref(),
    );
    if let Some(multiplicity) = &n.multiplicity {
        if let Some(node) = g.get_node_mut(&NodeId::new(uri, &qualified)) {
            node.declared_facts.multiplicity = Some(declared_multiplicity(multiplicity, false));
        }
    }
    // A typed end can also carry a trailing `::>` reference-subsetting clause.  The AST's
    // `references` field represents both that form and the reference-only form, so the syntax
    // discriminator is the only sound way to decide whether a typing edge is authored.
    if !n.uses_derived_syntax {
        add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
    }
    // Recorded separately from `DeclaredRelationshipFacts::typing` (which several unresolved-type
    // diagnostics walk directly): interface ends resolve leniently through
    // `resolve_expression_endpoint_qualified`/`resolve_type_target_in_workspace`, and folding this
    // into the generic typing-fact vector would make those diagnostics fire an extra
    // `unresolved_type_reference` alongside `interface_end_invalid` for every unresolved end.
    if let Some(node) = g.get_node_mut(&NodeId::new(uri, &qualified)) {
        node.declared_facts.interface_end_type = Some(n.type_name.clone());
    }
}

fn add_ref_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    wrap: &sysml_v2_parser::Node<RefDecl>,
) {
    super::ref_decl::materialize_ref_decl(
        g,
        uri,
        container_prefix,
        parent_id,
        wrap,
        super::ref_decl::RefDeclOptions::default(),
    );
}

fn add_connect_stmt(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    wrap: &sysml_v2_parser::Node<ConnectStmt>,
) {
    let n = &wrap.value;
    expressions::add_expression_edge_if_both_exist(
        g,
        uri,
        container_prefix,
        connection_end_expression(&n.from),
        connection_end_expression(&n.to),
        RelationshipKind::Connection,
    );
}

fn maybe_add_derivation_edge(g: &mut SemanticGraph, uri: &Url, parent_id: &NodeId) {
    try_wire_derivation_connection(g, uri, parent_id);
}

pub(super) fn add_connection_edges_from_end_typing(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
) {
    if g.structural_input_only {
        return;
    }
    let Some(parent) = g.get_node(parent_id) else {
        return;
    };
    if parent.element_kind == ElementKind::DerivationConnection {
        return;
    }
    let scope_prefix = parent
        .parent_id
        .as_ref()
        .map(|id| id.qualified_name.as_str());
    let mut end_targets: Vec<String> = g
        .children_of(parent)
        .into_iter()
        .filter(|child| child.element_kind == ElementKind::InterfaceEnd)
        .filter_map(|child| {
            g.outgoing_targets_by_kind(child, RelationshipKind::Typing)
                .into_iter()
                .next()
                .map(|target| target.id.qualified_name.clone())
                .or_else(|| {
                    child
                        .declared_facts
                        .declared_end_reference()
                        .and_then(|end_type| {
                            resolve_expression_endpoint_qualified(g, uri, scope_prefix, end_type)
                        })
                })
        })
        .collect();
    if end_targets.len() < 2 {
        return;
    }
    let mut seen = std::collections::HashSet::new();
    end_targets.retain(|target| seen.insert(target.clone()));
    if end_targets.len() < 2 {
        return;
    }

    // Binary connections emit one edge; n-ary connections use the first end as hub.
    let source = end_targets[0].clone();
    for target in end_targets.into_iter().skip(1) {
        let _ = add_edge_if_both_exist(g, uri, &source, &target, RelationshipKind::Connection);
    }
}

pub(super) fn build_from_interface_def_body(
    elements: &[sysml_v2_parser::Node<InterfaceDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    for element in elements {
        build_from_interface_def_body_element(element, uri, container_prefix, parent_id, g);
    }
    add_connection_edges_from_end_typing(g, uri, parent_id);
}

pub(super) fn build_from_interface_def_body_element(
    node: &sysml_v2_parser::Node<InterfaceDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::InterfaceDefBodyElement as E;
    match &node.value {
        E::Doc(doc) => {
            super::attach_doc_comment(g, parent_id, &doc.value.text);
        }
        E::EndDecl(w) => add_end_decl(g, uri, container_prefix, parent_id, w),
        E::RefDecl(w) => add_ref_decl(g, uri, container_prefix, parent_id, w),
        E::ConnectStmt(w) => add_connect_stmt(g, uri, container_prefix, w),
        E::AttributeDef(attribute) => super::package_body::materialize_attribute_def(
            g,
            uri,
            container_prefix,
            Some(parent_id),
            attribute,
        ),
        E::AttributeUsage(attribute) => {
            super::usage_builders::materialize_attribute_usage(
                attribute,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        E::ItemDef(item) => super::package_body::materialize_item_def(
            g,
            uri,
            container_prefix,
            Some(parent_id),
            item,
        ),
        E::ItemUsage(_) | E::PortUsage(_) | E::Error(_) => {}
        E::PortDef(port) => super::package_body::materialize_port_def(
            g,
            uri,
            container_prefix,
            Some(parent_id),
            port,
        ),
        // GH-85 (sysml-v2-parser): bare `flow <a> to <b>;` shorthand connecting two of this
        // interface's own ends.
        E::FlowUsage(flow) => {
            super::flow_usage::materialize_flow_usage(flow, uri, container_prefix, parent_id, g);
        }
    }
}

pub(super) fn build_from_connection_def_body_element(
    node: &sysml_v2_parser::Node<ConnectionDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::ConnectionDefBodyElement as E;
    match &node.value {
        E::EndDecl(w) => {
            add_end_decl(g, uri, container_prefix, parent_id, w);
            maybe_add_derivation_edge(g, uri, parent_id);
        }
        E::RefDecl(w) => add_ref_decl(g, uri, container_prefix, parent_id, w),
        E::ConnectStmt(w) => add_connect_stmt(g, uri, container_prefix, w),
        E::Doc(doc) => super::attach_doc_comment(g, parent_id, &doc.value.text),
        E::AttributeDef(attribute) => super::package_body::materialize_attribute_def(
            g,
            uri,
            container_prefix,
            Some(parent_id),
            attribute,
        ),
        E::AttributeUsage(attribute) => {
            super::usage_builders::materialize_attribute_usage(
                attribute,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        E::ItemDef(item) => super::package_body::materialize_item_def(
            g,
            uri,
            container_prefix,
            Some(parent_id),
            item,
        ),
        E::PortDef(port) => super::package_body::materialize_port_def(
            g,
            uri,
            container_prefix,
            Some(parent_id),
            port,
        ),
        E::ItemUsage(_)
        | E::PortUsage(_)
        | E::AssertConstraint(_)
        | E::OccurrenceUsage(_)
        | E::SuccessionUsage(_)
        | E::Error(_) => {}
    }
}

pub(super) fn build_from_connection_def_body(
    elements: &[sysml_v2_parser::Node<ConnectionDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    for element in elements {
        build_from_connection_def_body_element(element, uri, container_prefix, parent_id, g);
    }
    add_connection_edges_from_end_typing(g, uri, parent_id);
}
