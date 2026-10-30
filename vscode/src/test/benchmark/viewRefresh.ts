/** Opt-in Extension Development Host benchmark; deliberately outside the CI test suites. */
import * as assert from "node:assert/strict";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import { createHash } from "node:crypto";
import * as vscode from "vscode";
import { isPresentedFrame, parseRenderMeasurement } from "../../diagram/diagramPerformance";

type Event = Record<string, unknown> & { event: string };
type Sample = {
  state: "closed" | "visible" | "hidden";
  kind: "single" | "burst";
  version: number;
  editStartedMs: number;
  edits: number;
  modelDigest?: string;
  editToFrameAckMs?: number;
  frame?: Event;
  windowFocused?: boolean;
};

function required(name: string): string {
  const value = process.env[name];
  assert.ok(value, `${name} must be supplied explicitly`);
  return value;
}

function integer(name: string, fallback: number): number {
  const value = Number(process.env[name] ?? fallback);
  assert.ok(Number.isSafeInteger(value) && value > 0, `${name} must be a positive integer`);
  return value;
}

const delay = (ms: number): Promise<void> => new Promise(resolve => setTimeout(resolve, ms));

function filesUnder(root: string): string[] {
  return fs.readdirSync(root, { withFileTypes: true }).flatMap(entry => {
    const fullPath = path.join(root, entry.name);
    // Symlinks are excluded rather than traversed outside the declared measurement inputs.
    return entry.isDirectory() ? filesUnder(fullPath) : entry.isFile() ? [fullPath] : [];
  });
}

/** Read the native persisted OutputChannel, without introducing another extension event bus. */
class OutputRecords {
  readonly events: Event[] = [];
  private readonly offsets = new Map<string, number>();
  private readonly partial = new Map<string, string>();

  constructor(private readonly logRoot: string) {}

  poll(): void {
    if (!fs.existsSync(this.logRoot)) return;
    for (const file of filesUnder(this.logRoot).filter(file => /[\\/]\d+-SysML\.log$/.test(file))) {
      const offset = this.offsets.get(file) ?? 0;
      const size = fs.statSync(file).size;
      assert.ok(size >= offset, "SysML log was truncated during measurement");
      if (size === offset) continue;
      const bytes = Buffer.alloc(size - offset);
      const fd = fs.openSync(file, "r");
      try { fs.readSync(fd, bytes, 0, bytes.length, offset); } finally { fs.closeSync(fd); }
      this.offsets.set(file, size);
      const lines = ((this.partial.get(file) ?? "") + bytes.toString("utf8")).split(/\r?\n/);
      this.partial.set(file, lines.pop() ?? "");
      for (const line of lines) {
        const marker = "[SysML][perf] ";
        const start = line.indexOf(marker);
        if (start < 0) continue;
        const record = JSON.parse(line.slice(start + marker.length)) as Event;
        if (typeof record.event === "string") this.events.push(record);
      }
    }
  }

  async wait(label: string, predicate: (event: Event) => boolean, after = 0): Promise<Event> {
    const deadline = performance.now() + 60_000;
    while (performance.now() < deadline) {
      this.poll();
      const result = this.events.slice(after).find(predicate);
      if (result) return result;
      await delay(50);
    }
    throw new Error(`Timed out waiting for ${label}; last records: ${JSON.stringify(this.events.slice(-8))}`);
  }
}

function hashes(files: string[]): Record<string, string> {
  return Object.fromEntries([...new Set(files)].sort().map(file => [
    file, createHash("sha256").update(fs.readFileSync(file)).digest("hex"),
  ]));
}

function modelInputs(root: string): string[] {
  const result: string[] = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    if ([".git", "node_modules", "target", ".vscode-test"].includes(entry.name)) continue;
    const file = path.join(root, entry.name);
    if (entry.isDirectory()) result.push(...modelInputs(file));
    else if (entry.isFile() && (/\.(sysml|kerml|kpar|toml)$/i.test(entry.name) || entry.name === "settings.json" || entry.name === ".project.json")) result.push(file);
  }
  return result;
}

function hasFrame(event: Event): boolean {
  const measurement = parseRenderMeasurement(event);
  return event.event === "view:renderComplete" && measurement !== undefined && isPresentedFrame(measurement);
}

function missingLibraryContext(): boolean {
  return vscode.languages.getDiagnostics().some(([, diagnostics]) => diagnostics.some(diagnostic =>
    (typeof diagnostic.code === "object" ? diagnostic.code.value : diagnostic.code) === "missing_library_context"));
}

export async function run(): Promise<void> {
  const workspace = path.resolve(required("SPEC42_VIEW_WORKSPACE"));
  const documentPath = path.resolve(required("SPEC42_VIEW_DOCUMENT"));
  const reportPath = path.resolve(required("SPEC42_VIEW_REPORT"));
  const userData = path.resolve(required("SPEC42_VIEW_USER_DATA"));
  const server = path.resolve(required("SPEC42_SERVER_PATH"));
  const iterations = integer("SPEC42_VIEW_ITERATIONS", 20);
  const inactiveIterations = integer("SPEC42_VIEW_INACTIVE_ITERATIONS", 1);
  const burstSize = integer("SPEC42_VIEW_BURST_SIZE", 10);
  const records = new OutputRecords(path.join(userData, "logs"));
  const samples: Sample[] = [];
  const report: Record<string, unknown> = {
    schema: 1, status: "failed", workspace, documentPath, server, iterations, inactiveIterations, burstSize,
    vscodeVersion: vscode.version, platform: process.platform, arch: process.arch,
    machine: os.hostname(), cpuModel: os.cpus()[0]?.model, cpuCount: os.cpus().length,
    totalMemoryBytes: os.totalmem(),
    versions: process.versions, launchArgs: process.env.SPEC42_VIEW_LAUNCH_ARGS,
    frameDefinition: "Mounted SVG followed by two visible requestAnimationFrame callbacks and host acknowledgement; not a compositor timestamp",
    resourceScope: "ExtensionHost including benchmark driver; excludes server, workbench renderer and GPU",
    samples, events: records.events,
  };
  let document: vscode.TextDocument | undefined;
  let original: string | undefined;
  let before: Record<string, string> | undefined;
  let focusLosses = 0;
  const focusWatch = vscode.window.onDidChangeWindowState(state => { if (!state.focused) focusLosses++; });
  try {
    assert.equal(vscode.workspace.workspaceFolders?.length, 1);
    assert.equal(path.relative(fs.realpathSync(vscode.workspace.workspaceFolders![0].uri.fsPath), fs.realpathSync(workspace)), "");
    assert.equal(vscode.workspace.getConfiguration("files", vscode.Uri.file(documentPath)).get("autoSave"), "off", "Auto-save must be disabled before opening the source");
    assert.equal(vscode.workspace.getConfiguration("spec42").get("performanceLogging.enabled"), true);
    const extension = vscode.extensions.getExtension("Elan8.spec42");
    assert.ok(extension, "Development extension must be installed");
    const inputRoots = JSON.parse(required("SPEC42_VIEW_LIBRARY_INPUTS")) as string[];
    assert.ok(Array.isArray(inputRoots) && inputRoots.length > 0 && inputRoots.every(root => typeof root === "string"));
    const inputs = [documentPath, server, __filename,
      path.join(extension.extensionPath, "out", "extension.js"),
      path.join(extension.extensionPath, "media", "diagram-viewer.js"),
      ...modelInputs(workspace), ...inputRoots.flatMap(root => fs.statSync(root).isDirectory() ? modelInputs(root) : [root])];
    before = hashes(inputs);
    report.inputsBefore = before;
    document = await vscode.workspace.openTextDocument(documentPath);
    assert.equal(document.isDirty, false, "Benchmark requires a fresh isolated editor buffer");
    original = document.getText();
    await vscode.window.showTextDocument(document);
    await extension.activate();
    await vscode.commands.executeCommand("sysml.debug.waitForLanguageClientReady");
    await vscode.commands.executeCommand("workbench.action.closeSidebar");
    await vscode.commands.executeCommand("workbench.action.closeAuxiliaryBar");
    const initialUri = document.uri.toString();
    const initialVersion = document.version;
    await records.wait("initial document publication before steady-state edits", event =>
      event.event === "backend:publication" && event.editorVersions !== null && typeof event.editorVersions === "object" &&
      Object.entries(event.editorVersions).some(([key, version]) => vscode.Uri.parse(key).toString() === initialUri && version === initialVersion));
    report.resolvedLibrarySettings = Object.fromEntries(["libraryPaths", "disabledLibraries", "kparLibraryPaths"].map(key =>
      [key, vscode.workspace.getConfiguration("spec42").get(key)]));
    report.windowFocused = vscode.window.state.focused;
    report.resourcesBefore = { cpu: process.cpuUsage(), memory: process.memoryUsage() };

    let editSerial = 0;
    for (const state of ["closed", "visible", "hidden"] as const) {
      if (state === "visible") {
        records.poll();
        const after = records.events.length;
        await vscode.commands.executeCommand("spec42.diagram.open");
        await records.wait("initial visible view frame", hasFrame, after);
      } else if (state === "hidden") {
        records.poll();
        const after = records.events.length;
        await vscode.commands.executeCommand("workbench.action.closeSidebar");
        await vscode.commands.executeCommand("workbench.action.closeAuxiliaryBar");
        await records.wait("view hidden", event => event.event === "view:visibility" && event.visible === false, after);
      }
      for (const kind of ["single", "burst"] as const) {
        for (let iteration = 0; iteration < (state === "visible" ? iterations : inactiveIterations); iteration++) {
          assert.equal(missingLibraryContext(), false, "Missing library context invalidates a representative run");
          records.poll();
          const after = records.events.length;
          const focusedBefore = vscode.window.state.focused;
          const lossesBefore = focusLosses;
          let editStartedMs = 0;
          const edits = kind === "single" ? 1 : burstSize;
          for (let edit = 0; edit < edits; edit++) {
            // Only an unsaved trailing comment is changed. Capture the final burst edit's own clock.
            const change = new vscode.WorkspaceEdit();
            change.replace(document.uri, new vscode.Range(document.positionAt(original.length), document.positionAt(document.getText().length)), `\n// view-refresh-measurement ${++editSerial}\n`);
            editStartedMs = performance.now();
            assert.equal(await vscode.workspace.applyEdit(change), true);
            if (kind === "burst") await delay(10);
          }
          const sample: Sample = { state, kind, version: document.version, editStartedMs, edits };
          samples.push(sample);
          const uri = document.uri.toString();
          const publication = await records.wait("exact editor-version publication", event =>
            event.event === "backend:publication" && event.editorVersions !== null && typeof event.editorVersions === "object" &&
            Object.entries(event.editorVersions).some(([key, version]) => vscode.Uri.parse(key).toString() === uri && version === sample.version), after);
          assert.equal(typeof publication.modelDigest, "string");
          sample.modelDigest = publication.modelDigest as string;
          await records.wait(`${state} publication observed by view provider`, event =>
            event.event === "view:publicationObserved" && event.modelDigest === sample.modelDigest && event.viewState === state, after);
          if (state === "visible") {
            const frame = await records.wait("current visible view frame", event => hasFrame(event) && event.modelDigest === sample.modelDigest, after);
            assert.equal(typeof frame.extensionHostMs, "number");
            assert.ok((frame.extensionHostMs as number) >= editStartedMs, "Frame cannot predate its editor revision");
            sample.editToFrameAckMs = (frame.extensionHostMs as number) - editStartedMs;
            sample.frame = frame;
            sample.windowFocused = focusedBefore && vscode.window.state.focused && focusLosses === lossesBefore;
          } else {
            await delay(1000); // Observe beyond publication debounce; this delay is not a latency sample.
            records.poll();
            assert.equal(records.events.slice(after).some(event => event.modelDigest === sample.modelDigest &&
              ["view:generated", "view:drawComplete", "view:renderComplete"].includes(event.event)), false, `${state} view performed refresh work`);
          }
        }
      }
    }
    assert.equal(missingLibraryContext(), false);
    report.resourcesAfter = { cpu: process.cpuUsage(), memory: process.memoryUsage() };
    report.timingQualification = samples.filter(sample => sample.state === "visible").every(sample => sample.windowFocused)
      ? "focused-visible-frames" : "background-probe-only";
    report.status = "passed";
  } catch (error) {
    report.error = error instanceof Error ? error.stack : String(error);
    throw error;
  } finally {
    focusWatch.dispose();
    try {
      if (document && original !== undefined && document.isDirty) {
        await vscode.window.showTextDocument(document);
        await vscode.commands.executeCommand("workbench.action.files.revert");
        assert.equal(document.getText(), original, "Original editor text must be restored");
      }
      if (before) {
        report.inputsAfter = hashes(Object.keys(before));
        assert.deepEqual(report.inputsAfter, before, "Measurement must not change saved model or binary inputs");
      }
    } catch (error) {
      report.status = "failed";
      report.cleanupError = error instanceof Error ? error.stack : String(error);
    }
    records.poll();
    fs.mkdirSync(path.dirname(reportPath), { recursive: true });
    fs.writeFileSync(reportPath, JSON.stringify(report, null, 2) + "\n");
    assert.equal(report.status, "passed", `View measurement failed; see ${reportPath}`);
  }
}
