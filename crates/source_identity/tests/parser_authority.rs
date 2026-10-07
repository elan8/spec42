//! One crate may name the parser, and it is the one that lowers the AST to the semantic graph.
//!
//! This lives in `source_identity` because that crate has no parser dependency and never will --
//! a guard that the guarded thing could disable is not a guard. It replaces
//! `crates/sysml_parser/tests/boundary.rs`, which died with the facade and whose rule was a
//! *spelling* check: it rejected `git =`, `version =`, and `package = "sysml-v2-parser"`, so
//! `fuzz/Cargo.toml`'s `package = "spec42-sysml-parser", path = ...` slipped straight through it.
//! The rules below are stated positively instead, which is what closes that class of hole.
//!
//! `deny.toml` states the same authority rule natively -- `cargo deny check bans` bans the parser
//! except as a direct dependency of `sysml_resolution`, and fails outside the test suite. This file
//! is not redundant with it and covers what a dependency graph cannot express: manifest *shape*
//! (rule 1), the `fuzz/` nested workspace that the root graph does not reach (rules 2 and 3), and
//! reintroducing a local facade under a different package name, which a ban on the upstream name
//! cannot see (rule 3). Rule 5 overlaps `cargo deny` deliberately, so the rule still holds if the
//! CI step is ever dropped.

use std::fs;
use std::path::{Path, PathBuf};

/// The single crate permitted to depend on the parser.
const AUTHORITY_MANIFEST: &str = "crates/sysml_resolution/Cargo.toml";

/// That crate's package name, as `Cargo.lock` spells it.
const AUTHORITY_CRATE: &str = "sysml_resolution";

/// The upstream package name, as `Cargo.lock` spells it -- renames do not survive into the lockfile.
const PARSER_PACKAGE: &str = "sysml-v2-parser";

/// The upstream the pin must come from.
const PARSER_GIT_URL: &str = "https://github.com/elan8/sysml-v2-parser.git";

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
            if entry.file_type().map_or(true, |kind| kind.is_symlink()) {
                // A symlinked directory is scratch or tooling state, never a repository source,
                // and following one can loop.
                continue;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if path.is_dir() {
                if matches!(
                    name.as_ref(),
                    "target" | ".git" | ".claude" | ".cache" | "node_modules"
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

/// The dependency key of a manifest line, e.g. `parser` in `parser.workspace = true`.
fn dependency_key(line: &str) -> &str {
    line.trim()
        .trim_start_matches('[')
        .split(['=', '.'])
        .next()
        .unwrap_or_default()
        .trim()
        .trim_matches('"')
}

/// Dependency keys that resolve to the parser, including a workspace-level rename.
///
/// `parser = { package = "sysml-v2-parser", git = ... }` in `[workspace.dependencies]` lets any
/// member write `parser.workspace = true` -- a line containing no parser spelling at all, which a
/// text scan for the package name cannot see. Deriving the key set from the root is what lets
/// rule 2 see that line. Rule 5 backstops this by reading what Cargo actually resolved.
fn parser_alias_keys(root: &Path) -> Vec<String> {
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap_or_default();
    let mut keys = vec![PARSER_PACKAGE.to_string()];
    let mut in_workspace_dependencies = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_workspace_dependencies = trimmed == "[workspace.dependencies]";
            continue;
        }
        if !in_workspace_dependencies
            || trimmed.starts_with('#')
            || !trimmed.contains(PARSER_PACKAGE)
        {
            continue;
        }
        let key = dependency_key(trimmed);
        if !key.is_empty() && !keys.iter().any(|known| known == key) {
            keys.push(key.to_string());
        }
    }
    keys
}

fn parser_lines(manifest: &Path) -> Vec<String> {
    parser_lines_with_aliases(manifest, &[PARSER_PACKAGE.to_string()])
}

fn parser_lines_with_aliases(manifest: &Path, aliases: &[String]) -> Vec<String> {
    fs::read_to_string(manifest)
        .unwrap_or_default()
        .lines()
        .filter(|line| {
            let line = line.trim();
            if line.starts_with('#') {
                return false;
            }
            // A profile override (`[profile.<name>.package.<crate>]`) tunes how a crate is
            // compiled; it is not a dependency and names no source of the parser.
            if line.starts_with("[profile.") {
                return false;
            }
            line.contains(PARSER_PACKAGE)
                || aliases.iter().any(|alias| alias == dependency_key(line))
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
    let aliases = parser_alias_keys(&root);
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
        let lines = parser_lines_with_aliases(&manifest, &aliases);
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

/// The value of a `key = "value"` field in a lockfile stanza.
fn lock_field(stanza: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = \"");
    stanza
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix(&prefix))
        .and_then(|rest| rest.split('"').next())
        .map(str::to_string)
}

/// The package names in a lockfile stanza's `dependencies` array.
///
/// An entry is either `"name"` or `"name version (source)"` when Cargo needs to disambiguate, so
/// the package name is the first whitespace-delimited token. Comparing that token *exactly* is
/// what stops a hypothetical `sysml-v2-parser-testing` from matching a substring search.
fn lock_dependencies(stanza: &str) -> Vec<String> {
    let Some((_, rest)) = stanza.split_once("dependencies = [") else {
        return Vec::new();
    };
    let Some((body, _)) = rest.split_once(']') else {
        return Vec::new();
    };
    body.lines()
        .map(|line| line.trim().trim_end_matches(',').trim_matches('"'))
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| entry.split_whitespace().next())
        .map(str::to_string)
        .collect()
}

/// Rule 5: in the graph Cargo actually resolved, only the authority depends on the parser.
///
/// Rules 1-4 read manifests, so they can only ever see what a manifest *spells*. Two things defeat
/// that and this rule catches both:
///
/// * a workspace-level rename -- `parser = { package = "sysml-v2-parser", git = ... }` in the root
///   lets a member write `parser.workspace = true`, which contains no parser spelling at all;
/// * a transitive path -- some crate gaining the parser through a dependency that never names it.
///
/// `Cargo.lock` records resolved *package* names, so neither survives into it. This is the rule
/// that makes the boundary a property of the dependency graph rather than of manifest prose, and
/// it is why the previous guard's whole failure mode -- being a spelling check -- cannot recur.
#[test]
fn only_the_authority_depends_on_the_parser_in_the_resolved_graph() {
    let lock = fs::read_to_string(repo_root().join("Cargo.lock")).expect("read Cargo.lock");
    let mut dependents: Vec<String> = lock
        .split("[[package]]")
        .skip(1)
        .filter(|stanza| {
            lock_dependencies(stanza)
                .iter()
                .any(|dependency| dependency == PARSER_PACKAGE)
        })
        .filter_map(|stanza| lock_field(stanza, "name"))
        .collect();
    dependents.sort();
    dependents.dedup();
    assert_eq!(
        dependents,
        vec![AUTHORITY_CRATE.to_string()],
        "only `{AUTHORITY_CRATE}` may depend on `{PARSER_PACKAGE}` in the resolved dependency \
         graph; it is the crate that lowers the AST to the semantic graph. Reach syntax through \
         `sysml_resolution::syntax`, which hands out plain data. Resolved dependents: {dependents:?}"
    );
}
