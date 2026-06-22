use std::fs;
use std::path::Path;

#[test]
fn spec42_host_does_not_depend_on_protocol_or_runtime_crates() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");
    let forbidden = [
        "kernel",
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
            "spec42_host must not depend on {dep}"
        );
    }

    for required in ["semantic_core", "language_service"] {
        assert!(
            cargo_toml.contains(&format!("{required} =")),
            "spec42_host must depend on {required}"
        );
    }
}
