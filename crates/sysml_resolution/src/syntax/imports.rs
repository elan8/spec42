//! Imports and namespace references, with the ranges an editor needs.
//!
//! `closure_targets` answers what a source imports as flat strings, because that is all closure
//! resolution asks. An editor asks a different question about the same fact -- *where* the import
//! is written, and what shape it has -- so this walks the same members once more and publishes
//! the structured answer. Neither derives the other's shape from text.

use sysml_v2_parser::ast::{ImportShape, PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::{Node, ParsedDocument};

use super::token_util::{qualified_identification_name, span_to_source_range};
use super::{ImportScope, SyntaxFileImport, SyntaxImport, SyntaxRange};

pub(super) fn imports(document: &ParsedDocument) -> Vec<SyntaxImport<'_>> {
    let mut out = Vec::new();
    for element in &document.elements {
        match &element.value {
            RootElement::Import(import) => push_import(document, import, None, &mut out),
            RootElement::Package(package) => {
                let owner = package_owner(document, &package.value.identification);
                walk_body(document, &package.value.body, owner.as_deref(), &mut out);
            }
            RootElement::LibraryPackage(package) => {
                let owner = package_owner(document, &package.value.identification);
                walk_body(document, &package.value.body, owner.as_deref(), &mut out);
            }
            RootElement::Namespace(namespace) => {
                let owner = package_owner(document, &namespace.value.identification);
                walk_body(document, &namespace.value.body, owner.as_deref(), &mut out);
            }
            RootElement::Member(_) => {}
        }
    }
    out
}

fn package_owner(
    document: &ParsedDocument,
    identification: &sysml_v2_parser::ast::QualifiedIdentification,
) -> Option<String> {
    let name = qualified_identification_name(document, identification);
    (!name.is_empty()).then_some(name)
}

fn walk_body<'p>(
    document: &'p ParsedDocument,
    body: &PackageBody,
    owner: Option<&str>,
    out: &mut Vec<SyntaxImport<'p>>,
) {
    let PackageBody::Brace { elements, .. } = body else {
        return;
    };
    for member in elements {
        match &member.value {
            PackageBodyElement::Import(import) => push_import(document, import, owner, out),
            PackageBodyElement::Package(nested) => {
                let nested_owner = qualified_owner(document, owner, &nested.value.identification);
                walk_body(document, &nested.value.body, nested_owner.as_deref(), out);
            }
            PackageBodyElement::LibraryPackage(nested) => {
                let nested_owner = qualified_owner(document, owner, &nested.value.identification);
                walk_body(document, &nested.value.body, nested_owner.as_deref(), out);
            }
            _ => {}
        }
    }
}

fn qualified_owner(
    document: &ParsedDocument,
    owner: Option<&str>,
    identification: &sysml_v2_parser::ast::QualifiedIdentification,
) -> Option<String> {
    let name = package_owner(document, identification)?;
    Some(match owner {
        Some(owner) => format!("{owner}::{name}"),
        None => name,
    })
}

fn push_import<'p>(
    document: &'p ParsedDocument,
    import: &Node<sysml_v2_parser::ast::Import>,
    owner: Option<&str>,
    out: &mut Vec<SyntaxImport<'p>>,
) {
    let Some(view) = document.qualified_reference(import.value.target.reference) else {
        return;
    };
    let target = view.authored_text().trim();
    if target.is_empty() {
        return;
    }
    out.push(SyntaxImport {
        target,
        scope: scope_of(&import.value.target.shape),
        range: import_range(document, &import.span),
        file_target: file_target(document, target, &import.span),
        owner_package: owner.map(str::to_string),
    });
}

fn file_target<'p>(
    document: &'p ParsedDocument,
    target: &'p str,
    span: &sysml_v2_parser::Span,
) -> Option<SyntaxFileImport<'p>> {
    let value = target.trim_matches(['\'', '"']);
    if !value.starts_with("file://") {
        return None;
    }
    let import_range = import_range(document, span);
    for (line_offset, line) in document
        .source
        .as_str()
        .lines()
        .skip(import_range.start_line as usize)
        .take((import_range.end_line - import_range.start_line + 1) as usize)
        .enumerate()
    {
        let Some(byte_start) = line.find(value) else {
            continue;
        };
        let start_character = line[..byte_start].chars().count() as u32;
        let line = import_range.start_line + line_offset as u32;
        return Some(SyntaxFileImport {
            value,
            range: SyntaxRange {
                start_line: line,
                start_character,
                end_line: line,
                end_character: start_character + value.chars().count() as u32,
            },
        });
    }
    None
}

/// The shape the author wrote, as a scope rather than as the suffix that spells it.
fn scope_of(shape: &ImportShape) -> ImportScope {
    match shape {
        ImportShape::Membership {
            recursive_suffix: Some(_),
        } => ImportScope::Recursive,
        ImportShape::Membership {
            recursive_suffix: None,
        }
        | ImportShape::Filter { .. } => ImportScope::Element,
        ImportShape::Namespace {
            recursive_suffix: Some(_),
            ..
        } => ImportScope::Recursive,
        ImportShape::Namespace {
            recursive_suffix: None,
            ..
        } => ImportScope::Members,
    }
}

fn import_range(document: &ParsedDocument, span: &sysml_v2_parser::Span) -> SyntaxRange {
    match document.range(span) {
        Some(range) => SyntaxRange {
            start_line: range.start.line.saturating_sub(1),
            start_character: (range.start.column as u32).saturating_sub(1),
            end_line: range.end.line.saturating_sub(1),
            end_character: (range.end.column as u32).saturating_sub(1),
        },
        None => span_to_source_range(span),
    }
}

/// The first segment of a qualified name, which is the namespace it is rooted in.
pub(super) fn namespace_root(qualified_name: &str) -> Option<&str> {
    let root = qualified_name.split("::").next()?.trim();
    (!root.is_empty() && root.chars().all(|ch| ch.is_alphanumeric() || ch == '_')).then_some(root)
}
