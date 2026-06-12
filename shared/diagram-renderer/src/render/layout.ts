import ELK from "elkjs/lib/elk.bundled.js";
import { isOverviewVisualElementType, normalizeEdgeKind } from "../graph-normalization";
import { collectCompartments, computeNodeHeight } from "../sysml-node-builder";
import { interconnectionPreparedForLayout, type PreparedNode, type PreparedView } from "../prepare";
import { lcaOffsetForNodes } from "./ibd-route";
import {
  createInterconnectionLayoutBuildState,
  finalizeInterconnectionLayoutDto,
  recordInterconnectionLayoutContainer,
  recordInterconnectionLayoutNode,
} from "./interconnection-layout-dto";
import { buildInterconnectionElkBuild } from "./interconnection-elk-input";
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

export async function layoutPrepared(prepared: PreparedView): Promise<LayoutResult> {
  if (!prepared.nodes.length) return { nodes: [], edges: [] };
  const isInterconnectionView = prepared.view === "interconnection-view";
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
  if (isInterconnectionView) {
    return layoutInterconnectionPrepared(prepared);
  }
  const diagramNodes = prepared.nodes.filter((node) => isOverviewVisualElementType(node.kind));
  const visibleIds = new Set(diagramNodes.map((node) => node.id));
  const diagramEdges = prepared.edges.filter(
    (edge) => visibleIds.has(edge.source) && visibleIds.has(edge.target),
  );
  if (!diagramNodes.length) return { nodes: [], edges: [] };
  const width = isInterconnectionView ? ibdNodeWidth : nodeWidth;
  const height = isInterconnectionView ? ibdNodeHeight : nodeHeight;
  const graph = {
    id: "root",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.direction": isInterconnectionView ? "RIGHT" : "DOWN",
      "elk.spacing.nodeNode": isInterconnectionView ? "80" : "220",
      "elk.layered.spacing.nodeNodeBetweenLayers": isInterconnectionView ? "110" : "280",
      "elk.spacing.edgeNode": isInterconnectionView ? "80" : "120",
      "elk.spacing.edgeEdge": isInterconnectionView ? "60" : "120",
      "elk.edgeRouting": "ORTHOGONAL",
      "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
      "elk.separateConnectedComponents": "true",
      "elk.aspectRatio": isInterconnectionView ? "1.6" : "1.4",
      "elk.padding": isInterconnectionView ? "[top=70,left=70,bottom=70,right=70]" : "[top=100,left=100,bottom=100,right=100]",
      "org.eclipse.elk.portConstraints": "FIXED_SIDE",
      "org.eclipse.elk.json.edgeCoords": "ROOT"
    },
    children: diagramNodes.map((node) => {
      const compartments = collectCompartments(node);
      return {
        id: node.id,
        width,
        height: Math.max(height, computeNodeHeight(compartments, { maxLinesPerCompartment: 8 })),
      };
    }),
    edges: diagramEdges.map((edge) => ({ id: edge.id, sources: [edge.source], targets: [edge.target] }))
  };
  try {
    const laidOut = await elk.layout(graph);
    const byId = new Map(diagramNodes.map((node) => [node.id, node]));
    const layouts = new Map((laidOut.children || []).map((node: any) => [String(node.id), node]));
    return {
      nodes: diagramNodes.map((node) => {
        const compartments = collectCompartments(node);
        return { ...node, compartments, ...(layouts.get(node.id) || {}) };
      }),
      edges: diagramEdges.map((edge) => ({
        ...edge,
        sourceNode: byId.get(edge.source),
        targetNode: byId.get(edge.target),
        layout: (laidOut.edges || []).find((item: any) => item.id === edge.id) as LaidOutEdge["layout"]
      }))
    };
  } catch {
    // Match interconnection policy: no heuristic grid when ELK fails.
    return { nodes: [], edges: [] };
  }
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
    const nodePortAnchors = new Map<string, Record<string, { x: number; y: number; side: string }>>();

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
          anchors[portName] = { x: x - absX, y: y - absY, side: String(side || "") };
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
