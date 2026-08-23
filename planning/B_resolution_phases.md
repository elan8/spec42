# Proposal B — an internal phase architecture for `sysml_resolution`

Status: analysis + proposal. No code changes made.

## 0. Headline

The pipeline inside this crate is **already phase-ordered and barrier-disciplined at runtime** —
`SemanticModelBuildCoordinator::build_measured_with_library` (`model.rs:15142`) and
`SemanticModelStorage::resolve` (`model/resolver.rs:4827`) read like a textbook barrier sequence,
with comments naming the barriers. What is missing is any *structural* expression of those phases:
three files hold 40.5k of the 60k lines, `model/resolver.rs:11` is `use super::*` over the whole
20.9k-line `model.rs`, and 440 of 552 inline tests are full-build contract tests living next to the
code they happen to have been written beside. The risk in the brief is real, but it is a **module
and test-placement problem, not an ordering problem**. That makes it cheap to fix: almost all of it
is `mod` moves with no logic change.

## 1. The pipeline as it exists today

Ordering is authoritative in the two functions above; the table follows their call order.

| # | Phase | Writer (fn / type) | Reads | Produces | Where today (approx.) |
|---|---|---|---|---|---|
| 0 | Source admission | `SourceInput`, `BuildRequest::new`, `SourcePayload` | host-supplied text/handles | `OwnedSourceRecord`, `PublicationIdentity` | `lib.rs:163–640` |
| 1 | Parse | `SemanticModelBuildCoordinator::parse_source`; `syntax::ParsedSource` memo | `SourcePayload` | `Arc<ParsedDocument>` + `Vec<ParseError>` | `model.rs:15236–15259`; `syntax/parsed.rs` |
| 1b | Syntax-fidelity queries | `syntax::{outline, token_ranges, closure_targets, keywords}` | parsed tree | outline, folding, token roles, closure facts | `syntax/*` (5.4k) |
| 1c | Library closure | `library::LibraryClosureAuthority`, `PackageIndex` (memoised by listing digest) | closure facts + roots | library `SourceDocument` set | `library/mod.rs` |
| 2 | Lower (AST → elements) | `SemanticModelBuilder` — `admit_document`, `canonicalize_document`, ~200 `lower_*` fns | parsed trees | `Declaration`, `MembershipRecord`, `AuthoredReference`, `DeclarationFacts`, docs, unsupported/recovery records, pending evaluation facts | `model.rs:3015–14910` (11.9k) |
| 2b | Interning / dense ids | `SymbolTableBuilder`, `SymbolPathArenaBuilder` | lowered names | `SymbolTable`, `SymbolPathArena` | `model.rs:14913–15140` |
| 2c | Barrier | `SemanticModelBuilder::freeze` | builder | `SemanticModelStorage` (immutable) | `model.rs:3820` |
| 3 | Name resolution / scoping | `resolve_dense` → `resolve_dense_with_limit` (bounded to convergence) | storage + optional `SettledLibrary` seed | `ResolutionResults` (outcomes, inherited names), `NameIndex` ×2, `MembershipIndex` | `model/resolver.rs:4977–~5900` |
| 4 | Specialization / implied relationships | `library_specialization_anchors` (`resolver.rs:5902`), `synthesize_generated_library_specializations`, `…_redefinitions`, `synthesize_feature_membership_type_featurings` | storage + settled outcomes | `implied_relationships`, `library_specialization_anchors` | `resolver.rs:4849–4885` |
| 5 | Evaluation | `compute_evaluation` (`resolver.rs:1542`), `model/evaluation.rs`, folders `fold_arithmetic`/`fold_logical`/`fold_unary` (`model.rs:1523–2300`) | storage + resolution + `EvaluationPolicy` | `SettledEvaluation` facts, filter conditions | split across `model.rs` and `resolver.rs` |
| 6 | Index / derived-fact build | `IdentityIndex`, `DocumentIndex`, `ReverseReferenceIndex`, `EffectiveScopeIndex`, `inspection::ElementFactIndex`, `binding::BindingConnectorIndex`, `types::TypeIndex`, then `expression::ExpressionIndex` | storage + resolution + evaluation | the eight indexes on `ResolvedSemanticModel` | `resolver.rs:4901–4944` + `resolver/{inspection,binding,types,expression}.rs` |
| 7 | Conformance | `resolver/{conformance,expression_conformance,host_conformance,structural}.rs` (4.1k) | assembled model | conformance findings | called from phase 8 |
| 8 | Diagnostics | `ResolvedSemanticModel::derive_diagnostics` (`resolver.rs:2375`) | assembled model only | `diagnostics`, `diagnostics_by_document` | `resolver.rs:2375–2590` |
| 9 | Query surface | `ResolvedSemanticModel` methods (`target_at`, `references`, `prepare_rename`, `visible_members`, `direct_types`, `*_derived_*`) | finished model | typed `QueryOutcome`s | `resolver.rs:1828–4760` (~2.9k) |
| 9b | Contract projections | `inspection.rs`, `details.rs`, `diagram_query.rs`, `*_query.rs`, `diagnostics.rs`, `evaluation.rs`, `traceability.rs`, `type_query.rs` | `PublishedResolution` | public contract values | `src/*.rs` (~3.3k) |
| 9c | Canonical rendering | `resolver/writer.rs` | finished model | S-expression text | `model/resolver/writer.rs` |
| 10 | Publication / identity | `publication::PublicationAuthority`, `PreparedPublication`, `publication/session.rs` | request + library stratum cache | `PublishedResolution`, tokens, lifecycle | `publication/*` (0.8k) |

Phases 3→8 are strictly ordered by data dependency and never loop back. Phase 5's *classification*
half runs inside phase 2 (see below), and phase 1c runs before phase 0 completes for library files.

## 2. Ordering and ownership violations (worst 10)

1. **`model/resolver.rs:11` — `use super::*`.** The entire resolution phase imports the entire
   lowering phase's private surface by glob. Nothing prevents a future resolver edit from calling a
   `lower_*` builder method; nothing documents what resolution actually needs. This is the single
   structural defect that makes every other one possible.
2. **`model.rs:1523–2300` — evaluation folding lives in the lowering file.** `fold_arithmetic`,
   `fold_logical`, `fold_unary`, `fold_eval_node`, `literal_expression_value` are phase-5 logic
   physically inside phase 2 and reachable from every `lower_*`.
3. **Evaluation has two writers.** `classify_constraint_expression` / `classify_calc_expression`
   (`model.rs:2113`, `2152`) run during lowering and are stored via `push_evaluation_fact`
   (`model.rs:3737`); `compute_evaluation` (`resolver.rs:1542`) settles the same fact category
   later. One category, two producers in two phases — exactly the "derived fact with more than one
   derivation owner" AGENTS.md forbids.
4. **Diagnostic-bearing facts decided mid-construction.** `push_unsupported` (`model.rs:3717`) and
   `push_recovery` (`model.rs:3725`) commit `UnsupportedFamily`/recovery decisions during lowering,
   while `derive_diagnostics` (`resolver.rs:2375`) is the declared owner of the diagnostic contract.
   The *code* is chosen at lowering time; only its rendering is deferred.
5. **`resolver.rs:4855` — phase 4 mutates phase 3's product in place.** `resolution.implied_relationships.into_vec()`
   then extend/sort/dedup, writing back onto the `ResolutionResults` the solver returned. Implied
   relationships should be their own store keyed off a frozen `ResolutionResults`, so a reader
   cannot observe the pre-synthesis value.
6. **`resolver.rs:4933–4948` — the model is published half-built.** `expressions` and `diagnostics`
   are initialised to `default()` inside the struct literal, then overwritten. Between those points
   `&model` is passed to `ExpressionIndex::build` — an index builder holding a model whose
   diagnostics are silently empty. No type distinguishes `Resolved` from `Complete`.
7. **`resolver.rs:4761–4786` — resolution reaches up into the coordinator.** `prepared_library`
   constructs `super::PreparedLibrary` / `super::PreparedDocument` / `super::CoordinatorError`;
   phase 3–6 code building phase 10's types.
8. **`model.rs:15141–15270` — the coordinator lives inside the lowering file.** The owner of the
   *whole* phase order (parse ordering policy, `source_admission_rank`, timing) is 100 lines at the
   bottom of a 20.9k-line file whose subject is lowering.
9. **`resolver.rs:1828–4760` — the query phase lives inside the resolution phase.** ~2.9k lines of
   read-only phase-9 methods interleaved with solver internals in one file, sharing the same
   `use super::*` scope. A query method can reach solver state directly.
10. **`diagram_query.rs:378–1100` — projection-time semantic derivation.** `diagram_view` runs a
    `VecDeque` BFS with `stable_key()` string keys over the model on every call, and
    `diagram_candidate_selected` (`:758`) decides membership at query time. This is a derived-fact
    computation in the projection layer rather than a phase-6 index; it also has no memo, so it is
    recomputed per request.

Honourable mentions: `lib.rs` mixes the public contract types, `BuildRequest`/identity, and the
`PublishedResolution` delegating surface in 8.5k lines; `resolver/host_conformance.rs` (1.9k) sits
under `model/resolver` although it consumes the finished model.

## 3. Target module layout

```
src/
  lib.rs                     # crate contract types + PublishedResolution delegation ONLY
  pipeline/
    mod.rs                   # the phase order, made explicit and readable in one screen
    admission.rs             # phase 0: SourceInput/SourcePayload/BuildRequest/PublicationIdentity
    schedule.rs              # source_admission_rank, BuildSchedule, BuildPhaseDurations
  syntax/                    # unchanged (phase 1/1b)
  library/                   # unchanged (phase 1c)
  lower/
    mod.rs                   # SemanticModelBuilder + admit/canonicalize/freeze
    facts.rs                 # DeclarationFacts, MembershipRecord, modifiers, multiplicity records
    intern.rs                # SymbolTable(Builder), SymbolPathArena(Builder)
    parts.rs  actions.rs  states.rs  requirements.rs  connections.rs
    views.rs  constraints.rs  kerml.rs  metadata.rs   # the ~200 lower_* fns, by family
    storage.rs               # SemanticModelStorage (the phase-2 barrier product)
  resolve/
    mod.rs                   # resolve_dense + the convergence bound
    names.rs                 # NameKey, NameIndex, EffectiveScopeIndex, MembershipIndex
    results.rs               # ResolutionResults, ResolutionStatus, SolverStatus (frozen)
    implied.rs               # phase 4: anchors + synthesis, own store, no write-back
    library_seed.rs          # SettledLibrary admit/seed
  evaluate/
    mod.rs                   # compute_evaluation, the single evaluation writer
    fold.rs                  # fold_arithmetic/logical/unary, literal_expression_value
    classify.rs              # ExpressionEvalShape classification (moved out of lowering)
  index/
    identity.rs  documents.rs  reverse_references.rs
    elements.rs  bindings.rs  types.rs  expressions.rs
    diagrams.rs              # NEW: the phase-6 owner of diagram projection facts
  check/
    mod.rs  conformance.rs  expression.rs  host.rs  structural.rs
  diagnose/
    mod.rs                   # derive_diagnostics, the sole diagnostic writer
    codes.rs                 # DiagnosticCode contract (from diagnostics.rs)
  model/
    mod.rs                   # ResolvedSemanticModel struct + phase-typed assembly
    query/                   # phase 9: navigation, references, rename, scopes, types, derived
    render.rs                # writer.rs
  contract/                  # phase 9b: inspection, details, diagram, *_query, traceability
  publication/               # unchanged (phase 10)
```

Purpose, one line each where not obvious: `pipeline` is the only module that names more than one
phase, and is the readable statement of the order. `lower` owns authored facts and nothing else —
after the split it has no evaluation and no diagnostic vocabulary. `resolve` owns settled outcomes;
`implied.rs` publishes a separate store rather than editing `results.rs`'s. `evaluate` becomes the
single writer for evaluated values, receiving classification that lowering currently performs.
`index` holds every derived-fact store built at the phase-6 barrier, and gains `diagrams.rs` so
`contract/diagram.rs` becomes a pure projection. `check` and `diagnose` split rule evaluation from
diagnostic emission. `model/query` is read-only over a finished model. `contract` contains no
derivation. The facade-facing `pub use` list in `lib.rs` is unchanged throughout.

## 4. Enforcement — recommendation

**Recommended: a phase-typed assembly + an architecture test, in that priority.**

Primary (compile-time, cheap): give each phase a writer type that takes `&mut` only its own store
and *consumes* the previous phase's product — `Lowered → Resolved → Evaluated → Indexed → Complete`,
each a distinct type, with `ResolvedSemanticModel` reachable only from `Complete`. This deletes
violations 5 and 6 by construction: there is no half-built model to observe and no earlier store to
write back into, because the previous phase's value has been moved. Replace `use super::*` with
explicit imports as part of the same move.

Secondary (test-time, catches the rest): extend the existing pattern in
`crates/sysml_query/tests/architecture.rs` with a `phase_order.rs` that parses `use` statements per
module and asserts a phase-rank table — `lower` never names `resolve`/`evaluate`/`diagnose`,
`resolve` never names `pipeline`/`publication`, `contract` never names `resolve`/`lower`. Also ban
`use super::*` inside `src/` outside `#[cfg(test)]`.

Tradeoff, three lines:
- The type-state split forces every phase-crossing helper to be re-plumbed explicitly, which is
  real churn in the ~30 places that today read whatever they like through the glob.
- It cannot express "no lazy recomputation" or "no derivation in `contract/`" — only the `use`-graph
  test can, and that test is a heuristic that a determined `crate::` path can evade.
- Together they cost roughly one commit each and are the only two mechanisms the constrained code
  cannot disable from inside its own module.

## 5. The 552 inline tests

Measured: `lib.rs` 211, `model.rs` 230, `model/resolver.rs` 58, remainder 53. Of these, **~440 build
a full `BuildRequest` and assert against a rendered S-expression** (`model.rs` 238 sexpr call sites,
`lib.rs` 106 plus 54 `BuildRequest::new`). Those are contract tests wearing unit-test clothes: they
exercise parse→publish end to end and assert on the canonical projection, i.e. exactly the surface
`sysml_query` re-exports.

- **Move to `crates/sysml_resolution/tests/`, driven through the public `build()`/`PublishedResolution`
  surface** (and, where the assertion is really about the facade contract, up to `sysml_query`'s
  tests): every `*_lowers_to_a_declaration`, `*_resolves_its_*_reference`, `*_evaluates_to_*`,
  `unsupported_*`, and diagnostics-sexpr test in `model.rs:15437–20800` and `lib.rs:1194–8400`.
  Suggested files mirroring phases: `tests/lowering_contract.rs`, `tests/resolution_contract.rs`,
  `tests/evaluation_contract.rs`, `tests/diagnostics_contract.rs`, `tests/navigation_contract.rs`.
- **Keep inline, phase-local** (~110): the genuine internals — `canonicalization_assigns_dense_typed_slots_and_interns_names`,
  `symbol_interning_survives_hash_table_growth`, `semantic_paths_are_interned_across_arena_growth`,
  `document_identity_index_rejects_duplicates_after_growth_without_mutation`,
  `anonymous_ordinals_are_owner_local_*` (`model.rs:15292–15436`) → `lower/intern.rs`;
  the 58 in `resolver.rs` (solver bound, seed admission, index construction) → `resolve/`, `index/`;
  `library/mod.rs` 12, `publication/*` 10, `syntax/parsed.rs` 5 stay where they are — they test memo
  and lifecycle mechanics with no public expression.
- Net effect: no test is deleted or weakened, and every phase module ends up small enough that its
  remaining inline tests fit its own file.

## 6. Migration order

Each step is one green commit unless noted. Steps 1–7 are pure `mod` moves — no logic change, so
they are reviewable by `git diff -M` rename detection.

1. `pipeline/` extracted: move the coordinator, `source_admission_rank`, `BuildSchedule` out of
   `model.rs:15141–15270`. Smallest possible first cut; makes the phase order visible.
2. `lower/intern.rs`: move `SymbolTable*` / `SymbolPathArena*` (`model.rs:14913–15140`) + their 5 tests.
3. `lower/facts.rs`: move the record/enum block (`model.rs:2308–2975`).
4. `evaluate/fold.rs` + `evaluate/classify.rs`: move `model.rs:1452–2300`. First commit that changes
   a module boundary rather than just a file — verify no `lower_*` still calls a folder afterwards.
5. Split the `lower_*` families into `lower/{parts,actions,states,requirements,connections,views,
   constraints,kerml,metadata}.rs` (`model.rs:3840–14910`). 3–4 commits; `model.rs` ends near zero.
6. Split `model/resolver.rs`: `resolve/`, `index/`, `model/query/` (`resolver.rs:1828–4760`), keeping
   `diagnose/` and `check/` as moves of the existing `resolver/*.rs` files.
7. Replace `use super::*` with explicit imports in every new module. This is the commit that will
   actually fail to compile repeatedly — budget for it.
8. Phase type-state: introduce `Lowered/Resolved/Evaluated/Indexed/Complete`; fix violations 5, 6, 7
   as part of it. First behaviour-adjacent commit; keep it separate from every move.
9. `index/diagrams.rs` + `contract/diagram.rs` split (violation 10). Independent of 8.
10. Add `tests/phase_order.rs`.
11. Test relocation, one phase per commit, in the order of §5.

**Parallelism: essentially none for steps 1–7.** They all edit the same two files (`model.rs`,
`model/resolver.rs`), so any two of them in separate worktrees conflict on every hunk, and rename
detection is lost the moment two branches move overlapping ranges. Do them strictly sequentially on
one branch. Steps 9, 10, and 11 *can* run in parallel worktrees once step 7 has landed: step 9
touches `diagram_query.rs`, step 10 adds a new file, and step 11 only removes `#[cfg(test)]` blocks
and adds files under `tests/`. Step 8 must land before 11 finishes only if the type-state changes
any assertion, which it should not.
