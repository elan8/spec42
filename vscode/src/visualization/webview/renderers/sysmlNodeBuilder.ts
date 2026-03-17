/**
 * Shared SysML v2 node builder with standard compartments.
 * Per SysML v2 spec (Clause 7.26.5, Tables 9-10): Header, Attributes, Parts, Ports compartments.
 * Used by General View and Interconnection View (IBD) with configurable compartment visibility.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

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
export interface SysMLNodeCompartments {
    header: { stereotype: string; name: string };
    typedByName?: string | null;
    attributes: string[];
    parts: string[];
    ports: string[];
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

    if (!element?.children?.length) {
        return result;
    }

    const typeLower = (element.type || '').toLowerCase();
    const isRequirement = typeLower.includes('requirement');

    element.children.forEach((child: any) => {
        if (!child?.name) return;
        const cType = (child.type || '').toLowerCase();
        if (cType.includes('package') || cType.includes('state')) return;

        if (cType === 'attribute' || cType.includes('attribute')) {
            const dataType = child.attributes?.get ? child.attributes.get('dataType') : (child.attributes?.dataType);
            const typeStr = dataType ? ' : ' + String(dataType).split('::').pop() : '';
            result.attributes.push('  ' + child.name + typeStr);
        } else if (cType === 'port' || cType.includes('port')) {
            const portType = child.attributes?.get ? child.attributes.get('portType') : (child.attributes?.portType);
            const pTypeStr = portType ? ' : ' + portType : '';
            result.ports.push('  ' + child.name + pTypeStr);
        } else if (cType.includes('part')) {
            result.parts.push('  ' + child.name);
        } else if (cType.includes('action')) {
            const existing = result.other!.find((o) => o.title === 'Actions');
            if (existing) existing.lines.push('  ' + child.name);
            else result.other!.push({ title: 'Actions', lines: ['  ' + child.name] });
        } else if (cType.includes('requirement') || cType.includes('interface') || cType.includes('connect')) {
            const existing = result.other!.find((o) => o.title === 'Nested');
            if (existing) existing.lines.push('  ' + child.name);
            else result.other!.push({ title: 'Nested', lines: ['  ' + child.name] });
        }
    });

    if (element.ports?.length) {
        element.ports.forEach((p: any) => {
            const pName = typeof p === 'string' ? p : (p?.name || 'port');
            if (!result.ports.some((l) => l.includes(pName))) result.ports.push('  ' + pName);
        });
    }

    if (isRequirement && result.other!.length) {
        result.attributes = [];
        result.parts = [];
        result.ports = [];
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
            result.ports.push('  ' + p.name + (portType ? ' : ' + portType : ''));
        }
    });

    (part?.children || []).forEach((c: any) => {
        if (!c?.name || !c?.type) return;
        if (c.type === 'part') result.parts.push('  ' + c.name);
        else if (c.type === 'port') {
            const portType = c.attributes?.get ? c.attributes.get('portType') : (c.attributes?.portType);
            result.ports.push('  ' + c.name + (portType ? ' : ' + portType : ''));
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
        (cfg.showOther && !!compartments.other?.some((s) => s.lines.length > 0));
    if (cfg.showHeader && hasBodyCompartments) {
        h += COMPARTMENT_PADDING; // Gap between header and first compartment
    }

    const addComp = (lines: string[]) => {
        if (lines.length === 0) return;
        const n = cfg.maxLinesPerCompartment ? Math.min(lines.length, cfg.maxLinesPerCompartment) : lines.length;
        h += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + n * LINE_HEIGHT + COMPARTMENT_GAP;
    };

    if (cfg.showAttributes) addComp(compartments.attributes);
    if (cfg.showParts) addComp(compartments.parts);
    if (cfg.showPorts) addComp(compartments.ports);
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

    const strokeColor = options.typeColor || 'var(--vscode-charts-blue)';
    const strokeW = options.isDefinition ? '3px' : '2px';
    const nodeCategory = classifyNodeCategory(compartments.header.stereotype);
    const cornerRadius = nodeCategory === 'requirements'
        ? 16
        : nodeCategory === 'behavior'
            ? 12
            : options.isDefinition
                ? 4
                : 8;
    const bodyFill = 'color-mix(in srgb, ' + strokeColor + ' 7%, var(--vscode-editor-background))';
    const headerFill = 'color-mix(in srgb, ' + strokeColor + ' 14%, var(--vscode-button-secondaryBackground))';
    const dividerColor = 'color-mix(in srgb, ' + strokeColor + ' 24%, var(--vscode-panel-border))';
    const compartmentTitleColor = 'color-mix(in srgb, ' + strokeColor + ' 72%, var(--vscode-descriptionForeground))';
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
            .attr('height', 5)
            .attr('rx', Math.max(2, cornerRadius - 2))
            .style('fill', strokeColor);

        nodeG.append('rect')
            .attr('y', 5)
            .attr('width', options.width)
            .attr('height', headerH - 5)
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
            .style('fill', 'var(--vscode-editor-foreground)');

        if (compartments.typedByName) {
            const tbText = compartments.typedByName.length > 22 ? compartments.typedByName.substring(0, 20) + '..' : compartments.typedByName;
            nodeG.append('text')
                .attr('x', options.width / 2)
                .attr('y', 43)
                .attr('text-anchor', 'middle')
                .text(': ' + tbText)
                .style('font-size', '10px')
                .style('font-style', 'italic')
                .style('fill', '#569CD6');
        }

        contentY += headerH;
        // Gap between header and first compartment so boundary doesn't overlap header
        contentY += COMPARTMENT_PADDING;
    }

    const renderCompartment = (title: string, lines: string[]) => {
        if (lines.length === 0) return;
        const limit = cfg.maxLinesPerCompartment ? Math.min(lines.length, cfg.maxLinesPerCompartment) : lines.length;
        const slice = lines.slice(0, limit);
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
        nodeG.append('text')
            .attr('x', PADDING)
            .attr('y', contentY + 9)
            .text(title)
            .style('font-size', '9px')
            .style('font-weight', 'bold')
            .style('fill', compartmentTitleColor);
        contentY += COMPARTMENT_LABEL_HEIGHT;
        // Content lines
        slice.forEach((line) => {
            const truncated = line.length > 28 ? line.substring(0, 26) + '..' : line;
            nodeG.append('text')
                .attr('x', PADDING)
                .attr('y', contentY + 9)
                .text(truncated)
                .style('font-size', '9px')
                .style('fill', 'var(--vscode-descriptionForeground)');
            contentY += LINE_HEIGHT;
        });
        contentY += COMPARTMENT_PADDING;
        contentY += COMPARTMENT_GAP;
        // Only top line per compartment - no bottom line (avoids double lines between compartments)
    };

    if (cfg.showAttributes && compartments.attributes.length > 0) {
        renderCompartment('Attributes', compartments.attributes);
    }
    if (cfg.showParts && compartments.parts.length > 0) {
        renderCompartment('Parts', compartments.parts);
    }
    if (cfg.showPorts && compartments.ports.length > 0) {
        renderCompartment('Ports', compartments.ports);
    }
    if (cfg.showOther && compartments.other?.length) {
        compartments.other.forEach((sec) => {
            if (sec.lines.length > 0) renderCompartment(sec.title, sec.lines);
        });
    }

    return nodeG;
}
