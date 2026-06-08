import * as assert from "assert";
import * as vscode from "vscode";
import {
  closeAllEditorsForTests,
  configureServerForTests,
  getTestWorkspaceFolder,
  getFixturePath,
  integrationHookTimeoutMs,
  isCi,
  waitFor,
  waitForLanguageServerReady,
} from "./testUtils";

type DebugExtensionState = {
  serverHealthState: "starting" | "ready" | "indexing" | "degraded" | "restarting" | "crashed";
  serverHealthDetail: string;
  lastLoadedSemanticStateVersion?: number;
  lastSemanticIndexReadyWorkspaceFileCount?: number;
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
    this.timeout(integrationHookTimeoutMs);
    await configureServerForTests();
    getTestWorkspaceFolder();
    const doc = await vscode.workspace.openTextDocument(getFixturePath("Alpha.sysml"));
    await waitForLanguageServerReady(doc);
  });

  after(async () => {
    await closeAllEditorsForTests();
  });

  it("loads workspace model automatically when semantic index is ready", async function () {
    this.timeout(45000);
    const state = await waitFor(
      "semantic index ready and workspace model loaded",
      () =>
        vscode.commands.executeCommand<DebugExtensionState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        Boolean(
          value?.lastLoadedSemanticStateVersion !== undefined &&
          (value.lastLoadedSemanticStateVersion ?? 0) > 0 &&
          value.workspaceIndexSummary &&
          value.workspaceIndexSummary.loadedFiles >= 2
        ),
      40000,
      300
    );
    assert.ok(
      state.lastLoadedSemanticStateVersion,
      "Expected semantic index version after auto workspace reload"
    );
    assert.ok(
      state.workspaceIndexSummary &&
        state.workspaceIndexSummary.loadedFiles >= 2,
      "Expected multi-file workspace model without manual refresh"
    );
  });

  it("keeps the workspace usable while workspace indexing completes", async function () {
    this.timeout(isCi ? 60000 : 30000);
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
    this.timeout(isCi ? 60000 : 30000);
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
    this.timeout(isCi ? 60000 : 30000);
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
