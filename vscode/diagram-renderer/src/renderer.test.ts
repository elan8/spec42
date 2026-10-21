// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { renderVisualization } from "./renderer";
import type { RequestServerDraw } from "./render/types";

const LIGHT_THEME = { colorScheme: "light" as const };

function host(): HTMLElement {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });
  return target;
}

const SAMPLE_SVG = `<svg class="sysml-viz-svg" width="100%" height="100%" viewBox="0 0 200 100" role="img">
  <rect class="viz-bg" width="200" height="100" fill="#f6f7f9"></rect>
  <g class="viz-root">
    <g class="view-frame" style="pointer-events:none"><rect class="view-frame-border" x="0" y="0" width="200" height="100"></rect><text class="view-frame-name">«view» General</text></g>
    <g class="general-node viz-node" data-node-id="n:0">
      <g class="general-node-toggle sysml-disclosure" role="button" tabindex="0" aria-expanded="false"></g>
      <g class="sysml-compartment-toggle" data-compartment-key="parts" role="button" aria-expanded="true"></g>
    </g>
  </g>
</svg>`;

describe("shared renderer", () => {
  it("preserves each native view frame when mounting, fitting and exporting SVG", async () => {
    for (const [fixture, view] of [
      ["general-view", "general-view"],
      ["interconnection-view-fidelity", "interconnection-view"],
      ["action-flow-view", "action-flow-view"],
      ["state-transition-view", "state-transition-view"],
      ["sequence-view", "sequence-view"],
    ]) {
      const svg = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../../crates/diagram_draw/tests/fixtures", `${fixture}.rendered.svg`), "utf8");
      const original = new DOMParser().parseFromString(svg, "image/svg+xml");
      const target = host();
      const controller = await renderVisualization(target, { title:"View", view, nodes:[], edges:[] }, {
        theme: LIGHT_THEME,
        productIdentity: {modelDigest:"blake3:test", viewHandle:view},
        product: {schemaVersion:5},
        requestDraw: async () => svg,
      });
      expect(target.querySelectorAll(".view-frame")).toHaveLength(1);
      expect(target.querySelector(".view-frame-name")?.textContent).toBe(original.querySelector(".view-frame-name")?.textContent);
      controller.reset();
      const exported = new DOMParser().parseFromString(controller.exportSvg(), "image/svg+xml");
      expect(exported.querySelectorAll(".view-frame")).toHaveLength(1);
      expect(exported.querySelector(".view-frame-name")?.textContent).toBe(original.querySelector(".view-frame-name")?.textContent);
      controller.destroy();
    }
  });

  it("does not draw Browser, Grid, or Geometry locally when the host has no requestDraw", async () => {
    for (const view of ["browser-view", "grid-view", "geometry-view"]) {
      const target = host();
      await renderVisualization(target, {
        title: view,
        view,
        nodes: [{ id: "system", label: "System", kind: "part def" }],
        edges: [],
      }, { theme: LIGHT_THEME });
      expect(target.querySelector("svg")).toBeNull();
      expect(target.textContent).toMatch(/language server/i);
    }
  });

  it("does not draw native views locally when the host has no requestDraw", async () => {
    const target = host();
    await renderVisualization(target, {
      title: "General",
      view: "general-view",
      nodes: [{ id: "n:0", label: "Root", kind: "part" }],
      edges: [],
    }, { theme: LIGHT_THEME });
    expect(target.querySelector("svg")).toBeNull();
    expect(target.textContent).toMatch(/language server/i);
  });

  it("mounts server SVG, refits, and re-requests on disclosure", async () => {
    const target = host();
    const requestDraw = vi.fn<RequestServerDraw>(async () => SAMPLE_SVG);
    const onNodeClick = vi.fn();
    const controller = await renderVisualization(
      target,
      {
        title: "General",
        view: "general-view",
        nodes: [{ id: "n:0", label: "Root", kind: "part", uri: "file:///model.sysml", range: { start: { line: 1 }, end: { line: 1 } } }],
        edges: [],
      },
      {
        theme: LIGHT_THEME,
        productIdentity: { modelDigest: "blake3:test", viewHandle: "general/root" },
        product: { schemaVersion: 7 },
        requestDraw,
        onNodeClick,
      },
    );
    expect(requestDraw).toHaveBeenCalledTimes(1);
    expect(target.querySelector("svg.sysml-viz-svg")).toBeTruthy();
    expect(target.querySelector(".viz-root")).toBeTruthy();

    target.querySelector(".general-node-toggle")?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await vi.waitFor(() => expect(requestDraw).toHaveBeenCalledTimes(2));
    expect(requestDraw).toHaveBeenCalledTimes(2);
    const second = requestDraw.mock.calls[1]?.[2];
    expect(second?.disclosure.expandedNodeIds).toContain("n:0");

    target.querySelector("[data-node-id='n:0']")?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    expect(onNodeClick).toHaveBeenCalledWith(expect.objectContaining({ id: "n:0" }));

    expect(controller.exportSvg()).toContain("<svg");
    controller.destroy();
    expect(target.innerHTML).toBe("");
  });

    it("keeps the last SVG when a later draw request is declined", async () => {
      const target = host();
      const requestDraw = vi.fn<RequestServerDraw>()
        .mockResolvedValueOnce(SAMPLE_SVG)
        .mockResolvedValueOnce(null);
      await renderVisualization(
        target,
        { title: "General", view: "general-view", nodes: [{ id: "n:0", label: "Root", kind: "part" }], edges: [] },
        {
          theme: LIGHT_THEME,
          productIdentity: { modelDigest: "blake3:test", viewHandle: "general/root" },
          product: { schemaVersion: 7 },
          requestDraw,
        },
      );
      target.querySelector(".general-node-toggle")?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
      await vi.waitFor(() => expect(requestDraw).toHaveBeenCalledTimes(2));
      expect(target.querySelector("svg.sysml-viz-svg")).toBeTruthy();
    });

    it("does not let a previous controller clear a later mount on the same canvas", async () => {
      const target = host();
      const requestDraw = vi.fn<RequestServerDraw>(async () => SAMPLE_SVG);
      const options = {
        theme: LIGHT_THEME,
        productIdentity: { modelDigest: "blake3:test", viewHandle: "general/root" },
        product: { schemaVersion: 7 },
        requestDraw,
      };
      const prepared = {
        title: "General",
        view: "general-view" as const,
        nodes: [{ id: "n:0", label: "Root", kind: "part" }],
        edges: [],
      };
      const first = await renderVisualization(target, prepared, options);
      const second = await renderVisualization(target, prepared, options);
      expect(target.querySelector("svg.sysml-viz-svg")).toBeTruthy();
      first.destroy();
      expect(target.querySelector("svg.sysml-viz-svg")).toBeTruthy();
      second.destroy();
      expect(target.innerHTML).toBe("");
    });

    it("does not mutate the canvas when the host aborts before draw returns", async () => {
      const target = host();
      target.innerHTML = "<div id='keep'>keep</div>";
      const abort = new AbortController();
      const requestDraw = vi.fn<RequestServerDraw>(async (_identity, _revision, _request, signal) => {
        abort.abort();
        return await new Promise((resolve) => {
          if (signal.aborted) {
            resolve(null);
            return;
          }
          signal.addEventListener("abort", () => resolve(null), { once: true });
        });
      });
      const pending = renderVisualization(
        target,
        { title: "General", view: "general-view", nodes: [{ id: "n:0", label: "Root", kind: "part" }], edges: [] },
        {
          theme: LIGHT_THEME,
          productIdentity: { modelDigest: "blake3:test", viewHandle: "general/root" },
          abortSignal: abort.signal,
          product: { schemaVersion: 7 },
          requestDraw,
        },
      );
      const controller = await pending;
      expect(target.querySelector("#keep")?.textContent).toBe("keep");
      controller.destroy();
      expect(target.querySelector("#keep")?.textContent).toBe("keep");
    });
  });
