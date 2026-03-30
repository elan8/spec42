//! Builds semantic graph from parsed AST (packages, parts, ports, connections, etc.).

use std::collections::HashMap;
use sysml_parser::ast::{
    InterfaceDefBody, PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement,
    PartUsageBody, PartUsageBodyElement, PortDefBody, PortDefBodyElement, RequirementDefBody,
    StateDefBody, StateDefBodyElement, UseCaseDefBody,
};
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::{Range, Url};

use crate::ast_util::{identification_name, span_to_range};
use crate::semantic_model::relationships::{
    add_edge_if_both_exist, add_specializes_edge_if_exists, add_typing_edge_if_exists,
    find_part_def_in_root, type_ref_candidates,
};
use crate::semantic_model::{
    root_element_body, NodeId, RelationshipKind, SemanticGraph, SemanticNode,
};

/// Builds a semantic graph from a parsed RootNamespace (sysml-parser AST).
/// Adds the root package/namespace as a node and sets parent_id on its direct children
/// so that contains edges are emitted for the General View.
pub fn build_graph_from_doc(root: &RootNamespace, uri: &Url) -> SemanticGraph {
    let mut g = SemanticGraph::new();
    for node in &root.elements {
        let (elements, pkg_qualified, pkg_name_display, pkg_span) =
            match root_element_body(&node.value) {
                Some(t) => t,
                None => continue,
            };
        let pkg_qualified_disambiguated = qualified_name_for_node(
            &g,
            uri,
            None,
            if pkg_name_display == "(top level)" {
                ""
            } else {
                &pkg_name_display
            },
            "package",
        );
        let pkg_qualified_final = if pkg_qualified_disambiguated.is_empty() {
            pkg_qualified.clone()
        } else {
            pkg_qualified_disambiguated
        };
        add_node_and_recurse(
            &mut g,
            uri,
            &pkg_qualified_final,
            "package",
            pkg_name_display,
            span_to_range(pkg_span),
            HashMap::new(),
            None,
        );
        let package_node_id = NodeId::new(uri, &pkg_qualified_final);
        let child_prefix = if pkg_qualified == "(top level)" || pkg_qualified.is_empty() {
            None
        } else {
            Some(pkg_qualified_final.as_str())
        };
        for el in elements {
            build_from_package_body_element(
                el,
                uri,
                child_prefix,
                Some(&package_node_id),
                root,
                &mut g,
            );
        }
    }
    g
}

fn build_from_package_body_element(
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
            relationships_from_part_def(pd_node, uri, container_prefix, &qualified, g);
            if let PartDefBody::Brace { elements } = &pd_node.body {
                for child in elements {
                    build_from_part_def_body_element(
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
                    build_from_port_def_body_element(child, uri, Some(&qualified), &node_id, g);
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
            attrs.insert("target".to_string(), serde_json::json!(alias_node.target.clone()));
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "alias",
                name,
                range,
                attrs,
                parent_id,
            );
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
            if let RequirementDefBody::Brace { .. } = &rd_node.body {
                // Body currently carries constraints/docs/subjects, but no additional graph nodes yet.
            }
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
        }
        PBE::Satisfy(satisfy_node) => {
            add_expression_edge_if_both_exist(
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
                build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
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
                build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
            }
        }
        PBE::ItemDef(item_node) => {
            let name = identification_name(&item_node.identification);
            if !name.is_empty() {
                let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "item def");
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
                let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "enum def");
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
                let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "flow def");
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
            let qualified = qualified_name_for_node(g, uri, container_prefix, &flow_node.name, "flow");
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
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "dependency");
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
                let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "calc def");
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
                let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "case def");
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
                let qualified = qualified_name_for_node(
                    g,
                    uri,
                    container_prefix,
                    &name,
                    "verification def",
                );
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
            let qualified = qualified_name_for_node(
                g,
                uri,
                container_prefix,
                &c_node.name,
                "verification",
            );
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
                build_from_state_body(elements, uri, Some(&qualified), &node_id, g);
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
                build_from_state_body(elements, uri, Some(&qualified), &node_id, g);
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

fn build_from_part_def_body_element(
    node: &sysml_parser::Node<PartDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PartDefBodyElement as PDBE;
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
            if let Some(ref t) = n.typing {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
        }
        PDBE::PortUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "port");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = n.type_name {
                attrs.insert("portType".to_string(), serde_json::json!(t));
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
        PDBE::PartUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "part");
            let range = span_to_range(&n.span);
            let mut attrs = HashMap::new();
            attrs.insert("partType".to_string(), serde_json::json!(&n.type_name));
            if let Some(ref m) = n.multiplicity {
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
        PDBE::Connect(c) => {
            add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &c.from,
                &c.to,
                RelationshipKind::Connection,
            );
        }
        PDBE::InterfaceUsage(interface_usage) => {
            use sysml_parser::ast::InterfaceUsage;
            match &interface_usage.value {
                InterfaceUsage::TypedConnect { from, to, .. }
                | InterfaceUsage::Connection { from, to, .. } => {
                    add_expression_edge_if_both_exist(
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
        PDBE::Perform(perform_node) => {
            let perform_qualified = add_perform_usage_node(
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
            add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &allocate_node.source,
                &allocate_node.target,
                RelationshipKind::Allocate,
            );
        }
        _ => {}
    }
}

fn build_from_part_usage_body_element(
    node: &sysml_parser::Node<PartUsageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PartUsageBodyElement as PUBE;
    match &node.value {
        PUBE::AttributeUsage(n) => {
            let name = &n.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "attribute");
            let range = span_to_range(&n.span);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute",
                name.clone(),
                range,
                HashMap::new(),
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
            add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &c.from,
                &c.to,
                RelationshipKind::Connection,
            );
        }
        PUBE::Bind(b) => {
            add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &b.left,
                &b.right,
                RelationshipKind::Bind,
            );
        }
        PUBE::InterfaceUsage(interface_usage) => {
            use sysml_parser::ast::InterfaceUsage;
            match &interface_usage.value {
                InterfaceUsage::TypedConnect { from, to, .. }
                | InterfaceUsage::Connection { from, to, .. } => {
                    add_expression_edge_if_both_exist(
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
            let perform_qualified = add_perform_usage_node(
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
            add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &allocate_node.source,
                &allocate_node.target,
                RelationshipKind::Allocate,
            );
        }
        PUBE::Satisfy(satisfy_node) => {
            add_expression_edge_if_both_exist(
                g,
                uri,
                container_prefix,
                &satisfy_node.source,
                &satisfy_node.target,
                RelationshipKind::Satisfy,
            );
        }
        _ => {}
    }
}

fn add_perform_usage_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    action_name: &str,
    action_type: Option<&str>,
    range: Range,
) -> String {
    let qualified = qualified_name_for_node(g, uri, container_prefix, action_name, "action");
    if !g
        .node_index_by_id
        .contains_key(&NodeId::new(uri, &qualified))
    {
        let mut attrs = HashMap::new();
        if let Some(action_type) = action_type {
            attrs.insert("actionType".to_string(), serde_json::json!(action_type));
        }
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "action",
            action_name.to_string(),
            range,
            attrs,
            Some(parent_id),
        );
    }
    if let Some(action_type) = action_type {
        add_typing_edge_if_exists(g, uri, &qualified, action_type, container_prefix);
    }
    qualified
}

fn add_expression_edge_if_both_exist(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    left: &sysml_parser::Node<sysml_parser::Expression>,
    right: &sysml_parser::Node<sysml_parser::Expression>,
    kind: RelationshipKind,
) {
    let left_str = expr_node_to_qualified_string(left);
    let right_str = expr_node_to_qualified_string(right);
    if left_str.is_empty() || right_str.is_empty() {
        return;
    }
    let src = if kind == RelationshipKind::Connection {
        match resolve_expression_endpoint_strict(g, uri, container_prefix, &left_str) {
            EndpointResolution::Resolved(id) => id,
            EndpointResolution::Ambiguous => {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "ambiguous_connection_endpoint",
                    format!(
                        "Ambiguous connection endpoint '{}'. Use a fully qualified endpoint path.",
                        left_str
                    ),
                    span_to_range(&left.span),
                );
                return;
            }
            EndpointResolution::Unresolved => return,
        }
    } else {
        let Some(id) = resolve_expression_endpoint_legacy(g, uri, container_prefix, &left_str) else {
            if kind == RelationshipKind::Satisfy {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "unresolved_satisfy_source",
                    format!(
                        "Could not resolve satisfy source '{}'. Use a valid in-scope element name.",
                        left_str
                    ),
                    span_to_range(&left.span),
                );
            }
            return;
        };
        id
    };
    let tgt = if kind == RelationshipKind::Connection {
        match resolve_expression_endpoint_strict(g, uri, container_prefix, &right_str) {
            EndpointResolution::Resolved(id) => id,
            EndpointResolution::Ambiguous => {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "ambiguous_connection_endpoint",
                    format!(
                        "Ambiguous connection endpoint '{}'. Use a fully qualified endpoint path.",
                        right_str
                    ),
                    span_to_range(&right.span),
                );
                return;
            }
            EndpointResolution::Unresolved => return,
        }
    } else {
        let Some(id) = resolve_expression_endpoint_legacy(g, uri, container_prefix, &right_str) else {
            if kind == RelationshipKind::Satisfy {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "unresolved_satisfy_target",
                    format!(
                        "Could not resolve satisfy target '{}'. Use a valid in-scope element name.",
                        right_str
                    ),
                    span_to_range(&right.span),
                );
            }
            return;
        };
        id
    };
    add_edge_if_both_exist(g, uri, &src, &tgt, kind.clone());
    if kind == RelationshipKind::Connection {
        g.record_connection_occurrence(uri, NodeId::new(uri, &src), NodeId::new(uri, &tgt), span_to_range(&left.span));
    }
}

fn expr_node_to_qualified_string(n: &sysml_parser::Node<sysml_parser::Expression>) -> String {
    use sysml_parser::Expression;
    match &n.value {
        Expression::FeatureRef(s) => s.clone(),
        Expression::MemberAccess(box_base, member) => {
            format!("{}::{}", expr_node_to_qualified_string(box_base), member)
        }
        _ => "".to_string(),
    }
}

enum EndpointResolution {
    Resolved(String),
    Ambiguous,
    Unresolved,
}

fn resolve_expression_endpoint_strict(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> EndpointResolution {
    let expr_normalized = expression.replace('.', "::");
    let mut expression_forms = Vec::new();
    expression_forms.push(expression.to_string());
    if expr_normalized != expression {
        expression_forms.push(expr_normalized.clone());
    }

    let mut candidates = Vec::new();
    if let Some(prefix) = container_prefix {
        for form in &expression_forms {
            candidates.push(format!("{}::{}", prefix, form));
        }
    }
    candidates.extend(expression_forms.clone());

    for candidate in &candidates {
        let node_id = NodeId::new(uri, candidate);
        if g.node_index_by_id.contains_key(&node_id) {
            return EndpointResolution::Resolved(candidate.clone());
        }
    }

    let suffixes: Vec<String> = expression_forms
        .iter()
        .map(|form| format!("::{}", form))
        .collect();
    let mut matches: Vec<&NodeId> = g
        .nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            expression_forms.contains(&node_id.qualified_name)
                || suffixes
                    .iter()
                    .any(|suffix| node_id.qualified_name.ends_with(suffix))
        })
        .collect();
    // Ambiguous suffix resolution frequently causes false connection bindings; require uniqueness.
    matches.sort_by_key(|node_id| node_id.qualified_name.len());
    matches.dedup_by(|a, b| a.qualified_name == b.qualified_name);
    if matches.len() == 1 {
        EndpointResolution::Resolved(matches[0].qualified_name.clone())
    } else if matches.len() > 1 {
        EndpointResolution::Ambiguous
    } else {
        EndpointResolution::Unresolved
    }
}

fn resolve_expression_endpoint_legacy(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> Option<String> {
    let mut candidates = Vec::new();
    if let Some(prefix) = container_prefix {
        candidates.push(format!("{}::{}", prefix, expression));
    }
    candidates.push(expression.to_string());

    for candidate in &candidates {
        let node_id = NodeId::new(uri, candidate);
        if g.node_index_by_id.contains_key(&node_id) {
            return Some(candidate.clone());
        }
    }

    let suffix = format!("::{}", expression);
    g.nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            node_id.qualified_name == expression || node_id.qualified_name.ends_with(&suffix)
        })
        .min_by_key(|node_id| node_id.qualified_name.len())
        .map(|node_id| node_id.qualified_name.clone())
}

fn add_diagnostic_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    code: &str,
    message: String,
    range: Range,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, code, "diagnostic");
    let mut attrs = HashMap::new();
    attrs.insert("code".to_string(), serde_json::json!(code));
    attrs.insert("message".to_string(), serde_json::json!(message));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "diagnostic",
        code.to_string(),
        range,
        attrs,
        None,
    );
}

fn build_from_use_case_body(
    elements: &[sysml_parser::Node<sysml_parser::ast::UseCaseDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::UseCaseDefBodyElement as UCBE;
    for node in elements {
        if let UCBE::ActorUsage(actor_node) = &node.value {
            let name = &actor_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "actor");
            let range = span_to_range(&actor_node.span);
            let mut attrs = HashMap::new();
            attrs.insert(
                "actorType".to_string(),
                serde_json::json!(&actor_node.type_name),
            );
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "actor",
                name.clone(),
                range,
                attrs,
                Some(parent_id),
            );
            add_typing_edge_if_exists(g, uri, &qualified, &actor_node.type_name, container_prefix);
        }
    }
}

fn build_from_state_body(
    elements: &[sysml_parser::Node<StateDefBodyElement>],
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::StateDefBodyElement as SDBE;
    for node in elements {
        match &node.value {
            SDBE::StateUsage(state_node) => {
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
                    build_from_state_body(elements, uri, Some(&qualified), &state_id, g);
                }
            }
            SDBE::Transition(transition_node) => {
                if let Some(src_expr) = &transition_node.source {
                    let src_rel = expr_node_to_qualified_string(src_expr);
                    let tgt_rel = expr_node_to_qualified_string(&transition_node.target);
                    if !src_rel.is_empty() && !tgt_rel.is_empty() {
                        let (src, tgt) = if let Some(prefix) = container_prefix {
                            (
                                format!("{}::{}", prefix, src_rel),
                                format!("{}::{}", prefix, tgt_rel),
                            )
                        } else {
                            (src_rel, tgt_rel)
                        };
                        add_edge_if_both_exist(g, uri, &src, &tgt, RelationshipKind::Transition);
                    }
                }
            }
            _ => {}
        }
    }
}

fn build_from_port_def_body_element(
    node: &sysml_parser::Node<PortDefBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    use sysml_parser::ast::PortDefBodyElement as PDBE;
    if let PDBE::PortUsage(n) = &node.value {
        let name = &n.name;
        let qualified = qualified_name_for_node(g, uri, container_prefix, name, "port");
        let range = span_to_range(&n.span);
        let mut attrs = HashMap::new();
        if let Some(ref t) = n.type_name {
            attrs.insert("portType".to_string(), serde_json::json!(t));
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
}

fn relationships_from_part_def(
    _pd_node: &sysml_parser::PartDef,
    _uri: &Url,
    _container_prefix: Option<&str>,
    _qualified: &str,
    _g: &mut SemanticGraph,
) {
    // Specializes edge added in build_from_package_body_element for PartDef
}

fn qualified_name(container_prefix: Option<&str>, name: &str) -> String {
    match container_prefix {
        Some(p) if !p.is_empty() => format!("{}::{}", p, name),
        _ => name.to_string(),
    }
}

/// Returns a qualified name that is unique among siblings. When a node with the same
/// base qualified name already exists (e.g. package and part def with same name), appends
/// #kind to disambiguate.
fn qualified_name_for_node(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    name: &str,
    kind: &str,
) -> String {
    let base = qualified_name(container_prefix, name);
    let kind_suffix = kind.replace(' ', "_");
    let node_id = NodeId::new(uri, &base);
    if g.node_index_by_id.contains_key(&node_id) {
        format!("{}#{}", base, kind_suffix)
    } else {
        base
    }
}

#[allow(clippy::too_many_arguments)]
fn add_node_and_recurse(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    kind: &str,
    name: String,
    range: Range,
    attrs: HashMap<String, serde_json::Value>,
    parent_id: Option<&NodeId>,
) {
    let node_id = NodeId::new(uri, qualified);
    let node = SemanticNode {
        id: node_id.clone(),
        element_kind: kind.to_string(),
        name,
        range,
        attributes: attrs,
        parent_id: parent_id.cloned(),
    };
    let idx = g.graph.add_node(node);
    g.node_index_by_id.insert(node_id.clone(), idx);
    g.nodes_by_uri.entry(uri.clone()).or_default().push(node_id);
}

/// Expands a typed PartUsage by adding nodes for the type's nested parts and ports.
fn expand_typed_part_usage(
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
fn expand_part_def_members(
    root: &RootNamespace,
    uri: &Url,
    container_qualified: &str,
    part_def: &sysml_parser::Node<sysml_parser::PartDef>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
    pkg_prefix: &str,
    expansion_stack: &mut Vec<String>,
) {
    if let PartDefBody::Brace { elements } = &part_def.body {
        for node in elements {
            use sysml_parser::ast::PartDefBodyElement as PDBE;
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
                    add_node_if_not_exists(
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
                    add_node_if_not_exists(
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
                    add_node_if_not_exists(
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

#[allow(clippy::too_many_arguments)]
fn add_node_if_not_exists(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    kind: &str,
    name: String,
    parent_id: &NodeId,
    source_range: Range,
    attrs: HashMap<String, serde_json::Value>,
) {
    let node_id = NodeId::new(uri, qualified);
    if g.node_index_by_id.contains_key(&node_id) {
        return;
    }
    let mut attrs = attrs;
    attrs.insert("synthetic".to_string(), serde_json::json!(true));
    attrs.insert(
        "originRange".to_string(),
        serde_json::json!({
            "start": {"line": source_range.start.line, "character": source_range.start.character},
            "end": {"line": source_range.end.line, "character": source_range.end.character}
        }),
    );
    let parent_range = g
        .get_node(parent_id)
        .map(|node| node.range)
        .unwrap_or(source_range);
    let node = SemanticNode {
        id: node_id.clone(),
        element_kind: kind.to_string(),
        name,
        range: parent_range,
        attributes: attrs,
        parent_id: Some(parent_id.clone()),
    };
    let idx = g.graph.add_node(node);
    g.node_index_by_id.insert(node_id.clone(), idx);
    g.nodes_by_uri.entry(uri.clone()).or_default().push(node_id);
}
