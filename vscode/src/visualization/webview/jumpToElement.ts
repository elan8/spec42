/**
 * Unified jumpToElement helper. Ensures elementQualifiedName (id) is always sent when available
 * for accurate findElement lookup.
 */

export interface JumpToElementPayload {
    name: string;
    id?: string;
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
    if (options?.skipCentering) {
        msg.skipCentering = true;
    }
    if (options?.parentContext) {
        msg.parentContext = options.parentContext;
    }
    postMessage(msg);
}
