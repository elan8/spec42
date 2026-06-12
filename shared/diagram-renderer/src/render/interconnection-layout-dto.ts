import type {
  InterconnectionLayoutDto,
  InterconnectionLayoutNodeDto,
  InterconnectionLayoutPortDrawOrder,
} from "../prepare/types";
import { resolveIbdRoutePoints } from "./ibd-route";
import type { LaidOutEdge } from "./types";

export interface InterconnectionLayoutBuildState {
  nodes: Map<string, InterconnectionLayoutNodeDto>;
  diagnostics: string[];
}

export function createInterconnectionLayoutBuildState(): InterconnectionLayoutBuildState {
  return { nodes: new Map(), diagnostics: [] };
}

export function recordInterconnectionLayoutNode(
  state: InterconnectionLayoutBuildState,
  node: {
    id: string;
    x: number;
    y: number;
    width: number;
    height: number;
  },
  portAnchors: Record<string, { x: number; y: number; side: string }>,
  portDrawOrder: InterconnectionLayoutPortDrawOrder,
): void {
  state.nodes.set(node.id, {
    ...node,
    portAnchors,
    portDrawOrder,
  });
}

export function finalizeInterconnectionLayoutDto(
  state: InterconnectionLayoutBuildState,
  edges: LaidOutEdge[],
): InterconnectionLayoutDto {
  return {
    nodes: Array.from(state.nodes.values()),
    edges: edges.map((edge) => ({
      id: edge.id,
      routePoints: resolveIbdRoutePoints(edge) ?? [],
      sourcePortId: String(edge.attributes?.sourcePortId ?? ""),
      targetPortId: String(edge.attributes?.targetPortId ?? ""),
    })),
    diagnostics: [...state.diagnostics],
  };
}

export interface InterconnectionLayoutLookup {
  nodesById: Map<string, InterconnectionLayoutNodeDto>;
  edgesById: Map<string, InterconnectionLayoutDto["edges"][number]>;
}

export function buildInterconnectionLayoutLookup(
  layoutDto: InterconnectionLayoutDto,
): InterconnectionLayoutLookup {
  return {
    nodesById: new Map(layoutDto.nodes.map((node) => [node.id, node])),
    edgesById: new Map(layoutDto.edges.map((edge) => [edge.id, edge])),
  };
}
