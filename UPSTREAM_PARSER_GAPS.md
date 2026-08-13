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

## Resolved / not blocked (kept for history)

- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).
