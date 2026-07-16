use std::{
    fs,
    path::{Path, PathBuf},
};

use sysml_model::SysmlVisualizationResultDto;
use ts_rs::TS;

#[test]
fn export_interconnection_scene_typescript_bindings() {
    let committed_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../vscode/src/generated/backend")
        .components()
        .collect::<std::path::PathBuf>();

    if std::env::var("SPEC42_UPDATE_TS_BINDINGS").as_deref() == Ok("1") {
        SysmlVisualizationResultDto::export_all_to(&committed_dir)
            .expect("export visualization TypeScript bindings");
        return;
    }

    let temp_dir = tempfile::tempdir().expect("create temporary TypeScript export directory");
    SysmlVisualizationResultDto::export_all_to(temp_dir.path())
        .expect("export visualization TypeScript bindings");

    let mut generated_files = Vec::new();
    collect_files(temp_dir.path(), temp_dir.path(), &mut generated_files);
    generated_files.sort();

    for relative_path in generated_files {
        let generated = fs::read_to_string(temp_dir.path().join(&relative_path))
            .expect("read generated TypeScript binding");
        let committed_path = committed_dir.join(&relative_path);
        let committed = fs::read_to_string(&committed_path).unwrap_or_else(|_| {
            panic!(
                "missing committed TypeScript binding {}",
                committed_path.display()
            )
        });
        assert_eq!(
            committed,
            generated,
            "TypeScript binding {} is stale; rerun with SPEC42_UPDATE_TS_BINDINGS=1",
            committed_path.display()
        );
    }
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read generated TypeScript export directory") {
        let entry = entry.expect("read generated TypeScript export entry");
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, out);
            continue;
        }
        out.push(
            path.strip_prefix(root)
                .expect("generated file is under export root")
                .to_path_buf(),
        );
    }
}
