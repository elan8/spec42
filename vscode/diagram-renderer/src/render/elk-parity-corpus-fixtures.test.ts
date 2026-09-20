import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { parseDiagramProduct } from "../../../src/diagram/diagramViewerCore";
import { SYNTHETIC_CASES } from "../../visual/synthetic-cases";
import { prepareViewData } from "../prepare";
import type { PreparedView } from "../prepare/types";
import { buildInterconnectionElkGraphInput } from "./interconnection-elk-input";
import { buildGeneralElkGraphInput } from "./layout";
import { buildBehaviorElkGraphInput } from "../views/behavior-common";

/**
 * Extends the hand-authored ELK-input goldens in `elk-parity-fixtures.test.ts` with the exact ELK
 * graph JSON built from every checked-in repository diagram product plus the synthetic node-chrome
 * corpus, so the TypeScript graph builders stay owned by production-shaped inputs, not just
 * hand-picked cases.
 */

const SNAPSHOT_DIR = resolve(process.cwd(), "../../tests/snapshots/generation");
const CORPUS_FIXTURE_DIR = join(
  dirname(fileURLToPath(import.meta.url)),
  "../../test-fixtures/elk-parity/corpus",
);

function productFiles(): string[] {
  return readdirSync(SNAPSHOT_DIR)
    .filter((name) => name.startsWith("diagram_") && name.endsWith(".md"))
    .sort();
}

function preparedFromSnapshot(file: string): PreparedView {
  const text = readFileSync(resolve(SNAPSHOT_DIR, file), "utf8");
  const generated = text.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) throw new Error(`${file} has no exact diagram.json product`);
  return prepareViewData(parseDiagramProduct(match[1]) as unknown as Record<string, unknown>);
}

function assertCheckedInCorpusFixture(name: string, actual: Record<string, unknown>): void {
  const path = join(CORPUS_FIXTURE_DIR, `${name}.json`);
  if (process.env.UPDATE_ELK_FIXTURES === "1") {
    writeFileSync(path, `${JSON.stringify(actual, null, 2)}\n`, "utf8");
  }
  expect(existsSync(path), `missing checked-in corpus parity fixture ${path} (regenerate with UPDATE_ELK_FIXTURES=1)`).toBe(
    true,
  );
  expect(actual).toEqual(JSON.parse(readFileSync(path, "utf8")));
}

function behaviorOptions(prepared: PreparedView): { horizontal: boolean; mode: "action" | "state" } | null {
  if (prepared.view === "action-flow-view") {
    const horizontal = String(prepared.meta?.layoutDirection ?? "").toLowerCase() === "horizontal";
    return { horizontal, mode: "action" };
  }
  if (prepared.view === "state-transition-view") {
    const layoutMode = String(prepared.meta?.layoutDirection ?? "horizontal").toLowerCase();
    const horizontal = layoutMode !== "vertical" && layoutMode !== "force";
    return { horizontal, mode: "state" };
  }
  return null;
}

/** The exact ELK JSON the production renderer would send to `elk.layout()` for this view, or
 * `null` if the view does not use ELK (non-ELK D3 layouts, or a containment-only General View). */
function elkGraphInputFor(prepared: PreparedView): Record<string, unknown> | null {
  if (prepared.view === "general-view") return buildGeneralElkGraphInput(prepared);
  if (prepared.view === "interconnection-view") return buildInterconnectionElkGraphInput(prepared);
  const behavior = behaviorOptions(prepared);
  if (behavior) return buildBehaviorElkGraphInput(prepared, behavior);
  return null;
}

function corpusCases(): Array<{ name: string; prepared: PreparedView }> {
  const fromProducts = productFiles().map((file) => ({
    name: file.replace(/^diagram_/, "").replace(/\.md$/, ""),
    prepared: preparedFromSnapshot(file),
  }));
  const fromSynthetic = SYNTHETIC_CASES.map((entry) => ({
    name: `synthetic-${entry.id}`,
    prepared: entry.prepared,
  }));
  return [...fromProducts, ...fromSynthetic];
}

describe("ELK parity corpus fixtures", () => {
  it.each(corpusCases())("keeps the $name ELK input owned by the production graph builder", ({ name, prepared }) => {
    const graph = elkGraphInputFor(prepared);
    if (!graph) return; // non-ELK view or containment-only General View: nothing to compare.
    assertCheckedInCorpusFixture(name, graph);
  });
});
