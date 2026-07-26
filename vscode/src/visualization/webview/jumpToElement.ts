/**
 * Unified jumpToElement helper. Ensures elementQualifiedName (id) is always sent when available
 * for accurate findElement lookup.
 */

export interface JumpToElementPayload {
    name: string;
    id?: string;
    uri?: string;
    range?: {
        start: { line: number; character: number };
        end: { line: number; character: number };
    };
}

export interface JumpToElementOptions {
    skipCentering?: boolean;
    parentContext?: string;
}

/**
 * Post a jumpToElement message. Always sends elementQualifiedName when element.id is present.
 */
export function postJumpToElement(
    postMessage: (msg: unknown) => void,
    element: JumpToElementPayload,
    options?: JumpToElementOptions
): void {
    const msg: Record<string, unknown> = {
        command: 'jumpToElement',
        elementName: element.name,
    };
    if (element.id) {
        msg.elementQualifiedName = element.id;
    }
    if (element.uri) {
        msg.elementUri = element.uri;
    }
    if (element.range) {
        msg.elementRange = element.range;
    }
    if (options?.skipCentering) {
        msg.skipCentering = true;
    }
    if (options?.parentContext) {
        msg.parentContext = options.parentContext;
    }
    postMessage(msg);
}

/**
 * Post an inspectElement message so the Feature Inspector view can pin to the clicked node,
 * independent of whether jumpToElement can also resolve a source location for it.
 */
export function postInspectElement(
    postMessage: (msg: unknown) => void,
    element: Pick<JumpToElementPayload, 'uri' | 'range'>
): void {
    if (!element.uri || !element.range) {
        return;
    }
    postMessage({
        command: 'inspectElement',
        elementUri: element.uri,
        elementRange: element.range,
    });
}
