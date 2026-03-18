# Parity Matrix (ELK Java → elk-rust)

This matrix is scoped to **headless kernel parity** (graph model + algorithms + options + I/O), not Eclipse UI/GMF/IDE integration.

## Java module coverage (by plugin)


| ELK Java plugin / area                                   | Rust equivalent            | Status      | Notes                                                                                                                                                                |
| -------------------------------------------------------- | -------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org.eclipse.elk.graph` (graph model)                    | `elk-graph`                | **Implemented (practical)** | ELK-like model exists (nodes/ports/edges/labels/edge sections/hierarchy/geometry) + lossless property bag (scalars + null/arrays/objects) + validation helpers. |
| `org.eclipse.elk.core` (core infra/options)              | `elk-core`                 | **Partial** | Typed options exist, but not ELK’s typed option registry/meta-model (`core.meta`) or service layer.                                                                  |
| `org.eclipse.elk.core.service` (layout service registry) | (none)                     | **Missing** | No algorithm registry/dispatch layer; engines are invoked directly (e.g. `LayeredLayoutEngine`).                                                                     |
| `org.eclipse.elk.core.meta` (options meta-model)         | (none)                     | **Missing** | No compatibility layer for ELK’s option IDs, defaults, or metadata.                                                                                                  |
| `org.eclipse.elk.graph.json` (ELK JSON I/O)              | `elk-graph-json`           | **Implemented (baseline)** | Imports ELK Graph JSON into `elk-graph::ElkGraph`; exports modern ELK JSON from `ElkGraph` (IDs are synthesized). Fixture-based round-trip tests exist.              |
| `org.eclipse.elk.graph.text` (ELKT / text I/O)           | (none)                     | **Missing** | Useful for some ELK tooling, but lower priority than JSON.                                                                                                           |
| `org.eclipse.elk.alg.common` (shared alg utilities)      | `elk-layered` (ad-hoc)     | **Partial** | Some utilities exist but not as a reusable shared crate yet.                                                                                                         |
| `org.eclipse.elk.alg.layered` (layered)                  | `elk-layered`              | **Partial** | End-to-end pipeline exists, but ELK Layered has a large option/processor surface and many specialized routing/placement variants not yet ported.                     |
| `org.eclipse.elk.alg.force` (force)                      | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.radial` (radial)                    | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.mrtree` (tree)                      | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.rectpacking` (packing)              | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.spore`                              | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.topdownpacking`                     | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.vertiflex`                          | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |
| `org.eclipse.elk.alg.libavoid` (routing)                 | (none)                     | **Missing** | Not yet ported.                                                                                                                                                      |


## Functional capability coverage (headless)


| Capability                            | Status                       | Notes / pointers                                                                                                                      |
| ------------------------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| **Typed graph model**                 | **Implemented**              | `elk_graph::ElkGraph` supports nodes/ports/edges/labels/edge sections/hierarchy/geometry + `PropertyBag` (including arrays/objects/null). |
| **Layered layout pipeline**           | **Implemented (baseline)**   | Pipeline phases exist in `elk-layered` (import, cycle breaking, layering, ordering/crossing minimization, placement, routing/export). |
| **Directed graphs**                   | **Implemented**              | Deterministic ordering is used throughout for stability.                                                                              |
| **Port sides + basic constraints**    | **Implemented**              | `PortSide`, `PortConstraint`, and per-element overrides exist.                                                                        |
| **Connected component packing**       | **Implemented**              | Simple row packing heuristic exists (mirrors ELK’s `SimpleRowGraphPlacer` intent).                                                    |
| **Compound nodes (hierarchy)**        | **Partial**                  | Recursive child layout exists; cross-hierarchy handling exists but is not yet validated against ELK’s full behavior set.              |
| **Edge routing**                      | **Partial**                  | Orthogonal routing exists with several stability/avoidance heuristics; ELK has many more routing/labeling variants.                   |
| **Edge labels**                       | **Partial**                  | Labels exist and are placed for edges in `elk-layered`, but parity vs ELK’s label strategies is not yet proven.                       |
| **JSON I/O (ELK Graph JSON)**         | **Implemented (baseline)**   | `elk-graph-json` supports import; export exists for `ElkGraph` and round-trip is tested on fixtures (including rich properties/hyperedges). |
| **Option meta-model + compatibility** | **Missing**                  | Needed to accept ELK option IDs/properties from JSON/text models and match defaults.                                                  |
| **Additional algorithms**             | **Missing**                  | Only Layered is present today.                                                                                                        |


## Rough completeness (pragmatic)

- **Algorithm set coverage**: ~**8%** (Layered only vs ~12 headless `org.eclipse.elk.alg.`* plugins).\n+- **Headless parity readiness**: ~**15–25%** (Layered exists, but missing JSON I/O + option meta/services + other algorithms blocks end-to-end parity workflows).

