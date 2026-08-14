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

Add `--libraries standard` to admit the checked-in standard-library corpus alongside the selected
documents as `StandardLibrary` sources. That is what a real editor build looks like -- a small
workspace resolved against the whole library -- and it is the only way to see how much of a
publication's cost is the library rather than the model being edited.

## Recorded figures

Local evidence from one host, not a threshold. Median of seven parallel builds:

| Selection | Documents | Build | Parse | Lowering | Resolution |
|---|---|---|---|---|---|
| `--filter sysml.library` | 94 (1.35 MB) | 57.8 ms | 8.2 ms | 5.1 ms | 44.3 ms |
| `--filter standard_library_admission` | 1 | ~0.0 ms | ~0.0 ms | ~0.0 ms | ~0.0 ms |
| ... `--libraries standard` | 1 + 94 | 52.2 ms | 7.8 ms | 5.2 ms | 39.2 ms |
| ... `--reuse-library` | 1 + 94 | 33.0 ms | 0.0 ms | 5.1 ms | 27.8 ms |

Two things follow. Resolution is roughly three quarters of a library build, so reusing parsed
documents alone would recover little. And a one-document workspace costs the same as the library
alone, so essentially the entire cost of an editor rebuild is the library being resolved again.

Reusing a settled stratum removes the library's parse outright and the library's share of the
solve, taking the rebuild from 52 ms to 33 ms. What remains is work the stratum does not yet carry
forward: the library is lowered again (5 ms), and the publication barrier rebuilds its identity,
document and type indexes over the merged model (roughly 23 ms, of which the identity index is the
largest). Extending the stratum to carry those index prefixes is the next available step.

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
