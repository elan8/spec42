/**
 * Adapts LSP visualization DTOs for the shared diagram renderer.
 * When `preparedView` is present, shared prepare consumes it directly and skips semantic normalization.
 */

export function buildSharedRendererInput(
    data: object | null | undefined,
    view: string,
): Record<string, unknown> | null {
    if (!data) {
        return null;
    }
    const record = data as Record<string, unknown>;
    if (view === "interconnection-view" && isPreparedView(record.preparedView)) {
        return {
            view,
            currentView: record.currentView,
            selectedView: record.selectedView,
            selectedViewName: record.selectedViewName,
            emptyStateMessage: record.emptyStateMessage,
            viewCandidates: record.viewCandidates,
            preparedView: record.preparedView,
        };
    }
    if (view === "interconnection-view") {
        return {
            view,
            currentView: record.currentView,
            selectedView: record.selectedView,
            selectedViewName: record.selectedViewName,
            emptyStateMessage: record.emptyStateMessage,
            viewCandidates: record.viewCandidates,
            preparedView: {
                title: String(record.selectedViewName ?? "Interconnection View"),
                view,
                nodes: [],
                edges: [],
                meta: { missingPreparedView: true },
            },
        };
    }
    return {
        ...data,
        view,
    };
}

export function interconnectionBannerCounts(data: object | null | undefined): {
    partCount: number;
    connectorCount: number;
} {
    const record = data as Record<string, unknown> | null | undefined;
    const preparedView = record?.preparedView;
    if (isPreparedView(preparedView)) {
        return {
            partCount: preparedView.nodes.length,
            connectorCount: preparedView.edges.length,
        };
    }
    return {
        partCount: 0,
        connectorCount: 0,
    };
}

function isPreparedView(value: unknown): value is { nodes: unknown[]; edges: unknown[] } {
    return typeof value === "object"
        && value !== null
        && Array.isArray((value as { nodes?: unknown[] }).nodes)
        && Array.isArray((value as { edges?: unknown[] }).edges);
}
