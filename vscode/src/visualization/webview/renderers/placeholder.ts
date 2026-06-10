/**
 * Placeholder renderer - shows guidance when no diagram is available.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import { buildVisualizationEmptyStateHtml } from '../../emptyStateContent';

export { buildVisualizationEmptyStateHtml } from '../../emptyStateContent';
export type { VisualizationEmptyStateOptions } from '../../emptyStateContent';

export function renderVisualizationEmptyState(
    message: string,
    options: Parameters<typeof buildVisualizationEmptyStateHtml>[1] = {},
): void {
    const doc = (globalThis as { document?: { getElementById: (id: string) => HTMLElement | null } }).document;
    const container = doc?.getElementById('visualization');
    if (!container) {
        return;
    }
    container.innerHTML = buildVisualizationEmptyStateHtml(message, options);
}

/** @deprecated Use renderVisualizationEmptyState */
export function renderPlaceholderView(
    _width: number,
    _height: number,
    _data: any,
    message?: string,
): void {
    renderVisualizationEmptyState(
        message || 'No diagram to display. Open a SysML file, choose another view, or start from the recommended example in the Spec42 sidebar.',
    );
}
