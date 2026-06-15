import type { PreparedView, UnknownRecord, VisualizationPayload } from "./types";
import { asArray, asRecord, asString, elementTypeOf, nodeRange, nodeUri } from "./util";

function graphNodesForStandardView(visualization: VisualizationPayload): UnknownRecord[] {
  const graph = asRecord(visualization?.generalViewGraph ?? visualization?.graph);
  return asArray(graph.nodes).map(asRecord);
}

function graphEdgesForStandardView(visualization: VisualizationPayload): UnknownRecord[] {
  const graph = asRecord(visualization?.generalViewGraph ?? visualization?.graph);
  return asArray(graph.edges).map(asRecord);
}

function qualifiedNameOf(node: UnknownRecord): string {
  const attrs = asRecord(node.attributes);
  return asString(node.id ?? node.qualifiedName ?? attrs.qualifiedName ?? node.name);
}

function traceabilityLinkCount(nodeId: string, edges: UnknownRecord[]): number {
  let links = 0;
  for (const edge of edges) {
    const relType = asString(edge.type ?? edge.rel_type).toLowerCase();
    if (!/(satisfy|derivation|derive|verify|subject)/.test(relType)) continue;
    const source = asString(edge.source);
    const target = asString(edge.target);
    if (source === nodeId || target === nodeId) {
      links += 1;
    }
  }
  return links;
}

function packageLabelOf(qualifiedName: string): string {
  const segments = qualifiedName.split("::").filter(Boolean);
  return segments.length > 1 ? segments[0] : "";
}

function gridLayoutHint(visualization: VisualizationPayload): string | undefined {
  const hints = asRecord(visualization?.projectionHints);
  return asString(hints.gridLayout) || undefined;
}

export function prepareBrowser(visualization: VisualizationPayload): PreparedView {
  const graphNodes = graphNodesForStandardView(visualization)
    .map((node) => ({
      id: asString(node.id),
      label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
      kind: elementTypeOf(node) || "element",
      parentId: asString(node.parent_id ?? node.parentId ?? asRecord(node.attributes).parentId),
      qualifiedName: qualifiedNameOf(node),
      uri: nodeUri(node),
      range: nodeRange(node),
    }))
    .sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
  return {
    title: asString(visualization?.selectedViewName, "Browser View"),
    view: "browser-view",
    nodes: graphNodes.map((row, index) => ({
      id: row.id || `browser-row-${index}`,
      label: row.label,
      kind: row.kind,
      uri: row.uri,
      range: row.range,
      attributes: row,
    })),
    edges: [],
    meta: { rows: graphNodes, provisional: true },
  };
}

export function prepareGrid(visualization: VisualizationPayload): PreparedView {
  const graphEdges = graphEdgesForStandardView(visualization);
  const traceabilityLayout = gridLayoutHint(visualization) === "traceability";
  const cells = graphNodesForStandardView(visualization)
    .map((node) => {
      const attrs = asRecord(node.attributes);
      const qualifiedName = qualifiedNameOf(node);
      const nodeId = asString(node.id);
      const linkCount = traceabilityLinkCount(nodeId, graphEdges);
      return {
        id: nodeId,
        name: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
        kind: elementTypeOf(node) || "element",
        package: packageLabelOf(qualifiedName),
        qualifiedName,
        linkCount,
        attributeCount: asArray(attrs.attributes).length,
        partCount: asArray(attrs.parts).length,
        portCount: asArray(attrs.ports).length,
        uri: nodeUri(node),
        range: nodeRange(node),
      };
    })
    .sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
  return {
    title: asString(visualization?.selectedViewName, "Grid View"),
    view: "grid-view",
    nodes: cells.map((cell, index) => ({
      id: cell.id || `grid-row-${index}`,
      label: cell.name,
      kind: cell.kind,
      uri: cell.uri,
      range: cell.range,
      attributes: cell,
    })),
    edges: [],
    meta: { cells, traceabilityTable: traceabilityLayout, provisional: true },
  };
}

export function prepareGeometry(visualization: VisualizationPayload): PreparedView {
  const graphNodes = graphNodesForStandardView(visualization);
  const graphEdges = graphEdgesForStandardView(visualization);
  const elements = graphNodes
    .filter((node) => {
      const kind = elementTypeOf(node).toLowerCase();
      const name = asString(node.name ?? node.id).toLowerCase();
      return /(part|port|item|connection|geometry|shape|frame|coordinate|axis)/.test(`${kind} ${name}`);
    })
    .slice(0, 48)
    .map((node) => ({
      id: asString(node.id),
      label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
      kind: elementTypeOf(node) || "element",
      qualifiedName: asString(node.qualifiedName ?? asRecord(node.attributes).qualifiedName),
      uri: nodeUri(node),
      range: nodeRange(node),
    }));
  return {
    title: asString(visualization?.selectedViewName, "Geometry View"),
    view: "geometry-view",
    nodes: elements.map((element, index) => ({
      id: element.id || `geometry-node-${index}`,
      label: element.label,
      kind: element.kind,
      uri: element.uri,
      range: element.range,
      attributes: element,
    })),
    edges: graphEdges
      .map((edge, index) => ({
        id: asString(edge.id, `geometry-edge-${index}`),
        source: asString(edge.source),
        target: asString(edge.target),
        label: asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, ""),
      }))
      .filter(
        (edge) => elements.some((node) => node.id === edge.source) && elements.some((node) => node.id === edge.target),
      ),
    meta: { elements, provisional: true },
  };
}
