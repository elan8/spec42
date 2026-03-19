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

## Current Baseline

- No critical topology regressions in the current corpus gate.
- All fixtures route with high edge-section coverage.
- Shared-edge bend complexity between Java and Rust is now checked with a bounded delta and currently passes.

## Remaining Risk Area

- Dense visual similarity (exact lane geometry and connector aesthetics) is still partial; current gate is metric-based, not pixel-identical.

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
