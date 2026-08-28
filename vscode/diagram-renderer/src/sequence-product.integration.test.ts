// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import { prepareViewData } from "./prepare";
import { renderVisualization } from "./renderer";

function sequenceProductText(file = "diagram_sequence_complete.md"): string {
  const snapshot = readFileSync(
    resolve(process.cwd(), `../../tests/snapshots/generation/${file}`),
    "utf8",
  );
  const generated = snapshot.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) throw new Error(`${file} has no exact diagram.json product`);
  return match[1];
}

describe("typed SequenceView product", () => {
  it("renders the compiler-owned lifelines, endpoints, and order end to end", async () => {
    const product = parseDiagramProduct(sequenceProductText());
    const prepared = prepareViewData(product);
    const sequence = prepared.meta?.sequenceDiagram as {
      lifelines: Array<{ id: string; name: string }>;
      messages: Array<{ name: string; source: string; target: string; order: number }>;
    };

    expect(prepared.view).toBe("sequence-view");
    expect(sequence.lifelines.map((lifeline) => lifeline.name)).toEqual(["client", "server"]);
    expect(sequence.messages.map((message) => ({
      name: message.name,
      source: message.source,
      target: message.target,
      order: message.order,
    }))).toEqual([
      { name: "call", source: sequence.lifelines[0].id, target: sequence.lifelines[1].id, order: 1 },
      { name: "result", source: sequence.lifelines[1].id, target: sequence.lifelines[0].id, order: 2 },
    ]);

    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1280, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
    const controller = await renderVisualization(target, prepared, { theme: { colorScheme: "light" } });
    expect(target.querySelectorAll(".sequence-lifeline")).toHaveLength(2);
    expect(target.querySelectorAll(".sequence-message")).toHaveLength(2);
    expect(target.textContent).toContain("client");
    expect(target.textContent).toContain("server");
    for (const label of ["call", "result"]) {
      expect(target.textContent).toContain(label);
    }
    expect(controller.exportSvg()).not.toContain("NaN");
    controller.destroy();
  });

  it.each([
    ["diagram_sequence_ordering_cycle.md", "sequence-ordering-cycle"],
    ["diagram_sequence_unresolved_endpoint.md", "relationship-unresolved"],
    ["diagram_sequence_outside_lifeline.md", "sequence-message-endpoint-outside-lifeline"],
  ])("does not render a guessed message for %s", async (file, reason) => {
    const product = parseDiagramProduct(sequenceProductText(file));
    expect(product.completeness.status).toBe("incomplete");
    expect(product.completeness.reasons.some((entry) => entry.code === reason)).toBe(true);

    const prepared = prepareViewData(product);
    const sequence = prepared.meta?.sequenceDiagram as {
      lifelines: Array<{ id: string; name: string }>;
      messages: Array<{ name: string; source: string; target: string; order: number }>;
    };
    expect(sequence.lifelines.length).toBeGreaterThan(0);
    expect(sequence.messages).toEqual([]);

    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1280, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
    const controller = await renderVisualization(target, prepared, { theme: { colorScheme: "light" } });
    expect(target.querySelectorAll(".sequence-message")).toHaveLength(0);
    expect(controller.exportSvg()).not.toContain("NaN");
    controller.destroy();
  });
});
