#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;
#[path = "support/incremental_fixtures.rs"]
mod incremental_fixtures;

use comparison_fixtures::{load_snapshot, memory_document};
use incremental_fixtures::assert_snapshot_parity;
use tempfile::tempdir;
use workspace::{
    apply_document_changes, DocumentChanges, EngineBuilder, HostContext, WorkspaceLoadRequest,
};

fn multi_target(paths: Vec<std::path::PathBuf>) -> WorkspaceLoadRequest {
    WorkspaceLoadRequest {
        targets: paths,
        workspace_root: None,
        strict_diagnostics: false,
        validation_timing: Default::default(),
    }
}

fn incremental_engine(cache: &tempfile::TempDir) -> workspace::Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .experimental_incremental_updates(true)
        .build()
        .expect("engine")
}

#[test]
fn single_document_update_matches_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let model_path = cache.path().join("Demo.sysml");

    let initial_content = r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#;
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", initial_content);

    let updated_content = r#"
package Demo {
    part def Thing;
    part widget : Thing;
}
"#;
    let changed_doc = memory_document(&model_path, updated_content);
    let changes = DocumentChanges::new().replace(changed_doc.clone());

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            WorkspaceLoadRequest::single_target(model_path.clone()),
            HostContext::default(),
        )
        .expect("incremental update");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let request = WorkspaceLoadRequest::single_target(model_path.clone());
    let baseline = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("incremental", baseline.as_ref(), updated.as_ref());
}

#[test]
fn multi_file_workspace_single_doc_edit_matches_full_rebuild() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let path_a = cache.path().join("A.sysml");
    let path_b = cache.path().join("B.sysml");
    std::fs::write(&path_a, "package A { part def Thing; }").expect("write");
    std::fs::write(
        &path_b,
        r#"package B { private import A::*; part item : Thing; }"#,
    )
    .expect("write");

    let previous = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(vec![
                memory_document(&path_a, "package A { part def Thing; }"),
                memory_document(
                    &path_b,
                    r#"package B { private import A::*; part item : Thing; }"#,
                ),
            ]),
            multi_target(vec![path_a.clone(), path_b.clone()]),
            HostContext::default(),
        )
        .expect("initial");

    let new_b = r#"package B { private import A::*; part widget : Thing; }"#;
    std::fs::write(&path_b, new_b).expect("write updated");
    let changes = DocumentChanges::new().replace(memory_document(&path_b, new_b));
    let request = multi_target(vec![path_a.clone(), path_b.clone()]);

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes.clone(),
            request.clone(),
            HostContext::default(),
        )
        .expect("incremental");

    let merged = apply_document_changes(previous.documents(), &changes).expect("merge");
    let baseline = engine
        .load_workspace(
            workspace::InMemoryDocumentProvider::new(merged),
            request,
            HostContext::default(),
        )
        .expect("baseline");

    assert_snapshot_parity("multi-file", baseline.as_ref(), updated.as_ref());
}

/// Regression test for a real bug found while designing Tier 2 Phase 3b (see
/// shared graph-patch consolidation): `workspace` crate's
/// graph pipeline never called `evaluate_expressions`, in either the full-build or
/// incremental-update path, so evaluation facts were silently absent. Fixed by routing both paths through `sysml_model::finalize_and_evaluate`
/// instead of `finalize_workspace_graph`.
#[test]
fn full_build_publishes_typed_evaluation() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let content = r#"
package Demo {
    part def Rocket {
        attribute mass = 1 + 2;
    }
}
"#;
    let snapshot = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let uri = snapshot.documents()[0].uri.clone();
    let mass = snapshot
        .semantic_graph()
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("mass attribute node");
    assert!(
        matches!(snapshot.semantic_graph().expression_evaluation_for(mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.value == Some(sysml_model::EvaluatedValue::Integer(3)))
    );
}

#[test]
fn incremental_update_publishes_typed_evaluation() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let model_path = cache.path().join("Demo.sysml");

    let previous = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        "package Demo { part def Rocket { attribute mass = 1; } }",
    );

    let updated_content = "package Demo { part def Rocket { attribute mass = 1 + 2; } }";
    let changed_doc = memory_document(&model_path, updated_content);
    let changes = DocumentChanges::new().replace(changed_doc);

    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes,
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("incremental update");

    let uri = updated.documents()[0].uri.clone();
    let mass = updated
        .semantic_graph()
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("mass attribute node");
    assert!(
        matches!(updated.semantic_graph().expression_evaluation_for(mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.value == Some(sysml_model::EvaluatedValue::Integer(3)))
    );
}
