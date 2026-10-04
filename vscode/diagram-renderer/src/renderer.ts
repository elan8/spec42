import * as d3 from "d3";
import { resolveDiagramTheme } from "./theme";
import type { PreparedView } from "./prepare";
import type { PreparedNode } from "./prepare";
import {
  applyFit,
  contentBoundsFromViewBox,
  exportSvg,
} from "./render/export";
import { isNativeDiagramView, type ContentBounds, type DisclosureState, type RenderOptions } from "./render/types";
import { installNativeSvgTooltips } from "./render/diagram-tooltip";

export type { DisclosureState, RenderOptions } from "./render/types";
export { isNativeDiagramView, NATIVE_DIAGRAM_VIEWS } from "./render/types";

export interface RenderController {
  reset: () => void;
  exportSvg: () => string;
  destroy: () => void;
  getFitTransform: () => d3.ZoomTransform;
  getDisclosureState: () => DisclosureState;
}

const INERT_MESSAGE = "This diagram is drawn by the Spec42 language server. Reconnect to render it.";

function installZoom(
  svg: d3.Selection<SVGSVGElement, unknown, null, undefined>,
  root: d3.Selection<SVGGElement, unknown, null, undefined>,
  delegateZoom: boolean,
): d3.ZoomBehavior<SVGSVGElement, unknown> {
  const zoom = d3.zoom<SVGSVGElement, unknown>()
    .scaleExtent([0.08, 5])
    .on("start", () => svg.style("cursor", "grabbing"))
    .on("zoom", (event: { transform: d3.ZoomTransform }) => {
      root.attr("transform", event.transform.toString());
    })
    .on("end", () => svg.style("cursor", "grab"));
  if (!delegateZoom) {
    svg
      .call(zoom)
      .on("dblclick.zoom", null)
      .on("wheel.zoom", function (event: WheelEvent) {
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
  return zoom;
}

const canvasOwners = new WeakMap<HTMLElement, object>();

function claimCanvas(target: HTMLElement, owner: object): void {
  canvasOwners.set(target, owner);
}

function clearCanvasIfOwner(target: HTMLElement, owner: object): void {
  if (canvasOwners.get(target) !== owner) return;
  target.innerHTML = "";
  canvasOwners.delete(target);
}

function inertController(target: HTMLElement, disclosure: DisclosureState, message = INERT_MESSAGE): RenderController {
  const owner = {};
  claimCanvas(target, owner);
  target.replaceChildren();
  const empty = target.ownerDocument.createElement("div");
  empty.className = "empty";
  empty.textContent = message;
  target.appendChild(empty);
  return {
    reset: () => undefined,
    exportSvg: () => {
      throw new Error("Wait for the diagram to finish rendering before exporting it.");
    },
    destroy: () => {
      clearCanvasIfOwner(target, owner);
    },
    getFitTransform: () => d3.zoomIdentity,
    getDisclosureState: () => disclosure,
  };
}

function nodeById(prepared: PreparedView, id: string): PreparedNode | undefined {
  return prepared.nodes.find((node) => node.id === id);
}

async function renderNativeSvgView(
  target: HTMLElement,
  prepared: PreparedView,
  options: RenderOptions,
): Promise<RenderController> {
  const theme = resolveDiagramTheme(options.theme);
  const width = Math.max(720, target.clientWidth || 960);
  const height = Math.max(480, target.clientHeight || 640);
  const scheme = theme.colorScheme === "dark" ? "dark" : "light";
  const expanded = new Set(options.disclosureState?.expandedNodeIds ?? []);
  const sectionState = new Map<string, boolean>(
    (options.disclosureState?.sectionStates ?? []).map((state) => [
      `${state.nodeId}\u0000${state.sectionKey}`,
      state.expanded,
    ]),
  );
  const getDisclosureState = (): DisclosureState => ({
    expandedNodeIds: [...expanded],
    sectionStates: [...sectionState].map(([key, value]) => {
      const separator = key.indexOf("\u0000");
      return {
        nodeId: key.slice(0, separator),
        sectionKey: key.slice(separator + 1),
        expanded: value,
      };
    }),
  });

  const { requestDraw, productIdentity, product } = options;
  if (!requestDraw || !productIdentity || product === undefined) {
    return inertController(target, getDisclosureState());
  }

  let renderGeneration = 0;
  let activeAbort: AbortController | undefined;
  let destroyTooltips = (): void => undefined;
  let lastFitTransform = d3.zoomIdentity;
  let lastBounds: ContentBounds = { x: 0, y: 0, width: 100, height: 100 };
  let fitView = (): void => undefined;
  let lastSvg: string | null = null;
  const canvasOwner = {};

  const superseded = (generation?: number): boolean =>
    options.abortSignal?.aborted === true
    || activeAbort?.signal.aborted === true
    || (generation !== undefined && generation !== renderGeneration);

  const restoreFocus = (selector: string): void => {
    const element = target.querySelector<SVGGElement>(selector);
    if (element && typeof (element as unknown as HTMLElement).focus === "function") {
      (element as unknown as HTMLElement).focus();
    }
  };

  const mount = (svgMarkup: string, generation: number): void => {
    if (superseded(generation)) return;
    destroyTooltips();
    claimCanvas(target, canvasOwner);
    target.innerHTML = svgMarkup;
    const svgNode = target.querySelector<SVGSVGElement>("svg.sysml-viz-svg");
    const rootNode = svgNode?.querySelector<SVGGElement>("g.viz-root");
    if (!svgNode || !rootNode) return;
    svgNode.setAttribute("data-color-scheme", scheme);
    const svg = d3.select(svgNode);
    const root = d3.select(rootNode);
    const contentBounds = contentBoundsFromViewBox(svgNode);
    svg.attr("viewBox", `0 0 ${width} ${height}`);
    svg.select(".viz-bg").attr("width", width).attr("height", height);
    const zoom = installZoom(svg, root, options.delegateZoom === true);
    lastBounds = contentBounds;
    fitView = () => {
      lastFitTransform = applyFit(
        svg,
        zoom,
        root,
        contentBounds,
        width,
        height,
        prepared.view === "interconnection-view"
          || prepared.view === "action-flow-view"
          || prepared.view === "state-transition-view"
          || prepared.view === "sequence-view"
          || prepared.view === "browser-view"
          || prepared.view === "grid-view"
          || prepared.view === "geometry-view",
        options.delegateZoom === true,
      );
    };
    fitView();
    destroyTooltips = installNativeSvgTooltips(target, theme);
  };

  const redraw = async (
    focus?: { refocusNodeControl?: string; refocusSection?: { nodeId: string; key: string } },
  ): Promise<void> => {
    const generation = ++renderGeneration;
    activeAbort?.abort();
    const abort = new AbortController();
    activeAbort = abort;
    if (options.abortSignal?.aborted) {
      abort.abort();
    } else {
      options.abortSignal?.addEventListener("abort", () => abort.abort(), { once: true });
    }
    if (superseded(generation)) return;
    const svgMarkup = await requestDraw(
      productIdentity,
      generation,
      {
        product,
        width,
        height,
        colorScheme: scheme,
        disclosure: getDisclosureState(),
      },
      abort.signal,
    );
    if (superseded(generation)) return;
    if (!svgMarkup) {
      if (lastSvg) return;
      claimCanvas(target, canvasOwner);
      target.replaceChildren();
      const empty = target.ownerDocument.createElement("div");
      empty.className = "empty";
      empty.textContent = INERT_MESSAGE;
      target.appendChild(empty);
      return;
    }
    lastSvg = svgMarkup;
    mount(svgMarkup, generation);
    if (superseded(generation)) return;
    if (focus?.refocusNodeControl) {
      restoreFocus(`[data-node-id="${focus.refocusNodeControl}"] .general-node-toggle`);
    } else if (focus?.refocusSection) {
      restoreFocus(
        `[data-node-id="${focus.refocusSection.nodeId}"] [data-compartment-key="${focus.refocusSection.key}"]`,
      );
    }
  };

  const onActivate = (event: Event): void => {
    const origin = event.target instanceof Element ? event.target : null;
    if (!origin || !target.contains(origin)) return;
    if (event instanceof KeyboardEvent && event.key !== "Enter" && event.key !== " ") return;
    if (event instanceof KeyboardEvent) event.preventDefault();

    const section = origin.closest<SVGGElement>(".sysml-compartment-toggle");
    if (section) {
      event.stopPropagation();
      const nodeId = section.closest("[data-node-id]")?.getAttribute("data-node-id") ?? "";
      const key = section.getAttribute("data-compartment-key") ?? "";
      if (!nodeId || !key) return;
      const currentlyExpanded = section.getAttribute("aria-expanded") !== "false";
      sectionState.set(`${nodeId}\u0000${key}`, !currentlyExpanded);
      void redraw({ refocusSection: { nodeId, key } });
      return;
    }
    const toggle = origin.closest<SVGGElement>(".general-node-toggle");
    if (toggle) {
      event.stopPropagation();
      const nodeId = toggle.closest("[data-node-id]")?.getAttribute("data-node-id") ?? "";
      if (!nodeId) return;
      if (expanded.has(nodeId)) expanded.delete(nodeId);
      else expanded.add(nodeId);
      void redraw({ refocusNodeControl: nodeId });
      return;
    }
    if (!(event instanceof MouseEvent)) return;
    const nodeGroup = origin.closest<SVGGElement>("[data-node-id]");
    if (!nodeGroup || origin.closest(".sysml-disclosure")) return;
    const node = nodeById(prepared, nodeGroup.getAttribute("data-node-id") ?? "");
    if (node) options.onNodeClick?.(node);
  };

  target.addEventListener("click", onActivate);
  target.addEventListener("keydown", onActivate);
  await redraw();

  return {
    reset: () => fitView(),
    getFitTransform: () => lastFitTransform,
    getDisclosureState,
    exportSvg: () => {
      const svgNode = target.querySelector<SVGSVGElement>("svg.sysml-viz-svg");
      if (!svgNode) throw new Error("Wait for the diagram to finish rendering before exporting it.");
      return exportSvg(svgNode, lastBounds);
    },
    destroy: () => {
      renderGeneration += 1;
      activeAbort?.abort();
      destroyTooltips();
      target.removeEventListener("click", onActivate);
      target.removeEventListener("keydown", onActivate);
      clearCanvasIfOwner(target, canvasOwner);
    },
  };
}

export async function renderVisualization(
  target: HTMLElement,
  prepared: PreparedView,
  options: RenderOptions = {},
): Promise<RenderController> {
  if (!isNativeDiagramView(prepared.view)) {
    return inertController(target, { expandedNodeIds: [], sectionStates: [] }, `Unsupported view: ${prepared.view}`);
  }
  return renderNativeSvgView(target, prepared, options);
}
