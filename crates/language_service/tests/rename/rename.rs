use language_service::{apply_rename, prepare_rename, rename_target};

use super::support::{multi_doc, position_for_within, single_doc};

#[test]
fn rename_cross_file_updates_definition_and_use() {
    let def_path = "rename/def.sysml";
    let use_path = "rename/use.sysml";
    let def_content = "package P { part def Foo; }";
    let use_content = "package Q { import P::*; part f : Foo; }";
    let workspace = multi_doc(&[(def_path, def_content), (use_path, use_content)]);

    let pos = position_for_within(def_content, "part def Foo", "Foo");
    let range = prepare_rename(&workspace, def_path, pos).expect("prepare rename range");
    assert_eq!(range.start.line, 0);

    let edits = apply_rename(&workspace, def_path, pos, "Bar").expect("valid rename");
    assert!(!edits.is_empty(), "expected rename edits");
    assert!(
        edits
            .iter()
            .any(|e| e.path == def_path && e.replacement == "Bar"),
        "expected definition edit: {:?}",
        edits
    );
    assert!(
        edits
            .iter()
            .any(|e| e.path == use_path && e.replacement == "Bar"),
        "expected use-site edit: {:?}",
        edits
    );
}

#[test]
fn rename_cross_file_from_usage_with_long_identity_updates_both_files() {
    let def_path = "rename/long_def.sysml";
    let use_path = "rename/long_use.sysml";
    let def_content =
        "package Spec42MultiFileDefinitions { part def Spec42SmokeWidget; }";
    let use_content = "package Spec42MultiFileUsages { import Spec42MultiFileDefinitions::*; part item : Spec42SmokeWidget; }";
    let workspace = multi_doc(&[(def_path, def_content), (use_path, use_content)]);

    let pos = position_for_within(use_content, "part item : Spec42SmokeWidget", "Spec42SmokeWidget");
    let edits = apply_rename(
        &workspace,
        use_path,
        pos,
        "Spec42RenamedSmokeWidget",
    )
    .expect("valid rename from usage");

    assert!(edits.iter().any(|edit| edit.path == def_path));
    assert!(edits.iter().any(|edit| edit.path == use_path));
}

#[test]
fn rename_target_lists_definition_and_references() {
    let def_path = "rename/def.sysml";
    let use_path = "rename/use.sysml";
    let def_content = "package P { part def Foo; }";
    let use_content = "package Q { import P::*; part f : Foo; }";
    let workspace = multi_doc(&[(def_path, def_content), (use_path, use_content)]);

    let pos = position_for_within(def_content, "part def Foo", "Foo");
    let target = rename_target(&workspace, def_path, pos).expect("rename target");
    assert_eq!(target.name, "Foo");
    assert_eq!(target.definition.path, def_path);
    assert!(
        target.references.iter().any(|loc| loc.path == use_path),
        "expected reference in use file: {:?}",
        target.references
    );
}

#[test]
fn rename_rejects_a_same_scope_collision() {
    let path = "rename/collision.sysml";
    let content = "package P { part def Foo; part def Bar; part f : Foo; }";
    let workspace = single_doc(path, content);
    let pos = position_for_within(content, "part f : Foo", "Foo");
    assert!(
        apply_rename(&workspace, path, pos, "Bar").is_none(),
        "the owning rename query must reject a same-scope collision"
    );
}

#[test]
fn rename_does_not_touch_same_file_homonyms() {
    let path = "rename/homonyms.sysml";
    let content = r#"package IT {
    part def Laptop {
        port hdmi;
    }
    part def Monitor {
        port hdmi;
    }
    part def Room {
        part laptop : Laptop;
        part monitor : Monitor;
        connect laptop.hdmi to monitor.hdmi;
    }
}"#;
    let workspace = single_doc(path, content);
    let pos = position_for_within(content, "port hdmi;", "hdmi");
    let edits = apply_rename(&workspace, path, pos, "display").expect("valid rename");

    assert!(
        edits.iter().any(|e| e.range.start.line == 2),
        "should rename Laptop::hdmi declaration: {:?}",
        edits
    );
    assert!(
        !edits.iter().any(|e| e.range.start.line == 5),
        "should not rename Monitor::hdmi declaration: {:?}",
        edits
    );
    assert_eq!(
        edits.iter().filter(|e| e.range.start.line == 10).count(),
        1,
        "should rename only laptop.hdmi on connect line: {:?}",
        edits
    );
}

#[test]
fn rename_ignores_comments_and_strings() {
    let path = "rename/comments.sysml";
    let content = r#"package Demo {
    part def Engine;
    part vehicle : Engine;
    // Engine should stay in this comment
    attribute label = "Engine should stay in this string";
}"#;
    let workspace = single_doc(path, content);
    let pos = position_for_within(content, "part def Engine;", "Engine");
    let edits = apply_rename(&workspace, path, pos, "Motor").expect("valid rename");

    assert!(
        edits.iter().any(|e| e.range.start.line == 1),
        "should include declaration: {:?}",
        edits
    );
    assert!(
        edits.iter().any(|e| e.range.start.line == 2),
        "should include typed usage: {:?}",
        edits
    );
    assert!(
        edits
            .iter()
            .all(|e| e.range.start.line != 3 && e.range.start.line != 4),
        "should not edit comments or strings: {:?}",
        edits
    );
}

#[test]
fn prepare_rename_rejects_comments() {
    let path = "rename/comment-prepare.sysml";
    let content = "package Demo {\n  part def Engine;\n  // Engine comment\n}\n";
    let workspace = single_doc(path, content);
    let pos = super::support::position_at(2, 6);
    assert!(
        prepare_rename(&workspace, path, pos).is_none(),
        "prepareRename should reject comment positions"
    );
}
