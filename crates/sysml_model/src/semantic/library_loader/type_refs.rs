use super::*;
use crate::semantic::ast_util::{subsetting_target, typing_target};

pub(crate) fn collect_type_reference_targets_from_content(content: &str) -> Vec<String> {
    let Ok(parsed) = sysml_v2_parser::parse(content) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_type_reference_targets_from_root(&parsed, &mut out);
    out
}

pub(crate) fn collect_type_reference_targets_from_root(root: &ParsedRoot, out: &mut Vec<String>) {
    for element in &root.elements {
        match &element.value {
            RootElement::Package(package) => walk_package_type_refs(package, out),
            RootElement::LibraryPackage(package) => walk_library_package_type_refs(package, out),
            _ => {}
        }
    }
}

pub(crate) fn collect_type_reference_targets_from_package_body(body: &PackageBody, out: &mut Vec<String>) {
    let PackageBody::Brace { elements } = body else {
        return;
    };
    for member in elements {
        walk_package_body_element_type_refs(&member.value, out);
    }
}

pub(crate) fn walk_package_type_refs(package: &Node<Package>, out: &mut Vec<String>) {
    collect_type_reference_targets_from_package_body(&package.value.body, out);
}

pub(crate) fn walk_library_package_type_refs(package: &Node<LibraryPackage>, out: &mut Vec<String>) {
    collect_type_reference_targets_from_package_body(&package.value.body, out);
}

pub(crate) fn walk_package_body_element_type_refs(element: &PackageBodyElement, out: &mut Vec<String>) {
    match element {
        PackageBodyElement::Package(nested) => walk_package_type_refs(nested, out),
        PackageBodyElement::LibraryPackage(nested) => walk_library_package_type_refs(nested, out),
        PackageBodyElement::PartDef(part_def) => walk_part_def_type_refs(&part_def.value, out),
        PackageBodyElement::PartUsage(part_usage) => walk_part_usage_type_refs(&part_usage.value, out),
        PackageBodyElement::PortDef(port_def) => walk_port_def_type_refs(&port_def.value, out),
        PackageBodyElement::ItemDef(item_def) => {
            push_optional_type_reference(typing_target(item_def.value.specializes.as_deref()), out);
        }
        PackageBodyElement::MetadataDef(metadata_def) => {
            walk_metadata_def_type_refs(&metadata_def.value, out);
        }
        PackageBodyElement::MetadataUsage(metadata_usage) => {
            walk_metadata_usage_type_refs(&metadata_usage.value, out);
        }
        _ => {}
    }
}

pub(crate) fn walk_part_def_type_refs(part_def: &PartDef, out: &mut Vec<String>) {
    push_optional_type_reference(typing_target(part_def.specializes.as_deref()), out);
    let PartDefBody::Brace { elements } = &part_def.body else {
        return;
    };
    for member in elements {
        walk_part_def_body_element_type_refs(&member.value, out);
    }
}

pub(crate) fn walk_part_def_body_element_type_refs(element: &PartDefBodyElement, out: &mut Vec<String>) {
    match element {
        PartDefBodyElement::PartDef(part_def) => walk_part_def_type_refs(&part_def.value, out),
        PartDefBodyElement::PartUsage(part_usage) => {
            walk_part_usage_type_refs(&part_usage.value, out);
        }
        PartDefBodyElement::PortUsage(port_usage) => walk_port_usage_type_refs(&port_usage.value, out),
        PartDefBodyElement::AttributeDef(attribute_def) => {
            walk_attribute_def_type_refs(&attribute_def.value, out);
        }
        PartDefBodyElement::AttributeUsage(attribute_usage) => {
            walk_attribute_usage_type_refs(&attribute_usage.value, out);
        }
        PartDefBodyElement::ItemDef(item_def) => {
            push_optional_type_reference(typing_target(item_def.value.specializes.as_deref()), out);
        }
        PartDefBodyElement::ItemUsage(item_usage) => {
            walk_item_usage_type_refs(&item_usage.value, out);
        }
        PartDefBodyElement::Ref(ref_decl) => walk_ref_decl_type_refs(&ref_decl.value, out),
        PartDefBodyElement::ExhibitState(exhibit_state) => {
            push_optional_type_reference(exhibit_state.value.type_name.as_deref(), out);
        }
        PartDefBodyElement::Connection(connection) => {
            push_optional_type_reference(connection.value.type_name.as_deref(), out);
            push_optional_type_reference(subsetting_target(connection.value.subsets.as_deref()), out);
            push_optional_type_reference(subsetting_target(connection.value.redefines.as_deref()), out);
        }
        _ => {}
    }
}

pub(crate) fn walk_part_usage_type_refs(part_usage: &PartUsage, out: &mut Vec<String>) {
    push_type_reference(&part_usage.type_name, out);
    push_optional_type_reference(subsetting_target(part_usage.redefines.as_deref()), out);
    if let Some((subsets, _)) = &part_usage.subsets {
        push_type_reference(&subsets.value.target, out);
    }
    let PartUsageBody::Brace { elements } = &part_usage.body else {
        return;
    };
    for member in elements {
        walk_part_usage_body_element_type_refs(&member.value, out);
    }
}

pub(crate) fn walk_part_usage_body_element_type_refs(element: &PartUsageBodyElement, out: &mut Vec<String>) {
    match element {
        PartUsageBodyElement::PartUsage(part_usage) => {
            walk_part_usage_type_refs(&part_usage.value, out);
        }
        PartUsageBodyElement::PortUsage(port_usage) => walk_port_usage_type_refs(&port_usage.value, out),
        PartUsageBodyElement::AttributeUsage(attribute_usage) => {
            walk_attribute_usage_type_refs(&attribute_usage.value, out);
        }
        PartUsageBodyElement::Ref(ref_decl) => walk_ref_decl_type_refs(&ref_decl.value, out),
        _ => {}
    }
}

pub(crate) fn walk_port_def_type_refs(port_def: &PortDef, out: &mut Vec<String>) {
    push_optional_type_reference(typing_target(port_def.specializes.as_deref()), out);
    let PortDefBody::Brace { elements } = &port_def.body else {
        return;
    };
    for member in elements {
        match &member.value {
            PortDefBodyElement::PortUsage(port_usage) => {
                walk_port_usage_type_refs(&port_usage.value, out);
            }
            PortDefBodyElement::AttributeDef(attribute_def) => {
                walk_attribute_def_type_refs(&attribute_def.value, out);
            }
            PortDefBodyElement::AttributeUsage(attribute_usage) => {
                walk_attribute_usage_type_refs(&attribute_usage.value, out);
            }
            PortDefBodyElement::ItemUsage(item_usage) => {
                walk_item_usage_type_refs(&item_usage.value, out);
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_port_usage_type_refs(port_usage: &PortUsage, out: &mut Vec<String>) {
    push_optional_type_reference(port_usage.type_name.as_deref(), out);
    push_optional_type_reference(subsetting_target(port_usage.redefines.as_deref()), out);
    push_optional_type_reference(subsetting_target(port_usage.references.as_deref()), out);
    push_optional_type_reference(subsetting_target(port_usage.crosses.as_deref()), out);
    if let Some((subsets, _)) = &port_usage.subsets {
        push_type_reference(&subsets.value.target, out);
    }
    let PortBody::Brace { elements } = &port_usage.body else {
        return;
    };
    for member in elements {
        if let PortBodyElement::PortUsage(nested) = &member.value {
            walk_port_usage_type_refs(&nested.value, out);
        }
    }
}

pub(crate) fn walk_attribute_def_type_refs(attribute_def: &AttributeDef, out: &mut Vec<String>) {
    push_optional_type_reference(typing_target(attribute_def.typing.as_deref()), out);
    walk_attribute_body_type_refs(&attribute_def.body, out);
}

pub(crate) fn walk_attribute_usage_type_refs(attribute_usage: &AttributeUsage, out: &mut Vec<String>) {
    push_optional_type_reference(typing_target(attribute_usage.typing.as_deref()), out);
    push_optional_type_reference(subsetting_target(attribute_usage.redefines.as_deref()), out);
    push_optional_type_reference(subsetting_target(attribute_usage.references.as_deref()), out);
    push_optional_type_reference(subsetting_target(attribute_usage.crosses.as_deref()), out);
    walk_attribute_body_type_refs(&attribute_usage.body, out);
}

pub(crate) fn walk_attribute_body_type_refs(body: &AttributeBody, out: &mut Vec<String>) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };
    for member in elements {
        match &member.value {
            AttributeBodyElement::AttributeDef(attribute_def) => {
                walk_attribute_def_type_refs(&attribute_def.value, out);
            }
            AttributeBodyElement::AttributeUsage(attribute_usage) => {
                walk_attribute_usage_type_refs(&attribute_usage.value, out);
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_item_usage_type_refs(item_usage: &ItemUsage, out: &mut Vec<String>) {
    push_optional_type_reference(item_usage.type_name.as_deref(), out);
    walk_attribute_body_type_refs(&item_usage.body, out);
}

pub(crate) fn walk_ref_decl_type_refs(ref_decl: &RefDecl, out: &mut Vec<String>) {
    push_type_reference(&ref_decl.type_name, out);
}

pub(crate) fn walk_metadata_def_type_refs(metadata_def: &MetadataDef, out: &mut Vec<String>) {
    push_optional_type_reference(typing_target(metadata_def.specializes.as_deref()), out);
    walk_attribute_body_type_refs(&metadata_def.body, out);
}

pub(crate) fn walk_metadata_usage_type_refs(metadata_usage: &MetadataUsage, out: &mut Vec<String>) {
    push_optional_type_reference(metadata_usage.type_name.as_deref(), out);
    for target in &metadata_usage.about_targets {
        push_type_reference(target, out);
    }
    walk_attribute_body_type_refs(&metadata_usage.body, out);
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
