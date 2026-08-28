import { prepareViewData } from "../../diagram-renderer/src/prepare";
import { renderVisualization, type RenderController } from "../../diagram-renderer/src/renderer";
import type { PreparedView } from "../../diagram-renderer/src/prepare";
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
  error?: string;
};

const vscode = acquireVsCodeApi();

const canvas = must<HTMLElement>("diagram");
const viewSelect = must<HTMLSelectElement>("view-select");
const statusEl = must<HTMLElement>("status");
const refreshButton = must<HTMLButtonElement>("refresh");
const copyButton = must<HTMLButtonElement>("copy-json");
const exportSvgButton = must<HTMLButtonElement>("export-svg");
const exportPngButton = must<HTMLButtonElement>("export-png");

let controller: RenderController | undefined;
let currentProduct: DiagramProduct | undefined;

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
    canvas.replaceChildren(withText(message.placeholder));
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
  controller = await renderVisualization(canvas, prepared, {
    theme: { colorScheme: "vscode" },
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

/** Re-render off-screen with a literal (non-CSS-variable) theme so the SVG stands alone. */
async function standaloneSvg(): Promise<string> {
  if (!currentProduct) throw new Error("Open a diagram before exporting it.");
  const scheme = currentColorScheme();
  const prepared: PreparedView = prepareViewData(currentProduct);
  const holder = document.createElement("div");
  holder.style.cssText = "position:absolute;left:-99999px;top:0;width:1600px;height:1000px;pointer-events:none";
  document.body.appendChild(holder);
  try {
    const offscreen = await renderVisualization(holder, prepared, {
      theme: { colorScheme: scheme },
      delegateZoom: true,
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
refreshButton.addEventListener("click", () => vscode.postMessage({ type: "refresh" }));
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
  }
});

vscode.postMessage({ type: "ready" });
