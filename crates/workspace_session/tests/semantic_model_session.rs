use std::sync::Arc;

use workspace::{
    build_semantic_model_from_documents, ConstructionStrategy, EvaluationPolicy,
    SemanticConfiguration, SemanticModel, SysmlDocument, SysmlDocumentSourceKind,
};
use workspace_session::{
    SemanticBuildFailureKind, SemanticModelSession, SemanticPublicationOutcome,
};

fn model(uri: &str, content: &str) -> Arc<SemanticModel> {
    let document = SysmlDocument::from_uri(
        uri,
        content.to_string(),
        None,
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .unwrap();
    Arc::new(
        build_semantic_model_from_documents(
            &[document],
            ConstructionStrategy::Sequential,
            EvaluationPolicy::ResolvedOnly,
            SemanticConfiguration::default(),
        )
        .unwrap(),
    )
}

#[tokio::test]
async fn current_build_atomically_replaces_model_and_old_readers_remain_valid() {
    let first = model("memory://session/first.sysml", "package First {}");
    let second = model("memory://session/second.sysml", "package Second {}");
    let session = SemanticModelSession::new(first.clone());
    let retained = session.current();

    let token = session
        .begin_build(second.identity().clone())
        .await
        .unwrap();
    let outcome = session
        .finish_build(token, Ok(second.clone()))
        .await
        .unwrap();

    assert_eq!(outcome, SemanticPublicationOutcome::Published);
    assert!(Arc::ptr_eq(&retained, &first));
    assert!(Arc::ptr_eq(&session.current(), &second));
    assert_eq!(retained.identity(), first.identity());
}

#[tokio::test]
async fn failed_and_cancelled_builds_keep_prior_model_and_ready_lifecycle() {
    let first = model("memory://session/first.sysml", "package First {}");
    let session = SemanticModelSession::new(first.clone());

    let failed = session.begin_build(first.identity().clone()).await.unwrap();
    assert_eq!(
        session
            .finish_build(failed, Err(SemanticBuildFailureKind::Failed))
            .await
            .unwrap(),
        SemanticPublicationOutcome::DiscardedFailed
    );
    assert!(Arc::ptr_eq(&session.current(), &first));

    let cancelled = session.begin_build(first.identity().clone()).await.unwrap();
    assert_eq!(
        session
            .finish_build(cancelled, Err(SemanticBuildFailureKind::Cancelled))
            .await
            .unwrap(),
        SemanticPublicationOutcome::DiscardedCancelled
    );
    assert!(Arc::ptr_eq(&session.current(), &first));
}

#[tokio::test]
async fn mismatched_identity_is_rejected_without_replacing_prior_model() {
    let first = model("memory://session/first.sysml", "package First {}");
    let second = model("memory://session/second.sysml", "package Second {}");
    let session = SemanticModelSession::new(first.clone());
    let token = session.begin_build(first.identity().clone()).await.unwrap();

    assert_eq!(
        session.finish_build(token, Ok(second)).await.unwrap(),
        SemanticPublicationOutcome::DiscardedIdentityMismatch
    );
    assert!(Arc::ptr_eq(&session.current(), &first));
}

#[tokio::test]
async fn stale_out_of_order_build_cannot_replace_newer_model() {
    let first = model("memory://session/first.sysml", "package First {}");
    let second = model("memory://session/second.sysml", "package Second {}");
    let third = model("memory://session/third.sysml", "package Third {}");
    let session = SemanticModelSession::new(first);

    let stale = session
        .begin_build(second.identity().clone())
        .await
        .unwrap();
    let current = session.begin_build(third.identity().clone()).await.unwrap();

    assert_eq!(
        session.finish_build(stale, Ok(second)).await.unwrap(),
        SemanticPublicationOutcome::Stale
    );
    assert_eq!(
        session
            .finish_build(current, Ok(third.clone()))
            .await
            .unwrap(),
        SemanticPublicationOutcome::Published
    );
    assert!(Arc::ptr_eq(&session.current(), &third));
}

#[tokio::test]
async fn token_from_another_owner_is_stale() {
    let first = model("memory://session/first.sysml", "package First {}");
    let second = model("memory://session/second.sysml", "package Second {}");
    let left = SemanticModelSession::new(first);
    let right = SemanticModelSession::new(second.clone());
    let token = left.begin_build(second.identity().clone()).await.unwrap();

    assert_eq!(
        right.finish_build(token, Ok(second.clone())).await.unwrap(),
        SemanticPublicationOutcome::Stale
    );
    assert!(Arc::ptr_eq(&right.current(), &second));
}
