/**
 * Types for the visualizer webview. RenderContext is passed to renderers.
 */

/* eslint-disable @typescript-eslint/no-explicit-any */

export interface RenderContext {
    width: number;
    height: number;
    svg: any;
    g: any;
    zoom: any;
    getCy: () => any;
    layoutDirection: string;
    activityLayoutDirection: string;
    stateLayoutOrientation: string;
    selectedDiagramIndex: number;
    selectedDiagramId?: string | null;
    postMessage: (msg: unknown) => void;
    onStartInlineEdit: (nodeG: any, elementName: string, x: number, y: number, width: number) => void;
    renderPlaceholder: (width: number, height: number, viewName: string, message: string, data: any) => void;
    clearVisualHighlights: () => void;
    abortSignal?: AbortSignal;
}

export type PostMessageFn = (msg: unknown) => void;
