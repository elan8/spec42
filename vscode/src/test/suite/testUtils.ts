import * as assert from "assert";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";

export type ExtensionDebugState = {
  serverHealthState: "starting" | "ready" | "indexing" | "degraded" | "restarting" | "crashed";
  serverHealthDetail: string;
  visualizerOpen?: boolean;
  workspaceIndexSummary?: {
    scannedFiles: number;
    loadedFiles: number;
    truncated: boolean;
    cancelled: boolean;
    failures: number;
  };
  modelExplorer?: {
    lastRevealedElementId?: string;
    pendingWorkspaceLoadRunId?: string;
  };
  lastSemanticIndexReadyWorkspaceFileCount?: number;
  lastVisualizerRender?: {
    view: string;
    outcome: string;
    graphNodes: number;
    hasExportableSvg: boolean;
    timestampMs: number;
  };
};

export const isCi = Boolean(process.env.CI);
export const integrationHookTimeoutMs = isCi ? 90000 : 60000;
export const extensionServerReadyTimeoutMs = isCi ? 60000 : 30000;
export const languageServerReadyTimeoutMs = isCi ? 45000 : 45000;
export const visualizationPanelTimeoutMs = isCi ? 45000 : 45000;
export const diagramExportTimeoutMs = isCi ? 30000 : 30000;
export const visualizerRenderTimeoutMs = isCi ? 45000 : 45000;

export type VisualizerRenderResult = {
  view: string;
  outcome: string;
  graphNodes: number;
  hasExportableSvg: boolean;
  timestampMs: number;
  updateId?: string;
};

export type VisualizerSeedSummary = {
  modelReady: boolean;
  ibdConnectors: number;
  ibdParts: number;
  ibdPorts: number;
  graphNodes: number;
  viewCandidateCount: number;
  viewCandidateIds: string[];
  viewCandidateNames: string[];
  selectedView?: string;
  selectedViewName?: string;
  emptyStateMessage?: string;
  requestedViewId?: string;
  requestedSelectedView?: string;
};

/** Structured logs for integration tests — always emitted to the test host console (CI-visible). */
export function integrationTestLog(phase: string, payload: Record<string, unknown>): void {
  try {
    // eslint-disable-next-line no-console
    console.log(`[spec42-test][${phase}] ${JSON.stringify(payload)}`);
  } catch {
    // eslint-disable-next-line no-console
    console.log(`[spec42-test][${phase}]`, payload);
  }
}

function shouldLogInterconnectionDebug(viewId: string): boolean {
  return (
    viewId === "interconnection-view" ||
    process.env.SPEC42_TEST_DEBUG_INTERCONNECTION === "1"
  );
}

export function visualizationToSeedSummary(visualization: Record<string, unknown> | undefined): VisualizerSeedSummary {
  const ibd = visualization?.ibd as
    | { parts?: unknown[]; ports?: unknown[]; connectors?: unknown[] }
    | undefined;
  const viewCandidates = Array.isArray(visualization?.viewCandidates)
    ? (visualization.viewCandidates as Array<{ id?: string; name?: string }>)
    : [];
  return {
    modelReady: visualization?.modelReady !== false,
    ibdConnectors: ibd?.connectors?.length ?? 0,
    ibdParts: ibd?.parts?.length ?? 0,
    ibdPorts: ibd?.ports?.length ?? 0,
    graphNodes: (visualization?.graph as { nodes?: unknown[] } | undefined)?.nodes?.length ?? 0,
    viewCandidateCount: viewCandidates.length,
    viewCandidateIds: viewCandidates.map((candidate) => candidate.id ?? ""),
    viewCandidateNames: viewCandidates.map((candidate) => candidate.name ?? ""),
    selectedView: visualization?.selectedView as string | undefined,
    selectedViewName: visualization?.selectedViewName as string | undefined,
    emptyStateMessage: visualization?.emptyStateMessage as string | undefined,
  };
}

async function fetchVisualizationSnapshot(
  workspaceRootUri: vscode.Uri,
  viewId: string,
  selectedView?: string
): Promise<Record<string, unknown>> {
  const visualization = await vscode.commands.executeCommand<Record<string, unknown>>(
    "sysml.debug.getVisualizationForTests",
    workspaceRootUri.toString(),
    viewId,
    selectedView
  );
  const summary = visualizationToSeedSummary(visualization);
  const viewCandidates = Array.isArray(visualization?.viewCandidates)
    ? (visualization.viewCandidates as Array<{ rendererView?: string }>)
    : [];
  return {
    ...summary,
    emptyStateMessage: summary.emptyStateMessage ?? null,
    selectedView: summary.selectedView ?? null,
    selectedViewName: summary.selectedViewName ?? null,
    viewCandidateRendererViews: viewCandidates.map((candidate) => candidate.rendererView ?? ""),
  };
}

async function logVisualizerFailureContext(
  phase: string,
  workspaceRootUri: vscode.Uri,
  viewId: string,
  selectedView: string | undefined,
  extra: Record<string, unknown> = {}
): Promise<void> {
  if (!shouldLogInterconnectionDebug(viewId)) {
    return;
  }
  let extensionState: ExtensionDebugState | undefined;
  let visualization: Record<string, unknown> | undefined;
  try {
    extensionState = await getExtensionDebugState();
  } catch (error) {
    integrationTestLog(`${phase}:extensionStateError`, {
      message: error instanceof Error ? error.message : String(error),
    });
  }
  try {
    visualization = await fetchVisualizationSnapshot(workspaceRootUri, viewId, selectedView);
  } catch (error) {
    integrationTestLog(`${phase}:visualizationError`, {
      message: error instanceof Error ? error.message : String(error),
    });
  }
  integrationTestLog(phase, {
    viewId,
    selectedView: selectedView ?? null,
    extensionState,
    visualization,
    ...extra,
  });
}

async function getExtensionDebugState(): Promise<ExtensionDebugState> {
  return (await vscode.commands.executeCommand(
    "sysml.debug.getExtensionState"
  )) as ExtensionDebugState;
}

/** Wait until the extension bundle reports an open visualizer panel. */
export async function waitForVisualizerOpen(
  timeoutMs = visualizationPanelTimeoutMs
): Promise<void> {
  await waitFor(
    "visualization panel",
    () => getExtensionDebugState(),
    (state) => state?.visualizerOpen === true,
    timeoutMs,
    300
  );
}

export async function waitForVisualizerClosed(timeoutMs = 10000): Promise<void> {
  await waitFor(
    "visualization panel disposal",
    () => getExtensionDebugState(),
    (state) => !state?.visualizerOpen,
    timeoutMs,
    100
  );
}

export async function disposeVisualizer(): Promise<void> {
  await vscode.commands.executeCommand("sysml.debug.disposeVisualizer");
}

export async function triggerVisualizerExportForTest(): Promise<void> {
  await vscode.commands.executeCommand("sysml.debug.exportVisualizerDiagramForTest");
}

export async function getVisualizationForTests(
  workspaceRootUri: vscode.Uri,
  viewId: string,
  selectedView?: string
): Promise<any> {
  return await vscode.commands.executeCommand<any>(
    "sysml.debug.getVisualizationForTests",
    workspaceRootUri.toString(),
    viewId,
    selectedView
  );
}

export async function waitForVisualizationModel(
  workspaceRootUri: vscode.Uri,
  viewId: string,
  isReady: (visualization: any) => boolean,
  timeoutMs = visualizationPanelTimeoutMs,
  selectedView?: string
): Promise<any> {
  return await waitFor(
    `${viewId} visualization model`,
    () => getVisualizationForTests(workspaceRootUri, viewId, selectedView),
    (value) => Boolean(value && isReady(value)),
    timeoutMs,
    300
  );
}

export async function waitForVisualizerRender(
  viewId: string,
  isReady: (render: VisualizerRenderResult) => boolean,
  options?: {
    timeoutMs?: number;
    outcome?: string | string[];
    minGraphNodes?: number;
    seedSummary?: VisualizerSeedSummary;
    workspaceRootUri?: vscode.Uri;
    selectedView?: string;
  }
): Promise<VisualizerRenderResult> {
  const timeoutMs = options?.timeoutMs ?? visualizerRenderTimeoutMs;
  let render: VisualizerRenderResult | undefined;
  try {
    render = await vscode.commands.executeCommand<VisualizerRenderResult>(
      "sysml.debug.waitForVisualizerRender",
      {
        view: viewId,
        outcome: options?.outcome ?? ["diagram", "empty"],
        minGraphNodes: options?.minGraphNodes,
        timeoutMs,
      }
    );
  } catch (error) {
    if (options?.workspaceRootUri) {
      await logVisualizerFailureContext(
        "waitForVisualizerRender:timeout",
        options.workspaceRootUri,
        viewId,
        options.selectedView,
        {
          timeoutMs,
          error: error instanceof Error ? error.message : String(error),
          seedSummary: options.seedSummary ?? null,
        }
      );
    }
    throw error;
  }
  const ready = Boolean(render && isReady(render));
  if (!ready && options?.workspaceRootUri) {
    await logVisualizerFailureContext(
      "waitForVisualizerRender:notReady",
      options.workspaceRootUri,
      viewId,
      options.selectedView,
      {
        render: render ?? null,
        seedSummary: options.seedSummary ?? null,
      }
    );
  }
  assert.ok(
    ready,
    `${viewId} visualizer render was not ready` +
      (render
        ? ` (outcome=${render.outcome}, graphNodes=${render.graphNodes}, hasExportableSvg=${render.hasExportableSvg})`
        : "")
  );
  return render!;
}

function renderSettledForView(viewId: string, render: VisualizerRenderResult): boolean {
  if (render.outcome === "cancelled" || render.outcome === "error") {
    return false;
  }
  if (viewId === "interconnection-view") {
    return render.outcome === "diagram" && render.hasExportableSvg;
  }
  return render.outcome === "diagram" || render.outcome === "empty";
}

/** Drive production updateFlow and wait for webview renderComplete before export tests. */
export async function seedVisualizerWebviewFromModel(
  workspaceRootUri: vscode.Uri,
  viewId: string,
  isReady: (summary: VisualizerSeedSummary) => boolean,
  options?: { timeoutMs?: number; selectedView?: string; renderTimeoutMs?: number }
): Promise<VisualizerSeedSummary> {
  const modelTimeoutMs = options?.timeoutMs ?? visualizationPanelTimeoutMs;
  const renderTimeoutMs = options?.renderTimeoutMs ?? visualizerRenderTimeoutMs;
  const selectedView = options?.selectedView;

  if (shouldLogInterconnectionDebug(viewId)) {
    integrationTestLog("seedVisualizer:before", {
      workspaceRootUri: workspaceRootUri.toString(),
      viewId,
      selectedView: selectedView ?? null,
      lspSnapshot: await fetchVisualizationSnapshot(
        workspaceRootUri,
        viewId,
        selectedView
      ).catch((error) => ({
        error: error instanceof Error ? error.message : String(error),
      })),
    });
  }

  await waitForVisualizationModel(
    workspaceRootUri,
    viewId,
    (visualization) => {
      if (visualization?.modelReady === false) {
        return false;
      }
      return isReady(visualizationToSeedSummary(visualization));
    },
    modelTimeoutMs,
    selectedView
  );

  const summary = await vscode.commands.executeCommand<VisualizerSeedSummary>(
    "sysml.debug.seedVisualizerFromLspForTests",
    workspaceRootUri.toString(),
    viewId,
    selectedView
  );
  assert.ok(summary, `${viewId} visualizer seed returned no summary`);
  if (shouldLogInterconnectionDebug(viewId)) {
    integrationTestLog("seedVisualizer:afterLspSeed", summary);
  }

  const render = await waitForVisualizerRender(
    viewId,
    (value) => renderSettledForView(viewId, value),
    {
      timeoutMs: renderTimeoutMs,
      seedSummary: summary,
      workspaceRootUri,
      selectedView,
    }
  );

  const finalVisualization = await getVisualizationForTests(
    workspaceRootUri,
    viewId,
    selectedView
  );
  const result = {
    ...visualizationToSeedSummary(finalVisualization),
    graphNodes: render.graphNodes,
    requestedViewId: viewId,
    requestedSelectedView: selectedView,
  };
  assert.ok(
    isReady(result),
    `${viewId} visualizer model was not ready after render (ibdConnectors=${result.ibdConnectors}, selectedView=${result.selectedView ?? "auto"})`
  );
  if (shouldLogInterconnectionDebug(viewId)) {
    integrationTestLog("seedVisualizer:complete", {
      summary: result,
      render,
    });
  }
  return result;
}

export async function closeAllEditorsForTests(): Promise<void> {
  await vscode.commands.executeCommand("workbench.action.closeAllEditors");
  await waitFor(
    "all editors closed",
    async () => vscode.window.visibleTextEditors.length,
    (count) => count === 0,
    10000,
    100
  );
}

export async function clearVisualizerPackageSelection(): Promise<void> {
  await vscode.commands.executeCommand("sysml.debug.clearVisualizerPackageSelection");
}

export async function selectVisualizerPackage(packageName: string): Promise<void> {
  await vscode.commands.executeCommand(
    "sysml.debug.selectVisualizerPackage",
    packageName
  );
}

export function getTestWorkspaceFolder(): vscode.WorkspaceFolder {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  assert.ok(workspaceFolder, "Workspace folder should be open");
  return workspaceFolder;
}

export function getFixtureUri(relativePath: string): vscode.Uri {
  return vscode.Uri.joinPath(getTestWorkspaceFolder().uri, relativePath);
}

export function getFixturePath(relativePath: string): string {
  return getFixtureUri(relativePath).fsPath;
}

export function getExternalFixturePath(absolutePath: string): string {
  assert.ok(fs.existsSync(absolutePath), `Expected fixture to exist at ${absolutePath}`);
  return absolutePath;
}

export async function tryReadWorkspaceText(uri: vscode.Uri): Promise<string | undefined> {
  try {
    const bytes = await vscode.workspace.fs.readFile(uri);
    return Buffer.from(bytes).toString("utf8");
  } catch {
    return undefined;
  }
}

function tryResolveServerBinary(extensionPath: string): string {
  const platform = process.platform;
  const arch = process.arch;
  const binaryName = platform === "win32" ? "spec42.exe" : "spec42";

  // Allow CI to override explicitly.
  const fromEnv = (process.env.SPEC42_SERVER_PATH || "").trim();
  if (fromEnv) {
    return fromEnv;
  }

  // Repo-local cargo outputs (common for CI and local dev).
  const repoRoot = path.resolve(extensionPath, "..");
  const debugPath = path.join(repoRoot, "target", "debug", binaryName);
  if (fs.existsSync(debugPath)) return debugPath;
  const releasePath = path.join(repoRoot, "target", "release", binaryName);
  if (fs.existsSync(releasePath)) return releasePath;

  // Bundled server inside the extension (packaged layout).
  const bundledPath = path.join(extensionPath, "server", `${platform}-${arch}`, binaryName);
  if (fs.existsSync(bundledPath)) return bundledPath;

  // Fallback: rely on PATH.
  return "spec42";
}

export async function waitFor<T>(
  label: string,
  producer: () => PromiseLike<T | undefined>,
  isReady: (value: T | undefined) => boolean,
  timeoutMs = 15000,
  intervalMs = 250
): Promise<T> {
  const deadline = Date.now() + timeoutMs;
  let lastValue: T | undefined;
  while (Date.now() < deadline) {
    lastValue = await producer();
    if (isReady(lastValue)) {
      return lastValue as T;
    }
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  assert.fail(
    `${label} did not become ready within ${timeoutMs}ms. Last value: ${JSON.stringify(lastValue)}`
  );
}

export async function waitForModelExplorerWorkspaceReady(
  timeoutMs = extensionServerReadyTimeoutMs
): Promise<void> {
  await waitFor(
    "model explorer workspace indexing",
    () => getExtensionDebugState(),
    (state) => {
      if (!state || state.serverHealthState !== "ready") {
        return false;
      }
      if (state.modelExplorer?.pendingWorkspaceLoadRunId) {
        return false;
      }
      const indexedFiles =
        state.lastSemanticIndexReadyWorkspaceFileCount ??
        state.workspaceIndexSummary?.loadedFiles ??
        0;
      return indexedFiles > 0;
    },
    timeoutMs,
    300
  );
}

export async function waitForExtensionServerReady(
  timeoutMs = extensionServerReadyTimeoutMs
): Promise<void> {
  await waitFor(
    "extension server ready",
    () =>
      getExtensionDebugState(),
    (value) =>
      value?.serverHealthState === "ready" || value?.serverHealthState === "degraded",
    timeoutMs,
    300
  );
}

export async function configureServerForTests(options?: {
  forceRestart?: boolean;
}): Promise<void> {
  const testExportDir = path.join(os.tmpdir(), "spec42-vscode-test-exports");
  fs.mkdirSync(testExportDir, { recursive: true });
  process.env.SPEC42_TEST_EXPORT_DIR = testExportDir;

  const extension = vscode.extensions.all.find(
    (e) => e.packageJSON?.name === "spec42"
  );
  assert.ok(extension, "SysML Language Server extension should be installed");

  const serverPath = tryResolveServerBinary(extension.extensionPath);
  if (serverPath !== "spec42") {
    assert.ok(
      fs.existsSync(serverPath),
      `Expected SysML server binary for tests at ${serverPath}.`
    );
  }

  const currentServerPath = vscode.workspace
    .getConfiguration("spec42")
    .get<string>("serverPath")
    ?.trim();
  const serverPathChanged = currentServerPath !== serverPath;

  await vscode.workspace
    .getConfiguration("spec42")
    .update("serverPath", serverPath, vscode.ConfigurationTarget.Workspace);
  await extension.activate();

  let state: ExtensionDebugState | undefined;
  try {
    state = await getExtensionDebugState();
  } catch {
    state = undefined;
  }

  const shouldRestart =
    options?.forceRestart === true ||
    serverPathChanged ||
    state?.serverHealthState === "crashed";

  if (shouldRestart) {
    await vscode.commands.executeCommand("sysml.restartServer");
  }

  await waitForExtensionServerReady();
}

export async function waitForLanguageServerReady(
  doc: vscode.TextDocument,
  timeoutMs = languageServerReadyTimeoutMs
): Promise<void> {
  await vscode.window.showTextDocument(doc);
  await waitFor(
    "language server ready",
    async () => {
      const [symbols, hovers] = await Promise.all([
        vscode.commands.executeCommand<
          vscode.DocumentSymbol[] | vscode.SymbolInformation[]
        >(
          "vscode.executeDocumentSymbolProvider",
          doc.uri,
        ),
        vscode.commands.executeCommand<vscode.Hover[]>(
          "vscode.executeHoverProvider",
          doc.uri,
          new vscode.Position(0, 0)
        ),
      ]);
      return {
        symbols,
        hovers,
      };
    },
    (value) =>
      Boolean(
        value &&
        ((Array.isArray(value.symbols) && value.symbols.length > 0) ||
          (Array.isArray(value.hovers) && value.hovers.length > 0))
      ),
    timeoutMs,
    300
  );
}

export function getDiagramExportUri(workspaceUri: vscode.Uri, viewId: string): vscode.Uri {
  const configuredDir = (process.env.SPEC42_TEST_EXPORT_DIR || "").trim();
  if (!configuredDir) {
    return vscode.Uri.joinPath(workspaceUri, "test-output", "diagrams", `${viewId}.svg`);
  }

  const workspaceName = path.basename(workspaceUri.fsPath).replace(/[^a-zA-Z0-9_-]/g, "_");
  return vscode.Uri.file(path.join(configuredDir, workspaceName, `${viewId}.svg`));
}

export async function waitForDiagramExport(
  workspaceUri: vscode.Uri,
  viewId: string,
  isReady: (svgText: string) => boolean,
  timeoutMs = diagramExportTimeoutMs
): Promise<{ uri: vscode.Uri; svgText: string }> {
  const uri = getDiagramExportUri(workspaceUri, viewId);
  const svgText = await waitFor(
    `${viewId} svg export`,
    async () => (await tryReadWorkspaceText(uri)) ?? "",
    (value) => {
      const text = value ?? "";
      return text.includes("<svg") && isReady(text);
    },
    timeoutMs,
    200
  );
  return { uri, svgText };
}

export async function triggerDiagramExportAndWait(
  workspaceUri: vscode.Uri,
  viewId: string,
  isReady: (svgText: string) => boolean,
  timeoutMs = diagramExportTimeoutMs
): Promise<{ uri: vscode.Uri; svgText: string }> {
  const uri = getDiagramExportUri(workspaceUri, viewId);
  let lastSvg = "";
  try {
    const svgText = await waitFor(
      `${viewId} triggered svg export`,
      async () => {
        await triggerVisualizerExportForTest();
        lastSvg = (await tryReadWorkspaceText(uri)) ?? "";
        return lastSvg;
      },
      (value) => {
        const text = value ?? "";
        return text.includes("<svg") && isReady(text);
      },
      timeoutMs,
      500
    );
    if (shouldLogInterconnectionDebug(viewId)) {
      integrationTestLog("triggerDiagramExport:success", {
        viewId,
        exportUri: uri.toString(),
        svgLength: svgText.length,
        hasIbdConnector: svgText.includes("ibd-connector"),
        hasConnectedBlocks: svgText.includes("ConnectedBlocks::"),
        hasItPackage: svgText.includes("IT::"),
      });
    }
    return { uri, svgText };
  } catch (error) {
    if (shouldLogInterconnectionDebug(viewId)) {
      await logVisualizerFailureContext(
        "triggerDiagramExport:timeout",
        workspaceUri,
        viewId,
        undefined,
        {
          exportUri: uri.toString(),
          lastSvgLength: lastSvg.length,
          lastSvgPreview: lastSvg.slice(0, 400),
          hasSvgTag: lastSvg.includes("<svg"),
          hasIbdConnector: lastSvg.includes("ibd-connector"),
          hasConnectedBlocks: lastSvg.includes("ConnectedBlocks::"),
          hasItPackage: lastSvg.includes("IT::"),
          error: error instanceof Error ? error.message : String(error),
        }
      );
    }
    throw error;
  }
}
