# elkrs layout migration

Issue: [#83](https://github.com/elan8/spec42/issues/83)

Implementation follow-ups:

- [#118](https://github.com/elan8/spec42/issues/118) — native server adapter and shadow rollout
- [#119](https://github.com/elan8/spec42/issues/119) — versioned server-owned webview relayout

## Current decision

Proceed with the native-server integration behind an explicit rollout switch. The shared adapter is
now owned by `crates/diagram_layout`, and `server/native-layout-shadow` compiles a comparison seam
that returns the unchanged ELK.js result as primary plus the normalized native result. It is not yet
wired into the headless renderer or webview, so production behavior remains unchanged.

### Rollback switch

Two mechanisms, for two different questions:

- **Compile-time** (`elk-layout-spike`, `native-layout-shadow` Cargo features on `crates/server`,
  both opt-in): whether `elkrs`/`diagram_layout` are linked into the shipped binary at all. This is
  the #118 rollback switch. The default build (`default = ["embed-stdlib",
  "embed-kpar-libraries"]`) never links `elkrs` — confirmed via `cargo tree -p server -i elkrs`
  reporting "not a dependency" with no features enabled.
- **Runtime** (a future `SPEC42_LAYOUT_ENGINE` env var, `native`/`legacy`, matching the existing
  `SPEC42_*` convention in `crates/server/src/environment.rs`): only becomes necessary once #119's
  `spec42/layout` request ships enabled by default — at that point a compile-time-only switch can't
  support incident response without a new release. Not implemented yet; #118 has nothing that reads
  it, so adding the plumbing now would be dead code. #119 should add it alongside the request
  handler it actually gates.

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
cargo run -p elkrs_parity -- --iterations 3 --fail-on-difference
```

All 1,020 compared geometry scalars across eleven renderer-owned fixtures were exact at tolerance
`1e-9` after the compatibility adapter. Both engines returned deterministic output on all three
runs, with complete node, port, label, and edge identities and routed sections for every edge.
Coverage includes flat and package-hierarchical General View, two nested
Interconnection View fixtures, a 45,086-byte repository-model interconnection fixture, fixed and
external ports, port and edge labels, cross-hierarchy routing, action flow, state transitions, and
wide sibling chunking. Both horizontal and vertical action/state option variants are included. The
General View, behavior, and interconnection golden inputs are checked against their production
TypeScript builders, so layout-option and sizing drift fails the renderer suite.

Raw elkrs exposed one consistent difference. Intra-container edges were moved from the root to their
lowest common ancestor and their section coordinates were container-relative. The adapter restores
input edge order, moves the edges back to the root, and translates sections, bend points, junction
points, and edge labels to root coordinates. With that normalization, no node, container, port,
label, or edge-section differences remain in the current corpus.

Selected median timings from the three-run development-profile pass (initialization is included):

| Fixture | ELK.js/QuickJS | elkrs + adapter | Ratio |
|---|---:|---:|---:|
| two-part interconnection | 1,133 ms | 2.5 ms | 453x |
| repository-model interconnection | 6,586 ms | 34.5 ms | 191x |
| flat General View | 3,281 ms | 6.1 ms | 538x |
| action flow | 4,083 ms | 7.2 ms | 567x |
| state transition | 3,513 ms | 6.6 ms | 532x |
| wide sibling graph | 5,252 ms | 30.8 ms | 171x |

These numbers demonstrate migration headroom, not a release performance guarantee. The current
standalone adapter creates a fresh QuickJS runtime for every call, as does the current server test
path; production integration should benchmark both cold startup and a reused service.

### #118 update: expanded corpus, release-profile process benchmarks

The parity corpus grew from the 11 fixtures above to 30: `tools/elkrs_parity/fixtures/corpus/`
adds the exact ELK graph JSON built from every checked-in repository diagram product and the
synthetic node-chrome stress corpus (`vscode/diagram-renderer/src/render/elk-parity-corpus-fixtures.test.ts`,
regenerate via `UPDATE_ELK_FIXTURES=1 npm test`). `crates/server/tests/integration/layout_shadow_corpus.rs`
runs the same geometry-scalar comparison in-process as a fast CI gate.

One genuine divergence surfaced: `timer_interconnection.json` has two `FIXED_ORDER`, multi-port,
`CENTER`-aligned interconnection nodes where elkrs and ELK.js compute different port-driven node
heights, shifting downstream edge y-coordinates by a constant ~130px. This is a real
elkrs-vs-ELK.js algorithm difference in multi-port sizing, not a normalization bug (the
root-relative coordinate math was hand-verified correct). Tracked in `KNOWN_DIVERGENCES` in
`layout_shadow_corpus.rs`, not silently passed.

Also found and fixed: elkrs does not error on a root graph that authors two edges with the same
id — it silently renames the second one (e.g. `"duplicate"` -> `"duplicate_"`), which would have
returned Spec42 an edge under an id it never authored. `crates/diagram_layout` now rejects this
before calling elkrs (`reject_duplicate_input_edge_ids`).

`tools/elkrs_parity --process-mode cold|warm` (added for #118) re-execs the binary as a child
process per engine against a single fixture, isolating per-engine startup time and peak resident
memory (`/proc/self/status`'s `VmHWM`) that the in-process comparison above cannot attribute
per-engine. Release-profile numbers on `timer_interconnection.json` (18KB, the largest corpus
fixture):

| | ELK.js/QuickJS | elkrs + adapter | Ratio |
|---|---:|---:|---:|
| cold layout (median of 3) | 992 ms | 1.2 ms | ~830x |
| peak resident memory | 26.6 MiB | 6.9 MiB | 3.9x less |

Incremental shipped-binary size was **not** measured or ratcheted: `elkrs`/`diagram_layout` are
only linked behind the opt-in `native-layout-shadow` feature, never in the default build, so there
is no shipped size delta to protect yet. This is deferred to #119, when `spec42/layout` is expected
to ship the native engine enabled by default — measure and ratchet it then, against what actually
ships, following `crates/server/tests/integration/stdlib_bundle_ratchet.rs`'s pattern (a Rust test
asserting an `EXPECTED_*` size constant against a pre-built release artifact, skipping gracefully
when that artifact isn't available).

## Distribution and size constraints

The server-side ELK.js worker plus API currently occupy about 1.53 MiB; the browser ELK.js bundle is
about 1.53 MiB. A standalone optimized `elkrs.exe` from the pinned revision is 4.04 MiB, but that is
not an incremental linked-size measurement and must not be used as the expected server delta.

The recovered-project provenance and rebuilt oracle corpus are documented upstream and now recorded
in `THIRD_PARTY_NOTICES.md`. Apache-2.0 is compatible with distribution, subject to preserving its
license and attribution. Production packaging must add the Apache license text and verify packaged
notices before removing or replacing ELK.js assets.

## Required before enabling native layout by default

- ~~Run release-profile cold/warm benchmarks in separate processes for startup time, layout time,
  and peak working set.~~ Done (#118): see the update above. Incremental `spec42` binary size is
  intentionally not yet measured — see that section for why, and ratchet it once #119 makes native
  layout ship-enabled.
- ~~Expand parity fixtures beyond the initial hand-picked set to cover production-shaped inputs.~~
  Done (#118): 30 fixtures, including the full repository diagram-product corpus.
- Wire the feature-gated server shadow seam into a real headless request path and capture comparison
  diagnostics on the full visual corpus. Preserve explicit ELK failure diagnostics and the current
  deterministic fallback policy; never silently accept partial layout. **Not started** — #118's
  shadow seam (`layout_shadow.rs`) deliberately stays off the `headless_renderer.rs` production SVG
  path; #119 introduces a new `spec42/layout` request instead of modifying that path.
- Keep golden SVG marker tests and the full visual corpus green before deleting any ELK.js server
  assets. Still required; no ELK.js assets have been removed.
- Prototype the versioned server request/response path and verify cancellation plus stale-result
  rejection under rapid disclosure changes. Accept it only at p95 <= 100 ms on the representative
  packaged-extension corpus; otherwise reopen the WASM option. **Not started** — this is #119.
