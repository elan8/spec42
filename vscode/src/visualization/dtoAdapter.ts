/**
 * Adapts LSP visualization DTOs for the shared diagram renderer.
 * Payload normalization runs in shared prepareViewData via normalizeVisualizationPayload.
 */

export function buildSharedRendererInput(
    data: Record<string, unknown> | null | undefined,
    view: string,
): Record<string, unknown> | null {
    if (!data) {
        return null;
    }
    return {
        ...data,
        view,
    };
}

export function interconnectionBannerCounts(data: Record<string, unknown> | null | undefined): {
    partCount: number;
    connectorCount: number;
} {
    const ibd = data?.ibd as { parts?: unknown[]; connectors?: unknown[] } | undefined;
    return {
        partCount: Array.isArray(ibd?.parts) ? ibd.parts.length : 0,
        connectorCount: Array.isArray(ibd?.connectors) ? ibd.connectors.length : 0,
    };
}
