//! Completion integration tests.

use super::harness::TestSession;

fn completion_items(
    session: &mut TestSession,
    uri: &str,
    line: u32,
    character: u32,
) -> Vec<serde_json::Value> {
    let compl_json = session.request(
        "textDocument/completion",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        }),
    );
    compl_json["result"]
        .as_array()
        .cloned()
        .or_else(|| compl_json["result"]["items"].as_array().cloned())
        .expect("completion items")
}

fn position_for(content: &str, needle: &str) -> (u32, u32) {
    for (line_index, line) in content.lines().enumerate() {
        if let Some(character) = line.find(needle) {
            return (line_index as u32, (character + needle.len()) as u32);
        }
    }
    panic!("needle not found in content: {needle}");
}

fn completion_labels(
    session: &mut TestSession,
    uri: &str,
    line: u32,
    character: u32,
) -> Vec<String> {
    completion_items(session, uri, line, character)
        .iter()
        .filter_map(|i| i["label"].as_str().map(String::from))
        .collect()
}

fn resolve_completion_item(
    session: &mut TestSession,
    item: serde_json::Value,
) -> serde_json::Value {
    session.request("completionItem/resolve", item)["result"].clone()
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
    assert!(
        !labels.is_empty(),
        "completion should have at least one item"
    );
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
    assert!(
        controller_idx.is_some(),
        "expected Controller in {:?}",
        labels
    );
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

#[test]
fn completion_in_body_prefers_constructive_snippets() {
    let mut session = TestSession::new();
    let uri = "file:///body_context.sysml";
    let content = r#"package P {
    part def Vehicle {
        pa
    }
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 2, 10);
    assert!(
        labels.first().map(String::as_str) == Some("part def"),
        "expected `part def` snippet first in {:?}",
        labels
    );
}

#[test]
fn completion_returns_snippet_metadata() {
    let mut session = TestSession::new();
    let uri = "file:///snippet_context.sysml";
    let content = "pa";

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let items = completion_items(&mut session, uri, 0, 2);
    let part_def = items
        .iter()
        .find(|item| item["label"].as_str() == Some("part def"))
        .expect("part def snippet");
    assert_eq!(part_def["kind"].as_u64(), Some(15));
    assert_eq!(part_def["filterText"].as_str(), Some("part def"));
    let range = &part_def["textEdit"]["range"];
    assert_eq!(range["start"]["character"].as_u64(), Some(0));
    assert_eq!(range["end"]["character"].as_u64(), Some(2));
}

#[test]
fn completion_resolve_populates_documentation() {
    let mut session = TestSession::new();
    let uri = "file:///resolve_context.sysml";
    let content = r#"package P {
    part def Vehicle;
    part vehicle: Ve
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let items = completion_items(&mut session, uri, 2, 20);
    let vehicle = items
        .iter()
        .find(|item| item["label"].as_str() == Some("Vehicle"))
        .cloned()
        .expect("Vehicle completion item");
    let resolved = resolve_completion_item(&mut session, vehicle);
    assert!(
        resolved["documentation"].as_str().is_some()
            || resolved["documentation"]["value"].as_str().is_some(),
        "expected documentation after resolve: {}",
        resolved
    );
    let markdown = resolved["documentation"]["value"]
        .as_str()
        .or_else(|| resolved["documentation"].as_str())
        .expect("completion documentation text");
    assert!(
        markdown.contains("Qualified name") && markdown.contains("Vehicle"),
        "expected semantic markdown after resolve: {}",
        markdown
    );
}

#[test]
fn completion_keeps_homonyms_distinguishable() {
    let mut session = TestSession::new();
    let uri = "file:///homonym_context.sysml";
    let content = r#"package A {
    part def Sensor;
}
package B {
    part def Sensor;
}
package Main {
    part device: S
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let (line, character) = position_for(content, "part device: S");
    let items = completion_items(&mut session, uri, line, character);
    let sensors: Vec<_> = items
        .iter()
        .filter(|item| item["label"].as_str() == Some("Sensor"))
        .collect();
    assert!(
        sensors.len() >= 2,
        "expected homonym Sensor completions to remain distinct: {}",
        serde_json::Value::Array(items)
    );
    assert!(
        sensors
            .iter()
            .any(|item| item["labelDetails"]["description"].as_str() == Some("A"))
            && sensors
                .iter()
                .any(|item| item["labelDetails"]["description"].as_str() == Some("B")),
        "expected homonym completion label details to show containers: {:?}",
        sensors
    );
}

#[test]
fn completion_prefix_matches_outrank_substring_matches() {
    let mut session = TestSession::new();
    let uri = "file:///prefix_context.sysml";
    let content = r#"package P {
    part def RemoteMotor;
    part def Motor;
    part drive: Mo
}"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let (line, character) = position_for(content, "part drive: Mo");
    let labels = completion_labels(&mut session, uri, line, character);
    let motor_idx = labels.iter().position(|label| label == "Motor");
    let remote_idx = labels.iter().position(|label| label == "RemoteMotor");
    assert!(motor_idx.is_some(), "expected Motor in {:?}", labels);
    assert!(remote_idx.is_some(), "expected RemoteMotor in {:?}", labels);
    assert!(
        motor_idx.unwrap() < remote_idx.unwrap(),
        "prefix match should outrank substring match in {:?}",
        labels
    );
}

#[test]
fn completion_survives_incomplete_syntax() {
    let mut session = TestSession::new();
    let uri = "file:///broken_context.sysml";
    let content = r#"package P {
    part vehicle:
    pa
"#;

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();

    let labels = completion_labels(&mut session, uri, 2, 6);
    assert!(
        !labels.is_empty(),
        "expected non-empty completion in {:?}",
        labels
    );
}
