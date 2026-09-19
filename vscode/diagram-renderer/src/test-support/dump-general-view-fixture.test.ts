import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { prepareViewData } from "../prepare";
import { layoutPrepared } from "../render/layout";
import { generalViewGoldenPayload } from "./golden-parity-payloads";

/**
 * Regenerable fixture for the Rust drawing spike (`crates/diagram_draw`, spec42 #176 phase 1):
 * the already-laid-out General View graph -- the same `LaidOutNode[]`/`LaidOutEdge[]` shape
 * `drawNodes`/`drawEdges` draw from -- dumped from the real `prepareViewData` + `layoutPrepared`
 * pipeline so the Rust drawing port has a production-shaped input rather than a hand-authored one.
 *
 * Regenerate with `UPDATE_DIAGRAM_DRAW_FIXTURES=1 npx vitest run
 * src/test-support/dump-general-view-fixture.test.ts` whenever `generalViewGoldenPayload`,
 * `prepareViewData`, or `layoutPrepared` change in a way that affects the General View shape.
 */
const FIXTURE_PATH = join(
  dirname(fileURLToPath(import.meta.url)),
  "../../../../crates/diagram_draw/tests/fixtures/general-view.laid-out.json",
);

describe("diagram_draw fixture dump", () => {
  it("writes the laid-out General View graph when UPDATE_DIAGRAM_DRAW_FIXTURES=1", async () => {
    if (process.env.UPDATE_DIAGRAM_DRAW_FIXTURES !== "1") {
      expect(true).toBe(true);
      return;
    }
    const prepared = prepareViewData(generalViewGoldenPayload);
    const layout = await layoutPrepared(prepared);
    mkdirSync(dirname(FIXTURE_PATH), { recursive: true });
    writeFileSync(
      FIXTURE_PATH,
      `${JSON.stringify(
        { title: prepared.title, view: prepared.view, meta: prepared.meta ?? null, nodes: layout.nodes, edges: layout.edges },
        null,
        2,
      )}\n`,
      "utf8",
    );
  });
});
