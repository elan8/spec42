use super::*;

pub(crate) struct PackageIndex {
    pub(crate) packages: HashMap<PackageKey, Vec<IndexedFile>>,
    pub(crate) unit_catalog_files: Vec<IndexedFile>,
}

pub(crate) fn build_package_index(library_roots: &[String]) -> Result<PackageIndex, String> {
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
            let content = std::fs::read_to_string(path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let normalized_rel = rel.replace('\\', "/");
            if is_unit_catalog_path_hint(&lower, &normalized_rel)
                || content_contains_unit_definition(&content)
            {
                unit_catalog_files.push(IndexedFile {
                    root: root.clone(),
                    path: rel.clone(),
                });
            }
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

pub(crate) fn extract_package_name(content: &str) -> Option<String> {
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

pub(crate) fn is_stdlib_slice_root(root: &str) -> bool {
    root.replace('\\', "/")
        .to_ascii_lowercase()
        .ends_with("sysml.library")
}

pub(crate) fn package_declared_name(identification: &Identification) -> Option<String> {
    identification
        .name
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string)
}

pub(crate) fn workspace_declared_packages(
    workspace: &[WorkspaceSource<'_>],
) -> HashSet<PackageKey> {
    let mut defined = HashSet::new();
    for source in workspace {
        defined.extend(
            declared_packages_in_content(source.content)
                .into_iter()
                .map(PackageKey),
        );
    }
    defined
}

/// Qualified names of packages declared in a parsed SysML document (includes nested packages).
pub fn declared_packages_from_parsed(parsed: &ParsedRoot) -> HashSet<String> {
    let mut defined = HashSet::new();
    for_each_package_in_parsed(parsed, |qualified, _body| {
        defined.insert(qualified);
    });
    defined
}

/// Qualified names of packages declared in SysML source (includes nested packages).
pub fn declared_packages_in_content(content: &str) -> HashSet<String> {
    let Ok(parsed) = sysml_v2_parser::parse(content) else {
        return HashSet::new();
    };
    declared_packages_from_parsed(&parsed)
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

pub(crate) fn for_each_package_in_content(content: &str, visit: impl FnMut(String, &PackageBody)) {
    let Ok(parsed) = sysml_v2_parser::parse(content) else {
        return;
    };
    for_each_package_in_parsed(&parsed, visit);
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
    let PackageBody::Brace { elements } = body else {
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

pub(crate) fn collect_import_targets_from_package_body(body: &PackageBody) -> Vec<String> {
    let mut out = Vec::new();
    walk_package_body(body, &mut out);
    out
}

pub(crate) const QUANTITY_UNIT_CLOSURE_PACKAGES: &[&str] = &[
    "Measurement",
    "ISQ",
    "ISQBase",
    "ISQSpaceTime",
    "ISQMechanics",
    "ISQElectromagnetism",
    "ISQThermodynamics",
    "SI",
    "SIPrefixes",
    "USCustomaryUnits",
];

pub(crate) fn workspace_contains_unit_literal(content: &str) -> bool {
    let bytes = content.as_bytes();
    let mut i = 0usize;
    while i + 2 < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j].is_ascii_digit() || bytes[j] == b'.') {
                j += 1;
            }
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'[' {
                return true;
            }
        }
        i += 1;
    }
    false
}

pub(crate) fn is_unit_catalog_path_hint(lower_full_path: &str, relative_path: &str) -> bool {
    let normalized_rel = relative_path.replace('\\', "/").to_ascii_lowercase();
    lower_full_path.ends_with("units.sysml")
        || normalized_rel.contains("quantities and units/")
        || normalized_rel.contains("quantities%20and%20units/")
        || normalized_rel.contains("quantities_and_units")
        || normalized_rel.contains("qudv")
        || normalized_rel.ends_with("/si.sysml")
        || normalized_rel == "si.sysml"
}

pub(crate) fn content_contains_unit_definition(content: &str) -> bool {
    content.lines().any(|line| {
        let Some((_, after_attribute)) = line.split_once("attribute <") else {
            return false;
        };
        let Some((_, after_colon)) = after_attribute.split_once(':') else {
            return false;
        };
        after_colon
            .split([';', '{', '='])
            .next()
            .is_some_and(|dimension| dimension.contains("Unit"))
    })
}

pub(crate) fn enqueue_closure_targets_from_content(
    content: &str,
    options: &LibraryClosureOptions,
    queue: &mut VecDeque<PackageKey>,
) {
    for target in collect_import_targets_from_content(content) {
        for next in package_keys_for_import_target(&target) {
            queue.push_back(PackageKey(next));
        }
    }
    if options.bootstrap_typing_references {
        for target in collect_type_reference_targets_from_content(content) {
            for next in package_keys_for_import_target(&target) {
                queue.push_back(PackageKey(next));
            }
        }
    }
}

pub(crate) fn enqueue_imports_from_workspace_packages(
    workspace: &[WorkspaceSource<'_>],
    workspace_declared_packages: &HashSet<PackageKey>,
    options: &LibraryClosureOptions,
    queue: &mut VecDeque<PackageKey>,
) {
    for pkg in workspace_declared_packages {
        enqueue_imports_from_workspace_package(workspace, pkg, options, queue);
    }
}

pub(crate) fn enqueue_imports_from_workspace_package(
    workspace: &[WorkspaceSource<'_>],
    pkg: &PackageKey,
    options: &LibraryClosureOptions,
    queue: &mut VecDeque<PackageKey>,
) {
    for source in workspace {
        for_each_package_in_content(source.content, |qualified, body| {
            if qualified != pkg.0 {
                return;
            }
            for target in collect_import_targets_from_package_body(body) {
                for next in package_keys_for_import_target(&target) {
                    queue.push_back(PackageKey(next));
                }
            }
            if options.bootstrap_typing_references {
                let mut type_targets = Vec::new();
                collect_type_reference_targets_from_package_body(body, &mut type_targets);
                for target in type_targets {
                    for next in package_keys_for_import_target(&target) {
                        queue.push_back(PackageKey(next));
                    }
                }
            }
        });
    }
}

pub(crate) fn collect_import_targets_from_content(content: &str) -> Vec<String> {
    let Ok(parsed) = sysml_v2_parser::parse(content) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_import_targets_from_root(&parsed, &mut out);
    out
}

pub(crate) fn collect_import_targets_from_root(root: &ParsedRoot, out: &mut Vec<String>) {
    for element in &root.elements {
        match &element.value {
            RootElement::Package(package) => walk_package_imports(package, out),
            RootElement::LibraryPackage(package) => walk_library_package_imports(package, out),
            _ => {}
        }
    }
}

pub(crate) fn walk_package_body(body: &PackageBody, out: &mut Vec<String>) {
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

pub(crate) fn walk_package_imports(package: &Node<Package>, out: &mut Vec<String>) {
    walk_package_body(&package.value.body, out);
}

pub(crate) fn walk_library_package_imports(package: &Node<LibraryPackage>, out: &mut Vec<String>) {
    walk_package_body(&package.value.body, out);
}

pub(crate) fn push_import_target(import: &Node<Import>, out: &mut Vec<String>) {
    let target = import.value.target.trim();
    if !target.is_empty() {
        out.push(target.to_string());
    }
}

pub(crate) fn push_type_reference(target: &str, out: &mut Vec<String>) {
    let target = target.trim();
    if target.is_empty() || target.starts_with("checks meta ") {
        return;
    }
    out.push(target.to_string());
}

pub(crate) fn push_optional_type_reference(target: Option<&str>, out: &mut Vec<String>) {
    if let Some(target) = target {
        push_type_reference(target, out);
    }
}
