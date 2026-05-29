import type { Selection } from "d3";
import ELK from "elkjs/lib/elk.bundled.js";
import type { PreparedNode, PreparedView } from "../prepare";
import type { DiagramTheme } from "../theme";

export interface EdgeSection {
  startPoint?: { x: number; y: number };
  bendPoints?: Array<{ x: number; y: number }>;
  endPoint?: { x: number; y: number };
}

export const behaviorElk = new ELK();

export interface LaidOutRect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface BehaviorLayoutResult {
  positions: Map<string, LaidOutRect>;
  edgeSectionsById: Map<string, EdgeSection[]>;
}

export interface BehaviorSceneContext {
  root: Selection<SVGGElement, unknown, null, undefined>;
  prepared: PreparedView;
  theme: DiagramTheme;
  width: number;
  height: number;
}

export function nodeKind(node: PreparedNode): string {
  return String(node.kind || "action").toLowerCase();
}

export function pathFromSections(sections: EdgeSection[] | undefined): string | null {
  if (!sections?.length) return null;
  const parts: string[] = [];
  for (const section of sections) {
    if (!section.startPoint || !section.endPoint) continue;
    parts.push(`M${section.startPoint.x},${section.startPoint.y}`);
    for (const point of section.bendPoints ?? []) {
      parts.push(`L${point.x},${point.y}`);
    }
    parts.push(`L${section.endPoint.x},${section.endPoint.y}`);
  }
  return parts.length ? parts.join(" ") : null;
}

export function fallbackEdgePath(
  source: LaidOutRect,
  target: LaidOutRect,
  horizontal: boolean,
): { path: string; labelX: number; labelY: number } {
  if (horizontal) {
    const startX = source.x + source.width;
    const startY = source.y + source.height / 2;
    const endX = target.x;
    const endY = target.y + target.height / 2;
    const midX = (startX + endX) / 2;
    return {
      path: `M${startX},${startY} L${midX},${startY} L${midX},${endY} L${endX},${endY}`,
      labelX: midX,
      labelY: (startY + endY) / 2 - 6,
    };
  }
  const startX = source.x + source.width / 2;
  const startY = source.y + source.height;
  const endX = target.x + target.width / 2;
  const endY = target.y;
  const midY = (startY + endY) / 2;
  return {
    path: `M${startX},${startY} L${startX},${midY} L${endX},${midY} L${endX},${endY}`,
    labelX: (startX + endX) / 2,
    labelY: midY - 6,
  };
}

function nodeDimensions(node: PreparedNode, mode: "action" | "state"): { width: number; height: number } {
  const kind = nodeKind(node);
  if (mode === "state") {
    if (kind.includes("initial") || kind.includes("final")) return { width: 34, height: 34 };
    if (kind.includes("composite")) return { width: 340, height: 320 };
    return { width: 240, height: 180 };
  }
  if (kind.includes("initial") || kind.includes("final") || kind.includes("start") || kind.includes("done")) {
    return { width: 40, height: 40 };
  }
  if (kind.includes("decision") || kind.includes("merge")) return { width: 76, height: 76 };
  if (kind.includes("fork") || kind.includes("join")) return { width: 220, height: 14 };
  return { width: 220, height: 68 };
}

export async function layoutBehaviorGraph(
  prepared: PreparedView,
  options: { horizontal?: boolean; mode: "action" | "state" },
): Promise<BehaviorLayoutResult> {
  const horizontal = options.horizontal ?? false;
  const positions = new Map<string, LaidOutRect>();
  const edgeSectionsById = new Map<string, EdgeSection[]>();

  const children = prepared.nodes.map((node) => {
    const size = nodeDimensions(node, options.mode);
    return { id: node.id, width: size.width, height: size.height };
  });

  const edges = prepared.edges.map((edge) => ({
    id: edge.id,
    sources: [edge.source],
    targets: [edge.target],
  }));

  const graph = {
    id: prepared.title || "behavior",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.direction": horizontal ? "RIGHT" : "DOWN",
      "elk.edgeRouting": "ORTHOGONAL",
      "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
      "elk.spacing.nodeNode": horizontal ? "90" : "120",
      "elk.layered.spacing.nodeNodeBetweenLayers": horizontal ? "190" : "170",
      "elk.padding": "[top=80,left=80,bottom=80,right=80]",
    },
    children,
    edges,
  };

  const laidOut = await behaviorElk.layout(graph);
  for (const child of laidOut.children ?? []) {
    positions.set(String(child.id), {
      x: child.x ?? 0,
      y: child.y ?? 0,
      width: child.width ?? 200,
      height: child.height ?? 80,
    });
  }
  for (const edge of laidOut.edges ?? []) {
    if (edge.sections) {
      edgeSectionsById.set(String(edge.id), edge.sections as EdgeSection[]);
    }
  }

  return { positions, edgeSectionsById };
}

export function truncateLabel(text: string, max: number): string {
  const trimmed = text.trim();
  return trimmed.length > max ? `${trimmed.slice(0, max - 2)}..` : trimmed;
}
