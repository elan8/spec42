//! One epoch ticker per runtime.
//!
//! Wasmtime's epoch counter is engine-global and is meant to be advanced at a regular cadence
//! by a single source, with each store deciding for itself what a tick means. That is what
//! this module provides: a [`EpochController`] owning one thread per [`GeneratorRuntime`],
//! and a per-store [`Deadline`] the store's epoch callback consults.
//!
//! ```text
//! GeneratorRuntime
//!   └── EpochController          one ticker thread
//!         ├── Store A callback   checks A's deadline and cancellation
//!         ├── Store B callback   checks B's deadline and cancellation
//!         └── Store C callback   checks C's deadline and cancellation
//! ```
//!
//! The earlier shape spawned a thread per execution and tried to arm each store so only its
//! own tick could reach it. There is no such arming: `set_epoch_deadline` is relative to the
//! current epoch and Wasmtime adds the two, so a large delta overflows rather than isolating
//! anything. Deciding per store instead removes the arithmetic entirely.
//!
//! [`GeneratorRuntime`]: crate::GeneratorRuntime

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

use wasmtime::Engine;

/// Cadence the production ticker advances the engine epoch at.
pub(crate) const TICK_INTERVAL: Duration = Duration::from_millis(10);

/// Source of "now", so tests can drive time explicitly rather than sleeping.
pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> Instant;
}

/// Wall-clock time.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

/// A clock tests move by hand. No sleeping, no scheduling guesses.
#[derive(Debug)]
pub struct ManualClock {
    start: Instant,
    advanced: Mutex<Duration>,
}

impl ManualClock {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            start: Instant::now(),
            advanced: Mutex::new(Duration::ZERO),
        })
    }

    /// Moves the clock forward. Takes effect at the next epoch tick.
    pub fn advance(&self, by: Duration) {
        *self.advanced.lock().expect("manual clock poisoned") += by;
    }
}

impl Clock for ManualClock {
    fn now(&self) -> Instant {
        self.start + *self.advanced.lock().expect("manual clock poisoned")
    }
}

impl Clock for Arc<ManualClock> {
    fn now(&self) -> Instant {
        ManualClock::now(self)
    }
}

/// What one store wants a tick to mean.
///
/// Held by both the store's epoch callback and its caller, so cancellation observed on
/// another thread is visible to the callback without any further synchronisation.
#[derive(Debug, Clone)]
pub struct Deadline {
    expires_at: Option<Instant>,
    cancelled: Arc<AtomicBool>,
}

/// Why a store decided a tick was its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expiry {
    Cancelled,
    DeadlineExceeded,
}

impl Deadline {
    pub fn new(expires_at: Option<Instant>, cancelled: Arc<AtomicBool>) -> Self {
        Self {
            expires_at,
            cancelled,
        }
    }

    /// Decides what this tick means for this store. `None` means "not mine, keep going".
    pub fn evaluate(&self, now: Instant) -> Option<Expiry> {
        if self.cancelled.load(Ordering::Acquire) {
            return Some(Expiry::Cancelled);
        }
        if self.expires_at.is_some_and(|expires| now >= expires) {
            return Some(Expiry::DeadlineExceeded);
        }
        None
    }
}

/// Advances one engine's epoch on a fixed cadence.
///
/// Lives as long as the runtime rather than as long as an execution, so concurrent
/// executions share it and the thread count does not scale with the workload.
pub struct EpochController {
    stop: Option<mpsc::Sender<()>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl EpochController {
    /// Starts a ticker for `engine`.
    ///
    /// Holds a weak engine reference: a running ticker must not be what keeps an engine alive,
    /// and dropping the last strong reference is how the thread learns to stop.
    pub fn spawn(engine: &Engine, interval: Duration) -> Self {
        let engine = engine.weak();
        let (stop_tx, stop_rx) = mpsc::channel();
        let thread = std::thread::spawn(move || loop {
            match stop_rx.recv_timeout(interval) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
            let Some(engine) = engine.upgrade() else {
                break;
            };
            engine.increment_epoch();
        });
        Self {
            stop: Some(stop_tx),
            thread: Some(thread),
        }
    }

    /// Starts a ticker at the production cadence.
    pub fn spawn_default(engine: &Engine) -> Self {
        Self::spawn(engine, TICK_INTERVAL)
    }
}

impl Drop for EpochController {
    fn drop(&mut self) {
        drop(self.stop.take());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_manual_clock_only_moves_when_told_to() {
        let clock = ManualClock::new();
        let first = clock.now();
        assert_eq!(clock.now(), first, "the clock moved on its own");
        clock.advance(Duration::from_secs(5));
        assert_eq!(clock.now(), first + Duration::from_secs(5));
    }

    #[test]
    fn a_deadline_claims_only_its_own_expiry() {
        let cancelled = Arc::new(AtomicBool::new(false));
        let now = Instant::now();

        let open = Deadline::new(None, Arc::clone(&cancelled));
        assert_eq!(
            open.evaluate(now),
            None,
            "a tick is not an open run's expiry"
        );

        let future = Deadline::new(Some(now + Duration::from_secs(1)), Arc::clone(&cancelled));
        assert_eq!(future.evaluate(now), None);
        assert_eq!(
            future.evaluate(now + Duration::from_secs(1)),
            Some(Expiry::DeadlineExceeded)
        );

        cancelled.store(true, Ordering::Release);
        assert_eq!(open.evaluate(now), Some(Expiry::Cancelled));
        // Cancellation is reported ahead of an elapsed deadline: it is the more specific
        // reason, and the caller asked for it.
        assert_eq!(
            future.evaluate(now + Duration::from_secs(10)),
            Some(Expiry::Cancelled)
        );
    }
}

/// Observes the moment a guest first calls into the host.
///
/// Entry is signalled by the host-call path itself, so a scenario waiting on it knows the
/// guest is executing rather than merely that a thread started.
#[derive(Debug, Clone, Default)]
pub struct EntryObserver(Arc<AtomicBool>);

impl EntryObserver {
    pub fn new() -> Self {
        Self::default()
    }

    /// An observer nobody is watching.
    pub fn unobserved() -> Self {
        Self::default()
    }

    pub fn has_entered(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub(crate) fn flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.0)
    }
}
