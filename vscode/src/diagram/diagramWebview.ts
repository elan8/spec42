import { diagramLimitations } from "./diagramLimitations";
import { populateViewSelect, withExportBackground, type ViewChoice } from "./viewPresentation";
import { prepareViewData } from "../../diagram-renderer/src/prepare";
import { isNativeDiagramView, renderVisualization, type RenderController } from "../../diagram-renderer/src/renderer";
import type { PreparedView } from "../../diagram-renderer/src/prepare";
import type { DiagramProductIdentity, DisclosureState, RequestServerDraw, ViewportTransform } from "../../diagram-renderer/src/render/types";
import type { DiagramProduct } from "./diagramViewerCore";
import { isEmptyIncompleteDiagramProduct, sameProjectedNodes } from "./diagramProductState";

declare function acquireVsCodeApi(): { postMessage(message: unknown): void };

type RenderMessage = {
  type: "render";
  productJson: string;
  views: ViewChoice[];
  selectedHandle: string;
  header: string;
  placeholder?: string;
  loading?: boolean;
  error?: string;
};

type DrawResponseMessage = {
  type: "drawResponse";
  requestId: number;
  modelDigest: string;
  viewHandle: string;
  presentationRevision: number;
  svg: string;
};

type DrawErrorMessage = {
  type: "drawError";
  requestId: number;
  message?: string;
};

const vscode = acquireVsCodeApi();

/** Draw includes prepare+layout+SVG, so this is longer than the old layout-only timeout.
 * No response is treated as a decline: keep the last SVG or show an inert placeholder. */
const DRAW_REQUEST_TIMEOUT_MS = 8_000;

let drawRequestSeq = 0;
const pendingDrawRequests = new Map<
  number,
  {
    settle: (svg: string | null) => void;
    identity: DiagramProductIdentity;
    presentationRevision: number;
  }
>();

const requestServerDraw: RequestServerDraw = (identity, presentationRevision, request, signal) => {
  if (signal.aborted) return Promise.resolve(null);
  const requestId = ++drawRequestSeq;
  return new Promise((resolve) => {
    let timeout: ReturnType<typeof setTimeout>;
    const settle = (svg: string | null): void => {
      pendingDrawRequests.delete(requestId);
      clearTimeout(timeout);
      resolve(svg);
    };
    timeout = setTimeout(() => settle(null), DRAW_REQUEST_TIMEOUT_MS);
    signal.addEventListener("abort", () => settle(null), { once: true });
    pendingDrawRequests.set(requestId, { settle, identity, presentationRevision });
    vscode.postMessage({
      type: "drawRequest",
      requestId,
      modelDigest: identity.modelDigest,
      viewHandle: identity.viewHandle,
      presentationRevision,
      product: request.product,
      width: request.width,
      height: request.height,
      colorScheme: request.colorScheme,
      disclosure: request.disclosure,
    });
  });
};

function onDrawResponse(message: DrawResponseMessage | DrawErrorMessage): void {
  const pending = pendingDrawRequests.get(message.requestId);
  if (!pending) return;
  if (message.type === "drawError") {
    pending.settle(null);
    return;
  }
  if (
    message.modelDigest !== pending.identity.modelDigest ||
    message.viewHandle !== pending.identity.viewHandle ||
    message.presentationRevision !== pending.presentationRevision
  ) {
    pending.settle(null);
    return;
  }
  pending.settle(message.svg);
}

const canvas = must<HTMLElement>("diagram");
const viewSelect = must<HTMLSelectElement>("view-select");
const statusEl = must<HTMLElement>("status");
const homeButton = must<HTMLButtonElement>("home");
const copyButton = must<HTMLButtonElement>("copy-json");
const definitionButton = must<HTMLButtonElement>("view-definition");
const viewKindEl = must<HTMLElement>("view-kind");
const transparentExport = must<HTMLInputElement>("transparent-export");
let visibleViews: ViewChoice[] = [];

const exportSvgButton = must<HTMLButtonElement>("export-svg");
const exportPngButton = must<HTMLButtonElement>("export-png");

let controller: RenderController | undefined;
let currentProduct: DiagramProduct | undefined;
let currentPrepared: PreparedView | undefined;
let currentIdentity: DiagramProductIdentity | undefined;
let retainedViewport: ViewportTransform | undefined;
let renderGeneration = 0;
let renderAbort: AbortController | undefined;

function must<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) throw new Error(`diagram webview shell is missing #${id}`);
  return element as T;
}

function currentColorScheme(): "light" | "dark" {
  const classes = document.body.className;
  return classes.includes("vscode-light") || classes.includes("vscode-high-contrast-light") ? "light" : "dark";
}

function themeClassSignature(): string {
  return ["vscode-high-contrast-light", "vscode-light", "vscode-high-contrast", "vscode-dark"]
    .filter((name) => document.body.classList.contains(name))
    .join(" ");
}

function beginRender(): { generation: number; signal: AbortSignal } {
  const generation = ++renderGeneration;
  renderAbort?.abort();
  const abort = new AbortController();
  renderAbort = abort;
  return { generation, signal: abort.signal };
}

function isCurrentRender(generation: number, signal: AbortSignal): boolean {
  return generation === renderGeneration && !signal.aborted;
}

function clearRenderState(): void {
  controller?.destroy();
  controller = undefined;
  currentProduct = undefined;
  currentPrepared = undefined;
  currentIdentity = undefined;
  retainedViewport = undefined;
}

function openSource(node: { uri?: string; sourcePath?: string; range?: { start?: { line: number; character?: number }; end?: { line?: number; character?: number } } }): void {
  const range = node.range;
  const uri = node.uri ?? node.sourcePath;
  if (!uri || !range?.start || !range.end) return;
  vscode.postMessage({
    type: "openSource",
    target: {
      uri,
      startLine: range.start.line,
      startCharacter: range.start.character ?? 0,
      endLine: range.end.line ?? range.start.line,
      endCharacter: range.end.character ?? range.start.character ?? 0,
    },
  });
}

async function mountPrepared(
  generation: number,
  signal: AbortSignal,
  prepared: PreparedView,
  product: DiagramProduct,
  productIdentity: DiagramProductIdentity,
  disclosureState?: DisclosureState,
  viewport?: ViewportTransform,
): Promise<void> {
  const previous = controller;
  controller = undefined;
  previous?.destroy();
  const next = await renderVisualization(canvas, prepared, {
    theme: { colorScheme: currentColorScheme() },
    productIdentity,
    product,
    requestDraw: isNativeDiagramView(prepared.view) ? requestServerDraw : undefined,
    disclosureState,
    viewport,
    abortSignal: signal,
    onNodeClick: openSource,
    loadBranch: (key) => vscode.postMessage({ type: "loadBranch", key }),
  });
  if (!isCurrentRender(generation, signal)) {
    next.destroy();
    return;
  }
  controller = next;
}

function populateSelect(views: RenderMessage["views"], selectedHandle: string): void {
  visibleViews = views;
  populateViewSelect(viewSelect, views, selectedHandle);
  updateSelectedViewInfo();
}

function updateSelectedViewInfo(): void {
  const view = visibleViews.find((choice) => choice.handle === viewSelect.value);
  viewKindEl.textContent = view?.kindLabel ?? "";
  definitionButton.disabled = !view?.definition;
}

function setStatus(header: string, error: string | undefined): void {
  statusEl.textContent = "";
  if (error) {
    const marker = document.createElement("span");
    marker.className = "error";
    marker.textContent = `stale: ${error} · `;
    statusEl.appendChild(marker);
  }
  statusEl.appendChild(document.createTextNode(header));
}

function showLimitations(product?: DiagramProduct, viewHandle?: string): void {
  const host = must<HTMLElement>("limitations");
  const wasOpen = host.querySelector("details")?.open ?? false;
  host.replaceChildren();
  const limitations = product ? diagramLimitations(product) : [];
  host.hidden = limitations.length === 0;
  if (!product || limitations.length === 0) return;
  const details = document.createElement("details");
  details.open = wasOpen;
  const summary = document.createElement("summary");
  summary.textContent = `Projection limitations (${limitations.length})`;
  details.appendChild(summary);
  const list = document.createElement("ul");
  for (const limitation of limitations) {
    const item = document.createElement("li");
    const code = document.createElement("code");
    code.textContent = limitation.code;
    const text = document.createElement("p");
    text.textContent = limitation.explanation;
    item.append(code, text);
    for (const link of limitation.links) {
      const button = document.createElement("button");
      button.type = "button";
      button.textContent = link.label;
      button.addEventListener("click", () => vscode.postMessage({ type: "openSource", target: link.target }));
      item.appendChild(button);
    }
    list.appendChild(item);
  }
  const copy = document.createElement("button");
  copy.type = "button";
  copy.textContent = "Copy structured details";
  copy.addEventListener("click", () => vscode.postMessage({ type: "copyLimitations", modelDigest: product.modelDigest, viewHandle }));
  details.append(list, copy);
  host.appendChild(details);
}

async function render(message: RenderMessage): Promise<void> {
  const { generation, signal } = beginRender();
  try {
    await renderCurrent(message, generation, signal);
  } catch (error) {
    // A render that throws (a malformed product, a failed layout) must not leave the previous
    // view's SVG on the canvas looking current. A superseded render's failure is irrelevant.
    if (!isCurrentRender(generation, signal)) return;
    clearRenderState();
    showLimitations();
    const reason = error instanceof Error ? error.message : String(error);
    canvas.replaceChildren(withText(`The view could not be rendered: ${reason}`));
  }
}

async function renderCurrent(message: RenderMessage, generation: number, signal: AbortSignal): Promise<void> {
  populateSelect(message.views, message.selectedHandle);
  setStatus(message.header, message.error);

  if (message.placeholder) {
    showLimitations();
    if (!isCurrentRender(generation, signal)) return;
    clearRenderState();
    canvas.replaceChildren(message.loading ? withLoading(message.placeholder) : withText(message.placeholder));
    return;
  }

  let product: DiagramProduct;
  try {
    product = JSON.parse(message.productJson) as DiagramProduct;
  } catch {
    if (!isCurrentRender(generation, signal)) return;
    clearRenderState();
    showLimitations();
    canvas.replaceChildren(withText("The generated diagram product was not valid JSON."));
    return;
  }

  showLimitations(product, message.selectedHandle);
  if (isEmptyIncompleteDiagramProduct(product)) {
    if (!isCurrentRender(generation, signal)) return;
    clearRenderState();
    const reasons = diagramLimitations(product).map((reason) => reason.explanation).join(" ") || "the projection is empty";
    canvas.replaceChildren(withText(`Nothing to draw yet — ${reasons}.`));
    return;
  }

  const sameView = currentIdentity?.viewHandle === message.selectedHandle;
  retainedViewport = sameView ? controller?.getViewport() ?? retainedViewport : undefined;
  const viewport = retainedViewport;
  // Disclosure names nodes by product index; it is reusable only while the same nodes are projected.
  const disclosure = sameView && sameProjectedNodes(currentProduct, product) ? controller?.getDisclosureState() : undefined;
  currentProduct = product;

  const prepared = prepareViewData(product);
  currentPrepared = prepared;
  const productIdentity: DiagramProductIdentity = {
    modelDigest: product.modelDigest,
    viewHandle: message.selectedHandle,
  };
  currentIdentity = productIdentity;
  await mountPrepared(generation, signal, prepared, product, productIdentity, disclosure, viewport);
}

async function redrawPreservingDisclosure(): Promise<void> {
  if (!currentProduct || !currentPrepared || !currentIdentity) return;
  const disclosure = controller?.getDisclosureState();
  const viewport = controller?.getViewport();
  const { generation, signal } = beginRender();
  await mountPrepared(generation, signal, currentPrepared, currentProduct, currentIdentity, disclosure, viewport);
}

function withText(text: string): HTMLElement {
  const div = document.createElement("div");
  div.className = "empty";
  div.textContent = text;
  return div;
}

function withLoading(text: string): HTMLElement {
  const div = withText(text);
  div.classList.add("loading");
  const spinner = document.createElement("span");
  spinner.className = "loading-spinner";
  spinner.setAttribute("aria-hidden", "true");
  div.prepend(spinner);
  div.setAttribute("role", "status");
  div.setAttribute("aria-live", "polite");
  return div;
}

async function standaloneSvg(): Promise<string> {
  if (!currentProduct || !currentIdentity) throw new Error("Open a view before exporting it.");
  if (!controller) throw new Error("Wait for the view to finish rendering before exporting it.");
  const scheme = "light" as const;
  if (currentPrepared && isNativeDiagramView(currentPrepared.view)) {
    const svg = await requestServerDraw(
      currentIdentity,
      Date.now(),
      {
        product: currentProduct,
        width: 1600,
        height: 1000,
        colorScheme: scheme,
        disclosure: controller.getDisclosureState(),
      },
      new AbortController().signal,
    );
    if (!svg) throw new Error("Could not export the view while the language server is unavailable.");
    return withExportBackground(svg, transparentExport.checked);
  }
  const prepared: PreparedView = prepareViewData(currentProduct);
  const holder = document.createElement("div");
  holder.style.cssText = "position:absolute;left:-99999px;top:0;width:1600px;height:1000px;pointer-events:none";
  document.body.appendChild(holder);
  try {
    const offscreen = await renderVisualization(holder, prepared, {
      theme: { colorScheme: scheme },
      delegateZoom: true,
      disclosureState: controller.getDisclosureState(),
    });
    const svg = offscreen.exportSvg();
    offscreen.destroy();
    return withExportBackground(svg, transparentExport.checked);
  } finally {
    holder.remove();
  }
}

async function svgToPng(svg: string, scale = 2): Promise<string> {
  const source = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`;
  const image = new Image();
  await new Promise<void>((resolve, reject) => {
    image.onload = () => resolve();
    image.onerror = () => reject(new Error("the view SVG could not be rasterized"));
    image.src = source;
  });
  const viewBox = /viewBox="\s*(-?[\d.]+)\s+(-?[\d.]+)\s+(-?[\d.]+)\s+(-?[\d.]+)"/.exec(svg);
  const width = viewBox ? Math.max(1, Math.ceil(parseFloat(viewBox[3]))) : image.naturalWidth || 1600;
  const height = viewBox ? Math.max(1, Math.ceil(parseFloat(viewBox[4]))) : image.naturalHeight || 1000;
  const target = document.createElement("canvas");
  target.width = width * scale;
  target.height = height * scale;
  const context = target.getContext("2d");
  if (!context) throw new Error("this webview has no 2D canvas context");
  context.scale(scale, scale);
  context.drawImage(image, 0, 0, width, height);
  return target.toDataURL("image/png");
}

async function exportImage(format: "svg" | "png"): Promise<void> {
  try {
    const svg = await standaloneSvg();
    const data = format === "svg" ? svg : await svgToPng(svg);
    vscode.postMessage({ type: "export", format, data });
  } catch (error) {
    setStatus(error instanceof Error ? error.message : String(error), "export failed");
  }
}

definitionButton.addEventListener("click", () => {
  const target = visibleViews.find((view) => view.handle === viewSelect.value)?.definition;
  if (target) vscode.postMessage({type:"openSource", target});
});
viewSelect.addEventListener("change", () => {
  updateSelectedViewInfo();
  vscode.postMessage({ type: "switchView", handle: viewSelect.value });
});
homeButton.addEventListener("click", () => controller?.reset());
copyButton.addEventListener("click", () => vscode.postMessage({ type: "copyJson" }));
exportSvgButton.addEventListener("click", () => void exportImage("svg"));
exportPngButton.addEventListener("click", () => void exportImage("png"));

window.addEventListener("message", (event: MessageEvent) => {
  const message = event.data as { type?: string } | null;
  if (!message) return;
  if (message.type === "render") {
    document.body.classList.remove("busy");
    void render(message as RenderMessage);
  } else if (message.type === "busy") {
    document.body.classList.toggle("busy", Boolean((message as { busy?: unknown }).busy));
  } else if (message.type === "drawResponse" || message.type === "drawError") {
    onDrawResponse(message as DrawResponseMessage | DrawErrorMessage);
  }
});

vscode.postMessage({ type: "ready" });

let lastThemeSignature = themeClassSignature();
new MutationObserver(() => {
  const next = themeClassSignature();
  if (next === lastThemeSignature) return;
  lastThemeSignature = next;
  void redrawPreservingDisclosure();
}).observe(document.body, { attributes: true, attributeFilter: ["class"] });
