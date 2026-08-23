# Query benchmark baseline

The numbers a representation change is compared against. `design.md`: *a representation change is
admitted with a benchmark showing it neutral-or-better on the bundled standard-library corpus.*

How to reproduce, and what each tool measures, is in `DEVELOPMENT.md` § Performance Checks →
Query benchmarks. Both tools drive `sysml_query::Services` and the facade's typed queries only.

## Baseline 2026-08-23

| Fact | Value |
|---|---|
| Machine | Apple M2, 8 cores, macOS 26.3.1 |
| Toolchain | rustc 1.97.1 (release profile) |
| Commit | `be7d7d20` (branch `hardening/bench`) |
| Corpus | 94 bundled standard-library documents, 1,349,609 bytes, plus one user document |
| Published elements | 9,116 (summed from `document_symbols` over every admitted document) |

### Wall time — `cargo bench -p spec42-query-bench`

| Case | Fastest | Median | Mean | Slowest | Samples |
|---|---|---|---|---|---|
| `cold_build_stdlib` | 209.6 ms | **254.7 ms** | 341.3 ms | 531.3 ms | 5 |
| `warm_relink_one_document` | 67.99 ms | **83.43 ms** | 97.58 ms | 175.5 ms | 20 |
| `q_visible_members` | 41.82 µs | **42.16 µs** | 43.39 µs | 121.5 µs | 100 |
| `q_target_at` | 290 ns | **300.5 ns** | 2.761 µs | 245.1 µs | 100 |
| `q_references` | 1.003 µs | **1.087 µs** | 1.113 µs | 4.878 µs | 100 |
| `q_document_symbols` | 220.5 µs | **235.1 µs** | 909 µs | 42.9 ms | 100 |
| `q_diagnostics_for_document` | 29.27 ns | **29.44 ns** | 29.5 ns | 30.41 ns | 100 |

The slowest column is dominated by first-iteration effects (page faults, allocator growth) and by
the machine's other load; the median is the number to compare.

### Allocations — `spec42-query-bench-allocations`

One measured run per case under a counting global allocator. `elements` is what the case produced:
the published element count for the two build cases, the returned result count for the queries.

| Case | Allocations | Bytes | Elements | Allocations/element |
|---|---|---|---|---|
| `cold_build_stdlib` | 708,721 | 180,307,763 | 9,116 | **77.75** |
| `warm_relink_one_document` | 263,054 | 45,871,974 | 9,116 | **28.86** |
| `q_visible_members` | 601 | 59,439 | 114 | **5.27** |
| `q_target_at` | 4 | 456 | 1 | **4.00** |
| `q_references` | 8 | 416 | 3 | **2.67** |
| `q_document_symbols` | 4,554 | 466,888 | 592 | **7.69** |
| `q_diagnostics_for_document` | 0 | 0 | 0 | **0.00** |

### What these numbers say

- `q_diagnostics_for_document` allocates nothing and costs 29 ns: the document-scoped diagnostic
  index is a slice, exactly as `D_performance.md` §4 case 6 asks it to stay. This is the shape
  every other query case should move toward.
- `q_visible_members` — the completion keystroke — allocates 5.3 times per returned member, which
  is `VisibleMember`'s five `Box<str>` fields plus the `SymbolIdentity`. `q_target_at` allocates 4
  times for a single result for the same reason. These are the numbers items 2 and 5 of
  `D_performance.md` §6 move.
- `q_document_symbols` at 7.7 allocations per entry is `SymbolEntry`'s owned strings; the query
  itself is index-backed.
- The warm relink still allocates 263k times and costs 83 ms for a one-line edit, because only the
  library stratum is reused and the whole workspace is re-lowered (`D_performance.md` §3, item 8).
  Cold build is roughly 2.7× the relink, so the settled library stratum is being reused correctly;
  what remains is workspace-side reuse.

## Re-baseline — end of wave 3 (2026-08-24, same machine, quiet)

Commit: tip of `architecture-hardening` after waves 1–3. Medians (divan):

| Case | Median | vs. original baseline |
|---|---|---|
| cold_build_stdlib | 136.9 ms | 254.7 ms |
| warm_relink_one_document | 56.6 ms | 83.4 ms |
| q_visible_members | 5.90 µs (4 allocs) | 42.2 µs (601 allocs) |
| q_document_symbols | 20.8 µs | 235.1 µs (4,554 allocs) |
| q_target_at | 48 ns | 300 ns |
| q_references | 45 ns | 1.09 µs |
| q_diagnostics_for_document | 18 ns (0 allocs) | 29 ns (0 allocs) |
