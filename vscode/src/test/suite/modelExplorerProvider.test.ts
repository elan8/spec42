import * as assert from "assert";
import * as vscode from "vscode";
import { ModelExplorerProvider } from "../../explorer/modelExplorerProvider";
import type { SysMLModelResult } from "../../providers/sysmlModelTypes";

function createModelResult(id: string, name = "Drone"): SysMLModelResult {
  return {
    version: 1,
    graph: {
      nodes: [
        {
          id,
          type: "package",
          name,
          range: {
            start: { line: 0, character: 0 },
            end: { line: 0, character: 10 },
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

describe("ModelExplorerProvider", () => {
  it("deduplicates concurrent document loads for the same URI", async () => {
    let requestCount = 0;
    let resolveRequest: ((value: SysMLModelResult) => void) | undefined;
    const provider = new ModelExplorerProvider({
      getModel: async () => {
        requestCount += 1;
        return await new Promise<SysMLModelResult>((resolve) => {
          resolveRequest = resolve;
        });
      },
    } as any);

    const document = await vscode.workspace.openTextDocument({
      language: "sysml",
      content: "package Drone {}",
    });

    const first = provider.loadDocument(document);
    const second = provider.loadDocument(document);
    resolveRequest?.(createModelResult("Drone"));
    await Promise.all([first, second]);

    assert.strictEqual(requestCount, 1);
    assert.strictEqual(provider.getAllElements().length, 1);
  });

  it("ignores stale document load completions after switching documents", async () => {
    const pending = new Map<string, (value: SysMLModelResult) => void>();
    const provider = new ModelExplorerProvider({
      getModel: async (uri: string) =>
        await new Promise<SysMLModelResult>((resolve) => {
          pending.set(uri, resolve);
        }),
    } as any);

    const firstDocument = await vscode.workspace.openTextDocument({
      language: "sysml",
      content: "package First {}",
    });
    const secondDocument = await vscode.workspace.openTextDocument({
      language: "sysml",
      content: "package Second {}",
    });

    const firstLoad = provider.loadDocument(firstDocument);
    const secondLoad = provider.loadDocument(secondDocument);

    pending.get(secondDocument.uri.toString())?.(createModelResult("Second", "Second"));
    await secondLoad;
    pending.get(firstDocument.uri.toString())?.(createModelResult("First", "First"));
    await firstLoad;

    assert.strictEqual(provider.getLastUri()?.toString(), secondDocument.uri.toString());
    assert.strictEqual(provider.getAllElements()[0]?.name, "Second");
  });

  it("does not trigger a hidden getChildren fetch while a coordinated document load is pending", async () => {
    let requestCount = 0;
    let resolveRequest: ((value: SysMLModelResult) => void) | undefined;
    const provider = new ModelExplorerProvider({
      getModel: async () => {
        requestCount += 1;
        return await new Promise<SysMLModelResult>((resolve) => {
          resolveRequest = resolve;
        });
      },
    } as any);

    const document = await vscode.workspace.openTextDocument({
      language: "sysml",
      content: "package Drone {}",
    });
    await vscode.window.showTextDocument(document);

    const loadPromise = provider.loadDocument(document);
    const rootItems = await provider.getChildren();
    assert.strictEqual(requestCount, 1);
    assert.strictEqual(rootItems[0]?.label, "Loading model...");

    resolveRequest?.(createModelResult("Drone"));
    await loadPromise;
  });

  it("drops stale workspace load completions when a newer run supersedes them", async () => {
    const pending = new Map<string, (value: SysMLModelResult) => void>();
    let callCount = 0;
    const provider = new ModelExplorerProvider({
      getModel: async (uri: string) =>
        await new Promise<SysMLModelResult>((resolve) => {
          callCount += 1;
          pending.set(`${callCount}:${uri}`, resolve);
        }),
    } as any);

    const file = vscode.Uri.parse("file:///workspace/Drone.sysml");
    provider.setWorkspaceViewMode("bySemantic");

    const firstRun = provider.loadWorkspaceModel([file], { runId: "run-1" });
    const secondRun = provider.loadWorkspaceModel([file], { runId: "run-2" });

    pending.get(`2:${file.toString()}`)?.(createModelResult("Second", "Second"));
    const secondResult = await secondRun;
    pending.get(`1:${file.toString()}`)?.(createModelResult("First", "First"));
    const firstResult = await firstRun;

    assert.strictEqual(secondResult.committed, true);
    assert.strictEqual(secondResult.stale, false);
    assert.strictEqual(firstResult.committed, false);
    assert.strictEqual(firstResult.stale, true);
    assert.strictEqual(provider.getAllElements()[0]?.name, "Second");
  });

  it("hides workspace indexing info when switching back to by-file mode", async () => {
    const provider = new ModelExplorerProvider({
      getModel: async () => createModelResult("WorkspaceRoot"),
    } as any);

    provider.setWorkspaceViewMode("bySemantic");
    provider.setWorkspaceLoadStatus({
      state: "indexing",
      scannedFiles: 3,
      loadedFiles: 0,
      cancelled: false,
      failures: 0,
      truncated: false,
    });

    const semanticItems = await provider.getChildren();
    assert.strictEqual(semanticItems[0]?.label, "Workspace indexing in progress");

    provider.setWorkspaceViewMode("byFile");
    const byFileItems = await provider.getChildren();
    assert.strictEqual(byFileItems.length, 0);
  });

  it("does not restart workspace loading on refresh while semantic data is already loaded", async () => {
    let requestCount = 0;
    const provider = new ModelExplorerProvider({
      getModel: async () => {
        requestCount += 1;
        return createModelResult("WorkspaceRoot", "WorkspaceRoot");
      },
    } as any);
    const file = vscode.Uri.parse("file:///workspace/Drone.sysml");
    provider.setWorkspaceViewMode("bySemantic");

    await provider.loadWorkspaceModel([file], { runId: "run-1" });
    provider.refresh();

    assert.strictEqual(requestCount, 1);
    assert.strictEqual(provider.getAllElements()[0]?.name, "WorkspaceRoot");
  });
});
