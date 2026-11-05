import * as assert from "assert";
import * as vscode from "vscode";
import { State } from "vscode-languageclient/node";
import { DiagramViewProvider } from "../../diagram/diagramViewer";
import type { LspClientHandles } from "../../activation/lspClient";
import type { DiagramPresentationParams, DiagramPresentationResult } from "../../generated/diagram-presentation";
import { ANALYSIS_STATUS_CHANGED_METHOD } from "../../generated/editor-status";
import { DIAGRAM_PRODUCT_SCHEMA_VERSION } from "../../generated/diagram-product/schemaVersion";

const uri = "file:///workspace/model.sysml";
const reference = (name: string) => ({ kind: "qualified-name" as const, document: uri, qualifiedName: `P::${name}`, sourceDomain: "workspace" });
function ready(name = "first", version = "1"): DiagramPresentationResult {
  return {
    state: "ready", semanticStateVersion: version, modelDigest: "model", selectedHandle: name,
    selection: { kind: "general-view", reference: reference(name) }, durationUs: 10, expanded: [],
    views: ["first", "second"].map((name) => ({ handle: name, kind: "general-view", reference: reference(name), name, namespace: "P", source: { uri, range: { startLine: 0, startCharacter: 0, endLine: 0, endCharacter: 10 } } })),
    productJson: JSON.stringify({
      schemaVersion: DIAGRAM_PRODUCT_SCHEMA_VERSION, modelDigest: "model",
      documents: [{ uri, sourceDomain: "workspace" }], sources: [{ document: 0, range: [0, 0, 0, 10] }],
      references: [{ kind: "qualified-name", document: 0, qualifiedName: `P::${name}` }],
      selectedView: { reference: 0, kind: "general-view", name, source: 0 },
      completeness: { status: "complete", reasons: [] },
      projection: { kind: "general-view", exposedRoots: [], nodes: [], relationships: [], edges: [], metadata: { roots: [] }, scene: { kind: "general" } },
    }),
  };
}
function deferred<T>() {
  let resolve!: (value: T) => void;
  return { promise: new Promise<T>((done) => { resolve = done; }), resolve: (value: T) => resolve(value) };
}
async function until(condition: () => boolean): Promise<void> {
  const deadline = Date.now() + 5_000;
  while (!condition()) {
    assert.ok(Date.now() < deadline, "expected lifecycle transition did not complete");
    await new Promise((done) => setTimeout(done, 10));
  }
}
class Harness {
  readonly messages: Array<{ type: string; selectedHandle?: string; error?: string; header?: string }> = [];
  readonly requests: DiagramPresentationParams[] = [];
  private readonly notifications = new Map<string, () => void>();
  private stateListener: ((event: { newState: State }) => void) | undefined;
  private readonly client;
  readonly provider: DiagramViewProvider;
  readonly host;
  handler: (params: DiagramPresentationParams) => Promise<DiagramPresentationResult> = async () => ready();
  constructor() {
    this.client = {
      state: State.Running,
      onNotification: (method: string, listener: () => void) => { this.notifications.set(method, listener); return new vscode.Disposable(() => this.notifications.delete(method)); },
      onDidChangeState: (listener: (event: { newState: State }) => void) => { this.stateListener = listener; return new vscode.Disposable(() => { this.stateListener = undefined; }); },
      sendRequest: async (method: string, params: DiagramPresentationParams) => {
        assert.equal(method, "spec42/diagramPresentation", "catalog and product use one request");
        this.requests.push(params);
        return this.handler(params);
      },
    };
    this.provider = new DiagramViewProvider({} as vscode.ExtensionContext, { client: this.client } as unknown as LspClientHandles);
    this.host = this.provider as unknown as {
      view: vscode.WebviewView; webviewReady: boolean;
      resolveAnchorUri(): Promise<string>; requestRefresh(): void; onWebviewMessage(message: unknown): void;
      cancelPresentation(): void;
    };
    this.host.view = { visible: true, webview: { postMessage: async (message: typeof this.messages[number]) => { this.messages.push(message); return true; } } } as unknown as vscode.WebviewView;
    this.host.webviewReady = true;
    this.host.resolveAnchorUri = async () => uri;
  }
  refresh(): void { this.host.requestRefresh(); }
  wake(): void { this.notifications.get(ANALYSIS_STATUS_CHANGED_METHOD)!(); }
  switch(name: string): void { this.host.onWebviewMessage({ type: "switchView", handle: name }); }
  state(state: State): void { this.client.state = state; this.stateListener?.({ newState: state }); }
  renders(name?: string): number { return this.messages.filter((message) => message.type === "render" && message.header === "Complete projection" && (!name || message.selectedHandle === name)).length; }
  dispose(): void { this.provider.dispose(); }
}

describe("diagram presentation lifecycle", () => {
  let harness: Harness;
  beforeEach(() => { harness = new Harness(); });
  afterEach(() => harness.dispose());

  it("waits for a completion wake without polling or opening a source editor", async () => {
    let opened = 0;
    const listener = vscode.workspace.onDidOpenTextDocument(() => { opened++; });
    try {
      harness.handler = async () => ({ state: "updating", semanticStateVersion: "1" });
      harness.refresh();
      await until(() => harness.messages.some((message) => message.header === "Updating view…"));
      await new Promise((done) => setTimeout(done, 750));
      assert.equal(harness.requests.length, 1);
      harness.handler = async () => ready();
      harness.wake();
      await until(() => harness.renders() === 1);
      assert.equal(harness.requests.length, 2);
      assert.equal(opened, 0);
    } finally { listener.dispose(); }
  });

  it("does not lose a completion delivered before the pending response", async () => {
    const pending = deferred<DiagramPresentationResult>();
    harness.handler = () => pending.promise;
    harness.refresh();
    await until(() => harness.requests.length === 1);
    harness.handler = async () => ready();
    harness.wake();
    pending.resolve({ state: "updating", semanticStateVersion: "1" });
    await until(() => harness.renders() === 1);
    assert.equal(harness.requests.length, 2);
  });

  it("refreshes an empty catalog after a same-digest lifecycle completion", async () => {
    harness.handler = async () => ({ state: "empty", semanticStateVersion: "1", modelDigest: "model" });
    harness.refresh();
    await until(() => harness.messages.some((message) => message.header?.startsWith("No views are available")));
    harness.handler = async () => ready("first", "2");
    harness.wake();
    await until(() => harness.renders("first") === 1);
  });

  it("discards an obsolete selection response and coalesces the latest selection", async () => {
    harness.refresh();
    await until(() => harness.renders("first") === 1);
    const old = deferred<DiagramPresentationResult>();
    harness.handler = () => old.promise;
    harness.switch("second");
    await until(() => harness.requests.length === 2);
    harness.switch("first");
    harness.handler = async () => ready("first", "3");
    old.resolve(ready("second", "2"));
    await until(() => harness.renders("first") === 2);
    assert.equal(harness.renders("second"), 0);
    assert.equal(harness.requests.length, 3);
  });

  it("rechecks explicit supersession once, without a retry delay", async () => {
    harness.handler = async () => harness.requests.length === 1 ? { state: "superseded" } : ready();
    harness.refresh();
    await until(() => harness.renders() === 1);
    assert.equal(harness.requests.length, 2);
  });

  it("retains the frame for unchanged publications and sends the complete product prerequisites", async () => {
    harness.refresh();
    await until(() => harness.renders() === 1);
    harness.handler = async () => ({ state: "unchanged", semanticStateVersion: "2" });
    harness.wake();
    await until(() => harness.messages.some((message) => message.type === "status" && message.header === "Complete projection"));
    assert.equal(harness.renders(), 1);
    assert.deepStrictEqual(harness.requests[1].knownProduct, { modelDigest: "model", selection: { kind: "general-view", reference: reference("first") }, expanded: [] });
  });

  it("surfaces failures and recovers on a lifecycle wake even with an unchanged digest", async () => {
    harness.handler = async () => ({ state: "failed", semanticStateVersion: "1", message: "Construction failed" });
    harness.refresh();
    await until(() => harness.messages.some((message) => message.header === "Construction failed"));
    await new Promise((done) => setTimeout(done, 750));
    assert.equal(harness.requests.length, 1);
    harness.handler = async () => ready();
    harness.wake();
    await until(() => harness.renders() === 1);
  });

  it("cancels old request ownership across server restart", async () => {
    const old = deferred<DiagramPresentationResult>();
    harness.handler = () => old.promise;
    harness.refresh();
    await until(() => harness.requests.length === 1);
    harness.state(State.Stopped);
    harness.handler = async () => ready("first", "3");
    harness.state(State.Running);
    await until(() => harness.renders("first") === 1);
    old.resolve(ready("second", "1"));
    await new Promise((done) => setTimeout(done, 25));
    assert.equal(harness.renders("second"), 0);
  });
});

describe("diagram view title actions", () => {
  let harness: Harness;
  beforeEach(() => { harness = new Harness(); });
  afterEach(() => harness.dispose());
  const actions = () => (harness.provider as unknown as { actionContext?: { hasFrame: boolean; canGoToDefinition: boolean } }).actionContext;

  it("enables export once a frame exists and definition navigation only while it is current", async () => {
    harness.handler = async () => ({ state: "updating", semanticStateVersion: "1" });
    harness.refresh();
    await until(() => actions() !== undefined);
    assert.deepStrictEqual(actions(), { hasFrame: false, canGoToDefinition: false });
    harness.handler = async () => ready();
    harness.wake();
    await until(() => harness.renders() === 1);
    assert.deepStrictEqual(actions(), { hasFrame: true, canGoToDefinition: true });
    // A retained frame stays exportable while the next publication is being built.
    harness.handler = async () => ({ state: "updating", semanticStateVersion: "2" });
    harness.wake();
    await until(() => actions()?.canGoToDefinition === false);
    assert.deepStrictEqual(actions(), { hasFrame: true, canGoToDefinition: false });
  });

  it("fits the view and exports the format chosen in the quick pick", async () => {
    harness.refresh();
    await until(() => harness.renders() === 1);
    harness.provider.home();
    assert.ok(harness.messages.some((message) => message.type === "resetView"));

    const window = vscode.window as unknown as { showQuickPick: typeof vscode.window.showQuickPick };
    const original = window.showQuickPick;
    const offered: string[][] = [];
    let pick = 3;
    window.showQuickPick = (async (items: readonly vscode.QuickPickItem[]) => {
      offered.push(items.map((item) => item.label));
      return items[pick];
    }) as unknown as typeof vscode.window.showQuickPick;
    try {
      await harness.provider.exportView();
      assert.deepStrictEqual(harness.messages.at(-1), { type: "requestExport", format: "png", transparent: true });
      assert.equal(offered[0].length, 5);
      pick = 0;
      await harness.provider.exportView();
      assert.deepStrictEqual(harness.messages.at(-1), { type: "requestExport", format: "svg", transparent: false });
      const count = harness.messages.length;
      pick = -1;
      await harness.provider.exportView();
      assert.equal(harness.messages.length, count, "a dismissed quick pick exports nothing");
    } finally { window.showQuickPick = original; }
  });

  it("contributes Home, Go to definition and Export to the view title in that order", () => {
    const manifest = require("../../../package.json") as {
      contributes: { commands: Array<{ command: string; icon?: string; enablement?: string }>; menus: { "view/title": Array<{ command: string; when?: string; group?: string }> } };
    };
    const titles = manifest.contributes.menus["view/title"].filter((entry) => entry.when === "view == spec42DiagramView");
    assert.deepStrictEqual(titles.map((entry) => [entry.command, entry.group]), [
      ["spec42.diagram.home", "navigation@1"],
      ["spec42.diagram.goToDefinition", "navigation@2"],
      ["spec42.diagram.export", "navigation@3"],
    ]);
    const command = (id: string) => manifest.contributes.commands.find((entry) => entry.command === id)!;
    assert.equal(command("spec42.diagram.home").icon, "$(home)");
    assert.equal(command("spec42.diagram.goToDefinition").enablement, "spec42.diagram.canGoToDefinition");
    assert.equal(command("spec42.diagram.export").enablement, "spec42.diagram.hasFrame");
  });
});
