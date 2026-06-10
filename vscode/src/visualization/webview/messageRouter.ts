/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { quickHash } from './shared';
import { highlightElementInVisualization } from './selectionSync';
import { destroySharedRenderController } from './zoomController';
import type { VisualizerContext } from './visualizerContext';
import { updateActiveViewButton } from './viewControls';

export function registerMessageRouter(ctx: VisualizerContext): void {
    window.addEventListener('message', (event) => {
        const message = event.data;
        switch (message.command) {
            case 'showLoading':
                ctx.showLoading(message.message || 'Parsing SysML model...');
                break;
            case 'hideLoading':
                ctx.hideLoading();
                break;
            case 'modelNotReady':
                ctx.showLoading(message.message || 'Waiting for SysML model...');
                destroySharedRenderController(ctx);
                if (ctx.vizElement) {
                    ctx.vizElement.innerHTML = '';
                }
                ctx.currentData = null;
                ctx.filteredData = null;
                ctx.renderScheduler.dataHash = '';
                break;
            case 'update':
                ctx.webviewPerf('visualizer:webviewUpdateReceived', {
                    currentView: message.currentView || ctx.currentView,
                    graphNodes: message.graph?.nodes?.length || 0,
                    graphEdges: message.graph?.edges?.length || 0,
                    viewCandidates: message.viewCandidates?.length || 0,
                });
                {
                    const newHash = quickHash({
                        graph: message.graph,
                        generalViewGraph: message.generalViewGraph,
                        ibd: message.ibd,
                        selectedView: message.selectedView,
                        emptyStateMessage: message.emptyStateMessage,
                        currentView: message.currentView,
                    });

                    if (message.modelReady === false) {
                        ctx.showLoading(message.modelStatusMessage || 'Waiting for SysML model...');
                        destroySharedRenderController(ctx);
                        if (ctx.vizElement) {
                            ctx.vizElement.innerHTML = '';
                        }
                        ctx.currentData = null;
                        ctx.filteredData = null;
                        ctx.renderScheduler.dataHash = '';
                        return;
                    }

                    if (newHash === ctx.renderScheduler.dataHash && ctx.currentData) {
                        ctx.webviewPerf('visualizer:webviewUpdateSkippedUnchanged', {
                            currentView: ctx.currentView,
                        });
                        ctx.hideLoading();
                        return;
                    }
                    ctx.renderScheduler.dataHash = newHash;

                    ctx.showLoading('Rendering diagram...');

                    ctx.currentData = message;
                    ctx.filteredData = null;
                    if (message.currentView) {
                        ctx.currentView = message.currentView;
                    }
                    ctx.webviewLog('info', '[GENERAL][update-message]', {
                        incomingView: message.currentView || null,
                        selectedViewName: message.selectedViewName || null,
                        graphNodes: message.graph?.nodes?.length || 0,
                        graphEdges: message.graph?.edges?.length || 0,
                        viewCandidates: message.viewCandidates?.length || 0,
                    });

                    updateActiveViewButton(ctx, ctx.currentView);
                    void ctx.renderVisualization(ctx.currentView).catch((e) => {
                        console.error('Error in renderVisualization:', e);
                        ctx.hideLoading();
                    });
                }
                break;
            case 'changeView':
                if (message.view) {
                    ctx.changeView(message.view);
                }
                break;
            case 'selectPackage':
                if (message.packageName) {
                    const before = {
                        name: ctx.selectedDiagramName,
                        index: ctx.selectedDiagramIndex,
                    };
                    ctx.selectedDiagramId = message.packageName;
                    ctx.selectedDiagramName = message.packageName;
                    ctx.selectedDiagramPackagePath = null;
                    ctx.selectedDiagramIndex = 0;
                    ctx.logSelectionTransition('message.selectPackage', before, {
                        packageName: message.packageName,
                    });
                    ctx.changeView('general-view');
                }
                break;
            case 'setRequirementsVisibleForTest':
                if (ctx.currentView === 'general-view') {
                    void ctx.renderVisualization('general-view', false);
                }
                break;
            case 'export':
                if (message.format === 'png') {
                    ctx.exportHandler.exportPNG(message.scale || 2);
                } else if (message.format === 'svg') {
                    ctx.exportHandler.exportSVG();
                }
                break;
            case 'highlightElement':
                highlightElementInVisualization(ctx, message.elementName, message.skipCentering);
                break;
            case 'requestCurrentView':
                ctx.vscode.postMessage({
                    command: 'currentViewResponse',
                    view: ctx.currentView,
                });
                break;
            case 'exportDiagramForTest': {
                const maxAttempts = 120;
                let attempts = 0;
                const tryExportWhenReady = () => {
                    const exportPreview = ctx.exportHandler.getSvgStringForExport();
                    const hasExportableSvg =
                        typeof exportPreview === 'string' && exportPreview.includes('<svg');
                    const hasContent = (() => {
                        if (hasExportableSvg) {
                            return true;
                        }
                        const svgElement = document.querySelector('#visualization svg');
                        const groupElement = svgElement?.querySelector('g');
                        return !!(svgElement && groupElement && groupElement.childElementCount > 0);
                    })();
                    if (!ctx.renderScheduler.isRendering && !hasContent && ctx.currentData) {
                        void ctx.renderVisualization(ctx.currentView);
                    }
                    if ((ctx.renderScheduler.isRendering || !hasContent) && attempts < maxAttempts) {
                        attempts += 1;
                        setTimeout(tryExportWhenReady, 150);
                        return;
                    }
                    const svgString = exportPreview ?? ctx.exportHandler.getSvgStringForExport();
                    ctx.vscode.postMessage({
                        command: 'testDiagramExported',
                        viewId: ctx.currentView,
                        svgString: svgString ?? '',
                    });
                };
                tryExportWhenReady();
                break;
            }
        }
    });
}
