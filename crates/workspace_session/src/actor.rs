use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, watch};
use workspace::RelinkToken;

/// Implemented by embedder session state to let [`SessionActor::report_job_result`] check
/// whether a background job's token is still current without this crate knowing anything
/// about `M`'s internal layout. Typically delegates to a `workspace::WorkspaceSession` field:
/// `self.session.is_token_current(token)`.
pub trait TracksRelink {
    fn is_token_current(&self, token: &RelinkToken) -> bool;
}

/// Error returned to a [`SessionActor::mutate`] caller when the supplied closure panicked.
/// The actor itself survives — this only reports that this one mutation did not apply and the
/// published snapshot is unchanged from before the call.
#[derive(Debug, thiserror::Error)]
#[error("mutate closure panicked; session state left unchanged")]
pub struct MutatePanicked;

type BoxedAny = Box<dyn std::any::Any + Send>;
type BoxedApply<M> = Box<dyn FnOnce(&mut M) -> BoxedAny + Send>;

enum Command<M> {
    Mutate {
        apply: BoxedApply<M>,
        reply: oneshot::Sender<Result<BoxedAny, MutatePanicked>>,
    },
    JobResult {
        token: RelinkToken,
        merge: Box<dyn FnOnce(&mut M) + Send>,
    },
}

/// A single background task owning a private `M`, reachable only through its mailbox.
///
/// Readers never go through this actor at all — they hold a [`crate::SnapshotHandle`] cloned
/// off [`spawn`](Self::spawn) and read the latest published state lock-free. Writers either
/// apply a cheap, synchronous mutation inline via [`mutate`](Self::mutate) (resolves once
/// applied and published), or hand back the result of an expensive rebuild computed elsewhere
/// (e.g. via `tokio::task::spawn_blocking`) via [`report_job_result`](Self::report_job_result),
/// which is dropped silently if the token proves it was superseded.
#[derive(Clone)]
pub struct SessionActor<M> {
    tx: mpsc::UnboundedSender<Command<M>>,
}

impl<M: Clone + Send + Sync + TracksRelink + 'static> SessionActor<M> {
    /// Spawns the actor task and returns a handle to control it plus a snapshot handle for
    /// reading its published state.
    pub fn spawn(initial: M) -> (Self, crate::SnapshotHandle<M>) {
        let (tx, mut rx) = mpsc::unbounded_channel::<Command<M>>();
        let (watch_tx, watch_rx) = watch::channel(Arc::new(initial));

        tokio::spawn(async move {
            let mut state: Arc<M> = watch_tx.borrow().clone();
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Mutate { apply, reply } => {
                        let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| {
                            apply(Arc::make_mut(&mut state))
                        }));
                        match outcome {
                            Ok(boxed) => {
                                let _ = watch_tx.send(state.clone());
                                let _ = reply.send(Ok(boxed));
                            }
                            Err(payload) => {
                                tracing::error!(
                                    "mutate closure panicked: {}",
                                    panic_message(&payload)
                                );
                                let _ = reply.send(Err(MutatePanicked));
                                // Resync from the last-published value — discard any torn
                                // partial mutation the panicking closure may have made via
                                // Arc::make_mut before it unwound.
                                state = watch_tx.borrow().clone();
                            }
                        }
                    }
                    Command::JobResult { token, merge } => {
                        if !state.is_token_current(&token) {
                            // Superseded by a newer mutation/job — drop silently, same
                            // semantics as `WorkspaceSession::commit_relink` returning `false`.
                            continue;
                        }
                        let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| {
                            merge(Arc::make_mut(&mut state));
                        }));
                        match outcome {
                            Ok(()) => {
                                let _ = watch_tx.send(state.clone());
                            }
                            Err(payload) => {
                                tracing::error!(
                                    "report_job_result merge panicked: {}",
                                    panic_message(&payload)
                                );
                                state = watch_tx.borrow().clone();
                            }
                        }
                    }
                }
            }
        });

        (Self { tx }, crate::SnapshotHandle::new(watch_rx))
    }

    /// Applies a cheap, synchronous mutation inline on the actor and publishes the result
    /// before resolving, returning whatever `apply` returns. Use this for the fast path
    /// (e.g. patching one document's text) — never for anything that itself does slow work,
    /// since that would delay every other queued command behind it.
    pub async fn mutate<R: Send + 'static>(
        &self,
        apply: impl FnOnce(&mut M) -> R + Send + 'static,
    ) -> Result<R, MutatePanicked> {
        let (reply_tx, reply_rx) = oneshot::channel();
        let boxed_apply: BoxedApply<M> =
            Box::new(move |state: &mut M| Box::new(apply(state)) as BoxedAny);
        let _ = self.tx.send(Command::Mutate {
            apply: boxed_apply,
            reply: reply_tx,
        });
        match reply_rx.await.unwrap_or(Err(MutatePanicked)) {
            Ok(boxed) => Ok(*boxed
                .downcast::<R>()
                .expect("R matches the closure's own return type by construction")),
            Err(MutatePanicked) => Err(MutatePanicked),
        }
    }

    /// Fire-and-forget: hands back the result of a rebuild computed off the actor (typically
    /// via `tokio::task::spawn_blocking`). Merged in only if `token` is still current;
    /// otherwise dropped silently.
    pub fn report_job_result(
        &self,
        token: RelinkToken,
        merge: impl FnOnce(&mut M) + Send + 'static,
    ) {
        let _ = self.tx.send(Command::JobResult {
            token,
            merge: Box::new(merge),
        });
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> &str {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("non-string panic payload")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default)]
    struct TestState {
        generation: u64,
        value: i32,
    }

    impl TracksRelink for TestState {
        fn is_token_current(&self, token: &RelinkToken) -> bool {
            token.generation() == self.generation
        }
    }

    #[tokio::test]
    async fn mutate_applies_synchronously_and_publishes_before_returning() {
        let (actor, snapshot) = SessionActor::spawn(TestState::default());
        actor.mutate(|s| s.value = 5).await.unwrap();
        assert_eq!(snapshot.current().value, 5);
    }

    #[tokio::test]
    async fn stale_job_result_is_dropped_and_does_not_publish() {
        let (actor, snapshot) = SessionActor::spawn(TestState {
            generation: 2,
            value: 0,
        });
        actor.mutate(|s| s.value = 1).await.unwrap();

        // Mint a real RelinkToken at generation 1 via a fresh WorkspaceSession — the test
        // state expects generation 2, so this token is stale.
        let mut session = workspace::WorkspaceSession::new();
        session.complete_startup();
        let stale_token = session.schedule_relink();

        actor.report_job_result(stale_token, |s| s.value = 999);
        // Fence: an empty mutate only resolves after the mailbox (FIFO) has drained the
        // preceding report_job_result command, so this deterministically waits for it.
        actor.mutate(|_| {}).await.unwrap();

        assert_eq!(snapshot.current().value, 1, "stale job result must not publish");
    }

    #[tokio::test]
    async fn current_job_result_publishes() {
        let (actor, snapshot) = SessionActor::spawn(TestState {
            generation: 1,
            value: 0,
        });
        let mut session = workspace::WorkspaceSession::new();
        session.complete_startup();
        let current_token = session.schedule_relink(); // generation 1, matches state

        actor.report_job_result(current_token, |s| s.value = 42);
        actor.mutate(|_| {}).await.unwrap(); // fence

        assert_eq!(snapshot.current().value, 42);
    }

    #[tokio::test]
    async fn mutate_panic_does_not_wedge_the_actor_and_leaves_snapshot_unchanged() {
        let (actor, snapshot) = SessionActor::spawn(TestState::default());
        actor.mutate(|s| s.value = 3).await.unwrap();

        let result = actor.mutate(|_s: &mut TestState| panic!("boom")).await;
        assert!(result.is_err());
        assert_eq!(
            snapshot.current().value,
            3,
            "snapshot must be unchanged after a caught panic"
        );

        // Actor is still alive: a subsequent good mutate still works.
        actor.mutate(|s| s.value = 9).await.unwrap();
        assert_eq!(snapshot.current().value, 9);
    }

    #[tokio::test]
    async fn mutate_returns_value_produced_by_apply_closure() {
        let (actor, snapshot) = SessionActor::spawn(TestState::default());
        let result = actor
            .mutate(|s| {
                s.value = 5;
                s.value
            })
            .await
            .unwrap();
        assert_eq!(result, 5);
        assert_eq!(snapshot.current().value, 5);
    }
}
