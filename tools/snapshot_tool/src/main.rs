//! Standalone source-to-snapshot harness for Spec42.
//!
//! Snapshot Markdown files are the test cases. The runner reads each file's SOURCE section,
//! builds the immutable semantic model, renders each owned derived section, and either reports
//! stale files (`check`) or rewrites them (`update`). It is intentionally a binary rather than a
//! Rust test: review happens through the normal `git diff` of the Markdown files.

use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use language_service::{format_document_text, FormatOptions};
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
    /// Construction strategy used for the immutable semantic publication.
    #[arg(long, value_enum, default_value_t = Strategy::Sequential, global = true)]
    strategy: Strategy,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Strategy {
    Sequential,
    Parallel,
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

    let mut stale = Vec::new();
    for path in paths {
        let bytes =
            fs::read(&path).map_err(|error| format!("{}: read failed: {error}", path.display()))?;
        let original = match String::from_utf8(bytes) {
            Ok(original) => original,
            Err(_) => {
                eprintln!("SKIP {}: snapshot is not UTF-8", path.display());
                continue;
            }
        };
        let updated = regenerate_snapshot(&original, &path, cli.strategy)?;
        if updated != original {
            match cli.command {
                Command::Check => stale.push(path),
                Command::Update => fs::write(&path, updated)
                    .map_err(|error| format!("{}: write failed: {error}", path.display()))?,
            }
        }
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

fn regenerate_snapshot(fixture: &str, path: &Path, strategy: Strategy) -> Result<String, String> {
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
    let snapshot = ImmutableSourceSnapshot::new(source_documents.clone())
        .map_err(|error| format!("{}: invalid source snapshot: {error}", path.display()))?;
    let model = build_semantic_model(SemanticBuildRequest {
        sources: snapshot,
        construction: match strategy {
            Strategy::Sequential => ConstructionStrategy::Sequential,
            Strategy::Parallel => ConstructionStrategy::Parallel,
        },
        evaluation: EvaluationPolicy::Evaluate,
        configuration: SemanticConfiguration::default(),
    })
    .map_err(|error| format!("{}: semantic build failed: {error}", path.display()))?;

    let smg = render_semantic_model(&model)?;
    let format = render_format(&documents);
    // Diagnostics are intentionally owned by the semantic diagnostics stage. The concrete
    // adapter is kept behind this function so adding diagnostics does not expose model storage
    // to the harness or turn the Markdown fixture into a second semantic API.
    let diagnostics = render_diagnostics(&model, &documents, &source_documents)?;

    let fixture = replace_or_insert_section(fixture, "SMG", &smg)
        .ok_or_else(|| format!("{}: missing SOURCE/SMG section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "DIAGNOSTICS", &diagnostics)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    let fixture = replace_or_insert_full_section(&fixture, "FORMAT", &format)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    Ok(canonicalize_sections(&fixture))
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

/// Canonical top-level Markdown order. SOURCE is authored; all other sections are either owned
/// by this runner or preserved evidence. Reordering is part of update/check so a fixture cannot
/// silently acquire a second section layout over time.
const SECTION_ORDER: &[&str] = &[
    "META",
    "SOURCE",
    "DIAGNOSTICS",
    "TOKENS",
    "AST",
    "EXPECTED",
    "PROBLEMS",
    "FORMAT",
    "SMG",
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
        if name.is_some_and(|name| SECTION_ORDER.contains(&name)) {
            if let Some((previous_name, previous_start)) = marker.take() {
                sections.push((
                    previous_name,
                    &fixture[previous_start..offset],
                    previous_start,
                ));
            }
            marker = Some((name.expect("section name"), offset));
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
        let first =
            replace_or_insert_full_section(&first, "FORMAT", "~~~sysml\npackage A {}\n~~~\n")
                .unwrap();
        let second = replace_or_insert_section(&first, "SMG", "model").unwrap();
        let second = replace_or_insert_section(&second, "DIAGNOSTICS", "diagnostics").unwrap();
        let second =
            replace_or_insert_full_section(&second, "FORMAT", "~~~sysml\npackage A {}\n~~~\n")
                .unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn canonicalizes_shuffled_top_level_sections() {
        let fixture = "# SMG\nold\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# META\nmeta\n# DIAGNOSTICS\ndiag\n";
        let canonical = canonicalize_sections(fixture);
        assert_eq!(
            canonical,
            "# META\nmeta\n# SOURCE\n~~~sysml\npackage A {}\n~~~\n# DIAGNOSTICS\ndiag\n# SMG\nold\n"
        );
        assert_eq!(canonicalize_sections(&canonical), canonical);
    }
}
