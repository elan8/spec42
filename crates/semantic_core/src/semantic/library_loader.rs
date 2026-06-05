//! Import-scoped loading of SysML/KerML library files from configured roots.
//!
//! Workspace sources are parsed for `import` targets; only library packages in the
//! transitive closure (plus unit catalogs and optional bootstrap namespaces) are loaded.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use sysml_v2_parser::ast::{
    Import, LibraryPackage, Package, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::{Node, RootNamespace as ParsedRoot};
use walkdir::WalkDir;

/// Workspace file path and text used to seed the library import closure.
#[derive(Debug, Clone)]
pub struct WorkspaceSource<'a> {
    pub path: &'a str,
    pub content: &'a str,
}

/// Loaded library file (path relative to its root, UTF-8 content).
#[derive(Debug, Clone)]
pub struct LoadedLibraryFile {
    pub root: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PackageKey(String);

#[derive(Debug, Clone)]
struct IndexedFile {
    root: String,
    path: String,
}

/// Options for [`resolve_library_closure`].
#[derive(Debug, Clone)]
pub struct LibraryClosureOptions {
    /// When workspace imports `sysml::*` (or `sysml`), load packages under `sysml.library` / `kerml` roots.
    pub bootstrap_sysml_namespace: bool,
}

impl Default for LibraryClosureOptions {
    fn default() -> Self {
        Self {
            bootstrap_sysml_namespace: true,
        }
    }
}

/// Build the set of library files required by `workspace` imports (transitive closure).
pub fn resolve_library_closure(
    workspace: &[WorkspaceSource<'_>],
    library_roots: &[String],
    options: &LibraryClosureOptions,
) -> Result<Vec<LoadedLibraryFile>, String> {
    if library_roots.is_empty() {
        return Ok(Vec::new());
    }
    let index = build_package_index(library_roots)?;
    let mut seeds = HashSet::<PackageKey>::new();
    let mut wants_sysml_bootstrap = false;
    for source in workspace {
        for target in collect_import_targets_from_content(source.content) {
            if options.bootstrap_sysml_namespace
                && (target == "sysml" || target.starts_with("sysml::"))
            {
                wants_sysml_bootstrap = true;
            }
            for key in package_keys_for_import_target(&target) {
                seeds.insert(PackageKey(key));
            }
        }
    }
    if wants_sysml_bootstrap {
        for (key, entries) in &index.packages {
            if entries
                .iter()
                .any(|entry| is_stdlib_slice_root(&entry.root))
            {
                seeds.insert(key.clone());
            }
        }
    }
    let mut loaded_paths = HashSet::<(String, String)>::new();
    let mut files = Vec::<LoadedLibraryFile>::new();
    let mut queue: VecDeque<PackageKey> = seeds.into_iter().collect();
    while let Some(pkg) = queue.pop_front() {
        let Some(indexed) = index.packages.get(&pkg) else {
            continue;
        };
        for entry in indexed {
            let key = (entry.root.clone(), entry.path.clone());
            if !loaded_paths.insert(key.clone()) {
                continue;
            }
            let full_path = PathBuf::from(&entry.root).join(&entry.path);
            let content = std::fs::read_to_string(&full_path).map_err(|err| {
                format!("failed to read library file {}: {err}", full_path.display())
            })?;
            for target in collect_import_targets_from_content(&content) {
                for next in package_keys_for_import_target(&target) {
                    let next_key = PackageKey(next);
                    if index.packages.contains_key(&next_key) {
                        queue.push_back(next_key);
                    }
                }
            }
            files.push(LoadedLibraryFile {
                root: entry.root.clone(),
                path: entry.path.clone(),
                content,
            });
        }
    }
    for unit in &index.unit_catalog_files {
        let key = (unit.root.clone(), unit.path.clone());
        if !loaded_paths.insert(key.clone()) {
            continue;
        }
        let full_path = PathBuf::from(&unit.root).join(&unit.path);
        let content = std::fs::read_to_string(&full_path).map_err(|err| {
            format!(
                "failed to read library unit catalog {}: {err}",
                full_path.display()
            )
        })?;
        files.push(LoadedLibraryFile {
            root: unit.root.clone(),
            path: unit.path.clone(),
            content,
        });
    }
    files.sort_by(|a, b| {
        (a.root.as_str(), a.path.as_str()).cmp(&(b.root.as_str(), b.path.as_str()))
    });
    Ok(files)
}

struct PackageIndex {
    packages: HashMap<PackageKey, Vec<IndexedFile>>,
    unit_catalog_files: Vec<IndexedFile>,
}

fn build_package_index(library_roots: &[String]) -> Result<PackageIndex, String> {
    let mut packages = HashMap::<PackageKey, Vec<IndexedFile>>::new();
    let mut unit_catalog_files = Vec::<IndexedFile>::new();
    for root in library_roots {
        let root_path = PathBuf::from(root);
        if !root_path.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let lower = path.to_string_lossy().to_ascii_lowercase();
            if !(lower.ends_with(".sysml") || lower.ends_with(".kerml")) {
                continue;
            }
            let rel = path
                .strip_prefix(&root_path)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            let normalized_rel = rel.replace('\\', "/");
            if lower.ends_with("units.sysml") || normalized_rel.contains("Quantities and Units/") {
                unit_catalog_files.push(IndexedFile {
                    root: root.clone(),
                    path: rel.clone(),
                });
            }
            let content = std::fs::read_to_string(path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            if let Some(package) = extract_package_name(&content) {
                packages
                    .entry(PackageKey(package))
                    .or_default()
                    .push(IndexedFile {
                        root: root.clone(),
                        path: rel,
                    });
            }
        }
    }
    Ok(PackageIndex {
        packages,
        unit_catalog_files,
    })
}

fn extract_package_name(content: &str) -> Option<String> {
    for line in content.lines().take(80) {
        let trimmed = line.trim();
        let rest = trimmed
            .strip_prefix("standard library package ")
            .or_else(|| trimmed.strip_prefix("library package "))
            .or_else(|| trimmed.strip_prefix("package "));
        if let Some(rest) = rest {
            let name = rest
                .split(|c: char| !c.is_ascii_alphanumeric() && c != ':' && c != '_')
                .next()
                .unwrap_or("")
                .trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn is_stdlib_slice_root(root: &str) -> bool {
    root.replace('\\', "/")
        .to_ascii_lowercase()
        .ends_with("sysml.library")
}

fn collect_import_targets_from_content(content: &str) -> Vec<String> {
    let Ok(parsed) = sysml_v2_parser::parse(content) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_import_targets_from_root(&parsed, &mut out);
    out
}

fn collect_import_targets_from_root(root: &ParsedRoot, out: &mut Vec<String>) {
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

fn package_keys_for_import_target(target: &str) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn closure_loads_transitive_library_package() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("Base.sysml"),
            "package Base { attribute def Name; }",
        )
        .expect("base");
        fs::write(
            lib.join("Consumer.sysml"),
            "package Demo { import Base::*; part def P { attribute n : Name; } }",
        )
        .expect("consumer");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { import Demo::*; part def AppPart; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        let paths: Vec<_> = loaded.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.iter().any(|p| p.contains("Base.sysml")), "{paths:?}");
        assert!(
            paths.iter().any(|p| p.contains("Consumer.sysml")),
            "{paths:?}"
        );
    }

    #[test]
    fn closure_indexes_standard_library_package_declarations() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(
            lib.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("scalar values");
        let workspace = [WorkspaceSource {
            path: "loose.sysml",
            content: "package P { private import ScalarValues::Real; attribute x : Real; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(
            loaded.iter().any(|f| f.path.contains("ScalarValues.sysml")),
            "expected ScalarValues.sysml in closure, got {:?}",
            loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
        );
    }

    #[test]
    fn closure_omits_unreferenced_library_file() {
        let temp = tempfile::tempdir().expect("tempdir");
        let lib = temp.path().join("lib");
        fs::create_dir_all(&lib).expect("lib dir");
        fs::write(lib.join("Unused.sysml"), "package Unused { part def X; }").expect("unused");
        fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").expect("base");
        let workspace = [WorkspaceSource {
            path: "model.sysml",
            content: "package App { import Base::*; }",
        }];
        let roots = vec![lib.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
            .expect("closure");
        assert!(!loaded.iter().any(|f| f.path.contains("Unused.sysml")));
    }
}
