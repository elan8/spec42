import * as vscode from 'vscode';
import { getVisualizerLoadingMessage } from '../activation/workspaceLifecycle';
import { evaluateClientVisualizationReadiness } from './visualizationGate';
import { LspModelProvider } from '../providers/lspModelProvider';
import { fetchModelData, type FetchModelParams } from './modelFetcher';
import type { GraphNodeDTO } from '../providers/sysmlModelTypes';
import { SYSML_ENABLED_VIEWS } from './webview/constants';
import { logError } from '../logger';
import {
    BaseVisualizationPanelController,
    type BaseVisualizerRestoreState,
    type VisualizationPanelRuntimeState,
    type VisualizationPanelVariantConfig,
} from './baseVisualizationPanelController';
import { getVisualizerLocalResourceRoots, configureVisualizerWebview, getWebviewHtml } from './htmlBuilder';
import { createWebviewViewHost } from './visualizerHost';

export const RESTORE_STATE_KEY = 'sysmlVisualizerRestoreState';
export const VISUALIZER_VIEW_ID = 'sysmlVisualizerView';

const VISUALIZER_OPEN_CONTEXT_KEY = 'sysml.visualizerOpen';

function setVisualizerOpenContext(isOpen: boolean): void {
    VisualizationPanel._contextIsOpen = isOpen;
    void vscode.commands.executeCommand('setContext', VISUALIZER_OPEN_CONTEXT_KEY, isOpen);
}

export interface VisualizerRestoreState extends BaseVisualizerRestoreState {
    selectedView?: string;
}

function createVariantConfig(runtimeState: VisualizationPanelRuntimeState): VisualizationPanelVariantConfig<VisualizerRestoreState> {
    return {
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
 * VS Code calls resolveWebviewView once when the view becomes visible for the first time
 * and retains it (retainContextWhenHidden: true) so the diagram survives panel switches.
 */
export class VisualizationPanel implements vscode.WebviewViewProvider {
    public static currentPanel: VisualizationPanel | undefined;
    public static _contextIsOpen: boolean = false;

    private _extensionContext: vscode.ExtensionContext;
    private _lspModelProvider: LspModelProvider;
    private _runtimeState: VisualizationPanelRuntimeState | undefined;
    private _controller: BaseVisualizationPanelController<VisualizerRestoreState> | undefined;
    private _webviewView: vscode.WebviewView | undefined;

    public static get isOpen(): boolean {
        return VisualizationPanel._contextIsOpen;
    }

    private constructor(context: vscode.ExtensionContext, lspModelProvider: LspModelProvider) {
        this._extensionContext = context;
        this._lspModelProvider = lspModelProvider;
    }

    public static register(
        context: vscode.ExtensionContext,
        lspModelProvider: LspModelProvider,
    ): VisualizationPanel {
        const instance = new VisualizationPanel(context, lspModelProvider);
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

        const saved = this._extensionContext.workspaceState.get<VisualizerRestoreState>(RESTORE_STATE_KEY);

        this._runtimeState = {
            workspaceRootUri: workspaceRootUri.toString(),
            currentView: saved?.currentView && new Set<string>(SYSML_ENABLED_VIEWS).has(saved.currentView)
                ? saved.currentView
                : 'general-view',
            selectedView: saved?.selectedView,
            lspModelProvider: this._lspModelProvider,
        };

        const host = createWebviewViewHost(webviewView);
        this._controller = new BaseVisualizationPanelController(
            host,
            this._extensionContext.extensionUri,
            this._extensionContext,
            createVariantConfig(this._runtimeState),
        );

        setVisualizerOpenContext(webviewView.visible);
        webviewView.onDidChangeVisibility(() => {
            setVisualizerOpenContext(webviewView.visible);
        });
        webviewView.onDidDispose(() => {
            setVisualizerOpenContext(false);
            this._controller = undefined;
            this._runtimeState = undefined;
            this._webviewView = undefined;
        });
    }

    /** Reveal the view in the secondary sidebar. */
    public static reveal(): void {
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

    /** Close the secondary sidebar so VS Code destroys the WebviewView. */
    public dispose(): void {
        this._controller?.clearRestoreState();
        if (this._webviewView?.visible) {
            // Closing the auxiliary bar triggers onDidDispose, which clears state and
            // sets the context key. On next reveal, resolveWebviewView runs again.
            void vscode.commands.executeCommand('workbench.action.toggleAuxiliaryBar');
        } else {
            this._controller = undefined;
            this._runtimeState = undefined;
            this._webviewView = undefined;
            setVisualizerOpenContext(false);
        }
    }
}

