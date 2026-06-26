//! Optional regression against the sysml-robot-vacuum-cleaner showcase model.
//!
//! Default fixture: `third_party/sysml-robot-vacuum-cleaner` (see `scripts/fetch-robot-vacuum-cleaner.sh`).
//! Override with `SYSML_ROBOT_VACUUM_DIR` pointing at a checkout root containing `model/`.

#[path = "../../../../tests/fixtures/robot_vacuum_fixture.rs"]
mod robot_vacuum_fixture;

use kernel::build_sysml_visualization_for_paths;
use robot_vacuum_fixture::require_robot_vacuum_fixture;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;
use std::collections::HashMap;
use tower_lsp::lsp_types::NumberOrString;

fn diagnostic_code_counts(report: &kernel::ValidationReport) -> HashMap<String, usize> {
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

    let model_views: Vec<_> = probe
        .view_candidates
        .iter()
        .filter(|candidate| candidate.id.starts_with("ModelViews::"))
        .collect();
    assert_eq!(
        model_views.len(),
        3,
        "expected exactly 3 ModelViews catalog views, got {}",
        model_views.len()
    );
    for candidate in &model_views {
        assert!(
            candidate.supported,
            "view '{}' should be supported (view_type={:?}, renderer={:?})",
            candidate.name, candidate.view_type, candidate.renderer_view
        );
    }

    let product_structure = build_sysml_visualization_for_paths(
        &model_dir,
        Some(&root),
        &[],
        "general-view",
        Some("productStructure"),
    )
    .expect("product structure visualization");
    assert!(
        product_structure.empty_state_message.is_none(),
        "productStructure should render as GeneralView: {:?}",
        product_structure.empty_state_message
    );
    assert_eq!(
        product_structure
            .view_candidates
            .iter()
            .find(|c| c.name == "productStructure")
            .and_then(|c| c.renderer_view.as_deref()),
        Some("general-view"),
        "productStructure should map to general-view renderer"
    );
    let tree_graph = product_structure
        .general_view_graph
        .as_ref()
        .or(product_structure.graph.as_ref())
        .expect("graph for productStructure");
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
        "productStructure should show the robot part tree, got {} part usages",
        part_nodes.len()
    );
    assert!(
        !tree_graph
            .nodes
            .iter()
            .any(|node| node.id.contains("RequirementRole#metadata")),
        "productStructure should not include metadata annotation nodes"
    );

    let functional_architecture = build_sysml_visualization_for_paths(
        &model_dir,
        Some(&root),
        &[],
        "general-view",
        Some("functionalArchitecture"),
    )
    .expect("functional architecture visualization");
    assert!(
        functional_architecture.empty_state_message.is_none(),
        "functionalArchitecture should render as GeneralView: {:?}",
        functional_architecture.empty_state_message
    );
    assert_eq!(
        functional_architecture
            .view_candidates
            .iter()
            .find(|c| c.name == "functionalArchitecture")
            .and_then(|c| c.renderer_view.as_deref()),
        Some("general-view"),
        "functionalArchitecture should map to general-view renderer"
    );
    let func_graph = functional_architecture
        .general_view_graph
        .as_ref()
        .or(functional_architecture.graph.as_ref())
        .expect("graph for functionalArchitecture");
    let action_nodes: Vec<_> = func_graph
        .nodes
        .iter()
        .filter(|node| node.element_type.to_lowercase().contains("action"))
        .collect();
    assert!(
        action_nodes.len() >= 5,
        "functionalArchitecture should show capability actions, got {} action nodes",
        action_nodes.len()
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
    assert!(
        requirements_traceability
            .projection_hints
            .as_ref()
            .and_then(|hints| hints.grid_layout.as_deref())
            == Some("traceability"),
        "requirementsTraceability should expose traceability projection hints"
    );
    let trace_graph = requirements_traceability
        .general_view_graph
        .as_ref()
        .or(requirements_traceability.graph.as_ref())
        .expect("graph for requirementsTraceability");
    assert!(
        trace_graph.nodes.len() >= 10,
        "requirementsTraceability should include linked elements, got {} nodes",
        trace_graph.nodes.len()
    );
    let trace_edges: Vec<_> = trace_graph
        .edges
        .iter()
        .filter(|edge| {
            matches!(
                edge.rel_type.to_lowercase().as_str(),
                "derivation" | "satisfy" | "verify" | "subject"
            )
        })
        .collect();
    assert!(
        trace_edges.len() >= 5,
        "requirementsTraceability should show traceability links, got {} edges",
        trace_edges.len()
    );
    assert!(
        trace_graph
            .edges
            .iter()
            .any(|edge| edge.rel_type.eq_ignore_ascii_case("derivation")),
        "requirementsTraceability should include need→requirement derivation links"
    );
    assert!(
        trace_graph
            .edges
            .iter()
            .any(|edge| edge.rel_type.eq_ignore_ascii_case("satisfy")),
        "requirementsTraceability should include design→requirement satisfy links"
    );
    assert!(
        !trace_graph
            .nodes
            .iter()
            .any(|node| node.id.contains("RequirementRole#metadata")),
        "requirementsTraceability should not include metadata annotation nodes"
    );
}
