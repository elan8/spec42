//! The authority chain has exactly one dependant per link, and consumers reach it only through
//! the facade.
//!
//! `parser_authority.rs` guards the first link (`sysml-v2-parser` → `sysml_resolution`). This
//! file guards the next two: `sysml_resolution` may be depended on only by `sysml_query`, and
//! `sysml_source` only by `sysml_resolution`. It lives in `source_identity` for the same reason:
//! this crate is below the whole chain, has no dev-dependencies, and cannot be the thing a rule
//! here constrains -- a guard that the guarded thing could disable is not a guard.
//!
//! `deny.toml` states the same rules natively and fails at `cargo deny check bans`. This file
//! covers what the resolved graph cannot express: manifest shape, the `fuzz/` nested workspace
//! whose lockfile the root graph never reaches, and the dependant set read from every lockfile
//! as Cargo actually resolved it.

use std::fs;
use std::path::{Path, PathBuf};

/// Each link: the crate, and the one manifest that may depend on it.
const LINKS: &[(&str, &str)] = &[
    ("sysml_resolution", "crates/sysml_query/Cargo.toml"),
    ("sysml_source", "crates/sysml_resolution/Cargo.toml"),
];

/// The vocabulary crate is not a link of the chain -- it has more than one dependant by design --
/// but the same consumer rule applies to it, so the set of manifests that may name it is closed
/// here. The two authorities implement the contract and the facade re-exports it; nothing else
/// may name it, because a consumer that did would pin the vocabulary's crate identity instead of
/// the facade's.
const CONTRACT_CRATE: &str = "sysml_contract";
const CONTRACT_DEPENDANTS: &[&str] = &[
    "crates/sysml_query/Cargo.toml",
    "crates/sysml_resolution/Cargo.toml",
    "crates/sysml_source/Cargo.toml",
];

/// The only SysML crate `sysml_contract` itself may name. Its position in the map -- above
/// `source_identity`, below `sysml_source` -- is exactly this list being what it is: a crate that
/// cannot name the source authority or the parser cannot compute a fact, whatever it contains.
const CONTRACT_MAY_DEPEND_ON: &[&str] = &["source_identity"];

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

/// Every lockfile Cargo resolves for this repository: the root workspace and the nested fuzz
/// workspace.
fn lockfiles(root: &Path) -> Vec<PathBuf> {
    ["Cargo.lock", "fuzz/Cargo.lock"]
        .iter()
        .map(|name| root.join(name))
        .filter(|path| path.exists())
        .collect()
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

/// Manifest lines that depend on `package`, either by key or by `package = "..."` rename, with
/// the `[package] name = ...` line of the crate itself excluded.
fn dependency_lines(manifest: &Path, package: &str) -> Vec<String> {
    let text = fs::read_to_string(manifest).unwrap_or_default();
    let mut section = String::new();
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            section = trimmed.to_string();
            continue;
        }
        if trimmed.starts_with('#') || section == "[package]" {
            continue;
        }
        let renamed = trimmed.contains(&format!("package = \"{package}\""));
        if dependency_key(trimmed) == package || renamed {
            out.push(line.to_string());
        }
    }
    out
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

/// The package names in a lockfile stanza's `dependencies` array, compared exactly.
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

/// The names of every package in `lock` that directly depends on `package`.
fn lock_dependents(lock: &str, package: &str) -> Vec<String> {
    let mut dependents: Vec<String> = lock
        .split("[[package]]")
        .skip(1)
        .filter(|stanza| lock_dependencies(stanza).iter().any(|d| d == package))
        .filter_map(|stanza| lock_field(stanza, "name"))
        .collect();
    dependents.sort();
    dependents.dedup();
    dependents
}

/// Rule 1: only the designated dependant may name a chain crate, in any manifest in the
/// repository, including the `fuzz/` nested workspace.
#[test]
fn only_the_next_link_may_name_each_authority_crate() {
    let root = repo_root();
    let mut offenders = Vec::new();
    for (package, allowed) in LINKS {
        for manifest in manifests(&root) {
            let relative = manifest
                .strip_prefix(&root)
                .unwrap_or(&manifest)
                .to_string_lossy()
                .replace('\\', "/");
            let lines = dependency_lines(&manifest, package);
            if lines.is_empty() || relative == *allowed {
                continue;
            }
            offenders.push(format!("{relative} names {package}: {lines:?}"));
        }
    }
    assert!(
        offenders.is_empty(),
        "only the next link of the authority chain may depend on an authority crate; every \
         consumer reaches syntax, sources and semantics through `sysml_query`. Offending \
         manifests:\n  {}",
        offenders.join("\n  ")
    );
}

/// Rule 2: in every graph Cargo resolved, each chain crate has exactly one dependant.
///
/// Manifests can only show what they spell; a workspace-level rename or a transitive path
/// through a crate that never names the package defeats a manifest scan. The lockfile records
/// resolved package names, so neither survives into it. The fuzz workspace resolves its own
/// graph, which reaches the chain through `sysml_query` and must show the same single dependant
/// per link.
#[test]
fn each_link_has_exactly_one_dependant_in_every_resolved_graph() {
    let root = repo_root();
    for lockfile in lockfiles(&root) {
        let lock = fs::read_to_string(&lockfile).expect("read lockfile");
        for (package, allowed_manifest) in LINKS {
            let allowed_crate = Path::new(allowed_manifest)
                .parent()
                .and_then(Path::file_name)
                .map(|name| name.to_string_lossy().to_string())
                .expect("allowed manifest lives in a crate directory");
            let dependents = lock_dependents(&lock, package);
            assert_eq!(
                dependents,
                vec![allowed_crate.clone()],
                "{}: `{package}` must have exactly one direct dependant, `{allowed_crate}`; found \
                 {dependents:?}",
                lockfile.display()
            );
        }
    }
}

/// Rule 3: only the two authorities and the facade may name the vocabulary crate.
///
/// `deny.toml` states the same ban with the same three wrappers. This adds what the resolved
/// graph cannot express: the manifest scan reaches the `fuzz/` nested workspace, and it sees a
/// rename (`contract = { package = "sysml_contract" }`) that would otherwise enter the graph
/// under a name no ban mentions.
#[test]
fn only_the_authorities_and_the_facade_may_name_the_contract_crate() {
    let root = repo_root();
    let mut offenders = Vec::new();
    for manifest in manifests(&root) {
        let relative = manifest
            .strip_prefix(&root)
            .unwrap_or(&manifest)
            .to_string_lossy()
            .replace('\\', "/");
        let lines = dependency_lines(&manifest, CONTRACT_CRATE);
        if lines.is_empty() || CONTRACT_DEPENDANTS.contains(&relative.as_str()) {
            continue;
        }
        offenders.push(format!("{relative} names {CONTRACT_CRATE}: {lines:?}"));
    }
    assert!(
        offenders.is_empty(),
        "the semantic vocabulary is re-exported verbatim by `sysml_query`; only the crates that \
         implement it and the facade that publishes it may name it. Offending manifests:\n  {}",
        offenders.join("\n  ")
    );
}

/// Rule 4: the vocabulary crate sits above `source_identity` and below `sysml_source`.
///
/// Position in the dependency map is not decoration: `sysml_contract` may hold no state and
/// derive no fact, and the enforceable form of that is the set of crates it can reach. It names
/// `source_identity` so digests can appear in contract types, and nothing else -- so it cannot
/// reach a parser, a document, or a resolution graph, and a type that needed one could not be
/// moved into it by accident.
#[test]
fn the_contract_crate_sits_above_source_identity_and_below_the_source_authority() {
    let root = repo_root();
    let manifest = root.join("crates/sysml_contract/Cargo.toml");
    let text = fs::read_to_string(&manifest).expect("read sysml_contract manifest");

    let mut named: Vec<String> = Vec::new();
    let mut section = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            section = trimmed.trim_matches(['[', ']']).to_string();
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') || !section.ends_with("dependencies") {
            continue;
        }
        let key = dependency_key(trimmed);
        if key.starts_with("sysml") || key == "source_identity" || key == "kpar" {
            named.push(key.to_string());
        }
    }
    named.sort();
    named.dedup();
    assert_eq!(
        named,
        CONTRACT_MAY_DEPEND_ON
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<_>>(),
        "`sysml_contract` must name `source_identity` and no other SysML crate"
    );

    for (dependant, must_name) in [
        ("crates/sysml_source/Cargo.toml", CONTRACT_CRATE),
        ("crates/sysml_resolution/Cargo.toml", CONTRACT_CRATE),
    ] {
        assert!(
            !dependency_lines(&root.join(dependant), must_name).is_empty(),
            "{dependant} must depend on `{must_name}`: the authority implements the contract"
        );
    }
    assert!(
        dependency_lines(&manifest, "sysml_source").is_empty(),
        "`sysml_contract` is below the source authority and must not name it"
    );
}

/// Rule 5: the parser guard's lockfile rule also holds in the fuzz workspace.
///
/// `parser_authority.rs` reads the root lockfile. The fuzz workspace resolves its own graph and
/// must not reach the parser directly either.
#[test]
fn the_fuzz_workspace_does_not_reach_the_parser_directly() {
    let root = repo_root();
    let fuzz_lock = root.join("fuzz/Cargo.lock");
    if !fuzz_lock.exists() {
        return;
    }
    let lock = fs::read_to_string(&fuzz_lock).expect("read fuzz lockfile");
    let dependents = lock_dependents(&lock, "sysml-v2-parser");
    assert_eq!(
        dependents,
        vec!["sysml_resolution".to_string()],
        "in the fuzz workspace only sysml_resolution may depend on the parser: {dependents:?}"
    );
}

#[test]
fn the_manifest_scanner_sees_inline_renamed_and_inherited_forms_and_ignores_the_package_name() {
    let dir = std::env::temp_dir().join(format!("spec42-authority-chain-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let manifest = dir.join("Cargo.toml");
    fs::write(
        &manifest,
        "[package]\nname = \"sysml_resolution\"\n\n[dependencies]\n\
         sysml_resolution = { path = \"..\" }\n\
         res = { package = \"sysml_resolution\", path = \"..\" }\n\
         sysml_resolution.workspace = true\n\
         # sysml_resolution = { path = \"..\" }\n\
         sysml_resolution_extras = \"1\"\n",
    )
    .unwrap();
    let lines = dependency_lines(&manifest, "sysml_resolution");
    fs::remove_dir_all(&dir).ok();
    assert_eq!(lines.len(), 3, "{lines:?}");
}

#[test]
fn the_lockfile_scanner_matches_package_names_exactly() {
    let lock = "\
[[package]]
name = \"a\"
dependencies = [
 \"sysml_resolution\",
]

[[package]]
name = \"b\"
dependencies = [
 \"sysml_resolution 0.50.0 (path+file:///x)\",
]

[[package]]
name = \"c\"
dependencies = [
 \"sysml_resolution_extras\",
]
";
    assert_eq!(lock_dependents(lock, "sysml_resolution"), vec!["a", "b"]);
}
