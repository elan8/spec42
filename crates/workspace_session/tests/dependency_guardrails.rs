use std::fs;
use std::path::Path;

#[test]
fn workspace_session_depends_on_tokio_and_no_sysml_or_protocol_crate() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");

    let forbidden = [
        "tower-lsp",
        "tower_lsp",
        "axum",
        "lsp_server",
        "rmcp",
        "clap",
        "sysml_model",
        "sysml_diagnostics",
        "sysml_source",
        "sysml_resolution",
        "sysml_query",
        "workspace",
        "semantic_publication",
    ];
    for dep in forbidden {
        // Line-anchored: `version.workspace = true` is an inherited field, not a dependency on
        // the crate called `workspace`.
        assert!(
            !cargo_toml
                .lines()
                .any(|line| line.starts_with(&format!("{dep} ="))),
            "workspace_session is a generic actor and must not depend on {dep}"
        );
    }

    assert!(
        cargo_toml.contains("tokio ="),
        "workspace_session must depend on tokio"
    );
}
