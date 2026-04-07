import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import {
  buildElementPresentation,
  type ElementMetadata,
  type ElementPresentationContext,
} from "../../explorer/modelExplorerProvider";
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
  modelExplorer?: {
    lastRevealedElementId?: string;
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
    assert.ok(commands.includes("sysml.refreshModelTree"));
    assert.ok(!commands.includes("sysml.featureInspector.refresh"));
  });

  it("Snippet pack exposes editing baseline scaffolds", () => {
    const extension = vscode.extensions.all.find(
      (candidate) => candidate.packageJSON?.name === "spec42"
    );
    assert.ok(extension, "Expected spec42 extension metadata");
    const snippetContributions = extension.packageJSON?.contributes?.snippets;
    assert.ok(Array.isArray(snippetContributions), "Expected snippet contributions");
    assert.ok(
      snippetContributions.some((entry: { language: string; path: string }) => entry.language === "sysml" && entry.path === "./snippets/sysml.json"),
      "Expected SysML snippet contribution"
    );
    assert.ok(
      snippetContributions.some((entry: { language: string; path: string }) => entry.language === "kerml" && entry.path === "./snippets/sysml.json"),
      "Expected KerML snippet contribution"
    );

    const snippetPath = path.join(extension.extensionPath, "snippets", "sysml.json");
    const snippetText = require("fs").readFileSync(snippetPath, "utf8");
    const snippets = JSON.parse(snippetText) as Record<string, unknown>;
    assert.ok(snippets["SysML: package with imports"]);
    assert.ok(snippets["SysML: state machine skeleton"]);
    assert.ok(snippets["SysML: architecture system skeleton"]);
    assert.ok(snippets["SysML: multi-file usage skeleton"]);
  });

  it("Model Explorer reveals the selected source element", async function () {
    this.timeout(20000);
    const filePath = getFixturePath(FIXTURE_FILE);
    const doc = await vscode.workspace.openTextDocument(filePath);
    const editor = await vscode.window.showTextDocument(doc);
    const position = findPosition(doc, "part def PropulsionUnit");
    editor.selection = new vscode.Selection(position, position);

    const state = await waitFor(
      "model explorer source selection sync",
      () =>
        vscode.commands.executeCommand<ExtensionDebugState>(
          "sysml.debug.getExtensionState"
        ),
      (value) =>
        value?.modelExplorer?.lastRevealedElementId?.includes("PropulsionUnit") === true
    );
    assert.ok(state.modelExplorer?.lastRevealedElementId?.includes("PropulsionUnit"));
  });

  it("Model Explorer presentation summarizes typing, multiplicity, and source context", () => {
    const uri = vscode.Uri.file("C:/Git/spec42/example.sysml");
    const typeUri = vscode.Uri.file("C:/Git/spec42/types.sysml");
    const metadataById = new Map<string, ElementMetadata>([
      [
        "Drone::Airframe",
        {
          reference: {
            id: "Drone::Airframe",
            name: "Airframe",
            type: "part def",
            uri: typeUri,
            range: {
              start: { line: 1, character: 0 },
              end: { line: 1, character: 16 },
            },
          },
        },
      ],
    ]);
    const context: ElementPresentationContext = {
      activeUri: vscode.Uri.file("C:/Git/spec42/other.sysml"),
      metadataById,
      incomingRelationshipCounts: new Map([["Drone::body", 2]]),
    };
    const presentation = buildElementPresentation(
      {
        id: "Drone::body",
        type: "part",
        name: "body",
        range: {
          start: { line: 5, character: 2 },
          end: { line: 5, character: 18 },
        },
        children: [],
        attributes: { multiplicity: "1" },
        relationships: [{ type: "typing", source: "Drone::body", target: "Drone::Airframe" }],
      },
      uri,
      undefined,
      context
    );
    assert.strictEqual(presentation.description, ": Airframe [1] @ example.sysml");
    assert.ok(presentation.tooltip.includes("Qualified name: Drone::body"));
    assert.ok(presentation.tooltip.includes("Type: Airframe"));
    assert.ok(presentation.tooltip.includes("Relationships: 1 outgoing, 2 incoming"));
  });

  it("Model Explorer presentation summarizes specialization and parent context", () => {
    const uri = vscode.Uri.file("C:/Git/spec42/drone.sysml");
    const metadataById = new Map<string, ElementMetadata>([
      [
        "Drone::Base",
        {
          reference: {
            id: "Drone::Base",
            name: "Base",
            type: "part def",
            uri,
            range: {
              start: { line: 1, character: 0 },
              end: { line: 1, character: 14 },
            },
          },
        },
      ],
      [
        "Drone::Package",
        {
          reference: {
            id: "Drone::Package",
            name: "DronePackage",
            type: "package",
            uri,
            range: {
              start: { line: 0, character: 0 },
              end: { line: 20, character: 0 },
            },
          },
        },
      ],
    ]);
    const parentItem = {
      itemType: "sysml-element" as const,
      element: {
        id: "Drone::Package",
        type: "package",
        name: "DronePackage",
        range: {
          start: { line: 0, character: 0 },
          end: { line: 20, character: 0 },
        },
        children: [],
        attributes: {},
        relationships: [],
      },
      elementUri: uri,
    } as any;
    const presentation = buildElementPresentation(
      {
        id: "Drone::Advanced",
        type: "part def",
        name: "Advanced",
        range: {
          start: { line: 10, character: 0 },
          end: { line: 10, character: 20 },
        },
        children: [],
        attributes: {},
        relationships: [{ type: "specializes", source: "Drone::Advanced", target: "Drone::Base" }],
      },
      uri,
      parentItem,
      {
        activeUri: uri,
        metadataById,
        incomingRelationshipCounts: new Map(),
      }
    );
    assert.strictEqual(presentation.description, ":> Base");
    assert.ok(presentation.tooltip.includes("Parent: DronePackage"));
    assert.ok(presentation.tooltip.includes("Specializes: Base"));
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
