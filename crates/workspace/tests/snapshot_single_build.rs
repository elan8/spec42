use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use sysml_model::{
    SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind,
};
use workspace::{
    ChangesetDocumentProvider, EngineBuilder, HostContext, InMemoryDocumentProvider, Spec42Engine,
    WorkspaceLoadRequest,
};
use tempfile::tempdir;
use url::Url;

struct CountingProvider {
    inner: InMemoryDocumentProvider,
    loads: Arc<AtomicUsize>,
}

impl CountingProvider {
    fn new(documents: Vec<SysmlDocument>, loads: Arc<AtomicUsize>) -> Self {
        Self {
            inner: InMemoryDocumentProvider::new(documents),
            loads,
        }
    }
}

impl SysmlDocumentProvider for CountingProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        self.loads.fetch_add(1, Ordering::SeqCst);
        self.inner.load_documents()
    }
}

fn test_engine(cache: &tempfile::TempDir) -> Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .build()
        .expect("engine")
}

fn file_document(path: &std::path::Path, content: &str) -> SysmlDocument {
    let uri = Url::from_file_path(path).expect("file uri");
    SysmlDocument {
        uri,
        content: content.to_string(),
        path_hint: path
            .file_name()
            .map(|name| name.to_string_lossy().replace('\\', "/")),
        source_kind: SysmlDocumentSourceKind::Workspace,
        sha256: None,
        byte_size: None,
    }
}

#[test]
fn snapshot_queries_reuse_single_provider_load() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Demo.sysml");
    std::fs::write(
        &model_path,
        r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#,
    )
    .expect("write model");

    let document = file_document(&model_path, &std::fs::read_to_string(&model_path).unwrap());
    let loads = Arc::new(AtomicUsize::new(0));
    let provider = CountingProvider::new(vec![document], Arc::clone(&loads));
    let engine = test_engine(&cache);
    let snapshot = engine
        .load_workspace(
            provider,
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("snapshot");

    assert_eq!(loads.load(Ordering::SeqCst), 1);
    assert_eq!(snapshot.validation().summary.document_count, 1);
    let _language = snapshot.language_workspace();
    let _catalog = snapshot.view_catalog();
    let _view = snapshot
        .prepare_view("general-view", None)
        .expect("general view");
    assert_eq!(
        loads.load(Ordering::SeqCst),
        1,
        "provider.load_documents must be called only once during snapshot build"
    );
}

#[test]
fn changeset_provider_overlays_documents() {
    let cache = tempdir().expect("tempdir");
    let base_path = cache.path().join("Changed.sysml");
    std::fs::write(&base_path, "package Base { part def A; }").expect("write base");

    let changed_content = "package Changed { part def B; }";
    std::fs::write(&base_path, changed_content).expect("write changed");

    let engine = test_engine(&cache);
    let provider = ChangesetDocumentProvider::new(InMemoryDocumentProvider::new(vec![
        file_document(&base_path, "package Base { part def A; }"),
    ]))
    .with_changed(vec![file_document(&base_path, changed_content)]);

    let snapshot = engine
        .load_workspace(
            provider,
            WorkspaceLoadRequest::single_target(base_path),
            HostContext::default(),
        )
        .expect("snapshot");

    assert!(snapshot
        .semantic_projection()
        .nodes
        .iter()
        .any(|node| node.name == "B"));
}

#[test]
fn snapshot_projects_typed_feature_value_and_expression() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("FeatureValue.sysml");
    let content = "package Demo { attribute mass = 10; }";
    std::fs::write(&model_path, content).expect("write model");

    let engine = test_engine(&cache);
    let snapshot = engine
        .load_workspace(
            InMemoryDocumentProvider::new(vec![file_document(&model_path, content)]),
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("snapshot");
    let projection = snapshot.semantic_projection();

    let value = projection
        .feature_values
        .iter()
        .find(|value| value.kind == "bound")
        .expect("bound FeatureValue is projected");
    assert!(projection
        .expressions
        .iter()
        .any(|expression| expression.semantic_id == value.expression_id
            && expression.kind == "integerLiteral"));
}

#[test]
fn snapshot_projects_connector_ends_for_a_resolved_connect_statement() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Connector.sysml");
    let content = r#"
package Demo {
    port def CmdPort;
    part def Sensor {
        port cmd : CmdPort;
    }
    part def System {
        part sensorA : Sensor;
        part sensorB : Sensor;
        connect sensorA.cmd to sensorB.cmd;
    }
}
"#;
    std::fs::write(&model_path, content).expect("write model");

    let engine = test_engine(&cache);
    let snapshot = engine
        .load_workspace(
            InMemoryDocumentProvider::new(vec![file_document(&model_path, content)]),
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("snapshot");
    let projection = snapshot.semantic_projection();

    let connection = projection
        .relationships
        .iter()
        .find(|relationship| relationship.connect.is_some())
        .expect("Connection relationship with connect detail is projected");

    let ends: Vec<_> = projection
        .connector_ends
        .iter()
        .filter(|end| end.owner_id == connection.semantic_id)
        .collect();
    assert_eq!(ends.len(), 2, "both connector ends are projected");
    let end_0 = ends
        .iter()
        .find(|end| end.end_index == 0)
        .expect("end 0 present");
    let end_1 = ends
        .iter()
        .find(|end| end.end_index == 1)
        .expect("end 1 present");
    assert_eq!(
        end_0.target_feature_id.as_deref(),
        Some(connection.source_id.as_str())
    );
    assert_eq!(
        end_1.target_feature_id.as_deref(),
        Some(connection.target_id.as_str())
    );
}

#[test]
fn snapshot_projects_flow_detail_with_payload_and_succession_kind() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("FlowDetail.sysml");
    let content = r#"
package Demo {
    port def CmdPort;
    part def Sensor {
        port cmd : CmdPort;
    }
    attribute def Payload;
    part def System {
        part sensorA : Sensor;
        part sensorB : Sensor;
        succession flow dataFlow of Payload from sensorA.cmd to sensorB.cmd;
    }
}
"#;
    std::fs::write(&model_path, content).expect("write model");

    let engine = test_engine(&cache);
    let snapshot = engine
        .load_workspace(
            InMemoryDocumentProvider::new(vec![file_document(&model_path, content)]),
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("snapshot");
    let projection = snapshot.semantic_projection();

    let flow = projection
        .relationships
        .iter()
        .find(|relationship| relationship.flow.is_some())
        .expect("Flow relationship with flow detail is projected");
    assert_eq!(flow.kind.as_str(), "successionFlow");
    let detail = flow.flow.as_ref().expect("flow detail present");
    assert_eq!(detail.payload_expression.as_deref(), Some("Payload"));
    assert!(detail.source_expression.is_some());
    assert!(detail.target_expression.is_some());
}
