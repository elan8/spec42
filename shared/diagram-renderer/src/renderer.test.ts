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
        { id: "a", label: "A", kind: "part_def", attributes: { attributes: [{ name: "mass" }], ports: [{ name: "in" }] } },
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
    expect(svg).toContain("general-node");
    expect(svg).toContain("general-connector");
    expect(svg).toContain("attrs:1");
    expect(svg).toContain("ports:1");

    controller.reset();
    controller.destroy();
  });

  it("renders interconnection connectors and package containers with parity classes", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    const controller = await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        { id: "p1", label: "Engine", kind: "part", attributes: { ports: ["out"] } },
        { id: "p2", label: "Controller", kind: "part", attributes: { ports: ["in"] } },
      ],
      edges: [
        {
          id: "conn:engine-controller",
          source: "p1",
          target: "p2",
          label: "flow",
          attributes: { sourceId: "Engine.out", targetId: "Controller.in", relationType: "flow" },
        },
      ],
      meta: {
        packageContainerGroups: [{ id: "pkg1", name: "ConnectedBlocks", memberIds: ["p1", "p2"] }],
      },
    });

    const svg = controller.exportSvg();
    expect(svg).toContain("ibd-part");
    expect(svg).toContain("ibd-connector");
    expect(svg).toContain("data-connector-id=\"conn:engine-controller\"");
    expect(svg).toContain("ibd-container");
    expect(svg).toContain("ConnectedBlocks");
  });
});
