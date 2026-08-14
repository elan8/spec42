//! Runs the corpus as part of `cargo test --workspace`.
//!
//! Skips with a clear message when the plugins have not been built, matching how the other
//! fixture-dependent tests in this repo behave, so a fresh checkout without a wasm toolchain
//! does not fail.

use generator_conformance::{default_corpus_root, run_corpus};

#[test]
fn the_generator_conformance_corpus_matches_its_goldens() {
    let root = default_corpus_root();
    let plugins = root.join("plugins/target/wasm32-unknown-unknown/release");
    if !plugins.is_dir() {
        eprintln!("skipping: generator plugins not built. Run scripts/build-generator-plugins.sh");
        return;
    }

    let results = run_corpus(&root, None, false).expect("corpus should run");
    assert!(!results.is_empty(), "the corpus should not be empty");

    let failures: Vec<String> = results
        .iter()
        .filter(|result| !result.passed())
        .map(|result| format!("{}:\n  {}", result.id, result.failures.join("\n  ")))
        .collect();
    assert!(
        failures.is_empty(),
        "{} case(s) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
