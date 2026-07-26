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
fn lsp_feature_inspector_accepts_standard_and_legacy_uri_shapes() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_request_shape.sysml";
    let content = "package P {\n  part def Engine;\n}\n";
    session.initialize_default("feature_inspector_request_shape");
    session.did_open(uri, content, 1);
    session.barrier();

    let standard = inspect(&mut session, uri, 1, 12);
    assert_eq!(
        standard["result"]["containingElement"]["name"].as_str(),
        Some("Engine")
    );

    let legacy = session.request(
        "sysml/featureInspector",
        serde_json::json!({
            "uri": uri,
            "position": { "line": 1, "character": 12 }
        }),
    );
    assert_eq!(
        legacy["result"]["containingElement"]["name"].as_str(),
        Some("Engine")
    );

    let transition = session.request(
        "sysml/featureInspector",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "uri": uri,
            "position": { "line": 1, "character": 12 }
        }),
    );
    assert_eq!(
        transition["result"]["containingElement"]["name"].as_str(),
        Some("Engine")
    );

    let positional_artifact = session.request(
        "sysml/featureInspector",
        serde_json::json!([
            {
                "textDocument": { "uri": uri },
                "uri": uri,
                "position": { "line": 1, "character": 12 }
            },
            null
        ]),
    );
    assert_eq!(
        positional_artifact["result"]["containingElement"]["name"].as_str(),
        Some("Engine")
    );
}

#[test]
fn lsp_feature_inspector_classifies_keywords_with_structured_language_help() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_keyword_help.sysml";
    let content = "package P {\n  part def Engine;\n}\n";
    session.initialize_default("feature_inspector_keyword_help");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = inspect(&mut session, uri, 1, 3);
    assert_eq!(
        response["result"]["selection"]["kind"].as_str(),
        Some("keyword")
    );
    assert_eq!(
        response["result"]["selection"]["text"].as_str(),
        Some("part")
    );
    assert_eq!(
        response["result"]["selection"]["range"],
        serde_json::json!({
            "start": { "line": 1, "character": 2 },
            "end": { "line": 1, "character": 6 }
        })
    );
    assert_eq!(
        response["result"]["languageHelp"]["keyword"].as_str(),
        Some("part")
    );
    assert!(response["result"]["languageHelp"]["description"]
        .as_str()
        .is_some_and(|description| !description.is_empty()));
    assert!(response["result"]["languageHelp"]["syntax"]
        .as_str()
        .is_some_and(|syntax| syntax.contains("part def")));
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
    let element = &response["result"]["containingElement"];
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
    let targets = response["result"]["containingElement"]["typing"]["targets"]
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
    let typing = &response["result"]["containingElement"]["typing"];
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
    let specialization = &response["result"]["containingElement"]["specialization"];
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
        response["result"]["containingElement"]["name"].as_str(),
        Some("output")
    );
    assert_eq!(
        response["result"]["containingElement"]["type"].as_str(),
        Some("port")
    );
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
    let outgoing = motor["result"]["containingElement"]["outgoingRelationships"]
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
    let incoming = engine["result"]["containingElement"]["incomingRelationships"]
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
    assert!(whitespace["result"]["containingElement"].is_null());

    let missing = inspect(
        &mut session,
        "file:///feature_inspector_missing.sysml",
        0,
        0,
    );
    assert!(missing["result"]["containingElement"].is_null());
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
    let feature_element = &feature["result"]["containingElement"];
    assert_eq!(feature_element["name"].as_str(), Some("myFeature"));
    assert_eq!(feature_element["type"].as_str(), Some("feature decl"));
    assert_eq!(
        feature_element["typing"]["status"].as_str(),
        Some("notApplicable")
    );
    assert_eq!(
        feature_element["specialization"]["status"].as_str(),
        Some("notApplicable")
    );

    let classifier = inspect(&mut session, uri, 2, 10);
    let classifier_element = &classifier["result"]["containingElement"];
    assert_eq!(classifier_element["name"].as_str(), Some("VehicleClass"));
    assert_eq!(classifier_element["type"].as_str(), Some("classifier decl"));
    assert_eq!(
        classifier_element["typing"]["status"].as_str(),
        Some("notApplicable")
    );
}

#[test]
fn lsp_feature_inspector_distinguishes_element_reference_value_and_unit_tokens() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_selection_kinds.sysml";
    let content = concat!(
        "package P {\n",
        "  part def RPLIDARC1;\n",
        "  part def RobotLidar :> RPLIDARC1 {\n",
        "    attribute scanRate : FrequencyValue = 10 [Hz];\n",
        "  }\n",
        "}\n"
    );
    session.initialize_default("feature_inspector_selection_kinds");
    session.did_open(uri, content, 1);
    session.barrier();

    let definition = inspect(&mut session, uri, 2, 13);
    assert_eq!(
        definition["result"]["selection"]["kind"].as_str(),
        Some("element")
    );
    let element = &definition["result"]["containingElement"];
    assert_eq!(element["name"].as_str(), Some("RobotLidar"));
    assert_eq!(element["role"].as_str(), Some("definition"));
    assert!(element["declaration"]
        .as_str()
        .is_some_and(|declaration| declaration.contains("part def RobotLidar")));

    let reference = inspect(&mut session, uri, 2, 27);
    assert_eq!(
        reference["result"]["selection"]["kind"].as_str(),
        Some("reference")
    );
    assert_eq!(
        reference["result"]["referencedElement"]["name"].as_str(),
        Some("RPLIDARC1")
    );
    assert_eq!(
        reference["result"]["containingElement"]["name"].as_str(),
        Some("RobotLidar")
    );

    let usage = inspect(&mut session, uri, 3, 17);
    assert_eq!(
        usage["result"]["selection"]["kind"].as_str(),
        Some("element")
    );
    assert_eq!(
        usage["result"]["containingElement"]["role"].as_str(),
        Some("usage")
    );
    assert_eq!(
        usage["result"]["containingElement"]["name"].as_str(),
        Some("scanRate")
    );

    let value = inspect(&mut session, uri, 3, 42);
    assert_eq!(value["result"]["selection"]["kind"].as_str(), Some("value"));
    assert_eq!(value["result"]["selection"]["text"].as_str(), Some("10"));
    assert_eq!(
        value["result"]["selection"]["range"],
        serde_json::json!({
            "start": { "line": 3, "character": 42 },
            "end": { "line": 3, "character": 44 }
        })
    );

    let unit = inspect(&mut session, uri, 3, 46);
    assert_eq!(unit["result"]["selection"]["kind"].as_str(), Some("unit"));
    assert_eq!(unit["result"]["selection"]["text"].as_str(), Some("Hz"));
    assert_eq!(
        unit["result"]["selection"]["range"],
        serde_json::json!({
            "start": { "line": 3, "character": 46 },
            "end": { "line": 3, "character": 48 }
        })
    );
}

#[test]
fn lsp_feature_inspector_handles_other_and_unresolved_selections_without_false_help() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_other_tokens.sysml";
    let content = "package P {\n  Part def WrongCase;\n  part broken : MissingPart;\n}\n";
    session.initialize_default("feature_inspector_other_tokens");
    session.did_open(uri, content, 1);
    session.barrier();

    let case_mismatch = inspect(&mut session, uri, 1, 3);
    assert_ne!(
        case_mismatch["result"]["selection"]["kind"].as_str(),
        Some("keyword")
    );
    assert!(case_mismatch["result"]["languageHelp"].is_null());

    let unresolved = inspect(&mut session, uri, 2, 18);
    assert_ne!(
        unresolved["result"]["selection"]["kind"].as_str(),
        Some("reference")
    );
    assert!(unresolved["result"]["referencedElement"].is_null());

    let punctuation = inspect(&mut session, uri, 2, 27);
    assert_eq!(
        punctuation["result"]["selection"]["kind"].as_str(),
        Some("other")
    );
    assert!(punctuation["result"]["languageHelp"].is_null());
}

#[test]
fn lsp_feature_inspector_acceptance_covers_assert_and_software_queue() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_acceptance.sysml";
    let content =
        "package P {\n  part def SoftwareQueue {\n    assert constraint { true }\n  }\n}\n";
    session.initialize_default("feature_inspector_acceptance");
    session.did_open(uri, content, 1);
    session.barrier();

    let software_queue = inspect(&mut session, uri, 1, 13);
    assert_eq!(
        software_queue["result"]["selection"]["kind"].as_str(),
        Some("element")
    );
    assert_eq!(
        software_queue["result"]["containingElement"]["name"].as_str(),
        Some("SoftwareQueue")
    );
    assert_eq!(
        software_queue["result"]["containingElement"]["role"].as_str(),
        Some("definition")
    );

    let assert_keyword = inspect(&mut session, uri, 2, 6);
    assert_eq!(
        assert_keyword["result"]["selection"]["kind"].as_str(),
        Some("keyword")
    );
    assert_eq!(
        assert_keyword["result"]["languageHelp"]["keyword"].as_str(),
        Some("assert")
    );
    assert!(assert_keyword["result"]["referencedElement"].is_null());
}
