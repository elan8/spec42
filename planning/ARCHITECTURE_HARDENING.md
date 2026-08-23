# Architecture hardening

Tracker for the boundary/representation refactor decided in `design.md` (contract crate, phases
inside the authority, sibling hosts, borrowed facade views, benchmarks). The four analysis proposals that produced this plan landed and were deleted; their evidence
lives in git history (`planning/{A_contract_types,B_resolution_phases,C_host_boundaries,D_performance}.md`
before this commit).

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
- Library-search item `kind` stays in the host until per-root source kinds reach the LSP (C D13).
- Flaky under load: `parsed_and_text_admission_publish_the_same_identity_over_the_examples` (5 ms
  wall clock), `lsp_same_file_homonym_references_are_disambiguated_by_position` (50 ms retry loop),
  `lsp_rename`. Replace wall-clock assertions with deterministic ones.
- `planning/BENCH_BASELINE.md` was recorded on a loaded machine; re-record on an idle one.

## Wave 3 — landed on `hardening/wave3` (35 commits, linear)

| Item | Result |
|---|---|
| C 8–12 sibling hosts | validation path in `workspace`; `server` reaches it there; `lsp_server` has no `workspace` dependency; both host dependency sets pinned; host text-entry allow-list 38 → 3 |
| Syntax follow-ups A–C | 8 typed syntax queries; 11 consumer heuristics deleted; 7 `syntax_authority` exemptions removed (5 items remain, see `SYNTAX_FOLLOW_UPS.md`) |
| D 1 parse tree | sealed model holds `LineIndex` only; `phase_order.rs` rule 4 bans tree/text fields |
| D 3, 4 | identity derived from owner chain; member queries index-backed; `ScopeBits` rename |
| D 8 incremental reuse | per-document lowering memo keyed by digest, generational eviction, cold/warm parity proven by construction; warm relink −4.6% |
| A 3 `SymbolId` / `DocumentId` | `Copy` handles with `qualified_name`/`document_identity` accessors; strings only at protocol edges; retired `SymbolIdentity` |
| D 5, 7 borrowed views | `VisibleMembers` (completion 487 → 4 allocs), `PublishedDiagnostics` (no clone per query), `NavigationTarget` |
| Flaky tests | timer → `sources_parsed` counted fact; 9 sleep loops → publication barrier |
| Host bug | `rebuild_publication` prepared off-actor and took its build token in a later turn, so a stale-input build could out-rank a newer one and a just-opened document could be missing from the live publication forever — now one actor mutation |

Measured on the bundled standard library (9,116 elements), original baseline → wave 3:
cold build 255 → ~134 ms · warm relink 83 → ~55 ms · completion 42 µs / 601 allocs → ~19 µs / 4 allocs ·
document symbols 235 µs / 4,554 allocs → ~170 µs / 1,596 allocs · peak allocations −23%.

### Remaining under the facade rule (guard-protected, shrink-only)

`FACADE_OWNED_STRING_PRODUCT_FIELDS` in `crates/sysml_query/tests/architecture.rs` lists 17 product
fields that still own strings. Convert one family per change, copying the `VisibleMembers` /
`NavigationTarget` shape: `SymbolEntry` (document symbols — largest remaining allocation site),
`ElementInspection`, `Documentation`, `AuthoredUnit`, `QualifiedReferenceTarget`, `PackageTargets`,
the six `Diagram*` scene types, and the five syntax-service products (`SyntaxOutlineNode`,
`SyntaxImport`, `SyntaxToken`, `SyntaxUnitLiteral`, `SyntaxDiagnostic`) added by the follow-ups
work. Types that become field-free move to `sysml_contract`.

## After wave 3 — measured, not yet scheduled

- **Warm relink is 95% resolution, not lowering.** With per-document lowering reuse landed, a
  one-line edit on the 94-document standard library measures parse ≈ 13 µs, lowering ≈ 2.7 ms,
  resolution + implied relationships + evaluation + index + diagnostics ≈ 61 ms — all of which run
  over the whole workspace because they read cross-document facts. The next latency win is an
  incremental or document-local phase 3+; that is a design item, not a tuning item.
- `AuthoredExpression.node` still clones a parser subtree; needs a stable node id upstream.
- `LineIndex::range` cannot reject a span inside a UTF-8 code point (no text); parser spans are
  boundary-valid today. Add a debug assertion at the barrier if the parser ever changes.
- `concurrency_regressions.rs` still polls with `sleep(50ms)` around a superseded relink; needs a
  decision on which publication the test asserts.
