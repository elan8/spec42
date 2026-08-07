# Generator conformance and benchmark harness — plan

Status: proposal. Untracked working document.

Goal: a test harness for the generator ABI that does three jobs — prove conformance,
catch regressions as Spec42 evolves, and quantify the cost of changes over time.

## Summary

Two layers, TOML case files, committed golden trees.

- **Layer A (primary, in-process)** — `crates/generator_conformance`, calling
  `GeneratorRuntime::prepare` + `execute_prepared` directly.
- **Layer B (small, subprocess)** — `crates/server/tests/generator_cli.rs`, driving the
  real `spec42` binary for exit codes, `--check`/`--dry-run`/`--force`, the commit
  transaction, and cross-process determinism.

One new dependency: `wat` (dev-only), to assemble malformed guests from reviewable text.
Everything else (`toml`, `rayon`, `tempfile`, `walkdir`, `serde_json`, `libc`) is already
in `Cargo.lock`.

## The measurement that shapes the design

Measured on this repo, debug example plugin, three consecutive runs:

```
fuel=3184503  queries=3     (bit-identical every run)
prepare_ms=3017 / 2963 / 2931
```

`fuel_consumed` and `query_count` are exactly reproducible. Wall time is not, and
module compilation dominates everything (~3 s of a ~3.3 s run).

Two consequences:

1. **Assert on deterministic counters; trend on time.** Fuel is an exact instruction
   count, so a change in how much work guests do shows up as an exact diff. Wall time
   cannot gate anything.
2. **Prepare each module once and reuse it.** This is the entire performance story of
   the harness, and it is why Layer A exists.

Note on fuel: this uses fuel as a *deterministic cost metric*, which is independent of
whether fuel is used as a *runtime limit*. See "Fuel accounting" below.

| Assert exactly (PR-gating) | Track as trend (nightly, report-only) |
| --- | --- |
| `fuel_consumed` | wall duration (min-of-5) |
| `query_count` | thread CPU time |
| `peak_memory_bytes` (guest) | peak RSS (Linux subprocess only) |
| artifact paths, bytes, count, total | host allocation totals |
| diagnostics (level, message, element) | `module_prepare_ms` |
| `status`, exit code, failure category/phase | |

Fuel is a function of (module bytes, wasmtime version, host responses). That is a
feature — you want to notice a wasmtime bump changing guest cost — but it must be
legible. Record `generator_digest` and the wasmtime version alongside `fuel_consumed`.
A fuel diff *with* a digest diff means "you rebuilt the plugin". A fuel diff *without*
one means "the host changed how much work guests do" — that is the alarm.

## 1. Case format

TOML, array-of-tables, one file per theme, with a `[defaults]` table.

Chosen over RON/YAML (not in `Cargo.lock`; `toml` already is, as a direct dep of
`crates/server`), over JSON (hand-authored cases need comments explaining *why* a
given exit code is expected), and over a Rust-native table (a data format can be
consumed by roc-spec42's own corpus, and adding a case must not mean recompiling
the harness).

```toml
# generator-tests/cases/semantic_api.toml
[defaults]
model  = "coverage"
plugin = "query_all"
layer  = "in-process"
expect = "success"

[[case]]
id = "semantic-api/all-operations"
description = """
Calls every query operation in a fixed order and writes a normalised transcript.
Primary golden for the semantic surface: any change to ordering, metaclass strings,
relationship kinds or ElementDetail defaulting shows up here as a reviewable diff.
"""
args   = ["mode=transcript"]
golden = "semantic-api/all-operations"
assert = { query_count = 9, fuel_consumed = 41_233_918 }

[[case]]
id     = "semantic-api/response-exceeds-guest-buffer"
model  = "wide"
plugin = "big_response"
args   = ["op=find-all"]
golden = "semantic-api/response-exceeds-guest-buffer"
assert = { query_count = 2 }   # first returns -needed, second succeeds
notes  = "generator_sdk::call_query resize loop; response must exceed 64 KiB"
```

### Goldens

`generator-tests/golden/<case-id>/` holds `artifacts/` (byte-for-byte) and a
normalised `report.json`.

**Normalisation is mandatory.** `model_digest` hashes `metadata.engine_version`
(`generator_api/src/model.rs:141`), so it changes on every release — and the existing
example generator writes it straight into `README.md`
(`generator-examples/rust/src/lib.rs:11`). Corpus generators must not embed
`model_digest` or `spec42_version`; `report.json` drops `duration_ms`, `timings`,
`model_digest`, `spec42_version` and absolute paths.

Blessing: `cargo run -p generator_conformance -- --bless [--case <id>]`, which refuses
to run if the tree is dirty outside `generator-tests/golden/`, so the diff is always
attributable. CI runs the suite and fails on any uncommitted golden diff — the same
shape as the existing `scripts/*.mjs --check` steps.

Not `insta`: these goldens are binary artifact *trees*, not `Debug` output. A ~150-line
directory comparator (unified diff for UTF-8, hexdump excerpt otherwise) matches how
this repo already works and avoids a contributor-facing tool.

## 2. Plugin corpus

`generator-tests/plugins/` as a **single nested cargo workspace** (like
`generator-examples/rust/`, which declares an empty `[workspace]` to sit outside the
root). One `cargo build --workspace --release --target wasm32-unknown-unknown` builds
everything and shares dependency compilation.

| Plugin | Exercises |
| --- | --- |
| `query_all` | Every query operation in fixed order; `FIND(None)` and `FIND(Some)`; normalised transcript. Highest-value artifact in the corpus |
| `query_errors` | Unknown handle, empty handle, unknown metaclass, bad handle on each query |
| `big_response` | Response > 64 KiB → `call_query`'s resize path; plus a grow-twice variant |
| `artifact_limits_files` | N files → the pre-deserialisation count gate and `ArtifactSet::emit` |
| `artifact_limits_bytes` | `FileTooLarge`, `TotalTooLarge`, `max_artifact_result_bytes` |
| `artifact_paths` | `../escape`, `/abs`, `C:/drive`, `a\b`, `a//b`, `.`, `..`, empty, 4097-byte path, NUL — every `ArtifactError` variant |
| `path_collisions` | `README.md` + `readme.md`; NFC vs NFD — case/normalisation collision on macOS |
| `binary_artifact` | Bytes `0..=255`, BOM, CRLF, empty file → byte preservation through commit |
| `diagnostics` | All levels; valid/invalid/absent handle; > 64 KiB message; > 10,000 diagnostics |
| `deterministic` | Iterates the whole model, emits a large ordered artifact; run twice in **separate processes** and compared |
| `float_output` | Formats floats into artifacts — pins the NaN/relaxed-SIMD determinism config |
| `empty` | Zero artifacts → empty `ArtifactSet`; `--check` reports unchanged |
| `arg_echo` | argv passthrough including empty, unicode, flag-shaped args |
| `error_guest` / `panic_guest` | `Err(String)` and a trap |
| `slow` | Long-running but well-behaved — pins whatever timeout policy is chosen |

Release profile matters: the debug example wasm is 2.85 MB and costs ~3 s to compile.
Set `opt-level = "z"`, `lto = true`, `panic = "abort"`, `strip = true`,
`codegen-units = 1` in the nested workspace. (`panic = "abort"` changes nothing
observable — `wasm32-unknown-unknown` cannot unwind.)

### Malformed guests — `generator-tests/wat/`, `generator-tests/raw/`

The SDK's `export!` macro always emits correctly-typed exports and valid pointers, so
none of the following is reachable from Rust. These are conformance cases for the
*host's* validation and error reporting, not hostile-actor scenarios.

| Guest | Exercises |
| --- | --- |
| `no_memory.wat` | `memory` exported as a global |
| `wrong_signature.wat` | `spec42_generate` returning `i32` |
| `missing_export.wat` | Absent required export |
| `wasi_import.wat` | Imports `wasi_snapshot_preview1.fd_write` → fails to link |
| `unknown_op.wat` | `query` with operation `8`, `99`, `-1` |
| `oob_pointer.wat` | Pointer past memory end; negative pointer; negative length |
| `huge_transfer.wat` | `request_len` > 64 MiB |
| `bad_postcard.wat` | Garbage bytes as a request |
| `bad_result.wat` | Garbage packed ptr/len; trailing-bytes variant |
| `giant_result_len.wat` | Packed length above `max_artifact_result_bytes` |
| `bad_diagnostic.wat` | Invalid level; non-UTF-8 message |
| `alloc_returns_null.wat` | `spec42_alloc` returns 0 |
| `straight_line_calls.wat` | Loop-free host-call sequence — regression test for checkpoint placement |
| `truncated.wasm` (raw) | Valid module cut mid-section |
| `component.wasm` (raw) | A real component-model binary — realistic toolchain mistake |

**Assemble the WAT in the harness with the `wat` crate at test time.** `.wat` text
cannot be handed to the host: `generator_host` disables wasmtime's default features
(`Cargo.toml:10`), and `prepare()` gates on `Parser::is_core_wasm`, which needs the
binary magic. Committing 14 opaque `.wasm` blobs instead would make the most
validation-sensitive part of the corpus unreviewable in a diff.

### Non-Rust guests

A case may set `plugin_wasm = "path/to/prebuilt.wasm"` instead of `plugin = "<crate>"`,
and the harness binary takes `--corpus <dir>`, so roc-spec42 can point the same runner
at its own generators instead of reimplementing golden comparison in bash
(`roc-spec42/scripts/test.sh`).

## 3. Model corpus

`generator-tests/models/`, harness-owned. Do not reuse `vscode/testFixture/` (owned by
the extension tests, edited when those change) and do not depend on `examples/` (an
uninitialised submodule locally; `crates/server/tests/kitchen_timer_check.rs` already
carries that fragility).

| Model | Size | Purpose |
| --- | --- | --- |
| `minimal/` | ~6 lines | Default where the model is irrelevant — keeps load near zero |
| `coverage/` | ~80 lines | Every metaclass and relationship kind the API reports; multiplicities, doc comments, short names, every `ElementDetail` flag, one evaluated expression |
| `inheritance/` | ~40 lines | Deep specialization with name shadowing — pins the `effective_features` ordering rule |
| `wide/` | ~2,000 lines | Forces the > 64 KiB guest-buffer resize. Sized from measurement: 311 nodes ≈ 60 KB of postcard summaries, right at the threshold, so the drone fixture would be flaky. Generated by a script with `--check`, committed |
| `multi-file/` | 2 files | Cross-file resolution and workspace-relative URIs |
| `errors/` | ~5 lines | Semantic error → generation refuses before instantiation |
| `warnings/` | ~10 lines | Warnings only → generation proceeds |

Under 100 KB total. For performance work, reuse the existing vendored-fixture pattern
(`scripts/fetch-robot-vacuum-cleaner.sh` into gitignored `third_party/`, skip-if-absent,
`#[ignore]` by default) rather than committing a large model.

## 4. Architecture

```
crates/generator_conformance/
  src/{lib,case,runner,golden,metrics,main}.rs
  tests/conformance.rs                  # thin #[test] so `cargo test --workspace` runs it
crates/server/tests/generator_cli.rs    # Layer B; CARGO_BIN_EXE_spec42 resolves here
generator-tests/
  plugins/Cargo.toml                    # nested workspace
  wat/  raw/  models/  cases/  golden/
scripts/build-generator-plugins.sh
```

`crates/generator_conformance` is a workspace member so `cargo fmt`, `clippy
--workspace --all-targets`, and `cargo doc` cover it. `generator-tests/plugins` is a
nested workspace so those same commands do **not** try to build wasm-only crates for
the host target.

| | Layer A (in-process) | Layer B (subprocess) |
| --- | --- | --- |
| Cost per case | ~1 ms, module prepared once | ~3.3 s measured |
| Sees | `GeneratorExecution` directly: ns-resolution duration, fuel, query count, peak memory; `ArtifactSet` in memory | Exit codes, CLI parsing, JSON report, committed tree |
| Cannot see | Exit codes, commit transaction, symlink refusal, per-process nondeterminism | Anything below the CLI |

Layer B is not optional: `std`'s `HashMap` seed is per-process, and this repo has
already shipped a bug from exactly that (the interconnection-layout entry in
`CHANGELOG.md`, where iteration order was stable within a run but differed across
invocations). Determinism cases must run as two separate processes, byte-compared.

Split: ~90% Layer A, ~20-30 Layer B cases covering each exit code, the
ownership/`--force`/symlink matrix, commit rollback, and determinism pairs.

### Parallelism

`rayon` (already a direct dep of `workspace`, `sysml_model`, `lsp_server`).

```
for each model:              load snapshot ONCE     (~48 ms measured)
  for each plugin:           prepare module ONCE    (~3 s debug)
    par_iter over its cases: execute_prepared       (~1 ms)
```

**Do not share a `GeneratorModelView` across cases.** `expose_node`
(`generator_api/src/model.rs:465`) inserts into a shared `Mutex<HashMap>`, so the
handle index accumulates across invocations — a case could resolve a handle it never
legitimately obtained, masking the `UnknownHandle` bugs the suite exists to catch.
Share the `Arc<HostWorkspaceSnapshot>`; build a fresh view per case.

Also: `execute_prepared` spawns a watchdog thread per call, and the engine epoch is
shared across a `GeneratorRuntime`. Wall-clock numbers gathered under parallelism are
meaningless — the perf tier must run `--serial`.

Layer B gets one `TempDir` per case, and it must be a whole temp *root*, not a
subdirectory: `commit_outputs` stages into a sibling of the output directory.

### Building plugins

An explicit `scripts/build-generator-plugins.sh`, with a clear harness-side error when
artifacts are missing (the pattern `robot_vacuum_check.rs` already uses).

Do **not** use a `build.rs` that shells out to cargo: recursive cargo has no shared
lock, breaks `--offline`, and forces `wasm32-unknown-unknown` onto everyone running
`cargo clippy --workspace`.

CI caching needs the nested workspace declared:

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    workspaces: |
      . -> target
      generator-tests/plugins -> target
```

(The current `ci.yml` does not do this for `generator-examples/rust`, so today's smoke
step pays a cold plugin build every run.)

And `.gitattributes` needs:

```
generator-tests/golden/** -text
generator-tests/raw/** binary
```

Without it, a Windows checkout with `core.autocrlf=true` rewrites golden bytes — the
exact failure the existing `.gitattributes` comment documents for ts-rs output.

## 5. Instrumentation

**Free today** — `GeneratorExecution` already returns `duration`, `query_count`,
`fuel_consumed`, plus artifact count and total bytes. Caveat: `generation.rs:290`
truncates with `as_millis()`, which reads 0 for a real guest. Layer A must consume
`execution.duration` as a `Duration`.

**Guest peak memory** — wrap the store limiter:

```rust
struct MeteredLimits { inner: StoreLimits, peak_memory_bytes: usize }
impl ResourceLimiter for MeteredLimits {
    fn memory_growing(&mut self, current, desired, maximum) -> Result<bool> {
        self.peak_memory_bytes = self.peak_memory_bytes.max(desired);
        self.inner.memory_growing(current, desired, maximum)
    }
}
```

This is a true high-water mark: wasmtime calls `memory_growing(0, minimum, maximum)` at
memory *creation*, not only on `memory.grow`. Deterministic, free, and captures
requested peaks even when a grow is rejected. Recommend adding `peak_memory_bytes` to
`GeneratorExecution` and `GenerationReport` as public API — it is independently useful
to anyone tuning `--max-memory-bytes`, which the report currently says nothing about.

**Host allocations** — only a counting `#[global_allocator]` gives this, and it is
process-global, so numbers are meaningful only serially. Put it in a dedicated binary
target (`src/bin/alloc_profile.rs`) that defines the allocator itself. Do **not** put
it behind a Cargo feature: CI runs `clippy --all-features`, which would compile a
custom global allocator into every target.

**Wall and CPU time** — report **min of K repetitions** (K=5), not mean or median, plus
p50 and max for visibility into runner noise. Thread CPU time via
`clock_gettime(CLOCK_THREAD_CPUTIME_ID)` (`libc` already in the lock) removes scheduler
noise but not frequency scaling. Report; never gate.

**RSS** — cannot be measured per-case in-process (process peak RSS is monotonic).
Requires subprocess `getrusage(RUSAGE_CHILDREN)`, and `ru_maxrss` is **kilobytes on
Linux, bytes on macOS**. Nightly Linux only, report-only.

## 6. Regression workflow

The git diff is the regression report. No time-series database.

Baseline is the committed golden. Updating it is `--bless` plus a commit, so a reviewer
sees `"fuel_consumed": 41233918 -> 41890210` as a diff line next to the change that
caused it.

| Tier | Where | Gate |
| --- | --- | --- |
| Conformance (A + B) | `ci.yml`, replacing the current smoke step | **Fails** on any golden diff |
| Timing/resources | `nightly.yml` `performance` job | **Reports**; fails only on coarse budgets |

Nightly budgets follow the existing convention: emit
`target/spec42-perf/generator-conformance.json` via the `SPEC42_PERF_REPORT` mechanism
in `crates/lsp_server/tests/integration/perf_report.rs:284`, budgets embedded in the
Rust test, `scripts/check-perf-budgets.mjs` extended. Set them at ~3× observed min —
they exist to catch order-of-magnitude regressions, not drift.

Local investigation:

```sh
scripts/build-generator-plugins.sh
cargo run -p generator_conformance -- --case semantic-api/all-operations --explain
cargo run -p generator_conformance -- --case perf/wide-find-all --serial --repeat 5
cargo run -p generator_conformance -- --bless --case semantic-api/all-operations
```

## 7. Phasing

**Phase 0 — half a day.** First measure whether a release-profile plugin cuts the ~3 s
prepare time (needs `rustup target add wasm32-unknown-unknown`); it determines the whole
suite's shape. Then the vertical slice: one plugin, three models, golden compare, bless.

Create `crates/generator_conformance/*`, `generator-tests/plugins/query_all/`,
`generator-tests/models/{minimal,coverage,multi-file}/`,
`generator-tests/cases/semantic_api.toml`, the first golden,
`scripts/build-generator-plugins.sh`, and add the crate to the root workspace members.

Day-one value: a byte-exact golden of the entire semantic API surface plus exact fuel
and query counts — against today's coverage of one smoke command.

**Phase 1 — 1-2 days.** Rest of the Rust plugin corpus; `limits`, `diagnostics`,
`resources` case files; remaining models; the wide-model generator script.

**Phase 2 — 1-2 days.** WAT and raw malformed guests; `hostile.toml`; add `wat` as a
dev-dependency. Budget review time — this phase is where host validation bugs surface.

**Phase 3 — 1 day.** `crates/server/tests/generator_cli.rs`; `cli.toml`. Exit codes,
`--check`/`--dry-run`/`--force`, ownership matrix, symlink refusal, commit rollback,
cross-process determinism.

**Phase 4 — 1 day.** `MeteredLimits` and `peak_memory_bytes` through to the report;
`perf.toml`; extend `check-perf-budgets.mjs`; nightly step.

**Phase 5 — half a day.** Replace the CI smoke step; cache the nested workspace; fix
`.gitattributes`; `docs/generation/CONFORMANCE.md`; a `DEVELOPMENT.md` subsection;
`--corpus` for roc-spec42.

**Phase 6 — optional.** `alloc_profile.rs`. Explicitly out of scope: `wasm-smith`
fuzzing — no stable reviewable corpus, wasmtime is already fuzzed upstream, and the
value here is in the Spec42 decode/validation layer that the curated guests cover with
better failure messages.

## Fuel accounting

The harness uses `fuel_consumed` as its regression metric because it is the only
deterministic cost signal available. That is a separate decision from whether fuel is
enforced as a *limit* at runtime.

If `consume_fuel` is disabled in the shipped CLI (a reasonable choice for trusted,
user-invoked plugins — see the notes on the runtime configuration), the harness must
enable it explicitly in its own `Config` and accept that it is then measuring a
slightly different configuration from production. Fuel accounting adds instrumentation
to every basic block, so a fuel-enabled run is measurably slower than a fuel-disabled
one; the counter itself remains exact and comparable across harness runs.

The alternative — dropping fuel entirely and regressing on wall time — does not work.
Measured spread on an idle machine was already 3%, and shared CI runners are far worse.

## Things that will not work

1. Feeding `.wat` text to the host — the `wat` feature is disabled and `prepare()`
   requires the binary magic.
2. `criterion`/`divan` — the metric that matters is exact, so statistical machinery is
   aimed at the wrong problem.
3. `insta` — goldens are binary trees, not `Debug` output.
4. Sharing one `GeneratorModelView` across parallel cases — masks handle bugs.
5. Asserting wall time in PR CI.
6. Depending on `examples/` — empty submodule locally.
7. Goldens containing `model_digest` or `spec42_version` — both change per release.
8. A `build.rs` that builds the wasm corpus.
9. Per-case RSS from an in-process harness — process peak RSS is monotonic.
10. Subprocess-per-case as the primary layer — ~3.3 s each, 97% of it compilation.

## Open questions

1. **Internal suite or public conformance suite?** If roc-spec42 and future third-party
   SDKs run the same corpus against their own guests, the case-file schema becomes a
   stable contract and the crate needs a documented CLI and a published name.
2. **PR-gating wall-clock budget?** ~60 s is achievable with Layer A plus a handful of
   Layer B cases. ~15 s means Layer B moves entirely to nightly. This single number
   determines corpus size.
3. **Should `peak_memory_bytes` be public API** on `GeneratorExecution` and
   `GenerationReport`, or stay harness-private?
4. **Commit the generated `wide` model, or generate into `target/` at start-up?**
   Committing makes goldens diff meaningfully at the cost of ~2,000 generated lines.
