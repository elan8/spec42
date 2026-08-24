//! End-to-end validation that import-scoped library closure does not duplicate workspace packages.

use std::path::PathBuf;

use sysml_diagnostics::SemanticDiagnostic;
use workspace::{validate_paths, ValidationRequest};

fn test_engine(
    cache: &tempfile::TempDir,
    library_paths: Vec<std::path::PathBuf>,
) -> workspace::Spec42Engine {
    workspace::EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .library_paths(library_paths)
        .build()
        .expect("engine")
}
fn diagnostic_is_ambiguous_expose(diagnostic: &SemanticDiagnostic) -> bool {
    diagnostic.code == "view_expose_unresolved" && diagnostic.message.contains("ambiguous")
}

fn write_webshop_like_workspace(root: &std::path::Path) {
    std::fs::write(
        root.join("Views.sysml"),
        r#"
package Views {
    import WebShopExample::*;
    import WebShopArchitecture::*;

    view structure : GeneralView {
        expose WebShopExample::webshopSystem;
    }

    view checkoutFlow : SequenceView {
        expose WebShopArchitecture::CheckoutFlow;
    }
}
"#,
    )
    .expect("write Views.sysml");
    std::fs::write(
        root.join("WebShopExample.sysml"),
        r#"
package WebShopExample {
    private import ScalarValues::String;
    part def WebShopSystem;
    part webshopSystem : WebShopSystem;
}
"#,
    )
    .expect("write WebShopExample.sysml");
    std::fs::write(
        root.join("WebShopArchitecture.sysml"),
        r#"
package WebShopArchitecture {
    part def CheckoutFlow;
}
"#,
    )
    .expect("write WebShopArchitecture.sysml");
}

fn write_duplicate_library_root(root: &std::path::Path) {
    std::fs::create_dir_all(root).expect("library root");
    std::fs::write(
        root.join("WebShopExample.sysml"),
        "package WebShopExample { part def libraryOnlyPart; }",
    )
    .expect("write library WebShopExample.sysml");
    std::fs::write(
        root.join("WebShopArchitecture.sysml"),
        "package WebShopArchitecture { part def LibraryCheckoutFlow; }",
    )
    .expect("write library WebShopArchitecture.sysml");
    std::fs::write(
        root.join("ScalarValues.sysml"),
        "standard library package ScalarValues { attribute def Real; attribute def String; }",
    )
    .expect("write ScalarValues.sysml");
}

#[test]
fn validate_paths_resolves_view_expose_without_library_duplicates() {
    let temp = tempfile::tempdir().expect("tempdir");
    let workspace = temp.path().join("workspace");
    let library = temp.path().join("library");
    std::fs::create_dir_all(&workspace).expect("workspace dir");
    write_webshop_like_workspace(&workspace);
    write_duplicate_library_root(&library);

    let cache = tempfile::tempdir().expect("cache dir");
    let engine = test_engine(&cache, vec![library.clone()]);
    let report = validate_paths(
        &engine,
        &[],
        ValidationRequest {
            targets: vec![workspace.clone()],
            workspace_root: Some(workspace),
            library_paths: vec![library],
            parallel_enabled: false,
            strict_diagnostics: false,
        },
    )
    .expect("validation report");

    let views = report
        .documents
        .iter()
        .find(|doc| doc.uri.ends_with("Views.sysml"))
        .expect("Views.sysml diagnostics");
    let ambiguous: Vec<_> = views
        .diagnostics
        .iter()
        .filter(|diag| diagnostic_is_ambiguous_expose(diag))
        .map(|diag| diag.message.clone())
        .collect();
    assert!(
        ambiguous.is_empty(),
        "expected no ambiguous expose diagnostics, got {ambiguous:?}"
    );
}

#[test]
#[ignore = "requires C:\\Git\\sysml-examples\\webshop and C:\\Git\\sysml-domain-libraries"]
fn validate_paths_real_webshop_has_no_ambiguous_view_expose() {
    let workspace = PathBuf::from(r"C:\Git\sysml-examples\webshop");
    let library = PathBuf::from(r"C:\Git\sysml-domain-libraries");
    if !workspace.is_dir() || !library.is_dir() {
        return;
    }

    let cache = tempfile::tempdir().expect("cache dir");
    let engine = test_engine(&cache, vec![library.clone()]);
    let report = validate_paths(
        &engine,
        &[],
        ValidationRequest {
            targets: vec![workspace.clone()],
            workspace_root: Some(workspace),
            library_paths: vec![library],
            parallel_enabled: false,
            strict_diagnostics: false,
        },
    )
    .expect("validation report");

    let ambiguous: Vec<_> = report
        .documents
        .iter()
        .filter(|doc| doc.uri.ends_with("Views.sysml"))
        .flat_map(|doc| doc.diagnostics.iter())
        .filter(|diag| diagnostic_is_ambiguous_expose(diag))
        .map(|diag| diag.message.clone())
        .collect();
    assert!(
        ambiguous.is_empty(),
        "real webshop should not report ambiguous expose targets: {ambiguous:?}"
    );
}
