use super::*;
use crate::semantic::model::{
    ConstructionOwner, EvaluatedValue, EvaluationPublicationState, EvaluationStatus,
    ExpressionEvaluationQuery, RelationshipKind, SemanticNode,
};

fn expression_value(graph: &SemanticGraph, node: &SemanticNode) -> Option<EvaluatedValue> {
    match graph.expression_evaluation_for(node) {
        ExpressionEvaluationQuery::Result(result) => result.value.clone(),
        ExpressionEvaluationQuery::NotRun | ExpressionEvaluationQuery::NotApplicable => None,
    }
}

fn parse(_uri: &Url, content: &str) -> sysml_v2_parser::RootNamespace {
    sysml_v2_parser::parse(content).expect("parse")
}

#[test]
fn patch_with_none_clears_the_uris_nodes() {
    let uri = Url::parse("file:///demo.sysml").expect("uri");
    let mut graph = SemanticGraph::new();
    let parsed = parse(&uri, "package Demo { part def Engine; }");
    patch_graph_for_document(&mut graph, &uri, Some(&parsed), true);
    assert!(!graph.nodes_for_uri(&uri).is_empty());

    patch_graph_for_document(&mut graph, &uri, None, true);
    assert!(graph.nodes_for_uri(&uri).is_empty());
}

#[test]
fn patch_matches_manual_build_merge_and_cross_edges() {
    let uri = Url::parse("file:///demo.sysml").expect("uri");
    let content = "package Demo { part def Engine; part motor : Engine; }";
    let parsed = parse(&uri, content);

    let mut patched = SemanticGraph::new();
    patch_graph_for_document(&mut patched, &uri, Some(&parsed), false);

    let mut manual = SemanticGraph::new();
    manual.remove_nodes_for_uri(&uri);
    let doc_graph = build_graph_from_doc(&parsed, &uri);
    manual.merge(doc_graph);
    add_cross_document_edges_for_uri(&mut manual, &uri);

    let patched_names: std::collections::BTreeSet<_> = patched
        .nodes_for_uri(&uri)
        .iter()
        .map(|node| node.id.qualified_name.clone())
        .collect();
    let manual_names: std::collections::BTreeSet<_> = manual
        .nodes_for_uri(&uri)
        .iter()
        .map(|node| node.id.qualified_name.clone())
        .collect();
    assert_eq!(patched_names, manual_names);
    assert!(!patched_names.is_empty());
}

#[test]
fn evaluate_false_skips_expression_evaluation() {
    let uri = Url::parse("file:///demo.sysml").expect("uri");
    let parsed = parse(
        &uri,
        "package Demo { part def Rocket { attribute mass = 1 + 2; } }",
    );
    let mut graph = SemanticGraph::new();
    patch_graph_for_document(&mut graph, &uri, Some(&parsed), false);

    let mass = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("mass attribute node");
    assert_eq!(
        graph.evaluation_publication,
        EvaluationPublicationState::NotRun
    );
    assert!(matches!(
        graph.expression_evaluation_for(mass),
        ExpressionEvaluationQuery::NotRun
    ));
}

#[test]
fn evaluate_true_populates_evaluated_value() {
    let uri = Url::parse("file:///demo.sysml").expect("uri");
    let parsed = parse(
        &uri,
        "package Demo { part def Rocket { attribute mass = 1 + 2; } }",
    );
    let mut graph = SemanticGraph::new();
    patch_graph_for_document(&mut graph, &uri, Some(&parsed), true);

    let mass = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("mass attribute node");
    assert_eq!(
        graph.evaluation_publication,
        EvaluationPublicationState::Complete
    );
    assert_eq!(
        expression_value(&graph, mass),
        Some(EvaluatedValue::Integer(3))
    );
}

#[test]
fn full_and_incremental_typed_expression_evaluation_match() {
    let uri = Url::parse("file:///typed-evaluation-parity.sysml").expect("uri");
    let content = r#"
        package Demo {
            requirement def Capacity {
                attribute load = 12;
                attribute limit = 16;
                require constraint { load <= limit }
            }
        }
    "#;
    let document = SysmlDocument {
        uri: uri.clone(),
        content: content.to_string(),
        path_hint: None,
        source_kind: SysmlDocumentSourceKind::Workspace,
        content_digest: None,
        byte_size: None,
    };
    let (full, _) = build_and_link_graph(&[document]).expect("full graph");

    let parsed = parse(&uri, content);
    let mut incremental = SemanticGraph::new();
    patch_graph_for_document(&mut incremental, &uri, Some(&parsed), true);

    let outcomes = |graph: &SemanticGraph| {
        graph
            .node_ids_by_qualified_name
            .get("Demo::Capacity")
            .and_then(|ids| ids.first())
            .and_then(|id| graph.get_node(id))
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.analysis.as_ref())
            .map(|analysis| {
                (
                    analysis.expression.status,
                    analysis.expression.value.clone(),
                    analysis.passed,
                )
            })
            .expect("capacity requirement")
    };

    assert_eq!(outcomes(&incremental), outcomes(&full));
    assert_eq!(
        outcomes(&full),
        (
            EvaluationStatus::Ok,
            Some(EvaluatedValue::Boolean(true)),
            Some(true),
        )
    );
}

/// Track C: `link_parsed_documents_parallel_from`'s `evaluate` parameter must skip
/// expression evaluation (but still do structural relink/dependency-index work) when
/// `false`, and publish `NotRun` when `true` is deferred — the same contract
/// `patch_graph_for_document`'s `evaluate` flag already has, now on the parallel full-build
/// path `lsp_server`'s live-edit relink uses.
#[test]
fn link_parsed_documents_parallel_from_respects_evaluate_flag() {
    let uri = Url::parse("file:///demo.sysml").expect("uri");
    let content = "package Demo { part def Rocket { attribute mass = 1 + 2; } }";
    let parsed = sysml_v2_parser::parse(content).expect("parse");
    let entry = WorkspaceParsedDocument {
        uri: uri.clone(),
        content: content.to_string(),
        parsed,
        parse_time_ms: 1,
        parse_cached: false,
    };

    let (graph_no_eval, _) = link_parsed_documents_parallel_from(
        SemanticGraph::new(),
        vec![(SysmlDocumentSourceKind::Workspace, entry.clone())],
        false,
    );
    let mass_no_eval = graph_no_eval
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("mass attribute node");
    assert!(matches!(
        graph_no_eval.expression_evaluation_for(mass_no_eval),
        ExpressionEvaluationQuery::NotRun
    ));

    let (graph_eval, _) = link_parsed_documents_parallel_from(
        SemanticGraph::new(),
        vec![(SysmlDocumentSourceKind::Workspace, entry)],
        true,
    );
    let mass_eval = graph_eval
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.name == "mass")
        .expect("mass attribute node");
    assert_eq!(
        expression_value(&graph_eval, mass_eval),
        Some(EvaluatedValue::Integer(3))
    );
}

/// Equivalence fixture for `build_and_link_graph` vs. `build_and_link_graph_parallel`
/// Equivalence fixture for sequential vs parallel full-graph rebuild.
/// Exercises cross-document typing (`part mobility : MobilitySubsystem` resolved via
/// import) and cross-document subject edges (`subject robot : AutonomousFloorCleaningRobot`,
/// resolved via `private import Architecture::*`) — the two edge kinds
/// `resolve_cross_document_edges_for_uri` handles and the sequential
/// `link_workspace_relationships` scan is being replaced for. Also includes a
/// same-document `specializes` (`:>`) edge and a library document merged with
/// `merge_skip_existing_qualified_names`, to exercise both phases. Also includes a
/// cross-document derivation connection (`StakeholderNeeds`/`SystemRequirements`,
/// mirroring `requirement_derivation_semantics.rs`), exercising the
/// `link_workspace_derivations` rewiring that runs after parallel cross-document edge
/// resolution — the one thing that phase does not cover.
fn equivalence_fixture_documents() -> Vec<SysmlDocument> {
    const ARCHITECTURE: &str = r#"
package Architecture {
  part def MobilitySubsystem {
    attribute drivePowerW : Real = 28;
  }
  part def AutonomousFloorCleaningRobot {
    part mobility : MobilitySubsystem;
  }
  part def PremiumFloorCleaningRobot :> AutonomousFloorCleaningRobot;
}
"#;
    const ANALYSIS_CASES: &str = r#"
package AnalysisCases {
  private import Architecture::*;

  analysis def TotalPowerConsumptionAnalysis {
    attribute powerBudgetW : Real = 55;
    subject robot : AutonomousFloorCleaningRobot;
    return ref powerWithinBudget {
      return sum(robot.mobility.drivePowerW) <= powerBudgetW;
    }
  }
}
"#;
    const UNITS_LIBRARY: &str = r#"
package Units {
  attribute <m> 'metre' : LengthUnit;
}
"#;
    const STAKEHOLDER_NEEDS: &str = r#"
package StakeholderNeeds {
  requirement def CleanLargeAreas;
  requirement cleanLargeAreas : CleanLargeAreas;
}
"#;
    const SYSTEM_REQUIREMENTS: &str = r#"
package SystemRequirements {
  private import StakeholderNeeds::*;

  requirement def CleanAtLeastEightySquareMetersPerCharge;
  requirement cleanAtLeastEighty : CleanAtLeastEightySquareMetersPerCharge;

  #derivation connection {
    end #original ::> cleanLargeAreas;
    end #derive ::> cleanAtLeastEighty;
  }
}
"#;
    vec![
        SysmlDocument::from_memory_path(
            "equivalence",
            "Architecture.sysml",
            ARCHITECTURE.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture uri"),
        SysmlDocument::from_memory_path(
            "equivalence",
            "AnalysisCases.sysml",
            ANALYSIS_CASES.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("analysis uri"),
        SysmlDocument::from_memory_path(
            "equivalence",
            "Units.sysml",
            UNITS_LIBRARY.to_string(),
            SysmlDocumentSourceKind::Library,
            None,
            None,
        )
        .expect("library uri"),
        SysmlDocument::from_memory_path(
            "equivalence",
            "StakeholderNeeds.sysml",
            STAKEHOLDER_NEEDS.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("stakeholder needs uri"),
        SysmlDocument::from_memory_path(
            "equivalence",
            "SystemRequirements.sysml",
            SYSTEM_REQUIREMENTS.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("system requirements uri"),
    ]
}

fn node_qualified_names(graph: &SemanticGraph) -> std::collections::BTreeSet<String> {
    graph
        .graph
        .node_weights()
        .map(|node| node.id.qualified_name.clone())
        .collect()
}

fn edge_triples(graph: &SemanticGraph) -> std::collections::BTreeSet<(String, String, String)> {
    graph
        .graph
        .edge_indices()
        .map(|edge_idx| {
            let (src, tgt) = graph
                .graph
                .edge_endpoints(edge_idx)
                .expect("edge has endpoints");
            (
                graph.graph[src].id.qualified_name.clone(),
                graph.graph[tgt].id.qualified_name.clone(),
                graph.graph[edge_idx].kind.as_str().to_string(),
            )
        })
        .collect()
}

#[test]
fn parallel_build_matches_sequential_build_nodes_and_edges() {
    let documents = equivalence_fixture_documents();

    let (sequential_graph, sequential_parsed) =
        build_and_link_graph(&documents).expect("sequential build");
    let (parallel_graph, parallel_parsed) = build_and_link_graph_parallel(&documents);

    let sequential_nodes = node_qualified_names(&sequential_graph);
    let parallel_nodes = node_qualified_names(&parallel_graph);
    assert_eq!(
        sequential_nodes, parallel_nodes,
        "node sets differ between sequential and parallel full build"
    );
    assert!(!sequential_nodes.is_empty(), "fixture should produce nodes");

    let sequential_edges = edge_triples(&sequential_graph);
    let parallel_edges = edge_triples(&parallel_graph);
    assert_eq!(
        sequential_edges, parallel_edges,
        "edge sets differ between sequential and parallel full build"
    );
    assert!(!sequential_edges.is_empty(), "fixture should produce edges");

    let derivation_edge = (
        "StakeholderNeeds::cleanLargeAreas".to_string(),
        "SystemRequirements::cleanAtLeastEighty".to_string(),
        RelationshipKind::Derivation.as_str().to_string(),
    );
    assert!(
        sequential_edges.contains(&derivation_edge),
        "sequential build should contain the fixture's derivation-connection edge"
    );
    assert!(
        parallel_edges.contains(&derivation_edge),
        "parallel build should contain the fixture's derivation-connection edge \
             (link_workspace_derivations is the only phase that wires it)"
    );

    assert_eq!(
        sequential_parsed.len(),
        parallel_parsed.len(),
        "parsed document count should match"
    );
}

#[test]
fn parallel_build_evaluates_expressions_like_sequential_build() {
    let documents = equivalence_fixture_documents();
    let (sequential_graph, _) = build_and_link_graph(&documents).expect("sequential build");
    let (parallel_graph, _) = build_and_link_graph_parallel(&documents);

    let find_drive_power = |graph: &SemanticGraph| {
        graph
            .graph
            .node_weights()
            .find(|node| node.name == "drivePowerW")
            .and_then(|node| expression_value(graph, node))
    };
    let sequential_value = find_drive_power(&sequential_graph);
    let parallel_value = find_drive_power(&parallel_graph);
    assert_eq!(sequential_value, parallel_value);
    assert_eq!(sequential_value, Some(EvaluatedValue::Integer(28)));
}

// Equivalence for the Tier 2 unified-incremental-engine Phase 4 extraction:
// `link_parsed_documents_parallel` (fed pre-parsed documents, skipping the parse step)
// must produce the same graph as `build_and_link_graph_parallel` (which parses
// internally) for the same inputs. This is what lets `workspace::IncrementalWorkspace`
// build from already-parsed documents (e.g. served by a parse cache) without
// duplicating the merge/link sequence a second time.
//
// --- Track B Phase 1: relationship-linking frontier differential tests ---
//
// These compare `patch_graph_for_document_scoped` (frontier-scoped relink) against a full
// `build_and_link_graph` rebuild of the same document set after the same edit. See the
// Track B Phase 1 plan for the design this validates.

fn memory_doc(name: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "frontier",
        name,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("uri")
}

fn two_file_fixture(
    a_name: &str,
    a_content: &str,
    b_name: &str,
    b_content: &str,
) -> (Url, SysmlDocument, Url, SysmlDocument) {
    let a_doc = memory_doc(a_name, a_content);
    let b_doc = memory_doc(b_name, b_content);
    let a_uri = a_doc.uri.clone();
    let b_uri = b_doc.uri.clone();
    (a_uri, a_doc, b_uri, b_doc)
}

fn two_file_typing_fixture() -> (Url, SysmlDocument, Url, SysmlDocument) {
    two_file_fixture(
        "A.sysml",
        "package A { part def Thing; }",
        "B.sysml",
        "package B { private import A::*; part x : Thing; }",
    )
}

fn apply_scoped_patch(graph: &mut SemanticGraph, uri: &Url, content: &str) {
    let parsed = sysml_v2_parser::parse(content).expect("parse");
    patch_graph_for_document_scoped(graph, uri, Some(&parsed), true);
}

/// A rename in `A.sysml` that breaks `B.sysml`'s cross-file typing reference must drop the
/// edge identically under the scoped-patch path and a full rebuild — i.e. the frontier
/// mechanism doesn't leave a stale edge pointing at a node that no longer exists.
#[test]
fn frontier_patch_drops_edge_when_referenced_type_is_renamed_away() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_typing_fixture();
    let renamed_a = "package A { part def Widget; }";

    let mut graph = SemanticGraph::new();
    let initial = sysml_v2_parser::parse(&a_doc.content).expect("parse a");
    patch_graph_for_document_scoped(&mut graph, &a_uri, Some(&initial), false);
    apply_scoped_patch(&mut graph, &b_doc.uri, &b_doc.content);
    // Settle: b's initial patch already ran with evaluate:true via apply_scoped_patch, but
    // a's first patch used evaluate:false above so cross-doc edges get a clean second pass.
    finalize_and_evaluate_frontier(&mut graph, &a_uri);

    apply_scoped_patch(&mut graph, &a_uri, renamed_a);

    let baseline_docs = vec![
        SysmlDocument::from_memory_path(
            "frontier",
            "A.sysml",
            renamed_a.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("a uri"),
        b_doc.clone(),
    ];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        node_qualified_names(&graph),
        node_qualified_names(&baseline_graph)
    );
    assert_eq!(edge_triples(&graph), edge_triples(&baseline_graph));
    let typing_edge = (
        "B::x".to_string(),
        "A::Thing".to_string(),
        RelationshipKind::Typing.as_str().to_string(),
    );
    assert!(
        !edge_triples(&graph).contains(&typing_edge),
        "stale typing edge to the renamed-away type should be gone"
    );
}

/// The scenario the frontier mechanism is riskiest for: `A.sysml` is edited twice — once
/// breaking `B.sysml`'s reference, then again restoring it. An earlier (replaced) design
/// built the frontier from a cache of *resolved edges* rather than static dependencies, and
/// under that design `B` silently dropped out of the reverse index on the first edit and
/// was never re-checked on the second, leaving its typing edge missing even though the type
/// was back. This test is the regression gate for that bug — see the Track B Phase 1 plan's
/// design-gap note.
#[test]
fn frontier_patch_restores_edge_after_referenced_type_reappears() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_typing_fixture();
    let renamed_a = "package A { part def Widget; }";

    let mut graph = SemanticGraph::new();
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    apply_scoped_patch(&mut graph, &b_doc.uri, &b_doc.content);

    // Break it.
    apply_scoped_patch(&mut graph, &a_uri, renamed_a);
    // Restore it.
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);

    let baseline_docs = vec![a_doc.clone(), b_doc.clone()];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        edge_triples(&graph),
        edge_triples(&baseline_graph),
        "B's typing edge to A::Thing should be restored once A is patched back, matching \
             a full rebuild of the same document set"
    );
}

/// Sibling of `frontier_patch_restores_edge_after_referenced_type_reappears` for
/// `Specializes` edges — confirms the static-dependency-based frontier doesn't have a
/// per-relationship-kind gap.
#[test]
fn frontier_patch_restores_specializes_edge_after_referenced_type_reappears() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_fixture(
        "A.sysml",
        "package A { part def Base; }",
        "B.sysml",
        "package B { private import A::*; part def Derived :> Base; }",
    );
    let renamed_a = "package A { part def Renamed; }";

    let mut graph = SemanticGraph::new();
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    apply_scoped_patch(&mut graph, &b_doc.uri, &b_doc.content);
    apply_scoped_patch(&mut graph, &a_uri, renamed_a);
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);

    let baseline_docs = vec![a_doc.clone(), b_doc.clone()];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        edge_triples(&graph),
        edge_triples(&baseline_graph),
        "B's specializes edge to A::Base should be restored once A is patched back"
    );
    let specializes_edge = (
        "B::Derived".to_string(),
        "A::Base".to_string(),
        RelationshipKind::Specializes.as_str().to_string(),
    );
    assert!(edge_triples(&graph).contains(&specializes_edge));
}

/// Sibling of `frontier_patch_restores_edge_after_referenced_type_reappears` for `Subject`
/// edges (analysis/requirement case subjects), reusing the cross-file subject relationship
/// pattern from `equivalence_fixture_documents`.
#[test]
fn frontier_patch_restores_subject_edge_after_referenced_type_reappears() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_fixture(
            "Architecture.sysml",
            "package Architecture { part def Robot; }",
            "AnalysisCases.sysml",
            "package AnalysisCases { private import Architecture::*; analysis def TotalPowerConsumptionAnalysis { subject robot : Robot; } }",
        );
    let renamed_a = "package Architecture { part def RenamedRobot; }";

    let mut graph = SemanticGraph::new();
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    apply_scoped_patch(&mut graph, &b_doc.uri, &b_doc.content);
    apply_scoped_patch(&mut graph, &a_uri, renamed_a);
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);

    let baseline_docs = vec![a_doc.clone(), b_doc.clone()];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        edge_triples(&graph),
        edge_triples(&baseline_graph),
        "AnalysisCases's subject edge to Architecture::Robot should be restored once \
             Architecture is patched back"
    );
    let subject_edge = (
        "AnalysisCases::TotalPowerConsumptionAnalysis".to_string(),
        "Architecture::Robot".to_string(),
        RelationshipKind::Subject.as_str().to_string(),
    );
    assert!(edge_triples(&graph).contains(&subject_edge));
}

/// Three-file specializes chain (A <- B <- C): editing A should only need to refresh B
/// (which statically depends on A), not C (which only depends on B, not A directly) — but
/// the end result must still match a full rebuild either way.
#[test]
fn frontier_patch_handles_three_file_specializes_chain() {
    let a_content = "package A { part def Base { attribute x : Integer; } }";
    let b_content =
        "package B { private import A::*; part def Mid :> Base { attribute y : Integer; } }";
    let c_content = "package C { private import B::*; part def Leaf :> Mid; }";
    let a_doc = memory_doc("A.sysml", a_content);
    let b_doc = memory_doc("B.sysml", b_content);
    let c_doc = memory_doc("C.sysml", c_content);
    let a_uri = a_doc.uri.clone();

    let mut graph = SemanticGraph::new();
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    apply_scoped_patch(&mut graph, &b_doc.uri, &b_doc.content);
    apply_scoped_patch(&mut graph, &c_doc.uri, &c_doc.content);

    let edited_a = "package A { part def Base { attribute x : Integer; attribute z : Integer; } }";
    apply_scoped_patch(&mut graph, &a_uri, edited_a);

    let baseline_docs = vec![
        memory_doc("A.sysml", edited_a),
        b_doc.clone(),
        c_doc.clone(),
    ];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        node_qualified_names(&graph),
        node_qualified_names(&baseline_graph)
    );
    assert_eq!(edge_triples(&graph), edge_triples(&baseline_graph));
    let leaf_to_mid = (
        "C::Leaf".to_string(),
        "B::Mid".to_string(),
        RelationshipKind::Specializes.as_str().to_string(),
    );
    assert!(
        edge_triples(&graph).contains(&leaf_to_mid),
        "C's specializes edge to B::Mid should be unaffected by A's edit"
    );
}

/// A previously-unrelated file that doesn't import or reference the changed URI at all
/// should stay untouched by a scoped patch — the frontier shouldn't wrongly expand to
/// include it, and the result should still match a full rebuild trivially.
#[test]
fn frontier_patch_leaves_unrelated_file_untouched() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_fixture(
        "A.sysml",
        "package A { part def Thing; }",
        "B.sysml",
        "package B { part def Other; }", // no import of / reference to A
    );

    let mut graph = SemanticGraph::new();
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    apply_scoped_patch(&mut graph, &b_doc.uri, &b_doc.content);

    let edited_a = "package A { part def Thing; part def Extra; }";
    apply_scoped_patch(&mut graph, &a_uri, edited_a);

    let baseline_docs = vec![memory_doc("A.sysml", edited_a), b_doc.clone()];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        node_qualified_names(&graph),
        node_qualified_names(&baseline_graph)
    );
    assert_eq!(edge_triples(&graph), edge_triples(&baseline_graph));
}

/// Confirms the deliberately-whole-graph derivation-connection fallback still produces
/// correct results when reached via `patch_graph_for_document_scoped`: a connection in a
/// third file referencing the changed URI's requirement must still rewire correctly.
#[test]
fn frontier_patch_rewires_derivation_connection_after_referenced_file_edit() {
    let stakeholder_content =
            "package StakeholderNeeds { requirement def CleanLargeAreas; requirement cleanLargeAreas : CleanLargeAreas; }";
    let system_content = r#"package SystemRequirements {
  private import StakeholderNeeds::*;
  requirement def CleanAtLeastEightySquareMetersPerCharge;
  requirement cleanAtLeastEighty : CleanAtLeastEightySquareMetersPerCharge;
  #derivation connection {
    end #original ::> cleanLargeAreas;
    end #derive ::> cleanAtLeastEighty;
  }
}"#;
    let stakeholder_doc = memory_doc("StakeholderNeeds.sysml", stakeholder_content);
    let system_doc = memory_doc("SystemRequirements.sysml", system_content);
    let stakeholder_uri = stakeholder_doc.uri.clone();

    let mut graph = SemanticGraph::new();
    apply_scoped_patch(&mut graph, &stakeholder_uri, &stakeholder_doc.content);
    apply_scoped_patch(&mut graph, &system_doc.uri, &system_doc.content);

    // Re-patch StakeholderNeeds with identical content (a no-op content-wise, but exercises
    // the scoped path's remove+rebuild+derivation-rewire sequence for this URI).
    apply_scoped_patch(&mut graph, &stakeholder_uri, &stakeholder_doc.content);

    let baseline_docs = vec![stakeholder_doc.clone(), system_doc.clone()];
    let (baseline_graph, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    let derivation_edge = (
        "StakeholderNeeds::cleanLargeAreas".to_string(),
        "SystemRequirements::cleanAtLeastEighty".to_string(),
        RelationshipKind::Derivation.as_str().to_string(),
    );
    assert!(
        edge_triples(&graph).contains(&derivation_edge),
        "derivation connection should still be wired after a scoped patch"
    );
    assert_eq!(edge_triples(&graph), edge_triples(&baseline_graph));
}

#[test]
fn link_parsed_documents_parallel_matches_build_and_link_graph_parallel() {
    let documents = equivalence_fixture_documents();
    let (expected_graph, expected_parsed) = build_and_link_graph_parallel(&documents);

    let pre_parsed: Vec<SourceTaggedDocument> = documents
        .iter()
        .filter_map(|document| {
            let parsed = sysml_v2_parser::parse(&document.content).ok()?;
            Some((
                document.source_kind,
                WorkspaceParsedDocument {
                    uri: document.uri.clone(),
                    content: document.content.clone(),
                    parsed,
                    parse_time_ms: 1,
                    parse_cached: true,
                },
            ))
        })
        .collect();
    let (actual_graph, actual_parsed) = link_parsed_documents_parallel(pre_parsed, true);

    assert_eq!(
        node_qualified_names(&actual_graph),
        node_qualified_names(&expected_graph)
    );
    assert_eq!(edge_triples(&actual_graph), edge_triples(&expected_graph));
    assert_eq!(actual_parsed.len(), expected_parsed.len());
}

/// Equivalence for `link_parsed_documents_parallel_from` (Phase 4): merging the
/// remaining documents onto a base graph that already contains one of them (mirroring
/// `lsp_server`'s cached-library-subgraph reuse) should produce the same graph as
/// building everything together in one call — cross-document edge resolution must see
/// the base graph's URIs too, not just the newly-merged ones.
#[test]
fn link_parsed_documents_parallel_from_matches_building_everything_together() {
    let documents = equivalence_fixture_documents();
    let (expected_graph, _) = build_and_link_graph_parallel(&documents);

    fn parsed_entry(
        document: &SysmlDocument,
    ) -> (SysmlDocumentSourceKind, WorkspaceParsedDocument) {
        let parsed = sysml_v2_parser::parse(&document.content).expect("parse");
        (
            document.source_kind,
            WorkspaceParsedDocument {
                uri: document.uri.clone(),
                content: document.content.clone(),
                parsed,
                parse_time_ms: 1,
                parse_cached: true,
            },
        )
    }

    let units_doc = documents
        .iter()
        .find(|doc| doc.source_kind == SysmlDocumentSourceKind::Library)
        .expect("fixture has a library document");
    let (base_graph, _) = link_parsed_documents_parallel(vec![parsed_entry(units_doc)], true);

    let remaining: Vec<SourceTaggedDocument> = documents
        .iter()
        .filter(|doc| doc.uri != units_doc.uri)
        .map(parsed_entry)
        .collect();
    let (actual_graph, _) = link_parsed_documents_parallel_from(base_graph, remaining, true);

    assert_eq!(
        node_qualified_names(&actual_graph),
        node_qualified_names(&expected_graph)
    );
    assert_eq!(edge_triples(&actual_graph), edge_triples(&expected_graph));
}

/// Manual benchmark: full-graph relink (`finalize_and_evaluate`) vs frontier-scoped relink
/// (`finalize_and_evaluate_frontier`) on a fixture with real cross-file reference density —
/// PAIR_COUNT independent `A_i -> B_i` pairs (each pair has a real cross-document typing
/// edge), editing only pair 0 each iteration. Unlike Track A's disconnected one-package
/// fixture (which has zero cross-document edges and so cannot show any relink-scoping win
/// at all), this fixture is specifically designed to exercise
/// `link_workspace_relationships`'s per-node qualified-name/import resolution across many
/// files, while the frontier path only needs to touch the one edited pair.
///
/// `#[ignore]`d like Track A's `incremental_benchmark.rs` conventions — this is exploratory
/// evidence-gathering, not a CI-enforced regression guard yet. Per the Track B Phase 1 plan
/// and Track A's own lesson, do not wire real callers to the frontier path based on
/// intuition alone; run this and read the actual numbers first.
#[test]
#[ignore = "manual benchmark: log full-relink vs frontier-relink timings"]
fn benchmark_frontier_relink_vs_full_relink_on_cross_referenced_fixture() {
    const PAIR_COUNT: usize = 30;
    const ITERATIONS: u32 = 5;

    let mut documents = Vec::new();
    for i in 0..PAIR_COUNT {
        documents.push(memory_doc(
            &format!("A{i}.sysml"),
            &format!("package A{i} {{ part def Thing{i} {{ attribute mass{i} : Integer; }} }}"),
        ));
        documents.push(memory_doc(
            &format!("B{i}.sysml"),
            &format!("package B{i} {{ private import A{i}::*; part x{i} : Thing{i}; }}"),
        ));
    }

    let (initial_graph, _) = build_and_link_graph(&documents).expect("initial build");
    let a0_uri = documents[0].uri.clone();

    let mut full_total = std::time::Duration::ZERO;
    let mut frontier_total = std::time::Duration::ZERO;

    for iteration in 0..ITERATIONS {
        let edited_content = format!(
                "package A0 {{ part def Thing0 {{ attribute mass0 : Integer; attribute extra{iteration} : Integer; }} }}"
            );
        let parsed = sysml_v2_parser::parse(&edited_content).expect("parse");

        let mut full_graph = initial_graph.clone();
        let full_start = Instant::now();
        patch_graph_for_document(&mut full_graph, &a0_uri, Some(&parsed), true);
        full_total += full_start.elapsed();

        let mut frontier_graph = initial_graph.clone();
        let frontier_start = Instant::now();
        patch_graph_for_document_scoped(&mut frontier_graph, &a0_uri, Some(&parsed), true);
        frontier_total += frontier_start.elapsed();
    }

    eprintln!(
        "frontier relink benchmark ({PAIR_COUNT} pairs, {ITERATIONS} iterations): \
             full={full_total:?} frontier={frontier_total:?}"
    );
}

// --- B1: cross-document edge ownership regression suite ---
//
// `ROUNDTRIP_SEMGRAPH_PREREQS.md` B1: `cross_document_edges_by_source_uri` must be an
// accurate record of which Typing/Specializes/Subject edges the workspace cross-document
// linking pass owns, *no matter which construction path produced the graph* -- a normal
// whole-graph `build_and_link_graph`, `build_and_link_graph_parallel`, a decoded (serde
// round-tripped) graph, or one assembled one document at a time via
// `patch_graph_for_document_scoped`. Every test below starts from a **normal whole-graph
// build** (not a scoped-patch-assembled graph) before exercising the scoped/incremental
// refresh path, since that is exactly the path a purely scoped-construction test suite would
// never exercise.

/// Simulates a "decoded" graph: resets every `#[serde(skip)]` derived index --including
/// `cross_document_edges_by_source_uri`-- to its default and re-runs `rebuild_derived_indexes()`,
/// exactly what real deserialization does. A literal serde round-trip through postcard (the
/// project's actual cache codec) is blocked today by `SemanticNode.attributes:
/// HashMap<String, serde_json::Value>` (see B9's `deserialize_any` note), which is out of
/// scope here; `SemanticGraphData::simulate_decode_reset_for_test` exercises the exact same
/// post-deserialize contract without depending on a working full-graph codec.
fn roundtrip_through_serde(graph: &SemanticGraph) -> SemanticGraph {
    let mut data = graph.clone().into_data();
    data.simulate_decode_reset_for_test();
    SemanticGraph::from_data(data)
}

/// The set of (source qualified name, target qualified name, kind) triples for every
/// Typing/Specializes/Subject edge owned by `ConstructionOwner::WorkspaceCrossDocumentLinking`
/// -- i.e. exactly what `cross_document_edges_by_source_uri` should account for.
fn cross_document_owned_edge_triples(
    graph: &SemanticGraph,
) -> std::collections::BTreeSet<(String, String, String)> {
    graph
        .graph
        .edge_indices()
        .filter_map(|edge_idx| {
            let weight = &graph.graph[edge_idx];
            if weight.owner != ConstructionOwner::WorkspaceCrossDocumentLinking {
                return None;
            }
            if !matches!(
                weight.kind,
                RelationshipKind::Typing
                    | RelationshipKind::Specializes
                    | RelationshipKind::Subject
            ) {
                return None;
            }
            let (src, tgt) = graph.graph.edge_endpoints(edge_idx).expect("endpoints");
            Some((
                graph.graph[src].id.qualified_name.clone(),
                graph.graph[tgt].id.qualified_name.clone(),
                weight.kind.as_str().to_string(),
            ))
        })
        .collect()
}

/// Flattens `cross_document_edges_by_source_uri` into the same triple shape as
/// `cross_document_owned_edge_triples`, resolving each `NodeId` back to its qualified name via
/// the live graph (both share `SemanticGraphData::get_node`).
fn recorded_cross_document_edge_triples(
    graph: &SemanticGraph,
) -> std::collections::BTreeSet<(String, String, String)> {
    graph
        .cross_document_edges_by_source_uri
        .values()
        .flatten()
        .map(|(src_id, tgt_id, kind)| {
            (
                src_id.qualified_name.clone(),
                tgt_id.qualified_name.clone(),
                kind.as_str().to_string(),
            )
        })
        .collect()
}

/// Core B1 regression: build A+B as a normal whole-graph build (not a scoped-patch
/// assembly), edit A away so B's cross-document typing edge becomes unresolved, then edit A
/// back so the same edge resolves again -- via the scoped/incremental refresh path
/// (`patch_graph_for_document_scoped`, which calls `refresh_relationship_frontier`) applied
/// directly on top of the whole-graph build. After every step, B's Typing/Specializes/Subject
/// edges (and the `cross_document_edges_by_source_uri` bookkeeping behind them) must match a
/// fresh full rebuild of the same document set -- proving ownership was established correctly
/// by the whole build, not only by the scoped-patch path that originally recorded it.
#[test]
fn full_build_then_scoped_refresh_matches_full_rebuild_at_every_step() {
    let (a_uri, a_doc, b_uri, b_doc) = two_file_typing_fixture();

    let mut graph = {
        let (built, _) = build_and_link_graph(&[a_doc.clone(), b_doc.clone()]).expect("build");
        built
    };

    let assert_matches_fresh_rebuild = |graph: &SemanticGraph, a_content: &str, step: &str| {
        let baseline_docs = vec![memory_doc("A.sysml", a_content), b_doc.clone()];
        let (baseline, _) = build_and_link_graph(&baseline_docs).expect("baseline build");
        assert_eq!(
            node_qualified_names(graph),
            node_qualified_names(&baseline),
            "{step}: node set diverged from a fresh full rebuild"
        );
        assert_eq!(
            edge_triples(graph),
            edge_triples(&baseline),
            "{step}: edge set diverged from a fresh full rebuild"
        );
        assert_eq!(
            cross_document_owned_edge_triples(graph),
            cross_document_owned_edge_triples(&baseline),
            "{step}: cross-document-owned edge set diverged from a fresh full rebuild"
        );
        assert_eq!(
            recorded_cross_document_edge_triples(graph),
            cross_document_owned_edge_triples(graph),
            "{step}: cross_document_edges_by_source_uri bookkeeping diverged from the \
                 graph's own owned-edge state"
        );
    };
    assert_matches_fresh_rebuild(&graph, &a_doc.content, "initial full build");

    let typing_edge = (
        "B::x".to_string(),
        "A::Thing".to_string(),
        RelationshipKind::Typing.as_str().to_string(),
    );
    assert!(edge_triples(&graph).contains(&typing_edge));

    // Step 1: rename A's target away via the scoped path. B's edge must disappear -- an
    // unresolved reference is observably a removed edge, never a stale one left pointing at
    // a node that no longer exists.
    let renamed_a = "package A { part def Widget; }";
    apply_scoped_patch(&mut graph, &a_uri, renamed_a);
    assert!(
        !edge_triples(&graph).contains(&typing_edge),
        "unresolved reference must remove the stale edge, not leave it in place"
    );
    assert_matches_fresh_rebuild(&graph, renamed_a, "after renaming A's target away");

    // Step 2: restore A. B's edge must reappear.
    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    assert!(
        edge_triples(&graph).contains(&typing_edge),
        "edge should be restored once the referenced type reappears"
    );
    assert_matches_fresh_rebuild(&graph, &a_doc.content, "after restoring A's target");
    let _ = &b_uri;
}

/// Sibling of the core regression starting from a **decoded** graph (serde round-tripped, the
/// exact scenario B1 describes: `cross_document_edges_by_source_uri` is `#[serde(skip)]` and
/// starts empty after decode) rather than a live whole-graph build. The decoded graph must
/// behave identically to the live one at every step.
#[test]
fn decoded_full_build_then_scoped_refresh_matches_full_rebuild_at_every_step() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_typing_fixture();
    let (built, _) = build_and_link_graph(&[a_doc.clone(), b_doc.clone()]).expect("build");
    let mut graph = roundtrip_through_serde(&built);

    // Immediately after decode, the skipped map must already be fully reconstructed --
    // not merely "repaired lazily by whatever runs next".
    assert_eq!(
        recorded_cross_document_edge_triples(&graph),
        cross_document_owned_edge_triples(&built),
        "decode must eagerly rebuild cross_document_edges_by_source_uri from graph edges"
    );

    let typing_edge = (
        "B::x".to_string(),
        "A::Thing".to_string(),
        RelationshipKind::Typing.as_str().to_string(),
    );
    assert!(edge_triples(&graph).contains(&typing_edge));

    let renamed_a = "package A { part def Widget; }";
    apply_scoped_patch(&mut graph, &a_uri, renamed_a);
    assert!(
        !edge_triples(&graph).contains(&typing_edge),
        "decoded graph must drop the stale edge on refresh, exactly like a live graph"
    );
    let baseline_docs_renamed = vec![memory_doc("A.sysml", renamed_a), b_doc.clone()];
    let (baseline_renamed, _) = build_and_link_graph(&baseline_docs_renamed).expect("baseline");
    assert_eq!(edge_triples(&graph), edge_triples(&baseline_renamed));

    apply_scoped_patch(&mut graph, &a_uri, &a_doc.content);
    assert!(edge_triples(&graph).contains(&typing_edge));
    let baseline_docs_restored = vec![a_doc.clone(), b_doc.clone()];
    let (baseline_restored, _) = build_and_link_graph(&baseline_docs_restored).expect("baseline");
    assert_eq!(edge_triples(&graph), edge_triples(&baseline_restored));
}

/// Covers the "new target differs" half of B1's required resolution: A is edited so B's
/// reference still resolves, but to a *different* node than before. Starting from a normal
/// whole-graph build, the scoped refresh must retarget B's edge -- not leave the old target
/// alongside the new one.
#[test]
fn full_build_then_scoped_refresh_retargets_edge_when_new_target_differs() {
    let (a_uri, a_doc, _b_uri, b_doc) = two_file_fixture(
        "A.sysml",
        "package A { part def Thing; part def OtherThing; }",
        "B.sysml",
        "package B { private import A::*; part x : Thing; }",
    );
    let (mut graph, _) = build_and_link_graph(&[a_doc.clone(), b_doc.clone()]).expect("build");

    let old_edge = (
        "B::x".to_string(),
        "A::Thing".to_string(),
        RelationshipKind::Typing.as_str().to_string(),
    );
    let new_edge = (
        "B::x".to_string(),
        "A::OtherThing".to_string(),
        RelationshipKind::Typing.as_str().to_string(),
    );
    assert!(edge_triples(&graph).contains(&old_edge));
    assert!(!edge_triples(&graph).contains(&new_edge));

    // Force a genuine retarget: remove `Thing` first (B's edge must drop), then reintroduce
    // `Thing` as a structurally distinct node (a fresh specialization of `OtherThing`, not the
    // original bare definition) -- node identity is qualified-name-based, so this is the same
    // `NodeId` as before but a different underlying declaration, exercising the "new target
    // differs" case even though the qualified name round-trips.
    apply_scoped_patch(&mut graph, &a_uri, "package A { part def OtherThing; }");
    assert!(!edge_triples(&graph).contains(&old_edge));

    let retargeted_a = "package A { part def OtherThing; part def Thing :> OtherThing; }";
    apply_scoped_patch(&mut graph, &a_uri, retargeted_a);

    let baseline_docs = vec![memory_doc("A.sysml", retargeted_a), b_doc.clone()];
    let (baseline, _) = build_and_link_graph(&baseline_docs).expect("baseline build");

    assert_eq!(
        node_qualified_names(&graph),
        node_qualified_names(&baseline)
    );
    assert_eq!(edge_triples(&graph), edge_triples(&baseline));
    assert!(
        edge_triples(&graph).contains(&old_edge),
        "B::x -> A::Thing should resolve again once A::Thing exists, now as a distinct node"
    );
    assert_eq!(
        cross_document_owned_edge_triples(&graph),
        cross_document_owned_edge_triples(&baseline)
    );
}

/// Ownership-state equality across construction paths: build the same A+B document set via
/// whole-graph (`build_and_link_graph`), parallel (`build_and_link_graph_parallel`), and
/// document-at-a-time scoped-patch construction, and confirm all three agree on exactly which
/// edges are owned by `ConstructionOwner::WorkspaceCrossDocumentLinking` -- the same set that
/// `cross_document_edges_by_source_uri` is built from. This is the B1 requirement that whole,
/// parallel, and incremental builds establish identical ownership, not merely identical edges.
#[test]
fn whole_parallel_and_incremental_builds_agree_on_cross_document_edge_ownership() {
    let (a_uri, a_doc, b_uri, b_doc) = two_file_typing_fixture();
    let documents = vec![a_doc.clone(), b_doc.clone()];

    let (whole_graph, _) = build_and_link_graph(&documents).expect("whole build");
    let (parallel_graph, _) = build_and_link_graph_parallel(&documents);

    let mut incremental_graph = SemanticGraph::new();
    apply_scoped_patch(&mut incremental_graph, &a_uri, &a_doc.content);
    apply_scoped_patch(&mut incremental_graph, &b_uri, &b_doc.content);

    let whole_owned = cross_document_owned_edge_triples(&whole_graph);
    let parallel_owned = cross_document_owned_edge_triples(&parallel_graph);
    let incremental_owned = cross_document_owned_edge_triples(&incremental_graph);

    assert!(
        !whole_owned.is_empty(),
        "fixture must actually exercise a cross-document edge"
    );
    assert_eq!(
        whole_owned, parallel_owned,
        "whole vs parallel ownership diverged"
    );
    assert_eq!(
        whole_owned, incremental_owned,
        "whole vs incremental ownership diverged"
    );

    // And the edge sets themselves (not just the owned subset) must also agree, and the
    // recorded bookkeeping must exactly mirror the owned edges in every construction.
    assert_eq!(edge_triples(&whole_graph), edge_triples(&parallel_graph));
    assert_eq!(edge_triples(&whole_graph), edge_triples(&incremental_graph));
    for graph in [&whole_graph, &parallel_graph, &incremental_graph] {
        assert_eq!(
            recorded_cross_document_edge_triples(graph),
            cross_document_owned_edge_triples(graph)
        );
    }
}
