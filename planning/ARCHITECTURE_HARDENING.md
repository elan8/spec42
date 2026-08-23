# Architecture hardening

Tracker for the boundary/representation refactor decided in `design.md` (contract crate, phases
inside the authority, sibling hosts, borrowed facade views, benchmarks). Proposals with the full
evidence: `A_contract_types.md`, `B_resolution_phases.md`, `C_host_boundaries.md`,
`D_performance.md` in this directory. Delete each proposal when its items land.

Every item is a green commit; no guard is loosened. Items in one wave are conflict-free and run in
parallel worktrees; a wave waits for the previous one to merge.

## Wave 1 (parallel)

| Branch | Item | Source |
|---|---|---|
| `hardening/contract-crate` | create `sysml_contract` (leaf over `source_identity`), move `RESOLVED_CONTRACT` → typed `SEMANTIC_CONTRACT_VERSION` with value-assertion test, deny.toml wrappers ban, authority_chain/architecture tests updated. Enum migration deferred to wave 2. | A §2, §4, §5 commits 1–2 |
| `hardening/resolution-split` | B steps 1–7: pure `mod` moves of `model.rs`/`resolver.rs` into `pipeline/ lower/ resolve/ evaluate/ index/ check/ diagnose/ model/query/`, then explicit imports. Serial, one agent. | B §6 |
| `hardening/lsp-dedup` | C commits 1–3: rename `lsp_server/src/workspace` → `session`; one range projection; drop duplicate utf16/byte_offset/file_url helpers; generalise guards. | C §5 group A |
| `hardening/consumer-dedup` | C commits 4, 4b, 4c, 4d: one SymbolKind table; severity label fix (`information` vs `info`, CHANGELOG); one SysML file predicate; one digest/display-name. | C §5 groups B, E |
| `hardening/bench` | *Landed:* `tools/query_bench` — divan bench set (cold stdlib build, warm relink, completion/navigation/outline/diagnostics queries) plus an allocations-per-element measurement; baselines in `planning/BENCH_BASELINE.md`. | D §4, §6.6 |

## Wave 2 (after wave 1 merges)

- B step 8 phase type-state (`Lowered → … → Complete`), step 9 diagram index split, step 10 `tests/phase_order.rs`, step 11 test relocation (~440 tests → `tests/`).
- A: move leaf enums to `sysml_contract` in domain batches; flip facade `pub use`; introduce `SymbolId` (replaces `SymbolIdentity(Box<str>)`).
- D items 1, 3, 4: drop `Arc<ParsedDocument>` from the sealed model (settle ranges at the barrier); arena the `IdentityIndex`; index-backed member queries replacing the five full scans.
- C commits 5–7c: `diagnostics_postprocess` → `sysml_diagnostics`; `library_search` → `language_service`; `import_graph`/`library_closure` → typed queries.

## Wave 3

- D items 5, 7: borrowed accessors and `impl Iterator` on facade products (touches every host once).
- C commits 8–12: validation path → `workspace`; `server` via `workspace`; `lsp_server` drops `workspace`; dependency sets pinned in `architecture.rs`.
- D item 8: per-document incremental reuse for workspace documents (needs cold/warm parity tests).
