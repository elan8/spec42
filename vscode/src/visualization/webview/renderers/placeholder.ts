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
    const msg = message || 'No diagram to display. Open a SysML file, choose another view, or start from the recommended example in the Spec42 sidebar.';
    container.innerHTML = `<div style="padding:2em;color:var(--vscode-descriptionForeground);text-align:center;line-height:1.45">
        <p>${msg}</p>
        <p style="font-size:12px">Use the status bar for quick actions: Examples, Visualizer, Output, and Restart Server.</p>
    </div>`;
}
