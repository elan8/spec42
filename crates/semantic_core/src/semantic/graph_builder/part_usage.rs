use std::collections::HashMap;

use sysml_v2_parser::ast::{PartUsageBody, StateDefBody};
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::reference_resolution::{resolve_member_via_type, ResolveResult};
use crate::semantic::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::expressions;
use super::port_def::materialize_port_usage;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_from_part_usage_body_element(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => {
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
        }
        PUBE::PartUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref prefix) = n.usage_prefix {
                attrs.insert(
                    "usagePrefix".to_string(),
                    serde_json::json!(match prefix {
                        sysml_v2_parser::ast::DefinitionPrefix::Abstract => "abstract",
                        sysml_v2_parser::ast::DefinitionPrefix::Variation => "variation",
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
                Some(parent_id),
            );
            let node_id = NodeId::new(uri, &qualified);
            add_typing_edge_if_exists(g, uri, &qualified, &n.type_name, container_prefix);
            if let PartUsageBody::Brace { elements } = &n.body {
                for child in elements {
                    build_from_part_usage_body_element(child, uri, Some(&qualified), &node_id, g);
                }
            }
        }
        PUBE::PortUsage(n) => {
            materialize_port_usage(n, uri, container_prefix, parent_id, g);
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
            let value_expression = n
                .value
                .as_ref()
                .map(expressions::expression_to_debug_string);
            if let Some(ref v) = value_expression {
                attrs.insert("value".to_string(), serde_json::json!(v));
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
            if let Some(value_expression) = value_expression.as_deref() {
                if let Some(target) = expressions::resolve_expression_endpoint_legacy(
                    g,
                    uri,
                    container_prefix,
                    value_expression,
                ) {
                    add_edge_if_both_exist(
                        g,
                        uri,
                        &qualified,
                        &target,
                        RelationshipKind::Reference,
                    );
                }
            }
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
            super::metadata_def::add_metadata_annotation_node(
                g,
                uri,
                container_prefix,
                parent_id,
                &meta.value,
                &meta.span,
            );
        }
        PUBE::OccurrenceUsage(occ_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &occ_node.name, "occurrence");
            let range = span_to_range(&occ_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = occ_node.type_name {
                attrs.insert("occurrenceType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "occurrence",
                occ_node.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = occ_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PUBE::MetadataKeywordUsage(mk_node) => {
            super::metadata_keyword::add_metadata_keyword_node(
                g,
                uri,
                parent_id,
                &mk_node.value,
                &mk_node.span,
            );
        }
        PUBE::VariantUsage(variant) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &variant.name, "variant");
            let range = span_to_range(&variant.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "variant",
                variant.name.clone(),
                range,
                HashMap::new(),
                Some(parent_id),
            );
        }
        PUBE::EnumerationUsage(_) | PUBE::Annotation(_) | PUBE::Error(_) | PUBE::Doc(_) => {}
    }
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
                if target.element_kind == "port" {
                    "port"
                } else {
                    "attribute"
                }
            })
            .unwrap_or("attribute"),
        ResolveResult::Ambiguous | ResolveResult::Unresolved => "attribute",
    }
}
