import * as assert from "assert";
import * as vscode from "vscode";
import {
    configureServerForTests,
    disposeVisualizer,
    getDiagramExportUri,
    getFixturePath,
    getTestWorkspaceFolder,
    integrationHookTimeoutMs,
    triggerVisualizerExportForTest,
    waitForDiagramExport,
    waitFor,
    waitForLanguageServerReady,
    waitForVisualizerOpen,
} from "./testUtils";

const STATE_FIXTURE = getFixturePath("StateMachineDemo.sysml");

describe("State Transition Visualization", () => {
    before(async function () {
        this.timeout(integrationHookTimeoutMs);
        await configureServerForTests();

        const doc = await vscode.workspace.openTextDocument(STATE_FIXTURE);
        await waitForLanguageServerReady(doc);
    });

    afterEach(async () => {
        await disposeVisualizer();
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    });

    it("exports SVG for state-transition-view", async function () {
        this.timeout(60000);

        const workspaceFolder = getTestWorkspaceFolder();
        const doc = await vscode.workspace.openTextDocument(STATE_FIXTURE);
        await vscode.window.showTextDocument(doc, { preserveFocus: false });
        await waitForLanguageServerReady(doc);

        await vscode.commands.executeCommand("sysml.showVisualizer");
        await waitForVisualizerOpen();

        await vscode.commands.executeCommand("sysml.changeVisualizerView", "state-transition-view");
        const visualization = await waitFor(
            "state-transition visualization model",
            () =>
                vscode.commands.executeCommand<any>(
                    "sysml.debug.getVisualizationForTests",
                    workspaceFolder.uri.toString(),
                    "state-transition-view"
                ),
            (value) =>
                Boolean(
                    value?.graph?.nodes?.some((node: any) => node?.name === "TimerStateMachine")
                ),
            30000,
            300
        );
        await vscode.commands.executeCommand("sysml.debug.postVisualizerMessage", {
            command: "update",
            modelReady: visualization?.modelReady !== false,
            graph: visualization?.graph ?? { nodes: [], edges: [] },
            elements: visualization?.workspaceModel?.semantic,
            generalViewGraph: visualization?.generalViewGraph ?? visualization?.graph,
            ibd: visualization?.ibd,
            activityDiagrams: visualization?.activityDiagrams ?? [],
            sequenceDiagrams: visualization?.sequenceDiagrams ?? [],
            currentView: "state-transition-view",
            viewCandidates: visualization?.viewCandidates ?? [],
            selectedView: visualization?.selectedView,
            selectedViewName: visualization?.selectedViewName,
            emptyStateMessage: visualization?.emptyStateMessage,
        });

        const exportUri = getDiagramExportUri(workspaceFolder.uri, "state-transition-view");
        try {
            await vscode.workspace.fs.delete(exportUri, { useTrash: false });
        } catch {
            // Ignore if there is no previous export yet.
        }

        await triggerVisualizerExportForTest();
        await new Promise((resolve) => setTimeout(resolve, 800));
        await triggerVisualizerExportForTest();

        const { svgText } = await waitForDiagramExport(
            workspaceFolder.uri,
            "state-transition-view",
            (text) =>
                text.includes("TimerStateMachine")
                && text.includes("state-node")
                && text.includes("state-transition-edge"),
            30000
        );

        assert.ok(svgText.includes("<svg"), "state-transition-view export should contain svg markup");
        assert.ok(svgText.includes("state-node"), "state-transition-view export should include state nodes");
        assert.ok(svgText.includes("state-transition-edge"), "state-transition-view export should include transition paths");
        assert.ok(svgText.includes("TimerStateMachine"), "state-transition-view export should include the machine title");
    });
});
