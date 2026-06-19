use language_service::find_references;

use super::support::{
    any_on_line, any_on_line_at, count_on_line, multi_doc, paths, position_at,
    position_for_within, single_doc,
};

#[test]
fn find_references_on_definition_includes_declaration() {
    let content = "package P { part def Engine; part motor : Engine; }";
    let ws = single_doc("test.sysml", content);
    let pos = position_for_within(content, "part def Engine", "Engine");
    let result = find_references(&ws, "test.sysml", pos, true);
    assert!(
        !result.locations.is_empty(),
        "expected at least the declaration reference, got {}",
        result.locations.len()
    );
}

#[test]
fn find_references_excludes_declaration_when_requested() {
    let content = "package P { part def Engine; part motor : Engine; }";
    let ws = single_doc("test.sysml", content);
    let usage_pos = position_for_within(content, "part motor : Engine", "Engine");
    let result_with = find_references(&ws, "test.sysml", usage_pos, true);
    let result_without = find_references(&ws, "test.sysml", usage_pos, false);
    let def_pos = position_for_within(content, "part def Engine", "Engine");
    assert!(
        any_on_line_at(&result_with.locations, def_pos.line, def_pos.character),
        "include declaration should list the definition site: {:?}",
        result_with.locations
    );
    assert!(
        !any_on_line_at(
            &result_without.locations,
            def_pos.line,
            def_pos.character
        ),
        "exclude declaration should omit the definition site: {:?}",
        result_without.locations
    );
    assert_eq!(
        result_without.locations.len(),
        1,
        "exclude declaration should leave only usage references: {:?}",
        result_without.locations
    );
    assert!(
        result_without.locations[0].range.start.character > def_pos.character,
        "remaining reference should be the usage occurrence: {:?}",
        result_without.locations
    );
}

#[test]
fn find_references_cross_file_includes_definition_and_use() {
    let content_def = "package P { part def Widget; }";
    let content_use = "package Q { part w : Widget; }";
    let ws = multi_doc(&[("def.sysml", content_def), ("use.sysml", content_use)]);
    let pos = super::support::position_for(content_use, "Widget");
    let result = find_references(&ws, "use.sysml", pos, true);
    let result_paths = paths(&result.locations);
    assert!(
        result_paths.iter().any(|p| p.contains("def.sysml")),
        "expected def.sysml in references, got: {result_paths:?}"
    );
    assert!(
        result_paths.iter().any(|p| p.contains("use.sysml")),
        "expected use.sysml in references, got: {result_paths:?}"
    );
}

#[test]
fn same_file_homonym_references_are_disambiguated_by_position() {
    let content = r#"package IT {
    part def Laptop {
        port hdmi;
    }
    part def Monitor {
        port hdmi;
    }
}"#;
    let ws = single_doc("laptop.sysml", content);
    let pos = position_at(2, 13);
    let result = find_references(&ws, "laptop.sysml", pos, true);
    assert!(
        any_on_line(&result.locations, 2),
        "references should include Laptop::hdmi declaration: {:?}",
        result.locations
    );
    assert!(
        !any_on_line(&result.locations, 5),
        "references should not include Monitor::hdmi declaration: {:?}",
        result.locations
    );
}

#[test]
fn dotted_usage_disambiguates_same_name_members() {
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
    let ws = single_doc("dotted.sysml", content);
    let pos = position_at(2, 13);
    let result = find_references(&ws, "dotted.sysml", pos, true);
    assert!(
        any_on_line_at(&result.locations, 2, 13),
        "references should include Laptop::hdmi declaration: {:?}",
        result.locations
    );
    assert_eq!(
        count_on_line(&result.locations, 10),
        1,
        "references should include exactly one hdmi endpoint on connect line: {:?}",
        result.locations
    );
}

#[test]
fn same_short_name_in_other_file_is_not_counted_without_semantic_match() {
    let workspace_content = r#"package W {
    part def Laptop {
        port power;
    }
}"#;
    let library_content = r#"package L {
    part def Generator {
        port power;
    }
}"#;
    let ws = multi_doc(&[
        ("workspace.sysml", workspace_content),
        ("lib.sysml", library_content),
    ]);
    let pos = position_at(2, 13);
    let result = find_references(&ws, "workspace.sysml", pos, true);
    assert!(
        result
            .locations
            .iter()
            .all(|loc| loc.path.contains("workspace.sysml")),
        "references should stay in workspace file only: {:?}",
        result.locations
    );
    assert!(
        !result.locations.is_empty(),
        "expected workspace power declaration reference"
    );
}
