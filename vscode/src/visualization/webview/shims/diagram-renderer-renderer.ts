import type { PreparedView } from './diagram-renderer-prepare';

export interface RenderController {
    reset: () => void;
    exportSvg: () => string;
    destroy: () => void;
    getFitTransform: () => { toString: () => string };
}

export async function renderVisualization(
    _target: HTMLElement,
    _prepared: PreparedView,
    _options?: Record<string, unknown>,
): Promise<RenderController> {
    throw new Error('diagram-renderer shim is for typechecking only');
}
