use std::{fs, path::Path};

use sysml_model::InterconnectionSceneDto;
use ts_rs::TS;

#[test]
fn export_interconnection_scene_typescript_bindings() {
    let committed_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../vscode/src/generated/backend")
        .components()
        .collect::<std::path::PathBuf>();

    if std::env::var("SPEC42_UPDATE_TS_BINDINGS").as_deref() == Ok("1") {
        InterconnectionSceneDto::export_all_to(&committed_dir)
            .expect("export interconnection scene TypeScript bindings");
        return;
    }

    let temp_dir = tempfile::tempdir().expect("create temporary TypeScript export directory");
    InterconnectionSceneDto::export_all_to(temp_dir.path())
        .expect("export interconnection scene TypeScript bindings");

    let mut generated_files = fs::read_dir(temp_dir.path())
        .expect("read generated TypeScript export directory")
        .map(|entry| entry.expect("read generated TypeScript file").file_name())
        .collect::<Vec<_>>();
    generated_files.sort();

    for file_name in generated_files {
        let generated = fs::read_to_string(temp_dir.path().join(&file_name))
            .expect("read generated TypeScript binding");
        let committed_path = committed_dir.join(&file_name);
        let committed = fs::read_to_string(&committed_path)
            .unwrap_or_else(|_| panic!("missing committed TypeScript binding {}", committed_path.display()));
        assert_eq!(
            committed, generated,
            "TypeScript binding {} is stale; rerun with SPEC42_UPDATE_TS_BINDINGS=1",
            committed_path.display()
        );
    }
}
