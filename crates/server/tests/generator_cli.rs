//! Subprocess-level tests for `spec42 generate`.
//!
//! The in-process conformance corpus in `crates/generator_conformance` covers the ABI and the
//! semantic API, but structurally cannot see anything above `GeneratorRuntime`: exit codes,
//! the output transaction, the manifest, `--check`/`--dry-run`/`--force`, symlink refusal, or
//! behaviour that differs between processes. That gap let an output-root escape and a
//! reserved-name alias ship, so these run the real binary.
//!
//! Skips with a message when the plugin corpus has not been built, matching the other
//! fixture-dependent tests here.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../generator-tests")
}

fn plugin(name: &str) -> Option<PathBuf> {
    let path = corpus_root()
        .join("plugins/target/wasm32-unknown-unknown/release")
        .join(format!("spec42_conformance_{name}.wasm"));
    path.is_file().then_some(path)
}

fn model(name: &str) -> PathBuf {
    corpus_root().join("models").join(name).join("model.sysml")
}

/// Runs `spec42 generate`, returning the raw output so the caller can assert on the code.
fn generate(plugin: &Path, model: &Path, output: &Path, extra: &[&str], guest: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_spec42"));
    command
        .arg("--no-stdlib")
        .arg("generate")
        .arg(plugin)
        .arg(model)
        .arg("--output")
        .arg(output);
    command.args(extra);
    if !guest.is_empty() {
        command.arg("--");
        command.args(guest);
    }
    command.output().expect("spec42 should run")
}

fn code(output: &Output) -> i32 {
    output.status.code().expect("spec42 should exit normally")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Wraps a test body so it is skipped rather than failed when the corpus is absent.
macro_rules! with_plugin {
    ($name:ident, |$plugin:ident| $body:block) => {
        let Some($plugin) = plugin(stringify!($name)) else {
            eprintln!(
                "skipping: generator plugins not built. Run scripts/build-generator-plugins.sh"
            );
            return;
        };
        $body
    };
}

#[test]
fn a_successful_run_exits_zero_and_writes_a_manifest() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        let run = generate(&plugin, &model("minimal"), &output, &[], &[]);
        assert_eq!(code(&run), 0, "{}", stderr(&run));
        assert!(output.join(".spec42-generator-manifest.json").is_file());
    });
}

#[test]
fn check_reports_no_drift_immediately_after_generating() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        assert_eq!(
            code(&generate(&plugin, &model("minimal"), &output, &[], &[])),
            0
        );

        let checked = generate(&plugin, &model("minimal"), &output, &["--check"], &[]);
        assert_eq!(
            code(&checked),
            0,
            "immediate --check drifted: {}",
            stderr(&checked)
        );
    });
}

#[test]
fn check_exits_fifteen_when_output_would_change() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        assert_eq!(
            code(&generate(&plugin, &model("minimal"), &output, &[], &[])),
            0
        );
        std::fs::write(output.join("transcript.txt"), b"locally edited").unwrap();

        let checked = generate(&plugin, &model("minimal"), &output, &["--check"], &[]);
        assert_eq!(code(&checked), 15, "{}", stderr(&checked));
    });
}

#[test]
fn dry_run_writes_nothing() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        let run = generate(&plugin, &model("minimal"), &output, &["--dry-run"], &[]);
        assert_eq!(code(&run), 0, "{}", stderr(&run));
        assert!(!output.exists(), "--dry-run created the output directory");
    });
}

#[test]
fn an_unowned_file_conflicts_until_force_is_given() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        std::fs::create_dir_all(&output).unwrap();
        std::fs::write(output.join("transcript.txt"), b"not ours").unwrap();

        let refused = generate(&plugin, &model("minimal"), &output, &[], &[]);
        assert_eq!(code(&refused), 14, "{}", stderr(&refused));
        assert_eq!(
            std::fs::read(output.join("transcript.txt")).unwrap(),
            b"not ours"
        );

        let forced = generate(&plugin, &model("minimal"), &output, &["--force"], &[]);
        assert_eq!(code(&forced), 0, "{}", stderr(&forced));
        assert_ne!(
            std::fs::read(output.join("transcript.txt")).unwrap(),
            b"not ours"
        );
    });
}

#[test]
fn a_generator_error_exits_twelve_and_writes_nothing() {
    with_plugin!(error_guest, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        let run = generate(&plugin, &model("minimal"), &output, &[], &[]);
        assert_eq!(code(&run), 12, "{}", stderr(&run));
        assert!(!output.exists(), "a failed run created output");
    });
}

#[test]
fn a_malformed_module_exits_eleven() {
    let temp = tempfile::tempdir().unwrap();
    let bogus = temp.path().join("not-a-module.wasm");
    std::fs::write(&bogus, b"this is not WebAssembly").unwrap();
    let run = generate(
        &bogus,
        &model("minimal"),
        &temp.path().join("generated"),
        &[],
        &[],
    );
    assert_eq!(code(&run), 11, "{}", stderr(&run));
}

#[test]
fn an_invalid_model_exits_ten() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let broken = temp.path().join("broken.sysml");
        // A parse error, not an unresolved reference: unresolved references are warnings and
        // generation is documented to proceed through them.
        std::fs::write(&broken, "package Broken { part def Widget {\n").unwrap();
        let run = generate(&plugin, &broken, &temp.path().join("generated"), &[], &[]);
        assert_eq!(code(&run), 10, "{}", stderr(&run));
    });
}

/// The escape that shipped: `scratch/..` could not be canonicalized while `scratch` was
/// absent, so validation passed and commit then resolved the root to the workspace.
#[test]
fn an_output_root_that_escapes_once_created_is_refused() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let workspace = temp.path().join("workspace");
        std::fs::create_dir_all(&workspace).unwrap();
        std::fs::write(workspace.join("precious.txt"), b"must survive").unwrap();

        let mut command = Command::new(env!("CARGO_BIN_EXE_spec42"));
        let run = command
            .current_dir(&workspace)
            .arg("--no-stdlib")
            .arg("generate")
            .arg(&plugin)
            .arg(model("minimal"))
            .arg("--output")
            .arg("scratch/..")
            .output()
            .expect("spec42 should run");

        assert_eq!(code(&run), 14, "{}", stderr(&run));
        assert!(
            !workspace.join("scratch").exists(),
            "the escaping root was created before validation rejected it"
        );
        assert_eq!(
            std::fs::read(workspace.join("precious.txt")).unwrap(),
            b"must survive"
        );
    });
}

/// The alias that shipped: a case variant of the manifest name was accepted, and on a
/// case-folding filesystem the manifest then overwrote the artifact.
#[test]
fn a_case_variant_of_the_manifest_name_is_refused() {
    with_plugin!(artifact_paths, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        let run = generate(
            &plugin,
            &model("minimal"),
            &output,
            &[],
            &["path=.SPEC42-GENERATOR-MANIFEST.JSON"],
        );
        assert_eq!(code(&run), 14, "{}", stderr(&run));
        assert!(!output.exists());
    });
}

#[cfg(unix)]
#[test]
fn a_symlink_in_the_existing_output_tree_is_refused() {
    with_plugin!(query_all, |plugin| {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("generated");
        std::fs::create_dir_all(output.join("docs")).unwrap();
        symlink("/etc/hosts", output.join("docs/link.txt")).unwrap();

        let run = generate(&plugin, &model("minimal"), &output, &[], &[]);
        assert_eq!(code(&run), 14, "{}", stderr(&run));
    });
}

/// `std`'s `HashMap` seed differs per process, so determinism has to be checked across two
/// processes rather than two calls. This repo has already shipped a bug of exactly that
/// shape; see the interconnection-layout entry in CHANGELOG.md.
#[test]
fn output_is_identical_across_separate_processes() {
    with_plugin!(query_all, |plugin| {
        let temp = tempfile::tempdir().unwrap();
        let first = temp.path().join("first");
        let second = temp.path().join("second");
        assert_eq!(
            code(&generate(&plugin, &model("coverage"), &first, &[], &[])),
            0
        );
        assert_eq!(
            code(&generate(&plugin, &model("coverage"), &second, &[], &[])),
            0
        );

        let left = std::fs::read(first.join("transcript.txt")).unwrap();
        let right = std::fs::read(second.join("transcript.txt")).unwrap();
        assert_eq!(left, right, "two processes produced different bytes");
    });
}

#[test]
fn a_stale_generator_is_refused_with_an_abi_message() {
    // A module that exports the right shape but reports the wrong compatibility token.
    let source = r#"(module
  (memory (export "memory") 1)
  (func (export "spec42_abi_version") (result i64) (i64.const 1))
  (func (export "spec42_alloc") (param i32) (result i32) (i32.const 0))
  (func (export "spec42_generate") (param i32 i32) (result i64) (i64.const 0)))"#;
    let temp = tempfile::tempdir().unwrap();
    let stale = temp.path().join("stale.wasm");
    std::fs::write(&stale, wat::parse_str(source).unwrap()).unwrap();

    let run = generate(
        &stale,
        &model("minimal"),
        &temp.path().join("generated"),
        &[],
        &[],
    );
    assert_eq!(code(&run), 11, "{}", stderr(&run));
    assert!(
        stderr(&run).contains("incompatible Spec42 generator ABI"),
        "unhelpful message: {}",
        stderr(&run)
    );
}
