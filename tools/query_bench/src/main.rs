//! Allocations per element for each benchmark case.
//!
//! Wall time is divan's job (`cargo bench -p spec42-query-bench`). This binary answers the other
//! half of the representation question: how many heap allocations one case performs, and how many
//! that is per element it produced. A counting global allocator is the only way to state that as a
//! measurement rather than an estimate, and it cannot share a binary with divan's own profiler --
//! hence a separate target over the same fixture.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use clap::Parser;
use serde::Serialize;
use spec42_query_bench::{
    cold_build, completion_position, navigation_position, outcome_len, view_outcome_len,
    warm_relink, Corpus, Fixture,
};
use sysml_query::resolved_slice::QueryAnswer;

/// A pass-through allocator that counts allocations and bytes.
///
/// It never changes what is allocated; it only records it, so the numbers describe the same run
/// the timing benchmark measures.
struct CountingAllocator;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        System.dealloc(pointer, layout)
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(
            new_size.saturating_sub(layout.size()) as u64,
            Ordering::Relaxed,
        );
        System.realloc(pointer, layout, new_size)
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// One measured case: what it allocated and how many elements it produced.
#[derive(Serialize)]
struct Measurement {
    case: &'static str,
    allocations: u64,
    bytes: u64,
    elements: usize,
    allocations_per_element: f64,
    bytes_per_element: f64,
}

#[derive(Parser)]
#[command(name = "spec42-query-bench-allocations")]
struct Cli {
    /// Write pretty JSON to this path instead of stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Serialize)]
struct Report {
    schema_version: u32,
    benchmark: &'static str,
    corpus: CorpusMetadata,
    environment: Environment,
    configuration: Configuration,
    measurements: Vec<Measurement>,
}

#[derive(Serialize)]
struct CorpusMetadata {
    digest: String,
    library_documents: usize,
    library_source_bytes: usize,
    published_elements: usize,
}

#[derive(Serialize)]
struct Environment {
    os: &'static str,
    architecture: &'static str,
    logical_parallelism: usize,
    rustc: Option<String>,
    git_commit: Option<String>,
    git_dirty: Option<bool>,
    build_profile: &'static str,
}

#[derive(Serialize)]
struct Configuration {
    iterations_per_case: usize,
    construction_schedule: &'static str,
    allocator: &'static str,
}

/// Runs `case` once and reports what it allocated.
///
/// The counters are read before and after on this thread's clock; the build cases use rayon
/// internally, and `Relaxed` ordering on a global counter still totals every thread's allocations
/// because the measurement is taken after the build has joined.
fn measure(case: &'static str, run: impl FnOnce() -> usize) -> Measurement {
    let before_allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let before_bytes = ALLOCATED_BYTES.load(Ordering::Relaxed);
    let elements = run();
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - before_allocations;
    let bytes = ALLOCATED_BYTES.load(Ordering::Relaxed) - before_bytes;
    let allocations_per_element = ratio(allocations, elements);
    let bytes_per_element = ratio(bytes, elements);
    Measurement {
        case,
        allocations,
        bytes,
        elements,
        allocations_per_element,
        bytes_per_element,
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let corpus = Corpus::load()?;
    let fixture = Fixture::build(&corpus)?;
    let elements = fixture.element_count;

    let mut measurements = Vec::new();

    measurements.push(measure("cold_build_stdlib", || {
        let model = cold_build(&corpus).expect("cold build");
        black_box(model.publication().completeness());
        elements
    }));

    measurements.push(measure("warm_relink_one_document", || {
        let model = warm_relink(&fixture, 1).expect("warm relink");
        black_box(model.publication().completeness());
        elements
    }));

    measurements.push(measure("q_visible_members", || {
        view_outcome_len(fixture.model.completion().visible_members(
            &fixture.workspace_document,
            completion_position(),
            None,
        ))
    }));

    measurements.push(measure("q_target_at", || {
        let outcome = fixture
            .model
            .navigation()
            .target_at(&fixture.workspace_document, navigation_position());
        black_box(outcome.completeness);
        match outcome.answer {
            QueryAnswer::Resolved(target) => {
                black_box(&target);
                1
            }
            _ => 0,
        }
    }));

    let reference_outcome = fixture
        .model
        .navigation()
        .target_at(&fixture.workspace_document, navigation_position());
    black_box(reference_outcome.completeness);
    let reference_symbol = match reference_outcome.answer {
        QueryAnswer::Resolved(target) => Some(target.symbol),
        _ => None,
    };
    if let Some(symbol) = reference_symbol {
        measurements.push(measure("q_references", || {
            outcome_len(fixture.model.navigation().references(symbol, true))
        }));
    }

    measurements.push(measure("q_document_symbols", || {
        outcome_len(
            fixture
                .model
                .inspection()
                .document_symbols(&fixture.outline_document),
        )
    }));

    measurements.push(measure("q_diagnostics_for_document", || {
        fixture
            .model
            .diagnostics()
            .for_document(&fixture.workspace_document)
            .len()
    }));

    let cold = measurements
        .iter()
        .find(|measurement| measurement.case == "cold_build_stdlib")
        .expect("the cold build case is always measured");
    if cold.elements == 0 {
        return Err("the cold build published no elements; the corpus or fixture is wrong".into());
    }
    let report = Report {
        schema_version: 1,
        benchmark: "spec42-query-bench-allocations",
        corpus: CorpusMetadata {
            digest: corpus.digest.clone(),
            library_documents: corpus.library.len(),
            library_source_bytes: corpus.library_bytes,
            published_elements: elements,
        },
        environment: environment(),
        configuration: Configuration {
            iterations_per_case: 1,
            construction_schedule: "publication-owner-default",
            allocator: "std::alloc::System counting wrapper",
        },
        measurements,
    };
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    if let Some(output) = cli.output {
        std::fs::write(&output, format!("{json}\n"))
            .map_err(|error| format!("{}: {error}", output.display()))?;
    } else {
        println!("{json}");
    }
    Ok(())
}

fn ratio(value: u64, elements: usize) -> f64 {
    if elements == 0 {
        0.0
    } else {
        value as f64 / elements as f64
    }
}

fn environment() -> Environment {
    Environment {
        os: std::env::consts::OS,
        architecture: std::env::consts::ARCH,
        logical_parallelism: std::thread::available_parallelism().map_or(1, usize::from),
        rustc: command_output("rustc", &["--version", "--verbose"]),
        git_commit: command_output("git", &["rev-parse", "HEAD"]),
        git_dirty: command_output(
            "git",
            &["status", "--porcelain", "--untracked-files=normal"],
        )
        .map(|status| !status.is_empty()),
        build_profile: env!("SPEC42_BENCH_BUILD_PROFILE"),
    }
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}
