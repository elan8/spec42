use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind, TextRange,
};
use tempfile::tempdir;
use url::Url;

fn file_doc(filename: &str, content: &str) -> SysmlDocument {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join(filename);
    std::fs::write(&path, content).expect("write sysml");
    let uri = Url::from_file_path(&path).expect("file uri");
    std::mem::forget(dir);
    SysmlDocument::from_uri(
        uri.as_str(),
        content.to_string(),
        Some(filename.to_string()),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document")
}

fn diagnostic_range_for(content: &str, code: &str) -> TextRange {
    diagnostic_for(content, code).range
}

fn diagnostic_for(content: &str, code: &str) -> semantic_core::SemanticDiagnostic {
    let doc = file_doc("diagnostic-range.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == code)
        .cloned()
        .unwrap_or_else(|| panic!("missing diagnostic {code}: {diagnostics:#?}"))
}

fn assert_range_text(content: &str, range: TextRange, expected: &str) {
    let line = content
        .lines()
        .nth(range.start.line as usize)
        .expect("range line");
    let actual = line
        .chars()
        .skip(range.start.character as usize)
        .take((range.end.character - range.start.character) as usize)
        .collect::<String>();
    assert_eq!(actual, expected);
}

#[test]
fn unresolved_type_reference_points_at_type_token() {
    let content = "package Demo {\n  part vehicle : MissingType;\n}\n";
    let range = diagnostic_range_for(content, "unresolved_type_reference");
    assert_range_text(content, range, "MissingType");
}

#[test]
fn unresolved_ref_type_reference_points_at_type_token() {
    let content =
        "package Demo {\n  part def Orbit;\n  part system { ref target : MissingOrbit; }\n}\n";
    let range = diagnostic_range_for(content, "unresolved_ref_type_reference");
    assert_range_text(content, range, "MissingOrbit");
}

#[test]
fn unresolved_specializes_reference_points_at_specializes_target() {
    let content = "package Demo {\n  part def Vehicle :> MissingBase;\n}\n";
    let range = diagnostic_range_for(content, "unresolved_specializes_reference");
    assert_range_text(content, range, "MissingBase");
}

#[test]
fn unresolved_import_target_points_at_import_target() {
    let content = "package Demo {\n  import MissingLibrary::*;\n}\n";
    let range = diagnostic_range_for(content, "unresolved_import_target");
    assert_range_text(content, range, "MissingLibrary::*");
}

#[test]
fn unresolved_allocate_target_points_at_endpoint_token() {
    let content = "package Demo {\n  part source;\n  allocate source to MissingTarget;\n}\n";
    let diagnostic = diagnostic_for(content, "unresolved_allocate_target");
    assert_range_text(content, diagnostic.range, "MissingTarget");
    assert!(
        diagnostic
            .related_information
            .iter()
            .any(|info| info.message.contains("source")),
        "expected related information for resolved source endpoint: {diagnostic:#?}"
    );
}

#[test]
fn unresolved_viewpoint_target_points_at_endpoint_token() {
    let content = "package Demo {\n  view def StructuralView;\n  view structure : StructuralView;\n  satisfy structure by MissingViewpoint;\n}\n";
    let range = diagnostic_range_for(content, "unresolved_viewpoint_conformance_target");
    assert_range_text(content, range, "MissingViewpoint");
}

#[test]
fn transition_guard_non_boolean_points_at_transition() {
    let content = "package Demo {\n  state def Operating {\n    state off;\n    state on;\n    transition power_up first off if 42 then on;\n  }\n}\n";
    let range = diagnostic_range_for(content, "transition_guard_non_boolean");
    assert_range_text(content, range, "transition power_up first off if 42 then on;");
}

#[test]
fn view_filter_non_boolean_points_at_filter() {
    let content = "package Demo {\n  view def StructuralView;\n  view structure : StructuralView {\n    filter @MissingType;\n  }\n}\n";
    let range = diagnostic_range_for(content, "view_filter_non_boolean");
    assert_range_text(content, range, "filter @MissingType;");
}

#[test]
fn viewpoint_reference_unresolved_points_at_import() {
    let content = "package Demo {\n  viewpoint def ArchitectureViewpoint {\n    import MissingPackage::*;\n  }\n}\n";
    let range = diagnostic_range_for(content, "viewpoint_reference_unresolved");
    assert_range_text(content, range, "import MissingPackage::*;");
}
