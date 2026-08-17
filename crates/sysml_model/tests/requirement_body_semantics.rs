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
fn standalone_verification_graph_links_objective_verified_requirement_to_case() {
    let source = r#"package P {
  requirement def ReqA;
  verification def Check {
    objective { verify requirement ReqA; }
  }
}"#;
    let parsed = sysml_v2_parser::parse(source).expect("parse");
    let uri = url::Url::parse("file:///verification.sysml").expect("uri");
    // The case-subject edge is derived by the linking pass, not by construction; the assertion
    // below still guards the "exactly one" part, which is what this test is for.
    let mut graph = sysml_model::build_graph_from_doc(&parsed, &uri);
    sysml_model::link_workspace_relationships(&mut graph);

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
