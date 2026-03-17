/**
 * Placeholder renderer - shows a message when no diagram is available.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

export function renderPlaceholderView(
    _width: number,
    _height: number,
    _data: any,
    message?: string,
): void {
    const container = document.getElementById('visualization');
    if (!container) return;
    const msg = message || 'No diagram to display. Select a view or open a SysML file.';
    container.innerHTML = `<p style="padding:2em;color:var(--vscode-descriptionForeground);text-align:center">${msg}</p>`;
}
