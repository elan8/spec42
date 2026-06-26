import * as assert from "assert";
import { fetchModelData } from "../../visualization/modelFetcher";

describe("fetchModelData", () => {
    it("omits interconnection legacy payloads from webview updates", async () => {
        const provider = {
            getVisualization: async () => ({
                version: 1,
                view: "interconnection-view",
                workspaceRootUri: "file:///workspace",
                viewCandidates: [],
                ibd: { parts: [{ id: "legacy-part" }], connectors: [{ id: "legacy-edge" }], ports: [] },
                interconnectionScene: {
                    schemaVersion: 2,
                    view: { id: "view-1", name: "systemContext", type: "InterconnectionView", rootIds: [] },
                    nodes: [{ id: "n1", name: "Grid" }],
                    ports: [],
                    edges: [{ id: "e1", sourceNodeId: "n1", targetNodeId: "n2" }],
                    containers: [],
                    diagnostics: [],
                },
                preparedView: {
                    title: "systemContext",
                    view: "interconnection-view",
                    nodes: [{ id: "n1", label: "Grid", kind: "part" }],
                    edges: [{ id: "e1", source: "n1", target: "n2", label: "connects" }],
                },
            }),
        } as any;

        const msg = await fetchModelData({
            workspaceRootUri: "file:///workspace",
            lspModelProvider: provider,
            currentView: "interconnection-view",
            selectedView: "view-1",
        });

        assert.ok(msg);
        assert.strictEqual(msg.preparedView?.view, "interconnection-view");
        assert.strictEqual(msg.preparedView?.edges.length, 1);
        assert.strictEqual("ibd" in msg, false);
        assert.strictEqual("interconnectionScene" in msg, false);
    });
});
