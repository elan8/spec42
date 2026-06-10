import * as assert from "assert";
import { buildSharedRendererInput, interconnectionBannerCounts } from "../../visualization/dtoAdapter";

describe("dtoAdapter", () => {
    it("buildSharedRendererInput preserves view and graph payload", () => {
        const input = buildSharedRendererInput(
            {
                view: "general-view",
                graph: { nodes: [{ id: "a", name: "A" }], edges: [] },
            },
            "general-view",
        );

        assert.ok(input);
        assert.strictEqual(input?.view, "general-view");
        assert.ok(Array.isArray((input as { graph?: { nodes?: unknown[] } }).graph?.nodes));
    });

    it("interconnectionBannerCounts prefers raw ibd metrics", () => {
        const counts = interconnectionBannerCounts({
            ibd: { parts: [{}, {}], connectors: [{}] },
        });

        assert.strictEqual(counts.partCount, 2);
        assert.strictEqual(counts.connectorCount, 1);
    });
});
