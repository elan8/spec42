/**
 * SysML compartment collection and height math (extension + tests).
 * DOM rendering lives in webview/renderers/sysmlNodeBuilder.ts.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

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

export const DEFAULT_SYSML_NODE_CONFIG: Required<SysMLNodeConfig> = {
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

export function lineToDetailItem(line: string): SysMLNodeDetailItem {
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
    _nodeWidth: number
): number {
    const cfg = { ...DEFAULT_SYSML_NODE_CONFIG, ...config };
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
        h += COMPARTMENT_PADDING;
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
