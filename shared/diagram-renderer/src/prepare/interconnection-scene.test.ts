import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { prepareInterconnectionScene } from "./interconnection-scene";

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), "../../test-fixtures/interconnection");

describe("prepareInterconnectionScene", () => {
  it("maps canonical scene without string owner inference", () => {
    const payload = JSON.parse(readFileSync(join(fixtureDir, "scene-two-part-chain.json"), "utf8"));
    const prepared = prepareInterconnectionScene(payload.interconnectionScene, payload);
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(prepared.edges[0]).toMatchObject({
      source: "occ:Demo.Source",
      target: "occ:Demo.Target",
      attributes: {
        sourcePortId: "occ:Demo.Source.out",
        targetPortId: "occ:Demo.Target.in",
      },
    });
  });
});
