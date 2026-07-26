import * as vscode from "vscode";
import type {
  FeatureInspectorElementRef,
  FeatureInspectorResult,
  LspModelProvider,
} from "../providers/lspModelProvider";

export type FeatureInspectorRange = FeatureInspectorElementRef["range"];

/**
 * Sidebar webview showing the fully resolved semantics of the element under the cursor -- the
 * complement to hover, which is deliberately kept minimal (see the recent hover redesign). Backed
 * by the already-existing `sysml/featureInspector` LSP request; this file only adds the VS Code
 * side that consumes it.
 *
 * Modeled on `LibraryWebviewViewProvider` (single-file inline HTML/CSS/JS, no build step) rather
 * than the diagram-canvas `VisualizationPanel` -- this panel is structured text/lists, not a
 * diagram.
 */
export class FeatureInspectorViewProvider implements vscode.WebviewViewProvider {
  private view: vscode.WebviewView | undefined;
  private webviewReady = false;
  private pinned = false;
  private latestResult: FeatureInspectorResult | null | undefined;
  private latestError: string | undefined;

  private readonly _onResumeRequested = new vscode.EventEmitter<void>();
  /** Fires when the user clicks "resume following cursor", or when the panel becomes visible
   * again while not pinned -- the caller should immediately push a fresh live-cursor update. */
  readonly onResumeRequested = this._onResumeRequested.event;

  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly lspModelProvider: LspModelProvider
  ) {}

  resolveWebviewView(webviewView: vscode.WebviewView): void {
    this.view = webviewView;
    this.webviewReady = false;
    const webview = webviewView.webview;
    webview.options = { enableScripts: true };
    webview.html = this.getHtml(webview);

    webview.onDidReceiveMessage(async (message) => {
      if (message?.type === "ready") {
        this.webviewReady = true;
        if (this.latestError) {
          this.post({ type: "error", payload: this.latestError });
        } else if (this.latestResult !== undefined) {
          this.post({
            type: "update",
            payload: this.latestResult,
            pinned: this.pinned,
          });
        } else if (!this.pinned) {
          this._onResumeRequested.fire();
        }
        return;
      }
      if (message?.type === "openRange") {
        await this.openRange(message.payload as { uri: string; range: FeatureInspectorRange });
        return;
      }
      if (message?.type === "inspect") {
        await this.inspectTarget(message.payload as FeatureInspectorElementRef);
        return;
      }
      if (message?.type === "resume") {
        this.pinned = false;
        this._onResumeRequested.fire();
        return;
      }
    });

    webviewView.onDidChangeVisibility(() => {
      if (webviewView.visible && !this.pinned) {
        this._onResumeRequested.fire();
      }
    });
    webviewView.onDidDispose(() => {
      if (this.view === webviewView) {
        this.view = undefined;
        this.webviewReady = false;
      }
    });
  }

  isVisible(): boolean {
    return this.view?.visible ?? false;
  }

  isPinned(): boolean {
    return this.pinned;
  }

  /** Pushes a live-cursor update. Ignored while the panel is pinned to an explored target. */
  update(result: FeatureInspectorResult | undefined): void {
    if (this.pinned) {
      return;
    }
    this.latestResult = result ?? null;
    this.latestError = undefined;
    this.post({ type: "update", payload: this.latestResult, pinned: false });
  }

  private async inspectTarget(target: FeatureInspectorElementRef): Promise<void> {
    if (!target?.uri || !target.range) {
      return;
    }
    await this.inspectAt(target.uri, target.range.start);
  }

  /** Pins the inspector to the element at `position` in `uri`. Used when the caller only has a
   * source location (e.g. a diagram node click) rather than a full FeatureInspectorElementRef. */
  async inspectAt(uri: string, position: FeatureInspectorRange["start"]): Promise<void> {
    this.pinned = true;
    try {
      const result = await this.lspModelProvider.getFeatureInspector(uri, position);
      this.latestResult = result ?? null;
      this.latestError = undefined;
      this.post({ type: "update", payload: this.latestResult, pinned: true });
    } catch (error) {
      this.latestError = error instanceof Error ? error.message : String(error);
      this.post({
        type: "error",
        payload: this.latestError,
      });
    }
  }

  private async openRange(message: {
    uri: string;
    range: FeatureInspectorRange;
  }): Promise<void> {
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
    if (this.webviewReady) {
      void this.view?.webview.postMessage(message);
    }
  }

  private getHtml(webview: vscode.Webview): string {
    const nonce = getNonce();
    const codiconsCss = webview
      .asWebviewUri(vscode.Uri.joinPath(this.extensionUri, "media", "codicons", "codicon.css"))
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
    .section { border-top: 1px solid var(--vscode-panel-border); padding: 8px 0; }
    .section:first-child { border-top: none; padding-top: 0; }
    .title { font-weight: 600; }
    .muted { color: var(--vscode-descriptionForeground); font-size: 12px; }
    .header-line { display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
    .qualified-name { font-family: var(--vscode-editor-font-family); font-size: 12px; color: var(--vscode-descriptionForeground); margin-top: 2px; word-break: break-word; }
    .doc { margin-top: 8px; font-size: 12px; line-height: 1.45; white-space: pre-wrap; }
    .pill { font-size: 11px; border-radius: 3px; padding: 1px 5px; border: 1px solid var(--vscode-panel-border); }
    .pill.ok { color: var(--vscode-testing-iconPassed); }
    .pill.info { color: var(--vscode-descriptionForeground); }
    .pill.warning { color: var(--vscode-testing-iconFailed); }
    .field { display: flex; gap: 6px; font-size: 12px; margin-top: 4px; }
    .field .label { color: var(--vscode-descriptionForeground); min-width: 84px; flex-shrink: 0; }
    .field .value { font-family: var(--vscode-editor-font-family); word-break: break-word; }
    .row { display: flex; align-items: center; justify-content: space-between; gap: 6px; padding: 4px 2px; border-radius: 3px; cursor: pointer; }
    .row:hover { background: var(--vscode-list-hoverBackground); }
    .row-main { min-width: 0; }
    .row-name { font-size: 12px; }
    .row-sub { font-size: 11px; color: var(--vscode-descriptionForeground); word-break: break-word; }
    .icon-btn { border: 1px solid var(--vscode-button-border, var(--vscode-panel-border)); background: transparent; color: var(--vscode-foreground); border-radius: 4px; min-width: 22px; height: 22px; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; padding: 0 4px; flex-shrink: 0; }
    .icon-btn:hover { background: var(--vscode-toolbar-hoverBackground); }
    details { margin-top: 4px; }
    summary { cursor: pointer; font-size: 12px; }
    .rel-list { margin: 4px 0 0 0; }
    .pinned-banner { display: flex; align-items: center; justify-content: space-between; gap: 8px; background: var(--vscode-editorWidget-background); border: 1px solid var(--vscode-panel-border); border-radius: 4px; padding: 4px 8px; margin-bottom: 8px; font-size: 12px; }
    .pinned-banner button { border: none; background: transparent; color: var(--vscode-textLink-foreground); cursor: pointer; padding: 0; font-size: 12px; }
    .pinned-banner button:hover { text-decoration: underline; }
    .placeholder { color: var(--vscode-descriptionForeground); font-size: 12px; padding: 4px 0; }
  </style>
</head>
<body>
  <div id="root"><div class="placeholder">Place the cursor on a SysML/KerML declaration or keyword. The inspector shows language help, resolved semantics, relationships, and source.</div></div>

  <script nonce="${nonce}">
    const vscode = acquireVsCodeApi();
    const root = document.getElementById('root');

    function el(tag, className, text) {
      const node = document.createElement(tag);
      if (className) node.className = className;
      if (text !== undefined) node.textContent = text;
      return node;
    }

    function iconButton(title, icon, onClick) {
      const btn = el('button', 'icon-btn');
      btn.title = title;
      btn.appendChild(el('span', 'codicon codicon-' + icon));
      btn.addEventListener('click', (event) => { event.stopPropagation(); onClick(); });
      return btn;
    }

    function basename(uri) {
      try {
        const path = decodeURIComponent(new URL(uri).pathname);
        const parts = path.split('/');
        return parts[parts.length - 1] || uri;
      } catch {
        return uri;
      }
    }

    function attrText(attributes, key) {
      const value = attributes ? attributes[key] : undefined;
      if (value === undefined || value === null || value === '') return undefined;
      if (typeof value === 'object') return Array.isArray(value) ? value.filter(Boolean).join(', ') : undefined;
      return String(value);
    }

    function elementRefRow(target, statusPill) {
      const row = el('div', 'row');
      row.tabIndex = 0;
      const main = el('div', 'row-main');
      main.appendChild(el('div', 'row-name', target.name || '(unnamed)'));
      main.appendChild(el('div', 'row-sub', target.qualifiedName || ''));
      row.appendChild(main);
      const right = el('div', 'row-main');
      right.style.display = 'flex';
      right.style.alignItems = 'center';
      right.style.gap = '4px';
      right.style.flexShrink = '0';
      if (statusPill) right.appendChild(statusPill);
      right.appendChild(iconButton('Reveal in editor', 'go-to-file', () => {
        vscode.postMessage({ type: 'openRange', payload: { uri: target.uri, range: target.range } });
      }));
      row.appendChild(right);
      row.addEventListener('click', () => vscode.postMessage({ type: 'inspect', payload: target }));
      row.addEventListener('keydown', (event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault();
          vscode.postMessage({ type: 'inspect', payload: target });
        }
      });
      return row;
    }

    function pill(label, kind) {
      return el('span', 'pill ' + kind, label);
    }

    function resolutionSection(nodes, title, resolution) {
      if (!resolution || resolution.status === 'notApplicable') return;
      const section = el('div', 'section');
      section.appendChild(el('div', 'title', title));
      if (resolution.status === 'unresolved') {
        section.appendChild(pill('unresolved', 'warning'));
        nodes.push(section);
        return;
      }
      resolution.targets.forEach((target) => {
        section.appendChild(elementRefRow(target, pill('resolved', 'ok')));
      });
      nodes.push(section);
    }

    function relationshipsSection(nodes, title, relationships) {
      if (!relationships || !relationships.length) return;
      const section = el('div', 'section');
      const details = el('details', '');
      const summary = el('summary', '', title + ' (' + relationships.length + ')');
      details.appendChild(summary);
      const list = el('div', 'rel-list');
      relationships.forEach((relationship) => {
        const row = elementRefRow(relationship.peer, pill(relationship.type, 'info'));
        list.appendChild(row);
      });
      details.appendChild(list);
      section.appendChild(details);
      nodes.push(section);
    }

    function isDefinitionLike(element) {
      if (element.typing && element.typing.status !== 'notApplicable') return 'Usage';
      if (element.type && element.type.endsWith(' def')) return 'Definition';
      return undefined;
    }

    function renderPinnedBanner(nodes, pinned, name) {
      if (!pinned) return;
      const banner = el('div', 'pinned-banner');
      banner.appendChild(el('span', '', 'Pinned — showing ' + (name || 'element') + '.'));
      const resume = el('button', '', 'Resume following cursor');
      resume.addEventListener('click', () => vscode.postMessage({ type: 'resume' }));
      banner.appendChild(resume);
      nodes.push(banner);
    }

    function helpText(markdown) {
      return String(markdown || '')
        .replace(/\\*\\*/g, '')
        .replace(/\`/g, '')
        .replace(/\\n\\n\\*See SysML v2 specification for full syntax\\.\\*/g, '');
    }

    function render(payload, pinned) {
      const nodes = [];
      const element = payload ? payload.element : null;
      if (!element) {
        renderPinnedBanner(nodes, pinned, undefined);
        if (payload?.contextualHelpMarkdown) {
          const helpSection = el('div', 'section');
          helpSection.appendChild(el('div', 'title', 'SysML v2 language help'));
          helpSection.appendChild(el('div', 'doc', helpText(payload.contextualHelpMarkdown)));
          nodes.push(helpSection);
        } else {
          nodes.push(el('div', 'placeholder', 'Place the cursor on a SysML/KerML declaration or keyword. The inspector shows language help, resolved semantics, relationships, and source.'));
        }
        root.replaceChildren(...nodes);
        return;
      }

      renderPinnedBanner(nodes, pinned, element.name);

      const header = el('div', 'section');
      const headerLine = el('div', 'header-line');
      headerLine.appendChild(el('span', 'title', (element.type || 'element') + ' ' + (element.name || '')));
      const kindLabel = isDefinitionLike(element);
      if (kindLabel) headerLine.appendChild(pill(kindLabel, kindLabel === 'Definition' ? 'ok' : 'info'));
      header.appendChild(headerLine);
      header.appendChild(el('div', 'qualified-name', element.qualifiedName || ''));
      nodes.push(header);

      if (payload.contextualHelpMarkdown) {
        const helpSection = el('div', 'section');
        helpSection.appendChild(el('div', 'title', 'SysML v2 language help'));
        helpSection.appendChild(el('div', 'doc', helpText(payload.contextualHelpMarkdown)));
        nodes.push(helpSection);
      }

      const doc = attrText(element.attributes, 'doc');
      if (doc) {
        const docSection = el('div', 'section');
        docSection.appendChild(el('div', 'doc', doc));
        nodes.push(docSection);
      }

      resolutionSection(nodes, 'Declared type', element.typing);
      resolutionSection(nodes, 'Specializes', element.specialization);

      const attrFields = [
        ['Multiplicity', attrText(element.attributes, 'multiplicity')],
        ['Direction', attrText(element.attributes, 'direction')],
        ['Value', attrText(element.attributes, 'value') || attrText(element.attributes, 'defaultValue')],
        ['Evaluated', [attrText(element.attributes, 'evaluatedValue'), attrText(element.attributes, 'evaluatedUnit')].filter(Boolean).join(' ')],
      ].filter(([, value]) => value);
      if (attrFields.length) {
        const attrSection = el('div', 'section');
        attrFields.forEach(([label, value]) => {
          const field = el('div', 'field');
          field.appendChild(el('span', 'label', label));
          field.appendChild(el('span', 'value', value));
          attrSection.appendChild(field);
        });
        nodes.push(attrSection);
      }

      relationshipsSection(nodes, 'Incoming', element.incomingRelationships);
      relationshipsSection(nodes, 'Outgoing', element.outgoingRelationships);

      const locationSection = el('div', 'section');
      const locationRow = el('div', 'row');
      locationRow.style.cursor = 'default';
      const locationMain = el('div', 'row-main');
      locationMain.appendChild(el('div', 'row-name', 'Defined in'));
      locationMain.appendChild(el('div', 'row-sub', basename(element.uri)));
      locationRow.appendChild(locationMain);
      locationRow.appendChild(iconButton('Reveal in editor', 'go-to-file', () => {
        vscode.postMessage({ type: 'openRange', payload: { uri: element.uri, range: element.range } });
      }));
      locationSection.appendChild(locationRow);
      nodes.push(locationSection);

      root.replaceChildren(...nodes);
    }

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg?.type === 'update') {
        render(msg.payload || null, !!msg.pinned);
        return;
      }
      if (msg?.type === 'error') {
        const nodes = [];
        renderPinnedBanner(nodes, true, undefined);
        nodes.push(el('div', 'placeholder', 'Error: ' + (msg.payload || 'unknown')));
        root.replaceChildren(...nodes);
      }
    });
    vscode.postMessage({ type: 'ready' });
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
