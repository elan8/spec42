import * as assert from "assert";
import * as vscode from "vscode";
import * as fs from "fs/promises";
import * as os from "os";
import * as path from "path";
import { LSPErrorCodes, ResponseError } from "vscode-languageclient/node";
import { DiagramViewProvider } from "../../diagram/diagramViewer";
import type { LspClientHandles } from "../../activation/lspClient";
import { DIAGRAM_PRODUCT_SCHEMA_VERSION } from "../../generated/diagram-product/schemaVersion";

describe("diagram view publication recovery", () => {
  it("retries a pending view switch after a successful render without a publication notification", async () => {
    const directory = await fs.mkdtemp(path.join(os.tmpdir(), "spec42-view-retry-"));
    const file = path.join(directory, "model.sysml");
    await fs.writeFile(file, "package P;\n");
    const uri = vscode.Uri.file(file).toString();
    const messages: Array<{ type: string; selectedHandle?: string; error?: string; header?: string }> = [];
    let secondViewRequests = 0;
    const disposable = () => new vscode.Disposable(() => undefined);
    const catalog = {
      modelDigest: "model",
      views: ["first", "second"].map((name) => ({
        handle: name, kind: "general-view", name, namespace: "P", source: { uri },
        reference: { kind: "qualified-name", document: uri, qualifiedName: `P::${name}`, sourceDomain: "workspace" },
      })),
    };
    const client = {
      onNotification: disposable,
      onDidChangeState: disposable,
      sendRequest: async (method: string, params: { handle?: string }) => {
        if (method === "spec42/diagramViews") return catalog;
        assert.equal(method, "spec42/diagram");
        if (params.handle === "second" && ++secondViewRequests === 1) {
          throw new ResponseError(LSPErrorCodes.ContentModified, "Content modified");
        }
        return { modelDigest: "model", durationUs: 10, productJson: JSON.stringify({
          schemaVersion: DIAGRAM_PRODUCT_SCHEMA_VERSION, modelDigest: "model",
          documents: [{ uri, sourceDomain: "workspace" }], sources: [{ document: 0, range: [0, 0, 0, 10] }],
          references: [{ kind: "qualified-name", document: 0, qualifiedName: `P::${params.handle}` }],
          selectedView: { reference: 0, kind: "general-view", name: params.handle, source: 0 },
          completeness: { status: "complete", reasons: [] },
          projection: { kind: "general-view", exposedRoots: [], nodes: [], relationships: [], edges: [], metadata: { roots: [] }, scene: { kind: "general" } },
        }) };
      },
    };
    const provider = new DiagramViewProvider({} as vscode.ExtensionContext, { client } as unknown as LspClientHandles);
    // Supply the host surface directly so this lifecycle test does not open a real side panel.
    const host = provider as unknown as {
      view: vscode.WebviewView; anchorUri: string; webviewReady: boolean;
      regenerate(reason: "manual"): Promise<void>;
      onWebviewMessage(message: unknown): void;
    };
    host.view = { visible: true, webview: { postMessage: async (message: typeof messages[number]) => { messages.push(message); return true; } } } as unknown as vscode.WebviewView;
    host.anchorUri = uri;
    host.webviewReady = true;
    try {
      await host.regenerate("manual");
      assert.ok(messages.some((message) => message.type === "render" && message.selectedHandle === "first"));
      messages.length = 0;
      host.onWebviewMessage({ type: "switchView", handle: "second" });
      const deadline = Date.now() + 5_000;
      while (!messages.some((message) => message.type === "render" && message.selectedHandle === "second" && !message.error)) {
        assert.ok(Date.now() < deadline, "the selected view should recover automatically");
        await new Promise((resolve) => setTimeout(resolve, 20));
      }
      assert.equal(secondViewRequests, 2);
      assert.ok(messages.some((message) => message.error === "Updating view…"));
      assert.ok(messages.every((message) => !message.error?.includes("Content modified")));
    } finally {
      provider.dispose();
      await fs.rm(directory, { recursive: true, force: true });
    }
  });
});
