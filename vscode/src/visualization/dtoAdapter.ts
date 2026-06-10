/**
 * Adapts LSP visualization DTOs for the shared diagram renderer.
 * Centralizes the legacy prepareDataForView merge until behavior shaping moves fully into shared/prepare.ts.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import { prepareDataForView } from './prepareData';

export function buildSharedRendererInput(data: Record<string, any> | null | undefined, view: string): Record<string, unknown> | null {
    if (!data) {
        return null;
    }
    const prepared = prepareDataForView(data, view) as Record<string, unknown>;
    return {
        ...data,
        ...prepared,
        view,
        activityDiagrams:
            (prepared.diagrams as unknown[] | undefined) ??
            data.activityDiagrams,
        sequenceDiagrams:
            (prepared.diagrams as unknown[] | undefined) ??
            data.sequenceDiagrams,
        stateMachines:
            (prepared.stateMachines as unknown[] | undefined) ??
            data.stateMachines,
    };
}

export function interconnectionBannerCounts(data: Record<string, any> | null | undefined): {
    partCount: number;
    connectorCount: number;
} {
    const ibd = data?.ibd as { parts?: unknown[]; connectors?: unknown[] } | undefined;
    if (ibd) {
        return {
            partCount: Array.isArray(ibd.parts) ? ibd.parts.length : 0,
            connectorCount: Array.isArray(ibd.connectors) ? ibd.connectors.length : 0,
        };
    }
    const prepared = prepareDataForView(data ?? null, 'interconnection-view');
    return {
        partCount: Array.isArray(prepared?.parts) ? prepared.parts.length : 0,
        connectorCount: Array.isArray(prepared?.connectors) ? prepared.connectors.length : 0,
    };
}
