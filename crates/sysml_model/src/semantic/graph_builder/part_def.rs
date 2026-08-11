use std::collections::HashMap;

use sysml_v2_parser::ast::{InterfaceDefBody, PartDefBody, PartDefBodyElement};
use url::Url;

use crate::semantic::ast_util::{
    definition_feature_properties, identification_name, span_to_range, typing_targets,
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
use super::{
    add_node_and_recurse, attach_declared_typing_relationship, attach_feature_properties,
    qualified_name_for_node, resolve_addressable_name,
};

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
            g.register_declared_membership_facts(
                NodeId::new(uri, &qualified),
                crate::semantic::ast_util::declared_membership_facts(&n.membership),
            );
            if let Some(ref t) = n.typing {
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
            let node_id = NodeId::new(uri, &qualified);
            if let Some(ref v) = n.value.value {
                let rendered = expressions::expression_to_debug_string(&v.value.expression);
                if let Some(node) = g.get_node_mut(&node_id) {
                    node.expression_text.value = Some(rendered.clone());
                    node.expression_text.default_value = Some(rendered);
                }
            }
            attach_declared_typing_relationship(g, &node_id, n.typing.as_deref());
            attach_feature_properties(
                g,
                &node_id,
                DeclaredFeatureProperties {
                    is_ordered: Some(n.ordered),
                    is_unique: Some(!n.nonunique),
                    ..DeclaredFeatureProperties::default()
                },
            );
            for target in typing_targets(n.typing.as_deref()) {
                add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
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
            let mut attrs = HashMap::new();
            let name = resolve_addressable_name(
                &identification_name(&pd_node.identification),
                "part def",
                &mut attrs,
            );
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "part def");
            let range = span_to_range(&pd_node.span);
            if let Some(short_name) =
                crate::semantic::ast_util::declared_short_name(&pd_node.identification)
            {
                g.register_declared_short_name(NodeId::new(uri, &qualified), short_name);
            }
            g.register_declared_membership_facts(
                NodeId::new(uri, &qualified),
                crate::semantic::ast_util::declared_membership_facts(&pd_node.membership),
            );
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
            for target in typing_targets(pd_node.specializes.as_deref()) {
                add_specializes_edge_if_exists(g, uri, &qualified, target, container_prefix);
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
            let mut attrs = HashMap::new();
            let name = resolve_addressable_name(
                &identification_name(&item_node.identification),
                "item def",
                &mut attrs,
            );
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "item def");
            if let Some(short_name) =
                crate::semantic::ast_util::declared_short_name(&item_node.identification)
            {
                g.register_declared_short_name(NodeId::new(uri, &qualified), short_name);
            }
            g.register_declared_membership_facts(
                NodeId::new(uri, &qualified),
                crate::semantic::ast_util::declared_membership_facts(&item_node.membership),
            );
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
            for target in typing_targets(item_node.specializes.as_deref()) {
                add_specializes_edge_if_exists(g, uri, &qualified, target, container_prefix);
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
        PDBE::ItemUsage(item_node) => {
            usage_builders::materialize_item_usage(item_node, uri, container_prefix, parent_id, g);
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
        PDBE::Dependency(dep_node) => {
            super::package_body::materialize_dependency(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                dep_node,
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
                InterfaceUsage::TypedConnect {
                    interface_type,
                    from,
                    to,
                    ..
                } => {
                    expressions::add_interface_edge_if_both_exist(
                        g,
                        uri,
                        container_prefix,
                        from,
                        to,
                        interface_type.as_deref(),
                    );
                }
                InterfaceUsage::Connection { from, to, .. } => {
                    expressions::add_interface_edge_if_both_exist(
                        g,
                        uri,
                        container_prefix,
                        from,
                        to,
                        None,
                    );
                }
                // `interface name;` / `interface : Type { ... }` with no inline `connect`
                // clause (GH-16) -- a placeholder/to-be-redefined-later declaration with no
                // `from`/`to` endpoints to wire a connection edge between, so there is nothing
                // for this arm to do yet (same "no node materialized" treatment the two arms
                // above already give `TypedConnect`/`Connection`, neither of which creates an
                // interface-usage node here either).
                InterfaceUsage::Declaration { .. } => {}
            }
        }
        PDBE::InterfaceDef(id_node) => {
            let name = identification_name(&id_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "interface def");
            let range = span_to_range(&id_node.span);
            let attrs = HashMap::new();
            if let Some(short_name) =
                crate::semantic::ast_util::declared_short_name(&id_node.identification)
            {
                g.register_declared_short_name(NodeId::new(uri, &qualified), short_name);
            }
            g.register_declared_membership_facts(
                NodeId::new(uri, &qualified),
                crate::semantic::ast_util::declared_membership_facts(&id_node.membership),
            );
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
            usage_builders::materialize_connection_usage(
                connection_usage,
                uri,
                container_prefix,
                Some(parent_id),
                g,
            );
        }
        PDBE::CalcUsage(calc_node) => {
            super::calc_constraint_def::materialize_calc_usage(
                g,
                uri,
                container_prefix,
                parent_id,
                calc_node,
            );
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
        // A `use case`/`analysis`/`verification` def or usage nested inside a `part def { ... }`
        // body was previously dropped entirely -- same bug class as the `case`/`case def` gap
        // fixed above, no dispatch arm existed here even though the package-level `PBE::*`
        // dispatch already calls these same materializers (`package_body::materialize`).
        PDBE::UseCaseDef(n) => {
            super::package_body::materialize_use_case_def(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                n,
            );
        }
        PDBE::UseCaseUsage(n) => {
            super::package_body::materialize_use_case_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                n,
            );
        }
        PDBE::AnalysisCaseDef(n) => {
            super::package_body::materialize_analysis_case_def(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                n,
            );
        }
        PDBE::AnalysisCaseUsage(n) => {
            super::package_body::materialize_analysis_case_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                n,
            );
        }
        PDBE::VerificationCaseDef(n) => {
            super::package_body::materialize_verification_case_def(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                n,
            );
        }
        PDBE::VerificationCaseUsage(n) => {
            super::package_body::materialize_verification_case_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                n,
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
        PDBE::Bind(bind_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &bind_node.value.left,
                &bind_node.value.right,
                RelationshipKind::Bind,
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
        PDBE::ActionUsage(au) => {
            super::action::materialize_top_level_action_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                au.as_ref(),
            );
        }
        PDBE::StateUsage(su) => {
            super::package_body::materialize_state_usage(
                g,
                uri,
                container_prefix,
                Some(parent_id),
                su,
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
            g.register_declared_membership_facts(
                NodeId::new(uri, &qualified),
                crate::semantic::ast_util::declared_membership_facts(&enum_node.membership),
            );
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
            if let Some(node) = g.get_node_mut(&node_id) {
                node.source_text.keyword = Some(opaque.keyword.to_string());
                node.source_text.text = Some(opaque.text.to_string());
            }
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
