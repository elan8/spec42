# SysML Language Server Roadmap

A roadmap for evolving the sysml-language-server into a full-fledged professional SysML v2 language server. Based on the current implementation, the [SysML v2 specification](https://www.omg.org/spec/SysML/2.0/Language/), and LSP best practices.

---

## Current State

**Implemented LSP features:** text sync, diagnostics, hover, completion, go-to-definition, find references, rename, document symbols, workspace symbol search, code actions, formatting, semantic tokens, folding ranges.

**Custom methods:** `sysml/model`, `sysml/serverStats`, `sysml/clearCache`.

**Known limitations (from CHANGELOG):**
- Parser aligned with SysML v2 Release; does not claim full OMG spec compliance.
- Some constructs may have incomplete semantic token or outline coverage.

---

## LSP Features (Missing)

| ID | Item | Priority |
|----|------|----------|
| ls-1 | **Signature help** — Parameter hints for `part def X : Type`, `port def P { in x : Real }`, etc. | High |
| ls-2 | **Inlay hints** — Inferred types, parameter names at call sites | Medium |
| ls-3 | **Document links** — Clickable links for imports, cross-refs | Medium |
| ls-4 | **Type hierarchy** — Subtypes/supertypes for `part def` / `specializes` | Medium |
| ls-5 | **Call hierarchy** — Where actions/calculations are used | Medium |
| ls-6 | **Code lens** — References count, run test actions | Low |
| ls-7 | **Linked editing** — Rename tag pairs together | Low |
| ls-8 | **Document highlights** — Highlight same symbol under cursor | High |
| ls-9 | **Selection range** — Expand selection for blocks | Medium |
| ls-10 | **Moniker** — Symbol identity for LSIF/indexing | Low |

---

## SysML Spec Compliance

| ID | Item | Priority |
|----|------|----------|
| spec-1 | **Broader parser coverage** — Requirements, states, use cases, allocations, flows, views/viewpoints | High |
| spec-2 | **Semantic validation** — Multiplicity, typing, redefines, connection semantics | High |
| spec-3 | **Full validation suite CI** — Run SysML-v2-Release validation suite in CI | Medium |
| spec-4 | **Full BNF coverage** — Phased plan in [BNF_COVERAGE_PLAN.md](BNF_COVERAGE_PLAN.md) | Medium |

---

## UX Improvements

| ID | Item | Priority |
|----|------|----------|
| ux-1 | **Snippets** — Common SysML patterns (BDD, IBD, requirements, actions) | High |
| ux-2 | **Breadcrumb navigation** — Package hierarchy path in editor | Medium |
| ux-3 | **Outline icons** — Icons by element kind (part, port, action, etc.) | Low |
| ux-4 | **Bracket pairs / indent guides** — Visual structure for braces | Low |

---

## Performance & Reliability

| ID | Item | Priority |
|----|------|----------|
| perf-1 | **Incremental parsing** — Avoid full re-parse on large workspace edits | Medium |
| perf-2 | **Progress reporting** — Workspace scan progress notifications | Medium |

## Experimental Areas

The following areas are intentionally not release-gating for `1.0` and remain experimental until their tests are promoted from pending/ignored to required:

- `action-flow-view`
- `state-transition-view`
- `sequence-view`
- Additional visualization routing/layout quality beyond `general-view` and `interconnection-view`

See also [SUPPORTED-WORKFLOWS.md](SUPPORTED-WORKFLOWS.md) for the current release-facing support boundaries.

---

## Professional Polish

| ID | Item | Priority |
|----|------|----------|
| pro-1 | **Telemetry** — Opt-in crash/usage reporting | Low |
| pro-2 | **Trace / debug logging** — LSP trace for troubleshooting | Medium |
| pro-3 | **Marketplace publishing** — Open VSX, VS Code marketplace | High |
| pro-4 | **User documentation** — Quick start, troubleshooting, feature guide | High |

---

## Recommended Order

1. **Quick wins:** Document highlights (ls-8), selection range (ls-9)
2. **High impact:** Signature help (ls-1), snippets (ux-1), semantic validation (spec-2)
3. **Spec alignment:** Broader parser coverage (spec-1), validation suite CI (spec-3)
4. **Polish:** Documentation (pro-4), marketplace (pro-3), trace logging (pro-2)

---

## Reference

- [SysML v2 Specification](https://www.omg.org/spec/SysML/2.0/Language/)
- [LSP Specification 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [SysML-v2-Release](https://github.com/Systems-Modeling/SysML-v2-Release)
