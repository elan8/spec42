//! Compatibility coverage for the imported SysML snapshot corpus.
//!
//! These fixtures intentionally retain their tokens, AST, semantic-graph, and
//! diagnostic sections as reference evidence. They are not Spec42 contracts
//! because the internal representations differ. This runner instead checks the
//! shared, portable properties listed in the companion document.

use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use language_service::{format_document_text, FormatOptions};
use sysml_model::{build_and_link_graph, SysmlDocument, SysmlDocumentSourceKind};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/sysml_compatibility"
);
const IN_SCOPE_SNAPSHOT_COUNT: usize = 479;

#[derive(Default)]
struct Coverage {
    snapshots: usize,
    non_utf8_skipped: usize,
    source_documents: usize,
    parser_accepted: usize,
    parser_skipped: usize,
    semantic_completed: usize,
    semantic_panics: Vec<String>,
    formatter_idempotent: usize,
    formatter_goldens_equal: usize,
    formatter_golden_skipped: usize,
}

#[derive(Debug)]
struct SourceDocument {
    name: String,
    text: String,
}

#[test]
fn sysml_snapshot_corpus_is_accounted_for_without_interchange() {
    let snapshots = snapshot_paths(Path::new(FIXTURES));
    assert_eq!(
        snapshots.len(),
        IN_SCOPE_SNAPSHOT_COUNT,
        "update the declared corpus count when importing fixtures"
    );
    assert!(
        !Path::new(FIXTURES).join("interchange").exists(),
        "OMG JSON interchange is intentionally outside this migration"
    );

    let mut coverage = Coverage::default();
    for path in snapshots {
        let relative = path
            .strip_prefix(FIXTURES)
            .expect("fixture path")
            .display()
            .to_string();
        let fixture = match String::from_utf8(fs::read(&path).expect("read snapshot fixture")) {
            Ok(fixture) => fixture,
            Err(_) => {
                coverage.non_utf8_skipped += 1;
                coverage.snapshots += 1;
                eprintln!("SKIP fixture {relative}: source Markdown is not UTF-8; parser accepts UTF-8 text only");
                continue;
            }
        };
        let documents = source_documents(&fixture, &relative);
        assert!(
            !documents.is_empty(),
            "{relative}: compatibility fixture is missing SOURCE"
        );
        coverage.snapshots += 1;
        coverage.source_documents += documents.len();

        let parser_accepts_all = documents
            .iter()
            .all(|document| sysml_v2_parser::parse(&document.text).is_ok());
        if parser_accepts_all {
            coverage.parser_accepted += 1;
        } else {
            coverage.parser_skipped += 1;
            eprintln!("SKIP parser acceptance {relative}: pinned parser rejects recovery or unsupported syntax in the compatibility fixture");
        }

        // The semantic pipeline uses editor recovery.  It is useful even when
        // strict parsing rejects a fuzz/recovery fixture; a panic is never an
        // acceptable compatibility result.
        let semantic_documents = documents
            .iter()
            .map(|document| {
                SysmlDocument::from_memory_path(
                    "compatibility-snapshot",
                    &format!("{relative}/{}", document.name),
                    document.text.clone(),
                    SysmlDocumentSourceKind::Workspace,
                    None,
                    None,
                )
                .expect("memory URI")
            })
            .collect::<Vec<_>>();
        if catch_unwind(AssertUnwindSafe(|| {
            build_and_link_graph(&semantic_documents)
        }))
        .is_ok()
        {
            coverage.semantic_completed += 1;
        } else {
            coverage.semantic_panics.push(relative.clone());
        }

        for document in &documents {
            let once = format_document_text(&document.text, options());
            let twice = format_document_text(&once, options());
            assert_eq!(
                once, twice,
                "{relative}/{}: formatter must be idempotent",
                document.name
            );
            coverage.formatter_idempotent += 1;
        }

        if let Some(expected) = section(&fixture, "FORMAT") {
            let formatted = format_document_text(&documents[0].text, options());
            if formatted == expected {
                coverage.formatter_goldens_equal += 1;
            } else {
                coverage.formatter_golden_skipped += 1;
                eprintln!("SKIP formatter golden {relative}: the reference syntax-aware layout is not yet a Spec42 contract");
            }
        }
    }

    assert!(
        coverage.semantic_panics.is_empty(),
        "semantic pipeline panicked for: {:?}",
        coverage.semantic_panics
    );
    eprintln!(
        "SysML compatibility coverage: snapshots={}; non_utf8_skipped={}; source_documents={}; parser_accepted={}; parser_skipped={}; semantic_completed={}; formatter_idempotent={}; formatter_goldens_equal={}; formatter_golden_skipped={}",
        coverage.snapshots,
        coverage.non_utf8_skipped,
        coverage.source_documents,
        coverage.parser_accepted,
        coverage.parser_skipped,
        coverage.semantic_completed,
        coverage.formatter_idempotent,
        coverage.formatter_goldens_equal,
        coverage.formatter_golden_skipped,
    );
}

fn options() -> FormatOptions {
    FormatOptions {
        tab_size: 4,
        insert_spaces: true,
    }
}

fn snapshot_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    visit_markdown(root, &mut paths);
    paths.sort();
    paths
}

fn visit_markdown(directory: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("read fixture directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            visit_markdown(&path, paths);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
}

fn source_documents(fixture: &str, fallback_name: &str) -> Vec<SourceDocument> {
    let source = raw_section(fixture, "SOURCE").unwrap_or_default();
    let mut named = Vec::new();
    let mut current_name = None;
    let mut cursor = source;
    while let Some(index) = cursor.find("## ") {
        cursor = &cursor[index + 3..];
        let Some((name, rest)) = cursor.split_once('\n') else {
            break;
        };
        let Some((text, after)) = fenced_block(rest) else {
            break;
        };
        named.push(SourceDocument {
            name: name.trim().to_string(),
            text,
        });
        cursor = after;
        current_name = Some(());
    }
    if current_name.is_some() {
        named
    } else {
        fenced_block(source)
            .map(|(text, _)| {
                vec![SourceDocument {
                    name: fallback_name.to_string(),
                    text,
                }]
            })
            .unwrap_or_default()
    }
}

fn section(fixture: &str, name: &str) -> Option<String> {
    fenced_block(raw_section(fixture, name)?).map(|(text, _)| text)
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

#[test]
fn source_parser_handles_single_and_multi_file_sections() {
    let single = "# SOURCE\n~~~sysml\npackage A {}\n~~~\n# EXPECTED\n~~~\nNIL\n~~~\n";
    assert_eq!(source_documents(single, "single.md").len(), 1);
    let multi = "# SOURCE\n## A.sysml\n~~~sysml\npackage A {}\n~~~\n## B.sysml\n~~~sysml\npackage B {}\n~~~\n";
    let documents = source_documents(multi, "multi.md");
    assert_eq!(documents.len(), 2);
    assert_eq!(documents[1].name, "B.sysml");
}
