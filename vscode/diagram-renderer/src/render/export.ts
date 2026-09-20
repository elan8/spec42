import * as d3 from "d3";
import type { DiagramTheme } from "../theme";
import type { ContentBounds } from "./types";

export function applyFit(
  svg: d3.Selection<SVGSVGElement, unknown, null, undefined>,
  zoom: d3.ZoomBehavior<SVGSVGElement, unknown>,
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  bounds: ContentBounds,
  width: number,
  height: number,
  isDenseView = false,
  delegateZoom = false,
): d3.ZoomTransform {
  const padding = 48;
  const minScale = isDenseView ? 0.2 : 0.08;
  const maxScale = isDenseView ? 1.1 : 1.3;
  const scale = Math.min(
    maxScale,
    Math.max(minScale, Math.min((width - padding * 2) / bounds.width, (height - padding * 2) / bounds.height)),
  );
  const tx = (width - bounds.width * scale) / 2 - bounds.x * scale;
  const ty = (height - bounds.height * scale) / 2 - bounds.y * scale;
  const transform = d3.zoomIdentity.translate(tx, ty).scale(scale);
  if (delegateZoom) {
    root.attr("transform", transform.toString());
    return transform;
  }
  svg.transition().duration(180).call(zoom.transform, transform);
  return transform;
}

export function addMarkers(svg: d3.Selection<SVGSVGElement, unknown, null, undefined>, theme: DiagramTheme): void {
  const defs = svg.append("defs");
  defs.append("marker").attr("id", "viz-arrow").attr("markerWidth", 10).attr("markerHeight", 10).attr("refX", 9).attr("refY", 3).attr("orient", "auto").attr("markerUnits", "strokeWidth").append("path").attr("d", "M0,0 L0,6 L9,3 z").attr("fill", theme.edge.default);
}

export function exportSvg(svgNode: SVGSVGElement, bounds: ContentBounds): string {
  const clone = svgNode.cloneNode(true) as SVGSVGElement;
  clone.querySelector<SVGGElement>(".viz-root")?.removeAttribute("transform");
  clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  clone.setAttribute("viewBox", `${bounds.x - 40} ${bounds.y - 40} ${bounds.width + 80} ${bounds.height + 80}`);
  return new XMLSerializer().serializeToString(clone);
}

export function contentBoundsFromViewBox(svg: SVGSVGElement): ContentBounds {
  const viewBox = svg.viewBox?.baseVal;
  if (viewBox && viewBox.width > 0 && viewBox.height > 0) {
    return { x: viewBox.x, y: viewBox.y, width: viewBox.width, height: viewBox.height };
  }
  return { x: 0, y: 0, width: 100, height: 100 };
}
