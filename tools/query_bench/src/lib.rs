//! Fixture and case definitions shared by the divan bench target and the allocation counter.
//!
//! Everything here goes through `sysml_query`: `Services`, the source service's admission, the
//! publication service, and the typed queries of a `PublishedModel`. No authority crate is named,
//! and nothing recomputes a semantic fact for itself.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use sysml_query::resolved_slice::{PublishedModel, QueryOutcome, TextPosition};
use sysml_query::source::{SourceDocument, SourceKind};
use sysml_query::Services;

/// Where the checked-in standard-library corpus lives, relative to the snapshot root.
///
/// This is the same corpus `tools/semantic_benchmark` measures, so the cold-build numbers here and
/// there describe one workload.
pub const STANDARD_LIBRARY_DIRECTORY: &str = "sysml.library";

/// The scope every in-memory document is admitted under.
const SCOPE: &str = "query-bench";

/// The path the user document is admitted under.
pub const WORKSPACE_DOCUMENT: &str = "bench/workspace.sysml";

/// A small user document that resolves against the library: a completion point, a reference to a
/// library type, and a locally declared type with occurrences to find.
pub const WORKSPACE_SOURCE: &str = "\
package BenchWorkspace {
    private import ScalarValues::*;

    part def BenchVehicle {
        attribute mass : Real;
        attribute count : Integer;
    }

    part def BenchTrailer :> BenchVehicle {
        attribute load : Real;
    }

    part vehicle : BenchVehicle;
    part trailer : BenchTrailer;
}
";

/// One corpus document: the identity it is admitted under and its text.
#[derive(Debug, Clone)]
pub struct CorpusDocument {
    pub identity: String,
    pub text: String,
}

/// The corpus, loaded from disk once per process.
#[derive(Debug)]
pub struct Corpus {
    pub library: Vec<CorpusDocument>,
    pub library_bytes: usize,
}

impl Corpus {
    /// Loads the bundled standard-library corpus from the checked-in snapshots.
    pub fn load() -> Result<Self, String> {
        let root = repository_root().join("tests/snapshots");
        let library = load_documents(&root.join(STANDARD_LIBRARY_DIRECTORY))?;
        if library.is_empty() {
            return Err("the standard-library corpus is empty".into());
        }
        let library_bytes = library.iter().map(|document| document.text.len()).sum();
        Ok(Self {
            library,
            library_bytes,
        })
    }
}

/// A warm host: one `Services`, the library admitted, one publication already built.
///
/// Constructing this is what the cold-build case measures; the query cases measure only the
/// queries against `model`.
pub struct Fixture {
    pub services: Services,
    pub library: Vec<SourceDocument>,
    pub model: Arc<PublishedModel>,
    /// The largest library document, used as the outline case.
    pub outline_document: String,
    /// The workspace document's admitted URI, as every query names it.
    pub workspace_document: String,
    /// Published elements, summed from the outline query over every admitted document.
    pub element_count: usize,
}

impl Fixture {
    pub fn build(corpus: &Corpus) -> Result<Self, String> {
        let services = Services::new();
        let library = admit_library(&services, corpus)?;
        let model = publish(&services, &library, WORKSPACE_SOURCE)?;
        let outline_document = largest_admitted_uri(&library);
        let workspace_document = admit_workspace(&services, WORKSPACE_SOURCE)?
            .uri()
            .to_string();
        let element_count = published_element_count(&model, &library, &workspace_document);
        Ok(Self {
            services,
            library,
            model,
            outline_document,
            workspace_document,
            element_count,
        })
    }
}

/// Admits the whole library corpus under one source service.
pub fn admit_library(services: &Services, corpus: &Corpus) -> Result<Vec<SourceDocument>, String> {
    corpus
        .library
        .iter()
        .map(|document| {
            services
                .source
                .admit_memory(
                    SCOPE,
                    &format!("{STANDARD_LIBRARY_DIRECTORY}/{}", document.identity),
                    &document.text,
                    SourceKind::StandardLibrary,
                )
                .map_err(|error| format!("{}: {error}", document.identity))
        })
        .collect()
}

/// Publishes the library plus one workspace document.
pub fn publish(
    services: &Services,
    library: &[SourceDocument],
    workspace_source: &str,
) -> Result<Arc<PublishedModel>, String> {
    let mut documents = library.to_vec();
    documents.push(admit_workspace(services, workspace_source)?);
    services
        .publication
        .publish(&documents, [])
        .map_err(|error| format!("publish: {error:?}"))
}

fn admit_workspace(services: &Services, source: &str) -> Result<SourceDocument, String> {
    services
        .source
        .admit_memory(SCOPE, WORKSPACE_DOCUMENT, source, SourceKind::Workspace)
        .map_err(|error| format!("{WORKSPACE_DOCUMENT}: {error}"))
}

/// A full cold build: a brand-new `Services`, the library admitted from scratch, one publication.
pub fn cold_build(corpus: &Corpus) -> Result<Arc<PublishedModel>, String> {
    let services = Services::new();
    let library = admit_library(&services, corpus)?;
    publish(&services, &library, WORKSPACE_SOURCE)
}

/// One keystroke in the user document: republish against the already-settled library stratum.
pub fn warm_relink(fixture: &Fixture, revision: usize) -> Result<Arc<PublishedModel>, String> {
    let edited = format!("{WORKSPACE_SOURCE}// revision {revision}\n");
    publish(&fixture.services, &fixture.library, &edited)
}

/// A position inside the nth occurrence of `needle` in the workspace source.
pub fn workspace_position(needle: &str, occurrence: usize) -> TextPosition {
    position_of(WORKSPACE_SOURCE, needle, occurrence)
}

/// The completion point: on the `Real` type reference inside `BenchVehicle`'s body.
pub fn completion_position() -> TextPosition {
    workspace_position("attribute mass", 0)
}

/// The navigation point: the `BenchVehicle` reference in `part vehicle : BenchVehicle`.
pub fn navigation_position() -> TextPosition {
    workspace_position("BenchVehicle;", 0)
}

fn position_of(text: &str, needle: &str, occurrence: usize) -> TextPosition {
    let mut cursor = 0usize;
    let mut offset = None;
    for _ in 0..=occurrence {
        let found = text[cursor..]
            .find(needle)
            .unwrap_or_else(|| panic!("bench fixture is missing {needle:?}"));
        offset = Some(cursor + found);
        cursor += found + needle.len();
    }
    let offset = offset.expect("occurrence");
    let line = text[..offset].matches('\n').count() as u32;
    let line_start = text[..offset].rfind('\n').map_or(0, |index| index + 1);
    TextPosition::new(line, (offset - line_start) as u32 + 1)
}

fn largest_admitted_uri(library: &[SourceDocument]) -> String {
    library
        .iter()
        .max_by_key(|document| document.content().len())
        .expect("non-empty library corpus")
        .uri()
        .to_string()
}

/// How many published elements the model holds, as the facade reports them.
///
/// This is the denominator of every allocations-per-element number for the build cases.
pub fn published_element_count(
    model: &PublishedModel,
    library: &[SourceDocument],
    workspace_document: &str,
) -> usize {
    library
        .iter()
        .map(|document| document.uri().to_string())
        .chain(std::iter::once(workspace_document.to_owned()))
        .map(|uri| outcome_len(model.inspection().document_symbols(&uri)))
        .sum()
}

/// How many values an outcome carries; an unresolved outcome carries none.
pub fn outcome_len<T>(outcome: QueryOutcome<Box<[T]>>) -> usize {
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => values.len(),
        QueryOutcome::Ambiguous(alternatives) => {
            alternatives.iter().map(|values| values.len()).sum()
        }
        _ => 0,
    }
}

/// The repository root, found from this crate's manifest directory.
pub fn repository_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .expect("tools/<crate> lives two levels under the repository root")
        .to_path_buf()
}

fn load_documents(root: &Path) -> Result<Vec<CorpusDocument>, String> {
    let mut paths = Vec::new();
    collect_markdown(root, &mut paths)?;
    paths.sort();
    let mut documents = Vec::new();
    for path in paths {
        let relative = path.strip_prefix(root).unwrap_or(&path).to_string_lossy();
        let fixture =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        for (ordinal, (name, text)) in parse_source_documents(&fixture, &relative)?
            .into_iter()
            .enumerate()
        {
            documents.push(CorpusDocument {
                identity: format!("{relative}/{ordinal:03}-{name}"),
                text,
            });
        }
    }
    Ok(documents)
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
