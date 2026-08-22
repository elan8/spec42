//! The one reporting policy a host has, over diagnostics it does not decide.

use sysml_diagnostics::{document_diagnostics, DiagnosticSeverity, ReportingPolicy};
use sysml_query::resolved_slice::{
    build, BuildRequest, ConstructionStrategy, PublishedModel, SourceDocument, SourceKind,
};
use url::Url;

const DOCUMENT: &str = "memory://policy.sysml";

fn publish(source: &str) -> PublishedModel {
    let request = BuildRequest::resolved(
        vec![
            SourceDocument::from_uri(DOCUMENT, source.to_string(), SourceKind::Workspace)
                .expect("source"),
        ],
        ConstructionStrategy::Sequential,
    )
    .expect("request");
    build(request).expect("publication")
}

fn codes(source: &str, policy: ReportingPolicy) -> Vec<String> {
    let model = publish(source);
    document_diagnostics(&model, &Url::parse(DOCUMENT).expect("uri"), policy)
        .into_iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

/// A document the parser accepted reports the same thing under either policy.
///
/// Strictness is about a document that did not parse; it must not quietly narrow a document that
/// did, or batch validation and the editor would disagree about a model both could read.
#[test]
fn a_document_that_parses_reports_the_same_under_either_policy() {
    let source = "package P { part def A; part def A; part b; }";
    assert_eq!(
        codes(source, ReportingPolicy::strict(true)),
        codes(source, ReportingPolicy::default())
    );
    assert!(!codes(source, ReportingPolicy::default()).is_empty());
}

/// A document the parser rejected reports only the parser's own errors under the strict policy.
///
/// The semantic answers are still settled and still published -- the editor shows them -- but a
/// batch report about a model the author did not write is noise, so the host chooses not to show
/// them. Nothing is recomputed and no severity changes; this is a filter over settled values.
#[test]
fn a_document_that_does_not_parse_reports_only_parse_errors_under_the_strict_policy() {
    let source = "package P { part def A; part def A; part b; part def C { connect ; } }";
    let strict = codes(source, ReportingPolicy::strict(true));
    let interactive = codes(source, ReportingPolicy::default());

    assert!(
        !strict.is_empty(),
        "the parse error itself is still reported"
    );
    assert!(
        strict.len() < interactive.len(),
        "strict narrows to the parser's own errors: {strict:?} vs {interactive:?}"
    );

    let model = publish(source);
    let reported = document_diagnostics(
        &model,
        &Url::parse(DOCUMENT).expect("uri"),
        ReportingPolicy::strict(true),
    );
    assert!(
        reported.iter().all(|diagnostic| {
            diagnostic.source == "sysml" && diagnostic.severity == DiagnosticSeverity::Error
        }),
        "only parser-owned errors survive: {reported:#?}"
    );
}

/// A document the publication did not admit reports nothing, whatever the policy.
#[test]
fn an_unadmitted_document_reports_nothing() {
    let model = publish("package P { part def A; }");
    let elsewhere = Url::parse("memory://absent.sysml").expect("uri");
    assert!(document_diagnostics(&model, &elsewhere, ReportingPolicy::default()).is_empty());
}

/// Asking twice, and asking about documents in either order, answers identically.
#[test]
fn repeated_and_reordered_queries_answer_identically() {
    let model = publish("package P { part def A; part def A; part b; }");
    let admitted = Url::parse(DOCUMENT).expect("uri");
    let absent = Url::parse("memory://absent.sysml").expect("uri");

    let first = document_diagnostics(&model, &admitted, ReportingPolicy::default());
    let _ = document_diagnostics(&model, &absent, ReportingPolicy::default());
    let second = document_diagnostics(&model, &admitted, ReportingPolicy::default());
    assert_eq!(first, second);
}
