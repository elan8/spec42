import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import { VisualizationPanel } from "../../visualization/visualizationPanel";
import {
  configureServerForTests,
  getFixturePath,
  getTestWorkspaceFolder,
  waitFor,
  waitForLanguageServerReady,
} from "./testUtils";

const FIXTURE_FILE = "SurveillanceDrone.sysml";
type ExtensionDebugState = Awaited<ReturnType<typeof vscode.commands.executeCommand>> & {
  featureInspector?: {
    state: "inactive" | "loading" | "ready" | "empty" | "error";
    selectedElementId?: string;
    selectedElementName?: string;
    emptyMessage?: string;
    errorMessage?: string;
    lastRequestedUri?: string;
    lastRequestedPosition?: { line: number; character: number };
  };
};

function findPosition(doc: vscode.TextDocument, needle: string, occurrence = 0): vscode.Position {
  const text = doc.getText();
  let from = 0;
  let index = -1;
  for (let i = 0; i <= occurrence; i += 1) {
    index = text.indexOf(needle, from);
    assert.ok(index >= 0, `Could not find "${needle}" in ${doc.fileName}`);
    from = index + needle.length;
  }
  return doc.positionAt(index);
}

function findPositionWithinMatch(
  doc: vscode.TextDocument,
  needle: string,
  innerNeedle: string,
  occurrence = 0
): vscode.Position {
  const base = findPosition(doc, needle, occurrence);
  const innerOffset = needle.indexOf(innerNeedle);
  assert.ok(innerOffset >= 0, `Could not find "${innerNeedle}" inside "${needle}"`);
  return base.translate(0, innerOffset);
}

describe("Extension Test Suite", () => {
  before(async function () {
    this.timeout(30000);
    await configureServerForTests();
    getTestWorkspaceFolder();
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    await waitForLanguageServerReady(doc);
  });

  afterEach(async () => {
    if (VisualizationPanel.currentPanel) {
      VisualizationPanel.currentPanel.dispose();
    }
    await vscode.commands.executeCommand("workbench.action.closeAllEditors");
  });

  after(async () => {
    if (VisualizationPanel.currentPanel) {
      VisualizationPanel.currentPanel.dispose();
    }
    await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    await new Promise((r) => setTimeout(r, 250));
  });

  it("Extension should be present", () => {
    const found = vscode.extensions.all.some(
      (e) => e.packageJSON?.name === "spec42"
    );
    assert.ok(found, "SysML Language Server extension should be loaded");
  });

  it("SysML language should be registered", async () => {
    const languages = await vscode.languages.getLanguages();
    assert.ok(
      languages.includes("sysml"),
      "sysml language should be registered"
    );
  });

  it("Hierarchy commands should be registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("sysml.showTypeHierarchy"));
    assert.ok(commands.includes("sysml.showCallHierarchy"));
    assert.ok(commands.includes("sysml.featureInspector.refresh"));
  });

  it("Feature Inspector tracks active SysML selection", async function () {
    this.timeout(20000);
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    const editor = await vscode.window.showTextDocument(doc);
    const position = findPosition(doc, "part def Airframe");
    editor.selection = new vscode.Selection(position, position);

    const state = await waitFor(
      "feature inspector selection sync",
      () =>
        vscode.commands.executeCommand<ExtensionDebugState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        value?.featureInspector?.state === "ready" &&
        value.featureInspector.selectedElementName === "Airframe"
    );
    assert.strictEqual(state.featureInspector?.selectedElementName, "Airframe");
    assert.strictEqual(state.featureInspector?.lastRequestedUri, doc.uri.toString());
  });

  it("Feature Inspector clears for non-SysML editors", async function () {
    this.timeout(20000);
    const txtDoc = await vscode.workspace.openTextDocument({
      language: "plaintext",
      content: "just text",
    });
    await vscode.window.showTextDocument(txtDoc);

    const state = await waitFor(
      "feature inspector inactive state",
      () =>
        vscode.commands.executeCommand<ExtensionDebugState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        value?.featureInspector?.state === "inactive" &&
        value.featureInspector.emptyMessage?.includes("Open a SysML/KerML file") === true
    );
    assert.strictEqual(state.featureInspector?.state, "inactive");
  });

  it("Feature Inspector shows empty state for whitespace", async function () {
    this.timeout(20000);
    const doc = await vscode.workspace.openTextDocument({
      language: "sysml",
      content: "package Demo {\n  part def Engine;\n}\n",
    });
    const editor = await vscode.window.showTextDocument(doc);
    const position = new vscode.Position(3, 0);
    editor.selection = new vscode.Selection(position, position);

    const state = await waitFor(
      "feature inspector empty state",
      () =>
        vscode.commands.executeCommand<ExtensionDebugState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        value?.featureInspector?.state === "empty" &&
        value.featureInspector.emptyMessage?.includes("No inspectable element") === true
    );
    assert.strictEqual(state.featureInspector?.state, "empty");
  });

  it("Feature Inspector navigation command opens and reveals target range", async function () {
    this.timeout(20000);
    const doc = await vscode.workspace.openTextDocument(getFixturePath(FIXTURE_FILE));
    await vscode.window.showTextDocument(doc);
    const position = findPosition(doc, "part def Airframe");
    const targetRange = new vscode.Range(position, position.translate(0, "part def Airframe".length));

    await vscode.commands.executeCommand("sysml.featureInspector.openReference", {
      uri: doc.uri.toString(),
      range: {
        start: { line: targetRange.start.line, character: targetRange.start.character },
        end: { line: targetRange.end.line, character: targetRange.end.character },
      },
    });

    const activeEditor = vscode.window.activeTextEditor;
    assert.ok(activeEditor, "Expected an active editor after navigation");
    assert.strictEqual(activeEditor?.document.uri.toString(), doc.uri.toString());
    assert.strictEqual(activeEditor?.selection.active.line, targetRange.start.line);
    assert.strictEqual(activeEditor?.selection.active.character, targetRange.start.character);
  });

  it("Feature Inspector keeps latest selection after rapid cursor changes", async function () {
    this.timeout(20000);
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    const editor = await vscode.window.showTextDocument(doc);
    const first = findPosition(doc, "part def Airframe");
    const second = findPosition(doc, "part def PropulsionUnit");
    editor.selection = new vscode.Selection(first, first);
    editor.selection = new vscode.Selection(second, second);

    const state = await waitFor(
      "feature inspector final rapid selection state",
      () =>
        vscode.commands.executeCommand<ExtensionDebugState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        value?.featureInspector?.state === "ready" &&
        value.featureInspector.selectedElementName === "PropulsionUnit"
    );
    assert.strictEqual(state.featureInspector?.selectedElementName, "PropulsionUnit");
  });

  it("Hover over keyword returns content", async () => {
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    await vscode.window.showTextDocument(doc);
    const position = findPosition(doc, "part def Airframe");
    const hovers = await waitFor(
      "hover provider response",
      () =>
        vscode.commands.executeCommand<vscode.Hover[]>(
          "vscode.executeHoverProvider",
          doc.uri,
          position
        ),
      (value) => Array.isArray(value) && value.length > 0
    );
    const content = hovers[0].contents;
    const value = Array.isArray(content)
      ? content.map((c) => (typeof c === "string" ? c : c.value)).join("")
      : typeof content === "string"
        ? content
        : (content as { value: string }).value;
    assert.ok(
      value.toLowerCase().includes("part"),
      `Hover content should mention 'part': ${value}`
    );
  });

  it("Go to definition from usage to definition", async () => {
    const workspaceRoot = getTestWorkspaceFolder().uri.fsPath;
    const defPath = path.resolve(workspaceRoot, "..", "multi-file", "def.sysml");
    const usePath = path.resolve(workspaceRoot, "..", "multi-file", "use.sysml");
    const defDoc = await vscode.workspace.openTextDocument(defPath);
    await waitForLanguageServerReady(defDoc);
    const useDoc = await vscode.workspace.openTextDocument(usePath);
    await vscode.window.showTextDocument(useDoc);
    await waitForLanguageServerReady(useDoc);
    const locations = await waitFor(
      "definition provider response",
      () =>
        vscode.commands.executeCommand<vscode.Location[]>(
          "vscode.executeDefinitionProvider",
          useDoc.uri,
          findPosition(useDoc, "Widget")
        ),
      (value) => Array.isArray(value) && value.length > 0
    );
    assert.strictEqual(
      path.basename(locations[0].uri.fsPath),
      "def.sysml",
      "Definition should resolve to def.sysml"
    );
  });

  it("Hierarchy commands execute for SysML and KerML editors", async function () {
    this.timeout(20000);
    const sysmlDoc = await vscode.workspace.openTextDocument(getFixturePath(FIXTURE_FILE));
    const sysmlEditor = await vscode.window.showTextDocument(sysmlDoc);
    sysmlEditor.selection = new vscode.Selection(
      findPosition(sysmlDoc, "part def Airframe"),
      findPosition(sysmlDoc, "part def Airframe")
    );

    await vscode.commands.executeCommand("sysml.showTypeHierarchy");
    await vscode.commands.executeCommand("sysml.showCallHierarchy");

    const kermlDoc = await vscode.workspace.openTextDocument({
      language: "kerml",
      content: "package KernelPackage { part def KernelPart; }",
    });
    const kermlEditor = await vscode.window.showTextDocument(kermlDoc);
    kermlEditor.selection = new vscode.Selection(
      new vscode.Position(0, 28),
      new vscode.Position(0, 28)
    );

    await vscode.commands.executeCommand("sysml.showTypeHierarchy");
    await vscode.commands.executeCommand("sysml.showCallHierarchy");
  });

  it("Server stays usable after invalid intermediate edits", async function () {
    this.timeout(20000);
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    const editor = await vscode.window.showTextDocument(doc);

    const invalidEditApplied = await editor.edit((editBuilder) => {
      editBuilder.insert(
        new vscode.Position(doc.lineCount, 0),
        "\n}\n"
      );
    });
    assert.ok(invalidEditApplied, "Expected invalid intermediate edit to apply");

    const diagnostics = await waitFor(
      "diagnostics after invalid edit",
      async () => vscode.languages.getDiagnostics(doc.uri),
      (value) => Array.isArray(value) && value.length > 0
    );
    assert.ok(diagnostics.length > 0, "Expected diagnostics after invalid intermediate edit");

    const hoverPosition = findPosition(doc, "part def Airframe");
    const hovers = await waitFor(
      "hover after invalid edit",
      () =>
        vscode.commands.executeCommand<vscode.Hover[]>(
          "vscode.executeHoverProvider",
          doc.uri,
          hoverPosition
        ),
      (value) => Array.isArray(value) && value.length > 0
    );
    assert.ok(hovers.length > 0, "Server should still answer hover requests after invalid edits");

    await vscode.commands.executeCommand("workbench.action.files.revert");
  });

  it("Server recovers after manual restart", async function () {
    this.timeout(20000);
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    await vscode.window.showTextDocument(doc);

    await vscode.commands.executeCommand("sysml.restartServer");

    const hovers = await waitFor(
      "hover after manual restart",
      () =>
        vscode.commands.executeCommand<vscode.Hover[]>(
          "vscode.executeHoverProvider",
          doc.uri,
          findPosition(doc, "part def Airframe")
        ),
      (value) => Array.isArray(value) && value.length > 0
    );
    assert.ok(hovers.length > 0, "Server should recover after manual restart");
  });
});
