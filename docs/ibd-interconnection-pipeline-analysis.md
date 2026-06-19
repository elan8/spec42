# IBD and Interconnection View Pipeline Analysis

Date: 2026-06-12

## Progress (2026-06-12)

**Status:** Phases **0–5 are complete** for the VS Code / webview interconnection path (`systemContext`, `gridConnections`, drone fixtures). The production pipeline is: Rust `InterconnectionSceneDto` → extension fetch → shared prepare → ELK layout → `InterconnectionLayoutDto` → SVG. Legacy ibd-only prepare and attribute-based route fallbacks are removed.

Post–Phase 5 stabilization (same day):

- **Extension plumbing** — [modelFetcher.ts](../vscode/src/visualization/modelFetcher.ts) forwards `interconnectionScene` to the webview (Phase 5 had made the renderer scene-only, but the field was dropped on the update message).
- **Nested-port routing** — [ibd-route.ts](../shared/diagram-renderer/src/render/ibd-route.ts) restores port-center–driven offset selection (`edgeOwnerOffset`, `lcaOffset`) for ELK sections that arrive in container-local coordinates; [layout.ts](../shared/diagram-renderer/src/render/layout.ts) collects edges from the full ELK tree and stores owner/LCA offsets. Orthogonal endpoint stitching fixed for non-axis-aligned port joins.

**2026-06-19 validation:** Scoped IBD build (`ViewExposedPackages`) parity-tested against full-workspace filter; LSP interconnection responses omit `ibd` when `interconnectionScene` is present; nightly CI runs drone perf smoke with scoped IBD metrics.

### Landed

- **`InterconnectionSceneDto`** — built in Rust ([interconnection_scene.rs](../crates/semantic_core/src/semantic/interconnection_scene.rs)), attached to LSP visualization payloads.
- **Instance-centric scoping** — `normalize_ibd_to_instance_paths`, architecture-scope filtering, variant/alternative exclusion ([visualization_workspace.rs](../crates/semantic_core/src/semantic/visualization_workspace.rs)).
- **Canonical prepare path** — [interconnection-scene.ts](../shared/diagram-renderer/src/prepare/interconnection-scene.ts); [interconnection.ts](../shared/diagram-renderer/src/prepare/interconnection.ts) requires `interconnectionScene` from the language server.
- **Layout** — ELK node ID sanitization, `edgeCoords: ROOT`, `InterconnectionLayoutDto` built during layout ([interconnection-layout-dto.ts](../shared/diagram-renderer/src/render/interconnection-layout-dto.ts), [layout.ts](../shared/diagram-renderer/src/render/layout.ts), [ibd-route.ts](../shared/diagram-renderer/src/render/ibd-route.ts)). Offset candidates are confined to layout (not semantic inference): pick the offset that best aligns ELK sections with resolved port centers.
- **Debug export** — `sysml.debug.exportInterconnectionPipeline` (VS Code) and [pipeline-export.ts](../shared/diagram-renderer/src/pipeline-export.ts).
- **Fixtures** — `grid-system-context-scene.json`, `grid-connections-scene.json`, `nested-ring-minimal.json` (scene). Regenerate grid scenes: `cargo test -p semantic_core --test view_expose_powersystems_interconnection export_powersystems -- --nocapture`.
- **Phase 4 core (drawing)** — [drawing.ts](../shared/diagram-renderer/src/render/drawing.ts) reads port anchors and route points from `layout.interconnectionLayout` (no `_portAnchors` / `layoutRoutePoints` on attributes).
- **Phase 5 (frontend)** — [interconnection-legacy.ts](../shared/diagram-renderer/src/prepare/interconnection-legacy.ts) deleted; IBD root-selection heuristics removed from [normalize-payload.ts](../shared/diagram-renderer/src/prepare/normalize-payload.ts); layout/routing collapsed to canonical-only path.

### CI contract (primary gate)

| Test | Location | What it guards |
|------|----------|----------------|
| `layout.interconnection.test.ts` | `shared/diagram-renderer` | ELK input + layout DTO `containers[]` + `assessRouteQuality`; grid scenes + nested-ring |
| `ibd-route.test.ts` | `shared/diagram-renderer` | Orthogonal endpoint snap; container-offset selection for nested ports |
| `drawing.interconnection.test.ts` | `shared/diagram-renderer` | Edge paths resolve from layout DTO without attribute fallback |
| `route-quality.test.ts` | `shared/diagram-renderer` | Detached endpoints, bounds, node-boundary fallback detection |
| `modelFetcher.test.ts` | `vscode` | `interconnectionScene` forwarded from LSP result to webview update message |
| `interconnection_elk` (unit) | `semantic_core` | Rust `build_elk_graph_from_scene` structural parity vs TS ELK input goldens |
| `interconnection_elk_svg_from_scene_fixture` | `server` | CLI/API interconnection SVG from `interconnectionScene` (no ibd heuristic fallback) |
| `interconnection_elk_layout_matches_typescript_golden_when_present` | `server` | Rust ELK.js layout positions within ±2px of TS goldens (when `*-elk-layout.json` present) |
| `view_expose_powersystems_interconnection` | `semantic_core` | Semantic scoping, connector invariants, scene export (skipped if power systems repo absent) |
| `scoped_ibd_parity` | `semantic_core` | Scoped vs full-workspace IBD interconnection scene parity on `examples/drone` (CI) |
| `drone_interconnection_performance_smoke_report` | `kernel` | Nightly in-repo perf smoke: scoped IBD timing, slim LSP payload bytes |
| `lsp_interconnection_visualization_returns_slim_scene_only_payload_for_drone` | `kernel` | LSP `sysml/visualization` omits `ibd` when `interconnectionScene` is present |
| `powersystems.visualization.test.ts` | `vscode` | LSP scene + `exportInterconnectionPipeline` route summary (optional soak; requires power systems workspace) |

Local pipeline debug: `sysml.debug.exportInterconnectionPipeline` in VS Code, or `npm test` in `shared/diagram-renderer` against scene fixtures.

Regenerate ELK input goldens: `UPDATE_ELK_FIXTURES=1 npm test -- layout.interconnection` in `shared/diagram-renderer`.  
Regenerate layout position goldens: `UPDATE_LAYOUT_FIXTURES=1 npm test -- layout.interconnection` in `shared/diagram-renderer`.

### Remainder completed (2026-06-12)

- **Layout DTO containers** — `InterconnectionLayoutDto.containers[]` populated during ELK visit; [drawing.ts](../shared/diagram-renderer/src/render/drawing.ts) `drawInterconnectionContainers` reads layout only (legacy `packageContainerGroups` fallback for non-canonical tests).
- **Interconnection typing** — `InterconnectionPreparedView` / `InterconnectionPreparedNode` / `Port` / `Edge` in [types.ts](../shared/diagram-renderer/src/prepare/types.ts); [interconnection-scene.ts](../shared/diagram-renderer/src/prepare/interconnection-scene.ts) returns typed view; layout path uses `asInterconnectionPrepared`.
- **Server native SVG** — [diagrams.rs](../crates/server/src/diagrams.rs) `build_interconnection_elk_source` requires `interconnectionScene` and calls [interconnection_elk.rs](../crates/semantic_core/src/semantic/interconnection_elk.rs) `build_elk_graph_from_scene`; ELK options aligned with TS.
- **ELK input parity** — TS `buildInterconnectionElkGraphInput` + Rust `build_elk_graph_from_scene` + `*-elk-input.json` goldens.
- **Layout parity (optional gate)** — Rust `layout_elk_graph` vs TS positions on scene fixtures when `*-elk-layout.json` goldens exist.

### Out of scope / optional future

- **Diagnostics UI** — Not planned; `scene.diagnostics` and layout warnings may surface later via banner or LSP Problems only.
- **Moving production layout to Rust** — VS Code/webview keeps ELK.js; Rust ELK is for CLI SVG and parity tests via [elk_layout.rs](../crates/server/src/elk_layout.rs).

---

## Executive Summary

> **Update (2026-06-12):** The VS Code interconnection path now follows the target architecture below (canonical `InterconnectionSceneDto`, thin prepare, `InterconnectionLayoutDto` drawing). The remainder of this document records the pre-migration analysis and migration plan for context.

The IBD / Interconnection View pipeline had accumulated enough tactical fixes that it was no longer a reliable architecture. The visible symptoms in `systemContext` are connector routes that appear to run through unrelated areas, attach to surprising sides of parts, or change shape after small fixes. The deeper issue is not only ELK.js routing. It is that semantic ownership, endpoint identity, view scoping, port identity, port side selection, container hierarchy, layout construction, and SVG drawing are all partially inferred in multiple places.

The most important recommendation is to make the backend produce a canonical, typed interconnection scene and make the frontend a mostly dumb renderer. The frontend should not be responsible for resolving connector endpoint owners, guessing whether a connector target is a nested part or a container, deciding SysML endpoint identity from suffixes, or repairing ELK coordinate spaces after the fact.

The current code already contains a lot of the needed backend knowledge, but it is exposed through a DTO that is too weak and then reinterpreted in TypeScript. This is where the technical debt concentrates.

## Current Pipeline

The pipeline is roughly:

1. Rust semantic graph construction builds the workspace semantic graph.
2. Rust IBD extraction builds `IbdDataDto` in [crates/semantic_core/src/semantic/ibd.rs](../crates/semantic_core/src/semantic/ibd.rs).
3. Rust visualization workspace selection evaluates SysML views and scopes IBD payloads in [crates/semantic_core/src/semantic/visualization_workspace.rs](../crates/semantic_core/src/semantic/visualization_workspace.rs).
4. The VS Code extension fetches visualization DTOs (including `interconnectionScene`) in [vscode/src/visualization/modelFetcher.ts](../vscode/src/visualization/modelFetcher.ts).
5. The webview passes the update message through [vscode/src/visualization/dtoAdapter.ts](../vscode/src/visualization/dtoAdapter.ts) into the shared renderer.
6. For interconnection views, [shared/diagram-renderer/src/prepare/interconnection.ts](../shared/diagram-renderer/src/prepare/interconnection.ts) requires `interconnectionScene`; [normalize-payload.ts](../shared/diagram-renderer/src/prepare/normalize-payload.ts) no longer rebuilds ibd scope for this view.
7. Interconnection preparation adapts `InterconnectionSceneDto` into `PreparedNode` / `PreparedEdge` in [interconnection-scene.ts](../shared/diagram-renderer/src/prepare/interconnection-scene.ts).
8. Layout builds a compound ELK graph, infers ports and port sides, invokes ELK.js, converts coordinates, and stores route metadata in [shared/diagram-renderer/src/render/layout.ts](../shared/diagram-renderer/src/render/layout.ts).
9. Route correction snaps and offsets ELK edge sections in [shared/diagram-renderer/src/render/ibd-route.ts](../shared/diagram-renderer/src/render/ibd-route.ts).
10. Drawing emits SVG nodes, ports, containers, and paths in [shared/diagram-renderer/src/render/drawing.ts](../shared/diagram-renderer/src/render/drawing.ts).

This is too many semantic interpretation layers for one diagram type.

## Main Findings

### 1. The IBD DTO Is Not a Strong Contract

The DTO has `parts`, `ports`, `connectors`, `containerGroups`, `rootViews`, and string fields like `id`, `qualifiedName`, `sourceId`, `targetId`, `sourcePartId`, and `targetPartId`.

The problem is that these fields are not normalized around one identity model:

- Some strings use `::`, some use `.`, and several layers convert between them.
- `part.id` and `part.qualifiedName` can differ.
- `port.id` may be fully qualified or local-ish depending on where it came from.
- `connector.source` / `target`, `sourceId` / `targetId`, and `sourcePartId` / `targetPartId` overlap in meaning but are not guaranteed to be mutually consistent.
- Frontend code compensates with suffix matching, fallback matching, local name matching, and owner inference.

That means the renderer cannot know whether it is receiving facts or hints.

### 2. Frontend Preparation Re-resolves Semantic Ownership

[prepare/interconnection.ts](../shared/diagram-renderer/src/prepare/interconnection.ts) does more than adapt a DTO. It:

- synthesizes containers from backend groups;
- resolves container IDs by aliases;
- collapses outer boundaries based on selected root;
- resolves connector endpoint owner parts from endpoint strings;
- deduplicates connectors;
- decides whether backend `sourcePartId` / `targetPartId` should be trusted.

Those are semantic operations. They belong in the backend or in a shared semantic-scene builder with strong tests, not in the drawing-side renderer.

The recent `northSouthRing` issue is a good example: the backend could send a connector whose target part looked like `northSouthRing`, while the endpoint text identified `northSouthRing.ringSegmentBtoC.a`. The frontend then had to decide which one was the real owner. That should not be a frontend decision.

### 3. Port Side Inference Is Split and Fragile

Rust has `infer_port_side` in [ibd.rs](../crates/semantic_core/src/semantic/ibd.rs), and TypeScript has another side inference path in [render/layout.ts](../shared/diagram-renderer/src/render/layout.ts).

The TypeScript path also derives port usage from connector endpoints, then uses this to override target ports to `WEST` and source ports to `EAST`. This may improve left-to-right diagrams, but it is fragile because it depends on endpoint string matching. The current code now has to match:

- explicit port IDs;
- `parent.qualifiedName + "." + port.name`;
- local port names;
- owner suffixes;
- node labels.

This is a symptom of missing canonical endpoint IDs.

### 4. ELK Integration Is Doing Too Much in the Renderer

ELK graph construction is frontend-owned in [render/layout.ts](../shared/diagram-renderer/src/render/layout.ts). It builds compound nodes, ports, layout options, and edges. After ELK returns, the frontend:

- walks nested ELK nodes to compute absolute coordinates;
- stores `_portAnchors` and `_portDrawOrder` inside node attributes;
- collects edges from nested ELK containers with offsets;
- computes both `edgeOwnerOffset` and `lcaOffset`;
- chooses candidate offsets later in [ibd-route.ts](../shared/diagram-renderer/src/render/ibd-route.ts);
- snaps endpoints to ports after layout.

This creates a second pipeline after ELK: "interpret and repair ELK output." That is brittle, especially for compound graphs.

There is also Rust-side ELK support in [crates/server/src/elk_layout.rs](../crates/server/src/elk_layout.rs), but the VS Code IBD path uses frontend ELK.js. That means there are at least two possible layout hosts in the repository.

### 5. View Scoping Is Spread Across Backend and Frontend

Rust evaluates explicit SysML views and selects/scopes IBD data in [visualization_workspace.rs](../crates/semantic_core/src/semantic/visualization_workspace.rs). Then the TypeScript normalizer still selects roots and root views in [prepare/normalize-payload.ts](../shared/diagram-renderer/src/prepare/normalize-payload.ts), and interconnection preparation may collapse boundaries again.

This makes it hard to reason about a view like `systemContext`:

- Is the selected view already fully scoped by Rust?
- Is a `rootView` still selected on the frontend?
- Are package/container groups authoritative or hints?
- Are connectors supposed to include only exposed features, exposed feature subtrees, or all connectors touching exposed parts?

The SysML view semantics should be resolved once, in the backend, and the payload should say exactly what is visible.

### 6. Renderer Data Types Are Too Generic

`PreparedNode.attributes` is an open `Record<string, unknown>` carrying semantic data, layout data, and renderer-private fields such as `_portAnchors`, `_portDrawOrder`, `_isLayoutContainer`, and `_layoutDepth`.

This makes it easy to ship fast fixes but hard to know which fields are stable. It also makes tests less meaningful because they often assert rendered SVG details rather than a typed intermediate scene.

### 7. Tests Catch Presence, Not Diagram Quality

The power systems VS Code test currently confirms that the diagram has parts and connectors. That is useful, but it did not catch the screenshot failure because all connectors can exist and touch ports while still producing terrible routes.

We need semantic-scene tests and layout-quality tests:

- every connector endpoint resolves to a visible port;
- no connector targets a container when a deeper visible endpoint owner exists;
- no route point leaves the diagram content bounds by a large margin;
- target-only ports are not placed on the far side in a left-to-right layout unless explicitly modeled that way;
- long route lengths are flagged relative to Manhattan distance between endpoint ports;
- root view scoping is deterministic for explicit views.

## Recommended Target Architecture

### Backend Responsibilities

The backend should own:

- explicit SysML view evaluation;
- interconnection scene scoping;
- expansion from definitions to usage/instance paths;
- canonical IDs;
- connector endpoint resolution;
- port ownership;
- container hierarchy;
- semantic connector kind;
- optional semantic port direction/side hints.

The backend should emit a canonical `InterconnectionSceneDto`, not the current loosely related `parts` / `ports` / `connectors` arrays.

Suggested DTO shape:

```ts
interface InterconnectionSceneDto {
  schemaVersion: 1;
  view: {
    id: string;
    name: string;
    type: "InterconnectionView";
    rootIds: string[];
  };
  nodes: InterconnectionNodeDto[];
  ports: InterconnectionPortDto[];
  edges: InterconnectionEdgeDto[];
  containers: InterconnectionContainerDto[];
  diagnostics: InterconnectionSceneDiagnosticDto[];
}

interface InterconnectionNodeDto {
  id: string;              // canonical stable scene ID
  semanticId: string;      // semantic graph NodeId / qualified name
  qualifiedName: string;   // display/debug only, not matching key
  name: string;
  kind: "part" | "ref";
  typeName?: string;
  parentId?: string;       // canonical scene node/container ID
  source?: SourceRangeRef;
}

interface InterconnectionPortDto {
  id: string;              // canonical stable scene ID
  semanticId: string;
  ownerNodeId: string;     // canonical node ID
  name: string;
  typeName?: string;
  direction?: "in" | "out" | "inout" | "unknown";
  sideHint?: "west" | "east" | "north" | "south" | "auto";
}

interface InterconnectionEdgeDto {
  id: string;
  kind: "connection" | "flow" | "interface" | "binding" | "reference";
  sourcePortId: string;
  targetPortId: string;
  sourceNodeId: string;
  targetNodeId: string;
  semanticId?: string;
  label?: string;
}
```

The critical property: layout should never need to infer an edge's source or target port from strings.

### Layout Responsibilities

Choose one layout owner:

1. **Preferred near term:** keep ELK.js in the shared renderer, but feed it canonical scene nodes, ports, and edges.
2. **Preferred longer term:** move ELK graph construction and layout to the backend or a shared non-DOM layout service, then have the frontend draw an already laid-out scene.

Either way, the ELK integration should be isolated behind a typed module:

```ts
layoutInterconnectionScene(scene: InterconnectionSceneDto): InterconnectionLayoutDto
```

That module should return absolute coordinates, port anchors, and edge routes in one coordinate system. The drawing layer should not try multiple coordinate offsets.

### Frontend Responsibilities

The webview/shared renderer should own:

- drawing SVG;
- theme and interaction;
- zoom/export;
- rendering already-resolved nodes, ports, containers, and routes;
- showing diagnostics from the backend.

It should not own:

- semantic endpoint resolution;
- source/target owner inference;
- root view selection;
- definition-to-instance connector remapping;
- namespace/string matching for identity.

## Migration Plan

### Phase 0: Freeze and Instrument

Goal: stop making the debt worse while adding observability.

- Add a debug export for the prepared interconnection scene before ELK.
- Add a debug export for the ELK input graph and ELK output graph for VS Code integration tests.
- Add route quality metrics to integration tests: detached endpoints, route points outside content bounds, excessive route length, container-targeted edges without ports.
- Add `schemaVersion` to the current IBD payload.
- Document current ID meanings in the DTO types.

### Phase 1: Canonical IDs and Endpoint Resolution in Backend

Goal: remove frontend string guessing.

- Add canonical `nodeId`, `portId`, `sourcePortId`, and `targetPortId` fields to Rust IBD DTOs.
- Keep existing fields for compatibility during migration.
- Guarantee that every connector endpoint references a visible port or emits a backend diagnostic.
- Guarantee `sourcePartId` / `targetPartId` match the resolved endpoint owners.
- Move "nested endpoint owner wins over container" logic fully into Rust.
- Add Rust tests for `systemContext`-style nested ring endpoints.

### Phase 2: Replace Frontend Interconnection Preparation

Goal: turn `prepareInterconnection` into a mechanical adapter.

- Stop resolving endpoint owners in TypeScript.
- Stop alias-based container ID resolution in TypeScript.
- Stop connector deduping in TypeScript.
- Stop frontend root-view selection when `selectedView` is explicit.
- Define typed `InterconnectionPreparedNode`, `InterconnectionPreparedPort`, and `InterconnectionPreparedEdge` instead of open `attributes`.

### Phase 3: Isolate ELK Integration — **done (VS Code path)**

Goal: make layout deterministic and testable.

- Dedicated layout contract: `InterconnectionLayoutDto` built during ELK layout.
- ELK coordinate conversion isolated in [layout.ts](../shared/diagram-renderer/src/render/layout.ts) and [ibd-route.ts](../shared/diagram-renderer/src/render/ibd-route.ts).
- `edgeCoords: ROOT` plus a small set of layout offsets (`edgeOwnerOffset`, `lcaOffset`) chosen by port-center fit — not semantic string guessing. Required for routes to nested ports inside compound ELK nodes.
- Snapshot / route-quality tests on scene fixtures (grid fixtures, nested-ring, two-part chain).
- Backend `elk_layout.rs` remains optional parity only; production layout stays in ELK.js.

### Phase 4: Draw a Resolved Scene — **core done; remainder deferred**

Goal: make the frontend renderer boring.

- **Done:** Drawing consumes `InterconnectionLayoutDto`; SVG paths and port rectangles come from layout DTO fields; no `_portAnchors` / `layoutRoutePoints` on node attributes.
- **Deferred:** Container boxes in `InterconnectionLayoutDto` (still use `packageContainerGroups`); webview diagnostics panel for scene/layout warnings.

### Phase 5: Delete Compatibility Heuristics — **done**

Goal: pay down the debt after migration.

Removed from the interconnection path:

- suffix-based endpoint owner matching in TypeScript;
- frontend connector owner inference and ibd-only prepare ([interconnection-legacy.ts](../shared/diagram-renderer/src/prepare/interconnection-legacy.ts));
- root selection heuristics in `normalize-payload.ts` for interconnection-view;
- attribute fallbacks for ports/routes in drawing.

**Still open:** open-ended `PreparedNode.attributes` for IBD (Phase 2 typing); heuristic port-side hints in layout for `sideHint: auto` (layout-only, not owner resolution).

## Concrete Near-Term Fixes

These are worth doing before the full migration:

1. Add a failing integration assertion for the current `systemContext` screenshot class:
   - route must not extend far outside content bounds;
   - route length must not exceed a configurable multiple of endpoint Manhattan distance;
   - every edge source/target must be a port when endpoint IDs identify ports.

2. Add an exported "scene debug JSON" command:
   - raw backend IBD;
   - selected/scoped IBD;
   - prepared interconnection scene;
   - ELK input;
   - ELK output;
   - final SVG route summary.

3. Add a backend invariant check:
   - if `connector.targetId` resolves to `A.B.C.port`, then `targetPartId` must be `A.B.C`, not `A.B`.

4. Add a frontend hard failure in development mode:
   - if an edge has `sourceId` / `targetId` that names a port but layout falls back to node boundary routing, log a structured diagnostic.

## Proposed Success Criteria

| Criterion | Status |
|-----------|--------|
| `systemContext` and `gridConnections` render without frontend semantic heuristics | **Met** (VS Code path) |
| Backend DTO contains canonical IDs for nodes, ports, and connector endpoints | **Met** (`InterconnectionSceneDto`) |
| TypeScript no longer parses qualified SysML names to determine edge owners | **Met** (prepare is mechanical) |
| ELK input graph snapshot-tested independently of SVG | **Met** (scene fixtures + `buildInterconnectionElkGraph`) |
| SVG export auditable for route sanity | **Met** (`assessRouteQuality`, pipeline export) |
| Deterministic rendering across reloads | **Met** (canonical scene + layout DTO) |
| Typed prepared nodes/ports/edges (no open `attributes` for IBD) | **Open** |
| Server CLI SVG uses same scene contract | **Open** |
| Webview shows scene/layout diagnostics | **Open** |

## Recommendation

The canonical scene migration for VS Code is **complete enough to stop tactical route patching**. Further work should focus on deferred items (diagnostics UI, layout-DTO containers, server SVG parity, stricter TypeScript types) rather than re-expanding frontend semantic inference.

If route quality regresses, use `sysml.debug.exportInterconnectionPipeline` and the scene fixtures before changing prepare or layout heuristics.
