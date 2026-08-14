# Canonical resolution performance gate

This report-only command measures the one opaque whole-model `sysml_query::PublishedModel` build required by
`planning/RESOLUTION_LAYER_DESIGN.md` §11.3. Every workload selects SOURCE sections from the checked-in
snapshot corpus. It performs no network access, scoped-resolver comparison, semantic fallback, or
machine-specific timing assertion.

The primary acceptance workloads are:

- `standard-library`: one workspace consumer plus all checked-in SysML library sources;
- `many-files`: thousands of uniquely namespaced documents derived from a checked-in diagnostic snapshot,
  yielding a representative tens-of-thousands-node model through the same build path.

Small, medium, and large workloads remain useful phase-scaling probes.

Both primary workloads must complete the full release-mode publication, including canonical
resolution, in under one second on the review machine.

Run the bounded scaling workloads in release mode from the repository root:

```sh
cargo run --release -p spec42-resolution-benchmark -- \
  --iterations 3 \
  --replacement-builds 5 \
  --query-repetitions 3 \
  --output /tmp/spec42-resolution-report.json
```

For an iteration smoke while optimizing one primary workload:

```sh
cargo run --release -p spec42-resolution-benchmark -- \
  --model standard-library --iterations 1 --replacement-builds 0 --query-repetitions 1
```

Use `--model many-files` with the same sampling flags for the thousands-file primary workload.
With no `--model`, the command runs the bounded small, medium, and large scaling probes; primary
workloads are selected explicitly because an implementation that misses the target may take a long
time to complete them.

Each fresh build reports source acquisition, request preparation, and the complete opaque
publication build separately. Checked-in input facts include document and source-byte counts.
Replacement samples build a complete new publication while a separate thread continues issuing
typed, read-only queries against the prior immutable publication.

The report deliberately does not instrument the transitional mutable semantic graph. Parse,
construction, solve, index, evaluation, and validation phase timings—as well as semantic node,
reference, solver-pass, changed-slot, and index-work counters—remain explicit unavailable fields
until the new immutable IR owns and publishes those observations. They must not be reconstructed by
scanning raw graph storage or parsing a debug presentation.

Downstream timings always include operation and result counters and state their implementation:
eager adjacency index, sorted settled-fact lookup, or repeated immutable publication scan. This is
important because a fast scan on a small fixture is not evidence of the required query complexity,
and an unbounded lazy memo would require its own dependency/lifetime contract.

Primary workload reports assess the observed complete-publication wall time against the reviewed
sub-second target. They record `failed-on-this-run` when appropriate without turning host timing
into a default CI failure. Current samples belong in benchmark artifacts or review notes, not as
machine-specific assertions in this documentation. The scoped resolver is not a fallback.

The report also keeps current evidence gaps explicit: portable per-phase peak memory, immutable-IR
phase/work counters, and a generator consuming `PublishedModel` are not yet available. It does not
fabricate substitutes from process RSS, the mutable graph, or presentation text. Canonical
diagnostic rendering is timed and black-boxed, with bytes reported; no diagnostic facts are inferred
by parsing the S-expression.
