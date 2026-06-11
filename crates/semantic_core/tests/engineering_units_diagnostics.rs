//! Regression: engineering SI-prefixed unit literals with QUDV catalog sources.

use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind, UnitRegistry,
};
use std::path::Path;
use url::Url;

const SI_CATALOG_EXCERPT: &str = r#"
package SIPrefixes {
    attribute kilo: UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
    attribute mega: UnitPrefix { :>> symbol = "M"; :>> conversionFactor = 1E6; }
}
package SI {
    attribute <m> metre : LengthUnit;
    attribute <V> volt : ElectricPotentialUnit;
    attribute <W> watt : PowerUnit;
    attribute <A> ampere : ElectricCurrentUnit;
    attribute <h> hour: DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = s; :>> conversionFactor = 3600; } }
    attribute <s> second : DurationUnit;
    attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }
}
"#;

const GRID_USAGE_SYSML: &str = r#"
package GridQuantities {
    private import ScalarValues::*;
    attribute def PowerValue;
    attribute def ElectricPotentialDifferenceValue;
    attribute def EnergyValue;
    attribute def LengthValue;

    part def GridAsset {
        attribute nominalVoltage : ElectricPotentialDifferenceValue;
        attribute requiredCapacity : PowerValue;
        attribute energyCapacity : EnergyValue;
        attribute length : LengthValue;
    }

    part asset : GridAsset {
        attribute :>> nominalVoltage = 10 [kV];
        attribute :>> requiredCapacity = 16 [MW];
        attribute :>> energyCapacity = 20 [MWh];
        attribute :>> length = 2.4 [km];
    }
}
"#;

fn catalog_uri() -> Url {
    Url::parse("file:///sysml.library/Domain%20Libraries/Quantities%20and%20Units/SI.sysml")
        .expect("catalog uri")
}

#[test]
fn engineering_prefixed_units_resolve_from_indexed_qudv_catalog() {
    let catalog_uri = catalog_uri();
    let catalog_content = SI_CATALOG_EXCERPT.to_string();
    let catalog_doc = SysmlDocument {
        uri: catalog_uri.clone(),
        content: catalog_content.clone(),
        path_hint: Some(
            "Domain Libraries/Quantities and Units/SI.sysml".to_string(),
        ),
        source_kind: SysmlDocumentSourceKind::Library,
        sha256: None,
        byte_size: None,
    };
    let usage_doc = SysmlDocument::from_memory_path(
        "grid-usage",
        "GridQuantities.sysml",
        GRID_USAGE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("usage doc");
    let usage_uri = usage_doc.uri.clone();

    let indexed_sources = [(&catalog_uri, catalog_content.as_str())];
    let (graph, _) = build_semantic_graph_from_documents(&[catalog_doc, usage_doc])
        .expect("semantic graph");

    let registry = UnitRegistry::build_unified(&graph, &indexed_sources, &[]);
    for unit in ["kV", "MW", "MVA", "MWh", "km", "SI::s"] {
        assert!(
            registry.is_recognized_unit_expression(unit),
            "expected derived unit {unit}"
        );
    }

    let diagnostics = collect_diagnostics_from_graph(
        &graph,
        &usage_uri,
        DiagnosticsOptions {
            include_hints: false,
            indexed_sources: &indexed_sources,
        },
    );
    let unit_diags: Vec<_> = diagnostics
        .iter()
        .filter(|diag| {
            diag.code == "unknown_unit_symbol" || diag.code == "incompatible_unit_dimension"
        })
        .collect();
    assert!(
        unit_diags.is_empty(),
        "unexpected unit diagnostics: {unit_diags:#?}"
    );
}

#[test]
fn mismatched_unit_dimension_emits_incompatible_not_unknown() {
    let catalog_uri = catalog_uri();
    let catalog_content = SI_CATALOG_EXCERPT.to_string();
    let catalog_doc = SysmlDocument {
        uri: catalog_uri.clone(),
        content: catalog_content.clone(),
        path_hint: Some(
            "Domain Libraries/Quantities and Units/SI.sysml".to_string(),
        ),
        source_kind: SysmlDocumentSourceKind::Library,
        sha256: None,
        byte_size: None,
    };
    let usage_doc = SysmlDocument::from_memory_path(
        "grid-mismatch",
        "GridMismatch.sysml",
        r#"
            package GridMismatch {
                attribute def PowerValue;
                part def Asset {
                    attribute load : PowerValue;
                }
                part asset : Asset {
                    attribute load : PowerValue = 1 [km];
                }
            }
        "#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("usage doc");
    let usage_uri = usage_doc.uri.clone();
    let indexed_sources = [(&catalog_uri, catalog_content.as_str())];
    let (graph, _) = build_semantic_graph_from_documents(&[catalog_doc, usage_doc])
        .expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(
        &graph,
        &usage_uri,
        DiagnosticsOptions {
            include_hints: false,
            indexed_sources: &indexed_sources,
        },
    );
    assert!(
        diagnostics
            .iter()
            .any(|diag| diag.code == "incompatible_unit_dimension"),
        "expected incompatible_unit_dimension for PowerValue with [km]"
    );
    assert!(
        !diagnostics.iter().any(|diag| diag.code == "unknown_unit_symbol"),
        "km should be recognized: {diagnostics:?}"
    );
}

#[test]
fn sysml_powersystems_check_has_no_engineering_unit_catalog_warnings_when_stdlib_present() {
    let powersystems_root = Path::new(r"C:\Git\sysml-powersystems\sysml");
    let stdlib_root = Path::new(
        r"C:\Git\sysml-v2-release\sysml.library\Domain Libraries\Quantities and Units",
    );
    if !powersystems_root.is_dir() || !stdlib_root.is_dir() {
        return;
    }

    let mut documents = Vec::new();
    for entry in walkdir::WalkDir::new(stdlib_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sysml"))
    {
        let path = entry.path();
        let uri = Url::from_file_path(path).expect("stdlib uri");
        let content = std::fs::read_to_string(path).expect("read stdlib");
        documents.push(SysmlDocument {
            uri,
            content,
            path_hint: None,
            source_kind: SysmlDocumentSourceKind::Library,
            sha256: None,
            byte_size: None,
        });
    }

    let mut target_uri = None;
    for entry in walkdir::WalkDir::new(powersystems_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sysml"))
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path).expect("read workspace");
        let doc = SysmlDocument::from_memory_path(
            "powersystems",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("model.sysml"),
            content,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        if path.to_string_lossy().contains("DutchGridProfile") {
            target_uri = Some(doc.uri.clone());
        }
        documents.push(doc);
    }

    let Some(target_uri) = target_uri else {
        return;
    };

    let (graph, _) =
        build_semantic_graph_from_documents(&documents).expect("powersystems graph");
    let indexed_refs: Vec<(&Url, &str)> = documents
        .iter()
        .map(|doc| (&doc.uri, doc.content.as_str()))
        .collect();
    let diagnostics = collect_diagnostics_from_graph(
        &graph,
        &target_uri,
        DiagnosticsOptions {
            include_hints: false,
            indexed_sources: &indexed_refs,
        },
    );
    let unit_warnings: Vec<_> = diagnostics
        .iter()
        .filter(|diag| {
            diag.code == "unknown_unit_symbol" || diag.code == "incompatible_unit_dimension"
        })
        .map(|diag| (&diag.code, &diag.message))
        .collect();
    assert!(
        unit_warnings.is_empty(),
        "sysml-powersystems unit catalog warnings: {unit_warnings:?}"
    );
}
