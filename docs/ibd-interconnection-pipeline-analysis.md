# IBD and Interconnection View Pipeline Analysis

Date: 2026-06-12

## Progress (2026-06-12)

Phases 0–2 are largely complete for Stedin `systemContext` and `gridConnections`. The CI contract is fixture-based route quality in `shared/diagram-renderer`, backed by Rust scene export tests when the Stedin workspace is present.

### Landed

- **`InterconnectionSceneDto`** — built in Rust ([interconnection_scene.rs](../crates/semantic_core/src/semantic/interconnection_scene.rs)), attached to LSP visualization payloads.
- **Instance-centric scoping** — `normalize_ibd_to_instance_paths`, architecture-scope filtering, variant/alternative exclusion ([visualization_workspace.rs](../crates/semantic_core/src/semantic/visualization_workspace.rs)).
- **Canonical prepare path** — [interconnection-scene.ts](../shared/diagram-renderer/src/prepare/interconnection-scene.ts); legacy fallback remains in [interconnection-legacy.ts](../shared/diagram-renderer/src/prepare/interconnection-legacy.ts).
- **Layout fixes** — ELK node ID sanitization, `edgeCoords: ROOT` without nested offset guessing, canonical single-offset routing ([layout.ts](../shared/diagram-renderer/src/render/layout.ts), [ibd-route.ts](../shared/diagram-renderer/src/render/ibd-route.ts)).
- **Debug export** — `sysml.debug.exportInterconnectionPipeline` (VS Code) and [pipeline-export.ts](../shared/diagram-renderer/src/pipeline-export.ts).
- **Fixtures** — `stedin-system-context-scene.json`, `stedin-grid-connections-scene.json` (regenerate via `cargo test -p semantic_core --test view_expose_stedin_interconnection export_stedin -- --nocapture`).

### CI contract (primary gate)

| Test | Location | What it guards |
|------|----------|----------------|
| `layout.interconnection.test.ts` | `shared/diagram-renderer` | ELK input + `assessRouteQuality` on scene fixtures (two-part chain, Stedin systemContext, Stedin gridConnections) |
| `route-quality.test.ts` | `shared/diagram-renderer` | Detached endpoints, bounds, node-boundary fallback detection |
| `view_expose_stedin_interconnection` | `semantic_core` | Semantic scoping, connector invariants, scene export (skipped if Stedin repo absent) |
| `stedin.visualization.test.ts` | `vscode` | LSP scene + `exportInterconnectionPipeline` route summary (optional soak; requires Stedin workspace) |

Local pipeline debug: `node shared/diagram-renderer/scripts/diagnose-stedin-scene.mjs [scene.json]`.

### Deferred (next plans)

- **Phase 4** — Drawing consumes `InterconnectionLayoutDto` only; remove `_portAnchors` / `_portDrawOrder` from `PreparedNode.attributes`; webview diagnostics panel for `scene.diagnostics`.
- **Phase 5** — Require `interconnectionScene` on all interconnection views; delete [interconnection-legacy.ts](../shared/diagram-renderer/src/prepare/interconnection-legacy.ts) and frontend root-selection heuristics in [normalize-payload.ts](../shared/diagram-renderer/src/prepare/normalize-payload.ts).
- **Backend ELK** — Keep production layout in ELK.js; optional parity tests via [elk_layout.rs](../crates/server/src/elk_layout.rs) only.

---

## Executive Summary

The current IBD / Interconnection View pipeline has accumulated enough tactical fixes that it is no longer a reliable architecture. The visible symptoms in `systemContext` are connector routes that appear to run through unrelated areas, attach to surprising sides of parts, or change shape after small fixes. The deeper issue is not only ELK.js routing. It is that semantic ownership, endpoint identity, view scoping, port identity, port side selection, container hierarchy, layout construction, and SVG drawing are all partially inferred in multiple places.

The most important recommendation is to make the backend produce a canonical, typed interconnection scene and make the frontend a mostly dumb renderer. The frontend should not be responsible for resolving connector endpoint owners, guessing whether a connector target is a nested part or a container, deciding SysML endpoint identity from suffixes, or repairing ELK coordinate spaces after the fact.

The current code already contains a lot of the needed backend knowledge, but it is exposed through a DTO that is too weak and then reinterpreted in TypeScript. This is where the technical debt concentrates.

## Current Pipeline

The pipeline is roughly:

1. Rust semantic graph construction builds the workspace semantic graph.
2. Rust IBD extraction builds `IbdDataDto` in [crates/semantic_core/src/semantic/ibd.rs](../crates/semantic_core/src/semantic/ibd.rs).
3. Rust visualization workspace selection evaluates SysML views and scopes IBD payloads in [crates/semantic_core/src/semantic/visualization_workspace.rs](../crates/semantic_core/src/semantic/visualization_workspace.rs).
4. The VS Code extension fetches visualization DTOs in [vscode/src/visualization/modelFetcher.ts](../vscode/src/visualization/modelFetcher.ts).
5. The webview passes the DTO almost unchanged through [vscode/src/visualization/dtoAdapter.ts](../vscode/src/visualization/dtoAdapter.ts).
6. The shared renderer normalizes DTOs in [shared/diagram-renderer/src/prepare/normalize-payload.ts](../shared/diagram-renderer/src/prepare/normalize-payload.ts).
7. Interconnection preparation maps raw IBD objects into `PreparedNode` / `PreparedEdge` in [shared/diagram-renderer/src/prepare/interconnection.ts](../shared/diagram-renderer/src/prepare/interconnection.ts).
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

The Stedin VS Code test currently confirms that the diagram has parts and connectors. That is useful, but it did not catch the screenshot failure because all connectors can exist and touch ports while still producing terrible routes.

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

### Phase 3: Isolate ELK Integration

Goal: make layout deterministic and testable.

- Create a dedicated layout contract with canonical nodes, ports, edges, and containers.
- Keep all ELK coordinate conversion in one file.
- Remove candidate offset guessing from route rendering.
- Make ELK edge coordinate mode explicit and covered by tests.
- Add snapshot tests for ELK input graph on representative diagrams.
- Consider using backend `crates/server/src/elk_layout.rs` for parity/CLI tests, but do not maintain two divergent production layout paths.

### Phase 4: Draw a Resolved Scene

Goal: make the frontend renderer boring.

- Drawing consumes `InterconnectionLayoutDto`.
- SVG paths come from layout edge routes directly.
- Port rectangles come from layout port anchors directly.
- Containers come from layout container boxes directly.
- No `_portAnchors`, `_portDrawOrder`, or semantic hidden fields inside generic node attributes.

### Phase 5: Delete Compatibility Heuristics

Goal: pay down the debt after migration.

Remove:

- suffix-based endpoint owner matching in TypeScript;
- frontend connector owner inference;
- frontend port side usage matching by labels;
- root selection heuristics in `normalize-payload.ts`;
- fallback route sections except as explicit error-state diagnostics;
- open-ended semantic fields in `PreparedNode.attributes` for IBD.

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

The rewrite should be considered successful when:

- `systemContext` and `gridConnections` render without frontend semantic heuristics.
- The backend DTO contains canonical IDs for all nodes, ports, and connector endpoints.
- TypeScript no longer parses qualified SysML names to determine edge owners.
- ELK input graph can be snapshot-tested independently of SVG.
- SVG export can be audited mechanically for route sanity.
- Rendering a selected `InterconnectionView` is deterministic across reloads.

## Recommendation

Do not keep patching individual `systemContext` route shapes. The current pipeline makes every local fix risky because the same semantic information is re-derived several times. The next serious investment should be a canonical backend-owned interconnection scene DTO, followed by a thinner renderer and an isolated ELK layout adapter.

The immediate tactical fixes can reduce the worst symptoms, but they should be treated as stabilizers, not the final architecture.
