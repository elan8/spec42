import * as vscode from "vscode";
import type { LspModelProvider, SysMLLibrarySearchResult } from "../providers/lspModelProvider";
import {
  buildLibraryDashboardStatus,
  flattenLibrarySearchResults,
  LibraryDashboardStatus,
  SysandStatusViewModel,
  summarizeLibrarySearch,
} from "./libraryStatusViewModel";

type StdlibHeading = {
  pinnedVersion: string;
  format: string;
};

type DomainLibrariesHeading = {
  pinnedVersion: string;
  format: string;
};

type DomainLibrariesDoctorStatus = {
  resolvedPath?: string;
  sourceKind: string;
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
  getDomainLibrariesHeading: () => DomainLibrariesHeading;
  getDomainLibrariesStatus: () => Promise<DomainLibrariesDoctorStatus>;
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

      if (message?.type === "showDomainLibrariesInfo") {
        await vscode.commands.executeCommand("sysml.library.showDomainLibrariesStatus");
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
      const [summaryResult, sysand, domainDoctor] = await Promise.all([
        this.lspModelProvider.searchLibraries("", 50),
        this.options.getSysandStatus(),
        this.options.getDomainLibrariesStatus(),
      ]);
      const status = this.dashboardStatus(summaryResult, sysand, domainDoctor);
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
    domainDoctor: DomainLibrariesDoctorStatus
  ): LibraryDashboardStatus {
    const stdlibHeading = this.options.getStdlibHeading();
    const domainHeading = this.options.getDomainLibrariesHeading();
    return buildLibraryDashboardStatus({
      pinnedVersion: stdlibHeading.pinnedVersion,
      format: stdlibHeading.format,
      domainPinnedVersion: domainHeading.pinnedVersion,
      domainFormat: domainHeading.format,
      domainResolvedPath: domainDoctor.resolvedPath,
      domainSourceKind: domainDoctor.sourceKind,
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

  private async openResult(message: OpenRangeMessage): Promise<void> {
    if (!message?.uri || !message.range) {
      return;
    }
    const uri = vscode.Uri.parse(message.uri);
    const doc = await vscode.workspace.openTextDocument(uri);
    const editor = await vscode.window.showTextDocument(doc, {
      preserveFocus: false,
      preview: true,
    });
    const range = new vscode.Range(
      new vscode.Position(message.range.start.line, message.range.start.character),
      new vscode.Position(message.range.end.line, message.range.end.character)
    );
    editor.selection = new vscode.Selection(range.start, range.start);
    editor.revealRange(range, vscode.TextEditorRevealType.InCenter);
  }

  private post(message: unknown): void {
    this.view?.webview.postMessage(message);
  }

  private getHtml(webview: vscode.Webview): string {
    const nonce = getNonce();
    const codiconsCss = webview
      .asWebviewUri(
        vscode.Uri.joinPath(this.extensionUri, "media", "codicons", "codicon.css")
      )
      .toString();
    return `<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; font-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link nonce="${nonce}" rel="stylesheet" href="${codiconsCss}">
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); padding: 8px; }
    .section { border-top: 1px solid var(--vscode-panel-border); padding: 4px 0; }
    .section:first-child { border-top: none; padding-top: 0; }
    .section-head { display: flex; align-items: center; justify-content: space-between; gap: 8px; }
    .title { font-weight: 600; }
    .muted { color: var(--vscode-descriptionForeground); font-size: 12px; }
    .detail { margin-top: 3px; color: var(--vscode-descriptionForeground); font-size: 12px; line-height: 1.35; word-break: break-word; }
    .compact-row { display: flex; align-items: center; gap: 6px; flex-wrap: wrap; margin-top: 5px; color: var(--vscode-descriptionForeground); font-size: 12px; }
    .metric { color: var(--vscode-foreground); }
    .path { font-family: var(--vscode-editor-font-family); font-size: 11px; }
    .actions { display: flex; gap: 4px; flex-wrap: wrap; margin-top: 5px; }
    .icon-btn { border: 1px solid var(--vscode-button-border, var(--vscode-panel-border)); background: transparent; color: var(--vscode-foreground); border-radius: 4px; min-width: 24px; height: 24px; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; padding: 0 6px; }
    .icon-btn:hover { background: var(--vscode-toolbar-hoverBackground); }
    .pill { font-size: 11px; border-radius: 3px; padding: 1px 5px; border: 1px solid var(--vscode-panel-border); }
    .pill.ok { color: var(--vscode-testing-iconPassed); }
    .pill.info { color: var(--vscode-descriptionForeground); }
    .pill.warning { color: var(--vscode-testing-iconFailed); }
    .warning-list { margin: 6px 0 0 16px; padding: 0; color: var(--vscode-testing-iconFailed); font-size: 12px; }
    .search-row { display: flex; gap: 6px; margin: 8px 0; }
    input { flex: 1; min-width: 0; padding: 6px; border: 1px solid var(--vscode-input-border); background: var(--vscode-input-background); color: var(--vscode-input-foreground); }
    .result { border-top: 1px solid var(--vscode-panel-border); padding: 8px 0; }
    .result-title { display: flex; justify-content: space-between; gap: 8px; }
    .meta { font-size: 12px; color: var(--vscode-descriptionForeground); margin-top: 3px; display: flex; gap: 8px; flex-wrap: wrap; }
    details { margin-top: 6px; }
    summary { cursor: pointer; }
    .tree-package { margin-left: 10px; }
  </style>
</head>
<body>
  <div id="dashboard">
    <div class="section"><div class="muted">Loading library status...</div></div>
  </div>
  <div class="section">
    <div class="section-head">
      <div class="title">Lookup</div>
      <button id="btnBrowseAll" class="icon-btn" title="Browse all indexed library symbols">
        <span class="codicon codicon-list-tree"></span>
      </button>
    </div>
    <div class="search-row">
      <input id="query" type="text" placeholder="Search types, units, packages..." />
    </div>
    <div id="state" class="muted">Type to search or browse all libraries.</div>
    <div id="results"></div>
  </div>

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

    function compactRow(values) {
      const row = el('div', 'compact-row');
      values.filter(Boolean).forEach(value => row.appendChild(el('span', 'metric', String(value))));
      return row;
    }

    function formatBundle(format) {
      const normalized = String(format || '').trim();
      return normalized ? normalized.toUpperCase() : 'bundled';
    }

    function countText(packages, symbols) {
      return String(packages || 0) + ' packages / ' + String(symbols || 0) + ' symbols';
    }

    function renderDashboard(status) {
      const nodes = [];

      const std = el('div', 'section');
      const stdHead = el('div', 'section-head');
      stdHead.title = 'Release ' + (status?.stdlib?.pinnedVersion || 'unknown') + ' / ' + countText(status?.stdlib?.packageCount, status?.stdlib?.symbolCount) + ' / server-bundled';
      stdHead.appendChild(el('div', 'title', 'Standard Library'));
      stdHead.appendChild(el('span', 'pill ok', formatBundle(status?.stdlib?.format)));
      std.appendChild(stdHead);
      nodes.push(std);

      const domain = status?.domain || {};
      const domainSection = el('div', 'section');
      const domainHead = el('div', 'section-head');
      const domainPillClass = domain.available ? 'ok' : 'warning';
      const domainPillLabel = domain.available ? formatBundle(domain.format) : 'unavailable';
      domainHead.title = 'Revision ' + (domain.pinnedVersion || 'unknown') + ' / ' + countText(domain.packageCount, domain.symbolCount) + (domain.resolvedPath ? ' / ' + domain.resolvedPath : '');
      domainHead.appendChild(el('div', 'title', 'Domain Libraries'));
      domainHead.appendChild(el('span', 'pill ' + domainPillClass, domainPillLabel));
      domainSection.appendChild(domainHead);
      nodes.push(domainSection);

      const custom = status?.custom || {};
      const customSection = el('div', 'section');
      const customHead = el('div', 'section-head');
      const missing = Array.isArray(custom.missingPaths) ? custom.missingPaths : [];
      customHead.title = countText(custom.packageCount, custom.symbolCount) + (missing.length ? ' / ' + String(missing.length) + ' missing' : '');
      customHead.appendChild(el('div', 'title', 'Custom Libraries'));
      customHead.appendChild(el('span', 'pill ' + (missing.length ? 'warning' : 'info'), String((custom.configuredPaths || []).length) + ' path(s)'));
      customHead.appendChild(button('Manage custom library paths', 'settings-gear', 'manageCustomLibraries'));
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
        sysandHead.appendChild(el('span', 'pill ' + sysandClass, sysandLabel));
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

function getNonce(): string {
  let text = "";
  const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
