use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ConnectionDefBody, InterfaceDefBody, PackageBodyElement, PartDefBody,
    PortDefBody, StateDefBody, UseCaseDefBody,
};
use sysml_v2_parser::RootNamespace;
use url::Url;

use super::requirement_body::{import_member_label, walk_requirement_def_body};
use crate::semantic::ast_util::{
    attach_short_name_attribute, identification_name, span_to_range, text_range_to_json,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId, RelationshipKind};
use crate::semantic::relationships::{add_typing_edge_if_exists, try_wire_derivation_connection};

use super::action;
use super::analysis_case;
use super::attribute_body;
use super::calc_constraint_def;
use super::definition_body;
use super::expressions;
use super::kerml_library;
use super::modeled_kerml_name::extract_modeled_decl_name;
use super::package_packages;
use super::unit_metadata;
use super::verification;
use super::view_def;
use super::{
    add_node_and_recurse, insert_def_specialization_attr, qualified_name_for_node,
    wire_def_specialization_edge,
};
use super::{interface_def, part_def, port_def, state, usage_builders, use_case};

pub(super) fn build_from_package_body_element(
    node: &sysml_v2_parser::Node<PackageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(pkg_node) => {
            package_packages::build_nested_package(
                pkg_node,
                uri,
                container_prefix,
                parent_id,
                root,
                g,
                build_from_package_body_element,
            );
        }
        PBE::LibraryPackage(pkg_node) => {
            package_packages::build_nested_library_package(
                pkg_node,
                uri,
                container_prefix,
                parent_id,
                root,
                g,
                build_from_package_body_element,
            );
        }
        PBE::PartDef(pd_node) => {
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
            insert_def_specialization_attr(&mut attrs, pd_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "part def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                pd_node.specializes.as_deref(),
            );
            if let PartDefBody::Brace { elements } = &pd_node.body {
                for child in elements {
                    part_def::build_from_part_def_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        g,
                    );
                }
            }
        }
        PBE::PartUsage(pu_node) => {
            usage_builders::materialize_part_usage(pu_node, uri, container_prefix, parent_id, g);
        }
        PBE::FeatureDecl(feature_node) => {
            let fv = &feature_node.value;
            let name = extract_modeled_decl_name(&fv.keyword, &fv.text, "_feature");
            let semantic_metadata_parent = parent_id.and_then(|pid| {
                g.get_node(pid).and_then(|parent| {
                    (parent.element_kind == ElementKind::MetadataDef
                        && parent
                            .attributes
                            .get("metaclassRole")
                            .and_then(|value| value.as_str())
                            == Some("SemanticMetadata"))
                    .then_some(pid)
                })
            });
            if let Some(parent_id) = semantic_metadata_parent {
                kerml_library::add_kerml_library_feature_node(
                    g,
                    kerml_library::KermlLibraryNodeInput {
                        uri,
                        container_prefix,
                        parent_id,
                        display_name: name,
                        bnf_production: &fv.keyword,
                        text: &fv.text,
                        span: &feature_node.span,
                    },
                );
            } else {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "feature decl");
                let mut attrs = HashMap::new();
                attrs.insert("keyword".to_string(), serde_json::json!(&fv.keyword));
                attrs.insert("text".to_string(), serde_json::json!(&fv.text));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "feature decl",
                    name,
                    span_to_range(&feature_node.span),
                    attrs,
                    parent_id,
                );
            }
        }
        PBE::ClassifierDecl(classifier_node) => {
            let cv = &classifier_node.value;
            let name = extract_modeled_decl_name(&cv.keyword, &cv.text, "_classifier");
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "classifier decl");
            let mut attrs = HashMap::new();
            attrs.insert("keyword".to_string(), serde_json::json!(&cv.keyword));
            attrs.insert("text".to_string(), serde_json::json!(&cv.text));
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "classifier decl",
                name,
                span_to_range(&classifier_node.span),
                attrs,
                parent_id,
            );
        }
        PBE::PortDef(pd_node) => {
            let name = identification_name(&pd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "port def");
            let range = span_to_range(&pd_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &pd_node.identification);
            insert_def_specialization_attr(&mut attrs, pd_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "port def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                pd_node.specializes.as_deref(),
            );
            if let PortDefBody::Brace { elements } = &pd_node.body {
                for child in elements {
                    port_def::build_from_port_def_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        g,
                    );
                }
            }
        }
        PBE::InterfaceDef(id_node) => {
            let name = identification_name(&id_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "interface def");
            let range = span_to_range(&id_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &id_node.identification);
            insert_def_specialization_attr(&mut attrs, id_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "interface def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                id_node.specializes.as_deref(),
            );
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                interface_def::build_from_interface_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::AttributeDef(ad_node) => {
            let value = &ad_node.value;
            let name = &value.name;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name, "attribute def");
            let range = span_to_range(&ad_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = value.typing {
                attrs.insert("attributeType".to_string(), serde_json::json!(t));
            }
            unit_metadata::project_attribute_def_unit_metadata(&mut attrs, value);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            if let Some(ref t) = value.typing {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PBE::ActionDef(ad_node) => {
            let name = identification_name(&ad_node.identification);
            let qualified = action::materialize_action_def(
                g,
                uri,
                container_prefix,
                parent_id,
                ad_node,
                &name,
                ad_node.specializes.as_deref(),
            );
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                ad_node.specializes.as_deref(),
            );
        }
        PBE::ActionUsage(au_node) => {
            action::materialize_top_level_action_usage(
                g,
                uri,
                container_prefix,
                parent_id,
                au_node,
            );
        }
        PBE::AliasDef(alias_node) => {
            let mut name = identification_name(&alias_node.identification);
            if name.is_empty() {
                name = alias_node.target.clone();
            }
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "alias");
            let range = span_to_range(&alias_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &alias_node.identification);
            attrs.insert(
                "target".to_string(),
                serde_json::json!(alias_node.target.clone()),
            );
            add_node_and_recurse(g, uri, &qualified, "alias", name, range, attrs, parent_id);
        }
        PBE::RequirementDef(rd_node) => {
            let name = identification_name(&rd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "requirement def");
            let range = span_to_range(&rd_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &rd_node.identification);
            insert_def_specialization_attr(&mut attrs, rd_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "requirement def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                rd_node.specializes.as_deref(),
            );
            walk_requirement_def_body(
                g,
                uri,
                container_prefix,
                &qualified,
                &node_id,
                &rd_node.body,
            );
        }
        PBE::RequirementUsage(ru_node) => {
            usage_builders::materialize_requirement_usage(
                ru_node,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        PBE::Satisfy(satisfy_node) => {
            expressions::add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &satisfy_node.source,
                &satisfy_node.target,
                RelationshipKind::Satisfy,
            );
            if let Some(elements) = &satisfy_node.body_elements {
                super::requirement_body::walk_satisfy_constraint_elements(
                    elements,
                    uri,
                    container_prefix,
                    g,
                );
            }
        }
        PBE::AllocationUsage(alloc_node) => {
            let name = &alloc_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "allocation");
            let range = span_to_range(&alloc_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = alloc_node.type_name {
                attrs.insert("allocationType".to_string(), serde_json::json!(t));
            }
            if let Some(source) = alloc_node.source.as_ref() {
                attrs.insert(
                    "allocationSource".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(source)),
                );
            }
            if let Some(target) = alloc_node.target.as_ref() {
                attrs.insert(
                    "allocationTarget".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(target)),
                );
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "allocation",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            if let Some(ref t) = alloc_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let (Some(source), Some(target)) = (&alloc_node.source, &alloc_node.target) {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    source,
                    target,
                    RelationshipKind::Allocate,
                );
            }
        }
        PBE::ConcernUsage(cu_node) => {
            let name = &cu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "concern");
            let range = span_to_range(&cu_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = cu_node.type_name {
                attrs.insert("concernType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "concern",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            if let Some(ref t) = cu_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            let node_id = NodeId::new(uri, &qualified);
            walk_requirement_def_body(
                g,
                uri,
                container_prefix,
                &qualified,
                &node_id,
                &cu_node.body,
            );
        }
        PBE::UseCaseDef(ucd_node) => {
            let name = identification_name(&ucd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "use case def");
            let range = span_to_range(&ucd_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &ucd_node.identification);
            insert_def_specialization_attr(&mut attrs, ucd_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "use case def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                ucd_node.specializes.as_deref(),
            );
            if let UseCaseDefBody::Brace { elements } = &ucd_node.body {
                use_case::build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
            }
        }
        PBE::UseCaseUsage(ucu_node) => {
            let name = &ucu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "use case");
            let range = span_to_range(&ucu_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = ucu_node.type_name {
                attrs.insert("useCaseType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "use case",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            if let Some(ref t) = ucu_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let UseCaseDefBody::Brace { elements } = &ucu_node.body {
                use_case::build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
            }
        }
        PBE::ItemDef(item_node) => {
            let name = identification_name(&item_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "item def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &item_node.identification);
                insert_def_specialization_attr(&mut attrs, item_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "item def",
                    name,
                    span_to_range(&item_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    item_node.specializes.as_deref(),
                );
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
        PBE::IndividualDef(ind_node) => {
            let name = identification_name(&ind_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "individual def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &ind_node.identification);
                insert_def_specialization_attr(&mut attrs, ind_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "individual def",
                    name.clone(),
                    span_to_range(&ind_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    ind_node.specializes.as_deref(),
                );
                let node_id = NodeId::new(uri, &qualified);
                attribute_body::build_from_attribute_body(
                    &ind_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::MetadataDef(md_node) => {
            let name = identification_name(&md_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "metadata def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &md_node.identification);
                insert_def_specialization_attr(&mut attrs, md_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "metadata def",
                    name,
                    span_to_range(&md_node.span),
                    attrs,
                    parent_id,
                );
                let node_id = NodeId::new(uri, &qualified);
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    md_node.specializes.as_deref(),
                );
                super::metadata_def::build_from_metadata_attribute_body(
                    &md_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::MetadataUsage(mu_node) => {
            if let Some(parent_id) = parent_id {
                super::metadata_def::add_package_metadata_usage_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    mu_node,
                    &mu_node.span,
                );
            }
        }
        PBE::EnumDef(enum_node) => {
            let name = identification_name(&enum_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "enum def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &enum_node.identification);
                insert_def_specialization_attr(&mut attrs, enum_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "enum def",
                    name,
                    span_to_range(&enum_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    enum_node.specializes.as_deref(),
                );
            }
        }
        PBE::OccurrenceDef(occ_node) => {
            let name = identification_name(&occ_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "occurrence def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &occ_node.identification);
                insert_def_specialization_attr(&mut attrs, occ_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "occurrence def",
                    name,
                    span_to_range(&occ_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    occ_node.specializes.as_deref(),
                );
                let node_id = NodeId::new(uri, &qualified);
                definition_body::build_from_definition_body(
                    &occ_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::OccurrenceUsage(occ_node) => {
            usage_builders::materialize_occurrence_usage(occ_node, uri, container_prefix, parent_id, g);
        }
        PBE::ConnectionDef(conn_node) => {
            let name = identification_name(&conn_node.identification);
            let annotation = conn_node.annotation.as_deref();
            let base_name = if name.is_empty() {
                if annotation == Some("derivation") {
                    "_derivationConnection"
                } else {
                    "_connectionDef"
                }
            } else {
                name.as_str()
            };
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &conn_node.identification);
            if let Some(annotation) = annotation {
                attrs.insert(
                    "connectionAnnotation".to_string(),
                    serde_json::json!(annotation),
                );
            }
            insert_def_specialization_attr(&mut attrs, conn_node.specializes.as_deref());
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, base_name, "connection def");
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                if annotation == Some("derivation") {
                    "derivation connection"
                } else {
                    "connection def"
                },
                base_name.to_string(),
                span_to_range(&conn_node.span),
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                conn_node.specializes.as_deref(),
            );
            if let ConnectionDefBody::Brace { elements } = &conn_node.body {
                interface_def::build_from_connection_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
                if annotation == Some("derivation") {
                    try_wire_derivation_connection(g, uri, &node_id);
                }
            }
        }
        PBE::FlowDef(flow_node) => {
            let name = identification_name(&flow_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "flow def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &flow_node.identification);
                insert_def_specialization_attr(&mut attrs, flow_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "flow def",
                    name,
                    span_to_range(&flow_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    flow_node.specializes.as_deref(),
                );
                let node_id = NodeId::new(uri, &qualified);
                definition_body::build_from_definition_body(
                    &flow_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::FlowUsage(flow_node) => {
            if let Some(parent_id) = parent_id {
                super::flow_usage::materialize_flow_usage(
                    flow_node,
                    uri,
                    container_prefix,
                    parent_id,
                    g,
                );
            }
        }
        PBE::AllocationDef(alloc_node) => {
            let name = identification_name(&alloc_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "allocation def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &alloc_node.identification);
                insert_def_specialization_attr(&mut attrs, alloc_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "allocation def",
                    name,
                    span_to_range(&alloc_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    alloc_node.specializes.as_deref(),
                );
                let node_id = NodeId::new(uri, &qualified);
                definition_body::build_from_definition_body(
                    &alloc_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::Dependency(dep_node) => {
            let name = dep_node
                .identification
                .as_ref()
                .map(identification_name)
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| "dependency".to_string());
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "dependency");
            let mut attrs = HashMap::new();
            if let Some(ref ident) = dep_node.identification {
                attach_short_name_attribute(&mut attrs, ident);
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "dependency",
                name,
                span_to_range(&dep_node.span),
                attrs,
                parent_id,
            );
        }
        PBE::ConstraintDef(c_node) => {
            calc_constraint_def::build_constraint_def(g, uri, container_prefix, parent_id, c_node);
        }
        PBE::CalcDef(c_node) => {
            calc_constraint_def::build_calc_def(g, uri, container_prefix, parent_id, c_node);
        }
        PBE::CaseDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "case def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &c_node.identification);
                insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "case def",
                    name,
                    span_to_range(&c_node.span),
                    attrs,
                    parent_id,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    c_node.specializes.as_deref(),
                );
            }
        }
        PBE::CaseUsage(c_node) => {
            let qualified = qualified_name_for_node(g, uri, container_prefix, &c_node.name, "case");
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "case",
                c_node.name.clone(),
                span_to_range(&c_node.span),
                HashMap::new(),
                parent_id,
            );
        }
        PBE::AnalysisCaseDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "analysis def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &c_node.identification);
                insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "analysis def",
                    name,
                    span_to_range(&c_node.span),
                    attrs,
                    parent_id,
                );
                let node_id = NodeId::new(uri, &qualified);
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    c_node.specializes.as_deref(),
                );
                analysis_case::build_from_analysis_body(
                    &c_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::AnalysisCaseUsage(c_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &c_node.name, "analysis");
            let mut attrs = HashMap::new();
            if let Some(ref t) = c_node.type_name {
                attrs.insert("analysisType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "analysis",
                c_node.name.clone(),
                span_to_range(&c_node.span),
                attrs,
                parent_id,
            );
            if let Some(ref t) = c_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            let node_id = NodeId::new(uri, &qualified);
            analysis_case::build_from_analysis_body(
                &c_node.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PBE::VerificationCaseDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "verification def");
                let mut attrs = HashMap::new();
                attach_short_name_attribute(&mut attrs, &c_node.identification);
                insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "verification def",
                    name,
                    span_to_range(&c_node.span),
                    attrs,
                    parent_id,
                );
                let node_id = NodeId::new(uri, &qualified);
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    c_node.specializes.as_deref(),
                );
                verification::build_from_verification_body(
                    &c_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
        }
        PBE::VerificationCaseUsage(c_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &c_node.name, "verification");
            let mut attrs = HashMap::new();
            if let Some(ref t) = c_node.type_name {
                attrs.insert("verificationType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "verification",
                c_node.name.clone(),
                span_to_range(&c_node.span),
                attrs,
                parent_id,
            );
            if let Some(ref t) = c_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            let node_id = NodeId::new(uri, &qualified);
            verification::build_from_verification_body(
                &c_node.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PBE::Actor(actor_node) => {
            let name = identification_name(&actor_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "actor");
            let range = span_to_range(&actor_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &actor_node.identification);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "actor",
                name,
                range,
                attrs,
                parent_id,
            );
        }
        PBE::StateDef(sd_node) => {
            let name = identification_name(&sd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "state def");
            let range = span_to_range(&sd_node.span);
            let mut attrs = HashMap::new();
            attach_short_name_attribute(&mut attrs, &sd_node.identification);
            insert_def_specialization_attr(&mut attrs, sd_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "state def",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                sd_node.specializes.as_deref(),
            );
            if let StateDefBody::Brace { elements } = &sd_node.body {
                state::build_from_state_body(elements, uri, Some(&qualified), &node_id, g);
            }
        }
        PBE::StateUsage(su_node) => {
            let name = &su_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "state");
            let range = span_to_range(&su_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = su_node.type_name {
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
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            if let Some(ref t) = su_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            if let StateDefBody::Brace { elements } = &su_node.body {
                state::build_from_state_body(elements, uri, Some(&qualified), &node_id, g);
            }
        }
        PBE::ViewDef(vd_node) => {
            view_def::build_view_def(g, uri, container_prefix, parent_id, vd_node);
        }
        PBE::ViewpointDef(vpd_node) => {
            view_def::build_viewpoint_def(g, uri, container_prefix, parent_id, vpd_node);
        }
        PBE::RenderingDef(rd_node) => {
            view_def::build_rendering_def(g, uri, container_prefix, parent_id, rd_node);
        }
        PBE::ViewUsage(vu_node) => {
            view_def::build_view_usage(g, uri, container_prefix, parent_id, vu_node);
        }
        PBE::ViewpointUsage(vpu_node) => {
            view_def::build_viewpoint_usage(g, uri, container_prefix, parent_id, vpu_node);
        }
        PBE::RenderingUsage(ru_node) => {
            view_def::build_rendering_usage(g, uri, container_prefix, parent_id, ru_node);
        }
        PBE::Import(imp) => {
            if let Some(pid) = parent_id {
                let v = &imp.value;
                let name = import_member_label(&v.target);
                let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "import");
                let mut attrs = HashMap::new();
                attrs.insert("importTarget".to_string(), serde_json::json!(&v.target));
                attrs.insert("importAll".to_string(), serde_json::json!(v.is_import_all));
                if let Some(vis) = &v.visibility {
                    attrs.insert(
                        "visibility".to_string(),
                        serde_json::json!(format!("{vis:?}")),
                    );
                }
                attrs.insert("recursive".to_string(), serde_json::json!(v.is_recursive));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "import",
                    name,
                    span_to_range(&imp.span),
                    attrs,
                    Some(pid),
                );
            }
        }
        // Intentionally omitted from the graph: parse placeholders and documentation-only members.
        PBE::Doc(doc) => {
            if let Some(pid) = parent_id {
                super::attach_doc_comment(g, pid, &doc.value.text);
            }
        }
        PBE::Error(_) | PBE::Comment(_) => {}
        PBE::TextualRep(t) => {
            if let Some(pid) = parent_id {
                let tr = &t.value;
                let name = tr
                    .rep_identification
                    .as_ref()
                    .map(identification_name)
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "_textualRep".to_string());
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "textualRep");
                let mut attrs = HashMap::new();
                if let Some(ref rep_identification) = tr.rep_identification {
                    attach_short_name_attribute(&mut attrs, rep_identification);
                }
                attrs.insert("language".to_string(), serde_json::json!(&tr.language));
                attrs.insert("text".to_string(), serde_json::json!(&tr.text));
                if let Some(ref language_span) = tr.language_span {
                    attrs.insert(
                        "languageSpan".to_string(),
                        text_range_to_json(span_to_range(language_span)),
                    );
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "textualRep",
                    name,
                    span_to_range(&t.span),
                    attrs,
                    Some(pid),
                );
            }
        }
        PBE::Filter(f) => {
            view_def::build_filter_member(g, uri, container_prefix, parent_id, f);
        }
        PBE::KermlSemanticDecl(k) => {
            kerml_library::build_kerml_semantic_decl(g, uri, container_prefix, parent_id, k);
        }
        PBE::KermlFeatureDecl(k) => {
            kerml_library::build_kerml_feature_decl(g, uri, container_prefix, parent_id, k);
        }
        PBE::ExtendedLibraryDecl(k) => {
            kerml_library::build_extended_library_decl(g, uri, container_prefix, parent_id, k);
        }
    }
}
