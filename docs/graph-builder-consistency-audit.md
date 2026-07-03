# `sysml_model` Graph Builder Consistency Audit

**Date:** 2026-07-03
**Scope:** `crates/sysml_model/src/semantic/graph_builder/` — the AST-to-`SemanticGraph` lowering
pass. Triggered by a review of whether the graph builder consistently produces the right nodes
and edges across all branches, and whether nodes/edges stay in sync with the AST.
**Cross-referenced against:** `docs/architecture-audit.md` items #5 (untyped `attributes` bag)
and #26 (`package_body.rs` size), `SysML_v2.txt` (spec reference at
`elan8-monorepo/library/SysML_v2.txt`).

---

## Completed this session

| # | Finding | Fix | Files |
|---|---|---|---|
| 1 | `PartUsage` and `AttributeUsage` were hand-duplicated across 3 call sites (`package_body.rs`, `part_def.rs`, `part_usage.rs`). `part_def.rs`'s copy of `PartUsage` silently dropped the `usagePrefix` attribute the other two set. | Extracted `materialize_part_usage`/`materialize_attribute_usage` into new `usage_builders.rs`; all 3 sites now call the shared function. | `usage_builders.rs` (new), `package_body.rs`, `part_def.rs`, `part_usage.rs` |
| 2 | Dead stub: `stubs.rs::relationships_from_part_def()` was a no-op called from 2 sites, with a misleading comment. | Deleted `stubs.rs` and both call sites. | — |
| 3 | Specializes-edge wiring order was inconsistent: ~10 of the ~15 def-kind match arms wired `Specializes` *after* recursing into the def's body instead of before, breaking inherited-member resolution (e.g. `attribute redefines <inheritedPort>`) for any member declared in the body. No documented rationale found for the "after" ordering (`git log -S` on the introducing commit showed none); `add_specializes_edge_if_exists` only requires the *source* node to exist, which happens before either ordering. | Moved `wire_def_specialization_edge`/`add_specializes_edge_if_exists` to run before the body-recursion loop in all affected arms. | `package_body.rs` (10 arms), `part_def.rs` (nested `PartDef`), `view_def.rs` (`build_view_def`, `build_viewpoint_def`) |
| 4 | `OccurrenceUsage` inside a `part`-usage body (`part_usage.rs`) never recursed into its own body at all — the only one of 4 call sites (`package_body.rs`, `part_def.rs`, `part_usage.rs`, `occurrence_body.rs`) with this gap. Any children of a nested `occurrence { ... }` inside a `part` usage were silently dropped from the graph. | Extracted `materialize_occurrence_usage` into `usage_builders.rs`; all 4 sites now share it. | `usage_builders.rs`, `package_body.rs`, `part_def.rs`, `part_usage.rs`, `occurrence_body.rs` |
| 5 | `RequirementUsage` at the top-level package body (`package_body.rs`) never read the `subsets` field — the only one of 3 call sites (`package_body.rs`, `part_def.rs`, `state.rs`) with this gap. `requirement r2 : Req2 :> r1;` at package level silently dropped its `subsetsFeature` attribute. | Extracted `materialize_requirement_usage` into `usage_builders.rs`; all 3 sites now share it. | `usage_builders.rs`, `package_body.rs`, `part_def.rs`, `state.rs` |
| 6 | `package_body.rs`'s `build_from_package_body_element` was a single ~1150-line `match` with all 34 non-trivial constructs inlined — hard to navigate, hard to review incrementally. | Extracted each construct into its own `materialize_*` function (`materialize_part_def`, `materialize_connection_def`, `materialize_import`, ... 33 functions total); the dispatcher is now ~226 lines of one-line delegations. | `package_body.rs` (full rewrite, behavior-preserving) |
| 7 | **Spec violation** (SysML v2 §7.6.5 "Effective Names"): a usage with `redefines` but no declared name should get an *effective name* derived from the redefined feature (spec's own example: `part redefines cylinders[4];` → effective name `"cylinders"`). We instead created the node with an empty name and a malformed qualified path (e.g. `"P::SixCylinderEngine::"` with a trailing `::`), making it unaddressable by qualified name. Confirmed with the spec's exact example before and after the fix. | Added `effective_usage_name()` helper (`mod.rs`); applied at all 8 sites building a name+redefines-bearing usage node. | `mod.rs`, `usage_builders.rs`, `port_def.rs` (×3), `attribute_body.rs`, `occurrence_body.rs`, `requirement_body.rs` |

**Verification for all of the above:** full `cargo test -p sysml_model` (424 tests, 0 failures),
`cargo clippy -p sysml_model --lib --tests` (clean), regression tests added in
`crates/sysml_model/tests/usage_builder_context_parity.rs` (5 tests, each exercising the same
construct across every containing-body context it can legally appear in — this is the test
pattern that would have caught bugs #1, #4, #5, #7 before merge).

---

## Open items

### O1 — Anonymous Definitions are silently dropped from the graph (spec category B)

`ItemDef`, `MetadataDef`, `EnumDef`, `FlowDef`, `AllocationDef`, `CaseDef`, `AnalysisCaseDef`,
`VerificationCaseDef`, `OccurrenceDef` all guard on `if name.is_empty() { return; }` in
`package_body.rs`, meaning a fully anonymous definition (`item def { ... }`, no name, no short
name) is never added to the graph — its children are lost too.

**Spec status:** confirmed legal. `DefinitionDeclaration : Definition = Identification
SubclassificationPart?` and `Identification` has both `declaredName`/`declaredShortName` as
optional (`?`). Every `Element` also carries a spec-mandated `elementId` independent of name
(see O3 below), reinforcing that nameless-but-real elements are a first-class case, not an edge
case to special-case away.

**Why this is a separate problem from fix #7 above:** §7.6.5's effective-name mechanism only
applies to *Usages with an owned redefinition* (`redefines`). Definitions don't have a
`redefines` field — there is no textual source to derive a name from. A definition can be
completely, irreducibly anonymous.

**Decision needed before implementing:** how to represent an anonymous definition under our
`NodeId = (uri, qualified_name)` identity scheme, which requires a non-empty name to build a
usable path. Two options discussed, not yet chosen:
- Synthetic name per kind + ordinal, e.g. `_anonymous_item_def`, `_anonymous_item_def2` (matches
  the existing pattern in `occurrence_body.rs::add_assert_constraint_member`, which already
  synthesizes `_assertConstraint_{index}` for a different always-anonymous construct).
- Something closer to spec's `elementId` model — see O3, they're the same underlying question.

### O2 — `EnumerationUsage` silently ignored inside `part`-usage bodies

`PartUsageBodyElement::EnumerationUsage(_)` is matched into a no-op arm in `part_usage.rs`.
**Spec status:** confirmed legal and meaningful — `EnumerationUsage` is a first-class metaclass;
`Definition::ownedEnumeration` is explicitly derived as `ownedUsage->selectByKind
(EnumerationUsage)`, subsetting `ownedAttribute` (SysML_v2.txt §13046-13048). An `enum
someLiteral;` declared inside a `part` usage body is a real owned member per spec, not
decorative syntax.

Not yet scoped — needs the same "what attrs/kind does this node get" design pass as O1 before
implementing.

### O3 — No `elementId` equivalent; node identity is coupled to name

**What the spec says:** every `Element` (KerML base metaclass, SysML_v2.txt line ~12567) carries
`+elementId : String{id}` — a unique identity independent of `name`/`declaredName`. The
normative *construction* rule (name-based UUID v5) is specified only for standard-library
interchange files (§9.1, "Regardless of whether such a library model is interchanged in textual
notation, XMI or JSON format, the elementId for any Element in the library model shall be..."),
but the *property itself* is general — every element in every model has one, library or not.
It's what lets a tool track "this is the same element" across a rename, across a save/reload, or
across an XMI/JSON project-interchange round-trip.

**What we do today:** nothing analogous. `sysml_v2_parser::Node<T>` carries only `span` + `value`
— no id field, confirmed by inspecting the generated docs (`target/doc/sysml_v2_parser/ast/
struct.Node.html`). Our `NodeId = (uri: Url, qualified_name: String)` is a derived, name-based
key, not a stable identity:
- Renaming an element changes its `NodeId`.
- Two same-named siblings get disambiguated via `#kind`/`#kind2` suffixes
  (`qualified_name_for_node` in `mod.rs`) — an ordinal that can shift if sibling declaration
  order changes, i.e. not even stable *within* a single unchanged name.
- An anonymous element (O1, O2) can't get a `NodeId` at all without a synthetic name, because the
  whole indexing scheme is keyed on the qualified-name string.

**Does this matter for us?** Depends on what the tool needs to guarantee:
- If the only requirement is "build a fresh graph from source text and answer queries against
  it" (current usage: LSP-style analysis, diagnostics, views — always rebuilt from the parse),
  name-based identity is a reasonable, working simplification. This is probably why nobody hit
  this as a bug yet.
- It becomes a real gap the moment something needs identity to survive a rename, or needs to
  diff/merge two graph snapshots by element rather than by qualified name, or needs XMI/JSON
  project-interchange conformance (Clause 2, "Model Interchange Conformance" in the spec) — none
  of which `sysml_model` currently does, but "renaming shouldn't lose associated
  state/annotations/history" is a plausible future ask for any editor-adjacent tool.

**Recommendation:** don't build a parallel elementId system speculatively. Revisit if/when a
concrete feature needs stable cross-rebuild identity (e.g. a "sticky" selection across edits, an
undo/redo model, or real XMI/JSON interchange). If/when that happens, the natural seam is
`sysml_v2_parser::Node<T>` — the parser would need to assign+persist an id per parsed element
(likely a random UUID at first-parse, threaded through re-parses via position/heuristic
matching, since we don't have spec's name-based-UUID luxury for arbitrary user models). Flagging
here so it isn't rediscovered as a surprise later.

### O4 — `SemanticNode.attributes: HashMap<String, serde_json::Value>` is untyped

Already tracked in `docs/architecture-audit.md` (item #5, P2-5). ~70+ distinct string keys
(`multiplicity`, `redefines`, `value`, `portType`, `allocationType`, ...), re-parsed on hot
paths, typos fail silently. Precedent for the fix already exists in this codebase: `element_kind:
String` → `ElementKind` enum (`model.rs`). Same treatment could apply to the ~6-field cluster
shared by most usage kinds (`partType`/`attributeType`/etc., `multiplicity`, `redefines`,
`subsetsFeature`, `value`). Not started.

### O5 — No coverage test asserting every AST variant produces a graph node

We found 3 of the 4 confirmed drift bugs (#1, #4, #5) by manually diffing match arms across
files — there's no automated check that would have caught them before a human went looking. A
test that walks every variant of `PackageBodyElement`/`PartDefBodyElement`/
`PartUsageBodyElement` (etc.) with a minimal fixture and asserts *some* node is materialized
would catch future regressions of this shape. `usage_builder_context_parity.rs` covers the
specific constructs we already found bugs in, but isn't exhaustive over the AST.

---

## How to resume

Pick up at O1 (anonymous definitions) — it's the most concrete, has a clear spec citation, and
the earlier discussion narrowed the decision to two options (synthetic name vs. elementId-style
identity). O3's elementId question is deliberately *not* an action item — it's context for O1's
decision and a flag for later, not a task to schedule now.
