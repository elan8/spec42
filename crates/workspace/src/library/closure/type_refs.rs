use super::*;

/// An arena-backed type reference, as authored.
fn reference_text(
    document: &ParsedRoot,
    reference: Option<sysml_v2_parser::next::QualifiedReferenceId>,
) -> Option<String> {
    document
        .qualified_reference(reference?)
        .map(|view| view.authored_text().to_string())
}

/// A specialization clause's target, as authored.
///
/// Both clause kinds hold `QualifiedReferenceId`s now, so neither can answer "what does this name"
/// without the document that owns the arena.
fn subsetting_target<'a>(
    document: &'a ParsedRoot,
    relationship: Option<&sysml_v2_parser::next::ast::SubsettingRelationship>,
) -> Option<&'a str> {
    let target = relationship?.target.first().copied()?;
    document
        .qualified_reference(target)
        .map(|view| view.authored_text())
}

fn typing_target_display(
    document: &ParsedRoot,
    relationship: Option<&sysml_v2_parser::next::ast::TypingRelationship>,
) -> Option<String> {
    let target = relationship?.target.first().copied()?;
    document
        .qualified_reference(target)
        .map(|view| view.authored_text().to_string())
}

pub(crate) fn collect_type_reference_targets_from_content(content: &str) -> Vec<String> {
    let Ok(parsed) = sysml_v2_parser::next::parse(content) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_type_reference_targets_from_root(&parsed, &mut out);
    out
}

pub(crate) fn collect_type_reference_targets_from_root(
    document: &ParsedRoot,
    out: &mut Vec<String>,
) {
    for element in &document.elements {
        match &element.value {
            RootElement::Package(package) => walk_package_type_refs(document, package, out),
            RootElement::LibraryPackage(package) => {
                walk_library_package_type_refs(document, package, out)
            }
            _ => {}
        }
    }
}

pub(crate) fn collect_type_reference_targets_from_package_body(
    document: &ParsedRoot,
    body: &PackageBody,
    out: &mut Vec<String>,
) {
    let PackageBody::Brace { elements, .. } = body else {
        return;
    };
    for member in elements {
        walk_package_body_element_type_refs(document, &member.value, out);
    }
}

pub(crate) fn walk_package_type_refs(
    document: &ParsedRoot,
    package: &Node<Package>,
    out: &mut Vec<String>,
) {
    collect_type_reference_targets_from_package_body(document, &package.value.body, out);
}

pub(crate) fn walk_library_package_type_refs(
    document: &ParsedRoot,
    package: &Node<LibraryPackage>,
    out: &mut Vec<String>,
) {
    collect_type_reference_targets_from_package_body(document, &package.value.body, out);
}

pub(crate) fn walk_package_body_element_type_refs(
    document: &ParsedRoot,
    element: &PackageBodyElement,
    out: &mut Vec<String>,
) {
    match element {
        PackageBodyElement::Package(nested) => walk_package_type_refs(document, nested, out),
        PackageBodyElement::LibraryPackage(nested) => {
            walk_library_package_type_refs(document, nested, out)
        }
        PackageBodyElement::PartDef(part_def) => {
            walk_part_def_type_refs(document, &part_def.value, out)
        }
        PackageBodyElement::PartUsage(part_usage) => {
            walk_part_usage_type_refs(document, &part_usage.value, out)
        }
        PackageBodyElement::PortDef(port_def) => {
            walk_port_def_type_refs(document, &port_def.value, out)
        }
        PackageBodyElement::ItemDef(item_def) => {
            push_optional_typing_reference(document, item_def.value.specializes.as_deref(), out);
        }
        PackageBodyElement::MetadataDef(metadata_def) => {
            walk_metadata_def_type_refs(document, &metadata_def.value, out);
        }
        PackageBodyElement::MetadataUsage(metadata_usage) => {
            walk_metadata_usage_type_refs(document, &metadata_usage.value, out);
        }
        PackageBodyElement::ViewUsage(view) => {
            push_optional_type_reference(
                reference_text(document, view.value.type_name).as_deref(),
                out,
            );
        }
        _ => {}
    }
}

pub(crate) fn walk_part_def_type_refs(
    document: &ParsedRoot,
    part_def: &PartDef,
    out: &mut Vec<String>,
) {
    push_optional_typing_reference(document, part_def.specializes.as_deref(), out);
    let PartDefBody::Brace { elements, .. } = &part_def.body else {
        return;
    };
    for member in elements {
        walk_part_def_body_element_type_refs(document, &member.value, out);
    }
}

pub(crate) fn walk_part_def_body_element_type_refs(
    document: &ParsedRoot,
    element: &PartDefBodyElement,
    out: &mut Vec<String>,
) {
    match element {
        PartDefBodyElement::PartDef(part_def) => {
            walk_part_def_type_refs(document, &part_def.value, out)
        }
        PartDefBodyElement::PartUsage(part_usage) => {
            walk_part_usage_type_refs(document, &part_usage.value, out);
        }
        PartDefBodyElement::PortUsage(port_usage) => {
            walk_port_usage_type_refs(document, &port_usage.value, out)
        }
        PartDefBodyElement::AttributeDef(attribute_def) => {
            walk_attribute_def_type_refs(document, &attribute_def.value, out);
        }
        PartDefBodyElement::AttributeUsage(attribute_usage) => {
            walk_attribute_usage_type_refs(document, &attribute_usage.value, out);
        }
        PartDefBodyElement::ItemDef(item_def) => {
            push_optional_typing_reference(document, item_def.value.specializes.as_deref(), out);
        }
        PartDefBodyElement::ItemUsage(item_usage) => {
            walk_item_usage_type_refs(document, &item_usage.value, out);
        }
        PartDefBodyElement::Ref(ref_decl) => {
            walk_ref_decl_type_refs(document, &ref_decl.value, out)
        }
        PartDefBodyElement::ExhibitState(exhibit_state) => {
            push_optional_type_reference(
                typing_target_display(document, exhibit_state.value.typing.as_deref()).as_deref(),
                out,
            );
        }
        PartDefBodyElement::Connection(connection) => {
            push_optional_type_reference(
                reference_text(document, connection.value.type_reference).as_deref(),
                out,
            );
            push_optional_type_reference(
                subsetting_target(document, connection.value.subsets.as_deref()),
                out,
            );
            push_optional_type_reference(
                subsetting_target(document, connection.value.redefines.as_deref()),
                out,
            );
        }
        _ => {}
    }
}

pub(crate) fn walk_part_usage_type_refs(
    document: &ParsedRoot,
    part_usage: &PartUsage,
    out: &mut Vec<String>,
) {
    push_optional_type_reference(
        typing_target_display(document, part_usage.typing.as_deref()).as_deref(),
        out,
    );
    push_optional_type_reference(
        subsetting_target(document, part_usage.redefines.as_deref()),
        out,
    );
    if let Some((subsets, _)) = &part_usage.subsets {
        for target in &subsets.value.target {
            push_optional_type_reference(reference_text(document, Some(*target)).as_deref(), out);
        }
    }
    let PartUsageBody::Brace { elements, .. } = &part_usage.body else {
        return;
    };
    for member in elements {
        walk_part_usage_body_element_type_refs(document, &member.value, out);
    }
}

pub(crate) fn walk_part_usage_body_element_type_refs(
    document: &ParsedRoot,
    element: &PartUsageBodyElement,
    out: &mut Vec<String>,
) {
    match element {
        PartUsageBodyElement::PartUsage(part_usage) => {
            walk_part_usage_type_refs(document, &part_usage.value, out);
        }
        PartUsageBodyElement::PortUsage(port_usage) => {
            walk_port_usage_type_refs(document, &port_usage.value, out)
        }
        PartUsageBodyElement::AttributeUsage(attribute_usage) => {
            walk_attribute_usage_type_refs(document, &attribute_usage.value, out);
        }
        PartUsageBodyElement::Ref(ref_decl) => {
            walk_ref_decl_type_refs(document, &ref_decl.value, out)
        }
        _ => {}
    }
}

pub(crate) fn walk_port_def_type_refs(
    document: &ParsedRoot,
    port_def: &PortDef,
    out: &mut Vec<String>,
) {
    push_optional_typing_reference(document, port_def.specializes.as_deref(), out);
    let PortDefBody::Brace { elements, .. } = &port_def.body else {
        return;
    };
    for member in elements {
        match &member.value {
            PortDefBodyElement::PortUsage(port_usage) => {
                walk_port_usage_type_refs(document, &port_usage.value, out);
            }
            PortDefBodyElement::AttributeDef(attribute_def) => {
                walk_attribute_def_type_refs(document, &attribute_def.value, out);
            }
            PortDefBodyElement::AttributeUsage(attribute_usage) => {
                walk_attribute_usage_type_refs(document, &attribute_usage.value, out);
            }
            PortDefBodyElement::ItemUsage(item_usage) => {
                walk_item_usage_type_refs(document, &item_usage.value, out);
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_port_usage_type_refs(
    document: &ParsedRoot,
    port_usage: &PortUsage,
    out: &mut Vec<String>,
) {
    push_optional_type_reference(
        typing_target_display(document, port_usage.typing.as_deref()).as_deref(),
        out,
    );
    push_optional_type_reference(
        subsetting_target(document, port_usage.redefines.as_deref()),
        out,
    );
    push_optional_type_reference(
        subsetting_target(document, port_usage.references.as_deref()),
        out,
    );
    push_optional_type_reference(
        subsetting_target(document, port_usage.crosses.as_deref()),
        out,
    );
    if let Some((subsets, _)) = &port_usage.subsets {
        for target in &subsets.value.target {
            push_optional_type_reference(reference_text(document, Some(*target)).as_deref(), out);
        }
    }
    let PortBody::Brace { elements, .. } = &port_usage.body else {
        return;
    };
    for member in elements {
        if let PortBodyElement::PortUsage(nested) = &member.value {
            walk_port_usage_type_refs(document, &nested.value, out);
        }
    }
}

pub(crate) fn walk_attribute_def_type_refs(
    document: &ParsedRoot,
    attribute_def: &AttributeDef,
    out: &mut Vec<String>,
) {
    push_optional_typing_reference(document, attribute_def.typing.as_deref(), out);
    walk_attribute_body_type_refs(document, &attribute_def.body, out);
}

pub(crate) fn walk_attribute_usage_type_refs(
    document: &ParsedRoot,
    attribute_usage: &AttributeUsage,
    out: &mut Vec<String>,
) {
    push_optional_typing_reference(document, attribute_usage.typing.as_deref(), out);
    push_optional_type_reference(
        subsetting_target(document, attribute_usage.redefines.as_deref()),
        out,
    );
    push_optional_type_reference(
        subsetting_target(document, attribute_usage.references.as_deref()),
        out,
    );
    push_optional_type_reference(
        subsetting_target(document, attribute_usage.crosses.as_deref()),
        out,
    );
    walk_attribute_body_type_refs(document, &attribute_usage.body, out);
}

pub(crate) fn walk_attribute_body_type_refs(
    document: &ParsedRoot,
    body: &AttributeBody,
    out: &mut Vec<String>,
) {
    let AttributeBody::Brace { elements, .. } = body else {
        return;
    };
    for member in elements {
        match &member.value {
            AttributeBodyElement::AttributeDef(attribute_def) => {
                walk_attribute_def_type_refs(document, &attribute_def.value, out);
            }
            AttributeBodyElement::AttributeUsage(attribute_usage) => {
                walk_attribute_usage_type_refs(document, &attribute_usage.value, out);
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_item_usage_type_refs(
    document: &ParsedRoot,
    item_usage: &ItemUsage,
    out: &mut Vec<String>,
) {
    push_optional_type_reference(
        reference_text(document, item_usage.type_name).as_deref(),
        out,
    );
    walk_attribute_body_type_refs(document, &item_usage.body, out);
}

pub(crate) fn walk_ref_decl_type_refs(
    document: &ParsedRoot,
    ref_decl: &RefDecl,
    out: &mut Vec<String>,
) {
    push_optional_type_reference(
        typing_target_display(document, ref_decl.typing.as_deref()).as_deref(),
        out,
    );
}

pub(crate) fn walk_metadata_def_type_refs(
    document: &ParsedRoot,
    metadata_def: &MetadataDef,
    out: &mut Vec<String>,
) {
    push_optional_typing_reference(document, metadata_def.specializes.as_deref(), out);
    walk_attribute_body_type_refs(document, &metadata_def.body, out);
}

pub(crate) fn walk_metadata_usage_type_refs(
    document: &ParsedRoot,
    metadata_usage: &MetadataUsage,
    out: &mut Vec<String>,
) {
    push_optional_type_reference(
        reference_text(document, metadata_usage.type_reference).as_deref(),
        out,
    );
    for target in &metadata_usage.about_targets {
        push_optional_type_reference(reference_text(document, Some(*target)).as_deref(), out);
    }
    walk_attribute_body_type_refs(document, &metadata_usage.body, out);
}

fn push_optional_typing_reference(
    document: &ParsedRoot,
    relationship: Option<&sysml_v2_parser::next::ast::TypingRelationship>,
    out: &mut Vec<String>,
) {
    if let Some(target) = typing_target_display(document, relationship) {
        push_type_reference(&target, out);
    }
}

pub(crate) fn package_keys_for_import_target(target: &str) -> Vec<String> {
    let target = target
        .trim()
        .trim_end_matches("::*")
        .trim_end_matches("::**");
    if target.is_empty() {
        return Vec::new();
    }
    let mut keys = Vec::new();
    let parts: Vec<&str> = target.split("::").collect();
    for i in 0..parts.len() {
        keys.push(parts[..=i].join("::"));
    }
    keys
}
