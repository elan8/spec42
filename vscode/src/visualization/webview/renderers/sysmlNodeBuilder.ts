/**
 * Shared SysML v2 node builder with standard compartments.
 * Per SysML v2 spec (Clause 7.26.5, Tables 9-10): Header, Attributes, Parts, Ports compartments.
 * Used by General View and Interconnection View (IBD) with configurable compartment visibility.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import { DIAGRAM_STYLE } from '../styleTokens';

declare const d3: any;

function classifyNodeCategory(stereotype: string): 'structure' | 'behavior' | 'requirements' | 'other' {
    const type = (stereotype || '').toLowerCase();
    if (
        type.includes('part') ||
        type.includes('port') ||
        type.includes('attribute') ||
        type.includes('interface') ||
        type.includes('item') ||
        type.includes('occurrence')
    ) {
        return 'structure';
    }
    if (
        type.includes('action') ||
        type.includes('state') ||
        type.includes('calc') ||
        type.includes('analysis') ||
        type.includes('enumeration')
    ) {
        return 'behavior';
    }
    if (
        type.includes('requirement') ||
        type.includes('use case') ||
        type.includes('concern') ||
        type.includes('viewpoint') ||
        type.includes('stakeholder')
    ) {
        return 'requirements';
    }
    return 'other';
}

/** SysML v2 compartment data. Order: Header, Attributes, Parts, Ports per spec. */
export interface SysMLNodeDetailItem {
    name: string;
    typeName?: string | null;
    valueText?: string | null;
    declaredIn?: string | null;
    displayText: string;
}

export interface SysMLNodeSection {
    key: string;
    title: string;
    items: SysMLNodeDetailItem[];
    collapsed?: boolean;
    showAll?: boolean;
}

export interface SysMLNodeCompartments {
    header: { stereotype: string; name: string };
    typedByName?: string | null;
    attributes: SysMLNodeDetailItem[];
    parts: SysMLNodeDetailItem[];
    ports: SysMLNodeDetailItem[];
    collapsibleSections?: SysMLNodeSection[];
    /** Other content (Actions, Nested, etc.) for general view flexibility */
    other?: Array<{ title: string; lines: string[] }>;
}

/** Which compartments to render. All true = full spec layout. */
export interface SysMLNodeConfig {
    showHeader?: boolean;
    showAttributes?: boolean;
    showParts?: boolean;
    showPorts?: boolean;
    /** Include "other" sections (Actions, Nested) - general view only */
    showOther?: boolean;
    /** Max lines per compartment (0 = all) */
    maxLinesPerCompartment?: number;
}

/** IBD/Interconnection view preset: Header, Parts, Ports (no Attributes/Other) */
export const IBD_NODE_CONFIG: SysMLNodeConfig = {
    showHeader: true,
    showAttributes: false,
    showParts: true,
    showPorts: true,
    showOther: false,
    maxLinesPerCompartment: 6
};

const DEFAULT_CONFIG: Required<SysMLNodeConfig> = {
    showHeader: true,
    showAttributes: true,
    showParts: true,
    showPorts: true,
    showOther: true,
    maxLinesPerCompartment: 8
};

export const LINE_HEIGHT = 12;
export const COMPARTMENT_LABEL_HEIGHT = 14;
export const COMPARTMENT_GAP = 2;
/** Padding above/below compartment content (top and bottom divider spacing) */
const COMPARTMENT_PADDING = 4;
/** Header must fit stereotype (y~17) + name (y~31, ~12px tall) = at least 44 */
export const HEADER_COMPARTMENT_HEIGHT = 44;
export const TYPED_BY_HEIGHT = 14;
export const PADDING = 6;
const SHOW_MORE_LINE_HEIGHT = 12;
type CanvasLikeContext = { font: string; measureText: (text: string) => { width: number } };
let sharedTextMeasureContext: CanvasLikeContext | null = null;

function measureTextWidthPx(text: string, font: string): number {
    if (typeof globalThis === 'undefined') {
        // Conservative fallback for non-DOM contexts.
        return text.length * 7;
    }
    if (!sharedTextMeasureContext) {
        const doc = (globalThis as any).document;
        const canvas = doc?.createElement ? doc.createElement('canvas') : null;
        sharedTextMeasureContext = canvas?.getContext ? canvas.getContext('2d') : null;
    }
    const ctx = sharedTextMeasureContext;
    if (!ctx) return text.length * 7;
    ctx.font = font;
    return ctx.measureText(text).width;
}

function truncateTextToWidth(text: string, maxWidthPx: number, font: string): string {
    if (maxWidthPx <= 0) return '';
    if (measureTextWidthPx(text, font) <= maxWidthPx) return text;

    const ellipsis = '..';
    const ellipsisWidth = measureTextWidthPx(ellipsis, font);
    if (ellipsisWidth >= maxWidthPx) return '';

    let low = 0;
    let high = text.length;
    let best = '';
    while (low <= high) {
        const mid = Math.floor((low + high) / 2);
        const candidate = text.slice(0, mid) + ellipsis;
        if (measureTextWidthPx(candidate, font) <= maxWidthPx) {
            best = candidate;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    return best || ellipsis;
}

function normalizeDetailItem(item: any): SysMLNodeDetailItem | null {
    if (typeof item === 'string') {
        const text = item.trim();
        if (!text) return null;
        return { name: text, displayText: text };
    }
    if (!item || typeof item !== 'object') return null;
    const displayText = typeof item.displayText === 'string' && item.displayText.trim().length > 0
        ? normalizeUnitBrackets(item.displayText.trim())
        : (typeof item.name === 'string' ? normalizeUnitBrackets(item.name.trim()) : '');
    if (!displayText) return null;
    return {
        name: typeof item.name === 'string' && item.name.trim().length > 0 ? item.name.trim() : displayText,
        typeName: typeof item.typeName === 'string' ? normalizeUnitBrackets(item.typeName) : null,
        valueText: typeof item.valueText === 'string' ? normalizeUnitBrackets(item.valueText) : null,
        declaredIn: typeof item.declaredIn === 'string' ? item.declaredIn : null,
        displayText
    };
}

function detailItemsFromAttributeBag(element: any, key: string): SysMLNodeDetailItem[] {
    const attrs = element?.attributes;
    const rawValue = attrs && typeof attrs.get === 'function'
        ? attrs.get(key)
        : attrs?.[key];
    const raw = Array.isArray(rawValue) ? rawValue : [];
    return raw
        .map((item: any) => normalizeDetailItem(item))
        .filter((item: SysMLNodeDetailItem | null): item is SysMLNodeDetailItem => Boolean(item));
}

function lineToDetailItem(line: string): SysMLNodeDetailItem {
    const text = normalizeUnitBrackets(line.trim());
    return { name: text, displayText: text };
}

function normalizeUnitBrackets(text: string): string {
    if (!text) return text;
    let out = text;
    // Some upstream payloads can already include bracketed unit tokens and
    // arrive wrapped again, resulting in forms like [[kg]].
    while (/\[\[[^\[\]]+\]\]/.test(out)) {
        out = out.replace(/\[\[([^\[\]]+)\]\]/g, '[$1]');
    }
    return out;
}

/**
 * Collect compartments from general-view element (from buildGeneralViewGraph).
 */
export function collectCompartmentsFromElement(element: any): SysMLNodeCompartments {
    const headerName = (element?.name ?? element?.elementName ?? element?.label ?? 'Unnamed').toString();
    const result: SysMLNodeCompartments = {
        header: {
            stereotype: (element?.type || 'element').toLowerCase(),
            name: headerName
        },
        typedByName: null,
        attributes: [],
        parts: [],
        ports: [],
        collapsibleSections: [],
        other: []
    };

    if (element) {
        const attrs = element.attributes;
        result.typedByName = (attrs && (typeof attrs.get === 'function'
            ? attrs.get('partType') || attrs.get('type') || attrs.get('typedBy')
            : attrs.partType || attrs.type || attrs.typedBy)) || null;
        if (!result.typedByName && element.partType) result.typedByName = element.partType;
        if (!result.typedByName && element.typings?.length) {
            result.typedByName = String(element.typings[0]).replace(/^[:~]+/, '').trim();
        }
        if (!result.typedByName && element.typing) {
            result.typedByName = String(element.typing).replace(/^[:~]+/, '').trim();
        }
    }

    result.attributes = detailItemsFromAttributeBag(element, 'generalViewDirectAttributes');
    result.parts = detailItemsFromAttributeBag(element, 'generalViewDirectParts');
    result.ports = detailItemsFromAttributeBag(element, 'generalViewDirectPorts');

    const inheritedAttributes = detailItemsFromAttributeBag(element, 'generalViewInheritedAttributes');
    if (inheritedAttributes.length > 0) {
        result.collapsibleSections!.push({
            key: 'inherited-attributes',
            title: 'Inherited Attributes',
            items: inheritedAttributes,
            collapsed: true,
            showAll: false
        });
    }
    const inheritedParts = detailItemsFromAttributeBag(element, 'generalViewInheritedParts');
    if (inheritedParts.length > 0) {
        result.collapsibleSections!.push({
            key: 'inherited-parts',
            title: 'Inherited Parts',
            items: inheritedParts,
            collapsed: true,
            showAll: false
        });
    }

    return result;
}

/**
 * Collect compartments from IBD part (from prepareData interconnection-view).
 */
export function collectCompartmentsFromPart(part: any, ports: any[]): SysMLNodeCompartments {
    const result: SysMLNodeCompartments = {
        header: {
            stereotype: (part?.type || 'part').toLowerCase(),
            name: (part?.name || 'Unnamed').toString()
        },
        typedByName: null,
        attributes: [],
        parts: [],
        ports: [],
        collapsibleSections: [],
        other: []
    };

    if (part?.attributes?.get) {
        result.typedByName = part.attributes.get('partType') || part.attributes.get('type') || part.attributes.get('typedBy');
    }
    if (!result.typedByName && part?.partType) result.typedByName = part.partType;

    const partPorts = ports.filter((p: any) => p && (p.parentId === part.name || p.parentId === part.id || p.parentId === part.qualifiedName));
    partPorts.forEach((p: any) => {
        if (p?.name) {
            const portType = p.attributes?.get ? p.attributes.get('portType') : (p.attributes?.portType);
            const normalizedPortType = portType ? normalizeUnitBrackets(String(portType)) : null;
            result.ports.push({
                name: p.name,
                typeName: normalizedPortType,
                displayText: (p.name + (normalizedPortType ? ' : ' + normalizedPortType : '')).trim()
            });
        }
    });

    (part?.children || []).forEach((c: any) => {
        if (!c?.name || !c?.type) return;
        if (c.type === 'part') result.parts.push({ name: c.name, displayText: c.name });
        else if (c.type === 'port') {
            const portType = c.attributes?.get ? c.attributes.get('portType') : (c.attributes?.portType);
            const normalizedPortType = portType ? normalizeUnitBrackets(String(portType)) : null;
            result.ports.push({
                name: c.name,
                typeName: normalizedPortType,
                displayText: (c.name + (normalizedPortType ? ' : ' + normalizedPortType : '')).trim()
            });
        }
    });

    return result;
}

/**
 * Compute node height from compartments and config.
 */
export function computeNodeHeightFromCompartments(
    compartments: SysMLNodeCompartments,
    config: SysMLNodeConfig,
    nodeWidth: number
): number {
    const cfg = { ...DEFAULT_CONFIG, ...config };
    let h = PADDING * 2;

    if (cfg.showHeader) {
        h += HEADER_COMPARTMENT_HEIGHT;
        if (compartments.typedByName) h += TYPED_BY_HEIGHT;
    }
    const hasBodyCompartments = (cfg.showAttributes && compartments.attributes.length > 0) ||
        (cfg.showParts && compartments.parts.length > 0) ||
        (cfg.showPorts && compartments.ports.length > 0) ||
        (!!compartments.collapsibleSections?.some((section) => section.items.length > 0)) ||
        (cfg.showOther && !!compartments.other?.some((s) => s.lines.length > 0));
    if (cfg.showHeader && hasBodyCompartments) {
        h += COMPARTMENT_PADDING; // Gap between header and first compartment
    }

    const addComp = (items: SysMLNodeDetailItem[]) => {
        if (items.length === 0) return;
        const n = cfg.maxLinesPerCompartment ? Math.min(items.length, cfg.maxLinesPerCompartment) : items.length;
        h += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + n * LINE_HEIGHT + COMPARTMENT_GAP;
        if (cfg.maxLinesPerCompartment && items.length > cfg.maxLinesPerCompartment) {
            h += SHOW_MORE_LINE_HEIGHT;
        }
    };

    const addCollapsibleSection = (section: SysMLNodeSection) => {
        if (section.items.length === 0) return;
        h += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + COMPARTMENT_GAP;
        if (!section.collapsed) {
            const n = section.showAll || !cfg.maxLinesPerCompartment
                ? section.items.length
                : Math.min(section.items.length, cfg.maxLinesPerCompartment);
            h += n * LINE_HEIGHT;
            if (cfg.maxLinesPerCompartment && section.items.length > cfg.maxLinesPerCompartment) {
                h += SHOW_MORE_LINE_HEIGHT;
            }
        }
    };

    if (cfg.showAttributes) addComp(compartments.attributes);
    if (cfg.showParts) addComp(compartments.parts);
    if (cfg.showPorts) addComp(compartments.ports);
    if (compartments.collapsibleSections?.length) {
        compartments.collapsibleSections.forEach(addCollapsibleSection);
    }
    if (cfg.showOther && compartments.other?.length) {
        compartments.other.forEach((sec) => {
            const n = cfg.maxLinesPerCompartment ? Math.min(sec.lines.length, cfg.maxLinesPerCompartment) : sec.lines.length;
            h += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + n * LINE_HEIGHT + COMPARTMENT_GAP;
        });
    }

    return Math.max(60, h);
}

/**
 * Render a SysML node with clear compartments. Appends to parentGroup (D3 selection).
 * Returns the node group.
 */
export function renderSysMLNode(
    parentGroup: any,
    compartments: SysMLNodeCompartments,
    options: {
        x: number;
        y: number;
        width: number;
        height: number;
        config?: SysMLNodeConfig;
        isDefinition?: boolean;
        typeColor?: string;
        formatStereotype?: (type: string) => string;
        nodeClass?: string;
        dataElementName?: string;
        sectionKeyPrefix?: string;
        onSectionToggle?: (key: string, action: 'collapse' | 'rows') => void;
    }
): any {
    const cfg = { ...DEFAULT_CONFIG, ...(options.config || {}) };
    const formatStereo = options.formatStereotype || ((t: string) => '«' + t + '»');
    // Coordinates are relative to the group origin (0,0) - node has transform translate(x,y)
    let contentY = 0;

    const nodeG = parentGroup.append('g')
        .attr('class', (options.nodeClass || 'sysml-node') + (options.isDefinition ? ' definition-node' : ' usage-node'))
        .attr('transform', 'translate(' + options.x + ',' + options.y + ')')
        .attr('data-element-name', options.dataElementName || compartments.header.name)
        .style('cursor', 'pointer');

    const strokeColor = options.typeColor || DIAGRAM_STYLE.edgePrimary;
    const strokeW = options.isDefinition ? '3px' : '2px';
    const nodeCategory = classifyNodeCategory(compartments.header.stereotype);
    const cornerRadius = nodeCategory === 'requirements'
        ? 16
        : nodeCategory === 'behavior'
            ? 12
            : options.isDefinition
                ? 4
                : 8;
    const bodyFill = DIAGRAM_STYLE.nodeFill;
    const headerFill = DIAGRAM_STYLE.panelBackground;
    const dividerColor = DIAGRAM_STYLE.nodeBorder;
    const compartmentTitleColor = DIAGRAM_STYLE.textSecondary;
    const accentPrefix = nodeCategory === 'behavior'
        ? '[B] '
        : nodeCategory === 'requirements'
            ? '[R] '
            : nodeCategory === 'structure'
                ? '[S] '
                : '';

    nodeG.append('rect')
        .attr('width', options.width)
        .attr('height', options.height)
        .attr('rx', cornerRadius)
        .attr('class', 'graph-node-background sysml-node-bg')
        .attr('data-original-stroke', strokeColor)
        .attr('data-original-width', strokeW)
        .style('fill', bodyFill)
        .style('stroke', strokeColor)
        .style('stroke-width', strokeW)
        .style('stroke-dasharray', options.isDefinition ? '6,3' : 'none');

    // ---- Header compartment (Name Compartment per SysML v2) ----
    if (cfg.showHeader) {
        const headerH = HEADER_COMPARTMENT_HEIGHT + (compartments.typedByName ? TYPED_BY_HEIGHT : 0);
        nodeG.append('rect')
            .attr('y', 0)
            .attr('width', options.width)
            .attr('height', headerH)
            .attr('class', 'sysml-header-compartment')
            .attr('rx', Math.max(2, cornerRadius - 2))
            .style('fill', headerFill);

        const stereo = formatStereo(compartments.header.stereotype) || ('«' + compartments.header.stereotype + '»');
        nodeG.append('text')
            .attr('x', options.width / 2)
            .attr('y', 17)
            .attr('text-anchor', 'middle')
            .text(accentPrefix + stereo)
            .style('font-size', '9px')
            .style('fill', strokeColor);

        const displayName = compartments.header.name;
        const truncatedName = displayName.length > 26 ? displayName.substring(0, 24) + '..' : displayName;
        nodeG.append('text')
            .attr('class', 'node-name-text')
            .attr('x', options.width / 2)
            .attr('y', 31)
            .attr('text-anchor', 'middle')
            .text(truncatedName)
            .style('font-size', '11px')
            .style('font-weight', 'bold')
            .style('fill', DIAGRAM_STYLE.textPrimary);

        if (compartments.typedByName) {
            const tbText = compartments.typedByName.length > 22 ? compartments.typedByName.substring(0, 20) + '..' : compartments.typedByName;
            nodeG.append('text')
                .attr('x', options.width / 2)
                .attr('y', 43)
                .attr('text-anchor', 'middle')
                .text(': ' + tbText)
                .style('font-size', '10px')
                .style('font-style', 'italic')
                .style('fill', DIAGRAM_STYLE.edgePrimary);
        }

        contentY += headerH;
        // Gap between header and first compartment so boundary doesn't overlap header
        contentY += COMPARTMENT_PADDING;
    }

    const renderCompartment = (
        title: string,
        items: SysMLNodeDetailItem[],
        sectionKey?: string,
        collapsible = false,
        collapsed = false,
        showAll = false
    ) => {
        if (items.length === 0) return;
        const limit = showAll || !cfg.maxLinesPerCompartment
            ? items.length
            : Math.min(items.length, cfg.maxLinesPerCompartment);
        const slice = collapsed ? [] : items.slice(0, limit);
        const compTop = contentY;
        // Top line - compartment boundary
        nodeG.append('line')
            .attr('x1', PADDING)
            .attr('y1', compTop)
            .attr('x2', options.width - PADDING)
            .attr('y2', compTop)
            .attr('class', 'sysml-compartment-divider')
            .style('stroke', dividerColor)
            .style('stroke-width', '1px');
        contentY += 4;
        // Title (bold) on its own line
        const titleText = nodeG.append('text')
            .attr('x', PADDING)
            .attr('y', contentY + 9)
            .text(collapsible ? ((collapsed ? '▸ ' : '▾ ') + title) : title)
            .style('font-size', '9px')
            .style('font-weight', 'bold')
            .style('fill', compartmentTitleColor)
            .style('cursor', collapsible ? 'pointer' : 'default');
        if (collapsible && sectionKey && options.onSectionToggle) {
            titleText.on('click', function(event: any) {
                event.stopPropagation();
                options.onSectionToggle!(sectionKey, 'collapse');
            });
        }
        contentY += COMPARTMENT_LABEL_HEIGHT;
        // Content lines
        slice.forEach((item) => {
            const line = item.displayText;
            const maxTextWidth = options.width - (PADDING * 2);
            const truncated = truncateTextToWidth(line, maxTextWidth, '9px sans-serif');
            nodeG.append('text')
                .attr('x', PADDING)
                .attr('y', contentY + 9)
                .text(truncated)
                .style('font-size', '9px')
                .style('fill', DIAGRAM_STYLE.textSecondary)
                .append('title')
                .text(item.declaredIn ? `${item.displayText} (from ${item.declaredIn})` : item.displayText);
            contentY += LINE_HEIGHT;
        });
        if (!collapsed && cfg.maxLinesPerCompartment && items.length > cfg.maxLinesPerCompartment) {
            const hiddenCount = items.length - cfg.maxLinesPerCompartment;
            const toggleText = showAll ? 'Show less' : `+${hiddenCount} more`;
            const moreText = nodeG.append('text')
                .attr('x', PADDING)
                .attr('y', contentY + 9)
                .text(toggleText)
                .style('font-size', '9px')
                .style('font-weight', 'bold')
                .style('fill', DIAGRAM_STYLE.edgePrimary)
                .style('cursor', 'pointer');
            if (sectionKey && options.onSectionToggle) {
                moreText.on('click', function(event: any) {
                    event.stopPropagation();
                    options.onSectionToggle!(sectionKey, 'rows');
                });
            }
            contentY += SHOW_MORE_LINE_HEIGHT;
        }
        contentY += COMPARTMENT_PADDING;
        contentY += COMPARTMENT_GAP;
        // Only top line per compartment - no bottom line (avoids double lines between compartments)
    };

    if (cfg.showAttributes && compartments.attributes.length > 0) {
        renderCompartment('Attributes', compartments.attributes, options.sectionKeyPrefix ? options.sectionKeyPrefix + 'attributes' : undefined);
    }
    if (cfg.showParts && compartments.parts.length > 0) {
        renderCompartment('Parts', compartments.parts, options.sectionKeyPrefix ? options.sectionKeyPrefix + 'parts' : undefined);
    }
    if (cfg.showPorts && compartments.ports.length > 0) {
        renderCompartment('Ports', compartments.ports, options.sectionKeyPrefix ? options.sectionKeyPrefix + 'ports' : undefined);
    }
    if (compartments.collapsibleSections?.length) {
        compartments.collapsibleSections.forEach((section) => {
            if (section.items.length > 0) {
                renderCompartment(section.title, section.items, section.key, true, Boolean(section.collapsed), Boolean(section.showAll));
            }
        });
    }
    if (cfg.showOther && compartments.other?.length) {
        compartments.other.forEach((sec) => {
            if (sec.lines.length > 0) renderCompartment(sec.title, sec.lines.map((line) => lineToDetailItem(line)));
        });
    }

    return nodeG;
}
