import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { VisualizationPanel } from "../../visualization/visualizationPanel";
import {
  configureServerForTests,
  getTestWorkspaceFolder,
  tryReadWorkspaceText,
  waitFor,
  waitForDiagramExport,
  waitForLanguageServerReady,
} from "./testUtils";

type Rect = { x: number; y: number; width: number; height: number };

function parseGeneralPackageRects(svgText: string): Array<{ name: string; rect: Rect }> {
  const groupMatch = svgText.match(/<g class="general-packages">([\s\S]*?)<\/g>/);
  if (!groupMatch) {
    return [];
  }
  const group = groupMatch[1];
  const pairRegex =
    /<rect x="([0-9.\-]+)" y="([0-9.\-]+)" width="([0-9.\-]+)" height="([0-9.\-]+)"[^>]*\/?>\s*<text x="[0-9.\-]+" y="[0-9.\-]+"[^>]*>([^<]+)<\/text>/g;
  const out: Array<{ name: string; rect: Rect }> = [];
  let m: RegExpExecArray | null;
  while ((m = pairRegex.exec(group)) !== null) {
    out.push({
      name: m[5],
      rect: {
        x: Number(m[1]),
        y: Number(m[2]),
        width: Number(m[3]),
        height: Number(m[4]),
      },
    });
  }
  return out;
}

function overlaps(a: Rect, b: Rect): boolean {
  const overlapX = Math.min(a.x + a.width, b.x + b.width) - Math.max(a.x, b.x);
  const overlapY = Math.min(a.y + a.height, b.y + b.height) - Math.max(a.y, b.y);
  return overlapX > 0 && overlapY > 0;
}

describe("SysML Examples General View", () => {
  before(async function () {
    this.timeout(40000);
    await configureServerForTests();
    getTestWorkspaceFolder();
  });

  after(async () => {
    await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    await new Promise((r) => setTimeout(r, 250));
  });

  it("renders all top-level example packages in All Packages", async function () {
    this.timeout(90000);

    const workspaceRoot = getTestWorkspaceFolder().uri.fsPath;
    const expectedRoot = path.resolve("C:/Git/sysml-examples");
    if (path.resolve(workspaceRoot).toLowerCase() !== expectedRoot.toLowerCase()) {
      this.skip();
      return;
    }

    const sysmlFiles = await vscode.workspace.findFiles("**/*.sysml", "**/node_modules/**", 500);
    assert.ok(sysmlFiles.length > 0, "Expected sysml-examples workspace to contain .sysml files");
    const anchor = sysmlFiles.find((u) => fs.existsSync(u.fsPath)) ?? sysmlFiles[0];
    const doc = await vscode.workspace.openTextDocument(anchor);
    await waitForLanguageServerReady(doc, 30000);
    await vscode.window.showTextDocument(doc);

    await vscode.commands.executeCommand("sysml.visualizeFolder", getTestWorkspaceFolder().uri);
    await vscode.commands.executeCommand("sysml.changeVisualizerView", "general-view");

    const panel = await waitFor(
      "visualization panel",
      async () => VisualizationPanel.currentPanel,
      (value) => Boolean(value),
      30000,
      300
    );
    const exportUri = vscode.Uri.joinPath(
      getTestWorkspaceFolder().uri,
      "test-output",
      "diagrams",
      "general-view.svg"
    );
    const previousSvg = await tryReadWorkspaceText(exportUri);
    try {
      await vscode.workspace.fs.delete(exportUri, { useTrash: false });
    } catch {
      // ignore when no previous export exists
    }
    panel.getWebview()?.postMessage({ command: "exportDiagramForTest" });

    const { svgText } = await waitForDiagramExport(
      getTestWorkspaceFolder().uri,
      "general-view",
      (svg) => Boolean(svg) && svg !== (previousSvg ?? ""),
      45000
    );
    const expectedPackages = [
      "SurveillanceDrone",
      "KitchenTimer",
      "TrafficLightIntersection",
      "IT",
    ];
    const missing = expectedPackages.filter((pkg) => !svgText.includes(pkg));
    const packageLabelMatches = [...svgText.matchAll(/>([A-Za-z][A-Za-z0-9_]+)<\/text>/g)]
      .map((m) => m[1])
      .filter((name) => expectedPackages.includes(name));
    const presentUnique = [...new Set(packageLabelMatches)];
    assert.strictEqual(
      missing.length,
      0,
      `All Packages export missing: ${missing.join(", ")}; present package labels: ${presentUnique.join(", ")}`
    );

    const packageRects = parseGeneralPackageRects(svgText).filter((p) => expectedPackages.includes(p.name));
    assert.strictEqual(
      packageRects.length,
      expectedPackages.length,
      `Expected ${expectedPackages.length} package containers, found ${packageRects.length}: ${packageRects
        .map((p) => p.name)
        .join(", ")}`
    );
    const collisions: string[] = [];
    for (let i = 0; i < packageRects.length; i++) {
      for (let j = i + 1; j < packageRects.length; j++) {
        const a = packageRects[i];
        const b = packageRects[j];
        if (overlaps(a.rect, b.rect)) {
          collisions.push(`${a.name}<->${b.name}`);
        }
      }
    }
    assert.strictEqual(
      collisions.length,
      0,
      `General View package containers overlap: ${collisions.join(", ")}`
    );
  });
});

