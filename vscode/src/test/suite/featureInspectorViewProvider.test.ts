import * as assert from "assert";
import * as vscode from "vscode";
import { FeatureInspectorViewProvider } from "../../inspector/featureInspectorViewProvider";
import type {
  FeatureInspectorResult,
  LspModelProvider,
} from "../../providers/lspModelProvider";

class FakeWebview {
  options: vscode.WebviewOptions = {};
  html = "";
  readonly cspSource = "test-webview";
  readonly messages: unknown[] = [];
  private receiveMessage: ((message: unknown) => unknown) | undefined;

  asWebviewUri(uri: vscode.Uri): vscode.Uri {
    return uri;
  }

  onDidReceiveMessage(listener: (message: unknown) => unknown): vscode.Disposable {
    this.receiveMessage = listener;
    return new vscode.Disposable(() => {
      this.receiveMessage = undefined;
    });
  }

  postMessage(message: unknown): Thenable<boolean> {
    this.messages.push(message);
    return Promise.resolve(true);
  }

  async fireMessage(message: unknown): Promise<void> {
    await this.receiveMessage?.(message);
  }
}

function fakeView(webview: FakeWebview): vscode.WebviewView {
  return {
    webview,
    visible: true,
    onDidChangeVisibility: () => new vscode.Disposable(() => undefined),
    onDidDispose: () => new vscode.Disposable(() => undefined),
  } as unknown as vscode.WebviewView;
}

function inspectorResult(name: string): FeatureInspectorResult {
  return {
    version: 0,
    sourceUri: "file:///model.sysml",
    requestedPosition: { line: 1, character: 2 },
    element: {
      id: `P::${name}`,
      name,
      qualifiedName: `P::${name}`,
      type: "part",
      uri: "file:///model.sysml",
      range: {
        start: { line: 1, character: 2 },
        end: { line: 1, character: 10 },
      },
      attributes: {},
      typing: { status: "notApplicable", targets: [] },
      specialization: { status: "notApplicable", targets: [] },
      incomingRelationships: [],
      outgoingRelationships: [],
    },
  };
}

describe("FeatureInspectorViewProvider", () => {
  it("replays a pinned inspection that completed before the lazy webview was ready", async () => {
    const result = inspectorResult("motor");
    const modelProvider = {
      getFeatureInspector: async () => result,
    } as unknown as LspModelProvider;
    const provider = new FeatureInspectorViewProvider(
      vscode.Uri.file("C:/spec42"),
      modelProvider
    );

    await provider.inspectAt(result.sourceUri, result.requestedPosition);

    const webview = new FakeWebview();
    provider.resolveWebviewView(fakeView(webview));
    assert.deepStrictEqual(webview.messages, []);

    await webview.fireMessage({ type: "ready" });

    assert.deepStrictEqual(webview.messages, [
      { type: "update", payload: result, pinned: true },
    ]);
  });

  it("requests the current cursor when an unpinned webview reports ready", async () => {
    const provider = new FeatureInspectorViewProvider(
      vscode.Uri.file("C:/spec42"),
      {} as LspModelProvider
    );
    let resumeRequests = 0;
    provider.onResumeRequested(() => {
      resumeRequests += 1;
    });

    const webview = new FakeWebview();
    provider.resolveWebviewView(fakeView(webview));
    await webview.fireMessage({ type: "ready" });

    assert.strictEqual(resumeRequests, 1);
  });
});
