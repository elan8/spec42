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
      sendRequest: async (
        _method: string,
        _params: unknown,
        token?: vscode.CancellationToken
      ) => {
        requestCount += 1;
        if (token?.isCancellationRequested) {
          throw new vscode.CancellationError();
        }
        return await new Promise<SysMLModelResult>((resolve, reject) => {
          const timer = setTimeout(() => resolve(createModelResult()), 50);
          const subscription = token?.onCancellationRequested(() => {
            clearTimeout(timer);
            subscription?.dispose();
            reject(new vscode.CancellationError());
          });
        });
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

  it("getFeatureInspector sends canonical and transition URI shapes and parses the response", async () => {
    let capturedMethod: string | undefined;
    let capturedParams: unknown;
    let capturedArgumentCount = 0;
    const client = {
      sendRequest: async (...args: unknown[]) => {
        capturedArgumentCount = args.length;
        const [method, params] = args;
        assert.strictEqual(typeof method, "string");
        capturedMethod = method as string;
        capturedParams = params;
        return {
          version: 1,
          sourceUri: "file:///drone.sysml",
          requestedPosition: { line: 2, character: 7 },
          selection: { kind: "element", text: "motor" },
          containingElement: {
            id: "Drone::motor",
            name: "motor",
            qualifiedName: "Drone::motor",
            type: "part",
            role: "usage",
            declaration: "part motor : Engine;",
            uri: "file:///drone.sysml",
            range: {
              start: { line: 2, character: 2 },
              end: { line: 2, character: 20 },
            },
            documentation: "The main drive motor.",
            evaluation: { state: "notApplicable" },
            analysis: { state: "notApplicable" },
            typing: { status: "resolved", targets: [] },
            specialization: { status: "notApplicable", targets: [] },
            incomingRelationships: [],
            outgoingRelationships: [],
          },
        };
      },
    } as any;
    const provider = new LspModelProvider(client, Promise.resolve());

    const result = await provider.getFeatureInspector("file:///drone.sysml", {
      line: 2,
      character: 7,
    });

    assert.strictEqual(capturedMethod, "sysml/featureInspector");
    assert.strictEqual(
      capturedArgumentCount,
      2,
      "an absent cancellation token must not become a second positional JSON-RPC parameter"
    );
    assert.deepStrictEqual(capturedParams, {
      textDocument: { uri: "file:///drone.sysml" },
      uri: "file:///drone.sysml",
      position: { line: 2, character: 7 },
    });
    assert.strictEqual(result.containingElement?.name, "motor");
    assert.strictEqual(
      result.containingElement?.documentation,
      "The main drive motor."
    );
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
