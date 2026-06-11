import * as assert from "assert";
import * as fs from "fs";
import * as vscode from "vscode";
import {
    configureServerForTests,
    disposeVisualizer,
    getDiagramExportUri,
    integrationTestLog,
    seedVisualizerWebviewFromModel,
    triggerDiagramExportAndWait,
    waitForExtensionServerReady,
    waitForLanguageServerReady,
    waitForVisualizerOpen,
} from "./testUtils";

const STEDIN_REPO = "C:\\Git\\sysml-powersystems";
const STEDIN_VIEWS = `${STEDIN_REPO}\\sysml\\projects\\stedin-rijnmond-grid-expansion\\Views.sysml`;
const GRID_CONNECTIONS_VIEW = "gridConnections";
const stedinTimeoutMs = 180000;

function parsePartNames(svgText: string): string[] {
    const names = new Set<string>();
    const regex = /data-element-name="([^"]+)"/g;
    let match: RegExpExecArray | null;
    while ((match = regex.exec(svgText)) !== null) {
        names.add(match[1]);
    }
    return [...names].sort();
}

function parseSvgDebug(svgText: string): Record<string, unknown> {
    return {
        viewBox: svgText.match(/\bviewBox="([^"]+)"/)?.[1] ?? null,
        rootTransform: svgText.match(/<g class="viz-root" transform="([^"]+)"/)?.[1] ?? null,
        connectorCount: (svgText.match(/\bibd-connector\b/g) ?? []).length,
        partCount: (svgText.match(/\bibd-part\b/g) ?? []).length,
        partNames: parsePartNames(svgText).slice(0, 40),
        hasFeederNorth: svgText.includes("feederNorth"),
        hasCable01: svgText.includes("cable01"),
        hasPrimarySubstation: svgText.includes("primarySubstation"),
    };
}

describe("Stedin Interconnection Visualization", () => {
    before(async function () {
        this.timeout(stedinTimeoutMs);
        if (!fs.existsSync(STEDIN_REPO) || !fs.existsSync(STEDIN_VIEWS)) {
            this.skip();
        }
        await configureServerForTests();
        const viewsDoc = await vscode.workspace.openTextDocument(STEDIN_VIEWS);
        await waitForLanguageServerReady(viewsDoc, stedinTimeoutMs);
        await waitForExtensionServerReady(stedinTimeoutMs);
    });

    afterEach(async () => {
        await disposeVisualizer();
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    });

    it("renders gridConnections from the parent workspace root", async function () {
        this.timeout(stedinTimeoutMs);

        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        assert.ok(workspaceFolder, "expected the Stedin workspace folder to be open");

        const doc = await vscode.workspace.openTextDocument(STEDIN_VIEWS);
        await vscode.window.showTextDocument(doc, { preserveFocus: false, preview: false });
        await waitForLanguageServerReady(doc, stedinTimeoutMs);

        const snapshot = await vscode.commands.executeCommand<Record<string, unknown>>(
            "sysml.debug.getVisualizationForTests",
            workspaceFolder.uri.toString(),
            "interconnection-view",
            GRID_CONNECTIONS_VIEW
        );
        integrationTestLog("stedin:gridConnections:lspSnapshot", {
            selectedView: snapshot?.selectedView,
            selectedViewName: snapshot?.selectedViewName,
            viewCandidates: (snapshot?.viewCandidates as Array<{ id?: string; name?: string }> | undefined)?.map(
                (candidate) => ({ id: candidate.id, name: candidate.name })
            ),
            ibdParts: (snapshot?.ibd as { parts?: unknown[] } | undefined)?.parts?.length ?? 0,
            ibdConnectors: (snapshot?.ibd as { connectors?: unknown[] } | undefined)?.connectors?.length ?? 0,
            rootCandidates: (snapshot?.ibd as { rootCandidates?: unknown[] } | undefined)?.rootCandidates,
            defaultRoot: (snapshot?.ibd as { defaultRoot?: unknown } | undefined)?.defaultRoot,
        });

        await vscode.commands.executeCommand("sysml.showVisualizer");
        await waitForVisualizerOpen(stedinTimeoutMs);
        await vscode.commands.executeCommand("sysml.changeVisualizerView", "interconnection-view");
        await seedVisualizerWebviewFromModel(
            workspaceFolder.uri,
            "interconnection-view",
            (summary) => summary.ibdConnectors >= 15 && summary.ibdParts >= 10,
            {
                timeoutMs: stedinTimeoutMs,
                renderTimeoutMs: stedinTimeoutMs,
                selectedView: GRID_CONNECTIONS_VIEW,
            }
        );

        const uri = getDiagramExportUri(workspaceFolder.uri, "interconnection-view");
        try {
            await vscode.workspace.fs.delete(uri, { useTrash: false });
        } catch {
            // Ignore if no previous export exists.
        }

        const { svgText } = await triggerDiagramExportAndWait(
            workspaceFolder.uri,
            "interconnection-view",
            (text) => text.includes("ibd-connector") && text.includes("feederNorth"),
            stedinTimeoutMs
        );
        const debug = parseSvgDebug(svgText);
        integrationTestLog("stedin:gridConnections:svgDebug", debug);

        assert.ok(debug.hasFeederNorth, "expected feederNorth in rendered SVG");
        assert.ok(debug.hasCable01, "expected cable01 in rendered SVG");
        assert.ok(
            Number(debug.connectorCount) >= 15,
            `expected at least 15 connector paths, got ${debug.connectorCount}`
        );
    });
});
