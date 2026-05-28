// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { renderVisualization } from "./renderer";

function boundsFor(target: HTMLElement, nodeId: string): { x: number; y: number; width: number; height: number } {
  const node = target.querySelector(`[data-node-id="${nodeId}"]`);
  expect(node).toBeTruthy();
  const raw = node?.getAttribute("data-bounds") ?? "";
  const [x, y, width, height] = raw.split(",").map(Number);
  return { x, y, width, height };
}

function expectInside(
  child: { x: number; y: number; width: number; height: number },
  parent: { x: number; y: number; width: number; height: number },
): void {
  expect(child.x).toBeGreaterThanOrEqual(parent.x);
  expect(child.y).toBeGreaterThanOrEqual(parent.y);
  expect(child.x + child.width).toBeLessThanOrEqual(parent.x + parent.width);
  expect(child.y + child.height).toBeLessThanOrEqual(parent.y + parent.height);
}

function paintIndex(target: HTMLElement, nodeId: string): number {
  const nodes = Array.from(target.querySelectorAll("[data-node-id]"));
  return nodes.findIndex((node) => node.getAttribute("data-node-id") === nodeId);
}

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
    expect(svg).toContain("sysml-header-compartment");
    expect(svg).toContain("Attributes");
    expect(svg).toContain("Ports");
    expect(svg).toContain("mass");

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

  it("renders real parent parts as interconnection containers", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    const controller = await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        { id: "drone", label: "SurveillanceDrone", kind: "part def" },
        { id: "flightController", label: "flightController", kind: "part", attributes: { containerId: "drone" } },
      ],
      edges: [],
    });

    const svg = controller.exportSvg();
    expect(svg).toContain("ibd-container");
    expect(svg).toContain("SurveillanceDrone");
  });

  it("keeps nested IBD parts spatially inside parent containers", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1600, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 1000, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        { id: "drone", label: "SurveillanceDrone", kind: "part def", attributes: { qualifiedName: "Pkg.SurveillanceDrone" } },
        {
          id: "flightController",
          label: "flightController",
          kind: "part",
          attributes: { qualifiedName: "Pkg.SurveillanceDrone.flightController", containerId: "drone", ports: ["gpsIn", "gimbalCmd"] },
        },
        {
          id: "gps",
          label: "gps",
          kind: "part",
          attributes: { qualifiedName: "Pkg.SurveillanceDrone.flightController.gps", containerId: "flightController", ports: ["positionOut"] },
        },
        {
          id: "gimbal",
          label: "gimbal",
          kind: "part",
          attributes: { qualifiedName: "Pkg.SurveillanceDrone.gimbal", containerId: "drone", ports: ["commandIn"] },
        },
      ],
      edges: [
        {
          id: "gps-fc",
          source: "gps",
          target: "flightController",
          label: "flow",
          edgeKind: "flow",
          attributes: { sourceId: "Pkg.SurveillanceDrone.flightController.gps.positionOut", targetId: "Pkg.SurveillanceDrone.flightController.gpsIn", relationType: "flow" },
        },
        {
          id: "fc-gimbal",
          source: "flightController",
          target: "gimbal",
          label: "connection",
          edgeKind: "connection",
          attributes: { sourceId: "Pkg.SurveillanceDrone.flightController.gimbalCmd", targetId: "Pkg.SurveillanceDrone.gimbal.commandIn", relationType: "connection" },
        },
      ],
    });

    const drone = boundsFor(target, "drone");
    const flightController = boundsFor(target, "flightController");
    const gps = boundsFor(target, "gps");
    const gimbal = boundsFor(target, "gimbal");

    expectInside(flightController, drone);
    expectInside(gimbal, drone);
    expectInside(gps, flightController);
    expect(paintIndex(target, "drone")).toBeLessThan(paintIndex(target, "flightController"));
    expect(paintIndex(target, "flightController")).toBeLessThan(paintIndex(target, "gps"));

    const rootChildren = Array.from(target.querySelector(".viz-root")?.children ?? []);
    const nodeLayerIndex = rootChildren.findIndex((node) => node.getAttribute("class") === "viz-nodes");
    const edgeLayerIndex = rootChildren.findIndex((node) => node.getAttribute("class") === "viz-edges");
    expect(nodeLayerIndex).toBeGreaterThanOrEqual(0);
    expect(edgeLayerIndex).toBeGreaterThan(nodeLayerIndex);
    expect(target.querySelectorAll(".ibd-connector")).toHaveLength(2);
    expect(target.querySelectorAll(".viz-edge-label")).toHaveLength(0);
  });

  it("uses connector direction to choose ambiguous IBD port sides", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        {
          id: "battery",
          label: "battery",
          kind: "part",
          attributes: { qualifiedName: "Drone.battery", portDetails: [{ id: "Drone.battery.pwr", name: "pwr" }] },
        },
        {
          id: "distribution",
          label: "distribution",
          kind: "part",
          attributes: { qualifiedName: "Drone.distribution", portDetails: [{ id: "Drone.distribution.mainPower", name: "mainPower" }] },
        },
      ],
      edges: [
        {
          id: "power",
          source: "battery",
          target: "distribution",
          label: "connection",
          edgeKind: "connection",
          attributes: { sourceId: "Drone.battery.pwr", targetId: "Drone.distribution.mainPower", relationType: "connection" },
        },
      ],
    });

    const battery = boundsFor(target, "battery");
    const distribution = boundsFor(target, "distribution");
    const batteryPort = target.querySelector('[data-node-id="battery"] .port-icon');
    const distributionPort = target.querySelector('[data-node-id="distribution"] .port-icon');
    expect(batteryPort).toBeTruthy();
    expect(distributionPort).toBeTruthy();
    const batteryPortX = Number(batteryPort?.getAttribute("x")) + 5;
    const distributionPortX = Number(distributionPort?.getAttribute("x")) + 5;

    expect(batteryPortX).toBeGreaterThan(battery.width - 1);
    expect(distributionPortX).toBeLessThan(1);
  });
});
