//! The declarative case format.
//!
//! Cases are data rather than Rust so adding one does not mean recompiling the harness, and
//! so a downstream SDK can point the same runner at its own corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// What a case expects to happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expectation {
    #[default]
    Success,
    Failure,
}

/// Metrics a case asserts exactly.
///
/// Only deterministic counters belong here. Wall time is deliberately absent: it varies run
/// to run even on an idle machine, so asserting it would make the suite flaky rather than
/// informative. Timing is reported, never gated.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertions {
    pub query_count: Option<u64>,
    pub fuel_consumed: Option<u64>,
    pub peak_memory_bytes: Option<usize>,
    pub output_files: Option<usize>,
    pub output_bytes: Option<usize>,
}

/// Expected failure shape, matched on the categorical fields rather than a message prefix
/// where possible, so wording can change without breaking the suite.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureExpectation {
    pub category: Option<String>,
    pub phase: Option<String>,
    /// Substring match, for the cases where the message is the point.
    pub message_contains: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactLimitOverrides {
    pub max_files: Option<usize>,
    pub max_file_bytes: Option<usize>,
    pub max_total_bytes: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct CaseFile {
    #[serde(default)]
    defaults: Defaults,
    #[serde(rename = "case", default)]
    cases: Vec<RawCase>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Defaults {
    model: Option<String>,
    plugin: Option<String>,
    expect: Option<Expectation>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCase {
    id: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    notes: Option<String>,
    model: Option<String>,
    plugin: Option<String>,
    #[serde(default)]
    args: Vec<String>,
    expect: Option<Expectation>,
    #[serde(default)]
    golden: Option<String>,
    #[serde(default)]
    assert: Assertions,
    #[serde(default)]
    failure: FailureExpectation,
    #[serde(default)]
    artifact_limits: ArtifactLimitOverrides,
    /// Request fuel metering so `fuel_consumed` can be asserted.
    #[serde(default)]
    meter_fuel: bool,
}

/// A case with defaults resolved.
#[derive(Debug, Clone)]
pub struct Case {
    pub id: String,
    pub description: Option<String>,
    pub model: String,
    pub plugin: String,
    pub args: Vec<String>,
    pub expect: Expectation,
    pub golden: Option<String>,
    pub assertions: Assertions,
    pub failure: FailureExpectation,
    pub artifact_limits: ArtifactLimitOverrides,
    pub meter_fuel: bool,
}

impl Case {
    /// Directory holding this case's expected artifacts and report.
    pub fn golden_dir(&self, root: &Path) -> PathBuf {
        root.join("golden")
            .join(self.golden.as_deref().unwrap_or(&self.id))
    }
}

/// Loads every `cases/*.toml` under `root`, applying per-file defaults.
pub fn load_cases(root: &Path) -> Result<Vec<Case>, String> {
    let dir = root.join("cases");
    let mut files = std::fs::read_dir(&dir)
        .map_err(|error| format!("failed to read {}: {error}", dir.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .collect::<Vec<_>>();
    files.sort();

    let mut cases = Vec::new();
    let mut seen = BTreeMap::new();
    for file in files {
        let text = std::fs::read_to_string(&file)
            .map_err(|error| format!("failed to read {}: {error}", file.display()))?;
        let parsed: CaseFile = toml::from_str(&text)
            .map_err(|error| format!("failed to parse {}: {error}", file.display()))?;
        for raw in parsed.cases {
            let case = Case {
                model: raw
                    .model
                    .or_else(|| parsed.defaults.model.clone())
                    .ok_or_else(|| format!("case `{}` has no model", raw.id))?,
                plugin: raw
                    .plugin
                    .or_else(|| parsed.defaults.plugin.clone())
                    .ok_or_else(|| format!("case `{}` has no plugin", raw.id))?,
                expect: raw.expect.or(parsed.defaults.expect).unwrap_or_default(),
                description: raw.description.or(raw.notes),
                args: raw.args,
                golden: raw.golden,
                assertions: raw.assert,
                failure: raw.failure,
                artifact_limits: raw.artifact_limits,
                meter_fuel: raw.meter_fuel,
                id: raw.id,
            };
            if let Some(previous) = seen.insert(case.id.clone(), file.clone()) {
                return Err(format!(
                    "duplicate case id `{}` in {} and {}",
                    case.id,
                    previous.display(),
                    file.display()
                ));
            }
            cases.push(case);
        }
    }
    Ok(cases)
}
