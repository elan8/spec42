# SysML v2 Parser — Fixes Needed

This document tracks known gaps and bugs in the `sysml-v2-parser` crate and in Spec42's
graph-builder coverage of the AST it produces. Items are grouped by whether the fix
belongs in the **upstream parser** or in **Spec42's own graph-builder / semantic layer**.

---

## 1. Upstream parser (`sysml-v2-parser` crate)

### 1.1 `advance_to_closing_brace` recovery sites still unstructured

**Status:** Known backlog (noted in [`AST-SEMANTIC-COVERAGE.md`](AST-SEMANTIC-COVERAGE.md))

In several body contexts the parser calls `advance_to_closing_brace` as a recovery
strategy instead of producing structured AST nodes. This means Spec42 receives an
`Other` or `Error` variant and cannot walk the inner members. Affected sites:

| Body context | Impact |
|---|---|
| `action` ref bodies (`ref action …`) | Ref-action targets not resolved; no hover/go-to |
| In-out defaults (`in out … = expr`) | Default value expressions dropped |
| Satisfy connect bodies | Satisfaction relationships not emitted |

**What needs to change in the parser:** Replace `advance_to_closing_brace` at each
site with a proper body-element enum variant so callers can match on structured data.

---

### 1.2 `calc def` body members parsed but not as structured child nodes

**Status:** Parser produces `CalcDefBody::Brace { elements }` with `CalcDefBodyElement`
variants — but the variants for parameters (`InOutDecl`) and return value (`ReturnDecl`)
are not exposed with enough span information to create first-class graph nodes.

**What needs to change:** Parameters and return declarations inside a `calc def` body
need stable name/type spans so Spec42 can project them as typed children (see §1.2 in
Spec42 graph-builder gap below). If the parser already exposes these spans today, the
issue is purely in the graph-builder (see §2.1).

---

## 2. Spec42 graph-builder / semantic layer

These items require changes inside this repository, not in the upstream parser.

### 2.1 `calc def` body — parameters and return not in the semantic graph

**File:** [`crates/sysml_model/src/semantic/graph_builder/part_def.rs:472`](../../crates/sysml_model/src/semantic/graph_builder/part_def.rs)

```rust
if let CalcDefBody::Brace { .. } = &calc_node.value.body {
    // Calc body members (parameters, return) are not expanded into the graph yet.
}
```

Parameters and the return declaration of a `calc def` are silently ignored. They do
not appear as child nodes, are not typed, and produce no hover or navigation targets.
`package_body.rs` extracts them into JSON attributes (`extract_calc_metadata`) for
display purposes only — they are not wired into the graph edge/node model.

**Fix:** Walk `CalcDefBodyElement::InOutDecl` and `ReturnDecl` in `part_def.rs` the
same way action body parameters are handled in `action.rs`.

---

### 2.2 `Error`, `Other`, and `OpaqueMember` AST variants — minimal coverage

**File:** [`docs/engineering/AST-SEMANTIC-COVERAGE.md`](AST-SEMANTIC-COVERAGE.md) (policy row)

| Variant | Graph | Hover | Semantic tokens |
|---|---|---|---|
| `Error` / `Other` | Ignored | Ignored | Ignored |
| `OpaqueMember` | Minimal shell node | Partial | Ignored |

When the upstream parser cannot parse a member it emits one of these variants. Spec42
currently ignores them entirely. The result is that tokens inside a recovery region
fall back to the lexer-only highlighter (which cannot know types) and no diagnostics
are shown for the unparsed region.

**Fix (iterative):**
1. Emit a `SemanticNode` with `kind: ElementKind::Unknown` so the region is at least
   visible in the graph (enables future diagnostics).
2. Extend `ast_ranges.rs` to emit `TYPE_VARIABLE` / `TYPE_KEYWORD` tokens from the raw
   span when `OpaqueMember` carries a text range.

---

### 2.3 `DefinitionBodyElement` for occurrence and rendering families

**File:** [`docs/engineering/AST-SEMANTIC-COVERAGE.md`](AST-SEMANTIC-COVERAGE.md) (backlog section)

`occurrence def`, `rendering def`, and related definition bodies that carry
`DefinitionBodyElement` members beyond their shell node are not fully walked. The
general-view projection only sees the top-level definition; compartment detail
(inner attributes, constraints, usage members) is missing.

**Fix:** Extend `definition_body.rs` and `occurrence_body.rs` to walk the remaining
`DefinitionBodyElement` variants needed by the general-view compartment projection.

---

### 2.4 Expression evaluation — many patterns return `Unsupported`

**File:** `crates/sysml_model/src/semantic/evaluation/mod.rs`

Many expression forms return `EvalStatus::Unsupported`. This affects:
- Filter expressions on view `expose` targets
- Guard conditions on state transitions
- Constraint body expressions
- Analysis value expressions

The result is that these values cannot be statically evaluated, which blocks certain
diagnostic checks and prevents value display in hover.

**Fix (incremental):** Extend `expressions.rs` evaluator for the most common patterns
(arithmetic literals, boolean combinators, feature reference chains) before tackling
general `InvocationExpression`.

---

### 2.5 `doc` and annotation members — intentionally deferred

**File:** [`docs/engineering/AST-SEMANTIC-COVERAGE.md`](AST-SEMANTIC-COVERAGE.md) (WONTFIX row)

`doc "…"` members and standalone `comment` blocks are ignored. This is a deliberate
WONTFIX decision for the 1.0 release. Noted here for completeness — no action needed.

---

## 3. Diagnostic recovery quality

The parser emits structured error codes when it falls back to recovery. Spec42
post-processes these (`diagnostics_postprocess.rs`) and suppresses cascade errors.
The following recovery codes indicate body contexts that produce poor AST structure
and may warrant upstream parser improvements:

| Error code | Meaning | Likely upstream fix |
|---|---|---|
| `bare_feature_declaration_in_part_def` | Feature missing type/default | Structured recovery variant |
| `invalid_requirement_short_name_syntax` | Malformed `#short-name` | Structured parser rule |
| `recovered_root_body` | Root-level recovery | Skip whitespace at loop start (see §1.x) |
| `missing_body_or_semicolon` | Incomplete statement | Structured termination recovery |

For each: if the parser can produce a partial structured node instead of emitting a
recovery string, Spec42 can provide better diagnostics and tokens.

---

## Summary

| # | Location | Effort | Impact |
|---|---|---|---|
| 1.1 | Parser: `advance_to_closing_brace` sites | High (parser PR) | Ref-action, in-out, satisfy |
| 1.2 | Parser: `calc def` span coverage | Low–Medium | Calc parameter types |
| 2.1 | Graph-builder: `calc def` children | Medium | Hover, navigation |
| 2.2 | Graph-builder: `Error`/`Opaque` nodes | Low (shell only) | Token fallback, diagnostics |
| 2.3 | Graph-builder: occurrence/rendering body | Medium | General view compartments |
| 2.4 | Evaluator: expression coverage | High | Diagnostics, hover values |
