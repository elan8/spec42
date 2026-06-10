export interface VisualizationEmptyStateOptions {
    viewLabel?: string;
    data?: {
        elements?: unknown[];
        graph?: { nodes?: unknown[] };
        viewCandidates?: unknown[];
        selectedViewName?: string | null;
    } | null;
}

export interface EmptyStateTitleContext {
    viewCandidates?: unknown[];
    selectedViewName?: string | null;
    rendererViewLabel?: string | null;
}

export function resolveEmptyStateTitle(context: EmptyStateTitleContext = {}): string {
    const candidates = Array.isArray(context.viewCandidates) ? context.viewCandidates : [];
    if (candidates.length === 0) {
        return 'No views defined';
    }
    const selectedViewName = context.selectedViewName?.trim();
    if (selectedViewName) {
        return selectedViewName;
    }
    const rendererViewLabel = context.rendererViewLabel?.trim();
    if (rendererViewLabel) {
        return rendererViewLabel;
    }
    return 'Visualizer';
}

function escapeHtml(text: string): string {
    return text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}

function modelElementCount(data: VisualizationEmptyStateOptions['data']): number {
    if (!data) {
        return 0;
    }
    if (Array.isArray(data.elements) && data.elements.length > 0) {
        return data.elements.length;
    }
    if (Array.isArray(data.graph?.nodes)) {
        return data.graph.nodes.length;
    }
    return 0;
}

export function buildVisualizationEmptyStateHtml(
    message: string,
    options: VisualizationEmptyStateOptions = {},
): string {
    const data = options.data;
    const elementCount = modelElementCount(data);
    const hasModelContent = elementCount > 0;
    const recoveryHint = hasModelContent
        ? 'Try another visualizer view or open the source element.'
        : 'Define a SysML view with expose (and optional filter), or start from the recommended example in the Spec42 sidebar.';
    const viewLabel = options.viewLabel || 'Visualizer';
    const elementNote = hasModelContent
        ? `<p style="font-size:11px;margin-top:12px;opacity:0.8">${elementCount} element(s) in model</p>`
        : '';

    return `<div class="visualizer-empty-state" style="display:flex;align-items:center;justify-content:center;height:100%;min-height:240px;padding:2em;box-sizing:border-box">
      <div style="max-width:360px;text-align:center;line-height:1.45;color:var(--vscode-descriptionForeground)">
        <p style="font-size:18px;font-weight:600;color:var(--vscode-editor-foreground);margin:0 0 12px">${escapeHtml(viewLabel)}</p>
        <p style="margin:0 0 8px;font-size:13px">${escapeHtml(message)}</p>
        <p style="margin:0;font-size:12px;opacity:0.9">${escapeHtml(recoveryHint)}</p>
        ${elementNote}
      </div>
    </div>`;
}
