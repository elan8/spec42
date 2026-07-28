import * as vscode from 'vscode';
import { getVisualizerLoadingMessage } from '../activation/workspaceLifecycle';
import { evaluateClientVisualizationReadiness } from './visualizationGate';
import { LspModelProvider } from '../providers/lspModelProvider';
import { fetchModelData, type FetchModelParams } from './modelFetcher';
import type { GraphNodeDTO } from '../providers/sysmlModelTypes';
import { SYSML_ENABLED_VIEWS } from './webview/constants';
import {
    BaseVisualizationPanelController,
    type BaseVisualizerRestoreState,
    type VisualizationPanelRuntimeState,
    type VisualizationPanelVariantConfig,
} from './baseVisualizationPanelController';
import { getVisualizerLocalResourceRoots, configureVisualizerWebview, getWebviewHtml } from './htmlBuilder';
import { createWebviewPanelHost, createWebviewViewHost } from './visualizerHost';

export const RESTORE_STATE_KEY = 'sysmlVisualizerRestoreState';
export const VISUALIZER_VIEW_ID = 'sysmlVisualizerView';
export const VISUALIZER_EDITOR_VIEW_TYPE = 'sysml.visualizerEditor';

const VISUALIZER_OPEN_CONTEXT_KEY = 'sysml.visualizerOpen';
const VISUALIZER_IN_EDITOR_CONTEXT_KEY = 'sysml.visualizerInEditor';

type VisualizerHostMode = 'sidebar' | 'editor';

function setVisualizerOpenContext(isOpen: boolean): void {
    VisualizationPanel._contextIsOpen = isOpen;
    void vscode.commands.executeCommand('setContext', VISUALIZER_OPEN_CONTEXT_KEY, isOpen);
}

function setVisualizerInEditorContext(inEditor: boolean): void {
    VisualizationPanel._inEditor = inEditor;
    void vscode.commands.executeCommand('setContext', VISUALIZER_IN_EDITOR_CONTEXT_KEY, inEditor);
}

export interface VisualizerRestoreState extends BaseVisualizerRestoreState {
    selectedView?: string;
}

function createVariantConfig(
    runtimeState: VisualizationPanelRuntimeState,
    onInspectElement?: VisualizationPanelVariantConfig<VisualizerRestoreState>['onInspectElement'],
): VisualizationPanelVariantConfig<VisualizerRestoreState> {
    return {
        onInspectElement,
        panelTypeId: VISUALIZER_VIEW_ID,
        restoreStateKey: RESTORE_STATE_KEY,
        defaultTitle: 'SysML Visualizer',
        enabledViews: SYSML_ENABLED_VIEWS,
        defaultView: 'general-view',
        getLoadingMessage: () => getVisualizerLoadingMessage(),
        getRuntimeState: () => runtimeState,
        updateCurrentView: (view) => {
            runtimeState.currentView = view;
        },
        updateSelectedView: (selectedView) => {
            runtimeState.selectedView = selectedView || undefined;
        },
        serializeRestoreState: (state) => ({
            workspaceRootUri: state.workspaceRootUri,
            currentView: state.currentView,
            selectedView: state.selectedView,
        }),
        fetchUpdateMessage: (params: FetchModelParams) => fetchModelData(params),
        getContentHashSource: (state) => JSON.stringify({
            workspaceRootUri: state.workspaceRootUri,
            currentView: state.currentView,
            selectedView: state.selectedView ?? null,
            clientVisualizationReady: evaluateClientVisualizationReadiness().ready,
        }),
        normalizeView: (viewId) => new Set<string>(SYSML_ENABLED_VIEWS).has(viewId) ? viewId : 'general-view',
        shouldTrackUri: (uri, state) => {
            const workspaceRootUri = vscode.Uri.parse(state.workspaceRootUri);
            const rootPath = workspaceRootUri.fsPath.toLowerCase();
            return uri.fsPath.toLowerCase().startsWith(rootPath);
        },
    };
}

/**
 * WebviewView provider that registers the SysML Visualizer in the secondary sidebar.
 * The same controller can relocate into a WebviewPanel in the editor area; closing that
 * editor tab returns the visualizer to the sidebar.
 */
export class VisualizationPanel implements vscode.WebviewViewProvider {
    public static currentPanel: VisualizationPanel | undefined;
    public static _contextIsOpen: boolean = false;
    public static _inEditor: boolean = false;

    private _extensionContext: vscode.ExtensionContext;
    private _lspModelProvider: LspModelProvider;
    private _runtimeState: VisualizationPanelRuntimeState | undefined;
    private _controller: BaseVisualizationPanelController<VisualizerRestoreState> | undefined;
    private _webviewView: vscode.WebviewView | undefined;
    private _editorPanel: vscode.WebviewPanel | undefined;
    private _mode: VisualizerHostMode = 'sidebar';
    private _returningToSidebar = false;
    private _onInspectElement: VisualizationPanelVariantConfig<VisualizerRestoreState>['onInspectElement'];

    public static get isOpen(): boolean {
        return VisualizationPanel._contextIsOpen;
    }

    public static get isInEditor(): boolean {
        return VisualizationPanel._inEditor;
    }

    private constructor(
        context: vscode.ExtensionContext,
        lspModelProvider: LspModelProvider,
        onInspectElement?: VisualizationPanelVariantConfig<VisualizerRestoreState>['onInspectElement'],
    ) {
        this._extensionContext = context;
        this._lspModelProvider = lspModelProvider;
        this._onInspectElement = onInspectElement;
    }

    public static register(
        context: vscode.ExtensionContext,
        lspModelProvider: LspModelProvider,
        onInspectElement?: VisualizationPanelVariantConfig<VisualizerRestoreState>['onInspectElement'],
    ): VisualizationPanel {
        const instance = new VisualizationPanel(context, lspModelProvider, onInspectElement);
        VisualizationPanel.currentPanel = instance;
        context.subscriptions.push(
            vscode.window.registerWebviewViewProvider(VISUALIZER_VIEW_ID, instance, {
                webviewOptions: {
                    retainContextWhenHidden: true,
                },
            })
        );
        return instance;
    }

    /** Called by VS Code when the view becomes visible for the first time (or after reload). */
    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        _resolveContext: vscode.WebviewViewResolveContext,
        _token: vscode.CancellationToken,
    ): void {
        this._webviewView = webviewView;
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: getVisualizerLocalResourceRoots(this._extensionContext.extensionUri),
        };

        const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!workspaceRootUri) {
            const extVersion = vscode.extensions.getExtension('Elan8.spec42')?.packageJSON?.version ?? '0.0.0';
            configureVisualizerWebview(webviewView.webview, this._extensionContext.extensionUri);
            webviewView.webview.html = getWebviewHtml(
                webviewView.webview,
                this._extensionContext.extensionUri,
                extVersion,
                SYSML_ENABLED_VIEWS,
            );
            return;
        }

        this.ensureRuntimeState(workspaceRootUri);

        if (this._mode === 'editor') {
            this.showSidebarPlaceholder();
            setVisualizerOpenContext(true);
            webviewView.onDidChangeVisibility(() => {
                if (this._mode === 'editor') {
                    setVisualizerOpenContext(true);
                }
            });
            webviewView.onDidDispose(() => {
                if (this._mode === 'sidebar') {
                    setVisualizerOpenContext(false);
                    this._controller = undefined;
                    this._runtimeState = undefined;
                }
                this._webviewView = undefined;
            });
            return;
        }

        this.attachSidebarController();

        setVisualizerOpenContext(webviewView.visible);
        webviewView.onDidChangeVisibility(() => {
            if (this._mode === 'sidebar') {
                setVisualizerOpenContext(webviewView.visible);
            }
        });
        webviewView.onDidDispose(() => {
            if (this._mode === 'sidebar') {
                setVisualizerOpenContext(false);
                this._controller?.detach();
                this._controller = undefined;
                this._runtimeState = undefined;
            }
            this._webviewView = undefined;
        });
    }

    /** Reveal the active visualizer host (editor panel or secondary sidebar). */
    public static reveal(): void {
        const panel = VisualizationPanel.currentPanel;
        if (panel?._mode === 'editor' && panel._editorPanel) {
            panel._editorPanel.reveal(panel._editorPanel.viewColumn, false);
            return;
        }
        void vscode.commands.executeCommand(`${VISUALIZER_VIEW_ID}.focus`);
    }

    /** For backwards compatibility — used by commands that previously called createOrShow. */
    public static createOrShow(
        context: vscode.ExtensionContext,
        _document?: vscode.TextDocument,
        _customTitle?: string,
        lspModelProvider?: LspModelProvider,
        _workspaceRootUri?: vscode.Uri,
    ): void {
        if (lspModelProvider && VisualizationPanel.currentPanel) {
            VisualizationPanel.currentPanel._lspModelProvider = lspModelProvider;
            VisualizationPanel.currentPanel._controller?.setLspModelProvider(lspModelProvider);
        }
        VisualizationPanel.reveal();
    }

    /** Move the visualizer from the secondary sidebar into an editor tab. */
    public async moveToEditor(): Promise<void> {
        if (this._mode === 'editor' && this._editorPanel) {
            this._editorPanel.reveal(this._editorPanel.viewColumn, false);
            return;
        }

        const workspaceRootUri = vscode.workspace.workspaceFolders?.[0]?.uri;
        if (!workspaceRootUri) {
            vscode.window.showWarningMessage('Open a workspace folder to use the SysML Visualizer.');
            return;
        }

        this.ensureRuntimeState(workspaceRootUri);
        this._controller?.persistRestoreState();
        this._controller?.detach();
        this._controller = undefined;

        this._mode = 'editor';
        setVisualizerInEditorContext(true);
        this.showSidebarPlaceholder();

        const panel = vscode.window.createWebviewPanel(
            VISUALIZER_EDITOR_VIEW_TYPE,
            'SysML Visualizer',
            { viewColumn: vscode.ViewColumn.Beside, preserveFocus: false },
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: getVisualizerLocalResourceRoots(this._extensionContext.extensionUri),
            }
        );
        this._editorPanel = panel;

        const host = createWebviewPanelHost(panel);
        this._controller = new BaseVisualizationPanelController(
            host,
            this._extensionContext.extensionUri,
            this._extensionContext,
            createVariantConfig(this._runtimeState!, this._onInspectElement),
        );
        setVisualizerOpenContext(true);

        panel.onDidDispose(() => {
            this._editorPanel = undefined;
            if (this._returningToSidebar) {
                return;
            }
            if (this._mode === 'editor') {
                void this.returnToSidebar();
            }
        });
    }

    /** Move the visualizer from the editor tab back to the secondary sidebar. */
    public async returnToSidebar(): Promise<void> {
        if (this._mode !== 'editor' && !this._editorPanel) {
            VisualizationPanel.reveal();
            return;
        }

        this._returningToSidebar = true;
        try {
            // When the editor tab was closed, the controller is already detached via host
            // onDidDispose. Runtime state lives on this panel instance, so skip host title reads.
            if (this._controller && !this._controller.isDetached()) {
                this._controller.persistRestoreState();
                this._controller.detach();
            }
            this._controller = undefined;

            const editorPanel = this._editorPanel;
            this._editorPanel = undefined;
            editorPanel?.dispose();

            this._mode = 'sidebar';
            setVisualizerInEditorContext(false);

            if (this._webviewView) {
                this.attachSidebarController();
                setVisualizerOpenContext(this._webviewView.visible);
            } else {
                setVisualizerOpenContext(false);
            }

            VisualizationPanel.reveal();
        } finally {
            this._returningToSidebar = false;
        }
    }

    public exportVisualization(format: string, scale = 2): void {
        this._controller?.getWebview().postMessage({ command: 'export', format: format.toLowerCase(), scale });
    }

    public getDocument(): vscode.TextDocument | undefined {
        return this._runtimeState?.document;
    }

    public isNavigating(): boolean {
        return this._controller?.isNavigating() ?? false;
    }

    public tracksUri(uri: vscode.Uri): boolean {
        const workspaceRootUri = this._runtimeState?.workspaceRootUri;
        if (!workspaceRootUri) return false;
        const rootPath = vscode.Uri.parse(workspaceRootUri).fsPath.toLowerCase();
        return uri.fsPath.toLowerCase().startsWith(rootPath);
    }

    public getWebview(): vscode.Webview | undefined {
        return this._controller?.getWebview();
    }

    public setLspModelProvider(provider: LspModelProvider): void {
        this._lspModelProvider = provider;
        this._controller?.setLspModelProvider(provider);
    }

    public changeView(viewId: string): void {
        this._controller?.changeView(viewId);
    }

    public selectPackage(packageName: string): void {
        if (!this._runtimeState || !this._controller) return;
        this._runtimeState.selectedView = packageName;
        this._controller.refresh();
        this._controller.persistRestoreState();
    }

    public clearPackageSelection(): void {
        if (!this._runtimeState || !this._controller) return;
        this._runtimeState.selectedView = undefined;
        this._controller.refresh();
        this._controller.persistRestoreState();
    }

    public highlightElementByName(elementName: string, skipCentering = true): void {
        this._controller?.getWebview().postMessage({
            command: 'highlightElement',
            elementName,
            skipCentering,
        });
    }

    public revealSourceSelection(node: GraphNodeDTO): void {
        if (node.type === 'package') {
            this.selectPackage(node.id || node.name);
            return;
        }
        this.highlightElementByName(node.name, false);
    }

    public notifyFileChanged(uri: vscode.Uri): void {
        void this._controller?.notifyTrackedUriChanged(uri, 'fileChanged');
    }

    public refresh(): void {
        this._controller?.refresh();
    }

    public notifyWorkspaceLifecycleChanged(): void {
        this._controller?.notifyWorkspaceLifecycleChanged();
    }

    public requestUpdate(triggerSource = 'testSeed'): void {
        this._controller?.requestUpdate(triggerSource);
    }

    public prepareViewForTests(viewId: string, selectedView?: string): void {
        if (!this._runtimeState || !this._controller) return;
        this._runtimeState.currentView = this._controller.normalizeView(viewId);
        this._runtimeState.selectedView = selectedView;
        this._controller.requestUpdate('testSeed');
    }

    /** Close the active visualizer host. */
    public dispose(): void {
        this._returningToSidebar = true;
        try {
            this._mode = 'sidebar';
            setVisualizerInEditorContext(false);
            this._controller?.dispose();
            this._controller = undefined;
            this._runtimeState = undefined;

            if (this._editorPanel) {
                this._editorPanel.dispose();
                this._editorPanel = undefined;
                setVisualizerOpenContext(false);
                return;
            }

            if (this._webviewView?.visible) {
                // Closing the auxiliary bar triggers onDidDispose, which clears state and
                // sets the context key. On next reveal, resolveWebviewView runs again.
                void vscode.commands.executeCommand('workbench.action.toggleAuxiliaryBar');
            } else {
                this._webviewView = undefined;
                setVisualizerOpenContext(false);
            }
        } finally {
            this._returningToSidebar = false;
        }
    }

    private ensureRuntimeState(workspaceRootUri: vscode.Uri): void {
        if (this._runtimeState) {
            this._runtimeState.lspModelProvider = this._lspModelProvider;
            return;
        }
        const saved = this._extensionContext.workspaceState.get<VisualizerRestoreState>(RESTORE_STATE_KEY);
        this._runtimeState = {
            workspaceRootUri: workspaceRootUri.toString(),
            currentView: saved?.currentView && new Set<string>(SYSML_ENABLED_VIEWS).has(saved.currentView)
                ? saved.currentView
                : 'general-view',
            selectedView: saved?.selectedView,
            lspModelProvider: this._lspModelProvider,
        };
    }

    private attachSidebarController(): void {
        if (!this._webviewView || !this._runtimeState) {
            return;
        }
        this._controller?.detach();
        const host = createWebviewViewHost(this._webviewView);
        this._controller = new BaseVisualizationPanelController(
            host,
            this._extensionContext.extensionUri,
            this._extensionContext,
            createVariantConfig(this._runtimeState, this._onInspectElement),
        );
    }

    private showSidebarPlaceholder(): void {
        if (!this._webviewView) {
            return;
        }
        const webview = this._webviewView.webview;
        webview.html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <style>
    body {
      margin: 0;
      padding: 16px;
      font-family: var(--vscode-font-family);
      color: var(--vscode-foreground);
      background: var(--vscode-sideBar-background);
    }
    .title { font-weight: 600; margin-bottom: 8px; }
    .muted { color: var(--vscode-descriptionForeground); font-size: 12px; line-height: 1.45; }
  </style>
</head>
<body>
  <div class="title">Visualizer is open in the editor</div>
  <p class="muted">
    Close the SysML Visualizer editor tab to move it back here, or run
    <strong>Move Visualizer to Secondary Side Bar</strong> from the Command Palette.
  </p>
</body>
</html>`;
    }
}
