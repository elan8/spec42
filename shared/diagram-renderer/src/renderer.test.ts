// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { renderVisualization } from "./renderer";
import { resolveDiagramTheme, strokeColorForEdge, strokeColorForNode } from "./theme";

const LIGHT_THEME = { colorScheme: "light" as const };

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
  it("uses notation-neutral ink for all kinds", () => {
    const theme = resolveDiagramTheme({ colorScheme: "light" });
    expect(strokeColorForNode(theme)).toBe(theme.nodeBorder);
    expect(strokeColorForEdge("flow", theme)).toBe(theme.edge.default);
    expect(strokeColorForEdge("allocate", theme)).toBe(theme.edge.default);
    expect(theme.nodeBorder).not.toBe("#2D8A6E");
  });

  it("returns controller surface and SVG output", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 900, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 600, configurable: true });

    const controller = await renderVisualization(
      target,
      {
        title: "General",
        view: "general-view",
        nodes: [
          { id: "a", label: "A", kind: "part_def", attributes: { attributes: [{ name: "mass" }], ports: [{ name: "in" }] } },
          { id: "b", label: "B", kind: "part_def" },
        ],
        edges: [{ id: "e1", source: "a", target: "b", label: "typing", edgeKind: "typing" }],
      },
      { theme: LIGHT_THEME },
    );

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

  it("renders General view as SysML notation nodes with compartments", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(
      target,
      {
        title: "General",
        view: "general-view",
        nodes: [
        {
          id: "pkg",
          label: "Definitions",
          kind: "package",
          attributes: {
            packageMembers: ["Vehicle", "Engine"],
            imports: ["SysML::*"],
          },
        },
        {
          id: "vehicle-def",
          label: "Vehicle",
          kind: "part def",
          attributes: {
            attributes: ["mass: Mass"],
            parts: ["engine: Engine"],
            ports: ["pwr: PowerPort"],
            generalViewInheritedAttributes: ["^position"],
          },
        },
        {
          id: "vehicle-usage",
          label: "vehicle",
          kind: "part",
          attributes: {
            partType: "Vehicle",
            attributes: ["dryMass = 42 [kg]"],
          },
        },
      ],
      edges: [
        { id: "owns", source: "pkg", target: "vehicle-def", label: "owns", edgeKind: "hierarchy", attributes: { relationType: "hierarchy" } },
        { id: "typed", source: "vehicle-usage", target: "vehicle-def", label: "defined by", edgeKind: "typing", attributes: { relationType: "typing" } },
      ],
      },
      { theme: LIGHT_THEME },
    );

    expect(target.textContent).not.toContain("«package»");
    expect(target.textContent).toContain("«part def»");
    expect(target.textContent).toContain("«part»");
    expect(target.textContent).toContain(": Vehicle");
    expect(target.textContent).toContain("Attributes");
    expect(target.textContent).toContain("Parts");
    expect(target.textContent).toContain("Ports");
    expect(target.textContent).toContain("> Inherited Attributes");
    const definitionBg = target.querySelector('[data-node-id="vehicle-def"] .sysml-node-bg') as SVGRectElement | null;
    const usageBg = target.querySelector('[data-node-id="vehicle-usage"] .sysml-node-bg') as SVGRectElement | null;
    expect(definitionBg?.style.strokeDasharray).toBe("none");
    expect(usageBg?.style.strokeDasharray).toBe("none");
    expect(definitionBg?.getAttribute("rx")).toBe("0");
    expect(usageBg?.getAttribute("rx")).toBe("8");
    const theme = resolveDiagramTheme({ colorScheme: "light" });
    expect(definitionBg?.style.stroke).toBe(theme.nodeBorder);
    expect(definitionBg?.style.stroke).not.toBe("#2D8A6E");
  });

  it("renders General view relationship notation and suppresses generic labels", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1800, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 1200, configurable: true });

    await renderVisualization(
      target,
      {
        title: "General",
        view: "general-view",
        nodes: [
        { id: "a", label: "A", kind: "part def" },
        { id: "b", label: "B", kind: "part def" },
        { id: "c", label: "C", kind: "part" },
        { id: "d", label: "D", kind: "requirement" },
        { id: "e", label: "E", kind: "action" },
      ],
      edges: [
        { id: "specializes", source: "a", target: "b", label: "specializes", edgeKind: "specializes", attributes: { relationType: "specializes" } },
        { id: "typing", source: "c", target: "a", label: "typing", edgeKind: "typing", attributes: { relationType: "typing" } },
        { id: "hierarchy", source: "a", target: "c", label: "contains", edgeKind: "hierarchy", attributes: { relationType: "hierarchy" } },
        { id: "dependency", source: "e", target: "a", label: "riskImpact", edgeKind: "dependency", attributes: { relationType: "dependency" } },
        { id: "allocation", source: "e", target: "c", label: "allocation", edgeKind: "allocate", attributes: { relationType: "allocate" } },
        { id: "satisfy", source: "a", target: "d", label: "satisfy", edgeKind: "satisfy", attributes: { relationType: "satisfy" } },
        { id: "verify", source: "e", target: "d", label: "verify", edgeKind: "verify", attributes: { relationType: "verify" } },
        { id: "binding", source: "b", target: "d", label: "binding", edgeKind: "bind", attributes: { relationType: "binding" } },
        { id: "generic", source: "b", target: "e", label: "relationship", edgeKind: "relationship", attributes: { relationType: "relationship" } },
      ],
      },
      { theme: LIGHT_THEME },
    );

    const labels = Array.from(target.querySelectorAll(".viz-edge-label")).map((node) => node.textContent);
    expect(labels).toEqual(["riskImpact"]);
    expect((target.querySelector('[data-connector-id="specializes"]') as SVGPathElement | null)?.style.markerEnd).toContain("general-d3-specializes");
    expect((target.querySelector('[data-connector-id="typing"]') as SVGPathElement | null)?.style.markerEnd).toContain("general-d3-arrow-open");
    expect((target.querySelector('[data-connector-id="typing"]') as SVGPathElement | null)?.style.strokeDasharray).toBe("5,3");
    expect((target.querySelector('[data-connector-id="hierarchy"]') as SVGPathElement | null)?.style.markerStart).toContain("general-d3-diamond");
    expect((target.querySelector('[data-connector-id="dependency"]') as SVGPathElement | null)?.style.strokeDasharray).toBe("4,4");
    expect((target.querySelector('[data-connector-id="allocation"]') as SVGPathElement | null)?.style.strokeDasharray).toBe("8,4");
    expect((target.querySelector('[data-connector-id="satisfy"]') as SVGPathElement | null)?.style.strokeDasharray).toBe("7,4");
    expect((target.querySelector('[data-connector-id="verify"]') as SVGPathElement | null)?.style.strokeDasharray).toBe("7,4");
    expect((target.querySelector('[data-connector-id="binding"]') as SVGPathElement | null)?.style.strokeDasharray).toBe("2,2");
  });

  it("applies custom theme overrides to nodes and edges", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 900, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 600, configurable: true });

    await renderVisualization(
      target,
      {
        title: "General",
        view: "general-view",
        nodes: [
          { id: "a", label: "A", kind: "part def" },
          { id: "b", label: "B", kind: "requirement" },
        ],
        edges: [{ id: "satisfy", source: "a", target: "b", label: "satisfy", edgeKind: "satisfy", attributes: { relationType: "satisfy" } }],
      },
      {
        theme: {
          colorScheme: "light",
          nodeBorder: "#123456",
          edge: { default: "#abcdef" },
          highlight: "#fedcba",
        },
      },
    );

    const partStereotype = target.querySelector('[data-node-id="a"] text');
    const edge = target.querySelector('[data-connector-id="satisfy"]') as SVGPathElement | null;
    expect((partStereotype as SVGTextElement | null)?.style.fill).toBe("#123456");
    expect(edge?.getAttribute("stroke")).toBe("#abcdef");
  });

  it("renders light and dark schemes with distinct strokes", async () => {
    const lightTarget = document.createElement("div");
    const darkTarget = document.createElement("div");
    for (const el of [lightTarget, darkTarget]) {
      Object.defineProperty(el, "clientWidth", { value: 900, configurable: true });
      Object.defineProperty(el, "clientHeight", { value: 600, configurable: true });
    }
    const payload = {
      title: "General",
      view: "general-view" as const,
      nodes: [{ id: "a", label: "A", kind: "part def" }],
      edges: [] as const,
    };
    await renderVisualization(lightTarget, payload, { theme: { colorScheme: "light" } });
    await renderVisualization(darkTarget, payload, { theme: { colorScheme: "dark" } });
    const lightStroke = (lightTarget.querySelector(".sysml-node-bg") as SVGRectElement | null)?.style.stroke;
    const darkStroke = (darkTarget.querySelector(".sysml-node-bg") as SVGRectElement | null)?.style.stroke;
    expect(lightStroke).toBeTruthy();
    expect(darkStroke).toBeTruthy();
    expect(lightStroke).not.toBe(darkStroke);
    expect(lightTarget.querySelector("svg")?.getAttribute("data-color-scheme")).toBe("light");
    expect(darkTarget.querySelector("svg")?.getAttribute("data-color-scheme")).toBe("dark");
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

  it("renders nested interconnection with connectors after leaf node chrome", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1600, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 1000, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        {
          id: "features",
          label: "drone features",
          kind: "package",
          attributes: { isSyntheticContainer: true, isPackageContainer: true },
        },
        {
          id: "drone",
          label: "SurveillanceDrone",
          kind: "part def",
          attributes: { containerId: "features", ports: ["pwrIn"] },
        },
        {
          id: "instance",
          label: "droneInstance",
          kind: "part",
          attributes: { containerId: "drone", partType: "SurveillanceDrone", ports: ["mainPwr"] },
        },
        {
          id: "power",
          label: "power",
          kind: "part",
          attributes: { containerId: "instance", ports: ["pwrOut"] },
        },
        {
          id: "flight",
          label: "FlightControl",
          kind: "part",
          attributes: { containerId: "instance", ports: ["pwrIn", "cmdOut"] },
        },
        {
          id: "propulsion",
          label: "propulsion",
          kind: "part",
          attributes: { containerId: "instance", ports: ["pwrIn"] },
        },
      ],
      edges: [
        {
          id: "pwr-flow",
          source: "power",
          target: "flight",
          label: "flow",
          edgeKind: "flow",
          attributes: { sourceId: "power.pwrOut", targetId: "flight.pwrIn", relationType: "flow" },
        },
        {
          id: "prop-conn",
          source: "flight",
          target: "propulsion",
          label: "connection",
          edgeKind: "connection",
          attributes: { sourceId: "flight.cmdOut", targetId: "propulsion.pwrIn", relationType: "connection" },
        },
      ],
    });

    expect(target.querySelectorAll(".ibd-connector").length).toBeGreaterThanOrEqual(2);
    expect(target.querySelectorAll(".port-icon").length).toBeGreaterThan(0);
    const flightBg = target.querySelector('[data-node-id="flight"] .graph-node-background') as SVGRectElement | null;
    expect(flightBg?.style.strokeDasharray).toBe("none");
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
    expect(batteryPort?.getAttribute("data-port-side")).toBe("EAST");
    expect(distributionPort?.getAttribute("data-port-side")).toBe("WEST");
  });

  it("renders SysML IBD edge kinds with distinct notation styles and meaningful labels", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1600, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 1000, configurable: true });

    await renderVisualization(target, {
      title: "DroneInterconnection",
      view: "interconnection-view",
      nodes: [
        { id: "battery", label: "battery", kind: "part", attributes: { portDetails: [{ id: "Drone.battery.pwr", name: "pwr", portType: "PowerPort" }] } },
        { id: "distribution", label: "distribution", kind: "part", attributes: { portDetails: [{ id: "Drone.distribution.mainPower", name: "mainPower", portType: "~PowerPort" }] } },
        { id: "controller", label: "controller", kind: "part", attributes: { portDetails: [{ id: "Drone.controller.cmd", name: "cmd" }] } },
        { id: "motor", label: "motor", kind: "part", attributes: { portDetails: [{ id: "Drone.motor.cmd", name: "cmd" }] } },
        { id: "sensor", label: "sensor", kind: "part" },
      ],
      edges: [
        {
          id: "connection-edge",
          source: "battery",
          target: "distribution",
          label: "connection",
          edgeKind: "connection",
          attributes: { sourceId: "Drone.battery.pwr", targetId: "Drone.distribution.mainPower", relationType: "connection" },
        },
        {
          id: "flow-edge",
          source: "distribution",
          target: "controller",
          label: "Power",
          edgeKind: "flow",
          attributes: { sourceId: "Drone.distribution.mainPower", targetId: "Drone.controller.cmd", relationType: "flow", itemType: "Power" },
        },
        {
          id: "interface-edge",
          source: "controller",
          target: "motor",
          label: "MotorControl",
          edgeKind: "interface",
          attributes: { sourceId: "Drone.controller.cmd", targetId: "Drone.motor.cmd", relationType: "interface", interfaceName: "MotorControl" },
        },
        {
          id: "binding-edge",
          source: "controller",
          target: "sensor",
          label: "binding",
          edgeKind: "bind",
          attributes: { relationType: "binding" },
        },
        {
          id: "reference-edge",
          source: "sensor",
          target: "motor",
          label: "reference",
          edgeKind: "reference",
          attributes: { relationType: "reference" },
        },
      ],
      meta: { selectedRoot: "DroneInterconnection" },
    });

    expect(target.querySelector(".ibd-view-frame")?.getAttribute("data-view-name")).toBe("DroneInterconnection");
    const labels = Array.from(target.querySelectorAll(".viz-edge-label")).map((node) => node.textContent);
    expect(labels).toContain("Power");
    expect(labels).toContain("MotorControl");
    expect(labels).not.toContain("connection");
    expect(labels).not.toContain("binding");
    expect(labels).not.toContain("reference");

    const connection = target.querySelector('[data-connector-id="connection-edge"]') as SVGPathElement | null;
    const flow = target.querySelector('[data-connector-id="flow-edge"]') as SVGPathElement | null;
    const iface = target.querySelector('[data-connector-id="interface-edge"]') as SVGPathElement | null;
    const binding = target.querySelector('[data-connector-id="binding-edge"]') as SVGPathElement | null;
    const reference = target.querySelector('[data-connector-id="reference-edge"]') as SVGPathElement | null;
    expect(connection?.style.markerEnd).toContain("ibd-connection-dot");
    expect(flow?.style.markerEnd).toContain("ibd-flow-arrow");
    expect(iface?.style.markerEnd).toContain("ibd-interface-arrow");
    expect(binding?.style.strokeDasharray).toBe("6,4");
    expect(reference?.style.strokeDasharray).toBe("4,4");
    expect(target.textContent).toContain("pwr: PowerPort");
    expect(target.textContent).toContain("mainPower: ~PowerPort");
  });

  it("renders direct part-to-part IBD connections when ports are absent", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        { id: "airframe", label: "airframe", kind: "part" },
        { id: "payload", label: "payload", kind: "part" },
      ],
      edges: [
        { id: "mount", source: "airframe", target: "payload", label: "mountedTo", edgeKind: "connection", attributes: { relationType: "connection" } },
      ],
    });

    const connector = target.querySelector('[data-connector-id="mount"]');
    expect(connector).toBeTruthy();
    expect(connector?.getAttribute("d")).toMatch(/^M/);
  });
});
