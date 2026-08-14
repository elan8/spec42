//! Reproducible immutable semantic-build benchmark over checked-in snapshot SOURCE sections.

use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use serde::Serialize;
use sysml_query::resolved_slice::{
    build_measured, BuildRequest, ConstructionStrategy, LibraryStratum, SourceDocument, SourceKind,
};

#[derive(Debug, Parser)]
#[command(name = "spec42-semantic-benchmark")]
struct Cli {
    /// Repository root containing test/snapshots.
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
    /// Include only snapshot paths containing this text.
    #[arg(long)]
    filter: Option<String>,
    /// Number of fresh publications to build.
    #[arg(long, default_value_t = 5)]
    iterations: usize,
    /// Construction schedule used by the publication owner.
    #[arg(long, value_enum, default_value_t = Schedule::Parallel)]
    schedule: Schedule,
    /// Emit pretty JSON to this path instead of stdout.
    #[arg(long)]
    output: Option<PathBuf>,
    /// Admit the checked-in standard-library corpus alongside the selected workspace documents.
    ///
    /// The selection then measures what a real editor build costs: a small workspace resolved
    /// against the whole library, rather than the workspace alone.
    #[arg(long, value_enum, default_value_t = Libraries::None)]
    libraries: Libraries,
    /// Reuse one settled library stratum across iterations instead of resolving it every time.
    ///
    /// This is what an editor session does: the library is parsed and solved once when the session
    /// opens, and each edit rebuilds only the workspace against it.
    #[arg(long, requires = "libraries")]
    reuse_library: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Libraries {
    None,
    Standard,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Schedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Serialize)]
struct Report {
    schema_version: u32,
    corpus: CorpusFacts,
    schedule: &'static str,
    samples: Vec<Sample>,
    summary: Summary,
}

#[derive(Debug, Serialize)]
struct CorpusFacts {
    snapshots: usize,
    documents: usize,
    source_bytes: usize,
    library_documents: usize,
    library_source_bytes: usize,
}

#[derive(Debug, Serialize)]
struct Sample {
    request_preparation_ns: u64,
    build_wall_time_ns: u64,
    parse_ns: u64,
    lowering_ns: u64,
    resolution_ns: u64,
    unaccounted_build_ns: u64,
}

#[derive(Debug, Serialize)]
struct Summary {
    build_wall_time_ns: Distribution,
    parse_ns: Distribution,
    lowering_ns: Distribution,
    resolution_ns: Distribution,
    median_phase_percent: PhasePercent,
}

#[derive(Debug, Serialize)]
struct Distribution {
    min: u64,
    median: u64,
    max: u64,
}

#[derive(Debug, Serialize)]
struct PhasePercent {
    parse: f64,
    lowering: f64,
    resolution: f64,
}

#[derive(Debug)]
struct CorpusDocument {
    identity: String,
    text: String,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    if cli.iterations == 0 {
        return Err("--iterations must be positive".into());
    }
    let root = cli.repo_root.join("test/snapshots");
    let (snapshot_count, documents) = load_corpus(&root, cli.filter.as_deref())?;
    if documents.is_empty() {
        return Err("snapshot selection contains no SOURCE documents".into());
    }
    let library_documents = match cli.libraries {
        Libraries::None => Vec::new(),
        Libraries::Standard => load_corpus(&root.join(STANDARD_LIBRARY_DIRECTORY), None)?.1,
    };
    let facts = CorpusFacts {
        snapshots: snapshot_count,
        documents: documents.len(),
        source_bytes: documents.iter().map(|document| document.text.len()).sum(),
        library_documents: library_documents.len(),
        library_source_bytes: library_documents
            .iter()
            .map(|document| document.text.len())
            .sum(),
    };
    let library_documents_for_request = if cli.reuse_library {
        Vec::new()
    } else {
        library_documents.iter().collect::<Vec<_>>()
    };
    let stratum = if cli.reuse_library {
        let sources = library_documents
            .iter()
            .map(|document| {
                SourceDocument::from_memory_path(
                    "semantic-benchmark",
                    &format!("{STANDARD_LIBRARY_DIRECTORY}/{}", document.identity),
                    document.text.clone(),
                    SourceKind::StandardLibrary,
                )
                .map_err(|error| format!("{}: {error}", document.identity))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Some(LibraryStratum::build(sources).map_err(|error| format!("library stratum: {error}"))?)
    } else {
        None
    };
    let mut samples = Vec::with_capacity(cli.iterations);
    for _ in 0..cli.iterations {
        let request_started = Instant::now();
        let mut sources = documents
            .iter()
            .map(|document| {
                SourceDocument::from_memory_path(
                    "semantic-benchmark",
                    &document.identity,
                    document.text.clone(),
                    SourceKind::Workspace,
                )
                .map_err(|error| format!("{}: {error}", document.identity))
            })
            .collect::<Result<Vec<_>, _>>()?;
        for document in &library_documents_for_request {
            sources.push(
                SourceDocument::from_memory_path(
                    "semantic-benchmark",
                    &format!("{STANDARD_LIBRARY_DIRECTORY}/{}", document.identity),
                    document.text.clone(),
                    SourceKind::StandardLibrary,
                )
                .map_err(|error| format!("{}: {error}", document.identity))?,
            );
        }
        let strategy = match cli.schedule {
            Schedule::Sequential => ConstructionStrategy::Sequential,
            Schedule::Parallel => ConstructionStrategy::Parallel,
        };
        let request = match &stratum {
            Some(stratum) => BuildRequest::resolved_with_library(sources, strategy, stratum),
            None => BuildRequest::resolved(sources, strategy),
        }
        .map_err(|error| format!("prepare build: {error}"))?;
        let request_preparation_ns = nanos(request_started.elapsed());
        let build_started = Instant::now();
        let (model, measured) =
            build_measured(request).map_err(|error| format!("semantic build: {error}"))?;
        let build_wall_time_ns = nanos(build_started.elapsed());
        black_box(model.publication().completeness());
        let measured_ns = nanos(measured.parse)
            .saturating_add(nanos(measured.lowering))
            .saturating_add(nanos(measured.resolution));
        samples.push(Sample {
            request_preparation_ns,
            build_wall_time_ns,
            parse_ns: nanos(measured.parse),
            lowering_ns: nanos(measured.lowering),
            resolution_ns: nanos(measured.resolution),
            unaccounted_build_ns: build_wall_time_ns.saturating_sub(measured_ns),
        });
    }
    let build = distribution(samples.iter().map(|sample| sample.build_wall_time_ns));
    let parse = distribution(samples.iter().map(|sample| sample.parse_ns));
    let lowering = distribution(samples.iter().map(|sample| sample.lowering_ns));
    let resolution = distribution(samples.iter().map(|sample| sample.resolution_ns));
    let denominator = build.median.max(1) as f64;
    let report = Report {
        schema_version: 1,
        corpus: facts,
        schedule: match cli.schedule {
            Schedule::Sequential => "sequential",
            Schedule::Parallel => "parallel",
        },
        summary: Summary {
            median_phase_percent: PhasePercent {
                parse: parse.median as f64 * 100.0 / denominator,
                lowering: lowering.median as f64 * 100.0 / denominator,
                resolution: resolution.median as f64 * 100.0 / denominator,
            },
            build_wall_time_ns: build,
            parse_ns: parse,
            lowering_ns: lowering,
            resolution_ns: resolution,
        },
        samples,
    };
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    if let Some(output) = cli.output {
        fs::write(&output, format!("{json}\n"))
            .map_err(|error| format!("{}: {error}", output.display()))?;
    } else {
        println!("{json}");
    }
    Ok(())
}

/// Where the checked-in standard-library corpus lives, relative to the snapshot root.
const STANDARD_LIBRARY_DIRECTORY: &str = "sysml.library";

fn load_corpus(root: &Path, filter: Option<&str>) -> Result<(usize, Vec<CorpusDocument>), String> {
    let mut paths = Vec::new();
    collect_markdown(root, &mut paths)?;
    paths.sort();
    let mut snapshot_count = 0;
    let mut documents = Vec::new();
    for path in paths {
        let relative = path.strip_prefix(root).unwrap_or(&path).to_string_lossy();
        if filter.is_some_and(|filter| !relative.contains(filter)) {
            continue;
        }
        let fixture =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let sources = parse_source_documents(&fixture, &relative)?;
        snapshot_count += 1;
        for (ordinal, source) in sources.into_iter().enumerate() {
            documents.push(CorpusDocument {
                identity: format!("{relative}/{ordinal:03}-{}", source.0),
                text: source.1,
            });
        }
    }
    Ok((snapshot_count, documents))
}

fn collect_markdown(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in
        fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?
    {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_markdown(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn parse_source_documents(fixture: &str, fallback: &str) -> Result<Vec<(String, String)>, String> {
    let source = raw_section(fixture, "SOURCE")
        .ok_or_else(|| format!("{fallback}: missing SOURCE section"))?;
    let mut named = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let (name, rest) = cursor
            .split_once('\n')
            .ok_or_else(|| format!("{fallback}: malformed named SOURCE document"))?;
        let (text, after) = fenced_block(rest)
            .ok_or_else(|| format!("{fallback}: malformed SOURCE fence for {name}"))?;
        named.push((name.trim().to_string(), text));
        cursor = after;
    }
    if !named.is_empty() {
        return Ok(named);
    }
    fenced_block(source)
        .map(|(text, _)| vec![(fallback.to_string(), text)])
        .ok_or_else(|| format!("{fallback}: malformed SOURCE fence"))
}

fn raw_section<'a>(fixture: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("# {name}\n");
    let rest = &fixture[fixture.find(&marker)? + marker.len()..];
    Some(&rest[..rest.find("\n# ").unwrap_or(rest.len())])
}

fn fenced_block(input: &str) -> Option<(String, &str)> {
    let after_open = &input[input.find("~~~")? + 3..];
    let (_, body) = after_open.split_once('\n')?;
    if let Some(after_close) = body.strip_prefix("~~~") {
        return Some((String::new(), after_close));
    }
    let end = body.find("\n~~~")?;
    Some((body[..end].to_string(), &body[end + 4..]))
}

fn distribution(values: impl Iterator<Item = u64>) -> Distribution {
    let mut values = values.collect::<Vec<_>>();
    values.sort_unstable();
    Distribution {
        min: values[0],
        median: values[values.len() / 2],
        max: *values.last().unwrap(),
    }
}

fn nanos(duration: Duration) -> u64 {
    duration.as_nanos().min(u64::MAX as u128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_parser_ignores_generated_sections_and_preserves_named_documents() {
        let fixture = "# SOURCE\n## a.sysml\n~~~sysml\npackage A {}\n~~~\n## b.sysml\n~~~sysml\npackage B {}\n~~~\n# SMG\n~~~sexpr\n(old)\n~~~\n";
        let sources = parse_source_documents(fixture, "case.md").unwrap();
        assert_eq!(sources[0], ("a.sysml".into(), "package A {}".into()));
        assert_eq!(sources[1], ("b.sysml".into(), "package B {}".into()));
    }
}
