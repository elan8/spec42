# Spec42 roadmap

**Target:** 1.0.0

This file contains active product decisions and remaining work only. Completed implementation
history belongs in git and `CHANGELOG.md`; executable support claims belong in the conformance
matrix and tests.

## 1.0 direction

Spec42 1.0 is a local-first SysML v2 editor, validation, CI, and generation tool. One immutable
semantic publication owns facts and diagnostics for each workspace revision. Editor, CLI, and
generator consumers use typed queries over that same publication.

The release-gating surfaces are:

- formatting, navigation, hover, rename, completion, outline, and semantic highlighting;
- stable diagnostics with precise ranges and text/JSON/SARIF/JUnit reporting;
- bundled pinned libraries and reproducible local operation;
- the GitHub Action and generator host;
- generated conformance evidence checked by CI.

## Deliberately disabled surfaces

These are not compatibility promises for 1.0:

- The built-in Model Explorer and the legacy in-process diagram semantics stay removed. Diagram
  views are a repository-owned WASM generator plus the VS Code renderer over the same immutable
  publication: the General, Interconnection, Action Flow, State Transition, Sequence, Browser, and
  Grid view kinds consume typed projections; Geometry stays partial until authored geometry and 3D
  catch up. Where a projection is still incomplete the panel reports the typed reason rather than
  guessing. The `sysml/model` graph query is not restored.
- `model-summary` reports validation only. Structural nodes and relationships require a bounded
  typed query owned by the immutable publication.
- Call hierarchy and monikers require typed behavior/`perform` relationships.
- Add-import and qualify-ambiguous-name actions require typed candidates, provenance, and authored
  edit ranges.
- Semantic snapshot comparison requires stable-identity typed fact differences.
- Incremental publication is deferred until measurements justify it and full/cold parity,
  deterministic ordering, cancellation, and supersession are proved. Full rebuild is the supported
  correctness path.

## Remaining 1.0 gates

Open work is tracked in GitHub Issues:

| Item | Issue |
|---|---|
| Pin and verify the parser/conformance baseline | [#18](https://github.com/elan8/spec42/issues/18) |
| Ship 1.0.0 and the versioned GitHub Action | [#19](https://github.com/elan8/spec42/issues/19) |

## Post-1.0 candidates

The disabled surfaces above become candidates only through owner-defined typed contracts. Other
deferred language coverage remains visible in the generated conformance matrix rather than being
duplicated here.
