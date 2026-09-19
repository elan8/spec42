import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { layoutPrepared } from "../render/layout";
import { exportHeadlessSvg } from "../headless-export";
import { generalViewGoldenPayload } from "./golden-parity-payloads";

/**
 * Regenerable fixtures for the Rust drawing spike (`crates/diagram_draw`, spec42 #176 phase 1):
 *
 * - The already-laid-out General View graph -- the same `LaidOutNode[]`/`LaidOutEdge[]` shape
 *   `drawNodes`/`drawEdges` draw from -- dumped from the real `prepareViewData` + `layoutPrepared`
 *   pipeline, so the Rust drawing port has a production-shaped input rather than a hand-authored
 *   one.
 * - The actual SVG the existing TS/D3 pipeline renders from that same graph (via the identical
 *   `exportHeadlessSvg` call `headless-export.golden-parity.test.ts` uses), so the Rust output can
 *   be checked against real rendered content -- geometry, text, attributes -- not just the
 *   marker/class-count summary `svg-markers.ts` produces.
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-general-view-fixture.test.ts` whenever `generalViewGoldenPayload`,
 * `prepareViewData`, `layoutPrepared`, or the drawing pipeline change in a way that affects the
 * General View shape or rendering.
 */
const FIXTURES_DIR = join(dirname(fileURLToPath(import.meta.url)), "../../../../crates/diagram_draw/tests/fixtures");
const GRAPH_FIXTURE_PATH = join(FIXTURES_DIR, "general-view.laid-out.json");
const SVG_FIXTURE_PATH = join(FIXTURES_DIR, "general-view.rendered.svg");

describe("diagram_draw fixture dump", () => {
  it("writes the laid-out General View graph and its rendered SVG when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    const prepared = prepareViewData(generalViewGoldenPayload);
    const layout = await layoutPrepared(prepared);
    mkdirSync(FIXTURES_DIR, { recursive: true });
    writeFileSync(
      GRAPH_FIXTURE_PATH,
      `${JSON.stringify(
        { title: prepared.title, view: prepared.view, meta: prepared.meta ?? null, nodes: layout.nodes, edges: layout.edges },
        null,
        2,
      )}\n`,
      "utf8",
    );
    const svg = await exportHeadlessSvg(generalViewGoldenPayload, { width: 1280, height: 900, colorScheme: "light" });
    writeFileSync(SVG_FIXTURE_PATH, `${svg}\n`, "utf8");
  });
});
