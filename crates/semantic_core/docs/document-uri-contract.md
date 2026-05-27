# Document URI Contract

`semantic_core` treats document URI as provenance identity for semantic nodes.

## Required invariants

- URI must be a valid absolute URI (for example `file://`, `memory://`, `surreal://`).
- URI must be stable for the logical document across a graph build session.
- URI must be unique within the loaded document set.

## Roundtrip expectations

- `NodeId` stores `{ uri, qualified_name }`.
- Hosts must ensure URI can be mapped back to their source of truth:
  - file-backed: `file://...` path
  - DB-backed: stable synthetic URI like `surreal://org/project/doc-id/path.sysml`
- `path_hint` is optional display metadata and should not be used as identity.

## Subset-load behavior

When hosts load only query-selected documents, `semantic_core` keeps strict behavior:

- missing dependency documents remain unresolved
- unresolved diagnostics are emitted by existing diagnostics flow
