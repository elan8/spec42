/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/**
 * Export handler for diagram export (PNG, SVG, JSON).
 * Receives getCurrentData, getViewState, and postMessage via opts.
 */

export interface ExportHandlerOpts {
    getCurrentData: () => any;
    getViewState: () => { currentView: string };
    postMessage: (msg: unknown) => void;
}

export function prepareSvgForExport(svgElement: SVGSVGElement | null): SVGSVGElement | null {
    if (!svgElement) return null;

    // Clone the SVG to avoid modifying the original
    const clonedSvg = svgElement.cloneNode(true) as SVGSVGElement;

    // Get computed background color from the page
    const bgColor = getComputedStyle(document.body).backgroundColor || '#1e1e1e';

    // Find the main content group to get full content bounds
    const originalG = svgElement.querySelector('g');
    let contentBounds: DOMRect | null = null;

    if (originalG) {
        try {
            contentBounds = (originalG as SVGGraphicsElement).getBBox();
        } catch (e) {
            console.warn('Could not get content bounds');
        }
    }

    // Calculate full dimensions including all content
    let fullWidth: number;
    let fullHeight: number;
    const padding = 20;

    if (contentBounds && contentBounds.width > 0) {
        fullWidth = Math.max(contentBounds.x + contentBounds.width + padding, svgElement.clientWidth);
        fullHeight = Math.max(contentBounds.y + contentBounds.height + padding, svgElement.clientHeight);
    } else {
        fullWidth = svgElement.width?.baseVal?.value || svgElement.clientWidth || 800;
        fullHeight = svgElement.height?.baseVal?.value || svgElement.clientHeight || 600;
    }

    // Set SVG to full content size and adjust viewBox to show everything
    clonedSvg.setAttribute('width', fullWidth.toString());
    clonedSvg.setAttribute('height', fullHeight.toString());
    clonedSvg.setAttribute('viewBox', '0 0 ' + fullWidth + ' ' + fullHeight);

    // Reset transform on the cloned g to show unzoomed content
    const clonedG = clonedSvg.querySelector('g');
    if (clonedG && clonedG.hasAttribute('transform')) {
        clonedG.removeAttribute('transform');
    }

    // Resolve CSS variables and inline CSS class styles for export
    const elements = clonedSvg.querySelectorAll('*');
    const originalElements = svgElement.querySelectorAll('*');

    elements.forEach((el, index) => {
        const origEl = originalElements[index];
        if (!origEl) return;

        try {
            const tagName = (el as Element).tagName.toLowerCase();
            const computedStyle = window.getComputedStyle(origEl);

            // For path elements (tree links, connectors), inline critical stroke properties
            if (tagName === 'path') {
                const stroke = computedStyle.getPropertyValue('stroke');
                const strokeWidth = computedStyle.getPropertyValue('stroke-width');
                const fill = computedStyle.getPropertyValue('fill');
                const opacity = computedStyle.getPropertyValue('opacity');
                const strokeDasharray = computedStyle.getPropertyValue('stroke-dasharray');

                let inlineStyle = '';
                if (stroke && stroke !== 'none') inlineStyle += 'stroke: ' + stroke + '; ';
                if (strokeWidth) inlineStyle += 'stroke-width: ' + strokeWidth + '; ';
                if (fill) inlineStyle += 'fill: ' + fill + '; ';
                if (opacity && opacity !== '1') inlineStyle += 'opacity: ' + opacity + '; ';
                if (strokeDasharray && strokeDasharray !== 'none') inlineStyle += 'stroke-dasharray: ' + strokeDasharray + '; ';

                if (inlineStyle) {
                    (el as Element).setAttribute('style', inlineStyle);
                }
            }

            // For line elements, inline stroke properties
            if (tagName === 'line') {
                const stroke = computedStyle.getPropertyValue('stroke');
                const strokeWidth = computedStyle.getPropertyValue('stroke-width');
                const strokeDasharray = computedStyle.getPropertyValue('stroke-dasharray');

                let inlineStyle = '';
                if (stroke && stroke !== 'none') inlineStyle += 'stroke: ' + stroke + '; ';
                if (strokeWidth) inlineStyle += 'stroke-width: ' + strokeWidth + '; ';
                if (strokeDasharray && strokeDasharray !== 'none') inlineStyle += 'stroke-dasharray: ' + strokeDasharray + '; ';

                if (inlineStyle) {
                    (el as Element).setAttribute('style', inlineStyle);
                }
            }

            // For circle elements, inline fill and stroke
            if (tagName === 'circle') {
                const stroke = computedStyle.getPropertyValue('stroke');
                const strokeWidth = computedStyle.getPropertyValue('stroke-width');
                const fill = computedStyle.getPropertyValue('fill');

                let inlineStyle = '';
                if (stroke && stroke !== 'none') inlineStyle += 'stroke: ' + stroke + '; ';
                if (strokeWidth) inlineStyle += 'stroke-width: ' + strokeWidth + '; ';
                if (fill) inlineStyle += 'fill: ' + fill + '; ';

                if (inlineStyle) {
                    (el as Element).setAttribute('style', inlineStyle);
                }
            }

            // For text elements, inline font and fill
            if (tagName === 'text') {
                const fill = computedStyle.getPropertyValue('fill') || computedStyle.getPropertyValue('color');
                const fontSize = computedStyle.getPropertyValue('font-size');
                const fontFamily = computedStyle.getPropertyValue('font-family');
                const fontWeight = computedStyle.getPropertyValue('font-weight');

                let inlineStyle = (el as Element).getAttribute('style') || '';
                if (fill && !inlineStyle.includes('fill:')) inlineStyle += 'fill: ' + fill + '; ';
                if (fontSize && !inlineStyle.includes('font-size:')) inlineStyle += 'font-size: ' + fontSize + '; ';
                if (fontFamily && !inlineStyle.includes('font-family:')) inlineStyle += 'font-family: ' + fontFamily + '; ';
                if (fontWeight && !inlineStyle.includes('font-weight:')) inlineStyle += 'font-weight: ' + fontWeight + '; ';

                if (inlineStyle) {
                    (el as Element).setAttribute('style', inlineStyle);
                }
            }

            const existingStyle = (el as Element).getAttribute('style') || '';

            if (existingStyle.includes('var(')) {
                const styleProps = existingStyle.split(';').filter((s: string) => s.trim());
                const resolvedProps = styleProps.map((prop: string) => {
                    const colonIdx = prop.indexOf(':');
                    if (colonIdx === -1) return prop.trim();
                    const name = prop.substring(0, colonIdx).trim();
                    const value = prop.substring(colonIdx + 1).trim();
                    if (value && value.includes('var(')) {
                        const computed = computedStyle.getPropertyValue(name);
                        if (computed) {
                            return name + ': ' + computed;
                        }
                    }
                    return prop.trim();
                });

                (el as Element).setAttribute('style', resolvedProps.join('; ') + ';');
            }

            if (tagName === 'rect') {
                const stroke = computedStyle.getPropertyValue('stroke');
                const fill = computedStyle.getPropertyValue('fill');
                const strokeWidth = computedStyle.getPropertyValue('stroke-width');

                let currentStyle = (el as Element).getAttribute('style') || '';

                if (stroke && stroke !== 'none' && !currentStyle.includes('stroke:')) {
                    currentStyle += 'stroke: ' + stroke + '; ';
                }
                if (strokeWidth && !currentStyle.includes('stroke-width:')) {
                    currentStyle += 'stroke-width: ' + strokeWidth + '; ';
                }
                if (fill && !currentStyle.includes('fill:')) {
                    currentStyle += 'fill: ' + fill + '; ';
                }

                (el as Element).setAttribute('style', currentStyle);
            }
        } catch (e) {
            // Skip elements that can't be styled
        }
    });

    // Add background rect
    const bgRect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    bgRect.setAttribute('x', '0');
    bgRect.setAttribute('y', '0');
    bgRect.setAttribute('width', fullWidth.toString());
    bgRect.setAttribute('height', fullHeight.toString());
    bgRect.setAttribute('fill', bgColor);
    clonedSvg.insertBefore(bgRect, clonedSvg.firstChild);

    if (!clonedSvg.hasAttribute('xmlns')) {
        clonedSvg.setAttribute('xmlns', 'http://www.w3.org/2000/svg');
    }

    return clonedSvg;
}

export function createExportHandler(opts: ExportHandlerOpts) {
    const { getCurrentData, getViewState, postMessage } = opts;

    function exportJSON(): void {
        const currentData = getCurrentData();
        if (!currentData) {
            console.error('No data available for JSON export');
            return;
        }

        const jsonData = JSON.stringify(currentData, null, 2);
        const blob = new Blob([jsonData], { type: 'application/json' });
        const reader = new FileReader();

        reader.onloadend = function () {
            postMessage({
                command: 'export',
                format: 'json',
                data: reader.result
            });
        };

        reader.readAsDataURL(blob);
    }

    function exportPNG(scale?: number): void {
        const scaleFactor = scale || 2;

        const svgElement = document.querySelector('#visualization svg') as SVGSVGElement | null;
        if (!svgElement) {
            console.error('No SVG element found for PNG export');
            return;
        }

        const preparedSvg = prepareSvgForExport(svgElement);
        if (!preparedSvg) {
            console.error('Failed to prepare SVG for export');
            return;
        }

        const svgData = new XMLSerializer().serializeToString(preparedSvg);
        const width = parseInt(preparedSvg.getAttribute('width') || '800', 10);
        const height = parseInt(preparedSvg.getAttribute('height') || '600', 10);

        const canvas = document.createElement('canvas');
        canvas.width = width * scaleFactor;
        canvas.height = height * scaleFactor;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        ctx.scale(scaleFactor, scaleFactor);

        const img = new Image();
        img.onload = function () {
            ctx.drawImage(img, 0, 0, width, height);
            const pngData = canvas.toDataURL('image/png');
            postMessage({
                command: 'export',
                format: 'png',
                data: pngData
            });
        };
        img.onerror = function () {
            console.error('Failed to load SVG image for PNG export');
        };
        img.src = 'data:image/svg+xml;base64,' + btoa(unescape(encodeURIComponent(svgData)));
    }

    function exportSVG(): void {
        const svgString = getSvgStringForExport();
        if (!svgString) {
            console.error('No SVG available for export');
            return;
        }
        const svgBlob = new Blob([svgString], { type: 'image/svg+xml;charset=utf-8' });
        const reader = new FileReader();
        reader.onloadend = function () {
            postMessage({
                command: 'export',
                format: 'svg',
                data: reader.result
            });
        };
        reader.readAsDataURL(svgBlob);
    }

    /**
     * Returns the current diagram as an SVG string for testing/debugging.
     * Returns null if no SVG is available (e.g. placeholder or loading).
     */
    function getSvgStringForExport(): string | null {
        const svgElement = document.querySelector('#visualization svg') as SVGSVGElement | null;
        if (!svgElement) return null;

        const preparedSvg = prepareSvgForExport(svgElement);
        if (!preparedSvg) return null;

        return new XMLSerializer().serializeToString(preparedSvg);
    }

    return {
        exportJSON,
        exportPNG,
        exportSVG,
        prepareSvgForExport,
        getSvgStringForExport
    };
}
