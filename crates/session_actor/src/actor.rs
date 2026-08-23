use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, watch};

/// Implemented by embedder session state to let [`SessionActor::report_job_result`] check
/// whether a background job's token is still current without this crate knowing anything
/// about `M`'s internal layout or what its token is.
pub trait TracksRelink {
    /// The embedder's supersession token.
    type Token: Send + 'static;

    fn is_token_current(&self, token: &Self::Token) -> bool;

    /// Establishes a fresh owner boundary when this state seeds a new actor.
    fn rekey_for_actor(&mut self);
}

/// Error returned to a [`SessionActor::mutate`] caller when the supplied closure panicked.
/// The actor itself survives — this only reports that this one mutation did not apply and the
/// published snapshot is unchanged from before the call.
#[derive(Debug, thiserror::Error)]
#[error("mutate closure panicked; session state left unchanged")]
pub struct MutatePanicked;

type BoxedAny = Box<dyn std::any::Any + Send>;
type BoxedApply<M> = Box<dyn FnOnce(&mut M) -> Mutation<BoxedAny> + Send>;

/// Declares whether a synchronous actor mutation changed the coherent state that readers see.
///
/// Use [`Unchanged`](Self::Unchanged) for content-equivalent updates. The actor still returns
/// the closure's value, but retains the existing `Arc` and does not notify readers. This keeps
/// harmless editor echoes from invalidating work that depends on the current publication.
pub enum Mutation<R> {
    Changed(R),
    Unchanged(R),
}

/// Value returned by [`SessionActor::mutate_if_changed`].
#[derive(Debug)]
pub struct MutationOutcome<R> {
    pub value: R,
    pub published: bool,
}

enum Command<M: TracksRelink> {
    Mutate {
        apply: BoxedApply<M>,
        reply: oneshot::Sender<Result<(BoxedAny, bool), MutatePanicked>>,
    },
    JobResult {
        token: <M as TracksRelink>::Token,
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
pub struct SessionActor<M: TracksRelink> {
    tx: mpsc::UnboundedSender<Command<M>>,
}

impl<M: Clone + Send + Sync + TracksRelink + 'static> SessionActor<M> {
    /// Spawns the actor task and returns a handle to control it plus a snapshot handle for
    /// reading its published state.
    pub fn spawn(mut initial: M) -> (Self, crate::SnapshotHandle<M>) {
        initial.rekey_for_actor();
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
                            Ok(Mutation::Changed(boxed)) => {
                                let _ = watch_tx.send(state.clone());
                                let _ = reply.send(Ok((boxed, true)));
                            }
                            Ok(Mutation::Unchanged(boxed)) => {
                                // A no-op must retain the exact prior snapshot. `Arc::make_mut`
                                // may have detached it before the closure recognized equal
                                // content, so restore the last publication explicitly.
                                state = watch_tx.borrow().clone();
                                let _ = reply.send(Ok((boxed, false)));
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
                            // semantics as a superseded relink: nothing is applied.
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
            Box::new(move |state: &mut M| Mutation::Changed(Box::new(apply(state)) as BoxedAny));
        let _ = self.tx.send(Command::Mutate {
            apply: boxed_apply,
            reply: reply_tx,
        });
        match reply_rx.await.unwrap_or(Err(MutatePanicked)) {
            Ok((boxed, _published)) => Ok(*boxed
                .downcast::<R>()
                .expect("R matches the closure's own return type by construction")),
            Err(MutatePanicked) => Err(MutatePanicked),
        }
    }

    /// Applies a mutation that may be content-equivalent. An [`Mutation::Unchanged`] result is
    /// not published and does not replace the current snapshot; [`Mutation::Changed`] is
    /// published atomically before this method returns.
    pub async fn mutate_if_changed<R: Send + 'static>(
        &self,
        apply: impl FnOnce(&mut M) -> Mutation<R> + Send + 'static,
    ) -> Result<MutationOutcome<R>, MutatePanicked> {
        let (reply_tx, reply_rx) = oneshot::channel();
        let boxed_apply: BoxedApply<M> = Box::new(move |state: &mut M| match apply(state) {
            Mutation::Changed(value) => Mutation::Changed(Box::new(value) as BoxedAny),
            Mutation::Unchanged(value) => Mutation::Unchanged(Box::new(value) as BoxedAny),
        });
        let _ = self.tx.send(Command::Mutate {
            apply: boxed_apply,
            reply: reply_tx,
        });
        match reply_rx.await.unwrap_or(Err(MutatePanicked)) {
            Ok((boxed, published)) => Ok(MutationOutcome {
                value: *boxed
                    .downcast::<R>()
                    .expect("R matches the closure's own return type by construction"),
                published,
            }),
            Err(MutatePanicked) => Err(MutatePanicked),
        }
    }

    /// Fire-and-forget: hands back the result of a rebuild computed off the actor (typically
    /// via `tokio::task::spawn_blocking`). Merged in only if `token` is still current;
    /// otherwise dropped silently.
    pub fn report_job_result(&self, token: M::Token, merge: impl FnOnce(&mut M) + Send + 'static) {
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
        type Token = u64;

        fn is_token_current(&self, token: &u64) -> bool {
            *token == self.generation
        }

        fn rekey_for_actor(&mut self) {}
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

        // The state expects generation 2, so a generation-1 token is stale.
        actor.report_job_result(1, |s| s.value = 999);
        // Fence: an empty mutate only resolves after the mailbox (FIFO) has drained the
        // preceding report_job_result command, so this deterministically waits for it.
        actor.mutate(|_| {}).await.unwrap();

        assert_eq!(
            snapshot.current().value,
            1,
            "stale job result must not publish"
        );
    }

    #[tokio::test]
    async fn current_job_result_publishes() {
        let (actor, snapshot) = SessionActor::spawn(TestState {
            generation: 1,
            value: 0,
        });
        actor.report_job_result(1, |s| s.value = 42); // generation 1, matches state
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

    #[tokio::test]
    async fn unchanged_mutation_retains_the_published_snapshot() {
        let (actor, snapshot) = SessionActor::spawn(TestState {
            generation: 1,
            value: 5,
        });
        let before = snapshot.current();

        let outcome = actor
            .mutate_if_changed(|state| {
                assert_eq!(state.value, 5);
                Mutation::Unchanged("same content")
            })
            .await
            .unwrap();

        assert_eq!(outcome.value, "same content");
        assert!(!outcome.published);
        assert!(Arc::ptr_eq(&before, &snapshot.current()));
    }

    #[tokio::test]
    async fn changed_mutation_publishes_a_new_coherent_snapshot() {
        let (actor, snapshot) = SessionActor::spawn(TestState {
            generation: 1,
            value: 5,
        });
        let before = snapshot.current();

        let outcome = actor
            .mutate_if_changed(|state| {
                state.value = 8;
                Mutation::Changed(())
            })
            .await
            .unwrap();

        assert!(outcome.published);
        let after = snapshot.current();
        assert_eq!(after.value, 8);
        assert!(!Arc::ptr_eq(&before, &after));
    }
}
