import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { VisualizationPanel } from "../../visualization/visualizationPanel";
import {
  configureServerForTests,
  getFixturePath,
  getTestWorkspaceFolder,
  waitForDiagramExport,
  waitFor,
  waitForLanguageServerReady,
} from "./testUtils";

function getExtensionRoot(): string {
  const ext = vscode.extensions.all.find((e) => e.packageJSON?.name === "spec42");
  assert.ok(ext, "spec42 extension must be installed for tests");
  return ext.extensionPath;
}

function ensureDir(p: string): void {
  fs.mkdirSync(p, { recursive: true });
}

describe("SVG artifacts (SurveillanceDrone, elkjs)", () => {
  before(async function () {
    this.timeout(30000);
    await configureServerForTests();
    getTestWorkspaceFolder();
    const docPath = getFixturePath("SurveillanceDrone.sysml");
    const doc = await vscode.workspace.openTextDocument(docPath);
    await waitForLanguageServerReady(doc);
  });

  afterEach(async () => {
    if (VisualizationPanel.currentPanel) {
      VisualizationPanel.currentPanel.dispose();
    }
    await vscode.commands.executeCommand("workbench.action.closeAllEditors");
  });

  it("exports General + Interconnection SVGs to vscode/tests/output", async function () {
    this.timeout(90000);

    const workspaceFolder = getTestWorkspaceFolder();
    const docPath = getFixturePath("SurveillanceDrone.sysml");
    const doc = await vscode.workspace.openTextDocument(docPath);
    await vscode.window.showTextDocument(doc, { preview: false });
    await waitForLanguageServerReady(doc);

    await vscode.commands.executeCommand("sysml.showVisualizer");
    const panel = await waitFor(
      "visualization panel",
      async () => VisualizationPanel.currentPanel,
      (value) => Boolean(value),
      20000,
      300
    );

    // Root cause for empty general SVG in this test:
    // webview rendering can lag behind panel creation. Push one explicit model update.
    const model = (await vscode.commands.executeCommand(
      "sysml.debug.getModelForTests",
      doc.uri.toString(),
      ["graph", "ibd", "sequenceDiagrams", "activityDiagrams", "stats"]
    )) as any;
    panel.getWebview()?.postMessage({
      command: "update",
      graph: model?.graph,
      generalViewGraph: model?.generalViewGraph,
      ibd: model?.ibd,
      renderedDiagrams: model?.renderedDiagrams,
      sequenceDiagrams: model?.sequenceDiagrams ?? [],
      activityDiagrams: model?.activityDiagrams ?? [],
      currentView: "general-view",
    });
    const outputDir = vscode.Uri.joinPath(workspaceFolder.uri, "test-output", "diagrams");

    async function exportView(viewId: string): Promise<{ svgText: string; fileName: string }> {
      await vscode.commands.executeCommand("sysml.changeVisualizerView", viewId);
      const uri = vscode.Uri.joinPath(outputDir, `${viewId}.svg`);
      try {
        await vscode.workspace.fs.delete(uri, { useTrash: false });
      } catch {
        // ignore
      }
      panel.getWebview()?.postMessage({ command: "exportDiagramForTest" });
      const { svgText } = await waitForDiagramExport(
        workspaceFolder.uri,
        viewId,
        (text) => {
          if (viewId === "general-view") {
            return !text.includes("<g/></svg>");
          }
          if (viewId === "interconnection-view") {
            return text.includes("ibd-connector");
          }
          return true;
        },
        14000
      );
      assert.ok(svgText.includes("<svg"), `${viewId}.svg should contain svg markup`);
      return { svgText, fileName: `${viewId}.elkjs.svg` };
    }

    const general = await exportView("general-view");
    const interconnection = await exportView("interconnection-view");

    const gnssMatches = general.svgText.match(/data-element-name="gnss"/g) ?? [];
    assert.strictEqual(
      gnssMatches.length,
      1,
      `general-view should render exactly one gnss node, found ${gnssMatches.length}`
    );

    const extensionRoot = getExtensionRoot();
    const outDir = path.join(extensionRoot, "tests", "output");
    ensureDir(outDir);

    fs.writeFileSync(path.join(outDir, general.fileName), general.svgText, "utf8");
    fs.writeFileSync(path.join(outDir, interconnection.fileName), interconnection.svgText, "utf8");
  });
});

