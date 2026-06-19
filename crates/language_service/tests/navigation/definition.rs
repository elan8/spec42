use language_service::goto_definition;

use super::support::{multi_doc, paths, position_at, position_for, single_doc};

#[test]
fn goto_definition_resolves_part_def() {
    let content = "package P { part def Engine; }";
    let ws = single_doc("test.sysml", content);
    let pos = position_for(content, "Engine");
    let result = goto_definition(&ws, "test.sysml", pos);
    assert!(!result.locations.is_empty(), "expected definition for Engine");
    assert_eq!(result.locations[0].path, "test.sysml");
}

#[test]
fn goto_definition_resolves_same_file_usage_to_definition() {
    let content = "package P { part def A; part a : A; }";
    let ws = single_doc("def_test.sysml", content);
    let pos = position_at(0, 22);
    let result = goto_definition(&ws, "def_test.sysml", pos);
    assert!(
        !result.locations.is_empty(),
        "expected definition for usage of A, got {:?}",
        result.locations
    );
    assert_eq!(result.locations[0].path, "def_test.sysml");
}

#[test]
fn goto_definition_cross_file_resolves_to_definition() {
    let content_def = "package P { part def Engine; }";
    let content_use = "package Q { part e : Engine; }";
    let ws = multi_doc(&[("def.sysml", content_def), ("use.sysml", content_use)]);
    let pos = position_at(0, 22);
    let result = goto_definition(&ws, "use.sysml", pos);
    assert!(
        result.locations.iter().any(|loc| loc.path.contains("def.sysml")),
        "expected goto-definition to def.sysml, got: {:?}",
        result.locations
    );
}

#[test]
fn goto_definition_resolves_public_reexported_type() {
    let content_core = "package Core { attribute def Name; }";
    let content_domain = "package Domain { public import Core::*; }";
    let content_use =
        "package Demo { import Domain::*; part def Consumer { attribute groupName : Name; } }";
    let ws = multi_doc(&[
        ("core.sysml", content_core),
        ("domain.sysml", content_domain),
        ("use.sysml", content_use),
    ]);
    let pos = position_at(0, 75);
    let result = goto_definition(&ws, "use.sysml", pos);
    let result_paths = paths(&result.locations);
    assert!(
        result_paths.iter().any(|p| p.contains("core.sysml")),
        "goto_definition should resolve re-exported Name to core.sysml, got: {result_paths:?}"
    );
}
