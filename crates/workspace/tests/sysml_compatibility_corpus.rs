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
    semantic_rendered: usize,
    semantic_render_failures: Vec<String>,
    smg_exact_strict: usize,
    smg_exact_recovery: usize,
    smg_empty_recovery_skipped: usize,
    smg_empty_unsupported_skipped: usize,
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
        "OMG JSON interchange fixtures are intentionally outside this compatibility corpus"
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
        match catch_unwind(AssertUnwindSafe(|| {
            build_and_link_graph(&semantic_documents)
        })) {
            Ok(Ok((graph, _))) => {
                coverage.semantic_completed += 1;
                let rendering = graph.to_semantic_sexpr();
                assert!(
                    rendering.starts_with("(semantic-graph\n") && rendering.ends_with(')'),
                    "{relative}: semantic graph renderer emitted an invalid root"
                );
                let expected = section(&fixture, "SMG")
                    .unwrap_or_else(|| panic!("{relative}: compatibility fixture is missing SMG"));
                match empty_graph_skip(&rendering, &documents, parser_accepts_all) {
                    Some(skip) => {
                        assert_eq!(expected, skip, "{relative}: empty graph skip changed");
                        eprintln!("SKIP semantic graph {relative}: {skip}");
                        if parser_accepts_all {
                            coverage.smg_empty_unsupported_skipped += 1;
                        } else {
                            coverage.smg_empty_recovery_skipped += 1;
                        }
                    }
                    None => {
                        assert_eq!(
                            rendering, expected,
                            "{relative}: semantic graph golden changed; run `SPEC42_SEMANTIC_GRAPH_FIXTURE='{relative}' cargo test -p workspace --no-default-features --test sysml_compatibility_corpus print_semantic_graph_fixture -- --ignored --nocapture` to inspect the deterministic replacement"
                        );
                        if parser_accepts_all {
                            coverage.smg_exact_strict += 1;
                        } else {
                            coverage.smg_exact_recovery += 1;
                        }
                    }
                }
                coverage.semantic_rendered += 1;
            }
            Ok(Err(error)) => coverage
                .semantic_render_failures
                .push(format!("{relative}: graph construction failed: {error}")),
            Err(_) => coverage.semantic_panics.push(relative.clone()),
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
    assert!(
        coverage.semantic_render_failures.is_empty(),
        "semantic graph rendering failed for: {:?}",
        coverage.semantic_render_failures
    );
    assert_eq!(
        coverage.smg_exact_strict
            + coverage.smg_exact_recovery
            + coverage.smg_empty_recovery_skipped
            + coverage.smg_empty_unsupported_skipped
            + coverage.non_utf8_skipped,
        coverage.snapshots,
        "every fixture must have a Spec42 SMG golden or a concrete non-UTF-8 skip"
    );
    eprintln!(
        "SysML compatibility coverage: snapshots={}; non_utf8_skipped={}; source_documents={}; parser_accepted={}; parser_skipped={}; semantic_completed={}; semantic_rendered={}; smg_exact_strict={}; smg_exact_recovery={}; smg_empty_recovery_skipped={}; smg_empty_unsupported_skipped={}; formatter_idempotent={}; formatter_goldens_equal={}; formatter_golden_skipped={}",
        coverage.snapshots,
        coverage.non_utf8_skipped,
        coverage.source_documents,
        coverage.parser_accepted,
        coverage.parser_skipped,
        coverage.semantic_completed,
        coverage.semantic_rendered,
        coverage.smg_exact_strict,
        coverage.smg_exact_recovery,
        coverage.smg_empty_recovery_skipped,
        coverage.smg_empty_unsupported_skipped,
        coverage.formatter_idempotent,
        coverage.formatter_goldens_equal,
        coverage.formatter_golden_skipped,
    );
}

#[test]
#[ignore = "prints one full canonical semantic graph rendering for golden review"]
fn print_semantic_graph_fixture() {
    let requested = std::env::var("SPEC42_SEMANTIC_GRAPH_FIXTURE")
        .expect("set SPEC42_SEMANTIC_GRAPH_FIXTURE to a fixture path relative to the corpus root");
    let path = Path::new(FIXTURES).join(&requested);
    let fixture = String::from_utf8(fs::read(&path).expect("read snapshot fixture"))
        .expect("requested fixture must be UTF-8");
    let documents = source_documents(&fixture, &requested);
    let semantic_documents = documents
        .iter()
        .map(|document| {
            SysmlDocument::from_memory_path(
                "compatibility-snapshot",
                &format!("{requested}/{}", document.name),
                document.text.clone(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("memory URI")
        })
        .collect::<Vec<_>>();
    let (graph, _) = build_and_link_graph(&semantic_documents).expect("semantic graph");
    println!("{}", graph.to_semantic_sexpr());
}

#[test]
#[ignore = "rewrites checked-in semantic graph sections after deliberate review"]
fn regenerate_semantic_graph_sections() {
    for path in snapshot_paths(Path::new(FIXTURES)) {
        let relative = path
            .strip_prefix(FIXTURES)
            .expect("fixture path")
            .display()
            .to_string();
        let Ok(fixture) = String::from_utf8(fs::read(&path).expect("read snapshot fixture")) else {
            continue;
        };
        let documents = source_documents(&fixture, &relative);
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
        let (graph, _) = build_and_link_graph(&semantic_documents).expect("semantic graph");
        let expected = empty_graph_skip(
            &graph.to_semantic_sexpr(),
            &documents,
            documents
                .iter()
                .all(|document| sysml_v2_parser::parse(&document.text).is_ok()),
        )
        .unwrap_or_else(|| graph.to_semantic_sexpr());
        let updated = replace_section(&fixture, "SMG", &expected)
            .unwrap_or_else(|| panic!("{relative}: fixture is missing an SMG section"));
        fs::write(path, updated).expect("write fixture");
    }
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
    let body_end = body_start + body.find("\n~~~")?;
    let mut updated = String::with_capacity(fixture.len() + replacement.len());
    updated.push_str(&fixture[..body_start]);
    updated.push_str(replacement);
    updated.push_str(&fixture[body_end..]);
    Some(updated)
}

fn empty_graph_skip(
    rendering: &str,
    documents: &[SourceDocument],
    parser_accepts_all: bool,
) -> Option<String> {
    if rendering != empty_semantic_graph()
        || documents
            .iter()
            .all(|document| document.text.trim().is_empty())
    {
        return None;
    }
    let reason = if parser_accepts_all {
        "strictly parsed non-empty source produced no typed semantic graph facts"
    } else {
        "parser recovery for non-empty source produced no typed semantic graph facts"
    };
    let code = if parser_accepts_all {
        "SMG-EMPTY-STRICT"
    } else {
        "SMG-EMPTY-RECOVERY"
    };
    Some(format!(
        "(semantic-graph\n  (status (skip (code {code:?}) (reason {reason:?})))\n  (containment\n  )\n  (relationships\n  )\n  (pending-relationships\n  )\n  (pending-expression-relationships\n  )\n)"
    ))
}

fn empty_semantic_graph() -> &'static str {
    "(semantic-graph\n  (containment\n  )\n  (relationships\n  )\n  (pending-relationships\n  )\n  (pending-expression-relationships\n  )\n)"
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

#[test]
fn camera_fixture_has_an_explicit_strict_empty_semantic_graph_skip() {
    let relative = "kerml/camera.md";
    let fixture = fs::read_to_string(Path::new(FIXTURES).join(relative)).expect("camera fixture");
    let documents = source_documents(&fixture, relative);
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
    let (graph, _) = build_and_link_graph(&semantic_documents).expect("semantic graph");
    assert_eq!(
        empty_graph_skip(&graph.to_semantic_sexpr(), &documents, true),
        Some("(semantic-graph\n  (status (skip (code \"SMG-EMPTY-STRICT\") (reason \"strictly parsed non-empty source produced no typed semantic graph facts\")))\n  (containment\n  )\n  (relationships\n  )\n  (pending-relationships\n  )\n  (pending-expression-relationships\n  )\n)".to_string())
    );
    assert_eq!(
        section(&fixture, "SMG").expect("camera SMG section"),
        "(semantic-graph\n  (status (skip (code \"SMG-EMPTY-STRICT\") (reason \"strictly parsed non-empty source produced no typed semantic graph facts\")))\n  (containment\n  )\n  (relationships\n  )\n  (pending-relationships\n  )\n  (pending-expression-relationships\n  )\n)"
    );
}
