# Interconnection Dense Delta Notes

This note captures the current Java-vs-Rust re-baseline for dense SysML Interconnection fixtures.

## Corpus

- `interconnection_real_small`
- `interconnection_real_medium`
- `interconnection_real_dense`
- `interconnection_real_full_drone_like`

## Delta Classes Tracked

- Boundary anchor drift on cross-hierarchy connectors
- Lane collapse / unstable slot ordering under dense fanout
- Bend simplification differences
- Option alias / precedence mismatches for Interconnection-critical keys

## Fixture Coverage Mapping

| Fixture | Primary delta classes | Current status |
| --- | --- | --- |
| `interconnection_real_small` | boundary/frame mismatch, option aliasing | stable |
| `interconnection_real_medium` | channel selection mismatch, lane stability | stable |
| `interconnection_real_dense` | channel graph deficiency, lane collapse risk | stable with metric gate |
| `interconnection_real_full_drone_like` | scope selection mismatch, fallback-path mismatch | stable with fallback guardrails |
| `libavoid_obstacles` | direct libavoid obstacle/channel behavior | stable |
| `libavoid_narrow` | narrow-corridor channel selection | stable |

## Current Baseline

- No critical topology regressions in the current corpus gate.
- All fixtures route with high edge-section coverage.
- Shared-edge bend complexity between Java and Rust is now checked with a bounded delta and currently passes.

## Remaining Risk Area

- Dense visual similarity (exact lane geometry and connector aesthetics) is still partial; current gate is metric-based, not pixel-identical.

## Root-Cause Buckets (Debug Mapping)

The active debug gate for `interconnection_real_full_drone_like` classifies bad edges into concrete
root-cause buckets to avoid patchwork tuning:

- **FrameMismatch** (`layered/routing` + `libavoid/lib`):
  - Symptoms: endpoint out-of-bounds, nearest-port drift spikes, endpoint canonicalization deltas.
  - Java parity anchor: `OrthogonalEdgeRouter` endpoint coordinate usage must stay consistent per scope.
- **ScopeSelectionMismatch** (`layered/routing` + `pipeline/compound`):
  - Symptoms: edge appears at wrong recursion level, `no_local_edges` / deferred cross-hierarchy routing.
  - Java parity anchor: `HierarchicalPortOrthogonalEdgeRouter` split/restore flow by hierarchy level.
- **DummyRestorationMismatch** (`pipeline/compound`):
  - Symptoms: source/target identity not preserved after hierarchical dummy replacement.
  - Java parity anchor: post-route correction in `HierarchicalPortOrthogonalEdgeRouter`.
- **ChannelGraphDeficiency** (`libavoid/router`):
  - Symptoms: straight-line fallback-like routes through crowded regions despite available channels.
  - Java parity anchor: route generation behavior in `OrthogonalRoutingGenerator`.

## Java Port Candidates (Routing Focus)

Gate prerequisite: keep `interconnection_real_full_drone_like` green on invariants, bend budgets, and deterministic route signature before/after each port.

- `org.eclipse.elk.alg.layered.p5edges.OrthogonalEdgeRouter`
  - Primary orthogonal path construction and segment shaping logic for layered.
- `org.eclipse.elk.alg.layered.intermediate.HierarchicalPortOrthogonalEdgeRouter`
  - Cross-hierarchy boundary routing for nested compounds and port anchoring.
- `org.eclipse.elk.alg.layered.intermediate.OrthogonalRoutingGenerator`
  - Additional route-chain generation behavior used by layered orthogonal stages.
- `org.eclipse.elk.alg.layered.intermediate.HyperedgeDummyMerger` (if needed for bundled fanout behavior)
  - Candidate for dense fanout readability and bend-complexity improvements.
