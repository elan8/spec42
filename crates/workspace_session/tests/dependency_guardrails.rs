use std::fs;
use std::path::Path;

#[test]
fn workspace_session_depends_on_query_facade_but_not_implementation_or_protocol_crates() {
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
    ];
    for dep in forbidden {
        assert!(
            !cargo_toml.contains(&format!("{dep} =")),
            "workspace_session must stay protocol-neutral and must not depend on {dep}"
        );
    }

    for required in ["tokio", "workspace", "sysml_query"] {
        assert!(
            cargo_toml.contains(&format!("{required} =")),
            "workspace_session must depend on {required}"
        );
    }
}
