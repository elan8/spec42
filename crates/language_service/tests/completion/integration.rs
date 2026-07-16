use language_service::{complete, CompletionItemKindDto};

use super::support::{completion_labels, multi_doc, position_after, position_at, single_doc};

#[test]
fn complete_returns_keywords_for_top_level_prefix() {
    let path = "test.sysml";
    let content = "package P { part def X; }";
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(0, 2));
    assert!(!labels.is_empty(), "completion should have items");
    assert!(
        labels.iter().any(|l| l == "part" || l == "package"),
        "expected keywords in {:?}",
        labels
    );
}

#[test]
fn complete_prioritizes_def_after_part_keyword() {
    let path = "keyword_modifier.sysml";
    let content = r#"package P {
    part 
    part def engineMount;
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(1, 9));
    assert_eq!(
        labels.first().map(String::as_str),
        Some("def"),
        "def should be first after `part ` in {:?}",
        labels
    );
}

#[test]
fn complete_prioritizes_part_type_definitions() {
    let path = "type_context.sysml";
    let content = r#"package P {
    part def Laptop;
    attribute def Label;
    part laptop: La
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(3, 19));
    let laptop_idx = labels.iter().position(|label| label == "Laptop");
    let label_idx = labels.iter().position(|label| label == "Label");
    assert!(laptop_idx.is_some(), "expected Laptop in {:?}", labels);
    assert!(label_idx.is_some(), "expected Label in {:?}", labels);
    assert!(
        laptop_idx.unwrap() < label_idx.unwrap(),
        "part def should outrank attribute def in {:?}",
        labels
    );
}

#[test]
fn complete_prioritizes_port_types_for_port_usage() {
    let path = "port_context.sysml";
    let content = r#"package P {
    interface def CommandBus;
    part def Controller;
    part system {
        port control: C
    }
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(4, 23));
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
        "port-compatible type should outrank part def in {:?}",
        labels
    );
}

#[test]
fn complete_prioritizes_attribute_types() {
    let path = "attribute_context.sysml";
    let content = r#"package P {
    item def MassUnit;
    part def Motor;
    part vehicle {
        attribute mass: M
    }
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(4, 25));
    let mass_idx = labels.iter().position(|label| label == "MassUnit");
    let motor_idx = labels.iter().position(|label| label == "Motor");
    assert!(
        mass_idx.is_some() && motor_idx.is_some(),
        "expected symbols in {:?}",
        labels
    );
    assert!(
        mass_idx.unwrap() < motor_idx.unwrap(),
        "attribute-compatible type should outrank part def in {:?}",
        labels
    );
}

#[test]
fn complete_prioritizes_qualified_namespace_members() {
    let path = "qualified_context.sysml";
    let content = r#"package Pkg {
    part def Foo;
}
package Main {
    attribute def Force;
    part thing: Pkg::Fo
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(5, 22));
    let foo_idx = labels.iter().position(|label| label == "Foo");
    let force_idx = labels.iter().position(|label| label == "Force");
    assert!(foo_idx.is_some() && force_idx.is_some());
    assert!(
        foo_idx.unwrap() < force_idx.unwrap(),
        "qualified match should outrank unrelated symbol in {:?}",
        labels
    );
}

#[test]
fn complete_in_body_prefers_constructive_snippets() {
    let path = "body_context.sysml";
    let content = r#"package P {
    part def Vehicle {
        pa
    }
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(2, 10));
    assert_eq!(
        labels.first().map(String::as_str),
        Some("part def"),
        "expected `part def` snippet first in {:?}",
        labels
    );
}

#[test]
fn complete_returns_snippet_metadata() {
    let path = "snippet_context.sysml";
    let content = r#"package P {
    part def Vehicle {
        pa
    }
}"#;
    let workspace = single_doc(path, content);
    let result = complete(&workspace, path, position_at(2, 10)).expect("completion result");
    let part_def = result
        .items
        .iter()
        .find(|item| item.label == "part def")
        .expect("part def snippet");
    assert_eq!(part_def.kind, Some(CompletionItemKindDto::Snippet));
    assert_eq!(part_def.filter_text.as_deref(), Some("part def"));
    assert!(part_def.insert_text_format_snippet);
    let edit = part_def.text_edit.as_ref().expect("text edit");
    assert_eq!(edit.range.start.line, 2);
    assert_eq!(edit.range.end.character, 10);
}

#[test]
fn complete_includes_resolve_documentation_for_symbols() {
    let path = "resolve_context.sysml";
    let content = r#"package P {
    part def Vehicle;
    part vehicle: Ve
}"#;
    let workspace = single_doc(path, content);
    let result = complete(&workspace, path, position_at(2, 20)).expect("completion result");
    let vehicle = result
        .items
        .iter()
        .find(|item| item.label == "Vehicle")
        .expect("Vehicle item");
    let doc = vehicle
        .resolve_documentation
        .as_deref()
        .or(vehicle.documentation.as_deref())
        .expect("documentation");
    assert!(
        doc.contains("Qualified name") && doc.contains("Vehicle"),
        "expected semantic markdown: {doc}"
    );
}

#[test]
fn complete_keeps_homonyms_distinguishable() {
    let path = "homonym_context.sysml";
    let content = r#"package A {
    part def Sensor;
}
package B {
    part def Sensor;
}
package Main {
    part device: S
}"#;
    let workspace = single_doc(path, content);
    let pos = position_after(content, "part device: S");
    let result = complete(&workspace, path, pos).expect("completion result");
    let sensors: Vec<_> = result
        .items
        .iter()
        .filter(|item| item.label == "Sensor")
        .collect();
    assert!(
        sensors.len() >= 2,
        "expected distinct homonym Sensor items: {:?}",
        result.items
    );
    assert!(
        sensors.iter().any(|item| {
            item.label_details
                .as_ref()
                .and_then(|d| d.description.as_deref())
                == Some("A")
        }) && sensors.iter().any(|item| {
            item.label_details
                .as_ref()
                .and_then(|d| d.description.as_deref())
                == Some("B")
        }),
        "expected container label details: {:?}",
        sensors
    );
}

#[test]
fn complete_prefix_matches_outrank_substring_matches() {
    let path = "prefix_context.sysml";
    let content = r#"package P {
    part def RemoteMotor;
    part def Motor;
    part drive: Mo
}"#;
    let workspace = single_doc(path, content);
    let pos = position_after(content, "part drive: Mo");
    let labels = completion_labels(&workspace, path, pos);
    let motor_idx = labels.iter().position(|label| label == "Motor");
    let remote_idx = labels.iter().position(|label| label == "RemoteMotor");
    assert!(motor_idx.is_some() && remote_idx.is_some());
    assert!(
        motor_idx.unwrap() < remote_idx.unwrap(),
        "prefix match should outrank substring in {:?}",
        labels
    );
}

#[test]
fn complete_survives_incomplete_syntax() {
    let path = "broken_context.sysml";
    let content = r#"package P {
    part vehicle:
    pa
}"#;
    let workspace = single_doc(path, content);
    let labels = completion_labels(&workspace, path, position_at(2, 6));
    assert!(
        !labels.is_empty(),
        "expected non-empty completion in {:?}",
        labels
    );
}

#[test]
fn complete_cross_file_includes_workspace_symbols() {
    let use_content = "package Q { part motor: En }";
    let workspace = multi_doc(&[
        ("defs.sysml", "package P { part def Engine; }"),
        ("use.sysml", use_content),
    ]);
    let pos = position_after(use_content, "part motor: En");
    let labels = completion_labels(&workspace, "use.sysml", pos);
    assert!(
        labels.iter().any(|label| label == "Engine"),
        "expected cross-file symbol Engine in {:?}",
        labels
    );
}
