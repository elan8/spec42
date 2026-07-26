//! Optional regression against the sysml-robot-vacuum-cleaner showcase model.
//!
//! Default fixture: `third_party/sysml-robot-vacuum-cleaner` (see `scripts/fetch-robot-vacuum-cleaner.sh`).
//! Override with `SYSML_ROBOT_VACUUM_DIR` pointing at a checkout root containing `model/`.

#[path = "../../../../tests/fixtures/robot_vacuum_fixture.rs"]
mod robot_vacuum_fixture;

use lsp_server::build_sysml_visualization_for_paths;
use robot_vacuum_fixture::require_robot_vacuum_fixture;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;
use std::collections::HashMap;
use tower_lsp::lsp_types::NumberOrString;

fn diagnostic_code_counts(report: &lsp_server::ValidationReport) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for document in &report.documents {
        for diagnostic in &document.diagnostics {
            if let Some(NumberOrString::String(code)) = &diagnostic.code {
                *counts.entry(code.clone()).or_default() += 1;
            }
        }
    }
    counts
}

#[test]
#[ignore = "local showcase: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -- --ignored"]
fn robot_vacuum_showcase_diagnostic_baseline() {
    let (root, model_dir) = require_robot_vacuum_fixture();

    let cli = Cli {
        config_path: None,
        library_paths: Vec::new(),
        stdlib_path: None,
        domain_libraries_path: None,
        no_stdlib: false,
        stdio: false,
        command: None,
    };
    let report = perform_check(
        &cli,
        &CheckArgs {
            path: model_dir,
            workspace_root: Some(root),
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
            strict_diagnostics: false,
        },
    )
    .expect("robot vacuum validation report");

    let code_counts = diagnostic_code_counts(&report);

    assert_eq!(report.summary.error_count, 0, "expected zero errors");
    assert_eq!(
        code_counts
            .get("verification_case_invalid_shape")
            .copied()
            .unwrap_or(0),
        0,
        "verification cases with then-action and no explicit return are valid SysML v2 (S42-LIM-003)"
    );
    assert_eq!(
        code_counts
            .get("unresolved_pending_relationship")
            .copied()
            .unwrap_or(0),
        0,
        "unqualified verify requirement names must resolve via private import SystemRequirements::*"
    );
    assert_eq!(
        code_counts
            .get("unresolved_redefines_target")
            .copied()
            .unwrap_or(0),
        0,
        "specialized part local attributes must not emit unresolved_redefines_target"
    );
    assert_eq!(
        code_counts.get("unknown_unit_symbol").copied().unwrap_or(0),
        0,
        "MonetaryUnits::EUR should be recognized from bundled domain libraries"
    );
    assert_eq!(
        code_counts
            .get("analysis_evaluation_unresolved")
            .copied()
            .unwrap_or(0),
        0,
        "VerdictKind::pass verification returns should evaluate"
    );
    assert_eq!(
        code_counts
            .get("multiple_initial_states")
            .copied()
            .unwrap_or(0),
        0,
        "named transitions with first source must not count as initial transitions"
    );
}

#[test]
#[ignore = "local showcase: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -- --ignored"]
fn robot_vacuum_showcase_model_views_are_supported() {
    let (root, model_dir) = require_robot_vacuum_fixture();

    let probe =
        build_sysml_visualization_for_paths(&model_dir, Some(&root), &[], "general-view", None)
            .expect("robot vacuum visualization probe");

    let expected_views = [
        "productDecomposition",
        "interconnections",
        "firmwareRuntime",
        "requirementsTraceability",
        "cliffSafeStopGoldenThread",
        "selectedParts",
    ];
    let model_views: Vec<_> = probe
        .view_candidates
        .iter()
        .filter(|candidate| {
            candidate.id.starts_with("ModelViews::")
                && expected_views.contains(&candidate.name.as_str())
        })
        .collect();
    assert_eq!(
        model_views.len(),
        6,
        "expected the 6 public lean ModelViews catalog views, got {}: {:?}",
        model_views.len(),
        model_views
            .iter()
            .map(|candidate| (&candidate.id, &candidate.name, &candidate.view_type))
            .collect::<Vec<_>>()
    );
    for candidate in &model_views {
        assert!(
            candidate.supported,
            "view '{}' should be supported (view_type={:?}, renderer={:?})",
            candidate.name, candidate.view_type, candidate.renderer_view
        );
    }

    let product_decomposition = build_sysml_visualization_for_paths(
        &model_dir,
        Some(&root),
        &[],
        "general-view",
        Some("productDecomposition"),
    )
    .expect("product decomposition visualization");
    assert!(
        product_decomposition.empty_state_message.is_none(),
        "productDecomposition should render as GeneralView: {:?}",
        product_decomposition.empty_state_message
    );
    assert_eq!(
        product_decomposition
            .view_candidates
            .iter()
            .find(|c| c.name == "productDecomposition")
            .and_then(|c| c.renderer_view.as_deref()),
        Some("general-view"),
        "productDecomposition should map to general-view renderer"
    );
    let tree_graph = product_decomposition
        .general_view_graph
        .as_ref()
        .or(product_decomposition.graph.as_ref())
        .expect("graph for productDecomposition");
    let part_nodes: Vec<_> = tree_graph
        .nodes
        .iter()
        .filter(|node| {
            node.element_type.to_lowercase().contains("part")
                && !node.element_type.to_lowercase().contains("def")
        })
        .collect();
    assert!(
        part_nodes.len() >= 5,
        "productDecomposition should show the robot part tree, got {} part usages",
        part_nodes.len()
    );
    assert!(
        !tree_graph
            .nodes
            .iter()
            .any(|node| node.id.contains("RequirementRole#metadata")),
        "productDecomposition should not include metadata annotation nodes"
    );

    let firmware_runtime = build_sysml_visualization_for_paths(
        &model_dir,
        Some(&root),
        &[],
        "general-view",
        Some("firmwareRuntime"),
    )
    .expect("firmware runtime visualization");
    assert!(
        firmware_runtime.empty_state_message.is_none(),
        "firmwareRuntime should render as GeneralView: {:?}",
        firmware_runtime.empty_state_message
    );
    assert_eq!(
        firmware_runtime
            .view_candidates
            .iter()
            .find(|c| c.name == "firmwareRuntime")
            .and_then(|c| c.renderer_view.as_deref()),
        Some("general-view"),
        "firmwareRuntime should map to general-view renderer"
    );
    let runtime_graph = firmware_runtime
        .general_view_graph
        .as_ref()
        .or(firmware_runtime.graph.as_ref())
        .expect("graph for firmwareRuntime");
    let task_nodes: Vec<_> = runtime_graph
        .nodes
        .iter()
        .filter(|node| {
            matches!(
                node.name.as_str(),
                "missionController"
                    | "safetySupervisor"
                    | "actuatorControl"
                    | "sensorAcquisition"
                    | "navigation"
                    | "powerManager"
                    | "appService"
            )
        })
        .collect();
    assert!(
        task_nodes.len() == 7,
        "firmwareRuntime should show seven task usages, got {}",
        task_nodes.len()
    );

    let requirements_traceability = build_sysml_visualization_for_paths(
        &model_dir,
        Some(&root),
        &[],
        "general-view",
        Some("requirementsTraceability"),
    )
    .expect("requirements traceability visualization");
    assert!(
        requirements_traceability.empty_state_message.is_none(),
        "requirementsTraceability should render as filtered GeneralView: {:?}",
        requirements_traceability.empty_state_message
    );
    assert_eq!(
        requirements_traceability
            .view_candidates
            .iter()
            .find(|c| c.name == "requirementsTraceability")
            .and_then(|c| c.renderer_view.as_deref()),
        Some("general-view"),
        "requirementsTraceability should map to general-view renderer"
    );
    let trace_graph = requirements_traceability
        .graph
        .as_ref()
        .or(requirements_traceability.general_view_graph.as_ref())
        .expect("graph for requirementsTraceability");
    assert!(
        trace_graph.nodes.len() >= 10,
        "requirementsTraceability should include linked elements, got {} nodes",
        trace_graph.nodes.len()
    );
    assert!(
        !trace_graph
            .nodes
            .iter()
            .any(|node| node.id.contains("RequirementRole#metadata")),
        "requirementsTraceability should not include metadata annotation nodes"
    );
}
