# ELK layout parity harness

This development-only tool runs identical, checked-in ELK JSON graphs through Spec42's current
ELK.js/QuickJS adapter and the pinned public `elan8/elkrs` revision. The native implementation is
owned by `crates/diagram_layout`; it restores Spec42's root-authored/root-coordinate edge contract
when elkrs publishes an intra-container edge on its lowest common ancestor. The harness compares
layout geometry, not serialized JSON, so object-key order and number formatting do not create false
differences.

The comparison covers graph and container bounds, nodes, ports, node/port/edge labels, edge-section
start/end points, and bend points. It also verifies node/port/label/edge identity completeness,
requires routed sections for every edge, and rejects nondeterministic output across repeated runs.
Paths are stable and differences are sorted, making JSON output suitable for review or CI artifacts.

Run all fixtures:

```sh
cargo run -p elkrs_parity -- --iterations 5
```

Write a machine-readable report:

```sh
cargo run -p elkrs_parity -- --format json --output elkrs-parity.json
```

Pass one or more JSON paths to compare additional captured renderer inputs. Use
`--fail-on-difference` when exact geometry is required. The default is report-only because deciding
which differences are contractually acceptable is an explicit migration decision.

The first timed run includes engine initialization. `median_layout_us` is the median of all timed
runs and is intended for relative local comparison, not as a portable performance guarantee. Final
binary/package size is measured outside this harness (see `native_layout_binary_size_ratchet.rs`
under `crates/server/tests/integration/`).

## Cold/warm process-mode benchmarks

The default (in-process) mode above loads both engines into one already-warmed process, which is
the right thing for the geometry comparison but makes per-engine memory and startup numbers
misleading. `--process-mode cold` and `--process-mode warm` instead re-exec this binary as a child
process per engine, measuring one engine per process against a single fixture (skipping the
geometry comparison entirely -- that is the in-process default's job):

```sh
# Full process startup, paid every sample (re-execs once per iteration).
cargo run --release -p elkrs_parity -- --process-mode cold --iterations 5

# Startup paid once, amortized across all iterations inside one child process.
cargo run --release -p elkrs_parity -- --process-mode warm --iterations 20
```

Both require `--release`: a debug binary's own overhead would swamp the signal (the tool refuses
to run in debug unless `ELKRS_PARITY_ALLOW_DEBUG=1` is set, for iterating on the tool itself).
`--process-mode-fixture <path>` picks a different fixture; the default is the largest checked-in
corpus fixture. Peak resident memory is read from `/proc/self/status`'s `VmHWM` (Linux only;
reported as `null` elsewhere) -- each child's peak is its own, cleanly isolated by the process
boundary, unlike a shared in-process allocator would be.
