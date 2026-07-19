//! S42-001 slice: `semanticId` must survive a non-renaming, position-shifting edit (an edit
//! elsewhere in the document that moves a declaration's line/column without touching the element
//! or its qualified name). Before this fix, `semantic_element_id` hashed declaration position
//! instead of the already-unique `qualified_name`, so almost any unrelated edit silently
//! reassigned every downstream element's ID.

#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, memory_document, test_engine};
use tempfile::tempdir;
use workspace::{DocumentChanges, EngineBuilder, HostContext, WorkspaceLoadRequest};

fn incremental_engine(cache: &tempfile::TempDir) -> workspace::Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .experimental_incremental_updates(true)
        .build()
        .expect("engine")
}

#[test]
fn node_semantic_id_is_stable_across_a_position_shifting_non_renaming_edit() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let model_path = cache.path().join("Demo.sysml");

    let initial_content = r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#;
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", initial_content);

    // Same qualified names, but `item`'s declaration has moved: an unrelated sibling was
    // inserted above it. Neither `Thing` nor `item` was renamed.
    let updated_content = r#"
package Demo {
    part def Thing;
    part def Unrelated;

    part item : Thing;
}
"#;
    let changed_doc = memory_document(&model_path, updated_content);
    let changes = DocumentChanges::new().replace(changed_doc);
    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            changes,
            WorkspaceLoadRequest::single_target(model_path.clone()),
            HostContext::default(),
        )
        .expect("incremental update");

    let previous_id = |suffix: &str| {
        previous
            .semantic_projection()
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with(suffix))
            .unwrap_or_else(|| panic!("{suffix} in previous snapshot"))
            .semantic_id
            .clone()
    };
    let updated_id = |suffix: &str| {
        updated
            .semantic_projection()
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with(suffix))
            .unwrap_or_else(|| panic!("{suffix} in updated snapshot"))
            .semantic_id
            .clone()
    };

    assert_eq!(
        previous_id("::Thing"),
        updated_id("::Thing"),
        "Thing's declaration didn't move relative to itself and wasn't renamed"
    );
    assert_eq!(
        previous_id("::item"),
        updated_id("::item"),
        "item's declaration shifted position but wasn't renamed -- its semanticId must not change"
    );

    // Regression guard the other way: an actual rename is still expected to change the ID --
    // that's the deliberately out-of-scope part of S42-001, not a bug this fix introduces.
    let renamed_content = r#"
package Demo {
    part def Thing;
    part widget : Thing;
}
"#;
    let renamed_doc = memory_document(&model_path, renamed_content);
    let renamed = engine
        .update_snapshot(
            previous.as_ref(),
            DocumentChanges::new().replace(renamed_doc),
            WorkspaceLoadRequest::single_target(model_path.clone()),
            HostContext::default(),
        )
        .expect("incremental update");
    let renamed_id = |suffix: &str| {
        renamed
            .semantic_projection()
            .nodes
            .iter()
            .find(|node| node.qualified_name.ends_with(suffix))
            .unwrap_or_else(|| panic!("{suffix} in renamed snapshot"))
            .semantic_id
            .clone()
    };
    assert_ne!(
        previous_id("::item"),
        renamed_id("::widget"),
        "a genuine rename is still expected to change identity -- cross-rename tracking is \
         explicitly out of scope for this fix"
    );
}

#[test]
fn typing_relationship_id_is_stable_across_a_position_shifting_non_renaming_edit() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let model_path = cache.path().join("Demo.sysml");

    let initial_content = r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#;
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", initial_content);

    let updated_content = r#"
package Demo {
    part def Unrelated;
    part def Thing;
    part item : Thing;
}
"#;
    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            DocumentChanges::new().replace(memory_document(&model_path, updated_content)),
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("incremental update");

    let typing_id = |projection: &workspace::HostSemanticProjection| {
        projection
            .relationships
            .iter()
            .find(|relationship| {
                relationship.kind.as_str() == "typing" && relationship.source.ends_with("::item")
            })
            .expect("typing relationship from item")
            .semantic_id
            .clone()
    };

    assert_eq!(
        typing_id(previous.semantic_projection()),
        typing_id(updated.semantic_projection()),
        "the typing edge's endpoints didn't rename, so its semanticId must not change either"
    );
}

#[test]
fn connection_relationship_id_is_stable_when_the_connect_statement_moves() {
    let cache = tempdir().expect("tempdir");
    let engine = incremental_engine(&cache);
    let model_path = cache.path().join("Demo.sysml");

    let initial_content = r#"
package Demo {
    port def P;
    part def Sensor {
        port a : P;
    }
    part sensorA : Sensor;
    part sensorB : Sensor;
    part def Interconnect {
        part x : Sensor;
        part y : Sensor;
        connect x.a to y.a;
    }
}
"#;
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", initial_content);

    // Insert an unrelated sibling above the `connect` statement, shifting its position without
    // changing either endpoint's expression text.
    let updated_content = r#"
package Demo {
    port def P;
    part def Sensor {
        port a : P;
    }
    part sensorA : Sensor;
    part sensorB : Sensor;
    part def Interconnect {
        part x : Sensor;
        part y : Sensor;
        attribute note;

        connect x.a to y.a;
    }
}
"#;
    let updated = engine
        .update_snapshot(
            previous.as_ref(),
            DocumentChanges::new().replace(memory_document(&model_path, updated_content)),
            WorkspaceLoadRequest::single_target(model_path),
            HostContext::default(),
        )
        .expect("incremental update");

    let connect_id = |projection: &workspace::HostSemanticProjection| {
        projection
            .relationships
            .iter()
            .find(|relationship| relationship.kind.as_str() == "connection")
            .expect("connection relationship")
            .semantic_id
            .clone()
    };

    assert_eq!(
        connect_id(previous.semantic_projection()),
        connect_id(updated.semantic_projection()),
        "the connect statement moved but its endpoint expressions didn't change"
    );
}

#[test]
fn two_distinct_connections_between_the_same_endpoints_still_get_distinct_ids() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);

    // Two separate `connect` statements between the same two ports at different qualified paths
    // -- a case this codebase already treats as two distinct connectors, not a duplicate.
    let content = r#"
package Demo {
    port def P;
    part def Sensor {
        port a : P;
        port b : P;
    }
    part sensorA : Sensor;
    part sensorB : Sensor;
    part def Interconnect {
        part x : Sensor;
        part y : Sensor;
        connect x.a to y.a;
        connect x.b to y.b;
    }
}
"#;
    let snapshot = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let projection = snapshot.semantic_projection();
    let connection_ids: std::collections::HashSet<_> = projection
        .relationships
        .iter()
        .filter(|relationship| relationship.kind.as_str() == "connection")
        .map(|relationship| relationship.semantic_id.clone())
        .collect();
    assert_eq!(
        connection_ids.len(),
        2,
        "two distinct connect statements must still get two distinct relationship IDs"
    );
}
