import { dirname, join } from "node:path";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import { prepareViewData } from "../prepare";
import { buildInterconnectionLayoutLookup } from "./interconnection-layout-dto";
import { pathForIbdEdge } from "./drawing";
import { layoutPrepared } from "./layout";

const fixtureDir = join(dirname(fileURLToPath(import.meta.url)), "../../test-fixtures/interconnection");

describe("interconnection drawing from layout DTO", () => {
  it("resolves edge paths from layoutDto without attribute fallback", async () => {
    const prepared = prepareViewData(
      JSON.parse(readFileSync(join(fixtureDir, "scene-two-part-chain.json"), "utf8")),
    );
    const layout = await layoutPrepared(prepared);
    const layoutDto = layout.interconnectionLayout;
    expect(layoutDto).toBeDefined();
    const lookup = buildInterconnectionLayoutLookup(layoutDto!);
    const edge = layout.edges[0];
    expect(edge).toBeDefined();

    const pathFromDto = pathForIbdEdge(edge, lookup);
    expect(pathFromDto).toBeTruthy();

    const edgeWithoutAttrs = {
      ...edge,
      attributes: { ...(edge.attributes ?? {}) },
    };
    delete (edgeWithoutAttrs.attributes as Record<string, unknown>).layoutRoutePoints;
    const pathWithoutAttrs = pathForIbdEdge(edgeWithoutAttrs, lookup);
    expect(pathWithoutAttrs).toBe(pathFromDto);

    const pathWithoutLookup = pathForIbdEdge(edgeWithoutAttrs);
    expect(pathWithoutLookup).toBeTruthy();
  });
});
