use std::fs;
use std::path::{Path, PathBuf};
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
    if !args.tolerance.is_finite() || args.tolerance < 0.0 {
        bail!("--tolerance must be a finite non-negative number");
    }

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
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
