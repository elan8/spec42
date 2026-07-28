import * as vscode from "vscode";
import type {
  LspModelProvider,
  SysMLLibrarySearchResult,
  SysMLLibraryStatusResult,
} from "../providers/lspModelProvider";
import { KPAR_LIBRARIES_DEFAULTS } from "../generated/kparLibrariesDefaults";
import {
  buildLibraryDashboardStatus,
  flattenLibrarySearchResults,
  KparLibraryHeading,
  LibraryDashboardStatus,
  SysandStatusViewModel,
  summarizeLibrarySearch,
} from "./libraryStatusViewModel";

type StdlibHeading = {
  pinnedVersion: string;
  format: string;
};

type OpenRangeMessage = {
  uri: string;
  range: {
    start: { line: number; character: number };
    end: { line: number; character: number };
  };
};

type LibraryWebviewOptions = {
  getStdlibHeading: () => StdlibHeading;
  getKparHeadings: () => KparLibraryHeading[];
  getConfiguredLibraryPaths: () => string[];
  getMissingLibraryPaths: () => string[];
  getSysandStatus: () => Promise<SysandStatusViewModel>;
};

export class LibraryWebviewViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;
  private pendingQuery: string | undefined;

  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly lspModelProvider: LspModelProvider,
    private readonly options: LibraryWebviewOptions
  ) {}

  resolveWebviewView(webviewView: vscode.WebviewView): void | Thenable<void> {
    this.view = webviewView;
    const webview = webviewView.webview;
    webview.options = {
      enableScripts: true,
    };
    webview.html = this.getHtml(webview);

    webview.onDidReceiveMessage(async (message) => {
      if (message?.type === "initLoad") {
        await this.postDashboard();
        if (this.pendingQuery) {
          this.post({ type: "setQuery", payload: this.pendingQuery });
          await this.search(this.pendingQuery, "results");
          this.pendingQuery = undefined;
        }
        return;
      }

      if (message?.type === "search") {
        await this.search(String(message.query ?? "").trim(), "results");
        return;
      }

      if (message?.type === "browseAll") {
        await this.search("", "browse");
        return;
      }

      if (message?.type === "openResult") {
        await this.openResult(message.payload as OpenRangeMessage);
        return;
      }

      if (message?.type === "copyText") {
        const text = String(message.text ?? "");
        if (text) {
          await vscode.env.clipboard.writeText(text);
          vscode.window.setStatusBarMessage("Copied library text", 1600);
        }
        return;
      }

      if (message?.type === "manageCustomLibraries") {
        await vscode.commands.executeCommand("sysml.library.managePaths");
        return;
      }

      if (message?.type === "showStdlibInfo") {
        await vscode.commands.executeCommand("sysml.library.showStdLibStatus");
        return;
      }

      if (message?.type === "showKparLibraryInfo") {
        await vscode.commands.executeCommand(
          "sysml.library.showKparLibraryStatus",
          String(message.id ?? "")
        );
        return;
      }

      if (message?.type === "showDomainLibrariesInfo") {
        await vscode.commands.executeCommand(
          "sysml.library.showKparLibraryStatus",
          "domain"
        );
        return;
      }

      if (message?.type === "showSysandStatus") {
        await vscode.commands.executeCommand("sysml.sysand.showStatus");
        return;
      }

      if (message?.type === "refreshSysandDependencies") {
        await vscode.commands.executeCommand("sysml.sysand.refreshDependencies");
        return;
      }

      if (message?.type === "copySysandInstall") {
        await vscode.env.clipboard.writeText("cargo install sysand");
        vscode.window.setStatusBarMessage("Copied Sysand install command", 1600);
        return;
      }

      if (message?.type === "openSysandDocs") {
        await vscode.env.openExternal(vscode.Uri.parse("https://github.com/sensmetry/sysand"));
        return;
      }

      if (message?.type === "showOutput") {
        await vscode.commands.executeCommand("sysml.showOutput");
        return;
      }
    });
  }

  refresh(): void {
    void this.postDashboard();
  }

  async searchAndReveal(query: string): Promise<void> {
    await vscode.commands.executeCommand("workbench.view.extension.spec42");
    await vscode.commands.executeCommand("spec42Library.focus");
    const trimmed = query.trim();
    if (!this.view) {
      this.pendingQuery = trimmed;
      return;
    }
    this.post({ type: "setQuery", payload: trimmed });
    await this.search(trimmed, "results");
  }

  private async postDashboard(): Promise<void> {
    this.post({ type: "dashboardLoading" });
    try {
      const [summaryResult, sysand, libraryStatus] = await Promise.all([
        this.lspModelProvider.searchLibraries("", 50),
        this.options.getSysandStatus(),
        this.lspModelProvider.getLibraryStatus(),
      ]);
      const status = this.dashboardStatus(summaryResult, sysand, libraryStatus);
      this.post({ type: "dashboard", payload: status });
    } catch (error) {
      this.post({
        type: "error",
        payload: error instanceof Error ? error.message : String(error),
      });
    }
  }

  private dashboardStatus(
    result: SysMLLibrarySearchResult,
    sysand: SysandStatusViewModel,
    libraryStatus: SysMLLibraryStatusResult
  ): LibraryDashboardStatus {
    const stdlibHeading = this.options.getStdlibHeading();
    const kparHeadings =
      this.options.getKparHeadings().length > 0
        ? this.options.getKparHeadings()
        : KPAR_LIBRARIES_DEFAULTS.map((library) => ({
            id: library.id,
            displayName: library.displayName,
            pinnedVersion: library.version,
            format: library.format,
          }));

    return buildLibraryDashboardStatus({
      pinnedVersion:
        libraryStatus.stdlib.pinnedVersion || stdlibHeading.pinnedVersion,
      format: libraryStatus.stdlib.format || stdlibHeading.format,
      kparHeadings,
      kparStatuses: libraryStatus.kparLibraries.map((library) => ({
        id: library.id,
        displayName: library.displayName,
        resolvedPath: library.resolvedPath,
        sourceKind: library.sourceKind,
        pinnedVersion: library.pinnedVersion,
        installedVersion: library.installedVersion,
        isInstalled: library.isInstalled,
        versionMatches: library.versionMatches,
      })),
      configuredPaths: this.options.getConfiguredLibraryPaths(),
      missingPaths: this.options.getMissingLibraryPaths(),
      summary: summarizeLibrarySearch(result),
      sysand,
    });
  }

  private async search(
    query: string,
    responseType: "results" | "browse"
  ): Promise<void> {
    this.post({ type: "searching", payload: query });
    try {
      const result = await this.lspModelProvider.searchLibraries(
        query,
        query ? 100 : 500
      );
      const rows = flattenLibrarySearchResults(result, query);
      this.post({
        type: responseType,
        payload: {
          query,
          rows,
          tree: result,
          total: result.total,
          symbolTotal: result.symbolTotal,
        },
      });
    } catch (error) {
      this.post({
        type: "error",
        payload: error instanceof Error ? error.message : String(error),
      });
    }
  }

  private async openResult(payload: OpenRangeMessage): Promise<void> {
    if (!payload?.uri) {
      return;
    }
    const doc = await vscode.workspace.openTextDocument(vscode.Uri.parse(payload.uri));
    const editor = await vscode.window.showTextDocument(doc, { preview: true });
    if (payload.range?.start) {
      const position = new vscode.Position(
        payload.range.start.line,
        payload.range.start.character
      );
      editor.selection = new vscode.Selection(position, position);
      editor.revealRange(new vscode.Range(position, position));
    }
  }

  private post(message: unknown): void {
    void this.view?.webview.postMessage(message);
  }

  private getHtml(webview: vscode.Webview): string {
    const nonce = String(Date.now());
    const csp = [
      "default-src 'none'",
      `style-src ${webview.cspSource} 'unsafe-inline'`,
      `script-src 'nonce-${nonce}'`,
    ].join("; ");

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="Content-Security-Policy" content="${csp}" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Spec42 Libraries</title>
  <style>
    :root {
      color-scheme: light dark;
      --gap: 8px;
      --radius: 6px;
    }
    body {
      margin: 0;
      padding: 8px;
      font-family: var(--vscode-font-family);
      color: var(--vscode-foreground);
      background: var(--vscode-sideBar-background);
    }
    .toolbar {
      display: flex;
      gap: 6px;
      margin-bottom: 8px;
    }
    input[type="text"] {
      flex: 1;
      min-width: 0;
      padding: 4px 8px;
      border: 1px solid var(--vscode-input-border, transparent);
      background: var(--vscode-input-background);
      color: var(--vscode-input-foreground);
      border-radius: var(--radius);
    }
    button.icon-btn {
      border: none;
      background: transparent;
      color: var(--vscode-icon-foreground);
      cursor: pointer;
      padding: 2px 4px;
      border-radius: 4px;
    }
    button.icon-btn:hover {
      background: var(--vscode-toolbar-hoverBackground);
    }
    #dashboard { display: grid; gap: 6px; margin-bottom: 10px; }
    .section {
      border: 1px solid var(--vscode-widget-border, rgba(127,127,127,.35));
      border-radius: var(--radius);
      padding: 6px 8px;
      background: var(--vscode-editor-background);
    }
    .section-head {
      display: flex;
      align-items: center;
      gap: 6px;
    }
    .title {
      font-weight: 600;
      flex: 1;
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .section-trail {
      display: flex;
      align-items: center;
      gap: 4px;
      margin-left: auto;
      flex-shrink: 0;
    }
    .pill {
      font-size: 11px;
      padding: 1px 6px;
      border-radius: 999px;
      border: 1px solid transparent;
      white-space: nowrap;
      text-align: right;
    }
    .pill.ok { color: var(--vscode-testing-iconPassed); border-color: currentColor; }
    .pill.warning { color: var(--vscode-editorWarning-foreground); border-color: currentColor; }
    .pill.info { color: var(--vscode-descriptionForeground); border-color: currentColor; }
    .muted { color: var(--vscode-descriptionForeground); font-size: 12px; }
    .warning-list { margin: 6px 0 0; padding-left: 18px; color: var(--vscode-editorWarning-foreground); }
    .actions { display: flex; gap: 4px; margin-top: 6px; flex-wrap: wrap; }
    .result {
      border: 1px solid var(--vscode-widget-border, rgba(127,127,127,.35));
      border-radius: var(--radius);
      padding: 6px 8px;
      margin-bottom: 6px;
      background: var(--vscode-editor-background);
    }
    .result-title { display: flex; gap: 6px; align-items: center; }
    .meta { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 4px; font-size: 12px; color: var(--vscode-descriptionForeground); }
    details.tree-package { margin-left: 8px; }
  </style>
</head>
<body>
  <div id="dashboard"></div>
  <div class="toolbar">
    <button id="btnBrowseAll" class="icon-btn" title="Browse all indexed library symbols">Browse</button>
    <input id="query" type="text" placeholder="Search types, units, packages..." />
  </div>
  <div id="state" class="muted">Type to search or browse all libraries.</div>
  <div id="results"></div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const dashboard = document.getElementById('dashboard');
    const query = document.getElementById('query');
    const state = document.getElementById('state');
    const results = document.getElementById('results');
    const btnBrowseAll = document.getElementById('btnBrowseAll');
    let timer = null;
    let renderedRows = [];

    function el(tag, className, text) {
      const node = document.createElement(tag);
      if (className) node.className = className;
      if (text !== undefined) node.textContent = text;
      return node;
    }

    function button(title, icon, type, extra = {}) {
      const btn = el('button', 'icon-btn');
      btn.title = title;
      const span = el('span', 'codicon codicon-' + icon);
      btn.appendChild(span);
      btn.addEventListener('click', () => vscode.postMessage({ type, ...extra }));
      return btn;
    }

    function countText(packages, symbols) {
      if (packages === undefined && symbols === undefined) return '';
      return String(packages || 0) + ' packages / ' + String(symbols || 0) + ' symbols';
    }

    function versionLabel(library) {
      const pinned = library?.pinnedVersion || 'unknown';
      const installed = library?.installedVersion;
      if (!library?.available) {
        return pinned + ' missing';
      }
      if (installed && installed !== pinned) {
        return pinned + ' (have ' + installed + ')';
      }
      return installed || pinned;
    }

    function sectionTrail(...children) {
      const trail = el('div', 'section-trail');
      children.filter(Boolean).forEach(child => trail.appendChild(child));
      return trail;
    }

    function renderDashboard(status) {
      const nodes = [];

      const std = el('div', 'section');
      const stdHead = el('div', 'section-head');
      stdHead.title = 'Release ' + (status?.stdlib?.pinnedVersion || 'unknown') + ' / ' + countText(status?.stdlib?.packageCount, status?.stdlib?.symbolCount) + ' / server-bundled';
      stdHead.appendChild(el('div', 'title', 'Standard Library'));
      stdHead.appendChild(sectionTrail(el('span', 'pill ok', status?.stdlib?.pinnedVersion || 'bundled')));
      std.appendChild(stdHead);
      nodes.push(std);

      const kparLibraries = Array.isArray(status?.kparLibraries) && status.kparLibraries.length
        ? status.kparLibraries
        : (status?.domain ? [status.domain] : []);
      kparLibraries.forEach((library) => {
        const section = el('div', 'section');
        const head = el('div', 'section-head');
        const ok = !!library.available && !!library.versionMatches;
        const pillClass = ok ? 'ok' : 'warning';
        const counts = countText(library.packageCount, library.symbolCount);
        head.title = [
          library.displayName || library.id,
          'pinned ' + (library.pinnedVersion || 'unknown'),
          library.installedVersion ? 'installed ' + library.installedVersion : '',
          library.sourceKind || '',
          counts,
          library.resolvedPath || ''
        ].filter(Boolean).join(' / ');
        head.appendChild(el('div', 'title', library.displayName || library.id || 'Library'));
        head.appendChild(sectionTrail(
          el('span', 'pill ' + pillClass, versionLabel(library)),
          button('Library details', 'info', 'showKparLibraryInfo', { id: library.id })
        ));
        section.appendChild(head);
        nodes.push(section);
      });

      const custom = status?.custom || {};
      const customSection = el('div', 'section');
      const customHead = el('div', 'section-head');
      const missing = Array.isArray(custom.missingPaths) ? custom.missingPaths : [];
      customHead.title = countText(custom.packageCount, custom.symbolCount) + (missing.length ? ' / ' + String(missing.length) + ' missing' : '');
      customHead.appendChild(el('div', 'title', 'Custom Libraries'));
      customHead.appendChild(sectionTrail(
        el('span', 'pill ' + (missing.length ? 'warning' : 'info'), String((custom.configuredPaths || []).length) + ' path(s)'),
        button('Manage custom library paths', 'settings-gear', 'manageCustomLibraries')
      ));
      customSection.appendChild(customHead);
      if (missing.length) {
        const list = el('ul', 'warning-list');
        missing.forEach(path => list.appendChild(el('li', '', path)));
        customSection.appendChild(list);
      }
      nodes.push(customSection);

      const sysand = status?.sysand || {};
      if (sysand.installed || sysand.manifestPresent) {
        const sysandClass = sysand.installed && !sysand.warnings?.length ? 'ok' : 'warning';
        const sysandLabel = !sysand.installed && sysand.manifestPresent
          ? 'project, not installed'
          : sysand.projectRoot ? 'project ready' : 'installed';
        const sysandSection = el('div', 'section');
        const sysandHead = el('div', 'section-head');
        sysandHead.title = [
          sysand.version || '',
          sysand.projectRoot ? 'project: ' + sysand.projectRoot : 'no project manifest',
          String((sysand.dependencyRoots || []).length) + ' dependency root(s)',
          sysand.lockPresent ? 'lockfile present' : ''
        ].filter(Boolean).join(' / ');
        sysandHead.appendChild(el('div', 'title', 'Sysand Dependencies'));
        sysandHead.appendChild(sectionTrail(el('span', 'pill ' + sysandClass, sysandLabel)));
        sysandSection.appendChild(sysandHead);
        if (Array.isArray(sysand.warnings) && sysand.warnings.length) {
          const list = el('ul', 'warning-list');
          sysand.warnings.forEach(warning => list.appendChild(el('li', '', warning)));
          sysandSection.appendChild(list);
        }
        const sysandActions = el('div', 'actions');
        sysandActions.appendChild(button('Refresh dependency roots and restart language server', 'sync', 'refreshSysandDependencies'));
        if (!sysand.installed && sysand.manifestPresent) {
          sysandActions.appendChild(button('Copy Sysand install command', 'copy', 'copySysandInstall'));
          sysandActions.appendChild(button('Open Sysand documentation', 'link-external', 'openSysandDocs'));
        }
        sysandSection.appendChild(sysandActions);
        nodes.push(sysandSection);
      }

      dashboard.replaceChildren(...nodes);
    }

    function renderRows(rows, mode, queryText) {
      renderedRows = Array.isArray(rows) ? rows : [];
      if (!renderedRows.length) {
        state.textContent = queryText ? 'No results for "' + queryText + '".' : 'No library symbols found.';
        results.replaceChildren();
        return;
      }
      state.textContent = mode === 'browse'
        ? 'Browsing ' + renderedRows.length + ' indexed symbol(s).'
        : 'Found ' + renderedRows.length + ' symbol(s).';
      const nodes = renderedRows.map((row, index) => {
        const item = el('div', 'result');
        const title = el('div', 'result-title');
        title.appendChild(el('div', 'title', row.name || ''));
        title.appendChild(el('span', 'pill info', row.source || 'library'));
        item.appendChild(title);
        const meta = el('div', 'meta');
        [row.kind, row.packageName, row.container, row.path].filter(Boolean).forEach(value => meta.appendChild(el('span', '', String(value))));
        item.appendChild(meta);
        const actions = el('div', 'actions');
        actions.appendChild(button('Open definition', 'go-to-file', 'openResult', { payload: { uri: row.uri, range: row.range } }));
        actions.appendChild(button('Copy qualified name', 'copy', 'copyText', { text: row.qualifiedName }));
        actions.appendChild(button('Copy import statement', 'symbol-namespace', 'copyText', { text: row.importStatement }));
        item.appendChild(actions);
        item.addEventListener('dblclick', () => vscode.postMessage({ type: 'openResult', payload: { uri: row.uri, range: row.range } }));
        item.dataset.index = String(index);
        return item;
      });
      results.replaceChildren(...nodes);
    }

    function renderBrowseTree(tree) {
      const sources = Array.isArray(tree?.sources) ? tree.sources : [];
      const details = el('details', '');
      const summary = el('summary', 'title', 'Package tree');
      details.appendChild(summary);
      sources.forEach(source => {
        const sourceNode = el('details', 'tree-package');
        const sourceLabel = source.source === 'standard'
          ? 'Standard Library'
          : source.source === 'domain'
            ? 'Domain Libraries'
            : 'Custom Libraries';
        sourceNode.appendChild(el('summary', 'muted', sourceLabel));
        (source.packages || []).forEach(pkg => {
          const pkgNode = el('details', 'tree-package');
          pkgNode.appendChild(el('summary', 'muted', String(pkg.name || '(package)') + ' (' + String((pkg.symbols || []).length) + ')'));
          sourceNode.appendChild(pkgNode);
        });
        details.appendChild(sourceNode);
      });
      results.appendChild(details);
    }

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg?.type === 'dashboardLoading') {
        dashboard.replaceChildren(el('div', 'section', 'Loading library status...'));
        return;
      }
      if (msg?.type === 'dashboard') {
        renderDashboard(msg.payload || {});
        return;
      }
      if (msg?.type === 'setQuery') {
        query.value = msg.payload || '';
        return;
      }
      if (msg?.type === 'searching') {
        state.textContent = msg.payload ? 'Searching...' : 'Loading library symbols...';
        return;
      }
      if (msg?.type === 'error') {
        state.textContent = 'Error: ' + (msg.payload || 'unknown');
        results.replaceChildren();
        return;
      }
      if (msg?.type === 'results' || msg?.type === 'browse') {
        renderRows(msg.payload?.rows || [], msg.type, msg.payload?.query || query.value.trim());
        if (msg.type === 'browse') {
          renderBrowseTree(msg.payload?.tree || {});
        }
      }
    });

    query.addEventListener('input', () => {
      if (timer) clearTimeout(timer);
      timer = setTimeout(() => {
        const q = query.value.trim();
        if (!q) {
          state.textContent = 'Type to search or browse all libraries.';
          results.replaceChildren();
          return;
        }
        vscode.postMessage({ type: 'search', query: q });
      }, 200);
    });

    btnBrowseAll.addEventListener('click', () => {
      vscode.postMessage({ type: 'browseAll' });
    });

    vscode.postMessage({ type: 'initLoad' });
  </script>
</body>
</html>`;
  }
}
