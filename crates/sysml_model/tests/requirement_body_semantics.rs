use sysml_model::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
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

#[test]
fn requirement_subject_decl_emits_subject_edge() {
    let doc = workspace_doc(
        "subject.sysml",
        r#"package P {
  part def Vehicle;
  requirement def RangeReq {
    subject vehicle : Vehicle;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let req = graph
        .nodes_named("RangeReq")
        .into_iter()
        .find(|node| node.element_kind == "requirement def")
        .expect("requirement def");

    let subject = graph
        .children_of(req)
        .into_iter()
        .find(|child| child.element_kind == "subject")
        .expect("subject child");

    let has_subject_edge =
        graph
            .edges_for_uri_as_strings(&uri)
            .iter()
            .any(|(src, tgt, kind, _)| {
                *kind == RelationshipKind::Subject
                    && src == &req.id.qualified_name
                    && tgt == &subject.id.qualified_name
            });
    assert!(
        has_subject_edge,
        "expected Subject edge from requirement to subject usage"
    );
}

#[test]
fn requirement_verify_member_materializes_verified_requirement_node() {
    let doc = workspace_doc(
        "verify.sysml",
        r#"package P {
  requirement def BatteryRuntime;
  requirement def VerifyPack {
    verify BatteryRuntime;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let verify_def = graph
        .nodes_named("VerifyPack")
        .into_iter()
        .find(|node| node.element_kind == "requirement def")
        .expect("verification requirement def");

    let verified_requirement = graph
        .children_of(verify_def)
        .into_iter()
        .find(|child| child.element_kind == "verified requirement")
        .expect("verified requirement child on requirement def");
    assert_eq!(
        verified_requirement
            .declared_facts
            .relationships
            .subject
            .iter()
            .map(|target| (target.reference.as_str(), target.range))
            .collect::<Vec<_>>(),
        vec![("BatteryRuntime", None)],
        "verified requirement target must be an owned declared fact; the parser currently exposes only its string spelling"
    );

    let has_subject_to_runtime =
        graph
            .edges_for_uri_as_strings(&uri)
            .iter()
            .any(|(src, tgt, kind, _)| {
                *kind == RelationshipKind::Subject
                    && src.ends_with("::VerifyPack")
                    && tgt.ends_with("::BatteryRuntime")
            });
    assert!(
        has_subject_to_runtime,
        "expected Subject edge from verify requirement to verified target"
    );
}

#[test]
fn verification_case_cross_package_verify_requirement_resolves_via_import() {
    let requirements = workspace_doc(
        "SystemRequirements.sysml",
        r#"package SystemRequirements {
  requirement coverFloor;
}"#,
    );
    let verification = workspace_doc(
        "Verification.sysml",
        r#"package Verification {
  private import SystemRequirements::*;
  part def Device;
  verification verifyCleaningCoverage {
    subject robot : Device;
    objective {
      verify requirement coverFloor;
    }
  }
}"#,
    );
    let uri = verification.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[requirements, verification]).expect("semantic graph");

    let diagnostics = sysml_diagnostics::collect_diagnostics_from_graph(
        &graph,
        &uri,
        sysml_diagnostics::DiagnosticsOptions::default(),
    );
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "unresolved_pending_relationship"),
        "verification case cross-package verify should resolve via import, got: {:?}",
        diagnostics
            .iter()
            .filter(|diag| diag.code == "unresolved_pending_relationship")
            .map(|diag| &diag.message)
            .collect::<Vec<_>>()
    );
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "verified_requirement_invalid_target"),
        "verified requirement should resolve to imported requirement, got: {:?}",
        diagnostics
            .iter()
            .filter(|diag| diag.code == "verified_requirement_invalid_target")
            .map(|diag| &diag.message)
            .collect::<Vec<_>>()
    );

    let has_subject_to_requirement =
        graph
            .edges_for_uri_as_strings(&uri)
            .iter()
            .any(|(src, tgt, kind, _)| {
                *kind == RelationshipKind::Subject
                    && src.ends_with("::verifyCleaningCoverage")
                    && tgt.ends_with("::coverFloor")
            });
    assert!(
        has_subject_to_requirement,
        "expected Subject edge from verification case to imported requirement"
    );
}

#[test]
fn standalone_verification_graph_links_objective_verified_requirement_to_case() {
    let source = r#"package P {
  requirement def ReqA;
  verification def Check {
    objective { verify requirement ReqA; }
  }
}"#;
    let parsed = sysml_v2_parser::parse(source).expect("parse");
    let uri = url::Url::parse("file:///verification.sysml").expect("uri");
    let graph = sysml_model::build_graph_from_doc(&parsed, &uri);

    let case_subject_edges: Vec<_> = graph
        .edges_for_uri_as_strings(&uri)
        .into_iter()
        .filter(|(source, target, kind, _)| {
            source == "P::Check" && target == "P::ReqA" && *kind == RelationshipKind::Subject
        })
        .collect();
    assert_eq!(
        case_subject_edges.len(),
        1,
        "standalone graph construction must not omit or duplicate the verification case edge"
    );
}

#[test]
fn cross_package_verify_requirement_resolves_via_import() {
    let requirements = workspace_doc(
        "SystemRequirements.sysml",
        r#"package SystemRequirements {
  requirement coverFloor;
}"#,
    );
    let verification = workspace_doc(
        "Verification.sysml",
        r#"package Verification {
  private import SystemRequirements::*;
  requirement def VerifyCoverage {
    verify requirement coverFloor;
  }
}"#,
    );
    let uri = verification.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[requirements, verification]).expect("semantic graph");

    let diagnostics = sysml_diagnostics::collect_diagnostics_from_graph(
        &graph,
        &uri,
        sysml_diagnostics::DiagnosticsOptions::default(),
    );
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "unresolved_pending_relationship"),
        "cross-package verify should resolve via import, got: {:?}",
        diagnostics
            .iter()
            .filter(|diag| diag.code == "unresolved_pending_relationship")
            .map(|diag| &diag.message)
            .collect::<Vec<_>>()
    );

    let has_subject_to_requirement =
        graph
            .edges_for_uri_as_strings(&uri)
            .iter()
            .any(|(src, tgt, kind, _)| {
                *kind == RelationshipKind::Subject
                    && src.ends_with("::VerifyCoverage")
                    && tgt.ends_with("::coverFloor")
            });
    assert!(
        has_subject_to_requirement,
        "expected Subject edge from verify requirement to imported requirement"
    );
}

#[test]
fn viewpoint_body_materializes_stakeholder_and_purpose_nodes() {
    let fixture = include_str!("fixtures/parser_wave/viewpoint-stakeholder-purpose.sysml");
    let doc = workspace_doc("viewpoint-stakeholder-purpose.sysml", fixture);
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let viewpoint = graph
        .nodes_named("SafetyView")
        .into_iter()
        .find(|node| node.element_kind == "viewpoint def")
        .expect("viewpoint def");

    let kinds: Vec<_> = graph
        .children_of(viewpoint)
        .into_iter()
        .map(|child| child.element_kind.as_str())
        .collect();
    assert!(kinds.contains(&"stakeholder"));
    assert!(kinds.contains(&"purpose"));
    assert!(kinds.contains(&"frame"));
    let _ = uri;
}

#[test]
fn requirement_require_constraint_stays_on_analysis_constraints_attr() {
    let doc = workspace_doc(
        "constraint.sysml",
        r#"package P {
  requirement def Req1 {
    require constraint {
      in mass : Real;
      mass > 0.0;
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let req = graph
        .nodes_named("Req1")
        .into_iter()
        .find(|node| node.element_kind == "requirement def")
        .expect("requirement def");
    let constraints = req
        .declared_facts
        .analysis_case
        .as_ref()
        .map(|facts| facts.constraints.clone())
        .unwrap_or_default();
    assert!(
        !constraints.is_empty(),
        "expected analysisConstraints on requirement def"
    );

    assert!(
        graph
            .children_of(req)
            .iter()
            .any(|child| child.element_kind == "require constraint"),
        "expected require constraint child node on requirement def"
    );
    let _ = uri;
}

/// Regression for `planning/UNIFY_CACHE_PROGRESS.md` B9: a require constraint's aggregated typed
/// `DeclaredAnalysisConstraint` fact -- not a JSON `analysisConstraints` attribute -- drives the
/// `requirement_constraint_invalid_membership` diagnostic, for both a firing (missing parameter
/// type) and non-firing (fully typed parameter) case, with a stable code/severity/range.
#[test]
fn typed_analysis_constraint_expression_drives_requirement_constraint_diagnostic() {
    let firing_doc = workspace_doc(
        "untyped_param_constraint.sysml",
        r#"package P {
  requirement def Req1 {
    require constraint {
      in mass;
      mass > 0.0;
    }
  }
}"#,
    );
    let uri = firing_doc.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[firing_doc]).expect("semantic graph");
    let req = graph
        .nodes_named("Req1")
        .into_iter()
        .find(|node| node.element_kind == "requirement def")
        .expect("requirement def");
    let constraints = req
        .declared_facts
        .analysis_case
        .as_ref()
        .map(|facts| facts.constraints.clone())
        .unwrap_or_default();
    assert!(
        constraints.iter().any(|constraint| matches!(
            constraint,
            sysml_model::semantic::model::DeclaredAnalysisConstraint::RequireConstraint {
                params,
                ..
            } if params.iter().any(|param| param.param_type.as_deref().unwrap_or("").is_empty())
        )),
        "expected a typed require-constraint fact with an untyped parameter, got {constraints:?}"
    );
    let diagnostics = sysml_diagnostics::collect_diagnostics_from_graph(
        &graph,
        &uri,
        sysml_diagnostics::DiagnosticsOptions::default(),
    );
    let firing = diagnostics
        .iter()
        .find(|diag| diag.code == "requirement_constraint_invalid_membership")
        .expect("expected requirement_constraint_invalid_membership to fire");
    assert_eq!(
        firing.severity,
        sysml_diagnostics::DiagnosticSeverity::Warning
    );
    assert_eq!(firing.range, req.range);

    let passing_doc = workspace_doc(
        "populated_constraint.sysml",
        r#"package P {
  requirement def Req1 {
    require constraint {
      in mass : Real;
      mass > 0.0;
    }
  }
}"#,
    );
    let uri2 = passing_doc.uri.clone();
    let (graph2, _parsed2) =
        build_semantic_graph_from_documents(&[passing_doc]).expect("semantic graph");
    let req2 = graph2
        .nodes_named("Req1")
        .into_iter()
        .find(|node| node.element_kind == "requirement def")
        .expect("requirement def");
    let constraints2 = req2
        .declared_facts
        .analysis_case
        .as_ref()
        .map(|facts| facts.constraints.clone())
        .unwrap_or_default();
    assert!(
        constraints2.iter().any(|constraint| matches!(
            constraint,
            sysml_model::semantic::model::DeclaredAnalysisConstraint::RequireConstraint {
                expression,
                ..
            } if !expression.trim().is_empty()
        )),
        "expected a typed require-constraint fact with a non-empty expression, got {constraints2:?}"
    );
    let diagnostics2 = sysml_diagnostics::collect_diagnostics_from_graph(
        &graph2,
        &uri2,
        sysml_diagnostics::DiagnosticsOptions::default(),
    );
    assert!(
        !diagnostics2
            .iter()
            .any(|diag| diag.code == "requirement_constraint_invalid_membership"),
        "populated require constraint must not fire requirement_constraint_invalid_membership: {diagnostics2:?}"
    );
}
