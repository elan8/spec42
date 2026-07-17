use std::collections::HashMap;

use sysml_v2_parser::ast::{
    CalcDefBody, CalcDefBodyElement, InterfaceDefBody, PartDefBody, PartDefBodyElement,
};
use url::Url;

use crate::semantic::ast_util::{
    attach_short_name_attribute, declared_multiplicity, definition_feature_properties,
    identification_name, span_to_range,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{DeclaredFeatureProperties, NodeId, RelationshipKind};
use crate::semantic::relationships::{
    add_edge_if_both_exist, add_specializes_edge_if_exists, add_typing_edge_if_exists,
};

use super::attribute_body;
use super::expressions;
use super::interface_def;
use super::port_def::materialize_port_usage;
use super::state;
use super::usage_builders;
use super::{add_node_and_recurse, attach_feature_properties, qualified_name_for_node};

pub(super) fn build_from_part_def_body_element(
    node: &sysml_v2_parser::Node<PartDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PartDefBodyElement as PDBE;
    match &node.value {
        PDBE::AttributeDef(n) => {
            let name = &n.name;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name, "attribute def");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = n.typing {
                attrs.insert("attributeType".to_string(), serde_json::json!(t));
            }
            if let Some(ref v) = n.value.value {
                let rendered = expressions::expression_to_debug_string(&v.value.expression);
                attrs.insert("value".to_string(), serde_json::json!(rendered));
                attrs.insert("defaultValue".to_string(), serde_json::json!(rendered));
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
            let node_id = NodeId::new(uri, &qualified);
            attach_feature_properties(
                g,
                &node_id,
                DeclaredFeatureProperties {
                    is_ordered: Some(n.ordered),
                    is_unique: Some(!n.nonunique),
                    ..DeclaredFeatureProperties::default()
                },
            );
            if let Some(ref t) = n.typing {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PDBE::AttributeUsage(n) => {
            usage_builders::materialize_attribute_usage(n, uri, container_prefix, parent_id, g);
        }
        PDBE::ExhibitState(es_node) => {
            let es = &es_node.value;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &es.name, "exhibit state");
            let range = span_to_range(&es_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref state_type) = es.type_name {
                attrs.insert("stateType".to_string(), serde_json::json!(state_type));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "exhibit state",
                es.name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref state_type) = es.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, state_type, container_prefix);
            }
            let exhibit_state_id = NodeId::new(uri, &qualified);
            if let sysml_v2_parser::ast::StateDefBody::Brace { elements } = &es.body {
                state::build_from_state_body(elements, uri, Some(&qualified), &exhibit_state_id, g);
            }
        }
        PDBE::PortUsage(n) => {
            materialize_port_usage(n, uri, container_prefix, parent_id, g);
        }
        PDBE::PartDef(pd_node) => {
            let name = identification_name(&pd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "part def");
            let range = span_to_range(&pd_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &pd_node.identification);
            if let Some(ref p) = pd_node.definition_prefix {
                attrs.insert(
                    "definitionPrefix".to_string(),
                    serde_json::json!(match p {
                        sysml_v2_parser::ast::DefinitionPrefix::Abstract => "abstract",
                        sysml_v2_parser::ast::DefinitionPrefix::Variation => "variation",
                    }),
                );
            }
            if let Some(ref s) = pd_node.specializes {
                attrs.insert("specializes".to_string(), serde_json::json!(s));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "part def",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            let node_id = NodeId::new(uri, &qualified);
            attach_feature_properties(
                g,
                &node_id,
                definition_feature_properties(
                    pd_node.definition_prefix.as_ref(),
                    pd_node.is_individual,
                ),
            );
            if let Some(ref s) = pd_node.specializes {
                add_specializes_edge_if_exists(g, uri, &qualified, s, container_prefix);
            }
            if let PartDefBody::Brace { elements } = &pd_node.body {
                for child in elements {
                    build_from_part_def_body_element(child, uri, Some(&qualified), &node_id, g);
                }
            }
        }
        PDBE::PartUsage(n) => {
            usage_builders::materialize_part_usage(n, uri, container_prefix, Some(parent_id), g);
        }
        PDBE::OccurrenceUsage(occ_node) => {
            usage_builders::materialize_occurrence_usage(
                occ_node,
                uri,
                container_prefix,
                Some(parent_id),
                g,
            );
        }
        PDBE::ItemDef(item_node) => {
            let name = identification_name(&item_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "item def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &item_node.identification);
                if let Some(ref s) = item_node.specializes {
                    attrs.insert("specializes".to_string(), serde_json::json!(s));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "item def",
                    name,
                    span_to_range(&item_node.span),
                    attrs,
                    Some(parent_id),
                );
                if let Some(ref s) = item_node.specializes {
                    add_specializes_edge_if_exists(g, uri, &qualified, s, container_prefix);
                }
                let node_id = NodeId::new(uri, &qualified);
                attribute_body::build_from_attribute_body(
                    &item_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PDBE::ItemUsage(item_node) => {
            let name = &item_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "item");
            let range = span_to_range(&item_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = item_node.type_name {
                attrs.insert("itemType".to_string(), serde_json::json!(t));
            }
            if let Some(ref m) = item_node.multiplicity {
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
            let node_id = NodeId::new(uri, &qualified);
            if let Some(multiplicity) = &item_node.multiplicity {
                if let Some(node) = g.get_node_mut(&node_id) {
                    node.declared_facts.multiplicity =
                        Some(declared_multiplicity(multiplicity, false));
                }
            }
            if let Some(ref t) = item_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            attribute_body::build_from_attribute_body(
                &item_node.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PDBE::RequirementUsage(ru_node) => {
            usage_builders::materialize_requirement_usage(
                ru_node,
                uri,
                container_prefix,
                Some(parent_id),
                g,
            );
        }
        PDBE::Connect(c) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                crate::semantic::ast_util::connection_end_expression(&c.from),
                crate::semantic::ast_util::connection_end_expression(&c.to),
                RelationshipKind::Connection,
            );
        }
        PDBE::FlowUsage(flow) => {
            super::flow_usage::materialize_flow_usage(flow, uri, container_prefix, parent_id, g);
        }
        PDBE::InterfaceUsage(interface_usage) => {
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
        PDBE::InterfaceDef(id_node) => {
            let name = identification_name(&id_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "interface def");
            let range = span_to_range(&id_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &id_node.identification);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "interface def",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            let iface_id = NodeId::new(uri, &qualified);
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                interface_def::build_from_interface_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &iface_id,
                    g,
                );
            }
        }
        PDBE::Connection(connection_usage) => {
            let connection = &connection_usage.value;
            let name = connection
                .name
                .as_deref()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("_connection");
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "connection");
            let mut attrs = HashMap::new();
            if let Some(ref subsets) = connection.subsets {
                attrs.insert("subsetsFeature".to_string(), serde_json::json!(subsets));
            }
            if let Some(ref redefines) = connection.redefines {
                attrs.insert("redefines".to_string(), serde_json::json!(redefines));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "connection",
                name.to_string(),
                span_to_range(&connection_usage.span),
                attrs,
                Some(parent_id),
            );
            if let Some(ref type_name) = connection.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
            }
            let connection_node_id = NodeId::new(uri, &qualified);
            if let sysml_v2_parser::ast::ConnectionDefBody::Brace { elements } = &connection.body {
                super::interface_def::build_from_connection_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &connection_node_id,
                    g,
                );
            }
        }
        PDBE::CalcUsage(calc_node) => {
            let name = identification_name(&calc_node.value.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "calc");
            let range = span_to_range(&calc_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &calc_node.value.identification);
            if let Some(ref t) = calc_node.value.type_name {
                attrs.insert("calcType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "calc",
                name,
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = calc_node.value.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let CalcDefBody::Brace { elements } = &calc_node.value.body {
                let calc_node_id = NodeId::new(uri, &qualified);
                for element in elements {
                    match &element.value {
                        CalcDefBodyElement::InOutDecl(in_out) => {
                            super::action::add_in_out_decl(
                                g,
                                uri,
                                container_prefix,
                                &calc_node_id,
                                in_out,
                            );
                        }
                        CalcDefBodyElement::ReturnDecl(ret) => {
                            let ret_qualified = qualified_name_for_node(
                                g,
                                uri,
                                container_prefix,
                                &ret.value.name,
                                "return parameter",
                            );
                            let mut attrs = HashMap::new();
                            attrs.insert("direction".to_string(), serde_json::json!("return"));
                            attrs.insert(
                                "parameterType".to_string(),
                                serde_json::json!(&ret.value.type_name),
                            );
                            add_node_and_recurse(
                                g,
                                uri,
                                &ret_qualified,
                                "return parameter",
                                ret.value.name.clone(),
                                span_to_range(&ret.span),
                                attrs,
                                Some(&calc_node_id),
                            );
                            add_typing_edge_if_exists(
                                g,
                                uri,
                                &ret_qualified,
                                &ret.value.type_name,
                                container_prefix,
                            );
                        }
                        CalcDefBodyElement::Doc(doc) => {
                            super::attach_doc_comment(g, &calc_node_id, &doc.value.text);
                        }
                        CalcDefBodyElement::MetadataAnnotation(meta) => {
                            super::metadata_def::add_metadata_annotation_node(
                                g,
                                uri,
                                container_prefix,
                                &calc_node_id,
                                &meta.value,
                                &meta.span,
                            );
                        }
                        CalcDefBodyElement::Expression(_)
                        | CalcDefBodyElement::Other(_)
                        | CalcDefBodyElement::Error(_) => {}
                    }
                }
            }
        }
        // A `case`/`case def` nested inside a `part def { ... }` body was previously dropped
        // entirely -- no dispatch arm existed here, unlike the sibling `PDBE::CalcUsage` arm
        // above. Reuse the same `materialize_case_def`/`materialize_case_usage` builders the
        // package-level `PBE::CaseDef`/`PBE::CaseUsage` dispatch already calls
        // (`package_body::materialize`, re-exported `pub(crate)` from `package_body/mod.rs`).
        PDBE::CaseDef(c_node) => {
            super::package_body::materialize_case_def(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                c_node,
            );
        }
        PDBE::CaseUsage(c_node) => {
            super::package_body::materialize_case_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                c_node,
            );
        }
        PDBE::Perform(perform_node) => {
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
        PDBE::Allocate(allocate_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &allocate_node.source,
                &allocate_node.target,
                RelationshipKind::Allocate,
            );
        }
        PDBE::Ref(r) => {
            super::ref_decl::materialize_ref_decl(
                g,
                uri,
                container_prefix,
                parent_id,
                r,
                super::ref_decl::RefDeclOptions {
                    wire_value_reference: true,
                },
            );
        }
        PDBE::MetadataKeywordUsage(mk_node) => {
            super::metadata_keyword::add_metadata_keyword_node(
                g,
                uri,
                parent_id,
                &mk_node.value,
                &mk_node.span,
            );
        }
        PDBE::EnumerationUsage(enum_node) => {
            let name = &enum_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "enumeration");
            let range = span_to_range(&enum_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = enum_node.type_name {
                attrs.insert("enumerationType".to_string(), serde_json::json!(t));
            }
            if let Some(ref m) = enum_node.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "enumeration",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            if let Some(ref t) = enum_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            let node_id = NodeId::new(uri, &qualified);
            attribute_body::build_from_attribute_body(
                &enum_node.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PDBE::OpaqueMember(opaque_node) => {
            let opaque = &opaque_node.value;
            let name = if opaque.name.trim().is_empty() {
                format!("_opaque_{}", opaque.keyword)
            } else {
                opaque.name.clone()
            };
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "opaque member");
            let mut attrs = HashMap::new();
            attrs.insert("keyword".to_string(), serde_json::json!(opaque.keyword));
            attrs.insert("text".to_string(), serde_json::json!(opaque.text));
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "opaque member",
                name,
                span_to_range(&opaque_node.span),
                attrs,
                Some(parent_id),
            );
            let node_id = NodeId::new(uri, &qualified);
            attribute_body::build_from_attribute_body(
                &opaque.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PDBE::MetadataAnnotation(meta) => {
            super::metadata_def::add_metadata_annotation_node(
                g,
                uri,
                container_prefix,
                parent_id,
                &meta.value,
                &meta.span,
            );
        }
        PDBE::Doc(doc) => {
            super::attach_doc_comment(g, parent_id, &doc.value.text);
        }
        PDBE::Annotation(_) | PDBE::Error(_) | PDBE::Comment(_) | PDBE::Other(_) => {}
        // Not yet modeled in the semantic graph.
        PDBE::AssertConstraint(_) | PDBE::Satisfy(_) => {}
        PDBE::VariantUsage(n) => {
            usage_builders::materialize_variant_usage(n, uri, container_prefix, parent_id, g);
        }
        _ => {}
    }
}
