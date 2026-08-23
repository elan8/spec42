# Architecture hardening

Tracker for the boundary/representation refactor decided in `design.md` (contract crate, phases
inside the authority, sibling hosts, borrowed facade views, benchmarks). Proposals with the full
evidence: `A_contract_types.md`, `B_resolution_phases.md`, `C_host_boundaries.md`,
`D_performance.md` in this directory. Delete each proposal when its items land.

Every item is a green commit; no guard is loosened. Items in one wave are conflict-free and run in
parallel worktrees; a wave waits for the previous one to merge.

## Wave 1 — landed on `hardening/wave1` (24 commits, linear)

| Branch | Item | Source |
|---|---|---|
| `hardening/contract-crate` | create `sysml_contract` (leaf over `source_identity`), move `RESOLVED_CONTRACT` → typed `SEMANTIC_CONTRACT_VERSION` with value-assertion test, deny.toml wrappers ban, authority_chain/architecture tests updated. Enum migration deferred to wave 2. | A §2, §4, §5 commits 1–2 |
| `hardening/resolution-split` | B steps 1–7: pure `mod` moves of `model.rs`/`resolver.rs` into `pipeline/ lower/ resolve/ evaluate/ index/ check/ diagnose/ model/query/`, then explicit imports. Serial, one agent. | B §6 |
| `hardening/lsp-dedup` | C commits 1–3: rename `lsp_server/src/workspace` → `session`; one range projection; drop duplicate utf16/byte_offset/file_url helpers; generalise guards. | C §5 group A |
| `hardening/consumer-dedup` | C commits 4, 4b, 4c, 4d: one SymbolKind table; severity label fix (`information` vs `info`, CHANGELOG); one SysML file predicate; one digest/display-name. | C §5 groups B, E |
| `hardening/bench` | D item 6: divan bench set (cold stdlib build, warm relink, completion/navigation/diagnostics queries) + allocations-per-element assertion; baseline numbers recorded. | D §4, §6.6 |

## Wave 2 — landed on `hardening/wave2` (21 commits, linear)

Carried over from wave 1 reports:
- lowering still calls `classify_constraint_expression`/`classify_calc_expression` via
  `constraint_evaluation_shape`/`calc_evaluation_shape` — the second evaluation writer (B violation 3);
  removed by the type-state step, as a behaviour change, not a move.
- `model.rs` retains ~5.4k lines of inline contract tests; `lib.rs` ~8k. Relocate with B step 11.
- `build.rs` now emits `pub(crate)` rule tables; revisit when `resolve/implied.rs` owns its readers.
- D10 (`parse_untyped_part_usage_*`) and the duplicated `find_reference_ranges` test remain
  allow-listed in `lsp_server/tests/debt_guardrails.rs` until C commit 7b.
- `parsed_and_text_admission_publish_the_same_identity_over_the_examples` asserts a 5 ms wall-clock
  budget and flakes under load; replace with a deterministic assertion (parse count, not time).
- the `examples` submodule SHA the branch records is not on the submodule's remote; push it.

- B step 8 phase type-state (`Lowered → … → Complete`), step 9 diagram index split, step 10 `tests/phase_order.rs`, step 11 test relocation (~440 tests → `tests/`).
- A: move leaf enums to `sysml_contract` in domain batches; flip facade `pub use`; introduce `SymbolId` (replaces `SymbolIdentity(Box<str>)`).
- D items 1, 3, 4: drop `Arc<ParsedDocument>` from the sealed model (settle ranges at the barrier); arena the `IdentityIndex`; index-backed member queries replacing the five full scans.
- C commits 5–7c: `diagnostics_postprocess` → `sysml_diagnostics`; `library_search` → `language_service`; `import_graph`/`library_closure` → typed queries.

Carried over from wave 2 reports:
- `AuthoredExpression` clones the parser subtree per authored expression (+5% allocations on cold
  build, +0.8% time). Fix is a stable node id from the parser so the site can hold a reference;
  needs an upstream parser change (`planning/UPSTREAM_PARSER_GAPS.md`).
- 74 facade names still carry `SymbolIdentity` / `Box<str>` / `Box<[T]>`; they become movable to
  `sysml_contract` as a side effect of wave 3's `SymbolId` and view conversions. 9 never move
  (8 `spec42_constraint_manifest` *Kind re-exports, `DiagnosticCode`).
- `index/types.rs::SpecializationScope` is a distinct internal bitset enum sharing the contract
  enum's name; rename it (`ScopeBits`) when the index is next touched.
- Library-search item `kind` stays in the host until per-root source kinds reach the LSP (C D13).
- Flaky under load: `parsed_and_text_admission_publish_the_same_identity_over_the_examples` (5 ms
  wall clock), `lsp_same_file_homonym_references_are_disambiguated_by_position` (50 ms retry loop),
  `lsp_rename`. Replace wall-clock assertions with deterministic ones.
- `planning/BENCH_BASELINE.md` was recorded on a loaded machine; re-record on an idle one.

## Wave 3

- D items 5, 7: borrowed accessors and `impl Iterator` on facade products (touches every host once).
- C commits 8–12: validation path → `workspace`; `server` via `workspace`; `lsp_server` drops `workspace`; dependency sets pinned in `architecture.rs`.
- D item 8: per-document incremental reuse for workspace documents (needs cold/warm parity tests).
