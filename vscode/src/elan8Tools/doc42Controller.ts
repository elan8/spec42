import { mkdtemp, mkdir, rm, stat } from "node:fs/promises";
import { basename, dirname, join } from "node:path";
import * as vscode from "vscode";
import {
  DOC42_INSTALL_GUIDE, DOC42_VERSION, Doc42Diagnostic, Doc42Document,
  containsPath, doc42DocumentLabel, doc42Package, findDoc42, installDoc42, parseDoc42Json, readBuiltPdfs,
  resolveDoc42LibraryPaths, runDoc42,
} from "./doc42Cli";
import { Doc42Preview, Doc42Result } from "./doc42Preview";
import { checkElan8Token, elan8TokensPage, enterElan8TokenLabel, saveElan8Token } from "./elan8Token";
import { Elan8ToolsView, ToolGroup, toolAction } from "./elan8ToolsView";
import { appendLibraryPathArgs } from "../lmTools/spec42CliArgs";

const command = {
  install: "spec42.doc42.install",
  guide: "spec42.doc42.installationGuide",
  login: "spec42.elan8Tools.enterApiToken",
  checkToken: "spec42.elan8Tools.checkToken",
  plans: "spec42.elan8Tools.plans",
  generate: "spec42.doc42.generate",
  generateAll: "spec42.doc42.generateAll",
  check: "spec42.doc42.check",
  addDocumentGuide: "spec42.doc42.addDocumentGuide",
  open: "spec42.doc42.openLastPdf",
  refresh: "spec42.elan8Tools.refresh",
};

interface CheckReport {
  documents: Doc42Document[];
  errors: number;
  warnings: number;
  diagnostics: Doc42Diagnostic[];
}

interface Project {
  root: string;
  libraries: string[];
}

const plansLabel = "View plans and trials";
const plansUrl = "https://www.elan8.com/pricing#plans";

export function registerElan8Tools(context: vscode.ExtensionContext): void {
  const doc42 = new Doc42Controller(context);
  const tool = new ToolGroup("Doc42", "file-pdf", true, signal => doc42.actions(signal));
  tool.id = "doc42";
  tool.description = "PDFs";
  tool.tooltip = new vscode.MarkdownString(
    "**Doc42**\n\nGenerate engineering PDFs from your SysML v2 model.\n\n"
    + "Listing and checking documents are free. PDF generation requires an Elan8 subscription that includes Doc42 or an active trial.",
  );
  const plans = toolAction("Plans and trials", command.plans, "link-external");
  plans.tooltip = "See Elan8 plans, included apps, and available trials. Spec42 modeling and validation are free.";
  const tokenCheck = toolAction("Check token…", command.checkToken, "checklist");
  const tokenCheckHint = "Check this workstation’s Elan8 API token online.";
  tokenCheck.tooltip = tokenCheckHint;
  const tokenEntry = toolAction("Enter token…", command.login, "edit");
  tokenEntry.tooltip = "Verify and save an Elan8 API token for this workstation.";
  const token = new ToolGroup("Elan8 API token", "key", false, async () => [
    tokenCheck,
    tokenEntry,
  ]);
  token.id = "elan8-api-token";
  token.tooltip = "Check or enter the API token used by Elan8 apps on this workstation.";
  const localDesktop = vscode.env.uiKind === vscode.UIKind.Desktop && !vscode.env.remoteName;
  const view = new Elan8ToolsView(localDesktop ? [plans, tool, token] : [plans, tool]);
  const tree = vscode.window.createTreeView("spec42Elan8Tools", { treeDataProvider: view });
  const watcher = vscode.workspace.createFileSystemWatcher("**/{*.sysml,*.kerml,.project.json,spec42.toml}");
  let timer: NodeJS.Timeout | undefined;
  const refreshSoon = () => {
    clearTimeout(timer);
    timer = setTimeout(() => view.refresh(), 400);
  };
  context.subscriptions.push(doc42, view, tree,
    watcher, watcher.onDidCreate(refreshSoon), watcher.onDidChange(refreshSoon), watcher.onDidDelete(refreshSoon),
    { dispose: () => clearTimeout(timer) },
    vscode.workspace.onDidGrantWorkspaceTrust(() => view.refresh()),
    vscode.workspace.onDidChangeWorkspaceFolders(() => view.refresh()),
    vscode.workspace.onDidChangeConfiguration(event => {
      if (event.affectsConfiguration("spec42.libraryPaths") || event.affectsConfiguration("spec42.doc42.libraryPaths")) {
        view.refresh();
      }
    }),
    vscode.commands.registerCommand(command.refresh, () => {
      doc42.refreshAccess();
      tokenCheck.description = undefined;
      tokenCheck.tooltip = tokenCheckHint;
      view.refresh();
    }),
    vscode.commands.registerCommand(command.guide, () => vscode.env.openExternal(vscode.Uri.parse(DOC42_INSTALL_GUIDE))),
    vscode.commands.registerCommand(command.plans, () => vscode.env.openExternal(vscode.Uri.parse(plansUrl))),
    vscode.commands.registerCommand(command.addDocumentGuide, () => vscode.env.openExternal(vscode.Uri.parse(addDocumentGuide))),
    vscode.commands.registerCommand(command.install, async () => { await doc42.install(); view.refresh(); }),
    vscode.commands.registerCommand(command.login, async () => {
      if (!localDesktop) return;
      await doc42.enterApiToken();
      tokenCheck.description = undefined;
      tokenCheck.tooltip = tokenCheckHint;
      view.refresh();
    }),
    vscode.commands.registerCommand(command.checkToken, async () => {
      if (!localDesktop) return;
      const status = await doc42.checkToken();
      if (!status) return;
      tokenCheck.description = status;
      tokenCheck.tooltip = `${tokenCheckHint} Last check · ${status}`;
      doc42.refreshAccess();
      view.refresh();
    }),
    vscode.commands.registerCommand(command.check, (root?: string, all = false) =>
      doc42.chooseAndRun("check", root, all ? "all" : undefined)),
    vscode.commands.registerCommand(command.generate, async (root?: string, name?: string) => {
      await doc42.chooseAndRun("build", root, name === undefined ? undefined : { name });
      view.refresh();
    }),
    vscode.commands.registerCommand(command.generateAll, async (root?: string) => {
      await doc42.chooseAndRun("build", root, "all");
      view.refresh();
    }),
    vscode.commands.registerCommand(command.open, (root?: string) => doc42.openLast(root)),
  );
}

type Selection = { name: string } | "all";
const addDocumentGuide = "https://github.com/elan8/doc42-release/blob/main/docs/getting-started.md#create-your-own-document";

class Doc42Controller implements vscode.Disposable {
  private readonly lifetime = new AbortController();
  private readonly preview: Doc42Preview;
  private readonly storage: string;
  private readonly outputRoot: string;
  private access?: { executable: string; expires: number; usable: Promise<boolean> };
  private busy = false;

  constructor(private readonly context: vscode.ExtensionContext) {
    this.storage = join(context.globalStorageUri.fsPath, "tools", "doc42");
    this.outputRoot = join((context.storageUri ?? context.globalStorageUri).fsPath, "doc42-results");
    this.preview = new Doc42Preview(context.extensionUri);
  }

  async actions(signal: AbortSignal): Promise<vscode.TreeItem[]> {
    const guide = toolAction("Installation guide", command.guide, "link-external");
    if (!this.supported) {
      const unavailable = new vscode.TreeItem("Requires local Windows or Linux x64");
      unavailable.iconPath = new vscode.ThemeIcon("info");
      return [unavailable, guide];
    }
    if (!this.trusted) {
      const restricted = new vscode.TreeItem("Trust this workspace to run Doc42");
      restricted.iconPath = new vscode.ThemeIcon("shield");
      return [restricted, guide];
    }
    const install = await this.find(signal);
    if (!install.compatible) {
      const last = this.folders.some(folder => this.lastResults(folder.uri.fsPath).length)
        ? [toolAction("Open last build…", command.open, "file-pdf")] : [];
      const button = toolAction("Install Doc42", command.install, "cloud-download");
      button.description = install?.incompatible ? `Requires ${DOC42_VERSION}; found ${install.incompatible.version}` : undefined;
      return [button, ...last, guide];
    }
    const executable = install.compatible.path;
    const access = this.accessStatus(executable);
    const folders = this.folders;
    const active = vscode.window.activeTextEditor?.document.uri;
    const activeFolder = active && vscode.workspace.getWorkspaceFolder(active);
    let projects: vscode.TreeItem[];
    if (folders.length === 0) {
      projects = [new vscode.TreeItem("Open a model project folder to find documents")];
    } else if (folders.length === 1) {
      projects = await this.projectActions(folders[0], executable, signal);
    } else {
      projects = folders.map(folder => {
        const item = new ToolGroup(folder.name, "folder", folder === activeFolder,
          signal => this.projectActions(folder, executable, signal));
        item.id = `doc42-project:${folder.uri.toString()}`;
        item.tooltip = folder.uri.fsPath;
        return item;
      });
    }
    return [...projects, await access, guide];
  }

  refreshAccess(): void {
    this.access = undefined;
  }

  async checkToken(): Promise<string | undefined> {
    const status = await this.progress("Checking Elan8 API token online", signal => checkElan8Token(signal));
    if (!status) return;
    const result = {
      valid: { label: "Valid", message: "Elan8 accepted this workstation’s API token." },
      noAccess: { label: "Valid · no license", message: "The API token is valid. Elan8 could not issue a new license for its account." },
      missing: { label: "Missing", message: "No Elan8 API token was found on this workstation." },
      invalid: { label: "Invalid", message: "The Elan8 API token is malformed, expired, or revoked." },
      unverified: { label: "Unverified", message: "The Elan8 API token could not be verified. Try again when Elan8 is reachable." },
    }[status];
    if (status === "missing" || status === "invalid") {
      void vscode.window.showInformationMessage(result.message, "Open Elan8 API tokens").then(action => {
        if (action) return vscode.env.openExternal(vscode.Uri.parse(elan8TokensPage));
      });
    } else {
      void vscode.window.showInformationMessage(result.message);
    }
    return result.label;
  }

  private async accessStatus(executable: string): Promise<vscode.TreeItem> {
    // Model-file edits can refresh the tree often. Doc42 owns the signed license cache.
    if (this.access?.executable !== executable || Date.now() >= this.access.expires) {
      const usable = runDoc42(executable, ["license"], { signal: this.lifetime.signal, timeout: 5000 })
        .then(result => result.exitCode === 0, () => false);
      this.access = { executable, expires: Date.now() + 60_000, usable };
    }
    if (await this.access.usable) {
      const item = new vscode.TreeItem("Doc42 access verified");
      item.iconPath = new vscode.ThemeIcon("pass");
      return item;
    }
    const item = toolAction(enterElan8TokenLabel, command.login, "key");
    item.tooltip = "Doc42 access has not been verified. Enter the token for an account with Doc42 access.";
    return item;
  }

  private async projectActions(folder: vscode.WorkspaceFolder, executable: string, signal: AbortSignal): Promise<vscode.TreeItem[]> {
    const project = this.project(folder);
    const { root } = project;
    const actions: vscode.TreeItem[] = [];
    const checkProject = toolAction("Check project", command.check, "checklist", root, true);
    const addDocument = toolAction("Add a Doc42 document…", command.addDocumentGuide, "link-external");
    if (this.lastResults(root).length) {
      actions.push(toolAction("Open last build…", command.open, "file-pdf", root));
    }
    let documents: Doc42Document[];
    let diagnostics: CheckReport | undefined;
    try {
      documents = await this.list(executable, project, signal);
      if (documents.length === 0) {
        diagnostics = await this.readCheckReport(executable, root, this.modelArgs(project), signal);
        documents = diagnostics.documents;
      }
    } catch (error) {
      signal.throwIfAborted();
      const status = new vscode.TreeItem("Could not scan for Doc42 documents");
      status.iconPath = new vscode.ThemeIcon("warning");
      status.tooltip = error instanceof Error ? error.message : String(error);
      return [status, checkProject, addDocument, ...actions];
    }
    if (!documents.length) {
      const status = new vscode.TreeItem(
        diagnostics?.errors ? "Doc42 discovery found model errors" : "No Doc42 documents found",
      );
      if (diagnostics && (diagnostics.errors > 0 || diagnostics.warnings > 0)) {
        status.description = `${diagnostics.errors} errors, ${diagnostics.warnings} warnings`;
        status.iconPath = new vscode.ThemeIcon("warning");
        status.tooltip = "Doc42 found model diagnostics during discovery. Use Check project to review them.";
        return [status, checkProject, addDocument, ...actions];
      }
      return [status, addDocument, checkProject, ...actions];
    }
    const entries = documents.map(document => {
      const label = doc42DocumentLabel(document);
      const item = toolAction(label, command.generate, "play", root, document.qualified_name);
      item.id = `doc42-document:${JSON.stringify([root, document.qualified_name])}`;
      const details = this.documentDetails(document);
      item.description = details;
      item.tooltip = [`Generate PDF for ${label}`, details, document.qualified_name].filter(Boolean).join("\n");
      return item;
    });
    return [
      toolAction("Generate All", command.generateAll, "play", root),
      ...entries, toolAction("Check document…", command.check, "checklist", root), ...actions,
    ];
  }

  async install(): Promise<void> {
    if (!this.supported) {
      await vscode.env.openExternal(vscode.Uri.parse(DOC42_INSTALL_GUIDE));
      return;
    }
    if (!this.requireTrust()) return;
    await this.exclusive(async () => {
      await this.progress("Installing Doc42", async (signal, progress) => {
        if ((await this.find(signal)).compatible) return;
        progress.report({ message: `Downloading Doc42 ${DOC42_VERSION}…` });
        await installDoc42(this.storage, signal, message => progress.report({ message }));
        void vscode.window.showInformationMessage(`Doc42 ${DOC42_VERSION} is ready to use.`);
      });
    });
  }

  async enterApiToken(): Promise<void> {
    await this.exclusive(async () => {
      await this.saveApiToken();
    });
  }

  async chooseAndRun(mode: "build" | "check", root?: string, selection?: Selection): Promise<void> {
    if (!this.requireTrust() || !this.supported) return;
    await this.exclusive(async () => {
      const project = await this.selectProject(root);
      if (!project || !await this.saveProjectEdits(project)) return;
      const installation = await this.requireInstallation(this.lifetime.signal);
      const selected = selection ?? await this.chooseDocument(installation.path, project, mode);
      if (!selected || (!selection && !await this.saveProjectEdits(project))) return;
      const args = this.modelArgs(project);
      const checkArgs = selected === "all" ? args : [...args, "--doc", selected.name];
      let report = await this.check(installation.path, project.root, checkArgs);
      if (!report) return;
      if (report.errors || mode === "check") {
        await this.showDiagnostics(report);
        return;
      }
      if (!report.documents.length) {
        await this.showAddDocumentGuide();
        return;
      }

      const licensed = await this.progress("Checking Doc42 access", async signal =>
        runDoc42(installation.path, ["license"], { signal }),
      );
      if (!licensed) return;
      this.access = { executable: installation.path, expires: Date.now() + 60_000, usable: Promise.resolve(licensed.exitCode === 0) };
      if (licensed.exitCode !== 0) {
        const action = await vscode.window.showErrorMessage(
          licensed.stderr.trim() || "Doc42 needs an active Elan8 license to generate PDFs.",
          enterElan8TokenLabel,
          plansLabel,
        );
        if (action === plansLabel) {
          await vscode.commands.executeCommand(command.plans);
          return;
        }
        if (action !== enterElan8TokenLabel || !await this.saveApiToken()) return;
        if (!await this.saveProjectEdits(project)) return;
        report = await this.check(installation.path, project.root, checkArgs);
        if (!report) return;
        if (report.errors) {
          await this.showDiagnostics(report);
          return;
        }
      }

      const buildArgs = selected === "all" ? [...args, "--all"] : checkArgs;
      const names = selected === "all" ? report.documents.map(document => document.qualified_name) : [selected.name];
      if (!names.length) {
        await this.showAddDocumentGuide();
        return;
      }
      const checked = report;
      const label = selected === "all" ? "Generating all Doc42 PDFs" : "Generating Doc42 PDF";
      const results = await this.progress(label, signal =>
        this.generate(installation.path, project, buildArgs, names, checked, signal),
      );
      if (results) await this.openResults(results);
    });
  }

  async openLast(root?: string): Promise<void> {
    if (!this.requireTrust()) return;
    await this.exclusive(async () => {
      const project = await this.selectProject(root, true);
      if (!project) return;
      await this.openResults(this.lastResults(project.root));
    });
  }

  private async openResults(results: Doc42Result[]): Promise<void> {
    const result = results.length === 1 ? results[0] : (await vscode.window.showQuickPick(
      results.map(result => ({ label: result.title, description: basename(result.file), result })),
      { title: "Open a generated Doc42 PDF" },
    ))?.result;
    if (!result) return;
    if (!await stat(result.file).then(file => file.isFile(), () => false)) {
      throw new Error("The generated PDF is no longer available. Generate it again.");
    }
    this.preview.open(result);
  }

  dispose(): void {
    this.lifetime.abort();
    this.preview.dispose();
  }

  private get supported(): boolean {
    return Boolean(doc42Package) && vscode.env.uiKind === vscode.UIKind.Desktop && !vscode.env.remoteName;
  }

  private get trusted(): boolean {
    return vscode.workspace.isTrusted;
  }

  private get folders(): vscode.WorkspaceFolder[] {
    return (vscode.workspace.workspaceFolders ?? []).filter(folder => folder.uri.scheme === "file");
  }

  private requireTrust(): boolean {
    if (this.trusted) return true;
    void vscode.window.showWarningMessage("Trust this workspace to run Doc42.");
    return false;
  }

  private find(signal: AbortSignal) {
    return findDoc42(this.storage, this.folders.map(folder => folder.uri.fsPath), signal);
  }

  private async requireInstallation(signal: AbortSignal) {
    const { compatible } = await this.find(signal);
    if (!compatible) throw new Error(`Install Doc42 ${DOC42_VERSION} from ModelApps first.`);
    return compatible;
  }

  private async saveApiToken(): Promise<boolean> {
    const token = (await vscode.window.showInputBox({
      title: "Enter Elan8 API token",
      prompt: "Paste a valid Elan8 API token",
      password: true,
      ignoreFocusOut: true,
      validateInput: value => value.trim() ? undefined : "Enter your Elan8 API token.",
    }))?.trim();
    if (!token) return false;
    let status;
    try {
      status = await this.progress("Verifying Elan8 API token", signal => saveElan8Token(token, signal));
    } catch {
      void vscode.window.showErrorMessage("Could not write to the shared Elan8 API token store.");
      return false;
    }
    if (!status) return false;
    if (status === "invalid" || status === "unverified" || status === "missing") {
      void vscode.window.showErrorMessage(status === "invalid"
        ? "Enter an Elan8 API token that is valid and has not expired."
        : "Could not verify the Elan8 API token. Try again when Elan8 is reachable.");
      return false;
    }
    this.refreshAccess();
    const message = process.env.ELAN8_TOKEN?.trim()
      ? "Elan8 API token saved. Tools in this session use the ELAN8_TOKEN environment variable first."
      : status === "noAccess"
        ? "Elan8 API token saved. Elan8 could not issue a new license for its account."
        : "Elan8 API token saved.";
    void vscode.window.showInformationMessage(message);
    return true;
  }

  private async selectProject(root?: string, onlyWithResults = false): Promise<Project | undefined> {
    const folders = this.folders.filter(folder => !onlyWithResults || this.lastResults(folder.uri.fsPath).length);
    if (root !== undefined) {
      const folder = folders.find(folder => folder.uri.fsPath === root);
      if (!folder) throw new Error("The selected project is no longer available.");
      return this.project(folder);
    }
    if (!folders.length) {
      void vscode.window.showInformationMessage(onlyWithResults
        ? "No generated PDF is available." : "Open the model project folder in VS Code to use Doc42.");
      return;
    }
    const active = vscode.window.activeTextEditor?.document.uri;
    const activeFolder = active && vscode.workspace.getWorkspaceFolder(active);
    const folder = folders.find(item => item.uri.toString() === activeFolder?.uri.toString())
      ?? (folders.length === 1 ? folders[0] : (await vscode.window.showQuickPick(
        folders.map(folder => ({ label: folder.name, description: folder.uri.fsPath, folder })),
        { title: "Choose a Doc42 project folder" },
      ))?.folder);
    return folder && this.project(folder);
  }

  private project(folder: vscode.WorkspaceFolder): Project {
    const root = folder.uri.fsPath;
    const configuration = vscode.workspace.getConfiguration("spec42", folder.uri);
    const libraries = resolveDoc42LibraryPaths(
      root, configuration.get<string[]>("libraryPaths", []), configuration.get<string[] | null>("doc42.libraryPaths", null),
    );
    return { root, libraries };
  }

  private modelArgs(project: Project): string[] {
    return appendLibraryPathArgs([project.root, "--workspace-root", project.root, "--json"], project.libraries);
  }

  private async list(executable: string, project: Project, signal: AbortSignal): Promise<Doc42Document[]> {
    return parseDoc42Json<Doc42Document[]>(await runDoc42(executable, ["list", ...this.modelArgs(project)], {
      cwd: project.root, signal,
    }));
  }

  private async chooseDocument(executable: string, project: Project, mode: "build" | "check"): Promise<Selection | undefined> {
    const documents = await this.progress("Finding Doc42 documents", signal => this.list(executable, project, signal));
    if (!documents) return;
    if (!documents.length) {
      if (mode === "check") {
        const report = await this.check(executable, project.root, this.modelArgs(project));
        if (!report) return;
        if (report.diagnostics.length) {
          await this.showDiagnostics(report);
          return;
        }
      }
      await this.showAddDocumentGuide();
      return;
    }
    const items = documents.map(document => ({
      label: doc42DocumentLabel(document),
      description: this.documentDetails(document),
      detail: document.qualified_name,
      selection: { name: document.qualified_name } as Selection,
    }));
    if (mode === "build") {
      items.unshift({ label: "Generate All", description: "", detail: "", selection: "all" });
    }
    return (await vscode.window.showQuickPick(items, {
      title: mode === "build" ? "Generate Doc42 PDFs" : "Check a Doc42 document",
      matchOnDetail: true,
    }))?.selection;
  }

  private documentDetails(document: Doc42Document): string {
    return [document.number, document.revision && `rev ${document.revision}`].filter(Boolean).join(" · ");
  }

  private async showAddDocumentGuide(): Promise<void> {
    const action = await vscode.window.showInformationMessage("No Doc42 documents found.", "Add a Doc42 document…");
    if (action) await vscode.commands.executeCommand(command.addDocumentGuide);
  }

  private async saveProjectEdits(project: Project): Promise<boolean> {
    const dirty = vscode.workspace.textDocuments.filter(document => document.isDirty && document.uri.scheme === "file"
      && [project.root, ...project.libraries].some(root => containsPath(root, document.uri.fsPath)));
    if (!dirty.length) return true;
    const action = await vscode.window.showWarningMessage(
      "Doc42 uses saved files. Save changes in the selected project and its configured libraries?",
      { modal: true }, "Save and Continue",
    );
    if (!action) return false;
    for (const document of dirty) {
      if (!await document.save()) throw new Error(`Could not save ${basename(document.uri.fsPath)}.`);
    }
    return true;
  }

  private check(executable: string, cwd: string, args: string[]) {
    return this.progress("Checking Doc42 inputs", signal => this.readCheckReport(executable, cwd, args, signal));
  }

  private async readCheckReport(executable: string, cwd: string, args: string[], signal: AbortSignal): Promise<CheckReport> {
    return parseDoc42Json<CheckReport>(await runDoc42(executable, ["check", ...args], { cwd, signal }), [0, 1]);
  }

  private async showDiagnostics(report: CheckReport): Promise<void> {
    if (!report.diagnostics.length) {
      void vscode.window.showInformationMessage("Doc42 check passed.");
      return;
    }
    const result = await vscode.window.showQuickPick(report.diagnostics.map(diagnostic => ({
      label: diagnostic.message,
      description: `${diagnostic.severity} · ${diagnostic.code}`,
      detail: diagnostic.location ? `${vscode.Uri.parse(diagnostic.location.uri).fsPath}:${diagnostic.location.line + 1}` : undefined,
      diagnostic,
    })), { title: `Doc42 check · ${report.errors} errors, ${report.warnings} warnings`, matchOnDescription: true, matchOnDetail: true });
    const location = result?.diagnostic.location;
    if (!location) return;
    const uri = vscode.Uri.parse(location.uri);
    if (uri.scheme === "file") {
      const point = new vscode.Position(location.line, location.column);
      await vscode.window.showTextDocument(uri, { selection: new vscode.Range(point, point) });
    }
  }

  private async generate(
    executable: string, project: Project, args: string[], names: string[], report: CheckReport, signal: AbortSignal,
  ): Promise<Doc42Result[]> {
    await mkdir(this.outputRoot, { recursive: true });
    const run = await mkdtemp(join(this.outputRoot, "build-"));
    let published = false;
    try {
      const output = await runDoc42(executable, ["build", ...args, "--out", run], { cwd: project.root, signal });
      const built = await readBuiltPdfs(output, project.root, run, names);
      signal.throwIfAborted();
      const results = built.documents.map(document => ({
        file: document.file, title: doc42DocumentLabel(document), project: project.root,
      }));
      const previous = this.lastResults(project.root);
      await this.context.workspaceState.update(this.resultKey(project.root), results);
      published = true;
      // Both older single-PDF results and the current batch format own one build directory.
      if (previous.length) {
        this.preview.invalidate(previous.map(result => result.file));
        await rm(dirname(previous[0].file), { recursive: true, force: true }).catch(() => {});
      }
      if (built.warnings > 0) {
        void vscode.window.showWarningMessage(
          `Doc42 generated ${results.length} PDF(s) with ${built.warnings} warning(s).`, "Show check results",
        ).then(action => {
          if (action) void this.exclusive(() => this.showDiagnostics(report));
        });
      }
      return results;
    } finally {
      if (!published) await rm(run, { recursive: true, force: true }).catch(() => {});
    }
  }

  private resultKey(project: string): string {
    return `doc42.lastPdf:${project}`;
  }

  private lastResults(project: string): Doc42Result[] {
    const saved = this.context.workspaceState.get<Doc42Result | Doc42Result[]>(this.resultKey(project));
    const results = Array.isArray(saved) ? saved : saved ? [saved] : [];
    return results.filter(result => result.project === project && dirname(dirname(result.file)) === this.outputRoot
      && basename(dirname(result.file)).startsWith("build-"));
  }

  private async exclusive(run: () => Promise<void>): Promise<void> {
    if (this.busy) {
      void vscode.window.showInformationMessage("Doc42 is already working.");
      return;
    }
    this.busy = true;
    try {
      await run();
    } catch (error) {
      if (!this.lifetime.signal.aborted) {
        void vscode.window.showErrorMessage(`Doc42: ${error instanceof Error ? error.message : String(error)}`);
      }
    } finally {
      this.busy = false;
    }
  }

  private progress<T>(
    title: string, run: (signal: AbortSignal, progress: vscode.Progress<{ message?: string }>) => Promise<T>,
  ): Thenable<T | undefined> {
    return vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title, cancellable: true },
      async (progress, token) => {
        const operation = new AbortController();
        const cancellation = token.onCancellationRequested(() => operation.abort());
        if (token.isCancellationRequested) operation.abort();
        const signal = AbortSignal.any([operation.signal, this.lifetime.signal]);
        try {
          signal.throwIfAborted();
          return await run(signal, progress);
        } catch (error) {
          if (!signal.aborted) throw error;
          return undefined;
        } finally {
          cancellation.dispose();
        }
      },
    );
  }
}
