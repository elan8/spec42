//! Regression tests for drone-example diagnostics (SysML v2 spec alignment).

use sysml_model::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

fn workspace_doc(name: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "drone",
        name,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

fn has_code(diags: &[sysml_model::SemanticDiagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code == code)
}

#[test]
fn drone_requirements_actor_and_view_filters_do_not_emit_spec_false_positives() {
    let requirements = workspace_doc(
        "SurveillanceDroneRequirements.sysml",
        r#"package SurveillanceDroneRequirements {
            item def Operator;
            part def SurveillanceQuadrotorDrone;
            use case def PatrolOverwatch {
                subject drone : SurveillanceQuadrotorDrone;
                actor pilot : Operator;
            }
            use case def PointOfInterestOrbit {
                subject drone : SurveillanceQuadrotorDrone;
                actor pilot : Operator;
            }
        }"#,
    );
    let views = workspace_doc(
        "Views.sysml",
        r#"package Views {
            view def GeneralView;
            part def droneInstance;
            view structure : GeneralView {
                expose Views::droneInstance;
                filter @SysML::PartUsage or @SysML::PartDefinition
                    or @SysML::PortUsage or @SysML::PortDefinition;
            }
        }"#,
    );
    for doc in [&requirements, &views] {
        let uri = doc.uri.clone();
        let (graph, _parsed) =
            build_semantic_graph_from_documents(std::slice::from_ref(doc)).expect("graph");
        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(
            !has_code(&diagnostics, "incompatible_type_kind"),
            "{}: unexpected incompatible_type_kind: {:?}",
            doc.uri,
            diagnostics
                .iter()
                .filter(|d| d.code == "incompatible_type_kind")
                .map(|d| &d.message)
                .collect::<Vec<_>>()
        );
        assert!(
            !has_code(&diagnostics, "view_filter_non_boolean"),
            "{}: unexpected view_filter_non_boolean: {:?}",
            doc.uri,
            diagnostics
                .iter()
                .filter(|d| d.code == "view_filter_non_boolean")
                .map(|d| &d.message)
                .collect::<Vec<_>>()
        );
        assert!(
            !has_code(&diagnostics, "invalid_import_filter"),
            "{}: unexpected invalid_import_filter: {:?}",
            doc.uri,
            diagnostics
                .iter()
                .filter(|d| d.code == "invalid_import_filter")
                .map(|d| &d.message)
                .collect::<Vec<_>>()
        );
    }
}
