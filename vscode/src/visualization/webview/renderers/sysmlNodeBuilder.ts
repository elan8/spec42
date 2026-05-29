/**
 * Shared SysML v2 node builder with standard compartments.
 * Per SysML v2 spec (Clause 7.26.5, Tables 9-10): Header, Attributes, Parts, Ports compartments.
 * Used by General View and Interconnection View (IBD) with configurable compartment visibility.
 */
/* eslint-disable @typescript-eslint/no-explicit-any */

import { nodeBodyChromeStyle, resolveNodeChrome } from '../../../../../shared/diagram-renderer/src/node-notation';
import {
    COMPARTMENT_GAP,
    COMPARTMENT_LABEL_HEIGHT,
    DEFAULT_SYSML_NODE_CONFIG,
    HEADER_COMPARTMENT_HEIGHT,
    LINE_HEIGHT,
    lineToDetailItem,
    PADDING,
    TYPED_BY_HEIGHT,
    type SysMLNodeCompartments,
    type SysMLNodeConfig,
    type SysMLNodeDetailItem,
} from '../../sysmlCompartments';
import { DIAGRAM_STYLE } from '../styleTokens';

export {
    collectCompartmentsFromElement,
    collectCompartmentsFromPart,
    computeNodeHeightFromCompartments,
    IBD_NODE_CONFIG,
    LINE_HEIGHT,
    COMPARTMENT_LABEL_HEIGHT,
    COMPARTMENT_GAP,
    HEADER_COMPARTMENT_HEIGHT,
    TYPED_BY_HEIGHT,
    PADDING,
    type SysMLNodeCompartments,
    type SysMLNodeConfig,
    type SysMLNodeDetailItem,
    type SysMLNodeSection,
} from '../../sysmlCompartments';

declare const d3: any;

const COMPARTMENT_PADDING = 4;
const SHOW_MORE_LINE_HEIGHT = 12;

type CanvasLikeContext = { font: string; measureText: (text: string) => { width: number } };
let sharedTextMeasureContext: CanvasLikeContext | null = null;

function measureTextWidthPx(text: string, font: string): number {
    if (typeof globalThis === 'undefined') {
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
        isReference?: boolean;
        typeColor?: string;
        formatStereotype?: (type: string) => string;
        nodeClass?: string;
        dataElementName?: string;
        sectionKeyPrefix?: string;
        onSectionToggle?: (key: string, action: 'collapse' | 'rows') => void;
    }
): any {
    const cfg = { ...DEFAULT_SYSML_NODE_CONFIG, ...(options.config || {}) };
    const formatStereo = options.formatStereotype || ((t: string) => '«' + t + '»');
    let contentY = 0;

    const chrome = resolveNodeChrome(compartments.header.stereotype, {
        isDefinition: options.isDefinition,
        isReference: options.isReference
    });
    const nodeG = parentGroup.append('g')
        .attr('class', (options.nodeClass || 'sysml-node') + chrome.nodeClassSuffix)
        .attr('transform', 'translate(' + options.x + ',' + options.y + ')')
        .attr('data-element-name', options.dataElementName || compartments.header.name)
        .style('cursor', 'pointer');

    const strokeColor = options.typeColor || DIAGRAM_STYLE.nodeBorder;
    const body = nodeBodyChromeStyle(chrome, { generalView: true });
    const nodeCategory = classifyNodeCategory(compartments.header.stereotype);
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
        .attr('rx', body.cornerRadius)
        .attr('class', 'graph-node-background sysml-node-bg')
        .attr('data-original-stroke', strokeColor)
        .attr('data-original-width', body.strokeWidthPx + 'px')
        .style('fill', bodyFill)
        .style('stroke', strokeColor)
        .style('stroke-width', body.strokeWidthPx + 'px')
        .style('stroke-dasharray', body.strokeDasharray);

    if (cfg.showHeader) {
        const headerH = HEADER_COMPARTMENT_HEIGHT + (compartments.typedByName ? TYPED_BY_HEIGHT : 0);
        nodeG.append('rect')
            .attr('y', 0)
            .attr('width', options.width)
            .attr('height', headerH)
            .attr('class', 'sysml-header-compartment')
            .attr('rx', body.headerCornerRadius)
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
        nodeG.append('line')
            .attr('x1', PADDING)
            .attr('y1', compTop)
            .attr('x2', options.width - PADDING)
            .attr('y2', compTop)
            .attr('class', 'sysml-compartment-divider')
            .style('stroke', dividerColor)
            .style('stroke-width', '1px');
        contentY += 4;
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
