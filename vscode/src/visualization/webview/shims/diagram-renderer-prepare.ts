export interface PreparedNode {
    id: string;
    label: string;
    kind: string;
    sourcePath?: string | null;
    uri?: string | null;
    range?: unknown;
    attributes?: Record<string, unknown>;
}

export interface PreparedView {
    title?: string;
    view: string;
    nodes: PreparedNode[];
    edges: unknown[];
    meta?: Record<string, unknown>;
}

export function prepareViewData(visualizationInput: unknown): PreparedView {
    return visualizationInput as PreparedView;
}
