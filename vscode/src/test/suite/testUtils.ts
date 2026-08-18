import * as assert from "assert";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import * as util from "util";
import * as vscode from "vscode";

export type ExtensionDebugState = {
  serverHealthState: "starting" | "ready" | "indexing" | "degraded" | "restarting" | "crashed";
  serverHealthDetail: string;
};
export const isCi = Boolean(process.env.CI);
export const integrationHookTimeoutMs = isCi ? 90000 : 60000;
const extensionServerReadyTimeoutMs = isCi ? 60000 : 30000;
const languageServerReadyTimeoutMs = isCi ? 60000 : 45000;
/** Structured logs for integration tests — always emitted to the test host console (CI-visible). */
export function integrationTestLog(phase: string, payload: Record<string, unknown>): void {
  try {
    console.log(`[spec42-test][${phase}] ${JSON.stringify(payload)}`);
  } catch {
    console.log(`[spec42-test][${phase}]`, payload);
  }
}

function normalizeServerPathForComparison(serverPath: string): string {
  if (!serverPath || serverPath === "spec42") {
    return serverPath;
  }
  try {
    return fs.realpathSync(path.resolve(serverPath));
  } catch {
    return path.resolve(serverPath);
  }
}

function summarizeWaitValue(value: unknown): string {
  if (value === undefined) {
    return "undefined";
  }
  if (value === null) {
    return "null";
  }
  if (typeof value !== "object") {
    return String(value);
  }
  if (Array.isArray(value)) {
    return `Array(${value.length})`;
  }
  try {
    const record = value as Record<string, unknown>;
    const summary: Record<string, unknown> = {};
    for (const [key, entry] of Object.entries(record).slice(0, 24)) {
      if (entry === null || entry === undefined) {
        summary[key] = entry;
      } else if (typeof entry === "object") {
        summary[key] = Array.isArray(entry) ? `Array(${entry.length})` : "Object";
      } else {
        summary[key] = entry;
      }
    }
    return JSON.stringify(summary);
  } catch {
    return util.inspect(value, { depth: 2, maxArrayLength: 8, breakLength: 120 });
  }
}

async function getExtensionDebugState(): Promise<ExtensionDebugState> {
  return (await vscode.commands.executeCommand(
    "sysml.debug.getExtensionState"
  )) as ExtensionDebugState;
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
  let nextHeartbeatAt =
    timeoutMs > 15000 ? Date.now() + 10000 : Number.POSITIVE_INFINITY;
  while (Date.now() < deadline) {
    lastValue = await producer();
    if (isReady(lastValue)) {
      return lastValue as T;
    }
    if (Date.now() >= nextHeartbeatAt) {
      integrationTestLog("waitFor:heartbeat", {
        label,
        elapsedMs: timeoutMs - Math.max(0, deadline - Date.now()),
        timeoutMs,
        lastValue: summarizeWaitValue(lastValue),
      });
      nextHeartbeatAt = Date.now() + 10000;
    }
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  assert.fail(
    `${label} did not become ready within ${timeoutMs}ms. Last value: ${summarizeWaitValue(lastValue)}`
  );
}

export async function waitForExtensionServerReady(
  timeoutMs = extensionServerReadyTimeoutMs
): Promise<void> {
  await waitFor(
    "extension server ready",
    () => getExtensionDebugState(),
    (value) => {
      const state = value?.serverHealthState;
      if (!state || state === "crashed") {
        return false;
      }
      return state === "ready" || state === "degraded" || state === "indexing";
    },
    timeoutMs,
    300
  );
  const state = await getExtensionDebugState();
  if (state.serverHealthState === "crashed") {
    assert.fail(`extension server crashed: ${state.serverHealthDetail}`);
  }
  assert.ok(
    state.serverHealthState === "ready" ||
      state.serverHealthState === "degraded" ||
      state.serverHealthState === "indexing",
    `extension server did not reach a usable state (got ${state.serverHealthState}: ${state.serverHealthDetail})`
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
  const envServerPath = (process.env.SPEC42_SERVER_PATH || "").trim();
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
  const normalizedCurrent = currentServerPath
    ? normalizeServerPathForComparison(currentServerPath)
    : "";
  const normalizedTarget = normalizeServerPathForComparison(serverPath);
  const serverPathChanged =
    !envServerPath && normalizedCurrent !== normalizedTarget;

  integrationTestLog("configureServerForTests:start", {
    serverPath,
    envServerPath: envServerPath || null,
    currentServerPath: currentServerPath || null,
    serverPathChanged,
    forceRestart: options?.forceRestart === true,
  });

  if (!envServerPath && serverPathChanged) {
    await vscode.workspace
      .getConfiguration("spec42")
      .update("serverPath", serverPath, vscode.ConfigurationTarget.Workspace);
  }

  await extension.activate();

  try {
    await vscode.commands.executeCommand("sysml.debug.waitForLanguageClientReady");
  } catch {
    // Fall back to extension health polling when the debug command is unavailable.
  }

  await waitForExtensionServerReady();

  let state: ExtensionDebugState | undefined;
  try {
    state = await getExtensionDebugState();
  } catch {
    state = undefined;
  }

  const shouldRestart =
    options?.forceRestart === true || state?.serverHealthState === "crashed";

  integrationTestLog("configureServerForTests:ready", {
    shouldRestart,
    serverHealthState: state?.serverHealthState ?? null,
    serverHealthDetail: state?.serverHealthDetail ?? null,
  });

  if (shouldRestart) {
    await vscode.commands.executeCommand("sysml.restartServer");
    await waitForExtensionServerReady();
  }
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
