import { resolveEmptyStateTitle } from '../emptyStateContent';
import { VIEW_OPTIONS } from './constants';

export function resolveEmptyStateMessage(data: unknown): string | null {
    const message = (data as { emptyStateMessage?: unknown } | null | undefined)?.emptyStateMessage;
    if (typeof message !== 'string') {
        return null;
    }
    const trimmed = message.trim();
    return trimmed.length > 0 ? trimmed : null;
}

export function emptyStateTitleForData(data: unknown, rendererView: string): string {
    const payload = data as {
        viewCandidates?: unknown[];
        selectedViewName?: string | null;
    } | null | undefined;
    return resolveEmptyStateTitle({
        viewCandidates: payload?.viewCandidates,
        selectedViewName: payload?.selectedViewName,
        rendererViewLabel: VIEW_OPTIONS[rendererView]?.label || null,
    });
}
