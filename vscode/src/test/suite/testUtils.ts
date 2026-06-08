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
};

export const isCi = Boolean(process.env.CI);
export const integrationHookTimeoutMs = isCi ? 90000 : 60000;
export const extensionServerReadyTimeoutMs = isCi ? 60000 : 30000;
export const languageServerReadyTimeoutMs = isCi ? 45000 : 20000;
export const visualizationPanelTimeoutMs = isCi ? 45000 : 20000;
export const diagramExportTimeoutMs = isCi ? 30000 : 12000;

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
