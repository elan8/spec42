use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use elkrs_parity::{
    canonical_json_digest, compare_geometry, extract_geometry, median, validate_layout_output,
    Comparison, ComparisonStatus, GeometryCounts,
};
use serde::Serialize;
use serde_json::Value;

const DEFAULT_FIXTURES: &[&str] = &[
    "vscode/diagram-renderer/test-fixtures/interconnection/scene-two-part-chain-elk-input.json",
    "vscode/diagram-renderer/test-fixtures/interconnection/nested-ring-minimal-elk-input.json",
    "vscode/diagram-renderer/test-fixtures/interconnection/grid-system-context-elk-input.json",
    "tools/elkrs_parity/fixtures/general-flat.json",
    "tools/elkrs_parity/fixtures/general-hierarchical.json",
    "tools/elkrs_parity/fixtures/action-flow.json",
    "tools/elkrs_parity/fixtures/action-flow-down.json",
    "tools/elkrs_parity/fixtures/state-transition.json",
    "tools/elkrs_parity/fixtures/state-transition-right.json",
    "tools/elkrs_parity/fixtures/ports-labels-cross-hierarchy.json",
    "tools/elkrs_parity/fixtures/wide-siblings.json",
    "tools/elkrs_parity/fixtures/corpus/action_flow_complete.json",
    "tools/elkrs_parity/fixtures/corpus/action_flow_unresolved.json",
    "tools/elkrs_parity/fixtures/corpus/interconnection_complete.json",
    "tools/elkrs_parity/fixtures/corpus/interconnection_connector_incomplete.json",
    "tools/elkrs_parity/fixtures/corpus/interconnection_port_direction.json",
    "tools/elkrs_parity/fixtures/corpus/interconnection_unresolved.json",
    "tools/elkrs_parity/fixtures/corpus/state_transition_complete.json",
    "tools/elkrs_parity/fixtures/corpus/state_transition_unresolved.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-collapsed-root.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-dense-compartments.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-dense-relationships-collapsed.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-dense-relationships-expanded.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-expanded-root.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-kinds.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-nested-expansion.json",
    "tools/elkrs_parity/fixtures/corpus/synthetic-node-no-compartments.json",
    "tools/elkrs_parity/fixtures/corpus/timer_interconnection.json",
    "tools/elkrs_parity/fixtures/corpus/timer_state_transition.json",
    "tools/elkrs_parity/fixtures/corpus/webshop_action_flow.json",
];

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum ProcessMode {
    /// All samples run in this one process (current default; fast, used for CI).
    InProcess,
    /// Every sample re-execs this binary as a fresh child process, so the measured time includes
    /// process startup. Requires --release; a debug binary's startup cost swamps the signal.
    Cold,
    /// One child process runs every sample in a loop, amortizing startup across the batch. Also
    /// requires --release.
    Warm,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Engine {
    Elkjs,
    Elkrs,
}

#[derive(Debug, Parser)]
#[command(about = "Compare Spec42 ELK.js layout geometry with pinned elkrs")]
struct Args {
    /// Checked-in ELK JSON inputs. The Spec42 fixture set is used when omitted.
    #[arg(value_name = "INPUT")]
    inputs: Vec<PathBuf>,

    /// Number of timed calls to each engine per input.
    #[arg(long, default_value_t = 3, value_parser = clap::value_parser!(u32).range(1..))]
    iterations: u32,

    /// Numeric tolerance used to classify geometry as equal.
    #[arg(long, default_value_t = 1e-9)]
    tolerance: f64,

    /// Exit unsuccessfully when any geometry difference exceeds the tolerance.
    #[arg(long)]
    fail_on_difference: bool,

    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Write the report to a file instead of stdout.
    #[arg(long)]
    output: Option<PathBuf>,

    /// How to run --iterations samples: in one process (default), or out-of-process to capture
    /// startup cost (cold) or amortized steady-state cost (warm). Cold/warm skip the geometry
    /// comparison above (that is the in-process default's job) and report only timing and peak
    /// resident memory, against a single fixture.
    #[arg(long, value_enum, default_value_t = ProcessMode::InProcess)]
    process_mode: ProcessMode,

    /// Fixture to measure in --process-mode cold|warm. Defaults to the largest checked-in corpus
    /// fixture so the measurement reflects a realistic, not trivial, graph.
    #[arg(long)]
    process_mode_fixture: Option<PathBuf>,

    /// Internal: run as a --process-mode child. Not for direct use.
    #[arg(long, hide = true, value_enum)]
    internal_child_engine: Option<Engine>,
}

/// One child-process sample: layout wall time plus this process's peak resident set size at exit,
/// read from `/proc/self/status`'s `VmHWM` (Linux only; `None` elsewhere).
#[derive(Debug, Serialize, serde::Deserialize)]
struct ChildSample {
    median_layout_us: u128,
    min_layout_us: u128,
    peak_rss_kb: Option<u64>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProcessModeMeasurement {
    engine: &'static str,
    process_mode: &'static str,
    fixture: String,
    samples: u32,
    /// Only set for `cold`: total child wall time (startup + layout) minus the child's own
    /// reported layout time, median across samples.
    median_startup_us: Option<u128>,
    median_layout_us: u128,
    min_layout_us: u128,
    peak_rss_kb: Option<u64>,
    errors: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Report {
    schema_version: u32,
    elkrs_revision: &'static str,
    elk_compatibility_baseline: &'static str,
    tolerance: f64,
    fixtures: Vec<FixtureReport>,
    summary: Summary,
}

#[derive(Debug, Serialize)]
struct FixtureReport {
    input: String,
    input_bytes: u64,
    input_counts: GeometryCounts,
    elkjs: EngineMeasurement,
    elkrs: EngineMeasurement,
    comparison: Comparison,
}

#[derive(Debug, Default, Serialize)]
struct Summary {
    fixtures: usize,
    exact: usize,
    within_tolerance: usize,
    different: usize,
    engine_errors: usize,
}

#[derive(Debug, Serialize)]
struct EngineMeasurement {
    first_layout_us: u128,
    median_layout_us: u128,
    min_layout_us: u128,
    output_bytes: usize,
    output_digest: Option<String>,
    deterministic: bool,
    contract_errors: Vec<String>,
    error: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(engine) = args.internal_child_engine {
        return run_internal_child(engine, &args.inputs, args.iterations);
    }

    if !args.tolerance.is_finite() || args.tolerance < 0.0 {
        bail!("--tolerance must be a finite non-negative number");
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    if args.process_mode != ProcessMode::InProcess {
        return run_process_mode_benchmark(&args, &root);
    }

    let inputs = if args.inputs.is_empty() {
        DEFAULT_FIXTURES
            .iter()
            .map(|path| root.join(path))
            .collect()
    } else {
        args.inputs.clone()
    };

    let mut fixtures = Vec::with_capacity(inputs.len());
    for path in inputs {
        fixtures.push(compare_fixture(
            &root,
            &path,
            args.iterations,
            args.tolerance,
        )?);
    }

    let summary = summarize(&fixtures);
    let has_failure =
        summary.engine_errors > 0 || (args.fail_on_difference && summary.different > 0);
    let report = Report {
        schema_version: 1,
        elkrs_revision: diagram_layout::ELKRS_REVISION,
        elk_compatibility_baseline: diagram_layout::ELK_COMPATIBILITY_BASELINE,
        tolerance: args.tolerance,
        fixtures,
        summary,
    };

    let rendered = match args.format {
        OutputFormat::Json => serde_json::to_string_pretty(&report)?,
        OutputFormat::Text => render_text(&report),
    };
    if let Some(output) = args.output {
        fs::write(&output, format!("{rendered}\n"))
            .with_context(|| format!("write report {}", output.display()))?;
    } else {
        println!("{rendered}");
    }

    if has_failure {
        std::process::exit(1);
    }
    Ok(())
}

/// Reads this process's peak resident set size (`VmHWM`, in KiB) from `/proc/self/status`. `None`
/// on non-Linux platforms, or if the file is unreadable/unparseable.
fn read_peak_rss_kb() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|line| line.starts_with("VmHWM:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

/// `--internal-child-engine` entry point: lays out `inputs[0]` `iterations` times with a single
/// engine in this process, then reports median/min layout time and this process's own peak RSS.
/// Run as a fresh child (`--process-mode cold`) or a long-lived one (`--process-mode warm`) by
/// `run_process_mode_benchmark`; never invoked directly.
fn run_internal_child(engine: Engine, inputs: &[PathBuf], iterations: u32) -> Result<()> {
    let Some(fixture) = inputs.first() else {
        bail!("--internal-child-engine requires exactly one fixture path");
    };
    let input = fs::read_to_string(fixture).with_context(|| format!("read {}", fixture.display()))?;

    let mut durations = Vec::with_capacity(iterations as usize);
    let mut error = None;
    for _ in 0..iterations {
        let started = Instant::now();
        let result = match engine {
            Engine::Elkjs => spec42::elk_layout::layout_elk_graph(&input).map(|_| ()),
            Engine::Elkrs => diagram_layout::layout_json(&input)
                .map(|_| ())
                .map_err(|error| error.to_string()),
        };
        match result {
            Ok(()) => durations.push(started.elapsed()),
            Err(message) => {
                error = Some(message);
                break;
            }
        }
    }

    let sample = ChildSample {
        median_layout_us: median(&durations).as_micros(),
        min_layout_us: durations.iter().min().copied().unwrap_or_default().as_micros(),
        peak_rss_kb: read_peak_rss_kb(),
        error,
    };
    println!("{}", serde_json::to_string(&sample)?);
    Ok(())
}

/// `--process-mode cold|warm` entry point: re-execs this binary as a child for each engine, so
/// the measured cost is representative of a real process rather than the parent's already-warmed
/// allocator/JIT/OS-cache state. Skips the geometry comparison the in-process default mode does;
/// this mode answers "how fast and how much memory", not "are the two engines' outputs the same".
fn run_process_mode_benchmark(args: &Args, root: &Path) -> Result<()> {
    if !cfg!(debug_assertions) {
        // release build: proceed.
    } else if std::env::var("ELKRS_PARITY_ALLOW_DEBUG").is_err() {
        bail!(
            "--process-mode {:?} measures process startup and memory cost; a debug binary's own \
             overhead would swamp the signal. Rerun with `cargo run --release -p elkrs_parity`, \
             or set ELKRS_PARITY_ALLOW_DEBUG=1 to measure a debug build anyway (for iterating on \
             this tool itself, not for recording real numbers).",
            args.process_mode
        );
    }

    let fixture = args
        .process_mode_fixture
        .clone()
        .unwrap_or_else(|| root.join("tools/elkrs_parity/fixtures/corpus/timer_interconnection.json"));
    if !fixture.is_file() {
        bail!("--process-mode-fixture {} is not a file", fixture.display());
    }

    let self_exe = std::env::current_exe().context("locate this binary's own path to re-exec it")?;
    let mut measurements = Vec::new();
    for engine in [Engine::Elkjs, Engine::Elkrs] {
        let measurement = match args.process_mode {
            ProcessMode::InProcess => unreachable!("caller only invokes this for cold/warm"),
            ProcessMode::Cold => measure_cold(&self_exe, engine, &fixture, args.iterations)?,
            ProcessMode::Warm => measure_warm(&self_exe, engine, &fixture, args.iterations)?,
        };
        measurements.push(measurement);
    }

    let rendered = match args.format {
        OutputFormat::Json => serde_json::to_string_pretty(&measurements)?,
        OutputFormat::Text => measurements
            .iter()
            .map(render_process_mode_measurement)
            .collect::<Vec<_>>()
            .join("\n"),
    };
    if let Some(output) = &args.output {
        fs::write(output, format!("{rendered}\n"))
            .with_context(|| format!("write report {}", output.display()))?;
    } else {
        println!("{rendered}");
    }
    if measurements.iter().any(|m| !m.errors.is_empty()) {
        std::process::exit(1);
    }
    Ok(())
}

fn engine_flag(engine: Engine) -> &'static str {
    match engine {
        Engine::Elkjs => "elkjs",
        Engine::Elkrs => "elkrs",
    }
}

/// One fresh child process per sample: the parent's wall clock around each spawn is the "cold"
/// number (full process startup, paid every time), and each child also reports its own layout
/// time, so `median_startup_us` isolates the overhead specifically attributable to spawning a new
/// process rather than to the layout call itself.
fn measure_cold(
    self_exe: &Path,
    engine: Engine,
    fixture: &Path,
    iterations: u32,
) -> Result<ProcessModeMeasurement> {
    let mut total_durations = Vec::with_capacity(iterations as usize);
    let mut layout_durations = Vec::with_capacity(iterations as usize);
    let mut peak_rss_kb = None;
    let mut errors = Vec::new();

    for _ in 0..iterations {
        let started = Instant::now();
        let output = Command::new(self_exe)
            .arg("--internal-child-engine")
            .arg(engine_flag(engine))
            .arg("--iterations")
            .arg("1")
            .arg(fixture)
            .output()
            .with_context(|| format!("spawn cold child for {}", engine_flag(engine)))?;
        let total = started.elapsed();
        match parse_child_output(&output) {
            Ok(sample) => {
                total_durations.push(total);
                layout_durations.push(std::time::Duration::from_micros(
                    sample.median_layout_us as u64,
                ));
                peak_rss_kb = peak_rss_kb.max(sample.peak_rss_kb);
                if let Some(error) = sample.error {
                    errors.push(error);
                }
            }
            Err(error) => errors.push(error),
        }
    }

    let median_total = median(&total_durations).as_micros();
    let median_layout = median(&layout_durations).as_micros();
    Ok(ProcessModeMeasurement {
        engine: engine_flag(engine),
        process_mode: "cold",
        fixture: fixture.display().to_string(),
        samples: iterations,
        median_startup_us: Some(median_total.saturating_sub(median_layout)),
        median_layout_us: median_layout,
        min_layout_us: layout_durations
            .iter()
            .min()
            .copied()
            .unwrap_or_default()
            .as_micros(),
        peak_rss_kb,
        errors,
    })
}

/// One long-lived child process runs every sample internally: startup is paid once and amortized
/// across the whole batch, and the reported peak RSS covers the child's entire steady-state run.
fn measure_warm(
    self_exe: &Path,
    engine: Engine,
    fixture: &Path,
    iterations: u32,
) -> Result<ProcessModeMeasurement> {
    let output = Command::new(self_exe)
        .arg("--internal-child-engine")
        .arg(engine_flag(engine))
        .arg("--iterations")
        .arg(iterations.to_string())
        .arg(fixture)
        .output()
        .with_context(|| format!("spawn warm child for {}", engine_flag(engine)))?;
    let mut errors = Vec::new();
    let (median_layout_us, min_layout_us, peak_rss_kb) = match parse_child_output(&output) {
        Ok(sample) => {
            if let Some(error) = sample.error {
                errors.push(error);
            }
            (sample.median_layout_us, sample.min_layout_us, sample.peak_rss_kb)
        }
        Err(error) => {
            errors.push(error);
            (0, 0, None)
        }
    };
    Ok(ProcessModeMeasurement {
        engine: engine_flag(engine),
        process_mode: "warm",
        fixture: fixture.display().to_string(),
        samples: iterations,
        median_startup_us: None,
        median_layout_us,
        min_layout_us,
        peak_rss_kb,
        errors,
    })
}

fn parse_child_output(output: &std::process::Output) -> std::result::Result<ChildSample, String> {
    if !output.status.success() {
        return Err(format!(
            "child exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|error| {
        format!(
            "parse child output: {error}; stdout was {:?}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}

fn render_process_mode_measurement(measurement: &ProcessModeMeasurement) -> String {
    let startup = measurement
        .median_startup_us
        .map(|us| format!("{us}us"))
        .unwrap_or_else(|| "n/a (warm)".to_string());
    format!(
        "{} {} {} ({} samples): startup {startup}; layout median/min {}us/{}us; peak RSS {}",
        measurement.process_mode,
        measurement.engine,
        measurement.fixture,
        measurement.samples,
        measurement.median_layout_us,
        measurement.min_layout_us,
        measurement
            .peak_rss_kb
            .map(|kb| format!("{kb}KiB"))
            .unwrap_or_else(|| "n/a".to_string()),
    )
}

fn compare_fixture(
    root: &Path,
    path: &Path,
    iterations: u32,
    tolerance: f64,
) -> Result<FixtureReport> {
    let input = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let parsed: Value = serde_json::from_str(&input)
        .with_context(|| format!("parse ELK JSON input {}", path.display()))?;
    let input_geometry = extract_geometry(&parsed);

    let (mut elkjs_measurement, elkjs_output) = measure(iterations, || {
        spec42::elk_layout::layout_elk_graph(&input)
            .and_then(|json| serde_json::from_str(&json).map_err(|err| err.to_string()))
    });
    let (mut elkrs_measurement, elkrs_output) = measure(iterations, || {
        diagram_layout::layout_json(&input).map_err(|error| error.to_string())
    });

    if let Some(output) = &elkjs_output {
        elkjs_measurement.contract_errors = validate_layout_output(&parsed, output);
    }
    if let Some(output) = &elkrs_output {
        elkrs_measurement.contract_errors = validate_layout_output(&parsed, output);
    }

    let outputs_are_valid = elkjs_measurement.error.is_none()
        && elkrs_measurement.error.is_none()
        && elkjs_measurement.contract_errors.is_empty()
        && elkrs_measurement.contract_errors.is_empty();
    let comparison = match (outputs_are_valid, elkjs_output, elkrs_output) {
        (true, Some(elkjs), Some(elkrs)) => compare_geometry(
            &extract_geometry(&elkjs).values,
            &extract_geometry(&elkrs).values,
            tolerance,
        ),
        _ => Comparison {
            status: ComparisonStatus::EngineError,
            compared_scalars: 0,
            missing_from_elkjs: 0,
            missing_from_elkrs: 0,
            changed_scalars: 0,
            max_absolute_delta: 0.0,
            differences: Vec::new(),
        },
    };

    let display_path = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    Ok(FixtureReport {
        input: display_path,
        input_bytes: input.len() as u64,
        input_counts: input_geometry.counts,
        elkjs: elkjs_measurement,
        elkrs: elkrs_measurement,
        comparison,
    })
}

fn measure<F>(iterations: u32, mut layout: F) -> (EngineMeasurement, Option<Value>)
where
    F: FnMut() -> Result<Value, String>,
{
    let mut durations = Vec::with_capacity(iterations as usize);
    let mut last_output = None;
    let mut output_digest = None;
    for _ in 0..iterations {
        let started = Instant::now();
        match layout() {
            Ok(output) => {
                durations.push(started.elapsed());
                let digest = canonical_json_digest(&output);
                if output_digest
                    .as_ref()
                    .is_some_and(|expected| expected != &digest)
                {
                    return (
                        EngineMeasurement {
                            first_layout_us: durations[0].as_micros(),
                            median_layout_us: median(&durations).as_micros(),
                            min_layout_us: durations
                                .iter()
                                .min()
                                .copied()
                                .unwrap_or_default()
                                .as_micros(),
                            output_bytes: 0,
                            output_digest: None,
                            deterministic: false,
                            contract_errors: Vec::new(),
                            error: Some(format!(
                                "layout output changed across identical runs: expected {}, got {digest}",
                                output_digest.as_deref().unwrap_or("(missing)")
                            )),
                        },
                        None,
                    );
                }
                output_digest = Some(digest);
                last_output = Some(output);
            }
            Err(error) => {
                return (
                    EngineMeasurement {
                        first_layout_us: durations.first().copied().unwrap_or_default().as_micros(),
                        median_layout_us: median(&durations).as_micros(),
                        min_layout_us: durations
                            .iter()
                            .min()
                            .copied()
                            .unwrap_or_default()
                            .as_micros(),
                        output_bytes: 0,
                        output_digest: None,
                        deterministic: true,
                        contract_errors: Vec::new(),
                        error: Some(error),
                    },
                    None,
                );
            }
        }
    }
    let output_bytes = last_output
        .as_ref()
        .and_then(|value| serde_json::to_vec(value).ok())
        .map_or(0, |bytes| bytes.len());
    (
        EngineMeasurement {
            first_layout_us: durations[0].as_micros(),
            median_layout_us: median(&durations).as_micros(),
            min_layout_us: durations
                .iter()
                .min()
                .copied()
                .unwrap_or_default()
                .as_micros(),
            output_bytes,
            output_digest,
            deterministic: true,
            contract_errors: Vec::new(),
            error: None,
        },
        last_output,
    )
}

fn summarize(fixtures: &[FixtureReport]) -> Summary {
    let mut summary = Summary {
        fixtures: fixtures.len(),
        ..Summary::default()
    };
    for fixture in fixtures {
        match fixture.comparison.status {
            ComparisonStatus::Exact => summary.exact += 1,
            ComparisonStatus::WithinTolerance => summary.within_tolerance += 1,
            ComparisonStatus::Different => summary.different += 1,
            ComparisonStatus::EngineError => summary.engine_errors += 1,
        }
    }
    summary
}

fn render_text(report: &Report) -> String {
    let mut lines = vec![format!(
        "elkrs {} ({}) vs Spec42 ELK.js; tolerance {}",
        report.elkrs_revision, report.elk_compatibility_baseline, report.tolerance
    )];
    for fixture in &report.fixtures {
        lines.push(format!(
            "{}: {:?}; {} scalars; max delta {:.12}; ELK.js first/median {}us/{}us; elkrs first/median {}us/{}us",
            fixture.input,
            fixture.comparison.status,
            fixture.comparison.compared_scalars,
            fixture.comparison.max_absolute_delta,
            fixture.elkjs.first_layout_us,
            fixture.elkjs.median_layout_us,
            fixture.elkrs.first_layout_us,
            fixture.elkrs.median_layout_us,
        ));
        if let Some(error) = &fixture.elkjs.error {
            lines.push(format!("  ELK.js error: {error}"));
        }
        if let Some(error) = &fixture.elkrs.error {
            lines.push(format!("  elkrs error: {error}"));
        }
        for difference in fixture.comparison.differences.iter().take(20) {
            lines.push(format!(
                "  {}: ELK.js={:?} elkrs={:?} delta={:?}",
                difference.path, difference.elkjs, difference.elkrs, difference.absolute_delta
            ));
        }
        if fixture.comparison.differences.len() > 20 {
            lines.push(format!(
                "  ... {} additional differences (use --format json for all)",
                fixture.comparison.differences.len() - 20
            ));
        }
    }
    lines.push(format!(
        "summary: {} fixtures; {} exact; {} within tolerance; {} different; {} engine errors",
        report.summary.fixtures,
        report.summary.exact,
        report.summary.within_tolerance,
        report.summary.different,
        report.summary.engine_errors,
    ));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_layouts_must_be_byte_semantically_deterministic() {
        let mut run = 0;
        let (measurement, output) = measure(2, || {
            run += 1;
            Ok(serde_json::json!({ "id": "root", "width": run }))
        });
        assert!(!measurement.deterministic);
        assert!(measurement
            .error
            .unwrap()
            .contains("changed across identical runs"));
        assert!(output.is_none());
    }
}
