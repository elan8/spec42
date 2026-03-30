import * as vscode from 'vscode';
import type { LspModelProvider } from '../providers/lspModelProvider';
import { fetchModelData, hashContent } from './modelFetcher';
import { isVerboseLoggingEnabled, log, logError } from '../logger';

export interface UpdateFlowDeps {
    panel: vscode.WebviewPanel;
    document: vscode.TextDocument;
    fileUris: vscode.Uri[];
    lspModelProvider: LspModelProvider;
    getCurrentView: () => string;
    getPendingPackageName: () => string | undefined;
    getIsNavigating: () => boolean;
    getNeedsUpdateWhenVisible: () => boolean;
    getLastContentHash: () => string;
    setLastContentHash: (hash: string) => void;
    setNeedsUpdateWhenVisible: (value: boolean) => void;
    clearPendingPackageName: () => void;
}

export function createUpdateVisualizationFlow(deps: UpdateFlowDeps): { update: (force: boolean) => Promise<void> } {
    const {
        panel,
        document,
        fileUris,
        lspModelProvider,
        getCurrentView,
        getPendingPackageName,
        getIsNavigating,
        getNeedsUpdateWhenVisible,
        getLastContentHash,
        setLastContentHash,
        setNeedsUpdateWhenVisible,
        clearPendingPackageName,
    } = deps;

    async function doUpdateVisualization(): Promise<void> {
        try {
            log(
                'updateFlow:fetch:start',
                `doc=${document.uri.toString()}`,
                `fileUris=${fileUris.length}`,
                `currentView=${getCurrentView()}`,
                `pendingPackage=${getPendingPackageName() ?? '(none)'}`,
            );
            if (isVerboseLoggingEnabled()) {
                try {
                    // eslint-disable-next-line no-console
                    console.log(
                        '[viz][updateFlow:fetch:start]',
                        JSON.stringify({
                            doc: document.uri.toString(),
                            fileUris: fileUris.length,
                            currentView: getCurrentView(),
                            pendingPackage: getPendingPackageName() ?? null,
                        })
                    );
                } catch {
                    // ignore
                }
            }
            const msg = await fetchModelData({
                documentUri: document.uri.toString(),
                fileUris,
                lspModelProvider,
                currentView: getCurrentView(),
                pendingPackageName: getPendingPackageName(),
            });
            clearPendingPackageName();
            if (msg) {
                log(
                    'updateFlow:post:update',
                    `graphNodes=${msg.graph?.nodes?.length || 0}`,
                    `graphEdges=${msg.graph?.edges?.length || 0}`,
                    `generalNodes=${msg.generalViewGraph?.nodes?.length || 0}`,
                    `generalEdges=${msg.generalViewGraph?.edges?.length || 0}`,
                    `currentView=${msg.currentView}`,
                    `pendingPackage=${msg.pendingPackageName ?? '(none)'}`,
                );
                if (isVerboseLoggingEnabled()) {
                    try {
                        // eslint-disable-next-line no-console
                        console.log(
                            '[viz][updateFlow:post:update]',
                            JSON.stringify({
                                graphNodes: msg.graph?.nodes?.length || 0,
                                graphEdges: msg.graph?.edges?.length || 0,
                                generalNodes: msg.generalViewGraph?.nodes?.length || 0,
                                generalEdges: msg.generalViewGraph?.edges?.length || 0,
                                currentView: msg.currentView,
                                pendingPackage: msg.pendingPackageName ?? null,
                            })
                        );
                    } catch {
                        // ignore
                    }
                }
                panel.webview.postMessage(msg);
            } else {
                log('updateVisualization: no model data available, hiding loading state');
                panel.webview.postMessage({ command: 'hideLoading' });
            }
        } catch (error) {
            logError('updateVisualization failed', error);
            panel.webview.postMessage({ command: 'hideLoading' });
        }
    }

    async function update(forceUpdate: boolean = false): Promise<void> {
        if (getIsNavigating()) {
            return;
        }

        // During session restore the panel may exist but not yet be considered
        // visible. Allow forced updates (initial render / webviewReady) to run
        // so the visualizer doesn't get stuck in the loading state.
        if (!panel.visible && !forceUpdate) {
            setNeedsUpdateWhenVisible(true);
            return;
        }

        const content = document.getText();
        const contentHash = hashContent(content);

        if (!forceUpdate && contentHash === getLastContentHash()) {
            return;
        }
        setLastContentHash(contentHash);

        panel.webview.postMessage({ command: 'showLoading', message: 'Parsing SysML model...' });
        await new Promise(resolve => setTimeout(resolve, 0));

        await doUpdateVisualization();
    }

    return { update };
}
