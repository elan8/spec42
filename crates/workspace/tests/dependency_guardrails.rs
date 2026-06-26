use std::fs;
use std::path::Path;

#[test]
fn workspace_does_not_depend_on_protocol_or_runtime_crates() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");
    let forbidden = [
        "lsp_server",
        "tower-lsp",
        "tower_lsp",
        "tokio",
        "clap",
        "rmcp",
        "axum",
    ];
    for dep in forbidden {
        assert!(
            !cargo_toml.contains(&format!("{dep} =")),
            "workspace must not depend on {dep}"
        );
    }

    for required in ["sysml_model", "language_service"] {
        assert!(
            cargo_toml.contains(&format!("{required} =")),
            "workspace must depend on {required}"
        );
    }
}
