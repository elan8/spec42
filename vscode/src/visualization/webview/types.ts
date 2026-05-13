import type { VisualizationDataDto } from '../visualizationTypes';

/**
 * Types for the visualizer webview. RenderContext is passed to renderers.
 */

type D3SelectionLike = unknown;
type D3ZoomLike = unknown;
type CytoscapeLike = unknown;

export interface RenderContext {
    width: number;
    height: number;
    svg: D3SelectionLike;
    g: D3SelectionLike;
    zoom: D3ZoomLike;
    getCy: () => CytoscapeLike;
    layoutDirection: string;
    activityLayoutDirection: string;
    stateLayoutOrientation: string;
    selectedDiagramIndex: number;
    selectedDiagramId?: string | null;
    postMessage: (msg: unknown) => void;
    onStartInlineEdit: (nodeG: D3SelectionLike, elementName: string, x: number, y: number, width: number) => void;
    renderPlaceholder: (width: number, height: number, viewName: string, message: string, data: VisualizationDataDto | null) => void;
    clearVisualHighlights: () => void;
    abortSignal?: AbortSignal;
}

export type PostMessageFn = (msg: unknown) => void;
