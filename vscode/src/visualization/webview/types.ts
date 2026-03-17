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
    activityDebugLabels: boolean;
    stateLayoutOrientation: string;
    selectedDiagramIndex: number;
    postMessage: (msg: unknown) => void;
    onStartInlineEdit: (nodeG: any, elementName: string, x: number, y: number, width: number) => void;
    renderPlaceholder: (width: number, height: number, viewName: string, message: string, data: any) => void;
    clearVisualHighlights: () => void;
}

export type PostMessageFn = (msg: unknown) => void;
