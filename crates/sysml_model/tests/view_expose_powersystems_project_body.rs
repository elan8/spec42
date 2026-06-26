use sysml_model::{
    build_semantic_graph_from_documents, resolve_expose_target, resolve_expression_endpoint_strict,
    ExposeTargetResolution, ResolveResult, SysmlDocument, SysmlDocumentSourceKind,
};

const PROJECT_WITH_REDEFINES: &str = r#"
package RegionalGridExpansion {
    public import RegionalGridExpansion::Architecture::*;
    part regionalExpansionProject : DutchGridExpansionProject {
        part :>> operatorProfile {
            attribute :>> operator = DutchGridOperator::RegionalOperator;
        }
        part architecture : RegionalGridArchitecture {
            attribute :>> name = "Regional Grid Architecture";
        }

        variation part expansionAlternatives : RegionalGridArchitecture {
            variant baseVariant;
        }
    }
}
"#;

const ARCHITECTURE_SYSML: &str = r#"
package RegionalGridExpansion::Architecture {
    part def RegionalGridArchitecture {
        part feederNorth;
    }
}
package DutchGridProfile {
    enum def DutchGridOperator { enum RegionalOperator; }
    part def DutchGridExpansionProject {
        part operatorProfile;
        part architecture;
    }
}
"#;

#[test]
fn project_body_with_redefines_materializes_architecture_usage() {
    let project = SysmlDocument::from_memory_path(
        "powersystems",
        "Project.sysml",
        PROJECT_WITH_REDEFINES.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("project uri");
    let architecture = SysmlDocument::from_memory_path(
        "powersystems",
        "Architecture.sysml",
        ARCHITECTURE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("architecture uri");

    let (graph, _) =
        build_semantic_graph_from_documents(&[project.clone(), architecture]).expect("graph");

    let project_nodes: Vec<_> = graph
        .graph
        .node_weights()
        .filter(|node| {
            node.id.qualified_name.contains("regionalExpansionProject")
                || node.element_kind == "kermlDecl"
        })
        .map(|node| (node.id.qualified_name.clone(), node.element_kind.clone()))
        .collect();

    assert!(
        project_nodes
            .iter()
            .any(|(name, kind)| name.contains("regionalExpansionProject") && kind == "part"),
        "expected part usage for regionalExpansionProject, got: {project_nodes:?}"
    );
    assert!(
        !project_nodes
            .iter()
            .any(|(name, _)| name.contains("regionalExpansionProject") && name.contains("kerml")),
        "project root should not collapse to kermlDecl, got: {project_nodes:?}"
    );

    match resolve_expression_endpoint_strict(
        &graph,
        &project.uri,
        Some("RegionalGridExpansion"),
        "regionalExpansionProject.architecture",
    ) {
        ResolveResult::Resolved(id) => {
            assert!(
                id.qualified_name.contains("architecture"),
                "resolved architecture usage: {}",
                id.qualified_name
            );
        }
        other => panic!("expected architecture chain resolution, got {other:?}"),
    }

    match resolve_expose_target(
        &graph,
        None,
        Some("RegionalGridExpansion::Views"),
        "RegionalGridExpansion::regionalExpansionProject.architecture",
    ) {
        ExposeTargetResolution::Resolved(names) => {
            assert!(
                names.iter().any(|name| name.contains("architecture")),
                "expose should resolve architecture usage, got: {names:?}"
            );
        }
        other => panic!("expected expose resolution, got {other:?}"),
    }
}
