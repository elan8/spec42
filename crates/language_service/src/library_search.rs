//! Ranking, origin labelling and grouping for the library symbol browser.
//!
//! Editor intelligence over the symbol entries a publication already settled: which candidates a
//! query matches and how strongly, which library each one came from, and how they group into
//! sources and packages for a browsable tree. No host re-derives any of it, and none of it decides
//! a semantic fact — the entries themselves come from [`crate::symbol_entries_for_uri`] over the
//! published model.
//!
//! The `kind` an item carries is the host's own symbol-kind vocabulary, which the host fills in;
//! this module only groups on it.

use std::collections::BTreeMap;

use sysml_query::resolved_slice::{TextPosition, TextRange};
use url::Url;

use crate::symbol::SymbolEntry;

/// One ranked library symbol, in the protocol-neutral shape a host renders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySearchItem {
    pub name: String,
    pub kind: String,
    pub container: Option<String>,
    pub uri: String,
    pub range: TextRange,
    pub score: i64,
    pub source: String,
    pub path: String,
}

/// The symbols of one library document, under the package name it declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySearchPackage {
    pub name: String,
    pub path: String,
    pub source: String,
    pub symbols: Vec<LibrarySearchItem>,
}

/// The packages of one library origin (`standard`, `domain`, `custom`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibrarySearchSource {
    pub source: String,
    pub packages: Vec<LibrarySearchPackage>,
}

/// How strongly `name` matches an already lowercased `query_lc`, or `None` for no match.
///
/// Exact beats prefix beats substring beats subsequence, and a shorter name wins within a band, so
/// the ordering is total and independent of the order candidates arrive in.
pub fn library_search_score(name: &str, query_lc: &str) -> Option<i64> {
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

/// Groups ranked items into the source/package tree the browser renders.
pub fn build_library_tree(items: Vec<LibrarySearchItem>) -> Vec<LibrarySearchSource> {
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

/// Which library a document came from, as the browser groups them.
///
/// `standard_library_roots` is the configuration's own answer to "which roots are the standard
/// library", the same list the host classifies `SourceKind::StandardLibrary` from, so the
/// load-bearing case is decided by what configuration states rather than by a path substring.
///
/// The remaining split is a presentation grouping and degrades explicitly: `domain` names the
/// managed domain-library layout `library_catalog` materialises under the data directory, and
/// anything else is `custom`. Neither answer is a semantic fact, and neither changes what a
/// document resolves to.
pub fn library_source_label(uri: &Url, standard_library_roots: &[Url]) -> &'static str {
    if sysml_query::source::uri_under_any(uri, standard_library_roots) {
        return "standard";
    }
    if uri
        .path()
        .to_ascii_lowercase()
        .contains(MANAGED_DOMAIN_LIBRARY_SEGMENT)
    {
        return "domain";
    }
    "custom"
}

/// The directory segment the managed domain-library install uses.
const MANAGED_DOMAIN_LIBRARY_SEGMENT: &str = "/domain-libraries/";

/// A syntax-recovery search candidate for a document deliberately excluded from semantic
/// publication. This is not a resolved symbol and must never be used by semantic consumers.
#[derive(Debug, Clone)]
pub struct RecoverySearchSymbol(SymbolEntry);

impl RecoverySearchSymbol {
    /// Unwraps the candidate for the search index, naming what it is at the call site.
    pub fn into_search_only_symbol(self) -> SymbolEntry {
        self.0
    }
}

/// Recovers `<shortName>` declarations from a library document the publication did not admit.
///
/// Search-only recovery, and typed as such: a caller must name
/// [`RecoverySearchSymbol::into_search_only_symbol`] to use one, so a recovered candidate cannot be
/// mistaken for a published symbol.
pub fn search_symbols_from_recovered_short_names(
    parsed: &sysml_query::syntax::ParsedSource,
    uri: &Url,
) -> Vec<RecoverySearchSymbol> {
    parsed
        .recovered_short_names()
        .into_iter()
        .map(|short_name| {
            debug_assert_eq!(
                short_name.provenance,
                sysml_query::syntax::SyntaxRecoveryProvenance::ParserRecovery
            );
            RecoverySearchSymbol(SymbolEntry {
                name: short_name.name.to_owned(),
                uri: uri.clone(),
                range: TextRange::new(
                    TextPosition::new(
                        short_name.range.start_line,
                        short_name.range.start_character,
                    ),
                    TextPosition::new(short_name.range.end_line, short_name.range.end_character),
                ),
                container_name: None,
                detail: Some("recovered short name".to_string()),
                description: short_name
                    .declaration_name
                    .map(|name| format!("parser-recovered short name for {name}")),
                signature: None,
            })
        })
        .collect()
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
    use super::*;

    #[test]
    fn library_search_score_prefers_exact_match() {
        let exact = library_search_score("Engine", "engine").expect("score");
        let fuzzy = library_search_score("EngineController", "engine").expect("score");
        assert!(exact > fuzzy, "exact matches should score higher");
    }

    #[test]
    fn library_source_label_reads_the_configured_standard_library_roots() {
        let root = Url::parse("file:///tmp/data/standard-library/versions/2026-04/sysml.library/")
            .expect("url");
        let stdlib = Url::parse(
            "file:///tmp/data/standard-library/versions/2026-04/sysml.library/ScalarValues.sysml",
        )
        .expect("url");
        let domain = Url::parse("file:///tmp/data/domain-libraries/versions/dc378a9/tree/generic/RequirementMetadata.sysml")
            .expect("url");
        let custom = Url::parse("file:///workspace/libs/Domain.sysml").expect("url");
        let roots = vec![root];
        assert_eq!(library_source_label(&stdlib, &roots), "standard");
        assert_eq!(library_source_label(&domain, &roots), "domain");
        assert_eq!(library_source_label(&custom, &roots), "custom");
    }

    #[test]
    fn a_standard_library_root_outside_the_conventional_layout_is_still_standard() {
        let root = Url::parse("file:///opt/my-sysml-lib/").expect("url");
        let document = Url::parse("file:///opt/my-sysml-lib/ScalarValues.sysml").expect("url");
        assert_eq!(
            library_source_label(&document, std::slice::from_ref(&root)),
            "standard",
            "configuration states which roots are the standard library; a path convention does not"
        );
        assert_eq!(library_source_label(&document, &[]), "custom");
    }
}
