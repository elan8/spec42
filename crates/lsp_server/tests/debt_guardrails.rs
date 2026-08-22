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
    let state = fs::read_to_string(root.join("workspace/state.rs")).expect("workspace state");
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
