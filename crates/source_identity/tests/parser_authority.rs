//! One crate may name the parser, and it is the one that lowers the AST to the semantic graph.
//!
//! This lives in `source_identity` because that crate has no parser dependency and never will --
//! a guard that the guarded thing could disable is not a guard. It replaces
//! `crates/sysml_parser/tests/boundary.rs`, which died with the facade and whose rule was a
//! *spelling* check: it rejected `git =`, `version =`, and `package = "sysml-v2-parser"`, so
//! `fuzz/Cargo.toml`'s `package = "spec42-sysml-parser", path = ...` slipped straight through it.
//! The rules below are stated positively instead, which is what closes that class of hole.

use std::fs;
use std::path::{Path, PathBuf};

/// The single crate permitted to depend on the parser.
const AUTHORITY_MANIFEST: &str = "crates/sysml_resolution/Cargo.toml";

/// The upstream the pin must come from.
const PARSER_GIT_URL: &str = "https://github.com/lukewilliamboswell/sysml-v2-parser.git";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name> is two levels below the repository root")
        .to_path_buf()
}

/// Every `Cargo.toml` in the repository, excluding build output and tool scratch space.
fn manifests(root: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if path.is_dir() {
                if matches!(
                    name.as_ref(),
                    "target" | ".git" | ".claude" | "node_modules"
                ) {
                    continue;
                }
                walk(&path, out);
            } else if name == "Cargo.toml" {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, &mut out);
    out.sort();
    out
}

fn parser_lines(manifest: &Path) -> Vec<String> {
    fs::read_to_string(manifest)
        .unwrap_or_default()
        .lines()
        .filter(|line| {
            let line = line.trim();
            !line.starts_with('#') && line.contains("sysml-v2-parser")
        })
        .map(str::to_string)
        .collect()
}

/// Rule 1: the root workspace owns the pin, as a git revision and nothing else.
#[test]
fn the_root_workspace_owns_the_parser_pin() {
    let root = repo_root();
    let lines = parser_lines(&root.join("Cargo.toml"));
    assert_eq!(
        lines.len(),
        1,
        "the root workspace must declare exactly one parser dependency, found: {lines:?}"
    );
    let line = &lines[0];
    assert!(
        line.contains(PARSER_GIT_URL),
        "the parser pin must name the upstream git repository, got: {line}"
    );
    let rev = line
        .split("rev = \"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap_or_default();
    assert_eq!(
        rev.len(),
        40,
        "the parser pin must be a full 40-character revision, got: {rev:?}"
    );
    assert!(
        rev.chars().all(|c| c.is_ascii_hexdigit()),
        "the parser revision must be hexadecimal, got: {rev:?}"
    );
    for forbidden in ["version = ", "branch = ", "tag = ", "path = "] {
        assert!(
            !line.contains(forbidden),
            "the parser pin must be a bare git revision; found `{forbidden}` in: {line}"
        );
    }
}

/// Rule 2: only the lowering authority may name the parser, and only by inheritance.
///
/// This is the rule the goal is stated in. Every other crate reaches syntax through
/// `sysml_resolution::syntax`, which returns plain data -- so a crate without this dependency
/// cannot hold, cache, serialize, or walk a `ParsedDocument`, and that is a compile error rather
/// than a review comment.
#[test]
fn only_the_lowering_authority_may_name_the_parser() {
    let root = repo_root();
    let mut offenders = Vec::new();
    for manifest in manifests(&root) {
        let relative = manifest
            .strip_prefix(&root)
            .unwrap_or(&manifest)
            .to_string_lossy()
            .replace('\\', "/");
        if relative == "Cargo.toml" {
            continue;
        }
        let lines = parser_lines(&manifest);
        if lines.is_empty() {
            continue;
        }
        if relative != AUTHORITY_MANIFEST {
            offenders.push(format!("{relative}: {lines:?}"));
            continue;
        }
        for line in &lines {
            let inherits = line.contains("workspace = true") || line.contains(".workspace = true");
            assert!(
                inherits,
                "{relative} must inherit the pin with `workspace = true`, got: {line}"
            );
        }
    }
    assert!(
        offenders.is_empty(),
        "only `{AUTHORITY_MANIFEST}` may depend on the parser; it is the crate that lowers the \
         AST to the semantic graph. Reach syntax through `sysml_resolution::syntax` instead. \
         Offending manifests:\n  {}",
        offenders.join("\n  ")
    );
}

/// Rule 3: no manifest may reintroduce a repository-local parser facade.
///
/// The deleted `crates/sysml_parser` was exactly this: a `path =` dependency whose package name
/// was a parser alias. Naming the shape directly means bringing it back fails a test rather than
/// depending on a reviewer recognising it.
#[test]
fn no_manifest_reintroduces_a_local_parser_facade() {
    let root = repo_root();
    let mut offenders = Vec::new();
    for manifest in manifests(&root) {
        for line in parser_lines(&manifest) {
            let aliases_parser = line.contains("spec42-sysml-parser")
                || line.contains("package = \"sysml-v2-parser\"");
            if aliases_parser && line.contains("path = ") {
                offenders.push(format!("{}: {line}", manifest.display()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "a repository-local parser facade is not an acceptable boundary; depend on the pinned \
         revision from the root workspace. Offending manifests:\n  {}",
        offenders.join("\n  ")
    );
}

/// Rule 4: the lockfile resolves exactly one parser, from git.
///
/// Cheap, textual, and it catches what a manifest scan cannot: a stray transitive dependency
/// pulling a second copy in. Both identities carried version `0.54.0` before the facade was
/// removed, so a version comparison would have proved nothing -- only the source distinguishes
/// them.
#[test]
fn the_lockfile_resolves_one_parser_from_git() {
    let lock = fs::read_to_string(repo_root().join("Cargo.lock")).expect("read Cargo.lock");
    let stanzas: Vec<&str> = lock
        .split("[[package]]")
        .filter(|stanza| stanza.contains("name = \"sysml-v2-parser\""))
        .collect();
    assert_eq!(
        stanzas.len(),
        1,
        "expected exactly one resolved `sysml-v2-parser`, found {}",
        stanzas.len()
    );
    let stanza = stanzas[0];
    assert!(
        stanza.contains("source = \"git+"),
        "the resolved parser must come from git, got:\n{stanza}"
    );
    assert!(
        !stanza.contains("checksum = "),
        "a registry checksum means a crates.io copy was resolved:\n{stanza}"
    );
}
