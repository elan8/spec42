use std::collections::BTreeMap;

use tower_lsp::lsp_types::{Range, SymbolKind, Url};

use crate::workspace::IndexEntry;

#[derive(Debug, Clone)]
pub(crate) struct LibrarySearchItem {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) container: Option<String>,
    pub(crate) uri: String,
    pub(crate) range: Range,
    pub(crate) score: i64,
    pub(crate) source: String,
    pub(crate) path: String,
}

#[derive(Debug, Clone)]
pub(crate) struct LibrarySearchPackage {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) source: String,
    pub(crate) symbols: Vec<LibrarySearchItem>,
}

#[derive(Debug, Clone)]
pub(crate) struct LibrarySearchSource {
    pub(crate) source: String,
    pub(crate) packages: Vec<LibrarySearchPackage>,
}

pub(crate) fn symbol_kind_label(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::FILE => "file",
        SymbolKind::MODULE => "module",
        SymbolKind::NAMESPACE => "namespace",
        SymbolKind::PACKAGE => "package",
        SymbolKind::CLASS => "class",
        SymbolKind::METHOD => "method",
        SymbolKind::PROPERTY => "property",
        SymbolKind::FIELD => "field",
        SymbolKind::CONSTRUCTOR => "constructor",
        SymbolKind::ENUM => "enum",
        SymbolKind::INTERFACE => "interface",
        SymbolKind::FUNCTION => "function",
        SymbolKind::VARIABLE => "variable",
        SymbolKind::CONSTANT => "constant",
        SymbolKind::STRING => "string",
        SymbolKind::NUMBER => "number",
        SymbolKind::BOOLEAN => "boolean",
        SymbolKind::ARRAY => "array",
        SymbolKind::OBJECT => "object",
        SymbolKind::KEY => "key",
        SymbolKind::NULL => "null",
        SymbolKind::ENUM_MEMBER => "enumMember",
        SymbolKind::STRUCT => "struct",
        SymbolKind::EVENT => "event",
        SymbolKind::OPERATOR => "operator",
        SymbolKind::TYPE_PARAMETER => "typeParameter",
        _ => "symbol",
    }
}

pub(crate) fn normalized_library_symbol_name(
    entry: &crate::language::SymbolEntry,
    index_entry: Option<&IndexEntry>,
) -> String {
    if !is_generic_symbol_name(&entry.name) {
        return entry.name.clone();
    }
    if let Some(content) = index_entry.map(|idx| idx.content.as_str()) {
        if let Some(name) =
            extract_declared_name_from_line(content, entry.range.start.line as usize)
        {
            return name;
        }
    }
    entry.name.clone()
}

fn is_generic_symbol_name(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "" | "def" | "usage"
    )
}

pub(crate) fn extract_declared_name_from_line(content: &str, line_idx: usize) -> Option<String> {
    let line = content.lines().nth(line_idx)?.trim();
    if line.is_empty() {
        return None;
    }
    // Normalize punctuation so "allocation def Allocation :>" tokenizes predictably.
    let normalized = line
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '-' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>();
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }
    for i in 0..(tokens.len() - 1) {
        let tok = tokens[i].to_ascii_lowercase();
        if (tok == "def" || tok == "usage") && is_valid_decl_name(tokens[i + 1]) {
            return Some(tokens[i + 1].to_string());
        }
    }
    None
}

fn is_valid_decl_name(token: &str) -> bool {
    let mut chars = token.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '-')
}

pub(crate) fn build_library_tree(items: Vec<LibrarySearchItem>) -> Vec<LibrarySearchSource> {
    let mut by_source: BTreeMap<String, BTreeMap<String, Vec<LibrarySearchItem>>> = BTreeMap::new();
    let mut package_name_by_source_path: BTreeMap<(String, String), String> = BTreeMap::new();

    for item in &items {
        if item.kind == "module" && !item.name.trim().is_empty() {
            package_name_by_source_path
                .entry((item.source.clone(), item.path.clone()))
                .or_insert_with(|| item.name.clone());
        }
    }

    for item in items {
        let source = item.source.clone();
        let package_name = package_name_by_source_path
            .get(&(source.clone(), item.path.clone()))
            .cloned()
            .unwrap_or_else(|| package_name_from_path(&item.path));
        by_source
            .entry(source)
            .or_default()
            .entry(package_name)
            .or_default()
            .push(item);
    }

    let mut out = Vec::new();
    for (source, mut by_package) in by_source {
        let mut packages = Vec::new();
        for (package_name, symbols) in by_package.iter_mut() {
            symbols.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
            symbols.retain(|s| {
                // Do not duplicate the package module symbol as child entry.
                !(s.kind == "module" && s.name.eq_ignore_ascii_case(package_name))
            });
        }

        for (package_name, symbols) in by_package {
            if symbols.is_empty() {
                continue;
            }
            let path = symbols
                .first()
                .map(|s| s.path.clone())
                .unwrap_or_else(|| package_name.clone());
            packages.push(LibrarySearchPackage {
                name: package_name,
                path,
                source: source.clone(),
                symbols,
            });
        }

        packages.sort_by(|a, b| a.name.cmp(&b.name));
        out.push(LibrarySearchSource { source, packages });
    }

    out
}

fn package_name_from_path(path: &str) -> String {
    let file = path.rsplit('/').next().unwrap_or(path);
    if let Some(stem) = file.strip_suffix(".sysml") {
        return stem.to_string();
    }
    if let Some(stem) = file.strip_suffix(".kerml") {
        return stem.to_string();
    }
    file.to_string()
}

pub(crate) fn library_source_label(uri: &Url) -> &'static str {
    let path = uri.path().to_ascii_lowercase();
    if path.contains("/standard-library/") {
        "standard"
    } else {
        "custom"
    }
}

pub(crate) fn library_search_score(name: &str, query_lc: &str) -> Option<i64> {
    let name_lc = name.to_ascii_lowercase();
    if name_lc == query_lc {
        return Some(10_000);
    }
    if name_lc.starts_with(query_lc) {
        return Some(8_000 - (name_lc.len() as i64));
    }
    if let Some(pos) = name_lc.find(query_lc) {
        return Some(6_000 - (pos as i64) * 10 - (name_lc.len() as i64));
    }
    fuzzy_subsequence_score(&name_lc, query_lc).map(|s| 4_000 + s)
}

pub(crate) fn add_short_name_symbol_entries(
    entries: &mut Vec<crate::language::SymbolEntry>,
    content: &str,
    uri: &Url,
) {
    let mut existing_names: std::collections::HashSet<String> =
        entries.iter().map(|e| e.name.clone()).collect();
    for (line_idx, line) in content.lines().enumerate() {
        let mut cursor = 0usize;
        while let Some(open_rel) = line[cursor..].find('<') {
            let open = cursor + open_rel;
            let after_open = open + 1;
            let Some(close_rel) = line[after_open..].find('>') else {
                break;
            };
            let close = after_open + close_rel;
            let token = &line[after_open..close];
            cursor = close + 1;
            if !is_valid_decl_name(token) || existing_names.contains(token) {
                continue;
            }

            let start_char = line[..after_open].chars().count() as u32;
            let end_char = start_char + token.chars().count() as u32;
            let anchor = entries
                .iter()
                .find(|e| e.range.start.line == line_idx as u32 && !e.name.trim().is_empty());
            let (kind, container_name, detail, description) = match anchor {
                Some(a) => (
                    a.kind,
                    a.container_name.clone(),
                    a.detail.clone(),
                    Some(format!("short name for {}", a.name)),
                ),
                None => (
                    SymbolKind::VARIABLE,
                    None,
                    Some("short name".to_string()),
                    Some("short name from declaration".to_string()),
                ),
            };
            entries.push(crate::language::SymbolEntry {
                name: token.to_string(),
                uri: uri.clone(),
                range: Range::new(
                    tower_lsp::lsp_types::Position::new(line_idx as u32, start_char),
                    tower_lsp::lsp_types::Position::new(line_idx as u32, end_char),
                ),
                kind,
                container_name,
                detail,
                description,
                signature: None,
            });
            existing_names.insert(token.to_string());
        }
    }
}

fn fuzzy_subsequence_score(text: &str, query: &str) -> Option<i64> {
    if query.is_empty() {
        return Some(0);
    }
    let mut score: i64 = 0;
    let mut text_index = 0usize;
    let text_chars: Vec<char> = text.chars().collect();
    for ch in query.chars() {
        let mut found = None;
        for (idx, c) in text_chars.iter().enumerate().skip(text_index) {
            if *c == ch {
                found = Some(idx);
                break;
            }
        }
        let idx = found?;
        score += 100 - ((idx - text_index) as i64 * 3);
        text_index = idx + 1;
    }
    Some(score.max(0))
}

#[cfg(test)]
mod tests {
    use super::{extract_declared_name_from_line, library_search_score};

    #[test]
    fn extract_declared_name_handles_specialization_line() {
        let line = "standard library package Allocations { allocation def Allocation :> BinaryConnection; }";
        let name = extract_declared_name_from_line(line, 0);
        assert_eq!(name.as_deref(), Some("Allocation"));
    }

    #[test]
    fn library_search_score_prefers_exact_match() {
        let exact = library_search_score("Engine", "engine").expect("score");
        let fuzzy = library_search_score("EngineController", "engine").expect("score");
        assert!(exact > fuzzy, "exact matches should score higher");
    }
}
