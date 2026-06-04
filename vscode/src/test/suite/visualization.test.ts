import * as assert from "assert";
import * as vscode from "vscode";
import {
    configureServerForTests,
    disposeVisualizer,
    getFixturePath,
    getDiagramExportUri,
    getTestWorkspaceFolder,
    triggerVisualizerExportForTest,
    waitForExtensionServerReady,
    waitForLanguageServerReady,
    waitForVisualizerClosed,
    waitForVisualizerOpen,
} from "./testUtils";

const VIEW_IDS = ["general-view"];

const REAL_DRONE_FIXTURE = getFixturePath("SurveillanceDrone.sysml");

describe("Visualization Diagram Views", () => {
    before(async function () {
        this.timeout(30000);
        await configureServerForTests();
        getTestWorkspaceFolder();
        const docPath = REAL_DRONE_FIXTURE;
        const doc = await vscode.workspace.openTextDocument(docPath);
        await waitForLanguageServerReady(doc);
    });

    afterEach(async () => {
        await disposeVisualizer();
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    });

    after(async () => {
        await disposeVisualizer();
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
        await new Promise((r) => setTimeout(r, 250));
    });

    it("exports SVG for all views", async function () {
        this.timeout(60000);

        const workspaceFolder = getTestWorkspaceFolder();

        for (const viewId of VIEW_IDS) {
            await disposeVisualizer();
            await waitForVisualizerClosed();
            const docPath = REAL_DRONE_FIXTURE;
            const doc = await vscode.workspace.openTextDocument(docPath);
            await vscode.window.showTextDocument(doc, { preserveFocus: false });
            await waitForLanguageServerReady(doc);
            await waitForExtensionServerReady();
            await vscode.commands.executeCommand("sysml.showVisualizer");
            await waitForVisualizerOpen();
            await vscode.commands.executeCommand("sysml.changeVisualizerView", viewId);
            await new Promise((r) => setTimeout(r, 2000)); // Wait for render
            await triggerVisualizerExportForTest();
            await new Promise((r) => setTimeout(r, 1200)); // Wait for export + file write
        }

        for (const viewId of VIEW_IDS) {
            const uri = getDiagramExportUri(workspaceFolder.uri, viewId);
            try {
                const stat = await vscode.workspace.fs.stat(uri);
                assert.ok(stat.size >= 0, `${viewId}.svg should exist`);
                const bytes = await vscode.workspace.fs.readFile(uri);
                const svgText = Buffer.from(bytes).toString("utf8");
                assert.ok(svgText.includes("<svg"), `${viewId}.svg should contain svg markup`);
                if (viewId === "general-view") {
                    assert.ok(
                        svgText.includes("SurveillanceQuadrotorDrone"),
                        "general-view export should include the main drone node"
                    );
                }
            } catch {
                assert.fail(`${viewId}.svg was not created in the test export directory`);
            }
        }
    });
});
