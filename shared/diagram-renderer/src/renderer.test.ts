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

function expectFiniteRootTransform(target: HTMLElement): void {
  const transform = target.querySelector("g.viz-root")?.getAttribute("transform") ?? "";
  expect(transform).toMatch(/translate\(/);
  expect(transform).not.toMatch(/NaN/);
  const scaleMatch = transform.match(/scale\(([-\d.eE+]+)\)/);
  if (scaleMatch) {
    const scale = Number(scaleMatch[1]);
    expect(Number.isFinite(scale)).toBe(true);
    expect(scale).toBeGreaterThan(0);
  }
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

  it("renders IBD connectors with visible stroke in dark colorScheme", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(
      target,
      {
        title: "Interconnection",
        view: "interconnection-view",
        nodes: [
          { id: "a", label: "A", kind: "part", attributes: { ports: ["out"] } },
          { id: "b", label: "B", kind: "part", attributes: { ports: ["in"] } },
        ],
        edges: [
          {
            id: "c1",
            source: "a",
            target: "b",
            label: "connection",
            edgeKind: "connection",
            attributes: { relationType: "connection" },
          },
        ],
      },
      { theme: { colorScheme: "dark" } },
    );

    const connector = target.querySelector(".ibd-connector") as SVGPathElement | null;
    expect(connector).toBeTruthy();
    expect(connector?.getAttribute("d")).toBeTruthy();
    expect(connector?.style.stroke).toBeTruthy();
    expect(connector?.style.stroke).not.toBe("none");
    expect(target.querySelector("svg")?.getAttribute("data-color-scheme")).toBe("dark");
  });

  it("renders parts tree reference usage with dotted chrome", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "General",
      view: "general-view",
      nodes: [
        { id: "def", label: "HitchBall", kind: "part def" },
        { id: "ref", label: "hitchBall", kind: "ref", attributes: { isReference: true } },
        { id: "usage", label: "mount", kind: "part" },
      ],
      edges: [],
    });

    const defBg = target.querySelector('[data-node-id="def"] .sysml-node-bg') as SVGRectElement | null;
    const refBg = target.querySelector('[data-node-id="ref"] .sysml-node-bg') as SVGRectElement | null;
    const usageBg = target.querySelector('[data-node-id="usage"] .sysml-node-bg') as SVGRectElement | null;
    expect(defBg?.style.strokeDasharray).toBe("none");
    expect(refBg?.style.strokeDasharray).toBe("2,4");
    expect(usageBg?.style.strokeDasharray).toBe("none");
    expect(target.querySelector('[data-node-id="ref"]')?.classList.contains("viz-node--reference")).toBe(true);
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
      edges: [],
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

  it("skips redundant view frame when scoped root is already a layout container", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1600, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 1000, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      meta: { selectedRoot: "droneInstance" },
      nodes: [
        {
          id: "instance",
          label: "droneInstance",
          kind: "part",
          attributes: { isDiagramRoot: true, ports: ["mainPwr"] },
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
      ],
    });

    expect(target.querySelector(".ibd-view-frame")).toBeNull();
    expect(target.querySelectorAll(".ibd-container").length).toBeGreaterThanOrEqual(1);
    expect(target.querySelector('[data-element-name="droneInstance"]')).toBeTruthy();
  });

  it("routes nested container connectors without orphan segments", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        {
          id: "webshopSystem",
          label: "webshopSystem",
          kind: "part",
          attributes: { isSyntheticContainer: true },
        },
        {
          id: "storefront",
          label: "storefront",
          kind: "part",
          attributes: {
            containerId: "webshopSystem",
            portDetails: [
              { id: "occ:webshopSystem.storefront.apiOut", name: "apiOut", direction: "out", attributes: { scenePortId: "occ:webshopSystem.storefront.apiOut" } },
              { id: "occ:webshopSystem.storefront.catalogIn", name: "catalogIn", direction: "in", attributes: { scenePortId: "occ:webshopSystem.storefront.catalogIn" } },
            ],
          },
        },
        {
          id: "apiGateway",
          label: "apiGateway",
          kind: "part",
          attributes: {
            containerId: "webshopSystem",
            portDetails: [
              { id: "occ:webshopSystem.apiGateway.storefrontIn", name: "storefrontIn", direction: "in", attributes: { scenePortId: "occ:webshopSystem.apiGateway.storefrontIn" } },
              { id: "occ:webshopSystem.apiGateway.checkoutOut", name: "checkoutOut", direction: "out", attributes: { scenePortId: "occ:webshopSystem.apiGateway.checkoutOut" } },
            ],
          },
        },
        {
          id: "checkoutService",
          label: "checkoutService",
          kind: "part",
          attributes: {
            containerId: "webshopSystem",
            portDetails: [
              { id: "occ:webshopSystem.checkoutService.apiIn", name: "apiIn", direction: "in", attributes: { scenePortId: "occ:webshopSystem.checkoutService.apiIn" } },
              { id: "occ:webshopSystem.checkoutService.ordersOut", name: "ordersOut", direction: "out", attributes: { scenePortId: "occ:webshopSystem.checkoutService.ordersOut" } },
            ],
          },
        },
        {
          id: "ordersEventsTopic",
          label: "ordersEventsTopic",
          kind: "part",
          attributes: {
            containerId: "webshopSystem",
            portDetails: [{ id: "occ:webshopSystem.ordersEventsTopic.ordersIn", name: "ordersIn", direction: "in", attributes: { scenePortId: "occ:webshopSystem.ordersEventsTopic.ordersIn" } }],
          },
        },
      ],
      edges: [
        {
          id: "storefront-api",
          source: "storefront",
          target: "apiGateway",
          label: "connection",
          edgeKind: "connection",
          attributes: {
            sourcePortId: "occ:webshopSystem.storefront.apiOut",
            targetPortId: "occ:webshopSystem.apiGateway.storefrontIn",
            relationType: "connection",
          },
        },
        {
          id: "api-checkout",
          source: "apiGateway",
          target: "checkoutService",
          label: "connection",
          edgeKind: "connection",
          attributes: {
            sourcePortId: "occ:webshopSystem.apiGateway.checkoutOut",
            targetPortId: "occ:webshopSystem.checkoutService.apiIn",
            relationType: "connection",
          },
        },
        {
          id: "checkout-orders",
          source: "checkoutService",
          target: "ordersEventsTopic",
          label: "connection",
          edgeKind: "connection",
          attributes: {
            sourcePortId: "occ:webshopSystem.checkoutService.ordersOut",
            targetPortId: "occ:webshopSystem.ordersEventsTopic.ordersIn",
            relationType: "connection",
          },
        },
      ],
    });

    const parsePathPoints = (path: string): Array<{ x: number; y: number }> => {
      const tokens = path.trim().split(/[ML]/).map((part) => part.trim()).filter(Boolean);
      return tokens.map((token) => {
        const [x, y] = token.split(/[,\s]+/).map(Number);
        return { x, y };
      });
    };

    const portCenter = (nodeId: string, portName: string) => {
      const icon = target.querySelector(`[data-node-id="${nodeId}"] [data-port-name="${portName}"]`) as SVGRectElement | null;
      expect(icon).toBeTruthy();
      const node = boundsFor(target, nodeId);
      return {
        x: node.x + Number(icon?.getAttribute("x") ?? 0) + 5,
        y: node.y + Number(icon?.getAttribute("y") ?? 0) + 5,
      };
    };

    const connectors = Array.from(target.querySelectorAll(".ibd-connector")) as SVGPathElement[];
    expect(connectors.length).toBe(3);
    for (const connector of connectors) {
      const points = parsePathPoints(connector.getAttribute("d") ?? "");
      expect(points.length).toBeGreaterThanOrEqual(2);
      for (let index = 1; index < points.length; index += 1) {
        const prev = points[index - 1];
        const current = points[index];
        expect(Math.abs(prev.x - current.x) < 1e-3 || Math.abs(prev.y - current.y) < 1e-3).toBe(true);
      }
    }

    const storefrontApi = target.querySelector('[data-connector-id="storefront-api"]') as SVGPathElement | null;
    const apiCheckout = target.querySelector('[data-connector-id="api-checkout"]') as SVGPathElement | null;
    const checkoutOrders = target.querySelector('[data-connector-id="checkout-orders"]') as SVGPathElement | null;
    for (const [pathEl, sourceId, sourcePort, targetId, targetPort] of [
      [storefrontApi, "storefront", "apiOut", "apiGateway", "storefrontIn"],
      [apiCheckout, "apiGateway", "checkoutOut", "checkoutService", "apiIn"],
      [checkoutOrders, "checkoutService", "ordersOut", "ordersEventsTopic", "ordersIn"],
    ] as const) {
      const points = parsePathPoints(pathEl?.getAttribute("d") ?? "");
      const sourceCenter = portCenter(sourceId, sourcePort);
      const targetCenter = portCenter(targetId, targetPort);
      expect(Math.hypot(points[0].x - sourceCenter.x, points[0].y - sourceCenter.y)).toBeLessThan(4);
      expect(Math.hypot(points[points.length - 1].x - targetCenter.x, points[points.length - 1].y - targetCenter.y)).toBeLessThan(4);
    }
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

  it("renders context connectors when ELK omits edge sections for nested containers", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection View",
      view: "interconnection-view",
      nodes: [
        {
          id: "architecture",
          label: "RegionalGridArchitecture",
          kind: "package",
          attributes: { isSyntheticContainer: true },
        },
        {
          id: "primarySubstation",
          label: "primarySubstation",
          kind: "part",
          attributes: {
            containerId: "architecture",
            qualifiedName: "RegionalGridExpansion.architecture.primarySubstation",
            portDetails: [{ id: "RegionalGridExpansion.architecture.primarySubstation.hvConnection", name: "hvConnection" }],
          },
        },
        {
          id: "transformer",
          label: "transformer",
          kind: "part",
          attributes: {
            containerId: "primarySubstation",
            qualifiedName: "RegionalGridExpansion.architecture.primarySubstation.transformer",
          },
        },
        {
          id: "tennetConnection",
          label: "tennetConnection",
          kind: "part",
          attributes: {
            containerId: "architecture",
            qualifiedName: "RegionalGridExpansion.architecture.tennetConnection",
            portDetails: [{ id: "RegionalGridExpansion.architecture.tennetConnection.connection", name: "connection" }],
          },
        },
      ],
      edges: [
        {
          id: "hv-connection",
          source: "tennetConnection",
          target: "primarySubstation",
          label: "connection",
          edgeKind: "connection",
          attributes: {
            sourceId: "RegionalGridExpansion.architecture.tennetConnection.connection",
            targetId: "RegionalGridExpansion.architecture.primarySubstation.hvConnection",
            relationType: "connection",
          },
        },
      ],
    });

    const connector = target.querySelector('[data-connector-id="hv-connection"]') as SVGPathElement | null;
    expect(connector).toBeTruthy();
    expect(connector?.getAttribute("d")).toMatch(/^M/);
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

  it("aligns IBD connector endpoints with rendered port anchors", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        {
          id: "source",
          label: "source",
          kind: "part",
          attributes: {
            qualifiedName: "System.source",
            portDetails: [
              { id: "System.source.outA", name: "outA", direction: "out" },
              { id: "System.source.outB", name: "outB", direction: "out" },
            ],
          },
        },
        {
          id: "sink",
          label: "sink",
          kind: "part",
          attributes: {
            qualifiedName: "System.sink",
            portDetails: [
              { id: "System.sink.inA", name: "inA", direction: "in" },
              { id: "System.sink.inB", name: "inB", direction: "in" },
            ],
          },
        },
      ],
      edges: [
        {
          id: "edge-a",
          source: "source",
          target: "sink",
          label: "connection",
          edgeKind: "connection",
          attributes: { sourceId: "System.source.outA", targetId: "System.sink.inA", relationType: "connection" },
        },
        {
          id: "edge-b",
          source: "source",
          target: "sink",
          label: "connection",
          edgeKind: "connection",
          attributes: { sourceId: "System.source.outB", targetId: "System.sink.inB", relationType: "connection" },
        },
      ],
    });

    const parsePathPoints = (path: string): Array<{ x: number; y: number }> => {
      const tokens = path.trim().split(/[ML]/).map((part) => part.trim()).filter(Boolean);
      return tokens.map((token) => {
        const [x, y] = token.split(/[,\s]+/).map(Number);
        return { x, y };
      });
    };

    const portCenter = (nodeId: string, portName: string) => {
      const icon = target.querySelector(`[data-node-id="${nodeId}"] [data-port-name="${portName}"]`) as SVGRectElement | null;
      expect(icon).toBeTruthy();
      const node = boundsFor(target, nodeId);
      return {
        x: node.x + Number(icon?.getAttribute("x") ?? 0) + 5,
        y: node.y + Number(icon?.getAttribute("y") ?? 0) + 5,
      };
    };

    for (const edgeId of ["edge-a", "edge-b"]) {
      const connector = target.querySelector(`[data-connector-id="${edgeId}"]`) as SVGPathElement | null;
      expect(connector?.getAttribute("d")).toBeTruthy();
      const points = parsePathPoints(connector?.getAttribute("d") ?? "");
      expect(points.length).toBeGreaterThanOrEqual(2);
      const start = points[0];
      const end = points[points.length - 1];
      const sourcePort = edgeId === "edge-a" ? "outA" : "outB";
      const targetPort = edgeId === "edge-a" ? "inA" : "inB";
      const sourceCenter = portCenter("source", sourcePort);
      const targetCenter = portCenter("sink", targetPort);
      expect(Math.hypot(start.x - sourceCenter.x, start.y - sourceCenter.y)).toBeLessThan(4);
      expect(Math.hypot(end.x - targetCenter.x, end.y - targetCenter.y)).toBeLessThan(4);
    }
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

  it("matches qualified connector endpoints to local IBD port ids when choosing sides", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        {
          id: "source",
          label: "source",
          kind: "part",
          attributes: { qualifiedName: "System.source", portDetails: [{ id: "out", name: "out" }] },
        },
        {
          id: "sink",
          label: "sink",
          kind: "part",
          attributes: { qualifiedName: "System.sink", portDetails: [{ id: "gridConnection", name: "gridConnection" }] },
        },
      ],
      edges: [
        {
          id: "connection",
          source: "source",
          target: "sink",
          label: "connection",
          edgeKind: "connection",
          attributes: {
            sourceId: "System.source.out",
            targetId: "System.sink.gridConnection",
            relationType: "connection",
          },
        },
      ],
    });

    const sourcePort = target.querySelector('[data-node-id="source"] .port-icon');
    const sinkPort = target.querySelector('[data-node-id="sink"] .port-icon');

    expect(sourcePort?.getAttribute("data-port-side")).toBe("EAST");
    expect(sinkPort?.getAttribute("data-port-side")).toBe("WEST");
  });

  it("matches IBD port usage when endpoint and part qualified prefixes differ", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Interconnection",
      view: "interconnection-view",
      nodes: [
        {
          id: "txStationB",
          label: "txStationB",
          kind: "part",
          attributes: {
            qualifiedName: "RegionalGridExpansion.Architecture.RegionalGridArchitecture.txStationB",
            portDetails: [{ id: "lvConnection", name: "lvConnection" }],
          },
        },
        {
          id: "residentialAreaB",
          label: "residentialAreaB",
          kind: "part",
          attributes: {
            qualifiedName: "RegionalGridExpansion.Architecture.RegionalGridArchitecture.residentialAreaB",
            portDetails: [{ id: "gridConnection", name: "gridConnection" }],
          },
        },
      ],
      edges: [
        {
          id: "lv",
          source: "txStationB",
          target: "residentialAreaB",
          label: "connection",
          edgeKind: "connection",
          attributes: {
            sourceId: "RegionalGridExpansion.regionalExpansionProject.architecture.txStationB.lvConnection",
            targetId: "RegionalGridExpansion.regionalExpansionProject.architecture.residentialAreaB.gridConnection",
            relationType: "connection",
          },
        },
      ],
    });

    const txPort = target.querySelector('[data-node-id="txStationB"] .port-icon');
    const loadPort = target.querySelector('[data-node-id="residentialAreaB"] .port-icon');

    expect(txPort?.getAttribute("data-port-side")).toBe("EAST");
    expect(loadPort?.getAttribute("data-port-side")).toBe("WEST");
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

  it("renders action-flow view with activity nodes and flows", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(
      target,
      {
        title: "Robot Flow",
        view: "action-flow-view",
        nodes: [
          { id: "start", label: "start", kind: "initial" },
          { id: "move", label: "move", kind: "action" },
          { id: "done", label: "done", kind: "final" },
        ],
        edges: [
          { id: "f1", source: "start", target: "move", label: "" },
          { id: "f2", source: "move", target: "done", label: "complete" },
        ],
      },
      { delegateZoom: true },
    );

    expectFiniteRootTransform(target);
    expect(target.querySelectorAll(".action-flow-node").length).toBe(3);
    expect(target.querySelectorAll(".action-flow-edge").length).toBeGreaterThanOrEqual(2);
  });

  it("renders action-flow perform actions with parameter badges and flow final notation", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Perform Flow",
      view: "action-flow-view",
      nodes: [
        { id: "start", label: "start", kind: "initial" },
        {
          id: "perform",
          label: "authorizePayment",
          kind: "perform",
          attributes: { inputs: [{ name: "cart" }], outputs: [{ name: "receipt" }] },
        },
        { id: "end", label: "flowDone", kind: "flow-final" },
      ],
      edges: [
        { id: "f1", source: "start", target: "perform", label: "" },
        { id: "f2", source: "perform", target: "end", label: "" },
      ],
    });

    expect(target.querySelector(".perform-action-node")).toBeTruthy();
    expect(target.querySelectorAll(".action-parameter-badge").length).toBe(2);
    expect(target.querySelector(".flow-final-x")).toBeTruthy();
    expect(target.textContent).toContain("perform");
    expect(target.textContent).toContain("cart");
    expect(target.textContent).toContain("receipt");
  });

  it("renders action-flow connectors for qualified node ids with simple flow names", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(target, {
      title: "CheckoutPipeline",
      view: "action-flow-view",
      nodes: [
        { id: "WebShopBehavior::CheckoutPipeline::validateCart", label: "validateCart", kind: "action" },
        { id: "WebShopBehavior::CheckoutPipeline::authorizePayment", label: "authorizePayment", kind: "action" },
        { id: "WebShopBehavior::CheckoutPipeline::reserveInventory", label: "reserveInventory", kind: "action" },
      ],
      edges: [
        {
          id: "f1",
          source: "WebShopBehavior::CheckoutPipeline::validateCart",
          target: "WebShopBehavior::CheckoutPipeline::authorizePayment",
          label: "",
        },
        {
          id: "f2",
          source: "WebShopBehavior::CheckoutPipeline::authorizePayment",
          target: "WebShopBehavior::CheckoutPipeline::reserveInventory",
          label: "",
        },
      ],
    });

    expect(target.querySelectorAll(".action-flow-edge").length).toBe(2);
  });

  it("renders action-flow decision diamond and conditional succession labels", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Decision Flow",
      view: "action-flow-view",
      nodes: [
        { id: "start", label: "start", kind: "initial" },
        { id: "route", label: "validate", kind: "action" },
        { id: "check", label: "checkRoute", kind: "decision" },
        { id: "done", label: "done", kind: "final" },
      ],
      edges: [
        { id: "f1", source: "start", target: "route", label: "" },
        {
          id: "f2",
          source: "route",
          target: "check",
          label: "status == ok",
          attributes: { guard: "succession", succession: true, conditional: true },
        },
        { id: "f3", source: "check", target: "done", label: "" },
      ],
    });

    expect(target.querySelector("path.node-background[d]")).toBeTruthy();
    expect(target.querySelector(".aflow-conditional")).toBeTruthy();
    expect(target.textContent).toContain("[status == ok]");
  });

  it("renders state-transition view with pseudostates", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(
      target,
      {
        title: "Mode State Machine",
        view: "state-transition-view",
        nodes: [
          { id: "i", label: "Initial", kind: "initial" },
          { id: "idle", label: "idle", kind: "state" },
          { id: "f", label: "Final", kind: "final" },
        ],
        edges: [
          { id: "t1", source: "i", target: "idle", label: "" },
          { id: "t2", source: "idle", target: "f", label: "shutdown" },
        ],
      },
      { delegateZoom: true },
    );

    expectFiniteRootTransform(target);
    expect(target.querySelectorAll(".state-transition-node").length).toBe(3);
    expect(target.querySelectorAll(".state-transition-edge").length).toBeGreaterThanOrEqual(2);
  });

  it("renders composite state regions and entry do exit compartments", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(target, {
      title: "Composite",
      view: "state-transition-view",
      nodes: [
        {
          id: "operating",
          label: "Operating",
          kind: "composite state",
          attributes: {
            entry: "arm",
            doAction: "monitor",
            exit: "safe",
            regions: [{ name: "nominal" }, { name: "fault" }],
          },
        },
        { id: "terminate", label: "Terminate", kind: "terminate" },
      ],
      edges: [{ id: "self", source: "operating", target: "operating", label: "retry", attributes: { selfLoop: true } }],
    });

    expect(target.querySelectorAll(".state-region").length).toBe(2);
    expect(target.querySelectorAll(".state-action-compartment").length).toBe(3);
    expect(target.querySelector(".state-transition-edge")?.getAttribute("d")).toContain("C");
    expect(target.textContent).toContain("entry / arm");
    expect(target.textContent).toContain("do / monitor");
    expect(target.textContent).toContain("exit / safe");
    expect(target.querySelector(".terminate-state-x")).toBeTruthy();
  });

  it("renders state-transition guard effect accept send labels", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "Transitions",
      view: "state-transition-view",
      nodes: [
        { id: "idle", label: "idle", kind: "state" },
        { id: "running", label: "running", kind: "state" },
      ],
      edges: [
        {
          id: "t1",
          source: "idle",
          target: "running",
          label: "[armed] / start / accept StartPressed / send Notification",
          attributes: {
            guard: "armed",
            effect: "start",
            accept: "StartPressed",
            send: "Notification",
          },
        },
      ],
    });

    expect(target.textContent).toContain("[armed]");
    expect(target.textContent).toContain("accept StartPressed");
    expect(target.textContent).toContain("send Notification");
  });

  it("highlights action-flow nodes on click when onNodeClick is wired", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    const clicked: string[] = [];
    await renderVisualization(
      target,
      {
        title: "Robot Flow",
        view: "action-flow-view",
        nodes: [
          { id: "start", label: "start", kind: "initial" },
          { id: "move", label: "move", kind: "action" },
        ],
        edges: [{ id: "f1", source: "start", target: "move", label: "" }],
      },
      {
        delegateZoom: true,
        onNodeClick: (node) => clicked.push(node.id),
      },
    );

    const node = target.querySelector('.action-flow-node[data-node-id="move"]') as SVGGElement | null;
    expect(node).toBeTruthy();
    node?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    expect(clicked).toEqual(["move"]);
    expect(node?.classList.contains("highlighted-element")).toBe(true);
    expect(node?.querySelector(".node-background")).toBeTruthy();
  });

  it("renders sequence view from sequenceDiagram meta", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(
      target,
      {
        title: "Interaction",
        view: "sequence-view",
        nodes: [],
        edges: [],
        meta: {
          sequenceDiagram: {
            name: "Demo",
            lifelines: [
              { id: "user", name: "User" },
              { id: "robot", name: "Robot" },
            ],
            messages: [
              { id: "m1", source: "user", target: "robot", name: "command", order: 1 },
              { id: "m2", source: "robot", target: "user", name: "status", order: 2 },
            ],
          },
        },
      },
      { delegateZoom: true },
    );

    expectFiniteRootTransform(target);
    expect(target.querySelectorAll(".sequence-lifelines line").length).toBeGreaterThanOrEqual(2);
    expect(target.querySelectorAll(".sequence-message").length).toBe(2);
  });

  it("renders sequence fragments, activations, self messages, and return messages", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(target, {
      title: "Interaction",
      view: "sequence-view",
      nodes: [],
      edges: [],
      meta: {
        sequenceDiagram: {
          name: "Demo",
          lifelines: [
            { id: "user", name: "User" },
            { id: "robot", name: "Robot" },
          ],
          messages: [
            { id: "m1", source: "user", target: "robot", name: "command", order: 1, kind: "sync" },
            { id: "m2", source: "robot", target: "robot", name: "calculate", order: 2, kind: "sync" },
            { id: "m3", source: "robot", target: "user", name: "status", order: 3, kind: "return" },
          ],
          activations: [{ id: "a1", lifeline: "robot", startMessage: "m1", finishMessage: "m3" }],
          fragments: [{ id: "frag1", kind: "opt", operands: [{ guard: "ok", messageIds: ["m2"] }] }],
        },
      },
    });

    expect(target.querySelector(".sequence-fragment-opt")).toBeTruthy();
    expect(target.querySelector(".sequence-activation")).toBeTruthy();
    expect(target.querySelector(".sequence-message-self")).toBeTruthy();
    expect((target.querySelector(".sequence-message-return") as SVGElement | null)?.style.strokeDasharray).toBe("6,4");
    expect(target.textContent).toContain("[ok]");
  });

  it("renders provisional Browser, Grid, and Geometry standard views", async () => {
    for (const view of ["browser-view", "grid-view", "geometry-view"]) {
      const target = document.createElement("div");
      Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
      Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

      await renderVisualization(target, {
        title: view,
        view,
        nodes: [
          {
            id: "system",
            label: "System",
            kind: "part def",
            attributes: { name: "System", kind: "part def", attributeCount: 1, partCount: 2, portCount: 3 },
          },
          { id: "engine", label: "engine", kind: "part", attributes: { name: "engine", kind: "part" } },
        ],
        edges: [],
        meta: {
          rows: [
            { id: "system", label: "System", kind: "part def", qualifiedName: "Demo::System" },
            { id: "engine", label: "engine", kind: "part", qualifiedName: "Demo::System::engine" },
          ],
          cells: [
            { id: "system", name: "System", kind: "part def", attributeCount: 1, partCount: 2, portCount: 3 },
          ],
          elements: [
            { id: "system", label: "System", kind: "part def" },
            { id: "engine", label: "engine", kind: "part" },
          ],
          provisional: true,
        },
      }, { delegateZoom: true });

      expectFiniteRootTransform(target);
      expect(target.querySelector(".provisional-view-badge")).toBeTruthy();
      if (view === "browser-view") expect(target.querySelectorAll(".browser-row").length).toBeGreaterThan(0);
      if (view === "grid-view") expect(target.querySelectorAll(".grid-cell").length).toBeGreaterThan(0);
      if (view === "geometry-view") expect(target.querySelectorAll(".geometry-object").length).toBeGreaterThan(0);
    }
  });

  it("styles composition edges with diamond marker in general view", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 900, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 600, configurable: true });

    await renderVisualization(target, {
      title: "General",
      view: "general-view",
      nodes: [
        { id: "a", label: "A", kind: "part def" },
        { id: "b", label: "B", kind: "part" },
      ],
      edges: [{ id: "e1", source: "a", target: "b", label: "composition", edgeKind: "composition" }],
    });

    const path = target.querySelector(".general-connector") as SVGPathElement | null;
    expect(path?.style.strokeDasharray).toBe("6,3");
    expect(path?.style.markerStart).toContain("general-d3-diamond");
  });

  it("applies BNF interconnection connector markers per edge kind", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });

    await renderVisualization(target, {
      title: "IBD",
      view: "interconnection-view",
      nodes: [
        { id: "a", label: "A", kind: "part", attributes: { ports: ["p1"] } },
        { id: "b", label: "B", kind: "part", attributes: { ports: ["p2"] } },
      ],
      edges: [
        { id: "flow-e", source: "a", target: "b", label: "", edgeKind: "flow", attributes: { relationType: "flow", sourceId: "A.p1", targetId: "B.p2" } },
        { id: "iface-e", source: "a", target: "b", label: "", edgeKind: "interface", attributes: { relationType: "interface-connection", sourceId: "A.p1", targetId: "B.p2" } },
        { id: "bind-e", source: "a", target: "b", label: "", edgeKind: "bind", attributes: { relationType: "binding-connection", sourceId: "A.p1", targetId: "B.p2" } },
        { id: "conn-e", source: "a", target: "b", label: "", edgeKind: "connection", attributes: { relationType: "connection", sourceId: "A.p1", targetId: "B.p2" } },
      ],
    });

    const flow = target.querySelector('[data-connector-id="flow-e"]') as SVGPathElement | null;
    const iface = target.querySelector('[data-connector-id="iface-e"]') as SVGPathElement | null;
    const bind = target.querySelector('[data-connector-id="bind-e"]') as SVGPathElement | null;
    const conn = target.querySelector('[data-connector-id="conn-e"]') as SVGPathElement | null;
    expect(flow?.style.markerEnd).toContain("ibd-flow-arrow");
    expect(iface?.style.strokeDasharray).toBe("8,4");
    expect(iface?.style.markerEnd).toContain("ibd-interface-arrow");
    expect(bind?.style.strokeDasharray).toBe("6,4");
    expect(conn?.style.markerStart).toContain("ibd-connection-dot");
  });

  it("draws general package frames when multiple package namespaces are present", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1200, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 800, configurable: true });

    await renderVisualization(target, {
      title: "General",
      view: "general-view",
      meta: {
        packageContainerGroups: [
          { id: "pkg:A", name: "PackageA", memberIds: ["n1"] },
          { id: "pkg:B", name: "PackageB", memberIds: ["n2"] },
        ],
      },
      nodes: [
        { id: "n1", label: "PartA", kind: "part def", attributes: { qualifiedName: "PackageA::PartA" } },
        { id: "n2", label: "PartB", kind: "part def", attributes: { qualifiedName: "PackageB::PartB" } },
      ],
      edges: [],
    });

    expect(target.querySelectorAll(".general-package-frame").length).toBe(2);
    expect(target.textContent).toContain("PackageA");
    expect(target.textContent).toContain("PackageB");
  });

  it("styles redefinition edges like specializes in general view", async () => {
    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 900, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 600, configurable: true });

    await renderVisualization(target, {
      title: "General",
      view: "general-view",
      nodes: [
        { id: "a", label: "A", kind: "part def" },
        { id: "b", label: "B", kind: "part" },
      ],
      edges: [{ id: "e1", source: "a", target: "b", label: "redefinition", edgeKind: "redefinition" }],
    });

    const path = target.querySelector(".general-connector") as SVGPathElement | null;
    expect(path?.style.markerEnd).toContain("general-d3-specializes");
    expect(path?.style.strokeDasharray).toBe("5,3");
  });
});
