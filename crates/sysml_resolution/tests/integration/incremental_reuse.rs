//! Cold/warm parity for the per-document lowering memo, and the counted facts that state reuse.
//!
//! `AGENTS.md`: *new or modified memoization requires cold/warm parity*. The memo here is
//! snapshot-local in the sense that matters -- its key is the content digest of the document whose
//! product it holds, and nothing else feeds a lowering -- so the obligation is the one below: a
//! publication built warm, over a memo carrying the previous revision's documents, must be the
//! same publication a cold authority builds from the same sources. "Same" is checked as every
//! rendered projection *and* the dependency-complete identity, not as a spot check of one query.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sysml_resolution::publication::PublicationAuthority;
use sysml_resolution::syntax::SyntaxAuthority;
use sysml_resolution::PublishedResolution;
use sysml_source::{SourceAuthority, SourceDocument, SourceKind};

fn authority() -> PublicationAuthority {
    PublicationAuthority::new(Arc::new(SyntaxAuthority::new()))
}

/// Every rendered projection of a publication, concatenated: the whole observable model.
fn render(publication: &PublishedResolution) -> String {
    let mut output = String::new();
    let debug = publication.debug();
    debug.write_semantic_sexpr(&mut output).expect("semantic");
    debug
        .write_diagnostics_sexpr(&mut output)
        .expect("diagnostics");
    debug
        .write_navigation_sexpr(&mut output)
        .expect("navigation");
    debug.write_types_sexpr(&mut output).expect("types");
    output
}

/// Builds `documents` warm (against `warm`, which has already published `previous`) and cold, and
/// asserts the two publications are indistinguishable.
fn assert_warm_matches_cold(warm: &PublicationAuthority, documents: &[SourceDocument]) {
    let warm_publication = warm.publish(documents, []).expect("warm publication");
    let cold_publication = authority()
        .publish(documents, [])
        .expect("cold publication");
    assert_eq!(
        warm_publication.identity(),
        cold_publication.identity(),
        "a warm build and a cold build of the same sources are one publication identity"
    );
    assert_eq!(
        warm_publication.identity().model_digest(),
        cold_publication.identity().model_digest(),
        "typed model identity is independent of cache warmth"
    );
    assert_eq!(
        render(&warm_publication),
        render(&cold_publication),
        "a warm build and a cold build of the same sources render identically"
    );
}

fn admit(sources: &SourceAuthority, uri: &str, text: &str, kind: SourceKind) -> SourceDocument {
    sources.admit(uri, text, kind).expect("admitted document")
}

const LIBRARY: &str = "\
standard library package Bench {
    part def Wheel;
    part def Axle;
    attribute def Mass;
}
";

/// A synthetic workspace: several documents, each referring to the library and to its neighbours.
fn synthetic_workspace(sources: &SourceAuthority, revision: usize) -> Vec<SourceDocument> {
    let mut documents = vec![admit(
        sources,
        "memory://library/bench.sysml",
        LIBRARY,
        SourceKind::StandardLibrary,
    )];
    for index in 0..6usize {
        let comment = if index == 0 {
            format!("// revision {revision}\n")
        } else {
            String::new()
        };
        let text = format!(
            "{comment}package Doc{index} {{\n\
             \tprivate import Bench::*;\n\
             \tpart def Vehicle{index} {{\n\
             \t\tpart wheel : Wheel;\n\
             \t\tattribute mass : Mass;\n\
             \t}}\n\
             \tpart vehicle{index} : Vehicle{index};\n\
             }}\n"
        );
        documents.push(admit(
            sources,
            &format!("memory://workspace/doc{index}.sysml"),
            &text,
            SourceKind::Workspace,
        ));
    }
    documents
}

#[test]
fn a_warm_relink_of_a_multi_document_workspace_is_the_cold_publication() {
    let sources = SourceAuthority::new();
    let warm = authority();
    warm.publish(&synthetic_workspace(&sources, 0), [])
        .expect("initial publication");
    for revision in 1..4 {
        assert_warm_matches_cold(&warm, &synthetic_workspace(&sources, revision));
    }
}

#[test]
fn one_edited_document_is_the_only_document_lowered() {
    let sources = SourceAuthority::new();
    let warm = authority();
    let first = synthetic_workspace(&sources, 0);
    let (_, cold) = warm
        .prepare(&first, [])
        .expect("prepare")
        .build_measured()
        .expect("initial publication");
    // The library document is lowered once by the stratum build and reused by the workspace
    // build that follows it, so a first publication lowers every document exactly once overall.
    assert_eq!(
        cold.documents_reused, 1,
        "the library stratum's own lowering"
    );
    assert_eq!(cold.documents_lowered, first.len() - 1);

    let edited = synthetic_workspace(&sources, 1);
    let (_, measurements) = warm
        .prepare(&edited, [])
        .expect("prepare")
        .build_measured()
        .expect("warm publication");
    assert_eq!(
        measurements.documents_lowered, 1,
        "one edited document costs one lowering"
    );
    assert_eq!(measurements.documents_reused, edited.len() - 1);

    // Republishing the same sources lowers nothing at all.
    let (_, unchanged) = warm
        .prepare(&edited, [])
        .expect("prepare")
        .build_measured()
        .expect("republication");
    assert_eq!(unchanged.documents_lowered, 0);
    assert_eq!(unchanged.documents_reused, edited.len());
}

#[test]
fn an_edit_that_is_reverted_is_the_cold_publication_again() {
    let sources = SourceAuthority::new();
    let warm = authority();
    let original = synthetic_workspace(&sources, 0);
    warm.publish(&original, []).expect("initial publication");
    warm.publish(&synthetic_workspace(&sources, 1), [])
        .expect("edited publication");
    // The memo has evicted revision 0's document by now; reverting must still be exact.
    assert_warm_matches_cold(&warm, &original);
}

/// Every `.sysml`/`.kerml` file bundled under `examples/`, admitted as one workspace.
fn examples_corpus(sources: &SourceAuthority) -> Vec<SourceDocument> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut paths = Vec::new();
    collect_sources(&root, &mut paths);
    paths.sort();
    assert!(
        !paths.is_empty(),
        "no SysML sources under {}",
        root.display()
    );
    paths
        .iter()
        .map(|path| {
            let text = std::fs::read_to_string(path)
                .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
            let uri = format!(
                "memory://examples/{}",
                path.strip_prefix(&root).unwrap_or(path).display()
            );
            admit(sources, &uri, &text, SourceKind::Workspace)
        })
        .collect()
}

fn collect_sources(directory: &Path, into: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sources(&path, into);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "sysml" || extension == "kerml")
        {
            into.push(path);
        }
    }
}

#[test]
fn a_warm_relink_of_the_examples_corpus_is_the_cold_publication() {
    let sources = SourceAuthority::new();
    let corpus = examples_corpus(&sources);
    let warm = authority();
    warm.publish(&corpus, []).expect("initial publication");

    // Edit exactly one document: a trailing comment changes its digest and nothing else.
    let edited_uri = corpus
        .first()
        .expect("a non-empty examples corpus")
        .uri()
        .as_str()
        .to_owned();
    let mut edited = Vec::with_capacity(corpus.len());
    for document in &corpus {
        if document.uri().as_str() == edited_uri {
            let text = format!("{}\n// one keystroke\n", document.content());
            edited.push(admit(&sources, &edited_uri, &text, document.kind()));
        } else {
            edited.push(document.clone());
        }
    }

    let (_, measurements) = warm
        .prepare(&edited, [])
        .expect("prepare")
        .build_measured()
        .expect("warm publication");
    assert_eq!(
        measurements.documents_lowered, 1,
        "one keystroke in the examples corpus lowers one document"
    );
    assert_eq!(measurements.documents_reused, edited.len() - 1);

    assert_warm_matches_cold(&warm, &edited);
}
