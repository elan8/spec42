# Proposal C — Host boundaries: resolving the `lsp_server` / `workspace` / `server` graph

Analysis only; no code changed. Line counts are `src/` only (`lsp_server` = 10,633 across 53 files,
not 19.4k/75 — the earlier figure counted tests/fixtures).

## 1. `lsp_server/src` audit

| Bucket | Lines | Modules |
|---|---:|---|
| **Protocol adaptation** (legit host) | 5,518 | `lsp_runtime/**` (4,193), `language/mod.rs` (1,201 — thin `language_service` → `tower-lsp` wrappers), `language/symbols.rs` (124) |
| **Presentation / DTO** (legit host) | 1,051 | `views/**` (`feature_inspector.rs` 672, `dto.rs` 329, `library_search_adapter.rs` 42) |
| **Scheduling / session** (legit host) | 1,800 | `workspace/handle.rs` 649, `state.rs` 428, `services.rs` 337, `services/rebuild.rs` 199, `snapshot.rs` 105, `services/edits.rs` 68 |
| **Configuration** (legit host) | 288 | `host/config.rs` 255, `logging.rs` 25, `default_config.rs` 5 |
| **Thin adapters / glue** (legit) | 301 | `analysis/diagnostics_{adapter,core}.rs`, `semantic_tokens/mod.rs`, `common/text_span.rs`, `lib.rs` |
| **Mixed — config parsing + text probing** | 341 | `common/util.rs` (URI normalisation, incremental edit application, `untyped_part_usage_diagnostics`, `import_statement_ranges` — the last two probe SysML text) |
| **→ belongs in `workspace`** (batch validation) | 508 | `validation.rs` 69, `validation/built_workspace.rs` 232, `report.rs` 129, `discovery.rs` 78 |
| **→ belongs in `sysml_diagnostics`** (reporting policy) | 361 | `analysis/diagnostics_postprocess.rs` — dedup, cascade collapse, shadowed-by-parse-error suppression, priority ordering |
| **→ belongs in `language_service`** (editor intelligence) | 285 | `workspace/library_search.rs` — fuzzy scoring, symbol-kind labels, tree building, short-name recovery |
| **→ belongs in the authority / facade** | 180 | `workspace/import_graph.rs` 134 (semantic dependency selection for republish — a semantic derivation), `workspace/library_closure.rs` 46 (closure document loading) |
| **Total** | 10,633 | |

**Verdict:** ~1,334 lines (12.5%) sit in the wrong crate outright; another ~341 are mixed. The
single biggest structural fault is `validation/*`: it is a *batch* pipeline (takes `&Spec42Engine`,
walks a directory, publishes once, collects diagnostics) living in the editor host. That is the
sole reason `server` depends on `lsp_server` for "validation".

`lsp_server/src/workspace/` is a misnomer that shadows the `workspace` crate name inside the file
that imports it (`validation/discovery.rs` has both `crate::…` and `workspace::…` in scope). It is
really the *session* module and should be renamed `session/`.

## 2. Duplicated derivations across consumers

| # | Derivation | Locations |
|---|---|---|
| D1 | **Batch validation report shape** — documents + per-severity summary over a published directory | `crates/lsp_server/src/validation.rs:26` (`ValidationReport`/`ValidatedDocument`/`ValidationSummary`) vs `crates/workspace/src/snapshot/validation.rs:9,15,23` (`HostValidatedDocument`/`HostValidationSummary`/`HostValidationReport`) + `collect_host_validation_report` at `:30`. Two parallel report types, two summarisers (`crates/workspace/src/snapshot/validation.rs:60` `summarize`, `crates/lsp_server/src/validation/report.rs`). `server` re-exports the *lsp* one: `crates/server/src/reports.rs:1`. |
| D2 | **Target discovery / workspace-root inference** | `crates/lsp_server/src/validation/discovery.rs:10,19` re-wraps `crates/workspace/src/snapshot/discovery.rs:15,39` only to change the error type and soften the empty case. Duplicate wrapper, not duplicate logic — delete by moving the caller. |
| D3 | **path → `file://` URL** | `crates/lsp_server/src/validation/discovery.rs:28` reimplements with `Url::from_file_path` + `normalize_file_uri`, while `crates/workspace/src/snapshot/discovery.rs:49` correctly delegates to `sysml_query::source::path_to_file_url`. The lsp copy bypasses the source authority's normalisation. |
| D4 | **URI normalisation** | authority: `crates/sysml_source/src/lib.rs:184` `normalize_uri`; host copy: `crates/lsp_server/src/common/util.rs:26` `normalize_file_uri`; two further shims: `crates/lsp_server/src/workspace/snapshot.rs:90` and `crates/language_service/src/workspace.rs:34`. |
| D5 | **UTF-16 length of a str** | `crates/lsp_server/src/common/util.rs:60` `utf16_len` vs `crates/language_service/src/code_actions.rs:24` `utf16_len` — byte-identical. |
| D6 | **LSP position → byte offset** | `crates/lsp_server/src/views/feature_inspector.rs:390` `byte_offset` vs `crates/language_service/src/text.rs:6` `position_to_byte_offset` (already re-exported from `language_service`, and `lsp_server` already tests it at `language/mod.rs:387`). |
| D7 | **`TextRange` → LSP `Range`** | Owner `crates/lsp_server/src/common/text_span.rs:12` `to_lsp_range`; byte-identical copies at `crates/lsp_server/src/analysis/diagnostics_adapter.rs:32`, `crates/lsp_server/src/lsp_runtime/hierarchy.rs:24` (`lsp_range`), `crates/lsp_server/src/views/dto.rs:22` (`range_to_dto`), `crates/lsp_server/src/views/feature_inspector.rs:52` (`range_dto`), plus test copies at `language/mod.rs:41,56`. **Note:** `text_span.rs:39` already has a guardrail asserting no copy exists in `references_resolver.rs` — it catches one copy and misses five. Inside `language_service` the same converter pair is re-declared per module: `rename.rs:6,13`, `references.rs:30,37` (character-identical), `navigation.rs:247` (`probe`), `outline.rs:14`. |
| D8 | **Symbol-entry shape** | `crates/lsp_server/src/language/mod.rs:12` `symbol_entries_for_uri` maps `language_service::SymbolEntry` field-by-field into an lsp-local `SymbolEntry` (`language/symbols.rs`), and `crates/lsp_server/src/workspace/snapshot.rs:34` `convert_symbol_entry` maps it *back*. Round-trip conversion with no added information. |
| D9 | **Kind vocabulary, split across four unconnected tables** | `crates/lsp_server/src/language/symbols.rs:23` `outline_kind_to_lsp` and `crates/lsp_server/src/lsp_runtime/features/editing_features.rs:227` `workspace_symbol_kind` are **character-for-character identical** `&str -> SymbolKind` matches in the same crate (one for document symbols, one for workspace symbols). `crates/language_service/src/completion.rs:777` `query_kind_label` produces the very strings they match on, and `crates/lsp_server/src/workspace/library_search.rs:31` `symbol_kind_label` is the inverse table. Four tables, no single owner. |
| D10 | **SysML text probing outside the authority** | `crates/lsp_server/src/common/util.rs:31,81,100` (`parse_untyped_part_usage_line`, `untyped_part_usage_diagnostics`, `import_statement_ranges`) vs `crates/language_service/src/code_actions.rs:28` `parse_untyped_part_usage_name` — the same `part`/`part def`/`;`/`:` predicate, two return types. Also `crates/lsp_server/src/lsp_runtime/features/editing_features.rs:19` `collect_brace_folding_ranges` hand-rolls brace matching over raw text as a fallback for the parser-derived `collect_folding_ranges` (`language/symbols.rs:72`) — two folding derivations that can disagree. And `crates/lsp_server/src/language/mod.rs:335` `library_search_symbol_from_diagnostic` parses a diagnostic *message* to recover a symbol name. |
| D11 | **"is this a SysML file"** — five copies, two different case-sensitivity behaviours | Owner `crates/sysml_source/src/lib.rs:157` `is_sysml_like` (used by `FilesystemProvider::walk` at `:340`, honours gitignore) and re-exported at `crates/workspace/src/snapshot/discovery.rs:13`. Re-derived inline at `crates/library_catalog/src/library/bundle.rs:148` (case-**sensitive**), `crates/library_catalog/src/catalog.rs:417` (case-insensitive), `crates/server/src/environment.rs:603` (case-sensitive), `crates/kpar/src/pack.rs:353`. Each carries its own `walkdir` traversal. |
| D12 | **Severity → label**, five tables over one axis | Owner `crates/sysml_diagnostics/src/reporting.rs:94` `severity`. Downstream re-derivations: `crates/workspace/src/comparison/diagnostics.rs:214` `severity_label` → `"information"` vs `crates/server/src/reports.rs:349` `severity_label` → `"info"` — **same name, divergent output**, a latent inconsistency between the comparison harness and CLI output. Plus `reports.rs:340` `sarif_level`, `:285` `sarif_catalog_level`, `:359` `severity_number`. |
| D13 | **Library-source classification** | `crates/lsp_server/src/workspace/library_search.rs:143` `library_source_label` string-matches `/standard-library/` and `/domain-libraries/` in a URI path; `crates/workspace/src/provider/filesystem.rs:118` `library_source_kind` answers the same question *structurally* from the configured stdlib paths. The editor host re-derives by substring what configuration already states. Also `crates/library_catalog/src/library/types.rs:197` `stable_path_label` vs `library_search.rs:132` `package_name_from_path`. |
| D14 | **Library root defaulting applied twice** | `crates/library_catalog/src/library/bundle.rs:125` `discover_library_roots` applies the empty→install-path fallback at `:137`; `crates/library_catalog/src/catalog.rs:369` `stdlib_resolution_roots` re-applies it after `stdlib.rs:120` already delegated. Same decision at two layers. `crates/server/src/environment.rs:466` `resolve_library_paths` assembles a third list. |
| D15 | **Two entry points into one diagnostic core** | `crates/lsp_server/src/validation/built_workspace.rs:221` `collect_diagnostics_for_document` (batch) and `crates/lsp_server/src/lsp_runtime/diagnostics.rs:214` `collect_diagnostics_for_document` (async, interactive) — same name, same purpose, both over `analysis/diagnostics_core.rs:19`, differing only in which policy pair they pass (`diagnostics_core.rs:36,46` vs `:58,63`). |
| D16 | **"should we advise about libraries"** derived twice | `crates/lsp_server/src/validation/report.rs:47` `should_suggest_library_roots` computes the advice into the report; `crates/server/src/lib.rs:41` `perform_check` then re-scans the workspace via `crates/server/src/environment.rs:575` `workspace_references_standard_library` — and at `lib.rs:58` greps the report's own advice strings for `"standard library"` to avoid colliding with itself. |
| D17 | **Artifact digest format** | `crates/generator_host/src/lib.rs:1360` `digest` vs `crates/server/src/generation/mod.rs:727` `digest` — byte-identical `format!("sha256:{:x}", …)`. Two crates independently define a format that must agree for generation results to verify. |
| D18 | **Display-name fallback** | `crates/generator_api/src/model.rs:1491` `display_label` ("name, else qualified_name") re-derives from published values a decision the resolver already made at `crates/sysml_resolution/src/model/resolver/inspection.rs:234` / `host_conformance.rs:260`. |

Also duplicated but lower-value: recursive directory listing with `strip_prefix` + `\`→`/`
normalisation at `crates/generator_conformance/src/golden.rs:107`,
`crates/library_catalog/src/library/bundle.rs:170`, `crates/server/src/generation/mod.rs:553`,
`crates/library_catalog/src/catalog.rs:427`; and the two near-identical cache probes at
`crates/library_catalog/build.rs:512,526`.

**Not duplication** (checked, single-owner — do not "fix"): `position_to_byte_offset`,
`word_at_position`, `line_prefix_at_position` live once in `crates/language_service/src/text.rs`
and are re-exported through `lsp_server/src/language/mod.rs:6-10`; outline/folding is genuinely
single-owner via `language_service/src/outline.rs:32,37`.

## 3. Resolving the graph

Three moves:

**(a) `workspace` becomes the batch library, and grows the validation path.**
`lsp_server::validation::*` moves to `workspace::validation`, expressed over the *existing*
`HostValidationReport` (D1 collapses to one type). The editor host keeps nothing of it — the LSP
never calls `validate_paths`; only the CLI does. Rename the crate `workspace` → `sysml_workspace`
(or `batch_host`) at the same time to stop shadowing `lsp_server::workspace`; `lsp_server::workspace`
→ `lsp_server::session`.

**(b) `server` stops depending on `lsp_server` for validation.**
After (a), `server`'s four touchpoints are: `run_lsp` + `default_server_config` (`lib.rs:33,45,241,253`),
`init_tracing` (`main.rs`), `CustomRpcContext/Provider` (`library_status_rpc.rs`), and the validation
report types (`reports.rs`). Only the first three are genuinely the editor host. So `server` keeps a
dependency on `lsp_server` — but as a *binary composition* edge (it spawns the LSP), not a
"reach validation through the editor host" edge. `reports.rs` retargets to `workspace`.
Optionally make it a feature-gated dep so `spec42 check` builds without the editor host.

**(c) `lsp_server` sheds intelligence.**
`workspace/library_search.rs` → `language_service::library_search` (scoring, labels, tree) with
`views/library_search_adapter.rs` staying as the DTO layer. `analysis/diagnostics_postprocess.rs`
→ `sysml_diagnostics` (it is reporting policy: dedup, cascade collapse, shadowing). `import_graph.rs`
and `library_closure.rs` → a typed query on `sysml_query` (`PublishedModel::documents_affected_by`
and the library service). `common/util.rs`'s text probes → syntax-service queries. D4–D9 collapse
onto the existing single owners.

Target graph (changed edges marked):

```text
sysml_query ──► language_service ──┐
            ──► sysml_diagnostics ─┤
            ──► sysml_tokens ──────┤
            ──► kpar ──► library_catalog
                                   │
              workspace (batch host + validation library)  ◄── (a)
                 ▲            ▲
                 │            │
   lsp_server ───┘ (b: shrinks to session/protocol only)
       ▲
       │ binary composition only (run_lsp / config / tracing / rpc traits)
    server ────────────────────────► workspace   (validation, reports)  ◄── (b)
           ────────────────────────► sysml_query, library_catalog, kpar
```

Updated design.md "May depend on" rows:

| Crate | Role | May depend on (SysML crates) |
|---|---|---|
| `language_service` | protocol-neutral editor intelligence over typed queries, **including library search ranking and labels** | `sysml_query` |
| `sysml_diagnostics` | transport-neutral diagnostic values, **reporting policy and cascade/shadowing post-processing**; decides nothing semantic | `sysml_query` |
| `workspace` | batch host **and validation library**: engine, directory snapshot, validation, comparison, schema versions | `sysml_query`, `library_catalog`, `language_service`, `sysml_diagnostics` |
| `lsp_server` | editor host: document lifecycle, LSP handlers, host adapters. **Owns no validation pipeline and no batch entry point** | `session_actor`, `language_service`, `sysml_tokens`, `sysml_diagnostics`, `sysml_query`, `library_catalog`, `generator_*` — **not `workspace`** |
| `server` | CLI, MCP, and LSP binary; reaches validation through `workspace`, and `lsp_server` **only to launch the editor host** | consumers above |

Note the strongest form of (b): after (a) and (c), `lsp_server` no longer needs `workspace` at all —
its only uses are `Spec42Engine` (for `validate_paths`) and the discovery helpers, both of which
leave. That restores the sibling-hosts framing in design.md literally.

## 4. Guard changes

`crates/sysml_query/tests/architecture.rs`
- `host_crates_keep_their_declared_dependency_sets()` (`:793`) currently pins `session_actor`,
  `library_catalog`, `workspace`. Add pinned sets for `lsp_server` (asserting **absence** of
  `workspace`) and `server` (asserting `workspace` present; `lsp_server` present but marked as the
  launch-only edge). This is the single assertion that makes step (b) irreversible.
- `migrated_validation_paths_cannot_return_to_the_graph()` (`:345`) — extend its path list from
  `lsp_server/src/validation` to `workspace/src/validation`, and add a check that
  `crates/lsp_server/src/validation` **does not exist**.
- `DESIGNATED_CONSUMERS` (`:11`) unchanged.

`crates/lsp_server/tests/debt_guardrails.rs`
- `lsp_workspace_does_not_own_semantic_build_or_library_cache()` (`:78`) — retarget its path from
  `src/workspace` to `src/session` after the rename.
- `library_closure_never_runs_on_the_edit_path()` (`:206`) — unchanged, but `library_closure.rs`
  leaving the crate makes it trivially true; keep it as a regression fence.
- `MAX_ALLOW_ATTRIBUTES_IN_SRC = 38` (`:5`) should ratchet down as ~1.3k lines leave.

`crates/sysml_query/tests/syntax_authority.rs`
- `EXEMPTIONS` (`:65`) — the entries covering `lsp_server/src/common/util.rs` and
  `workspace/library_search.rs` must be deleted, not repointed, once D10 and library search move.
  `every_exemption_names_an_existing_file_whose_justifying_property_still_holds()` (`:884`) already
  fails loudly if a path disappears, so the migration is self-checking.
- `SHADOW_ALLOW` (`:381`) — drop `utf16_len`, `byte_offset`, `normalize_file_uri` entries if present.

**New guards to catch (2):**

1. **`no_sysml_text_entry_points_in_hosts`** — AST visitor over `crates/lsp_server/src` and
   `crates/server/src`: fail on any `fn` whose parameter is `&str`/`String` *named* `source`,
   `text`, `content`, or `line`, unless the file is on a small allow-list (incremental-edit
   application in `common/util.rs`, formatting range computation). Catches D6, D10, and any future
   re-invention. Model it on the existing `ProbeVisitor` in `syntax_authority.rs:756`.
2. **`hosts_declare_no_document_keyed_maps_outside_the_session_allow_list`** — fail on any
   `HashMap<Url, _>` / `BTreeMap<Url, _>` / `HashMap<PathBuf, _>` field in a host crate outside an
   explicit `(file, field)` allow-list, mirroring `PARSED_TREE_FIELD_ALLOWLIST`
   (`syntax_authority.rs:215`). `ServerState::index` is the only legitimate one; this catches
   derived-data caching creeping back into `views/` or `validation/`.
3. **`one_range_projection_per_target`** — generalise the existing point guardrail at
   `crates/lsp_server/src/common/text_span.rs:39` (which names one file) into an AST count: any `fn`
   in `lsp_server` taking a `TextRange`/`SyntaxRange` and returning `Range`/`RangeDto`; assert ≤ 2.
   Catches D7's six copies where the current named-file check catches one.
4. **`no_duplicate_free_function_bodies_across_consumer_crates`** — hash normalised token streams of
   free functions across all consumer/host crates and fail on exact collisions with an allow-list.
   This is the highest-leverage new guard: it catches D5, D9 (identical `SymbolKind` tables), D17
   (identical `digest`), D7's byte-identical copies, and `language_service`'s per-module converter
   pairs, in one check.
5. **`sysml_file_admission_has_one_owner`** — grep-level: fail on any `"sysml"`/`"kerml"` string
   literal outside `sysml_source/src/lib.rs` and the `kpar` archive-format module. Catches D11 and
   its case-sensitivity divergence.
6. **`severity_labels_agree`** — a unit test asserting `workspace::comparison::severity_label` and
   `server::reports::severity_label` return equal strings for every variant, or better, that only
   one such function exists. Catches D12's live inconsistency.

## 5. Migration order (small green commits)

| # | Commit | Parallel? |
|---|---|---|
| 1 | `refactor(lsp): rename src/workspace → src/session` (mechanical; removes the name shadow) | **A** |
| 2 | `refactor(lsp): single range projection` — collapse D7's six copies onto `common::text_span::to_lsp_range`, and `language_service`'s per-module pairs onto one converter; generalise the `text_span.rs:39` guardrail (guard 3) | **A** |
| 3 | `refactor(lsp): drop duplicate utf16_len / byte_offset / path_to_file_url` — D3, D5, D6 onto `language_service` + `sysml_query::source`; add guard 4 | **A** |
| 4 | `refactor: one symbol-kind vocabulary` — D8, D9; one `SymbolKind` table and one `SymbolEntry` type from `language_service`; delete the round-trip | **B** (after 1) |
| 4b | `fix: severity labels agree` — collapse D12 onto `sysml_diagnostics::reporting`; add guard 6. **Behaviour change** (`"information"` vs `"info"` in CLI output) — land alone with a CHANGELOG note | **B** |
| 4c | `refactor: one SysML file-admission predicate` — D11 onto `sysml_source::is_sysml_like`; fixes the case-sensitivity divergence; add guard 5 | **E** |
| 4d | `refactor: one artifact digest and one display-name fallback` — D17, D18 | **E** |
| 5 | `feat(diagnostics): reporting policy owns cascade collapse and shadowing` — move `diagnostics_postprocess.rs` to `sysml_diagnostics`; delete the syntax_authority exemption | **C** |
| 6 | `feat(language_service): library search ranking and labels` — move `library_search.rs`; `views/library_search_adapter.rs` stays | **C** |
| 7 | `feat(query): documents_affected_by and library closure loading as typed queries` — `import_graph.rs`, `library_closure.rs` leave the host | **D** (touches the authority; keep serial) |
| 7b | `refactor(lsp): folding has one derivation` — delete `collect_brace_folding_ranges` (D10); if the parser fallback is genuinely needed, it becomes a syntax-service recovery mode | **D** |
| 7c | `refactor(catalog): library roots default once` — D13, D14 | **C** |
| 8 | `feat(workspace): own the validation path` — move `validation/**`, collapse D1 onto `HostValidationReport`, D2 and D15 disappear (one entry point, policy chosen by the caller) | serial (after 5) |
| 9 | `refactor(server): reach validation through workspace` — retarget `reports.rs`; drop the second library-advice derivation (D16); `lsp_server` dep becomes launch-only | serial (after 8) |
| 10 | `refactor(lsp): drop the workspace dependency` + `test(arch): pin lsp_server and server dependency sets` | serial (after 9) |
| 11 | `test(arch): no SysML text entry points in hosts; no document-keyed maps outside the session` — guards 1 and 2, with the ratchet on `MAX_ALLOW_ATTRIBUTES_IN_SRC` | serial (last) |
| 12 | `docs: sibling hosts — design.md crate map and dependency table` | serial (last) |

Worktree groups **A**, **B**, **C**, **D**, **E** are independent of each other; within a group commits are
ordered. Commits 8–12 must land in sequence on one branch because each changes the crate graph and
the guard that pins it.
