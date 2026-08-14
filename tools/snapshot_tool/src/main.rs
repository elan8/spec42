//! Standalone source-to-snapshot harness for Spec42.
//!
//! Snapshot Markdown files are the test cases. The runner reads each file's SOURCE section,
//! builds the immutable semantic model, renders each owned derived section, and either reports
//! stale files (`check`) or rewrites them (`update`). It is intentionally a binary rather than a
//! Rust test: review happens through the normal `git diff` of the Markdown files.
//!
//! A fixture may admit the standard library by declaring `libraries=standard` in its META block.
//! The library sources are then admitted as `StandardLibrary`-role documents, so the fixture's
//! references resolve against them while the owned projections keep reporting only the fixture's
//! own authored documents.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use clap::{Parser, Subcommand};
use rayon::prelude::*;
use sysml_query::resolved_slice::{
    build as build_published_model, BuildRequest, ConstructionStrategy, EditorProbe,
    PublishedModel, SourceDocument as QuerySourceDocument, SourceKind, TextPosition,
};

#[derive(Debug, Parser)]
#[command(
    name = "spec42-snapshot",
    about = "Regenerate Spec42 Markdown source snapshots"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Root directory containing Markdown snapshots.
    #[arg(long, default_value = "test/snapshots", global = true)]
    root: PathBuf,
    /// Restrict the operation to one path relative to --root (or an explicit path).
    #[arg(long, global = true)]
    fixture: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compute derived sections and fail if any snapshot would change.
    Check,
    /// Rewrite all owned derived sections in place. Review with `git diff`.
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceDocument {
    name: String,
    text: String,
}

/// Which libraries a fixture admits alongside its authored `SOURCE` documents.
///
/// A closed set with no default beyond `None`: an unrecognised `libraries` value is an error, so a
/// typo cannot silently produce a workspace-only publication that looks like a library-admitting
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LibrarySelection {
    None,
    Standard,
}

/// The directory of the checked-in standard-library corpus, relative to the snapshot root.
///
/// The runner cannot reach the packaged KPAR standard library: `workspace` depends on
/// `sysml_model`, which is outside this binary's enforced dependency closure. The library fixtures
/// already carry the same pinned library text in their own `SOURCE` sections, so they are the
/// admission input as well as fixtures in their own right.
const STANDARD_LIBRARY_DIRECTORY: &str = "sysml.library";

/// Lazily loaded library sources, shared by every fixture that admits them.
struct LibraryCorpus {
    root: PathBuf,
    standard: OnceLock<Result<Vec<QuerySourceDocument>, String>>,
}

impl LibraryCorpus {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            standard: OnceLock::new(),
        }
    }

    fn sources(&self, selection: LibrarySelection) -> Result<&[QuerySourceDocument], String> {
        match selection {
            LibrarySelection::None => Ok(&[]),
            LibrarySelection::Standard => self
                .standard
                .get_or_init(|| load_standard_library(&self.root))
                .as_deref()
                .map_err(|error| error.clone()),
        }
    }
}

fn load_standard_library(root: &Path) -> Result<Vec<QuerySourceDocument>, String> {
    let directory = root.join(STANDARD_LIBRARY_DIRECTORY);
    let mut paths = Vec::new();
    visit_markdown(&directory, &mut paths)?;
    paths.sort();
    if paths.is_empty() {
        return Err(format!(
            "no standard-library fixtures found under {}",
            directory.display()
        ));
    }
    let mut sources = Vec::with_capacity(paths.len());
    for path in paths {
        let fallback_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("library.md");
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let documents = parse_source_documents(&text, fallback_name)?;
        for document in documents {
            let name = format!("{STANDARD_LIBRARY_DIRECTORY}/{}", document.name);
            sources.push(
                QuerySourceDocument::from_memory_path(
                    "snapshot",
                    &name,
                    document.text,
                    SourceKind::StandardLibrary,
                )
                .map_err(|error| format!("{}: invalid library source: {error}", path.display()))?,
            );
        }
    }
    Ok(sources)
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let paths = snapshot_paths(&cli.root, cli.fixture.as_deref())?;
    if paths.is_empty() {
        return Err(format!(
            "no Markdown snapshots found under {}",
            cli.root.display()
        ));
    }
    let libraries = LibraryCorpus::new(cli.root.clone());

    // Rayon uses its bounded global worker pool; fixture work is isolated and writes happen only
    // after every worker has completed, in deterministic path order.
    let mut results: Vec<_> = paths
        .par_iter()
        .map(|path| FixtureWorkResult {
            path: path.clone(),
            result: evaluate_fixture(path, &libraries),
        })
        .collect();
    sort_work_results(&mut results);

    let mut failures = Vec::new();
    let mut stale = Vec::new();
    let mut writes = Vec::new();
    for result in results {
        match result.result {
            Ok(FixtureOutcome::Clean) => {}
            Ok(FixtureOutcome::StaleText(updated)) => match cli.command {
                Command::Check => stale.push(result.path),
                Command::Update => writes.push((result.path, updated.into_bytes())),
            },
            Err(error) => failures.push((result.path, error)),
        }
    }

    if !failures.is_empty() {
        eprintln!("snapshot processing errors:");
        for (path, error) in failures {
            eprintln!("  {}: {error}", path.display());
        }
        return Err("snapshot processing failed".to_string());
    }

    for (path, bytes) in writes {
        fs::write(&path, bytes)
            .map_err(|error| format!("{}: write failed: {error}", path.display()))?;
    }

    if stale.is_empty() {
        return Ok(());
    }
    eprintln!("stale snapshots (run `cargo run -p spec42-snapshot -- update`):");
    for path in stale {
        eprintln!("  {}", path.display());
    }
    Err("snapshot check failed".to_string())
}

enum FixtureOutcome {
    Clean,
    StaleText(String),
}

struct FixtureWorkResult {
    path: PathBuf,
    result: Result<FixtureOutcome, String>,
}

fn sort_work_results(results: &mut [FixtureWorkResult]) {
    results.sort_by(|left, right| left.path.cmp(&right.path));
}

fn evaluate_fixture(path: &Path, libraries: &LibraryCorpus) -> Result<FixtureOutcome, String> {
    let bytes = fs::read(path).map_err(|error| format!("read failed: {error}"))?;
    let original =
        String::from_utf8(bytes).map_err(|error| format!("snapshot is not UTF-8: {error}"))?;
    let updated = regenerate_snapshot(&original, path, libraries)?;
    Ok(if updated == original {
        FixtureOutcome::Clean
    } else {
        FixtureOutcome::StaleText(updated)
    })
}

fn snapshot_paths(root: &Path, fixture: Option<&Path>) -> Result<Vec<PathBuf>, String> {
    let root = if let Some(fixture) = fixture {
        if fixture.is_absolute() {
            fixture.to_path_buf()
        } else {
            let under_root = root.join(fixture);
            if under_root.exists() {
                under_root
            } else {
                fixture.to_path_buf()
            }
        }
    } else {
        root.to_path_buf()
    };
    if !root.exists() {
        return Err(format!("snapshot path does not exist: {}", root.display()));
    }
    if root.is_file() {
        return (root.extension().is_some_and(|extension| extension == "md"))
            .then_some(vec![root.clone()])
            .ok_or_else(|| format!("snapshot is not Markdown: {}", root.display()));
    }
    let mut paths = Vec::new();
    visit_markdown(&root, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn visit_markdown(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("{}: read directory failed: {error}", directory.display()))?
    {
        let path = entry
            .map_err(|error| format!("{}: directory entry failed: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            visit_markdown(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn regenerate_snapshot(
    fixture: &str,
    path: &Path,
    libraries: &LibraryCorpus,
) -> Result<String, String> {
    let fallback_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("snapshot.md");
    let documents = parse_source_documents(fixture, fallback_name)?;
    let selection = parse_library_selection(fixture, fallback_name)?;
    let mut source_documents = documents
        .iter()
        .map(|document| {
            QuerySourceDocument::from_memory_path(
                "snapshot",
                &document.name,
                document.text.clone(),
                SourceKind::Workspace,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("{}: invalid source: {error}", path.display()))?;
    source_documents.extend_from_slice(libraries.sources(selection)?);
    let probes = parse_editor_probes(fixture, &documents, fallback_name)?;
    let sequential = render_owned_sections(
        build_model(&source_documents, ConstructionStrategy::Sequential, path)?,
        &documents,
        &source_documents,
        &probes,
    )?;
    let parallel = render_owned_sections(
        build_model(&source_documents, ConstructionStrategy::Parallel, path)?,
        &documents,
        &source_documents,
        &probes,
    )?;
    ensure_strategy_parity(path, &sequential, &parallel)?;
    ensure_sections_balanced(&sequential)
        .map_err(|error| format!("{}: {error}", path.display()))?;

    let fixture = replace_or_insert_section(fixture, "SMG", &sequential.smg)
        .ok_or_else(|| format!("{}: missing SOURCE/SMG section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "DIAGNOSTICS", &sequential.diagnostics)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "TYPES", &sequential.types)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "NAVIGATION", &sequential.navigation)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = if probes.is_empty() {
        fixture
    } else {
        replace_or_insert_section(&fixture, "EDITOR RESULTS", &sequential.editor_queries)
            .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?
    };
    Ok(canonicalize_sections(&fixture))
}

struct OwnedSections {
    smg: String,
    types: String,
    diagnostics: String,
    navigation: String,
    editor_queries: String,
}

fn build_model(
    source_documents: &[QuerySourceDocument],
    construction: ConstructionStrategy,
    path: &Path,
) -> Result<PublishedModel, String> {
    let request = BuildRequest::resolved(source_documents.to_vec(), construction)
        .map_err(|error| format!("{}: invalid semantic input: {error}", path.display()))?;
    build_published_model(request)
        .map_err(|error| format!("{}: semantic build failed: {error}", path.display()))
}

fn render_owned_sections(
    model: PublishedModel,
    documents: &[SourceDocument],
    source_documents: &[QuerySourceDocument],
    probes: &[EditorProbe],
) -> Result<OwnedSections, String> {
    // Both strings are complete owner-defined projections. The SMG includes publication phase,
    // completeness, evaluation state, and all owned facts; diagnostics includes canonical order.
    let smg = render_semantic_model(&model)?;
    let diagnostics = render_diagnostics(&model, documents, source_documents)?;
    let mut types = String::new();
    model
        .debug()
        .write_types_sexpr(&mut types)
        .map_err(|error| format!("type rendering failed: {error}"))?;
    let mut navigation = String::new();
    model
        .debug()
        .write_navigation_sexpr(&mut navigation)
        .map_err(|error| format!("navigation rendering failed: {error}"))?;
    let mut editor_queries = String::new();
    model
        .debug()
        .write_editor_queries_sexpr(probes, &mut editor_queries)
        .map_err(|error| format!("editor-query rendering failed: {error}"))?;
    Ok(OwnedSections {
        smg,
        types,
        diagnostics,
        navigation,
        editor_queries,
    })
}

/// Rejects an owned section whose S-expression does not close.
///
/// These sections are a contract, not decoration: a reader that parses them has to be able to.
/// Three separate producers had drifted out of balance without any check noticing, because a
/// snapshot only ever had to match its own previous text. Parentheses inside quoted strings are
/// content, not structure, so the scan tracks quoting.
fn ensure_balanced(name: &str, text: &str) -> Result<(), String> {
    let mut depth = 0i64;
    let mut quoted = false;
    let mut escaped = false;
    for character in text.chars() {
        if quoted {
            match character {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => quoted = false,
                _ => {}
            }
            continue;
        }
        match character {
            '"' => quoted = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return Err(format!("{name} section closes more elements than it opens"));
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err(format!("{name} section leaves {depth} element(s) open"));
    }
    Ok(())
}

fn ensure_sections_balanced(sections: &OwnedSections) -> Result<(), String> {
    ensure_balanced("SMG", &sections.smg)?;
    ensure_balanced("TYPES", &sections.types)?;
    ensure_balanced("DIAGNOSTICS", &sections.diagnostics)?;
    ensure_balanced("NAVIGATION", &sections.navigation)?;
    ensure_balanced("EDITOR RESULTS", &sections.editor_queries)
}

fn ensure_strategy_parity(
    path: &Path,
    sequential: &OwnedSections,
    parallel: &OwnedSections,
) -> Result<(), String> {
    if sequential.smg != parallel.smg {
        return Err(format!(
            "{}: sequential and parallel semantic-model outputs differ",
            path.display()
        ));
    }
    if sequential.diagnostics != parallel.diagnostics {
        return Err(format!(
            "{}: sequential and parallel diagnostics outputs differ",
            path.display()
        ));
    }
    if sequential.types != parallel.types {
        return Err(format!(
            "{}: sequential and parallel type outputs differ",
            path.display()
        ));
    }
    if sequential.navigation != parallel.navigation {
        return Err(format!(
            "{}: sequential and parallel navigation outputs differ",
            path.display()
        ));
    }
    if sequential.editor_queries != parallel.editor_queries {
        return Err(format!(
            "{}: sequential and parallel editor-query outputs differ",
            path.display()
        ));
    }
    Ok(())
}

fn render_semantic_model(model: &PublishedModel) -> Result<String, String> {
    let mut output = String::new();
    model
        .debug()
        .write_semantic_sexpr(&mut output)
        .map_err(|error| format!("semantic-model rendering failed: {error}"))?;
    Ok(output)
}

fn render_diagnostics(
    model: &PublishedModel,
    _documents: &[SourceDocument],
    _source_documents: &[QuerySourceDocument],
) -> Result<String, String> {
    let mut rendered = String::new();
    model
        .debug()
        .write_diagnostics_sexpr(&mut rendered)
        .map_err(|error| format!("diagnostic rendering failed: {error}"))?;
    Ok(rendered)
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

/// Reads the fixture's `libraries` META key.
///
/// Absent means workspace-only, which is what every fixture authored before libraries could be
/// admitted means. A present-but-unrecognised value is rejected rather than treated as absent.
fn parse_library_selection(fixture: &str, fallback_name: &str) -> Result<LibrarySelection, String> {
    let Some(section) = raw_section(fixture, "META") else {
        return Ok(LibrarySelection::None);
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!("{fallback_name}: malformed META fence"));
    };
    let mut selection = LibrarySelection::None;
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() != "libraries" {
            continue;
        }
        selection = match value.trim() {
            "none" => LibrarySelection::None,
            "standard" => LibrarySelection::Standard,
            other => {
                return Err(format!(
                    "{fallback_name}: unknown META libraries value {other:?} (expected \"none\" or \"standard\")"
                ))
            }
        };
    }
    Ok(selection)
}

fn parse_editor_probes(
    fixture: &str,
    documents: &[SourceDocument],
    fallback_name: &str,
) -> Result<Vec<EditorProbe>, String> {
    let Some(section) = raw_section(fixture, "EDITOR QUERIES") else {
        return Ok(Vec::new());
    };
    let Some((text, _)) = fenced_block(section) else {
        return Err(format!("{fallback_name}: malformed EDITOR QUERIES fence"));
    };
    let mut probes = Vec::new();
    for (line_index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        if fields.next() != Some("probe") {
            return Err(format!(
                "{fallback_name}: EDITOR QUERIES line {} must start with `probe`",
                line_index + 1
            ));
        }
        let document = fields
            .next()
            .ok_or_else(|| format!("{fallback_name}: missing probe document"))?;
        if !documents.iter().any(|candidate| candidate.name == document) {
            return Err(format!(
                "{fallback_name}: unknown probe document {document:?}"
            ));
        }
        let line = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("{fallback_name}: invalid probe line"))?;
        let character = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| format!("{fallback_name}: invalid probe character"))?;
        let mut qualifier = None;
        let mut rename_to = None;
        for option in fields {
            if let Some(value) = option.strip_prefix("qualifier=") {
                qualifier = Some(value.to_string());
            } else if let Some(value) = option.strip_prefix("rename=") {
                rename_to = Some(value.to_string());
            } else {
                return Err(format!(
                    "{fallback_name}: unknown editor probe option {option:?}"
                ));
            }
        }
        probes.push(EditorProbe {
            document: format!("memory://snapshot/{document}"),
            position: TextPosition { line, character },
            qualifier,
            rename_to,
        });
    }
    Ok(probes)
}

fn raw_section<'a>(fixture: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("# {name}\n");
    let start = fixture.find(&marker)? + marker.len();
    let rest = &fixture[start..];
    let end = rest.find("\n# ").unwrap_or(rest.len());
    Some(&rest[..end])
}

fn replace_or_insert_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    if let Some(updated) = replace_section(fixture, name, replacement) {
        return Some(updated);
    }
    let insertion = fixture.find("\n# ").unwrap_or(fixture.len());
    let section = format!("\n# {name}\n~~~sexpr\n{replacement}\n~~~");
    let mut updated = String::with_capacity(fixture.len() + section.len());
    updated.push_str(&fixture[..insertion]);
    updated.push_str(&section);
    updated.push_str(&fixture[insertion..]);
    Some(updated)
}

/// Canonical top-level Markdown order. SOURCE is authored; the other sections are owned by this
/// runner. Canonicalization drops sections outside this ownership contract.
const SECTION_ORDER: &[&str] = &[
    "META",
    "SOURCE",
    "EDITOR QUERIES",
    "DIAGNOSTICS",
    "SMG",
    "TYPES",
    "NAVIGATION",
    "EDITOR RESULTS",
];

fn canonicalize_sections(fixture: &str) -> String {
    let mut sections = Vec::<(&str, &str, usize)>::new();
    let mut marker = None;
    for (offset, line) in fixture.split_inclusive('\n').scan(0usize, |offset, line| {
        let start = *offset;
        *offset += line.len();
        Some((start, line))
    }) {
        let name = line
            .strip_prefix("# ")
            .and_then(|line| line.strip_suffix('\n'));
        if let Some(name) = name {
            if let Some((previous_name, previous_start)) = marker.take() {
                sections.push((
                    previous_name,
                    &fixture[previous_start..offset],
                    previous_start,
                ));
            }
            marker = Some((name, offset));
        }
    }
    if let Some((previous_name, previous_start)) = marker {
        sections.push((previous_name, &fixture[previous_start..], previous_start));
    }
    if sections.len() < 2 {
        return fixture.to_string();
    }
    let prefix_end = sections[0].2;
    let prefix = &fixture[..prefix_end];
    sections.retain(|(name, _, _)| SECTION_ORDER.contains(name));
    sections.sort_by_key(|(name, _, original)| {
        (
            SECTION_ORDER
                .iter()
                .position(|candidate| candidate == name)
                .unwrap_or(SECTION_ORDER.len()),
            *original,
        )
    });
    let mut output = String::with_capacity(fixture.len());
    output.push_str(prefix);
    for (_, body, _) in sections {
        output.push_str(body.trim_end_matches('\n'));
        output.push('\n');
    }
    output
}

fn replace_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    let marker = format!("# {name}\n");
    let section_start = fixture.find(&marker)? + marker.len();
    let section_end = fixture[section_start..]
        .find("\n# ")
        .map_or(fixture.len(), |index| section_start + index);
    let section = &fixture[section_start..section_end];
    fenced_block(section)?;
    let mut updated = String::with_capacity(fixture.len() + replacement.len() + 14);
    updated.push_str(&fixture[..section_start]);
    updated.push_str("~~~sexpr\n");
    updated.push_str(replacement.trim_end_matches('\n'));
    updated.push_str("\n~~~");
    updated.push_str(&fixture[section_end..]);
    Some(updated)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_does_not_allow_a_strategy_override() {
        assert!(
            Cli::try_parse_from(["spec42-snapshot", "check", "--strategy", "parallel"]).is_err()
        );
    }

    #[test]
    fn work_results_are_sorted_for_deterministic_reporting() {
        let mut results = vec![
            FixtureWorkResult {
                path: PathBuf::from("z.md"),
                result: Err("z failure".to_string()),
            },
            FixtureWorkResult {
                path: PathBuf::from("a.md"),
                result: Err("a failure".to_string()),
            },
        ];
        sort_work_results(&mut results);
        assert_eq!(
            results
                .iter()
                .map(|result| result.path.as_path())
                .collect::<Vec<_>>(),
            vec![Path::new("a.md"), Path::new("z.md")]
        );
    }

    fn owned_sections(smg: &str) -> OwnedSections {
        OwnedSections {
            smg: smg.to_string(),
            types: "same".to_string(),
            diagnostics: "same".to_string(),
            navigation: "same".to_string(),
            editor_queries: "same".to_string(),
        }
    }

    #[test]
    fn parity_mismatch_is_reported_before_owned_output_is_selected() {
        let error = ensure_strategy_parity(
            Path::new("fixture.md"),
            &owned_sections("sequential"),
            &owned_sections("parallel"),
        )
        .expect_err("mismatched owned output must fail parity");
        assert!(error.contains("semantic-model outputs differ"));
    }

    /// Every owned section is compared, not only the first: the editor-query section carries the
    /// inspection output, which is the one most likely to depend on construction order.
    #[test]
    fn parity_covers_every_owned_section() {
        let mut parallel = owned_sections("same");
        parallel.editor_queries = "different".to_string();
        let error =
            ensure_strategy_parity(Path::new("fixture.md"), &owned_sections("same"), &parallel)
                .expect_err("a differing editor-query section must fail parity");
        assert!(error.contains("editor-query outputs differ"));
    }

    #[test]
    fn parses_single_and_multi_source_documents() {
        let single = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n";
        assert_eq!(
            parse_source_documents(single, "single.md").unwrap()[0].text,
            "package A {}"
        );
        let multi = "# SOURCE\n## A.sysml\n~~~sysml\npackage A {}\n~~~\n## B.sysml\n~~~sysml\npackage B {}\n~~~\n";
        let documents = parse_source_documents(multi, "multi.md").unwrap();
        assert_eq!(documents.len(), 2);
        assert_eq!(documents[1].name, "B.sysml");
    }

    #[test]
    fn replaces_existing_section_without_touching_neighbors() {
        let fixture = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n# SMG\n~~~sexpr\nold\n~~~\n# DIAGNOSTICS\n~~~sexpr\nkeep\n~~~\n";
        let updated = replace_section(fixture, "SMG", "new").unwrap();
        assert!(updated.contains("# SMG\n~~~sexpr\nnew\n~~~"));
        assert!(updated.contains("# DIAGNOSTICS\n~~~sexpr\nkeep\n~~~"));
    }

    #[test]
    fn inserting_owned_sections_is_idempotent() {
        let fixture = "# META\n~~~ini\ntype=file\n~~~\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n";
        let first = replace_or_insert_section(fixture, "SMG", "model").unwrap();
        let first = replace_or_insert_section(&first, "DIAGNOSTICS", "diagnostics").unwrap();
        let first = replace_or_insert_section(&first, "NAVIGATION", "navigation").unwrap();
        let second = replace_or_insert_section(&first, "SMG", "model").unwrap();
        let second = replace_or_insert_section(&second, "DIAGNOSTICS", "diagnostics").unwrap();
        let second = replace_or_insert_section(&second, "NAVIGATION", "navigation").unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn canonicalizes_shuffled_top_level_sections() {
        let fixture = "# SMG\nold\n# NAVIGATION\nnav\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\nmeta\n# DIAGNOSTICS\ndiag\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# DIAGNOSTICS\ndiag\n# SMG\nold\n# NAVIGATION\nnav\n"
        );
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn normalizes_out_of_contract_sections_and_is_idempotent() {
        let fixture = "# META\nmeta\n# SOURCE\nsource\n# EXTRA\nextra\n# DIAGNOSTICS\ndiag\n# NOTES\nnotes\n# FORMAT\nformat\n# SMG\nsmg\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\nsource\n# DIAGNOSTICS\ndiag\n# SMG\nsmg\n"
        );
        assert!(!canonical.contains("# EXTRA\n"));
        assert!(!canonical.contains("# NOTES\n"));
        assert!(!canonical.contains("# FORMAT\n"));
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }
}
