/* eslint-disable @typescript-eslint/no-explicit-any */
import type { Selection, ZoomBehavior } from 'd3';
import type { RenderOutcome } from '../renderContract';
import type { RenderScheduler } from './renderScheduler';
import type { createExportHandler } from './export';

export type SharedRenderController = {
    reset: () => void;
    exportSvg: () => string;
    destroy: () => void;
    getFitTransform: () => { toString: () => string };
};

export type ExportHandler = ReturnType<typeof createExportHandler>;

export interface VisualizerContext {
    vscode: { postMessage: (msg: unknown) => void };
    experimentalViews: Set<string>;
    currentData: any;
    currentView: string;
    selectedDiagramIndex: number;
    selectedDiagramId: string | null;
    selectedDiagramName: string | null;
    selectedDiagramPackagePath: string | null;
    lastView: string;
    svg: Selection<SVGSVGElement, unknown, null, undefined> | null;
    g: Selection<SVGGElement, unknown, null, undefined> | null;
    zoom: ZoomBehavior<SVGSVGElement, unknown> | null;
    layoutDirection: string;
    activityLayoutDirection: string;
    stateLayoutOrientation: string;
    filteredData: any;
    showMetadata: boolean;
    showCategoryHeaders: boolean;
    vizElement: HTMLElement | null;
    sharedRenderController: SharedRenderController | null;
    renderScheduler: RenderScheduler;
    exportHandler: ExportHandler;
    verboseWebviewLogging: boolean;
    resizeTimeout: ReturnType<typeof setTimeout> | undefined;
    lastRenderedWidth: number;
    lastRenderedHeight: number;
    lastUpdateId: string | null;
    lastRenderOutcome: RenderOutcome | null;
    lastRenderHasExportableSvg: boolean;

    webviewPerf: (event: string, data?: Record<string, unknown>) => void;
    webviewLog: (level: 'info' | 'warn' | 'error', ...args: any[]) => void;
    showLoading: (message?: string) => void;
    hideLoading: () => void;
    logSelectionTransition: (
        step: string,
        before: { name: any; index: number },
        extra?: Record<string, any>,
    ) => void;
    updateDimensionsDisplay: () => void;
    renderVisualization: (
        view: string,
        preserveZoomOverride?: unknown,
        allowDuringResize?: boolean,
    ) => Promise<void>;
    changeView: (view: string) => void;
    isExperimentalRendererView: (viewId: string | null | undefined) => boolean;
    resolveActiveRendererView: (activeView: string) => string;
}
