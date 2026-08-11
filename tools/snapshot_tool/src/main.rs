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
        let original = fs::read_to_string(&path)
            .map_err(|error| format!("{}: read failed: {error}", path.display()))?;
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
    let documents = parse_source_documents(fixture, &path.display().to_string())?;
    let source_documents = documents
        .iter()
        .map(|document| {
            SysmlDocument::from_memory_path(
                "snapshot",
                &format!("{}/{}", path.display(), document.name),
                document.text.clone(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let snapshot = ImmutableSourceSnapshot::new(source_documents)
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
    let diagnostics = render_diagnostics(&model, &documents)?;

    let fixture = replace_or_insert_section(fixture, "SMG", &smg)
        .ok_or_else(|| format!("{}: missing SOURCE/SMG section", path.display()))?;
    let fixture = replace_or_insert_section(&fixture, "DIAGNOSTICS", &diagnostics)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))?;
    replace_or_insert_section(&fixture, "FORMAT", &format)
        .ok_or_else(|| format!("{}: missing SOURCE section", path.display()))
}

fn render_semantic_model(model: &SemanticModel) -> Result<String, String> {
    let mut output = String::new();
    model
        .write_debug_sexpr(&mut output)
        .map_err(|error| format!("semantic-model rendering failed: {error}"))?;
    Ok(output)
}

fn render_diagnostics(
    _model: &SemanticModel,
    _documents: &[SourceDocument],
) -> Result<String, String> {
    // The diagnostics owner is being migrated alongside the semantic publication. Keeping this
    // as an explicit adapter makes the dependency visible and prevents accidentally rebuilding a
    // legacy mutable graph here. The adapter is replaced by the diagnostics-owned writer before
    // this tool is enabled for the full corpus.
    Err(
        "semantic diagnostics writer is not available yet; refusing to rebuild the legacy graph"
            .to_string(),
    )
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
    updated.push_str(replacement);
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
}
