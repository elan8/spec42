# ADR 0002: `language_service` crate for protocol-neutral editor intelligence

| Field | Value |
| --- | --- |
| **Status** | Accepted |
| **Date** | 2026-06-19 |
| **Authors** | Spec42 maintainers |

## Context

Spec42 exposes editor features (hover, go-to-definition, find references, completion, formatting, and more) primarily through `kernel::lsp_runtime`, which is coupled to `tower-lsp`, `tokio`, and LSP-specific types (`Hover`, `Location`, `Url` from `tower_lsp::lsp_types`).

Babel42 and future in-browser Monaco authoring need the same semantic navigation behavior without importing the LSP stack. `semantic_core` already provides graph construction, resolution, and diagnostics, but it does not expose a stable, editor-oriented API for navigation requests at a logical document path and text position.

Phase 1 of the language-service extraction targets **navigation only** (hover, go-to-definition, references) as the highest-value, lowest-risk slice.

## Decision

Introduce a new workspace crate, `crates/language_service`, that:

1. Depends on **`semantic_core` only** (plus `sysml-v2-parser`, `serde`, `url`) — no `kernel`, `tower-lsp`, or `tokio`.
2. Exposes **protocol-neutral DTOs** using `semantic_core::{TextPosition, TextRange}`:
   - `SourceLocation`, `HoverResult`, `DefinitionResult`, `ReferencesResult`
3. Provides **`InMemoryWorkspace`** for headless and test use, built from `SysmlDocument` / `SysmlDocumentProvider` inputs.
4. Defines a **`WorkspaceSnapshot`** trait so hosts (`kernel::ServerState`, in-memory workspaces) expose document text, semantic graph, and symbol table without duplicating query logic.
5. Implements navigation entry points: `hover`, `goto_definition`, `find_references`.

`kernel` remains the LSP adapter: `lsp_runtime/features/navigation_requests.rs` delegates to `language_service` and maps neutral DTOs to LSP types via `kernel::common::text_span`.

### Layering

```text
semantic_core     — graph, resolution, diagnostics, providers
language_service  — editor intelligence (navigation phase 1)
kernel            — LSP/runtime adapters, document lifecycle, protocol mapping
```

### Dependency rule (enforced by test)

`language_service` must not depend on `kernel`, `tower-lsp`, or `tokio`.

## Consequences

### Positive

- Babel42 can add `language_service = { path = "../spec42/crates/language_service" }` and expose JSON/HTTP endpoints without the LSP stack.
- Headless integration tests can validate navigation without spawning an LSP subprocess.
- Neutral DTOs are serde-friendly and suitable as a stable JSON contract for future APIs.

### Trade-offs

- Some logic is duplicated at the boundary (symbol table conversion in `kernel::workspace::snapshot`) until more features move out of `kernel`.
- `InMemoryWorkspace` rebuilds the full workspace; incremental `didChange` updates remain in `kernel` for now.

## Non-goals (this ADR / phase 1)

- Babel42 HTTP endpoints or Monaco providers
- Completion, rename, formatting, code actions, folding, document symbols (later phases)
- WASM packaging
- Replacing `semantic_core` responsibilities

## Follow-ups

| Feature | Suggested phase |
| --- | --- |
| Completion | Phase 2 — extract from `lsp_runtime/features/completion.rs` |
| Document/workspace symbols, folding | Phase 3 |
| Rename, formatting, code actions | Phase 4 |
| Incremental workspace API for edit sessions | When Babel42 perf requires it |
