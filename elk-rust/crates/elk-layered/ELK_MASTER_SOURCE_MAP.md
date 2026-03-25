## Java ELK (`C:\Git\elk-master`) → elk-rust source map (Layered)

This file is a quick index of the upstream Java implementation locations that are most relevant
to the SysML **InterconnectionView** diffs (ports/anchors, orthogonal routing lanes, and
cross-hierarchy / hierarchical port routing).

### Orthogonal routing slot assignment (lane/slot logic)

- **Java**: `plugins/org.eclipse.elk.alg.layered/.../p5edges/orthogonal/OrthogonalRoutingGenerator.java`
  - Builds hyperedge segments, dependency graph, breaks cycles, assigns routing slots.
- **Rust**: `elk-rust/crates/elk-layered/src/pipeline/orthogonal_routing_generator.rs`
  - “Minimal port” of slot assignment; ensure it is *actually wired* into the routing stage.

Related Java pieces:
- `.../intermediate/loops/routing/RoutingSlotAssigner.java`
- `.../p5edges/OrthogonalEdgeRouter.java`

### Hierarchical ports orthogonal routing (external/hierarchical port dummy handling)

- **Java**: `plugins/org.eclipse.elk.alg.layered/.../intermediate/HierarchicalPortOrthogonalEdgeRouter.java`
  - Restores temporary dummies, computes dummy coords, routes ext-port edges, fixes coords, corrects slants.
- **Rust**: `elk-rust/crates/elk-layered/src/pipeline/hierarchical_ports.rs`
  - Invoked from `elk-rust/crates/elk-layered/src/pipeline/intermediate.rs`.

### Compound / cross-hierarchy edge restore (postprocessing split edges)

- **Java**: `plugins/org.eclipse.elk.alg.layered/.../compound/CompoundGraphPostprocessor.java`
  - Rebuilds original cross-hierarchy edges, transforms bend points to a reference graph, removes dummy edges.
- **Rust**: `elk-rust/crates/elk-layered/src/pipeline/compound.rs`
  - `postprocess_cross_hierarchy_edges(...)` is the intended equivalent; ensure it restores + removes
    temporary segment edges so exported graphs match Java’s edge set.

