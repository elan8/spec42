import {
    prepareViewData,
    type PreparedNode,
    type PreparedView
} from "@spec42/diagram-renderer/prepare";
import {
    renderVisualization,
    type RenderController
} from "@spec42/diagram-renderer/renderer";
import {
    jumpPayloadFromNode,
    nodeSupportsSourceNavigation,
} from "@spec42/diagram-renderer/behavior-interaction";

/**
 * Thin adapter around the shared renderer package to keep Spec42's
 * webview protocol and orchestration untouched during incremental adoption.
 */
export function prepareSharedViewData(payload: unknown): PreparedView {
    return prepareViewData(payload);
}

export interface SharedRenderAdapterOptions {
    selectedNodeId?: string | null;
    onNodeNavigate?: (node: PreparedNode) => void;
}

export interface SharedRenderAdapterController {
    reset: () => void;
    exportSvg: () => string;
    destroy: () => void;
    getFitTransform: () => { toString: () => string };
}

export { jumpPayloadFromNode, nodeSupportsSourceNavigation };

export async function renderSharedView(
    target: HTMLElement,
    prepared: PreparedView,
    options: SharedRenderAdapterOptions = {}
): Promise<SharedRenderAdapterController> {
    const controller: RenderController = await renderVisualization(target, prepared, {
        selectedNodeId: options.selectedNodeId ?? null,
        onNodeClick: options.onNodeNavigate,
        theme: { colorScheme: "vscode" },
        delegateZoom: true,
    });

    return {
        reset: () => controller.reset(),
        exportSvg: () => controller.exportSvg(),
        destroy: () => controller.destroy(),
        getFitTransform: () => controller.getFitTransform(),
    };
}
