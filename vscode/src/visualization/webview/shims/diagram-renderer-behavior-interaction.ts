import type { PreparedNode } from './diagram-renderer-prepare';

export function jumpPayloadFromNode(node: PreparedNode, parentContext?: string): {
    name: string;
    id?: string;
    uri?: string;
    range?: unknown;
    parentContext?: string;
} {
    return { name: node.label, parentContext };
}

export function nodeSupportsSourceNavigation(_node: PreparedNode): boolean {
    return false;
}
