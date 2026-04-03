import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { VisualizationPanel } from "../../visualization/visualizationPanel";
import {
  configureServerForTests,
  getDiagramExportUri,
  getTestWorkspaceFolder,
  waitFor,
  waitForDiagramExport,
  waitForLanguageServerReady,
} from "./testUtils";

type Rect = { x: number; y: number; width: number; height: number };
type Point = { x: number; y: number };

function parsePathPoints(pathD: string): Point[] {
  const out: Point[] = [];
  const regex = /[ML]([0-9.\-]+),([0-9.\-]+)/g;
  let m: RegExpExecArray | null;
  while ((m = regex.exec(pathD)) !== null) {
    out.push({ x: Number(m[1]), y: Number(m[2]) });
  }
  return out;
}

function parseGeneralNodeRects(svgText: string): Map<string, Rect> {
  const out = new Map<string, Rect>();
  const nodeRegex =
    /<g class="general-node[^"]*" transform="translate\(([0-9.\-]+),([0-9.\-]+)\)" data-element-name="([^"]+)"[\s\S]*?<rect width="([0-9.\-]+)" height="([0-9.\-]+)"/g;
  let m: RegExpExecArray | null;
  while ((m = nodeRegex.exec(svgText)) !== null) {
    out.set(m[3], {
      x: Number(m[1]),
      y: Number(m[2]),
      width: Number(m[4]),
      height: Number(m[5]),
    });
  }
  return out;
}

function parseGeneralEdges(svgText: string): Array<{ source: string; target: string; pathD: string; edgeType: string }> {
  const out: Array<{ source: string; target: string; pathD: string; edgeType: string }> = [];
  const edgeRegex =
    /<path d="([^"]+)" class="general-connector" data-source="([^"]+)" data-target="([^"]+)" data-type="([^"]+)"/g;
  let m: RegExpExecArray | null;
  while ((m = edgeRegex.exec(svgText)) !== null) {
    out.push({ pathD: m[1], source: m[2], target: m[3], edgeType: m[4] });
  }
  return out;
}

function segmentIntersectsRect(a: Point, b: Point, rect: Rect): boolean {
  const rx1 = rect.x;
  const ry1 = rect.y;
  const rx2 = rect.x + rect.width;
  const ry2 = rect.y + rect.height;
  if (Math.abs(a.x - b.x) < 0.0001) {
    const x = a.x;
    if (x <= rx1 || x >= rx2) return false;
    const sy1 = Math.min(a.y, b.y);
    const sy2 = Math.max(a.y, b.y);
    return sy2 > ry1 && sy1 < ry2;
  }
  if (Math.abs(a.y - b.y) < 0.0001) {
    const y = a.y;
    if (y <= ry1 || y >= ry2) return false;
    const sx1 = Math.min(a.x, b.x);
    const sx2 = Math.max(a.x, b.x);
    return sx2 > rx1 && sx1 < rx2;
  }
  return false;
}

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
    const exportUri = getDiagramExportUri(getTestWorkspaceFolder().uri, "general-view");
    try {
      await vscode.workspace.fs.delete(exportUri, { useTrash: false });
    } catch {
      // ignore when no previous export exists
    }
    panel.getWebview()?.postMessage({ command: "exportDiagramForTest" });

    const { svgText } = await waitForDiagramExport(
      getTestWorkspaceFolder().uri,
      "general-view",
      (svg) => Boolean(svg),
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

    const nodeRectsByName = parseGeneralNodeRects(svgText);
    const edgeItems = parseGeneralEdges(svgText);
    assert.ok(
      edgeItems.some((e) => e.edgeType === "subject"),
      "Expected General View to include subject requirement edges when Requirements is ON"
    );
    assert.ok(
      edgeItems.some((e) => e.edgeType === "satisfy"),
      "Expected General View to include satisfy requirement edges when Requirements is ON"
    );
    const nonOrthogonalEdges: string[] = [];
    for (const edge of edgeItems) {
      const pts = parsePathPoints(edge.pathD);
      if (pts.length < 2) continue;
      let orthogonal = true;
      for (let i = 0; i < pts.length - 1; i++) {
        const a = pts[i];
        const b = pts[i + 1];
        if (Math.abs(a.x - b.x) > 0.0001 && Math.abs(a.y - b.y) > 0.0001) {
          orthogonal = false;
          break;
        }
      }
      if (!orthogonal) {
        nonOrthogonalEdges.push(`${edge.source}->${edge.target}`);
      }
    }
    assert.strictEqual(
      nonOrthogonalEdges.length,
      0,
      `General View has non-orthogonal routed edges: ${nonOrthogonalEdges.join(", ")}`
    );

    const laptopToKeyboard = edgeItems.find(
      (e) => e.source === "gv-it-laptop" && e.target === "gv-it-laptop-keyboard"
    );
    const displayRect = nodeRectsByName.get("display");
    if (laptopToKeyboard && displayRect) {
      const pts = parsePathPoints(laptopToKeyboard.pathD);
      let intersectsDisplay = false;
      for (let i = 0; i < pts.length - 1; i++) {
        if (segmentIntersectsRect(pts[i], pts[i + 1], displayRect)) {
          intersectsDisplay = true;
          break;
        }
      }
      assert.strictEqual(
        intersectsDisplay,
        false,
        "Laptop->keyboard edge intersects display node rectangle"
      );
    }

    panel.getWebview()?.postMessage({ command: "setRequirementsVisibleForTest", enabled: false });
    try {
      await vscode.workspace.fs.delete(exportUri, { useTrash: false });
    } catch {
      // ignore when no previous export exists
    }
    panel.getWebview()?.postMessage({ command: "exportDiagramForTest" });
    const { svgText: svgRequirementsOff } = await waitForDiagramExport(
      getTestWorkspaceFolder().uri,
      "general-view",
      (svg) => Boolean(svg) && svg.includes("general-connector"),
      45000
    );
    const offEdges = parseGeneralEdges(svgRequirementsOff);
    const forbiddenRequirementEdgeTypes = offEdges.filter((e) =>
      ["subject", "satisfy", "verify"].includes(e.edgeType)
    );
    assert.strictEqual(
      forbiddenRequirementEdgeTypes.length,
      0,
      `Expected no subject/satisfy/verify edges when Requirements is OFF, found: ${forbiddenRequirementEdgeTypes
        .map((e) => `${e.edgeType}:${e.source}->${e.target}`)
        .join(", ")}`
    );
    const offNodes = parseGeneralNodeRects(svgRequirementsOff);
    const requirementLikeNodeNames = [...offNodes.keys()].filter((name) => /req$/i.test(name));
    assert.strictEqual(
      requirementLikeNodeNames.length,
      0,
      `Expected requirement nodes to be hidden when Requirements is OFF, found: ${requirementLikeNodeNames.join(", ")}`
    );

    panel.getWebview()?.postMessage({ command: "setRequirementsVisibleForTest", enabled: true });
  });
});
