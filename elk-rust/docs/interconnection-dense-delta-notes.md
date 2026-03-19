# Interconnection Dense Delta Notes

This note captures the current Java-vs-Rust re-baseline for dense SysML Interconnection fixtures.

## Corpus

- `interconnection_real_small`
- `interconnection_real_medium`
- `interconnection_real_dense`

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
