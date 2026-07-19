use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use sysml_model::{SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind};
use tempfile::tempdir;
use url::Url;
use workspace::{
    ChangesetDocumentProvider, EngineBuilder, HostContext, HostMembershipKind,
    HostRelationshipMetaclass, InMemoryDocumentProvider, Spec42Engine, WorkspaceLoadRequest,
};

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
    let provider =
        ChangesetDocumentProvider::new(InMemoryDocumentProvider::new(vec![file_document(
            &base_path,
            "package Base { part def A; }",
        )]))
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

    let payload_def = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name == "Demo::Payload")
        .expect("Payload attribute def is projected");
    assert_eq!(
        detail.payload_type_id.as_deref(),
        Some(payload_def.semantic_id.as_str()),
        "`of Payload` resolves to the real Payload attribute def, not raw text"
    );
}

#[test]
fn snapshot_materializes_textual_representation_as_addressable_node() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("TextualRep.sysml");
    let content = r#"
package Demo {
    rep language "sysml" /* package body text */
    requirement def Spec {
        rep docRep language "markdown" /* requirement body text */
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

    let package_rep = projection
        .nodes
        .iter()
        .find(|node| {
            node.element_kind.as_str() == "textualRep" && node.parent.as_deref() == Some("Demo")
        })
        .expect("package-level textual representation is projected");
    assert_eq!(
        package_rep.attributes.get("language"),
        Some(&serde_json::json!("sysml"))
    );
    assert!(package_rep
        .attributes
        .get("text")
        .and_then(|v| v.as_str())
        .is_some_and(|text| text.contains("package body text")));

    let requirement_rep = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name == "Demo::Spec::docRep")
        .expect("named requirement-body textual representation is projected");
    assert_eq!(requirement_rep.element_kind.as_str(), "textualRep");
    assert_eq!(
        requirement_rep.attributes.get("language"),
        Some(&serde_json::json!("markdown"))
    );
}

#[test]
fn snapshot_materializes_conjugated_port_definition_eagerly() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("ConjugatedPort.sysml");
    let content = r#"
package Demo {
    port def PowerPort {
        in enabled : Boolean;
    }
    port def UnusedPort;
    part def Sensor {
        port p1 : ~PowerPort;
        port p2 : ~PowerPort;
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

    // Eager creation (KerML 8.3.12.2 / SysML v2 8.2.2.12 Note 1): every non-conjugated port def
    // gets exactly one conjugate, including PowerPort's own declaration and UnusedPort, which is
    // never referenced with `~` anywhere -- proving this doesn't depend on usage resolution.
    let power_port = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name == "Demo::PowerPort")
        .expect("PowerPort def");
    let unused_port = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name == "Demo::UnusedPort")
        .expect("UnusedPort def");
    for base in [power_port, unused_port] {
        let conjugates: Vec<_> = projection
            .nodes
            .iter()
            .filter(|node| {
                node.element_kind.as_str() == "conjugated port definition"
                    && node.parent.as_deref() == Some(base.qualified_name.as_str())
            })
            .collect();
        assert_eq!(
            conjugates.len(),
            1,
            "{} should have exactly one conjugate",
            base.qualified_name
        );
    }

    let power_conjugate = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name == "Demo::PowerPort::~PowerPort")
        .expect("PowerPort's conjugate at the spec's own qualified-name convention (8.4.8.2)");
    assert_eq!(power_conjugate.name, "~PowerPort");
    assert_eq!(power_conjugate.parent.as_deref(), Some("Demo::PowerPort"));

    let conjugation = projection
        .relationships
        .iter()
        .find(|relationship| relationship.kind.as_str() == "portConjugation")
        .expect("PortConjugation relationship (8.3.12.4)");
    assert_eq!(conjugation.source, power_conjugate.qualified_name);
    assert_eq!(conjugation.target, power_port.qualified_name);

    // Both `~PowerPort`-typed usages are typed by the conjugate, not PowerPort directly.
    for usage_name in ["Demo::Sensor::p1", "Demo::Sensor::p2"] {
        let usage = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name == usage_name)
            .unwrap_or_else(|| panic!("{usage_name} usage"));
        let typing = projection
            .relationships
            .iter()
            .find(|relationship| {
                relationship.kind.as_str() == "typing"
                    && relationship.source == usage.qualified_name
            })
            .unwrap_or_else(|| panic!("{usage_name} typing relationship"));
        assert_eq!(
            typing.target, power_conjugate.qualified_name,
            "{usage_name} should be typed by the conjugate, not PowerPort directly"
        );
    }
}

#[test]
fn snapshot_conjugated_port_structural_mismatch_uses_feature_check_not_fallback() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("ConjugatedMismatch.sysml");
    let content = r#"
package Demo {
    port def PowerPort {
        in enabled : Boolean;
    }
    part def Sensor {
        port cmd : ~PowerPort;
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

    // Connecting two ~PowerPort-typed ports (same conjugation on both sides) mirrors the same
    // "in" direction on both -- structurally incompatible. Before the fix in this round,
    // `effective_port_features` returned empty for any conjugated port (it only matched
    // ElementKind::PortDef targets, and conjugated usages are now typed by the
    // ConjugatedPortDefinition instead), so this diagnostic only fired via the coarse
    // text-based "same conjugation" fallback (`conjugated_port_inconsistent`). After the fix,
    // the structural feature-direction check (which correctly follows the conjugate to the real
    // PortDef's children) fires first with the more precise code.
    let diagnostics = &snapshot.validation().documents[0].diagnostics;
    let mismatch = diagnostics
        .iter()
        .find(|d| {
            d.code == "flow_direction_incompatible" || d.code == "conjugated_port_inconsistent"
        })
        .expect("connecting two ~PowerPort ports should be flagged incompatible");
    assert_eq!(
        mismatch.code, "flow_direction_incompatible",
        "structural feature check should fire for conjugated ports, not just the coarse fallback"
    );
}

#[test]
fn snapshot_classifies_satisfy_and_subject_as_their_own_metaclass() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("SatisfySubject.sysml");
    let content = r#"
package Demo {
    part def Component;
    part comp1 : Component;
    requirement def Req {
        subject sys : Component;
    }
    requirement req1 : Req;
    satisfy req1 by comp1;
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

    assert!(
        projection
            .relationships
            .iter()
            .any(|relationship| relationship.kind.as_str() == "satisfy"
                && relationship.metaclass == HostRelationshipMetaclass::Satisfy),
        "a satisfy relationship classifies as the Satisfy metaclass"
    );
    assert!(
        projection
            .relationships
            .iter()
            .any(|relationship| relationship.kind.as_str() == "subject"
                && relationship.metaclass == HostRelationshipMetaclass::Subject),
        "a subject relationship classifies as the Subject metaclass"
    );
}

#[test]
fn snapshot_walks_case_def_body_for_subject_and_actor_members() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("CaseBody.sysml");
    let content = r#"
package Demo {
    part def System;
    part def Operator;
    case def MyCase {
        subject sys : System;
        actor op : Operator;
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

    // Regression guard: materialize_case_def previously never walked its body at all, so
    // subject/actor members inside `case def { ... }` were silently dropped from the graph.
    assert!(
        projection
            .nodes
            .iter()
            .any(|node| node.qualified_name.ends_with("::MyCase::sys")),
        "case def body's subject member is now materialized"
    );
    assert!(
        projection
            .nodes
            .iter()
            .any(|node| node.qualified_name.ends_with("::MyCase::op")),
        "case def body's actor member is now materialized"
    );
}

#[test]
fn snapshot_populates_occurrence_def_and_usage_facts() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Occurrence.sysml");
    let content = r#"
package Demo {
    abstract occurrence def Event;
    occurrence happening : Event;
    individual specificEvent : Event;
    snapshot momentSnap : Event;
    timeslice windowSlice : Event;
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

    // Regression guard: materialize_occurrence_def never set isAbstract; materialize_occurrence_usage
    // never set is_individual/portionKind or attach_feature_properties at all.
    let event_def = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::Event"))
        .expect("occurrence def node materialized");
    assert_eq!(
        event_def
            .facts
            .feature_properties
            .as_ref()
            .map(|p| p.is_abstract),
        Some(true)
    );

    let individual_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::specificEvent"))
        .expect("individual occurrence usage materialized");
    assert_eq!(
        individual_node
            .facts
            .feature_properties
            .as_ref()
            .map(|p| p.is_individual),
        Some(true),
        "`individual` usage sets is_individual on declared feature properties"
    );

    let snapshot_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::momentSnap"))
        .expect("snapshot occurrence usage materialized");
    assert_eq!(
        snapshot_node.attributes.get("portionKind"),
        Some(&serde_json::json!("snapshot"))
    );
    // Regression guard (S42-008): `portionKind`/`isPortion` previously only existed as this raw
    // debug attribute; `DeclaredFeatureProperties`/`HostFeatureProperties` had no slot for them
    // at all, so Babel42's `isPortion` DTO field was permanently hardcoded to `null`.
    let snapshot_properties = snapshot_node
        .facts
        .feature_properties
        .as_ref()
        .expect("snapshot usage has declared feature properties");
    assert!(snapshot_properties.is_portion);
    assert_eq!(
        snapshot_properties.portion_kind.as_deref(),
        Some("snapshot")
    );

    let timeslice_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::windowSlice"))
        .expect("timeslice occurrence usage materialized");
    assert_eq!(
        timeslice_node.attributes.get("portionKind"),
        Some(&serde_json::json!("timeslice"))
    );
    let timeslice_properties = timeslice_node
        .facts
        .feature_properties
        .as_ref()
        .expect("timeslice usage has declared feature properties");
    assert!(timeslice_properties.is_portion);
    assert_eq!(
        timeslice_properties.portion_kind.as_deref(),
        Some("timeslice")
    );

    let happening_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::happening"))
        .expect("plain occurrence usage materialized");
    assert_eq!(
        happening_node
            .facts
            .feature_properties
            .as_ref()
            .map(|p| p.is_individual),
        Some(false)
    );
    assert!(
        !happening_node
            .facts
            .feature_properties
            .as_ref()
            .expect("plain usage has declared feature properties")
            .is_portion,
        "a plain `occurrence` usage (no snapshot/timeslice keyword) is not a portion"
    );
}

#[test]
fn snapshot_classifies_expose_as_namespace_or_membership_import() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Expose.sysml");
    let content = r#"
package Demo {
    part def Vehicle {
        part engine : Engine;
    }
    part def Engine;
    view v : GeneralView {
        expose Vehicle::engine;
        expose Vehicle::*;
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

    // Regression guard: `expose` previously only recorded hasExpose/exposeTargets text
    // attributes on the owning view node -- no addressable element or relationship existed for
    // it at all. `expose` is normatively an Import per its own BNF, so it should classify the
    // same way ordinary `import` statements already do.
    let membership_import = projection
        .relationships
        .iter()
        .find(|relationship| {
            relationship.metaclass == HostRelationshipMetaclass::MembershipImport
                && relationship.target.ends_with("::engine")
        })
        .expect("expose Vehicle::engine classifies as MembershipImport");
    assert!(membership_import.target.ends_with("::engine"));

    let namespace_import = projection
        .relationships
        .iter()
        .find(|relationship| relationship.metaclass == HostRelationshipMetaclass::NamespaceImport)
        .expect("expose Vehicle::* classifies as NamespaceImport");
    let namespace_import_node = projection
        .nodes
        .iter()
        .find(|node| node.semantic_id == namespace_import.target_id)
        .expect("namespace import node projected");
    assert_eq!(
        namespace_import_node.attributes.get("importTarget"),
        Some(&serde_json::json!("Vehicle::*"))
    );
}

#[test]
fn snapshot_projects_filter_condition_as_an_addressable_expression() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Filter.sysml");
    let content = r#"
package Demo {
    part def Sensor {
        attribute active : Boolean;
    }
    view v : GeneralView {
        filter active;
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

    // Regression guard: `filter`'s condition was previously only a debug-text "condition"
    // attribute; neither `add_view_filter_node` nor `build_filter_member` ever set
    // `declared_facts.own_expression`, so no addressable Expression existed for it.
    let filter_node = projection
        .nodes
        .iter()
        .find(|node| node.element_kind.as_str() == "filter")
        .expect("filter node materialized");
    let expression_id = filter_node
        .facts
        .content_expression_id
        .as_ref()
        .expect("filter node has content_expression_id");
    assert!(
        projection
            .expressions
            .iter()
            .any(|expression| &expression.semantic_id == expression_id),
        "filter condition is a real projected Expression"
    );
}

#[test]
fn snapshot_materializes_terminate_while_and_if_control_nodes() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("ControlNodes.sysml");
    let content = r#"
package Demo {
    action def Cleanup;
    action def Recover;
    action def Routine {
        while true {
            action step : Cleanup;
        }
        for i in 1..3 {
            action loopStep : Cleanup;
        }
        if true {
            perform action recoveryStep : Recover;
        } else {
            perform action fallbackStep : Recover;
        }
        terminate;
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

    // Regression guard: TerminateStmt/WhileStmt/IfStmt previously matched a silent no-op arm in
    // both build_from_action_def_body and build_from_action_usage_body and were dropped from the
    // graph entirely.
    let while_node = projection
        .nodes
        .iter()
        .find(|node| node.element_kind.as_str() == "while")
        .expect("while node materialized");
    assert!(
        projection.nodes.iter().any(|node| node.parent.as_deref()
            == Some(while_node.qualified_name.as_str())
            && node.qualified_name.ends_with("::step")),
        "while body's nested action is a child of the while node"
    );

    // Regression guard: add_for_loop previously recursed with the outer container_prefix
    // instead of its own qualified name (unlike while/if), so a nested action's qualified name
    // did not reflect the loop's nesting -- only its `parent` field did.
    let for_loop_node = projection
        .nodes
        .iter()
        .find(|node| node.element_kind.as_str() == "for loop")
        .expect("for loop node materialized");
    assert!(
        projection.nodes.iter().any(|node| node
            .qualified_name
            .starts_with(&format!("{}::", for_loop_node.qualified_name))
            && node.qualified_name.ends_with("::loopStep")),
        "for-loop body's nested action's qualified name nests under the loop node, got: {:?}",
        projection
            .nodes
            .iter()
            .map(|node| &node.qualified_name)
            .collect::<Vec<_>>()
    );

    let if_node = projection
        .nodes
        .iter()
        .find(|node| node.element_kind.as_str() == "if")
        .expect("if node materialized");
    assert_eq!(
        if_node.attributes.get("hasElse"),
        Some(&serde_json::json!(true))
    );
    assert!(
        projection.nodes.iter().any(|node| node.parent.as_deref()
            == Some(if_node.qualified_name.as_str())
            && node.qualified_name.ends_with("::recoveryStep")),
        "if then-body's nested action is a child of the if node"
    );

    // Regression guard: the else branch previously was not walked at all, only flagged via
    // hasElse.
    let else_node = projection
        .nodes
        .iter()
        .find(|node| node.element_kind.as_str() == "else")
        .expect("else node materialized");
    assert_eq!(
        else_node.parent.as_deref(),
        Some(if_node.qualified_name.as_str())
    );
    assert!(
        projection.nodes.iter().any(|node| node.parent.as_deref()
            == Some(else_node.qualified_name.as_str())
            && node.qualified_name.ends_with("::fallbackStep")),
        "if else-body's nested action is a child of the else node"
    );

    assert!(
        projection
            .nodes
            .iter()
            .any(|node| node.element_kind.as_str() == "terminate"),
        "terminate node materialized"
    );
}

#[test]
fn snapshot_classifies_concern_def_separately_from_concern_usage() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Concern.sysml");
    let content = r#"
package Demo {
    concern def SafetyConcern;
    concern c1 : SafetyConcern;
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

    // Regression guard: `concern_usage` (sysml-v2-parser) parses both `concern` and
    // `concern def` into the same AST struct; materialize_concern_usage previously always
    // tagged the resulting node "concern", so a `concern def` declaration was indistinguishable
    // from a bare `concern` usage in the graph.
    let def_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::SafetyConcern"))
        .expect("concern def node materialized");
    assert_eq!(
        def_node.element_kind.as_str(),
        "concern def",
        "concern def classifies as its own kind, not the usage kind"
    );

    let usage_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::c1"))
        .expect("concern usage node materialized");
    assert_eq!(usage_node.element_kind.as_str(), "concern");
}

#[test]
fn snapshot_resolves_typing_for_calc_constraint_and_case_usages() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("TypingTargets.sysml");
    let content = r#"
package Demo {
    calc def ComputeLoad;
    case def InspectionCase;
    case inspection : InspectionCase;
    part def Instrument {
        calc load : ComputeLoad;
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

    // Regression guard: CalcDef/CaseDef were missing from TYPING_TARGET_KINDS even though
    // SPECIALIZES_TARGET_KINDS already allowed them, so a `calc`/`case` usage's Typing edge to
    // its definition never resolved. `case` is exercised at package level here, not nested in a
    // `part def` body -- see `snapshot_materializes_case_def_and_case_usage_nested_in_part_def`
    // below for that scenario, since `PartDefBodyElement::CaseDef`/`::CaseUsage` now have a
    // dispatch arm in `graph_builder/part_def.rs`. Also found in the same investigation and
    // fixed alongside the allowlist: `materialize_case_usage` never wired a typing edge even at
    // package level, unlike its analysis/verification/use-case siblings.
    // ConstraintDef is in the same allowlist fix (kinds.rs), but a bare `constraint check : X;`
    // usage isn't exercisable here: the parser has no distinct ConstraintUsage AST node, so it
    // folds into another ConstraintDef rather than a usage with a typing edge (see
    // sysml-v2-parser's constraint_def doc comment).
    for (usage_suffix, definition_suffix) in [
        ("::Instrument::load", "::ComputeLoad"),
        ("::inspection", "::InspectionCase"),
    ] {
        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.kind.as_str() == "typing"
                    && relationship.source.ends_with(usage_suffix)
                    && relationship.target.ends_with(definition_suffix)
            }),
            "expected a resolved typing edge from {usage_suffix} to {definition_suffix}"
        );
    }
}

#[test]
fn snapshot_materializes_case_def_and_case_usage_nested_in_part_def() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("NestedCase.sysml");
    let content = r#"
package Demo {
    case def InspectionCase;
    part def Instrument {
        case def LocalCase {
            subject sys : Instrument;
        }
        case inspection : InspectionCase;
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

    // Regression guard: `PartDefBodyElement::CaseDef`/`::CaseUsage` previously had no dispatch
    // arm at all in `graph_builder/part_def.rs`, unlike the sibling `PDBE::CalcUsage` arm, so a
    // `case`/`case def` nested inside a `part def { ... }` body was silently dropped from the
    // graph entirely -- at both the definition and usage level.
    assert!(
        projection
            .nodes
            .iter()
            .any(|node| node.qualified_name.ends_with("::Instrument::LocalCase")),
        "case def nested in a part def body is now materialized"
    );
    assert!(
        projection.nodes.iter().any(|node| node
            .qualified_name
            .ends_with("::Instrument::LocalCase::sys")),
        "the nested case def's own body is walked (subject member materialized)"
    );

    let inspection_node = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::Instrument::inspection"))
        .expect("case usage nested in a part def body is now materialized");
    // Regression guard: `ElementKind` had no `From<&str>` arm for the bare "case" kind-string
    // (only "case def"), so even a package-level case usage fell to `Unknown("case")` and could
    // never become a concrete API resource.
    assert_eq!(
        inspection_node.element_kind.as_str(),
        "case",
        "nested case usage classifies as ElementKind::Case, not Unknown(\"case\")"
    );
    assert!(
        projection.relationships.iter().any(|relationship| {
            relationship.kind.as_str() == "typing"
                && relationship.source.ends_with("::Instrument::inspection")
                && relationship.target.ends_with("::InspectionCase")
        }),
        "nested case usage's typing edge to its definition resolves"
    );
}

#[test]
fn snapshot_materializes_use_case_analysis_and_verification_nested_in_part_def() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("NestedCaseFamilies.sysml");
    let content = r#"
package Demo {
    use case def InspectUseCase;
    analysis def InspectAnalysis;
    verification def InspectVerification;
    part def Instrument {
        use case def LocalUseCase {
            subject sys : Instrument;
        }
        use case localUseCase : InspectUseCase;
        analysis def LocalAnalysis {
            subject sys : Instrument;
        }
        analysis localAnalysis : InspectAnalysis;
        verification def LocalVerification {
            subject sys : Instrument;
        }
        verification localVerification : InspectVerification;
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

    // Regression guard: `PartDefBodyElement::UseCaseDef`/`::UseCaseUsage`,
    // `::AnalysisCaseDef`/`::AnalysisCaseUsage`, and `::VerificationCaseDef`/
    // `::VerificationCaseUsage` previously had no dispatch arm at all in
    // `graph_builder/part_def.rs` -- same bug class as the `case`/`case def` gap fixed
    // previously -- so any of the six nested inside a `part def { ... }` body were silently
    // dropped from the graph entirely, at both the definition and usage level.
    for (def_suffix, def_kind, usage_suffix, usage_kind, definition_target) in [
        (
            "::Instrument::LocalUseCase",
            "use case def",
            "::Instrument::localUseCase",
            "use case",
            "::InspectUseCase",
        ),
        (
            "::Instrument::LocalAnalysis",
            "analysis def",
            "::Instrument::localAnalysis",
            "analysis",
            "::InspectAnalysis",
        ),
        (
            "::Instrument::LocalVerification",
            "verification def",
            "::Instrument::localVerification",
            "verification",
            "::InspectVerification",
        ),
    ] {
        let def_node = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with(def_suffix))
            .unwrap_or_else(|| panic!("{def_suffix} nested in a part def body is materialized"));
        assert_eq!(def_node.element_kind.as_str(), def_kind);
        assert!(
            projection
                .nodes
                .iter()
                .any(|node| node.qualified_name.ends_with(&format!("{def_suffix}::sys"))),
            "the nested {def_kind}'s own body is walked (subject member materialized)"
        );

        let usage_node = projection
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with(usage_suffix))
            .unwrap_or_else(|| panic!("{usage_suffix} nested in a part def body is materialized"));
        assert_eq!(usage_node.element_kind.as_str(), usage_kind);

        assert!(
            projection.relationships.iter().any(|relationship| {
                relationship.kind.as_str() == "typing"
                    && relationship.source.ends_with(usage_suffix)
                    && relationship.target.ends_with(definition_target)
            }),
            "nested {usage_kind} usage's typing edge to {definition_target} resolves"
        );
    }
}

#[test]
fn snapshot_materializes_bare_constraint_usage_and_resolves_its_typing() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("ConstraintUsage.sysml");
    let content = r#"
package Demo {
    constraint def MassConstraint {
        in totalMass : MassValue;
    }
    constraint mc : MassConstraint;
    abstract constraint constraintChecks: MassConstraint[0..*] nonunique :> booleanEvaluations;
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

    // Regression guard: `sysml-v2-parser` 0.40.0 added `ConstraintUsage` as a distinct AST node
    // (previously bare `constraint` usages folded into `ConstraintDef`); Spec42's `ElementKind`
    // already had a `"constraint"` string arm (`ElementKind::Constraint`) but nothing ever
    // constructed it until this dispatch existed.
    let mc = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::mc"))
        .expect("bare constraint usage is materialized");
    assert_eq!(
        mc.element_kind.as_str(),
        "constraint",
        "bare constraint usage classifies as ElementKind::Constraint"
    );
    assert!(
        projection.relationships.iter().any(|relationship| {
            relationship.kind.as_str() == "typing"
                && relationship.source.ends_with("::mc")
                && relationship.target.ends_with("::MassConstraint")
        }),
        "bare constraint usage's typing edge to its definition resolves"
    );

    // Regression guard: the real-library `constraintChecks` shape (abstract + typing + trailing
    // multiplicity + nonunique + subsetting, all def-less) must also materialize, not merely the
    // simple typed form.
    assert!(
        projection
            .nodes
            .iter()
            .any(|node| node.qualified_name.ends_with("::constraintChecks")),
        "the real-library constraintChecks shape is materialized"
    );
}

#[test]
fn snapshot_materializes_enumerated_values_and_resolves_enum_usage_typing() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Enumeration.sysml");
    let content = r#"
package Demo {
    enum def Status {
        active;
        inactive = 1;
        degraded { doc /* transient */ }
    }
    enum current : Status;
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

    // `sysml-v2-parser` 0.39.0 gives each enumerated value a real span (previously a bare
    // `String`, so it could never become an addressable element). Each value now materializes as
    // its own child node of the `EnumDef`, regardless of which trailing form (bare `;`, `= expr`,
    // or inline `{ ... }`) it uses.
    for value_name in ["active", "inactive", "degraded"] {
        let node = projection
            .nodes
            .iter()
            .find(|node| {
                node.qualified_name
                    .ends_with(&format!("::Status::{value_name}"))
            })
            .unwrap_or_else(|| panic!("enumerated value {value_name} materialized"));
        assert_eq!(node.element_kind.as_str(), "enumerated value");
        assert_eq!(node.parent.as_deref(), Some("Demo::Status"));
    }

    // `enum current : Status;` resolves its Typing edge like any other usage (EnumDef was already
    // a valid typing target before this round; this exercises the previously-unwired package-level
    // `PackageBodyElement::EnumerationUsage` dispatch, which used to be a no-op).
    assert!(
        projection.relationships.iter().any(|relationship| {
            relationship.kind.as_str() == "typing"
                && relationship.source.ends_with("::current")
                && relationship.target.ends_with("::Status")
        }),
        "expected a resolved typing edge from ::current to ::Status"
    );
}

#[test]
fn snapshot_projects_transition_trigger_guard_effect_as_addressable_children() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Transition.sysml");
    let content = r#"
package Demo {
    item def Fault;
    state def Health {
        state nominal;
        state degraded;
        state critical;
        transition t1 first nominal accept sig : Fault if 1 < 2 do assign x := 1 then degraded;
        transition t2 first degraded do action recover then critical;
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

    let t1 = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::t1"))
        .expect("t1 transition");
    let t2 = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name.ends_with("::t2"))
        .expect("t2 transition");

    // t1 has trigger, guard, and effect; t2 has only effect (no accept, no if).
    for (transition, expected_children) in
        [(t1, &["trigger", "guard", "effect"][..]), (t2, &["effect"])]
    {
        let children: Vec<_> = projection
            .nodes
            .iter()
            .filter(|node| node.parent.as_deref() == Some(transition.qualified_name.as_str()))
            .collect();
        assert_eq!(
            children.len(),
            expected_children.len(),
            "unexpected children for {}",
            transition.qualified_name
        );
        for suffix in expected_children {
            let child = children
                .iter()
                .find(|child| child.qualified_name.ends_with(&format!("::{suffix}")))
                .unwrap_or_else(|| panic!("{suffix} child of {}", transition.qualified_name));
            let membership = projection
                .relationships
                .iter()
                .find(|relationship| relationship.target_id == child.semantic_id)
                .unwrap_or_else(|| panic!("membership relationship for {}", child.qualified_name));
            assert_eq!(
                membership.membership_kind,
                Some(HostMembershipKind::TransitionFeatureMembership),
                "{}",
                child.qualified_name
            );
        }
    }

    // Trigger: `accept sig : Fault` is a typed payload clause, carried on attributes (baseline
    // slice — not yet a resolved AcceptActionUsage), not via content_expression_id.
    let trigger = projection
        .nodes
        .iter()
        .find(|node| {
            node.parent.as_deref() == Some(t1.qualified_name.as_str())
                && node.qualified_name.ends_with("::trigger")
        })
        .expect("t1 trigger");
    assert_eq!(
        trigger
            .attributes
            .get("payloadName")
            .and_then(|v| v.as_str()),
        Some("sig")
    );
    assert_eq!(
        trigger
            .attributes
            .get("payloadType")
            .and_then(|v| v.as_str()),
        Some("Fault")
    );
    assert!(trigger.facts.content_expression_id.is_none());

    // Guard: `1 < 2` round-trips through the unmodified `declared_expression()` converter into a
    // real, addressable Expression — not a debug string.
    let guard = projection
        .nodes
        .iter()
        .find(|node| {
            node.parent.as_deref() == Some(t1.qualified_name.as_str())
                && node.qualified_name.ends_with("::guard")
        })
        .expect("t1 guard");
    let guard_expression_id = guard
        .facts
        .content_expression_id
        .as_deref()
        .expect("guard has content_expression_id");
    let guard_expression = projection
        .expressions
        .iter()
        .find(|expression| expression.semantic_id == guard_expression_id)
        .expect("guard expression is projected");
    assert_eq!(guard_expression.kind, "binary");
    assert_eq!(guard_expression.operator.as_deref(), Some("<"));
    assert_eq!(guard_expression.operand_ids.len(), 2);
    let left = projection
        .expressions
        .iter()
        .find(|expression| expression.semantic_id == guard_expression.operand_ids[0])
        .expect("left operand projected");
    assert_eq!(left.kind, "integerLiteral");

    // Effect: t1's `assign x := 1` and t2's `action recover` (the no-span `Perform` case) both
    // materialize without panicking, falling back to the owning transition's own range.
    let t1_effect = projection
        .nodes
        .iter()
        .find(|node| {
            node.parent.as_deref() == Some(t1.qualified_name.as_str())
                && node.qualified_name.ends_with("::effect")
        })
        .expect("t1 effect");
    assert_eq!(
        t1_effect
            .attributes
            .get("effectExpression")
            .and_then(|v| v.as_str()),
        Some("assign x := 1")
    );

    let t2_effect = projection
        .nodes
        .iter()
        .find(|node| {
            node.parent.as_deref() == Some(t2.qualified_name.as_str())
                && node.qualified_name.ends_with("::effect")
        })
        .expect("t2 effect");
    assert_eq!(
        t2_effect
            .attributes
            .get("effectExpression")
            .and_then(|v| v.as_str()),
        Some("action recover")
    );
    assert_eq!(t2_effect.range, t2.range);
}
