# Next algorithm to port (post JSON I/O)

Now that `elk-graph-json` exists, we can start using a corpus-driven parity loop:

1) import ELK Graph JSON → 2) run Rust layout → 3) export JSON/SVG snapshots → 4) compare against ELK Java output.

## Recommendation: port `org.eclipse.elk.alg.mrtree` next

**Why this one first**

- **High value / low coupling**: tree layout is broadly useful and is comparatively self-contained versus force/radial,
  and far less entangled with sophisticated routing stacks (e.g. libavoid).
- **Clear correctness envelope**: tree algorithms have strong structural invariants (parent/child ordering, non-overlap,
  monotonic layering, edge directions) that translate well to automated tests.
- **Good parity signal**: it exercises core graph features (hierarchy, ports, labels, spacing) without requiring the
  full ELK option meta-model on day one.

**What to port (minimal parity slice)**

- A single-entry `MrTreeLayoutEngine` implementing `elk_core::LayoutEngine`.
- Basic options:
  - direction
  - node spacing / layer spacing
  - padding
- Node placement for rooted trees with deterministic ordering.
- Simple edge routing (straight or orthogonal stubs) using existing `elk-core` edge section structures.

**Prereqs / enabling work**

- Extend `elk-graph-json` option parsing just enough to read a tree layout selection + key spacing options from JSON.
- Add a small fixture set (JSON) that covers:
  - single-root tree
  - forest (multiple roots)
  - deep tree
  - wide tree
  - ports/labels present

## Alternative if you want faster visible wins: `org.eclipse.elk.alg.rectpacking`

If Spec42’s near-term needs skew toward packing diagrams/components rather than tree rendering, `rectpacking` is an
excellent “first non-layered algorithm” because it can be validated with simple geometric invariants.

It also naturally complements the existing layered connected-component packing heuristic by providing a standalone,
testable packing module.

## Why not force/radial next

- They tend to be more sensitive to numeric stability, iteration counts, and pseudo-random seeds, which makes
  cross-language parity harder early on.
- They provide weaker “exact match” signals; you often compare distributions/statistics rather than exact positions.

