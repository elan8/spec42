use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

fn has_code(diagnostics: &[semantic_core::SemanticDiagnostic], code: &str) -> bool {
    diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}

#[test]
fn action_flow_to_part_def_emits_succession_invalid() {
    let doc = workspace_doc(
        "flow_invalid.sysml",
        r#"package Demo {
  part def WrongPart;
  action def Step1;
  action def Pipeline {
    action step1 : Step1;
    flow step1 to WrongPart;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "succession_endpoint_invalid"),
        "expected succession_endpoint_invalid, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn requirement_satisfy_wrong_target_emits_kind_diagnostic() {
    let doc = workspace_doc(
        "satisfy_invalid.sysml",
        r#"package Demo {
  requirement def ReqA;
  part def System;
  requirement r1 : ReqA;
  satisfy r1 by System;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "satisfy_invalid_endpoint_kind"),
        "expected satisfy_invalid_endpoint_kind, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn view_without_expose_emits_empty_expose_diagnostic() {
    let doc = workspace_doc(
        "view_empty.sysml",
        r#"package Demo {
  view def StructuralView;
  view structure : StructuralView {
    filter @Type;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "view_expose_empty"),
        "expected view_expose_empty, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn untyped_metadata_annotation_emits_unresolved_diagnostic() {
    let doc = workspace_doc(
        "metadata_unresolved.sysml",
        r#"package Demo {
  metadata orphan;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "metadata_annotation_unresolved"),
        "expected metadata_annotation_unresolved, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn include_missing_use_case_emits_invalid_target() {
    let doc = workspace_doc(
        "include_invalid.sysml",
        r#"package Demo {
  use case def Main {
    include MissingCase;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "use_case_include_invalid_target"),
        "expected use_case_include_invalid_target, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn transition_comparison_guard_has_no_non_boolean_diagnostic() {
    let doc = workspace_doc(
        "guard_comparison.sysml",
        r#"package Demo {
  state def Operating {
    then off;
    state off;
    state on;
    transition power_up first off if off == on then on;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "transition_guard_non_boolean"),
        "unexpected transition_guard_non_boolean: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn transition_non_boolean_guard_emits_diagnostic() {
    let doc = workspace_doc(
        "guard_invalid.sysml",
        r#"package Demo {
  state def Operating {
    state off;
    state on;
    transition power_up first off if 42 then on;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "transition_guard_non_boolean"),
        "expected transition_guard_non_boolean, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn multiple_initial_transitions_emit_cardinality_diagnostic() {
    let doc = workspace_doc(
        "initial_multi.sysml",
        r#"package Demo {
  state def Operating {
    state off;
    state on;
    state paused;
    then off;
    then paused;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "multiple_initial_states"),
        "expected multiple_initial_states, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn state_def_without_initial_transition_emits_information() {
    let doc = workspace_doc(
        "initial_missing.sysml",
        r#"package Demo {
  state def Operating {
    state off;
    state on;
    transition t off then on;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "missing_initial_state"),
        "expected missing_initial_state, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn accept_payload_with_wrong_kind_emits_diagnostic() {
    let doc = workspace_doc(
        "accept_invalid.sysml",
        r#"package Demo {
  part def WrongKind;
  action wait accept evt : WrongKind;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "accept_payload_incompatible"),
        "expected accept_payload_incompatible, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn view_filter_non_boolean_emits_diagnostic_for_non_boolean_literal() {
    let doc = workspace_doc(
        "view_filter.sysml",
        r#"package Demo {
  view def StructuralView;
  view structure : StructuralView {
    filter 42;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "view_filter_non_boolean"),
        "expected view_filter_non_boolean, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn view_metaclass_filter_does_not_emit_non_boolean_diagnostic() {
    let doc = workspace_doc(
        "view_metaclass_filter.sysml",
        r#"package Demo {
  view def StructuralView;
  view structure : StructuralView {
    filter @SysML::PartUsage or @SysML::PartDefinition
      or @SysML::PortUsage or @SysML::PortDefinition;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "view_filter_non_boolean"),
        "metaclass filter expressions are Boolean per SysML §7.26.2, got: {:?}",
        diagnostics
            .iter()
            .filter(|d| {
                d.code == "view_filter_non_boolean" || d.code == "invalid_import_filter"
            })
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn classification_filter_does_not_emit_non_boolean_diagnostic() {
    let doc = workspace_doc(
        "classification_filter.sysml",
        r#"package Demo {
  view def StructuralView;
  view structure : StructuralView {
    filter @MissingType;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "view_filter_non_boolean"),
        "@-prefixed classification is Boolean-typed even when metaclass is unresolved, got: {:?}",
        diagnostics
            .iter()
            .filter(|d| d.code == "view_filter_non_boolean")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn verification_assignment_value_mismatch_emits_diagnostic() {
    let doc = workspace_doc(
        "assign_value.sysml",
        r#"package Demo {
  part def System {
    attribute count : Integer;
  }
  verification def VerifyCount {
    subject system : System;
    assign system.count := "text";
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "assignment_value_incompatible"),
        "expected assignment_value_incompatible, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn requirement_constraint_bad_parameter_emits_membership_diagnostic() {
    let doc = workspace_doc(
        "constraint_bad.sysml",
        r#"package Demo {
  requirement def EnduranceReq {
    require constraint {
      in x;
      x > 0
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "requirement_constraint_invalid_membership"),
        "expected requirement_constraint_invalid_membership, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn verification_then_action_without_verdict_emits_shape_diagnostic() {
    let doc = workspace_doc(
        "verify_shape.sysml",
        r#"package Demo {
  action def Step;
  verification def BadVerify {
    then action step : Step;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "verification_case_invalid_shape"),
        "expected verification_case_invalid_shape, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn verification_without_subject_emits_case_subject_diagnostic() {
    let doc = workspace_doc(
        "case_subject.sysml",
        r#"package Demo {
  requirement def ReqA;
  verification def BadVerify {
    objective { verify ReqA; }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "case_subject_missing"),
        "expected case_subject_missing, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn viewpoint_unresolved_import_emits_reference_diagnostic() {
    let doc = workspace_doc(
        "viewpoint_import.sysml",
        r#"package Demo {
  viewpoint def ArchitectureViewpoint {
    import MissingPackage::*;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "viewpoint_reference_unresolved"),
        "expected viewpoint_reference_unresolved, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn duplicate_metadata_def_emits_collision_diagnostic() {
    let doc = workspace_doc(
        "metadata_collision.sysml",
        r#"package Demo {
  metadata def Tag;
  metadata def Tag;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "metadata_keyword_collision"),
        "expected metadata_keyword_collision, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn transition_typed_accept_wrong_kind_emits_diagnostic() {
    let doc = workspace_doc(
        "transition_accept.sysml",
        r#"package Demo {
  part def WrongKind;
  state def Operating {
    then idle;
    state idle;
    state running;
    transition typed_accept first idle accept evt : WrongKind then running;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "accept_payload_incompatible"),
        "expected accept_payload_incompatible, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn send_payload_wrong_kind_emits_diagnostic() {
    let doc = workspace_doc(
        "send_payload.sysml",
        r#"package Demo {
  part def WrongKind;
  action def Notify {
    send message : WrongKind;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "send_payload_incompatible"),
        "expected send_payload_incompatible, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn multiple_final_states_emit_cardinality_diagnostic() {
    let doc = workspace_doc(
        "final_state.sysml",
        r#"package Demo {
  state def DoneStates {
    then idle;
    state idle;
    final expired;
    final state completed;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "multiple_final_states"),
        "expected multiple_final_states, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn state_def_without_final_state_emits_information() {
    let doc = workspace_doc(
        "final_missing.sysml",
        r#"package Demo {
  state def Operating {
    then off;
    state off;
    state on;
    transition t off then on;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "missing_final_state"),
        "expected missing_final_state, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn state_def_with_final_state_has_no_missing_final_diagnostic() {
    let doc = workspace_doc(
        "final_present.sysml",
        r#"package Demo {
  state def Operating {
    then off;
    state off;
    final done;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "missing_final_state"),
        "unexpected missing_final_state: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn state_def_with_done_transition_has_no_missing_final_diagnostic() {
    let doc = workspace_doc(
        "final_done.sysml",
        r#"package Demo {
  state def OnOff {
    then off;
    state off;
    state on;
    transition to_done first on then done;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "missing_final_state"),
        "unexpected missing_final_state: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn guarded_initial_transitions_do_not_emit_multiple_initial_states() {
    let doc = workspace_doc(
        "initial_guarded.sysml",
        r#"package Demo {
  state def OperationalStates {
    in attribute isStarting : Boolean;
    entry action initial;
    transition first initial if not isStarting then off;
    transition first initial if isStarting then starting;
    state off;
    state starting;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "multiple_initial_states"),
        "unexpected multiple_initial_states: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn metadata_keyword_usage_unresolved_emits_diagnostic() {
    let doc = workspace_doc(
        "metadata_keyword.sysml",
        r#"package Demo {
  part def Widget {
    #UnknownMeta;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "metadata_keyword_unresolved"),
        "expected metadata_keyword_unresolved, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn viewpoint_stakeholder_unresolved_emits_reference_diagnostic() {
    let doc = workspace_doc(
        "viewpoint_stakeholder.sysml",
        r#"package Demo {
  viewpoint def SafetyView {
    stakeholder SafetyConcern;
    purpose ReliabilityPurpose;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "viewpoint_reference_unresolved"),
        "expected viewpoint_reference_unresolved, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn metadata_usage_with_valid_type_has_no_annotation_diagnostic() {
    let doc = workspace_doc(
        "metadata_ok.sysml",
        r#"package Demo {
  metadata def Tag;
  metadata tag1 : Tag;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "metadata_annotation_unresolved"),
        "unexpected metadata_annotation_unresolved: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn invalid_verdict_value_uses_return_expression_span() {
    let doc = workspace_doc(
        "invalid_verdict_value.sysml",
        r#"package P {
  verification def VerifyRuntime {
    return ref verdictResult { return VerdictKind::unknown; }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "invalid_verdict_value"),
        "expected invalid_verdict_value, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message, d.range))
            .collect::<Vec<_>>()
    );
    let verdict_node = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "verdict")
        .expect("verdict node");
    assert!(
        verdict_node.range.start.line >= 2,
        "verdict node range should come from parsed return expression, got {:?}",
        verdict_node.range
    );
}

#[test]
fn metadata_about_unresolved_emits_diagnostic() {
    let doc = workspace_doc(
        "metadata_about_unresolved.sysml",
        r#"package Demo {
  metadata def Tag;
  metadata note : Tag about MissingTarget;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "metadata_about_unresolved"),
        "expected metadata_about_unresolved, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn metadata_binding_missing_emits_diagnostic() {
    let doc = workspace_doc(
        "metadata_binding_missing.sysml",
        r#"package Demo {
  metadata def ApprovalAnnotation {
    attribute approved;
    attribute approver;
  }
  part def Design {
    @ApprovalAnnotation : ApprovalAnnotation {
      approved = true;
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "metadata_binding_missing"),
        "expected metadata_binding_missing, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}
