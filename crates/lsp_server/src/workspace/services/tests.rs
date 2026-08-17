use std::fs;

use super::{
    apply_document_changes, apply_document_changes_fast, rebuild_all_document_links,
    remove_document, store_document_text,
};
use crate::analysis::diagnostics_core;
use crate::workspace::state::ServerState;
use tower_lsp::lsp_types::{NumberOrString, Position, Range, TextDocumentContentChangeEvent, Url};

fn fixture_uri() -> Url {
    Url::parse("file:///C:/workspace/test.sysml").expect("fixture uri")
}

fn find_attribute_node<'a>(
    state: &'a ServerState,
    uri: &Url,
    name: &str,
) -> &'a crate::semantic::SemanticNode {
    state
        .semantic_graph
        .nodes_for_uri(uri)
        .into_iter()
        .find(|node| node.element_kind == "attribute" && node.name == name)
        .expect("attribute node")
}

fn expression<'a>(
    state: &'a ServerState,
    node: &crate::semantic::SemanticNode,
) -> sysml_model::ExpressionEvaluationQuery<'a> {
    state.semantic_graph.expression_evaluation_for(node)
}

fn register_units_library_document(state: &mut ServerState) -> Url {
    const UNITS_CATALOG: &str = r#"
            package Units {
                attribute <m> 'metre' : LengthUnit;
                attribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }
                attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
            }
        "#;
    let temp = tempfile::tempdir().expect("temp dir");
    let library_root = temp.path().canonicalize().expect("canonical library root");
    let units_path = library_root
        .join("sysml.library")
        .join("Domain Libraries")
        .join("Quantities and Units")
        .join("FixtureUnits.sysml");
    fs::create_dir_all(
        units_path
            .parent()
            .expect("fixture units parent directory exists"),
    )
    .expect("create units fixture directory");
    fs::write(&units_path, UNITS_CATALOG).expect("write units fixture");
    let units_uri = Url::from_file_path(&units_path).expect("units uri");
    // Keep the temporary directory alive for the lifetime of the test process.
    std::mem::forget(temp);
    let warning = store_document_text(state, &units_uri, UNITS_CATALOG.to_string());
    assert!(warning.is_none());
    units_uri
}

#[test]
fn store_apply_and_remove_document_keeps_index_and_symbol_table_in_sync() {
    let uri = fixture_uri();
    let mut state = ServerState::default();

    let warning = store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Engine; part motor : Engine; }".to_string(),
    );
    assert!(warning.is_none());
    assert!(state.index.contains_key(&uri));
    let first_entry = state.index.get(&uri).expect("stored doc");
    assert!(first_entry.parse_metadata.parse_time_ms > 0);
    assert!(!first_entry.parse_metadata.parse_cached);
    assert!(!state.symbol_table.is_empty());
    assert!(!state.semantic_graph.nodes_for_uri(&uri).is_empty());

    let warnings = apply_document_changes(
        &mut state,
        &uri,
        2,
        vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 24), Position::new(0, 30))),
            range_length: None,
            text: "Motor".to_string(),
        }],
    );
    assert!(warnings.is_empty());
    assert!(state
        .index
        .get(&uri)
        .expect("updated doc")
        .content
        .contains("Motor"));
    assert!(
        state
            .index
            .get(&uri)
            .expect("updated doc")
            .parse_metadata
            .parse_time_ms
            > 0
    );
    assert!(
        !state
            .index
            .get(&uri)
            .expect("updated doc")
            .parse_metadata
            .parse_cached
    );

    remove_document(&mut state, &uri);
    assert!(!state.index.contains_key(&uri));
    assert!(state.semantic_graph.nodes_for_uri(&uri).is_empty());
    assert!(state.symbol_table.iter().all(|entry| entry.uri != uri));
}

#[test]
fn fast_apply_updates_document_without_running_workspace_evaluation() {
    let uri = fixture_uri();
    let mut state = ServerState::default();

    store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Rocket { attribute mass = 1 + 1; } }".to_string(),
    );
    let mass = find_attribute_node(&state, &uri, "mass");
    assert!(
        matches!(expression(&state, mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.value == Some(sysml_model::EvaluatedValue::Integer(2)))
    );

    let warnings = apply_document_changes_fast(
        &mut state,
        &uri,
        2,
        vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 54), Position::new(0, 55))),
            range_length: None,
            text: "2".to_string(),
        }],
    );
    assert!(warnings.is_empty());
    assert!(state
        .index
        .get(&uri)
        .expect("updated doc")
        .content
        .contains("1 + 2"));
    let mass = find_attribute_node(&state, &uri, "mass");
    assert!(matches!(
        expression(&state, mass),
        sysml_model::ExpressionEvaluationQuery::NotRun
    ));
}

#[test]
fn rebuild_all_document_links_relinks_library_documents_after_dependency_ingest() {
    let temp = tempfile::tempdir().expect("temp dir");
    let library_root = temp.path().canonicalize().expect("canonical library root");
    let importer_uri =
        Url::from_file_path(library_root.join("AImporter.sysml")).expect("importer uri");
    let dependency_uri =
        Url::from_file_path(library_root.join("ZBase.sysml")).expect("dependency uri");
    let library_root_uri = Url::from_file_path(&library_root).expect("library root uri");
    let mut state = ServerState::default();
    state.library_paths = vec![library_root_uri];

    store_document_text(
            &mut state,
            &importer_uri,
            "package Demo { import Base::*; part def RuntimeCluster { attribute clusterName : Name; } }"
                .to_string(),
        );
    store_document_text(
        &mut state,
        &dependency_uri,
        "package Base { attribute def Name; }".to_string(),
    );

    rebuild_all_document_links(&mut state);

    let rebuilt_diagnostics = diagnostics_core::collect_document_diagnostics(
        state.published_model.as_deref(),
        &importer_uri,
        diagnostics_core::lsp_reporting(),
        diagnostics_core::lsp_postprocess_options(),
    );
    assert!(
        rebuilt_diagnostics.iter().all(|d| {
            d.code.as_ref().is_none_or(|code| {
                !matches!(
                    code,
                    NumberOrString::String(value) if value == "unresolved_type_reference"
                )
            })
        }),
        "expected no unresolved_type_reference after full relink, got: {rebuilt_diagnostics:#?}"
    );
}

#[test]
fn rebuild_all_document_links_relinks_public_reexport_chains_after_dependency_ingest() {
    let temp = tempfile::tempdir().expect("temp dir");
    let library_root = temp.path().canonicalize().expect("canonical library root");
    let importer_uri =
        Url::from_file_path(library_root.join("CImporter.sysml")).expect("importer uri");
    let reexport_uri =
        Url::from_file_path(library_root.join("BReexport.sysml")).expect("reexport uri");
    let dependency_uri =
        Url::from_file_path(library_root.join("ABase.sysml")).expect("dependency uri");
    let library_root_uri = Url::from_file_path(&library_root).expect("library root uri");
    let mut state = ServerState::default();
    state.library_paths = vec![library_root_uri];

    store_document_text(
            &mut state,
            &importer_uri,
            "package Consumer { import Domain::*; part def RuntimeCluster { attribute clusterName : Name; } }"
                .to_string(),
        );
    store_document_text(
        &mut state,
        &reexport_uri,
        "package Domain { public import Base::*; }".to_string(),
    );
    store_document_text(
        &mut state,
        &dependency_uri,
        "package Base { attribute def Name; }".to_string(),
    );

    rebuild_all_document_links(&mut state);

    let rebuilt_diagnostics = diagnostics_core::collect_document_diagnostics(
        state.published_model.as_deref(),
        &importer_uri,
        diagnostics_core::lsp_reporting(),
        diagnostics_core::lsp_postprocess_options(),
    );
    assert!(
            rebuilt_diagnostics.iter().all(|d| {
                d.code.as_ref().is_none_or(|code| {
                    !matches!(
                        code,
                        NumberOrString::String(value) if value == "unresolved_type_reference"
                    )
                })
            }),
            "expected no unresolved_type_reference after public re-export relink, got: {rebuilt_diagnostics:#?}"
        );
}

#[test]
fn store_document_text_publishes_typed_evaluation() {
    let uri = fixture_uri();
    let mut state = ServerState::default();
    let warning = store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Rocket { attribute mass = (1 + 2); } }".to_string(),
    );
    assert!(warning.is_none());

    let mass = find_attribute_node(&state, &uri, "mass");
    assert!(
        matches!(expression(&state, mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.status == sysml_model::EvaluationStatus::Ok && result.value == Some(sysml_model::EvaluatedValue::Integer(3)) && result.unit.is_none())
    );
}

#[test]
fn rebuild_all_document_links_recomputes_typed_evaluation() {
    let uri = fixture_uri();
    let mut state = ServerState::default();
    store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Rocket { attribute mass = (8 + 4) / 3; } }".to_string(),
    );

    rebuild_all_document_links(&mut state);

    let mass = find_attribute_node(&state, &uri, "mass");
    assert!(
        matches!(expression(&state, mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.value == Some(sysml_model::EvaluatedValue::Integer(4)))
    );
}

#[test]
fn store_document_text_resolves_referenced_attributes() {
    let uri = fixture_uri();
    let mut state = ServerState::default();
    let warning = store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Rocket { attribute base = 10; attribute mass = base + 5; } }"
            .to_string(),
    );
    assert!(warning.is_none());

    let mass = find_attribute_node(&state, &uri, "mass");
    assert!(
        matches!(expression(&state, mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.value == Some(sysml_model::EvaluatedValue::Integer(15)))
    );
}

#[test]
fn rebuild_all_document_links_recomputes_referenced_attributes() {
    let uri = fixture_uri();
    let mut state = ServerState::default();
    store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute base = 20; attribute offset = base + 2; attribute mass = offset + 3; } }"
                .to_string(),
        );

    rebuild_all_document_links(&mut state);

    let mass = find_attribute_node(&state, &uri, "mass");
    assert!(
        matches!(expression(&state, mass), sysml_model::ExpressionEvaluationQuery::Result(result) if result.value == Some(sysml_model::EvaluatedValue::Integer(25)))
    );
}

#[test]
fn store_document_text_reports_unit_evaluation_unsupported_without_typed_unit_facts() {
    let mut state = ServerState::default();
    let _units_uri = register_units_library_document(&mut state);
    let uri = fixture_uri();
    let warning = store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Rocket { attribute distance = 1 [m] + 50 [cm] + 1 [ft]; } }"
            .to_string(),
    );
    assert!(warning.is_none());

    let distance = find_attribute_node(&state, &uri, "distance");
    assert!(
        matches!(expression(&state, distance), sysml_model::ExpressionEvaluationQuery::Result(result) if result.status == sysml_model::EvaluationStatus::Unsupported && result.value.is_none() && result.unit.is_none())
    );
}

#[test]
fn rebuild_all_document_links_keeps_unit_evaluation_explicitly_unsupported() {
    let mut state = ServerState::default();
    let _units_uri = register_units_library_document(&mut state);
    let uri = fixture_uri();
    store_document_text(
        &mut state,
        &uri,
        "package Demo { part def Rocket { attribute distance = 1 [m] + 1 [ft]; } }".to_string(),
    );

    rebuild_all_document_links(&mut state);

    let distance = find_attribute_node(&state, &uri, "distance");
    assert!(
        matches!(expression(&state, distance), sysml_model::ExpressionEvaluationQuery::Result(result) if result.status == sysml_model::EvaluationStatus::Unsupported && result.value.is_none() && result.unit.is_none())
    );
}
