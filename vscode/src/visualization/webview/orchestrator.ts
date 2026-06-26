/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
// Orchestrator: webview bootstrap wiring focused modules together.

import { GRADUATED_BEHAVIOR_VIEWS, STRUCTURAL_VIEWS } from './constants';
import { createExportHandler } from './export';
import { registerMessageRouter } from './messageRouter';
import { renderVisualization as renderVisualizationPipeline } from './renderPipeline';
import { RenderScheduler } from './renderScheduler';
import { installGlobalCommandBindings } from './globalCommands';
import { setupVisualizerControls } from './uiControls';
import type { VisualizerContext } from './visualizerContext';
import {
    getNextLayoutDirection,
    populateViewDropdown,
    updateActiveViewButton,
    updateLayoutDirectionButton,
} from './viewControls';
import { resetZoom, setupSharedRendererWheelCapture, zoomToFit } from './zoomController';

export function initializeOrchestrator(api: { postMessage: (msg: unknown) => void }): void {
    const experimentalViews = new Set(
        (
            Array.isArray(typeof window !== 'undefined' && window.__VIZ_INIT?.experimentalViews)
                ? window.__VIZ_INIT.experimentalViews
                : []
        ).filter((viewId: string) => !GRADUATED_BEHAVIOR_VIEWS.has(viewId)),
    );

    const verboseWebviewLogging = Boolean(
        typeof window !== 'undefined' && window.__VIZ_INIT?.verboseLogging,
    );

    function webviewPerf(event: string, data?: Record<string, unknown>) {
        try {
            if (ctx.vscode && typeof ctx.vscode.postMessage === 'function') {
                ctx.vscode.postMessage({ command: 'webviewPerf', event, data });
            }
        } catch {
            // ignore
        }
    }

    const renderScheduler = new RenderScheduler(webviewPerf);

    const ctx: VisualizerContext = {
        vscode: api,
        experimentalViews,
        currentData: null,
        currentView: 'general-view',
        selectedDiagramIndex: 0,
        selectedDiagramId: null,
        selectedDiagramName: null,
        selectedDiagramPackagePath: null,
        lastView: 'general-view',
        svg: null,
        g: null,
        zoom: null,
        layoutDirection: 'horizontal',
        activityLayoutDirection: 'vertical',
        stateLayoutOrientation: 'force',
        filteredData: null,
        showMetadata: false,
        showCategoryHeaders: true,
        vizElement: null,
        sharedRenderController: null,
        renderScheduler,
        exportHandler: null as any,
        verboseWebviewLogging,
        resizeTimeout: undefined,
        lastRenderedWidth: 0,
        lastRenderedHeight: 0,
        lastUpdateId: null,
        lastRenderOutcome: null,
        lastRenderHasExportableSvg: false,
        webviewPerf,
        webviewLog: () => {},
        showLoading: () => {},
        hideLoading: () => {},
        logSelectionTransition: () => {},
        updateDimensionsDisplay: () => {},
        renderVisualization: async () => {},
        changeView: () => {},
        isExperimentalRendererView: () => false,
        resolveActiveRendererView: () => 'general-view',
    };

    ctx.exportHandler = createExportHandler({
        getCurrentData: () => ctx.currentData,
        getViewState: () => ({ currentView: ctx.currentView }),
        postMessage: (msg) => ctx.vscode.postMessage(msg),
        getExportSvg: () => ctx.sharedRenderController?.exportSvg() ?? null,
        onPerformance: (event, data) => ctx.webviewPerf(event, data),
    });

    function showLoading(message = 'Rendering diagram...') {
        const overlay = document.getElementById('loading-overlay');
        const textEl = overlay?.querySelector('.loading-text');
        if (overlay) {
            if (textEl) textEl.textContent = message;
            overlay.classList.remove('hidden');
        }
        document.body.style.cursor = 'wait';
    }

    function hideLoading() {
        const overlay = document.getElementById('loading-overlay');
        if (overlay) {
            overlay.classList.add('hidden');
        }
        document.body.style.cursor = '';
    }

    function webviewLog(level: 'info' | 'warn' | 'error', ...args: any[]) {
        if (level === 'info' && !verboseWebviewLogging) {
            return;
        }
        try {
            if (ctx.vscode && typeof ctx.vscode.postMessage === 'function') {
                ctx.vscode.postMessage({ command: 'webviewLog', level, args });
            }
        } catch {
            // ignore
        }
    }

    function logSelectionTransition(
        step: string,
        before: { name: any; index: number },
        extra?: Record<string, any>,
    ) {
        webviewLog('info', '[GENERAL][selection-transition]', {
            step,
            beforeName: before.name,
            beforeIndex: before.index,
            afterName: ctx.selectedDiagramName,
            afterIndex: ctx.selectedDiagramIndex,
            ...(extra || {}),
        });
    }

    function isExperimentalRendererView(viewId: string | null | undefined): boolean {
        if (!viewId || GRADUATED_BEHAVIOR_VIEWS.has(viewId)) {
            return false;
        }
        return experimentalViews.has(viewId);
    }

    function resolveActiveRendererView(activeView: string): string {
        const selectedCandidate = Array.isArray(ctx.currentData?.viewCandidates)
            ? ctx.currentData.viewCandidates.find(
                  (candidate: any) => candidate.id === ctx.currentData?.selectedView,
              )
            : null;
        const candidateRenderer = selectedCandidate?.supported ? selectedCandidate?.rendererView : null;
        return typeof candidateRenderer === 'string' && candidateRenderer.length > 0
            ? candidateRenderer
            : activeView;
    }

    function updateActivityDebugButtonVisibility(view: string) {
        const legendBtn = document.getElementById('legend-btn');
        const legendPopup = document.getElementById('legend-popup');
        if (legendBtn) {
            const cytoscapeViews = ['general', 'general-view'];
            legendBtn.style.display = cytoscapeViews.includes(view) ? 'inline-block' : 'none';
            if (!cytoscapeViews.includes(view) && legendPopup) {
                legendPopup.style.display = 'none';
                legendBtn.classList.remove('active');
                legendBtn.style.background = '';
                legendBtn.style.color = '';
            }
        }
    }

    function updateDimensionsDisplay() {
        const vizElement = document.getElementById('visualization');
        if (vizElement) {
            const width = Math.round(vizElement.clientWidth);
            const height = Math.round(vizElement.clientHeight);
            const statusText = document.getElementById('status-text');
            if (statusText) {
                statusText.innerHTML =
                    'Panel: ' + width + ' x ' + height + 'px - Resize via VS Code panel';
                const statusBar = document.getElementById('status-bar');
                if (statusBar) statusBar.style.display = 'flex';
                setTimeout(() => {
                    if (statusText.innerHTML?.includes('Panel:')) {
                        statusText.textContent = 'Ready';
                    }
                }, 3000);
            }
        }
    }

    function cancelOutstandingRenderRequests(reason = 'view-switch') {
        renderScheduler.cancelOutstandingRenderRequests(reason);
    }

    function shouldAnimateStructuralTransition(nextView: string) {
        return (
            STRUCTURAL_VIEWS.has(ctx.lastView) &&
            STRUCTURAL_VIEWS.has(nextView) &&
            nextView !== ctx.lastView
        );
    }

    function animateStructuralTransition(callback: () => void) {
        const viz = document.getElementById('visualization');
        if (!viz) {
            callback();
            return;
        }

        viz.classList.add('structural-transition-active', 'fade-out');

        setTimeout(() => {
            callback();

            requestAnimationFrame(() => {
                viz.classList.remove('fade-out');
                viz.classList.add('fade-in');

                setTimeout(() => {
                    viz.classList.remove('fade-in', 'structural-transition-active');
                }, 350);
            });
        }, 220);
    }

    function changeView(view: string) {
        clearTimeout(ctx.resizeTimeout);
        cancelOutstandingRenderRequests('view-switch');

        window.userHasManuallyZoomed = false;

        const proceedWithRender = () => {
            ctx.currentView = view;

            const before = { name: ctx.selectedDiagramName, index: ctx.selectedDiagramIndex };
            ctx.selectedDiagramIndex = 0;
            logSelectionTransition('changeView.resetIndex', before, { view });

            ctx.vscode.postMessage({
                command: 'viewChanged',
                view: view,
            });

            updateActiveViewButton(ctx, view);
            updateActivityDebugButtonVisibility(view);

            renderScheduler.setPendingViewRenderTimeout(
                setTimeout(() => {
                    renderScheduler.clearPendingViewRenderTimeout();
                    void ctx.renderVisualization(view);
                }, 50),
            );

            ctx.lastView = view;
        };

        if (shouldAnimateStructuralTransition(view)) {
            animateStructuralTransition(proceedWithRender);
        } else {
            proceedWithRender();
        }
    }

    function toggleLayoutDirection() {
        if (ctx.currentView === 'action-flow-view') {
            ctx.activityLayoutDirection = getNextLayoutDirection(ctx.activityLayoutDirection);
        } else {
            ctx.layoutDirection = getNextLayoutDirection(ctx.layoutDirection);
        }
        updateLayoutDirectionButton(ctx, ctx.currentView);
        void ctx.renderVisualization(ctx.currentView);
    }

    function handleResize() {
        const vizElement = document.getElementById('visualization');
        if (!vizElement) return;

        const currentWidth = vizElement.clientWidth;
        const currentHeight = vizElement.clientHeight;

        clearTimeout(ctx.resizeTimeout);
        updateDimensionsDisplay();

        ctx.resizeTimeout = setTimeout(() => {
            if (currentWidth !== ctx.lastRenderedWidth || currentHeight !== ctx.lastRenderedHeight) {
                ctx.lastRenderedWidth = currentWidth;
                ctx.lastRenderedHeight = currentHeight;

                if (ctx.currentData && !renderScheduler.isRendering) {
                    void ctx.renderVisualization(ctx.currentView, null, true);
                }
            }
        }, 500);
    }

    ctx.webviewLog = webviewLog;
    ctx.showLoading = showLoading;
    ctx.hideLoading = hideLoading;
    ctx.logSelectionTransition = logSelectionTransition;
    ctx.updateDimensionsDisplay = updateDimensionsDisplay;
    ctx.renderVisualization = (view, preserveZoomOverride, allowDuringResize) =>
        renderVisualizationPipeline(ctx, view, preserveZoomOverride, allowDuringResize);
    ctx.changeView = changeView;
    ctx.isExperimentalRendererView = isExperimentalRendererView;
    ctx.resolveActiveRendererView = resolveActiveRendererView;

    window.userHasManuallyZoomed = false;

    window.addEventListener('error', (e) => {
        console.error('JavaScript Error:', e.error?.message || e.message);
    });

    registerMessageRouter(ctx);

    window.addEventListener('keydown', (event) => {
        if (event.ctrlKey && event.key === 'd') {
            event.preventDefault();
            updateDimensionsDisplay();
        }
    });

    if (window.ResizeObserver) {
        const resizeObserver = new ResizeObserver((entries) => {
            requestAnimationFrame(() => {
                for (const entry of entries) {
                    if (entry.target.id === 'visualization') {
                        handleResize();
                        break;
                    }
                }
            });
        });

        setTimeout(() => {
            const visualizationElement = document.getElementById('visualization');
            if (visualizationElement) {
                ctx.lastRenderedWidth = visualizationElement.clientWidth;
                ctx.lastRenderedHeight = visualizationElement.clientHeight;
                resizeObserver.observe(visualizationElement);
            }
        }, 100);
    }

    window.addEventListener('resize', () => {
        requestAnimationFrame(() => {
            handleResize();
        });
    });

    const resetZoomBound = () => resetZoom(ctx);
    const zoomToFitBound = () => zoomToFit(ctx);

    installGlobalCommandBindings(ctx, {
        changeView,
        resetZoom: resetZoomBound,
        zoomToFit: zoomToFitBound,
    });

    setupVisualizerControls({
        getCurrentView: () => ctx.currentView,
        resetZoom: resetZoomBound,
        toggleLayoutDirection,
        populateViewDropdown: () => populateViewDropdown(ctx),
        updateActiveViewButton: (view) => updateActiveViewButton(ctx, view),
        exportPNG: (scale) => ctx.exportHandler.exportPNG(scale),
        exportSVG: () => ctx.exportHandler.exportSVG(),
        exportJSON: () => ctx.exportHandler.exportJSON(),
    });

    setupSharedRendererWheelCapture();
    webviewPerf('visualizer:webviewInitialized');
    ctx.vscode.postMessage({ command: 'webviewReady' });
}
