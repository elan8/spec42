import type { Selection } from "d3";
import ELK from "elkjs/lib/elk.bundled.js";
import type { PreparedNode, PreparedView } from "../prepare";
import type { RenderOptions } from "../renderer";
import type { DiagramTheme } from "../theme";
import { collectElkEdgeLabels, edgeLabelPositionFromSections, estimateElkLabelBox, type ElkLabelBox } from "./elk-label-utils";

export { edgeLabelPositionFromSections };

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
  edgeLabelsById: Map<string, ElkLabelBox[]>;
}

export interface BehaviorSceneContext {
  root: Selection<SVGGElement, unknown, null, undefined>;
  prepared: PreparedView;
  theme: DiagramTheme;
  width: number;
  height: number;
  options?: RenderOptions;
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

export function buildSelfLoopPath(node: LaidOutRect): { path: string; labelX: number; labelY: number } {
  const startX = node.x + node.width;
  const startY = node.y + node.height / 2 - 8;
  const loopRadius = 28;
  return {
    path:
      `M${startX},${startY}` +
      ` C${startX + loopRadius},${startY - loopRadius}` +
      ` ${startX + loopRadius},${startY + loopRadius}` +
      ` ${startX},${startY + 18}`,
    labelX: startX + loopRadius + 8,
    labelY: startY,
  };
}

export function fallbackEdgePath(
  source: LaidOutRect,
  target: LaidOutRect,
  horizontal: boolean,
): { path: string; labelX: number; labelY: number } {
  if (
    source.x === target.x &&
    source.y === target.y &&
    source.width === target.width &&
    source.height === target.height
  ) {
    return buildSelfLoopPath(source);
  }
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

function transitionDisplayLabel(label: string): string {
  const trimmed = label.trim();
  if (!trimmed || trimmed.toLowerCase() === "entry") return "";
  return trimmed;
}

export async function layoutBehaviorGraph(
  prepared: PreparedView,
  options: { horizontal?: boolean; mode: "action" | "state" },
): Promise<BehaviorLayoutResult> {
  const horizontal = options.horizontal ?? false;
  const positions = new Map<string, LaidOutRect>();
  const edgeSectionsById = new Map<string, EdgeSection[]>();
  const edgeLabelsById = new Map<string, ElkLabelBox[]>();

  const children = prepared.nodes.map((node) => {
    const size = nodeDimensions(node, options.mode);
    return { id: node.id, width: size.width, height: size.height };
  });

  const edges = prepared.edges.map((edge) => {
    const displayLabel = transitionDisplayLabel(edge.label);
    const base = {
      id: edge.id,
      sources: [edge.source],
      targets: [edge.target],
    };
    if (!displayLabel) {
      return base;
    }
    const labelBox = estimateElkLabelBox(`${edge.id}::label`, displayLabel, {
      minWidth: 38,
      minHeight: 16,
      paddingX: 8,
      paddingY: 6,
      charWidth: 6,
    });
    return {
      ...base,
      labels: [
        {
          id: labelBox.id,
          text: labelBox.text,
          width: labelBox.width,
          height: labelBox.height,
          layoutOptions: {
            "org.eclipse.elk.edgeLabels.placement": "CENTER",
            "org.eclipse.elk.edgeLabels.inline": "false",
          },
        },
      ],
    };
  });

  const isState = options.mode === "state";
  const graph = {
    id: prepared.title || "behavior",
    layoutOptions: isState
      ? {
          "elk.algorithm": "layered",
          "elk.direction": horizontal ? "RIGHT" : "DOWN",
          "elk.hierarchyHandling": "INCLUDE_CHILDREN",
          "elk.edgeRouting": "ORTHOGONAL",
          "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
          "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
          "elk.layered.spacing.nodeNodeBetweenLayers": "230",
          "elk.spacing.nodeNode": "190",
          "elk.spacing.edgeNode": "130",
          "elk.spacing.edgeEdge": "110",
          "elk.spacing.edgeLabel": "12",
          "elk.padding": "[top=100,left=90,bottom=90,right=90]",
          "elk.separateConnectedComponents": "true",
          "elk.json.edgeCoords": "ROOT",
        }
      : {
          "elk.algorithm": "layered",
          "elk.direction": horizontal ? "RIGHT" : "DOWN",
          "elk.edgeRouting": "ORTHOGONAL",
          "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
          "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
          "elk.spacing.nodeNode": horizontal ? "90" : "120",
          "elk.layered.spacing.nodeNodeBetweenLayers": horizontal ? "190" : "170",
          "elk.spacing.edgeNode": "80",
          "elk.spacing.edgeEdge": "60",
          "elk.spacing.edgeLabel": "12",
          "elk.padding": "[top=80,left=80,bottom=80,right=80]",
          "elk.separateConnectedComponents": "true",
          "elk.json.edgeCoords": "ROOT",
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
  collectElkEdgeLabels(laidOut, { x: 0, y: 0 }, edgeLabelsById);

  return { positions, edgeSectionsById, edgeLabelsById };
}

export function truncateLabel(text: string, max: number): string {
  const trimmed = text.trim();
  return trimmed.length > max ? `${trimmed.slice(0, max - 2)}..` : trimmed;
}
