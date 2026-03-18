# JSON ↔ `elk_core::Graph` mapping (initial compatibility layer)

This describes how ELK Graph JSON (as implemented by ELK Java’s `org.eclipse.elk.graph.json`) maps into the current
Rust data model (`elk_core::Graph`) and back.

The goal of the first milestone is **load → layout → export** for a meaningful subset of models, while keeping the
design extensible for deeper parity (option registry, coordinate modes, hyperedges, etc.).

## IDs

ELK JSON allows `id` as **string or integer**. `elk_core` IDs are **dense indices** (`NodeId`, `PortId`, `EdgeId`, `LabelId`).

### Import

- Maintain a mapping:
  - `JsonId → NodeId`
  - `JsonId → PortId`
  - `JsonId → EdgeId`
  - `JsonId → (EdgeId, section_index)` (since `elk_core::EdgeSection` is stored inline)
- Accept JSON IDs as:
  - string
  - integer (fits `i64`/`u64`)
- Reject/skip invalid IDs (non-integer numbers, objects, arrays).

### Export

- By default, emit **string IDs** derived from Rust indices (e.g. `n0`, `p0`, `e0`) unless the imported model
  provided stable IDs, in which case preserve them (future enhancement).

## Graph hierarchy

ELK JSON represents hierarchy through nested nodes:

- root node object contains `children: [...]` of node objects
- each child node may contain its own `children`

`elk_core` represents hierarchy via:

- `Node.parent: Option<NodeId>`
- `Node.children: Vec<NodeId>`

### Import

- Create all nodes in a **DFS** over JSON `children`.
- Set `parent` and push into `parent.children`.
- For node positions:
  - Treat `x`,`y` as **preferred position** relative to parent (ELK commonly uses relative coords in compound graphs).
  - Store as `Node.preferred_position = Some(Point{x,y})` and keep `bounds.origin` as `(0,0)` until layout.

### Export

- Serialize `children` recursively.
- Emit `x`,`y`,`width`,`height` from the current `bounds` (or from preferred positions if layout omitted).

## Ports

ELK JSON ports are nested under a node’s `ports: [...]`.

`elk_core` ports are stored globally in `Graph.ports`, referenced by `Node.ports`.

### Import

- Create ports while visiting a node.
- `Port.node = owning NodeId`.
- **Port side**: ELK JSON doesn’t have a dedicated `side` field; it is typically encoded via `layoutOptions`.
  - Initial rule: default to `PortSide::East` (or derive from known option keys when present; see “Options”).
- Position/size:
  - Preserve `width`,`height` as port size.
  - Preserve `x`,`y` as `Port.bounds.origin` initially **as-is** (future: interpret according to port constraints).

### Export

- Emit port `id`, `x`,`y`,`width`,`height`, labels, and layout options.

## Labels

ELK JSON supports `labels: [...]` on nodes, ports, and edges.

`elk_core` uses `Graph.labels` with per-owner lists:

- `Node.labels: Vec<LabelId>`
- `Port.labels: Vec<LabelId>`
- `Edge.labels: Vec<LabelId>`

### Import

- Create a `Label` for each JSON label with:
  - `text` (default empty if missing)
  - `size` from `width`,`height` (defaults to 0 if missing)
  - `position` from `x`,`y` (defaults to 0 if missing)
- Attach to the owning element list.

### Export

- Emit `text`, optional `id` if present/known, `x`,`y`,`width`,`height`, and layout options.

## Edges

ELK JSON edges live on the *containing node* under `edges: [...]`.

`elk_core` stores edges globally in `Graph.edges`.

### Import (minimal milestone)

- Support both encodings:
  - **modern**: `sources: [...]`, `targets: [...]` arrays
  - **legacy**: `source`, `sourcePort`, `target`, `targetPort`
- `elk_core::EdgeEndpoint` supports only one source and one target, so for now:
  - If multiple sources/targets are present, pick the **first** valid source and first valid target and emit a warning.
  - Hyperedge parity is a follow-up.
- For edge route geometry:
  - Modern: if `sections` are present, take the **first** section and map:
    - `startPoint` → `EdgeSection.start`
    - `bendPoints[]` → `EdgeSection.bend_points`
    - `endPoint` → `EdgeSection.end`
  - Legacy: map `sourcePoint`/`bendPoints`/`targetPoint` similarly.

### Export

- Emit modern encoding with `sources` and `targets` arrays containing referenced node/port ids.
- Emit `sections` when layout info is present (`edge.sections` non-empty).

## Layout options / overrides

ELK JSON stores options in `layoutOptions` (or legacy `properties`) as string→primitive pairs.

`elk_core` currently stores typed overrides in `ElementLayoutOptions` (graph/node/port/edge/label) and some global `LayoutOptions`.

### Import

- Parse a **small subset** of options into typed fields (initially only those already present in `elk_core`), for example:
  - direction, edge routing, hierarchy handling, port constraints, layer constraints, content alignment, label placements
- Preserve all other key/value pairs in an “unknown options” map (new field or sidecar structure) for round-trip and future parity.

### Precedence

- ELK semantics: element `layoutOptions` override parent options.
- Rust model: `ElementLayoutOptions::inherit_from` already models this. Apply it consistently:
  - Graph-level defaults → node overrides → port/edge/label overrides.

## Coordinate modes (deferred)

ELK supports `CoreOptions.JSON_SHAPE_COORDS` / `JSON_EDGE_COORDS` to interpret coordinates in different frames.
For the first milestone, treat all numeric coords as **already in the local frame** of the imported container.

