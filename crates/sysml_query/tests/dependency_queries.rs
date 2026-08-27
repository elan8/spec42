//! `workspace_documents_affected_by` is the one derivation of "what must a host republish".
//!
//! It answers from the publication's settled import and alias facts, and keeps the unsettled case
//! explicit: a publication that could not settle the dependency graph over-invalidates and says
//! so, rather than returning a narrower set that looks exact.

use std::sync::Arc;

use sysml_query::resolved_slice::PublishedModel;
use sysml_query::source::{SourceKind, SourceService, Url};

fn model(sources: &[(&str, &str)]) -> Arc<PublishedModel> {
    let source = SourceService::new();
    let documents = sources
        .iter()
        .map(|(uri, content)| source.admit(uri, content, SourceKind::Workspace).unwrap())
        .collect::<Vec<_>>();
    sysml_query::Services::new()
        .publication
        .publish(&documents, [])
        .unwrap()
}

fn uri(value: &str) -> Url {
    Url::parse(value).unwrap()
}

#[test]
fn follows_nested_public_imports_transitively() {
    let a = "file:///workspace/a.sysml";
    let b = "file:///workspace/b.sysml";
    let c = "file:///workspace/c.sysml";
    let model = model(&[
        (a, "package A { part def T; }"),
        (b, "package B { package Nested { public import A::*; } }"),
        (c, "package C { import B::Nested::*; part p : T; }"),
    ]);
    let affected = model
        .dependencies()
        .workspace_documents_affected_by([uri(a), uri(b), uri(c)], &uri(a));
    assert!(!affected.is_conservative());
    assert_eq!(affected.into_uris(), vec![uri(b), uri(c)]);
}

#[test]
fn alias_binding_is_a_semantic_dependency() {
    let a = "file:///workspace/a.sysml";
    let b = "file:///workspace/b.sysml";
    let model = model(&[
        (a, "package A { part def Thing; }"),
        (b, "package B { alias PublicThing for A::Thing; }"),
    ]);
    let affected = model
        .dependencies()
        .workspace_documents_affected_by([uri(a), uri(b)], &uri(a));
    assert!(!affected.is_conservative());
    assert_eq!(affected.into_uris(), vec![uri(b)]);
}

#[test]
fn recovery_is_explicit_and_overinvalidates() {
    let a = "file:///workspace/a.sysml";
    let b = "file:///workspace/b.sysml";
    let c = "file:///workspace/c.sysml";
    let model = model(&[
        (a, "package A {}"),
        (b, "package B { import Missing::*;"),
        (c, "package C {}"),
    ]);
    let affected = model
        .dependencies()
        .workspace_documents_affected_by([uri(a), uri(b), uri(c)], &uri(a));
    assert!(
        affected.is_conservative(),
        "an unsettled dependency graph must say so rather than look exact"
    );
    assert_eq!(affected.into_uris(), vec![uri(b), uri(c)]);
}

#[test]
fn an_exact_empty_answer_is_distinguishable_from_over_invalidation() {
    let a = "file:///workspace/a.sysml";
    let b = "file:///workspace/b.sysml";
    let model = model(&[(a, "package A { part def T; }"), (b, "package B {}")]);
    let affected = model
        .dependencies()
        .workspace_documents_affected_by([uri(a), uri(b)], &uri(a));
    assert!(!affected.is_conservative());
    assert!(
        affected.uris().is_empty(),
        "nothing imports A, so nothing else needs republishing"
    );
}
