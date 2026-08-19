// @vitest-environment jsdom
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import { prepareViewData } from "./prepare";
import { renderVisualization } from "./renderer";

function timerStateProductText(): string {
  const snapshot = readFileSync(
    resolve(process.cwd(), "../../tests/snapshots/generation/diagram_timer_state_transition.md"),
    "utf8",
  );
  const generated = snapshot.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) throw new Error("timer State Transition snapshot has no exact diagram.json product");
  return match[1];
}

describe("timer typed State Transition scene", () => {
  it("renders states and semantic transitions rather than the ownership tree", async () => {
    const product = parseDiagramProduct(timerStateProductText());
    const prepared = prepareViewData(product);

    expect(prepared.view).toBe("state-transition-view");
    expect(prepared.nodes.map((node) => node.label).sort()).toEqual(["", "expired", "idle", "paused", "running"]);
    expect(prepared.nodes.some((node) => node.label === "TimerStateMachine")).toBe(false);
    expect(prepared.nodes.some((node) => node.label === "to_paused")).toBe(false);
    expect(prepared.edges).toHaveLength(9);
    expect(prepared.edges.some((edge) => edge.label === "StartPressed" && edge.source !== edge.target)).toBe(true);
    expect(prepared.edges.filter((edge) => edge.source === edge.target)).toHaveLength(2);

    const target = document.createElement("div");
    Object.defineProperty(target, "clientWidth", { value: 1400, configurable: true });
    Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
    const controller = await renderVisualization(target, prepared, { theme: { colorScheme: "light" } });
    expect(target.querySelectorAll(".state-transition-node")).toHaveLength(5);
    expect(target.querySelectorAll(".state-transition-edge")).toHaveLength(9);
    expect(target.textContent).toContain("StartPressed");
    expect(target.textContent).not.toContain("to_paused");
    controller.destroy();
  });
});
