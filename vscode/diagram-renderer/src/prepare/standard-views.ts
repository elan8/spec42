import type { PreparedView, UnknownRecord, VisualizationPayload } from "./types";
import { asArray, asRecord, asString, elementTypeOf, firstPresent, nodeRange, nodeUri } from "./util";

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

function projectionHints(visualization: VisualizationPayload): UnknownRecord {
  return asRecord(visualization?.projectionHints);
}

function gridSubtypeHint(visualization: VisualizationPayload): string | undefined {
  return asString(projectionHints(visualization).gridSubtype) || undefined;
}

function browserLayoutHint(visualization: VisualizationPayload): string | undefined {
  return asString(projectionHints(visualization).browserLayout) || undefined;
}

function treeRootHints(visualization: VisualizationPayload): string[] {
  return asArray(projectionHints(visualization).treeRoots).map((value) => asString(value)).filter(Boolean);
}

interface ColumnViewHint { label: string; renderingType: string; supported: boolean }

function columnViewsHint(visualization: VisualizationPayload): ColumnViewHint[] {
  return asArray(projectionHints(visualization).columnViews)
    .map(asRecord)
    .map((entry) => {
      const renderingType = asString(entry.renderingType);
      return {
        label: asString(entry.label, "Column"),
        renderingType,
        supported: renderingType === "asTextualNotation",
      };
    });
}

// Standard-view payloads normalize nullable graph fields to `undefined`
// so PreparedNode contracts remain consistent across renderers.
function optionalUri(node: UnknownRecord): string | undefined {
  return nodeUri(node) ?? undefined;
}

function optionalRange(node: UnknownRecord): UnknownRecord | undefined {
  return nodeRange(node) ?? undefined;
}

interface BrowserRow {
  id: string;
  label: string;
  kind: string;
  parentId: string;
  qualifiedName: string;
  visibility?: string;
  uri?: string;
  range?: UnknownRecord;
  depth: number;
  hasChildren: boolean;
}

function buildHierarchyRows(
  graphNodes: Array<{
    id: string;
    label: string;
    kind: string;
    parentId: string;
    qualifiedName: string;
    visibility?: string;
    uri?: string;
    range?: UnknownRecord;
  }>,
  treeRoots: string[],
): BrowserRow[] {
  const byId = new Map(graphNodes.map((node) => [node.id, node]));
  const childrenByParent = new Map<string, typeof graphNodes>();
  for (const node of graphNodes) {
    if (!node.parentId || !byId.has(node.parentId)) {
      continue;
    }
    const siblings = childrenByParent.get(node.parentId) ?? [];
    siblings.push(node);
    childrenByParent.set(node.parentId, siblings);
  }
  // Sibling order follows graphNodes' own order (source declaration order for SysML `ordered`
  // membership features) -- do not alphabetize, it discards semantically meaningful order.

  const roots =
    treeRoots.length > 0
      ? treeRoots.map((id) => byId.get(id)).filter((node): node is (typeof graphNodes)[number] => Boolean(node))
      : graphNodes.filter((node) => !node.parentId || !byId.has(node.parentId));
  // `roots` already reflects exposure order (treeRoots hint) or declaration order (graphNodes
  // filter) -- do not re-sort, that would discard the order those sources intentionally carry.

  const rows: BrowserRow[] = [];
  const visit = (node: (typeof graphNodes)[number], depth: number) => {
    const children = childrenByParent.get(node.id) ?? [];
    rows.push({
      ...node,
      depth,
      hasChildren: children.length > 0,
    });
    for (const child of children) {
      visit(child, depth + 1);
    }
  };

  for (const root of roots) {
    visit(root, 0);
  }
  return rows;
}

function buildRelationshipMatrix(
  nodeIds: string[],
  graphEdges: UnknownRecord[],
): Array<{ source: string; target: string; present: boolean; labels: string[] }> {
  const edgeByPair = new Map<string, string[]>();
  for (const edge of graphEdges) {
    const source = asString(edge.source);
    const target = asString(edge.target);
    if (!source || !target) continue;
    const label = asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, "");
    if (!label) continue;
    const key = `${source}::${target}`;
    const labels = edgeByPair.get(key) ?? [];
    labels.push(label);
    edgeByPair.set(key, labels);
  }
  const cells: Array<{ source: string; target: string; present: boolean; labels: string[] }> = [];
  for (const source of nodeIds) {
    for (const target of nodeIds) {
      const labels = edgeByPair.get(`${source}::${target}`) ?? [];
      cells.push({ source, target, present: labels.length > 0, labels });
    }
  }
  return cells;
}

export function prepareBrowser(visualization: VisualizationPayload): PreparedView {
  const graphNodes = graphNodesForStandardView(visualization).map((node) => ({
    id: asString(node.id),
    label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
    kind: elementTypeOf(node) || "element",
    parentId: asString(firstPresent(node.parent_id, node.parentId, asRecord(node.attributes).parentId)),
    qualifiedName: qualifiedNameOf(node),
    visibility: asString(asRecord(node.attributes).visibility) || undefined,
    uri: optionalUri(node),
    range: optionalRange(node),
  }));
  const hierarchyLayout = browserLayoutHint(visualization) === "hierarchy";
  const rows = hierarchyLayout
    ? buildHierarchyRows(graphNodes, treeRootHints(visualization))
    : graphNodes
        .map((row) => ({ ...row, depth: 0, hasChildren: false }))
        .sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
  return {
    title: asString(visualization?.selectedViewName, "Browser View"),
    view: "browser-view",
    nodes: rows.map((row, index) => ({
      id: row.id || `browser-row-${index}`,
      label: row.label,
      kind: row.kind,
      uri: row.uri,
      range: row.range,
      attributes: { ...row },
    })),
    edges: [],
    meta: { rows, hierarchyLayout, provisional: !hierarchyLayout },
  };
}

export function prepareGrid(visualization: VisualizationPayload): PreparedView {
  const graphEdges = graphEdgesForStandardView(visualization);
  const relationshipMatrix = gridSubtypeHint(visualization) === "relationship_matrix";
  const cells = graphNodesForStandardView(visualization)
    .map((node) => {
      const attrs = asRecord(node.attributes);
      const qualifiedName = qualifiedNameOf(node);
      const nodeId = asString(node.id);
      return {
        id: nodeId,
        name: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
        kind: elementTypeOf(node) || "element",
        qualifiedName,
        attributeCount: asArray(attrs.attributes).length,
        partCount: asArray(attrs.parts).length,
        portCount: asArray(attrs.ports).length,
        unsupportedRendering: "Unsupported rendering",
        uri: optionalUri(node),
        range: optionalRange(node),
      };
    })
    .sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
  const nodeIds = cells.map((cell) => cell.id).filter(Boolean);
  const matrixCells = relationshipMatrix ? buildRelationshipMatrix(nodeIds, graphEdges) : [];
  const columnViews = columnViewsHint(visualization);
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
    meta: {
      cells,
      relationshipMatrix,
      matrixRowIds: relationshipMatrix ? nodeIds : [],
      matrixColIds: relationshipMatrix ? nodeIds : [],
      matrixCells,
      columns:
        columnViews.length > 0
          ? columnViews.map((column) => ({
              key: column.supported ? "name" : "unsupportedRendering",
              label: column.label,
              renderingType: column.renderingType,
              notationStatus: column.supported ? "normative" : "unsupported",
            }))
          : undefined,
      // Both an element table and a relationship matrix are rectangular GridView
      // presentations described by §9.2.20.2.5.
      provisional: columnViews.some((column) => !column.supported),
    },
  };
}

export function prepareGeometry(visualization: VisualizationPayload): PreparedView {
  const graphNodes = graphNodesForStandardView(visualization);
  const graphEdges = graphEdgesForStandardView(visualization);
  const hints = projectionHints(visualization);
  const elements = graphNodes.map((node) => ({
    id: asString(node.id),
    label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
    kind: elementTypeOf(node) || "element",
    qualifiedName: qualifiedNameOf(node),
    uri: optionalUri(node),
    range: optionalRange(node),
  }));
  const nodeIds = new Set(elements.map((element) => element.id));
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
      .filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target)),
    meta: {
      elements,
      geometryMode: asString(hints.geometryMode, "2d"),
      geometryProjection: asString(hints.geometryProjection, "orthographic"),
      provisional: true,
    },
  };
}
