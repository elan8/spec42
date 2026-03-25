# SysML Language Server Roadmap

A roadmap for evolving the sysml-language-server into a full-fledged professional SysML v2 language server. Based on the current implementation, the [SysML v2 specification](https://www.omg.org/spec/SysML/2.0/Language/), and LSP best practices.

---

## Current State

**Implemented LSP features:** text sync, diagnostics, hover, completion, signature help, go-to-definition, find references, rename, document symbols, workspace symbol search, code actions, code lens, formatting, semantic tokens, folding ranges, document highlights, selection range, document links, inlay hints, linked editing, moniker, type hierarchy, call hierarchy.

**Custom methods:** `sysml/model`, `sysml/serverStats`, `sysml/clearCache`.

**Stable workflows (release-gating for `1.0`):**
- Core editing loop in VS Code for `.sysml` / `.kerml` without server crashes in normal use
- Diagnostics while typing invalid intermediate text
- Hover, go-to-definition, references, rename, document symbols, workspace symbol search
- Formatting, semantic tokens, and folding ranges

**Usable with caveats:**
- Workspace indexing in larger repositories may require tuning `spec42.workspace.maxFilesPerPattern`
- Library path indexing is useful but depends on parser coverage
- Model Explorer and visualization quality depend on parser/model completeness and partial indexing
- `general-view` and `interconnection-view` are release-enabled; dense routing/layout can still be rough

**Current support boundaries:**
- Parser coverage is aligned with the current SysML v2 Release-focused subset
- The project does not claim full OMG SysML v2 specification coverage yet
- Full semantic validation for all constructs is not complete
- Non-core visualization views remain experimental

**Validation/testing status:**
- Full SysML v2 validation suite exists, but is `#[ignore]`d in standard `cargo test` due to runtime cost
- CI deterministically covers release-critical editor workflows

---

## LSP Features

| ID | Item | Priority | Status |
|----|------|----------|--------|
| ls-1 | **Signature help** — Parameter hints for `part def X : Type`, `port def P { in x : Real }`, etc. | High | Done |
| ls-2 | **Inlay hints** — Inferred types, parameter names at call sites | Medium | Done |
| ls-3 | **Document links** — Clickable links for imports, cross-refs | Medium | Done |
| ls-4 | **Type hierarchy** — Subtypes/supertypes for `part def` / `specializes` | Medium | Done (advertised through compatibility `experimental.typeHierarchyProvider` due current `tower-lsp` capability surface) |
| ls-5 | **Call hierarchy** — Where actions/calculations are used | Medium | Done |
| ls-6 | **Code lens** — References count, run test actions | Low | Done |
| ls-7 | **Linked editing** — Rename tag pairs together | Low | Done |
| ls-8 | **Document highlights** — Highlight same symbol under cursor | High | Done |
| ls-9 | **Selection range** — Expand selection for blocks | Medium | Done |
| ls-10 | **Moniker** — Symbol identity for LSIF/indexing | Low | Done |

---

## SysML Spec Compliance

| ID | Item | Priority | Status |
|----|------|----------|--------|
| spec-1 | **Broader parser coverage** — Requirements, states, use cases, allocations, flows, views/viewpoints | High | In progress |
| spec-2 | **Semantic validation** — Multiplicity, typing, redefines, connection semantics | High | In progress |
| spec-3 | **Full validation suite CI** — Run SysML-v2-Release validation suite in CI by default | Medium | Partial (`#[ignore]` locally by default) |
| spec-4 | **Full BNF coverage** — Phased plan in [BNF_COVERAGE_PLAN.md](BNF_COVERAGE_PLAN.md) | Medium | In progress |

---

## UX Improvements

| ID | Item | Priority | Status |
|----|------|----------|--------|
| ux-1 | **Snippets** — Common SysML patterns (BDD, IBD, requirements, actions) | High | Planned |
| ux-2 | **Breadcrumb navigation** — Package hierarchy path in editor | Medium | Planned |
| ux-3 | **Outline icons** — Icons by element kind (part, port, action, etc.) | Low | Planned |
| ux-4 | **Bracket pairs / indent guides** — Visual structure for braces | Low | Planned |

---

## Performance & Reliability

| ID | Item | Priority | Status |
|----|------|----------|--------|
| perf-1 | **Incremental parsing** — Avoid full re-parse on large workspace edits | Medium | Planned |
| perf-2 | **Progress reporting** — Workspace scan progress notifications | Medium | Planned |

## Experimental Areas

The following areas are intentionally not release-gating for `1.0` and remain experimental until their tests are promoted from pending/ignored to required:

- `action-flow-view`
- `state-transition-view`
- `sequence-view`
- Additional visualization routing/layout quality beyond `general-view` and `interconnection-view`

See also [SUPPORTED-WORKFLOWS.md](SUPPORTED-WORKFLOWS.md) for the current release-facing support boundaries.

---

## Professional Polish

| ID | Item | Priority | Status |
|----|------|----------|--------|
| pro-1 | **Telemetry** — Opt-in crash/usage reporting | Low | Planned |
| pro-2 | **Trace / debug logging** — LSP trace for troubleshooting | Medium | In progress |
| pro-3 | **Marketplace publishing** — VS Code marketplace released; Open VSX remaining | High | Partial |
| pro-4 | **User documentation** — Quick start, troubleshooting, feature guide | High | In progress |

---

## Recommended Order

1. **Close LSP editing gaps:** continue precision hardening and edge-case tests for newly completed handlers (ls-1..ls-10)
2. **Raise SysML confidence:** semantic validation (spec-2), broader parser coverage (spec-1), BNF plan execution (spec-4)
3. **Harden scale behavior:** indexing/perf work (perf-1, perf-2), plus deterministic large-workspace expectations
4. **Productize:** finish user-facing docs (pro-4), improve troubleshooting traces (pro-2), complete remaining marketplace targets (pro-3)

---

## Reference

- [SysML v2 Specification](https://www.omg.org/spec/SysML/2.0/Language/)
- [LSP Specification 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [SysML-v2-Release](https://github.com/Systems-Modeling/SysML-v2-Release)
