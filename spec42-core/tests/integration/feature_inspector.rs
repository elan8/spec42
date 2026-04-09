//! sysml/featureInspector integration tests.

use super::harness::TestSession;

fn inspect(session: &mut TestSession, uri: &str, line: u32, character: u32) -> serde_json::Value {
    session.request(
        "sysml/featureInspector",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        }),
    )
}

#[test]
fn lsp_feature_inspector_resolves_same_file_typing() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_same_file.sysml";
    let content = "package P {\n  part def Engine;\n  part motor : Engine;\n}\n";
    session.initialize_default("feature_inspector_same_file");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = inspect(&mut session, uri, 2, 7);
    let element = &response["result"]["element"];
    assert_eq!(element["name"].as_str(), Some("motor"));
    assert_eq!(element["typing"]["status"].as_str(), Some("resolved"));
    let targets = element["typing"]["targets"]
        .as_array()
        .expect("typing targets");
    assert!(
        targets
            .iter()
            .any(|target| target["name"].as_str() == Some("Engine")),
        "expected Engine typing target, got {targets:#?}"
    );
}

#[test]
fn lsp_feature_inspector_resolves_cross_file_typing() {
    let mut session = TestSession::new();
    let defs_uri = "file:///feature_inspector_defs.sysml";
    let defs = "package R {\n  requirement def EnduranceReq;\n}\n";
    let usage_uri = "file:///feature_inspector_usage.sysml";
    let usage = "package R {\n  requirement enduranceCheck : EnduranceReq;\n}\n";
    session.initialize_default("feature_inspector_cross_file");
    session.did_open(defs_uri, defs, 1);
    session.did_open(usage_uri, usage, 2);
    session.barrier();

    let response = inspect(&mut session, usage_uri, 1, 14);
    let targets = response["result"]["element"]["typing"]["targets"]
        .as_array()
        .expect("typing targets");
    assert!(
        targets.iter().any(|target| {
            target["name"].as_str() == Some("EnduranceReq")
                && target["uri"].as_str() == Some(defs_uri)
        }),
        "expected cross-file EnduranceReq target in defs file, got {targets:#?}"
    );
}

#[test]
fn lsp_feature_inspector_reports_unresolved_typing() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_unresolved.sysml";
    let content = "package R {\n  requirement brokenCheck : MissingReq;\n}\n";
    session.initialize_default("feature_inspector_unresolved");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = inspect(&mut session, uri, 1, 15);
    let typing = &response["result"]["element"]["typing"];
    assert_eq!(typing["status"].as_str(), Some("unresolved"));
    assert_eq!(
        typing["targets"].as_array().map(|targets| targets.len()),
        Some(0)
    );
}

#[test]
fn lsp_feature_inspector_reports_specialization_targets() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_specialization.sysml";
    let content = "package P {\n  part def Vehicle;\n  part def Drone :> Vehicle;\n}\n";
    session.initialize_default("feature_inspector_specialization");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = inspect(&mut session, uri, 2, 12);
    let specialization = &response["result"]["element"]["specialization"];
    assert_eq!(specialization["status"].as_str(), Some("resolved"));
    let targets = specialization["targets"]
        .as_array()
        .expect("specialization targets");
    assert!(
        targets
            .iter()
            .any(|target| target["name"].as_str() == Some("Vehicle")),
        "expected Vehicle specialization target, got {targets:#?}"
    );
}

#[test]
fn lsp_feature_inspector_uses_deepest_node_at_position() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_deepest.sysml";
    let content = "package P {\n  part def Engine {\n    port output;\n  }\n}\n";
    session.initialize_default("feature_inspector_deepest");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = inspect(&mut session, uri, 2, 9);
    assert_eq!(
        response["result"]["element"]["name"].as_str(),
        Some("output")
    );
    assert_eq!(response["result"]["element"]["type"].as_str(), Some("port"));
}

#[test]
fn lsp_feature_inspector_returns_direct_relationships_without_contains() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_relationships.sysml";
    let content = "package P {\n  part def Engine;\n  part motor : Engine;\n}\n";
    session.initialize_default("feature_inspector_relationships");
    session.did_open(uri, content, 1);
    session.barrier();

    let motor = inspect(&mut session, uri, 2, 7);
    let outgoing = motor["result"]["element"]["outgoingRelationships"]
        .as_array()
        .expect("motor outgoing relationships");
    assert!(
        outgoing
            .iter()
            .any(|relationship| relationship["type"].as_str() == Some("typing")),
        "expected typing outgoing relationship, got {outgoing:#?}"
    );
    assert!(
        outgoing
            .iter()
            .all(|relationship| relationship["type"].as_str() != Some("contains")),
        "contains edges must not appear in inspector relationships: {outgoing:#?}"
    );

    let engine = inspect(&mut session, uri, 1, 12);
    let incoming = engine["result"]["element"]["incomingRelationships"]
        .as_array()
        .expect("Engine incoming relationships");
    assert!(
        incoming.iter().any(
            |relationship| relationship["type"].as_str() == Some("typing")
                && relationship["peer"]["name"].as_str() == Some("motor")
        ),
        "expected incoming typing relationship from motor, got {incoming:#?}"
    );
}

#[test]
fn lsp_feature_inspector_returns_null_for_whitespace_and_missing_documents() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_empty.sysml";
    let content = "package P {\n  part def Engine;\n}\n";
    session.initialize_default("feature_inspector_empty");
    session.did_open(uri, content, 1);
    session.barrier();

    let whitespace = inspect(&mut session, uri, 3, 0);
    assert!(whitespace["result"]["element"].is_null());

    let missing = inspect(
        &mut session,
        "file:///feature_inspector_missing.sysml",
        0,
        0,
    );
    assert!(missing["result"]["element"].is_null());
}

#[test]
fn lsp_feature_inspector_surfaces_feature_and_classifier_decls_without_resolution_intent() {
    let mut session = TestSession::new();
    let uri = "file:///feature_classifier_inspector.sysml";
    let content = "package P {\n  feature myFeature : BaseFeature;\n  class VehicleClass;\n}\n";
    session.initialize_default("feature_classifier_inspector");
    session.did_open(uri, content, 1);
    session.barrier();

    let feature = inspect(&mut session, uri, 1, 12);
    let feature_element = &feature["result"]["element"];
    assert_eq!(feature_element["name"].as_str(), Some("myFeature"));
    assert_eq!(feature_element["type"].as_str(), Some("feature decl"));
    assert_eq!(feature_element["typing"]["status"].as_str(), Some("notApplicable"));
    assert_eq!(
        feature_element["specialization"]["status"].as_str(),
        Some("notApplicable")
    );

    let classifier = inspect(&mut session, uri, 2, 10);
    let classifier_element = &classifier["result"]["element"];
    assert_eq!(classifier_element["name"].as_str(), Some("VehicleClass"));
    assert_eq!(classifier_element["type"].as_str(), Some("classifier decl"));
    assert_eq!(
        classifier_element["typing"]["status"].as_str(),
        Some("notApplicable")
    );
}
