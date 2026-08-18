import type { LaidOutEdge, LaidOutNode } from "./types";
import { resolveIbdRoutePoints } from "./ibd-route";

export interface RoutePoint {
  x: number;
  y: number;
}

export interface ContentBounds {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export interface RouteQualityOptions {
  boundsMargin?: number;
  maxLengthRatio?: number;
  maxDetachedDistance?: number;
}

export interface EdgeRouteQuality {
  edgeId: string;
  pointCount: number;
  routeLength: number;
  manhattanDistance: number;
  lengthRatio: number;
  detachedSource: boolean;
  detachedTarget: boolean;
  outsideBounds: boolean;
  usedNodeBoundaryFallback: boolean;
  points: RoutePoint[];
}

export interface RouteQualityReport {
  edges: EdgeRouteQuality[];
  violations: string[];
  passed: boolean;
}

export function polylineLength(points: RoutePoint[]): number {
  let length = 0;
  for (let index = 1; index < points.length; index += 1) {
    const previous = points[index - 1];
    const current = points[index];
    length += Math.hypot(current.x - previous.x, current.y - previous.y);
  }
  return length;
}

export function manhattanDistance(a: RoutePoint, b: RoutePoint): number {
  return Math.abs(a.x - b.x) + Math.abs(a.y - b.y);
}

export function contentBoundsFromNodes(nodes: LaidOutNode[], margin = 0): ContentBounds | null {
  if (!nodes.length) return null;
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  for (const node of nodes) {
    const x = node.x ?? 0;
    const y = node.y ?? 0;
    const width = node.width ?? 0;
    const height = node.height ?? 0;
    minX = Math.min(minX, x);
    minY = Math.min(minY, y);
    maxX = Math.max(maxX, x + width);
    maxY = Math.max(maxY, y + height);
  }
  return {
    minX: minX - margin,
    minY: minY - margin,
    maxX: maxX + margin,
    maxY: maxY + margin,
  };
}

export function routePointsOutsideBounds(
  points: RoutePoint[],
  bounds: ContentBounds,
): boolean {
  return points.some(
    (point) =>
      point.x < bounds.minX ||
      point.x > bounds.maxX ||
      point.y < bounds.minY ||
      point.y > bounds.maxY,
  );
}

export function endpointDetached(
  routePoint: RoutePoint | undefined,
  portCenter: RoutePoint | undefined,
  maxDistance: number,
): boolean {
  if (!routePoint || !portCenter) return true;
  return Math.hypot(routePoint.x - portCenter.x, routePoint.y - portCenter.y) > maxDistance;
}

export function assessEdgeRouteQuality(
  edge: LaidOutEdge,
  bounds: ContentBounds | null,
  options: RouteQualityOptions = {},
): EdgeRouteQuality {
  const boundsMargin = options.boundsMargin ?? 120;
  const maxDetachedDistance = options.maxDetachedDistance ?? 24;
  const attrs = (edge.attributes ?? {}) as Record<string, unknown>;
  const sourcePort = (attrs._sourcePortCenter ?? null) as RoutePoint | null;
  const targetPort = (attrs._targetPortCenter ?? null) as RoutePoint | null;
  const sourcePortId = String(attrs.sourcePortId ?? "");
  const targetPortId = String(attrs.targetPortId ?? "");
  const points = resolveIbdRoutePoints(edge) ?? [];
  const routeLength = polylineLength(points);
  const portDistance =
    sourcePort && targetPort ? manhattanDistance(sourcePort, targetPort) : 0;
  const lengthRatio = portDistance > 0 ? routeLength / portDistance : routeLength > 0 ? Number.POSITIVE_INFINITY : 0;
  const expandedBounds = bounds
    ? {
        minX: bounds.minX - boundsMargin,
        minY: bounds.minY - boundsMargin,
        maxX: bounds.maxX + boundsMargin,
        maxY: bounds.maxY + boundsMargin,
      }
    : null;
  const usedNodeBoundaryFallback = Boolean(
    (sourcePortId || targetPortId) && (!sourcePort || !targetPort),
  );
  return {
    edgeId: edge.id,
    pointCount: points.length,
    routeLength,
    manhattanDistance: portDistance,
    lengthRatio,
    detachedSource: endpointDetached(points[0], sourcePort ?? undefined, maxDetachedDistance),
    detachedTarget: endpointDetached(points[points.length - 1], targetPort ?? undefined, maxDetachedDistance),
    outsideBounds: expandedBounds ? routePointsOutsideBounds(points, expandedBounds) : false,
    usedNodeBoundaryFallback,
    points,
  };
}

export function assessRouteQuality(
  edges: LaidOutEdge[],
  nodes: LaidOutNode[],
  options: RouteQualityOptions = {},
): RouteQualityReport {
  const maxLengthRatio = options.maxLengthRatio ?? 4;
  const bounds = contentBoundsFromNodes(nodes);
  const edgeReports = edges.map((edge) => assessEdgeRouteQuality(edge, bounds, options));
  const violations: string[] = [];
  for (const report of edgeReports) {
    if (report.pointCount < 2) {
      violations.push(`${report.edgeId}: missing route points`);
    }
    if (report.usedNodeBoundaryFallback) {
      violations.push(`${report.edgeId}: node-boundary fallback with explicit port ids`);
    }
    if (report.detachedSource || report.detachedTarget) {
      violations.push(`${report.edgeId}: detached route endpoint`);
    }
    if (report.outsideBounds) {
      violations.push(`${report.edgeId}: route outside content bounds`);
    }
    if (Number.isFinite(report.lengthRatio) && report.lengthRatio > maxLengthRatio) {
      violations.push(`${report.edgeId}: route length ratio ${report.lengthRatio.toFixed(2)} > ${maxLengthRatio}`);
    }
  }
  return {
    edges: edgeReports,
    violations,
    passed: violations.length === 0,
  };
}

export function detachedEndpointViolations(report: RouteQualityReport): string[] {
  return report.edges
    .filter((edge) => edge.detachedSource || edge.detachedTarget)
    .map((edge) => `${edge.edgeId}: detached route endpoint`);
}

export function outOfBoundsViolations(report: RouteQualityReport): string[] {
  return report.edges
    .filter((edge) => edge.outsideBounds)
    .map((edge) => `${edge.edgeId}: route outside content bounds`);
}

/** Returns detached-endpoint violations; empty when all endpoints snap to ports. */
export const assertNoDetachedEndpoints = detachedEndpointViolations;

/** Returns out-of-bounds violations; empty when routes stay inside the layout bounds. */
export const assertWithinBounds = outOfBoundsViolations;

export function summarizeRoutes(edges: LaidOutEdge[], nodes: LaidOutNode[]): Record<string, unknown> {
  const report = assessRouteQuality(edges, nodes);
  return {
    passed: report.passed,
    violationCount: report.violations.length,
    violations: report.violations,
    edges: report.edges.map((edge) => ({
      edgeId: edge.edgeId,
      pointCount: edge.pointCount,
      routeLength: Number(edge.routeLength.toFixed(2)),
      manhattanDistance: Number(edge.manhattanDistance.toFixed(2)),
      lengthRatio: Number.isFinite(edge.lengthRatio) ? Number(edge.lengthRatio.toFixed(2)) : null,
      detachedSource: edge.detachedSource,
      detachedTarget: edge.detachedTarget,
      outsideBounds: edge.outsideBounds,
      usedNodeBoundaryFallback: edge.usedNodeBoundaryFallback,
    })),
  };
}
