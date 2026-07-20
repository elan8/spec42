use std::fs;
use std::path::Path;

#[test]
fn sysml_diagnostics_depends_on_sysml_model_but_not_protocol_or_runtime_crates() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("read Cargo.toml");

    let forbidden = [
        "tokio",
        "tower-lsp",
        "tower_lsp",
        "lsp_server",
        "clap",
        "rmcp",
        "axum",
    ];
    for dep in forbidden {
        assert!(
            !cargo_toml.contains(&format!("{dep} =")),
            "sysml_diagnostics must not depend on {dep}"
        );
    }

    for required in ["sysml_model"] {
        assert!(
            cargo_toml.contains(&format!("{required} =")),
            "sysml_diagnostics must depend on {required}"
        );
    }
}

/// Locks in the boundary the `kinds.rs` classifier consolidation established: diagnostics
/// reads element-kind classification from `sysml_model::semantic::kinds`, never from the
/// projection/view-rendering modules. A prior version of this code delegated
/// `is_port_like`/`is_part_like` to `ibd::is_port_like`/`is_part_like` instead — see the
/// commit that fixed it. Without this guardrail, that coupling could silently regress since
/// nothing else in the crate graph would catch it until a future `sysml_projection` split
/// made it a hard compile error.
#[test]
fn diagnostics_does_not_reference_projection_modules() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let forbidden_patterns = [
        "ibd::",
        "model_projection",
        "sequence_views",
        "state_views",
        "activity_graph",
        "interconnection_",
        "visualization",
        "prepared_view",
        "extracted_model",
        "explicit_views",
        "component_view",
        "view_projection",
        "render_snapshot",
    ];

    let mut offenders = Vec::new();
    for file in list_rs_files(&src_dir) {
        let content = fs::read_to_string(&file).expect("read source file");
        for pattern in forbidden_patterns {
            if content.contains(pattern) {
                offenders.push(format!("{}: contains `{pattern}`", file.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "sysml_diagnostics must not reference projection/view-rendering modules:\n{}",
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
