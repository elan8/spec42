use std::sync::Arc;
use tokio::sync::watch;

/// Lock-free, always-immediately-available read handle onto an actor's latest published
/// state.
///
/// Wraps a `tokio::sync::watch::Receiver<Arc<T>>`. [`current`](Self::current) never blocks
/// and never awaits — it's the "give a reader a possibly-stale snapshot, never stall it
/// behind a rebuild" half of this crate's design. [`wait_for`](Self::wait_for) is the opt-in
/// exception for the rare caller that actually wants to await a fresher value.
#[derive(Debug, Clone)]
pub struct SnapshotHandle<T> {
    rx: watch::Receiver<Arc<T>>,
}

impl<T> SnapshotHandle<T> {
    pub(crate) fn new(rx: watch::Receiver<Arc<T>>) -> Self {
        Self { rx }
    }

    /// Returns the latest published snapshot. Non-blocking: an `Arc` clone of whatever value
    /// the actor most recently published, however stale.
    pub fn current(&self) -> Arc<T> {
        self.rx.borrow().clone()
    }

    /// Awaits a value satisfying `predicate`, for callers that need to observe a specific
    /// transition rather than "whatever's there now". Returns `Err` only if the actor task
    /// has shut down (sender dropped) before the predicate is satisfied.
    pub async fn wait_for(
        &mut self,
        mut predicate: impl FnMut(&T) -> bool,
    ) -> Result<Arc<T>, watch::error::RecvError> {
        self.rx
            .wait_for(|arc| predicate(arc))
            .await
            .map(|guard| guard.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn current_returns_latest_published_value_without_blocking() {
        let (tx, rx) = watch::channel(Arc::new(0_i32));
        let handle = SnapshotHandle::new(rx);
        assert_eq!(*handle.current(), 0);
        tx.send(Arc::new(42)).unwrap();
        assert_eq!(*handle.current(), 42);
    }

    #[tokio::test]
    async fn wait_for_resolves_once_predicate_matches() {
        let (tx, rx) = watch::channel(Arc::new(0_i32));
        let mut handle = SnapshotHandle::new(rx);
        let waiter = tokio::spawn(async move { handle.wait_for(|v| *v == 7).await.map(|a| *a) });
        tx.send(Arc::new(1)).unwrap();
        tx.send(Arc::new(7)).unwrap();
        assert_eq!(waiter.await.unwrap().unwrap(), 7);
    }
}
