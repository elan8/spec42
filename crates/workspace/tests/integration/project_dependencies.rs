use crate::comparison_fixtures::memory_document;
use library_catalog::ProjectDependencyResolution;
use workspace::{
    EngineBuilder, HostContext, InMemoryProvider, SourceKind, ValidationTiming,
    WorkspaceLoadRequest,
};

fn first_model(root: &std::path::Path) -> std::path::PathBuf {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)
            .expect("read library root")
            .flatten()
        {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("sysml" | "kerml")
            ) {
                return path;
            }
        }
    }
    panic!("library root has no model")
}

fn write_project(root: &std::path::Path, resource: &str, version: &str) {
    std::fs::write(
        root.join(".project.json"),
        format!(
            r#"{{"name":"Model","version":"1.0.0","usage":[{{"resource":"{resource}","versionConstraint":"{version}"}}]}}"#
        ),
    )
    .expect("write manifest");
}

#[test]
fn bundled_project_satisfies_exact_manifest_usage() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    write_project(
        temp.path(),
        "https://www.omg.org/spec/SysML/20250201/Systems-Library.kpar",
        "2.0.0",
    );
    let engine = EngineBuilder::default()
        .cache_dir(temp.path().join("cache"))
        .embed_standard_library()
        .build()
        .expect("engine");
    let systems_root = engine
        .library_catalog()
        .dependency_candidates
        .iter()
        .find(|candidate| candidate.resource.ends_with("/Systems-Library.kpar"))
        .expect("systems candidate")
        .package_roots[0]
        .clone();
    let analysis_root = engine
        .library_catalog()
        .dependency_candidates
        .iter()
        .find(|candidate| candidate.resource.ends_with("/Analysis-Library.kpar"))
        .expect("analysis candidate")
        .package_roots[0]
        .clone();
    let systems_root = systems_root.canonicalize().expect("canonical systems root");
    let analysis_root = analysis_root
        .canonicalize()
        .expect("canonical analysis root");
    let systems_file = first_model(&systems_root);
    let analysis_file = first_model(&analysis_root);
    let library_document = |path: &std::path::Path| {
        engine.source().admit_url(
            workspace::path_to_file_url(path).expect("library URL"),
            &std::fs::read_to_string(path).expect("read library"),
            SourceKind::Library,
        )
    };
    let snapshot = engine
        .load_workspace(
            InMemoryProvider::new(vec![
                memory_document(&model, "package Model;"),
                library_document(&systems_file),
                library_document(&analysis_file),
            ]),
            WorkspaceLoadRequest::single_target(model)
                .with_validation_timing(ValidationTiming::Deferred),
            HostContext::default(),
        )
        .expect("snapshot");
    assert!(matches!(
        &snapshot.project_dependencies()[0],
        ProjectDependencyResolution::Satisfied {
            selected_version,
            ..
        } if selected_version == "2.0.0"
    ));
    assert!(
        snapshot.documents().iter().any(|document| document
            .uri()
            .to_file_path()
            .is_ok_and(|path| path.starts_with(&systems_root))),
        "documents: {:?}, root: {}",
        snapshot
            .documents()
            .iter()
            .map(|d| d.uri().to_string())
            .collect::<Vec<_>>(),
        systems_root.display()
    );
    assert!(
        !snapshot.documents().iter().any(|document| document
            .uri()
            .to_file_path()
            .is_ok_and(|path| path.starts_with(&analysis_root))),
        "an undeclared bundled project must not enter a manifest publication"
    );
}

#[test]
fn version_mismatch_is_an_external_workspace_error() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    write_project(
        temp.path(),
        "https://www.omg.org/spec/SysML/20250201/Systems-Library.kpar",
        "9.0.0",
    );
    let engine = EngineBuilder::default()
        .cache_dir(temp.path().join("cache"))
        .embed_standard_library()
        .build()
        .expect("engine");
    let error = engine
        .load_workspace(
            InMemoryProvider::new(vec![memory_document(&model, "package Model;")]),
            WorkspaceLoadRequest::single_target(model)
                .with_validation_timing(ValidationTiming::Deferred),
            HostContext::default(),
        )
        .expect_err("mismatch must fail");
    assert!(error.to_string().contains("version-mismatch"));
    assert!(error.to_string().contains("2.0.0"));
}
