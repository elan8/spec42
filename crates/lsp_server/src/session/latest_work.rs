//! One active job and one replaceable pending complete input state.
//!
//! Callers ingest ordered changes before submitting their resulting immutable inputs. Replacing
//! a pending value coalesces work, never source deltas. A completion must carry the token minted
//! by this queue for that active interval; old or foreign completions cannot advance it.

use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct WorkToken(Arc<()>);

pub(crate) struct Work<T> {
    token: WorkToken,
    value: T,
}

impl<T> Work<T> {
    pub(crate) fn into_parts(self) -> (WorkToken, T) {
        (self.token, self.value)
    }
}

pub(crate) enum Submission<T> {
    Start(Work<T>),
    Queued,
    Closed,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StaleCompletion;

pub(crate) struct LatestWork<T> {
    active: Option<WorkToken>,
    pending: Option<T>,
    closed: bool,
}

impl<T> Default for LatestWork<T> {
    fn default() -> Self {
        Self {
            active: None,
            pending: None,
            closed: false,
        }
    }
}

impl<T> LatestWork<T> {
    pub(crate) fn submit(&mut self, value: T) -> Submission<T> {
        if self.closed {
            Submission::Closed
        } else if self.active.is_none() {
            Submission::Start(self.start(value))
        } else {
            self.pending = Some(value);
            Submission::Queued
        }
    }

    pub(crate) fn complete(
        &mut self,
        token: &WorkToken,
    ) -> Result<Option<Work<T>>, StaleCompletion> {
        if !self
            .active
            .as_ref()
            .is_some_and(|active| Arc::ptr_eq(&active.0, &token.0))
        {
            return Err(StaleCompletion);
        }
        Ok(match self.pending.take() {
            Some(value) => Some(self.start(value)),
            None => {
                self.active = None;
                None
            }
        })
    }

    pub(crate) fn occupancy(&self) -> (bool, bool) {
        (self.active.is_some(), self.pending.is_some())
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.closed
    }

    /// Stop accepting work and release pending inputs. The caller cancels the active worker
    /// through its owning lifecycle; its eventual completion can still release this interval.
    pub(crate) fn close(&mut self) {
        self.closed = true;
        self.pending = None;
    }

    fn start(&mut self, value: T) -> Work<T> {
        let token = WorkToken(Arc::new(()));
        self.active = Some(token.clone());
        Work { token, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn start<T>(submission: Submission<T>) -> Work<T> {
        let Submission::Start(work) = submission else {
            panic!("expected idle queue to start work")
        };
        work
    }

    #[test]
    fn a_burst_retains_one_active_and_only_the_latest_complete_input_state() {
        let mut queue = LatestWork::default();
        let (active, initial) = start(queue.submit(Vec::<u32>::new())).into_parts();
        assert!(initial.is_empty());
        let mut desired = Vec::new();
        for edit in 0..100 {
            desired.push(edit);
            assert!(matches!(queue.submit(desired.clone()), Submission::Queued));
        }
        let (latest, inputs) = queue.complete(&active).unwrap().unwrap().into_parts();
        assert_eq!(inputs, desired);
        assert!(queue.complete(&latest).unwrap().is_none());
        assert!(matches!(queue.submit(vec![101]), Submission::Start(_)));
    }

    #[test]
    fn replaced_pending_inputs_are_dropped_instead_of_accumulating() {
        struct Input(Arc<AtomicUsize>);
        impl Drop for Input {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let dropped = Arc::new(AtomicUsize::new(0));
        let mut queue = LatestWork::default();
        let (active, active_input) = start(queue.submit(Input(dropped.clone()))).into_parts();
        for _ in 0..10_000 {
            assert!(matches!(
                queue.submit(Input(dropped.clone())),
                Submission::Queued
            ));
        }
        assert_eq!(dropped.load(Ordering::SeqCst), 9_999);
        let (latest, latest_input) = queue.complete(&active).unwrap().unwrap().into_parts();
        assert!(queue.complete(&latest).unwrap().is_none());
        drop((active_input, latest_input));
        assert_eq!(dropped.load(Ordering::SeqCst), 10_001);
    }

    #[test]
    fn closing_discards_pending_inputs_and_rejects_new_work() {
        let mut queue = LatestWork::default();
        let (active, _) = start(queue.submit(1)).into_parts();
        assert!(matches!(queue.submit(2), Submission::Queued));
        queue.close();
        assert!(matches!(queue.submit(3), Submission::Closed));
        assert!(queue.complete(&active).unwrap().is_none());
        assert!(matches!(queue.complete(&active), Err(StaleCompletion)));
    }

    #[test]
    fn old_or_foreign_completions_cannot_discard_active_or_pending_work() {
        let mut queue = LatestWork::default();
        let (older, _) = start(queue.submit(1)).into_parts();
        assert!(matches!(queue.submit(2), Submission::Queued));
        let (current, value) = queue.complete(&older).unwrap().unwrap().into_parts();
        assert_eq!(value, 2);
        assert!(matches!(queue.submit(3), Submission::Queued));
        assert!(matches!(queue.complete(&older), Err(StaleCompletion)));
        let mut foreign = LatestWork::default();
        let (foreign_token, _) = start(foreign.submit(99)).into_parts();
        assert!(matches!(
            queue.complete(&foreign_token),
            Err(StaleCompletion)
        ));
        let (latest, value) = queue.complete(&current).unwrap().unwrap().into_parts();
        assert_eq!(value, 3);
        assert!(queue.complete(&latest).unwrap().is_none());
        assert!(foreign.complete(&foreign_token).unwrap().is_none());
    }
}
