// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import { prepareViewData } from "./prepare";
import { renderVisualization } from "./renderer";
import { layoutPrepared } from "./render/layout";

function timerProductText(): string {
  const snapshot = readFileSync(
    resolve(process.cwd(), "../../tests/snapshots/generation/diagram_timer_general.md"),
    "utf8",
  );
  const generated = snapshot.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) throw new Error("timer General View snapshot has no exact diagram.json product");
  return match[1];
}

describe("timer repository product through the bundled webview path", () => {
  it("validates, prepares, lays out, and renders visible graph content", async () => {
    const product = parseDiagramProduct(timerProductText());
    const prepared = prepareViewData(product);
    expect(prepared.nodes.length).toBeGreaterThan(1);
    expect(prepared.edges.length).toBeGreaterThan(0);
    expect(prepared.edges.every((edge) =>
      prepared.nodes.some((node) => node.id === edge.source)
      && prepared.nodes.some((node) => node.id === edge.target))).toBe(true);
    const timer = prepared.nodes.find((node) => node.label === "timerInstance");
    expect(timer?.attributes?.typedByName).toBe("KitchenTimer");
    const partMembers = (timer?.attributes?.typedCompartments as Array<Record<string, unknown>>)
      .find((compartment) => compartment.kind === "parts")?.members as Array<Record<string, unknown>>;
    expect(partMembers.find((member) => member.name === "pcb")?.typeName).toBe("TimerPCB");

    const layout = await layoutPrepared(prepared);
    expect(layout.nodes.length).toBeGreaterThan(0);

    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
    const controller = await renderVisualization(target, prepared, {
      theme: { colorScheme: "light" },
    });
    const svg = controller.exportSvg();
    expect(svg).toContain("<svg");
    expect(svg).toContain("general-node");
    expect(svg).toContain("KitchenTimer");
    expect(target.querySelectorAll("[data-node-id]").length).toBeGreaterThan(0);
    controller.destroy();
  });
});
