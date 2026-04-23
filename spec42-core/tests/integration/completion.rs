//! Completion integration tests.

use super::harness::TestSession;

fn completion_labels(
    session: &mut TestSession,
    uri: &str,
    line: u32,
    character: u32,
) -> Vec<String> {
    let compl_json = session.request(
        "textDocument/completion",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        }),
    );
    let items = compl_json["result"]
        .as_array()
        .or_else(|| compl_json["result"]["items"].as_array());
    assert!(
        items.is_some(),
        "completion should return array: {}",
        compl_json
    );
    items
        .unwrap()
        .iter()
        .filter_map(|i| i["label"].as_str().map(String::from))
        .collect()
}

#[test]
fn lsp_completion() {
    let mut session = TestSession::new();

    let uri = "file:///test2.sysml";
    let content = "package P { part def X; }";

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();
    let labels = completion_labels(&mut session, uri, 0, 2);
    assert!(!labels.is_empty(), "completion should have at least one item");
    assert!(
        labels.iter().any(|l| l == "part" || l == "package"),
        "completion should include keywords: {:?}",
        labels
    );
}

#[test]
fn completion_prioritizes_def_after_part_keyword() {
    let mut session = TestSession::new();
    let uri = "file:///keyword_modifier.sysml";
    let content = r#"package P {
    part 
    part def engineMount;
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 1, 9);
    let first = labels.first().map(String::as_str);
    assert!(
        first == Some("def"),
        "def should be the top suggestion after `part ` in {:?}",
        labels
    );
}

#[test]
fn completion_prioritizes_part_type_definitions() {
    let mut session = TestSession::new();
    let uri = "file:///type_context.sysml";
    let content = r#"package P {
    part def Laptop;
    attribute def Label;
    part laptop: La
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 3, 19);
    let laptop_idx = labels.iter().position(|label| label == "Laptop");
    let label_idx = labels.iter().position(|label| label == "Label");
    assert!(laptop_idx.is_some(), "expected Laptop in {:?}", labels);
    assert!(label_idx.is_some(), "expected Label in {:?}", labels);
    assert!(
        laptop_idx.unwrap() < label_idx.unwrap(),
        "part definition should outrank attribute definition in {:?}",
        labels
    );
}

#[test]
fn completion_prioritizes_port_types_for_port_usage() {
    let mut session = TestSession::new();
    let uri = "file:///port_context.sysml";
    let content = r#"package P {
    interface def CommandBus;
    part def Controller;
    part system {
        port control: C
    }
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 4, 23);
    let command_idx = labels.iter().position(|label| label == "CommandBus");
    let controller_idx = labels.iter().position(|label| label == "Controller");
    assert!(command_idx.is_some(), "expected CommandBus in {:?}", labels);
    assert!(controller_idx.is_some(), "expected Controller in {:?}", labels);
    assert!(
        command_idx.unwrap() < controller_idx.unwrap(),
        "port-compatible definitions should outrank unrelated part definitions in {:?}",
        labels
    );
}

#[test]
fn completion_prioritizes_attribute_types() {
    let mut session = TestSession::new();
    let uri = "file:///attribute_context.sysml";
    let content = r#"package P {
    item def MassUnit;
    part def Motor;
    part vehicle {
        attribute mass: M
    }
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 4, 25);
    let mass_idx = labels.iter().position(|label| label == "MassUnit");
    let motor_idx = labels.iter().position(|label| label == "Motor");
    assert!(mass_idx.is_some(), "expected MassUnit in {:?}", labels);
    assert!(motor_idx.is_some(), "expected Motor in {:?}", labels);
    assert!(
        mass_idx.unwrap() < motor_idx.unwrap(),
        "attribute-compatible definitions should outrank part defs in {:?}",
        labels
    );
}

#[test]
fn completion_prioritizes_qualified_namespace_members() {
    let mut session = TestSession::new();
    let uri = "file:///qualified_context.sysml";
    let content = r#"package Pkg {
    part def Foo;
}
package Main {
    attribute def Force;
    part thing: Pkg::Fo
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 5, 22);
    let foo_idx = labels.iter().position(|label| label == "Foo");
    let force_idx = labels.iter().position(|label| label == "Force");
    assert!(foo_idx.is_some(), "expected Foo in {:?}", labels);
    assert!(force_idx.is_some(), "expected Force in {:?}", labels);
    assert!(
        foo_idx.unwrap() < force_idx.unwrap(),
        "qualified namespace match should outrank unrelated symbols in {:?}",
        labels
    );
}

#[test]
fn completion_prioritizes_typed_members() {
    let mut session = TestSession::new();
    let uri = "file:///member_context.sysml";
    let content = r#"package P {
    part def Vehicle {
        part engine;
    }
    part def engineMount;
    part vehicle: Vehicle;
    attribute useCase = vehicle.eng
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 6, 34);
    let engine_idx = labels.iter().position(|label| label == "engine");
    let mount_idx = labels.iter().position(|label| label == "engineMount");
    assert!(engine_idx.is_some(), "expected engine in {:?}", labels);
    assert!(mount_idx.is_some(), "expected engineMount in {:?}", labels);
    assert!(
        engine_idx.unwrap() < mount_idx.unwrap(),
        "typed member should outrank unrelated symbol in {:?}",
        labels
    );
}
