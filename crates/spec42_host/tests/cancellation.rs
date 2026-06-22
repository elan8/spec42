use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use semantic_core::{SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind};
use spec42_host::{
    CancellationToken, EngineBuilder, HostContext, Spec42Engine, WorkspaceLoadRequest,
};
use tempfile::tempdir;

struct SlowDocumentProvider {
    cancellation: CancellationToken,
    steps: usize,
    observed_loads: Arc<AtomicUsize>,
}

impl SlowDocumentProvider {
    fn new(cancellation: CancellationToken, steps: usize, observed_loads: Arc<AtomicUsize>) -> Self {
        Self {
            cancellation,
            steps,
            observed_loads,
        }
    }
}

impl SysmlDocumentProvider for SlowDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        self.observed_loads.fetch_add(1, Ordering::SeqCst);
        for step in 0..self.steps {
            if self.cancellation.is_cancelled() {
                return Err(format!("cancelled while loading documents at step {step}"));
            }
            thread::sleep(Duration::from_millis(20));
        }
        let doc = SysmlDocument::from_memory_path(
            "workspace",
            "Slow.sysml",
            "package Slow { part def Thing; }".to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        Ok(vec![doc])
    }
}

fn test_engine(cache: &tempfile::TempDir) -> Spec42Engine {
    EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .build()
        .expect("engine")
}

#[test]
fn load_workspace_returns_cancelled_when_token_fires_during_slow_load() {
    let cache = tempdir().expect("tempdir");
    let model_path = cache.path().join("Slow.sysml");
    std::fs::write(&model_path, "package Slow { part def Thing; }").expect("write");

    let cancellation = CancellationToken::new();
    let cancel_token = cancellation.clone();
    let loads = Arc::new(AtomicUsize::new(0));
    let provider = SlowDocumentProvider::new(cancellation, 50, Arc::clone(&loads));

    let engine = test_engine(&cache);
    let mut context = HostContext::default();
    context.cancellation = cancel_token.clone();

    let handle = thread::spawn(move || {
        engine.load_workspace(
            provider,
            WorkspaceLoadRequest::single_target(model_path),
            context,
        )
    });

    thread::sleep(Duration::from_millis(30));
    cancel_token.cancel();

    let result = handle.join().expect("thread join");
    let err = result.expect_err("expected cancellation");
    assert_eq!(err.code(), "cancelled");
    assert_eq!(loads.load(Ordering::SeqCst), 1);
}
