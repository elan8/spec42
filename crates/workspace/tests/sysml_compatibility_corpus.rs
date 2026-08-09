//! Compatibility coverage for the imported SysML snapshot corpus.
//!
//! Token, AST, and diagnostic sections remain fixture evidence. The SMG section
//! is the exact canonical semantic-graph projection asserted by this runner.

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
    smg_empty_strict_skipped: usize,
    formatter_idempotent: usize,
    formatter_exact_strict: usize,
    formatter_exact_recovery: usize,
    formatter_safety_strict_documents: usize,
    formatter_safety_recovery_documents: usize,
    formatter_non_utf8_skipped: usize,
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
                let bytes = fs::read(&path).expect("read snapshot fixture");
                let metadata = String::from_utf8_lossy(&bytes);
                let reason = formatter_skip_reason(&metadata).unwrap_or_else(|error| {
                    panic!("{relative}: malformed formatter skip: {error}")
                });
                assert!(
                    reason.is_some(),
                    "{relative}: non-UTF-8 fixture requires formatter=skip with formatter_skip_reason"
                );
                coverage.formatter_non_utf8_skipped += 1;
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

        let parser_errors = documents
            .iter()
            .filter_map(|document| {
                sysml_v2_parser::parse(&document.text)
                    .err()
                    .map(|error| format!("{}: {error:?}", document.name))
            })
            .collect::<Vec<_>>();
        let parser_accepts_all = parser_errors.is_empty();
        if parser_accepts_all {
            coverage.parser_accepted += 1;
        } else {
            coverage.parser_skipped += 1;
            eprintln!(
                "SKIP parser acceptance {relative}: strict parser rejected fixture: {}",
                parser_errors.join("; ")
            );
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
                let skip_reason = semantic_graph_skip_reason(&fixture).unwrap_or_else(|error| {
                    panic!("{relative}: malformed semantic graph skip: {error}")
                });
                let requires_skip = rendering == empty_semantic_graph()
                    && documents
                        .iter()
                        .any(|document| !document.text.trim().is_empty());
                match (skip_reason, requires_skip) {
                    (Some(reason), true) => {
                        assert_eq!(rendering, expected, "{relative}: semantic graph golden changed");
                        eprintln!("SKIP semantic graph {relative}: {reason}");
                        if parser_accepts_all {
                            coverage.smg_empty_strict_skipped += 1;
                        } else {
                            coverage.smg_empty_recovery_skipped += 1;
                        }
                    }
                    (Some(_), false) => panic!(
                        "{relative}: semantic_graph=skip is stale because the source now materializes graph facts"
                    ),
                    (None, true) => panic!(
                        "{relative}: non-empty source produced no typed semantic graph facts; META must declare semantic_graph=skip with semantic_graph_skip_reason"
                    ),
                    (None, false) => {
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

        let expected_format = section_documents(&fixture, "FORMAT", &relative)
            .unwrap_or_else(|| panic!("{relative}: compatibility fixture is missing FORMAT"));
        assert_eq!(
            documents
                .iter()
                .map(|document| &document.name)
                .collect::<Vec<_>>(),
            expected_format
                .iter()
                .map(|document| &document.name)
                .collect::<Vec<_>>(),
            "{relative}: FORMAT document names must exactly match SOURCE document names"
        );
        let formatter_skip = formatter_skip_reason(&fixture)
            .unwrap_or_else(|error| panic!("{relative}: malformed formatter skip: {error}"));
        let mut formatter_safety_failures = Vec::new();
        let mut formatter_golden_mismatches = Vec::new();
        for (document, expected) in documents.iter().zip(&expected_format) {
            let once = format_document_text(&document.text, options());
            let twice = format_document_text(&once, options());
            assert_eq!(
                once, twice,
                "{relative}/{}: formatter must be idempotent",
                document.name
            );
            coverage.formatter_idempotent += 1;
            if let Err(error) = formatter_safety_check(&document.text, &once) {
                formatter_safety_failures.push(format!("{}: {error}", document.name));
            } else if sysml_v2_parser::parse(&document.text).is_ok() {
                coverage.formatter_safety_strict_documents += 1;
            } else {
                coverage.formatter_safety_recovery_documents += 1;
            }
            if once != expected.text {
                formatter_golden_mismatches.push(document.name.as_str());
            }
        }
        match (formatter_skip, formatter_safety_failures.is_empty()) {
            (Some(_), true) => panic!(
                "{relative}: formatter=skip is stale because formatter output preserves parser semantics and recovery viability"
            ),
            (Some(reason), false) => eprintln!("SKIP formatter {relative}: {reason}"),
            (None, false) => panic!(
                "{relative}: formatter safety check failed; META must declare formatter=skip with formatter_skip_reason: {}",
                formatter_safety_failures.join("; ")
            ),
            (None, true) => {
                assert!(
                    formatter_golden_mismatches.is_empty(),
                    "{relative}: formatter golden changed for {}; run `SPEC42_FORMATTER_FIXTURE='{relative}' cargo test -p workspace --no-default-features --test sysml_compatibility_corpus print_formatter_fixture -- --ignored --nocapture` to inspect the deterministic replacement",
                    formatter_golden_mismatches.join(", ")
                );
                if parser_accepts_all {
                    coverage.formatter_exact_strict += 1;
                } else {
                    coverage.formatter_exact_recovery += 1;
                }
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
            + coverage.smg_empty_strict_skipped
            + coverage.non_utf8_skipped,
        coverage.snapshots,
        "every fixture must have a Spec42 SMG golden or a concrete non-UTF-8 skip"
    );
    assert_eq!(
        coverage.formatter_exact_strict
            + coverage.formatter_exact_recovery
            + coverage.formatter_non_utf8_skipped,
        coverage.snapshots,
        "every fixture must have an exact Spec42 formatter golden or a concrete non-UTF-8 skip"
    );
    eprintln!(
        "SysML compatibility coverage: snapshots={}; non_utf8_skipped={}; source_documents={}; parser_accepted={}; parser_skipped={}; semantic_completed={}; semantic_rendered={}; smg_exact_strict={}; smg_exact_recovery={}; smg_empty_recovery_skipped={}; smg_empty_strict_skipped={}; formatter_idempotent={}; formatter_exact_strict={}; formatter_exact_recovery={}; formatter_safety_strict_documents={}; formatter_safety_recovery_documents={}; formatter_non_utf8_skipped={}",
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
        coverage.smg_empty_strict_skipped,
        coverage.formatter_idempotent,
        coverage.formatter_exact_strict,
        coverage.formatter_exact_recovery,
        coverage.formatter_safety_strict_documents,
        coverage.formatter_safety_recovery_documents,
        coverage.formatter_non_utf8_skipped,
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
#[ignore = "prints one canonical formatter rendering for golden review"]
fn print_formatter_fixture() {
    let requested = std::env::var("SPEC42_FORMATTER_FIXTURE")
        .expect("set SPEC42_FORMATTER_FIXTURE to a fixture path relative to the corpus root");
    let path = Path::new(FIXTURES).join(&requested);
    let fixture = String::from_utf8(fs::read(&path).expect("read snapshot fixture"))
        .expect("requested fixture must be UTF-8");
    let documents = source_documents(&fixture, &requested);
    let formatted = documents
        .iter()
        .map(|document| SourceDocument {
            name: document.name.clone(),
            text: format_document_text(&document.text, options()),
        })
        .collect::<Vec<_>>();
    print!("{}", format_documents_section(&formatted));
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
        let rendering = graph.to_semantic_sexpr();
        let requires_skip = rendering == empty_semantic_graph()
            && documents
                .iter()
                .any(|document| !document.text.trim().is_empty());
        let metadata = updated_semantic_graph_metadata(
            section(&fixture, "META")
                .unwrap_or_else(|| panic!("{relative}: fixture is missing META")),
            requires_skip,
            documents
                .iter()
                .all(|document| sysml_v2_parser::parse(&document.text).is_ok()),
        );
        let fixture = replace_section(&fixture, "META", &metadata)
            .unwrap_or_else(|| panic!("{relative}: fixture is missing a META section"));
        let updated = replace_section(&fixture, "SMG", &rendering)
            .unwrap_or_else(|| panic!("{relative}: fixture is missing an SMG section"));
        fs::write(path, updated).expect("write fixture");
    }
}

#[test]
#[ignore = "rewrites checked-in formatter sections after deliberate review"]
fn regenerate_formatter_sections() {
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
        let formatted = documents
            .iter()
            .map(|document| SourceDocument {
                name: document.name.clone(),
                text: format_document_text(&document.text, options()),
            })
            .collect::<Vec<_>>();
        let updated =
            replace_or_insert_format_section(&fixture, &format_documents_section(&formatted))
                .unwrap_or_else(|| panic!("{relative}: fixture is missing an EXPECTED section"));
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
    section_documents(fixture, "SOURCE", fallback_name).unwrap_or_default()
}

fn section_documents(
    fixture: &str,
    section_name: &str,
    fallback_name: &str,
) -> Option<Vec<SourceDocument>> {
    let source = raw_section(fixture, section_name)?;
    Some(documents_from_section(source, fallback_name))
}

fn documents_from_section(source: &str, fallback_name: &str) -> Vec<SourceDocument> {
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

fn replace_or_insert_format_section(fixture: &str, replacement: &str) -> Option<String> {
    let format_marker = "# FORMAT\n";
    if let Some(start) = fixture.find(format_marker) {
        let after_marker = start + format_marker.len();
        let end = fixture[after_marker..]
            .find("\n# ")
            .map_or(fixture.len(), |index| after_marker + index + 1);
        let mut updated = String::with_capacity(fixture.len() + replacement.len());
        updated.push_str(&fixture[..after_marker]);
        updated.push_str(replacement);
        updated.push_str(&fixture[end..]);
        return Some(updated);
    }
    let insertion = fixture.find("# EXPECTED\n")?;
    let section = format!("# FORMAT\n{replacement}");
    let mut updated = String::with_capacity(fixture.len() + section.len());
    updated.push_str(&fixture[..insertion]);
    updated.push_str(&section);
    updated.push_str(&fixture[insertion..]);
    Some(updated)
}

fn format_documents_section(documents: &[SourceDocument]) -> String {
    assert!(
        !documents.is_empty(),
        "formatter section requires source documents"
    );
    if documents.len() == 1 {
        return format!("~~~sysml\n{}\n~~~\n", documents[0].text);
    }
    documents
        .iter()
        .map(|document| format!("## {}\n~~~sysml\n{}\n~~~\n", document.name, document.text))
        .collect::<Vec<_>>()
        .join("")
}

fn semantic_graph_skip_reason(fixture: &str) -> Result<Option<String>, String> {
    let state = unique_metadata_value(fixture, "semantic_graph")?;
    let reason = unique_metadata_value(fixture, "semantic_graph_skip_reason")?;
    match (state.as_deref(), reason) {
        (None, None) => Ok(None),
        (Some("skip"), Some(reason)) if !reason.trim().is_empty() => Ok(Some(reason)),
        (Some("skip"), Some(_)) => Err("semantic_graph_skip_reason must be non-empty".to_string()),
        (Some("skip"), None) => {
            Err("semantic_graph=skip requires semantic_graph_skip_reason".to_string())
        }
        (None, Some(_)) => {
            Err("semantic_graph_skip_reason requires semantic_graph=skip".to_string())
        }
        (Some(other), _) => Err(format!("unsupported semantic_graph state {other:?}")),
    }
}

fn formatter_skip_reason(fixture: &str) -> Result<Option<String>, String> {
    skip_reason(fixture, "formatter", "formatter_skip_reason")
}

fn skip_reason(fixture: &str, state_key: &str, reason_key: &str) -> Result<Option<String>, String> {
    let state = unique_metadata_value(fixture, state_key)?;
    let reason = unique_metadata_value(fixture, reason_key)?;
    match (state.as_deref(), reason) {
        (None, None) => Ok(None),
        (Some("skip"), Some(reason)) if !reason.trim().is_empty() => Ok(Some(reason)),
        (Some("skip"), Some(_)) => Err(format!("{reason_key} must be non-empty")),
        (Some("skip"), None) => Err(format!("{state_key}=skip requires {reason_key}")),
        (None, Some(_)) => Err(format!("{reason_key} requires {state_key}=skip")),
        (Some(other), _) => Err(format!("unsupported {state_key} state {other:?}")),
    }
}

fn formatter_safety_check(source: &str, formatted: &str) -> Result<(), String> {
    match sysml_v2_parser::parse(source) {
        Ok(original) => {
            let reparsed = sysml_v2_parser::parse(formatted).map_err(|error| {
                format!("strictly parsed source no longer parses after formatting: {error}")
            })?;
            if original.normalize_for_test_comparison() != reparsed.normalize_for_test_comparison()
            {
                return Err("formatting changed the normalized strict parse tree".to_string());
            }
        }
        Err(_) => {
            let original = sysml_v2_parser::parse_for_editor(source);
            let reparsed = sysml_v2_parser::parse_for_editor(formatted);
            if reparsed.is_ok() {
                return Err("formatting changed a recovery input into a strict parse".to_string());
            }
            if original.root.normalize_for_test_comparison()
                != reparsed.root.normalize_for_test_comparison()
            {
                return Err("formatting changed the normalized recovery parse tree".to_string());
            }
            if recovery_diagnostic_signature(&original.errors)
                != recovery_diagnostic_signature(&reparsed.errors)
            {
                return Err("formatting changed recovery diagnostic kinds or messages".to_string());
            }
        }
    }
    Ok(())
}

fn recovery_diagnostic_signature(errors: &[sysml_v2_parser::ParseError]) -> Vec<String> {
    errors
        .iter()
        .map(|error| {
            format!(
                "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
                error.category,
                error.severity,
                error.code,
                error.message,
                error.expected,
                error.found,
                error.is_cascade,
            )
        })
        .collect()
}

fn unique_metadata_value(fixture: &str, key: &str) -> Result<Option<String>, String> {
    let mut values = Vec::new();
    if let Some(metadata) = section(fixture, "META") {
        for line in metadata.lines() {
            let Some((line_key, value)) = line.split_once('=') else {
                continue;
            };
            if line_key.trim() == key {
                values.push(value.trim().to_string());
            }
        }
    }
    match values.as_slice() {
        [] => Ok(None),
        [value] => Ok(Some(value.clone())),
        _ => Err(format!("META contains duplicate {key} entries")),
    }
}

fn updated_semantic_graph_metadata(
    metadata: String,
    requires_skip: bool,
    parser_accepts_all: bool,
) -> String {
    let mut lines = metadata
        .lines()
        .filter(|line| {
            !line.starts_with("semantic_graph=") && !line.starts_with("semantic_graph_skip_reason=")
        })
        .map(str::to_string)
        .collect::<Vec<_>>();
    if requires_skip {
        lines.push("semantic_graph=skip".to_string());
        let reason = if parser_accepts_all {
            "strictly parsed non-empty source produced no typed semantic graph facts"
        } else {
            "parser recovery for non-empty source produced no typed semantic graph facts"
        };
        lines.push(format!("semantic_graph_skip_reason={reason}"));
    }
    lines.join("\n")
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
    assert_eq!(graph.to_semantic_sexpr(), empty_semantic_graph());
    assert_eq!(
        section(&fixture, "SMG").expect("camera SMG section"),
        empty_semantic_graph()
    );
    assert_eq!(
        semantic_graph_skip_reason(&fixture),
        Ok(Some(
            "strictly parsed non-empty source produced no typed semantic graph facts".to_string()
        ))
    );
}

#[test]
fn semantic_graph_skip_metadata_requires_non_empty_reason() {
    let valid = "# META\n~~~ini\nsemantic_graph=skip\nsemantic_graph_skip_reason=known semantic materialization bug\n~~~\n";
    assert_eq!(
        semantic_graph_skip_reason(valid),
        Ok(Some("known semantic materialization bug".to_string()))
    );
    let missing_reason = "# META\n~~~ini\nsemantic_graph=skip\n~~~\n";
    assert!(semantic_graph_skip_reason(missing_reason).is_err());
    let empty_reason = "# META\n~~~ini\nsemantic_graph=skip\nsemantic_graph_skip_reason=   \n~~~\n";
    assert!(semantic_graph_skip_reason(empty_reason).is_err());
    let orphan_reason = "# META\n~~~ini\nsemantic_graph_skip_reason=known bug\n~~~\n";
    assert!(semantic_graph_skip_reason(orphan_reason).is_err());
    let duplicate_state = "# META\n~~~ini\nsemantic_graph=skip\nsemantic_graph=skip\nsemantic_graph_skip_reason=known bug\n~~~\n";
    assert!(semantic_graph_skip_reason(duplicate_state).is_err());
}
