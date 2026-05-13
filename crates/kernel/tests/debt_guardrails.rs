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
    let forbidden = [
        root.join("graph_builder"),
        root.join("evaluation/units.rs"),
    ];

    let existing = forbidden
        .iter()
        .filter(|path| path.exists())
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();

    assert!(
        existing.is_empty(),
        "reusable semantic implementations belong in semantic_core, not kernel:\n{}",
        existing.join("\n")
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
    let count = count_pattern(&test_root, "it.skip(")
        + count_pattern(&test_root, "describe.skip(");

    assert_eq!(
        count, MAX_FRONTEND_SKIPPED_TESTS,
        "frontend skipped test count regressed: expected {MAX_FRONTEND_SKIPPED_TESTS}, got {count}"
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
