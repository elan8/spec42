use std::fs;
use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("parser facade is under crates/")
        .to_path_buf()
}

#[test]
fn production_manifests_cannot_select_an_upstream_parser() {
    let root = repository_root();
    let facade = root.join("crates/sysml_parser/Cargo.toml");
    let mut offenders = Vec::new();

    for manifest in manifests_below(&root) {
        if manifest == facade {
            continue;
        }
        let source = fs::read_to_string(&manifest).expect("read manifest");
        for (line_number, line) in source.lines().enumerate() {
            if line.contains("sysml-v2-parser-next")
                || (line.contains("sysml-v2-parser")
                    && (line.contains("git =")
                        || line.contains("version =")
                        || line.contains("package = \"sysml-v2-parser\"")))
            {
                offenders.push(format!(
                    "{}:{}: {}",
                    manifest.strip_prefix(&root).unwrap_or(&manifest).display(),
                    line_number + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "only crates/sysml_parser may select parser packages or sources:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn compatibility_and_canonical_parsers_agree_on_acceptance_and_recovery() {
    let valid = "package P { part def A; part a : A; }";
    assert!(sysml_v2_parser::parse(valid).is_ok());
    assert!(sysml_v2_parser::next::parse(valid).is_ok());

    let malformed = "package P { part def A part a : A; }";
    let compatibility = sysml_v2_parser::parse_for_editor(malformed);
    let canonical = sysml_v2_parser::next::parse_for_editor(malformed);
    assert!(
        !compatibility.errors.is_empty(),
        "the compatibility syntax surface must report recovery"
    );
    assert!(
        !canonical.errors.is_empty(),
        "semantic construction must report recovery for the same source"
    );
}

fn manifests_below(directory: &Path) -> Vec<PathBuf> {
    let mut manifests = Vec::new();
    let Ok(entries) = fs::read_dir(directory) else {
        return manifests;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|name| name.to_str());
        if matches!(name, Some("target" | ".git" | ".claude")) {
            continue;
        }
        if path.is_dir() {
            manifests.extend(manifests_below(&path));
        } else if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml") {
            manifests.push(path);
        }
    }
    manifests
}
