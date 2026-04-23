use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone)]
struct RustCrate {
    name: String,
    src_dir: PathBuf,
    root_file: PathBuf,
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
    let crates = discover_rust_crates(workspace_root);
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
                    &krate.name,
                    &file.module_segments,
                    path,
                    known_modules,
                    &import_aliases,
                ) {
                    register_dependency(
                        &mut context,
                        &from_id,
                        target,
                        "use",
                        &file_anchor(file, Some(range.clone())),
                    );
                }
            }
            for (path, range) in &file.type_paths {
                if let Some(target) = resolve_dependency_target(
                    &krate.name,
                    &file.module_segments,
                    path,
                    known_modules,
                    &import_aliases,
                ) {
                    register_dependency(
                        &mut context,
                        &from_id,
                        target,
                        "typeRef",
                        &file_anchor(file, Some(range.clone())),
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

fn discover_rust_crates(workspace_root: &Path) -> Vec<RustCrate> {
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
                .map(|name| name.to_string_lossy().replace('-', "_"))
                .unwrap_or_else(|| "crate".to_string())
        });
        crates.push(RustCrate {
            name: crate_name,
            src_dir,
            root_file,
        });
    }
    crates.sort_by(|left, right| left.name.cmp(&right.name));
    crates
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
            return Some(name.replace('-', "_"));
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
        if !entry.file_type().is_file() || !entry.path().extension().is_some_and(|ext| ext == "rs")
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
            kind if is_type_context(kind) => {
                collect_type_paths(node, source, type_paths, false);
            }
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

fn is_type_context(kind: &str) -> bool {
    matches!(
        kind,
        "field_declaration"
            | "parameter"
            | "function_item"
            | "type_alias"
            | "const_item"
            | "static_item"
            | "let_declaration"
            | "tuple_struct_pattern"
            | "tuple_struct_item"
    )
}

fn collect_type_paths(
    node: Node,
    source: &str,
    out: &mut Vec<(String, Range)>,
    nested_in_type: bool,
) {
    let kind = node.kind();
    let is_path_like = matches!(kind, "scoped_type_identifier" | "type_identifier");
    if is_path_like {
        let text = utf8_text(node, source);
        if is_meaningful_type_path(&text) {
            out.push((text, node_range(node)));
            return;
        }
    }
    let next_nested = nested_in_type
        || kind.contains("type")
        || matches!(kind, "parameter" | "field_declaration" | "function_item");
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_type_paths(child, source, out, next_nested);
    }
    if kind == "identifier" && nested_in_type {
        let text = utf8_text(node, source);
        if is_meaningful_type_path(&text) {
            out.push((text, node_range(node)));
        }
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
    crate_name: &str,
    current_module: &[String],
    path: &str,
    known_modules: &BTreeSet<Vec<String>>,
    aliases: &HashMap<String, Vec<String>>,
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

    if resolved_segments.is_empty() {
        return None;
    }

    if resolved_segments[0] == "crate" || resolved_segments[0] == crate_name {
        let internal_segments = resolved_segments.into_iter().skip(1).collect::<Vec<_>>();
        let target_module_segments = deepest_known_module_prefix(&internal_segments, known_modules);
        let target_id = if target_module_segments.is_empty() {
            crate_id(crate_name)
        } else {
            module_id(crate_name, &target_module_segments)
        };
        return Some(target_id);
    }

    let target_module_segments = deepest_known_module_prefix(&resolved_segments, known_modules);
    if !target_module_segments.is_empty() {
        return Some(module_id(crate_name, &target_module_segments));
    }

    Some(external_crate_id(&resolved_segments[0]))
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
        .flat_map(|segment| segment.split('.'))
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            segment
                .trim_start_matches('&')
                .trim_start_matches("mut ")
                .trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '_')
                .to_string()
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

#[cfg(test)]
mod tests {
    use super::{
        crate_id, expand_use_tree, external_crate_id, extract_rust_workspace_architecture,
        module_id, module_segments_for_file, parse_use_statement_paths,
        workspace_contains_rust_code,
    };
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn parses_nested_use_trees() {
        let paths = parse_use_statement_paths("use crate::domain::{model::{User, Team}, repo};");
        assert_eq!(
            paths,
            vec![
                "crate::domain::model::User".to_string(),
                "crate::domain::model::Team".to_string(),
                "crate::domain::repo".to_string()
            ]
        );
        assert_eq!(
            expand_use_tree("super::{self, api::Client}"),
            vec!["super".to_string(), "super::api::Client".to_string()]
        );
    }

    #[test]
    fn normalizes_module_paths_for_rs_and_mod_rs() {
        let root = PathBuf::from("/repo/src");
        let crate_root = root.join("lib.rs");
        assert_eq!(
            module_segments_for_file(&root, &root.join("domain/user.rs"), &crate_root),
            vec!["domain".to_string(), "user".to_string()]
        );
        assert_eq!(
            module_segments_for_file(&root, &root.join("domain/mod.rs"), &crate_root),
            vec!["domain".to_string()]
        );
    }

    #[test]
    fn extracts_workspace_architecture_for_rust_modules_and_dependencies() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo_app\"\nversion = \"0.1.0\"\n",
        )
        .expect("cargo");
        fs::create_dir_all(root.join("src/domain")).expect("mkdir");
        fs::write(
            root.join("src/lib.rs"),
            r#"
                mod domain;
                use crate::domain::model::User;

                pub fn run(user: User) {}
            "#,
        )
        .expect("lib");
        fs::write(
            root.join("src/domain/mod.rs"),
            r#"
                pub mod model;
            "#,
        )
        .expect("domain mod");
        fs::write(
            root.join("src/domain/model.rs"),
            r#"
                use serde::Serialize;

                pub struct User {
                    pub id: String,
                }

                pub fn encode(user: &User) -> impl Serialize {
                    user.id.clone()
                }
            "#,
        )
        .expect("model");

        assert!(workspace_contains_rust_code(root));
        let model = extract_rust_workspace_architecture(root);

        assert!(model
            .components
            .iter()
            .any(|component| component.id == module_id("demo_app", &["domain".to_string()])));
        assert!(model.components.iter().any(|component| component.id
            == module_id("demo_app", &["domain".to_string(), "model".to_string()])));
        assert!(model.dependencies.iter().any(|dependency| {
            dependency.from == crate_id("demo_app")
                && dependency.to
                    == module_id("demo_app", &["domain".to_string(), "model".to_string()])
                && dependency.kind == "use"
        }));
        assert!(model.dependencies.iter().any(|dependency| {
            dependency.from == module_id("demo_app", &["domain".to_string(), "model".to_string()])
                && dependency.to == external_crate_id("serde")
        }));
    }
}
