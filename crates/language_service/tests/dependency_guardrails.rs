use std::fs;
use std::path::Path;

#[test]
fn language_service_does_not_depend_on_kernel_tower_lsp_or_tokio() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");
    let forbidden = ["kernel", "tower-lsp", "tower_lsp", "tokio"];
    for dep in forbidden {
        assert!(
            !cargo_toml.contains(&format!("{dep} =")),
            "language_service must not depend on {dep}"
        );
    }
}
