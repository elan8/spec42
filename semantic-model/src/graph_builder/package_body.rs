use std::collections::HashMap;

use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, ConnectionDefBody, InOut, InterfaceDefBody, PackageBody,
    PackageBodyElement, PartDefBody, PartUsageBody, PortDefBody, StateDefBody, UseCaseDefBody,
};
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use super::requirement_body::{import_member_label, walk_requirement_def_body};
use crate::ast_util::{identification_name, span_to_range};
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::relationships::{
    add_edge_if_both_exist, add_specializes_edge_if_exists, add_typing_edge_if_exists,
    normalize_for_lookup,
};

use super::expressions;
use super::modeled_kerml_name::extract_modeled_decl_name;
use super::{add_node_and_recurse, qualified_name_for_node};
use super::{interface_def, part_def, part_usage, port_def, state, stubs, use_case};

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
            let name = identification_name(&pkg_node.identification);
            let name_display = if name.is_empty() {
                "(top level)"
            } else {
                name.as_str()
            };
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, name_display, "package");
            let node_id = NodeId::new(uri, &qualified);
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "package",
                name_display.to_string(),
                span_to_range(&pkg_node.span),
                HashMap::new(),
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
            attrs.insert("ordered".to_string(), serde_json::json!(pu_node.ordered));
            if let Some((ref feat, ref val)) = pu_node.subsets {
                attrs.insert("subsetsFeature".to_string(), serde_json::json!(feat));
                if let Some(v) = val {
                    attrs.insert(
                        "subsetsValue".to_string(),
                        serde_json::json!(expressions::expression_to_debug_string(v)),
                    );
                }
            }
            if let Some(ref r) = pu_node.redefines {
                attrs.insert("redefines".to_string(), serde_json::json!(r));
            }
            if let Some(ref v) = pu_node.value.value {
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
        PBE::FeatureDecl(feature_node) => {
            let fv = &feature_node.value;
            let name = extract_modeled_decl_name(&fv.keyword, &fv.text, "_feature");
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
            if let Some(ref s) = pd_node.specializes {
                attrs.insert("specializes".to_string(), serde_json::json!(s));
            }
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
            if let Some(ref s) = pd_node.specializes {
                add_specializes_edge_if_exists(g, uri, &qualified, s, container_prefix);
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
            let node_id = NodeId::new(uri, &qualified);
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                for el in elements {
                    interface_def::build_from_interface_def_body_element(
                        el,
                        uri,
                        Some(&qualified),
                        &node_id,
                        g,
                    );
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
            let action_id = NodeId::new(uri, &qualified);
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
            if let ActionDefBody::Brace { elements } = &ad_node.body {
                for element in elements {
                    match &element.value {
                        ActionDefBodyElement::InOutDecl(in_out) => {
                            let parameter = &in_out.value;
                            let child_qualified = qualified_name_for_node(
                                g,
                                uri,
                                Some(&qualified),
                                &parameter.name,
                                "in out parameter",
                            );
                            let mut attrs = HashMap::new();
                            attrs.insert(
                                "direction".to_string(),
                                serde_json::json!(match parameter.direction {
                                    InOut::In => "in",
                                    InOut::Out => "out",
                                    InOut::InOut => "inout",
                                }),
                            );
                            attrs.insert(
                                "parameterType".to_string(),
                                serde_json::json!(&parameter.type_name),
                            );
                            add_node_and_recurse(
                                g,
                                uri,
                                &child_qualified,
                                "in out parameter",
                                parameter.name.clone(),
                                span_to_range(&in_out.span),
                                attrs,
                                Some(&action_id),
                            );
                            add_typing_edge_if_exists(
                                g,
                                uri,
                                &child_qualified,
                                &parameter.type_name,
                                Some(&qualified),
                            );
                        }
                        ActionDefBodyElement::Perform(perform) => {
                            let step_name = if perform.value.action_name.trim().is_empty() {
                                perform
                                    .value
                                    .type_name
                                    .clone()
                                    .unwrap_or_else(|| "perform".to_string())
                            } else {
                                perform.value.action_name.clone()
                            };
                            let child_qualified = qualified_name_for_node(
                                g,
                                uri,
                                Some(&qualified),
                                &step_name,
                                "perform",
                            );
                            let mut attrs = HashMap::new();
                            if let Some(ref action_type) = perform.value.type_name {
                                attrs.insert(
                                    "actionType".to_string(),
                                    serde_json::json!(action_type),
                                );
                            }
                            add_node_and_recurse(
                                g,
                                uri,
                                &child_qualified,
                                "perform",
                                step_name,
                                span_to_range(&perform.span),
                                attrs,
                                Some(&action_id),
                            );
                            if let Some(ref action_type) = perform.value.type_name {
                                add_typing_edge_if_exists(
                                    g,
                                    uri,
                                    &child_qualified,
                                    action_type,
                                    Some(&qualified),
                                );
                            }
                        }
                        ActionDefBodyElement::Bind(bind) => {
                            expressions::add_expression_edge_if_both_exist(
                                g,
                                uri,
                                Some(&qualified),
                                &bind.value.left,
                                &bind.value.right,
                                RelationshipKind::Bind,
                            );
                        }
                        ActionDefBodyElement::Flow(flow) => {
                            expressions::add_expression_edge_if_both_exist(
                                g,
                                uri,
                                Some(&qualified),
                                &flow.value.from,
                                &flow.value.to,
                                RelationshipKind::Flow,
                            );
                        }
                        ActionDefBodyElement::FirstStmt(first) => {
                            expressions::add_expression_edge_if_both_exist(
                                g,
                                uri,
                                Some(&qualified),
                                &first.value.first,
                                &first.value.then,
                                RelationshipKind::Flow,
                            );
                        }
                        ActionDefBodyElement::MergeStmt(merge) => {
                            let merge_target =
                                expressions::expression_to_debug_string(&merge.value.merge);
                            let child_qualified = qualified_name_for_node(
                                g,
                                uri,
                                Some(&qualified),
                                &merge_target,
                                "merge",
                            );
                            let mut attrs = HashMap::new();
                            attrs
                                .insert("mergeTarget".to_string(), serde_json::json!(merge_target));
                            add_node_and_recurse(
                                g,
                                uri,
                                &child_qualified,
                                "merge",
                                "merge".to_string(),
                                span_to_range(&merge.span),
                                attrs,
                                Some(&action_id),
                            );
                        }
                        ActionDefBodyElement::ActionUsage(action_usage) => {
                            let au_node = action_usage.as_ref();
                            let name = &au_node.name;
                            let child_qualified =
                                qualified_name_for_node(g, uri, Some(&qualified), name, "action");
                            let mut attrs = HashMap::new();
                            attrs.insert(
                                "actionType".to_string(),
                                serde_json::json!(&au_node.type_name),
                            );
                            if let Some((ref accept_name, ref accept_type)) = au_node.accept {
                                attrs.insert(
                                    "acceptName".to_string(),
                                    serde_json::json!(accept_name),
                                );
                                attrs.insert(
                                    "acceptType".to_string(),
                                    serde_json::json!(accept_type),
                                );
                            }
                            add_node_and_recurse(
                                g,
                                uri,
                                &child_qualified,
                                "action",
                                name.clone(),
                                span_to_range(&au_node.span),
                                attrs,
                                Some(&action_id),
                            );
                            add_typing_edge_if_exists(
                                g,
                                uri,
                                &child_qualified,
                                &au_node.type_name,
                                Some(&qualified),
                            );
                            add_edge_if_both_exist(
                                g,
                                uri,
                                &action_id.qualified_name,
                                &child_qualified,
                                RelationshipKind::Perform,
                            );
                        }
                        ActionDefBodyElement::Doc(_) | ActionDefBodyElement::Error(_) => {}
                        _ => {}
                    }
                }
            }
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
            let node_id = NodeId::new(uri, &qualified);
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
            let node_id = NodeId::new(uri, &qualified);
            walk_requirement_def_body(
                g,
                uri,
                container_prefix,
                &qualified,
                &node_id,
                &ru_node.body,
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
            if let Some(annotation) = annotation {
                attrs.insert(
                    "connectionAnnotation".to_string(),
                    serde_json::json!(annotation),
                );
            }
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
            if let ConnectionDefBody::Brace { elements } = &conn_node.body {
                interface_def::build_from_connection_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
                if annotation == Some("derivation") {
                    let original_end = g.child_named(&node_id, "#original").into_iter().next();
                    let derived_end = g.child_named(&node_id, "#derive").into_iter().next();
                    let original_target = original_end.and_then(|node| {
                        g.outgoing_targets_by_kind(node, RelationshipKind::Typing)
                            .into_iter()
                            .next()
                            .map(|target| target.id.clone())
                    });
                    let derived_target = derived_end.and_then(|node| {
                        g.outgoing_targets_by_kind(node, RelationshipKind::Typing)
                            .into_iter()
                            .next()
                            .map(|target| target.id.clone())
                    });
                    if let (Some(original_target), Some(derived_target)) =
                        (original_target, derived_target)
                    {
                        if let (Some(&src_idx), Some(&tgt_idx)) = (
                            g.node_index_by_id.get(&original_target),
                            g.node_index_by_id.get(&derived_target),
                        ) {
                            g.graph
                                .add_edge(src_idx, tgt_idx, RelationshipKind::Derivation);
                        }
                        if let Some(connection) = g.get_node_mut(&node_id) {
                            connection.attributes.insert(
                                "derivationOriginal".to_string(),
                                serde_json::json!(normalize_for_lookup(
                                    &original_target.qualified_name
                                )),
                            );
                            connection.attributes.insert(
                                "derivationDerived".to_string(),
                                serde_json::json!(normalize_for_lookup(
                                    &derived_target.qualified_name
                                )),
                            );
                        }
                    }
                }
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
        PBE::Error(_) | PBE::Doc(_) | PBE::Comment(_) => {}
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
                attrs.insert("language".to_string(), serde_json::json!(&tr.language));
                attrs.insert("text".to_string(), serde_json::json!(&tr.text));
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
            if let Some(pid) = parent_id {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, "_filter", "filter");
                let mut attrs = HashMap::new();
                attrs.insert(
                    "condition".to_string(),
                    serde_json::json!(expressions::expression_to_debug_string(&f.value.condition)),
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
        }
        PBE::KermlSemanticDecl(k) => {
            if let Some(pid) = parent_id {
                let kv = &k.value;
                let display_name =
                    extract_modeled_decl_name(&kv.bnf_production, &kv.text, "_kermlSemantic");
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &display_name, "kermlDecl");
                let mut attrs = HashMap::new();
                attrs.insert(
                    "bnfProduction".to_string(),
                    serde_json::json!(&kv.bnf_production),
                );
                attrs.insert("text".to_string(), serde_json::json!(&kv.text));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "kermlDecl",
                    display_name,
                    span_to_range(&k.span),
                    attrs,
                    Some(pid),
                );
            }
        }
        PBE::KermlFeatureDecl(k) => {
            if let Some(pid) = parent_id {
                let kv = &k.value;
                let display_name =
                    extract_modeled_decl_name(&kv.bnf_production, &kv.text, "_kermlFeature");
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &display_name, "kermlDecl");
                let mut attrs = HashMap::new();
                attrs.insert(
                    "bnfProduction".to_string(),
                    serde_json::json!(&kv.bnf_production),
                );
                attrs.insert("text".to_string(), serde_json::json!(&kv.text));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "kermlDecl",
                    display_name,
                    span_to_range(&k.span),
                    attrs,
                    Some(pid),
                );
            }
        }
        PBE::ExtendedLibraryDecl(k) => {
            if let Some(pid) = parent_id {
                let kv = &k.value;
                let display_name =
                    extract_modeled_decl_name(&kv.bnf_production, &kv.text, "_extendedLibrary");
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &display_name, "kermlDecl");
                let mut attrs = HashMap::new();
                attrs.insert(
                    "bnfProduction".to_string(),
                    serde_json::json!(&kv.bnf_production),
                );
                attrs.insert("text".to_string(), serde_json::json!(&kv.text));
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "kermlDecl",
                    display_name,
                    span_to_range(&k.span),
                    attrs,
                    Some(pid),
                );
            }
        }
    }
}
