//! Import relationships between workspace documents (for incremental diagnostic republish).

use sysml_v2_parser::ast::{
    Import, LibraryPackage, Node, Package, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use crate::common::util;
use crate::syntax::ast_util::identification_name;
use crate::workspace::state::ServerState;

/// Workspace documents (excluding `provider_uri`) that import a top-level package from `provider_uri`.
pub(crate) fn workspace_uris_importing_declarations_from(
    state: &ServerState,
    provider_uri: &Url,
) -> Vec<Url> {
    let Some(provider) = state.index.get(provider_uri) else {
        return Vec::new();
    };
    let exported = top_level_package_names(provider.parsed.as_ref(), &provider.content);
    if exported.is_empty() {
        return Vec::new();
    }

    state
        .index
        .iter()
        .filter(|(uri, _)| *uri != provider_uri)
        .filter(|(uri, _)| !util::uri_under_any_library(uri, &state.library_paths))
        .filter(|(_, entry)| {
            document_imports_any_package(entry.parsed.as_ref(), &entry.content, &exported)
        })
        .map(|(uri, _)| uri.clone())
        .collect()
}

fn top_level_package_names(parsed: Option<&RootNamespace>, content: &str) -> Vec<String> {
    if let Some(root) = parsed {
        return package_names_from_root(root);
    }
    sysml_v2_parser::parse(content)
        .ok()
        .map(|root| package_names_from_root(&root))
        .unwrap_or_default()
}

fn package_names_from_root(root: &RootNamespace) -> Vec<String> {
    root.elements
        .iter()
        .filter_map(|element| match &element.value {
            RootElement::Package(package) => Some(identification_name(&package.identification)),
            RootElement::LibraryPackage(package) => {
                Some(identification_name(&package.identification))
            }
            _ => None,
        })
        .filter(|name| !name.is_empty())
        .collect()
}

fn document_imports_any_package(
    parsed: Option<&RootNamespace>,
    content: &str,
    packages: &[String],
) -> bool {
    let targets = import_targets_from_document(parsed, content);
    packages.iter().any(|package| {
        targets
            .iter()
            .any(|target| import_references_package(target, package))
    })
}

fn import_targets_from_document(parsed: Option<&RootNamespace>, content: &str) -> Vec<String> {
    if let Some(root) = parsed {
        let mut out = Vec::new();
        collect_import_targets_from_root(root, &mut out);
        return out;
    }
    sysml_v2_parser::parse(content)
        .ok()
        .map(|root| {
            let mut out = Vec::new();
            collect_import_targets_from_root(&root, &mut out);
            out
        })
        .unwrap_or_default()
}

fn collect_import_targets_from_root(root: &RootNamespace, out: &mut Vec<String>) {
    for element in &root.elements {
        match &element.value {
            RootElement::Package(package) => walk_package_imports(package, out),
            RootElement::LibraryPackage(package) => walk_library_package_imports(package, out),
            _ => {}
        }
    }
}

fn walk_package_body(body: &PackageBody, out: &mut Vec<String>) {
    let PackageBody::Brace { elements } = body else {
        return;
    };
    for member in elements {
        match &member.value {
            PackageBodyElement::Import(import) => push_import_target(import, out),
            PackageBodyElement::Package(nested) => walk_package_imports(nested, out),
            PackageBodyElement::LibraryPackage(nested) => walk_library_package_imports(nested, out),
            _ => {}
        }
    }
}

fn walk_package_imports(package: &Node<Package>, out: &mut Vec<String>) {
    walk_package_body(&package.value.body, out);
}

fn walk_library_package_imports(package: &Node<LibraryPackage>, out: &mut Vec<String>) {
    walk_package_body(&package.value.body, out);
}

fn push_import_target(import: &Node<Import>, out: &mut Vec<String>) {
    let target = import.value.target.trim();
    if !target.is_empty() {
        out.push(target.to_string());
    }
}

fn import_references_package(import_target: &str, package: &str) -> bool {
    let target = import_target
        .trim()
        .trim_end_matches("::*")
        .trim_end_matches("::**");
    target == package || target.starts_with(&format!("{package}::"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::state::{IndexEntry, ParseMetadata, ServerState};
    use std::collections::HashMap;

    fn entry(content: &str) -> IndexEntry {
        let parsed = sysml_v2_parser::parse(content).ok();
        IndexEntry {
            content: content.to_string(),
            parsed,
            parse_metadata: ParseMetadata::default(),
            include_in_semantic_graph: true,
        }
    }

    #[test]
    fn detects_importer_of_top_level_package() {
        let provider = Url::parse("file:///workspace/a.sysml").unwrap();
        let importer = Url::parse("file:///workspace/b.sysml").unwrap();
        let mut index = HashMap::new();
        index.insert(provider.clone(), entry("package A { attribute def Name; }"));
        index.insert(
            importer.clone(),
            entry("package B { import A::*; part def P { attribute n : Name; } }"),
        );
        let state = ServerState {
            index,
            ..ServerState::default()
        };
        let peers = workspace_uris_importing_declarations_from(&state, &provider);
        assert_eq!(peers, vec![importer]);
    }

    #[test]
    fn ignores_unrelated_workspace_files() {
        let provider = Url::parse("file:///workspace/a.sysml").unwrap();
        let other = Url::parse("file:///workspace/c.sysml").unwrap();
        let mut index = HashMap::new();
        index.insert(provider.clone(), entry("package A { }"));
        index.insert(
            other.clone(),
            entry("package C { import Other::*; part def P; }"),
        );
        let state = ServerState {
            index,
            ..ServerState::default()
        };
        let peers = workspace_uris_importing_declarations_from(&state, &provider);
        assert!(peers.is_empty());
    }
}
