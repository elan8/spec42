//! The library-closure queries: what a source imports and what types it names.
//!
//! Moved here from `workspace::library::closure` because every one of these walks the AST. The
//! closure *policy* -- which packages to pull in, in what order, from which roots -- stays in
//! `workspace`, which is where it belongs; this answers only the syntactic questions that policy
//! asks.

use std::collections::HashSet;

use sysml_v2_parser::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, Import, ItemUsage,
    LibraryPackage, MetadataBody, MetadataBodyElement, MetadataDef, MetadataUsage, Package,
    PackageBody, PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, PartUsage,
    PartUsageBody, PartUsageBodyElement, PortBody, PortBodyElement, PortDef, PortDefBody,
    PortDefBodyElement, PortUsage, QualifiedIdentification, RefDecl, RootElement,
};
use sysml_v2_parser::{Node, ParsedDocument as ParsedRoot};

/// An arena-backed type reference, as authored.
fn reference_text(
    document: &ParsedRoot,
    reference: Option<sysml_v2_parser::QualifiedReferenceId>,
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
    relationship: Option<&sysml_v2_parser::ast::SubsettingRelationship>,
) -> Option<&'a str> {
    let target = relationship?.target.first().copied()?;
    document
        .qualified_reference(target)
        .map(|view| view.authored_text())
}

fn typing_target_display(
    document: &ParsedRoot,
    relationship: Option<&sysml_v2_parser::ast::TypingRelationship>,
) -> Option<String> {
    let target = relationship?.target.first().copied()?;
    document
        .qualified_reference(target)
        .map(|view| view.authored_text().to_string())
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
    walk_metadata_body_type_refs(document, &metadata_usage.body, out);
}

/// Walks a `MetadataBody`'s reference redefinitions (`MetadataBodyUsage`), whose targets are
/// source-backed references rather than the attribute declarations this body used to carry.
pub(crate) fn walk_metadata_body_type_refs(
    document: &ParsedRoot,
    body: &MetadataBody,
    out: &mut Vec<String>,
) {
    let MetadataBody::Brace { elements, .. } = body else {
        return;
    };
    for member in elements {
        if let MetadataBodyElement::Usage(usage) = &member.value {
            push_optional_type_reference(
                reference_text(document, Some(usage.value.target)).as_deref(),
                out,
            );
            walk_metadata_body_type_refs(document, &usage.value.body, out);
        }
    }
}

fn push_optional_typing_reference(
    document: &ParsedRoot,
    relationship: Option<&sysml_v2_parser::ast::TypingRelationship>,
    out: &mut Vec<String>,
) {
    if let Some(target) = typing_target_display(document, relationship) {
        push_type_reference(&target, out);
    }
}

/// The declared label of a namespace-owning declaration.
///
/// Only the simple alternative carries its own label; a qualified path (`package A::B { ... }`) is
/// an arena identity, and a package key built from it belongs to the document that owns it. The
/// callers here key packages by simple name, so a qualified declaration yields no key -- the same
/// answer the old `Option<String>` field gave, now stated rather than incidental.
pub(crate) fn package_declared_name(identification: &QualifiedIdentification) -> Option<String> {
    identification
        .simple_name()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string)
}

/// Qualified names of packages declared in a parsed SysML document (includes nested packages).
pub(super) fn declared_packages_from_parsed(parsed: &ParsedRoot) -> HashSet<String> {
    let mut defined = HashSet::new();
    for_each_package_in_parsed(parsed, |qualified, _body| {
        defined.insert(qualified);
    });
    defined
}

pub(crate) fn for_each_package_in_parsed(
    parsed: &ParsedRoot,
    mut visit: impl FnMut(String, &PackageBody),
) {
    for element in &parsed.elements {
        match &element.value {
            RootElement::Package(package) => visit_package_tree(package, None, &mut visit),
            RootElement::LibraryPackage(package) => {
                visit_library_package_tree(package, None, &mut visit)
            }
            _ => {}
        }
    }
}

pub(crate) fn visit_package_tree(
    package: &Node<Package>,
    parent: Option<&str>,
    visit: &mut impl FnMut(String, &PackageBody),
) {
    let Some(name) = package_declared_name(&package.value.identification) else {
        return;
    };
    let qualified = match parent {
        Some(prefix) => format!("{prefix}::{name}"),
        None => name,
    };
    visit(qualified.clone(), &package.value.body);
    walk_nested_packages(&package.value.body, Some(qualified.as_str()), visit);
}

pub(crate) fn visit_library_package_tree(
    package: &Node<LibraryPackage>,
    parent: Option<&str>,
    visit: &mut impl FnMut(String, &PackageBody),
) {
    let Some(name) = package_declared_name(&package.value.identification) else {
        return;
    };
    let qualified = match parent {
        Some(prefix) => format!("{prefix}::{name}"),
        None => name,
    };
    visit(qualified.clone(), &package.value.body);
    walk_nested_packages(&package.value.body, Some(qualified.as_str()), visit);
}

pub(crate) fn walk_nested_packages(
    body: &PackageBody,
    parent: Option<&str>,
    visit: &mut impl FnMut(String, &PackageBody),
) {
    let PackageBody::Brace { elements, .. } = body else {
        return;
    };
    for member in elements {
        match &member.value {
            PackageBodyElement::Package(nested) => visit_package_tree(nested, parent, visit),
            PackageBodyElement::LibraryPackage(nested) => {
                visit_library_package_tree(nested, parent, visit)
            }
            _ => {}
        }
    }
}

pub(crate) fn collect_import_targets_from_package_body(
    document: &ParsedRoot,
    body: &PackageBody,
) -> Vec<String> {
    let mut out = Vec::new();
    walk_package_body(document, body, &mut out);
    out
}

pub(crate) fn collect_import_targets_from_root(document: &ParsedRoot, out: &mut Vec<String>) {
    for element in &document.elements {
        match &element.value {
            RootElement::Package(package) => walk_package_imports(document, package, out),
            RootElement::LibraryPackage(package) => {
                walk_library_package_imports(document, package, out)
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_package_body(document: &ParsedRoot, body: &PackageBody, out: &mut Vec<String>) {
    let PackageBody::Brace { elements, .. } = body else {
        return;
    };
    for member in elements {
        match &member.value {
            PackageBodyElement::Import(import) => push_import_target(document, import, out),
            PackageBodyElement::Package(nested) => walk_package_imports(document, nested, out),
            PackageBodyElement::LibraryPackage(nested) => {
                walk_library_package_imports(document, nested, out)
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_package_imports(
    document: &ParsedRoot,
    package: &Node<Package>,
    out: &mut Vec<String>,
) {
    walk_package_body(document, &package.value.body, out);
}

pub(crate) fn walk_library_package_imports(
    document: &ParsedRoot,
    package: &Node<LibraryPackage>,
    out: &mut Vec<String>,
) {
    walk_package_body(document, &package.value.body, out);
}

pub(crate) fn push_import_target(
    document: &ParsedRoot,
    import: &Node<Import>,
    out: &mut Vec<String>,
) {
    let Some(view) = document.qualified_reference(import.value.target.reference) else {
        return;
    };
    let target = view.authored_text().trim();
    if target.is_empty() {
        return;
    }
    // The arena owns the qualified name; the `::*` / `::**` suffix is the *shape* of the import,
    // not part of the name. The seed key spells the authored form, so reattach it.
    use sysml_v2_parser::ast::ImportShape;
    let suffix = match &import.value.target.shape {
        ImportShape::Membership {
            recursive_suffix: Some(_),
        } => "::**",
        ImportShape::Membership {
            recursive_suffix: None,
        } => "",
        ImportShape::Namespace {
            recursive_suffix: Some(_),
            ..
        } => "::*::**",
        ImportShape::Namespace {
            recursive_suffix: None,
            ..
        } => "::*",
        ImportShape::Filter { .. } => "",
    };
    out.push(format!("{target}{suffix}"));
}

/// Everything closure resolution asks, from one already-parsed tree.
pub(super) fn closure_facts(document: &ParsedRoot) -> super::SyntaxClosureFacts {
    let mut type_reference_targets = Vec::new();
    let mut packages = Vec::new();
    for_each_package_in_parsed(document, |qualified_name, body| {
        let mut package_type_references = Vec::new();
        collect_type_reference_targets_from_package_body(
            document,
            body,
            &mut package_type_references,
        );
        packages.push(PackageTargets {
            qualified_name,
            import_targets: collect_import_targets_from_package_body(document, body),
            type_reference_targets: package_type_references,
        });
    });
    collect_type_reference_targets_from_root(document, &mut type_reference_targets);
    let mut import_targets = Vec::new();
    collect_import_targets_from_root(document, &mut import_targets);
    super::SyntaxClosureFacts {
        declared_packages: declared_packages_from_parsed(document),
        import_targets,
        type_reference_targets,
        packages,
    }
}

/// What one package in a source imports and names, keyed by its qualified name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageTargets {
    pub qualified_name: String,
    pub import_targets: Vec<String>,
    pub type_reference_targets: Vec<String>,
}

fn push_type_reference(target: &str, out: &mut Vec<String>) {
    let target = target.trim();
    if target.is_empty() || target.starts_with("checks meta ") {
        return;
    }
    out.push(target.to_string());
}

fn push_optional_type_reference(target: Option<&str>, out: &mut Vec<String>) {
    if let Some(target) = target {
        push_type_reference(target, out);
    }
}

#[cfg(test)]
mod tests {
    use super::super::SyntaxAuthority;

    /// The library-closure seed a bare type reference produces.
    ///
    /// `lsp_feature_inspector_resolves_a_target_in_an_admitted_library` admits its library purely
    /// through this: the workspace source has no import, so `Domain` reaches the closure only if
    /// the `part w : Domain::Wheel;` typing is reported as a type-reference target.
    #[test]
    fn a_part_usage_typing_seeds_its_package() {
        assert_eq!(
            SyntaxAuthority::new()
                .parse_text("package App { part w : Domain::Wheel; }")
                .closure_facts()
                .type_reference_targets,
            vec!["Domain::Wheel".to_string()]
        );
    }

    #[test]
    fn a_library_package_declares_its_name() {
        assert!(SyntaxAuthority::new()
            .parse_text("library package Domain { part def Wheel; }")
            .closure_facts()
            .declared_packages
            .contains("Domain"));
    }
}
