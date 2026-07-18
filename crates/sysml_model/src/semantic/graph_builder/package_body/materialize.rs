use super::*;
use crate::semantic::model::DeclaredFeatureProperties;

pub(super) fn materialize_part_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    pd_node: &Node<PartDef>,
) {
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
    attach_feature_properties(
        g,
        &node_id,
        definition_feature_properties(pd_node.definition_prefix.as_ref(), pd_node.is_individual),
    );
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        pd_node.specializes.as_deref(),
    );
    if let PartDefBody::Brace { elements } = &pd_node.body {
        for child in elements {
            part_def::build_from_part_def_body_element(child, uri, Some(&qualified), &node_id, g);
        }
    }
}

pub(super) fn materialize_feature_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    feature_node: &Node<FeatureDecl>,
) {
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
        let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "feature decl");
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

pub(super) fn materialize_classifier_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    classifier_node: &Node<ClassifierDecl>,
) {
    let cv = &classifier_node.value;
    let name = extract_modeled_decl_name(&cv.keyword, &cv.text, "_classifier");
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "classifier decl");
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

pub(crate) fn materialize_port_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    pd_node: &Node<PortDef>,
) {
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
            port_def::build_from_port_def_body_element(child, uri, Some(&qualified), &node_id, g);
        }
    }
}

pub(super) fn materialize_interface_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    id_node: &Node<InterfaceDef>,
) {
    let name = identification_name(&id_node.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "interface def");
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
        interface_def::build_from_interface_def_body(elements, uri, Some(&qualified), &node_id, g);
    }
}

pub(crate) fn materialize_attribute_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    ad_node: &Node<AttributeDef>,
) {
    let value = &ad_node.value;
    let name = &value.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "attribute def");
    let range = span_to_range(&ad_node.span);
    let mut attrs = HashMap::new();
    if let Some(t) = crate::semantic::ast_util::typing_target(value.typing.as_deref()) {
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
    let node_id = NodeId::new(uri, &qualified);
    attach_feature_properties(
        g,
        &node_id,
        crate::semantic::model::DeclaredFeatureProperties {
            is_ordered: Some(value.ordered),
            is_unique: Some(!value.nonunique),
            ..crate::semantic::model::DeclaredFeatureProperties::default()
        },
    );
    if let Some(value) = &value.value {
        if let Some(node) = g.get_node_mut(&node_id) {
            node.declared_facts.feature_value =
                Some(crate::semantic::ast_util::declared_feature_value(value));
        }
    }
    if let Some(t) = crate::semantic::ast_util::typing_target(value.typing.as_deref()) {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
}

pub(super) fn materialize_alias_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    alias_node: &Node<AliasDef>,
) {
    let mut name = identification_name(&alias_node.identification);
    if name.is_empty() {
        name = alias_node.target.to_display_string();
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "alias");
    let range = span_to_range(&alias_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &alias_node.identification);
    attrs.insert(
        "target".to_string(),
        serde_json::json!(alias_node.target.to_display_string()),
    );
    add_node_and_recurse(g, uri, &qualified, "alias", name, range, attrs, parent_id);
}

pub(super) fn materialize_requirement_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    rd_node: &Node<RequirementDef>,
) {
    let name = identification_name(&rd_node.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "requirement def");
    let range = span_to_range(&rd_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &rd_node.identification);
    insert_def_specialization_attr(&mut attrs, rd_node.specializes.as_deref());
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(rd_node.is_abstract),
    );
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

pub(super) fn materialize_satisfy(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    satisfy_node: &Node<sysml_v2_parser::ast::Satisfy>,
) {
    expressions::add_expression_edge_if_both_exist(
        g,
        uri,
        container_prefix,
        &satisfy_node.source,
        &satisfy_node.target,
        RelationshipKind::Satisfy,
    );
    if let Some(elements) = &satisfy_node.body_elements {
        super::super::requirement_body::walk_satisfy_constraint_elements(
            elements,
            uri,
            container_prefix,
            g,
        );
    }
}

pub(super) fn materialize_allocation_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    alloc_node: &Node<AllocationUsage>,
) {
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

pub(super) fn materialize_concern_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    cu_node: &Node<ConcernUsage>,
) {
    let name = &cu_node.name;
    // `concern_usage` (sysml-v2-parser) parses both `concern` and `concern def` into the same
    // `ConcernUsage` struct, distinguished only by `is_definition` -- there is no separate
    // `ConcernDef` AST node (see that field's doc comment). Classify accordingly, the same way
    // `case`/`case def` already do via two distinct materializers.
    let kind = if cu_node.is_definition {
        "concern def"
    } else {
        "concern"
    };
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, kind);
    let range = span_to_range(&cu_node.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = cu_node.type_name {
        attrs.insert("concernType".to_string(), serde_json::json!(t));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        kind,
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

pub(crate) fn materialize_use_case_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    ucd_node: &Node<UseCaseDef>,
) {
    let name = identification_name(&ucd_node.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "use case def");
    let range = span_to_range(&ucd_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &ucd_node.identification);
    insert_def_specialization_attr(&mut attrs, ucd_node.specializes.as_deref());
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(ucd_node.is_abstract),
    );
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

pub(crate) fn materialize_use_case_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    ucu_node: &Node<UseCaseUsage>,
) {
    let name = &ucu_node.name;
    let qualified = qualified_name_for_node(g, uri, container_prefix, name, "use case");
    let range = span_to_range(&ucu_node.span);
    let mut attrs = HashMap::new();
    if let Some(ref t) = ucu_node.type_name {
        attrs.insert("useCaseType".to_string(), serde_json::json!(t));
    }
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(ucu_node.is_abstract),
    );
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

pub(crate) fn materialize_item_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    item_node: &Node<ItemDef>,
) {
    let name = identification_name(&item_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "item def");
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
    attribute_body::build_from_attribute_body(&item_node.body, uri, Some(&qualified), &node_id, g);
}

pub(super) fn materialize_individual_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    ind_node: &Node<IndividualDef>,
) {
    let name = identification_name(&ind_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "individual def");
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
    attribute_body::build_from_attribute_body(&ind_node.body, uri, Some(&qualified), &node_id, g);
}

pub(super) fn materialize_metadata_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    md_node: &Node<MetadataDef>,
) {
    let name = identification_name(&md_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "metadata def");
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
    super::super::metadata_def::build_from_metadata_attribute_body(
        &md_node.body,
        uri,
        Some(&qualified),
        &node_id,
        g,
    );
}

pub(super) fn materialize_enum_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    enum_node: &Node<EnumDef>,
) {
    let name = identification_name(&enum_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "enum def");
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
    // Each enumerated value becomes its own addressable child node, now that the parser retains
    // a real span per value (`sysml-v2-parser` 0.39.0's `EnumeratedValue`) instead of a bare
    // string. Owned by the enclosing EnumDef via the normal parent_id membership mechanism.
    if let EnumerationBody::Brace { values } = &enum_node.body {
        materialize_enumerated_values(g, uri, &qualified, values);
    }
}

fn materialize_enumerated_values(
    g: &mut SemanticGraph,
    uri: &Url,
    enum_def_qualified: &str,
    values: &[Node<EnumeratedValue>],
) {
    let parent_id = NodeId::new(uri, enum_def_qualified);
    for value_node in values {
        let name = value_node.value.name.clone();
        if name.is_empty() {
            continue;
        }
        let qualified =
            qualified_name_for_node(g, uri, Some(enum_def_qualified), &name, "enumerated value");
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "enumerated value",
            name,
            span_to_range(&value_node.span),
            HashMap::new(),
            Some(&parent_id),
        );
    }
}

pub(super) fn materialize_enum_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    enum_node: &Node<EnumerationUsage>,
) {
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
        parent_id,
    );
    if let Some(ref t) = enum_node.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    let node_id = NodeId::new(uri, &qualified);
    attribute_body::build_from_attribute_body(&enum_node.body, uri, Some(&qualified), &node_id, g);
}

pub(super) fn materialize_occurrence_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    occ_node: &Node<OccurrenceDef>,
) {
    let name = identification_name(&occ_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "occurrence def");
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
    let node_id = NodeId::new(uri, &qualified);
    // `OccurrenceDef` has a plain `is_abstract: bool`, not the `DefinitionPrefix` enum
    // `definition_feature_properties` expects (that helper is for the `PartDef`-family shape),
    // so build declared properties directly -- `attach_feature_properties` is the mechanism
    // Babel42's `isAbstract` DTO field actually reads (`insert_definition_feature_properties`
    // falls back to a `definitionPrefix` string attribute otherwise, which occurrence never
    // sets); a raw `isAbstract` attribute alone is not consumed.
    attach_feature_properties(
        g,
        &node_id,
        DeclaredFeatureProperties {
            is_abstract: occ_node.is_abstract,
            ..DeclaredFeatureProperties::default()
        },
    );
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        occ_node.specializes.as_deref(),
    );
    definition_body::build_from_definition_body(&occ_node.body, uri, Some(&qualified), &node_id, g);
}

pub(super) fn materialize_connection_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    conn_node: &Node<ConnectionDef>,
) {
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
    let qualified = qualified_name_for_node(g, uri, container_prefix, base_name, "connection def");
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
        interface_def::build_from_connection_def_body(elements, uri, Some(&qualified), &node_id, g);
        if annotation == Some("derivation") {
            try_wire_derivation_connection(g, uri, &node_id);
        }
    }
}

pub(super) fn materialize_flow_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    flow_node: &Node<sysml_v2_parser::ast::FlowDef>,
) {
    let name = identification_name(&flow_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "flow def");
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

pub(super) fn materialize_allocation_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    alloc_node: &Node<AllocationDef>,
) {
    let name = identification_name(&alloc_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "allocation def");
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

pub(super) fn materialize_dependency(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    dep_node: &Node<Dependency>,
) {
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

// `pub(crate)` rather than `pub(super)`: `part_def.rs`'s `PDBE::CaseDef`/`::CaseUsage` arms
// (a sibling module under `graph_builder`, reached via the `pub(crate) use materialize::*`
// re-export in `package_body/mod.rs`) reuse these builders for `case`/`case def` nested in a
// `part def { ... }` body, instead of duplicating the body-walking logic.
pub(crate) fn materialize_case_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<CaseDef>,
) {
    let name = identification_name(&c_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "case def");
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &c_node.identification);
    insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(c_node.is_abstract),
    );
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
    let node_id = NodeId::new(uri, &qualified);
    wire_def_specialization_edge(
        g,
        uri,
        &qualified,
        container_prefix,
        c_node.specializes.as_deref(),
    );
    // Bug fix: unlike the sibling use_case/analysis_case/verification_case builders, this
    // never walked the body -- subject/actor/objective/include members were silently dropped.
    // `CaseDef::body` is the same `UseCaseDefBody` type `use_case_def` walks, so reuse its walker.
    if let UseCaseDefBody::Brace { elements } = &c_node.body {
        use_case::build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
    }
}

pub(crate) fn materialize_case_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<CaseUsage>,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, &c_node.name, "case");
    let mut attrs = HashMap::new();
    if let Some(ref t) = c_node.type_name {
        attrs.insert("caseType".to_string(), serde_json::json!(t));
    }
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(c_node.is_abstract),
    );
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "case",
        c_node.name.clone(),
        span_to_range(&c_node.span),
        attrs,
        parent_id,
    );
    // Bug fix: unlike the sibling analysis/verification/use-case usage builders, this never
    // wired a typing edge even though `CaseUsage.type_name` is captured by the parser.
    if let Some(ref t) = c_node.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, t, container_prefix);
    }
    let node_id = NodeId::new(uri, &qualified);
    // Same fix as materialize_case_def: this never walked the body before.
    if let UseCaseDefBody::Brace { elements } = &c_node.body {
        use_case::build_from_use_case_body(elements, uri, Some(&qualified), &node_id, g);
    }
}

pub(crate) fn materialize_analysis_case_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<AnalysisCaseDef>,
) {
    let name = identification_name(&c_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "analysis def");
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &c_node.identification);
    insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(c_node.is_abstract),
    );
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
    analysis_case::build_from_analysis_body(&c_node.body, uri, Some(&qualified), &node_id, g);
}

pub(crate) fn materialize_analysis_case_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<AnalysisCaseUsage>,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, &c_node.name, "analysis");
    let mut attrs = HashMap::new();
    if let Some(ref t) = c_node.type_name {
        attrs.insert("analysisType".to_string(), serde_json::json!(t));
    }
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(c_node.is_abstract),
    );
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
    analysis_case::build_from_analysis_body(&c_node.body, uri, Some(&qualified), &node_id, g);
}

pub(crate) fn materialize_verification_case_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<VerificationCaseDef>,
) {
    let name = identification_name(&c_node.identification);
    if name.is_empty() {
        return;
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "verification def");
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &c_node.identification);
    insert_def_specialization_attr(&mut attrs, c_node.specializes.as_deref());
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(c_node.is_abstract),
    );
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
    verification::build_from_verification_body(&c_node.body, uri, Some(&qualified), &node_id, g);
}

pub(crate) fn materialize_verification_case_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    c_node: &Node<VerificationCaseUsage>,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, &c_node.name, "verification");
    let mut attrs = HashMap::new();
    if let Some(ref t) = c_node.type_name {
        attrs.insert("verificationType".to_string(), serde_json::json!(t));
    }
    attrs.insert(
        "isAbstract".to_string(),
        serde_json::json!(c_node.is_abstract),
    );
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
    verification::build_from_verification_body(&c_node.body, uri, Some(&qualified), &node_id, g);
}

pub(super) fn materialize_actor(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    actor_node: &Node<sysml_v2_parser::ast::ActorDecl>,
) {
    let name = identification_name(&actor_node.identification);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "actor");
    let range = span_to_range(&actor_node.span);
    let mut attrs = HashMap::new();
    attach_short_name_attribute(&mut attrs, &actor_node.identification);
    add_node_and_recurse(g, uri, &qualified, "actor", name, range, attrs, parent_id);
}

pub(super) fn materialize_state_def(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    sd_node: &Node<StateDef>,
) {
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

pub(super) fn materialize_state_usage(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    su_node: &Node<StateUsage>,
) {
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

pub(super) fn materialize_import(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    imp: &Node<Import>,
) {
    let Some(pid) = parent_id else {
        return;
    };
    let v = &imp.value;
    let name = import_member_label(&v.target);
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "import");
    let mut attrs = HashMap::new();
    attrs.insert("importTarget".to_string(), serde_json::json!(&v.target));
    attrs.insert("importAll".to_string(), serde_json::json!(v.is_import_all));
    if let Some(vis) = &v.membership.visibility {
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

pub(super) fn materialize_textual_rep(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    t: &Node<TextualRepresentation>,
) {
    let Some(pid) = parent_id else {
        return;
    };
    let tr = &t.value;
    let name = tr
        .rep_identification
        .as_ref()
        .map(identification_name)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "_textualRep".to_string());
    let qualified = qualified_name_for_node(g, uri, container_prefix, &name, "textualRep");
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
