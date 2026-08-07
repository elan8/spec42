//! Deterministic epoch scenarios.
//!
//! Every test here drives time itself: a [`ManualClock`] supplies "now", the tick interval is
//! set beyond any test's lifetime so the background ticker never fires, and expiry happens
//! only when the test advances the clock and calls `tick_epoch`. Guests signal that they have
//! entered execution through a host query, so a scenario never has to guess whether wasm is
//! running yet.
//!
//! No sleeps, no timing assumptions, no scheduling races.

use std::sync::Arc;
use std::time::Duration;

use generator_api::{ArtifactLimits, GeneratorModelView, QueryLimits};
use generator_host::{
    CancellationHandle, EntryObserver, GeneratorFailureCategory, GeneratorHostError,
    GeneratorRuntime, ManualClock, RuntimeLimits, RuntimeOptions,
};
use spec42_generator_protocol::{Operation, COMPATIBILITY_TOKEN};

/// A tick interval longer than any test run, so the only ticks are the ones tests raise.
const NEVER: Duration = Duration::from_secs(86_400);

/// Upper bound on how long a scenario will drive before declaring the interrupt broken.
/// Generous enough to absorb scheduling, small enough to fail rather than hang.
const MAX_TICKS: usize = 20_000;

/// A running scenario and the observer reporting when its guest entered WebAssembly.
struct Running {
    observer: EntryObserver,
    handle: std::thread::JoinHandle<Result<(), GeneratorHostError>>,
}

/// Postcard encoding of `Ok::<Vec<Artifact>, String>(vec![])`.
const EMPTY_RESULT: &str = "\\00\\00";
const RESULT_PTR: u32 = 1024;
const RESULT_LEN: u64 = 2;

/// Builds a guest whose body is `generate_body`.
fn guest(generate_body: &str) -> Vec<u8> {
    let packed = (RESULT_LEN << 32) | u64::from(RESULT_PTR);
    wat::parse_str(
        format!(
            r#"(module
  (import "spec42" "query" (func $query (param i32 i32 i32 i32 i32) (result i64)))
  (memory (export "memory") 1)
  (data (i32.const {RESULT_PTR}) "{EMPTY_RESULT}")
  (func (export "spec42_abi_version") (result i64) (i64.const {COMPATIBILITY_TOKEN}))
  (func (export "spec42_alloc") (param i32) (result i32) (i32.const 2048))
  (func (export "spec42_generate") (param i32 i32) (result i64)
    {generate_body}
    (i64.const {packed})))"#
        )
        .as_str(),
    )
    .expect("fixture assembles")
}

/// Returns immediately.
fn plain_guest() -> Vec<u8> {
    guest("")
}

/// Announces entry with one query, then spins until a tick interrupts it.
///
/// The query is the barrier: once the host has served it, wasm is definitely executing, so a
/// scenario can advance the clock knowing the guest is in the loop.
fn spinning_guest() -> Vec<u8> {
    let announce = format!(
        "(drop (call $query (i32.const {}) (i32.const 0) (i32.const 0) (i32.const 4096) (i32.const 4096)))",
        Operation::Roots.code()
    );
    guest(&format!("{announce}\n    (loop $forever (br $forever))"))
}

fn model() -> Arc<GeneratorModelView> {
    use std::fs;
    use workspace::{
        EngineBuilder, HostContext, HostFilesystemProvider, ValidationTiming, WorkspaceLoadRequest,
    };

    let temp = Box::leak(Box::new(tempfile::tempdir().unwrap()));
    let path = temp.path().join("model.sysml");
    fs::write(&path, "package P { part def Widget; }\n").unwrap();
    let engine = EngineBuilder::default()
        .cache_dir(temp.path().join("cache"))
        .no_stdlib(true)
        .build()
        .unwrap();
    let provider =
        HostFilesystemProvider::from_paths(&path, Some(temp.path()), engine.package_roots());
    let request = WorkspaceLoadRequest::single_target(path)
        .with_workspace_root(Some(temp.path().to_path_buf()))
        .with_validation_timing(ValidationTiming::Deferred);
    Arc::new(GeneratorModelView::new(
        engine
            .load_workspace(provider, request, HostContext::default())
            .unwrap(),
        QueryLimits::default(),
    ))
}

struct Harness {
    runtime: Arc<GeneratorRuntime>,
    clock: Arc<ManualClock>,
}

impl Harness {
    fn new() -> Self {
        let clock = ManualClock::new();
        let runtime = GeneratorRuntime::with_options_and_clock(
            RuntimeOptions::default(),
            Arc::new(Arc::clone(&clock)),
            NEVER,
        )
        .expect("runtime");
        Self {
            runtime: Arc::new(runtime),
            clock,
        }
    }

    fn run(
        &self,
        module: &[u8],
        wall_time: Option<Duration>,
        cancellation: CancellationHandle,
    ) -> Result<(), GeneratorHostError> {
        self.runtime
            .execute(
                module,
                model(),
                &[],
                RuntimeLimits {
                    wall_time,
                    ..RuntimeLimits::default()
                },
                ArtifactLimits::default(),
                cancellation,
            )
            .map(|_| ())
    }

    /// Runs `module` on a thread, returning an observer that reports genuine guest entry.
    ///
    /// Entry is signalled by the host-call path when the guest issues its announce query, so
    /// waiting on it establishes that WebAssembly is executing. The previous version sent its
    /// "entered" signal *before* calling `execute`, which a pre-cancelled handle satisfies
    /// without the guest ever running -- so the cancellation scenarios could pass while
    /// proving nothing.
    fn spawn(
        &self,
        module: Vec<u8>,
        wall_time: Option<Duration>,
        cancellation: CancellationHandle,
    ) -> Running {
        let observer = EntryObserver::new();
        let runtime = Arc::clone(&self.runtime);
        let thread_observer = observer.clone();
        let handle = std::thread::spawn(move || {
            let prepared = runtime.prepare(&module)?;
            runtime
                .execute_prepared_observed(
                    &prepared,
                    model(),
                    &[],
                    RuntimeLimits {
                        wall_time,
                        ..RuntimeLimits::default()
                    },
                    ArtifactLimits::default(),
                    cancellation,
                    &thread_observer,
                )
                .map(|_| ())
        });
        Running { observer, handle }
    }

    /// Ticks until the guest has genuinely entered WebAssembly.
    ///
    /// Bounded: a guest that never enters fails the scenario rather than hanging CI.
    fn await_entry(&self, running: &Running) {
        for _ in 0..MAX_TICKS {
            if running.observer.has_entered() {
                return;
            }
            assert!(
                !running.handle.is_finished(),
                "the run finished before the guest entered WebAssembly"
            );
            self.runtime.tick_epoch();
            std::thread::yield_now();
        }
        panic!("the guest never entered WebAssembly within {MAX_TICKS} ticks");
    }

    /// Advances the clock and ticks until the run finishes.
    ///
    /// Bounded for the same reason: a broken interrupt should fail the scenario, not stall
    /// the job. On timeout the wedged thread is left detached deliberately -- joining it
    /// would reintroduce the hang this bound exists to prevent.
    fn drive_until_done(&self, running: Running, step: Duration) -> Result<(), GeneratorHostError> {
        for _ in 0..MAX_TICKS {
            if running.handle.is_finished() {
                return running.handle.join().expect("scenario thread panicked");
            }
            self.clock.advance(step);
            self.runtime.tick_epoch();
            std::thread::yield_now();
        }
        panic!("the run did not finish within {MAX_TICKS} ticks; the interrupt did not land");
    }
}

#[test]
fn two_plain_runs_are_unaffected_by_ticks() {
    let harness = Harness::new();
    let first = harness.spawn(plain_guest(), None, CancellationHandle::new());
    let second = harness.spawn(plain_guest(), None, CancellationHandle::new());
    for running in [first, second] {
        harness
            .drive_until_done(running, Duration::from_secs(1))
            .expect("a plain run must not be interrupted by ticks");
    }
}

#[test]
fn a_deadline_run_does_not_interrupt_a_plain_run_beside_it() {
    let harness = Harness::new();
    let plain = harness.spawn(plain_guest(), None, CancellationHandle::new());
    let expiring = harness.spawn(
        spinning_guest(),
        Some(Duration::from_secs(30)),
        CancellationHandle::new(),
    );

    let expired = harness.drive_until_done(expiring, Duration::from_secs(10));
    assert_eq!(
        expired
            .expect_err("the spinning guest should time out")
            .category,
        GeneratorFailureCategory::ResourceExhausted
    );
    harness
        .drive_until_done(plain, Duration::from_secs(1))
        .expect("the plain run must survive its neighbour's deadline");
}

#[test]
fn two_unequal_deadlines_expire_independently() {
    let harness = Harness::new();
    let shorter = harness.spawn(
        spinning_guest(),
        Some(Duration::from_secs(5)),
        CancellationHandle::new(),
    );
    let longer = harness.spawn(
        spinning_guest(),
        Some(Duration::from_secs(1_000)),
        CancellationHandle::new(),
    );

    // One second per tick: the shorter deadline must expire while the longer one runs on.
    assert_eq!(
        harness
            .drive_until_done(shorter, Duration::from_secs(1))
            .expect_err("the shorter deadline should expire")
            .category,
        GeneratorFailureCategory::ResourceExhausted
    );
    assert_eq!(
        harness
            .drive_until_done(longer, Duration::from_secs(100))
            .expect_err("the longer deadline should expire once time reaches it")
            .category,
        GeneratorFailureCategory::ResourceExhausted
    );
}

#[test]
fn a_compute_bound_guest_observes_cancellation_without_a_deadline() {
    let harness = Harness::new();
    let cancellation = CancellationHandle::new();
    let running = harness.spawn(spinning_guest(), None, cancellation.clone());
    // Wait for the guest's announce query: entry is reported by the host-call path, so this
    // establishes that WebAssembly is executing and not merely that a thread was spawned.
    harness.await_entry(&running);
    cancellation.cancel();

    assert_eq!(
        harness
            .drive_until_done(running, Duration::from_secs(1))
            .expect_err("a cancelled compute loop should stop")
            .category,
        GeneratorFailureCategory::Cancelled
    );
}

#[test]
fn a_host_calling_guest_observes_cancellation() {
    // Loops around a host call, so cancellation can be seen either by the epoch callback or
    // by the host-call guard; both report the same category.
    let calling = guest(&format!(
        "(loop $forever (drop (call $query (i32.const {}) (i32.const 0) (i32.const 0) \
         (i32.const 4096) (i32.const 4096))) (br $forever))",
        Operation::Roots.code()
    ));
    let harness = Harness::new();
    let cancellation = CancellationHandle::new();
    let running = harness.spawn(calling, None, cancellation.clone());
    harness.await_entry(&running);
    cancellation.cancel();

    assert_eq!(
        harness
            .drive_until_done(running, Duration::from_secs(1))
            .expect_err("a cancelled host-calling guest should stop")
            .category,
        GeneratorFailureCategory::Cancelled
    );
}

#[test]
fn a_plain_run_after_a_deadline_run_succeeds_on_the_same_runtime() {
    let harness = Harness::new();
    let expiring = harness.spawn(
        spinning_guest(),
        Some(Duration::from_secs(5)),
        CancellationHandle::new(),
    );
    assert_eq!(
        harness
            .drive_until_done(expiring, Duration::from_secs(1))
            .expect_err("the deadline should expire")
            .category,
        GeneratorFailureCategory::ResourceExhausted
    );

    // The epoch has advanced; later runs on this runtime must be unaffected. This is the
    // regression for the overflow that arming with a sentinel delta produced.
    for attempt in 0..3 {
        harness
            .run(&plain_guest(), None, CancellationHandle::new())
            .unwrap_or_else(|error| panic!("run {attempt} after a deadline failed: {error}"));
    }
}

#[test]
fn runs_continue_to_work_after_a_cancellation() {
    let harness = Harness::new();
    let cancellation = CancellationHandle::new();
    let running = harness.spawn(spinning_guest(), None, cancellation.clone());
    // Wait for the guest's announce query: entry is reported by the host-call path, so this
    // establishes that WebAssembly is executing and not merely that a thread was spawned.
    harness.await_entry(&running);
    cancellation.cancel();
    harness
        .drive_until_done(running, Duration::from_secs(1))
        .expect_err("the cancelled run should stop");

    harness
        .run(&plain_guest(), None, CancellationHandle::new())
        .expect("a run after a cancellation must succeed");
}
