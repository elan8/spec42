use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use glob::glob;
use tower_lsp::lsp_types::{Position, Range};
use tree_sitter::{Node, Parser, TreeCursor};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAnchor {
    pub file_path: String,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareComponent {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub parent_id: Option<String>,
    pub crate_name: String,
    pub module_path: String,
    pub anchors: Vec<SourceAnchor>,
    pub is_external: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareDependency {
    pub from: String,
    pub to: String,
    pub kind: String,
    pub source_anchor: Option<SourceAnchor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SoftwareArchitectureModel {
    pub components: Vec<SoftwareComponent>,
    pub dependencies: Vec<SoftwareDependency>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SoftwareAnalysisSummary {
    pub crate_count: usize,
    pub module_count: usize,
    pub dependency_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SoftwareWorkspaceModel {
    pub workspace_root: String,
    pub architecture: SoftwareArchitectureModel,
    pub summary: SoftwareAnalysisSummary,
}

#[derive(Debug, Clone)]
struct RustCrate {
    name: String,
    src_dir: PathBuf,
    root_file: PathBuf,
    declared_dependencies: HashMap<String, CargoDependencySpec>,
}

#[derive(Debug, Clone)]
struct RustFileInfo {
    file_path: String,
    module_segments: Vec<String>,
    use_paths: Vec<(String, Range)>,
    type_paths: Vec<(String, Range)>,
    inline_module_segments: Vec<Vec<String>>,
}

#[derive(Debug, Default)]
struct BuildContext {
    components: BTreeMap<String, SoftwareComponent>,
    dependencies: BTreeMap<(String, String, String), SoftwareDependency>,
    known_module_ids: HashSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DependencySourceKind {
    Use,
    TypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DependencyTarget {
    InternalModule(String),
    ExternalCrate(String),
    Ignore,
}

#[derive(Debug, Clone, Default)]
struct CargoWorkspaceContext {
    packages: Vec<CargoPackageContext>,
    workspace_crates_by_name: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct CargoPackageContext {
    crate_name: String,
    src_dir: PathBuf,
    root_file: PathBuf,
    declared_dependencies: HashMap<String, CargoDependencySpec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CargoDependencySpec {
    canonical_name: String,
    scope: CargoDependencyScope,
    optional: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CargoDependencyScope {
    Dependencies,
    DevDependencies,
    BuildDependencies,
}

pub fn workspace_contains_rust_code(workspace_root: &Path) -> bool {
    if workspace_root.join("Cargo.toml").exists() {
        return true;
    }
    WalkDir::new(workspace_root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .any(|entry| {
            entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "rs")
        })
}

pub fn extract_rust_workspace_architecture(workspace_root: &Path) -> SoftwareArchitectureModel {
    let cargo_context = build_cargo_workspace_context(workspace_root);
    let crates = if cargo_context.packages.is_empty() {
        discover_rust_crates_fallback(workspace_root)
    } else {
        cargo_context
            .packages
            .iter()
            .map(|package| RustCrate {
                name: package.crate_name.clone(),
                src_dir: package.src_dir.clone(),
                root_file: package.root_file.clone(),
                declared_dependencies: package.declared_dependencies.clone(),
            })
            .collect()
    };
    if crates.is_empty() {
        return SoftwareArchitectureModel::default();
    }

    let mut context = BuildContext::default();
    let mut crate_modules: HashMap<String, BTreeSet<Vec<String>>> = HashMap::new();
    let mut crate_files: HashMap<String, Vec<RustFileInfo>> = HashMap::new();

    for krate in &crates {
        let mut modules = BTreeSet::new();
        modules.insert(Vec::new());
        let files = collect_rust_files(krate);
        for file in &files {
            modules.insert(file.module_segments.clone());
            for inline in &file.inline_module_segments {
                modules.insert(inline.clone());
            }
        }
        crate_modules.insert(krate.name.clone(), modules);
        crate_files.insert(krate.name.clone(), files);
    }

    for krate in &crates {
        register_crate_and_modules(
            &mut context,
            krate,
            crate_modules
                .get(&krate.name)
                .expect("crate modules should be indexed"),
            crate_files
                .get(&krate.name)
                .expect("crate files should be indexed"),
        );
    }

    for krate in &crates {
        let known_modules = crate_modules
            .get(&krate.name)
            .expect("crate modules should be indexed");
        let files = crate_files
            .get(&krate.name)
            .expect("crate files should be indexed");
        for file in files {
            let from_id = module_id(&krate.name, &file.module_segments);
            let import_aliases = build_import_alias_map(&file.use_paths);
            for (path, range) in &file.use_paths {
                if let Some(target) = resolve_dependency_target(
                    krate,
                    &file.module_segments,
                    path,
                    known_modules,
                    &import_aliases,
                    &cargo_context,
                    DependencySourceKind::Use,
                ) {
                    register_dependency(
                        &mut context,
                        &from_id,
                        target,
                        "use",
                        &file_anchor(file, Some(*range)),
                    );
                }
            }
            for (path, range) in &file.type_paths {
                if let Some(target) = resolve_dependency_target(
                    krate,
                    &file.module_segments,
                    path,
                    known_modules,
                    &import_aliases,
                    &cargo_context,
                    DependencySourceKind::TypeRef,
                ) {
                    register_dependency(
                        &mut context,
                        &from_id,
                        target,
                        "typeRef",
                        &file_anchor(file, Some(*range)),
                    );
                }
            }
        }
    }

    SoftwareArchitectureModel {
        components: context.components.into_values().collect(),
        dependencies: context.dependencies.into_values().collect(),
    }
}

pub fn analyze_rust_workspace(workspace_root: &Path) -> SoftwareWorkspaceModel {
    let architecture = extract_rust_workspace_architecture(workspace_root);
    let summary = SoftwareAnalysisSummary {
        crate_count: architecture
            .components
            .iter()
            .filter(|component| component.kind == "crate")
            .count(),
        module_count: architecture
            .components
            .iter()
            .filter(|component| component.kind == "module")
            .count(),
        dependency_count: architecture.dependencies.len(),
    };

    SoftwareWorkspaceModel {
        workspace_root: normalize_file_path(workspace_root),
        architecture,
        summary,
    }
}

fn discover_rust_crates_fallback(workspace_root: &Path) -> Vec<RustCrate> {
    let mut crates = Vec::new();
    for entry in WalkDir::new(workspace_root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() || entry.file_name() != "Cargo.toml" {
            continue;
        }
        let package_dir = match entry.path().parent() {
            Some(parent) => parent.to_path_buf(),
            None => continue,
        };
        let src_dir = package_dir.join("src");
        if !src_dir.is_dir() {
            continue;
        }
        let root_file = if src_dir.join("lib.rs").is_file() {
            src_dir.join("lib.rs")
        } else if src_dir.join("main.rs").is_file() {
            src_dir.join("main.rs")
        } else {
            continue;
        };
        let crate_name = extract_package_name(entry.path()).unwrap_or_else(|| {
            package_dir
                .file_name()
                .map(|name| normalize_crate_name(&name.to_string_lossy()))
                .unwrap_or_else(|| "crate".to_string())
        });
        crates.push(RustCrate {
            name: crate_name,
            src_dir,
            root_file,
            declared_dependencies: HashMap::new(),
        });
    }
    crates.sort_by(|left, right| left.name.cmp(&right.name));
    crates
}

fn build_cargo_workspace_context(workspace_root: &Path) -> CargoWorkspaceContext {
    let root_manifest_path = workspace_root.join("Cargo.toml");
    let root_manifest = parse_cargo_manifest(&root_manifest_path);
    let workspace_dependency_aliases = root_manifest
        .as_ref()
        .map(parse_workspace_dependency_aliases)
        .unwrap_or_default();
    let member_manifest_paths =
        discover_workspace_member_manifests(workspace_root, root_manifest.as_ref());

    let mut packages = Vec::new();
    let mut workspace_crates_by_name = HashMap::new();
    for manifest_path in member_manifest_paths {
        let Some(package) =
            parse_cargo_package_context(&manifest_path, &workspace_dependency_aliases)
        else {
            continue;
        };
        workspace_crates_by_name.insert(package.crate_name.clone(), package.crate_name.clone());
        packages.push(package);
    }
    packages.sort_by(|left, right| left.crate_name.cmp(&right.crate_name));

    CargoWorkspaceContext {
        packages,
        workspace_crates_by_name,
    }
}

fn discover_workspace_member_manifests(
    workspace_root: &Path,
    root_manifest: Option<&toml::Value>,
) -> Vec<PathBuf> {
    let root_manifest_path = workspace_root.join("Cargo.toml");
    let mut manifests = BTreeSet::new();

    let root_is_package = root_manifest
        .and_then(|manifest| manifest.get("package"))
        .and_then(toml::Value::as_table)
        .is_some();
    if root_is_package && root_manifest_path.is_file() {
        manifests.insert(root_manifest_path.clone());
    }

    let workspace_table = root_manifest
        .and_then(|manifest| manifest.get("workspace"))
        .and_then(toml::Value::as_table);
    let Some(workspace_table) = workspace_table else {
        if manifests.is_empty() && root_manifest_path.is_file() {
            manifests.insert(root_manifest_path);
        }
        return manifests.into_iter().collect();
    };

    let excludes = workspace_table
        .get("exclude")
        .and_then(toml::Value::as_array)
        .map(|items| {
            items.iter()
                .filter_map(toml::Value::as_str)
                .map(normalize_glob_pattern)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let members = workspace_table
        .get("members")
        .and_then(toml::Value::as_array)
        .map(|items| {
            items.iter()
                .filter_map(toml::Value::as_str)
                .map(normalize_glob_pattern)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    for member in members {
        let pattern = workspace_root.join(&member).to_string_lossy().replace('\\', "/");
        let Ok(paths) = glob(&pattern) else {
            continue;
        };
        for path in paths.filter_map(Result::ok) {
            let manifest_path = if path.is_dir() {
                path.join("Cargo.toml")
            } else if path
                .file_name()
                .is_some_and(|file_name| file_name == "Cargo.toml")
            {
                path
            } else {
                path.join("Cargo.toml")
            };
            if !manifest_path.is_file() {
                continue;
            }
            let relative = manifest_path
                .strip_prefix(workspace_root)
                .ok()
                .map(|path| path.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            if excludes
                .iter()
                .any(|exclude| glob_pattern_matches(exclude, &relative))
            {
                continue;
            }
            manifests.insert(manifest_path);
        }
    }

    if manifests.is_empty() && root_manifest_path.is_file() {
        manifests.insert(root_manifest_path);
    }

    manifests.into_iter().collect()
}

fn normalize_glob_pattern(pattern: &str) -> String {
    pattern.trim().replace('\\', "/")
}

fn glob_pattern_matches(pattern: &str, value: &str) -> bool {
    glob::Pattern::new(pattern)
        .map(|compiled| compiled.matches(value))
        .unwrap_or(false)
}

fn parse_cargo_manifest(cargo_toml: &Path) -> Option<toml::Value> {
    let content = fs::read_to_string(cargo_toml).ok()?;
    toml::from_str(&content).ok()
}

fn parse_cargo_package_context(
    cargo_toml: &Path,
    workspace_dependency_aliases: &HashMap<String, String>,
) -> Option<CargoPackageContext> {
    let manifest = parse_cargo_manifest(cargo_toml)?;
    let package_name = manifest
        .get("package")
        .and_then(toml::Value::as_table)
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)?
        .to_string();
    let crate_name = normalize_crate_name(&package_name);
    let package_dir = cargo_toml.parent()?.to_path_buf();
    let src_dir = package_dir.join("src");
    if !src_dir.is_dir() {
        return None;
    }
    let root_file = if src_dir.join("lib.rs").is_file() {
        src_dir.join("lib.rs")
    } else if src_dir.join("main.rs").is_file() {
        src_dir.join("main.rs")
    } else {
        return None;
    };

    Some(CargoPackageContext {
        crate_name,
        src_dir,
        root_file,
        declared_dependencies: parse_package_declared_dependencies(
            &manifest,
            workspace_dependency_aliases,
        ),
    })
}

fn parse_workspace_dependency_aliases(manifest: &toml::Value) -> HashMap<String, String> {
    manifest
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|workspace| workspace.get("dependencies"))
        .map(|value| {
            parse_dependency_specs_from_value(
                value,
                &HashMap::new(),
                CargoDependencyScope::Dependencies,
            )
            .into_iter()
            .map(|(alias, spec)| (alias, spec.canonical_name))
            .collect()
        })
        .unwrap_or_default()
}

fn parse_package_declared_dependencies(
    manifest: &toml::Value,
    workspace_dependency_aliases: &HashMap<String, String>,
) -> HashMap<String, CargoDependencySpec> {
    let mut aliases = HashMap::new();
    for (key, scope) in [
        ("dependencies", CargoDependencyScope::Dependencies),
        ("dev-dependencies", CargoDependencyScope::DevDependencies),
        ("build-dependencies", CargoDependencyScope::BuildDependencies),
    ] {
        let Some(value) = manifest.get(key) else {
            continue;
        };
        aliases.extend(parse_dependency_specs_from_value(
            value,
            workspace_dependency_aliases,
            scope,
        ));
    }
    aliases
}

fn parse_dependency_specs_from_value(
    value: &toml::Value,
    workspace_dependency_aliases: &HashMap<String, String>,
    scope: CargoDependencyScope,
) -> HashMap<String, CargoDependencySpec> {
    let Some(table) = value.as_table() else {
        return HashMap::new();
    };
    let mut aliases = HashMap::new();
    for (alias, spec) in table {
        let Some(canonical) =
            dependency_canonical_name(alias, spec, workspace_dependency_aliases)
        else {
            continue;
        };
        aliases.insert(
            normalize_crate_name(alias),
            CargoDependencySpec {
                canonical_name: normalize_crate_name(&canonical),
                scope,
                optional: spec
                    .as_table()
                    .and_then(|table| table.get("optional"))
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(false),
            },
        );
    }
    aliases
}

fn dependency_canonical_name(
    alias: &str,
    spec: &toml::Value,
    workspace_dependency_aliases: &HashMap<String, String>,
) -> Option<String> {
    if spec.is_str() {
        return Some(alias.to_string());
    }
    let table = spec.as_table()?;
    if let Some(package_name) = table.get("package").and_then(toml::Value::as_str) {
        return Some(package_name.to_string());
    }
    if table
        .get("workspace")
        .and_then(toml::Value::as_bool)
        .unwrap_or(false)
    {
        return workspace_dependency_aliases
            .get(&normalize_crate_name(alias))
            .cloned()
            .or_else(|| Some(alias.to_string()));
    }
    Some(alias.to_string())
}

fn extract_package_name(cargo_toml: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_toml).ok()?;
    let mut in_package = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if !in_package || !trimmed.starts_with("name") {
            continue;
        }
        let (_, value) = trimmed.split_once('=')?;
        let name = value.trim().trim_matches('"');
        if !name.is_empty() {
            return Some(normalize_crate_name(name));
        }
    }
    None
}

fn collect_rust_files(krate: &RustCrate) -> Vec<RustFileInfo> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_rust::language())
        .expect("tree-sitter-rust language should load");

    let mut files = Vec::new();
    for entry in WalkDir::new(&krate.src_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() || entry.path().extension().is_none_or(|ext| ext != "rs")
        {
            continue;
        }
        let Ok(source) = fs::read_to_string(entry.path()) else {
            continue;
        };
        let module_segments =
            module_segments_for_file(&krate.src_dir, entry.path(), &krate.root_file);
        let parsed = parser.parse(&source, None);
        let Some(tree) = parsed else {
            files.push(RustFileInfo {
                file_path: normalize_file_path(entry.path()),
                module_segments,
                use_paths: Vec::new(),
                type_paths: Vec::new(),
                inline_module_segments: Vec::new(),
            });
            continue;
        };
        let mut use_paths = Vec::new();
        let mut type_paths = Vec::new();
        let mut inline_module_segments = Vec::new();
        collect_file_data(
            tree.root_node(),
            &source,
            &module_segments,
            &mut use_paths,
            &mut type_paths,
            &mut inline_module_segments,
        );
        files.push(RustFileInfo {
            file_path: normalize_file_path(entry.path()),
            module_segments,
            use_paths,
            type_paths,
            inline_module_segments,
        });
    }
    files.sort_by(|left, right| left.module_segments.cmp(&right.module_segments));
    files
}

fn module_segments_for_file(src_dir: &Path, file_path: &Path, root_file: &Path) -> Vec<String> {
    if file_path == root_file {
        return Vec::new();
    }
    let Ok(relative) = file_path.strip_prefix(src_dir) else {
        return Vec::new();
    };
    let mut parts: Vec<String> = relative
        .iter()
        .map(|part| part.to_string_lossy().to_string())
        .collect();
    if parts.last().is_some_and(|part| part == "mod.rs") {
        parts.pop();
    } else if let Some(last) = parts.last_mut() {
        if let Some(stripped) = last.strip_suffix(".rs") {
            *last = stripped.to_string();
        }
    }
    parts
}

fn collect_file_data(
    root: Node,
    source: &str,
    module_segments: &[String],
    use_paths: &mut Vec<(String, Range)>,
    type_paths: &mut Vec<(String, Range)>,
    inline_module_segments: &mut Vec<Vec<String>>,
) {
    let mut stack = vec![(root, module_segments.to_vec())];
    while let Some((node, module_path)) = stack.pop() {
        match node.kind() {
            "use_declaration" => {
                let text = utf8_text(node, source);
                for path in parse_use_statement_paths(&text) {
                    use_paths.push((path, node_range(node)));
                }
            }
            "mod_item" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = utf8_text(name_node, source).trim().to_string();
                    if !name.is_empty() {
                        let mut child_path = module_path.clone();
                        child_path.push(name);
                        inline_module_segments.push(child_path.clone());
                        if node.child_by_field_name("body").is_some() {
                            let mut cursor = node.walk();
                            for child in node.named_children(&mut cursor) {
                                if child.kind() == "declaration_list" {
                                    stack.push((child, child_path.clone()));
                                }
                            }
                        }
                    }
                }
            }
            "field_declaration" | "parameter" | "type_alias" | "const_item" | "static_item"
            | "let_declaration" | "tuple_struct_pattern" | "tuple_struct_item" => {
                collect_declared_type_paths(node, source, type_paths);
            }
            "function_item" => collect_function_type_paths(node, source, type_paths),
            _ => {}
        }

        let mut cursor: TreeCursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "declaration_list" && node.kind() == "mod_item" {
                continue;
            }
            stack.push((child, module_path.clone()));
        }
    }
}

fn collect_declared_type_paths(node: Node, source: &str, out: &mut Vec<(String, Range)>) {
    if let Some(type_node) = node.child_by_field_name("type") {
        collect_type_paths(type_node, source, out);
    }
}

fn collect_function_type_paths(node: Node, source: &str, out: &mut Vec<(String, Range)>) {
    if let Some(return_type) = node.child_by_field_name("return_type") {
        collect_type_paths(return_type, source, out);
    }
}

fn collect_type_paths(node: Node, source: &str, out: &mut Vec<(String, Range)>) {
    let kind = node.kind();
    let is_path_like = matches!(kind, "scoped_type_identifier" | "type_identifier");
    if is_path_like {
        let text = utf8_text(node, source);
        if is_meaningful_type_path(&text) {
            out.push((text, node_range(node)));
            return;
        }
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_type_paths(child, source, out);
    }
}

fn is_meaningful_type_path(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    !matches!(
        trimmed,
        "str"
            | "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
    )
}

fn utf8_text(node: Node, source: &str) -> String {
    node.utf8_text(source.as_bytes())
        .map(str::to_string)
        .unwrap_or_default()
}

fn node_range(node: Node) -> Range {
    let start = node.start_position();
    let end = node.end_position();
    Range {
        start: Position {
            line: start.row as u32,
            character: start.column as u32,
        },
        end: Position {
            line: end.row as u32,
            character: end.column as u32,
        },
    }
}

fn parse_use_statement_paths(statement: &str) -> Vec<String> {
    let trimmed = statement.trim();
    let without_prefix = trimmed
        .trim_start_matches("pub ")
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("pub(super) ")
        .trim_start_matches("pub(self) ")
        .trim_start_matches("use ")
        .trim_end_matches(';')
        .trim();
    expand_use_tree(without_prefix)
        .into_iter()
        .filter(|path| !path.is_empty() && path != "self")
        .collect()
}

fn expand_use_tree(tree: &str) -> Vec<String> {
    let tree = tree.trim();
    if tree.is_empty() {
        return Vec::new();
    }
    if let Some(open_idx) = find_top_level_char(tree, '{') {
        let close_idx =
            matching_brace_index(tree, open_idx).unwrap_or(tree.len().saturating_sub(1));
        let prefix = tree[..open_idx].trim_end_matches("::").trim();
        let suffix = if close_idx + 1 < tree.len() {
            tree[close_idx + 1..].trim()
        } else {
            ""
        };
        let mut expanded = Vec::new();
        for item in split_top_level(&tree[open_idx + 1..close_idx], ',') {
            for child in expand_use_tree(item) {
                let combined = match child.as_str() {
                    "self" => prefix.to_string(),
                    _ if prefix.is_empty() => child,
                    _ => format!("{prefix}::{child}"),
                };
                expanded.push(combined);
            }
        }
        if suffix.starts_with("::") {
            let suffix_trimmed = suffix.trim_start_matches("::");
            return expanded
                .into_iter()
                .flat_map(|base| {
                    expand_use_tree(suffix_trimmed)
                        .into_iter()
                        .map(move |tail| format!("{base}::{tail}"))
                })
                .collect();
        }
        return expanded;
    }
    if tree == "*" {
        return Vec::new();
    }
    vec![tree.to_string()]
}

fn split_top_level(input: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0_i32;
    let mut start = 0_usize;
    for (idx, ch) in input.char_indices() {
        match ch {
            '{' | '(' | '[' | '<' => depth += 1,
            '}' | ')' | ']' | '>' => depth -= 1,
            _ => {}
        }
        if ch == separator && depth == 0 {
            let part = input[start..idx].trim();
            if !part.is_empty() {
                parts.push(part);
            }
            start = idx + ch.len_utf8();
        }
    }
    let tail = input[start..].trim();
    if !tail.is_empty() {
        parts.push(tail);
    }
    parts
}

fn find_top_level_char(input: &str, needle: char) -> Option<usize> {
    let mut depth = 0_i32;
    for (idx, ch) in input.char_indices() {
        if ch == needle && depth == 0 {
            return Some(idx);
        }
        match ch {
            '{' | '(' | '[' | '<' => depth += 1,
            '}' | ')' | ']' | '>' => depth -= 1,
            _ => {}
        }
    }
    None
}

fn matching_brace_index(input: &str, open_idx: usize) -> Option<usize> {
    let mut depth = 0_i32;
    for (idx, ch) in input.char_indices().skip_while(|(idx, _)| *idx < open_idx) {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(idx);
                }
            }
            _ => {}
        }
    }
    None
}

fn register_crate_and_modules(
    context: &mut BuildContext,
    krate: &RustCrate,
    modules: &BTreeSet<Vec<String>>,
    files: &[RustFileInfo],
) {
    let crate_id = crate_id(&krate.name);
    context.known_module_ids.insert(crate_id.clone());
    let crate_anchor = SourceAnchor {
        file_path: normalize_file_path(&krate.root_file),
        range: Some(Range::default()),
    };
    context
        .components
        .entry(crate_id.clone())
        .or_insert(SoftwareComponent {
            id: crate_id.clone(),
            name: krate.name.clone(),
            kind: "crate".to_string(),
            parent_id: None,
            crate_name: krate.name.clone(),
            module_path: krate.name.clone(),
            anchors: vec![crate_anchor],
            is_external: false,
        });

    let mut anchors_by_module: HashMap<Vec<String>, Vec<SourceAnchor>> = HashMap::new();
    for file in files {
        anchors_by_module
            .entry(file.module_segments.clone())
            .or_default()
            .push(file_anchor(file, None));
    }

    for module in modules {
        if module.is_empty() {
            continue;
        }
        let id = module_id(&krate.name, module);
        context.known_module_ids.insert(id.clone());
        let parent_id = if module.len() == 1 {
            Some(crate_id.clone())
        } else {
            Some(module_id(&krate.name, &module[..module.len() - 1]))
        };
        let name = module.last().cloned().unwrap_or_default();
        let module_path = format!("{}::{}", krate.name, module.join("::"));
        let anchors = anchors_by_module.get(module).cloned().unwrap_or_default();
        context
            .components
            .entry(id.clone())
            .or_insert(SoftwareComponent {
                id,
                name,
                kind: "module".to_string(),
                parent_id,
                crate_name: krate.name.clone(),
                module_path,
                anchors,
                is_external: false,
            });
    }
}

fn build_import_alias_map(use_paths: &[(String, Range)]) -> HashMap<String, Vec<String>> {
    let mut aliases = HashMap::new();
    for (path, _) in use_paths {
        let segments = split_path_segments(path);
        if let Some(last) = segments.last() {
            aliases.insert(last.clone(), segments);
        }
    }
    aliases
}

fn resolve_dependency_target(
    krate: &RustCrate,
    current_module: &[String],
    path: &str,
    known_modules: &BTreeSet<Vec<String>>,
    aliases: &HashMap<String, Vec<String>>,
    cargo_context: &CargoWorkspaceContext,
    source_kind: DependencySourceKind,
) -> Option<String> {
    let segments = split_path_segments(path);
    if segments.is_empty() {
        return None;
    }
    let first = segments[0].as_str();
    let resolved_segments = if first == "crate" {
        vec!["crate".to_string()]
            .into_iter()
            .chain(segments.into_iter().skip(1))
            .collect::<Vec<_>>()
    } else if first == "self" {
        current_module
            .iter()
            .cloned()
            .chain(segments.into_iter().skip(1))
            .collect::<Vec<_>>()
    } else if first == "super" {
        current_module
            .iter()
            .take(current_module.len().saturating_sub(1))
            .cloned()
            .chain(segments.into_iter().skip(1))
            .collect::<Vec<_>>()
    } else if let Some(alias_target) = aliases.get(first) {
        alias_target
            .iter()
            .cloned()
            .chain(segments.into_iter().skip(1))
            .collect::<Vec<_>>()
    } else {
        segments
    };

    match classify_dependency_target(
        krate,
        &resolved_segments,
        known_modules,
        cargo_context,
        source_kind,
    ) {
        DependencyTarget::InternalModule(target_id) => Some(target_id),
        DependencyTarget::ExternalCrate(crate_name) => Some(external_crate_id(&crate_name)),
        DependencyTarget::Ignore => None,
    }
}

fn classify_dependency_target(
    krate: &RustCrate,
    resolved_segments: &[String],
    known_modules: &BTreeSet<Vec<String>>,
    cargo_context: &CargoWorkspaceContext,
    source_kind: DependencySourceKind,
) -> DependencyTarget {
    if resolved_segments.is_empty() {
        return DependencyTarget::Ignore;
    }

    let first = resolved_segments[0].as_str();
    if is_ignored_external_segment(first) {
        return DependencyTarget::Ignore;
    }

    if first == "crate" || first == krate.name {
        let internal_segments = resolved_segments.iter().skip(1).cloned().collect::<Vec<_>>();
        let target_module_segments = deepest_known_module_prefix(&internal_segments, known_modules);
        let target_id = if target_module_segments.is_empty() {
            crate_id(&krate.name)
        } else {
            module_id(&krate.name, &target_module_segments)
        };
        return DependencyTarget::InternalModule(target_id);
    }

    let target_module_segments = deepest_known_module_prefix(resolved_segments, known_modules);
    if !target_module_segments.is_empty() {
        return DependencyTarget::InternalModule(module_id(&krate.name, &target_module_segments));
    }

    if let Some(declared_dependency) = krate.declared_dependencies.get(first) {
        let canonical_dependency = &declared_dependency.canonical_name;
        if cargo_context
            .workspace_crates_by_name
            .contains_key(canonical_dependency)
        {
            return DependencyTarget::InternalModule(crate_id(canonical_dependency));
        }
        return DependencyTarget::ExternalCrate(canonical_dependency.clone());
    }

    if cargo_context.workspace_crates_by_name.contains_key(first) {
        return DependencyTarget::InternalModule(crate_id(first));
    }

    if !krate.declared_dependencies.is_empty() || !cargo_context.packages.is_empty() {
        return DependencyTarget::Ignore;
    }

    if !should_treat_as_external_crate(resolved_segments, source_kind) {
        return DependencyTarget::Ignore;
    }

    DependencyTarget::ExternalCrate(resolved_segments[0].clone())
}

fn should_treat_as_external_crate(
    resolved_segments: &[String],
    source_kind: DependencySourceKind,
) -> bool {
    let Some(first) = resolved_segments.first() else {
        return false;
    };
    if !is_crate_like_segment(first) {
        return false;
    }

    match source_kind {
        DependencySourceKind::Use => true,
        DependencySourceKind::TypeRef => resolved_segments.len() > 1,
    }
}

fn is_ignored_external_segment(segment: &str) -> bool {
    matches!(
        segment,
        "Self"
            | "self"
            | "super"
            | "crate"
            | "default"
            | "new"
            | "from"
            | "into"
    )
}

fn is_crate_like_segment(segment: &str) -> bool {
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
}

fn deepest_known_module_prefix(
    segments: &[String],
    known_modules: &BTreeSet<Vec<String>>,
) -> Vec<String> {
    for idx in (1..=segments.len()).rev() {
        let candidate = segments[..idx].to_vec();
        if known_modules.contains(&candidate) {
            return candidate;
        }
    }
    Vec::new()
}

fn register_dependency(
    context: &mut BuildContext,
    from: &str,
    to: String,
    kind: &str,
    anchor: &SourceAnchor,
) {
    if from == to {
        return;
    }
    if !context.components.contains_key(&to) {
        context.components.insert(
            to.clone(),
            SoftwareComponent {
                id: to.clone(),
                name: to.trim_start_matches("rust:extern:").to_string(),
                kind: "externalCrate".to_string(),
                parent_id: None,
                crate_name: to.trim_start_matches("rust:extern:").to_string(),
                module_path: to.trim_start_matches("rust:extern:").to_string(),
                anchors: Vec::new(),
                is_external: true,
            },
        );
    }
    let key = (from.to_string(), to.clone(), kind.to_string());
    context
        .dependencies
        .entry(key)
        .or_insert_with(|| SoftwareDependency {
            from: from.to_string(),
            to,
            kind: kind.to_string(),
            source_anchor: Some(anchor.clone()),
        });
}

fn split_path_segments(path: &str) -> Vec<String> {
    path.split("::")
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            normalize_crate_name(
                segment
                    .trim_start_matches('&')
                    .trim_start_matches("mut ")
                    .trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '_'),
            )
        })
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn file_anchor(file: &RustFileInfo, range: Option<Range>) -> SourceAnchor {
    SourceAnchor {
        file_path: file.source_path(),
        range,
    }
}

impl RustFileInfo {
    fn source_path(&self) -> String {
        self.file_path.clone()
    }
}

fn normalize_file_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn normalize_crate_name(name: &str) -> String {
    name.replace('-', "_")
}

fn crate_id(crate_name: &str) -> String {
    format!("rust:crate:{crate_name}")
}

fn external_crate_id(crate_name: &str) -> String {
    format!("rust:extern:{crate_name}")
}

fn module_id(crate_name: &str, module_segments: &[String]) -> String {
    if module_segments.is_empty() {
        return crate_id(crate_name);
    }
    format!("rust:module:{crate_name}::{}", module_segments.join("::"))
}
