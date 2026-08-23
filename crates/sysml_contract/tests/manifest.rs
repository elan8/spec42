//! The contract crate stays a leaf: one SysML dependency and nothing that computes.
//!
//! The crate's whole value is that it cannot derive a fact. That is a property of its dependency
//! set, not of its current contents, so it is asserted against the manifest: `source_identity` is
//! the only SysML crate it may name, and the parser, the source authority, an async runtime, or a
//! serialisation framework may not appear at all outside an optional feature.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")
}

/// The dependency keys declared in every dependency table of the manifest.
fn dependency_keys(text: &str) -> BTreeSet<String> {
    let mut section = String::new();
    let mut keys = BTreeSet::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            section = trimmed.trim_matches(['[', ']']).to_string();
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !section.ends_with("dependencies") {
            continue;
        }
        let key = trimmed
            .split(['=', '.'])
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('"');
        if !key.is_empty() {
            keys.insert(key.to_string());
        }
    }
    keys
}

#[test]
fn the_contract_crate_names_exactly_one_sysml_crate() {
    let text = fs::read_to_string(manifest_path()).expect("read sysml_contract manifest");
    let sysml: BTreeSet<String> = dependency_keys(&text)
        .into_iter()
        .filter(|key| key.starts_with("sysml") || key == "source_identity" || key == "kpar")
        .collect();
    assert_eq!(
        sysml,
        BTreeSet::from(["source_identity".to_string()]),
        "the contract crate may depend on `source_identity` and no other SysML crate"
    );
}

#[test]
fn the_contract_crate_cannot_parse_serialise_or_await() {
    let text = fs::read_to_string(manifest_path()).expect("read sysml_contract manifest");
    assert!(
        !text.contains("[features]"),
        "a feature on the contract crate can admit a dependency; add one only deliberately"
    );
    let forbidden = [
        "sysml-v2-parser",
        "sysml_source",
        "sysml_resolution",
        "tokio",
        "serde",
        "url",
        "ignore",
    ];
    let keys = dependency_keys(&text);
    let offenders: Vec<&str> = forbidden
        .into_iter()
        .filter(|name| keys.contains(*name) || text.contains(&format!("package = \"{name}\"")))
        .collect();
    assert!(
        offenders.is_empty(),
        "the contract crate computes nothing and performs no I/O: {offenders:?}"
    );
}
