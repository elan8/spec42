# SysML Parser Update Requirements

## Context

Spec42 indexing currently fails for the managed standard library (`SysML-v2-Release`, pinned `2026-02`).
Server logs show widespread parse failures at line `0:0` on library files, including files that start with:

- `standard library package SysML {`

Because parsing fails, symbol extraction and library search return no usable results.

## Goal

Update `sysml-parser` so current SysML v2 standard library sources parse successfully (or at least recover with usable AST for symbol extraction), without using frontend/backend fallback heuristics.

## What To Update In `sysml-parser`

## 1) Grammar support for standard-library package headers

Add/verify grammar support for package declarations that begin with:

- `standard library package <Name> { ... }`

The parser should treat this as a valid package construct (with an optional `standard library` modifier).

Expected outcome:

- No `0:0 expected keyword or token` on files beginning with this construct.

## 2) Compatibility with current SysML v2 library syntax

Validate and, where needed, update grammar rules for constructs heavily used in:

- `sysml.library/Systems Library/*.sysml`
- `sysml.library/Kernel Libraries/**/*.kerml`
- `sysml.library/Domain Libraries/**/*.sysml`

Priority is parse correctness for declarations needed by indexing/search:

- package and nested package declarations
- `part def`, `metadata def`, `port def`, `attribute def`, `action def` (and related definition forms)
- imports (`public import`, `private import`)
- basic specialization forms (`specializes`, `:`, etc.) used in definitions

## 3) Error recovery and diagnostics quality

Ensure parser recovery does not fail immediately at file start for valid-but-new syntax.

Improve diagnostics to point at first truly unrecognized token/production when grammar mismatch occurs.

Expected outcome:

- For unsupported syntax, diagnostics are local and informative.
- For supported syntax, parser yields an AST usable by Spec42 semantic/indexing stages.

## 4) AST shape stability

Keep AST node shapes stable where possible, or document breaking changes clearly.
Spec42 relies on parser output for semantic graph and symbol table generation.

If AST changes are required:

- provide migration notes for downstream consumers
- include before/after examples for affected node variants

## Test Plan For `sysml-parser`

## A) Add fixture-based parse tests from official library

Use real files from `Systems-Modeling/SysML-v2-Release` (`2026-02`) as fixtures.
Minimum fixture set:

- `sysml.library/Systems Library/SysML.sysml`
- 2-3 files from `Kernel Libraries/...`
- 2-3 files from `Domain Libraries/...`

Assertions:

- parse returns AST (non-None / success path)
- no top-level fatal diagnostic at line 0 for valid files

## B) Add targeted grammar tests

Unit tests for:

- `standard library package` header
- imports in package body
- representative `* def` declarations with specialization

## C) Regression tests

Run existing parser suite to ensure legacy syntax remains supported.

## Integration Validation In Spec42

After updating `sysml-parser` dependency in Spec42:

1. Rebuild server.
2. Start extension with managed stdlib installed.
3. Confirm logs show:
   - no mass parse failures for stdlib files
   - library scan loads files with successful parse
4. Confirm `librarySearch` reports `library_symbols > 0`.
5. Confirm Library UI shows default tree data and search results.

## Dependency Update Steps (Spec42 side)

In `spec42-core/Cargo.toml`, update `sysml-parser` git reference to a commit/tag containing the fixes.

Then run:

- build/test for `spec42-core`
- targeted integration tests for library search/indexing
- extension smoke test in VS Code

## Acceptance Criteria

- Standard library files from pinned release parse successfully in parser tests.
- Spec42 startup no longer emits broad `0:0 expected keyword or token` for stdlib.
- `sysml/librarySearch` returns non-empty results for known library symbols.
- No fallback parsing/search logic required in Spec42.
