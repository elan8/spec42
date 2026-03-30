use std::collections::HashMap;

use sysml_parser::ast::{
    InterfaceDefBody, PackageBody, PackageBodyElement, PartDefBody, PartUsageBody, PortDefBody,
    StateDefBody, UseCaseDefBody,
};
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use crate::ast_util::{identification_name, span_to_range};
use crate::graph::SemanticGraph;
use crate::graph_builder_requirement_subjects::add_requirement_subject_edges;
use crate::model::{NodeId, RelationshipKind, SemanticNode};
use crate::relationships::{add_specializes_edge_if_exists, add_typing_edge_if_exists};

use super::expressions;
use super::{add_node_and_recurse, qualified_name_for_node};
use super::{part_def, part_usage, port_def, state, stubs, use_case};

pub(super) fn build_from_package_body_element(
    node: &sysml_parser::Node<PackageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(pkg_node) => {
            let name = identification_name(&pkg_node.identification);
            let name_display = if name.is_empty() {
                "(top level)"
            } else {
                name.as_str()
            };
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name_display, "package");
            let node_id = NodeId::new(uri, &qualified);
            let range = span_to_range(&pkg_node.span);
            let sem_node = SemanticNode {
                id: node_id.clone(),
                element_kind: "package".to_string(),
                name: name_display.to_string(),
                range,
                attributes: HashMap::new(),
                parent_id: parent_id.cloned(),
            };
            let idx = g.graph.add_node(sem_node);
            g.node_index_by_id.insert(node_id.clone(), idx);
            g.nodes_by_uri
                .entry(uri.clone())
                .or_default()
                .push(node_id.clone());
            let prefix = if name.is_empty() {
                container_prefix.map(str::to_string)
            } else {
                Some(qualified.clone())
            };
            if let PackageBody::Brace { elements } = &pkg_node.body {
                for child in elements {
                    build_from_package_body_element(
                        child,
                        uri,
                        prefix.as_deref(),
                        Some(&node_id),
                        root,
                        g,
                    );
                }
            }
        }
        PBE::LibraryPackage(pkg_node) => {
            let name = identification_name(&pkg_node.identification);
            let name_display = if name.is_empty() {
                "(top level)"
            } else {
                name.as_str()
            };
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name_display, "package");
            let node_id = NodeId::new(uri, &qualified);
            let mut attrs = HashMap::new();
            attrs.insert(
                "isStandardLibrary".to_string(),
                serde_json::json!(pkg_node.is_standard),
            );
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "package",
                name_display.to_string(),
                span_to_range(&pkg_node.span),
                attrs,
                parent_id,
            );
            let prefix = if name.is_empty() {
                container_prefix.map(str::to_string)
            } else {
                Some(qualified.clone())
            };
            if let PackageBody::Brace { elements } = &pkg_node.body {
                for child in elements {
                    build_from_package_body_element(
                        child,
                        uri,
                        prefix.as_deref(),
                        Some(&node_id),
                        root,
                        g,
                    );
                }
            }
        }
        PBE::PartDef(pd_node) => {
            let name = identification_name(&pd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "part def");
            let range = span_to_range(&pd_node.span);
            let mut attrs = HashMap::new();
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
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            stubs::relationships_from_part_def(pd_node, uri, container_prefix, &qualified, g);
            if let PartDefBody::Brace { elements } = &pd_node.body {
                for child in elements {
                    part_def::build_from_part_def_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        root,
                        g,
                    );
                }
            }
            if let Some(ref s) = pd_node.specializes {
                add_specializes_edge_if_exists(g, uri, &qualified, s, container_prefix);
            }
        }
        PBE::PartUsage(pu_node) => {
            let name = &pu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
            let range = span_to_range(&pu_node.span);
            let mut attrs = HashMap::new();
            attrs.insert(
                "partType".to_string(),
                serde_json::json!(&pu_node.type_name),
            );
            if let Some(ref m) = pu_node.multiplicity {
                attrs.insert("multiplicity".to_string(), serde_json::json!(m));
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
            add_typing_edge_if_exists(g, uri, &qualified, &pu_node.type_name, container_prefix);
            if let PartUsageBody::Brace { elements } = &pu_node.body {
                for child in elements {
                    part_usage::build_from_part_usage_body_element(
                        child,
                        uri,
                        Some(&qualified),
                        &node_id,
                        root,
                        g,
                    );
                }
            }
            part_usage::expand_typed_part_usage(
                root,
                uri,
                &qualified,
                &pu_node.type_name,
                container_prefix,
                &node_id,
                g,
            );
        }
        PBE::PortDef(pd_node) => {
            let name = identification_name(&pd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "port def");
            let range = span_to_range(&pd_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "port def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
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
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "interface");
            let range = span_to_range(&id_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "interface",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
            let _node_id = NodeId::new(uri, &qualified);
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                for _ in elements {
                    // EndDecl, RefDecl, ConnectStmt - we don't add graph nodes for them for now
                }
            }
        }
        PBE::AttributeDef(ad_node) => {
            let name = &ad_node.name;
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name, "attribute def");
            let range = span_to_range(&ad_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = ad_node.typing {
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
                parent_id,
            );
            if let Some(ref t) = ad_node.typing {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PBE::ActionDef(ad_node) => {
            let name = identification_name(&ad_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "action def");
            let range = span_to_range(&ad_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "action def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::ActionUsage(au_node) => {
            let name = &au_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "action");
            let range = span_to_range(&au_node.span);
            let mut attrs = HashMap::new();
            attrs.insert(
                "actionType".to_string(),
                serde_json::json!(&au_node.type_name),
            );
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "action",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            add_typing_edge_if_exists(g, uri, &qualified, &au_node.type_name, container_prefix);
        }
        PBE::AliasDef(alias_node) => {
            let mut name = identification_name(&alias_node.identification);
            if name.is_empty() {
                name = alias_node.target.clone();
            }
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "alias");
            let range = span_to_range(&alias_node.span);
            let mut attrs = HashMap::new();
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
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "requirement def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
            add_requirement_subject_edges(g, uri, container_prefix, &qualified, &rd_node.body);
        }
        PBE::RequirementUsage(ru_node) => {
            let name = &ru_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "requirement");
            let range = span_to_range(&ru_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = ru_node.type_name {
                attrs.insert("requirementType".to_string(), serde_json::json!(t));
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
            if let Some(ref t) = ru_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            add_requirement_subject_edges(g, uri, container_prefix, &qualified, &ru_node.body);
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
        }
        PBE::UseCaseDef(ucd_node) => {
            let name = identification_name(&ucd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "use case def");
            let range = span_to_range(&ucd_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "use case def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
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
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "item def",
                    name,
                    span_to_range(&item_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::IndividualDef(ind_node) => {
            let name = identification_name(&ind_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "individual def");
                let mut attrs = HashMap::new();
                if let Some(ref s) = ind_node.specializes {
                    attrs.insert("specializes".to_string(), serde_json::json!(s.clone()));
                }
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
                if let Some(ref s) = ind_node.specializes {
                    add_specializes_edge_if_exists(g, uri, &qualified, s, container_prefix);
                }
            }
        }
        PBE::MetadataDef(md_node) => {
            let name = identification_name(&md_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "metadata def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "metadata def",
                    name,
                    span_to_range(&md_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::EnumDef(enum_node) => {
            let name = identification_name(&enum_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "enum def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "enum def",
                    name,
                    span_to_range(&enum_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::OccurrenceDef(occ_node) => {
            let name = identification_name(&occ_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "occurrence def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "occurrence def",
                    name,
                    span_to_range(&occ_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::OccurrenceUsage(occ_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &occ_node.name, "occurrence");
            let mut attrs = HashMap::new();
            if let Some(ref t) = occ_node.type_name {
                attrs.insert("occurrenceType".to_string(), serde_json::json!(t.clone()));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "occurrence",
                occ_node.name.clone(),
                span_to_range(&occ_node.span),
                attrs,
                parent_id,
            );
            if let Some(ref t) = occ_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PBE::ConnectionDef(conn_node) => {
            let name = identification_name(&conn_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "connection def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "connection def",
                    name,
                    span_to_range(&conn_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::FlowDef(flow_node) => {
            let name = identification_name(&flow_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "flow def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "flow def",
                    name,
                    span_to_range(&flow_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::FlowUsage(flow_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &flow_node.name, "flow");
            let mut attrs = HashMap::new();
            if let Some(ref t) = flow_node.type_name {
                attrs.insert("flowType".to_string(), serde_json::json!(t.clone()));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "flow",
                flow_node.name.clone(),
                span_to_range(&flow_node.span),
                attrs,
                parent_id,
            );
            if let Some(ref t) = flow_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PBE::AllocationDef(alloc_node) => {
            let name = identification_name(&alloc_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "allocation def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "allocation def",
                    name,
                    span_to_range(&alloc_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::AllocationUsage(alloc_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &alloc_node.name, "allocation");
            let mut attrs = HashMap::new();
            if let Some(ref t) = alloc_node.type_name {
                attrs.insert("allocationType".to_string(), serde_json::json!(t.clone()));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "allocation",
                alloc_node.name.clone(),
                span_to_range(&alloc_node.span),
                attrs,
                parent_id,
            );
            if let Some(ref t) = alloc_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
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
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "dependency",
                name,
                span_to_range(&dep_node.span),
                HashMap::new(),
                parent_id,
            );
        }
        PBE::ConstraintDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "constraint def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "constraint def",
                    name,
                    span_to_range(&c_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::CalcDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "calc def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "calc def",
                    name,
                    span_to_range(&c_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::CaseDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "case def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "case def",
                    name,
                    span_to_range(&c_node.span),
                    HashMap::new(),
                    parent_id,
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
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "analysis def",
                    name,
                    span_to_range(&c_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::AnalysisCaseUsage(c_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &c_node.name, "analysis");
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "analysis",
                c_node.name.clone(),
                span_to_range(&c_node.span),
                HashMap::new(),
                parent_id,
            );
        }
        PBE::VerificationCaseDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "verification def");
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "verification def",
                    name,
                    span_to_range(&c_node.span),
                    HashMap::new(),
                    parent_id,
                );
            }
        }
        PBE::VerificationCaseUsage(c_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &c_node.name, "verification");
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "verification",
                c_node.name.clone(),
                span_to_range(&c_node.span),
                HashMap::new(),
                parent_id,
            );
        }
        PBE::Actor(actor_node) => {
            let name = identification_name(&actor_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "actor def");
            let range = span_to_range(&actor_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "actor def",
                name,
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::StateDef(sd_node) => {
            let name = identification_name(&sd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "state def");
            let range = span_to_range(&sd_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "state def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
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
            let name = identification_name(&vd_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "view def");
            let range = span_to_range(&vd_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "view def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::ViewpointDef(vpd_node) => {
            let name = identification_name(&vpd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "viewpoint def");
            let range = span_to_range(&vpd_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "viewpoint def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::RenderingDef(rd_node) => {
            let name = identification_name(&rd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "rendering def");
            let range = span_to_range(&rd_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "rendering def",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::ViewUsage(vu_node) => {
            let name = &vu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "view");
            let range = span_to_range(&vu_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "view",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::ViewpointUsage(vpu_node) => {
            let name = &vpu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "viewpoint");
            let range = span_to_range(&vpu_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "viewpoint",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::RenderingUsage(ru_node) => {
            let name = &ru_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "rendering");
            let range = span_to_range(&ru_node.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "rendering",
                name.clone(),
                range,
                HashMap::new(),
                parent_id,
            );
        }
        PBE::Import(_) => {}
        _ => {}
    }
}
