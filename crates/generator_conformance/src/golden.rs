//! Golden comparison and blessing.
//!
//! Goldens are committed directory trees rather than snapshot-library output, because what
//! is being pinned is the exact bytes a generator writes -- including binary files. The git
//! diff of a blessed change is the regression report: a reviewer sees
//! `"query_count": 9 -> 11` next to the source change that caused it.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::runner::CaseRun;

const REPORT_NAME: &str = "report.json";
const ARTIFACTS_DIR: &str = "artifacts";

/// Compares a run against its golden, returning one message per difference.
pub fn compare(run: &CaseRun, golden_dir: &Path) -> Result<Vec<String>, String> {
    if !golden_dir.exists() {
        return Ok(vec![format!(
            "no golden at {} (run with --bless to create it)",
            golden_dir.display()
        )]);
    }

    let mut differences = Vec::new();

    let expected_report = fs::read_to_string(golden_dir.join(REPORT_NAME))
        .map_err(|error| format!("failed to read golden report: {error}"))?;
    let actual_report = render_report(run)?;
    if expected_report.trim() != actual_report.trim() {
        differences.push(format!(
            "report differs:\n{}",
            unified(&expected_report, &actual_report)
        ));
    }

    let artifacts_dir = golden_dir.join(ARTIFACTS_DIR);
    let expected_paths = list_artifacts(&artifacts_dir)?;
    let actual_paths: BTreeSet<String> = run.artifacts.keys().cloned().collect();

    for path in expected_paths.difference(&actual_paths) {
        differences.push(format!("missing artifact `{path}`"));
    }
    for path in actual_paths.difference(&expected_paths) {
        differences.push(format!("unexpected artifact `{path}`"));
    }
    for path in expected_paths.intersection(&actual_paths) {
        let expected = fs::read(artifacts_dir.join(path))
            .map_err(|error| format!("failed to read golden artifact `{path}`: {error}"))?;
        let actual = &run.artifacts[path];
        if &expected != actual {
            differences.push(
                match (
                    String::from_utf8(expected),
                    String::from_utf8(actual.clone()),
                ) {
                    (Ok(expected), Ok(actual)) => {
                        format!(
                            "artifact `{path}` differs:\n{}",
                            unified(&expected, &actual)
                        )
                    }
                    _ => format!(
                        "binary artifact `{path}` differs ({} vs {} bytes)",
                        fs::metadata(artifacts_dir.join(path))
                            .map(|meta| meta.len())
                            .unwrap_or_default(),
                        actual.len()
                    ),
                },
            );
        }
    }

    Ok(differences)
}

/// Writes the run's output as the new golden.
pub fn bless(run: &CaseRun, golden_dir: &Path) -> Result<(), String> {
    let artifacts_dir = golden_dir.join(ARTIFACTS_DIR);
    if artifacts_dir.exists() {
        fs::remove_dir_all(&artifacts_dir)
            .map_err(|error| format!("failed to clear golden artifacts: {error}"))?;
    }
    fs::create_dir_all(&artifacts_dir)
        .map_err(|error| format!("failed to create golden directory: {error}"))?;
    for (path, bytes) in &run.artifacts {
        let target = artifacts_dir.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::write(&target, bytes)
            .map_err(|error| format!("failed to write golden artifact `{path}`: {error}"))?;
    }
    fs::write(golden_dir.join(REPORT_NAME), render_report(run)?)
        .map_err(|error| format!("failed to write golden report: {error}"))
}

fn render_report(run: &CaseRun) -> Result<String, String> {
    let mut json = serde_json::to_string_pretty(&run.report)
        .map_err(|error| format!("failed to encode report: {error}"))?;
    json.push('\n');
    Ok(json)
}

fn list_artifacts(dir: &Path) -> Result<BTreeSet<String>, String> {
    let mut paths = BTreeSet::new();
    if !dir.exists() {
        return Ok(paths);
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(&current)
            .map_err(|error| format!("failed to read {}: {error}", current.display()))?
        {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let relative = path
                    .strip_prefix(dir)
                    .map_err(|error| error.to_string())?
                    .to_string_lossy()
                    .replace('\\', "/");
                paths.insert(relative);
            }
        }
    }
    Ok(paths)
}

/// A minimal line diff. Enough to read a golden change in a terminal without pulling in a
/// diffing crate for what is only ever shown to a human.
fn unified(expected: &str, actual: &str) -> String {
    let expected: Vec<&str> = expected.lines().collect();
    let actual: Vec<&str> = actual.lines().collect();
    let mut out = String::new();
    let mut shown = 0;
    for index in 0..expected.len().max(actual.len()) {
        let left = expected.get(index).copied();
        let right = actual.get(index).copied();
        if left == right {
            continue;
        }
        if shown >= 40 {
            out.push_str("  ... (further differences elided)\n");
            break;
        }
        shown += 1;
        if let Some(left) = left {
            out.push_str(&format!("  -{left}\n"));
        }
        if let Some(right) = right {
            out.push_str(&format!("  +{right}\n"));
        }
    }
    if out.is_empty() {
        out.push_str("  (differs only in trailing whitespace)\n");
    }
    out
}
