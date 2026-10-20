use crate::comparison_fixtures::memory_document;
use library_catalog::ProjectDependencyResolution;
use workspace::{
    EngineBuilder, HostContext, InMemoryProvider, SourceKind, ValidationTiming,
    WorkspaceLoadRequest,
};

const SYSTEMS_RESOURCE: &str = "https://www.omg.org/spec/SysML/20250201/Systems-Library.kpar";
const ANALYSIS_RESOURCE: &str = "https://www.omg.org/spec/SysML/20250201/Analysis-Library.kpar";

#[test]
fn installed_sysand_sources_and_source_scope_are_admitted_by_the_batch_loader() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::create_dir_all(root.join(".sysand/lib/types")).unwrap();
    std::fs::create_dir(root.join("scratch")).unwrap();
    std::fs::create_dir(root.join("model")).unwrap();
    std::fs::write(
        root.join("model/Model.sysml"),
        "package Model { private import Types::*; part item : Item; }",
    )
    .unwrap();
    std::fs::write(root.join("scratch/Draft.sysml"), "package Draft;").unwrap();
    std::fs::write(root.join("syside.toml"), "exclude = ['scratch/**']").unwrap();
    std::fs::write(
        root.join(".sysand/lib/types/Types.sysml"),
        "package Types { part def Item; }",
    )
    .unwrap();
    std::fs::write(
        root.join(".sysand/lib/types/.project.json"),
        r#"{"name":"Types","version":"1.0.0"}"#,
    )
    .unwrap();
    write_project(root, "https://example.test/types", "1.0.0");
    std::fs::write(root.join("sysand-lock.toml"), "lock_version = '0.6'\n[[project]]\nname = 'Types'\nversion = '1.0.0'\nidentifiers = ['https://example.test/types']\n").unwrap();
    std::fs::write(root.join(".sysand/env.toml"), "version = '0.2'\n[[project]]\nname = 'Types'\nversion = '1.0.0'\nidentifiers = ['https://example.test/types']\npath = 'lib/types'\n").unwrap();
    let engine = EngineBuilder::default()
        .cache_dir(root.join("cache"))
        .no_stdlib(true)
        .build()
        .unwrap();
    for target in [root.to_path_buf(), root.join("model/Model.sysml")] {
        let provider = workspace::HostFilesystemProvider::from_paths(
            &target,
            None,
            &[],
            engine.services().clone(),
        );
        let snapshot = engine
            .load_workspace(
                provider,
                WorkspaceLoadRequest::single_target(target)
                    .with_validation_timing(ValidationTiming::Deferred),
                HostContext::default(),
            )
            .unwrap();
        assert_eq!(snapshot.workspace_root(), root.canonicalize().unwrap());
        assert_eq!(
            snapshot
                .documents()
                .iter()
                .filter(|document| document.kind() == SourceKind::Workspace)
                .count(),
            1
        );
        assert_eq!(
            snapshot
                .documents()
                .iter()
                .filter(|document| document.kind() == SourceKind::Library)
                .count(),
            1
        );
        assert!(
            matches!(&snapshot.project_dependencies()[0], ProjectDependencyResolution::Satisfied { selected_version, .. } if selected_version == "1.0.0")
        );
    }
}

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

fn embedded_engine(root: &std::path::Path) -> workspace::Spec42Engine {
    EngineBuilder::default()
        .cache_dir(root.join("cache"))
        .embed_standard_library()
        .build()
        .expect("engine")
}

fn candidate_root(engine: &workspace::Spec42Engine, resource: &str) -> std::path::PathBuf {
    engine
        .library_catalog()
        .dependency_candidates
        .iter()
        .find(|candidate| candidate.resource == resource)
        .unwrap_or_else(|| panic!("candidate for {resource}"))
        .package_roots[0]
        .canonicalize()
        .expect("canonical candidate root")
}

fn load_with_candidate_documents(
    engine: &workspace::Spec42Engine,
    model: &std::path::Path,
    roots: &[std::path::PathBuf],
) -> Result<std::sync::Arc<workspace::HostWorkspaceSnapshot>, workspace::WorkspaceError> {
    let mut documents = vec![memory_document(model, "package Model;")];
    for root in roots {
        let path = first_model(root);
        documents.push(engine.source().admit_url(
            workspace::path_to_file_url(&path).expect("library URL"),
            &std::fs::read_to_string(path).expect("read library"),
            SourceKind::Library,
        ));
    }
    engine.load_workspace(
        InMemoryProvider::new(documents),
        WorkspaceLoadRequest::single_target(model.to_path_buf())
            .with_validation_timing(ValidationTiming::Deferred),
        HostContext::default(),
    )
}

fn has_document_under(snapshot: &workspace::HostWorkspaceSnapshot, root: &std::path::Path) -> bool {
    snapshot.documents().iter().any(|document| {
        document
            .uri()
            .to_file_path()
            .is_ok_and(|path| path.canonicalize().unwrap_or(path).starts_with(root))
    })
}

#[test]
fn bundled_project_satisfies_exact_manifest_usage() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    write_project(temp.path(), SYSTEMS_RESOURCE, "2.0.0");
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
        has_document_under(&snapshot, &systems_root),
        "documents: {:?}, root: {}",
        snapshot
            .documents()
            .iter()
            .map(|d| d.uri().to_string())
            .collect::<Vec<_>>(),
        systems_root.display()
    );
    assert!(has_document_under(&snapshot, &analysis_root));
}

#[test]
fn manifestless_workspace_keeps_all_bundled_defaults() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    let engine = embedded_engine(temp.path());
    let systems_root = candidate_root(&engine, SYSTEMS_RESOURCE);
    let analysis_root = candidate_root(&engine, ANALYSIS_RESOURCE);

    let snapshot = load_with_candidate_documents(
        &engine,
        &model,
        &[systems_root.clone(), analysis_root.clone()],
    )
    .expect("manifestless snapshot");

    assert!(snapshot.project_dependencies().is_empty());
    assert!(has_document_under(&snapshot, &systems_root));
    assert!(has_document_under(&snapshot, &analysis_root));
}

#[test]
fn unresolved_resource_is_an_external_workspace_error() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    write_project(temp.path(), "https://example.invalid/Missing.kpar", "1.0.0");
    let engine = embedded_engine(temp.path());

    let error = load_with_candidate_documents(&engine, &model, &[])
        .expect_err("unknown resource must fail");
    assert!(error.to_string().contains("unresolved"), "{error}");
    assert!(error.to_string().contains("Missing.kpar"), "{error}");
}

#[test]
fn malformed_manifest_and_invalid_constraint_are_external_workspace_errors() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    let engine = embedded_engine(temp.path());

    std::fs::write(temp.path().join(".project.json"), "{").expect("write invalid manifest");
    let malformed = load_with_candidate_documents(&engine, &model, &[])
        .expect_err("malformed manifest must fail");
    assert!(
        malformed.to_string().contains("Invalid project manifest"),
        "{malformed}"
    );

    write_project(temp.path(), SYSTEMS_RESOURCE, "not a constraint !!!");
    let invalid_constraint = load_with_candidate_documents(&engine, &model, &[])
        .expect_err("invalid constraint must fail");
    assert!(
        invalid_constraint
            .to_string()
            .contains("invalid-version-constraint"),
        "{invalid_constraint}"
    );
}

#[test]
fn multiple_declared_dependencies_are_admitted_together() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    std::fs::write(
        temp.path().join(".project.json"),
        format!(
            r#"{{"name":"Model","version":"1.0.0","usage":[{{"resource":"{SYSTEMS_RESOURCE}","versionConstraint":"2.0.0"}},{{"resource":"{ANALYSIS_RESOURCE}","versionConstraint":"2.0.0"}}]}}"#
        ),
    )
    .expect("write manifest");
    let engine = embedded_engine(temp.path());
    let systems_root = candidate_root(&engine, SYSTEMS_RESOURCE);
    let analysis_root = candidate_root(&engine, ANALYSIS_RESOURCE);

    let snapshot = load_with_candidate_documents(
        &engine,
        &model,
        &[systems_root.clone(), analysis_root.clone()],
    )
    .expect("snapshot");
    assert_eq!(snapshot.project_dependencies().len(), 2);
    assert!(has_document_under(&snapshot, &systems_root));
    assert!(has_document_under(&snapshot, &analysis_root));
}

#[test]
fn rebuilding_after_manifest_change_recomputes_dependency_admission() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    let engine = embedded_engine(temp.path());
    let systems_root = candidate_root(&engine, SYSTEMS_RESOURCE);
    let analysis_root = candidate_root(&engine, ANALYSIS_RESOURCE);
    let roots = [systems_root.clone(), analysis_root.clone()];

    write_project(temp.path(), SYSTEMS_RESOURCE, "2.0.0");
    let before = load_with_candidate_documents(&engine, &model, &roots).expect("first snapshot");
    assert!(has_document_under(&before, &systems_root));
    assert!(has_document_under(&before, &analysis_root));
    assert!(matches!(
        &before.project_dependencies()[0],
        ProjectDependencyResolution::Satisfied { resource, .. } if resource == SYSTEMS_RESOURCE
    ));

    write_project(temp.path(), ANALYSIS_RESOURCE, "2.0.0");
    let after = load_with_candidate_documents(&engine, &model, &roots).expect("rebuilt snapshot");
    assert!(has_document_under(&after, &systems_root));
    assert!(has_document_under(&after, &analysis_root));
    assert!(matches!(
        &after.project_dependencies()[0],
        ProjectDependencyResolution::Satisfied { resource, .. } if resource == ANALYSIS_RESOURCE
    ));
}

#[test]
fn version_mismatch_is_an_external_workspace_error() {
    let temp = tempfile::tempdir().expect("tempdir");
    let model = temp.path().join("Model.sysml");
    std::fs::write(&model, "package Model;").expect("write model");
    write_project(temp.path(), SYSTEMS_RESOURCE, "9.0.0");
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
