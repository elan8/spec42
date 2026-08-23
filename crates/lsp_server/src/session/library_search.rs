use std::collections::BTreeMap;

use sysml_query::resolved_slice::{TextPosition, TextRange};
use tower_lsp::lsp_types::{Range, SymbolKind, Url};

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
    } else if path.contains("/domain-libraries/") {
        "domain"
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

/// A syntax-recovery search candidate for a document deliberately excluded from semantic
/// publication. This is not a resolved symbol and must never be used by semantic consumers.
#[derive(Debug, Clone)]
pub(crate) struct RecoverySearchSymbol(crate::language::SymbolEntry);

impl RecoverySearchSymbol {
    pub(crate) fn into_search_only_symbol(self) -> crate::language::SymbolEntry {
        self.0
    }
}

pub(crate) fn recover_short_name_search_symbols(
    content: &str,
    uri: &Url,
) -> Vec<RecoverySearchSymbol> {
    let mut entries: Vec<RecoverySearchSymbol> = Vec::new();
    let mut existing_names = std::collections::HashSet::new();
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
                .find(|e| e.0.range.start.line == line_idx as u32 && !e.0.name.trim().is_empty());
            let (container_name, detail, description) = match anchor {
                Some(a) => (
                    a.0.container_name.clone(),
                    a.0.detail.clone(),
                    Some(format!("short name for {}", a.0.name)),
                ),
                None => (
                    None,
                    Some("short name".to_string()),
                    Some("short name from declaration".to_string()),
                ),
            };
            entries.push(RecoverySearchSymbol(crate::language::SymbolEntry {
                name: token.to_string(),
                uri: uri.clone(),
                range: TextRange::new(
                    TextPosition::new(line_idx as u32, start_char),
                    TextPosition::new(line_idx as u32, end_char),
                ),
                container_name,
                detail,
                description,
                signature: None,
            }));
            existing_names.insert(token.to_string());
        }
    }
    entries
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
    use super::{library_search_score, library_source_label};
    use tower_lsp::lsp_types::Url;

    #[test]
    fn library_search_score_prefers_exact_match() {
        let exact = library_search_score("Engine", "engine").expect("score");
        let fuzzy = library_search_score("EngineController", "engine").expect("score");
        assert!(exact > fuzzy, "exact matches should score higher");
    }

    #[test]
    fn library_source_label_classifies_bundled_roots() {
        let stdlib = Url::parse(
            "file:///tmp/data/standard-library/versions/2026-04/sysml.library/ScalarValues.sysml",
        )
        .expect("url");
        let domain = Url::parse("file:///tmp/data/domain-libraries/versions/dc378a9/tree/generic/RequirementMetadata.sysml")
            .expect("url");
        let custom = Url::parse("file:///workspace/libs/Domain.sysml").expect("url");
        assert_eq!(library_source_label(&stdlib), "standard");
        assert_eq!(library_source_label(&domain), "domain");
        assert_eq!(library_source_label(&custom), "custom");
    }
}
