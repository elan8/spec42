# Upstream sysml-v2-parser gaps blocking spec42 snapshot work

Tracks semantic gaps discovered while closing the snapshot delta on the new parser-owned
pipeline (branch `closing-the-gap`, PR lukewilliamboswell/spec42#6) that trace back to the
pinned `sysml-v2-parser-next` revision rather than to `sysml_resolution`/`sysml_query`. Each
entry should carry enough detail to file/update an upstream issue against
`feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Open

### 1. Top-level `feature` declarations are unparsed grammar

- **Symptom:** `feature x : Integer;` at package scope produces `unsupported_grammar_form`
  (source: parser), cascading to `unsupported_package_member` (semantic). Zero declarations
  published.
- **Representative input:** `feature x : Integer;`
- **Representative snapshot:** `test/snapshots/feature_typing.md`
- **Scale:** ~131/503 snapshots blocked as of this writing (largest single blocked bucket).
- **Status:** reported to Luke directly (parser repo owner); branch already diverges from
  `main`, which has more grammar coverage merged. Likely a rebase/merge issue rather than
  missing grammar work, per other local `sysml-v2-parser` worktrees at `main`.

### 2. Class specialization (`:>`) inside a `class` body is unparsed

- **Symptom:** `class B :> A { }` produces the same `unsupported_grammar_form` /
  `unsupported_package_member` pair as (1).
- **Representative snapshot:** `test/snapshots/class_specialization.md`
- **Status:** same as (1) — reported, awaiting upstream branch update.

### 3. `CalcDef` drops the parsed `:>` specialization clause

- **Symptom:** `calc def X :> Y { ... }` parses successfully (no `unsupported_grammar_form`,
  no parser-level error) but the `:>` specialization relationship is silently discarded before
  it ever reaches `sysml_resolution`. `ast::view::CalcDef` (crate `sysml-v2-parser`,
  `src/ast/view.rs`) has only `identification`, `body`, and `membership` fields — no
  `specializes: Option<Node<TypingRelationship>>` field, unlike the structurally analogous
  `ActionDef`/`ItemDef`/`PartDef`/`RequirementDef` etc., which all carry one. The shared
  `parse_definition_prefix` helper (`src/parser/definition_prefix.rs`) *does* parse the `:>`
  clause into `DefinitionPrefixResult::specializes` for every definition kind including calc —
  `parser::constraint::parse_calc_def` (`src/parser/constraint.rs`) simply never reads
  `prefix.specializes` when constructing the `CalcDef` node, so the parsed relationship is
  thrown away rather than surfaced as unsupported. This is a genuine typed-AST gap, not a
  grammar/tokenizing gap: the parse succeeds and the information exists transiently inside the
  parser but never reaches the AST.
- **Representative input:** `abstract calc def Calculation :> Action, Evaluation { ... }`
  (Systems Library `Calculations.sysml`).
- **Representative snapshots:** `test/snapshots/sysml.library/calculations.md`,
  `test/snapshots/sysml.library/vector_calculations.md`,
  `test/snapshots/sysml.library/tensor_calculations.md`,
  `test/snapshots/sysml.library/spatial_items.md`,
  `test/snapshots/sysml.library/time.md`,
  `test/snapshots/sysml/examples/sys_ml_v2_spec_annex_a_simple_vehicle_model.md` — 6 of the 26
  fixtures using `calc def` specialize via `:>`.
- **Impact:** blocks lowering `calc def`/`calc` declaration facts (`DeclarationKind::
  CalculationDefinition`) with full parity to the other `*Definition` kinds, since the
  in-scope requirement includes `:>` specialization resolving through the existing
  ancestor-closure fixed point (same shape as `action def`, see commit `f4ae83f7`) and the typed
  AST currently offers no field to lower that relationship from for `calc def` specifically
  (`CalcUsage.redefines`/`type_name` are present and fine; only `CalcDef.specializes` is
  missing).
- **Status:** blocking. Needs an upstream `sysml-v2-parser` change adding
  `specializes: Option<Node<TypingRelationship>>` to `ast::view::CalcDef` and wiring
  `parser::constraint::parse_calc_def` to populate it from `prefix.specializes` (mirroring
  every other `*_def` parser in that module). Not attempted here — out of scope for
  `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around without either (a)
  re-parsing the specialization clause independently (duplicating parser logic, fragile) or (b)
  shipping calc def declarations with unconditionally-absent specialization facts, silently
  under-reporting a relationship that real Systems Library content depends on.

## Resolved / not blocked (kept for history)

- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).
