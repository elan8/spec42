import * as d3 from "d3";
import type { DiagramTheme } from "../theme";
import type { ContentBounds, LayoutResult } from "./types";
import { nodeHeight, nodeWidth } from "./types";

export function contentBounds(layout: LayoutResult): ContentBounds {
  if (!layout.nodes.length) return { x: 0, y: 0, width: 100, height: 100 };
  const minX = Math.min(...layout.nodes.map((node) => node.x || 0));
  const minY = Math.min(...layout.nodes.map((node) => node.y || 0));
  const maxX = Math.max(...layout.nodes.map((node) => (node.x || 0) + (node.width || nodeWidth)));
  const maxY = Math.max(...layout.nodes.map((node) => (node.y || 0) + (node.height || nodeHeight)));
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

export function applyFit(
  svg: d3.Selection<SVGSVGElement, unknown, null, undefined>,
  zoom: d3.ZoomBehavior<SVGSVGElement, unknown>,
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  bounds: ContentBounds,
  width: number,
  height: number,
  isInterconnectionView = false,
  delegateZoom = false,
): d3.ZoomTransform {
  const padding = 48;
  const minScale = isInterconnectionView ? 0.2 : 0.08;
  const maxScale = isInterconnectionView ? 1.1 : 1.3;
  const scale = Math.min(
    maxScale,
    Math.max(minScale, Math.min((width - padding * 2) / bounds.width, (height - padding * 2) / bounds.height)),
  );
  const tx = (width - bounds.width * scale) / 2 - bounds.x * scale;
  const ty = (height - bounds.height * scale) / 2 - bounds.y * scale;
  const transform = d3.zoomIdentity.translate(tx, ty).scale(scale);
  if (delegateZoom) {
    // Host applies this via d3.zoom; keep attr in sync for first paint before host wiring.
    root.attr("transform", transform.toString());
    return transform;
  }
  svg.transition().duration(180).call(zoom.transform, transform);
  return transform;
}

export function addMarkers(svg: d3.Selection<SVGSVGElement, unknown, null, undefined>, theme: DiagramTheme): void {
  const defs = svg.append("defs");
  defs.append("marker").attr("id", "viz-arrow").attr("markerWidth", 10).attr("markerHeight", 10).attr("refX", 9).attr("refY", 3).attr("orient", "auto").attr("markerUnits", "strokeWidth").append("path").attr("d", "M0,0 L0,6 L9,3 z").attr("fill", theme.edge.default);
  defs.append("marker").attr("id", "general-d3-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 8).attr("refY", 0).attr("markerWidth", 5).attr("markerHeight", 5).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4").style("fill", theme.edge.default);
  defs.append("marker").attr("id", "general-d3-arrow-open").attr("viewBox", "0 -5 10 10").attr("refX", 9).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4").style("fill", "none").style("stroke", theme.edge.default).style("stroke-width", "1.3");
  defs.append("marker").attr("id", "general-d3-specializes").attr("viewBox", "0 -6 12 12").attr("refX", 11).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,0L10,-4L10,4Z").style("fill", theme.nodeFill).style("stroke", theme.edge.default).style("stroke-width", "1.2");
  defs.append("marker").attr("id", "general-d3-diamond").attr("viewBox", "0 -6 12 12").attr("refX", 2).attr("refY", 0).attr("markerWidth", 7).attr("markerHeight", 7).attr("orient", "auto").append("path").attr("d", "M0,0L5,-4L10,0L5,4Z").style("fill", theme.edge.default);
  defs.append("marker").attr("id", "ibd-connection-dot").attr("viewBox", "-5 -5 10 10").attr("refX", 0).attr("refY", 0).attr("markerWidth", 5).attr("markerHeight", 5).attr("orient", "auto").append("circle").attr("r", 3).style("fill", theme.nodeFill).style("stroke", theme.edge.default).style("stroke-width", "1.5");
  defs.append("marker").attr("id", "ibd-flow-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 10).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4Z").style("fill", theme.edge.default);
  defs.append("marker").attr("id", "ibd-interface-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 10).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4Z").style("fill", "none").style("stroke", theme.edge.default).style("stroke-width", "1.5");
}

export function exportSvg(svgNode: SVGSVGElement, bounds: ContentBounds): string {
  const clone = svgNode.cloneNode(true) as SVGSVGElement;
  clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  clone.setAttribute("viewBox", `${bounds.x - 40} ${bounds.y - 40} ${bounds.width + 80} ${bounds.height + 80}`);
  return new XMLSerializer().serializeToString(clone);
}