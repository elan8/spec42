use kernel::{default_server_config, validate_paths, ValidationRequest};
use std::fs;
use std::sync::Arc;
use tower_lsp::lsp_types::{DiagnosticSeverity, NumberOrString};

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

    let config = Arc::new(default_server_config());
    let report = validate_paths(
        &config,
        ValidationRequest {
            targets: vec![path],
            workspace_root: Some(temp.path().to_path_buf()),
            library_paths: Vec::new(),
            parallel_enabled: false,
        },
    )
    .expect("validation report");

    let parse_errors = report.documents[0]
        .diagnostics
        .iter()
        .filter(|d| {
            d.source.as_deref() == Some("sysml")
                && matches!(d.severity, Some(DiagnosticSeverity::ERROR) | None)
        })
        .count();
    assert!(
        parse_errors <= 1,
        "expected at most one top-level parse error per file, got {parse_errors}: {:?}",
        report.documents[0].diagnostics
    );
    assert!(
        report.documents[0].diagnostics.iter().any(|d| {
            matches!(
                &d.code,
                Some(NumberOrString::String(code))
                    if code == "missing_semicolon" || code == "recovery_cascade_suppressed"
            )
        }),
        "expected a root parse diagnostic: {:?}",
        report.documents[0].diagnostics
    );
}
