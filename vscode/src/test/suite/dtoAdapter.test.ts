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

    it("buildSharedRendererInput trims interconnection legacy payload when preparedView is present", () => {
        const input = buildSharedRendererInput(
            {
                currentView: "interconnection-view",
                ibd: { parts: [{ id: "legacy" }], connectors: [] },
                interconnectionScene: { schemaVersion: 2, edges: [{ id: "legacy-edge" }] },
                preparedView: {
                    title: "Connections",
                    view: "interconnection-view",
                    nodes: [{ id: "n1" }],
                    edges: [],
                },
            },
            "interconnection-view",
        );

        assert.ok(input);
        assert.ok(input.preparedView);
        assert.strictEqual(input.ibd, undefined);
        assert.strictEqual(input.interconnectionScene, undefined);
    });

    it("interconnectionBannerCounts prefers preparedView metrics", () => {
        const counts = interconnectionBannerCounts({
            preparedView: {
                title: "Connections",
                view: "interconnection-view",
                nodes: [{}, {}, {}],
                edges: [{}, {}],
            },
            ibd: { parts: [{}], connectors: [] },
        });

        assert.strictEqual(counts.partCount, 3);
        assert.strictEqual(counts.connectorCount, 2);
    });

    it("buildSharedRendererInput does not fall back to interconnection legacy payloads", () => {
        const input = buildSharedRendererInput(
            {
                currentView: "interconnection-view",
                ibd: { parts: [{ id: "legacy" }, { id: "legacy-2" }], connectors: [{ id: "c1" }] },
                interconnectionScene: { schemaVersion: 2, edges: [{ id: "legacy-edge" }] },
            },
            "interconnection-view",
        );

        assert.ok(input);
        assert.ok(input.preparedView);
        assert.strictEqual(input.ibd, undefined);
        assert.strictEqual(input.interconnectionScene, undefined);
        assert.deepStrictEqual((input.preparedView as { nodes?: unknown[] }).nodes, []);
        assert.deepStrictEqual((input.preparedView as { edges?: unknown[] }).edges, []);
    });

    it("interconnectionBannerCounts ignores raw ibd metrics", () => {
        const counts = interconnectionBannerCounts({
            ibd: { parts: [{}, {}], connectors: [{}] },
        });

        assert.strictEqual(counts.partCount, 0);
        assert.strictEqual(counts.connectorCount, 0);
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
