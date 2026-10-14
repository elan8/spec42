//! Conformance and benchmark harness for the Spec42 generator ABI.
//!
//! Three jobs: prove the ABI behaves as specified, catch behavioural change as Spec42
//! evolves, and quantify the cost of changes over time. See `docs/generation/CONFORMANCE.md`.

pub mod case;
pub mod golden;
pub mod runner;

use std::path::{Path, PathBuf};

pub use case::{load_cases, Case};
pub use runner::{check_expectations, CaseRun, Corpus};

/// Result of checking one case.
pub struct CaseResult {
    pub id: String,
    pub failures: Vec<String>,
    pub duration: std::time::Duration,
}

impl CaseResult {
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Locates the default corpus, which lives beside the workspace rather than in the crate so
/// a downstream SDK can point `--corpus` at its own.
pub fn default_corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../generator-tests")
}

/// Runs every case in the corpus, optionally rewriting goldens instead of comparing.
pub fn run_corpus(
    root: &Path,
    filter: Option<&str>,
    bless: bool,
) -> Result<Vec<CaseResult>, String> {
    let mut cases = load_cases(root)?;
    if let Some(filter) = filter {
        cases.retain(|case| case.id.contains(filter));
        if cases.is_empty() {
            return Err(format!("no case matched `{filter}`"));
        }
    }

    let corpus = Corpus::new(root.to_path_buf());
    let runs = corpus.run(cases)?;

    let mut results = Vec::new();
    for run in runs {
        let mut failures = check_expectations(&run);
        let golden_dir = run.case.golden_dir(root);
        if bless {
            // Only bless cases whose declared expectations already hold; otherwise a broken
            // run would be recorded as the new truth.
            if failures.is_empty() {
                golden::bless(&run, &golden_dir)?;
            }
        } else {
            failures.extend(golden::compare(&run, &golden_dir)?);
        }
        results.push(CaseResult {
            id: run.case.id.clone(),
            failures,
            duration: run.duration,
        });
    }
    Ok(results)
}
