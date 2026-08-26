//! Contract tests for the evaluation phase, driven through the crate's public
//! `build()` / `PublishedResolution` surface. Relocated verbatim from the inline
//! `#[cfg(test)]` modules of `src/lib.rs` and `src/model.rs`.

#![allow(clippy::too_many_lines)]

#[allow(unused_imports)]
use crate::common::*;
#[allow(unused_imports)]
use sysml_resolution::*;

#[test]
fn skipping_evaluation_publishes_not_run_rather_than_nothing() {
    let source = "package P { attribute mass : Integer = 5; }";
    let request = || {
        BuildRequest::new(
            vec![SourceInput::new(
                "memory://test.sysml",
                source.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .unwrap()
    };
    let evaluated = build(request()).unwrap();
    let mass = identity_of(&evaluated, "memory://test.sysml", "P::mass");
    let evaluation = settled(evaluated.evaluate(mass));
    assert_eq!(
        evaluation.state,
        EvaluationState::Literal(EvaluatedScalar::Integer(5))
    );

    let skipped = build(request().with_evaluation_policy(EvaluationPolicy::Skip)).unwrap();
    let mass = identity_of(&skipped, "memory://test.sysml", "P::mass");
    assert_eq!(
        settled(skipped.evaluate(mass)).state,
        EvaluationState::NotRun
    );
}
#[test]
fn a_missing_quantity_library_leaves_measurement_applicability_unavailable() {
    let workspace = "package P { attribute plain = 1; }";
    let published = publication_for(&[("memory://q.sysml", workspace)]);
    let symbol = probe_symbol(&published, workspace, "memory://q.sysml", "plain");
    let QueryAnswer::Resolved(evaluation) = published.evaluate(symbol).answer else {
        panic!("the probe must resolve");
    };
    assert_eq!(
        evaluation.expected_measurement,
        ExpectedMeasurement::Unavailable
    );
}

/// With the library admitted, the same shape of element gets the affirmative answer.
#[test]
fn an_admitted_quantity_library_answers_a_non_quantity_element_affirmatively() {
    let workspace = "package P { attribute plain : ScalarValues::Integer = 1; }";
    let published = against_measurement_library(workspace, ConstructionSchedule::Sequential);
    let symbol = probe_symbol(&published, workspace, "memory://workspace.sysml", "plain");
    let QueryAnswer::Resolved(evaluation) = published.evaluate(symbol).answer else {
        panic!("the probe must resolve");
    };
    assert_eq!(
        evaluation.expected_measurement,
        ExpectedMeasurement::NotApplicable
    );
}

/// Every migrated expression rule the parity cases below rely on actually firing.
const MEASUREMENT_CODES: [&str; 4] = [
    "incompatible_unit_dimension",
    "unknown_unit_symbol",
    "attribute_value_type_mismatch",
    "non_boolean_expression",
];

/// Evaluation, unit resolution and the decisions they feed must not depend on the schedule
/// that built the publication.
#[test]
fn parallel_and_sequential_construction_publish_the_same_evaluation_and_units() {
    let sequential = measurement_publication(ConstructionSchedule::Sequential);
    let parallel = measurement_publication(ConstructionSchedule::Parallel);
    assert_eq!(
        sequential, parallel,
        "evaluation, unit and measurement facts must not depend on construction schedule"
    );
    for code in MEASUREMENT_CODES {
        assert!(
            sequential.contains(code),
            "the parity workspace must actually exercise {code}, got: {sequential}"
        );
    }
}

/// The same facts, reached through a settled library stratum rather than a cold solve.
#[test]
fn a_seeded_publication_matches_an_unseeded_one_for_evaluation_and_units() {
    let library = std::sync::Arc::new(
        build_library_stratum(vec![SourceInput::new(
            "memory://measurement.sysml",
            MEASUREMENT_LIBRARY_SOURCE.to_string(),
            SourceKind::StandardLibrary,
        )])
        .expect("measurement stratum"),
    );
    let seeded = build(
        BuildRequest::with_library(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                MEASUREMENT_WORKSPACE.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
            library,
        )
        .expect("seeded request"),
    )
    .expect("seeded build");
    let seeded = render_publication(&seeded);
    assert_eq!(
        seeded,
        measurement_publication(ConstructionSchedule::Sequential),
        "unit and evaluation decisions must not depend on library-stratum reuse"
    );
    for code in MEASUREMENT_CODES {
        assert!(
            seeded.contains(code),
            "the parity workspace must actually exercise {code}, got: {seeded}"
        );
    }
}

/// The verdict channel is a projection of the same settled value channel, gated by the
/// element's kind, so the two cannot disagree.
#[test]
fn analysis_evaluation_is_a_second_channel_over_the_settled_value() {
    let published = detail_publication(
        &[(
            "memory://analysis.sysml",
            concat!(
                "package P {\n",
                "  attribute plain = 1;\n",
                "  constraint holds { true }\n",
                "  constraint fails { false }\n",
                "  constraint broken { missing }\n",
                "}\n",
            ),
        )],
        ConstructionSchedule::Sequential,
    );

    let plain = details_of(&published, "memory://analysis.sysml", "P::plain");
    assert_eq!(
        plain.analysis,
        AnalysisEvaluation::NotApplicable,
        "an attribute's value is not a verdict"
    );
    assert_eq!(
        plain.evaluation.state,
        EvaluationState::Literal(EvaluatedScalar::Integer(1))
    );

    assert_eq!(
        details_of(&published, "memory://analysis.sysml", "P::holds").analysis,
        AnalysisEvaluation::Verdict(true)
    );
    assert_eq!(
        details_of(&published, "memory://analysis.sysml", "P::fails").analysis,
        AnalysisEvaluation::Verdict(false)
    );

    let broken = details_of(&published, "memory://analysis.sysml", "P::broken");
    assert!(
        matches!(broken.analysis, AnalysisEvaluation::Unsettled(_)),
        "an unsettled constraint must not read as a failing verdict, got {:?}",
        broken.analysis
    );
}

/// A build that does not evaluate reports the verdict channel as not run, which is neither a
/// verdict nor an inapplicable element.
#[test]
fn a_skipped_evaluation_policy_reports_the_verdict_channel_as_not_run() {
    let request = BuildRequest::new(
        vec![SourceInput::new(
            "memory://skip.sysml",
            "package P { constraint holds { true } }".to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap()
    .with_evaluation_policy(EvaluationPolicy::Skip);
    let published = build(request).unwrap();
    let holds = details_of(&published, "memory://skip.sysml", "P::holds");
    assert_eq!(holds.evaluation.state, EvaluationState::NotRun);
    assert_eq!(holds.analysis, AnalysisEvaluation::NotRun);
}
