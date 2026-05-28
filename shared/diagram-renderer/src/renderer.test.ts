// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { renderVisualization } from "./renderer";

describe("shared renderer", () => {
  it("returns controller surface and SVG output", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 900, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 600, configurable: true });

    const controller = await renderVisualization(target, {
      title: "General",
      view: "general-view",
      nodes: [
        { id: "a", label: "A", kind: "part_def" },
        { id: "b", label: "B", kind: "part_def" },
      ],
      edges: [{ id: "e1", source: "a", target: "b", label: "typing", edgeKind: "typing" }],
    });

    expect(typeof controller.reset).toBe("function");
    expect(typeof controller.exportSvg).toBe("function");
    expect(typeof controller.destroy).toBe("function");

    const svg = controller.exportSvg();
    expect(svg).toContain("<svg");
    expect(svg).toContain("viewBox");

    controller.reset();
    controller.destroy();
  });
});
