import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";

type DebugExtensionState = {
  serverHealthState: "starting" | "ready" | "indexing" | "degraded" | "restarting" | "crashed";
  serverHealthDetail: string;
};

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

export async function configureServerForTests(): Promise<void> {
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

  await vscode.workspace
    .getConfiguration("spec42")
    .update("serverPath", serverPath, vscode.ConfigurationTarget.Workspace);
  const wasActive = extension.isActive;
  await extension.activate();
  if (wasActive) {
    await vscode.commands.executeCommand("sysml.restartServer");
  }

  await waitFor(
    "extension server health",
    () =>
      vscode.commands.executeCommand<DebugExtensionState>(
        "sysml.debug.getExtensionState"
      ),
    (value) =>
      Boolean(
        value &&
        (value.serverHealthState === "ready" || value.serverHealthState === "degraded")
      ),
    20000,
    300
  );
}

export async function waitForLanguageServerReady(
  doc: vscode.TextDocument,
  timeoutMs = 20000
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

export async function waitForDiagramExport(
  workspaceUri: vscode.Uri,
  viewId: string,
  isReady: (svgText: string) => boolean,
  timeoutMs = 12000
): Promise<{ uri: vscode.Uri; svgText: string }> {
  const uri = vscode.Uri.joinPath(workspaceUri, "test-output", "diagrams", `${viewId}.svg`);
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
