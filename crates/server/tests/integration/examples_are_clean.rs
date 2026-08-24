//! Every bundled example under `examples/` validates clean: no errors, no warnings, and no
//! informational diagnostics either — an `info` such as `missing_library_anchor` means the
//! publication admitted less of the standard library than the model needs.

use std::path::{Path, PathBuf};

use crate::common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;

fn example_workspaces() -> Vec<PathBuf> {
    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut workspaces = std::fs::read_dir(&examples)
        .unwrap_or_else(|error| panic!("reading {}: {error}", examples.display()))
        .map(|entry| entry.expect("examples entry").path())
        .filter(|path| path.is_dir() && contains_sysml_sources(path))
        .collect::<Vec<_>>();
    workspaces.sort();
    assert!(
        !workspaces.is_empty(),
        "no example workspaces with SysML sources under {}",
        examples.display()
    );
    workspaces
}

fn contains_sysml_sources(dir: &Path) -> bool {
    std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.flatten().any(|entry| {
            let path = entry.path();
            path.extension()
                .is_some_and(|extension| extension == "sysml" || extension == "kerml")
                || (path.is_dir() && contains_sysml_sources(&path))
        })
    })
}

#[test]
fn every_example_workspace_validates_without_diagnostics() {
    with_isolated_data_dir(|| {
        let mut failures = Vec::new();
        for workspace in example_workspaces() {
            let cli = Cli {
                config_path: None,
                library_paths: vec![],
                stdlib_path: None,
                kpar_library_paths: Vec::new(),
                disabled_kpar_libraries: Vec::new(),
                no_stdlib: false,
                stdio: false,
                command: None,
            };
            let args = CheckArgs {
                path: workspace.clone(),
                workspace_root: None,
                format: OutputFormat::Json,
                warnings_as_errors: false,
                baseline: None,
                strict_diagnostics: false,
            };
            let report = match perform_check(&cli, &args) {
                Ok(report) => report,
                Err(error) => {
                    failures.push(format!("{}: check failed: {error}", workspace.display()));
                    continue;
                }
            };
            let summary = &report.summary;
            if summary.error_count + summary.warning_count + summary.information_count > 0 {
                let mut detail = format!(
                    "{}: {} error(s), {} warning(s), {} info(s)",
                    workspace.display(),
                    summary.error_count,
                    summary.warning_count,
                    summary.information_count
                );
                for document in &report.documents {
                    for diagnostic in &document.diagnostics {
                        detail.push_str(&format!("\n  {}: {diagnostic:?}", document.uri));
                    }
                }
                failures.push(detail);
            }
        }
        assert!(
            failures.is_empty(),
            "example workspaces must validate clean:\n{}",
            failures.join("\n")
        );
    });
}
