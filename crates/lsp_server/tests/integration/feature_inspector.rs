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

/// The inspector's typing intent/targets come from the typed declared typing fact, not the legacy
/// `*Type` attribute projections. Exercise constructs whose projection keys were `useCaseType`,
/// `concernType`, `stateType`, and `viewType` in both the resolved and unresolved case: the typed
/// fact must drive exactly the same observable `typing.status`/`typing.targets` as before.
#[test]
fn lsp_feature_inspector_typing_comes_from_typed_facts_for_non_part_constructs() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_typed_facts.sysml";
    let content = concat!(
        "package P {\n",
        "  use case def Checkout;\n",
        "  concern def Safety;\n",
        "  state def Idle;\n",
        "  view def Overview;\n",
        "  use case buy : Checkout;\n",
        "  concern hazards : Safety;\n",
        "  state waiting : Idle;\n",
        "  view summary : Overview;\n",
        "  use case broken : MissingUseCaseDef;\n",
        "  concern brokenConcern : MissingConcernDef;\n",
        "  state brokenState : MissingStateDef;\n",
        "  view brokenView : MissingViewDef;\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_typed_facts");
    session.did_open(uri, content, 1);
    session.barrier();

    for (line, character, name, target) in [
        (5u32, 12u32, "buy", "Checkout"),
        (6, 12, "hazards", "Safety"),
        (7, 10, "waiting", "Idle"),
        (8, 9, "summary", "Overview"),
    ] {
        let response = inspect(&mut session, uri, line, character);
        let element = &response["result"]["containingElement"];
        assert_eq!(element["name"].as_str(), Some(name));
        assert_eq!(
            element["typing"]["status"].as_str(),
            Some("resolved"),
            "{name} should resolve from its typed typing fact, got {element:#?}"
        );
        let targets = element["typing"]["targets"]
            .as_array()
            .expect("typing targets");
        assert!(
            targets
                .iter()
                .any(|candidate| candidate["name"].as_str() == Some(target)),
            "expected {target} typing target for {name}, got {targets:#?}"
        );
    }

    for (line, character, name) in [
        (9u32, 12u32, "broken"),
        (10, 12, "brokenConcern"),
        (11, 10, "brokenState"),
        (12, 9, "brokenView"),
    ] {
        let response = inspect(&mut session, uri, line, character);
        let element = &response["result"]["containingElement"];
        assert_eq!(element["name"].as_str(), Some(name));
        assert_eq!(
            element["typing"]["status"].as_str(),
            Some("unresolved"),
            "{name} declared a typing that cannot resolve, got {element:#?}"
        );
        assert_eq!(
            element["typing"]["targets"]
                .as_array()
                .map(|targets| targets.len()),
            Some(0),
            "{name} must not present a guessed typing target"
        );
    }
}

#[test]
fn lsp_feature_inspector_resolves_cross_file_typing() {
    let mut session = TestSession::new();
    let defs_uri = "file:///feature_inspector_defs.sysml";
    let defs = "package Defs {\n  requirement def EnduranceReq;\n}\n";
    let usage_uri = "file:///feature_inspector_usage.sysml";
    let usage =
        "package Usage {\n  import Defs::*;\n  requirement enduranceCheck : EnduranceReq;\n}\n";
    session.initialize_default("feature_inspector_cross_file");
    session.did_open(defs_uri, defs, 1);
    session.did_open(usage_uri, usage, 2);
    session.barrier();

    let response = inspect(&mut session, usage_uri, 2, 14);
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

/// Two documents that both declare `package P` declare two packages, and the immutable
/// publication keeps them apart -- which is what makes each separately addressable. An
/// unqualified name therefore does not reach the other document's members, and the inspector
/// reports that rather than binding to a sibling that happens to share a spelling. The same
/// publication already answers go-to-definition this way; this pins the inspector agreeing.
#[test]
fn lsp_feature_inspector_reports_an_unqualified_name_across_same_named_packages_as_unresolved() {
    let mut session = TestSession::new();
    let defs_uri = "file:///feature_inspector_same_package_defs.sysml";
    let usage_uri = "file:///feature_inspector_same_package_usage.sysml";
    session.initialize_default("feature_inspector_same_package");
    session.did_open(
        defs_uri,
        "package R {\n  requirement def EnduranceReq;\n}\n",
        1,
    );
    session.did_open(
        usage_uri,
        "package R {\n  requirement enduranceCheck : EnduranceReq;\n}\n",
        2,
    );
    session.barrier();

    let typing = &inspect(&mut session, usage_uri, 1, 14)["result"]["containingElement"]["typing"];
    assert_eq!(typing["status"].as_str(), Some("unresolved"));
    assert_eq!(
        typing["targets"].as_array().map(|targets| targets.len()),
        Some(0)
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
fn lsp_feature_inspector_exposes_effective_semantics_and_inherited_features() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_effective_semantics.sysml";
    let content = concat!(
        "package P {\n",
        "  item def Signal;\n",
        "  part def Wheel;\n",
        "  part def Vehicle {\n",
        "    doc /* A reusable vehicle definition. */\n",
        "    part wheel[4] : Wheel;\n",
        "    part spare[0..*] : Wheel;\n",
        "  }\n",
        "  part def Rover :> Vehicle {\n",
        "    part :>> wheel[4];\n",
        "  }\n",
        "  part rover : Rover;\n",
        "  action def Process {\n",
        "    in command : Signal[0..*];\n",
        "  }\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_effective_semantics");
    session.did_open(uri, content, 1);
    session.barrier();

    let vehicle = inspect(&mut session, uri, 3, 12);
    let vehicle_element = &vehicle["result"]["containingElement"];
    assert_eq!(vehicle["result"]["version"].as_u64(), Some(2));
    assert_eq!(
        vehicle_element["documentation"].as_str(),
        Some("A reusable vehicle definition.")
    );

    let command = inspect(&mut session, uri, 13, 9);
    let command_element = &command["result"]["containingElement"];
    assert_eq!(command_element["direction"].as_str(), Some("in"));

    let base_wheel = inspect(&mut session, uri, 5, 11);
    assert_eq!(
        base_wheel["result"]["containingElement"]["multiplicity"].as_str(),
        Some("4")
    );
    let spare = inspect(&mut session, uri, 6, 11);
    assert_eq!(
        spare["result"]["containingElement"]["multiplicity"].as_str(),
        Some("0..*")
    );

    let rover_def = inspect(&mut session, uri, 8, 12);
    let inherited = rover_def["result"]["containingElement"]["inheritedFeatures"]
        .as_array()
        .expect("Rover inherited features");
    assert!(
        inherited.iter().any(|entry| {
            entry["feature"]["name"].as_str() == Some("spare")
                && entry["declaredIn"]["name"].as_str() == Some("Vehicle")
        }),
        "expected Vehicle::spare as inherited feature, got {inherited:#?}"
    );
    assert!(
        inherited
            .iter()
            .all(|entry| entry["feature"]["name"].as_str() != Some("wheel")),
        "the local wheel redefinition must suppress the inherited wheel: {inherited:#?}"
    );

    let wheel = inspect(&mut session, uri, 9, 14);
    let wheel_element = &wheel["result"]["containingElement"];
    assert_eq!(
        wheel_element["redefinition"]["status"].as_str(),
        Some("resolved")
    );
    assert_eq!(
        wheel_element["redefinition"]["targets"][0]["name"].as_str(),
        Some("wheel")
    );
    assert_eq!(
        wheel_element["effectiveTyping"]["status"].as_str(),
        Some("resolved")
    );
    assert_eq!(
        wheel_element["effectiveTyping"]["targets"][0]["name"].as_str(),
        Some("Wheel")
    );

    let rover_usage = inspect(&mut session, uri, 11, 8);
    let usage_inherited = rover_usage["result"]["containingElement"]["inheritedFeatures"]
        .as_array()
        .expect("rover usage effective features");
    // `Rover` redefines `wheel` anonymously, so the named declaration is still Vehicle's. The
    // publication reports the type that declares the feature rather than the nearest type that
    // redefines it under no name of its own.
    assert!(
        usage_inherited.iter().any(|entry| {
            entry["feature"]["name"].as_str() == Some("wheel")
                && entry["declaredIn"]["name"].as_str() == Some("Vehicle")
        }),
        "typed usages should expose effective features from their definition: {usage_inherited:#?}"
    );
}

#[test]
fn lsp_feature_inspector_exposes_subsetting_as_a_resolved_semantic_relation() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_subsetting.sysml";
    let content = concat!(
        "package P {\n",
        "  part def Sensor;\n",
        "  part sensor : Sensor;\n",
        "  part selectedSensor subsets P::sensor;\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_subsetting");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = inspect(&mut session, uri, 3, 9);
    let subsetting = &response["result"]["containingElement"]["subsetting"];
    assert_eq!(subsetting["status"].as_str(), Some("resolved"));
    assert_eq!(subsetting["targets"][0]["name"].as_str(), Some("sensor"));
    assert_eq!(
        response["result"]["containingElement"]["effectiveTyping"]["targets"][0]["name"].as_str(),
        Some("Sensor")
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
        Some("PortUsage")
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
    assert_eq!(feature_element["type"].as_str(), Some("Feature"));
    // A KerML `feature` declares a typing like any other feature. The graph-backed inspector
    // reported `notApplicable` here because its declared-fact projection had no entry for this
    // production; the publication reports what the author wrote, which did not resolve.
    assert_eq!(
        feature_element["typing"]["status"].as_str(),
        Some("unresolved")
    );
    assert_eq!(
        feature_element["typing"]["targets"]
            .as_array()
            .map(|targets| targets.len()),
        Some(0)
    );
    assert_eq!(
        feature_element["specialization"]["status"].as_str(),
        Some("notApplicable")
    );

    let classifier = inspect(&mut session, uri, 2, 10);
    let classifier_element = &classifier["result"]["containingElement"];
    assert_eq!(classifier_element["name"].as_str(), Some("VehicleClass"));
    assert_eq!(classifier_element["type"].as_str(), Some("Class"));
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
        reference["result"]["referenced"]["status"].as_str(),
        Some("resolved")
    );
    assert_eq!(
        reference["result"]["referenced"]["element"]["name"].as_str(),
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
    // A reference the publication could not settle keeps its own outcome; it is neither absent
    // nor a resolved target the inspector could show.
    assert_eq!(
        unresolved["result"]["referenced"]["status"].as_str(),
        Some("unresolved")
    );

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
    assert_eq!(
        assert_keyword["result"]["referenced"]["status"].as_str(),
        Some("none")
    );
}

/// Never select the first candidate of an ambiguous reference. The inspector reports the outcome
/// and every candidate, and publishes no target the publication refused to choose.
#[test]
fn lsp_feature_inspector_reports_an_ambiguous_typing_without_choosing_a_candidate() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_ambiguous.sysml";
    let content = concat!(
        "package P {\n",
        "  package A { part def Shared; }\n",
        "  package B { part def Shared; }\n",
        "  package C {\n",
        "    import A::*;\n",
        "    import B::*;\n",
        "    part unit : Shared;\n",
        "  }\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_ambiguous");
    session.did_open(uri, content, 1);
    session.barrier();

    let typing = &inspect(&mut session, uri, 6, 10)["result"]["containingElement"]["typing"];
    assert_eq!(typing["status"].as_str(), Some("ambiguous"));
    assert_eq!(
        typing["targets"].as_array().map(|targets| targets.len()),
        Some(0),
        "an ambiguous family must publish no chosen target: {typing:#?}"
    );
    assert_eq!(
        typing["candidates"]
            .as_array()
            .map(|candidates| candidates.len()),
        Some(2),
        "every candidate must remain visible: {typing:#?}"
    );
}

/// Every published evaluation state reaches the wire as its own variant, so a missing value can
/// never be read as a successful evaluation.
#[test]
fn lsp_feature_inspector_projects_each_published_evaluation_state() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_evaluation.sysml";
    let content = concat!(
        "package P {\n",
        "  part def Empty;\n",
        "  attribute plain = 1;\n",
        "  attribute computed = 2 + 3;\n",
        "  attribute weighed = 1200 [kg];\n",
        "  attribute opaque;\n",
        "  attribute derived = opaque;\n",
        "  attribute divided = 1 / 0;\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_evaluation");
    session.did_open(uri, content, 1);
    session.barrier();

    let evaluation = |session: &mut TestSession, line: u32, character: u32| {
        inspect(session, uri, line, character)["result"]["containingElement"]["evaluation"].clone()
    };

    // An element with no expression is affirmatively not applicable, not a valueless success.
    assert_eq!(
        evaluation(&mut session, 1, 12)["state"].as_str(),
        Some("notApplicable")
    );

    let plain = evaluation(&mut session, 2, 13);
    assert_eq!(plain["state"].as_str(), Some("literal"));
    assert_eq!(plain["value"].as_i64(), Some(1));
    assert!(plain["unit"].is_null());

    let computed = evaluation(&mut session, 3, 13);
    assert_eq!(computed["state"].as_str(), Some("evaluated"));
    assert_eq!(computed["value"].as_i64(), Some(5));

    let weighed = evaluation(&mut session, 4, 13);
    assert_eq!(weighed["state"].as_str(), Some("literal"));
    assert_eq!(weighed["value"].as_i64(), Some(1200));
    assert_eq!(weighed["unit"].as_str(), Some("kg"));

    let derived = evaluation(&mut session, 6, 13);
    assert_eq!(
        derived["state"].as_str(),
        Some("nonConstant"),
        "an operand with no constant value is correctly not constant: {derived:#?}"
    );
    assert!(derived["value"].is_null());

    let divided = evaluation(&mut session, 7, 13);
    assert_eq!(divided["state"].as_str(), Some("failed"));
    assert_eq!(divided["reason"].as_str(), Some("divisionByZero"));
    assert!(divided["value"].is_null());
}

/// The verdict channel and the value channel are separate, and both sides of a verdict are
/// reported. A constraint whose expression did not settle is unsettled, not failing.
#[test]
fn lsp_feature_inspector_reports_verdicts_apart_from_values() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_verdict.sysml";
    let content = concat!(
        "package P {\n",
        "  constraint holds { true }\n",
        "  constraint fails { false }\n",
        "  constraint unsettled { missing }\n",
        "  attribute notAVerdict = true;\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_verdict");
    session.did_open(uri, content, 1);
    session.barrier();

    let analysis = |session: &mut TestSession, line: u32, character: u32| {
        inspect(session, uri, line, character)["result"]["containingElement"]["analysis"].clone()
    };

    let holds = analysis(&mut session, 1, 15);
    assert_eq!(holds["state"].as_str(), Some("verdict"));
    assert_eq!(holds["passed"].as_bool(), Some(true));

    let fails = analysis(&mut session, 2, 15);
    assert_eq!(fails["state"].as_str(), Some("verdict"));
    assert_eq!(fails["passed"].as_bool(), Some(false));

    let unsettled = analysis(&mut session, 3, 15);
    assert_eq!(
        unsettled["state"].as_str(),
        Some("unsettled"),
        "an unsettled constraint must not read as a failing verdict: {unsettled:#?}"
    );

    assert_eq!(
        analysis(&mut session, 4, 13)["state"].as_str(),
        Some("notApplicable"),
        "an attribute's boolean value is not a verdict"
    );
}

/// A metadata annotation is a settled binding to the definition it names.
#[test]
fn lsp_feature_inspector_publishes_metadata_bindings() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_metadata.sysml";
    let content = concat!(
        "package P {\n",
        "  metadata def Safety;\n",
        "  part def Vehicle {\n",
        "    @Safety;\n",
        "  }\n",
        "  part def Plain;\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_metadata");
    session.did_open(uri, content, 1);
    session.barrier();

    let vehicle = inspect(&mut session, uri, 2, 13);
    let metadata = vehicle["result"]["containingElement"]["metadata"]
        .as_array()
        .expect("metadata bindings");
    assert_eq!(metadata.len(), 1, "{metadata:#?}");
    assert_eq!(metadata[0]["name"].as_str(), Some("Safety"));

    let plain = inspect(&mut session, uri, 5, 13);
    assert_eq!(
        plain["result"]["containingElement"]["metadata"]
            .as_array()
            .map(|entries| entries.len()),
        Some(0)
    );
}

/// Modifiers and multiplicity are the author's own, not an effective value overwritten onto them.
#[test]
fn lsp_feature_inspector_reports_authored_modifiers_and_multiplicity() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_modifiers.sysml";
    let content = concat!(
        "package P {\n",
        "  abstract part def Chassis {\n",
        "    attribute mass [1..*] ordered nonunique;\n",
        "    ref part spare [0..1];\n",
        "  }\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_modifiers");
    session.did_open(uri, content, 1);
    session.barrier();

    let chassis = inspect(&mut session, uri, 1, 22);
    let modifiers = chassis["result"]["containingElement"]["modifiers"]
        .as_array()
        .expect("chassis modifiers")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();
    assert!(modifiers.contains(&"abstract"), "{modifiers:?}");

    let mass = inspect(&mut session, uri, 2, 16);
    let mass_element = &mass["result"]["containingElement"];
    assert_eq!(mass_element["multiplicity"].as_str(), Some("1..*"));
    let modifiers = mass_element["modifiers"]
        .as_array()
        .expect("mass modifiers")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();
    assert!(modifiers.contains(&"ordered"), "{modifiers:?}");
    assert!(modifiers.contains(&"nonunique"), "{modifiers:?}");

    let spare = inspect(&mut session, uri, 3, 14);
    let spare_element = &spare["result"]["containingElement"];
    assert_eq!(spare_element["multiplicity"].as_str(), Some("0..1"));
    assert!(spare_element["modifiers"]
        .as_array()
        .expect("spare modifiers")
        .iter()
        .any(|value| value.as_str() == Some("ref")));
}

/// Relationships keep their provenance in both directions, so an implied redefinition is never
/// shown as one the author wrote.
#[test]
fn lsp_feature_inspector_keeps_authored_and_implied_relationships_apart() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_provenance.sysml";
    let content = concat!(
        "package P {\n",
        "  part def Wheel;\n",
        "  part def Base { part wheel : Wheel; }\n",
        "  part def Derived :> Base { part wheel : Wheel; }\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_provenance");
    session.did_open(uri, content, 1);
    session.barrier();

    let derived_wheel = inspect(&mut session, uri, 3, 34);
    let outgoing = derived_wheel["result"]["containingElement"]["outgoingRelationships"]
        .as_array()
        .expect("outgoing relationships");
    assert!(
        outgoing
            .iter()
            .any(|entry| entry["type"].as_str() == Some("typing")
                && entry["provenance"].as_str() == Some("authored")),
        "{outgoing:#?}"
    );
    assert!(
        outgoing
            .iter()
            .any(|entry| entry["type"].as_str() == Some("redefinition")
                && entry["provenance"].as_str() == Some("implied")),
        "the resolver-derived redefinition must not present as authored: {outgoing:#?}"
    );
    assert_eq!(
        derived_wheel["result"]["containingElement"]["redefinition"]["status"].as_str(),
        Some("notApplicable"),
        "the author declared no redefinition, so the family is not applicable"
    );
}

/// Repeating a request, and asking for other elements in between, cannot change an answer.
#[test]
fn lsp_feature_inspector_answers_are_stable_across_repetition_and_query_order() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_stability.sysml";
    let content = concat!(
        "package P {\n",
        "  part def Wheel;\n",
        "  part def Vehicle { part wheel : Wheel; part spare : Wheel; }\n",
        "  part car : Vehicle;\n",
        "}\n",
    );
    session.initialize_default("feature_inspector_stability");
    session.did_open(uri, content, 1);
    session.barrier();

    let probes = [(1u32, 13u32), (2, 13), (2, 26), (3, 9)];
    let forward = probes
        .iter()
        .map(|(line, character)| inspect(&mut session, uri, *line, *character)["result"].clone())
        .collect::<Vec<_>>();
    let mut reverse = probes
        .iter()
        .rev()
        .map(|(line, character)| inspect(&mut session, uri, *line, *character)["result"].clone())
        .collect::<Vec<_>>();
    reverse.reverse();
    assert_eq!(forward, reverse, "query order changed a published answer");

    for (index, (line, character)) in probes.iter().enumerate() {
        assert_eq!(
            forward[index],
            inspect(&mut session, uri, *line, *character)["result"],
            "repeating a request changed its answer"
        );
    }
}

/// A workspace element typed by an admitted library declaration resolves to that library document.
#[test]
fn lsp_feature_inspector_resolves_a_target_in_an_admitted_library() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root = temp.path().canonicalize().expect("canonical temp root");
    let workspace_root = root.join("workspace");
    let library_root = root.join("libraries");
    std::fs::create_dir_all(&workspace_root).expect("workspace dir");
    std::fs::create_dir_all(&library_root).expect("library dir");
    let workspace_uri =
        url::Url::from_file_path(workspace_root.join("app.sysml")).expect("workspace uri");
    let library_uri =
        url::Url::from_file_path(library_root.join("domain.sysml")).expect("library uri");

    let mut session = TestSession::new();
    session.initialize_with_options(
        "feature_inspector_library",
        Some(serde_json::json!({
            "libraryPaths": [library_root.to_string_lossy().to_string()]
        })),
    );
    session.did_open(
        library_uri.as_str(),
        "library package Domain { part def Wheel; }",
        1,
    );
    session.did_open(
        workspace_uri.as_str(),
        "package App { part w : Domain::Wheel; }",
        1,
    );
    session.barrier();

    let typing = &inspect(&mut session, workspace_uri.as_str(), 0, 19)["result"]
        ["containingElement"]["typing"];
    assert_eq!(typing["status"].as_str(), Some("resolved"), "{typing:#?}");
    assert_eq!(
        typing["targets"][0]["uri"].as_str(),
        Some(library_uri.as_str()),
        "{typing:#?}"
    );
}

/// Recovery-produced input still answers for the declarations the parser did recover, and the
/// broken one keeps an explicit outcome rather than a fabricated target.
#[test]
fn lsp_feature_inspector_answers_over_recovery_produced_input() {
    let mut session = TestSession::new();
    let uri = "file:///feature_inspector_recovery.sysml";
    let content = "package P {\n  part def Wheel;\n  part broken : ;\n}\n";
    session.initialize_default("feature_inspector_recovery");
    session.did_open(uri, content, 1);
    session.barrier();

    let wheel = inspect(&mut session, uri, 1, 12);
    assert_eq!(
        wheel["result"]["containingElement"]["name"].as_str(),
        Some("Wheel")
    );
    assert_eq!(
        wheel["result"]["containingElement"]["typing"]["status"].as_str(),
        Some("notApplicable")
    );
}
