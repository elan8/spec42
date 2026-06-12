import * as d3 from "d3";
import { resolveDiagramTheme } from "./theme";
import type { PreparedView } from "./prepare";
import { addActionFlowMarkers, renderActionFlowView } from "./views/action-flow";
import { renderSequenceView, addSequenceMarkers } from "./views/sequence";
import { addStateTransitionMarkers, renderStateTransitionView } from "./views/state-transition";
import { renderBrowserView, renderGeometryView, renderGridView } from "./views/standard-views";
import {
  addMarkers,
  applyFit,
  contentBounds,
  exportSvg,
} from "./render/export";
import {
  drawEdges,
  drawGeneralPackageContainers,
  drawIbdViewFrame,
  drawInterconnectionContainers,
  drawNodes,
  shouldDrawIbdViewFrame,
} from "./render/drawing";
import { layoutPrepared } from "./render/layout";
import { contentBoundsFromExtents, type ContentBounds } from "./render/types";
import type { RenderOptions } from "./render/types";

export type { RenderOptions } from "./render/types";

export interface RenderController {
  reset: () => void;
  exportSvg: () => string;
  destroy: () => void;
  getFitTransform: () => d3.ZoomTransform;
}

import type { PreparedNode } from "./prepare";

export async function renderVisualization(
  target: HTMLElement,
  prepared: PreparedView,
  options: RenderOptions = {},
): Promise<RenderController> {
  target.innerHTML = "";
  const theme = resolveDiagramTheme(options.theme);
  const width = Math.max(720, target.clientWidth || 960);
  const height = Math.max(480, target.clientHeight || 640);
  const svg = d3
    .select(target)
    .append("svg")
    .attr("class", "sysml-viz-svg")
    .attr("width", "100%")
    .attr("height", "100%")
    .attr("viewBox", `0 0 ${width} ${height}`)
    .attr("role", "img")
    .attr("aria-label", prepared.title || "SysML view")
    .style("touch-action", "none")
    .style("cursor", "grab");
  if (theme.colorScheme === "light" || theme.colorScheme === "dark" || theme.colorScheme === "auto") {
    const scheme =
      theme.colorScheme === "auto"
        ? typeof window !== "undefined" && window.matchMedia?.("(prefers-color-scheme: dark)")?.matches
          ? "dark"
          : "light"
        : theme.colorScheme;
    svg.attr("data-color-scheme", scheme);
  }
  svg.append("rect").attr("class", "viz-bg").attr("width", width).attr("height", height);
  svg
    .select(".viz-bg")
    .attr("fill", theme.canvasBackground);
  addMarkers(svg, theme);

  const root = svg.append("g").attr("class", "viz-root");
  const delegateZoom = options.delegateZoom === true;
  const zoom = d3.zoom<SVGSVGElement, unknown>()
    .scaleExtent([0.08, 5])
    .on("start", () => svg.style("cursor", "grabbing"))
    .on("zoom", (event: any) => {
      root.attr("transform", event.transform.toString());
    })
    .on("end", () => svg.style("cursor", "grab"));
  if (!delegateZoom) {
    svg
      .call(zoom)
      .on("dblclick.zoom", null)
      .on("wheel.zoom", function(event: WheelEvent) {
        event.preventDefault();
        event.stopPropagation();
        const mouse = d3.pointer(event, this as SVGSVGElement);
        const currentTransform = d3.zoomTransform(this as SVGSVGElement);
        const factor = event.deltaY > 0 ? 0.7 : 1.45;
        const newScale = Math.min(Math.max(currentTransform.k * factor, 0.08), 5);
        const translateX = mouse[0] - (mouse[0] - currentTransform.x) * (newScale / currentTransform.k);
        const translateY = mouse[1] - (mouse[1] - currentTransform.y) * (newScale / currentTransform.k);
        d3.select(this as SVGSVGElement)
          .transition()
          .duration(50)
          .call(zoom.transform, d3.zoomIdentity.translate(translateX, translateY).scale(newScale));
      });
  }

  const view = prepared.view;
  const isInterconnectionView = view === "interconnection-view";
  const isBehaviorView =
    view === "action-flow-view" ||
    view === "state-transition-view" ||
    view === "sequence-view" ||
    view === "browser-view" ||
    view === "grid-view" ||
    view === "geometry-view";

  let bounds: ContentBounds;
  if (view === "action-flow-view") {
    addActionFlowMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    bounds = contentBoundsFromExtents(await renderActionFlowView({ root, prepared, theme, width, height, options }));
  } else if (view === "state-transition-view") {
    addStateTransitionMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    bounds = contentBoundsFromExtents(await renderStateTransitionView({ root, prepared, theme, width, height, options }));
  } else if (view === "sequence-view") {
    addSequenceMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    bounds = contentBoundsFromExtents(renderSequenceView({ root, prepared, theme, width, height, options }));
  } else if (view === "browser-view") {
    bounds = contentBoundsFromExtents(renderBrowserView({ root, prepared, theme, width, height, options }));
  } else if (view === "grid-view") {
    bounds = contentBoundsFromExtents(renderGridView({ root, prepared, theme, width, height, options }));
  } else if (view === "geometry-view") {
    bounds = contentBoundsFromExtents(renderGeometryView({ root, prepared, theme, width, height, options }));
  } else {
    const layout = await layoutPrepared(prepared);
    if (isInterconnectionView) {
      if (shouldDrawIbdViewFrame(prepared)) {
        drawIbdViewFrame(root, prepared, contentBounds(layout), theme);
      }
      drawInterconnectionContainers(root, prepared, layout.nodes, theme, layout.interconnectionLayout);
      drawNodes(root, layout.nodes, options, isInterconnectionView, theme, layout.interconnectionLayout);
      drawEdges(root, layout.edges, isInterconnectionView, theme, layout.interconnectionLayout);
    } else {
      drawGeneralPackageContainers(root, prepared, layout.nodes, theme);
      drawEdges(root, layout.edges, isInterconnectionView, theme);
      drawNodes(root, layout.nodes, options, isInterconnectionView, theme);
    }
    bounds = contentBounds(layout);
  }

  let lastFitTransform = d3.zoomIdentity;
  const fitView = () => {
    lastFitTransform = applyFit(
      svg,
      zoom,
      root,
      bounds,
      width,
      height,
      isInterconnectionView || isBehaviorView,
      delegateZoom,
    );
  };
  fitView();

  return {
    reset: () => fitView(),
    getFitTransform: () => lastFitTransform,
    exportSvg: () => exportSvg(svg.node() as SVGSVGElement, bounds),
    destroy: () => {
      target.innerHTML = "";
    },
  };
}