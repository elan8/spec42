# elkrs layout migration

Issue: [#83](https://github.com/elan8/spec42/issues/83)

Implementation follow-ups:

- [#118](https://github.com/elan8/spec42/issues/118) — native server adapter and shadow rollout
- [#119](https://github.com/elan8/spec42/issues/119) — versioned server-owned webview relayout

## Current decision

Proceed to a separately reviewed native-server integration behind an explicit rollout switch. Do
not replace the production ELK.js/QuickJS or webview paths in the parity change.

The server integration must own one neutral ELK JSON adapter and preserve the prepared-view/diagram
product as its input boundary. The adapter must retain the root-authored, root-coordinate edge
normalization proven by `tools/elkrs_parity`; calling `elkrs::layout_json` directly is not compatible
with the current Spec42 contract for every hierarchical graph.

For the first webview migration, request layout from the Rust server rather than shipping elkrs as a
second WASM layout runtime. The native timings below leave ample room for local IPC, this keeps one
layout implementation and one option adapter, and it avoids adding a WASM package/build/update path.
Disclosure requests must carry the immutable diagram-product identity plus a presentation-state
revision. Cancellation is advisory; the client commits a response only when both identities still
match. The TypeScript/D3 drawing and interaction layer remains in the webview.

Reconsider WASM only if an interactive server-layout prototype misses a 100 ms p95 disclosure
relayout budget on a representative packaged extension. `elkrs` itself compiles for
`wasm32-unknown-unknown`, so this remains a viable fallback rather than a prerequisite.

## Parity evidence

Pinned dependency:

- public repository: `https://github.com/elan8/elkrs`
- revision: `8309be8cf614cfe277c572b28e4f79a1703f8e32`
- compatibility baseline: ELK 0.11.0
- license: Apache-2.0

Run on 2026-09-03 on Windows x86-64 in the Cargo development profile:

```sh
cargo run -p elkrs_parity -- --iterations 1 --fail-on-difference --format json
```

All 1,064 compared geometry scalars across eleven fixtures were exact at tolerance `1e-9` after the
compatibility adapter. Coverage includes flat and package-hierarchical General View, two nested
Interconnection View fixtures, a 45,086-byte repository-model interconnection fixture, fixed and
external ports, port and edge labels, cross-hierarchy routing, action flow, state transitions, and
wide sibling chunking. Both horizontal and vertical action/state option variants are included.

Raw elkrs exposed one consistent difference. Intra-container edges were moved from the root to their
lowest common ancestor and their section coordinates were container-relative. The adapter restores
input edge order, moves the edges back to the root, and translates sections, bend points, junction
points, and edge labels to root coordinates. With that normalization, no node, container, port,
label, or edge-section differences remain in the current corpus.

Selected single-run timings (debug builds; initialization is included):

| Fixture | ELK.js/QuickJS | elkrs + adapter | Ratio |
|---|---:|---:|---:|
| two-part interconnection | 1,129 ms | 3.0 ms | 376x |
| repository-model interconnection | 5,360 ms | 33.1 ms | 162x |
| flat General View | 3,413 ms | 4.3 ms | 790x |
| action flow | 3,406 ms | 5.3 ms | 646x |
| state transition | 3,505 ms | 8.3 ms | 421x |
| wide sibling graph | 10,551 ms | 44.7 ms | 236x |

These numbers demonstrate migration headroom, not a release performance guarantee. The current
standalone adapter creates a fresh QuickJS runtime for every call, as does the current server test
path; production integration should benchmark both cold startup and a reused service.

## Distribution and size constraints

The server-side ELK.js worker plus API currently occupy about 1.53 MiB; the browser ELK.js bundle is
about 1.53 MiB. A standalone optimized `elkrs.exe` from the pinned revision is 4.04 MiB, but that is
not an incremental linked-size measurement and must not be used as the expected server delta.

The recovered-project provenance and rebuilt oracle corpus are documented upstream and now recorded
in `THIRD_PARTY_NOTICES.md`. Apache-2.0 is compatible with distribution, subject to preserving its
license and attribution. Production packaging must add the Apache license text and verify packaged
notices before removing or replacing ELK.js assets.

## Required before enabling native layout by default

- Move the compatibility adapter from the spike tool to a small owning Rust layout crate with typed
  errors and tests for malformed input, duplicate edge ids, and nested authored edges.
- Add a shadow-mode server integration and compare its normalized output with ELK.js on the full
  visual corpus. Preserve explicit ELK failure diagnostics and the current deterministic fallback
  policy; never silently accept partial layout.
- Run release-profile cold/warm benchmarks in separate processes for startup time, layout time, peak
  working set, and incremental `spec42` binary size. The in-process spike intentionally does not
  claim engine-attributed peak memory because both engines are linked and loaded together.
- Make the hand-authored General View and behavior fixtures renderer-generated so option drift is
  detected at their owning TypeScript builders rather than only during review.
- Keep golden SVG marker tests and the full visual corpus green before deleting any ELK.js server
  assets.
- Prototype the versioned server request/response path and verify cancellation plus stale-result
  rejection under rapid disclosure changes. Accept it only at p95 <= 100 ms on the representative
  packaged-extension corpus; otherwise reopen the WASM option.
