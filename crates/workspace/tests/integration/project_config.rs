use workspace::{
    EngineBuilder, HostContext, HostFilesystemProvider, ValidationState, ValidationTiming,
    WorkspaceLoadRequest,
};

fn engine(root: &std::path::Path) -> workspace::Spec42Engine {
    EngineBuilder::default()
        .cache_dir(root.join("cache"))
        .no_stdlib(true)
        .build()
        .expect("engine")
}

fn load(
    root: &std::path::Path,
) -> Result<std::sync::Arc<workspace::HostWorkspaceSnapshot>, workspace::WorkspaceError> {
    let engine = engine(root);
    let provider =
        HostFilesystemProvider::from_paths(root, Some(root), &[], engine.services().clone());
    engine.load_workspace(
        provider,
        WorkspaceLoadRequest::single_target(root.to_path_buf())
            .with_workspace_root(Some(root.to_path_buf()))
            .with_validation_timing(ValidationTiming::Eager),
        HostContext::default(),
    )
}

fn reported_uris(snapshot: &workspace::HostWorkspaceSnapshot) -> Vec<String> {
    match snapshot.validation() {
        ValidationState::Ready(report) => report
            .documents
            .iter()
            .map(|document| document.uri.clone())
            .collect(),
        ValidationState::Deferred => panic!("validation was deferred"),
    }
}

#[test]
fn syside_toml_does_not_change_admission() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("Model.sysml"), "package Model;").unwrap();
    std::fs::write(temp.path().join("syside.toml"), "include = []").unwrap();
    let snapshot = load(temp.path()).expect("syside.toml is not a project config");
    assert_eq!(snapshot.documents().len(), 1);
    let uris = reported_uris(&snapshot);
    assert_eq!(uris.len(), 1);
    assert!(uris[0].contains("Model.sysml"));
}

#[test]
fn an_include_that_excludes_every_requested_file_fails_the_load() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("Model.sysml"), "package Model;").unwrap();
    std::fs::write(temp.path().join("spec42.toml"), "include = ['missing/**']").unwrap();
    let error = load(temp.path()).expect_err("empty admission must not succeed");
    let message = error.to_string();
    assert!(message.contains("spec42.toml"), "{message}");
    assert!(message.contains("library_paths"), "{message}");
}

#[test]
fn an_escaping_include_is_rejected_before_an_empty_export() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join("Model.sysml"), "package Model;").unwrap();
    std::fs::write(temp.path().join("spec42.toml"), "include = ['../library']").unwrap();
    let error = load(temp.path()).expect_err("escaping include");
    let message = error.to_string();
    assert!(message.contains("escapes the project root"), "{message}");
}

#[test]
fn excluded_files_are_omitted_from_the_validation_report() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir(temp.path().join("scratch")).unwrap();
    std::fs::write(temp.path().join("Model.sysml"), "package Model;").unwrap();
    std::fs::write(temp.path().join("scratch/Draft.sysml"), "package Draft;").unwrap();
    std::fs::write(temp.path().join("spec42.toml"), "exclude = ['scratch/**']").unwrap();
    let snapshot = load(temp.path()).expect("partial exclude still admits Model");
    let uris = reported_uris(&snapshot);
    assert_eq!(uris.len(), 1);
    assert!(uris[0].contains("Model.sysml"));
}

#[test]
fn project_library_paths_survive_manifest_dependency_filtering() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("project");
    let library = temp.path().join("library");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&library).unwrap();
    std::fs::write(
        root.join(".project.json"),
        r#"{"name":"App","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(root.join("spec42.toml"), "library_paths = ['../library']").unwrap();
    std::fs::write(
        root.join("Model.sysml"),
        "package Model { private import Types::*; part item : Item; }",
    )
    .unwrap();
    let library_source = library.join("Types.sysml");
    std::fs::write(&library_source, "package Types { part def Item; }").unwrap();
    let library_uri = workspace::path_to_file_url(&library_source).unwrap();

    let snapshot = load(&root).expect("configured libraries supplement manifest dependencies");
    let document = snapshot
        .documents()
        .iter()
        .find(|document| document.uri() == &library_uri)
        .expect("configured library document retained");
    assert_eq!(document.kind(), workspace::SourceKind::Library);
    let library_url = workspace::path_to_file_url(&library).unwrap();
    assert!(snapshot
        .library_paths()
        .iter()
        .any(|path| { workspace::path_to_file_url(path).unwrap() == library_url }));
    assert!(snapshot.library_urls().contains(&library_url));
}
