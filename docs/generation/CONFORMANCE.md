# Generator conformance harness

Runs a corpus of WebAssembly plugins against fixed SysML models and compares the result with
committed goldens. It exists to prove the ABI behaves as [specified](./ABI.md), to catch
behavioural change as Spec42 evolves, and to quantify what a change costs.

```sh
scripts/build-generator-plugins.sh                          # once, and after editing a plugin
cargo run -p generator_conformance --bin generator-conformance
cargo run -p generator_conformance --bin generator-conformance -- --case semantic-api
cargo run -p generator_conformance --bin generator-conformance -- --bless
```

`cargo test --workspace` runs the same corpus, and skips with a message if the plugins have
not been built.

## Layout

```
generator-tests/
  plugins/          nested cargo workspace, wasm32-unknown-unknown only
  models/<name>/model.sysml
  cases/*.toml      declarative cases
  golden/<case-id>/ report.json + artifacts/
```

Plugins are a **nested** workspace so `cargo clippy --workspace` at the repo root does not
try to build wasm-only crates for the host. They are built in release: the debug build of a
plugin is ~36× larger and costs ~3× more to compile, and module compilation dominates the
harness runtime.

## Writing a case

```toml
[defaults]
model  = "coverage"
plugin = "query_all"
expect = "success"

[[case]]
id = "semantic-api/all-operations"
description = "Why this case exists and what regression it would catch."
args   = ["mode=transcript"]
assert = { output_files = 1, query_count = 42 }
```

For a case that must fail, match on the categorical fields rather than exact wording so a
message can be reworded without breaking the suite:

```toml
[[case]]
id = "artifact-paths/parent-escape"
args = ["path=../escape.txt"]
expect = "failure"
failure = { category = "outputpolicy", message_contains = "forbidden component" }
```

Cases are data rather than Rust so adding one does not recompile the harness, and so a
downstream SDK can point `--corpus` at its own corpus and run the identical driver.

## What is asserted, and what is only reported

This split is the point of the design.

| Asserted exactly | Reported, never gated |
| --- | --- |
| `query_count` | wall duration |
| `fuel_consumed` (with `meter_fuel = true`) | |
| `peak_memory_bytes` | |
| artifact paths, bytes and contents | |
| diagnostics | |
| outcome, failure category and phase | |

Measured on this repo, three consecutive runs of the same plugin and model produced
identical `fuel_consumed` and `query_count` every time, while wall time varied. So fuel is
the regression detector and timing is context. Asserting a duration would make the suite
flaky rather than informative.

`fuel_consumed` is a function of the plugin binary, the Wasmtime version and the host's
responses. It moving is meaningful: with an unchanged plugin, it means Spec42 changed how
much work guests do.

Goldens deliberately exclude `model_digest` and `spec42_version`. `model_digest` includes
the engine version, so embedding it would make every golden churn on every release — which
is also why corpus plugins must never write it into an artifact.

## Blessing

`--bless` rewrites goldens from current behaviour, and only for cases whose declared
expectations already hold, so a broken run cannot be recorded as the new truth. Commit the
diff: it *is* the regression report, and a reviewer sees `"query_count": 9 → 11` next to the
change that caused it. CI runs the corpus and fails on any uncommitted golden diff.

## Adding a plugin

Add a crate to `generator-tests/plugins/`, list it in that workspace's `members`, and give it
a case. Plugins must be deterministic: no clock, no randomness, no iteration order of a hash
map, and nothing derived from the Spec42 version.

Some failure modes cannot be produced through the Rust SDK, because `export!` always emits a
correct module — a stale schema fingerprint, a bad pointer, an unknown operation code. Those
are covered by hand-written WAT fixtures in `crates/generator_host/tests/guest_abi.rs`
instead, which assemble modules at test time rather than committing opaque binaries.
