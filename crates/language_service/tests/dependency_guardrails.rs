use std::fs;
use std::path::Path;

#[test]
fn language_service_does_not_depend_on_kernel_tower_lsp_or_tokio() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");
    let forbidden = ["lsp_server", "tower-lsp", "tower_lsp", "tokio"];
    for dep in forbidden {
        assert!(
            !cargo_toml.contains(&format!("{dep} =")),
            "language_service must not depend on {dep}"
        );
    }
}

#[test]
fn language_service_consumes_an_injected_publication_instead_of_building_one() {
    let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut pending = vec![source_dir];
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).expect("read source directory") {
            let path = entry.expect("source entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = fs::read_to_string(&path).expect("read Rust source");
                let production_source = source.split("#[cfg(test)]").next().unwrap_or(&source);
                for forbidden in ["BuildRequest::resolved", "resolved_with_library"] {
                    assert!(
                        !production_source.contains(forbidden),
                        "language_service must consume the host publication; found {forbidden} in {}",
                        path.display()
                    );
                }
            }
        }
    }
}
