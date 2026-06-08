use std::collections::HashMap;
use std::fs;

use sysml_v2_parser::ast::{
    CalcDefBody, CalcDefBodyElement,
    ConnectionDefBody, ConstraintDefBody, ConstraintDefBodyElement, InOut, InterfaceDefBody,
    PackageBodyElement, PartDefBody, PartUsageBody, PortDefBody, RequirementDefBody, StateDefBody,
    UseCaseDefBody, ViewBody, ViewBodyElement, ViewDefBody, ViewDefBodyElement, ViewRenderingUsage,
};
use sysml_v2_parser::RootNamespace;
use url::Url;

use super::requirement_body::{import_member_label, walk_requirement_def_body};
use crate::semantic::ast_util::{identification_name, span_to_range, text_range_to_json};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::{
    add_specializes_edge_if_exists, add_typing_edge_if_exists,
    try_wire_derivation_connection,
};

use super::action;
use super::analysis_case;
use super::attribute_body;
use super::definition_body;
use super::expressions;
use super::modeled_kerml_name::extract_modeled_decl_name;
use super::package_packages;
use super::verification;
use super::{add_node_and_recurse, qualified_name_for_node};
use super::{interface_def, part_def, part_usage, port_def, state, stubs, use_case};

fn direction_to_str(direction: &InOut) -> &'static str {
    match direction {
        InOut::In => "in",
        InOut::Out => "out",
        InOut::InOut => "inout",
    }
}

fn add_view_filter_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    filter: &sysml_v2_parser::Node<sysml_v2_parser::ast::FilterMember>,
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
    attrs.insert("filterOwnerKind".to_string(), serde_json::json!(filter_owner_kind));
    if let Some(vis) = &filter.value.visibility {
        attrs.insert("visibility".to_string(), serde_json::json!(format!("{vis:?}")));
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

fn add_view_rendering_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    rendering: &sysml_v2_parser::Node<ViewRenderingUsage>,
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
        attrs.insert("renderingType".to_string(), serde_json::json!(rendering_type));
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

fn annotate_view_usage_body(
    g: &mut SemanticGraph,
    view_id: &NodeId,
    body: &ViewBody,
    uri: &Url,
) {
    let ViewBody::Brace { elements } = body else {
        return;
    };
    if let Some(view_node) = g.get_node_mut(view_id) {
        view_node
            .attributes
            .insert("hasViewBody".to_string(), serde_json::json!(true));
    }
    let mut has_expose = false;
    for element in elements {
        match &element.value {
            ViewBodyElement::Expose(_) => has_expose = true,
            ViewBodyElement::ViewRendering(rendering) => {
                add_view_rendering_node(g, uri, view_id, rendering);
            }
            ViewBodyElement::Filter(filter) => {
                add_view_filter_node(g, uri, view_id, filter, "view");
            }
            ViewBodyElement::Error(_)
            | ViewBodyElement::Other(_)
            | ViewBodyElement::Doc(_)
            | ViewBodyElement::Satisfy(_) => {}
        }
    }
    if has_expose {
        if let Some(view_node) = g.get_node_mut(view_id) {
            view_node
                .attributes
                .insert("hasExpose".to_string(), serde_json::json!(true));
        }
    }
}

fn compact_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn insert_def_specialization_attr(
    attrs: &mut HashMap<String, serde_json::Value>,
    specializes: Option<&str>,
) {
    if let Some(s) = specializes {
        attrs.insert("specializes".to_string(), serde_json::json!(s));
    }
}

fn wire_def_specialization_edge(
    g: &mut SemanticGraph,
    uri: &Url,
    qualified: &str,
    container_prefix: Option<&str>,
    specializes: Option<&str>,
) {
    if let Some(s) = specializes {
        add_specializes_edge_if_exists(g, uri, qualified, s, container_prefix);
    }
}

fn expression_text_from_span(uri: &Url, span: &sysml_v2_parser::Span, fallback: &str) -> String {
    let Some(path) = uri.to_file_path().ok() else {
        return fallback.to_string();
    };
    let Ok(content) = fs::read_to_string(path) else {
        return fallback.to_string();
    };
    let range = span_to_range(span);
    let start = range.start.line as usize;
    let end = range.end.line as usize;
    let lines: Vec<&str> = content.lines().collect();
    if start >= lines.len() || end >= lines.len() || start > end {
        return fallback.to_string();
    }
    compact_whitespace(&lines[start..=end].join(" "))
}

fn extract_constraint_metadata(
    uri: &Url,
    body: &ConstraintDefBody,
) -> (Vec<serde_json::Value>, Option<String>) {
    let mut params = Vec::new();
    let mut expression: Option<String> = None;
    if let ConstraintDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                ConstraintDefBodyElement::InOutDecl(param) => params.push(serde_json::json!({
                    "direction": direction_to_str(&param.value.direction),
                    "name": param.value.name,
                    "type": param.value.type_name,
                })),
                ConstraintDefBodyElement::Expression(expr) => {
                    let rendered = expression_text_from_span(
                        uri,
                        &expr.span,
                        &expressions::expression_to_debug_string(expr),
                    );
                    if !rendered.trim().is_empty() {
                        expression = Some(rendered);
                    }
                }
                ConstraintDefBodyElement::Error(_)
                | ConstraintDefBodyElement::Doc(_)
                | ConstraintDefBodyElement::Other(_) => {}
            }
        }
    }
    (params, expression)
}

fn strip_calc_return_expression(text: &str) -> String {
    text.trim()
        .strip_prefix("return")
        .map(str::trim)
        .unwrap_or(text.trim())
        .trim_end_matches(';')
        .trim()
        .to_string()
}

fn extract_calc_metadata(
    uri: &Url,
    body: &CalcDefBody,
) -> (
    Vec<serde_json::Value>,
    Option<serde_json::Value>,
    Option<String>,
) {
    let mut params = Vec::new();
    let mut return_decl: Option<serde_json::Value> = None;
    let mut expression: Option<String> = None;
    if let CalcDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                CalcDefBodyElement::InOutDecl(param) => params.push(serde_json::json!({
                    "direction": direction_to_str(&param.value.direction),
                    "name": param.value.name,
                    "type": param.value.type_name,
                })),
                CalcDefBodyElement::ReturnDecl(ret) => {
                    return_decl = Some(serde_json::json!({
                        "name": ret.value.name,
                        "type": ret.value.type_name,
                    }));
                }
                CalcDefBodyElement::Expression(expr) => {
                    let rendered = expression_text_from_span(
                        uri,
                        &expr.span,
                        &expressions::expression_to_debug_string(expr),
                    );
                    let rendered = strip_calc_return_expression(&rendered);
                    if !rendered.is_empty() {
                        expression = Some(rendered);
                    }
                }
                CalcDefBodyElement::Other(preview) => {
                    let rendered = strip_calc_return_expression(preview);
                    if expression.is_none() && !rendered.is_empty() {
                        expression = Some(rendered);
                    }
                }
                CalcDefBodyElement::Error(_) | CalcDefBodyElement::Doc(_) => {}
            }
        }
    }
    (params, return_decl, expression)
}

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
            stubs::relationships_from_part_def(pd_node, uri, container_prefix, &qualified, g);
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
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                pd_node.specializes.as_deref(),
            );
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
                        g,
                    );
                }
            }
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
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                pd_node.specializes.as_deref(),
            );
        }
        PBE::InterfaceDef(id_node) => {
            let name = identification_name(&id_node.identification);
            let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "interface");
            let range = span_to_range(&id_node.span);
            let mut attrs = HashMap::new();
            insert_def_specialization_attr(&mut attrs, id_node.specializes.as_deref());
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "interface",
                name.clone(),
                range,
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            if let InterfaceDefBody::Brace { elements } = &id_node.body {
                interface_def::build_from_interface_def_body(
                    elements,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
            }
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                id_node.specializes.as_deref(),
            );
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
            action::materialize_top_level_action_usage(g, uri, container_prefix, parent_id, au_node);
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
            let mut attrs = HashMap::new();
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
            walk_requirement_def_body(
                g,
                uri,
                container_prefix,
                &qualified,
                &node_id,
                &rd_node.body,
            );
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                rd_node.specializes.as_deref(),
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
            if let UseCaseDefBody::Brace { elements } = &ucd_node.body {
                use_case::build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
            }
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                ucd_node.specializes.as_deref(),
            );
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
                super::metadata_def::build_from_metadata_attribute_body(
                    &md_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
                );
                wire_def_specialization_edge(
                    g,
                    uri,
                    &qualified,
                    container_prefix,
                    md_node.specializes.as_deref(),
                );
            }
        }
        PBE::MetadataUsage(mu_node) => {
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &mu_node.name, "metadata usage");
            let mut attrs = HashMap::new();
            if let Some(ref t) = mu_node.type_name {
                attrs.insert("metadataType".to_string(), serde_json::json!(t));
            }
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "metadata usage",
                mu_node.name.clone(),
                span_to_range(&mu_node.span),
                attrs,
                parent_id,
            );
            let node_id = NodeId::new(uri, &qualified);
            if let Some(ref t) = mu_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            super::metadata_def::build_from_metadata_attribute_body(
                &mu_node.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PBE::EnumDef(enum_node) => {
            let name = identification_name(&enum_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "enum def");
                let mut attrs = HashMap::new();
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
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                conn_node.specializes.as_deref(),
            );
        }
        PBE::FlowDef(flow_node) => {
            let name = identification_name(&flow_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "flow def");
                let mut attrs = HashMap::new();
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
            let node_id = NodeId::new(uri, &qualified);
            definition_body::build_from_definition_body(
                &flow_node.body,
                uri,
                Some(&qualified),
                &node_id,
                g,
            );
        }
        PBE::AllocationDef(alloc_node) => {
            let name = identification_name(&alloc_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "allocation def");
                let mut attrs = HashMap::new();
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
                let (params, expression) = extract_constraint_metadata(uri, &c_node.body);
                let mut attrs = HashMap::new();
                attrs.insert(
                    "analysisKind".to_string(),
                    serde_json::json!("constraint_def"),
                );
                attrs.insert(
                    "analysisParams".to_string(),
                    serde_json::Value::Array(params),
                );
                if let Some(expr) = expression {
                    attrs.insert("analysisExpression".to_string(), serde_json::json!(expr));
                }
                insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "constraint def",
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
        PBE::CalcDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "calc def");
                let (params, return_decl, expression) = extract_calc_metadata(uri, &c_node.body);
                let mut attrs = HashMap::new();
                attrs.insert("analysisKind".to_string(), serde_json::json!("calc_def"));
                let params_json = serde_json::Value::Array(params.clone());
                attrs.insert("analysisParams".to_string(), params_json.clone());
                attrs.insert("parameters".to_string(), params_json);
                if let Some(ret) = return_decl {
                    attrs.insert("analysisReturn".to_string(), ret);
                }
                if let Some(expr) = expression {
                    attrs.insert("analysisExpression".to_string(), serde_json::json!(expr));
                }
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "calc def",
                    name,
                    span_to_range(&c_node.span),
                    attrs,
                    parent_id,
                );
            }
        }
        PBE::CaseDef(c_node) => {
            let name = identification_name(&c_node.identification);
            if !name.is_empty() {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &name, "case def");
                let mut attrs = HashMap::new();
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
                analysis_case::build_from_analysis_body(
                    &c_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
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
                verification::build_from_verification_body(
                    &c_node.body,
                    uri,
                    Some(&qualified),
                    &node_id,
                    g,
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
            let mut attrs = HashMap::new();
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
            if let StateDefBody::Brace { elements } = &sd_node.body {
                state::build_from_state_body(elements, uri, Some(&qualified), &node_id, g);
            }
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                sd_node.specializes.as_deref(),
            );
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
            let mut attrs = HashMap::new();
            insert_def_specialization_attr(&mut attrs, vd_node.specializes.as_deref());
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
            if let ViewDefBody::Brace { elements } = &vd_node.body {
                for element in elements {
                    match &element.value {
                        ViewDefBodyElement::Filter(filter) => {
                            add_view_filter_node(g, uri, &view_def_id, filter, "view def");
                        }
                        ViewDefBodyElement::ViewRendering(rendering) => {
                            add_view_rendering_node(g, uri, &view_def_id, rendering);
                        }
                        ViewDefBodyElement::Error(_)
                        | ViewDefBodyElement::Other(_)
                        | ViewDefBodyElement::Doc(_) => {}
                    }
                }
            }
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                vd_node.specializes.as_deref(),
            );
        }
        PBE::ViewpointDef(vpd_node) => {
            let name = identification_name(&vpd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "viewpoint def");
            let range = span_to_range(&vpd_node.span);
            let mut attrs = HashMap::new();
            insert_def_specialization_attr(&mut attrs, vpd_node.specializes.as_deref());
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
            if let RequirementDefBody::Brace { .. } = &vpd_node.body {
                walk_requirement_def_body(
                    g,
                    uri,
                    container_prefix,
                    &qualified,
                    &viewpoint_def_id,
                    &vpd_node.body,
                );
            }
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                vpd_node.specializes.as_deref(),
            );
        }
        PBE::RenderingDef(rd_node) => {
            let name = identification_name(&rd_node.identification);
            let qualified =
                qualified_name_for_node(g, uri, container_prefix, &name, "rendering def");
            let range = span_to_range(&rd_node.span);
            let mut attrs = HashMap::new();
            insert_def_specialization_attr(&mut attrs, rd_node.specializes.as_deref());
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
                rd_node.specializes.as_deref(),
            );
        }
        PBE::ViewUsage(vu_node) => {
            let name = &vu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "view");
            let range = span_to_range(&vu_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = vu_node.type_name {
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
            if let Some(ref t) = vu_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
            annotate_view_usage_body(g, &view_id, &vu_node.body, uri);
        }
        PBE::ViewpointUsage(vpu_node) => {
            let name = &vpu_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "viewpoint");
            let range = span_to_range(&vpu_node.span);
            let mut attrs = HashMap::new();
            attrs.insert(
                "viewpointType".to_string(),
                serde_json::json!(vpu_node.type_name.as_str()),
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
                vpu_node.type_name.as_str(),
                container_prefix,
            );
            let viewpoint_id = NodeId::new(uri, &qualified);
            walk_requirement_def_body(
                g,
                uri,
                container_prefix,
                &qualified,
                &viewpoint_id,
                &vpu_node.body,
            );
        }
        PBE::RenderingUsage(ru_node) => {
            let name = &ru_node.name;
            let qualified = qualified_name_for_node(g, uri, container_prefix, name, "rendering");
            let range = span_to_range(&ru_node.span);
            let mut attrs = HashMap::new();
            if let Some(ref t) = ru_node.type_name {
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
            if let Some(ref t) = ru_node.type_name {
                add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
            }
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
            if let Some(pid) = parent_id {
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, "_filter", "filter");
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
