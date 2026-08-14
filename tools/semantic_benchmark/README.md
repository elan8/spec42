# Immutable semantic benchmark

This report-only harness builds fresh immutable semantic publications from the checked-in snapshot
`SOURCE` corpus. Phase durations come from the publication owner at the parse, canonical lowering,
and resolution/index/evaluation barriers; the harness does not infer them from projections or use a
private semantic path.

Build with symbols and run the complete corpus:

```sh
cargo build --profile profiling -p spec42-semantic-benchmark
target/profiling/spec42-semantic-benchmark --iterations 5
```

Use `--filter sysml.library` (or another substring of a path under `test/snapshots`) for a focused
slice. Select `--schedule sequential` when comparing construction schedules.

Capture a flame profile without including compilation:

```sh
samply record --save-only -o /tmp/spec42-semantic-profile.json \
  target/profiling/spec42-semantic-benchmark \
  --iterations 100 --output /tmp/spec42-semantic-profile-run.json
samply load /tmp/spec42-semantic-profile.json
```

Timings are local evidence, not CI thresholds. Parallel phase figures are elapsed wall time rather
than summed worker CPU time. The benchmark retains every sample and reports min/median/max so warmup
and host variance remain visible.
