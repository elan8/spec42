//! Standalone source-to-snapshot harness for Spec42.
//!
//! Snapshot Markdown files are the test cases. The runner reads each file's SOURCE section,
//! builds the immutable semantic model, renders each owned derived section, and either reports
//! stale files (`check`) or rewrites them (`update`). It is intentionally a binary rather than a
//! Rust test: review happens through the normal `git diff` of the Markdown files.

use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use language_service::{format_document_text, FormatOptions};
use rayon::prelude::*;
use sysml_diagnostics::{
    collect_document_diagnostics_from_model, write_diagnostics_sexpr, DiagnosticsOptions,
};
use sysml_model::{
    build_semantic_model, ConstructionStrategy, EvaluationPolicy, ImmutableSourceSnapshot,
    SemanticBuildRequest, SemanticConfiguration, SemanticModel, SysmlDocument,
    SysmlDocumentSourceKind,
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

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let paths = snapshot_paths(&cli.root, cli.fixture.as_deref())?;
    if paths.is_empty() {
        return Err(format!(
            "no Markdown snapshots found under {}",
            cli.root.display()
        ));
    }

    // Rayon uses its bounded global worker pool; fixture work is isolated and writes happen only
    // after every worker has completed, in deterministic path order.
    let mut results: Vec<_> = paths
        .par_iter()
        .map(|path| FixtureWorkResult {
            path: path.clone(),
            result: evaluate_fixture(path),
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

fn evaluate_fixture(path: &Path) -> Result<FixtureOutcome, String> {
    let bytes = fs::read(path).map_err(|error| format!("read failed: {error}"))?;
    let original =
        String::from_utf8(bytes).map_err(|error| format!("snapshot is not UTF-8: {error}"))?;
    let updated = regenerate_snapshot(&original, path)?;
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

fn regenerate_snapshot(fixture: &str, path: &Path) -> Result<String, String> {
    let fallback_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("snapshot.md");
    let documents = parse_source_documents(fixture, fallback_name)?;
    let source_documents = documents
        .iter()
        .map(|document| {
            SysmlDocument::from_memory_path(
                "snapshot",
                &format!("snapshot/{}", document.name),
                document.text.clone(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let sequential = render_owned_sections(
        build_model(&source_documents, ConstructionStrategy::Sequential, path)?,
        &documents,
        &source_documents,
    )?;
    let parallel = render_owned_sections(
        build_model(&source_documents, ConstructionStrategy::Parallel, path)?,
        &documents,
        &source_documents,
    )?;
    ensure_strategy_parity(path, &sequential, &parallel)?;

    let format = render_format(&documents);

    let fixture = replace_or_insert_section(fixture, "SMG", &sequential.smg)
        .ok_or_else(|| format!("{}: missing SOURCE/SMG section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "DIAGNOSTICS", &sequential.diagnostics)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "NAVIGATION", &sequential.navigation)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_full_section(&fixture, "FORMAT", &format)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    Ok(canonicalize_sections(&fixture))
}

struct OwnedSections {
    smg: String,
    diagnostics: String,
    navigation: String,
}

fn build_model(
    source_documents: &[SysmlDocument],
    construction: ConstructionStrategy,
    path: &Path,
) -> Result<SemanticModel, String> {
    let snapshot = ImmutableSourceSnapshot::new(source_documents.to_vec())
        .map_err(|error| format!("{}: invalid source snapshot: {error}", path.display()))?;
    build_semantic_model(SemanticBuildRequest {
        sources: snapshot,
        construction,
        evaluation: EvaluationPolicy::Evaluate,
        configuration: SemanticConfiguration::default(),
    })
    .map_err(|error| format!("{}: semantic build failed: {error}", path.display()))
}

fn render_owned_sections(
    model: SemanticModel,
    documents: &[SourceDocument],
    source_documents: &[SysmlDocument],
) -> Result<OwnedSections, String> {
    // Both strings are complete owner-defined projections. The SMG includes publication phase,
    // completeness, evaluation state, and all owned facts; diagnostics includes canonical order.
    let smg = render_semantic_model(&model)?;
    let diagnostics = render_diagnostics(&model, documents, source_documents)?;
    let mut navigation = String::new();
    model
        .write_navigation_debug_sexpr(&mut navigation)
        .map_err(|error| format!("navigation rendering failed: {error}"))?;
    Ok(OwnedSections {
        smg,
        diagnostics,
        navigation,
    })
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
    if sequential.navigation != parallel.navigation {
        return Err(format!(
            "{}: sequential and parallel navigation outputs differ",
            path.display()
        ));
    }
    Ok(())
}

fn render_semantic_model(model: &SemanticModel) -> Result<String, String> {
    let mut output = String::new();
    model
        .write_debug_sexpr(&mut output)
        .map_err(|error| format!("semantic-model rendering failed: {error}"))?;
    Ok(output)
}

fn render_diagnostics(
    model: &SemanticModel,
    documents: &[SourceDocument],
    source_documents: &[SysmlDocument],
) -> Result<String, String> {
    let mut rendered = String::from("(fixture-diagnostics\n");
    for (document, source_document) in documents.iter().zip(source_documents) {
        let diagnostics = collect_document_diagnostics_from_model(
            model,
            false,
            &source_document.uri,
            &document.text,
            false,
            DiagnosticsOptions::default(),
        );
        rendered.push_str(&format!("  (document {:?}\n", document.name));
        let mut rendered_diagnostics = String::new();
        write_diagnostics_sexpr(&diagnostics, &mut rendered_diagnostics)
            .map_err(|error| format!("diagnostic rendering failed: {error}"))?;
        for line in rendered_diagnostics.lines() {
            rendered.push_str("    ");
            rendered.push_str(line);
            rendered.push('\n');
        }
        rendered.push_str("  )\n");
    }
    rendered.push(')');
    Ok(rendered)
}

fn render_format(documents: &[SourceDocument]) -> String {
    let options = FormatOptions {
        tab_size: 4,
        insert_spaces: true,
    };
    if documents.len() == 1 {
        return format!(
            "~~~sysml\n{}\n~~~\n",
            format_document_text(&documents[0].text, options)
        );
    }
    documents
        .iter()
        .map(|document| {
            format!(
                "## {}\n~~~sysml\n{}\n~~~\n",
                document.name,
                format_document_text(&document.text, options)
            )
        })
        .collect()
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

fn replace_or_insert_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    if let Some(updated) = replace_section(fixture, name, replacement) {
        return Some(updated);
    }
    let insertion = fixture.find("\n# ").unwrap_or(fixture.len());
    let section = format!("\n# {name}\n~~~sexpr\n{replacement}\n~~~\n");
    let mut updated = String::with_capacity(fixture.len() + section.len());
    updated.push_str(&fixture[..insertion]);
    updated.push_str(&section);
    updated.push_str(&fixture[insertion..]);
    Some(updated)
}

fn replace_or_insert_full_section(fixture: &str, name: &str, replacement: &str) -> Option<String> {
    let marker = format!("# {name}\n");
    if let Some(start) = fixture.find(&marker) {
        let content_start = start + marker.len();
        let end = fixture[content_start..]
            .find("\n# ")
            .map_or(fixture.len(), |index| content_start + index);
        let mut updated = String::with_capacity(fixture.len() + replacement.len());
        updated.push_str(&fixture[..content_start]);
        // Keep the formatter's canonical trailing newline. Dropping it makes update/check
        // alternate forever for a snapshot whose FORMAT section is last in the file.
        updated.push_str(replacement);
        updated.push_str(&fixture[end..]);
        return Some(updated);
    }
    let section = format!("\n# {name}\n{}", replacement.trim_end_matches('\n'));
    let mut updated = String::with_capacity(fixture.len() + section.len());
    updated.push_str(fixture.trim_end());
    updated.push_str(&section);
    updated.push('\n');
    Some(updated)
}

/// Canonical top-level Markdown order. SOURCE is authored; the other sections are owned by this
/// runner. Canonicalization drops sections outside this ownership contract.
const SECTION_ORDER: &[&str] = &[
    "META",
    "SOURCE",
    "DIAGNOSTICS",
    "FORMAT",
    "SMG",
    "NAVIGATION",
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
    let opening = section.find("~~~")?;
    let after_opening = &section[opening + 3..];
    let (_, body) = after_opening.split_once('\n')?;
    let body_start = section_start + opening + 3 + (after_opening.len() - body.len());
    let body_end = if body.starts_with("~~~") {
        body_start
    } else {
        body_start + body.find("\n~~~")?
    };
    let mut updated = String::with_capacity(fixture.len() + replacement.len());
    updated.push_str(&fixture[..body_start]);
    updated.push_str(replacement.trim_end_matches('\n'));
    updated.push_str(&fixture[body_end..]);
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

    #[test]
    fn parity_mismatch_is_reported_before_owned_output_is_selected() {
        let sequential = OwnedSections {
            smg: "sequential".to_string(),
            diagnostics: "same".to_string(),
            navigation: "same".to_string(),
        };
        let parallel = OwnedSections {
            smg: "parallel".to_string(),
            diagnostics: "same".to_string(),
            navigation: "same".to_string(),
        };
        let error = ensure_strategy_parity(Path::new("fixture.md"), &sequential, &parallel)
            .expect_err("mismatched owned output must fail parity");
        assert!(error.contains("semantic-model outputs differ"));
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
        let first =
            replace_or_insert_full_section(&first, "FORMAT", "~~~sysml\npackage A {}\n~~~\n")
                .unwrap();
        let second = replace_or_insert_section(&first, "SMG", "model").unwrap();
        let second = replace_or_insert_section(&second, "DIAGNOSTICS", "diagnostics").unwrap();
        let second = replace_or_insert_section(&second, "NAVIGATION", "navigation").unwrap();
        let second =
            replace_or_insert_full_section(&second, "FORMAT", "~~~sysml\npackage A {}\n~~~\n")
                .unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn canonicalizes_shuffled_top_level_sections() {
        let fixture = "# SMG\nold\n# NAVIGATION\nnav\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\nmeta\n# DIAGNOSTICS\ndiag\n# FORMAT\nformat\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# DIAGNOSTICS\ndiag\n# FORMAT\nformat\n# SMG\nold\n# NAVIGATION\nnav\n"
        );
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }

    #[test]
    fn normalizes_out_of_contract_sections_and_is_idempotent() {
        let fixture = "# META\nmeta\n# SOURCE\nsource\n# EXTRA\nextra\n# DIAGNOSTICS\ndiag\n# NOTES\nnotes\n# FORMAT\nformat\n# SMG\nsmg\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\nsource\n# DIAGNOSTICS\ndiag\n# FORMAT\nformat\n# SMG\nsmg\n"
        );
        assert!(!canonical.contains("# EXTRA\n"));
        assert!(!canonical.contains("# NOTES\n"));
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }
}
