use std::fs;
use sysml_diagnostics::DiagnosticSeverity;
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

fn validate_source(source: &str) -> Vec<sysml_diagnostics::SemanticDiagnostic> {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("diagnostics.sysml");
    fs::write(&path, source).expect("write");
    let cache = tempfile::tempdir().expect("cache dir");
    let engine = test_engine(&cache, Vec::new());
    validate_paths(
        &engine,
        &[],
        ValidationRequest {
            targets: vec![path],
            workspace_root: Some(temp.path().to_path_buf()),
            library_paths: Vec::new(),
            parallel_enabled: false,
            strict_diagnostics: false,
        },
    )
    .expect("validation report")
    .documents
    .into_iter()
    .next()
    .expect("validated document")
    .diagnostics
}

#[test]
fn check_collapses_cascade_parse_errors_per_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("cascade.sysml");
    fs::write(
        &path,
        r#"package P {
part def Carrier {
  part a : A
  part b : B
}
}"#,
    )
    .expect("write");

    let cache = tempfile::tempdir().expect("cache dir");
    let engine = test_engine(&cache, Vec::new());
    let report = validate_paths(
        &engine,
        &[],
        ValidationRequest {
            targets: vec![path],
            workspace_root: Some(temp.path().to_path_buf()),
            library_paths: Vec::new(),
            parallel_enabled: false,
            strict_diagnostics: false,
        },
    )
    .expect("validation report");

    let parse_errors = report.documents[0]
        .diagnostics
        .iter()
        .filter(|d| d.source == "sysml" && d.severity == DiagnosticSeverity::Error)
        .count();
    assert!(
        parse_errors <= 1,
        "expected at most one top-level parse error per file, got {parse_errors}: {:?}",
        report.documents[0].diagnostics
    );
    assert!(
        report.documents[0]
            .diagnostics
            .iter()
            .any(|d| { d.code == "missing_semicolon" || d.code == "recovery_cascade_suppressed" }),
        "expected a root parse diagnostic: {:?}",
        report.documents[0].diagnostics
    );
}

#[test]
fn check_keeps_semantic_warnings_after_parse_error_by_default() {
    let temp = tempfile::tempdir().expect("tempdir");
    let path = temp.path().join("mixed.sysml");
    fs::write(
        &path,
        r#"package P {
  part def Broken {
    part a : MissingType
    attribute label : UnknownType;
  }
}"#,
    )
    .expect("write");

    let cache = tempfile::tempdir().expect("cache dir");
    let engine = test_engine(&cache, Vec::new());
    let report = validate_paths(
        &engine,
        &[],
        ValidationRequest {
            targets: vec![path],
            workspace_root: Some(temp.path().to_path_buf()),
            library_paths: Vec::new(),
            parallel_enabled: false,
            strict_diagnostics: false,
        },
    )
    .expect("validation report");

    assert!(
        report.documents[0]
            .diagnostics
            .iter()
            .any(|d| d.code == "unresolved_type_reference"),
        "expected semantic unresolved_type_reference after parse error by default: {:?}",
        report.documents[0].diagnostics
    );
}

#[test]
fn check_keeps_each_independent_structural_parse_error() {
    let diagnostics = validate_source(
        r#"package V {
  part def Car {
    action a { return x : ScalarValues::Real; }
    action b { return y : ScalarValues::Real; }
    action c { return z : ScalarValues::Real; }
  }
}"#,
    );

    let parse_errors: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.source == "sysml" && diagnostic.severity == DiagnosticSeverity::Error
        })
        .map(|diagnostic| {
            (
                diagnostic.code.as_str(),
                diagnostic.range.start.line,
                diagnostic.range.start.character,
                diagnostic.range.end.line,
                diagnostic.range.end.character,
                diagnostic.related_information.len(),
            )
        })
        .collect();
    assert_eq!(
        parse_errors,
        vec![
            ("unexpected_keyword_in_scope", 2, 15, 2, 46, 0),
            ("unexpected_keyword_in_scope", 3, 15, 3, 46, 0),
            ("unexpected_keyword_in_scope", 4, 15, 4, 46, 0),
        ],
        "{diagnostics:#?}"
    );
}

#[test]
fn check_keeps_recovery_error_beside_independent_structural_error() {
    let diagnostics = validate_source(
        r#"package Missing {
  part def Carrier {
    part a : A
    part b : B
  }
}
package Structural {
  part def Car {
    action a { return x : ScalarValues::Real; }
  }
}"#,
    );

    let parser_diagnostics: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.source == "sysml")
        .map(|diagnostic| {
            (
                diagnostic.code.as_str(),
                diagnostic.severity,
                diagnostic.range.start.line,
                diagnostic.range.start.character,
                diagnostic.range.end.line,
                diagnostic.range.end.character,
                diagnostic.related_information.len(),
            )
        })
        .collect();
    assert_eq!(
        parser_diagnostics,
        vec![
            (
                "missing_semicolon",
                DiagnosticSeverity::Error,
                2,
                4,
                3,
                4,
                0
            ),
            (
                "recovery_cascade_suppressed",
                DiagnosticSeverity::Warning,
                2,
                4,
                3,
                4,
                0,
            ),
            (
                "unexpected_keyword_in_scope",
                DiagnosticSeverity::Error,
                8,
                15,
                8,
                46,
                0,
            ),
        ],
        "{diagnostics:#?}"
    );
}
