import * as vscode from 'vscode';
import { LspModelProvider } from '../providers/lspModelProvider';
import type { GraphNodeDTO } from '../providers/sysmlModelTypes';
import { createMessageDispatcher } from './messageHandlers';
import { createUpdateVisualizationFlow } from './updateFlow';
import { getWebviewHtml } from './htmlBuilder';
import { DEFAULT_ENABLED_VIEWS, EXPERIMENTAL_VIEWS } from './webview/constants';
import { logError } from '../logger';

export const RESTORE_STATE_KEY = 'sysmlVisualizerRestoreState';

function getEnabledVisualizationViewIds(): Set<string> {
    const enabled = new Set<string>(DEFAULT_ENABLED_VIEWS);
    const includeExperimentalViews =
        vscode.workspace.getConfiguration('spec42').get<boolean>('visualization.enableExperimentalViews') ??
        vscode.workspace.getConfiguration('sysml-language-server').get<boolean>('visualization.enableExperimentalViews') ??
        false;
    if (includeExperimentalViews) {
        for (const viewId of EXPERIMENTAL_VIEWS) {
            enabled.add(viewId);
        }
    }
    return enabled;
}

function parseFileUri(value: string, label: string): vscode.Uri | undefined {
    try {
        const uri = vscode.Uri.parse(value);
        if (uri.scheme !== 'file') {
            logError(`Visualization restore ignored non-file URI for ${label}: ${value}`);
            return undefined;
        }
        return uri;
    } catch (error) {
        logError(`Visualization restore failed to parse URI for ${label}: ${value}`, error);
        return undefined;
    }
}

async function createCombinedDocumentProxy(fileUris: vscode.Uri[]): Promise<vscode.TextDocument> {
    const openDocs: vscode.TextDocument[] = [];
    let combinedContent = '';
    let failedCount = 0;
    for (const fileUri of fileUris) {
        try {
            const doc = await vscode.workspace.openTextDocument(fileUri);
            openDocs.push(doc);
            const fileName = fileUri.fsPath.split(/[/\\]/).pop() ?? '';
            combinedContent += `// === ${fileName} ===\n`;
            combinedContent += doc.getText();
            combinedContent += '\n\n';
        } catch (error) {
            failedCount += 1;
            logError(`createCombinedDocumentProxy: failed to open ${fileUri.toString()}`, error);
        }
    }
    if (openDocs.length === 0) {
        throw new Error(`No documents could be opened for combined visualization (${failedCount} failed).`);
    }
    const firstDoc = openDocs[0];
    return {
        getText: () => combinedContent,
        uri: firstDoc.uri,
        languageId: 'sysml' as const,
        version: firstDoc.version,
        lineCount: combinedContent.split('\n').length,
        lineAt: (line: number) => firstDoc.lineAt(Math.min(line, firstDoc.lineCount - 1)),
        offsetAt: (position: vscode.Position) => firstDoc.offsetAt(position),
        positionAt: (offset: number) => firstDoc.positionAt(offset),
        getWordRangeAtPosition: (position: vscode.Position) => firstDoc.getWordRangeAtPosition(position),
        validateRange: (range: vscode.Range) => firstDoc.validateRange(range),
        validatePosition: (position: vscode.Position) => firstDoc.validatePosition(position),
        fileName: firstDoc.fileName,
        isUntitled: false,
        isDirty: false,
        isClosed: false,
        eol: firstDoc.eol,
        save: () => Promise.resolve(false),
    } as unknown as vscode.TextDocument;
}

export interface VisualizerRestoreState {
    documentUri: string;
    fileUris: string[];
    currentView: string;
    title?: string;
}

export class VisualizationPanel {
    public static currentPanel: VisualizationPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private _disposables: vscode.Disposable[] = [];
    private _currentView: string = 'general-view'; // Store current view state - SysML v2 general-view
    private _isNavigating: boolean = false; // Flag to prevent view reset during navigation
    private _fileChangeDebounceTimer: ReturnType<typeof setTimeout> | undefined; // Debounce file change notifications
    private _requestCurrentViewTimer: ReturnType<typeof setTimeout> | undefined;
    private _lastContentHash: string = ''; // Cache content hash to skip unchanged updates
    private _needsUpdateWhenVisible: boolean = false; // Deferred update when panel is hidden
    private _lastViewColumn: vscode.ViewColumn | undefined; // Track view column to detect panel moves
    private _fileUris: vscode.Uri[] = []; // All source file URIs (for folder-level visualization)
    private _extensionVersion: string = '';
    private _pendingPackageName: string | undefined; // Package to select when data arrives
    private _updateFlow: ReturnType<typeof createUpdateVisualizationFlow>;

    private constructor(
        panel: vscode.WebviewPanel,
        extensionUri: vscode.Uri,
        private _document: vscode.TextDocument,
        private _lspModelProvider: LspModelProvider,
        fileUris?: vscode.Uri[],
        private _context?: vscode.ExtensionContext,
        initialCurrentView?: string,
    ) {
        this._fileUris = fileUris ?? [];
        if (initialCurrentView && getEnabledVisualizationViewIds().has(initialCurrentView)) {
            this._currentView = initialCurrentView;
        }
        this._extensionVersion = vscode.extensions.getExtension('Elan8.spec42')?.packageJSON?.version ?? '0.0.0';
        this._panel = panel;
        this._panel.onDidDispose(() => this.dispose(), null, this._disposables);

        this._lastViewColumn = panel.viewColumn;

        // When the panel becomes visible again or is moved (e.g. dragged to
        // a floating window), force a re-render so the visualizer recovers.
        this._panel.onDidChangeViewState(() => {
            const columnChanged = this._panel.viewColumn !== this._lastViewColumn;
            this._lastViewColumn = this._panel.viewColumn;

            if (this._panel.visible) {
                if (this._needsUpdateWhenVisible || columnChanged) {
                    this._needsUpdateWhenVisible = false;
                    // Reset content hash so the update is not skipped
                    this._lastContentHash = '';
                    this.updateVisualization(true, 'panelReveal');
                }
            }
        }, null, this._disposables);

        this._panel.webview.html = getWebviewHtml(this._panel.webview, extensionUri, this._extensionVersion);

        this._updateFlow = createUpdateVisualizationFlow({
            panel: this._panel,
            getDocument: () => this._document,
            getFileUris: () => this._fileUris,
            lspModelProvider: this._lspModelProvider,
            getCurrentView: () => this._currentView,
            getPendingPackageName: () => this._pendingPackageName,
            getIsNavigating: () => this._isNavigating,
            getNeedsUpdateWhenVisible: () => this._needsUpdateWhenVisible,
            getLastContentHash: () => this._lastContentHash,
            setLastContentHash: (h) => { this._lastContentHash = h; },
            setNeedsUpdateWhenVisible: (v) => { this._needsUpdateWhenVisible = v; },
            clearPendingPackageName: () => { this._pendingPackageName = undefined; },
        });

        this._requestCurrentViewTimer = setTimeout(() => {
            this._requestCurrentViewTimer = undefined;
            try {
                this._panel.webview.postMessage({ command: 'requestCurrentView' });
            } catch {
                // Panel may be disposed during teardown in tests.
            }
        }, 100);

        const dispatch = createMessageDispatcher({
            panel: this._panel,
            document: this._document,
            lspModelProvider: this._lspModelProvider,
            fileUris: this._fileUris,
            updateVisualization: (force, triggerSource) => this.updateVisualization(force, triggerSource),
            setNavigating: (v) => { this._isNavigating = v; },
            setCurrentView: (v) => {
                this._currentView = v;
                this.persistRestoreState();
            },
            setLastContentHash: (h) => { this._lastContentHash = h; },
        });

        this._panel.webview.onDidReceiveMessage(dispatch, null, this._disposables);

        // Let the webview bootstrap drive the first forced render via the
        // `webviewReady` message. Doing it here as well causes an immediate
        // duplicate fetch/render cycle on startup.
        this.persistRestoreState();
    }

    private persistRestoreState(): void {
        if (!this._context) return;
        const state: VisualizerRestoreState = {
            documentUri: this._document.uri.toString(),
            fileUris: this._fileUris.map(u => u.toString()),
            currentView: this._currentView,
            title: this._panel.title !== 'SysML Model Visualizer' ? this._panel.title : undefined,
        };
        this._context.workspaceState.update(RESTORE_STATE_KEY, state);
    }

    public static createOrShow(context: vscode.ExtensionContext, document: vscode.TextDocument, customTitle?: string, lspModelProvider?: LspModelProvider, fileUris?: vscode.Uri[]): void {
        const extensionUri = context.extensionUri;
        // Determine the best column layout for side-by-side viewing
        const activeColumn = vscode.window.activeTextEditor?.viewColumn;
        let visualizerColumn: vscode.ViewColumn;

        if (activeColumn === vscode.ViewColumn.One) {
            visualizerColumn = vscode.ViewColumn.Two;
        } else if (activeColumn === vscode.ViewColumn.Two) {
            visualizerColumn = vscode.ViewColumn.Three;
        } else {
            // Default: put visualizer on the right
            visualizerColumn = vscode.ViewColumn.Beside;
        }

        const title = customTitle || 'SysML Model Visualizer';

        if (VisualizationPanel.currentPanel) {
            // If panel exists, update title and reveal it
            VisualizationPanel.currentPanel._panel.title = title;
            VisualizationPanel.currentPanel._panel.reveal(visualizerColumn);
            if (lspModelProvider) {
                VisualizationPanel.currentPanel._lspModelProvider = lspModelProvider;
            }
            // Track whether file URIs changed (folder→folder or file→folder)
            let fileUrisChanged = false;
            if (fileUris) {
                const oldSet = new Set(VisualizationPanel.currentPanel._fileUris.map(u => u.toString()));
                const newSet = new Set(fileUris.map(u => u.toString()));
                fileUrisChanged = oldSet.size !== newSet.size
                    || [...newSet].some(u => !oldSet.has(u));
                VisualizationPanel.currentPanel._fileUris = fileUris;
            }
            // Update if the document changed OR the set of file URIs changed
            if (VisualizationPanel.currentPanel._document !== document || fileUrisChanged) {
                VisualizationPanel.currentPanel._document = document;
                VisualizationPanel.currentPanel._lastContentHash = ''; // force re-parse
                VisualizationPanel.currentPanel.updateVisualization(true, 'createOrShow');
            }
            VisualizationPanel.currentPanel.persistRestoreState();
            return;
        }

        if (!lspModelProvider) {
            return;  // Cannot create panel without an LSP model provider
        }

        const panel = vscode.window.createWebviewPanel(
            'sysmlVisualizer',
            title,
            visualizerColumn,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [
                    vscode.Uri.joinPath(extensionUri, 'media')
                ]
            }
        );

        VisualizationPanel.currentPanel = new VisualizationPanel(panel, extensionUri, document, lspModelProvider, fileUris, context);
    }

    /** Restore a webview panel from persisted state (e.g. after VS Code restart). */
    public static async restore(
        panel: vscode.WebviewPanel,
        context: vscode.ExtensionContext,
        lspModelProvider: LspModelProvider,
        savedState: VisualizerRestoreState,
    ): Promise<void> {
        const extensionUri = context.extensionUri;
        let document: vscode.TextDocument;
        let fileUris: vscode.Uri[];
        const restoredFileUris = savedState.fileUris
            .map((value, index) => parseFileUri(value, `fileUris[${index}]`))
            .filter((uri): uri is vscode.Uri => Boolean(uri));

        if (restoredFileUris.length > 1) {
            fileUris = restoredFileUris;
            document = await createCombinedDocumentProxy(fileUris);
        } else {
            const documentUri = parseFileUri(savedState.documentUri, 'documentUri');
            if (!documentUri) {
                throw new Error('Saved visualization state does not contain a valid file document URI.');
            }
            document = await vscode.workspace.openTextDocument(documentUri);
            fileUris = restoredFileUris.length === 1
                ? [restoredFileUris[0]]
                : [];
        }

        if (savedState.title) {
            panel.title = savedState.title;
        }

        const view = getEnabledVisualizationViewIds().has(savedState.currentView) ? savedState.currentView : 'general-view';
        VisualizationPanel.currentPanel = new VisualizationPanel(
            panel,
            extensionUri,
            document,
            lspModelProvider,
            fileUris,
            context,
            view,
        );
    }

    public exportVisualization(format: string, scale: number = 2) {
        this._panel.webview.postMessage({ command: 'export', format: format.toLowerCase(), scale });
    }

    private updateVisualization(forceUpdate: boolean = false, triggerSource: string = 'unknown'): Promise<void> {
        return this._updateFlow.update(forceUpdate, triggerSource);
    }

    public getDocument(): vscode.TextDocument {
        return this._document;
    }

    public isNavigating(): boolean {
        return this._isNavigating;
    }

    public tracksUri(uri: vscode.Uri): boolean {
        const target = uri.toString();
        return this._document.uri.toString() === target
            || this._fileUris.some((fileUri) => fileUri.toString() === target);
    }

    /** Exposes webview for tests (e.g. postMessage exportDiagramForTest). */
    public getWebview(): vscode.Webview {
        return this._panel.webview;
    }

    /** Update the LspModelProvider. */
    public setLspModelProvider(provider: LspModelProvider): void {
        this._lspModelProvider = provider;
    }

    public changeView(viewId: string): void {
        this._panel.webview.postMessage({
            command: 'changeView',
            view: viewId
        });
        this._currentView = viewId;
    }

    public selectPackage(packageName: string): void {
        // Store as pending so the next data message carries it to the webview
        this._pendingPackageName = packageName;
        this._currentView = 'general-view';
        // Also post directly in case the webview already has data
        this._panel.webview.postMessage({
            command: 'selectPackage',
            packageName: packageName
        });
    }

    public highlightElementByName(elementName: string, skipCentering: boolean = true): void {
        this._panel.webview.postMessage({
            command: 'highlightElement',
            elementName,
            skipCentering,
        });
    }

    public revealSourceSelection(node: GraphNodeDTO): void {
        if (node.type === 'package') {
            this.selectPackage(node.name);
            return;
        }
        this.highlightElementByName(node.name, false);
    }

    public notifyFileChanged(uri: vscode.Uri) {
        // Always force — the LSP server parses asynchronously, so the
        // model data may have changed even when the document text hasn't
        // (e.g. after a sysml/status 'end' notification).
        const uriStr = uri.toString();
        const docUri = this._document.uri.toString();
        const isTracked = docUri === uriStr
            || this._fileUris.some(u => u.toString() === uriStr);

        if (isTracked) {
            // Debounce: coalesce multiple notifications from
            // onDidChangeTextDocument, onDidSaveTextDocument, and the
            // file-system watcher into a single visualizer refresh.
            if (this._fileChangeDebounceTimer) {
                clearTimeout(this._fileChangeDebounceTimer);
            }
            this._fileChangeDebounceTimer = setTimeout(() => {
                this._fileChangeDebounceTimer = undefined;
                this.updateVisualization(true, 'fileChanged');
            }, 400);
        }
    }

    /** Force a visualizer refresh (e.g. after cache clear). */
    public refresh(): void {
        this.updateVisualization(true, 'manualRefresh');
    }

    public dispose() {
        VisualizationPanel.currentPanel = undefined;
        this._context?.workspaceState.update(RESTORE_STATE_KEY, undefined);
        if (this._requestCurrentViewTimer) {
            clearTimeout(this._requestCurrentViewTimer);
            this._requestCurrentViewTimer = undefined;
        }
        if (this._fileChangeDebounceTimer) {
            clearTimeout(this._fileChangeDebounceTimer);
            this._fileChangeDebounceTimer = undefined;
        }
        this._panel.dispose();
        while (this._disposables.length) {
            const disposable = this._disposables.pop();
            if (disposable) {
                disposable.dispose();
            }
        }
    }
}
