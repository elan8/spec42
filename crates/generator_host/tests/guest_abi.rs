//! End-to-end checks of the guest ABI against real WebAssembly modules.
//!
//! These are written as WAT rather than built with the Rust SDK on purpose: the SDK always
//! emits a correct module, so the failure modes the host must reject — a stale schema, a bad
//! pointer, an unknown operation — are unreachable through it.

use std::sync::Arc;

use generator_api::{ArtifactLimits, GeneratorModelView, QueryLimits};
use generator_host::{
    CancellationHandle, GeneratorFailureCategory, GeneratorHostError, GeneratorRuntime,
    RuntimeLimits,
};
use spec42_generator_protocol::{Operation, COMPATIBILITY_TOKEN};

/// Postcard encoding of `Ok::<Vec<Artifact>, String>(vec![])`: variant 0, then length 0.
const EMPTY_RESULT: &str = "\\00\\00";
const RESULT_PTR: u32 = 1024;
const RESULT_LEN: u64 = 2;

fn guest(fingerprint: u64, generate_body: &str, extra: &str) -> Vec<u8> {
    let packed = (RESULT_LEN << 32) | u64::from(RESULT_PTR);
    let source = format!(
        r#"(module
  (import "spec42" "query" (func $query (param i32 i32 i32 i32 i32) (result i64)))
  (import "spec42" "diagnostic" (func $diagnostic (param i32 i32 i32 i32 i32)))
  (memory (export "memory") 1)
  (data (i32.const {RESULT_PTR}) "{EMPTY_RESULT}")
  (func (export "spec42_abi_version") (result i64) (i64.const {fingerprint}))
  (func (export "spec42_alloc") (param i32) (result i32) (i32.const 2048))
  (func (export "spec42_generate") (param i32 i32) (result i64)
    {generate_body}
    (i64.const {packed}))
  {extra}
)"#
    );
    wat::parse_str(&source).expect("fixture should assemble")
}

/// A guest that succeeds, returning no artifacts.
fn conforming_guest() -> Vec<u8> {
    guest(COMPATIBILITY_TOKEN, "", "")
}

fn model() -> Arc<GeneratorModelView> {
    use std::fs;
    use workspace::{
        EngineBuilder, HostContext, HostFilesystemProvider, ValidationTiming, WorkspaceLoadRequest,
    };

    // Leaked deliberately: the snapshot borrows nothing, but the temp dir must outlive it and
    // these are short-lived test processes.
    let temp = Box::leak(Box::new(tempfile::tempdir().unwrap()));
    let path = temp.path().join("model.sysml");
    fs::write(&path, "package P { part def Widget { attribute mass; } }\n").unwrap();
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
    let snapshot = engine
        .load_workspace(provider, request, HostContext::default())
        .unwrap();
    Arc::new(GeneratorModelView::new(snapshot, QueryLimits::default()))
}

fn run(module: &[u8]) -> Result<generator_host::GeneratorExecution, GeneratorHostError> {
    let runtime = GeneratorRuntime::new().expect("runtime");
    runtime.execute(
        module,
        model(),
        &[],
        RuntimeLimits::default(),
        ArtifactLimits::default(),
        CancellationHandle::new(),
    )
}

#[test]
fn a_conforming_guest_runs_and_returns_no_artifacts() {
    let execution = run(&conforming_guest()).expect("conforming guest should run");
    assert_eq!(execution.artifacts.len(), 0);
    assert_eq!(execution.query_count, 0);
    // Unmetered by default, so there is no fuel figure to report.
    assert_eq!(execution.fuel_consumed, None);
}

#[test]
fn a_guest_built_against_an_incompatible_abi_is_refused_before_it_runs() {
    let stale = guest(COMPATIBILITY_TOKEN ^ 0xFFFF, "", "");
    let error = run(&stale).expect_err("schema mismatch should be refused");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
    assert!(
        error.message.contains("incompatible Spec42 generator ABI"),
        "unexpected message: {}",
        error.message
    );
}

#[test]
fn a_module_without_the_version_export_is_refused_at_prepare() {
    let source = r#"(module
  (memory (export "memory") 1)
  (func (export "spec42_alloc") (param i32) (result i32) (i32.const 0))
  (func (export "spec42_generate") (param i32 i32) (result i64) (i64.const 0)))"#;
    let runtime = GeneratorRuntime::new().expect("runtime");
    let error = runtime
        .prepare(&wat::parse_str(source).unwrap())
        .expect_err("missing version export should be refused");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
    assert_eq!(error.message, "module does not export `spec42_abi_version`");
}

#[test]
fn wasi_imports_do_not_link() {
    let source = format!(
        r#"(module
  (import "wasi_snapshot_preview1" "fd_write"
    (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "spec42_abi_version") (result i64) (i64.const {COMPATIBILITY_TOKEN}))
  (func (export "spec42_alloc") (param i32) (result i32) (i32.const 0))
  (func (export "spec42_generate") (param i32 i32) (result i64) (i64.const 0)))"#
    );
    let runtime = GeneratorRuntime::new().expect("runtime");
    let error = runtime
        .prepare(&wat::parse_str(&source).unwrap())
        .expect_err("a WASI import must not link");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
}

#[test]
fn an_unknown_query_operation_is_an_abi_violation_naming_the_operation() {
    let body = "(drop (call $query (i32.const 99) (i32.const 0) (i32.const 0) \
                (i32.const 4096) (i32.const 64)))";
    let error = run(&guest(COMPATIBILITY_TOKEN, body, "")).expect_err("unknown operation");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
    assert_eq!(error.message, "unknown Spec42 query operation 99");
}

#[test]
fn an_out_of_bounds_query_pointer_is_an_abi_violation() {
    let body = format!(
        "(drop (call $query (i32.const {}) (i32.const 999999) (i32.const 16) \
         (i32.const 4096) (i32.const 64)))",
        Operation::Find.code()
    );
    let error = run(&guest(COMPATIBILITY_TOKEN, &body, "")).expect_err("out of bounds read");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
}

#[test]
fn an_oversized_transfer_is_refused_by_the_abi_limit() {
    let body = format!(
        "(drop (call $query (i32.const {}) (i32.const 0) (i32.const 2000000000) \
         (i32.const 4096) (i32.const 64)))",
        Operation::Find.code()
    );
    let error = run(&guest(COMPATIBILITY_TOKEN, &body, "")).expect_err("oversized transfer");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
    assert_eq!(error.message, "guest transfer exceeds the ABI limit");
}

#[test]
fn a_result_pointer_outside_guest_memory_is_refused_without_allocating() {
    // Claims a 64 MiB result living past the end of a one-page memory.
    let packed = (64u64 * 1024 * 1024) << 32;
    let source = format!(
        r#"(module
  (memory (export "memory") 1)
  (func (export "spec42_abi_version") (result i64) (i64.const {COMPATIBILITY_TOKEN}))
  (func (export "spec42_alloc") (param i32) (result i32) (i32.const 2048))
  (func (export "spec42_generate") (param i32 i32) (result i64) (i64.const {packed})))"#
    );
    let error = run(&wat::parse_str(&source).unwrap()).expect_err("result outside memory");
    assert_eq!(error.category, GeneratorFailureCategory::ApiIncompatible);
    assert_eq!(error.message, "generator result lies outside guest memory");
}

#[test]
fn a_trapping_guest_is_reported_as_a_trap_with_its_reason() {
    let error = run(&guest(COMPATIBILITY_TOKEN, "(unreachable)", "")).expect_err("guest trap");
    assert_eq!(error.category, GeneratorFailureCategory::Trap);
    assert!(
        error.message.contains("unreachable"),
        "trap reason was lost: {}",
        error.message
    );
}

#[test]
fn fuel_exhaustion_is_resource_exhaustion_when_a_budget_is_requested() {
    use generator_host::RuntimeOptions;

    // A guest that spins forever; only the fuel budget can stop it.
    let spin = guest(COMPATIBILITY_TOKEN, "(loop $forever (br $forever))", "");
    let runtime = GeneratorRuntime::with_options(RuntimeOptions {
        fuel_metering: true,
    })
    .expect("runtime");
    let error = runtime
        .execute(
            &spin,
            model(),
            &[],
            RuntimeLimits {
                fuel: Some(100_000),
                ..RuntimeLimits::default()
            },
            ArtifactLimits::default(),
            CancellationHandle::new(),
        )
        .expect_err("the spin loop should exhaust its fuel");
    assert_eq!(error.category, GeneratorFailureCategory::ResourceExhausted);
}

#[test]
fn a_fuel_budget_without_metering_is_rejected_rather_than_ignored() {
    let runtime = GeneratorRuntime::new().expect("runtime");
    let error = runtime
        .execute(
            &conforming_guest(),
            model(),
            &[],
            RuntimeLimits {
                fuel: Some(1_000),
                ..RuntimeLimits::default()
            },
            ArtifactLimits::default(),
            CancellationHandle::new(),
        )
        .expect_err("a silently ignored budget would be worse than an error");
    assert_eq!(error.category, GeneratorFailureCategory::ResourceExhausted);
    assert!(error.message.contains("fuel metering"));
}

#[test]
fn cancellation_before_execution_is_reported_as_cancelled() {
    let runtime = GeneratorRuntime::new().expect("runtime");
    let cancellation = CancellationHandle::new();
    cancellation.cancel();
    let error = runtime
        .execute(
            &conforming_guest(),
            model(),
            &[],
            RuntimeLimits::default(),
            ArtifactLimits::default(),
            cancellation,
        )
        .expect_err("cancelled run");
    assert_eq!(error.category, GeneratorFailureCategory::Cancelled);
}

/// A timed-out execution must not disturb others sharing the runtime.
///
/// Epoch ticks are engine-global, so this used to be a rule callers had to follow. Arming
/// non-deadline stores with a huge delta does not work either: `set_epoch_deadline` is
/// relative to the current epoch and Wasmtime adds the two, so `u64::MAX` panics in debug and
/// wraps to an already-expired value in release once the epoch has advanced at all.
#[test]
fn a_timed_out_execution_does_not_affect_later_runs_on_the_same_runtime() {
    use generator_host::RuntimeOptions;

    let runtime = GeneratorRuntime::with_options(RuntimeOptions::default()).expect("runtime");
    let spin = runtime
        .prepare(&guest(
            COMPATIBILITY_TOKEN,
            "(loop $forever (br $forever))",
            "",
        ))
        .expect("spin prepares");
    let conforming = runtime
        .prepare(&conforming_guest())
        .expect("conforming prepares");

    // Time one out, which advances the engine epoch.
    let timed_out = runtime.execute_prepared(
        &spin,
        model(),
        &[],
        RuntimeLimits {
            wall_time: Some(std::time::Duration::from_millis(50)),
            ..RuntimeLimits::default()
        },
        ArtifactLimits::default(),
        CancellationHandle::new(),
    );
    assert_eq!(
        timed_out
            .expect_err("the spin loop should time out")
            .category,
        GeneratorFailureCategory::ResourceExhausted
    );

    // Every later run on this runtime must be unaffected, repeatedly.
    for attempt in 0..3 {
        runtime
            .execute_prepared(
                &conforming,
                model(),
                &[],
                RuntimeLimits::default(),
                ArtifactLimits::default(),
                CancellationHandle::new(),
            )
            .unwrap_or_else(|error| {
                panic!("run {attempt} after a timeout failed: {error}");
            });
    }
}

/// The same, concurrently: one execution times out while others are mid-flight.
#[test]
fn a_concurrent_timeout_does_not_interrupt_its_siblings() {
    use generator_host::RuntimeOptions;
    use std::sync::Arc as StdArc;

    let runtime =
        StdArc::new(GeneratorRuntime::with_options(RuntimeOptions::default()).expect("runtime"));
    let spin = StdArc::new(
        runtime
            .prepare(&guest(
                COMPATIBILITY_TOKEN,
                "(loop $forever (br $forever))",
                "",
            ))
            .expect("spin prepares"),
    );
    let conforming = StdArc::new(runtime.prepare(&conforming_guest()).expect("prepares"));
    let shared_model = model();

    let timeout_thread = {
        let (runtime, spin, model) = (
            StdArc::clone(&runtime),
            StdArc::clone(&spin),
            Arc::clone(&shared_model),
        );
        std::thread::spawn(move || {
            runtime.execute_prepared(
                &spin,
                model,
                &[],
                RuntimeLimits {
                    wall_time: Some(std::time::Duration::from_millis(50)),
                    ..RuntimeLimits::default()
                },
                ArtifactLimits::default(),
                CancellationHandle::new(),
            )
        })
    };

    let siblings: Vec<_> = (0..4)
        .map(|index| {
            let (runtime, conforming, model) = (
                StdArc::clone(&runtime),
                StdArc::clone(&conforming),
                Arc::clone(&shared_model),
            );
            std::thread::spawn(move || {
                // Keep running across the whole window in which the sibling times out.
                for _ in 0..40 {
                    runtime
                        .execute_prepared(
                            &conforming,
                            Arc::clone(&model),
                            &[],
                            RuntimeLimits::default(),
                            ArtifactLimits::default(),
                            CancellationHandle::new(),
                        )
                        .unwrap_or_else(|error| {
                            panic!("sibling {index} was interrupted by another run: {error}")
                        });
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
            })
        })
        .collect();

    assert_eq!(
        timeout_thread
            .join()
            .expect("timeout thread")
            .expect_err("the spin loop should time out")
            .category,
        GeneratorFailureCategory::ResourceExhausted
    );
    for sibling in siblings {
        sibling.join().expect("a sibling execution panicked");
    }
}

/// Cancellation must reach a guest that never returns to the host.
#[test]
fn a_compute_bound_guest_can_be_cancelled_without_a_deadline() {
    let runtime = GeneratorRuntime::new().expect("runtime");
    let spin = runtime
        .prepare(&guest(
            COMPATIBILITY_TOKEN,
            "(loop $forever (br $forever))",
            "",
        ))
        .expect("spin prepares");
    let cancellation = CancellationHandle::new();

    let canceller = {
        let cancellation = cancellation.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            cancellation.cancel();
        })
    };

    let error = runtime
        .execute_prepared(
            &spin,
            model(),
            &[],
            RuntimeLimits::default(),
            ArtifactLimits::default(),
            cancellation,
        )
        .expect_err("a cancelled spin loop should stop");
    assert_eq!(error.category, GeneratorFailureCategory::Cancelled);
    canceller.join().unwrap();
}
