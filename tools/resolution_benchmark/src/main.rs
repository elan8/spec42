//! Reproducible report-only gate for canonical whole-model resolution.
//!
//! The workload definitions point at checked-in snapshot SOURCE sections. This command records
//! raw observations and structural work counters; it deliberately has no machine-specific pass
//! threshold and never compares against the removed scoped resolver.

// The parser AST's body-element enums nest deeply enough that rustdoc exceeds the default
// limit proving auto-trait bounds (`RefUnwindSafe`) for the types reachable from a published
// model. Compilation is unaffected; only documentation generation walks the whole closure.
#![recursion_limit = "256"]

use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use serde::Serialize;
use sysml_query::resolved_slice::{
    build_measured, AdmittedSource as QuerySourceDocument, BuildMeasurements, BuildRequest,
    ConstructionStrategy, PublicationCompleteness, PublishedModel, QueryOutcome, SourceKind,
    SpecializationScope, SymbolIdentity, TextPosition,
};

const REVIEWED_PRIMARY_TARGET_NS: u64 = 1_000_000_000;

#[derive(Debug, Parser)]
#[command(
    name = "spec42-resolution-benchmark",
    about = "Record the canonical whole-model resolution performance gate"
)]
struct Cli {
    /// Repository root containing the workload manifests and snapshots.
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
    /// Run only one workload size. The default runs small, medium, and large.
    #[arg(long, value_enum)]
    model: Option<ModelSize>,
    /// Number of independent fresh whole-model samples per workload.
    #[arg(long, default_value_t = 3)]
    iterations: usize,
    /// Number of editor replacement builds per workload.
    #[arg(long, default_value_t = 5)]
    replacement_builds: usize,
    /// Repetitions of each downstream workload after every fresh build.
    #[arg(long, default_value_t = 3)]
    query_repetitions: usize,
    /// Write pretty JSON to this path instead of stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ModelSize {
    Small,
    Medium,
    Large,
    StandardLibrary,
    ManyFiles,
}

impl ModelSize {
    fn name(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::StandardLibrary => "standard-library",
            Self::ManyFiles => "many-files",
        }
    }
}

#[derive(Debug, Serialize)]
struct Report {
    schema_version: u32,
    command: String,
    environment: Environment,
    methodology: Methodology,
    unavailable_measurements: Vec<UnavailableMeasurement>,
    workloads: Vec<WorkloadReport>,
}

#[derive(Debug, Serialize)]
struct Environment {
    os: &'static str,
    architecture: &'static str,
    logical_parallelism: usize,
    build_profile: &'static str,
}

#[derive(Debug, Serialize)]
struct Methodology {
    construction: &'static str,
    evaluation: &'static str,
    fresh_build_definition: &'static str,
    replacement_definition: &'static str,
    acceptance_rule: &'static str,
    query_evidence: &'static str,
}

#[derive(Debug, Serialize)]
struct UnavailableMeasurement {
    name: &'static str,
    reason: &'static str,
}

#[derive(Debug, Serialize)]
struct WorkloadReport {
    name: String,
    manifest: String,
    inputs: Vec<ManifestInput>,
    input: InputFacts,
    reviewed_target: Option<TargetAssessment>,
    fresh_builds: Vec<FreshBuildSample>,
    fresh_build_wall_time_ns: SampleSummary,
    downstream_queries: Vec<Vec<QueryMeasurement>>,
    replacements: Vec<ReplacementSample>,
    replacement_wall_time_ns: SampleSummary,
    query_contract_gaps: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct InputFacts {
    documents: usize,
    source_bytes: usize,
}

#[derive(Debug, Serialize)]
struct TargetAssessment {
    requirement: &'static str,
    target_wall_time_ns: u64,
    observed_max_wall_time_ns: u64,
    status: &'static str,
    enforcement: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct ManifestInput {
    source_kind: &'static str,
    snapshot: String,
    expanded_documents: usize,
}

#[derive(Debug, Serialize)]
struct FreshBuildSample {
    source_acquisition_ns: u64,
    request_preparation_ns: u64,
    publication_build_wall_time_ns: u64,
    request_and_build_wall_time_ns: u64,
    publication_completeness: &'static str,
    /// Measured by the publication owner at its own phase barriers, not inferred by this tool.
    owner_phases: OwnerPhases,
}

/// Elapsed time at the publication's stable phase barriers.
///
/// Parallel phases report wall time rather than summed worker CPU time, and source acquisition
/// and request construction happen outside them, so these do not sum to the build wall time.
#[derive(Debug, Serialize)]
struct OwnerPhases {
    parse_ns: u64,
    lowering_ns: u64,
    resolution_ns: u64,
}

impl From<BuildMeasurements> for OwnerPhases {
    fn from(measurements: BuildMeasurements) -> Self {
        Self {
            parse_ns: nanos(measurements.parse),
            lowering_ns: nanos(measurements.lowering),
            resolution_ns: nanos(measurements.resolution),
        }
    }
}

#[derive(Debug, Serialize)]
struct QueryMeasurement {
    name: &'static str,
    implementation: &'static str,
    elapsed_ns: u64,
    repetitions: usize,
    operation_count: u64,
    result_count: Option<u64>,
    output_bytes: u64,
}

#[derive(Debug, Serialize)]
struct ReplacementSample {
    source_preparation_ns: u64,
    request_preparation_ns: u64,
    publication_build_wall_time_ns: u64,
    request_and_build_wall_time_ns: u64,
    publication_completeness: &'static str,
    prior_model_reader_preparation_ns: u64,
    prior_model_reader_batches: u64,
    prior_model_reader_operations: u64,
    prior_model_reader_results: u64,
}

#[derive(Debug, Serialize)]
struct SampleSummary {
    samples: usize,
    min: Option<u64>,
    median: Option<u64>,
    max: Option<u64>,
}

struct LoadedWorkload {
    documents: Vec<BenchmarkDocument>,
    inputs: Vec<ManifestInput>,
}

#[derive(Clone)]
struct PreparedQueries {
    probes: Vec<(usize, TextPosition)>,
    /// The symbols the probes actually landed on, deduplicated. Every downstream query below is
    /// keyed on one of these, so result counts describe real published elements rather than the
    /// probe count.
    symbols: Vec<SymbolIdentity>,
}

#[derive(Clone)]
struct BenchmarkDocument {
    path: String,
    text: String,
    source_kind: SourceKind,
}

#[derive(Default)]
struct QueryCounts {
    operations: u64,
    results: u64,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    if cli.iterations == 0 || cli.query_repetitions == 0 {
        return Err("--iterations and --query-repetitions must be positive".to_string());
    }
    let sizes = cli
        .model
        .map(|model| vec![model])
        .unwrap_or_else(|| vec![ModelSize::Small, ModelSize::Medium, ModelSize::Large]);
    let workloads = sizes
        .into_iter()
        .map(|size| run_workload(&cli, size))
        .collect::<Result<Vec<_>, _>>()?;
    let report = Report {
        schema_version: 1,
        command: format!(
            "cargo run --release -p spec42-resolution-benchmark -- {}--iterations {} --replacement-builds {} --query-repetitions {}",
            cli.model
                .map(|model| format!("--model {} ", model.name()))
                .unwrap_or_default(),
            cli.iterations,
            cli.replacement_builds,
            cli.query_repetitions
        ),
        environment: Environment {
            os: std::env::consts::OS,
            architecture: std::env::consts::ARCH,
            logical_parallelism: thread::available_parallelism().map_or(1, usize::from),
            build_profile: if cfg!(debug_assertions) { "debug" } else { "release" },
        },
        methodology: Methodology {
            construction: "BuildRequest selects the parallel construction schedule; the opaque publication owner controls internal scheduling and deterministic canonicalization",
            evaluation: "the publication evaluates supported constant expressions; per-phase parse, lowering, and resolution wall times are reported by the owner at its own barriers",
            fresh_build_definition: "a new immutable source snapshot and opaque PublishedModel are constructed for every sample; no semantic cache is reused, while OS filesystem and allocator caches are not controlled",
            replacement_definition: "one workspace document receives a fixed-size comment edit; a reader thread repeatedly queries the prior immutable publication while the replacement model is built",
            acceptance_rule: "report-only architectural review; there is no machine-specific timing threshold and no scoped-resolver fallback",
            query_evidence: "source-document adapter reconstruction is reported separately from timed queries; query timings are paired with operation/result counters and an implementation classification so scans cannot masquerade as indexed work. Every timed query reads settled publication facts and performs no resolution",
        },
        unavailable_measurements: vec![
            UnavailableMeasurement {
                name: "portable_peak_memory_bytes",
                reason: "the build owner has no portable per-phase allocator accounting; process RSS would mix tool, parser, and prior-reader memory and is not reported as a precise build metric",
            },
            UnavailableMeasurement {
                name: "semantic_structure_and_work_counters",
                reason: "node, reference, solver-pass, changed-slot, and index-work counters are not published by the resolution owner; this tool does not reconstruct them by scanning storage or parsing a debug projection",
            },
            UnavailableMeasurement {
                name: "generator_traversal",
                reason: "generators consume the immutable publication, but wiring a generator host into this gate is separate work; it is not substituted with a projection render",
            },
        ],
        workloads,
    };
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| format!("serialize report: {error}"))?;
    if let Some(output) = cli.output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!("{}: create report directory: {error}", parent.display())
            })?;
        }
        fs::write(&output, format!("{json}\n"))
            .map_err(|error| format!("{}: write report: {error}", output.display()))?;
        eprintln!("wrote {}", output.display());
    } else {
        println!("{json}");
    }
    Ok(())
}

fn run_workload(cli: &Cli, size: ModelSize) -> Result<WorkloadReport, String> {
    let relative_manifest = PathBuf::from(format!(
        "tests/benchmarks/canonical_resolution/{}.txt",
        size.name()
    ));
    let manifest = cli.repo_root.join(&relative_manifest);
    let first = load_workload(&cli.repo_root, &manifest, size.name())?;
    let inputs = first.inputs.clone();
    let input = InputFacts {
        documents: first.documents.len(),
        source_bytes: first
            .documents
            .iter()
            .map(|document| document.text.len())
            .sum(),
    };
    let mut fresh_builds = Vec::new();
    let mut downstream_queries = Vec::new();
    let mut last_documents = first.documents;
    let mut last_model = None;
    for _ in 0..cli.iterations {
        let acquisition_started = Instant::now();
        let loaded = load_workload(&cli.repo_root, &manifest, size.name())?;
        let source_acquisition_ns = nanos(acquisition_started.elapsed());
        let documents = loaded.documents;
        let built = build_model(&documents)?;
        let request_and_build_wall_time_ns = built
            .request_preparation_ns
            .saturating_add(built.publication_build_wall_time_ns);
        downstream_queries.push(measure_downstream(
            &built.model,
            &documents,
            cli.query_repetitions,
        )?);
        fresh_builds.push(FreshBuildSample {
            source_acquisition_ns,
            request_preparation_ns: built.request_preparation_ns,
            publication_build_wall_time_ns: built.publication_build_wall_time_ns,
            request_and_build_wall_time_ns,
            publication_completeness: completeness_name(&built.model),
            owner_phases: built.measurements.into(),
        });
        last_documents = documents;
        last_model = Some(Arc::new(built.model));
    }
    let (replacements, _) = measure_replacements(
        last_model.expect("positive iterations"),
        &last_documents,
        cli.replacement_builds,
    )?;
    let fresh_build_wall_time_ns = summarize(
        fresh_builds
            .iter()
            .map(|sample| sample.request_and_build_wall_time_ns),
    );
    let reviewed_target =
        assess_reviewed_target(size, &fresh_build_wall_time_ns, !cfg!(debug_assertions));
    Ok(WorkloadReport {
        name: size.name().to_string(),
        manifest: relative_manifest.to_string_lossy().into_owned(),
        inputs,
        input,
        reviewed_target,
        fresh_build_wall_time_ns,
        replacement_wall_time_ns: summarize(
            replacements
                .iter()
                .map(|sample| sample.request_and_build_wall_time_ns),
        ),
        fresh_builds,
        downstream_queries,
        replacements,
        query_contract_gaps: vec![
            "canonical model projection is a deliberate full publication scan and is reported separately from indexed point queries",
        ],
    })
}

struct BuiltModel {
    model: PublishedModel,
    request_preparation_ns: u64,
    publication_build_wall_time_ns: u64,
    measurements: BuildMeasurements,
}

fn build_model(documents: &[BenchmarkDocument]) -> Result<BuiltModel, String> {
    let request_started = Instant::now();
    let sources = query_documents(documents)?;
    let request = BuildRequest::resolved(sources, ConstructionStrategy::Parallel)
        .map_err(|error| format!("invalid benchmark source snapshot: {error}"))?;
    let request_preparation_ns = nanos(request_started.elapsed());
    let build_started = Instant::now();
    let (model, measurements) =
        build_measured(request).map_err(|error| format!("semantic build failed: {error}"))?;
    Ok(BuiltModel {
        model,
        request_preparation_ns,
        publication_build_wall_time_ns: nanos(build_started.elapsed()),
        measurements,
    })
}

/// The publication's own settled phase, not a summary of it.
///
/// The immutable owner distinguishes three ways a build can fail to settle where the retired
/// facade reported one "editor recovery" state, so a sample that did not converge can no longer
/// be read as one that merely recovered from a parse error.
fn completeness_name(model: &PublishedModel) -> &'static str {
    match model.publication().completeness() {
        PublicationCompleteness::Complete => "complete",
        PublicationCompleteness::ParseRecovery => "parse-recovery",
        PublicationCompleteness::UnsupportedSyntax => "unsupported-syntax",
        PublicationCompleteness::NonConverged => "non-converged",
    }
}

fn measure_downstream(
    model: &PublishedModel,
    documents: &[BenchmarkDocument],
    repetitions: usize,
) -> Result<Vec<QueryMeasurement>, String> {
    let source_adapter_started = Instant::now();
    let query_documents = query_documents(documents)?;
    let source_adapter_elapsed = source_adapter_started.elapsed();
    let preparation_started = Instant::now();
    let prepared = prepare_queries(model, documents, &query_documents)?;
    let preparation_elapsed = preparation_started.elapsed();
    let mut measurements = vec![QueryMeasurement {
        name: "source_adapter_preparation",
        implementation: "reconstruct opaque facade source documents from retained benchmark inputs; excluded from query timings",
        elapsed_ns: nanos(source_adapter_elapsed),
        repetitions: 1,
        operation_count: documents.len() as u64,
        result_count: Some(query_documents.len() as u64),
        output_bytes: 0,
    }, QueryMeasurement {
        name: "query_probe_preparation",
        implementation: "bounded source-token probe selection plus eager navigation-index queries; no semantic storage scan",
        elapsed_ns: nanos(preparation_elapsed),
        repetitions: 1,
        operation_count: prepared.probes.len() as u64,
        result_count: Some(prepared.symbols.len() as u64),
        output_bytes: 0,
    }];

    let navigation_started = Instant::now();
    let mut navigation = QueryCounts::default();
    for _ in 0..repetitions {
        let counts = run_navigation_queries(model, &query_documents, &prepared);
        navigation.operations += counts.operations;
        navigation.results += counts.results;
    }
    measurements.push(QueryMeasurement {
        name: "navigation_at_source_positions",
        implementation: "eager immutable per-document interval index",
        elapsed_ns: nanos(navigation_started.elapsed()),
        repetitions,
        operation_count: navigation.operations,
        result_count: Some(navigation.results),
        output_bytes: 0,
    });

    let reference_started = Instant::now();
    let mut references = QueryCounts::default();
    for _ in 0..repetitions {
        let counts = run_reference_queries(model, &prepared);
        references.operations += counts.operations;
        references.results += counts.results;
    }
    measurements.push(QueryMeasurement {
        name: "references_for_resolved_symbols",
        implementation: "eager reverse-reference index keyed by published symbol identity",
        elapsed_ns: nanos(reference_started.elapsed()),
        repetitions,
        operation_count: references.operations,
        result_count: Some(references.results),
        output_bytes: 0,
    });

    let type_started = Instant::now();
    let mut types = QueryCounts::default();
    for _ in 0..repetitions {
        let counts = run_type_queries(model, &prepared);
        types.operations += counts.operations;
        types.results += counts.results;
    }
    measurements.push(QueryMeasurement {
        name: "effective_types_and_supertypes",
        implementation: "settled publication-barrier type index; no traversal per call",
        elapsed_ns: nanos(type_started.elapsed()),
        repetitions,
        operation_count: types.operations,
        result_count: Some(types.results),
        output_bytes: 0,
    });

    let diagnostics_started = Instant::now();
    let mut diagnostic_results = 0u64;
    for _ in 0..repetitions {
        let published = model.diagnostics().published();
        diagnostic_results += published.diagnostics.len() as u64;
        black_box(published);
    }
    measurements.push(QueryMeasurement {
        name: "diagnostics",
        implementation:
            "typed diagnostics settled at the publication barrier; each call is a read, not a scan",
        elapsed_ns: nanos(diagnostics_started.elapsed()),
        repetitions,
        operation_count: repetitions as u64,
        result_count: Some(diagnostic_results),
        output_bytes: 0,
    });

    let projection_started = Instant::now();
    let mut projection_bytes = 0u64;
    for _ in 0..repetitions {
        let mut output = String::new();
        model
            .debug()
            .write_semantic_sexpr(&mut output)
            .map_err(|error| format!("model projection failed: {error}"))?;
        projection_bytes += output.len() as u64;
        black_box(output);
    }
    measurements.push(QueryMeasurement {
        name: "canonical_model_projection",
        implementation:
            "intentional full read-only publication traversal; not an indexed point query",
        elapsed_ns: nanos(projection_started.elapsed()),
        repetitions,
        operation_count: repetitions as u64,
        result_count: Some(repetitions as u64),
        output_bytes: projection_bytes,
    });
    Ok(measurements)
}

/// Selects the probe positions and resolves each once, so the timed workloads below query
/// published symbols rather than re-deriving what a position points at on every repetition.
fn prepare_queries(
    model: &PublishedModel,
    documents: &[BenchmarkDocument],
    query_documents: &[QuerySourceDocument],
) -> Result<PreparedQueries, String> {
    let probes = source_probes(documents)?;
    let mut symbols = std::collections::BTreeSet::new();
    for (document, position) in &probes {
        if let QueryOutcome::Resolved(target) | QueryOutcome::Recovered(target) = model
            .navigation()
            .target_at(query_documents[*document].identity(), *position)
        {
            symbols.insert(target.symbol);
        }
    }
    Ok(PreparedQueries {
        probes,
        symbols: symbols.into_iter().collect(),
    })
}

fn run_navigation_queries(
    model: &PublishedModel,
    documents: &[QuerySourceDocument],
    prepared: &PreparedQueries,
) -> QueryCounts {
    let mut counts = QueryCounts::default();
    for (document, position) in &prepared.probes {
        counts.operations += 1;
        let outcome = model
            .navigation()
            .target_at(documents[*document].identity(), *position);
        counts.results += u64::from(matches!(
            outcome,
            QueryOutcome::Resolved(_) | QueryOutcome::Recovered(_)
        ));
        black_box(outcome);
    }
    counts
}

fn run_reference_queries(model: &PublishedModel, prepared: &PreparedQueries) -> QueryCounts {
    let mut counts = QueryCounts::default();
    for symbol in &prepared.symbols {
        counts.operations += 1;
        let occurrences = model.navigation().references(symbol, true);
        if let QueryOutcome::Resolved(locations) | QueryOutcome::Recovered(locations) = &occurrences
        {
            counts.results += locations.len() as u64;
        }
        black_box(occurrences);
    }
    counts
}

fn run_type_queries(model: &PublishedModel, prepared: &PreparedQueries) -> QueryCounts {
    let mut counts = QueryCounts::default();
    for symbol in &prepared.symbols {
        counts.operations += 2;
        let effective = model.types().effective_types(symbol);
        let supertypes = model
            .types()
            .all_supertypes(symbol, SpecializationScope::AnySpecialization);
        if let QueryOutcome::Resolved(types) | QueryOutcome::Recovered(types) = &effective {
            counts.results += types.len() as u64;
        }
        if let QueryOutcome::Resolved(types) | QueryOutcome::Recovered(types) = &supertypes {
            counts.results += types.len() as u64;
        }
        black_box((effective, supertypes));
    }
    counts
}

fn measure_replacements(
    mut prior: Arc<PublishedModel>,
    base_documents: &[BenchmarkDocument],
    replacement_builds: usize,
) -> Result<(Vec<ReplacementSample>, Arc<PublishedModel>), String> {
    let mut samples = Vec::new();
    for replacement in 0..replacement_builds {
        let preparation_started = Instant::now();
        let documents = replacement_documents(base_documents, replacement)?;
        let source_preparation_ns = nanos(preparation_started.elapsed());

        let reader_preparation_started = Instant::now();
        let reader_documents = query_documents(base_documents)?;
        let prepared = prepare_queries(&prior, base_documents, &reader_documents)?;
        let prior_model_reader_preparation_ns = nanos(reader_preparation_started.elapsed());
        let stop = Arc::new(AtomicBool::new(false));
        let reader_stop = Arc::clone(&stop);
        let reader_model = Arc::clone(&prior);
        let reader = thread::spawn(move || {
            let mut batches = 0u64;
            let mut operations = 0u64;
            let mut results = 0u64;
            while !reader_stop.load(Ordering::Acquire) {
                let navigation =
                    run_navigation_queries(&reader_model, &reader_documents, &prepared);
                let references = run_reference_queries(&reader_model, &prepared);
                batches += 1;
                operations += navigation.operations + references.operations;
                results += navigation.results + references.results;
            }
            (batches, operations, results)
        });

        let built = build_model(&documents)?;
        let request_and_build_wall_time_ns = built
            .request_preparation_ns
            .saturating_add(built.publication_build_wall_time_ns);
        stop.store(true, Ordering::Release);
        let (batches, operations, results) = reader
            .join()
            .map_err(|_| "prior-model reader thread panicked".to_string())?;
        samples.push(ReplacementSample {
            source_preparation_ns,
            request_preparation_ns: built.request_preparation_ns,
            publication_build_wall_time_ns: built.publication_build_wall_time_ns,
            request_and_build_wall_time_ns,
            publication_completeness: completeness_name(&built.model),
            prior_model_reader_preparation_ns,
            prior_model_reader_batches: batches,
            prior_model_reader_operations: operations,
            prior_model_reader_results: results,
        });
        prior = Arc::new(built.model);
    }
    Ok((samples, prior))
}

fn replacement_documents(
    base_documents: &[BenchmarkDocument],
    replacement: usize,
) -> Result<Vec<BenchmarkDocument>, String> {
    let edit_marker = if replacement.is_multiple_of(2) {
        "// editor replacement a\n"
    } else {
        "// editor replacement b\n"
    };
    let edit_index = base_documents
        .iter()
        .position(|document| {
            matches!(
                document.source_kind,
                SourceKind::Workspace | SourceKind::External
            )
        })
        .ok_or_else(|| "replacement workload has no workspace document".to_string())?;
    base_documents
        .iter()
        .enumerate()
        .map(|(index, document)| {
            let text = if index == edit_index {
                format!("{}\n{edit_marker}", document.text)
            } else {
                document.text.clone()
            };
            Ok(BenchmarkDocument {
                path: document.path.clone(),
                text,
                source_kind: document.source_kind,
            })
        })
        .collect()
}

fn load_workload(repo_root: &Path, manifest: &Path, name: &str) -> Result<LoadedWorkload, String> {
    let manifest_text = fs::read_to_string(manifest)
        .map_err(|error| format!("{}: read manifest: {error}", manifest.display()))?;
    let mut documents = Vec::new();
    let mut inputs = Vec::new();
    for (line_index, raw_line) in manifest_text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        let kind = fields.next().expect("non-empty line");
        let relative = fields.next().ok_or_else(|| {
            format!(
                "{}:{}: expected `<source-kind> <snapshot>`",
                manifest.display(),
                line_index + 1
            )
        })?;
        if fields.next().is_some() {
            return Err(format!(
                "{}:{}: unexpected manifest fields",
                manifest.display(),
                line_index + 1
            ));
        }
        let input_ordinal = inputs.len();
        if kind == "standard-library-directory" {
            let directory = repo_root.join(relative);
            let mut snapshots = fs::read_dir(&directory)
                .map_err(|error| {
                    format!("{}: read source directory: {error}", directory.display())
                })?
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
                .collect::<Vec<_>>();
            snapshots.sort();
            let before = documents.len();
            for (snapshot_ordinal, snapshot) in snapshots.iter().enumerate() {
                let sources = read_snapshot_sources(snapshot)?;
                append_sources(
                    &mut documents,
                    name,
                    input_ordinal,
                    snapshot_ordinal,
                    sources,
                    SourceKind::StandardLibrary,
                )?;
            }
            inputs.push(ManifestInput {
                source_kind: "standard-library",
                snapshot: relative.to_string(),
                expanded_documents: documents.len() - before,
            });
            continue;
        }
        if let Some(repetitions) = kind.strip_prefix("workspace-repeat:") {
            let repetitions = repetitions.parse::<usize>().map_err(|error| {
                format!(
                    "{}:{}: invalid repeat count {repetitions:?}: {error}",
                    manifest.display(),
                    line_index + 1
                )
            })?;
            let sources = read_snapshot_sources(&repo_root.join(relative))?;
            let before = documents.len();
            for repetition in 0..repetitions {
                for (source_ordinal, source) in sources.iter().enumerate() {
                    let package = format!("Benchmark{repetition:04}");
                    let content =
                        source
                            .text
                            .replacen("package Demo", &format!("package {package}"), 1);
                    if content == source.text {
                        return Err(format!(
                            "{relative}: repeated benchmark source must contain `package Demo`"
                        ));
                    }
                    let path = format!(
                        "{name}/{input_ordinal:03}/{repetition:05}/{source_ordinal:03}-{}",
                        source.name
                    );
                    documents.push(BenchmarkDocument {
                        path,
                        text: content,
                        source_kind: SourceKind::Workspace,
                    });
                }
            }
            inputs.push(ManifestInput {
                source_kind: "workspace-derived-repeat",
                snapshot: relative.to_string(),
                expanded_documents: documents.len() - before,
            });
            continue;
        }
        let (source_kind, source_kind_name) = match kind {
            "workspace" => (SourceKind::Workspace, "workspace"),
            "library" => (SourceKind::Library, "library"),
            "standard-library" => (SourceKind::StandardLibrary, "standard-library"),
            _ => {
                return Err(format!(
                    "{}:{}: unknown source kind {kind:?}",
                    manifest.display(),
                    line_index + 1
                ))
            }
        };
        let sources = read_snapshot_sources(&repo_root.join(relative))?;
        let expanded_documents = sources.len();
        append_sources(&mut documents, name, input_ordinal, 0, sources, source_kind)?;
        inputs.push(ManifestInput {
            source_kind: source_kind_name,
            snapshot: relative.to_string(),
            expanded_documents,
        });
    }
    if documents.is_empty() {
        return Err(format!("{}: workload is empty", manifest.display()));
    }
    Ok(LoadedWorkload { documents, inputs })
}

fn read_snapshot_sources(snapshot_path: &Path) -> Result<Vec<SourceDocument>, String> {
    let fixture = fs::read_to_string(snapshot_path)
        .map_err(|error| format!("{}: read snapshot: {error}", snapshot_path.display()))?;
    let fallback = snapshot_path
        .file_name()
        .and_then(|file| file.to_str())
        .unwrap_or("snapshot.md");
    parse_source_documents(&fixture, fallback)
}

fn append_sources(
    documents: &mut Vec<BenchmarkDocument>,
    workload: &str,
    input_ordinal: usize,
    snapshot_ordinal: usize,
    sources: Vec<SourceDocument>,
    source_kind: SourceKind,
) -> Result<(), String> {
    for (source_ordinal, source) in sources.into_iter().enumerate() {
        let path = format!(
            "{workload}/{input_ordinal:03}/{snapshot_ordinal:03}/{source_ordinal:03}-{}",
            source.name
        );
        documents.push(BenchmarkDocument {
            path,
            text: source.text,
            source_kind,
        });
    }
    Ok(())
}

fn query_documents(documents: &[BenchmarkDocument]) -> Result<Vec<QuerySourceDocument>, String> {
    documents
        .iter()
        .map(|document| {
            QuerySourceDocument::from_memory_path(
                "resolution-benchmark",
                &document.path,
                document.text.clone(),
                document.source_kind,
            )
            .map_err(|error| format!("{}: construct query document: {error}", document.path))
        })
        .collect()
}

/// Select a bounded, deterministic set of identifier positions from each checked-in source.
/// This is source workload preparation, not a semantic scan. Query result counts reveal how many
/// probes intersect references in the eager navigation index.
fn source_probes(documents: &[BenchmarkDocument]) -> Result<Vec<(usize, TextPosition)>, String> {
    const MAX_PROBES_PER_DOCUMENT: usize = 256;

    let mut probes = Vec::new();
    for (document_index, document) in documents.iter().enumerate() {
        let mut selected = 0usize;
        for (line_index, line) in document.text.lines().enumerate() {
            let mut previous_was_identifier = false;
            for (byte_index, character) in line.char_indices() {
                let is_identifier = character == '_' || character.is_alphanumeric();
                if is_identifier && !previous_was_identifier {
                    let utf16_character = line[..byte_index].encode_utf16().count();
                    let line = u32::try_from(line_index).map_err(|_| {
                        format!(
                            "{}: line index does not fit the query position contract",
                            document.path
                        )
                    })?;
                    let character = u32::try_from(utf16_character).map_err(|_| {
                        format!(
                            "{}: UTF-16 column does not fit the query position contract",
                            document.path
                        )
                    })?;
                    probes.push((document_index, TextPosition { line, character }));
                    selected += 1;
                    if selected == MAX_PROBES_PER_DOCUMENT {
                        break;
                    }
                }
                previous_was_identifier = is_identifier;
            }
            if selected == MAX_PROBES_PER_DOCUMENT {
                break;
            }
        }
    }
    Ok(probes)
}

struct SourceDocument {
    name: String,
    text: String,
}

fn parse_source_documents(
    fixture: &str,
    fallback_name: &str,
) -> Result<Vec<SourceDocument>, String> {
    let source = raw_section(fixture, "SOURCE")
        .ok_or_else(|| format!("{fallback_name}: missing # SOURCE section"))?;
    let mut named = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let Some((name, rest)) = cursor.split_once('\n') else {
            return Err(format!("{fallback_name}: malformed named SOURCE document"));
        };
        let Some((text, after)) = fenced_block(rest) else {
            return Err(format!(
                "{fallback_name}: malformed SOURCE fence for {name}"
            ));
        };
        named.push(SourceDocument {
            name: name.trim().to_string(),
            text,
        });
        cursor = after;
    }
    if !named.is_empty() {
        return Ok(named);
    }
    fenced_block(source)
        .map(|(text, _)| {
            vec![SourceDocument {
                name: fallback_name.to_string(),
                text,
            }]
        })
        .ok_or_else(|| format!("{fallback_name}: malformed SOURCE fence"))
}

fn raw_section<'a>(fixture: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("# {name}\n");
    let start = fixture.find(&marker)? + marker.len();
    let rest = &fixture[start..];
    let end = rest.find("\n# ").unwrap_or(rest.len());
    Some(&rest[..end])
}

fn fenced_block(input: &str) -> Option<(String, &str)> {
    let start = input.find("~~~")?;
    let after_open = &input[start + 3..];
    let (_, body) = after_open.split_once('\n')?;
    if let Some(after_close) = body.strip_prefix("~~~") {
        return Some((String::new(), after_close));
    }
    let end = body.find("\n~~~")?;
    Some((body[..end].to_string(), &body[end + 4..]))
}

fn summarize(values: impl Iterator<Item = u64>) -> SampleSummary {
    let mut values = values.collect::<Vec<_>>();
    values.sort_unstable();
    SampleSummary {
        samples: values.len(),
        min: values.first().copied(),
        median: values.get(values.len() / 2).copied(),
        max: values.last().copied(),
    }
}

fn assess_reviewed_target(
    size: ModelSize,
    summary: &SampleSummary,
    release_build: bool,
) -> Option<TargetAssessment> {
    matches!(size, ModelSize::StandardLibrary | ModelSize::ManyFiles).then(|| {
        let observed = summary.max.unwrap_or(u64::MAX);
        TargetAssessment {
            requirement: "complete release-mode canonical publication in under one second",
            target_wall_time_ns: REVIEWED_PRIMARY_TARGET_NS,
            observed_max_wall_time_ns: observed,
            status: if !release_build {
                "not-assessed-debug-build"
            } else if observed < REVIEWED_PRIMARY_TARGET_NS {
                "passed-on-this-run"
            } else {
                "failed-on-this-run"
            },
            enforcement: "reported evidence only; this command does not fail CI on host timing",
        }
    })
}

fn nanos(duration: Duration) -> u64 {
    duration.as_nanos().min(u64::MAX as u128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_and_named_snapshot_sources_without_generated_sections() {
        let single = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n# SMG\n~~~sexpr\n(old)\n~~~\n";
        let parsed = parse_source_documents(single, "single.md").unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "single.md");
        assert_eq!(parsed[0].text, "package A {}");

        let named = "# SOURCE\n## a.sysml\n~~~sysml\npackage A {}\n~~~\n## b.sysml\n~~~sysml\npackage B {}\n~~~\n# SMG\n~~~sexpr\n(old)\n~~~\n";
        let parsed = parse_source_documents(named, "multi.md").unwrap();
        assert_eq!(
            parsed
                .iter()
                .map(|document| document.name.as_str())
                .collect::<Vec<_>>(),
            ["a.sysml", "b.sysml"]
        );
    }

    #[test]
    fn summaries_retain_raw_sample_bounds_without_thresholds() {
        let summary = summarize([9, 2, 5].into_iter());
        assert_eq!(summary.samples, 3);
        assert_eq!(summary.min, Some(2));
        assert_eq!(summary.median, Some(5));
        assert_eq!(summary.max, Some(9));
    }

    #[test]
    fn reviewed_target_is_report_only_and_primary_workload_scoped() {
        let passing = summarize([REVIEWED_PRIMARY_TARGET_NS - 1].into_iter());
        let failing = summarize([REVIEWED_PRIMARY_TARGET_NS].into_iter());
        assert!(assess_reviewed_target(ModelSize::Large, &failing, true).is_none());
        assert_eq!(
            assess_reviewed_target(ModelSize::StandardLibrary, &passing, true)
                .unwrap()
                .status,
            "passed-on-this-run"
        );
        assert_eq!(
            assess_reviewed_target(ModelSize::ManyFiles, &failing, true)
                .unwrap()
                .status,
            "failed-on-this-run"
        );
        assert_eq!(
            assess_reviewed_target(ModelSize::ManyFiles, &passing, false)
                .unwrap()
                .status,
            "not-assessed-debug-build"
        );
    }
}
