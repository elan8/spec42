import * as assert from "assert";
import * as vscode from "vscode";
import {
    closeAllEditorsForTests,
    configureServerForTests,
    disposeVisualizer,
    getFixturePath,
    getDiagramExportUri,
    getTestWorkspaceFolder,
    integrationHookTimeoutMs,
    isCi,
    triggerDiagramExportAndWait,
    waitForExtensionServerReady,
    waitForLanguageServerReady,
    waitForVisualizationModel,
    waitForVisualizerClosed,
    waitForVisualizerOpen,
} from "./testUtils";

const VIEW_IDS = ["general-view"];

const REAL_DRONE_FIXTURE = getFixturePath("SurveillanceDrone.sysml");

describe("Visualization Diagram Views", () => {
    before(async function () {
        this.timeout(integrationHookTimeoutMs);
        await configureServerForTests();
        getTestWorkspaceFolder();
        const docPath = REAL_DRONE_FIXTURE;
        const doc = await vscode.workspace.openTextDocument(docPath);
        await waitForExtensionServerReady();
        await waitForLanguageServerReady(doc);
    });

    afterEach(async () => {
        await disposeVisualizer();
        await closeAllEditorsForTests();
    });

    after(async () => {
        await disposeVisualizer();
        await closeAllEditorsForTests();
    });

    it("exports SVG for all views", async function () {
        this.timeout(isCi ? 120000 : 60000);

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
            await waitForVisualizationModel(
                workspaceFolder.uri,
                viewId,
                (visualization) =>
                    viewId !== "general-view" ||
                    visualization?.graph?.nodes?.some(
                        (node: any) => node?.name === "SurveillanceQuadrotorDrone"
                    ) === true,
                isCi ? 45000 : 20000
            );
            const exportUri = getDiagramExportUri(workspaceFolder.uri, viewId);
            try {
                await vscode.workspace.fs.delete(exportUri, { useTrash: false });
            } catch {
                // Ignore if there is no previous export yet.
            }
            const { svgText } = await triggerDiagramExportAndWait(
                workspaceFolder.uri,
                viewId,
                (text) =>
                    viewId !== "general-view" ||
                    text.includes("SurveillanceQuadrotorDrone"),
                isCi ? 45000 : 20000
            );
            assert.ok(svgText.includes("<svg"), `${viewId}.svg should contain svg markup`);
            if (viewId === "general-view") {
                assert.ok(
                    svgText.includes("SurveillanceQuadrotorDrone"),
                    "general-view export should include the main drone node"
                );
            }
        }
    });
});
