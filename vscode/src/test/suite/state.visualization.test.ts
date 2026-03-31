import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import { VisualizationPanel } from "../../visualization/visualizationPanel";
import {
    configureServerForTests,
    getFixturePath,
    getTestWorkspaceFolder,
    waitFor,
    waitForDiagramExport,
    waitForLanguageServerReady,
} from "./testUtils";

const STATE_FIXTURE = getFixturePath("StateMachineDemo.sysml");

describe("State Transition Visualization", () => {
    before(async function () {
        this.timeout(30000);
        process.env.SPEC42_SERVER_PATH = path.resolve(__dirname, "../../../../target/debug/spec42.exe");
        await configureServerForTests();
        await vscode.workspace
            .getConfiguration("spec42")
            .update("visualization.enableExperimentalViews", true, vscode.ConfigurationTarget.Workspace);

        const doc = await vscode.workspace.openTextDocument(STATE_FIXTURE);
        await waitForLanguageServerReady(doc);
    });

    afterEach(async () => {
        if (VisualizationPanel.currentPanel) {
            VisualizationPanel.currentPanel.dispose();
        }
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    });

    it("exports SVG for state-transition-view", async function () {
        this.timeout(60000);

        const workspaceFolder = getTestWorkspaceFolder();
        const doc = await vscode.workspace.openTextDocument(STATE_FIXTURE);
        await vscode.window.showTextDocument(doc);
        await waitForLanguageServerReady(doc);

        await vscode.commands.executeCommand("sysml.showVisualizer");
        const panel = await waitFor(
            "visualization panel",
            async () => VisualizationPanel.currentPanel,
            (value) => Boolean(value),
            20000,
            300
        );

        await vscode.commands.executeCommand("sysml.changeVisualizerView", "state-transition-view");
        await new Promise((resolve) => setTimeout(resolve, 2500));
        panel.getWebview()?.postMessage({ command: "exportDiagramForTest" });

        const { svgText } = await waitForDiagramExport(
            workspaceFolder.uri,
            "state-transition-view",
            (text) =>
                text.includes("State Machine: TimerStateMachine")
                && text.includes("state-node")
                && text.includes("state-transition"),
            20000
        );

        assert.ok(svgText.includes("<svg"), "state-transition-view export should contain svg markup");
        assert.ok(svgText.includes("state-node"), "state-transition-view export should include state nodes");
        assert.ok(svgText.includes("state-transition"), "state-transition-view export should include transition paths");
        assert.ok(svgText.includes("TimerStateMachine"), "state-transition-view export should include the machine title");
    });
});
