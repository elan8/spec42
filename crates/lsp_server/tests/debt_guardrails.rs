use std::fs;
use std::path::{Path, PathBuf};

const MAX_IGNORE_ATTRIBUTES: usize = 6;
const MAX_ALLOW_ATTRIBUTES_IN_SRC: usize = 38;
const MAX_FRONTEND_SKIPPED_TESTS: usize = 0;

#[test]
fn ignored_test_count_does_not_increase() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let count = count_pattern(&root.join("src"), "#[ignore]")
        + count_pattern(&root.join("tests"), "#[ignore]");
    assert!(
        count <= MAX_IGNORE_ATTRIBUTES,
        "ignored test count regressed: {count} > {MAX_IGNORE_ATTRIBUTES}"
    );
}

#[test]
fn allow_attribute_count_in_src_does_not_increase() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let count = count_pattern(&root.join("src"), "#[allow(");
    assert!(
        count <= MAX_ALLOW_ATTRIBUTES_IN_SRC,
        "allow attribute count regressed in src: {count} > {MAX_ALLOW_ATTRIBUTES_IN_SRC}"
    );
}

#[test]
fn kernel_semantic_layer_contains_only_shims_and_runtime_modules() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/semantic");
    let forbidden = [root.join("graph_builder"), root.join("evaluation/units.rs")];

    let existing = forbidden
        .iter()
        .filter(|path| path.exists())
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();

    assert!(
        existing.is_empty(),
        "reusable semantic implementations belong in sysml_resolution, not kernel:\n{}",
        existing.join("\n")
    );
}

#[test]
fn frontend_normalize_payload_line_count_does_not_increase() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest.parent().and_then(Path::parent).expect("repo root");
    let normalize_path = repo_root.join("vscode/diagram-renderer/src/prepare/normalize-payload.ts");
    let contents = fs::read_to_string(normalize_path).expect("normalize-payload.ts");
    const MAX_NORMALIZE_PAYLOAD_LINES: usize = 100;
    let line_count = contents.lines().count();
    assert!(
        line_count <= MAX_NORMALIZE_PAYLOAD_LINES,
        "normalize-payload.ts regressed: {line_count} lines > {MAX_NORMALIZE_PAYLOAD_LINES}"
    );
}

#[test]
fn frontend_skipped_test_count_does_not_increase() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf();
    let test_root = repo_root.join("vscode/src/test");
    let count = count_pattern(&test_root, "it.skip(") + count_pattern(&test_root, "describe.skip(");

    assert_eq!(
        count, MAX_FRONTEND_SKIPPED_TESTS,
        "frontend skipped test count regressed: expected {MAX_FRONTEND_SKIPPED_TESTS}, got {count}"
    );
}

#[test]
fn lsp_workspace_does_not_own_semantic_build_or_library_cache() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for forbidden in [
        "BuildRequest::resolved",
        "resolved_with_library",
        "LibraryStratum::build",
        "CachedLibraryStratum",
        "sysml_resolution::",
        "sysml_source::",
    ] {
        assert_eq!(
            count_occurrences(&root, forbidden),
            0,
            "the LSP workspace must publish through sysml_query::publication; found {forbidden}"
        );
    }
}

#[test]
fn syntax_recovery_cannot_enter_the_admitted_symbol_projection() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let state = fs::read_to_string(root.join("session/state.rs")).expect("workspace state");
    assert!(
        !state.contains("recover_short_name_search_symbols"),
        "the committed symbol table must contain only exact PublishedModel query results"
    );

    // Recovery has one declaration and four deliberately search-only call sites. A new use must
    // make its non-admitted provenance explicit and update this architectural gate deliberately.
    assert_eq!(
        count_occurrences(&root, "recover_short_name_search_symbols"),
        5,
        "syntax-recovery search projection escaped its reviewed boundary"
    );
    assert_eq!(
        count_occurrences(&root, "normalized_library_symbol_name"),
        0,
        "library search must not replace typed-query names by re-parsing source text"
    );
}

#[test]
fn diagnostic_dependency_guessing_remains_explicitly_recovery_only() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    assert_eq!(
        count_occurrences(&root, "workspace_uris_importing_declarations_from"),
        0,
        "syntax inspection must not masquerade as a resolved import graph"
    );
    assert_eq!(
        count_occurrences(&root, "conservatively_affected_diagnostic_documents"),
        0,
        "diagnostic dependency selection must use the PublishedModel query"
    );
    assert_eq!(
        count_occurrences(&root, "collect_import_targets_from_root"),
        0,
        "LSP code must not reconstruct semantic dependencies by walking parser imports"
    );
}

fn count_pattern(root: &Path, pattern: &str) -> usize {
    let mut count = 0usize;
    visit_rs_files(root, &mut |path| {
        if let Ok(contents) = fs::read_to_string(path) {
            count += count_attribute_lines(&contents, pattern);
        }
    });
    count
}

fn count_occurrences(root: &Path, pattern: &str) -> usize {
    let mut count = 0;
    visit_rs_files(root, &mut |path| {
        if let Ok(contents) = fs::read_to_string(path) {
            count += contents.matches(pattern).count();
        }
    });
    count
}

fn count_attribute_lines(contents: &str, pattern: &str) -> usize {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with(pattern))
        .count()
}

fn visit_rs_files(root: &Path, on_file: &mut dyn FnMut(&Path)) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit_rs_files(&path, on_file);
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            on_file(&path);
        }
    }
}

/// Production occurrences only: text before each file's `#[cfg(test)] mod` block.
fn count_production_occurrences(root: &Path, pattern: &str) -> usize {
    let mut total = 0;
    visit_rs_files(root, &mut |path| {
        let source = std::fs::read_to_string(path).expect("read source");
        let production = source.split("#[cfg(test)]\nmod ").next().unwrap_or(&source);
        total += production.matches(pattern).count();
    });
    total
}

/// One `Services` per host process: the editor host constructs its services exactly once (the
/// `ServerState` default used when no embedding engine supplies them) and threads clones.
#[test]
fn the_editor_host_constructs_one_services_value() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    assert_eq!(
        count_production_occurrences(&root, "Services::new("),
        1,
        "construct Services once and hand clones of its handles around"
    );
}

/// Library-closure resolution is a startup and reconfiguration cost, never a per-edit one.
#[test]
fn library_closure_never_runs_on_the_edit_path() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for file in [
        "lsp_runtime/documents/sync.rs",
        "lsp_runtime/documents/mod.rs",
        "session/handle.rs",
    ] {
        let source = std::fs::read_to_string(src.join(file)).expect("read source");
        for forbidden in [".library.resolve(", "load_library_closure_documents("] {
            assert!(
                !source.contains(forbidden),
                "{file} resolves the library closure on the edit path: {forbidden}"
            );
        }
    }
}

/// No two functions in the editor-host crates may share a body.
///
/// A byte-identical body in two places is a derivation with two owners: it drifts silently, and
/// every duplication C section 2 catalogues for these crates (`to_lsp_range`, `utf16_len`, the
/// per-module converter pairs) presented exactly this way. Bodies are compared after whitespace
/// normalisation and only above a size floor, so trivial forwarding bodies do not trip it.
#[test]
fn no_duplicate_free_function_bodies_across_consumer_crates() {
    use std::collections::BTreeMap;

    /// Known collisions still owned by a later migration step, keyed by their sorted site list.
    ///
    /// Each entry is debt with a named destination in C section 5; none may be extended, and an
    /// entry that stops colliding must be deleted rather than repointed.
    const ALLOWED: &[&[&str]] = &[
        // D10 — the SysML text probe leaves the host with the syntax-service queries.
        &[
            "language_service/src/code_actions.rs::parse_untyped_part_usage_name",
            "lsp_server/src/common/util.rs::parse_untyped_part_usage_line",
        ],
        // D8 — collapses with the round-trip `SymbolEntry` conversion.
        &[
            "language_service/src/symbol.rs::symbol_hover_markdown",
            "lsp_server/src/common/util.rs::symbol_hover_markdown",
        ],
        // D8 — the host's re-exported test of the owner's behaviour.
        &[
            "language_service/src/symbol.rs::find_reference_ranges_finds_multiple_occurrences",
            "lsp_server/src/language/mod.rs::test_find_reference_ranges_multiple",
        ],
        // D9 — one `SymbolKind` vocabulary, owned by `language_service`.
        &[
            "lsp_server/src/language/symbols.rs::outline_kind_to_lsp",
            "lsp_server/src/lsp_runtime/features/editing_features.rs::workspace_symbol_kind",
        ],
    ];
    const MIN_BODY_CHARS: usize = 80;

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest.parent().expect("crates directory").to_path_buf();
    let crates = [
        manifest.join("src"),
        manifest.join("../language_service/src"),
    ];

    let mut by_body: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for root in &crates {
        visit_rs_files(root, &mut |path| {
            let Ok(contents) = fs::read_to_string(path) else {
                return;
            };
            let relative = path
                .canonicalize()
                .ok()
                .and_then(|p| p.strip_prefix(&crates_dir).map(Path::to_path_buf).ok())
                .unwrap_or_else(|| path.to_path_buf());
            for (name, body) in function_bodies(&contents) {
                if body.len() < MIN_BODY_CHARS {
                    continue;
                }
                by_body
                    .entry(body)
                    .or_default()
                    .push(format!("{}::{name}", relative.display()));
            }
        });
    }

    let collisions: Vec<String> = by_body
        .into_iter()
        .filter(|(_, sites)| sites.len() > 1)
        .filter_map(|(body, mut sites)| {
            sites.sort();
            ALLOWED
                .iter()
                .all(|allowed| *allowed != sites.as_slice())
                .then(|| format!("{sites:?} share the body {body}"))
        })
        .collect();
    assert!(
        collisions.is_empty(),
        "identical function bodies must collapse onto one owner; found {}: {collisions:#?}",
        collisions.len()
    );
}

/// `(name, whitespace-normalised body)` for every `fn` declaration with a body in `contents`.
fn function_bodies(contents: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = contents;
    while let Some(at) = rest.find("fn ") {
        rest = &rest[at + 3..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let Some(open) = rest.find('{') else { break };
        let mut depth = 0usize;
        let mut end = None;
        for (index, ch) in rest[open..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(open + index + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { break };
        let body = rest[open..end]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !name.is_empty() {
            out.push((name, body));
        }
        rest = &rest[end..];
    }
    out
}
