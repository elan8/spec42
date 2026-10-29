use language_service::{complete, CompletionItemKindDto, InMemoryWorkspace, WorkspaceSnapshot};
use sysml_query::resolved_slice::TextPosition;
use url::Url;

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
    assert!(
        labels.iter().any(|label| label == "Foo"),
        "expected the qualified member Foo in {labels:?}"
    );
    assert!(
        !labels.iter().any(|label| label == "Force"),
        "after `Pkg::` only Pkg's members are names that can follow; got {labels:?}"
    );
}

/// The editor text the user is typing, over a publication built from an older revision.
struct EditedWorkspace<'w> {
    published: &'w InMemoryWorkspace,
    editor_text: &'w str,
}

impl WorkspaceSnapshot for EditedWorkspace<'_> {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url> {
        self.published.resolve_uri_for_path(path)
    }
    fn path_for_uri(&self, uri: &Url) -> String {
        self.published.path_for_uri(uri)
    }
    fn document_text(&self, uri: &Url) -> Option<&str> {
        self.published.document_text(uri)
    }
    fn editor_text(&self, _uri: &Url) -> Option<&str> {
        Some(self.editor_text)
    }
    fn parsed(&self, uri: &Url) -> Option<sysml_query::syntax::ParsedSource> {
        self.published.parsed(uri)
    }
    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel> {
        self.published.published_model()
    }
    fn symbol_table(&self) -> &[language_service::SymbolEntry] {
        self.published.symbol_table()
    }
    fn index_uris(&self) -> Vec<Url> {
        self.published.index_uris()
    }
}

/// Completion is asked while the user types, before analysis of the new text has finished. The
/// context and the replaced range must come from the text being typed, not from the revision the
/// publication was built from.
#[test]
fn complete_reads_the_editor_text_ahead_of_the_publication() {
    let path = "typing.sysml";
    let published = "package P {\n    part def Engine;\n    attribute def Energy;\n}\n";
    let editor =
        "package P {\n    part def Engine;\n    attribute def Energy;\n    part motor : En\n}\n";
    let base = single_doc(path, published);
    let workspace = EditedWorkspace {
        published: &base,
        editor_text: editor,
    };
    let position = position_after(editor, "part motor : En");
    let result = complete(&workspace, path, position).expect("completion result");
    let labels: Vec<_> = result
        .items
        .iter()
        .map(|item| item.label.as_str())
        .collect();
    let engine = labels.iter().position(|label| *label == "Engine");
    let energy = labels.iter().position(|label| *label == "Energy");
    assert!(
        engine.is_some_and(|engine| energy.is_none_or(|energy| engine < energy)),
        "the part type context comes from the editor text; got {labels:?}"
    );
    let engine_item = &result.items[engine.unwrap()];
    let edit = engine_item.text_edit.as_ref().expect("text edit");
    assert_eq!(
        (edit.range.start, edit.range.end),
        (TextPosition::new(3, 17), position),
        "the edit replaces the typed `En` in the editor text"
    );
    assert!(
        result.semantic_status.predates_source && result.is_incomplete,
        "an answer over an older publication says so: {:?}",
        result.semantic_status
    );
    assert!(
        result.items.iter().all(|item| !item.preselect),
        "nothing is preselected, so Enter keeps inserting a newline"
    );
}

#[test]
fn complete_over_a_current_publication_is_not_marked_as_predating_it() {
    let path = "current.sysml";
    let content = "package P {\n    part def Engine;\n    part motor : En\n}\n";
    let workspace = single_doc(path, content);
    let result = complete(&workspace, path, position_after(content, "part motor : En"))
        .expect("completion result");
    assert!(!result.semantic_status.predates_source);
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
    private import A::*;
    private import B::*;
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
    let use_content = "package Q { private import P::*; part motor: En }";
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

/// The completion item's icon must reflect what the symbol actually is.
///
/// This assertion did not exist before the element kind became a typed enum, which is how
/// `element_kind_to_completion_kind` came to be called with a raw `"PartDefinition"` while
/// matching display labels such as `"part def"`: no arm ever fired and every item was rendered as
/// `Reference`.
#[test]
fn complete_labels_each_symbol_with_its_own_item_kind() {
    let path = "item_kinds.sysml";
    let content = r#"package P {
    part def Lorry;
    attribute def Load;
    action def Launch;
    part p: L
}"#;
    let workspace = single_doc(path, content);
    let items = complete(&workspace, path, position_at(4, 13))
        .map(|result| result.items)
        .unwrap_or_default();

    let kind_of = |label: &str| {
        items
            .iter()
            .find(|item| item.label == label)
            .unwrap_or_else(|| {
                let labels = items.iter().map(|item| &item.label).collect::<Vec<_>>();
                panic!("expected {label} among {labels:?}")
            })
            .kind
    };

    assert_eq!(kind_of("Lorry"), Some(CompletionItemKindDto::Class));
    assert_eq!(kind_of("Load"), Some(CompletionItemKindDto::Property));
    assert_eq!(kind_of("Launch"), Some(CompletionItemKindDto::Function));
    assert!(
        items
            .iter()
            .filter(|item| item.kind != Some(CompletionItemKindDto::Keyword)
                && item.kind != Some(CompletionItemKindDto::Snippet))
            .any(|item| item.kind != Some(CompletionItemKindDto::Reference)),
        "every model symbol fell through to Reference, which is the bug this test exists for"
    );
}
