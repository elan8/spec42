// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { headerFillPath, nodeBodyChromeStyle, nodeOutlinePath, resolveNodeChrome } from "./node-notation";
import {
  DISCLOSURE_TARGET_SIZE,
  NAME_MAX_LINES,
  NODE_WIDTH_MAX,
  NODE_WIDTH_MIN,
  collectCompartments,
  computeNodeHeight,
  computeNodeWidth,
  layoutNodeHeader,
  wrapElementName,
} from "./sysml-node-builder";
import { nodeChromeStyleSheet } from "./render/node-chrome-style";
import { renderVisualization } from "./renderer";
import { resolveDiagramTheme } from "./theme";
import type { PreparedView } from "./prepare/types";

const LONG_NAME = "VehicleThermalManagementSubsystemAssembly";

function hostElement(width = 1200, height = 800): HTMLElement {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: width, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: height, configurable: true });
  return target;
}

function generalView(overrides: Partial<PreparedView> = {}): PreparedView {
  return {
    title: "General",
    view: "general-view",
    meta: { exposedRoots: ["n:0", "n:3"] },
    nodes: [
      {
        id: "n:0",
        label: LONG_NAME,
        kind: "PartDefinition",
        uri: "memory://chrome.sysml",
        range: { start: { line: 1, character: 0 }, end: { line: 1, character: 4 } },
        attributes: {
          notationRole: "definition",
          owner: null,
          typedCompartments: [
            { kind: "attributes", provenance: "direct", members: [{ name: "coolantMass : MassValue" }] },
            { kind: "parts", provenance: "direct", members: [{ name: "radiator : Radiator" }] },
            { kind: "attributes", provenance: "inherited", members: [{ name: "serialNumber : String" }] },
          ],
        },
      },
      { id: "n:1", label: "radiator", kind: "PartUsage", attributes: { notationRole: "usage", owner: 0 } },
      { id: "n:2", label: "pump", kind: "PartUsage", attributes: { notationRole: "usage", owner: 0 } },
      { id: "n:3", label: "Radiator", kind: "PartDefinition", attributes: { notationRole: "definition", owner: null } },
    ],
    edges: [
      { id: "e:0", source: "n:1", target: "n:3", label: "", edgeKind: "typing" },
      { id: "e:1", source: "n:2", target: "n:3", label: "", edgeKind: "reference" },
    ],
    ...overrides,
  };
}

async function settle(check: () => boolean): Promise<void> {
  for (let attempt = 0; attempt < 80 && !check(); attempt += 1) {
    await new Promise((resolve) => setTimeout(resolve, 5));
  }
}

function pathCoordinates(d: string): Array<{ x: number; y: number }> {
  // Every command in the emitted paths is absolute; arc parameters are `r,r 0 0 1 x,y`.
  const points: Array<{ x: number; y: number }> = [];
  const pattern = /([MALHV])([^MALHVZ]*)/gi;
  let cursor = { x: 0, y: 0 };
  for (const match of d.matchAll(pattern)) {
    const command = match[1].toUpperCase();
    const numbers = (match[2].match(/-?\d+(?:\.\d+)?/g) ?? []).map(Number);
    if (command === "M" || command === "L") {
      cursor = { x: numbers[0], y: numbers[1] };
    } else if (command === "H") {
      cursor = { x: numbers[0], y: cursor.y };
    } else if (command === "V") {
      cursor = { x: cursor.x, y: numbers[0] };
    } else if (command === "A") {
      cursor = { x: numbers[numbers.length - 2], y: numbers[numbers.length - 1] };
    }
    points.push({ ...cursor });
  }
  return points;
}

describe("node chrome geometry", () => {
  it("keeps the header fill inside the body stroke and clear of the rounded corners", () => {
    for (const [radius, strokeWidth] of [[0, 2], [8, 1.5], [8, 3], [4, 2]] as const) {
      const d = headerFillPath(200, 40, radius, strokeWidth);
      const inset = strokeWidth / 2;
      for (const point of pathCoordinates(d)) {
        expect(point.x).toBeGreaterThanOrEqual(inset - 1e-9);
        expect(point.x).toBeLessThanOrEqual(200 - inset + 1e-9);
        expect(point.y).toBeGreaterThanOrEqual(inset - 1e-9);
        expect(point.y).toBeLessThanOrEqual(40 - inset + 1e-9);
      }
      // The header's own top corners are concentric with the body corners, never tighter.
      expect(d).not.toContain(`A${radius},${radius}`);
    }
  });

  it("outlines the body without independently rounded header corners", () => {
    const square = nodeOutlinePath(200, 80, 0);
    expect(square).toBe("M0,0H200V80H0Z");
    const rounded = nodeOutlinePath(200, 80, 8);
    expect(rounded.startsWith("M8,0")).toBe(true);
    expect(rounded.endsWith("Z")).toBe(true);
    // Bottom edge of the header fill is straight: exactly one arc pair, at the top.
    const headerArcs = headerFillPath(200, 40, 8, 2).match(/A/g) ?? [];
    expect(headerArcs).toHaveLength(2);
  });

  it("reserves non-overlapping header regions for the control, text and badge", () => {
    const compartments = collectCompartments({
      label: LONG_NAME,
      kind: "PartDefinition",
      attributes: { partType: "ThermalAssembly" },
    });
    const header = layoutNodeHeader(compartments, {
      width: 240,
      strokeWidthPx: 2,
      state: { disclosure: "collapsed", hiddenRelationshipCount: 128 },
    });
    expect(header.disclosureTarget).not.toBeNull();
    expect(header.badge).not.toBeNull();
    const control = header.disclosureTarget!;
    const badge = header.badge!;
    expect(control.x + control.width).toBeLessThanOrEqual(header.textLeft);
    expect(header.textRight).toBeLessThanOrEqual(badge.x);
    expect(control.y).toBeGreaterThanOrEqual(0);
    expect(control.y + control.height).toBeLessThanOrEqual(header.height);
    expect(badge.y).toBeGreaterThanOrEqual(0);
    expect(badge.y + badge.height).toBeLessThanOrEqual(header.height);
    // Stereotype, name lines and the typing line each own a distinct band.
    const baselines = [header.stereotypeBaseline, ...header.nameBaselines, header.typingBaseline!];
    for (let index = 1; index < baselines.length; index += 1) {
      expect(baselines[index]).toBeGreaterThan(baselines[index - 1]);
    }
    expect(baselines[baselines.length - 1]).toBeLessThanOrEqual(header.height);
  });

  it("derives header height from header content rather than a fixed coordinate", () => {
    const plain = collectCompartments({ label: "Pump", kind: "PartUsage", attributes: {} });
    const typed = collectCompartments({ label: "Pump", kind: "PartUsage", attributes: { partType: "CoolantPump" } });
    const longName = collectCompartments({ label: LONG_NAME, kind: "PartUsage", attributes: {} });
    const input = { width: 200, strokeWidthPx: 2 };
    const plainHeight = layoutNodeHeader(plain, input).height;
    expect(layoutNodeHeader(typed, input).height).toBeGreaterThan(plainHeight);
    expect(layoutNodeHeader(longName, input).height).toBeGreaterThan(plainHeight);
    expect(layoutNodeHeader(longName, input).nameLines.length).toBe(NAME_MAX_LINES);
  });

  it("wraps long names over bounded lines and never loses the full text", () => {
    expect(wrapElementName("Short", 20)).toEqual(["Short"]);
    const wrapped = wrapElementName(LONG_NAME, 20);
    expect(wrapped.length).toBeLessThanOrEqual(NAME_MAX_LINES);
    for (const line of wrapped) expect(line.length).toBeLessThanOrEqual(20);
    expect(wrapped.join("").startsWith("VehicleThermal")).toBe(true);
    // A name that cannot fit is marked as elided rather than silently cut.
    expect(wrapElementName("A".repeat(200), 12).join("")).toContain("…");
  });

  it("derives node width from content within a bounded range", () => {
    const narrow = collectCompartments({ label: "Pump", kind: "PartUsage", attributes: {} });
    const wide = collectCompartments({
      label: "Pump",
      kind: "PartUsage",
      attributes: {
        typedCompartments: [
          {
            kind: "attributes",
            provenance: "direct",
            members: [{ name: "measuredCoolantInletTemperature : TemperatureValue" }],
          },
        ],
      },
    });
    const enormous = collectCompartments({
      label: "Pump",
      kind: "PartUsage",
      attributes: {
        typedCompartments: [
          { kind: "attributes", provenance: "direct", members: [{ name: "x".repeat(400) }] },
        ],
      },
    });
    expect(computeNodeWidth(narrow)).toBe(NODE_WIDTH_MIN);
    expect(computeNodeWidth(wide)).toBeGreaterThan(NODE_WIDTH_MIN);
    expect(computeNodeWidth(enormous)).toBe(NODE_WIDTH_MAX);
  });

  it("measures a node without compartments as exactly its header", () => {
    const compartments = collectCompartments({ label: "Radiator", kind: "PartDefinition", attributes: {} });
    const header = layoutNodeHeader(compartments, { width: 200, strokeWidthPx: 2 });
    expect(computeNodeHeight(compartments, {}, { width: 200 })).toBe(header.height);
  });
});

describe("node chrome rendering", () => {
  it("draws a continuous border with the header fill clipped inside it", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const node = target.querySelector('[data-node-id="n:0"]')!;
    const background = node.querySelector(".sysml-node-bg") as SVGRectElement;
    const chrome = resolveNodeChrome("definition");
    const body = nodeBodyChromeStyle(chrome, { generalView: true });
    expect(background.getAttribute("width")).toBe(node.getAttribute("data-bounds")!.split(",")[2]);
    expect(background.style.strokeWidth).toBe(`${body.strokeWidthPx}px`);

    const fill = node.querySelector(".sysml-header-compartment")!;
    // A path, not a rect: a rect could only approximate the body's corners and would paint over
    // the inner half of the stroke.
    expect(fill.tagName.toLowerCase()).toBe("path");
    const width = Number(background.getAttribute("width"));
    const inset = body.strokeWidthPx / 2;
    for (const point of pathCoordinates(fill.getAttribute("d")!)) {
      expect(point.x).toBeGreaterThanOrEqual(inset - 1e-9);
      expect(point.x).toBeLessThanOrEqual(width - inset + 1e-9);
      expect(point.y).toBeGreaterThanOrEqual(inset - 1e-9);
    }
  });

  it("terminates compartment dividers inside the node boundary", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const node = target.querySelector('[data-node-id="n:0"]')!;
    const [, , boundsWidth, boundsHeight] = node.getAttribute("data-bounds")!.split(",").map(Number);
    const body = nodeBodyChromeStyle(resolveNodeChrome("definition"), { generalView: true });
    const inset = body.strokeWidthPx / 2;
    const dividers = Array.from(node.querySelectorAll(".sysml-compartment-divider"));
    expect(dividers.length).toBeGreaterThan(0);
    for (const divider of dividers) {
      expect(Number(divider.getAttribute("x1"))).toBeGreaterThanOrEqual(inset);
      expect(Number(divider.getAttribute("x2"))).toBeLessThanOrEqual(boundsWidth - inset);
      const y = Number(divider.getAttribute("y1"));
      expect(y).toBeGreaterThan(0);
      expect(y).toBeLessThan(boundsHeight);
    }
  });

  it("keeps the drawn disclosure control, name and badge from overlapping", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const node = target.querySelector('[data-node-id="n:0"]')!;
    const controlTarget = node.querySelector(".general-node-toggle .sysml-disclosure-target")!;
    const badgeRect = node.querySelector(".general-hidden-relationships rect")!;
    const controlRight = Number(controlTarget.getAttribute("x")) + Number(controlTarget.getAttribute("width"));
    const badgeLeft = Number(badgeRect.getAttribute("x"));
    expect(controlRight).toBeLessThan(badgeLeft);

    const width = Number(node.getAttribute("data-bounds")!.split(",")[2]);
    const compartments = collectCompartments({
      label: LONG_NAME,
      kind: "PartDefinition",
      attributes: (generalView().nodes[0].attributes ?? {}),
    });
    const header = layoutNodeHeader(compartments, {
      width,
      strokeWidthPx: 2,
      state: { disclosure: "collapsed", hiddenRelationshipCount: 2 },
    });
    expect(controlRight).toBeLessThanOrEqual(header.textLeft);
    expect(header.textRight).toBeLessThanOrEqual(badgeLeft);
  });

  it("gives the disclosure control at least a 24x24 pointer target", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const hit = target.querySelector('[data-node-id="n:0"] .general-node-toggle .sysml-disclosure-target')!;
    expect(Number(hit.getAttribute("width"))).toBeGreaterThanOrEqual(DISCLOSURE_TARGET_SIZE);
    expect(Number(hit.getAttribute("height"))).toBeGreaterThanOrEqual(DISCLOSURE_TARGET_SIZE);
    expect((hit as SVGElement).style.pointerEvents).toBe("all");
  });

  it("exposes an accessible role, label, tooltip and expansion state", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const control = target.querySelector('[data-node-id="n:0"] .general-node-toggle')!;
    expect(control.getAttribute("role")).toBe("button");
    expect(control.getAttribute("tabindex")).toBe("0");
    expect(control.getAttribute("aria-expanded")).toBe("false");
    expect(control.getAttribute("aria-label")).toBe(`Expand ${LONG_NAME}`);
    expect(control.querySelector("title")?.textContent).toContain("show its nested elements");

    control.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await settle(() => Boolean(target.querySelector('[data-node-id="n:1"]')));
    const expandedControl = target.querySelector('[data-node-id="n:0"] .general-node-toggle')!;
    expect(expandedControl.getAttribute("aria-expanded")).toBe("true");
    expect(expandedControl.getAttribute("aria-label")).toBe(`Collapse ${LONG_NAME}`);
    expect(expandedControl.querySelector("title")?.textContent).toContain("hide its nested elements");
  });

  it("expands and collapses on click without triggering source navigation", async () => {
    const target = hostElement();
    const clicked: string[] = [];
    await renderVisualization(target, generalView(), {
      theme: { colorScheme: "light" },
      onNodeClick: (node) => clicked.push(node.id),
    });
    expect(target.querySelector('[data-node-id="n:1"]')).toBeNull();

    target.querySelector('[data-node-id="n:0"] .general-node-toggle')!
      .dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await settle(() => Boolean(target.querySelector('[data-node-id="n:1"]')));
    expect(target.querySelector('[data-node-id="n:1"]')).toBeTruthy();
    expect(clicked).toEqual([]);

    target.querySelector('[data-node-id="n:0"] .general-node-toggle')!
      .dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await settle(() => !target.querySelector('[data-node-id="n:1"]'));
    expect(target.querySelector('[data-node-id="n:1"]')).toBeNull();
    expect(clicked).toEqual([]);

    // The node itself still navigates.
    target.querySelector('[data-node-id="n:0"]')!.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    expect(clicked).toEqual(["n:0"]);
  });

  it("activates the disclosure control from the keyboard with Enter and Space", async () => {
    for (const key of ["Enter", " "]) {
      const target = hostElement();
      await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
      const control = target.querySelector('[data-node-id="n:0"] .general-node-toggle')!;
      control.dispatchEvent(new KeyboardEvent("keydown", { key, bubbles: true }));
      await settle(() => Boolean(target.querySelector('[data-node-id="n:1"]')));
      expect(target.querySelector('[data-node-id="n:1"]')).toBeTruthy();
    }
  });

  it("ignores unrelated keys on the disclosure control", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const control = target.querySelector('[data-node-id="n:0"] .general-node-toggle')!;
    control.dispatchEvent(new KeyboardEvent("keydown", { key: "a", bubbles: true }));
    await new Promise((resolve) => setTimeout(resolve, 30));
    expect(target.querySelector('[data-node-id="n:1"]')).toBeNull();
  });

  it("shows hidden relationships as a badge with an explanatory tooltip", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const badge = target.querySelector('[data-node-id="n:0"] .general-hidden-relationships')!;
    expect(badge.querySelector("text")?.textContent).toBe("2");
    expect(badge.getAttribute("aria-label")).toContain("2 relationships");
    const tooltip = badge.querySelector("title")?.textContent ?? "";
    expect(tooltip).toContain("hidden");
    expect(tooltip).toContain("Expand");
    // Compact badge, not a sentence painted across the header.
    expect(Number(badge.querySelector("rect")!.getAttribute("width"))).toBeLessThanOrEqual(40);

    target.querySelector('[data-node-id="n:0"] .general-node-toggle')!
      .dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await settle(() => Boolean(target.querySelector('[data-node-id="n:1"]')));
    expect(target.querySelector(".general-hidden-relationships")).toBeNull();
  });

  it("uses the same disclosure language for compartments and preserves their state across redraws", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const section = () =>
      target.querySelector('[data-node-id="n:0"] [data-compartment-key="inherited-attributes"]')!;
    expect(section().getAttribute("aria-expanded")).toBe("false");
    expect(section().classList.contains("sysml-disclosure")).toBe(true);
    expect(section().querySelector(".sysml-disclosure-box")).toBeTruthy();
    const collapsedHeight = Number(target.querySelector('[data-node-id="n:0"]')!
      .getAttribute("data-bounds")!.split(",")[3]);

    section().dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await settle(() => section().getAttribute("aria-expanded") === "true");
    expect(section().getAttribute("aria-expanded")).toBe("true");
    const expandedHeight = Number(target.querySelector('[data-node-id="n:0"]')!
      .getAttribute("data-bounds")!.split(",")[3]);
    expect(expandedHeight).toBeGreaterThan(collapsedHeight);

    // A node expansion redraw must not reset the compartment the viewer opened. Inherited members
    // are never drawn as nodes, so the compartment survives the expansion.
    target.querySelector('[data-node-id="n:0"] .general-node-toggle')!
      .dispatchEvent(new MouseEvent("click", { bubbles: true }));
    await settle(() => Boolean(target.querySelector('[data-node-id="n:1"]')));
    expect(section().getAttribute("aria-expanded")).toBe("true");
  });

  it("keeps disclosure controls legible in both colour schemes", async () => {
    for (const colorScheme of ["light", "dark"] as const) {
      const theme = resolveDiagramTheme({ colorScheme });
      const target = hostElement();
      await renderVisualization(target, generalView(), { theme: { colorScheme } });
      const box = target.querySelector(
        '[data-node-id="n:0"] .general-node-toggle .sysml-disclosure-box',
      ) as SVGRectElement;
      const glyph = target.querySelector(
        '[data-node-id="n:0"] .general-node-toggle .sysml-disclosure-glyph',
      ) as SVGPathElement;
      expect(box.style.stroke).toBe(theme.controlStroke);
      expect(box.style.fill).toBe(theme.controlFill);
      expect(glyph.style.fill).toBe(theme.controlForeground);
      expect(theme.controlStroke).not.toBe(theme.controlFill);
      expect(theme.controlForeground).not.toBe(theme.panelBackground);

      const style = target.querySelector("style.sysml-node-chrome-style")?.textContent ?? "";
      expect(style).toContain(theme.controlHoverFill);
      expect(style).toContain(theme.focusRing);
      expect(style).toContain(":hover");
      expect(style).toContain(":focus-visible");
    }
  });

  it("emits hover and focus rules for both node and compartment controls", () => {
    const sheet = nodeChromeStyleSheet(resolveDiagramTheme({ colorScheme: "light" }));
    expect(sheet).toContain(".sysml-disclosure:hover .sysml-disclosure-box");
    expect(sheet).toContain(".sysml-disclosure:focus-visible .sysml-disclosure-target");
    expect(sheet).toContain("cursor: pointer");
  });

  it("keeps the full element name available when the drawn name is wrapped", async () => {
    const target = hostElement();
    await renderVisualization(target, generalView(), { theme: { colorScheme: "light" } });
    const nameGroup = target.querySelector('[data-node-id="n:0"] .viz-node-name')!;
    expect(nameGroup.querySelectorAll("text").length).toBeLessThanOrEqual(NAME_MAX_LINES);
    expect(nameGroup.querySelector("title")?.textContent).toBe(LONG_NAME);
  });
});
