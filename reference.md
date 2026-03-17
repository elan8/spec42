Upstream orthogonal routing (core of Phase 2)
OrthogonalEdgeRouter.process()

Upstream: C:\Git\elk-master\plugins\org.eclipse.elk.alg.layered\src\org\eclipse\elk\alg\layered\p5edges\OrthogonalEdgeRouter.java
Here (closest equivalent): c:\Git\spec42\elk-rust\crates\elk-layered\src\pipeline\routing.rs (export_to_graph is where we currently generate orthogonal paths and decide how much “routing area” we consume implicitly via lane offsets)
What to mirror: routing between consecutive layers with explicit slot counts and spacing rules, instead of global “lane offsets” that encourage long shared corridors.
OrthogonalRoutingGenerator.routeEdges(...) (hyperedge segments + slot assignment)

Upstream: ...\p5edges\orthogonal\OrthogonalRoutingGenerator.java
Here: implement new Rust module(s) under c:\Git\spec42\elk-rust\crates\elk-layered\src\pipeline\routing/ (or within routing.rs), providing:
HyperedgeSegment abstraction (per “between-layers” routing window)
Dependency graph creation (createDependencyIfNecessary) using conflict thresholds
Cycle breaking (critical vs non-critical)
Topological numbering to assign routing slots
Bend-point generation from slots
Why: this is the upstream mechanism that prevents the “everything goes through one bus corridor” look.
Conflict / cycle logic (why bus happens today)
Conflict thresholds and critical overlaps
Upstream: constants and logic in OrthogonalRoutingGenerator:
CONFLICT_THRESHOLD_FACTOR, CRITICAL_CONFLICT_THRESHOLD_FACTOR
countConflicts, breakCriticalCycles, breakNonCriticalCycles, topologicalNumbering
Here: add analogous thresholds derived from options.layered.spacing.edge_spacing and implement cycle handling around “segment ordering”.
Hierarchical / external-port style orthogonal routing (useful for compound graphs)
HierarchicalPortOrthogonalEdgeRouter
Upstream: ...\intermediate\HierarchicalPortOrthogonalEdgeRouter.java
Here: relevant when we improve cross-hierarchy routing (later milestone); current warning notes in:
c:\Git\spec42\sysml-diagrams\src\layout\elk_adapter.rs (collect_warnings)
What to mirror (later): route edges incident to “external”/boundary ports with dedicated directional routing passes and then correct slanted segments.
Layering/packing parity (Phase 3)
Interconnection-specific layer merging
Here today: c:\Git\spec42\elk-rust\crates\elk-layered\src\pipeline\layering.rs (merge_layers_for_interconnection_view)
Plan: re-evaluate after slot-based routing lands; the right merge strategy depends on whether routing can keep corridors compact without exploding width.
Where SysML turns knobs (so we don’t “tune around” missing algorithms)
View profile + options mapping
c:\Git\spec42\sysml-diagrams\src\layout\elk_adapter.rs (map_layout_options, map_direction)
Note: keep option tuning secondary until routing parity is improved.
