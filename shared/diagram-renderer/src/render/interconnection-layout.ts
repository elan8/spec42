import type { InterconnectionLayoutDto, PreparedView } from "../prepare/types";
import { resolveIbdRoutePoints } from "./ibd-route";
import { buildInterconnectionElkGraph, layoutInterconnectionPrepared } from "./layout";
import type { LayoutResult } from "./types";

export { buildInterconnectionElkGraph };

export async function layoutInterconnectionScene(
  prepared: PreparedView,
): Promise<{ layout: LayoutResult; layoutDto: InterconnectionLayoutDto }> {
  const layout = await layoutInterconnectionPrepared(prepared);
  const layoutDto: InterconnectionLayoutDto = {
    nodes: layout.nodes.map((node) => ({
      id: node.id,
      x: node.x ?? 0,
      y: node.y ?? 0,
      width: node.width ?? 0,
      height: node.height ?? 0,
      portAnchors:
        ((node.attributes ?? {})._portAnchors as Record<string, { x: number; y: number; side: string }>) ??
        {},
    })),
    edges: layout.edges.map((edge) => ({
      id: edge.id,
      routePoints: resolveIbdRoutePoints(edge) ?? [],
      sourcePortId: String(edge.attributes?.sourcePortId ?? ""),
      targetPortId: String(edge.attributes?.targetPortId ?? ""),
    })),
    diagnostics: [],
  };
  return { layout, layoutDto };
}
