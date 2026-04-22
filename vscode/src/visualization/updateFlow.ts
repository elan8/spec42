import * as vscode from 'vscode';
import type { LspModelProvider } from '../providers/lspModelProvider';
import { fetchModelData, hashContent } from './modelFetcher';
import { isVerboseLoggingEnabled, log, logError, logPerfEvent } from '../logger';

export interface UpdateFlowDeps {
    panel: vscode.WebviewPanel;
    getDocument: () => vscode.TextDocument;
    getWorkspaceRootUri: () => string;
    lspModelProvider: LspModelProvider;
    getCurrentView: () => string;
    getSelectedView: () => string | undefined;
    setCurrentView: (view: string) => void;
    getIsNavigating: () => boolean;
    getNeedsUpdateWhenVisible: () => boolean;
    getLastContentHash: () => string;
    setLastContentHash: (hash: string) => void;
    setNeedsUpdateWhenVisible: (value: boolean) => void;
}

function logPerf(event: string, extra?: Record<string, unknown>): void {
    logPerfEvent(event, extra);
}

export function createUpdateVisualizationFlow(deps: UpdateFlowDeps): { update: (force: boolean, triggerSource?: string) => Promise<void> } {
    const {
        panel,
        getDocument,
        getWorkspaceRootUri,
        lspModelProvider,
        getCurrentView,
        getSelectedView,
        setCurrentView,
        getIsNavigating,
        getNeedsUpdateWhenVisible,
        getLastContentHash,
        setLastContentHash,
        setNeedsUpdateWhenVisible,
    } = deps;
    let bootstrapCompleted = false;
    let inFlightUpdate:
        | {
            key: string;
            triggerSource: string;
            isBootstrap: boolean;
            promise: Promise<void>;
        }
        | undefined;

    function currentUpdateKey(): string {
        const document = getDocument();
        return JSON.stringify({
            documentUri: document.uri.toString(),
            workspaceRootUri: getWorkspaceRootUri(),
            currentView: getCurrentView(),
            selectedView: getSelectedView() ?? null,
        });
    }

    async function doUpdateVisualization(): Promise<void> {
        const document = getDocument();
        const workspaceRootUri = getWorkspaceRootUri();
        const updateStartedAt = Date.now();
        try {
            log(
                'updateFlow:fetch:start',
                `doc=${document.uri.toString()}`,
                `workspaceRootUri=${workspaceRootUri}`,
                `currentView=${getCurrentView()}`,
                `selectedView=${getSelectedView() ?? '(auto)'}`,
            );
            if (isVerboseLoggingEnabled()) {
                try {
                    // eslint-disable-next-line no-console
                    console.log(
                        '[viz][updateFlow:fetch:start]',
                        JSON.stringify({
                            doc: document.uri.toString(),
                            workspaceRootUri,
                            currentView: getCurrentView(),
                            selectedView: getSelectedView() ?? null,
                        })
                    );
                } catch {
                    // ignore
                }
            }
            const msg = await fetchModelData({
                workspaceRootUri,
                lspModelProvider,
                currentView: getCurrentView(),
                selectedView: getSelectedView(),
            });
            logPerf('visualizer:fetchModelDataCompleted', {
                currentView: getCurrentView(),
                totalMs: Date.now() - updateStartedAt,
                hasMessage: !!msg,
                workspaceRootUri,
            });
            if (msg) {
                if (msg.currentView && msg.currentView !== getCurrentView()) {
                    setCurrentView(msg.currentView);
                }
                log(
                    'updateFlow:post:update',
                    `graphNodes=${msg.graph?.nodes?.length || 0}`,
                    `graphEdges=${msg.graph?.edges?.length || 0}`,
                    `viewCandidates=${msg.viewCandidates?.length || 0}`,
                    `currentView=${msg.currentView}`,
                    `selectedView=${msg.selectedViewName ?? '(auto)'}`,
                );
                if (isVerboseLoggingEnabled()) {
                    try {
                        // eslint-disable-next-line no-console
                        console.log(
                            '[viz][updateFlow:post:update]',
                            JSON.stringify({
                                graphNodes: msg.graph?.nodes?.length || 0,
                                graphEdges: msg.graph?.edges?.length || 0,
                                viewCandidates: msg.viewCandidates?.length || 0,
                                currentView: msg.currentView,
                                selectedView: msg.selectedViewName ?? null,
                            })
                        );
                    } catch {
                        // ignore
                    }
                }
                const postStartedAt = Date.now();
                logPerf('visualizer:webviewPostMessageStarted', {
                    command: msg.command,
                    currentView: msg.currentView,
                    graphNodes: msg.graph?.nodes?.length || 0,
                    graphEdges: msg.graph?.edges?.length || 0,
                });
                const delivered = await panel.webview.postMessage(msg);
                logPerf('visualizer:webviewPostMessageCompleted', {
                    command: msg.command,
                    currentView: msg.currentView,
                    delivered,
                    totalMs: Date.now() - postStartedAt,
                });
            } else {
                log('updateVisualization: no model data available, hiding loading state');
                await panel.webview.postMessage({ command: 'hideLoading' });
            }
        } catch (error) {
            logError('updateVisualization failed', error);
            await panel.webview.postMessage({ command: 'hideLoading' });
        }
    }

    async function update(forceUpdate: boolean = false, triggerSource = 'unknown'): Promise<void> {
        if (getIsNavigating()) {
            return;
        }

        const isBootstrapTrigger = triggerSource === 'webviewReady';
        if (!bootstrapCompleted && forceUpdate && !isBootstrapTrigger) {
            logPerf('visualizer:updateSkippedDuplicateStartup', {
                triggerSource,
                reason: 'bootstrapPending',
            });
            return;
        }

        const key = currentUpdateKey();
        if (inFlightUpdate && inFlightUpdate.key === key) {
            logPerf('visualizer:updateJoined', {
                triggerSource,
                joinedTriggerSource: inFlightUpdate.triggerSource,
                isBootstrap: inFlightUpdate.isBootstrap,
            });
            return await inFlightUpdate.promise;
        }

        // During session restore the panel may exist but not yet be considered
        // visible. Allow forced updates (initial render / webviewReady) to run
        // so the visualizer doesn't get stuck in the loading state.
        if (!panel.visible && !forceUpdate) {
            setNeedsUpdateWhenVisible(true);
            return;
        }

        const document = getDocument();
        const content = document.getText();
        const contentHash = hashContent(content);

        if (!forceUpdate && contentHash === getLastContentHash()) {
            return;
        }
        logPerf('visualizer:updateRequested', {
            triggerSource,
            forceUpdate,
            isBootstrap: !bootstrapCompleted,
            currentView: getCurrentView(),
        });
        setLastContentHash(contentHash);
        const promise = (async () => {
            logPerf('visualizer:updateStarted', {
                triggerSource,
                isBootstrap: !bootstrapCompleted,
                currentView: getCurrentView(),
            });
            panel.webview.postMessage({ command: 'showLoading', message: 'Parsing SysML model...' });
            await new Promise(resolve => setTimeout(resolve, 0));
            const startedAt = Date.now();
            try {
                await doUpdateVisualization();
                bootstrapCompleted = true;
                logPerf('visualizer:updateCompleted', {
                    triggerSource,
                    isBootstrap: isBootstrapTrigger,
                    currentView: getCurrentView(),
                    totalMs: Date.now() - startedAt,
                });
            } finally {
                if (inFlightUpdate?.key === key) {
                    inFlightUpdate = undefined;
                }
            }
        })();
        inFlightUpdate = {
            key,
            triggerSource,
            isBootstrap: !bootstrapCompleted,
            promise,
        };
        await promise;
    }

    return { update };
}
