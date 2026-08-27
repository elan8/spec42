use std::fs;
use std::path::Path;

/// The batch host reaches SysML only through the facade and owns no library provisioning, no
/// storage, no async runtime and no protocol.
#[test]
fn workspace_is_a_batch_host_over_the_facade() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");
    let forbidden = [
        "sysml-v2-parser",
        "sysml_source",
        "sysml_resolution",
        "semantic_publication",
        "kpar",
        "lsp_server",
        "tower-lsp",
        "tower_lsp",
        "tokio",
        "clap",
        "rmcp",
        "axum",
        "walkdir",
        "ignore",
        "postcard",
        "zip",
        "directories",
        "dirs",
        "toml",
    ];
    let normal_dependencies = cargo_toml
        .split("[dev-dependencies]")
        .next()
        .unwrap_or_default();
    for dep in forbidden {
        // Line-anchored: `version.workspace = true` is an inherited field, not a dependency.
        assert!(
            !normal_dependencies
                .lines()
                .any(|line| line.starts_with(&format!("{dep} ="))),
            "workspace must not depend on {dep}"
        );
    }

    for required in ["sysml_query", "library_catalog", "language_service"] {
        assert!(
            normal_dependencies
                .lines()
                .any(|line| line.starts_with(&format!("{required} ="))),
            "workspace must depend on {required}"
        );
    }
    assert!(
        !cargo_toml.contains("build = "),
        "library provisioning and its build script belong to library_catalog"
    );
}
