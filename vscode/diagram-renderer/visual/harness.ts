/**
 * Chrome visual-review harness entry point.
 *
 * Renders one corpus case per page so screenshots are taken at the real viewport size:
 *
 *   harness.html?case=<id>&theme=light|dark[&w=<px>&h=<px>][&chrome=0]
 *
 * The page sets `data-visual-ready="1"` on <html> once layout, drawing, fitting, and any scripted
 * disclosure activation have settled, so a screenshot driver never captures a partial frame.
 */
import { isEmptyIncompleteDiagramProduct } from "../../src/diagram/diagramProductState";
import { prepareViewData } from "../src/prepare";
import type { PreparedView } from "../src/prepare/types";
import { renderVisualization } from "../src/renderer";
import { SYNTHETIC_CASES, type VisualCase } from "./synthetic-cases";

interface ProductCase {
  id: string;
  title: string;
  view: string;
  product: Record<string, unknown>;
}

declare const __SPEC42_PRODUCT_CASES__: ProductCase[];

const productCases: ProductCase[] = typeof __SPEC42_PRODUCT_CASES__ === "undefined" ? [] : __SPEC42_PRODUCT_CASES__;

interface ResolvedCase {
  id: string;
  title: string;
  prepared: PreparedView | null;
  /** Set when the product publishes an empty incomplete state instead of a view. */
  incompleteReasons?: string[];
  expand: string[];
}

function allCaseIds(): string[] {
  return [...SYNTHETIC_CASES.map((entry) => entry.id), ...productCases.map((entry) => entry.id)];
}

function resolveCase(id: string): ResolvedCase | null {
  const synthetic = SYNTHETIC_CASES.find((entry: VisualCase) => entry.id === id);
  if (synthetic) {
    return {
      id: synthetic.id,
      title: synthetic.title,
      prepared: JSON.parse(JSON.stringify(synthetic.prepared)) as PreparedView,
      expand: synthetic.expand ?? [],
    };
  }
  const product = productCases.find((entry) => entry.id === id);
  if (!product) return null;
  const state = product.product as unknown as {
    completeness: { status: string; reasons?: Array<{ code?: string }> };
    projection: { nodes: unknown[] };
  };
  if (isEmptyIncompleteDiagramProduct(state)) {
    // Same explicit incomplete state the VS Code webview shows; it must not look like an empty
    // but successful diagram.
    return {
      id: product.id,
      title: product.title,
      prepared: null,
      incompleteReasons: (state.completeness.reasons ?? []).map((reason) => reason.code ?? "unknown"),
      expand: [],
    };
  }
  return {
    id: product.id,
    title: product.title,
    prepared: prepareViewData(product.product),
    expand: [],
  };
}

function frame(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function activateDisclosure(host: HTMLElement, nodeId: string): Promise<void> {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    const control = host.querySelector<SVGGElement>(`[data-node-id="${nodeId}"] .general-node-toggle`);
    if (control) {
      control.dispatchEvent(new MouseEvent("click", { bubbles: true }));
      // The redraw is asynchronous (ELK relayout); wait for the control to report the new state.
      for (let settle = 0; settle < 120; settle += 1) {
        const after = host.querySelector<SVGGElement>(`[data-node-id="${nodeId}"] .general-node-toggle`);
        if (after?.getAttribute("aria-expanded") === "true") return;
        await frame(10);
      }
      return;
    }
    await frame(10);
  }
}

function renderIndex(root: HTMLElement, theme: string): void {
  const list = document.createElement("ul");
  list.className = "case-index";
  for (const id of allCaseIds()) {
    const item = document.createElement("li");
    const link = document.createElement("a");
    link.href = `?case=${encodeURIComponent(id)}&theme=${encodeURIComponent(theme)}`;
    link.textContent = id;
    item.appendChild(link);
    list.appendChild(item);
  }
  root.appendChild(list);
}

async function main(): Promise<void> {
  const params = new URLSearchParams(window.location.search);
  const theme = params.get("theme") === "dark" ? "dark" : "light";
  document.documentElement.setAttribute("data-harness-theme", theme);

  const caseId = params.get("case");
  const host = document.getElementById("diagram");
  const caption = document.getElementById("caption");
  if (!(host instanceof HTMLElement) || !(caption instanceof HTMLElement)) return;

  if (params.get("chrome") === "0") {
    caption.style.display = "none";
  }

  if (!caseId) {
    caption.textContent = "spec42 diagram visual corpus";
    renderIndex(host, theme);
    document.documentElement.setAttribute("data-visual-ready", "1");
    return;
  }

  const width = Number(params.get("w") ?? "");
  const height = Number(params.get("h") ?? "");
  if (Number.isFinite(width) && width > 0) host.style.width = `${width}px`;
  if (Number.isFinite(height) && height > 0) host.style.height = `${height}px`;

  const resolved = resolveCase(caseId);
  if (!resolved) {
    caption.textContent = `unknown case: ${caseId}`;
    document.documentElement.setAttribute("data-visual-ready", "error");
    return;
  }

  caption.textContent = `${resolved.id} — ${resolved.title} — ${theme}`;
  if (!resolved.prepared) {
    const empty = document.createElement("div");
    empty.className = "incomplete";
    empty.textContent = `incomplete diagram product: ${resolved.incompleteReasons?.join(", ") ?? "unknown"}`;
    host.appendChild(empty);
    document.documentElement.setAttribute("data-visual-ready", "1");
    return;
  }
  const controller = await renderVisualization(host, resolved.prepared, {
    theme: { colorScheme: theme },
    onNodeClick: () => {
      // Source navigation is a host concern; the harness only records that it fired.
      document.documentElement.setAttribute("data-node-click", "1");
    },
  });
  for (const nodeId of resolved.expand) {
    await activateDisclosure(host, nodeId);
  }
  controller.reset();
  // `applyFit` animates the fit transform over 180ms; wait past it so a driver never screenshots
  // (or clicks) a frame that is still moving.
  await frame(320);
  document.documentElement.setAttribute("data-visual-ready", "1");
}

void main();
