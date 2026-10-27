// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { diagramLimitations } from "../../src/diagram/diagramLimitations";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import type { DiagramProduct } from "../../src/generated/diagram-product/DiagramProduct";

vi.mock("./renderer", () => ({
  isNativeDiagramView: () => true,
  renderVisualization: async () => ({ destroy() {}, getViewport() {}, getDisclosureState() {}, reset() {} }),
}));

function fixture(name: string): DiagramProduct {
  const text = readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../../tests/snapshots/generation", `${name}.md`), "utf8").replace(/\r\n/g, "\n");
  return parseDiagramProduct(text.split("## diagram.json\n~~~json\n")[1].split("\n~~~")[0]);
}

describe("canonical sequence limitation presentation", () => {
  it("explains actual succession and event witnesses with source navigation", () => {
    const explicit = diagramLimitations(fixture("sequence_message_cycle_and_blocked"));
    expect(explicit).toHaveLength(1);
    expect(explicit[0].explanation).toContain("One");
    expect(explicit[0].explanation).toContain("Two");
    expect(explicit[0].explanation).toContain("authored succession");
    expect(explicit[0].links.length).toBeGreaterThan(0);
    const events = diagramLimitations(fixture("sequence_event_cycle"));
    expect(events[0].explanation).toContain("event causality");
    expect(events[0].explanation).toContain("A::sent");
    expect(events[0].links.every((link) => link.target.endLine >= link.target.startLine)).toBe(true);
  });

  it("explains unsupported omission and clears details after repair or narrow exposure", () => {
    const omitted = diagramLimitations(fixture("sequence_omitted_nested"));
    expect(omitted[0].code).toBe("sequence-content-omitted");
    expect(omitted[0].links[0].label).toContain("Omitted");
    expect(diagramLimitations(fixture("sequence_recovery"))[0].code).toBe("parse-recovery");
    expect(diagramLimitations(fixture("sequence_recovery_repaired"))).toEqual([]);
    expect(diagramLimitations(fixture("sequence_narrow_exposure"))).toEqual([]);
  });

  it("explains unresolved and outside-lifeline endpoints without calling them cycles", () => {
    const outside = diagramLimitations(fixture("diagram_sequence_outside_lifeline"));
    expect(outside.some((detail) => detail.code === "sequence-message-endpoint-outside-lifeline")).toBe(true);
    expect(outside.some((detail) => detail.code === "sequence-ordering-cycle")).toBe(false);
    const unresolved = diagramLimitations(fixture("diagram_sequence_unresolved_endpoint"));
    expect(unresolved.some((detail) => detail.code === "relationship-unresolved")).toBe(true);
    expect(unresolved.every((detail) => detail.links.length > 0)).toBe(true);
  });

  it("keeps accessible details available for nonempty and empty products, and clears stale details", async () => {
    document.body.innerHTML = `
      <select id="view-select"></select><span id="status"></span><span id="view-kind"></span>
      <button id="home"></button><button id="copy-json"></button><button id="view-definition"></button>
      <input id="transparent-export" type="checkbox"><button id="export-svg"></button><button id="export-png"></button>
      <aside id="limitations" hidden></aside><main id="diagram"></main>`;
    const postMessage = vi.fn();
    vi.stubGlobal("acquireVsCodeApi", () => ({ postMessage }));
    await import("../../src/diagram/diagramWebview");
    const send = (product: DiagramProduct, placeholder?: string) => window.dispatchEvent(new MessageEvent("message", { data: {
      type: "render", productJson: JSON.stringify(product), views: [], selectedHandle: "scenario", header: "Projection", placeholder,
    } }));
    const product = fixture("sequence_message_cycle_and_blocked");
    send(product);
    await vi.waitFor(() => expect(document.querySelector("#limitations details")).not.toBeNull());
    const summary = document.querySelector<HTMLElement>("summary")!;
    summary.focus();
    expect(document.activeElement).toBe(summary);
    expect(document.querySelector("#limitations")!.hasAttribute("hidden")).toBe(false);
    const buttons = [...document.querySelectorAll<HTMLButtonElement>("#limitations button")];
    buttons[0].click();
    expect(postMessage).toHaveBeenCalledWith(expect.objectContaining({ type: "openSource", target: expect.objectContaining({ uri: expect.any(String) }) }));
    buttons.at(-1)!.click();
    expect(postMessage).toHaveBeenCalledWith({ type: "copyLimitations", modelDigest: product.modelDigest, viewHandle: "scenario" });

    document.querySelector<HTMLDetailsElement>("details")!.open = true;
    send(fixture("sequence_event_cycle"));
    await vi.waitFor(() => expect(document.querySelectorAll("#limitations li")).toHaveLength(2));
    expect(document.querySelector<HTMLDetailsElement>("details")!.open).toBe(true);

    send(fixture("sequence_recovery_repaired"));
    await vi.waitFor(() => expect(document.querySelector("#limitations")!.hasAttribute("hidden")).toBe(true));
    expect(document.querySelectorAll("#limitations li")).toHaveLength(0);

    const empty = fixture("diagram_sequence_unresolved");
    send(empty);
    await vi.waitFor(() => expect(document.querySelector("#diagram")!.textContent).toContain("expose target cannot be resolved"));
    expect(document.querySelector("#limitations")!.hasAttribute("hidden")).toBe(false);
    send(empty, "Loading another view");
    expect(document.querySelector("#limitations")!.hasAttribute("hidden")).toBe(true);
    vi.unstubAllGlobals();
  });
});
