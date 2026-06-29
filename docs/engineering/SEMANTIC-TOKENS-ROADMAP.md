# Semantic tokens roadmap

Tracks known issues, quality gaps, and improvement priorities for the `sysml_tokens` crate and LSP semantic highlighting.

**Related:** [AST-SEMANTIC-COVERAGE.md](./AST-SEMANTIC-COVERAGE.md) (parser surface → graph/symbols/tokens matrix).

## Architecture

```
Lexer (per-line fallback) → merge ← AST ranges (ast_ranges.rs)
                              ↑
                    refine_declaration_ranges (name-only defs)
```

Legend: `keyword`, `string`, `number`, `comment`, `operator`, `variable`, `type`, `namespace`, `class`, `interface`, `property`, `function`.

VS Code highlighting is **100% LSP semantic tokens** (no TextMate grammar). Zed uses a separate tree-sitter `highlights.scm`.

---

## Known issues

### Coverage gaps

| Issue | Impact | Status |
|-------|--------|--------|
| Many `PackageBodyElement` variants fall through `_ => {}` | Constraint/case/verification/use-case/metadata usage at package level get lexer-only coloring | **P0 — done** (constraint/calc/enum/use-case/verification/case/metadata/occurrence/allocation/concern) |
| KerML decls (`FeatureDecl`, `ClassifierDecl`, …) unhandled | `.kerml` files poorly highlighted | P2 |
| Doc / comment / textual-rep members ignored | Intentional per policy | WONTFIX 1.0 |

### Span precision

| Issue | Impact | Status |
|-------|--------|--------|
| Definition nodes emitted full-span before merge | `def` could lose keyword color; names missed `class` | **Mitigated** via `refine_declaration_ranges` |
| Usages without `name_span` use full node span | `state idle` colors whole line as `property` | **P0 — done** (`name_span` + `word_range_within_span` lookup) |
| `transition` lines use full span as `property` | Transition keywords/names not individually colored | P1 |
| Text-based `refine_declaration_ranges` heuristic | Fragile for `<short> name`, multi-line headers, `:>` before body | P1 (parser `name_span` on `Identification`) |

### Merge / lexer

| Issue | Impact | Status |
|-------|--------|--------|
| `span_len > 2 * len` guard in `apply_ast_semantic_ranges` | Opaque; blocked name coloring before refine fix | P1 — revisit after precise spans |
| Lexer `last_was_colon` type heuristic is single-line only | Wrapped type refs stay `variable` | P2 |
| Qualified names (`Pkg::Type`) not per-segment | Only `::` colored as operator | P2 |
| `@` / `#` metadata tokenized as `variable` | Metadata keywords not distinguished | P2 |
| No expression-level tokens in constraints/guards | Expected per AST matrix | P2 |

### LSP / client

| Issue | Impact | Status |
|-------|--------|--------|
| No token modifiers (`declaration`, `readonly`, …) | Themes cannot distinguish def vs usage | P2 |
| No `result_id` for incremental refresh | Full document recompute every time | P2 |
| No `semanticTokenScopes` in extension | Theme-dependent inconsistent colors | **P0 — done** |
| Debug merge logging built but always disabled (`log_out: None`) | Hard to diagnose client reports | P1 |

### Tests

| Issue | Impact | Status |
|-------|--------|--------|
| Spot-check tests only (identifier present?) | Regressions in token *type* easy to miss | **P0 — done** (`golden_tokens.rs` + type assertions) |
| No systematic coverage of package-level gaps | Untested variants stay broken | **P0 — partial** (constraint/calc/enum + StateMachineDemo fixture) |

---

## Priority plan

### P0 — high ROI (current sprint)

1. ~~**Precise usage spans**~~ — `push_usage_name_type_spans`, `word_range_within_span`; `ast_semantic_ranges(root, source)`.
2. ~~**Wire missing package-level variants**~~ — constraint/calc/enum/use-case/verification/case/analysis/metadata/occurrence/allocation/concern/actor.
3. ~~**Golden semantic-token tests**~~ — `crates/sysml_tokens/tests/golden_tokens.rs`.
4. ~~**`semanticTokenScopes`**~~ — `vscode/package.json`.

Remaining P0 follow-up: KerML package members, `Satisfy`/`Dependency`, more golden fixtures (VehicleDefinitions).

### P1 — structural cleanup

5. Parser `Identification.name_span` (upstream); shrink or remove `refine_declaration_ranges`.
6. Revisit or document `span_len > 2*len` merge guard.
7. Opt-in semantic token debug logging via server config.
8. Transition / satisfy / dependency member-level spans.

### P2 — polish

9. Token modifiers in legend + emission.
10. `semanticTokens/resultId` for incremental updates.
11. Expression/constraint highlighting.
12. KerML package members; align Zed tree-sitter with LSP legend.

---

## Quality scorecard (baseline)

| Area | Grade | Notes |
|------|-------|-------|
| Core structural SysML (part/port/action/state/requirement) | B+ | Primary investment area |
| Definition keyword vs name | B | After `refine_declaration_ranges` |
| Cases / verification / constraints at package level | C- → B- | P0 target |
| KerML | D | Unhandled |
| Parse-error fallback | B | Lexer usable |
| Cross-editor consistency | C | VS Code LSP vs Zed tree-sitter |
| Test confidence | C → B- | P0 golden tests |

---

## Changelog

- **2026-06-29** — P0 implementation: usage span helpers, package-level AST coverage, golden tests, VS Code `semanticTokenScopes`; `ast_semantic_ranges` now takes source text.
- **2026-06-29** — Initial roadmap from semantic-token engine review.
