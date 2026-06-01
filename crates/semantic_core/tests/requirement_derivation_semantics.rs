use semantic_core::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
};

const STAKEHOLDER_NEEDS: &str = r#"package StakeholderNeeds {
  requirement def CleanLargeAreas;
  requirement cleanLargeAreas : CleanLargeAreas;
}
"#;

const SYSTEM_REQUIREMENTS: &str = r#"package SystemRequirements {
  private import StakeholderNeeds::*;

  requirement def CleanAtLeastEightySquareMetersPerCharge;
  requirement cleanAtLeastEighty : CleanAtLeastEightySquareMetersPerCharge;

  #derivation connection {
    end #original ::> cleanLargeAreas;
    end #derive ::> cleanAtLeastEighty;
  }
}
"#;

fn document(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "requirement-derivation",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri")
}

fn derivation_edges(graph: &semantic_core::SemanticGraph) -> Vec<(String, String)> {
    graph
        .graph
        .edge_indices()
        .filter_map(|edge_index| {
            let (source_index, target_index) = graph.graph.edge_endpoints(edge_index)?;
            if graph.graph.edge_weight(edge_index)? != &RelationshipKind::Derivation {
                return None;
            }
            let source = graph.graph.node_weight(source_index)?;
            let target = graph.graph.node_weight(target_index)?;
            Some((
                source.id.qualified_name.clone(),
                target.id.qualified_name.clone(),
            ))
        })
        .collect()
}

#[test]
fn requirement_usage_derivation_links_across_workspace_files() {
    let documents = vec![
        document("StakeholderNeeds.sysml", STAKEHOLDER_NEEDS),
        document("SystemRequirements.sysml", SYSTEM_REQUIREMENTS),
    ];
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&documents).expect("semantic graph");

    let derivations = derivation_edges(&graph);
    assert_eq!(
        derivations,
        vec![(
            "StakeholderNeeds::cleanLargeAreas".to_string(),
            "SystemRequirements::cleanAtLeastEighty".to_string(),
        )],
        "expected derivation from imported stakeholder usage to system usage"
    );
}

#[test]
fn requirement_usage_derivation_links_within_one_package() {
    let content = r#"package Demo {
  package UserNeeds {
    requirement def TrackParcel;
    requirement trackParcel : TrackParcel;
  }
  package SystemRequirements {
    private import UserNeeds::*;
    requirement def DetectParcel;
    requirement detectParcel : DetectParcel;
    #derivation connection {
      end #original ::> trackParcel;
      end #derive ::> detectParcel;
    }
  }
}"#;
    let documents = vec![document("model.sysml", content)];
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&documents).expect("semantic graph");

    let derivations = derivation_edges(&graph);
    assert_eq!(
        derivations,
        vec![(
            "Demo::UserNeeds::trackParcel".to_string(),
            "Demo::SystemRequirements::detectParcel".to_string(),
        )]
    );
}
