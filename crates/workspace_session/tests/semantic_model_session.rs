use std::sync::Arc;
use std::time::Duration;

use workspace::{
    build_semantic_model_from_documents, ConstructionStrategy, EvaluationPolicy,
    SemanticBuildRequest, SemanticConfiguration, SemanticModel, SysmlDocument,
    SysmlDocumentSourceKind,
};
use workspace_session::{
    SemanticBuildFailureKind, SemanticModelSession, SemanticPublicationOutcome,
};

fn request(uri: &str, content: &str, evaluation: EvaluationPolicy) -> SemanticBuildRequest {
    let document = SysmlDocument::from_uri(
        uri,
        content.to_string(),
        None,
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .unwrap();
    SemanticBuildRequest {
        sources: workspace::ImmutableSourceSnapshot::new(vec![document]).unwrap(),
        construction: ConstructionStrategy::Sequential,
        evaluation,
        configuration: SemanticConfiguration::default(),
    }
}

fn model(build: &SemanticBuildRequest) -> Arc<SemanticModel> {
    Arc::new(
        build_semantic_model_from_documents(
            build.sources.documents(),
            build.construction,
            build.evaluation,
            build.configuration.clone(),
        )
        .unwrap(),
    )
}

#[tokio::test]
async fn current_build_atomically_replaces_model_and_old_readers_remain_valid() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let second_request = request(
        "memory://session/second.sysml",
        "package Second {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let second = model(&second_request);
    let session = SemanticModelSession::new(first.clone());
    let retained = session.current();

    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    let token = session.begin_build(&second_request).await.unwrap();
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Reindexing);
    assert_eq!(&second_request.identity(), token.identity());
    let outcome = session
        .finish_build(token, Ok(second.clone()))
        .await
        .unwrap();

    assert_eq!(outcome, SemanticPublicationOutcome::Published);
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&retained, &first));
    assert!(Arc::ptr_eq(&session.current(), &second));
    assert_eq!(retained.identity(), first.identity());
}

#[tokio::test]
async fn failed_and_cancelled_builds_keep_prior_model_and_ready_lifecycle() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let session = SemanticModelSession::new(first.clone());

    let failed = session.begin_build(&first_request).await.unwrap();
    assert_eq!(
        session
            .finish_build(failed, Err(SemanticBuildFailureKind::Failed))
            .await
            .unwrap(),
        SemanticPublicationOutcome::DiscardedFailed
    );
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&session.current(), &first));

    let cancelled = session.begin_build(&first_request).await.unwrap();
    assert_eq!(
        session
            .finish_build(cancelled, Err(SemanticBuildFailureKind::Cancelled))
            .await
            .unwrap(),
        SemanticPublicationOutcome::DiscardedCancelled
    );
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&session.current(), &first));
}

#[tokio::test]
async fn incomplete_recovery_build_is_retained_as_prior_complete_model() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let recovery_request = request(
        "memory://session/recovery.sysml",
        "package Recovery { part p : ;",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let recovery = model(&recovery_request);
    assert_eq!(
        recovery.completeness(),
        workspace::SemanticCompleteness::EditorRecovery
    );
    let session = SemanticModelSession::new(first.clone());
    let token = session.begin_build(&recovery_request).await.unwrap();

    assert_eq!(
        session.finish_build(token, Ok(recovery)).await.unwrap(),
        SemanticPublicationOutcome::DiscardedIncomplete
    );
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&session.current(), &first));
}

#[tokio::test]
async fn mismatched_identity_is_rejected_without_replacing_prior_model() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let second_request = request(
        "memory://session/second.sysml",
        "package Second {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let second = model(&second_request);
    let session = SemanticModelSession::new(first.clone());
    let token = session.begin_build(&first_request).await.unwrap();

    assert_eq!(
        session.finish_build(token, Ok(second)).await.unwrap(),
        SemanticPublicationOutcome::DiscardedIdentityMismatch
    );
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&session.current(), &first));
}

#[tokio::test]
async fn superseded_failure_and_cancellation_do_not_replace_or_finish_newer_build() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let second_request = request(
        "memory://session/second.sysml",
        "package Second {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let third_request = request(
        "memory://session/third.sysml",
        "package Third {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let third = model(&third_request);
    let session = SemanticModelSession::new(first.clone());

    let stale = session.begin_build(&second_request).await.unwrap();
    let current = session.begin_build(&third_request).await.unwrap();
    assert_eq!(
        session
            .finish_build(stale, Err(SemanticBuildFailureKind::Failed))
            .await
            .unwrap(),
        SemanticPublicationOutcome::Stale
    );
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Reindexing);
    assert_eq!(
        session
            .finish_build(current, Err(SemanticBuildFailureKind::Cancelled))
            .await
            .unwrap(),
        SemanticPublicationOutcome::DiscardedCancelled
    );
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&session.current(), &first));
    assert!(!Arc::ptr_eq(&session.current(), &third));
}

#[tokio::test]
async fn stale_out_of_order_build_cannot_replace_newer_model() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let second_request = request(
        "memory://session/second.sysml",
        "package Second {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let third_request = request(
        "memory://session/third.sysml",
        "package Third {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let second = model(&second_request);
    let third = model(&third_request);
    let session = SemanticModelSession::new(first);

    let stale = session.begin_build(&second_request).await.unwrap();
    let current = session.begin_build(&third_request).await.unwrap();

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
    assert_eq!(session.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&session.current(), &third));
}

#[tokio::test]
async fn token_from_another_owner_is_stale() {
    let first_request = request(
        "memory://session/first.sysml",
        "package First {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let second_request = request(
        "memory://session/second.sysml",
        "package Second {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let first = model(&first_request);
    let second = model(&second_request);
    let left = SemanticModelSession::new(first);
    let right = SemanticModelSession::new(second.clone());
    let token = left.begin_build(&second_request).await.unwrap();

    assert_eq!(
        right.finish_build(token, Ok(second.clone())).await.unwrap(),
        SemanticPublicationOutcome::Stale
    );
    assert_eq!(right.lifecycle(), workspace::SessionLifecycle::Ready);
    assert!(Arc::ptr_eq(&right.current(), &second));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_readers_retain_coherent_models_during_publication() {
    let initial_request = request(
        "memory://session/initial.sysml",
        "package Initial {}",
        EvaluationPolicy::ResolvedOnly,
    );
    let initial = model(&initial_request);
    let session = Arc::new(SemanticModelSession::new(initial));
    let reader_session = session.clone();
    let reader = tokio::spawn(async move {
        for _ in 0..2_000 {
            let retained = reader_session.current();
            assert_eq!(
                retained.completeness(),
                workspace::SemanticCompleteness::Complete
            );
            assert!(!retained.identity().source_digest.is_empty());
        }
    });

    for index in 0..32 {
        let build = request(
            &format!("memory://session/{index}.sysml"),
            &format!("package P{index} {{}}"),
            EvaluationPolicy::ResolvedOnly,
        );
        let result = model(&build);
        let token = session.begin_build(&build).await.unwrap();
        assert_eq!(
            session.finish_build(token, Ok(result)).await.unwrap(),
            SemanticPublicationOutcome::Published
        );
    }
    tokio::time::timeout(Duration::from_secs(2), reader)
        .await
        .expect("readers must not block on publication")
        .unwrap();
}
