import { prepareViewData } from "../../diagram-renderer/src/prepare";
import { isNativeDiagramView, renderVisualization, type RenderController } from "../../diagram-renderer/src/renderer";
import type { PreparedView } from "../../diagram-renderer/src/prepare";
import type { DiagramProductIdentity, RequestServerDraw } from "../../diagram-renderer/src/render/types";
import type { DiagramProduct } from "./diagramViewerCore";
import { isEmptyIncompleteDiagramProduct } from "./diagramProductState";

declare function acquireVsCodeApi(): { postMessage(message: unknown): void };

type RenderMessage = {
  type: "render";
  productJson: string;
  views: Array<{ handle: string; label: string; group: string }>;
  selectedHandle: string;
  header: string;
  incompleteReasons: string[];
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
const exportSvgButton = must<HTMLButtonElement>("export-svg");
const exportPngButton = must<HTMLButtonElement>("export-png");

let controller: RenderController | undefined;
let currentProduct: DiagramProduct | undefined;
let currentPrepared: PreparedView | undefined;
let currentIdentity: DiagramProductIdentity | undefined;

function must<T extends HTMLElement>(id: string): T {
  const element = document.getElementById(id);
  if (!element) throw new Error(`diagram webview shell is missing #${id}`);
  return element as T;
}

function currentColorScheme(): "light" | "dark" {
  const classes = document.body.className;
  return classes.includes("vscode-light") || classes.includes("vscode-high-contrast-light") ? "light" : "dark";
}

function populateSelect(views: RenderMessage["views"], selectedHandle: string): void {
  const groups = new Map<string, HTMLOptGroupElement>();
  const children: Array<HTMLOptGroupElement | HTMLOptionElement> = [];
  const singleGroup = new Set(views.map((view) => view.group)).size <= 1;
  for (const view of views) {
    const option = document.createElement("option");
    option.value = view.handle;
    option.textContent = view.label;
    option.selected = view.handle === selectedHandle;
    if (singleGroup) {
      children.push(option);
      continue;
    }
    let group = groups.get(view.group);
    if (!group) {
      group = document.createElement("optgroup");
      group.label = view.group;
      groups.set(view.group, group);
      children.push(group);
    }
    group.appendChild(option);
  }
  viewSelect.replaceChildren(...children);
  viewSelect.disabled = views.length <= 1;
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

async function render(message: RenderMessage): Promise<void> {
  populateSelect(message.views, message.selectedHandle);
  setStatus(message.header, message.error);

  if (message.placeholder) {
    controller?.destroy();
    controller = undefined;
    currentProduct = undefined;
    currentPrepared = undefined;
    currentIdentity = undefined;
    canvas.replaceChildren(message.loading ? withLoading(message.placeholder) : withText(message.placeholder));
    return;
  }

  let product: DiagramProduct;
  try {
    product = JSON.parse(message.productJson) as DiagramProduct;
  } catch {
    canvas.replaceChildren(withText("The generated diagram product was not valid JSON."));
    return;
  }

  controller?.destroy();
  controller = undefined;
  currentProduct = product;

  if (isEmptyIncompleteDiagramProduct(product)) {
    const reasons = message.incompleteReasons.length > 0
      ? message.incompleteReasons.join(", ")
      : "the projection is empty";
    canvas.replaceChildren(withText(`Nothing to draw yet — ${reasons}.`));
    return;
  }

  const prepared = prepareViewData(product);
  currentPrepared = prepared;
  const productIdentity: DiagramProductIdentity = {
    modelDigest: product.modelDigest,
    viewHandle: message.selectedHandle,
  };
  currentIdentity = productIdentity;
  const scheme = currentColorScheme();
  controller = await renderVisualization(canvas, prepared, {
    theme: { colorScheme: scheme },
    productIdentity,
    product,
    requestDraw: isNativeDiagramView(prepared.view) ? requestServerDraw : undefined,
    onNodeClick: (node) => {
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
    },
  });
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
  if (!currentProduct || !currentIdentity) throw new Error("Open a diagram before exporting it.");
  if (!controller) throw new Error("Wait for the diagram to finish rendering before exporting it.");
  const scheme = currentColorScheme();
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
    if (!svg) throw new Error("Could not export the diagram while the language server is unavailable.");
    return withBackground(svg, scheme);
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
    return withBackground(svg, scheme);
  } finally {
    holder.remove();
  }
}

function withBackground(svg: string, scheme: "light" | "dark"): string {
  const fill = scheme === "light" ? "#f6f7f9" : "#1a1a1a";
  const parsed = new DOMParser().parseFromString(svg, "image/svg+xml");
  const root = parsed.documentElement;
  if (root.nodeName.toLowerCase() !== "svg") return svg;
  const rect = parsed.createElementNS("http://www.w3.org/2000/svg", "rect");
  rect.setAttribute("x", "-100%");
  rect.setAttribute("y", "-100%");
  rect.setAttribute("width", "300%");
  rect.setAttribute("height", "300%");
  rect.setAttribute("fill", fill);
  root.insertBefore(rect, root.firstChild);
  return new XMLSerializer().serializeToString(root);
}

async function svgToPng(svg: string, scale = 2): Promise<string> {
  const source = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`;
  const image = new Image();
  await new Promise<void>((resolve, reject) => {
    image.onload = () => resolve();
    image.onerror = () => reject(new Error("the diagram SVG could not be rasterized"));
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

viewSelect.addEventListener("change", () => {
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
