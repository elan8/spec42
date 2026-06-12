import * as assert from "assert";
import { fetchModelData } from "../../visualization/modelFetcher";

describe("fetchModelData", () => {
    it("forwards interconnectionScene from LSP visualization result", async () => {
        const scene = {
            schemaVersion: 1,
            viewId: "view-1",
            viewName: "systemContext",
            nodes: [{ id: "n1", name: "Grid" }],
            edges: [{ id: "e1", sourceNodeId: "n1", targetNodeId: "n2" }],
            diagnostics: [],
        };
        const provider = {
            getVisualization: async () => ({
                version: 1,
                view: "interconnection-view",
                workspaceRootUri: "file:///workspace",
                viewCandidates: [],
                ibd: { parts: [], connectors: [], ports: [] },
                interconnectionScene: scene,
            }),
        } as any;

        const msg = await fetchModelData({
            workspaceRootUri: "file:///workspace",
            lspModelProvider: provider,
            currentView: "interconnection-view",
            selectedView: "view-1",
        });

        assert.ok(msg);
        assert.strictEqual(msg.interconnectionScene?.schemaVersion, 1);
        assert.strictEqual(msg.interconnectionScene?.edges?.length, 1);
    });
});
