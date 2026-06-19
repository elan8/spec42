//! JSON schema parity between Rust PreparedViewDto and TS prepare/types.ts consumers.

use std::fs;
use std::path::PathBuf;

use semantic_core::{
    prepare_interconnection_prepared_view, prepare_view_from_visualization, InterconnectionSceneDto,
    PreparedViewDto, SysmlVisualizationResultDto,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../shared/diagram-renderer/test-fixtures/interconnection")
        .join(name)
}

#[derive(serde::Deserialize)]
struct InterconnectionFixture {
    view: String,
    #[serde(rename = "interconnectionScene")]
    interconnection_scene: InterconnectionSceneDto,
}

fn response_from_fixture(fixture: InterconnectionFixture) -> SysmlVisualizationResultDto {
    SysmlVisualizationResultDto {
        version: 0,
        model_ready: true,
        view: fixture.view,
        workspace_root_uri: "file:///fixture".to_string(),
        view_candidates: Vec::new(),
        selected_view: None,
        selected_view_name: Some(fixture.interconnection_scene.view.name.clone()),
        empty_state_message: None,
        package_groups: None,
        graph: None,
        general_view_graph: None,
        workspace_model: None,
        activity_diagrams: None,
        activity_diagram_candidates: None,
        sequence_diagrams: None,
        sequence_diagram_candidates: None,
        state_machines: None,
        state_machine_candidates: None,
        ibd: None,
        interconnection_scene: Some(fixture.interconnection_scene),
        stats: None,
        projection_hints: None,
        prepared_view: None,
    }
}

#[test]
fn deserializes_ts_interconnection_fixture_into_prepared_view_dto() {
    let raw = fs::read_to_string(fixture_path("scene-two-part-chain.json"))
        .expect("read interconnection fixture");
    let fixture: InterconnectionFixture =
        serde_json::from_str(&raw).expect("deserialize interconnection fixture");
    let response = response_from_fixture(fixture);
    let prepared = prepare_interconnection_prepared_view(&response).expect("prepare interconnection");
    assert_eq!(prepared.view, "interconnection-view");
    assert_eq!(prepared.nodes.len(), 2);
    assert_eq!(prepared.edges.len(), 1);
    let roundtrip: PreparedViewDto =
        serde_json::from_str(&serde_json::to_string(&prepared).expect("serialize prepared view"))
            .expect("deserialize prepared view json");
    assert_eq!(roundtrip, prepared);
}

#[test]
fn prepare_view_from_visualization_matches_interconnection_fixture() {
    let raw = fs::read_to_string(fixture_path("nested-ring-minimal.json"))
        .expect("read nested ring fixture");
    let fixture: InterconnectionFixture =
        serde_json::from_str(&raw).expect("deserialize nested ring fixture");
    let response = response_from_fixture(fixture);
    let prepared =
        prepare_view_from_visualization(&response).expect("prepare view from visualization");
    assert_eq!(prepared.view, "interconnection-view");
    assert!(!prepared.nodes.is_empty());
}
