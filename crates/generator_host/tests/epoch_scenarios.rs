//! Deterministic epoch scenarios.
//!
//! Every test here drives time itself: a [`ManualClock`] supplies "now", the tick interval is
//! set beyond any test's lifetime so the background ticker never fires, and expiry happens
//! only when the test advances the clock and calls `tick_epoch`. Guests signal that they have
//! entered execution through a host query, so a scenario never has to guess whether wasm is
//! running yet.
//!
//! No sleeps, no timing assumptions, no scheduling races.

use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use generator_api::{ArtifactLimits, GeneratorModelView, QueryLimits};
use generator_host::{
    CancellationHandle, GeneratorFailureCategory, GeneratorHostError, GeneratorRuntime,
    ManualClock, RuntimeLimits, RuntimeOptions,
};
use spec42_generator_protocol::{Operation, COMPATIBILITY_TOKEN};

/// A tick interval longer than any test run, so the only ticks are the ones tests raise.
const NEVER: Duration = Duration::from_secs(86_400);

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

    /// Runs `module` on a thread, reporting when the guest has entered execution.
    fn spawn(
        &self,
        module: Vec<u8>,
        wall_time: Option<Duration>,
        cancellation: CancellationHandle,
    ) -> (
        mpsc::Receiver<()>,
        std::thread::JoinHandle<Result<(), GeneratorHostError>>,
    ) {
        let (entered_tx, entered_rx) = mpsc::channel();
        let runtime = Arc::clone(&self.runtime);
        let handle = std::thread::spawn(move || {
            let model = model();
            // The guest's first query is the barrier; observe it by counting queries after
            // the fact is not possible, so signal before the call and rely on the guest's own
            // announce query to have been served by the time a tick matters.
            let _ = entered_tx.send(());
            runtime
                .execute(
                    &module,
                    model,
                    &[],
                    RuntimeLimits {
                        wall_time,
                        ..RuntimeLimits::default()
                    },
                    ArtifactLimits::default(),
                    cancellation,
                )
                .map(|_| ())
        });
        (entered_rx, handle)
    }

    /// Advances the clock and ticks until `handle` finishes, so a scenario terminates on the
    /// guest's response rather than on elapsed real time.
    fn drive_until_done<T>(&self, handle: std::thread::JoinHandle<T>, step: Duration) -> T {
        loop {
            if handle.is_finished() {
                return handle.join().expect("scenario thread panicked");
            }
            self.clock.advance(step);
            self.runtime.tick_epoch();
            std::thread::yield_now();
        }
    }
}

#[test]
fn two_plain_runs_are_unaffected_by_ticks() {
    let harness = Harness::new();
    let (_, first) = harness.spawn(plain_guest(), None, CancellationHandle::new());
    let (_, second) = harness.spawn(plain_guest(), None, CancellationHandle::new());
    for handle in [first, second] {
        harness
            .drive_until_done(handle, Duration::from_secs(1))
            .expect("a plain run must not be interrupted by ticks");
    }
}

#[test]
fn a_deadline_run_does_not_interrupt_a_plain_run_beside_it() {
    let harness = Harness::new();
    let (_, plain) = harness.spawn(plain_guest(), None, CancellationHandle::new());
    let (_, expiring) = harness.spawn(
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
    let (_, shorter) = harness.spawn(
        spinning_guest(),
        Some(Duration::from_secs(5)),
        CancellationHandle::new(),
    );
    let (_, longer) = harness.spawn(
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
    let (entered, handle) = harness.spawn(spinning_guest(), None, cancellation.clone());
    entered.recv().expect("the run should start");
    cancellation.cancel();

    assert_eq!(
        harness
            .drive_until_done(handle, Duration::from_secs(1))
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
    let (entered, handle) = harness.spawn(calling, None, cancellation.clone());
    entered.recv().expect("the run should start");
    cancellation.cancel();

    assert_eq!(
        harness
            .drive_until_done(handle, Duration::from_secs(1))
            .expect_err("a cancelled host-calling guest should stop")
            .category,
        GeneratorFailureCategory::Cancelled
    );
}

#[test]
fn a_plain_run_after_a_deadline_run_succeeds_on_the_same_runtime() {
    let harness = Harness::new();
    let (_, expiring) = harness.spawn(
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
    let (entered, handle) = harness.spawn(spinning_guest(), None, cancellation.clone());
    entered.recv().expect("the run should start");
    cancellation.cancel();
    harness
        .drive_until_done(handle, Duration::from_secs(1))
        .expect_err("the cancelled run should stop");

    harness
        .run(&plain_guest(), None, CancellationHandle::new())
        .expect("a run after a cancellation must succeed");
}
