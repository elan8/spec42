use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use serde::Serialize;
use serde_json::Value;

mod elkrs_adapter;

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
    error: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct GeometryCounts {
    nodes: usize,
    ports: usize,
    labels: usize,
    edge_sections: usize,
    bend_points: usize,
    scalars: usize,
}

#[derive(Debug, Serialize)]
struct Comparison {
    status: ComparisonStatus,
    compared_scalars: usize,
    missing_from_elkjs: usize,
    missing_from_elkrs: usize,
    changed_scalars: usize,
    max_absolute_delta: f64,
    differences: Vec<GeometryDifference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ComparisonStatus {
    Exact,
    WithinTolerance,
    Different,
    EngineError,
}

#[derive(Debug, Serialize)]
struct GeometryDifference {
    path: String,
    elkjs: Option<f64>,
    elkrs: Option<f64>,
    absolute_delta: Option<f64>,
}

#[derive(Debug, Default)]
struct Geometry {
    values: BTreeMap<String, f64>,
    counts: GeometryCounts,
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
        elkrs_revision: "8309be8cf614cfe277c572b28e4f79a1703f8e32",
        elk_compatibility_baseline: "ELK 0.11.0",
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

    let (elkjs_measurement, elkjs_output) = measure(iterations, || {
        spec42::elk_layout::layout_elk_graph(&input)
            .and_then(|json| serde_json::from_str(&json).map_err(|err| err.to_string()))
    });
    let (elkrs_measurement, elkrs_output) =
        measure(iterations, || elkrs_adapter::layout_json(&input));

    let comparison = match (elkjs_output, elkrs_output) {
        (Some(elkjs), Some(elkrs)) => compare_geometry(
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
    for _ in 0..iterations {
        let started = Instant::now();
        match layout() {
            Ok(output) => {
                durations.push(started.elapsed());
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
            error: None,
        },
        last_output,
    )
}

fn median(durations: &[Duration]) -> Duration {
    if durations.is_empty() {
        return Duration::ZERO;
    }
    let mut sorted = durations.to_vec();
    sorted.sort_unstable();
    sorted[sorted.len() / 2]
}

fn extract_geometry(root: &Value) -> Geometry {
    let mut geometry = Geometry::default();
    visit_graph(root, "graph", &mut geometry);
    geometry
}

fn visit_graph(value: &Value, path: &str, geometry: &mut Geometry) {
    record_rect(value, path, geometry);
    if path == "graph" || value.get("children").is_some() {
        geometry.counts.nodes += 1;
    }

    visit_named_array(
        value,
        "labels",
        path,
        "label",
        geometry,
        |item, item_path, geometry| {
            geometry.counts.labels += 1;
            record_rect(item, item_path, geometry);
        },
    );
    visit_named_array(
        value,
        "ports",
        path,
        "port",
        geometry,
        |port, port_path, geometry| {
            geometry.counts.ports += 1;
            record_rect(port, port_path, geometry);
            visit_named_array(
                port,
                "labels",
                port_path,
                "label",
                geometry,
                |item, item_path, geometry| {
                    geometry.counts.labels += 1;
                    record_rect(item, item_path, geometry);
                },
            );
        },
    );
    // ELK may publish an edge on the root or on its lowest-common-ancestor container. The edge id
    // is the stable identity consumed by Spec42, so compare the same edge directly even when the
    // engines disagree about its owning JSON object or coordinate frame.
    if let Some(edges) = value.get("edges").and_then(Value::as_array) {
        for (index, edge) in edges.iter().enumerate() {
            let id = edge.get("id").and_then(Value::as_str).unwrap_or("");
            let suffix = if id.is_empty() {
                format!("{path}:{index}")
            } else {
                escape_path(id)
            };
            visit_edge(edge, &format!("edge:{suffix}"), geometry);
        }
    }
    visit_named_array(value, "children", path, "node", geometry, visit_graph);
}

fn visit_edge(edge: &Value, path: &str, geometry: &mut Geometry) {
    visit_named_array(
        edge,
        "labels",
        path,
        "label",
        geometry,
        |item, item_path, geometry| {
            geometry.counts.labels += 1;
            record_rect(item, item_path, geometry);
        },
    );
    let Some(sections) = edge.get("sections").and_then(Value::as_array) else {
        return;
    };
    for (index, section) in sections.iter().enumerate() {
        geometry.counts.edge_sections += 1;
        let section_path = format!("{path}/section:{index}");
        record_point(
            section.get("startPoint"),
            &format!("{section_path}/start"),
            geometry,
        );
        record_point(
            section.get("endPoint"),
            &format!("{section_path}/end"),
            geometry,
        );
        if let Some(points) = section.get("bendPoints").and_then(Value::as_array) {
            for (point_index, point) in points.iter().enumerate() {
                geometry.counts.bend_points += 1;
                record_point(
                    Some(point),
                    &format!("{section_path}/bend:{point_index}"),
                    geometry,
                );
            }
        }
    }
}

fn visit_named_array<F>(
    value: &Value,
    key: &str,
    parent_path: &str,
    kind: &str,
    geometry: &mut Geometry,
    mut visit: F,
) where
    F: FnMut(&Value, &str, &mut Geometry),
{
    let Some(items) = value.get(key).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        let id = item.get("id").and_then(Value::as_str).unwrap_or("");
        let suffix = if id.is_empty() {
            index.to_string()
        } else {
            escape_path(id)
        };
        let item_path = format!("{parent_path}/{kind}:{suffix}");
        visit(item, &item_path, geometry);
    }
}

fn escape_path(value: &str) -> String {
    value.replace('%', "%25").replace('/', "%2F")
}

fn record_rect(value: &Value, path: &str, geometry: &mut Geometry) {
    for key in ["x", "y", "width", "height"] {
        record_number(value.get(key), &format!("{path}/{key}"), geometry);
    }
}

fn record_point(value: Option<&Value>, path: &str, geometry: &mut Geometry) {
    let Some(value) = value else { return };
    for key in ["x", "y"] {
        record_number(value.get(key), &format!("{path}/{key}"), geometry);
    }
}

fn record_number(value: Option<&Value>, path: &str, geometry: &mut Geometry) {
    if let Some(number) = value.and_then(Value::as_f64) {
        geometry.values.insert(path.to_string(), number);
        geometry.counts.scalars += 1;
    }
}

fn compare_geometry(
    elkjs: &BTreeMap<String, f64>,
    elkrs: &BTreeMap<String, f64>,
    tolerance: f64,
) -> Comparison {
    let paths: BTreeSet<_> = elkjs.keys().chain(elkrs.keys()).collect();
    let mut differences = Vec::new();
    let mut compared_scalars = 0;
    let mut missing_from_elkjs = 0;
    let mut missing_from_elkrs = 0;
    let mut changed_scalars = 0;
    let mut max_absolute_delta: f64 = 0.0;
    let mut has_nonzero_delta = false;

    for path in paths {
        let left = elkjs.get(path).copied();
        let right = elkrs.get(path).copied();
        match (left, right) {
            (Some(elkjs_value), Some(elkrs_value)) => {
                compared_scalars += 1;
                let delta = (elkjs_value - elkrs_value).abs();
                max_absolute_delta = max_absolute_delta.max(delta);
                has_nonzero_delta |= delta > 0.0;
                if delta > tolerance {
                    changed_scalars += 1;
                    differences.push(GeometryDifference {
                        path: path.clone(),
                        elkjs: left,
                        elkrs: right,
                        absolute_delta: Some(delta),
                    });
                }
            }
            (None, Some(_)) => {
                missing_from_elkjs += 1;
                differences.push(GeometryDifference {
                    path: path.clone(),
                    elkjs: None,
                    elkrs: right,
                    absolute_delta: None,
                });
            }
            (Some(_), None) => {
                missing_from_elkrs += 1;
                differences.push(GeometryDifference {
                    path: path.clone(),
                    elkjs: left,
                    elkrs: None,
                    absolute_delta: None,
                });
            }
            (None, None) => unreachable!(),
        }
    }

    let status = if missing_from_elkjs > 0 || missing_from_elkrs > 0 || changed_scalars > 0 {
        ComparisonStatus::Different
    } else if has_nonzero_delta {
        ComparisonStatus::WithinTolerance
    } else {
        ComparisonStatus::Exact
    };
    Comparison {
        status,
        compared_scalars,
        missing_from_elkjs,
        missing_from_elkrs,
        changed_scalars,
        max_absolute_delta,
        differences,
    }
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
    fn ignores_json_formatting_and_object_order() {
        let left: Value = serde_json::from_str(
            r#"{"id":"root","children":[{"id":"a","x":1,"y":2,"width":3,"height":4}]}"#,
        )
        .unwrap();
        let right: Value = serde_json::from_str(
            r#"{ "children": [ { "height": 4.0, "width": 3, "y": 2, "x": 1, "id": "a" } ], "id": "root" }"#,
        )
        .unwrap();
        let comparison = compare_geometry(
            &extract_geometry(&left).values,
            &extract_geometry(&right).values,
            0.0,
        );
        assert_eq!(comparison.status, ComparisonStatus::Exact);
    }

    #[test]
    fn reports_sorted_geometry_paths_and_missing_values() {
        let left: Value =
            serde_json::from_str(r#"{"id":"root","children":[{"id":"a","x":1,"y":2}]}"#).unwrap();
        let right: Value =
            serde_json::from_str(r#"{"id":"root","children":[{"id":"a","x":3,"width":4}]}"#)
                .unwrap();
        let comparison = compare_geometry(
            &extract_geometry(&left).values,
            &extract_geometry(&right).values,
            0.0,
        );
        let paths: Vec<_> = comparison
            .differences
            .iter()
            .map(|diff| diff.path.as_str())
            .collect();
        assert_eq!(
            paths,
            vec!["graph/node:a/width", "graph/node:a/x", "graph/node:a/y",]
        );
        assert_eq!(comparison.status, ComparisonStatus::Different);
        assert_eq!(comparison.changed_scalars, 1);
        assert_eq!(comparison.missing_from_elkjs, 1);
        assert_eq!(comparison.missing_from_elkrs, 1);
    }

    #[test]
    fn tolerance_classifies_small_numeric_drift() {
        let left = BTreeMap::from([("node/x".to_string(), 1.0)]);
        let right = BTreeMap::from([("node/x".to_string(), 1.0 + 1e-10)]);
        let comparison = compare_geometry(&left, &right, 1e-9);
        assert_eq!(comparison.status, ComparisonStatus::WithinTolerance);
        assert!(comparison.differences.is_empty());
    }
}
