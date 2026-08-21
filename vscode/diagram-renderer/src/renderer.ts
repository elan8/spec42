import * as d3 from "d3";
import { resolveDiagramTheme } from "./theme";
import type { PreparedView } from "./prepare";
import { addActionFlowMarkers, renderActionFlowView } from "./views/action-flow";
import { renderSequenceView, addSequenceMarkers } from "./views/sequence";
import { addStateTransitionMarkers, renderStateTransitionView } from "./views/state-transition";
import { renderBrowserView, renderGeometryView, renderGridView } from "./views/standard-views-render";
import {
  addMarkers,
  applyFit,
  contentBounds,
  exportSvg,
} from "./render/export";
import {
  drawEdges,
  drawGeneralPackageContainers,
  drawInterconnectionPortOverlays,
  drawIbdViewFrame,
  drawInterconnectionContainers,
  drawNodes,
  shouldDrawIbdViewFrame,
} from "./render/drawing";
import { layoutPrepared } from "./render/layout";
import { contentBoundsFromExtents, type ContentBounds } from "./render/types";
import type { RenderOptions } from "./render/types";
import { installDiagramTooltips } from "./render/diagram-tooltip";
import { installNodeChromeStyles } from "./render/node-chrome-style";

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
  const renderStartedAt = Date.now();
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
  installNodeChromeStyles(svg, theme);

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
  let generalRenderGeneration = 0;
  if (view === "action-flow-view") {
    addActionFlowMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    const drawStartedAt = Date.now();
    bounds = contentBoundsFromExtents(await renderActionFlowView({ root, prepared, theme, width, height, options }));
    options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
  } else if (view === "state-transition-view") {
    addStateTransitionMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    const drawStartedAt = Date.now();
    bounds = contentBoundsFromExtents(await renderStateTransitionView({ root, prepared, theme, width, height, options }));
    options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
  } else if (view === "sequence-view") {
    addSequenceMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
    const drawStartedAt = Date.now();
    bounds = contentBoundsFromExtents(renderSequenceView({ root, prepared, theme, width, height, options }));
    options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
  } else if (view === "browser-view") {
    const drawStartedAt = Date.now();
    bounds = contentBoundsFromExtents(renderBrowserView({ root, prepared, theme, width, height, options }));
    options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
  } else if (view === "grid-view") {
    const drawStartedAt = Date.now();
    bounds = contentBoundsFromExtents(renderGridView({ root, prepared, theme, width, height, options }));
    options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
  } else if (view === "geometry-view") {
    const drawStartedAt = Date.now();
    bounds = contentBoundsFromExtents(renderGeometryView({ root, prepared, theme, width, height, options }));
    options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
  } else {
    const layoutStartedAt = Date.now();
    const layout = await layoutPrepared(prepared);
    const layoutMs = Date.now() - layoutStartedAt;
    const drawStartedAt = Date.now();
    if (isInterconnectionView) {
      if (shouldDrawIbdViewFrame(prepared)) {
        drawIbdViewFrame(root, prepared, contentBounds(layout), theme);
      }
      drawInterconnectionContainers(root, prepared, layout.nodes, theme, layout.interconnectionLayout);
      drawNodes(root, layout.nodes, options, isInterconnectionView, theme, layout.interconnectionLayout);
      drawEdges(root, layout.edges, isInterconnectionView, theme, layout.interconnectionLayout);
      drawInterconnectionPortOverlays(root);
    } else {
      // Expansion and compartment disclosure are renderer-owned presentation state. They live for
      // the lifetime of this controller, so redraws inside one instance preserve what the viewer
      // opened. The projection below is the single place that state reaches layout and drawing.
      const expanded = new Set<string>();
      const sectionState = new Map<string, boolean>();
      const sectionKey = (nodeId: string, key: string) => `${nodeId}\u0000${key}`;
      const roots = new Set(Array.isArray(prepared.meta?.exposedRoots) ? prepared.meta?.exposedRoots as string[] : []);
      const ownerOf = (node: PreparedNode): string | undefined => {
        const owner = node.attributes?.owner;
        return typeof owner === "number" ? `n:${owner}` : undefined;
      };
      const owners = new Set(
        prepared.nodes.map(ownerOf).filter((value): value is string => Boolean(value)),
      );
      const compartmentSectionStateFor = (nodeId: string): Record<string, boolean> | undefined => {
        const prefix = `${nodeId}\u0000`;
        let state: Record<string, boolean> | undefined;
        for (const [key, value] of sectionState) {
          if (!key.startsWith(prefix)) continue;
          state = state ?? {};
          state[key.slice(prefix.length)] = value;
        }
        return state;
      };
      const visibleProjection = (): PreparedView => {
        const visible = new Set<string>();
        const visit = (node: PreparedNode): boolean => {
          if (visible.has(node.id)) return true;
          const owner = ownerOf(node);
          if (!owner) {
            if (roots.size === 0 || roots.has(node.id)) visible.add(node.id);
            return visible.has(node.id);
          }
          const parent = prepared.nodes.find((candidate) => candidate.id === owner);
          if (!parent || !visit(parent) || !expanded.has(owner)) return false;
          visible.add(node.id);
          return true;
        };
        prepared.nodes.forEach(visit);
        return {
          ...prepared,
          nodes: prepared.nodes.filter((node) => visible.has(node.id)).map((node) => {
            const isExpanded = expanded.has(node.id);
            const attributes: Record<string, unknown> = { ...node.attributes };
            if (owners.has(node.id)) {
              attributes.disclosure = isExpanded ? "expanded" : "collapsed";
            }
            if (isExpanded) {
              // Members that are now drawn as their own nodes are dropped from the compartment so
              // the same membership is not shown twice; members that never become nodes (inherited
              // features in particular) stay listed.
              const typed = Array.isArray(node.attributes?.typedCompartments)
                ? (node.attributes.typedCompartments as Array<Record<string, unknown>>)
                : [];
              attributes.typedCompartments = typed
                .map((compartment) => {
                  const members = Array.isArray(compartment.members) ? compartment.members : [];
                  return {
                    ...compartment,
                    members: members.filter((member) => {
                      const id = member && typeof member === "object"
                        ? (member as Record<string, unknown>).id
                        : undefined;
                      return !(typeof id === "string" && visible.has(id));
                    }),
                  };
                })
                .filter((compartment) => (compartment.members as unknown[]).length > 0);
            }
            const sections = compartmentSectionStateFor(node.id);
            if (sections) attributes.compartmentSectionState = sections;
            return { ...node, attributes };
          }),
          edges: prepared.edges.filter((edge) => visible.has(edge.source) && visible.has(edge.target)),
        };
      };
      const disclosure = {
        toggleNode: (nodeId: string): void => {
          if (expanded.has(nodeId)) expanded.delete(nodeId);
          else expanded.add(nodeId);
          void redrawGeneral({ refocusNodeControl: nodeId });
        },
        toggleSection: (nodeId: string, key: string, currentlyExpanded: boolean): void => {
          sectionState.set(sectionKey(nodeId, key), !currentlyExpanded);
          void redrawGeneral({ refocusSection: { nodeId, key } });
        },
      };
      const generalOptions: RenderOptions = { ...options, disclosure };
      const restoreFocus = (selector: string): void => {
        const element = target.querySelector<SVGGElement>(selector);
        if (element && typeof (element as unknown as HTMLElement).focus === "function") {
          (element as unknown as HTMLElement).focus();
        }
      };
      const redrawGeneral = async (
        focus?: { refocusNodeControl?: string; refocusSection?: { nodeId: string; key: string } },
      ): Promise<void> => {
        const generation = ++generalRenderGeneration;
        const visible = visibleProjection();
        const nextLayout = await layoutPrepared(visible);
        if (generation !== generalRenderGeneration) return;
        root.selectAll("*").remove();
        drawGeneralPackageContainers(root, visible, nextLayout.nodes, theme);
        drawEdges(root, nextLayout.edges, false, theme);
        drawNodes(root, nextLayout.nodes, generalOptions, false, theme);
        bounds = contentBounds(nextLayout);
        if (focus?.refocusNodeControl) {
          restoreFocus(`[data-node-id="${focus.refocusNodeControl}"] .general-node-toggle`);
        } else if (focus?.refocusSection) {
          restoreFocus(
            `[data-node-id="${focus.refocusSection.nodeId}"] [data-compartment-key="${focus.refocusSection.key}"]`,
          );
        }
      };
      await redrawGeneral();
    }
    options.onPerformance?.("sharedRenderer:layout", {
      view,
      layoutMs,
      nodeCount: prepared.nodes.length,
      edgeCount: prepared.edges.length,
    });
    options.onPerformance?.("sharedRenderer:draw", {
      view,
      drawMs: Date.now() - drawStartedAt,
      laidOutNodes: layout.nodes.length,
      laidOutEdges: layout.edges.length,
    });
    if (isInterconnectionView) bounds = contentBounds(layout);
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
  const destroyTooltips = installDiagramTooltips(target, prepared, theme);
  options.onPerformance?.("sharedRenderer:render", {
    view,
    totalMs: Date.now() - renderStartedAt,
    nodeCount: prepared.nodes.length,
    edgeCount: prepared.edges.length,
  });

  return {
    reset: () => fitView(),
    getFitTransform: () => lastFitTransform,
    exportSvg: () => exportSvg(svg.node() as SVGSVGElement, bounds),
    destroy: () => {
      destroyTooltips();
      target.innerHTML = "";
    },
  };
}
