//! Executes cases in-process against `GeneratorRuntime`.
//!
//! In-process rather than by subprocess because module compilation dominates a run: a debug
//! plugin costs ~2.2 s to compile and ~1 ms to execute. Preparing each module once and
//! reusing it across every case that names it is the difference between a suite that runs in
//! seconds and one that runs in minutes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use generator_api::{ArtifactLimits, GeneratorModelView, QueryLimits};
use generator_host::{
    CancellationHandle, GeneratorRuntime, PreparedGenerator, RuntimeLimits, RuntimeOptions,
};
use rayon::prelude::*;
use semantic_publication::PublicationCoordinator;
use serde::Serialize;
use sysml_query::resolved_slice::PublishedModel;
use sysml_source::{SysmlDocument, SysmlDocumentSourceKind};

use crate::case::{Case, Expectation};

/// Everything a case produced, normalised for comparison.
///
/// Deliberately excludes wall time, `model_digest` and `spec42_version`: the first is not
/// reproducible, and the other two change on every Spec42 release, so including them would
/// make every golden churn without signalling anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CaseReport {
    pub outcome: Outcome,
    pub artifacts: BTreeMap<String, ArtifactRecord>,
    pub diagnostics: Vec<DiagnosticRecord>,
    pub query_count: u64,
    pub fuel_consumed: Option<u64>,
    pub peak_memory_bytes: usize,
    pub output_files: usize,
    pub output_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Outcome {
    Success,
    Failure {
        category: String,
        phase: String,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactRecord {
    pub bytes: usize,
    /// Present only for valid UTF-8, so text artifacts diff readably and binary ones do not
    /// pollute the report.
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiagnosticRecord {
    pub level: String,
    pub message: String,
    pub scoped: bool,
}

/// Outcome of running one case, before golden comparison.
pub struct CaseRun {
    pub case: Case,
    pub report: CaseReport,
    pub artifacts: BTreeMap<String, Vec<u8>>,
    pub duration: std::time::Duration,
}

pub struct Corpus {
    root: PathBuf,
    plugin_dir: PathBuf,
}

impl Corpus {
    pub fn new(root: PathBuf) -> Self {
        let plugin_dir = root
            .join("plugins/target/wasm32-unknown-unknown/release")
            .to_path_buf();
        Self { root, plugin_dir }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn plugin_path(&self, name: &str) -> PathBuf {
        self.plugin_dir
            .join(format!("spec42_conformance_{name}.wasm"))
    }

    fn model_path(&self, name: &str) -> PathBuf {
        self.root.join("models").join(name).join("model.sysml")
    }

    /// Runs every case, grouping by model and plugin so each snapshot is loaded once and each
    /// module compiled once.
    pub fn run(&self, cases: Vec<Case>) -> Result<Vec<CaseRun>, String> {
        let mut by_model: BTreeMap<String, Vec<Case>> = BTreeMap::new();
        for case in cases {
            by_model.entry(case.model.clone()).or_default().push(case);
        }

        let mut runs = Vec::new();
        for (model_name, cases) in by_model {
            let snapshot = self.load_model(&model_name)?;
            let mut by_plugin: BTreeMap<String, Vec<Case>> = BTreeMap::new();
            for case in cases {
                by_plugin.entry(case.plugin.clone()).or_default().push(case);
            }
            for (plugin_name, cases) in by_plugin {
                // Fuel metering is an engine-level setting, so cases that want the number run
                // on their own runtime. Grouping keeps that to at most two per plugin.
                for metered in [false, true] {
                    let subset: Vec<Case> = cases
                        .iter()
                        .filter(|case| case.meter_fuel == metered)
                        .cloned()
                        .collect();
                    if subset.is_empty() {
                        continue;
                    }
                    let runtime = GeneratorRuntime::with_options(RuntimeOptions {
                        fuel_metering: metered,
                        compilation_cache: false,
                    })
                    .map_err(|error| error.to_string())?;
                    let module =
                        std::fs::read(self.plugin_path(&plugin_name)).map_err(|error| {
                            format!(
                                "failed to read plugin `{plugin_name}` ({}): {error}. Run \
                             scripts/build-generator-plugins.sh",
                                self.plugin_path(&plugin_name).display()
                            )
                        })?;
                    let prepared = runtime.prepare(&module).map_err(|error| {
                        format!("failed to prepare plugin `{plugin_name}`: {error}")
                    })?;

                    let mut executed = subset
                        .into_par_iter()
                        .map(|case| run_one(&runtime, &prepared, &snapshot, case))
                        .collect::<Vec<_>>();
                    runs.append(&mut executed);
                }
            }
        }
        runs.sort_by(|left, right| left.case.id.cmp(&right.case.id));
        Ok(runs)
    }

    fn load_model(&self, name: &str) -> Result<Arc<PublishedModel>, String> {
        let path = self.model_path(name);
        if !path.is_file() {
            return Err(format!("model `{name}` not found at {}", path.display()));
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|error| format!("failed to read model `{name}`: {error}"))?;
        let source = SysmlDocument::from_memory_path(
            "generator-conformance",
            "model.sysml",
            content,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .map_err(|error| error.to_string())?;
        PublicationCoordinator::new()
            .publish(&[source], std::iter::empty::<Box<str>>())
            .map_err(|error| format!("failed to load model `{name}`: {error}"))
    }
}

fn run_one(
    runtime: &GeneratorRuntime,
    prepared: &PreparedGenerator,
    snapshot: &Arc<PublishedModel>,
    case: Case,
) -> CaseRun {
    // A fresh view per case: the handle index accumulates as elements are exposed, so a
    // shared view would let one case resolve a handle it never legitimately obtained,
    // masking exactly the unknown-handle behaviour the suite exists to pin.
    let model = Arc::new(GeneratorModelView::new(
        Arc::clone(snapshot),
        snapshot.publication().model_digest(),
        env!("CARGO_PKG_VERSION"),
        QueryLimits::default(),
    ));

    let defaults = ArtifactLimits::default();
    let limits = ArtifactLimits {
        max_files: case.artifact_limits.max_files.unwrap_or(defaults.max_files),
        max_file_bytes: case
            .artifact_limits
            .max_file_bytes
            .unwrap_or(defaults.max_file_bytes),
        max_total_bytes: case
            .artifact_limits
            .max_total_bytes
            .unwrap_or(defaults.max_total_bytes),
    };

    let started = std::time::Instant::now();
    let result = runtime.execute_prepared(
        prepared,
        model,
        &case.args,
        RuntimeLimits {
            fuel: case.meter_fuel.then_some(1_000_000_000),
            ..RuntimeLimits::default()
        },
        limits,
        CancellationHandle::new(),
    );
    let duration = started.elapsed();

    let (report, artifacts) = match result {
        Ok(execution) => {
            let artifacts: BTreeMap<String, Vec<u8>> = execution
                .artifacts
                .entries()
                .map(|(path, content)| (path.to_string(), content.to_vec()))
                .collect();
            let report = CaseReport {
                outcome: Outcome::Success,
                artifacts: artifacts
                    .iter()
                    .map(|(path, bytes)| {
                        (
                            path.clone(),
                            ArtifactRecord {
                                bytes: bytes.len(),
                                text: String::from_utf8(bytes.clone()).ok(),
                            },
                        )
                    })
                    .collect(),
                diagnostics: execution
                    .diagnostics
                    .iter()
                    .map(|diagnostic| DiagnosticRecord {
                        level: format!("{:?}", diagnostic.level).to_ascii_lowercase(),
                        message: diagnostic.message.clone(),
                        scoped: diagnostic.element_id.is_some(),
                    })
                    .collect(),
                query_count: execution.query_count,
                fuel_consumed: execution.fuel_consumed,
                peak_memory_bytes: execution.peak_memory_bytes,
                output_files: execution.artifacts.len(),
                output_bytes: execution.artifacts.total_bytes(),
            };
            (report, artifacts)
        }
        Err(error) => (
            CaseReport {
                outcome: Outcome::Failure {
                    category: format!("{:?}", error.category).to_ascii_lowercase(),
                    phase: error.phase.to_string(),
                    message: error.message.clone(),
                },
                artifacts: BTreeMap::new(),
                diagnostics: Vec::new(),
                query_count: 0,
                fuel_consumed: None,
                peak_memory_bytes: 0,
                output_files: 0,
                output_bytes: 0,
            },
            BTreeMap::new(),
        ),
    };

    CaseRun {
        case,
        report,
        artifacts,
        duration,
    }
}

/// Checks a run against the expectations declared in its case, independent of goldens.
pub fn check_expectations(run: &CaseRun) -> Vec<String> {
    let mut failures = Vec::new();
    let case = &run.case;

    match (&run.report.outcome, case.expect) {
        (Outcome::Success, Expectation::Failure) => {
            failures.push("expected failure but the generator succeeded".to_owned());
        }
        (Outcome::Failure { message, .. }, Expectation::Success) => {
            failures.push(format!(
                "expected success but the generator failed: {message}"
            ));
        }
        _ => {}
    }

    if let Outcome::Failure {
        category,
        phase,
        message,
    } = &run.report.outcome
    {
        if let Some(expected) = &case.failure.category {
            if expected != category {
                failures.push(format!("category: expected `{expected}`, got `{category}`"));
            }
        }
        if let Some(expected) = &case.failure.phase {
            if expected != phase {
                failures.push(format!("phase: expected `{expected}`, got `{phase}`"));
            }
        }
        if let Some(expected) = &case.failure.message_contains {
            if !message.contains(expected) {
                failures.push(format!("message does not contain `{expected}`: {message}"));
            }
        }
    }

    let assertions = &case.assertions;
    let mut compare = |label: &str, expected: Option<u64>, actual: u64| {
        if let Some(expected) = expected {
            if expected != actual {
                failures.push(format!("{label}: expected {expected}, got {actual}"));
            }
        }
    };
    compare(
        "query_count",
        assertions.query_count,
        run.report.query_count,
    );
    compare(
        "output_files",
        assertions.output_files.map(|value| value as u64),
        run.report.output_files as u64,
    );
    compare(
        "output_bytes",
        assertions.output_bytes.map(|value| value as u64),
        run.report.output_bytes as u64,
    );
    compare(
        "peak_memory_bytes",
        assertions.peak_memory_bytes.map(|value| value as u64),
        run.report.peak_memory_bytes as u64,
    );
    if let Some(expected) = assertions.fuel_consumed {
        match run.report.fuel_consumed {
            Some(actual) if actual != expected => {
                failures.push(format!("fuel_consumed: expected {expected}, got {actual}"));
            }
            None => failures.push(
                "fuel_consumed asserted but the case did not set meter_fuel = true".to_owned(),
            ),
            _ => {}
        }
    }

    failures
}
