//! Incremental-vs-full parity over a deterministic edit sequence.
//!
//! `AGENTS.md`: at the same declared phase and model identity, full, incremental, cached, and
//! parallel paths must be observably equivalent. Warm publication (the host session's lowering
//! memo) is the incremental path that exists today; a cold `PublicationAuthority` is the full
//! path. This harness applies a fixed sequence of document edits and, after each step, requires
//! the warm publication to be the cold one: dependency-complete identity, model digest, and
//! every rendered projection, including diagnostics.
//!
//! Counted reuse facts are part of the contract: a one-document edit must lower that document
//! and no other. The canonical digest/diagnostic report of the sequence is snapshotted with
//! `insta` so an identity or diagnostic-code change is a reviewable snapshot diff rather than a
//! silent pass of `assert_eq!` on two moving values.

use std::sync::Arc;

use sysml_resolution::publication::PublicationAuthority;
use sysml_resolution::syntax::SyntaxAuthority;
use sysml_resolution::{BuildMeasurements, PublishedResolution};
use sysml_source::{SourceAuthority, SourceDocument, SourceKind};

fn authority() -> PublicationAuthority {
    PublicationAuthority::new(Arc::new(SyntaxAuthority::new()))
}

fn admit(sources: &SourceAuthority, uri: &str, text: &str, kind: SourceKind) -> SourceDocument {
    sources.admit(uri, text, kind).expect("admitted document")
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
    debug
        .write_expressions_sexpr(&mut output)
        .expect("expressions");
    debug
        .write_metadata_annotations_sexpr(&mut output)
        .expect("metadata annotations");
    debug
        .write_connections_sexpr(&mut output)
        .expect("connections");
    debug
        .write_projection_sexpr(&mut output)
        .expect("projection");
    output
}

fn diagnostics_sexpr(publication: &PublishedResolution) -> String {
    let mut output = String::new();
    publication
        .debug()
        .write_diagnostics_sexpr(&mut output)
        .expect("diagnostics");
    output
}

fn diagnostic_codes(diagnostics: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let mut rest = diagnostics;
    while let Some(idx) = rest.find("(code \"") {
        rest = &rest[idx + "(code \"".len()..];
        match rest.find('"') {
            Some(end) => {
                codes.push(rest[..end].to_owned());
                rest = &rest[end + 1..];
            }
            None => break,
        }
    }
    codes
}

/// Builds `documents` warm (against `warm`) and cold, and asserts the two publications are
/// indistinguishable. Returns the warm publication so the caller can record its canonical
/// identity and diagnostics.
fn assert_warm_matches_cold(
    warm: &PublicationAuthority,
    documents: &[SourceDocument],
    label: &str,
) -> (PublishedResolution, BuildMeasurements) {
    let (warm_publication, measurements) = warm
        .prepare(documents, [])
        .unwrap_or_else(|error| panic!("{label}: warm prepare: {error}"))
        .build_measured()
        .unwrap_or_else(|error| panic!("{label}: warm publication: {error}"));
    let cold_publication = authority()
        .publish(documents, [])
        .unwrap_or_else(|error| panic!("{label}: cold publication: {error}"));
    assert_eq!(
        warm_publication.identity(),
        cold_publication.identity(),
        "{label}: a warm build and a cold build of the same sources are one publication identity"
    );
    assert_eq!(
        warm_publication.identity().model_digest(),
        cold_publication.identity().model_digest(),
        "{label}: typed model identity is independent of cache warmth"
    );
    assert_eq!(
        render(&warm_publication),
        render(&cold_publication),
        "{label}: a warm build and a cold build of the same sources render identically"
    );
    (warm_publication, measurements)
}

fn replace_uri(
    sources: &SourceAuthority,
    documents: &[SourceDocument],
    uri: &str,
    text: &str,
) -> Vec<SourceDocument> {
    documents
        .iter()
        .map(|document| {
            if document.uri().as_str() == uri {
                admit(sources, uri, text, document.kind())
            } else {
                document.clone()
            }
        })
        .collect()
}

fn add_document(
    sources: &SourceAuthority,
    documents: &[SourceDocument],
    uri: &str,
    text: &str,
    kind: SourceKind,
) -> Vec<SourceDocument> {
    let mut next = documents.to_vec();
    next.push(admit(sources, uri, text, kind));
    next
}

fn remove_uri(documents: &[SourceDocument], uri: &str) -> Vec<SourceDocument> {
    documents
        .iter()
        .filter(|document| document.uri().as_str() != uri)
        .cloned()
        .collect()
}

const LIBRARY: &str = "\
package Bench {
    part def Wheel;
    part def Axle;
}
";

const VEHICLE: &str = "\
package Vehicle {
    private import Bench::*;
    part def Car {
        part wheel : Wheel;
    }
}
";

const FLEET: &str = "\
package Fleet {
    private import Vehicle::*;
    part car : Car;
}
";

const FLEET_COMMENTED: &str = "\
package Fleet {
    private import Vehicle::*;
    // one keystroke
    part car : Car;
}
";

const FLEET_UNRESOLVED: &str = "\
package Fleet {
    private import Vehicle::*;
    part car : Missing;
}
";

const DRIVER: &str = "\
package Driver {
    private import Vehicle::*;
    part car : Car;
}
";

const LIBRARY_URI: &str = "memory://library/bench.sysml";
const VEHICLE_URI: &str = "memory://workspace/vehicle.sysml";
const FLEET_URI: &str = "memory://workspace/fleet.sysml";
const DRIVER_URI: &str = "memory://workspace/driver.sysml";

fn baseline(sources: &SourceAuthority) -> Vec<SourceDocument> {
    vec![
        admit(sources, LIBRARY_URI, LIBRARY, SourceKind::Workspace),
        admit(sources, VEHICLE_URI, VEHICLE, SourceKind::Workspace),
        admit(sources, FLEET_URI, FLEET, SourceKind::Workspace),
    ]
}

struct Step {
    label: &'static str,
    expected_lowered: usize,
    documents: Vec<SourceDocument>,
}

/// A representative edit sequence: comment-only, unresolved reference, revert, add, remove.
///
/// Shared by the correctness test and the reuse-count test below so the two concerns can be
/// asserted independently over the same fixture rather than duplicating it.
fn edit_sequence(sources: &SourceAuthority) -> Vec<Step> {
    let mut documents = baseline(sources);
    let mut steps = Vec::new();
    steps.push(Step {
        label: "baseline",
        expected_lowered: 0,
        documents: documents.clone(),
    });

    documents = replace_uri(sources, &documents, FLEET_URI, FLEET_COMMENTED);
    steps.push(Step {
        label: "comment-only-on-fleet",
        expected_lowered: 1,
        documents: documents.clone(),
    });

    documents = replace_uri(sources, &documents, FLEET_URI, FLEET_UNRESOLVED);
    steps.push(Step {
        label: "unresolved-type-on-fleet",
        expected_lowered: 1,
        documents: documents.clone(),
    });

    documents = replace_uri(sources, &documents, FLEET_URI, FLEET);
    steps.push(Step {
        label: "revert-fleet-to-baseline",
        expected_lowered: 1,
        documents: documents.clone(),
    });

    documents = add_document(
        sources,
        &documents,
        DRIVER_URI,
        DRIVER,
        SourceKind::Workspace,
    );
    steps.push(Step {
        label: "add-driver",
        expected_lowered: 1,
        documents: documents.clone(),
    });

    documents = remove_uri(&documents, DRIVER_URI);
    steps.push(Step {
        label: "remove-driver",
        expected_lowered: 0,
        documents,
    });

    steps
}

fn report_step(label: &str, publication: &PublishedResolution) -> String {
    let codes = diagnostic_codes(&diagnostics_sexpr(publication));
    let codes = if codes.is_empty() {
        "[]".to_owned()
    } else {
        format!("[{}]", codes.join(", "))
    };
    format!(
        "{label}\n  source {}\n  model {}\n  codes {codes}\n",
        publication.identity().source_digest(),
        publication.identity().model_digest(),
    )
}

/// Each step is a new publication of the whole workspace. Warm construction may reuse every
/// unchanged document's lowering; it must not produce a different identity or diagnostic set than
/// building the same sources cold. The revert and remove steps must also restore the exact
/// baseline identity and model digest -- not merely agree with their own cold rebuild, which would
/// still pass if edit-then-undo left both warm and cold wrong in the same way.
///
/// This test does not check how many documents were relowered: that is a memo-policy fact, not a
/// correctness fact, and is asserted on its own below by
/// `an_edit_sequence_lowers_only_the_changed_documents` so a smarter memo cannot fail this test
/// even though warm and cold still agree.
#[test]
fn an_edit_sequence_agrees_warm_and_cold() {
    let sources = SourceAuthority::new();
    let warm = authority();
    let steps = edit_sequence(&sources);
    // Populate the memo so the baseline observation is a true warm republish.
    warm.publish(&steps[0].documents, [])
        .expect("seed publication");

    let mut baseline_identity = None;
    let mut report = String::new();
    for (index, step) in steps.iter().enumerate() {
        let (publication, _measurements) =
            assert_warm_matches_cold(&warm, &step.documents, step.label);
        match step.label {
            "baseline" => baseline_identity = Some(publication.identity().clone()),
            "revert-fleet-to-baseline" | "remove-driver" => {
                let expected = baseline_identity
                    .as_ref()
                    .expect("baseline step observed before revert/remove");
                assert_eq!(
                    publication.identity(),
                    expected,
                    "{}: must restore the baseline publication identity",
                    step.label
                );
            }
            _ => {}
        }
        report.push_str(&format!("step {index} "));
        report.push_str(&report_step(step.label, &publication));
    }

    insta::assert_snapshot!(report);
}

/// The reuse contract for the memo: a one-document edit lowers that document and no other.
///
/// This is a memo-policy fact rather than a correctness fact -- see
/// `an_edit_sequence_agrees_warm_and_cold` above, which asserts warm/cold parity without
/// depending on these counts. A future memo that, say, recognized a reverted document as
/// bit-identical to a prior lowering would change what this test expects without changing
/// whether warm and cold agree.
#[test]
fn an_edit_sequence_lowers_only_the_changed_documents() {
    let sources = SourceAuthority::new();
    let warm = authority();
    let steps = edit_sequence(&sources);
    warm.publish(&steps[0].documents, [])
        .expect("seed publication");

    for step in &steps {
        let (_publication, measurements) = warm
            .prepare(&step.documents, [])
            .unwrap_or_else(|error| panic!("{}: warm prepare: {error}", step.label))
            .build_measured()
            .unwrap_or_else(|error| panic!("{}: warm publication: {error}", step.label));
        assert_eq!(
            measurements.documents_lowered, step.expected_lowered,
            "{}: expected {} document(s) lowered, got {}",
            step.label, step.expected_lowered, measurements.documents_lowered
        );
    }
}
