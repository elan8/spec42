# ELK Graph JSON format (what we need to support)

This document is derived from ELK’s Java implementation in
`C:\Git\elk-master\plugins\org.eclipse.elk.graph.json` (notably `JsonImporter.xtend` / `JsonExporter.xtend`).

## Top-level

- The **top-level value is a JSON object** representing the **root node**.
- Every element is expected to have an `id` (string or integer). In ELK Java:
  - `id` may be **string** or **integer**.
  - Missing `id` is an import error for nodes/ports/edges/sections; labels may omit it.

## Common shape fields (nodes, ports, labels)

All shapes may carry:

- `x`, `y` (numbers) — position
- `width`, `height` (numbers) — size

Import accepts missing fields and treats NaN/∞ as `0.0`.

## Nodes

A node object may contain:

- `id`: string | integer (required)
- `children`: array of node objects
- `ports`: array of port objects
- `labels`: array of label objects
- `edges`: array of edge objects (edges are stored on the **containing node** / hierarchy level)
- `layoutOptions`: object of string→(primitive) (see “Options/properties”)
  - Legacy alias: `properties` is accepted on import as a fallback.
- `individualSpacings`: object (see “Individual spacings”)

## Ports

A port object may contain:

- `id`: string | integer (required)
- `labels`: array of label objects
- `layoutOptions` / `properties`: object
- `individualSpacings`: object
- `x`, `y`, `width`, `height`

**Note:** Port side (`north/south/east/west`) is *not* encoded as a dedicated JSON field in the graph-json plugin.
In ELK it is usually conveyed via `layoutOptions` (e.g. core/port options).

## Labels

A label object may contain:

- `text`: string (used by exporter; importer reads it)
- `id`: string | integer (optional)
- `layoutOptions` / `properties`: object
- `x`, `y`, `width`, `height`

## Edges

There are two edge encodings supported by ELK’s importer:

### Modern (hyperedge-capable)

- `id`: string | integer (required)
- `sources`: array of IDs (each ID references either a node `id` or a port `id`)
- `targets`: array of IDs (same rules)
- `sections`: array of edge-section objects (optional)
- `labels`: array of label objects (optional)
- `junctionPoints`: array of `{x,y}` objects (optional)
- `layoutOptions` / `properties`: object (optional)

### Legacy (“primitive edge”)

- `id`: string | integer (required)
- `source`: node id (required)
- `sourcePort`: port id (optional)
- `target`: node id (required)
- `targetPort`: port id (optional)
- `sourcePoint`: `{x,y}` (optional)
- `targetPoint`: `{x,y}` (optional)
- `bendPoints`: array of `{x,y}` (optional)
- `labels`: array of label objects (optional)
- `layoutOptions` / `properties`: object (optional)

## Edge sections

Each section object:

- `id`: string | integer (required)
- `startPoint`: `{x,y}` (required)
- `endPoint`: `{x,y}` (required)
- `bendPoints`: array of `{x,y}` (optional)
- `incomingShape`: id of a node/port (optional)
- `outgoingShape`: id of a node/port (optional)
- `incomingSections`: array of section ids (optional)
- `outgoingSections`: array of section ids (optional)
- `layoutOptions` / `properties`: object (optional)

Special case in ELK: if an edge has one source, one target, and one section with missing `incomingShape`/`outgoingShape`,
ELK fills them from the edge endpoints.

## Options / properties

ELK imports layout options from:

- `layoutOptions` object if present, else
- legacy `properties` object

Values are coerced to strings by the JSON adapter and then parsed via ELK’s option metadata registry
(`LayoutMetaDataService.getOptionDataBySuffix(id)`).

For elk-rust, initial support should focus on:

- preserving unknown option key/values (for round-trip),
- decoding a **small known subset** into `elk_core::LayoutOptions` / `ElementLayoutOptions`,
- and keeping everything else accessible for later parity.

## Coordinate modes (advanced)

ELK uses core options to control how coordinates are interpreted and transferred back to JSON:

- `CoreOptions.JSON_SHAPE_COORDS` (shape coords mode)
- `CoreOptions.JSON_EDGE_COORDS` (edge coords mode)

elk-rust’s first milestone can ignore these and treat coordinates as **absolute within the current imported graph**,
as long as we can still load the model corpus and run layout.

