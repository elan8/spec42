# Parity Matrix (ELK Java → elk-rust)

This matrix is scoped to **headless kernel parity** (graph model + algorithms + options + I/O), not Eclipse UI/GMF/IDE integration.

## Java module coverage (by plugin)


| ELK Java plugin / area                                   | Rust equivalent            | Status      | Notes                                                                                                                                                                |
| -------------------------------------------------------- | -------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org.eclipse.elk.graph` (graph model)                    | `elk-graph`                | **Implemented (practical)** | ELK-like model exists (nodes/ports/edges/labels/edge sections/hierarchy/geometry) + lossless property bag (scalars + null/arrays/objects) + validation helpers. |
| `org.eclipse.elk.core` (core infra/options)              | `elk-core`                 | **Implemented (baseline)** | Core option preflight contract exists (normalize + validate + warnings), with Rust-native facade types; concrete registry behavior is supplied through `elk-meta` and `elk-service`. |
| `org.eclipse.elk.core.service` (layout service registry) | `elk-service`              | **Implemented (baseline)** | Algorithm registry + dispatcher for `ElkGraph`, keyed by explicit `elk.algorithm` on the root (currently registers layered).                                        |
| `org.eclipse.elk.core.meta` (options meta-model)         | `elk-meta`                 | **Implemented (baseline)** | Option registry with aliasing + basic `PropertyBag` validation helpers (core + layered option keys/aliases).                                                         |
| `org.eclipse.elk.graph.json` (ELK JSON I/O)              | `elk-graph-json`           | **Implemented (baseline)** | Imports ELK Graph JSON into `elk-graph::ElkGraph`; exports modern ELK JSON from `ElkGraph` (IDs are synthesized). Fixture-based round-trip tests exist.              |
| `org.eclipse.elk.graph.text` (ELKT / text I/O)           | (none)                     | **Missing** | Useful for some ELK tooling, but lower priority than JSON.                                                                                                           |
| `org.eclipse.elk.alg.common` (shared alg utilities)      | `elk-alg-common`           | **Implemented (baseline)** | Shared utilities crate (components/packing, geometry, meta-aware property helpers, hierarchy helpers) used by `elk-layered`.                                         |
| `org.eclipse.elk.alg.layered` (layered)                  | `elk-layered`              | **Partial** | End-to-end pipeline exists with slot-based lane refinement wired into routing and hierarchy-aware boundary anchoring in export; ELK Layered still has broader processor/option parity gaps.                     |
| `org.eclipse.elk.alg.force` (force)                      | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.radial` (radial)                    | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.mrtree` (tree)                      | `elk-tree`                 | **Implemented (smoke)** | Basic rooted tree layout on `ElkGraph` + minimal routing; dispatchable via `elk-service` using `elk.algorithm=org.eclipse.elk.mrtree`.                                |
| `org.eclipse.elk.alg.rectpacking` (packing)              | `elk-rectpacking`          | **Implemented (baseline)** | Deterministic shelf packing of top-level nodes; non-overlap + root bounds; dispatch via `elk.algorithm=org.eclipse.elk.rectpacking`.                                  |
| `org.eclipse.elk.alg.spore`                              | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.topdownpacking`                     | `elk-topdownpacking`       | **Implemented (baseline)** | Hierarchy-aware packing (post-order size, pre-order placement); compounds pack children; dispatch via `elk.algorithm=org.eclipse.elk.topdownpacking`.               |
| `org.eclipse.elk.alg.vertiflex`                          | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.libavoid` (routing)                 | `elk-libavoid`             | **Implemented (baseline)** | Native Rust obstacle-avoiding router (visibility graph + A*); routes edges only; dispatch via `elk.algorithm=org.eclipse.elk.libavoid`.                              |


## Functional capability coverage (headless)


| Capability                            | Status                       | Notes / pointers                                                                                                                      |
| ------------------------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| **Typed graph model**                 | **Implemented**              | `elk_graph::ElkGraph` supports nodes/ports/edges/labels/edge sections/hierarchy/geometry + `PropertyBag` (including arrays/objects/null). |
| **Layered layout pipeline**           | **Implemented (baseline)**   | Pipeline phases exist in `elk-layered` (import, cycle breaking, layering, ordering/crossing minimization, placement, routing/export). |
| **Directed graphs**                   | **Implemented**              | Deterministic ordering is used throughout for stability.                                                                              |
| **Port sides + basic constraints**    | **Implemented**              | `PortSide`, `PortConstraint`, and per-element overrides exist.                                                                        |
| **Connected component packing**       | **Implemented**              | Simple row packing heuristic exists (mirrors ELK’s `SimpleRowGraphPlacer` intent).                                                    |
| **Compound nodes (hierarchy)**        | **Partial**                  | Recursive child layout exists; cross-hierarchy handling exists but is not yet validated against ELK’s full behavior set.              |
| **Edge routing**                      | **Partial**                  | Orthogonal routing includes lane slot refinement + boundary-anchor shaping and optional unnecessary-bendpoint retention; ELK still has additional hierarchy processors and route-chain semantics not yet ported.                   |
| **Edge labels**                       | **Partial**                  | Labels exist and are placed for edges in `elk-layered`, but parity vs ELK’s label strategies is not yet proven.                       |
| **JSON I/O (ELK Graph JSON)**         | **Implemented (baseline)**   | `elk-graph-json` supports import; export exists for `ElkGraph` and round-trip is tested on fixtures (including rich properties/hyperedges). |
| **Option meta-model + compatibility** | **Implemented (baseline)**   | `elk-meta` provides alias/deprecated-key normalization and validation (unknown key, wrong type, disallowed scope); defaults/constraints can be expanded incrementally toward ELK parity. |
| **Additional algorithms**             | **Partial**                  | `elk-layered`, `elk-tree` (mrtree), `elk-rectpacking`, `elk-topdownpacking`, `elk-libavoid` (routing-only) are present; others pending. |


## Rough completeness (pragmatic)

- **Algorithm set coverage**: ~**50%** (layered, tree, rectpacking, topdownpacking, libavoid vs ~12 headless `org.eclipse.elk.alg.*` plugins).
- **Headless parity readiness**: ~**40–50%** (core model, JSON I/O, meta, service, and several algorithms in place).

