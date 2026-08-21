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

/// Expand the cheap top-level package index only where a workspace and a library share a
/// namespace root.
///
/// The normal index deliberately reads only the first package declaration in each file so
/// import-closure startup does not parse the entire library corpus. That is sufficient until a
/// workspace contributes to the same namespace as a library, for example
/// `Elan8::Photonics` beside bundled `Elan8::Method` and `Elan8::Electronics`. In that case the
/// top-level `Elan8` workspace package satisfies the root seed, and the nested library packages
/// need their own index entries to remain reachable.
pub(crate) fn expand_library_namespaces_shared_with_workspace(
    index: &mut PackageIndex,
    workspace_packages: &HashSet<PackageKey>,
) -> Result<(), String> {
    let shared_roots: Vec<PackageKey> = workspace_packages
        .iter()
        .filter(|package| !package.0.contains("::") && index.packages.contains_key(*package))
        .cloned()
        .collect();

    for root in shared_roots {
        let candidates = index.packages.get(&root).cloned().unwrap_or_default();
        for candidate in candidates {
            let full_path = PathBuf::from(&candidate.root).join(&candidate.path);
            let content = std::fs::read_to_string(&full_path).map_err(|err| {
                format!(
                    "failed to read shared-namespace library file {}: {err}",
                    full_path.display()
                )
            })?;
            for package in sysml_resolution::syntax::declared_package_names(&content) {
                let key = PackageKey(package);
                if key == root {
                    continue;
                }
                let entries = index.packages.entry(key).or_default();
                if !entries
                    .iter()
                    .any(|entry| entry.root == candidate.root && entry.path == candidate.path)
                {
                    entries.push(candidate.clone());
                }
            }
        }
    }

    Ok(())
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

pub(crate) fn workspace_declared_packages(
    workspace: &[WorkspaceSource<'_>],
) -> HashSet<PackageKey> {
    let mut defined = HashSet::new();
    for source in workspace {
        defined.extend(
            sysml_resolution::syntax::declared_package_names(source.content)
                .into_iter()
                .map(PackageKey),
        );
    }
    defined
}

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
    for target in sysml_resolution::syntax::import_targets(content) {
        for next in package_keys_for_import_target(&target) {
            queue.push_back(PackageKey(next));
        }
    }
    if options.bootstrap_typing_references {
        for target in sysml_resolution::syntax::type_reference_targets(content) {
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
        for package in sysml_resolution::syntax::package_targets(source.content) {
            if package.qualified_name != pkg.0 {
                continue;
            }
            for target in package.import_targets {
                for next in package_keys_for_import_target(&target) {
                    queue.push_back(PackageKey(next));
                }
            }
            if options.bootstrap_typing_references {
                for target in package.type_reference_targets {
                    for next in package_keys_for_import_target(&target) {
                        queue.push_back(PackageKey(next));
                    }
                }
            }
        }
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
