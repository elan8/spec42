import ELK from "elkjs/lib/elk.bundled.js";
import {
  isConnectorUsageElementType,
  isOverviewVisualElementType,
  normalizeEdgeKind,
} from "../graph-normalization";
import {
  collectCompartments,
  computeNodeHeight,
  computeNodeWidth,
  nodeChromeStateFromAttributes,
} from "../sysml-node-builder";
import {
  interconnectionPreparedForLayout,
  type PreparedNode,
  type PreparedView,
} from "../prepare";
import type { InterconnectionLayoutNodeDto } from "../prepare/types";
import { lcaOffsetForNodes } from "./ibd-route";
import {
  createInterconnectionLayoutBuildState,
  finalizeInterconnectionLayoutDto,
  recordInterconnectionLayoutContainer,
  recordInterconnectionLayoutNode,
} from "./interconnection-layout-dto";
import { buildInterconnectionElkBuild } from "./interconnection-elk-input";
import { buildElkLayoutOptions } from "./elk-options";
import {
  ibdNodeHeight,
  ibdNodeWidth,
  nodeHeight,
  nodeWidth,
  type EdgeSection,
  type LaidOutEdge,
  type LaidOutNode,
  type LayoutResult,
} from "./types";

const elk = new ELK();

/**
 * Measured height of a General View node. Height is whatever the node chrome layout needs for the
 * header plus its compartment blocks -- there is no fixed minimum box, so a node without
 * compartments is exactly its own header instead of a header plus dead space.
 */
function generalNodeBox(
  node: PreparedNode,
  compartments: ReturnType<typeof collectCompartments>,
): { width: number; height: number } {
  const state = nodeChromeStateFromAttributes(node.attributes);
  const width = computeNodeWidth(compartments, { maxLinesPerCompartment: 8 }, state);
  return {
    width,
    height: computeNodeHeight(compartments, { maxLinesPerCompartment: 8 }, { width, state }),
  };
}

function fallbackGeneralLayout(
  nodes: PreparedNode[],
  edges: PreparedView["edges"],
): LayoutResult {
  const columns = Math.max(1, Math.ceil(Math.sqrt(nodes.length)));
  const horizontalGap = 90;
  const verticalGap = 90;
  const nodeData = nodes.map((node) => {
    const compartments = collectCompartments(node);
    return { node, compartments, ...generalNodeBox(node, compartments) };
  });
  const rowHeight = Math.max(nodeHeight, ...nodeData.map(({ height }) => height));
  const columnWidth = Math.max(nodeWidth, ...nodeData.map(({ width }) => width));
  const laidOutNodes: LaidOutNode[] = nodeData.map(({ node, compartments, width, height }, index) => {
    return {
      ...node,
      compartments,
      x: (index % columns) * (columnWidth + horizontalGap),
      y: Math.floor(index / columns) * (rowHeight + verticalGap),
      width,
      height,
    };
  });
  const byId = new Map(laidOutNodes.map((node) => [node.id, node]));
  return {
    nodes: laidOutNodes,
    edges: edges.map((edge) => {
      const sourceNode = byId.get(edge.source);
      const targetNode = byId.get(edge.target);
      const startPoint = {
        x: (sourceNode?.x ?? 0) + (sourceNode?.width ?? nodeWidth),
        y: (sourceNode?.y ?? 0) + (sourceNode?.height ?? nodeHeight) / 2,
      };
      const endPoint = {
        x: targetNode?.x ?? 0,
        y: (targetNode?.y ?? 0) + (targetNode?.height ?? nodeHeight) / 2,
      };
      const midX = (startPoint.x + endPoint.x) / 2;
      return {
        ...edge,
        sourceNode,
        targetNode,
        layout: {
          sections: [{
            startPoint,
            bendPoints: [
              { x: midX, y: startPoint.y },
              { x: midX, y: endPoint.y },
            ],
            endPoint,
          }],
        },
      };
    }),
  };
}

/** GeneralView hierarchy is semantic, so ownership depth—not unrelated connector direction—owns
 * row placement. Keep every depth on one horizontal row and route relationships afterwards. */
function hierarchyGeneralLayout(
  nodes: PreparedNode[],
  edges: PreparedView["edges"],
  parentById: Map<string, string>,
): LayoutResult {
  const nodeOrder = new Map(nodes.map((node, index) => [node.id, index]));
  const childrenByParent = new Map<string, PreparedNode[]>();
  for (const node of nodes) {
    const parent = parentById.get(node.id);
    if (!parent) continue;
    const children = childrenByParent.get(parent) ?? [];
    children.push(node);
    childrenByParent.set(parent, children);
  }
  for (const children of childrenByParent.values()) {
    children.sort((left, right) => (nodeOrder.get(left.id) ?? 0) - (nodeOrder.get(right.id) ?? 0));
  }

  const rows: PreparedNode[][] = [];
  const visited = new Set<string>();
  let current = nodes.filter((node) => !parentById.has(node.id));
  while (current.length > 0) {
    const row = current.filter((node) => !visited.has(node.id));
    if (row.length === 0) break;
    rows.push(row);
    row.forEach((node) => visited.add(node.id));
    current = row.flatMap((node) => childrenByParent.get(node.id) ?? []);
  }
  const unvisited = nodes.filter((node) => !visited.has(node.id));
  if (unvisited.length > 0) rows.push(unvisited);

  const horizontalGap = 140;
  const verticalGap = 180;
  const measuredRows = rows.map((row) => row.map((node) => {
    const compartments = collectCompartments(node);
    return { node, compartments, ...generalNodeBox(node, compartments) };
  }));
  const rowWidths = measuredRows.map((row) =>
    row.reduce((sum, item) => sum + item.width, 0) + Math.max(0, row.length - 1) * horizontalGap,
  );
  const diagramWidth = Math.max(0, ...rowWidths);
  const laidOutNodes: LaidOutNode[] = [];
  let y = 0;
  for (const [rowIndex, row] of measuredRows.entries()) {
    let x = (diagramWidth - rowWidths[rowIndex]) / 2;
    const rowHeight = Math.max(nodeHeight, ...row.map((item) => item.height));
    for (const item of row) {
      laidOutNodes.push({ ...item.node, compartments: item.compartments, x, y, width: item.width, height: item.height });
      x += item.width + horizontalGap;
    }
    y += rowHeight + verticalGap;
  }

  const byId = new Map(laidOutNodes.map((node) => [node.id, node]));
  const rowFor = new Map(rows.flatMap((row, depth) => row.map((node) => [node.id, depth] as const)));
  let sameRowLane = 0;
  const routedEdges: LaidOutEdge[] = edges.map((edge) => {
    const sourceNode = byId.get(edge.source);
    const targetNode = byId.get(edge.target);
    if (!sourceNode || !targetNode) return { ...edge, sourceNode, targetNode };
    const sourceX = (sourceNode.x ?? 0) + (sourceNode.width ?? nodeWidth) / 2;
    const targetX = (targetNode.x ?? 0) + (targetNode.width ?? nodeWidth) / 2;
    const sourceDepth = rowFor.get(edge.source) ?? 0;
    const targetDepth = rowFor.get(edge.target) ?? 0;
    let section: EdgeSection;
    if (sourceDepth === targetDepth) {
      const laneY = Math.min(sourceNode.y ?? 0, targetNode.y ?? 0) - 28 - (sameRowLane++ % 12) * 10;
      section = {
        startPoint: { x: sourceX, y: sourceNode.y ?? 0 },
        bendPoints: [{ x: sourceX, y: laneY }, { x: targetX, y: laneY }],
        endPoint: { x: targetX, y: targetNode.y ?? 0 },
      };
    } else {
      const downward = (targetNode.y ?? 0) > (sourceNode.y ?? 0);
      const startY = (sourceNode.y ?? 0) + (downward ? (sourceNode.height ?? nodeHeight) : 0);
      const endY = (targetNode.y ?? 0) + (downward ? 0 : (targetNode.height ?? nodeHeight));
      const middleY = (startY + endY) / 2;
      section = {
        startPoint: { x: sourceX, y: startY },
        bendPoints: [{ x: sourceX, y: middleY }, { x: targetX, y: middleY }],
        endPoint: { x: targetX, y: endY },
      };
    }
    return { ...edge, sourceNode, targetNode, layout: { sections: [section] } };
  });
  return { nodes: laidOutNodes, edges: routedEdges };
}

export async function layoutPrepared(prepared: PreparedView): Promise<LayoutResult> {
  if (!prepared.nodes.length) return { nodes: [], edges: [] };
  if (prepared.view === "interconnection-view") {
    return layoutInterconnectionPrepared(prepared);
  }
  if (
    prepared.view === "action-flow-view" ||
    prepared.view === "state-transition-view" ||
    prepared.view === "sequence-view" ||
    prepared.view === "browser-view" ||
    prepared.view === "grid-view" ||
    prepared.view === "geometry-view"
  ) {
    return { nodes: [], edges: [] };
  }
  // Only general-view reaches here — interconnection-view returned above, and the other 6 kinds
  // returned `{ nodes: [], edges: [] }` (laid out elsewhere; see views/behavior-common.ts and
  // views/standard-views-render.ts).
  // Relationship usages remain in the prepared semantic projection for compartments, navigation
  // and traceability. Connector usages are never structural peers in a General View: when their
  // ends resolve they are drawn as edges, and when they do not resolve there is no meaningful
  // structural box to draw. Other relationship declarations are suppressed once represented by
  // a composed edge.
  const representedRelationshipNodes = new Set(
    prepared.edges.flatMap((edge) => {
      const origin = edge.attributes?.originNodeId;
      const kind = normalizeEdgeKind(edge.edgeKind ?? edge.label);
      return typeof origin === "string" && origin !== edge.source && origin !== edge.target && kind !== "hierarchy"
        ? [origin]
        : [];
    }),
  );
  const diagramNodes = prepared.nodes.filter(
    (node) => isOverviewVisualElementType(node.kind) &&
      !isConnectorUsageElementType(node.kind) &&
      !representedRelationshipNodes.has(node.id),
  );
  const visibleIds = new Set(diagramNodes.map((node) => node.id));
  const diagramEdges = prepared.edges.filter(
    (edge) => visibleIds.has(edge.source) && visibleIds.has(edge.target),
  );
  if (!diagramNodes.length) return { nodes: [], edges: [] };

  const packageGroups =
    (prepared.meta?.packageContainerGroups as
      | Array<{ id: string; name: string; memberIds: string[] }>
      | undefined) ?? [];
  const useHierarchy = packageGroups.length >= 2;
  const containmentParent = new Map(
    diagramEdges
      .filter((edge) => normalizeEdgeKind(edge.edgeKind ?? edge.label) === "hierarchy")
      .map((edge) => [edge.target, edge.source]),
  );
  const useContainmentRows = !useHierarchy && containmentParent.size > 0;
  if (useContainmentRows) {
    return hierarchyGeneralLayout(diagramNodes, diagramEdges, containmentParent);
  }

  const leafElkNode = (node: PreparedNode) => {
    const compartments = collectCompartments(node);
    const box = generalNodeBox(node, compartments);
    return {
      id: node.id,
      width: box.width,
      height: box.height,
    };
  };

  // Large graphs without a containment hierarchy still need compact wrapping. Semantic
  // containment graphs returned above and deliberately retain one horizontal row per depth.
  // Synthetic chunk ids remain layout-only and are never drawn as package containers.
  const WIDE_SIBLING_THRESHOLD = 8;
  const chunkedElkChildren = (idPrefix: string, elkNodes: unknown[]): unknown[] => {
    if (elkNodes.length <= WIDE_SIBLING_THRESHOLD) return elkNodes;
    const chunkSize = Math.max(1, Math.ceil(Math.sqrt(elkNodes.length) / 2));
    const chunks: unknown[] = [];
    for (let i = 0; i < elkNodes.length; i += chunkSize) {
      chunks.push({
        id: `${idPrefix}#chunk${chunks.length}`,
        layoutOptions: {
          "elk.direction": "DOWN",
          "elk.padding": "[top=8,left=8,bottom=8,right=8]",
        },
        children: elkNodes.slice(i, i + chunkSize),
      });
    }
    return chunks;
  };

  // General-view: give ELK real package containment (mirroring the IBD hierarchy pattern in
  // interconnection-elk-input.ts) so each package lays out as a compact block instead of a flat
  // layered graph scattering package members anywhere, which otherwise produces very wide,
  // tangled diagrams for models with more than a handful of packages.
  let children: unknown[];
  let flatChildrenWereChunked = false;
  if (useHierarchy) {
    const memberToPackage = new Map<string, string>();
    for (const group of packageGroups) {
      for (const memberId of group.memberIds) memberToPackage.set(memberId, group.id);
    }
    const byPackage = new Map<string, unknown[]>();
    const orphans: unknown[] = [];
    for (const node of diagramNodes) {
      const pkgId = memberToPackage.get(node.id);
      const elkNode = leafElkNode(node);
      if (pkgId) {
        const list = byPackage.get(pkgId) ?? [];
        list.push(elkNode);
        byPackage.set(pkgId, list);
      } else {
        orphans.push(elkNode);
      }
    }
    const containers = packageGroups
      .filter((group) => (byPackage.get(group.id) ?? []).length > 0)
      .map((group) => ({
        id: group.id,
        layoutOptions: {
          "elk.direction": "DOWN",
          "elk.padding": "[top=36,left=20,bottom=20,right=20]",
        },
        children: chunkedElkChildren(group.id, byPackage.get(group.id) ?? []),
      }));
    children = [...containers, ...orphans];
  } else {
    const flatChildren = diagramNodes.map(leafElkNode);
    // A containment projection already has meaningful ranks: one horizontal row per ownership
    // depth. Preserve that hierarchy even for wide sibling sets. Chunk only unrelated flat graphs.
    children = chunkedElkChildren("root", flatChildren);
    flatChildrenWereChunked = children !== flatChildren;
  }

  const graph = {
    id: "root",
    layoutOptions: buildElkLayoutOptions("general", {
      "elk.hierarchyHandling": useHierarchy || flatChildrenWereChunked ? "INCLUDE_CHILDREN" : undefined,
    }),
    children,
    edges: diagramEdges.map((edge) => ({ id: edge.id, sources: [edge.source], targets: [edge.target] }))
  };
  let laidOut: Awaited<ReturnType<typeof elk.layout>>;
  try {
    laidOut = await elk.layout(graph as unknown as Parameters<typeof elk.layout>[0]);
  } catch (hierarchicalError) {
    if (!useHierarchy) {
      return fallbackGeneralLayout(diagramNodes, diagramEdges);
    }
    // The embedded QuickJS ELK worker has a lower recursion ceiling than browsers. A large graph
    // with package containment can exceed it even though the same graph lays out in the webview.
    // Retry the exact render product without ELK hierarchy; package frames are still drawn from
    // the semantic package groups after layout.
    const flatGraph = {
      id: "root",
      layoutOptions: buildElkLayoutOptions("general"),
      children: diagramNodes.map(leafElkNode),
      edges: diagramEdges.map((edge) => ({
        id: edge.id,
        sources: [edge.source],
        targets: [edge.target],
      })),
    };
    try {
      laidOut = await elk.layout(flatGraph as unknown as Parameters<typeof elk.layout>[0]);
    } catch {
      return fallbackGeneralLayout(diagramNodes, diagramEdges);
    }
  }
  const byId = new Map(diagramNodes.map((node) => [node.id, node]));

  // Resolve absolute positions recursively: with real package containment, leaf node x/y from
  // ELK are relative to their containing package node, not the diagram root.
  const layouts = new Map<string, any>();
  const visit = (elkNode: any, ox: number, oy: number) => {
    const absX = ox + (elkNode.x ?? 0);
    const absY = oy + (elkNode.y ?? 0);
    layouts.set(String(elkNode.id), { ...elkNode, x: absX, y: absY });
    for (const child of elkNode.children ?? []) visit(child, absX, absY);
  };
  for (const child of laidOut.children ?? []) visit(child, 0, 0);

  // Edges may be recorded on the lowest common ancestor container's own `.edges` array rather
  // than the root's, even with `edgeCoords: ROOT` section coordinates — collect recursively.
  const edgesById = new Map<string, any>();
  const collectEdges = (elkNode: any) => {
    for (const elkEdge of elkNode.edges ?? []) {
      if (elkEdge?.id) edgesById.set(String(elkEdge.id), elkEdge);
    }
    for (const child of elkNode.children ?? []) collectEdges(child);
  };
  collectEdges(laidOut);

  return {
    nodes: diagramNodes.map((node) => {
      const compartments = collectCompartments(node);
      return { ...node, compartments, ...(layouts.get(node.id) || {}) };
    }),
    edges: diagramEdges.map((edge) => ({
      ...edge,
      sourceNode: byId.get(edge.source),
      targetNode: byId.get(edge.target),
      layout: edgesById.get(edge.id) as LaidOutEdge["layout"]
    }))
  };
}

export async function layoutInterconnectionPrepared(prepared: PreparedView): Promise<LayoutResult> {
  const interconnection = interconnectionPreparedForLayout(prepared);
  const layoutBuildState = createInterconnectionLayoutBuildState();
  const { elkGraphInput, elkEdges, nodesById, preparedIdForElkId, portDrawOrderFor } =
    buildInterconnectionElkBuild(interconnection);

  const nodeBoundaryPoint = (node: LaidOutNode, role: "source" | "target"): { x: number; y: number } => ({
    x: (node.x ?? 0) + (role === "source" ? (node.width ?? ibdNodeWidth) : 0),
    y: (node.y ?? 0) + (node.height ?? ibdNodeHeight) / 2,
  });
  const fallbackEdgeSections = (
    sourceNode: LaidOutNode | undefined,
    targetNode: LaidOutNode | undefined,
    sourcePortCenter?: { x: number; y: number },
    targetPortCenter?: { x: number; y: number },
  ): EdgeSection[] | undefined => {
    if (!sourceNode || !targetNode) return undefined;
    const startPoint = sourcePortCenter ?? nodeBoundaryPoint(sourceNode, "source");
    const endPoint = targetPortCenter ?? nodeBoundaryPoint(targetNode, "target");
    const midX = (startPoint.x + endPoint.x) / 2;
    return [
      {
        startPoint,
        bendPoints: [
          { x: midX, y: startPoint.y },
          { x: midX, y: endPoint.y },
        ],
        endPoint,
      },
    ];
  };

  try {
    const laidOut = await elk.layout(elkGraphInput as unknown as Parameters<typeof elk.layout>[0]);
    const laidOutNodes = new Map<string, LaidOutNode>();
    const portCenters = new Map<string, { x: number; y: number }>();
    const nodePortAnchors = new Map<string, InterconnectionLayoutNodeDto["portAnchors"]>();

    const visit = (elkNode: any, ox: number, oy: number, depth: number) => {
      const absX = ox + (elkNode.x ?? 0);
      const absY = oy + (elkNode.y ?? 0);
      const preparedId = preparedIdForElkId.get(String(elkNode.id)) ?? String(elkNode.id);
      const base = nodesById.get(preparedId);
      for (const port of elkNode.ports ?? []) {
        const pw = port.width ?? 10;
        const ph = port.height ?? 10;
        const side = port?.layoutOptions?.["org.eclipse.elk.port.side"];
        const x =
          side === "WEST"
            ? absX + (port.x ?? 0)
            : side === "EAST"
              ? absX + (port.x ?? 0) + pw
              : absX + (port.x ?? 0) + pw / 2;
        const y = absY + (port.y ?? 0) + ph / 2;
        portCenters.set(String(port.id), { x, y });
        if (base) {
          const portName = String(port.id).split("__port__").pop() ?? String(port.id);
          const anchors = nodePortAnchors.get(base.id) ?? {};
          const label = port.labels?.[0];
          anchors[portName] = {
            x: x - absX,
            y: y - absY,
            side: String(side || ""),
            ...(label
              ? {
                  label: {
                    x: (port.x ?? 0) + (label.x ?? 0),
                    y: (port.y ?? 0) + (label.y ?? 0),
                    width: label.width ?? 0,
                    height: label.height ?? 0,
                    text: String(label.text ?? ""),
                  },
                }
              : {}),
          };
          nodePortAnchors.set(base.id, anchors);
        }
      }
      if (base) {
        const attrs = base.attributes ?? {};
        const hasLayoutChildren = Array.isArray(elkNode.children) && elkNode.children.length > 0;
        const isContainerFrame = hasLayoutChildren || Boolean(attrs.isSyntheticContainer);
        const portDrawOrder = portDrawOrderFor(base);
        const portAnchors = nodePortAnchors.get(base.id) ?? {};
        const laidOutWidth = elkNode.width ?? ibdNodeWidth;
        const laidOutHeight = elkNode.height ?? ibdNodeHeight;
        recordInterconnectionLayoutNode(
          layoutBuildState,
          { id: base.id, x: absX, y: absY, width: laidOutWidth, height: laidOutHeight },
          portAnchors,
          portDrawOrder,
        );
        if (isContainerFrame) {
          recordInterconnectionLayoutContainer(layoutBuildState, {
            id: base.id,
            label: base.label,
            x: absX,
            y: absY,
            width: laidOutWidth,
            height: laidOutHeight,
          });
        }
        laidOutNodes.set(base.id, {
          ...base,
          x: absX,
          y: absY,
          width: laidOutWidth,
          height: laidOutHeight,
          attributes: {
            ...(base.attributes ?? {}),
            _isLayoutContainer: hasLayoutChildren,
            _layoutDepth: depth,
          },
        });
      }
      for (const child of elkNode.children ?? []) {
        visit(child, absX, absY, depth + 1);
      }
    };

    for (const child of laidOut.children ?? []) {
      visit(child, 0, 0, 0);
    }

    const edgeLayout = new Map<string, { edge: any; offset: { x: number; y: number } }>();
    const collectElkEdgesWithOffsets = (
      elkNode: any,
      containerOffset: { x: number; y: number },
    ) => {
      for (const elkEdge of elkNode.edges ?? []) {
        const edgeId = String(elkEdge?.id ?? "");
        if (!edgeId) continue;
        edgeLayout.set(edgeId, { edge: elkEdge, offset: containerOffset });
      }
      for (const child of elkNode.children ?? []) {
        collectElkEdgesWithOffsets(child, {
          x: containerOffset.x + (child.x ?? 0),
          y: containerOffset.y + (child.y ?? 0),
        });
      }
    };
    collectElkEdgesWithOffsets(laidOut, { x: 0, y: 0 });
    for (const elkEdge of laidOut.edges ?? []) {
      const edgeId = String(elkEdge?.id ?? "");
      if (!edgeId) continue;
      edgeLayout.set(edgeId, { edge: elkEdge, offset: { x: 0, y: 0 } });
    }

    const nodes = interconnection.nodes
      .map((node) => laidOutNodes.get(node.id))
      .filter((value): value is LaidOutNode => Boolean(value));

    const edges = interconnection.edges.map((edge) => {
      const layoutRecord = edgeLayout.get(edge.id);
      const elkEdge = elkEdges.find((item) => item.id === edge.id);
      const sourceNode = laidOutNodes.get(edge.source);
      const targetNode = laidOutNodes.get(edge.target);
      const sourcePortCenter = elkEdge?.sourcePortId ? portCenters.get(elkEdge.sourcePortId) : undefined;
      const targetPortCenter = elkEdge?.targetPortId ? portCenters.get(elkEdge.targetPortId) : undefined;
      if (
        (edge.attributes?.sourcePortId || edge.attributes?.targetPortId) &&
        (!sourcePortCenter || !targetPortCenter)
      ) {
        layoutBuildState.diagnostics.push(
          `node-boundary fallback for edge ${edge.id}`,
        );
      }
      return {
        ...edge,
        sourceNode,
        targetNode,
        layout: layoutRecord?.edge.sections?.length
          ? {
              sections: layoutRecord.edge.sections as EdgeSection[],
              edgeOwnerOffset: layoutRecord.offset,
              lcaOffset:
                sourceNode && targetNode
                  ? lcaOffsetForNodes(sourceNode, targetNode, laidOutNodes)
                  : { x: 0, y: 0 },
            }
          : {
              sections: fallbackEdgeSections(sourceNode, targetNode, sourcePortCenter, targetPortCenter),
              edgeOwnerOffset: { x: 0, y: 0 },
              lcaOffset: { x: 0, y: 0 },
            },
        attributes: {
          ...(edge.attributes ?? {}),
          _sourcePortCenter: sourcePortCenter,
          _targetPortCenter: targetPortCenter,
        },
      } satisfies LaidOutEdge;
    });

    return {
      nodes,
      edges,
      interconnectionLayout: finalizeInterconnectionLayoutDto(layoutBuildState, edges),
    };
  } catch {
    // Interconnection notation must not degrade into a heuristic layout if ELK fails.
    return { nodes: [], edges: [] };
  }
}

export function buildInterconnectionElkGraph(prepared: PreparedView): Record<string, unknown> {
  const nodesById = new Map(prepared.nodes.map((node) => [node.id, node]));
  const childrenByParent = new Map<string, PreparedNode[]>();
  const roots: PreparedNode[] = [];
  for (const node of prepared.nodes) {
    const attrs = (node.attributes ?? {}) as Record<string, unknown>;
    const parentId = typeof attrs.containerId === "string" ? attrs.containerId : "";
    if (parentId && nodesById.has(parentId)) {
      const current = childrenByParent.get(parentId) ?? [];
      current.push(node);
      childrenByParent.set(parentId, current);
    } else {
      roots.push(node);
    }
  }
  return {
    id: "root",
    roots: roots.map((node) => node.id),
    edges: prepared.edges.map((edge) => ({
      id: edge.id,
      source: edge.source,
      target: edge.target,
      sourcePortId: edge.attributes?.sourcePortId,
      targetPortId: edge.attributes?.targetPortId,
    })),
    canonicalScene: Boolean(prepared.meta?.canonicalScene),
  };
}
