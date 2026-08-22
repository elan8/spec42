use std::fs;
use std::path::Path;

/// This crate carries diagnostic values and reporting policy, and decides nothing semantic.
///
/// Its manifest is the guard: reaching the mutable semantic model, the parser, or a transport
/// would each let a rule, a re-parse, or a protocol assumption back in. What it may depend on is
/// the typed facade over the immutable publication, and nothing else with an opinion about
/// meaning.
#[test]
fn the_reporting_crate_reads_the_published_facade_and_nothing_semantic() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");

    let forbidden = [
        "sysml_model",
        "sysml-v2-parser",
        "sysml_resolution",
        "tokio",
        "tower-lsp",
        "tower_lsp",
        "lsp_server",
        "workspace",
        "clap",
        "rmcp",
        "axum",
    ];
    for dep in forbidden {
        // Matched at the start of a line: `version.workspace = true` is an inherited field, not a
        // dependency on the crate called `workspace`.
        assert!(
            !cargo_toml
                .lines()
                .any(|line| line.starts_with(&format!("{dep} ="))),
            "sysml_diagnostics must not depend on {dep}: it renders published diagnostics rather \
             than deciding them"
        );
    }

    assert!(
        cargo_toml.contains("sysml_query ="),
        "sysml_diagnostics reads the published diagnostics through the typed facade"
    );
}

/// The reporting policy filters settled values; it never re-decides one.
///
/// A rule is a semantic decision wherever it lives, so the source may not name the model, resolve
/// anything, or read a diagnostic's own message. The message is presentation the owner produced;
/// recovering a fact from it is the inference this crate exists to make unnecessary.
#[test]
fn the_reporting_crate_decides_nothing_semantic() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let forbidden_patterns = [
        "SemanticGraph",
        "SemanticNode",
        "sysml_model",
        "collect_diagnostics_from_graph",
        "evaluation_facts_for",
        "expression_evaluation_for",
        "resolve_",
        "diagnostic.message.contains",
        "sysml_v2_parser",
    ];

    let mut offenders = Vec::new();
    for file in list_rs_files(&manifest_dir.join("src")) {
        let content = fs::read_to_string(&file).expect("read source file");
        for pattern in forbidden_patterns {
            if content.contains(pattern) {
                offenders.push(format!("{}: contains `{pattern}`", file.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "sysml_diagnostics must not decide semantics:\n{}",
        offenders.join("\n")
    );
}

fn list_rs_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir).expect("read src dir");
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            files.extend(list_rs_files(&path));
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}
