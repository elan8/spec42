import * as assert from "assert";
import * as vscode from "vscode";
import { VisualizationPanel } from "../../visualization/visualizationPanel";
import {
    configureServerForTests,
    getExternalFixturePath,
    getDiagramExportUri,
    getTestWorkspaceFolder,
    waitFor,
    waitForLanguageServerReady,
} from "./testUtils";

const DRONE_FIXTURE = getExternalFixturePath("C:\\Git\\sysml-examples\\drone\\sysml\\SurveillanceDrone.sysml");

type ParsedRoute = {
    points: Array<{ x: number; y: number }>;
    source: string;
    target: string;
};

function parseConnectorRoutes(svgText: string): ParsedRoute[] {
    const routes: ParsedRoute[] = [];
    const pathRegex = /<path\b([^>]*)>/g;
    let match: RegExpExecArray | null;
    while ((match = pathRegex.exec(svgText)) !== null) {
        const attributes = match[1];
        const isClientIbd = attributes.includes('class="ibd-connector"');
        const isDiagramEdge = attributes.includes('class="diagram-edge') || attributes.includes('class="diagram-edge ');
        if (!isClientIbd && !isDiagramEdge) {
            continue;
        }
        const dMatch = attributes.match(/\bd="([^"]+)"/);
        if (!dMatch) {
            continue;
        }
        const points: Array<{ x: number; y: number }> = [];
        const tokenRegex = /[ML]([0-9.+-]+),([0-9.+-]+)/g;
        let tokenMatch: RegExpExecArray | null;
        while ((tokenMatch = tokenRegex.exec(dMatch[1])) !== null) {
            points.push({ x: Number(tokenMatch[1]), y: Number(tokenMatch[2]) });
        }
        if (points.length >= 2) {
            routes.push({
                points,
                // Client IBD renderer includes data-source/data-target; fallback SVG parsing does not.
                source: (attributes.match(/\bdata-source="([^"]*)"/)?.[1] || ""),
                target: (attributes.match(/\bdata-target="([^"]*)"/)?.[1] || ""),
            });
        }
    }
    return routes;
}

function parsePartBounds(svgText: string): Array<{ name: string; x: number; y: number; width: number; height: number; isContainer: boolean }> {
    const bounds: Array<{ name: string; x: number; y: number; width: number; height: number; isContainer: boolean }> = [];
    // Client IBD renderer: <g transform="translate(x,y)" class="... ibd-part ..."> <rect width height ...>
    const clientRegex = /<g transform="translate\(([0-9.+-]+),([0-9.+-]+)\)" class="([^"]*ibd-part[^"]*)"[^>]*data-element-name="([^"]+)"[^>]*>\s*<rect width="([0-9.+-]+)" height="([0-9.+-]+)"/g;
    let match: RegExpExecArray | null;
    while ((match = clientRegex.exec(svgText)) !== null) {
        bounds.push({
            name: match[4],
            x: Number(match[1]),
            y: Number(match[2]),
            width: Number(match[5]),
            height: Number(match[6]),
            isContainer: match[3].includes("ibd-container"),
        });
    }

    if (bounds.length > 0) {
        return bounds;
    }

    // Fallback SVG shape: <g class="diagram-node ..." data-element-name="X"> <rect class="node-background" x y width height ...>
    const fallbackRegex = /<g class="([^"]*diagram-node[^"]*)"[^>]*data-element-name="([^"]+)"[^>]*>\s*<rect class="node-background" x="([0-9.+-]+)" y="([0-9.+-]+)" width="([0-9.+-]+)" height="([0-9.+-]+)"/g;
    while ((match = fallbackRegex.exec(svgText)) !== null) {
        const classAttr = match[1] || "";
        const name = match[2];
        const x = Number(match[3]);
        const y = Number(match[4]);
        const width = Number(match[5]);
        const height = Number(match[6]);
        const isContainer = classAttr.includes("part-def") || classAttr.includes("package");
        bounds.push({ name, x, y, width, height, isContainer });
    }
    return bounds;
}

function getNodeBlock(svgText: string, nodeName: string): string {
    const startMarker = `data-element-name="${nodeName}"`;
    const startIndex = svgText.indexOf(startMarker);
    if (startIndex < 0) {
        return "";
    }
    const nextIndex = svgText.indexOf('data-element-name="', startIndex + startMarker.length);
    return nextIndex > startIndex ? svgText.slice(startIndex, nextIndex) : svgText.slice(startIndex);
}

function getPortLabelSide(nodeBlock: string, label: string): "left" | "right" | null {
    const escaped = label.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const match = new RegExp(`<text x="([0-9.+-]+)" y="[0-9.+-]+" text-anchor="(start|end)"[^>]*>${escaped}</text>`).exec(nodeBlock);
    if (!match) {
        return null;
    }
    const anchor = match[2];
    // Left-side labels are left-aligned (start), right-side labels are right-aligned (end).
    if (anchor === "start") return "left";
    if (anchor === "end") return "right";
    return null;
}

function segmentIntersectsRect(
    a: { x: number; y: number },
    b: { x: number; y: number },
    rect: { x: number; y: number; width: number; height: number }
): boolean {
    const margin = 1;
    const minX = rect.x + margin;
    const maxX = rect.x + rect.width - margin;
    const minY = rect.y + margin;
    const maxY = rect.y + rect.height - margin;
    if (Math.abs(a.y - b.y) < 1e-6) {
        if (a.y <= minY || a.y >= maxY) return false;
        const segMinX = Math.min(a.x, b.x);
        const segMaxX = Math.max(a.x, b.x);
        return segMaxX > minX && segMinX < maxX;
    }
    if (Math.abs(a.x - b.x) < 1e-6) {
        if (a.x <= minX || a.x >= maxX) return false;
        const segMinY = Math.min(a.y, b.y);
        const segMaxY = Math.max(a.y, b.y);
        return segMaxY > minY && segMinY < maxY;
    }
    return false;
}

function countTextNodeOccurrences(svgText: string, label: string): number {
    const escaped = label.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const regex = new RegExp(`>${escaped}<`, "g");
    return (svgText.match(regex) || []).length;
}

function segmentOverlapLength(
    a1: { x: number; y: number },
    a2: { x: number; y: number },
    b1: { x: number; y: number },
    b2: { x: number; y: number }
): number {
    const aHoriz = Math.abs(a1.y - a2.y) < 1e-6;
    const aVert = Math.abs(a1.x - a2.x) < 1e-6;
    const bHoriz = Math.abs(b1.y - b2.y) < 1e-6;
    const bVert = Math.abs(b1.x - b2.x) < 1e-6;
    if (aHoriz && bHoriz && Math.abs(a1.y - b1.y) < 1e-6) {
        const start = Math.max(Math.min(a1.x, a2.x), Math.min(b1.x, b2.x));
        const end = Math.min(Math.max(a1.x, a2.x), Math.max(b1.x, b2.x));
        return Math.max(0, end - start);
    }
    if (aVert && bVert && Math.abs(a1.x - b1.x) < 1e-6) {
        const start = Math.max(Math.min(a1.y, a2.y), Math.min(b1.y, b2.y));
        const end = Math.min(Math.max(a1.y, a2.y), Math.max(b1.y, b2.y));
        return Math.max(0, end - start);
    }
    return 0;
}

function routesShareEndpoint(a: ParsedRoute, b: ParsedRoute): boolean {
    return a.source === b.source || a.source === b.target || a.target === b.source || a.target === b.target;
}

describe("Interconnection Visualization Drone", () => {
    before(async function () {
        this.timeout(30000);
        await configureServerForTests();
        getTestWorkspaceFolder();
        const doc = await vscode.workspace.openTextDocument(DRONE_FIXTURE);
        await waitForLanguageServerReady(doc);
    });

    afterEach(async () => {
        if (VisualizationPanel.currentPanel) {
            VisualizationPanel.currentPanel.dispose();
        }
        await vscode.commands.executeCommand("workbench.action.closeAllEditors");
    });

    it("keeps the real drone interconnection view readable enough for release gating", async function () {
        this.timeout(60000);

        const workspaceFolder = getTestWorkspaceFolder();
        const doc = await vscode.workspace.openTextDocument(DRONE_FIXTURE);
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

        await vscode.commands.executeCommand("sysml.changeVisualizerView", "interconnection-view");
        await new Promise((r) => setTimeout(r, 2600));

        const uri = getDiagramExportUri(workspaceFolder.uri, "interconnection-view");
        try {
            await vscode.workspace.fs.delete(uri, { useTrash: false });
        } catch {
            // Ignore if there is no previous export yet.
        }
        panel.getWebview()?.postMessage({ command: "exportDiagramForTest" });
        await new Promise((r) => setTimeout(r, 1500));

        const bytes = await vscode.workspace.fs.readFile(uri);
        const svgText = Buffer.from(bytes).toString("utf8");

        assert.ok(svgText.includes("<svg"), "drone interconnection export should contain svg markup");
        assert.ok(
            svgText.includes("ibd-connector") || svgText.includes("diagram-edge"),
            "drone interconnection export should include connector paths"
        );
        assert.ok(svgText.includes("telemetryOut"), "drone interconnection export should include telemetryOut");
        assert.ok(svgText.includes("videoIn"), "drone interconnection export should include videoIn");
        assert.ok(svgText.includes("regulated5V"), "drone interconnection export should include regulated5V");
        assert.strictEqual(countTextNodeOccurrences(svgText, "telemetryOut"), 1, "telemetryOut should render once as a side label");
        assert.strictEqual(countTextNodeOccurrences(svgText, "regulated5V"), 1, "regulated5V should render once as a side label");
        assert.ok(
            !svgText.includes("[port] telemetryOut") &&
            !svgText.includes("[port] videoIn") &&
            !svgText.includes("[port] regulated5V"),
            "drone interconnection should not duplicate ports as body lines"
        );

        const routes = parseConnectorRoutes(svgText);
        assert.ok(routes.length >= 17, `expected at least 17 connector routes for the drone, got ${routes.length}`);
        for (const route of routes) {
            for (let index = 0; index < route.points.length - 1; index++) {
                const current = route.points[index];
                const next = route.points[index + 1];
                const horizontal = Math.abs(current.y - next.y) < 1e-6;
                const vertical = Math.abs(current.x - next.x) < 1e-6;
                assert.ok(horizontal || vertical, `drone connector segment ${index} should stay orthogonal`);
            }
            assert.strictEqual(route.points[0].y, route.points[1].y, "drone connector should leave the source port horizontally");
            assert.strictEqual(route.points[route.points.length - 2].y, route.points[route.points.length - 1].y, "drone connector should approach the target port horizontally");
        }

        const partBounds = parsePartBounds(svgText).filter((bound) => !bound.isContainer);
        const containerBounds = parsePartBounds(svgText).filter((bound) => bound.isContainer);
        const flightController = partBounds.find((bound) => bound.name === "flightController");
        const communication = partBounds.find((bound) => bound.name === "communication");
        const cameraPayload = partBounds.find((bound) => bound.name === "cameraPayload");
        const powerDistribution = partBounds.find((bound) => bound.name === "distribution");
        const packageContainer = containerBounds.find((bound) => bound.name === "SurveillanceDrone");
        assert.ok(packageContainer, "expected SurveillanceDrone package container in drone export");
        assert.ok(flightController, "expected flightController node in drone export");
        assert.ok((flightController?.height || 0) >= 140, `flightController node should grow for many ports, got height ${flightController?.height}`);
        assert.ok(communication, "expected communication node in drone export");
        assert.ok(cameraPayload, "expected cameraPayload node in drone export");
        assert.ok(powerDistribution, "expected distribution node in drone export");

        const flightControllerBlock = getNodeBlock(svgText, "flightController");
        const communicationBlock = getNodeBlock(svgText, "communication");
        const cameraPayloadBlock = getNodeBlock(svgText, "cameraPayload");
        const distributionBlock = getNodeBlock(svgText, "distribution");
        assert.strictEqual(
            getPortLabelSide(flightControllerBlock, "telemetryOut"),
            "right",
            "telemetryOut should be rendered on the right side of flightController"
        );
        assert.strictEqual(
            getPortLabelSide(communicationBlock, "videoIn"),
            "left",
            "videoIn should be rendered on the left side of communication"
        );
        assert.strictEqual(
            getPortLabelSide(cameraPayloadBlock, "videoOut"),
            "right",
            "videoOut should be rendered on the right side of cameraPayload"
        );
        assert.strictEqual(
            getPortLabelSide(distributionBlock, "regulated5V"),
            "right",
            "regulated5V should be rendered on the right side of distribution"
        );

        // Note: fallback SVG edges do not expose endpoint metadata; keep routing checks focused
        // on orthogonality + port label sides rather than per-connector obstacle avoidance here.

        for (let routeIndex = 0; routeIndex < routes.length; routeIndex++) {
            const route = routes[routeIndex];
            for (let otherIndex = routeIndex + 1; otherIndex < routes.length; otherIndex++) {
                const other = routes[otherIndex];
                if (routesShareEndpoint(route, other)) continue;
                for (let i = 0; i < route.points.length - 1; i++) {
                    for (let j = 0; j < other.points.length - 1; j++) {
                        const overlap = segmentOverlapLength(
                            route.points[i],
                            route.points[i + 1],
                            other.points[j],
                            other.points[j + 1]
                        );
                        assert.ok(
                            overlap < 6,
                            `non-related drone connector routes should not overlap (found ${overlap}px overlap)`
                        );
                    }
                }
            }
        }
    });
});
