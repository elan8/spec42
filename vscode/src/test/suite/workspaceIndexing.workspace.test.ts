import * as assert from "assert";
import * as vscode from "vscode";
import {
  configureServerForTests,
  getTestWorkspaceFolder,
  getFixturePath,
  waitFor,
  waitForLanguageServerReady,
} from "./testUtils";

type DebugExtensionState = {
  serverHealthState: "starting" | "ready" | "indexing" | "degraded" | "restarting" | "crashed";
  serverHealthDetail: string;
  workspaceIndexSummary?: {
    scannedFiles: number;
    loadedFiles: number;
    truncated: boolean;
    cancelled: boolean;
    failures?: number;
  };
};

describe("Workspace Indexing Smoke Test", () => {
  before(async function () {
    this.timeout(30000);
    await configureServerForTests();
    getTestWorkspaceFolder();
    const doc = await vscode.workspace.openTextDocument(getFixturePath("Alpha.sysml"));
    await waitForLanguageServerReady(doc, 25000);
  });

  after(async () => {
    await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    await new Promise((r) => setTimeout(r, 250));
  });

  it("keeps the workspace usable while workspace indexing completes", async function () {
    this.timeout(30000);
    const doc = await vscode.workspace.openTextDocument(getFixturePath("Alpha.sysml"));
    await vscode.window.showTextDocument(doc);

    const state = await waitFor(
      "workspace indexing state",
      () =>
        vscode.commands.executeCommand<DebugExtensionState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        Boolean(
          value?.workspaceIndexSummary &&
          value.workspaceIndexSummary.scannedFiles > 0 &&
          value.workspaceIndexSummary.loadedFiles > 0
        ),
      25000,
      300
    );

    assert.ok(state.workspaceIndexSummary, "Expected workspace indexing summary");
    assert.ok(
      state.serverHealthState === "degraded" || state.serverHealthState === "ready",
      `Expected usable server health state, got ${state.serverHealthState}`
    );

    const hovers = await waitFor(
      "hover after truncated indexing",
      () =>
        vscode.commands.executeCommand<vscode.Hover[]>(
          "vscode.executeHoverProvider",
          doc.uri,
          doc.positionAt(doc.getText().indexOf("part def AlphaPart"))
        ),
      (value) => Array.isArray(value) && value.length > 0
    );
    assert.ok(hovers.length > 0, "Hover should keep working in a truncated workspace");
  });

  it("starts workspace indexing when switching to semantic model mode", async function () {
    this.timeout(30000);
    await vscode.commands.executeCommand("sysml.switchToByFile");

    await vscode.commands.executeCommand("sysml.switchToSemanticModel");

    const state = await waitFor(
      "semantic workspace indexing state",
      () =>
        vscode.commands.executeCommand<DebugExtensionState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        Boolean(
          value?.workspaceIndexSummary &&
          value.workspaceIndexSummary.scannedFiles > 0
        ),
      25000,
      300
    );

    assert.ok(state.workspaceIndexSummary, "Expected workspace indexing summary after switching to semantic mode");
    assert.ok(
      state.workspaceIndexSummary.scannedFiles > 0,
      "Expected semantic mode to trigger workspace scanning"
    );
  });

  it("starts workspace indexing when switching to by-file mode", async function () {
    this.timeout(30000);
    await vscode.commands.executeCommand("sysml.switchToSemanticModel");

    await vscode.commands.executeCommand("sysml.switchToByFile");

    const state = await waitFor(
      "by-file workspace indexing state",
      () =>
        vscode.commands.executeCommand<DebugExtensionState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        Boolean(
          value?.workspaceIndexSummary &&
          value.workspaceIndexSummary.scannedFiles > 0
        ),
      25000,
      300
    );

    assert.ok(state.workspaceIndexSummary, "Expected workspace indexing summary after switching to by-file mode");
    assert.ok(
      state.workspaceIndexSummary.scannedFiles > 0,
      "Expected by-file mode to remain workspace-backed"
    );
  });
});
