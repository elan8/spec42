#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::memory_document;
use spec42_host::{DocumentChanges, EngineBuilder, HostContext, WorkspaceLoadRequest};
use std::time::Instant;
use tempfile::tempdir;

fn incremental_engine(cache: &tempfile::TempDir) -> spec42_host::Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .experimental_incremental_updates(true)
        .build()
        .expect("engine")
}

fn multi_file_fixture(cache: &tempfile::TempDir) -> (Vec<std::path::PathBuf>, Vec<spec42_host::SysmlDocument>) {
    let mut paths = Vec::new();
    let mut documents = Vec::new();
    for index in 0..8 {
        let filename = format!("Model{index}.sysml");
        let path = cache.path().join(&filename);
        let content = format!(
            "package Model{index} {{ part def Part{index}; part item{index} : Part{index}; }}"
        );
        std::fs::write(&path, &content).expect("write");
        paths.push(path.clone());
        documents.push(memory_document(&path, &content));
    }
    (paths, documents)
}

#[test]
#[ignore = "manual benchmark: log full vs incremental update timings"]
fn benchmark_single_document_incremental_vs_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let (paths, documents) = multi_file_fixture(&cache);
    let request = WorkspaceLoadRequest {
        targets: paths.clone(),
        workspace_root: Some(cache.path().to_path_buf()),
        strict_diagnostics: false,
    };

    let previous = engine
        .load_workspace(
            spec42_host::InMemoryDocumentProvider::new(documents.clone()),
            request.clone(),
            HostContext::default(),
        )
        .expect("initial load");

    let edit_path = paths[3].clone();
    let edited_content =
        "package Model3 { part def Part3; part item3 : Part3; part extra : Part3; }";
    std::fs::write(&edit_path, edited_content).expect("write edit");
    let changes = DocumentChanges::new().replace(memory_document(&edit_path, edited_content));

    let full_start = Instant::now();
    let merged = spec42_host::apply_document_changes(previous.documents(), &changes).expect("merge");
    let _full = engine
        .load_workspace(
            spec42_host::InMemoryDocumentProvider::new(merged),
            request.clone(),
            HostContext::default(),
        )
        .expect("full rebuild");
    let full_ms = full_start.elapsed().as_millis();

    let incremental_start = Instant::now();
    let _incremental = engine
        .update_snapshot(
            previous.as_ref(),
            changes,
            request,
            HostContext::default(),
        )
        .expect("incremental update");
    let incremental_ms = incremental_start.elapsed().as_millis();

    eprintln!("incremental benchmark (8 files, 1 edit): full={full_ms}ms incremental={incremental_ms}ms");
}
