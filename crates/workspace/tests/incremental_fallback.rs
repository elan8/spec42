#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;
#[path = "support/incremental_fixtures.rs"]
mod incremental_fixtures;

use comparison_fixtures::{load_snapshot, memory_document};
use incremental_fixtures::assert_snapshot_parity;
use workspace::{
    apply_document_changes, DocumentChanges, EngineBuilder, HostContext, WorkspaceLoadRequest,
};
use tempfile::tempdir;
use url::Url;

fn fallback_engine(cache: &tempfile::TempDir, incremental: bool) -> workspace::Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .experimental_incremental_updates(incremental)
        .build()
        .expect("engine")
}

fn multi_target(paths: Vec<std::path::PathBuf>) -> WorkspaceLoadRequest {
    WorkspaceLoadRequest {
        targets: paths,
        workspace_root: None,
        strict_diagnostics: false,
        validation_timing: Default::default(),
    }
}

#[test]
fn add_document_falls_back_to_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let engine = fallback_engine(&cache, true);
    let path_a = cache.path().join("A.sysml");
    let path_c = cache.path().join("C.sysml");

    let previous = load_snapshot(&engine, &cache, "A.sysml", "package A { part def Thing; }");
    let changes = DocumentChanges::new().with_added(vec![memory_document(
        &path_c,
        "package C { part def Other; }",
    )]);
    let request = WorkspaceLoadRequest::single_target(path_a.clone());

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            request.clone(),
            HostContext::default(),
        )
        .expect("update");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let baseline = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("add fallback", baseline.as_ref(), updated.as_ref());
}

#[test]
fn remove_document_falls_back_to_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let engine = fallback_engine(&cache, true);
    let path_a = cache.path().join("A.sysml");
    let path_b = cache.path().join("B.sysml");
    std::fs::write(&path_a, "package A { part def Thing; }").expect("write");
    std::fs::write(&path_b, "package B { part def Other; }").expect("write");

    let previous = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(vec![
                memory_document(&path_a, "package A { part def Thing; }"),
                memory_document(&path_b, "package B { part def Other; }"),
            ]),
            multi_target(vec![path_a.clone(), path_b.clone()]),
            HostContext::default(),
        )
        .expect("initial");

    let removed = Url::from_file_path(&path_b).expect("uri");
    let changes = DocumentChanges::new().with_removed(vec![removed]);
    let request = multi_target(vec![path_a.clone()]);

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            request.clone(),
            HostContext::default(),
        )
        .expect("update");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let baseline = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("remove fallback", baseline.as_ref(), updated.as_ref());
}

#[test]
fn multi_changed_documents_fall_back_to_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let engine = fallback_engine(&cache, true);
    let path_a = cache.path().join("A.sysml");
    let path_b = cache.path().join("B.sysml");
    std::fs::write(&path_a, "package A { part def One; }").expect("write");
    std::fs::write(&path_b, "package B { part def Two; }").expect("write");

    let previous = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(vec![
                memory_document(&path_a, "package A { part def One; }"),
                memory_document(&path_b, "package B { part def Two; }"),
            ]),
            multi_target(vec![path_a.clone(), path_b.clone()]),
            HostContext::default(),
        )
        .expect("initial");

    let changes = DocumentChanges::new().with_changed(vec![
        memory_document(&path_a, "package A { part def Alpha; }"),
        memory_document(&path_b, "package B { part def Beta; }"),
    ]);
    let request = multi_target(vec![path_a.clone(), path_b.clone()]);

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            request.clone(),
            HostContext::default(),
        )
        .expect("update");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let baseline = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("multi-changed fallback", baseline.as_ref(), updated.as_ref());
}

/// Covers the third `can_use_incremental_update` guard (`update.rs:83`): if the engine's
/// library catalog hash no longer matches the one recorded on the previous snapshot, the
/// incremental path must not be attempted — fall back to a full rebuild instead. Two engines
/// with different `extra_library_paths` produce different `library_catalog().content_hash`
/// values (the hash is computed over package-root paths and library version config, not file
/// contents — see `catalog.rs`'s `hash_package_roots`), which is enough to simulate a host
/// reconstructing `Spec42Engine` with a different library configuration between snapshots.
#[test]
fn library_catalog_change_falls_back_to_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let lib_a = tempdir().expect("lib_a tempdir");
    let lib_b = tempdir().expect("lib_b tempdir");

    let engine_a = EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .extra_library_paths(vec![lib_a.path().to_path_buf()])
        .experimental_incremental_updates(true)
        .build()
        .expect("engine_a");
    let engine_b = EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .extra_library_paths(vec![lib_b.path().to_path_buf()])
        .experimental_incremental_updates(true)
        .build()
        .expect("engine_b");
    assert_ne!(
        engine_a.library_catalog().content_hash,
        engine_b.library_catalog().content_hash,
        "test fixture should produce distinct library catalog hashes"
    );

    let model_path = cache.path().join("Demo.sysml");
    let previous = load_snapshot(
        &engine_a,
        &cache,
        "Demo.sysml",
        "package Demo { part def Thing; part item : Thing; }",
    );

    let updated_content = "package Demo { part def Thing; part widget : Thing; }";
    let changes = DocumentChanges::new().replace(memory_document(&model_path, updated_content));
    let request = WorkspaceLoadRequest::single_target(model_path.clone());

    // engine_b's catalog hash differs from what `previous` recorded, so this must fall back
    // to a full rebuild rather than attempting (and mis-serving) an incremental patch.
    let updated = engine_b
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            request.clone(),
            HostContext::default(),
        )
        .expect("update");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let baseline = engine_b
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("library catalog fallback", baseline.as_ref(), updated.as_ref());
}

#[test]
fn experimental_flag_off_still_correct_via_fallback() {
    let cache = tempdir().expect("tempdir");
    let engine = fallback_engine(&cache, false);
    let model_path = cache.path().join("Demo.sysml");

    let previous = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        "package Demo { part def Thing; part item : Thing; }",
    );

    let updated_content = "package Demo { part def Thing; part widget : Thing; }";
    let changes =
        DocumentChanges::new().replace(memory_document(&model_path, updated_content));
    let request = WorkspaceLoadRequest::single_target(model_path.clone());

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            request.clone(),
            HostContext::default(),
        )
        .expect("update");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let baseline = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("flag off", baseline.as_ref(), updated.as_ref());
}
