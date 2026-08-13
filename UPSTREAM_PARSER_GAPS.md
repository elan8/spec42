# Upstream sysml-v2-parser gaps blocking spec42 snapshot work

Tracks semantic gaps discovered while closing the snapshot delta on the new parser-owned
pipeline (branch `closing-the-gap`, PR lukewilliamboswell/spec42#6) that trace back to the
pinned `sysml-v2-parser-next` revision rather than to `sysml_resolution`/`sysml_query`. Each
entry should carry enough detail to file/update an upstream issue against
`feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Open

All 12 gaps previously tracked here were confirmed resolved upstream in `0757de13` (see
`## Resolved / not blocked` below for the per-gap notes). Nothing is currently open.

## Resolved / not blocked (kept for history)
- Gap 1. Top-level `feature` declarations were unparsed grammar. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 2. Class specialization (`:>`) inside a `class` body was unparsed. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 3. `CalcDef` dropped the parsed `:>` specialization clause. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 4. `ConstraintUsage` dropped the parsed `:>`/`:>>` specialization clauses. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 5. `AnalysisCaseUsage`/`CaseUsage` dropped the parsed `:>`/`:>>` specialization clauses. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 6. `InterfaceUsage` had no `subsets`/`redefines` fields. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 7. `individual <kind> <name>;` short usage forms were misparsed/unparseable for `item`/`occurrence`/`port`. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 8. `ViewUsage` had no `subsets` field. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 9. `ConcernUsage` had no `specializes`/`subsets`/`redefines` field. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 10. Bare forward-declared `classifier X;` collapsed to a raw-text fallback node. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 11. `item <name> : <Type>;` nested in an attribute body was captured opaquely. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 12. `#<keyword> def <Name> ...` ExtendedDefinition short form had no grammar production. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).


- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).
