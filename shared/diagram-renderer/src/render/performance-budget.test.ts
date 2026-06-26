import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { performance } from "node:perf_hooks";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { prepareViewData } from "../prepare";
import { layoutPrepared } from "./layout";

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), "../../test-fixtures/interconnection");

function loadFixture(name: string): unknown {
  return JSON.parse(readFileSync(join(fixtureDir, name), "utf8"));
}

function budgetMs(name: string, fallback: number): number {
  const value = process.env[name];
  if (!value) return fallback;
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}

describe("shared renderer performance budgets", () => {
  it("prepares and lays out the grid interconnection fixture within the budget", async () => {
    const payload = {
      view: "interconnection-view",
      interconnectionScene: loadFixture("grid-system-context-scene.json"),
    };

    const prepareStartedAt = performance.now();
    const prepared = prepareViewData(payload);
    const prepareMs = performance.now() - prepareStartedAt;

    const layoutStartedAt = performance.now();
    const layout = await layoutPrepared(prepared);
    const layoutMs = performance.now() - layoutStartedAt;

    expect(prepared.nodes.length).toBeGreaterThan(0);
    expect(layout.nodes.length).toBeGreaterThan(0);
    expect(prepareMs).toBeLessThan(budgetMs("SPEC42_RENDERER_PREPARE_BUDGET_MS", 250));
    expect(layoutMs).toBeLessThan(budgetMs("SPEC42_RENDERER_LAYOUT_BUDGET_MS", 2500));
  });
});
