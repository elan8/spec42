import * as assert from "assert";
import * as vscode from "vscode";
import {
  graphScopesForDocument,
  graphScopesForWorkspace,
  LspModelProvider,
} from "../../providers/lspModelProvider";
import type { SysMLModelResult } from "../../providers/sysmlModelTypes";

function createModelResult(): SysMLModelResult {
  return {
    version: 1,
    graph: {
      nodes: [
        {
          id: "Drone",
          type: "package",
          name: "Drone",
          range: {
            start: { line: 0, character: 0 },
            end: { line: 10, character: 0 },
          },
          attributes: {},
        },
      ],
      edges: [],
    },
    stats: {
      totalElements: 1,
      resolvedElements: 1,
      unresolvedElements: 0,
      parseTimeMs: 1,
      modelBuildTimeMs: 1,
      parseCached: true,
    },
  };
}

describe("LspModelProvider", () => {
  it("deduplicates concurrent identical getModel requests", async () => {
    let requestCount = 0;
    const pending = new Promise<SysMLModelResult>((resolve) => {
      setTimeout(() => resolve(createModelResult()), 20);
    });
    const client = {
      sendRequest: async () => {
        requestCount += 1;
        return await pending;
      },
    } as any;
    const provider = new LspModelProvider(client, Promise.resolve());

    const [first, second] = await Promise.all([
      provider.getModel("file:///drone.sysml", ["graph", "stats"]),
      provider.getModel("file:///drone.sysml", ["stats", "graph"]),
    ]);

    assert.strictEqual(requestCount, 1);
    assert.strictEqual(first.graph?.nodes.length, 1);
    assert.strictEqual(second.graph?.nodes.length, 1);
  });

  it("reuses cached graph+stats for graph-only requests", async () => {
    let requestCount = 0;
    const client = {
      sendRequest: async () => {
        requestCount += 1;
        return createModelResult();
      },
    } as any;
    const provider = new LspModelProvider(client, Promise.resolve());

    await provider.getModel("file:///drone.sysml", ["graph", "stats"]);
    const graphOnly = await provider.getModel("file:///drone.sysml", ["graph"]);

    assert.strictEqual(requestCount, 1);
    assert.strictEqual(graphOnly.graph?.nodes[0]?.id, "Drone");
  });

  it("invalidates cached model results for a URI", async () => {
    let requestCount = 0;
    const client = {
      sendRequest: async () => {
        requestCount += 1;
        return createModelResult();
      },
    } as any;
    const provider = new LspModelProvider(client, Promise.resolve());

    await provider.getModel("file:///drone.sysml", ["graph", "stats"]);
    provider.invalidateModelCache("file:///drone.sysml");
    await provider.getModel("file:///drone.sysml", ["graph", "stats"]);

    assert.strictEqual(requestCount, 2);
  });

  it("retries getModel when a joined in-flight request was cancelled", async () => {
    let requestCount = 0;
    const client = {
      sendRequest: async () => {
        requestCount += 1;
        await new Promise((resolve) => setTimeout(resolve, 50));
        return createModelResult();
      },
    } as any;
    const provider = new LspModelProvider(client, Promise.resolve());
    const cts = new vscode.CancellationTokenSource();
    const cancelled = provider.getModel("file:///drone.sysml", ["graph"], cts.token);
    const recovered = provider.getModel("file:///drone.sysml", ["graph"]);
    cts.cancel();
    await assert.rejects(cancelled, (error) => error instanceof vscode.CancellationError);
    const result = await recovered;
    assert.ok(requestCount >= 2);
    assert.strictEqual(result.graph?.nodes?.length, 1);
    cts.dispose();
  });

  it("passes cancellation tokens through to sysml/model requests", async () => {
    let capturedToken: vscode.CancellationToken | undefined;
    const client = {
      sendRequest: async (
        _method: string,
        _params: unknown,
        token?: vscode.CancellationToken
      ) => {
        capturedToken = token;
        return createModelResult();
      },
    } as any;
    const provider = new LspModelProvider(client, Promise.resolve());
    const cts = new vscode.CancellationTokenSource();

    await provider.getModel("file:///drone.sysml", ["graph"], cts.token);

    assert.strictEqual(capturedToken, cts.token);
    cts.dispose();
  });

  it("exposes workspace and document graph scope helpers", () => {
    assert.deepStrictEqual(graphScopesForWorkspace(), [
      "graph",
      "stats",
      "workspaceVisualization",
    ]);
    assert.deepStrictEqual(graphScopesForDocument(), ["graph", "stats"]);
  });
});
