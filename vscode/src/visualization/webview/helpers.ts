/**
 * Shared helper functions for the visualizer webview.
 * Pure data/formatting utilities used across renderers.
 */

/* eslint-disable @typescript-eslint/no-explicit-any */

import { normalizeAttributes } from './shared';
import { buildElementDisplayLabel } from './shared';

export function isMetadataElement(type: string | null | undefined): boolean {
    return type === 'doc' ||
           type === 'comment' ||
           type === 'metadata' ||
           type === 'metadata def';
}

export function extractDocumentation(element: any): string | null {
    if (!element) return null;
    if (element.children) {
        const docElements = element.children.filter((child: any) => isMetadataElement(child.type));
        if (docElements.length > 0) {
            return docElements.map((doc: any) => doc.name || 'Documentation').join(' ');
        }
    }
    if (isMetadataElement(element.type)) {
        return element.name || 'Documentation';
    }
    return null;
}

export function convertToHierarchy(elements: any[]): any {
    function convertElement(el: any): any {
        if (!el || !el.name || !el.type) {
            if (typeof console !== 'undefined' && console.warn) {
                console.warn('Skipping invalid element:', el);
            }
            return null;
        }
        const properties = normalizeAttributes(el.attributes) as Record<string, unknown>;
        const documentation = extractDocumentation(el);
        if (documentation) {
            properties['documentation'] = documentation;
        }
        const validChildren = el.children
            ? el.children.map(convertElement).filter((child: any) => child !== null)
            : [];
        return {
            name: el.name,
            type: el.type,
            properties,
            children: validChildren,
            element: el
        };
    }
    const validElements = (elements || []).map(convertElement).filter((el: any) => el !== null);
    if (validElements.length === 1) {
        return validElements[0];
    }
    return {
        name: 'Model Root',
        type: 'root',
        properties: {},
        children: validElements
    };
}

export function flattenElements(elements: any[], result: any[] = []): any[] {
    (elements || []).forEach((el: any) => {
        if (isMetadataElement(el.type)) return;
        const properties = normalizeAttributes(el.attributes) as Record<string, unknown>;
        const documentation = extractDocumentation(el);
        if (documentation) {
            properties['documentation'] = documentation;
        }
        result.push({
            name: el.name,
            type: el.type,
            properties,
            pillar: el.pillar,
            element: el
        });
        if (el.children && el.children.length > 0) {
            flattenElements(el.children, result);
        }
    });
    return result;
}

export function wrapTextToLines(text: string | null | undefined, maxCharsPerLine: number, maxLines = 3): string[] {
    if (!text) return [];
    const words = String(text).split(/\s+/);
    const lines: string[] = [];
    let currentLine = '';

    words.forEach((word: string) => {
        const tentative = currentLine.length === 0 ? word : currentLine + ' ' + word;
        if (tentative.length > maxCharsPerLine && currentLine.length > 0) {
            lines.push(currentLine);
            currentLine = word;
        } else {
            currentLine = tentative;
        }
    });

    if (currentLine.length > 0) {
        lines.push(currentLine);
    }

    if (lines.length > maxLines) {
        const trimmed = lines.slice(0, maxLines);
        const lastIndex = trimmed.length - 1;
        trimmed[lastIndex] = trimmed[lastIndex] + '…';
        return trimmed;
    }
    return lines;
}

export function truncateLabel(text: string | null | undefined, maxChars: number): string {
    if (!text) return '';
    if (text.length <= maxChars) return text;
    return text.substring(0, Math.max(1, maxChars - 1)) + '…';
}

export function countAllElements(elements: any[]): number {
    if (!elements) return 0;
    let count = elements.length;
    elements.forEach((element: any) => {
        if (element.children && element.children.length > 0) {
            count += countAllElements(element.children);
        }
    });
    return count;
}

export function filterElementsRecursive(elements: any[], searchTerm: string): any[] {
    return elements.filter((element: any) => {
        const nameMatch = (element.name || '').toLowerCase().includes(searchTerm);
        const typeMatch = (element.type || '').toLowerCase().includes(searchTerm);
        let propertyMatch = false;
        if (element.properties) {
            for (const [key, value] of Object.entries(element.properties)) {
                if (String(key).toLowerCase().includes(searchTerm) ||
                    String(value).toLowerCase().includes(searchTerm)) {
                    propertyMatch = true;
                    break;
                }
            }
        }
        let hasMatchingChildren = false;
        if (element.children && element.children.length > 0) {
            const filteredChildren = filterElementsRecursive(element.children, searchTerm);
            if (filteredChildren.length > 0) {
                (element as any).children = filteredChildren;
                hasMatchingChildren = true;
            }
        }
        return nameMatch || typeMatch || propertyMatch || hasMatchingChildren;
    });
}

export function createLinksFromHierarchy(elements: any[], parent: any = null, links: any[] = []): any[] {
    (elements || []).forEach((el: any) => {
        if (parent) {
            links.push({ source: parent.name, target: el.name });
        }
        if (el.children && el.children.length > 0) {
            createLinksFromHierarchy(el.children, el, links);
        }
    });
    return links;
}

export function buildEnhancedElementLabel(element: any): string {
    if (!element) return '';
    const baseLabel = buildElementDisplayLabel(element);
    const lines = [baseLabel];
    if (element.children && element.children.length > 0) {
        const attributes: string[] = [];
        const ports: string[] = [];
        element.children.forEach((child: any) => {
            if (!child || !child.type) return;
            const typeLower = child.type.toLowerCase();
            if (typeLower === 'attribute' || typeLower.includes('attribute')) {
                const attrName = child.name || 'unnamed';
                const attrType = child.typing || '';
                attributes.push(attrType ? attrName + ': ' + attrType : attrName);
            } else if (typeLower.includes('port')) {
                const portName = child.name || 'unnamed';
                const portType = child.typing || '';
                const direction = typeLower.includes('in') ? '→' : typeLower.includes('out') ? '←' : '↔';
                ports.push(direction + ' ' + portName + (portType ? ': ' + portType : ''));
            }
        });
        if (attributes.length > 0) {
            const shown = attributes.slice(0, 3);
            lines.push('', 'Attributes:');
            shown.forEach((a: string) => lines.push('  • ' + a));
            if (attributes.length > 3) {
                lines.push('  +' + (attributes.length - 3) + ' more');
            }
        }
        if (ports.length > 0) {
            lines.push('', 'Ports:');
            const shown = ports.slice(0, 3);
            shown.forEach((p: string) => lines.push('  ' + p));
            if (ports.length > 3) {
                lines.push('  +' + (ports.length - 3) + ' more');
            }
        }
    }
    return lines.join('\n');
}

export function getLibraryChain(element: any): any {
    if (!element || !element.attributes) return null;
    const attrs = element.attributes;
    if (typeof attrs.get === 'function') {
        return attrs.get('specializationChain');
    }
    return attrs.specializationChain;
}

export function getLibraryKind(element: any): any {
    if (!element || !element.attributes) return null;
    const attrs = element.attributes;
    if (typeof attrs.get === 'function') {
        return attrs.get('libraryKind');
    }
    return attrs.libraryKind;
}

export function slugify(value: string | null | undefined): string {
    if (!value) return 'unknown';
    return value.toLowerCase().replace(/[^a-z0-9]+/g, '-');
}
