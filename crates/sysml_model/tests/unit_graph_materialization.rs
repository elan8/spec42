//! Graph materialization tests for unit catalog attributes.

use sysml_model::{
    build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind, UnitRegistry,
};

const SI_CATALOG: &str = r#"
package SIPrefixes {
    attribute kilo: UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
    attribute mega: UnitPrefix { :>> symbol = "M"; :>> conversionFactor = 1E6; }
}
package SI {
    attribute <m> metre : LengthUnit;
    attribute <kg> kilogram : MassUnit;
    attribute <V> volt : ElectricPotentialUnit;
    attribute <W> watt : PowerUnit;
    attribute <s> second : DurationUnit;
    attribute <km> kilometre : LengthUnit {
        :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; }
    }
}
"#;

#[test]
fn unit_catalog_short_names_materialize_on_graph_nodes() {
    let doc = SysmlDocument::from_memory_path(
        "si-catalog",
        "si.sysml",
        SI_CATALOG.to_string(),
        SysmlDocumentSourceKind::Library,
        None,
        None,
    )
    .expect("document");
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph should build");
    let metre = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|n| n.name == "metre")
        .expect("metre node");
    assert_eq!(
        metre.attributes.get("shortName").and_then(|v| v.as_str()),
        Some("m")
    );
    assert_eq!(
        metre
            .attributes
            .get("attributeType")
            .and_then(|v| v.as_str()),
        Some("LengthUnit")
    );
}

#[test]
fn graph_backed_registry_recognizes_materialized_units() {
    let doc = SysmlDocument::from_memory_path(
        "si-catalog",
        "si.sysml",
        SI_CATALOG.to_string(),
        SysmlDocumentSourceKind::Library,
        None,
        None,
    )
    .expect("document");
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph should build");
    let registry = UnitRegistry::from_graph(&graph);
    assert!(registry.is_recognized_unit_expression("m"));
    assert!(registry.is_recognized_unit_expression("km"));
    assert!(registry.is_recognized_unit_expression("kV"));
}

#[test]
fn graph_only_registry_parity_with_engineering_catalog() {
    let catalog_doc = SysmlDocument::from_memory_path(
        "si-catalog",
        "SI.sysml",
        SI_CATALOG.to_string(),
        SysmlDocumentSourceKind::Library,
        None,
        None,
    )
    .expect("document");
    let (graph, _) = build_semantic_graph_from_documents(&[catalog_doc]).expect("graph");
    let registry = UnitRegistry::from_graph(&graph);
    for unit in ["m", "kg", "kV", "MW", "km", "SI::s"] {
        assert!(
            registry.is_recognized_unit_expression(unit),
            "expected graph-only unit {unit}"
        );
    }
}

#[test]
fn library_closure_seeds_quantity_packages_on_unit_literals() {
    use sysml_model::{resolve_library_closure, LibraryClosureOptions, WorkspaceSource};
    use std::fs;

    let temp = tempfile::tempdir().expect("tempdir");
    let lib = temp.path().join("lib");
    let si_dir = lib.join("Quantities and Units");
    fs::create_dir_all(&si_dir).expect("dir");
    fs::write(
        si_dir.join("si.sysml"),
        "package SI { attribute <m> metre : LengthUnit; }",
    )
    .expect("write si");
    fs::write(lib.join("Base.sysml"), "package Base { part def Y; }").expect("base");
    let workspace = [WorkspaceSource {
        path: "grid.sysml",
        content: "package Grid { attribute x = 10 [kV]; }",
    }];
    let roots = vec![lib.to_string_lossy().replace('\\', "/")];
    let loaded = resolve_library_closure(&workspace, &roots, &LibraryClosureOptions::default())
        .expect("closure");
    assert!(
        loaded.iter().any(|f| f.path.contains("si.sysml")),
        "unit literal should seed quantity library closure, got {:?}",
        loaded.iter().map(|f| &f.path).collect::<Vec<_>>()
    );
}

const MEASUREMENT_TAXONOMY: &str = r#"
package Measurement {
    abstract attribute def MeasurementUnit;
    attribute def SimpleUnit :> MeasurementUnit;
    attribute def LengthUnit :> SimpleUnit;
    attribute def PowerUnit :> SimpleUnit;
    attribute def ElectricPotentialUnit :> SimpleUnit;
    attribute def ElectricPotentialDifferenceUnit :> ElectricPotentialUnit;
}
"#;

#[test]
fn measurement_unit_taxonomy_materializes_as_attribute_defs_with_edges() {
    use sysml_model::semantic::units::is_measurement_unit_compatible;
    use sysml_model::{link_workspace_relationships, RelationshipKind};

    let doc = SysmlDocument::from_memory_path(
        "measurement",
        "measurement.sysml",
        MEASUREMENT_TAXONOMY.to_string(),
        SysmlDocumentSourceKind::Library,
        None,
        None,
    )
    .expect("document");
    let uri = doc.uri.clone();
    let (mut graph, _) = build_semantic_graph_from_documents(&[doc]).expect("graph should build");
    link_workspace_relationships(&mut graph);

    let length_unit = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|n| n.name == "LengthUnit")
        .expect("LengthUnit attribute def");
    assert_eq!(length_unit.element_kind, "attribute def");

    let length_edges: Vec<_> = graph
        .outgoing_targets_by_kind(length_unit, RelationshipKind::Typing)
        .into_iter()
        .map(|n| n.name.as_str())
        .collect();
    assert!(
        length_edges.contains(&"SimpleUnit"),
        "LengthUnit should specialize SimpleUnit, got {length_edges:?}"
    );

    assert!(is_measurement_unit_compatible(
        &graph,
        "ElectricPotentialDifferenceUnit",
        "ElectricPotentialUnit"
    ));
    assert!(!is_measurement_unit_compatible(
        &graph,
        "PowerUnit",
        "LengthUnit"
    ));
}
