import type { EdgeSection, LaidOutEdge, LaidOutNode } from "./types";

export function pruneRoutePoints(points: Array<{ x: number; y: number }>): Array<{ x: number; y: number }> {
  const pruned: Array<{ x: number; y: number }> = [];
  for (const point of points) {
    const last = pruned[pruned.length - 1];
    if (last && Math.abs(last.x - point.x) < 1e-6 && Math.abs(last.y - point.y) < 1e-6) {
      continue;
    }
    pruned.push({ x: point.x, y: point.y });
    while (pruned.length >= 3) {
      const a = pruned[pruned.length - 3];
      const b = pruned[pruned.length - 2];
      const c = pruned[pruned.length - 1];
      const sameX = Math.abs(a.x - b.x) < 1e-6 && Math.abs(b.x - c.x) < 1e-6;
      const sameY = Math.abs(a.y - b.y) < 1e-6 && Math.abs(b.y - c.y) < 1e-6;
      if (!sameX && !sameY) break;
      pruned.splice(pruned.length - 2, 1);
    }
  }
  return pruned;
}

export function pointsFromElkSections(
  sections: EdgeSection[],
  offset: { x: number; y: number },
): Array<{ x: number; y: number }> {
  const points: Array<{ x: number; y: number }> = [];
  for (const section of sections) {
    if (section.startPoint) {
      points.push({ x: section.startPoint.x + offset.x, y: section.startPoint.y + offset.y });
    }
    for (const bend of section.bendPoints ?? []) {
      points.push({ x: bend.x + offset.x, y: bend.y + offset.y });
    }
    if (section.endPoint) {
      points.push({ x: section.endPoint.x + offset.x, y: section.endPoint.y + offset.y });
    }
  }
  return pruneRoutePoints(points);
}

function containerChain(node: LaidOutNode, nodesById: Map<string, LaidOutNode>): string[] {
  const chain: string[] = [];
  let current: LaidOutNode | undefined = node;
  while (current) {
    chain.push(current.id);
    const parentId: string = String((current.attributes as Record<string, unknown> | undefined)?.containerId ?? "");
    const parentNode = parentId && nodesById.has(parentId) ? nodesById.get(parentId) : undefined;
    current = parentNode;
  }
  return chain;
}

export function lcaOffsetForNodes(
  sourceNode: LaidOutNode,
  targetNode: LaidOutNode,
  laidOutNodes: Map<string, LaidOutNode>,
): { x: number; y: number } {
  const sourceChain = containerChain(sourceNode, laidOutNodes);
  const targetSet = new Set(containerChain(targetNode, laidOutNodes));
  const lcaId = sourceChain.find((id) => targetSet.has(id));
  if (!lcaId) return { x: 0, y: 0 };
  const lca = laidOutNodes.get(lcaId);
  return lca ? { x: lca.x ?? 0, y: lca.y ?? 0 } : { x: 0, y: 0 };
}

export function uniqueOffsets(offsets: Array<{ x: number; y: number }>): Array<{ x: number; y: number }> {
  const seen = new Set<string>();
  const unique: Array<{ x: number; y: number }> = [];
  for (const offset of offsets) {
    const key = `${offset.x.toFixed(3)},${offset.y.toFixed(3)}`;
    if (seen.has(key)) continue;
    seen.add(key);
    unique.push(offset);
  }
  return unique;
}

export function routeEndpointError(
  points: Array<{ x: number; y: number }>,
  source: { x: number; y: number },
  target: { x: number; y: number },
): number {
  if (points.length < 2) return Number.POSITIVE_INFINITY;
  const start = points[0];
  const end = points[points.length - 1];
  return Math.hypot(start.x - source.x, start.y - source.y) + Math.hypot(end.x - target.x, end.y - target.y);
}

export function snapRouteEndpoints(
  points: Array<{ x: number; y: number }>,
  source?: { x: number; y: number } | null,
  target?: { x: number; y: number } | null,
): Array<{ x: number; y: number }> {
  if (points.length < 2) return points;
  const route = points.map((point) => ({ x: point.x, y: point.y }));
  if (source) route[0] = { x: source.x, y: source.y };
  if (target) route[route.length - 1] = { x: target.x, y: target.y };
  return pruneRoutePoints(route);
}

export function resolveIbdRoutePoints(edge: LaidOutEdge): Array<{ x: number; y: number }> | null {
  const sections = edge.layout?.sections;
  if (!sections?.length) return null;
  const sourceNode = edge.sourceNode;
  const targetNode = edge.targetNode;
  if (!sourceNode || !targetNode) return null;

  const attrs = (edge.attributes ?? {}) as Record<string, unknown>;
  const sourcePort = (attrs._sourcePortCenter ?? null) as { x: number; y: number } | null;
  const targetPort = (attrs._targetPortCenter ?? null) as { x: number; y: number } | null;
  const lcaOffset = edge.layout?.lcaOffset ?? { x: 0, y: 0 };
  const edgeOwnerOffset = edge.layout?.edgeOwnerOffset ?? { x: 0, y: 0 };
  const candidates = uniqueOffsets([{ x: 0, y: 0 }, edgeOwnerOffset, lcaOffset]);

  let bestPoints: Array<{ x: number; y: number }> | null = null;
  let bestError = Number.POSITIVE_INFINITY;
  for (const offset of candidates) {
    const points = pointsFromElkSections(sections, offset);
    if (points.length < 2) continue;
    const error = sourcePort && targetPort ? routeEndpointError(points, sourcePort, targetPort) : 0;
    if (error < bestError) {
      bestError = error;
      bestPoints = points;
    }
  }

  if (!bestPoints) return null;
  return snapRouteEndpoints(bestPoints, sourcePort, targetPort);
}
