import * as assert from "assert";
import { buildSharedRendererInput, interconnectionBannerCounts } from "../../visualization/dtoAdapter";

const VIEW_IDS = [
    "general-view",
    "interconnection-view",
    "action-flow-view",
    "state-transition-view",
    "sequence-view",
    "browser-view",
    "grid-view",
    "geometry-view",
] as const;

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

    it("interconnectionBannerCounts returns zero when ibd absent", () => {
        const counts = interconnectionBannerCounts(null);
        assert.strictEqual(counts.partCount, 0);
        assert.strictEqual(counts.connectorCount, 0);
    });

    for (const view of VIEW_IDS) {
        it(`buildSharedRendererInput sets view for ${view}`, () => {
            const input = buildSharedRendererInput({ graph: { nodes: [], edges: [] } }, view);
            assert.ok(input);
            assert.strictEqual(input?.view, view);
        });
    }
});
