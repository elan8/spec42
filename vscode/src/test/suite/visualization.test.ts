import * as assert from "assert";
import * as vscode from "vscode";
import {
    configureServerForTests,
    getFixturePath,
    getDiagramExportUri,
    getTestWorkspaceFolder,
    waitFor,
    waitForExtensionServerReady,
    waitForLanguageServerReady,
} from "./testUtils";

const isCi = Boolean(process.env.CI);
const visualizationPanelTimeoutMs = isCi ? 45000 : 20000;

type ExtensionDebugState = {
    visualizerOpen?: boolean;
};

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
        await vscode.commands.executeCommand("sysml.debug.disposeVisualizer");
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    });

    after(async () => {
        await vscode.commands.executeCommand("sysml.debug.disposeVisualizer");
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
        await new Promise((r) => setTimeout(r, 250));
    });

    it("exports SVG for all views", async function () {
        this.timeout(60000);

        const workspaceFolder = getTestWorkspaceFolder();

        for (const viewId of VIEW_IDS) {
            const openState = await vscode.commands.executeCommand<ExtensionDebugState>(
                "sysml.debug.getExtensionState"
            );
            if (openState?.visualizerOpen) {
                await vscode.commands.executeCommand("sysml.debug.disposeVisualizer");
                await waitFor(
                    "visualization panel disposal",
                    async () =>
                        vscode.commands.executeCommand<ExtensionDebugState>(
                            "sysml.debug.getExtensionState"
                        ),
                    (value) => !value?.visualizerOpen,
                    10000,
                    100
                );
            }
            const docPath = REAL_DRONE_FIXTURE;
            const doc = await vscode.workspace.openTextDocument(docPath);
            await vscode.window.showTextDocument(doc, { preserveFocus: false });
            await waitForLanguageServerReady(doc);
            await waitForExtensionServerReady();
            await vscode.commands.executeCommand("sysml.showVisualizer");
            await waitFor(
                "visualization panel",
                async () =>
                    vscode.commands.executeCommand<ExtensionDebugState>(
                        "sysml.debug.getExtensionState"
                    ),
                (value) => value?.visualizerOpen === true,
                visualizationPanelTimeoutMs,
                300
            );
            await vscode.commands.executeCommand("sysml.changeVisualizerView", viewId);
            await new Promise((r) => setTimeout(r, 2000)); // Wait for render
            await vscode.commands.executeCommand("sysml.debug.exportVisualizerDiagramForTest");
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
