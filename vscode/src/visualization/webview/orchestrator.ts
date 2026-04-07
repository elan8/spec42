/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
// Orchestrator: message handling, state, and dispatch to modular renderers.
// Config (elkWorkerUrl) is set by a minimal inline script in HTML before this bundle loads.

import { prepareDataForView, graphToElementTree } from '../prepareData';
import {
    quickHash,
    buildElementDisplayLabel,
    formatSysMLStereotype,
    getNodeColor,
    getNodeBorderStyle,
    getTypeColor,
    isActorElement,
    cloneElements,
    normalizeAttributes,
    getElementProperties,
    formatStereotype,
    normalizeTypeForDisplay,
    isLibraryValidated
} from './shared';
import {
    STRUCTURAL_VIEWS,
    MIN_CANVAS_ZOOM,
    MAX_CANVAS_ZOOM,
    MIN_SYSML_ZOOM,
    MAX_SYSML_ZOOM,
    ORIENTATION_LABELS,
    STATE_LAYOUT_LABELS,
    STATE_LAYOUT_ICONS,
    VIEW_OPTIONS,
    GENERAL_VIEW_PALETTE,
    GENERAL_VIEW_CATEGORIES,
    GENERAL_VIEW_CONCERNS,
    getGeneralViewEnabledCategoryIds
} from './constants';
import {
    convertToHierarchy,
    isMetadataElement,
    flattenElements,
    extractDocumentation,
    wrapTextToLines,
    truncateLabel,
    countAllElements,
    createLinksFromHierarchy,
    buildEnhancedElementLabel,
    getLibraryChain,
    getLibraryKind,
    slugify
} from './helpers';
import { renderActivityView as renderActivityViewModule } from './renderers/activity';
import { renderStateView as renderStateViewModule } from './renderers/state';
import { renderGeneralViewD3 } from './renderers/generalView';
import { renderIbdView } from './renderers/ibd';
import { createExportHandler } from './export';
import { postJumpToElement } from './jumpToElement';
import { buildGeneralViewGraph } from './graphBuilders';

    let vscode: { postMessage: (msg: unknown) => void };

    function webviewPerf(event: string, data?: Record<string, unknown>) {
        try {
            if (vscode && typeof vscode.postMessage === 'function') {
                vscode.postMessage({ command: 'webviewPerf', event, data });
            }
        } catch {
            // ignore
        }
    }

    export function initializeOrchestrator(api: { postMessage: (msg: unknown) => void }): void {
        vscode = api;
        webviewPerf('visualizer:webviewInitialized');
        vscode.postMessage({ command: 'webviewReady' });
    }

    // ELK Worker URL (must be set before ELK is instantiated)
    const elkWorkerUrl = (typeof window !== 'undefined' && (window).__VIZ_INIT?.elkWorkerUrl) ?? '';
    const enabledViews = Array.isArray((typeof window !== 'undefined' && (window).__VIZ_INIT?.enabledViews))
        ? (window).__VIZ_INIT.enabledViews
        : ['general-view'];
    const experimentalViews = new Set(
        Array.isArray((typeof window !== 'undefined' && (window).__VIZ_INIT?.experimentalViews))
            ? (window).__VIZ_INIT.experimentalViews
            : []
    );
    const verboseWebviewLogging = Boolean((typeof window !== 'undefined' && (window).__VIZ_INIT?.verboseLogging));

    let currentData = null;
    let currentView = 'general-view';  // SysML v2 general-view as default
    let selectedDiagramIndex = 0; // Track currently selected diagram for multi-diagram views
    let selectedDiagramName = null; // Track selected diagram by name to preserve across updates
    let selectedIbdRoot = null; // Block to show as root in Interconnection View (IBD)
    let lastView = currentView;
    let svg = null;
    let g = null;
    let zoom = null;
    let layoutDirection = 'horizontal'; // Universal layout direction: 'horizontal', 'vertical', or 'auto'
    let activityLayoutDirection = 'vertical'; // Action-flow diagrams default to top-down
    let stateLayoutOrientation = 'force'; // State-transition layout: 'horizontal', 'vertical', or 'force'
    let filteredData = null; // Active filter state shared across views
    const defaultGeneralConcernIds = new Set(GENERAL_VIEW_CONCERNS.map((concern) => concern.id));
    let enabledGeneralConcerns = new Set(defaultGeneralConcernIds);
    let isRendering = false;
    let showMetadata = false;
    let showCategoryHeaders = true; // Show category headers in General View
    // Export handler - uses getCurrentData/getViewState for lazy evaluation
    const exportHandler = createExportHandler({
        getCurrentData: () => currentData,
        getViewState: () => ({ currentView }),
        postMessage: (msg) => vscode && vscode.postMessage(msg)
    });

    // ============== LOADING INDICATOR FUNCTIONALITY ==============
    function showLoading(message = 'Rendering diagram...') {
        const overlay = document.getElementById('loading-overlay');
        const textEl = overlay?.querySelector('.loading-text');
        if (overlay) {
            if (textEl) textEl.textContent = message;
            overlay.classList.remove('hidden');
        }
        // Set cursor to wait/hourglass while loading
        document.body.style.cursor = 'wait';
    }

    function hideLoading() {
        const overlay = document.getElementById('loading-overlay');
        if (overlay) {
            overlay.classList.add('hidden');
        }
        // Reset cursor to default
        document.body.style.cursor = '';
    }

    // Send logs to the extension Output channel (works in tests too)
    function webviewLog(level: 'info' | 'warn' | 'error', ...args: any[]) {
        if (level === 'info' && !verboseWebviewLogging) {
            return;
        }
        try {
            if (vscode && typeof vscode.postMessage === 'function') {
                vscode.postMessage({ command: 'webviewLog', level, args });
            }
        } catch {
            // ignore
        }
    }

    function logSelectionTransition(step: string, before: { name: any; index: number }, extra?: Record<string, any>) {
        webviewLog('info', '[GENERAL][selection-transition]', {
            step,
            beforeName: before.name,
            beforeIndex: before.index,
            afterName: selectedDiagramName,
            afterIndex: selectedDiagramIndex,
            ...(extra || {}),
        });
    }

    function updateActivityDebugButtonVisibility(view) {
        // Show legend button only for Cytoscape-based views
        const legendBtn = document.getElementById('legend-btn');
        const legendPopup = document.getElementById('legend-popup');
        if (legendBtn) {
            const cytoscapeViews = ['general', 'general-view'];
            legendBtn.style.display = cytoscapeViews.includes(view) ? 'inline-block' : 'none';
            // Hide popup when switching away from cytoscape views
            if (!cytoscapeViews.includes(view) && legendPopup) {
                legendPopup.style.display = 'none';
                legendBtn.classList.remove('active');
                legendBtn.style.background = '';
                legendBtn.style.color = '';
            }
        }
    }

    // buildEnhancedElementLabel, getLibraryChain, getLibraryKind imported from ./helpers

    // Track manual zoom interactions to preserve user's zoom state
    window.userHasManuallyZoomed = false;

    // Global error handler to catch any JavaScript errors
    window.addEventListener('error', (e) => {
        console.error('JavaScript Error:', e.error?.message || e.message);
    });

    // Track last rendered data to avoid unnecessary re-renders
    let lastDataHash = '';
    let pendingRenderRequest: { view: string; preserveZoomOverride: any; allowDuringResize: boolean } | null = null;

    function ensureVisualizationCanvas(width: number, height: number): void {
        const root = d3.select('#visualization');

        if (!svg || svg.empty()) {
            svg = root
                .append('svg')
                .attr('width', width)
                .attr('height', height);
        } else {
            svg
                .attr('width', width)
                .attr('height', height);
        }

        if (!zoom) {
            zoom = d3.zoom()
                .scaleExtent([MIN_CANVAS_ZOOM, MAX_CANVAS_ZOOM])
                .on('zoom', (event) => {
                    g.attr('transform', event.transform);
                    if (event.sourceEvent) {
                        window.userHasManuallyZoomed = true;
                    }
                });

            svg.call(zoom)
                .on('dblclick.zoom', null)
                .on('wheel.zoom', function(event) {
                    event.preventDefault();

                    window.userHasManuallyZoomed = true;

                    const mouse = d3.pointer(event, this);
                    const currentTransform = d3.zoomTransform(this);
                    const factor = event.deltaY > 0 ? 0.7 : 1.45;
                    const newScale = Math.min(
                        Math.max(currentTransform.k * factor, MIN_CANVAS_ZOOM),
                        MAX_CANVAS_ZOOM
                    );
                    const translateX = mouse[0] - (mouse[0] - currentTransform.x) * (newScale / currentTransform.k);
                    const translateY = mouse[1] - (mouse[1] - currentTransform.y) * (newScale / currentTransform.k);

                    d3.select(this)
                        .transition()
                        .duration(50)
                        .call(zoom.transform, d3.zoomIdentity.translate(translateX, translateY).scale(newScale));
                });
        }

        g = svg.select('g.codex-render-root');
        if (g.empty()) {
            g = svg.append('g').attr('class', 'codex-render-root');
        }
    }

    function populateViewDropdown() {
        const viewDropdownMenu = document.getElementById('view-dropdown-menu');
        if (!viewDropdownMenu) return;
        viewDropdownMenu.innerHTML = '';
        enabledViews.forEach((viewId) => {
            const option = VIEW_OPTIONS[viewId];
            if (!option) return;
            const item = document.createElement('button');
            item.className = 'view-dropdown-item';
            item.setAttribute('data-view', viewId);
            const experimentalBadge = experimentalViews.has(viewId)
                ? '<span class="view-badge">Experimental</span>'
                : '';
            item.innerHTML =
                '<span class="codicon codicon-' + option.icon + ' icon"></span>' +
                '<span class="view-text">' + option.shortLabel + '</span>' +
                experimentalBadge;
            item.addEventListener('click', (e) => {
                const selectedView = e.currentTarget.getAttribute('data-view');
                viewDropdownMenu.classList.remove('show');
                if (selectedView) {
                    changeView(selectedView);
                }
            });
            viewDropdownMenu.appendChild(item);
        });
    }

    function updateViewStatusBanner(activeView) {
        const banner = document.getElementById('view-status-banner');
        if (!banner) return;
        if (activeView === 'interconnection-view' && currentData) {
            const preparedData = prepareDataForView({ ...currentData, selectedIbdRoot }, 'interconnection-view');
            const partCount = Array.isArray(preparedData?.parts) ? preparedData.parts.length : 0;
            const connectorCount = Array.isArray(preparedData?.connectors) ? preparedData.connectors.length : 0;
            const selectedRoot = preparedData?.selectedIbdRoot || selectedIbdRoot || 'the selected block';

            if (partCount > 0 && connectorCount === 0) {
                banner.className = 'experimental';
                banner.textContent = `Interconnection View found no internal connectors for ${selectedRoot}. Try another block if you expected connections here.`;
                banner.style.display = 'block';
                return;
            }
        }
        if (experimentalViews.has(activeView)) {
            const option = VIEW_OPTIONS[activeView];
            banner.className = 'experimental';
            banner.textContent = (option?.label || activeView) + ' is experimental. Layout, routing, or element coverage may still be incomplete.';
            banner.style.display = 'block';
            return;
        }
        banner.className = '';
        banner.textContent = '';
        banner.style.display = 'none';
    }

    window.addEventListener('message', event => {
        const message = event.data;
        switch (message.command) {
            case 'showLoading':
                showLoading(message.message || 'Parsing SysML model...');
                break;
            case 'hideLoading':
                hideLoading();
                break;
            case 'update':
                webviewPerf('visualizer:webviewUpdateReceived', {
                    currentView: message.currentView || currentView,
                    graphNodes: message.graph?.nodes?.length || 0,
                    graphEdges: message.graph?.edges?.length || 0,
                    generalDiagramNodes: message.diagramGeneral?.scene?.generalView?.nodes?.length || 0,
                    generalDiagramEdges: message.diagramGeneral?.scene?.generalView?.edges?.length || 0,
                });
                // Quick hash check - skip render if data unchanged
                const newHash = quickHash({
                    graph: message.graph,
                    diagramGeneral: message.diagramGeneral,
                    diagramInterconnection: message.diagramInterconnection
                });

                if (newHash === lastDataHash && currentData) {
                    // Data unchanged, skip expensive re-render
                    webviewPerf('visualizer:webviewUpdateSkippedUnchanged', {
                        currentView,
                    });
                    hideLoading();
                    return;
                }
                lastDataHash = newHash;

                // Update loading message - parsing is done, now rendering
                showLoading('Rendering diagram...');

                // Preserve selected diagram by name across updates
                // Don't reset selectedDiagramIndex here - let updateDiagramSelector restore it by name
                // selectedDiagramIndex will be updated in updateDiagramSelector if the diagram still exists

                currentData = message;
                filteredData = null; // Reset filter when new data arrives
                webviewLog('info', '[GENERAL][update-message]', {
                    incomingView: message.currentView || null,
                    pendingPackageName: message.pendingPackageName || null,
                    graphNodes: message.graph?.nodes?.length || 0,
                    graphEdges: message.graph?.edges?.length || 0,
                    generalSceneNodes: message.diagramGeneral?.scene?.generalView?.nodes?.length || 0,
                    generalSceneEdges: message.diagramGeneral?.scene?.generalView?.edges?.length || 0,
                });

                // If the extension requested a specific package, apply it
                // before anything else so updateDiagramSelector picks it up.
                if (message.pendingPackageName) {
                    const before = { name: selectedDiagramName, index: selectedDiagramIndex };
                    selectedDiagramName = message.pendingPackageName;
                    selectedDiagramIndex = 0; // Will be corrected by updateDiagramSelector
                    currentView = 'general-view';
                    logSelectionTransition('message.update.pendingPackageName', before, {
                        pendingPackageName: message.pendingPackageName,
                    });
                } else if (message.currentView) {
                    // Use the view state from the message if provided, otherwise keep current
                    currentView = message.currentView;
                }

                updateActiveViewButton(currentView); // Highlight current view
                try {
                    renderVisualization(currentView);
                } catch (e) {
                    console.error('Error in renderVisualization:', e);
                }
                break;
            case 'changeView':
                // Handle view change request from extension
                if (message.view) {
                    changeView(message.view);
                }
                break;
            case 'selectPackage':
                // Switch to General View and select a specific package in the dropdown
                if (message.packageName) {
                    const before = { name: selectedDiagramName, index: selectedDiagramIndex };
                    selectedDiagramName = message.packageName;
                    selectedDiagramIndex = 0; // Will be corrected by updateDiagramSelector
                    logSelectionTransition('message.selectPackage', before, {
                        packageName: message.packageName,
                    });
                    changeView('general-view');
                }
                break;
            case 'setRequirementsVisibleForTest':
                if (typeof message.enabled === 'boolean') {
                    if (message.enabled) {
                        enabledGeneralConcerns.add('requirements');
                    } else {
                        enabledGeneralConcerns.delete('requirements');
                    }
                    renderGeneralChips();
                    if (currentView === 'general-view') {
                        renderVisualization('general-view', false);
                    }
                }
                break;
            case 'export':
                if (message.format === 'png') {
                    exportHandler.exportPNG(message.scale || 2);
                } else if (message.format === 'svg') {
                    exportHandler.exportSVG();
                }
                break;
            case 'highlightElement':
                highlightElementInVisualization(message.elementName, message.skipCentering);
                break;
            case 'requestCurrentView':
                // Send back the current view state
                vscode.postMessage({
                    command: 'currentViewResponse',
                    view: currentView
                });
                break;
            case 'exportDiagramForTest':
                // Export current diagram SVG for testing/review (writes to test-output/diagrams/)
                // Wait for async renderers (elkjs) to finish so exports are not empty.
                {
                    const maxAttempts = 60;
                    let attempts = 0;
                    const tryExportWhenReady = () => {
                        const hasContent = (() => {
                            const svgElement = document.querySelector('#visualization svg');
                            const groupElement = svgElement?.querySelector('g');
                            return !!(svgElement && groupElement && groupElement.childElementCount > 0);
                        })();
                        if (!isRendering && !hasContent && currentData) {
                            // Root cause: export can race before the requested view finished drawing.
                            // Trigger an explicit re-render once before giving up.
                            renderVisualization(currentView);
                        }
                        if ((isRendering || !hasContent) && attempts < maxAttempts) {
                            attempts += 1;
                            setTimeout(tryExportWhenReady, 150);
                            return;
                        }
                        const svgString = exportHandler.getSvgStringForExport();
                        vscode.postMessage({
                            command: 'testDiagramExported',
                            viewId: currentView,
                            svgString: svgString ?? ''
                        });
                    };
                    tryExportWhenReady();
                }
                break;
        }
    });

    // Update panel dimensions display
    function updateDimensionsDisplay() {
        const vizElement = document.getElementById('visualization');
        if (vizElement) {
            const width = Math.round(vizElement.clientWidth);
            const height = Math.round(vizElement.clientHeight);
            const statusText = document.getElementById('status-text');
            if (statusText) {
                statusText.innerHTML = 'Panel: ' + width + ' x ' + height + 'px - Resize via VS Code panel';
                const statusBar = document.getElementById('status-bar');
                if (statusBar) statusBar.style.display = 'flex';
                setTimeout(() => {
                    if (statusText.innerHTML?.includes('Panel:')) {
                        statusText.textContent = 'Ready';
                    }
                }, 3000);
            }
        }
    }

    // Resize handler - only triggers after user stops dragging
    let resizeTimeout;
    let lastRenderedWidth = 0;
    let lastRenderedHeight = 0;

    function handleResize() {
        const vizElement = document.getElementById('visualization');
        if (!vizElement) return;

        const currentWidth = vizElement.clientWidth;
        const currentHeight = vizElement.clientHeight;

        // Clear any pending resize
        clearTimeout(resizeTimeout);

        // Update dimensions display immediately during drag
        updateDimensionsDisplay();

        // Wait until resize stops before re-rendering
        resizeTimeout = setTimeout(() => {
            if (currentWidth !== lastRenderedWidth || currentHeight !== lastRenderedHeight) {
                lastRenderedWidth = currentWidth;
                lastRenderedHeight = currentHeight;

                if (currentData && !isRendering) {
                    renderVisualization(currentView, null, true);
                }
            }
        }, 500);
    }

    // Add keyboard shortcut to show dimensions (Ctrl+D)
    window.addEventListener('keydown', (event) => {
        if (event.ctrlKey && event.key === 'd') {
            event.preventDefault();
            updateDimensionsDisplay();
        }
    });

    // Use ResizeObserver for container size changes (more reliable than window resize)
    if (window.ResizeObserver) {
        const resizeObserver = new ResizeObserver(entries => {
            // Use requestAnimationFrame to avoid layout thrashing
            requestAnimationFrame(() => {
                for (let entry of entries) {
                    if (entry.target.id === 'visualization') {
                        handleResize();
                        break;
                    }
                }
            });
        });

        // Start observing when DOM is ready
        setTimeout(() => {
            const visualizationElement = document.getElementById('visualization');
            if (visualizationElement) {
                // Initialize lastRenderedWidth/Height to prevent spurious re-render on first observe
                lastRenderedWidth = visualizationElement.clientWidth;
                lastRenderedHeight = visualizationElement.clientHeight;
                resizeObserver.observe(visualizationElement);
            }
        }, 100);
    }

    // Also listen to window resize events as a fallback
    // This catches cases where the VS Code panel is resized
    window.addEventListener('resize', () => {
        requestAnimationFrame(() => {
            handleResize();
        });
    });

    // Inline editing for element names in General View
    var activeInlineEdit = null;

    function startInlineEdit(nodeG, elementName, x, y, width) {
        // Cancel any existing inline edit
        if (activeInlineEdit) {
            cancelInlineEdit();
        }

        // Find the name text element within this node
        var nameText = nodeG.select('.node-name-text');
        if (nameText.empty()) {
            // Try to find any text that matches the element name
            nodeG.selectAll('text').each(function() {
                var textEl = d3.select(this);
                if (textEl.text() === elementName || textEl.attr('data-element-name') === elementName) {
                    nameText = textEl;
                }
            });
        }

        if (nameText.empty()) return;

        // Get the text element's position within the node
        var textY = parseFloat(nameText.attr('y')) || 31;
        var fontSize = nameText.style('font-size') || '11px';

        // Hide the original text
        nameText.style('visibility', 'hidden');

        // Create input container inside the node itself (not in main g)
        // Position it to match the text location
        var inputHeight = 20;
        var inputY = textY - inputHeight / 2 - 3;
        var inputPadding = 8;

        // Create foreignObject inside the node group for proper positioning
        var fo = nodeG.append('foreignObject')
            .attr('class', 'inline-edit-container')
            .attr('x', inputPadding)
            .attr('y', inputY)
            .attr('width', width - inputPadding * 2)
            .attr('height', inputHeight + 4);

        var input = fo.append('xhtml:input')
            .attr('type', 'text')
            .attr('value', elementName)
            .attr('class', 'inline-edit-input')
            .style('width', '100%')
            .style('height', inputHeight + 'px')
            .style('font-size', fontSize)
            .style('font-weight', 'bold')
            .style('font-family', 'var(--vscode-editor-font-family)')
            .style('text-align', 'center')
            .style('padding', '2px 4px')
            .style('border', '1px solid var(--vscode-focusBorder)')
            .style('border-radius', '3px')
            .style('background', 'var(--vscode-input-background)')
            .style('color', 'var(--vscode-input-foreground)')
            .style('outline', 'none')
            .style('box-sizing', 'border-box')
            .style('box-shadow', '0 0 0 1px var(--vscode-focusBorder)');

        // Store reference to active edit
        activeInlineEdit = {
            foreignObject: fo,
            input: input,
            nameText: nameText,
            originalName: elementName,
            nodeG: nodeG
        };

        // Focus and select all text
        var inputNode = input.node();
        setTimeout(function() {
            inputNode.focus();
            inputNode.select();
        }, 10);

        // Handle keyboard events
        input.on('keydown', function(event) {
            if (event.key === 'Enter') {
                event.preventDefault();
                commitInlineEdit();
            } else if (event.key === 'Escape') {
                event.preventDefault();
                cancelInlineEdit();
            }
            event.stopPropagation();
        });

        // Handle blur (clicking outside)
        input.on('blur', function() {
            // Small delay to allow Enter key to process first
            setTimeout(function() {
                if (activeInlineEdit) {
                    cancelInlineEdit();
                }
            }, 100);
        });

        // Prevent click from bubbling to node
        input.on('click', function(event) {
            event.stopPropagation();
        });
    }

    function commitInlineEdit() {
        if (!activeInlineEdit) return;

        var newName = activeInlineEdit.input.node().value.trim();
        var oldName = activeInlineEdit.originalName;

        // Clean up UI
        activeInlineEdit.nameText.style('visibility', 'visible');
        activeInlineEdit.foreignObject.remove();

        if (newName && newName !== oldName) {
            // Update the text display immediately for responsiveness
            activeInlineEdit.nameText.text(newName);

            // Send rename command to extension
            vscode.postMessage({
                command: 'renameElement',
                oldName: oldName,
                newName: newName
            });
        }

        activeInlineEdit = null;
    }

    function cancelInlineEdit() {
        if (!activeInlineEdit) return;

        // Restore original text visibility
        activeInlineEdit.nameText.style('visibility', 'visible');
        activeInlineEdit.foreignObject.remove();
        activeInlineEdit = null;
    }

    function clearVisualHighlights() {
        // Remove visual highlights without refreshing the view
        // Current highlight mechanism: only node border rects.
        d3.selectAll('.outline-highlighted').each(function() {
            const t = d3.select(this);
            t.classed('outline-highlighted', false);
            const origStroke = t.attr('data-original-stroke');
            const origWidth = t.attr('data-original-width');
            if (origStroke) {
                t.style('stroke', origStroke);
            } else {
                t.style('stroke', null);
            }
            if (origWidth) {
                t.style('stroke-width', origWidth);
            } else {
                t.style('stroke-width', null);
            }
        });

        // Back-compat: if any stale highlighted-element classes exist, remove them.
        d3.selectAll('.highlighted-element').classed('highlighted-element', false);
        d3.selectAll('.selected').classed('selected', false);

        // Restore original stroke/width from saved data attributes on all node backgrounds
        d3.selectAll('.node-group').style('opacity', null);
        d3.selectAll('.node-group .node-background').each(function() {
            const el = d3.select(this);
            el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
            el.style('stroke-width', el.attr('data-original-width') || '1px');
        });
        d3.selectAll('.general-node .node-background').each(function() {
            const el = d3.select(this);
            el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
            el.style('stroke-width', el.attr('data-original-width') || '2px');
        });
        d3.selectAll('.state-node .node-background').each(function() {
            const el = d3.select(this);
            el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
            el.style('stroke-width', el.attr('data-original-width') || '2px');
        });
        d3.selectAll('.activity-action .node-background').each(function() {
            const el = d3.select(this);
            el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
            el.style('stroke-width', el.attr('data-original-width') || '2px');
        });
        d3.selectAll('.ibd-part rect:first-child').each(function() {
            const el = d3.select(this);
            const orig = el.attr('data-original-stroke');
            if (orig) {
                el.style('stroke', orig);
                el.style('stroke-width', el.attr('data-original-width') || '2px');
            }
        });
        d3.selectAll('.graph-node-group').style('opacity', null);
        d3.selectAll('.hierarchy-cell').style('opacity', null);
    }

    function renderGeneralChips(typeStats = {}) {
        const container = document.getElementById('general-chips');
        if (!container) return;
        container.innerHTML = '';
        if (currentView !== 'general-view') return;

        const concernStats = new Map();
        GENERAL_VIEW_CONCERNS.forEach((concern) => {
            const count = concern.categories.reduce((total, categoryId) => total + (Number(typeStats?.[categoryId]) || 0), 0);
            concernStats.set(concern.id, count);
        });

        const concernRow = document.createElement('div');
        concernRow.className = 'general-presets';

        GENERAL_VIEW_CONCERNS.forEach((concern) => {
            const button = document.createElement('button');
            button.type = 'button';
            button.className = 'general-preset-btn';
            if (enabledGeneralConcerns.has(concern.id)) {
                button.classList.add('active');
            }
            button.style.setProperty('--general-chip-accent', concern.color);
            button.setAttribute('data-concern-id', concern.id);
            button.setAttribute(
                'title',
                enabledGeneralConcerns.has(concern.id)
                    ? `Hide ${concern.label.toLowerCase()} elements`
                    : `Show ${concern.label.toLowerCase()} elements`
            );
            const count = concernStats.get(concern.id);
            button.innerHTML = `<span>${concern.label}</span><span class="general-preset-count">${count}</span>`;
            button.addEventListener('click', async () => {
                if (enabledGeneralConcerns.has(concern.id)) {
                    enabledGeneralConcerns.delete(concern.id);
                } else {
                    enabledGeneralConcerns.add(concern.id);
                }
                renderGeneralChips(typeStats);
                if (currentView === 'general-view') {
                    await renderVisualization('general-view', false);
                }
            });
            concernRow.appendChild(button);
        });

        const actionsRow = document.createElement('div');
        actionsRow.className = 'general-presets';

        const showAllButton = document.createElement('button');
        showAllButton.type = 'button';
        showAllButton.className = 'general-preset-btn general-preset-btn-secondary';
        const allEnabled = enabledGeneralConcerns.size === defaultGeneralConcernIds.size;
        if (allEnabled) {
            showAllButton.classList.add('active');
        }
        showAllButton.textContent = 'Show all';
        showAllButton.title = 'Enable all General View concerns';
        showAllButton.addEventListener('click', async () => {
            enabledGeneralConcerns = new Set(defaultGeneralConcernIds);
            renderGeneralChips(typeStats);
            if (currentView === 'general-view') {
                await renderVisualization('general-view', false);
            }
        });
        actionsRow.appendChild(showAllButton);

        const hideAllButton = document.createElement('button');
        hideAllButton.type = 'button';
        hideAllButton.className = 'general-preset-btn general-preset-btn-secondary';
        const allDisabled = enabledGeneralConcerns.size === 0;
        if (allDisabled) {
            hideAllButton.classList.add('active');
        }
        hideAllButton.textContent = 'Hide all';
        hideAllButton.title = 'Disable all General View concerns';
        hideAllButton.addEventListener('click', async () => {
            enabledGeneralConcerns = new Set();
            renderGeneralChips(typeStats);
            if (currentView === 'general-view') {
                await renderVisualization('general-view', false);
            }
        });
        actionsRow.appendChild(hideAllButton);

        container.appendChild(concernRow);
        container.appendChild(actionsRow);
    }

    function getCategoryForType(typeLower) {
        for (const cat of GENERAL_VIEW_CATEGORIES) {
            if (cat.keywords.some(kw => typeLower.includes(kw))) {
                return cat.id;
            }
        }
        return 'other';
    }

    function buildGeneralViewGraphForView(dataOrElements, relationships = []) {
        return buildGeneralViewGraph(dataOrElements, relationships, {
            enabledGeneralCategories: getGeneralViewEnabledCategoryIds(enabledGeneralConcerns),
            webviewLog
        });
    }

    function highlightElementInVisualization(elementName, skipCentering = false) {
        // Remove any existing highlights without refreshing
        clearVisualHighlights();

        // Find and highlight the element based on current view
        let targetElement = null;
        let elementData = null;

        if (false) {  // tree view removed
            // In tree view, find by node data
            d3.selectAll('.node-group').each(function(d) {
                if (d && d.data && d.data.name === elementName) {
                    targetElement = d3.select(this);
                    elementData = d.data;
                }
            });
        } else if (currentView === 'general-view') {
            // In General View (Cytoscape), find nodes by data-element-name attribute
            d3.selectAll('.general-node').each(function() {
                const node = d3.select(this);
                const nodeName = node.attr('data-element-name');
                if (nodeName === elementName) {
                    targetElement = node;
                    elementData = { name: elementName, type: 'element' };
                }
            });
        } else if (currentView === 'interconnection-view') {
            // In Interconnection View (ibd), find parts by data-element-name attribute
            d3.selectAll('.ibd-part').each(function() {
                const partG = d3.select(this);
                const partName = partG.attr('data-element-name');
                if (partName === elementName) {
                    targetElement = partG;
                    elementData = { name: elementName, type: 'part' };
                }
            });
        } else if (currentView === 'state-transition-view') {
            d3.selectAll('.state-node').each(function() {
                const stateNode = d3.select(this);
                const stateName = stateNode.attr('data-element-name');
                if (stateName === elementName) {
                    targetElement = stateNode;
                    elementData = { name: elementName, type: 'state' };
                }
            });
        }

        if (targetElement && elementData) {
            // Add highlight class for styling
            targetElement.classed('highlighted-element', true);

            // Apply direct style to node-background for immediate visual feedback
            // This works for general-node, ibd-part, and node-group elements
            targetElement.select('.node-background')
                .style('stroke', '#FFD700')
                .style('stroke-width', '3px');
            // For IBD parts, the rect is a direct child
            targetElement.select('rect')
                .style('stroke', '#FFD700')
                .style('stroke-width', '3px');

            // Update status bar
            const statusBar = document.getElementById('status-bar');
            const statusText = document.getElementById('status-text');
            if (statusText) statusText.textContent = 'Selected: ' + elementData.name + ' [' + elementData.type + ']';
            if (statusBar) statusBar.style.display = 'flex';

            // Only center the view if not skipping (i.e., click came from text editor, not diagram)
            if (!skipCentering) {
                const bbox = targetElement.node().getBBox();
                const centerX = bbox.x + bbox.width / 2;
                const centerY = bbox.y + bbox.height / 2;

                const transform = d3.zoomTransform(svg.node());
                const scale = Math.min(1.5, transform.k); // Don't zoom in too much
                const translateX = (svg.node().clientWidth / 2) - (centerX * scale);
                const translateY = (svg.node().clientHeight / 2) - (centerY * scale);

                svg.transition()
                    .duration(750)
                    .call(zoom.transform, d3.zoomIdentity.translate(translateX, translateY).scale(scale));
            }
        }
    }

    function changeView(view) {
        // Clear any existing resize timeout to avoid conflicts
        clearTimeout(resizeTimeout);

        // Reset manual zoom flag so the new view auto-fits
        window.userHasManuallyZoomed = false;

        const proceedWithRender = () => {
            currentView = view;

            // Reset diagram selection when switching views
            const before = { name: selectedDiagramName, index: selectedDiagramIndex };
            selectedDiagramIndex = 0;
            logSelectionTransition('changeView.resetIndex', before, { view });

            // Notify the panel that the view has changed
            vscode.postMessage({
                command: 'viewChanged',
                view: view
            });

            // Update button highlighting to show active view
            updateActiveViewButton(view);

            // Show/hide activity debug button based on view
            updateActivityDebugButtonVisibility(view);

            // Small delay to allow UI to update before rendering
            setTimeout(() => {
                renderVisualization(view);
            }, 50);

            lastView = view;
        };

        if (shouldAnimateStructuralTransition(view)) {
            animateStructuralTransition(proceedWithRender);
        } else {
            proceedWithRender();
        }
    }

    function shouldAnimateStructuralTransition(nextView) {
        return STRUCTURAL_VIEWS.has(lastView) &&
            STRUCTURAL_VIEWS.has(nextView) &&
            nextView !== lastView;
    }

    function animateStructuralTransition(callback) {
        const viz = document.getElementById('visualization');
        if (!viz) {
            callback();
            return;
        }

        viz.classList.add('structural-transition-active', 'fade-out');

        // Allow fade-out to complete before rendering the next view
        setTimeout(() => {
            callback();

            // Trigger fade-in on next frame so DOM has new content
            requestAnimationFrame(() => {
                viz.classList.remove('fade-out');
                viz.classList.add('fade-in');

                setTimeout(() => {
                    viz.classList.remove('fade-in', 'structural-transition-active');
                }, 350);
            });
        }, 220);
    }

    function updateActiveViewButton(activeView) {
        // Show/hide appropriate chip containers based on active view
        const generalChips = document.getElementById('general-chips');
        if (generalChips) {
            generalChips.style.display = activeView === 'general-view' ? 'flex' : 'none';
        }

        // Show/hide layout direction button for specific views
        const layoutDirBtn = document.getElementById('layout-direction-btn');
        if (layoutDirBtn) {
            const showLayoutBtn = false;
            layoutDirBtn.style.display = showLayoutBtn ? 'inline-flex' : 'none';
        }

        const dropdownButton = document.getElementById('view-dropdown-btn');
        const dropdownConfig = VIEW_OPTIONS[activeView];
        if (dropdownButton) {
            if (dropdownConfig) {
                dropdownButton.classList.add('view-btn-active');
                dropdownButton.innerHTML = '<span class="codicon codicon-chevron-down" style="margin-right: 2px;"></span><span>' + dropdownConfig.label + '</span>';
            } else {
                dropdownButton.classList.remove('view-btn-active');
                dropdownButton.innerHTML = '<span class="codicon codicon-chevron-down" style="margin-right: 2px;"></span><span>Views</span>';
            }
        }

        document.querySelectorAll('.view-dropdown-item').forEach(item => {
            const isMatch = item.getAttribute('data-view') === activeView;
            item.classList.toggle('active', isMatch);
        });

        // Show/hide state layout button based on view
        updateLayoutDirectionButton(activeView);

        // Update diagram selector visibility and content based on view
        updateDiagramSelector(activeView);
        updateViewStatusBanner(activeView);
    }

    // Update diagram selector for multi-diagram views
    function updateDiagramSelector(activeView) {
        const pkgDropdown = document.getElementById('pkg-dropdown');
        const pkgMenu = document.getElementById('pkg-dropdown-menu');
        const pkgLabel = document.getElementById('pkg-dropdown-label');
        const pkgSummary = document.getElementById('pkg-dropdown-summary');

        const setSelectorSummary = (text) => {
            if (!pkgSummary) return;
            if (text) {
                pkgSummary.textContent = text;
                pkgSummary.classList.add('visible');
                pkgSummary.title = text;
            } else {
                pkgSummary.textContent = '';
                pkgSummary.classList.remove('visible');
                pkgSummary.removeAttribute('title');
            }
        };

        if (!pkgDropdown || !pkgMenu || !currentData) {
            if (pkgDropdown) pkgDropdown.style.display = 'none';
            setSelectorSummary('');
            return;
        }

        // Determine if this view supports multiple diagrams
        let diagrams = [];
        let labelText = 'Package';

        if (activeView === 'general-view') {
            // For General View, extract top-level packages
            const elements = currentData?.elements ?? (currentData?.graph ? graphToElementTree(currentData.graph) : []);

            const packagesArray = [];
            const seenPackages = new Set();

            // Always add "All Packages" option first
            diagrams.push({ name: 'All Packages', element: null, isAll: true });

            // Find all packages recursively up to depth 3 (includes nested packages like PartsTree, ActionTree, etc.)
            function findPackages(elementList, depth = 0) {
                elementList.forEach(el => {
                    const typeLower = (el.type || '').toLowerCase();
                    if (typeLower.includes('package') && !seenPackages.has(el.name)) {
                        seenPackages.add(el.name);
                        packagesArray.push({ name: el.name, element: el });
                    }
                    // Recurse into all children to find nested packages
                    if (el.children && el.children.length > 0) {
                        findPackages(el.children, depth + 1);
                    }
                });
            }

            findPackages(elements);

            // Add packages to diagrams array
            packagesArray.forEach(pkg => {
                diagrams.push(pkg);
            });
            webviewLog('info', '[GENERAL][selector]', {
                selectedDiagramName,
                selectedDiagramIndex,
                packageCount: packagesArray.length,
                packageNames: packagesArray.map((p: any) => p?.name).filter(Boolean),
            });

            labelText = 'Package';
        } else if (activeView === 'action-flow-view') {
            // Get activity diagrams
            const preparedData = prepareDataForView(currentData, 'action-flow-view');
            diagrams = (preparedData?.diagrams || []).map((diagram: any) => ({
                ...diagram,
                label: diagram?.name || 'Action Flow',
            }));
            labelText = 'Action Flow';
        } else if (activeView === 'state-transition-view') {
            const preparedData = prepareDataForView(currentData, 'state-transition-view');
            diagrams = (preparedData?.stateMachines || []).map((machine: any) => ({
                name: machine.name,
                element: machine.container || null,
                label: machine.name,
            }));
            labelText = 'State Machine';
        } else if (activeView === 'interconnection-view') {
            const preparedData = prepareDataForView({ ...currentData, selectedIbdRoot }, 'interconnection-view');
            const candidates = preparedData?.ibdRootCandidates || [];
            const rootSummaries = preparedData?.ibdRootSummaries || [];
            if (candidates.length > 0) {
                diagrams = candidates.map(name => {
                    return { name, label: name };
                });
                labelText = 'Block';
                const preferredRoot = preparedData?.selectedIbdRoot && candidates.indexOf(preparedData.selectedIbdRoot) >= 0
                    ? preparedData.selectedIbdRoot
                    : (selectedIbdRoot && candidates.indexOf(selectedIbdRoot) >= 0 ? selectedIbdRoot : candidates[0] || null);
                const before = { name: selectedDiagramName, index: selectedDiagramIndex };
                selectedDiagramIndex = preferredRoot ? candidates.indexOf(preferredRoot) : 0;
                selectedDiagramName = preferredRoot;
                selectedIbdRoot = selectedDiagramName;
                logSelectionTransition('selector.interconnection.preferredRoot', before, {
                    preferredRoot,
                    candidateCount: candidates.length,
                });
            } else {
                diagrams = [{ name: 'Default', element: null }];
                labelText = 'Block';
            }
        }

        // Show/hide selector based on number of diagrams
        if (diagrams.length <= 1) {
            pkgDropdown.style.display = 'none';
            const before = { name: selectedDiagramName, index: selectedDiagramIndex };
            selectedDiagramIndex = 0;
            selectedDiagramName = diagrams.length === 1 ? diagrams[0].name : null;
            logSelectionTransition('selector.single-option-reset', before, {
                activeView,
                diagramsLength: diagrams.length,
            });
            setSelectorSummary('');
            return;
        }

        pkgDropdown.style.display = 'flex';
        if (pkgLabel) pkgLabel.textContent = labelText;

        // Try to restore selection by name if we have a previously selected diagram
        if (selectedDiagramName) {
            const matchingIndex = diagrams.findIndex(d => d.name === selectedDiagramName);
            if (matchingIndex >= 0) {
                const before = { name: selectedDiagramName, index: selectedDiagramIndex };
                selectedDiagramIndex = matchingIndex;
                if (pkgLabel) pkgLabel.textContent = diagrams[matchingIndex]?.label || selectedDiagramName;
                logSelectionTransition('selector.restore-hit', before, { matchingIndex, selectedDiagramName });
                webviewLog('info', '[GENERAL][selector-restore-hit]', { selectedDiagramName, matchingIndex });
            } else {
                // Diagram no longer exists, reset to first
                const previousSelection = selectedDiagramName;
                const before = { name: selectedDiagramName, index: selectedDiagramIndex };
                selectedDiagramIndex = 0;
                selectedDiagramName = diagrams[0]?.name || null;
                logSelectionTransition('selector.restore-miss-fallback', before, { previousSelection });
                webviewLog('warn', '[GENERAL][selector-restore-miss]', { previousSelection, fallbackSelection: selectedDiagramName });
            }
        } else {
            // No previous selection, initialize with first diagram
            const before = { name: selectedDiagramName, index: selectedDiagramIndex };
            selectedDiagramName = diagrams[0]?.name || null;
            logSelectionTransition('selector.init-first', before);
            webviewLog('info', '[GENERAL][selector-init]', { selectedDiagramName });
        }

        if (activeView === 'interconnection-view') {
            setSelectorSummary('');
        } else {
            setSelectorSummary('');
        }

        const selectedDiagram = diagrams[selectedDiagramIndex];
        if (pkgLabel && selectedDiagram) {
            pkgLabel.textContent = selectedDiagram.label || selectedDiagram.name || labelText;
        }

        // Populate dropdown menu
        pkgMenu.innerHTML = '';
        diagrams.forEach((d, idx) => {
            const item = document.createElement('button');
            item.className = 'view-dropdown-item';
            item.textContent = d.label || d.name || 'Diagram ' + (idx + 1);
            if (idx === selectedDiagramIndex) item.classList.add('active');
            item.addEventListener('click', function() {
                const before = { name: selectedDiagramName, index: selectedDiagramIndex };
                selectedDiagramIndex = idx;
                selectedDiagramName = d.name;
                if (activeView === 'interconnection-view') selectedIbdRoot = d.name;
                logSelectionTransition('selector.user-click', before, { selectedName: d.name, selectedIdx: idx });
                // Selecting a different diagram should open it fit-to-window.
                window.userHasManuallyZoomed = false;
                // Update active state
                pkgMenu.querySelectorAll('.view-dropdown-item').forEach(i => i.classList.remove('active'));
                item.classList.add('active');
                // Update label
                if (pkgLabel) pkgLabel.textContent = d.label || d.name;
                if (activeView === 'interconnection-view') {
                    setSelectorSummary('');
                }
                // Close menu
                pkgMenu.classList.remove('show');
                // Re-render
                renderVisualization(currentView);
            });
            pkgMenu.appendChild(item);
        });

        // Ensure selected index is valid
        if (selectedDiagramIndex >= diagrams.length) {
            const before = { name: selectedDiagramName, index: selectedDiagramIndex };
            selectedDiagramIndex = 0;
            selectedDiagramName = diagrams[0]?.name || null;
            logSelectionTransition('selector.index-out-of-range-fallback', before, { diagramsLength: diagrams.length });
            if (pkgLabel && diagrams[0]) {
                pkgLabel.textContent = diagrams[0].label || diagrams[0].name || labelText;
            }
        }
    }

    // Universal layout direction labels and icons
    const LAYOUT_DIRECTION_LABELS = {
        'horizontal': 'Left → Right',
        'vertical': 'Top → Down',
        'auto': 'Auto Layout'
    };
    const LAYOUT_DIRECTION_ICONS = {
        'horizontal': 'codicon-arrow-right',
        'vertical': 'codicon-arrow-down',
        'auto': 'codicon-editor-layout'
    };

    function updateLayoutDirectionButton(activeView) {
        const layoutBtn = document.getElementById('layout-direction-btn');
        if (layoutBtn) {
            // Use activity-specific direction for activity view
            const effectiveDirection = activeView === 'action-flow-view' ? activityLayoutDirection : layoutDirection;
            const iconClass = LAYOUT_DIRECTION_ICONS[effectiveDirection] || 'codicon-arrow-right';
            const label = LAYOUT_DIRECTION_LABELS[effectiveDirection] || 'Left → Right';
            layoutBtn.innerHTML = '<span class="codicon ' + iconClass + '"></span> ' + label;

            // Update tooltip to show next option
            const nextMode = getNextLayoutDirection(effectiveDirection);
            const nextLabel = LAYOUT_DIRECTION_LABELS[nextMode];
            layoutBtn.title = 'Switch to ' + nextLabel;

            // Sync with view-specific orientations for backwards compatibility
            stateLayoutOrientation = layoutDirection === 'auto' ? 'force' : layoutDirection;
        }
    }

    function getNextLayoutDirection(current) {
        const modes = ['horizontal', 'vertical', 'auto'];
        const currentIndex = modes.indexOf(current);
        return modes[(currentIndex + 1) % modes.length];
    }

    function toggleLayoutDirection() {
        // Use activity-specific direction for activity view
        if (currentView === 'action-flow-view') {
            activityLayoutDirection = getNextLayoutDirection(activityLayoutDirection);
        } else {
            layoutDirection = getNextLayoutDirection(layoutDirection);
        }
        updateLayoutDirectionButton(currentView);
        // Re-render the current view
        renderVisualization(currentView);
    }

    function updateStateLayoutButton(activeView) {
        // Legacy function - now handled by updateLayoutDirectionButton
    }

    function updateUsecaseLayoutButton(activeView) {
        // Legacy function - now handled by updateLayoutDirectionButton
    }

    function getNextLayoutMode(current) {
        const modes = ['horizontal', 'vertical', 'force'];
        const currentIndex = modes.indexOf(current);
        return modes[(currentIndex + 1) % modes.length];
    }

    function toggleStateLayout() {
        layoutDirection = getNextLayoutDirection(layoutDirection);
        stateLayoutOrientation = layoutDirection === 'auto' ? 'force' : layoutDirection;
        updateLayoutDirectionButton(currentView);
        // Re-render the state view
        if (currentView === 'state-transition-view') {
            renderVisualization('state-transition-view');
        }
    }

    function toggleUsecaseLayout() {
        layoutDirection = getNextLayoutDirection(layoutDirection);
        updateLayoutDirectionButton(currentView);
        // Re-render the usecase view
        if (false) {  // usecase view removed
        }
    }

    // Make functions globally accessible for HTML onclick handlers
    window.changeView = changeView;

    async function renderVisualization(view, preserveZoomOverride = null, allowDuringResize = false) {
        if (!currentData) {
            return;
        }

        const renderStartedAt = Date.now();

        if (isRendering) {
            // A render is in-flight; queue the latest request so we don't lose updates
            // when switching folders/projects quickly.
            pendingRenderRequest = { view, preserveZoomOverride, allowDuringResize };
            webviewPerf('visualizer:webviewRenderQueued', {
                view,
                allowDuringResize,
            });
            return;
        }

        // Only reset manual zoom flag when the view type actually changes
        // This preserves zoom state when the same view is re-rendered due to data changes
        const viewChanged = view !== lastView;
        if (viewChanged) {
            window.userHasManuallyZoomed = false;
        }

        // Use filtered data if available, otherwise use original data
        let baseData = filteredData || currentData;

        // Apply package filter for views that support it (excluding elk which handles it internally).
        // Use selected diagram NAME (not index) so filtering remains stable even if package order
        // differs between selector construction and render-time element traversal.
        const hasSpecificPackageSelection =
            !!selectedDiagramName && selectedDiagramName !== 'All Packages';
        if (view === 'general-view') {
            webviewLog('info', '[GENERAL][render-start]', {
                selectedDiagramName,
                selectedDiagramIndex,
                hasSpecificPackageSelection,
                graphNodes: baseData?.graph?.nodes?.length || 0,
                graphEdges: baseData?.graph?.edges?.length || 0,
            });
        }
        if (hasSpecificPackageSelection &&
            (view === 'interconnection-view' || view === 'general-view')) {

            const elements = baseData?.elements ?? (baseData?.graph ? graphToElementTree(baseData.graph) : []);
            const packagesArray = [];
            const seenPackages = new Set();

            // Find all packages recursively (SysML v2 spec allows nested packages up to depth 3)
            function findPackagesForRender(elementList, depth = 0) {
                (elementList || []).forEach(el => {
                    const typeLower = (el.type || '').toLowerCase();
                    if (typeLower.includes('package') && depth <= 3 && !seenPackages.has(el.name)) {
                        seenPackages.add(el.name);
                        packagesArray.push({ name: el.name, element: el });
                    }
                    // Recurse into all children to find nested packages
                    if (el.children && el.children.length > 0) {
                        findPackagesForRender(el.children, depth + 1);
                    }
                });
            }

            findPackagesForRender(elements);

            const selectedPackage = packagesArray.find((p: any) => p.name === selectedDiagramName)
                || (selectedDiagramIndex > 0 ? packagesArray[selectedDiagramIndex - 1] : null);
            if (selectedPackage) {
                if (view === 'general-view') {
                    webviewLog('info', '[GENERAL][filter-target]', {
                        selectedDiagramName,
                        selectedDiagramIndex,
                        selectedPackageName: selectedPackage?.name,
                        packageCount: packagesArray.length,
                    });
                }

                // Create filtered baseData with only this package's contents
                if (selectedPackage.element) {
                    let nextBaseData = {
                        ...baseData,
                        elements: [selectedPackage.element]
                    };

                    // General View renders from graph/generalViewGraph, not elements.
                    // Build a package-scoped subgraph so dropdown selection actually
                    // affects what gets rendered.
                    if (view === 'general-view' && baseData?.graph?.nodes) {
                        const selectedElementIds = new Set<string>();
                        const collectElementIds = (el: any) => {
                            if (!el) return;
                            if (typeof el.id === 'string' && el.id.length > 0) {
                                selectedElementIds.add(el.id);
                            }
                            if (Array.isArray(el.children)) {
                                el.children.forEach((child: any) => collectElementIds(child));
                            }
                        };
                        collectElementIds(selectedPackage.element);

                        if (selectedElementIds.size > 0) {
                            const sourceGraph = (baseData.generalViewGraph?.nodes?.length || baseData.generalViewGraph?.edges?.length)
                                ? baseData.generalViewGraph
                                : baseData.graph;
                            const filteredNodes = (sourceGraph?.nodes || []).filter((n: any) => selectedElementIds.has(n.id));
                            const filteredEdges = (sourceGraph?.edges || []).filter((e: any) =>
                                selectedElementIds.has(e.source) && selectedElementIds.has(e.target)
                            );
                            const filteredGraph = { nodes: filteredNodes, edges: filteredEdges };
                            if (view === 'general-view') {
                                webviewLog('info', '[GENERAL][filter-applied]', {
                                    selectedPackageName: selectedPackage?.name,
                                    selectedElementIds: selectedElementIds.size,
                                    beforeNodes: sourceGraph?.nodes?.length || 0,
                                    beforeEdges: sourceGraph?.edges?.length || 0,
                                    afterNodes: filteredNodes.length,
                                    afterEdges: filteredEdges.length,
                                });
                            }

                            nextBaseData = {
                                ...nextBaseData,
                                graph: filteredGraph,
                                generalViewGraph: filteredGraph
                            };
                        }
                    }

                    baseData = nextBaseData;
                }
            } else if (view === 'general-view') {
                webviewLog('warn', '[GENERAL][filter-miss]', {
                    selectedDiagramName,
                    selectedDiagramIndex,
                    packageNames: packagesArray.map((p: any) => p?.name).filter(Boolean),
                });
            }
        }

        const dataForPrepare = view === 'interconnection-view' ? { ...baseData, selectedIbdRoot } : baseData;
        const prepareStartedAt = Date.now();
        const dataToRender = prepareDataForView(dataForPrepare, view);
        const prepareMs = Date.now() - prepareStartedAt;
        if (view === 'interconnection-view') {
            const ibd = (dataForPrepare as any)?.ibd;
            const deepPropulsion = Array.isArray(ibd?.parts)
                ? ibd.parts
                    .map((p: any) => p?.qualifiedName)
                    .filter((qn: any) => typeof qn === 'string' && qn.includes('.propulsion.') && qn.split('.').length >= 4)
                : [];
            webviewLog(
                'info',
                '[IBD] prepare',
                {
                    hasIbd: !!ibd,
                    defaultRoot: ibd?.defaultRoot ?? null,
                    rootCandidates: Array.isArray(ibd?.rootCandidates) ? ibd.rootCandidates : null,
                    selectedIbdRoot_state: selectedIbdRoot ?? null,
                    selectedIbdRoot_prepared: (dataToRender as any)?.selectedIbdRoot ?? null,
                    partsCount: Array.isArray((dataToRender as any)?.parts) ? (dataToRender as any).parts.length : null,
                    connectorsCount: Array.isArray((dataToRender as any)?.connectors) ? (dataToRender as any).connectors.length : null,
                    deepPropulsionCount: deepPropulsion.length,
                    deepPropulsionSample: deepPropulsion.slice(0, 5),
                }
            );
        }

        isRendering = true;

        // Show loading indicator
        showLoading('Rendering ' + (VIEW_OPTIONS[view]?.label || view) + '...');

        let didFinishRender = false;
        const finishRender = () => {
            if (didFinishRender) return;
            didFinishRender = true;
            clearTimeout(renderSafetyTimeout);
            isRendering = false;
            hideLoading();
            webviewPerf('visualizer:webviewRenderCompleted', {
                view,
                prepareMs,
                totalMs: Date.now() - renderStartedAt,
            });
            if (pendingRenderRequest) {
                const next = pendingRenderRequest;
                pendingRenderRequest = null;
                setTimeout(() => {
                    renderVisualization(next.view, next.preserveZoomOverride, next.allowDuringResize);
                }, 0);
            }
        };

        // Safety timeout: auto-reset isRendering after 10 seconds to prevent permanent lockup
        const renderSafetyTimeout = setTimeout(() => {
            webviewPerf('visualizer:webviewRenderSafetyTimeout', {
                view,
                prepareMs,
                elapsedMs: Date.now() - renderStartedAt,
            });
            finishRender();
        }, 10000);

        // Test basic setup
        const vizElement = document.getElementById('visualization');

        // Add error handling around rendering
        try {
        webviewPerf('visualizer:webviewRenderStarted', {
            view,
            prepareMs,
            graphNodes: dataToRender?.graph?.nodes?.length || 0,
            graphEdges: dataToRender?.graph?.edges?.length || 0,
        });

        // Preserve current zoom state before clearing
        let currentTransform = d3.zoomIdentity;
        let shouldPreserveZoom = false;

        if (svg && zoom) {
            try {
                currentTransform = d3.zoomTransform(svg.node());
                // Only preserve zoom if user has manually interacted
                shouldPreserveZoom = window.userHasManuallyZoomed === true;
            } catch (e) {
                // If there's an error getting transform, don't preserve
                shouldPreserveZoom = false;
                currentTransform = d3.zoomIdentity;
            }
        }

        const width = document.getElementById('visualization').clientWidth;
        const height = document.getElementById('visualization').clientHeight;
        ensureVisualizationCanvas(width, height);
        g.selectAll('*').remove();

        // Restore the zoom state after creating new elements, but do it after render
        const restoreZoom = () => {
            if (shouldPreserveZoom && currentTransform) {
                // Use a slight delay to ensure elements are rendered
                setTimeout(() => {
                    svg.transition()
                        .duration(0)  // No animation for restore
                        .call(zoom.transform, currentTransform);
                }, 10);
            }
        };

        // Build context for modular renderers
        function buildRenderContext(w, h) {
            return {
                width: w,
                height: h,
                svg,
                g,
                zoom,
                getCy: () => null,
                layoutDirection,
                activityLayoutDirection,
                stateLayoutOrientation,
                selectedDiagramIndex,
                postMessage: (msg) => vscode.postMessage(msg),
                onStartInlineEdit: (nodeG, elementName, x, y, wd) => startInlineEdit(nodeG, elementName, x, y, wd),
                renderPlaceholder: (wd, ht, viewName, message, d) => renderPlaceholderView(wd, ht, viewName, message, d),
                clearVisualHighlights,
                elkWorkerUrl
            };
        }

        // Add global click handler to close expanded details when clicking on empty space
        svg.on('click', (event) => {
            // Only close if clicking on the SVG background (not on nodes or details)
            if (event.target === svg.node() || event.target === g.node()) {
                // Clear all highlights when clicking on empty space
                clearVisualHighlights();
                g.selectAll('.expanded-details').remove();
                // Reset graph view selections (clearVisualHighlights already restores node-background)
                g.selectAll('.graph-node-background').each(function() {
                    const el = d3.select(this);
                    el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
                    el.style('stroke-width', el.attr('data-original-width') || '2px');
                });
                g.selectAll('.node-group').classed('selected', false);
                g.selectAll('.graph-node-group').classed('selected', false);
                g.selectAll('.hierarchy-cell').classed('selected', false);
                g.selectAll('.elk-node').classed('selected', false);

                // Clear IBD connector highlights
                g.selectAll('.ibd-connector').each(function() {
                    const el = d3.select(this);
                    const origStroke = el.attr('data-original-stroke');
                    const origWidth = el.attr('data-original-width');
                    if (origStroke) {
                        el.style('stroke', origStroke)
                          .style('stroke-width', origWidth)
                          .classed('connector-highlighted', false);
                        el.attr('data-original-stroke', null)
                          .attr('data-original-width', null);
                    }
                });

                // Clear General View connector highlights
                g.selectAll('.general-connector').each(function() {
                    const el = d3.select(this);
                    const origStroke = el.attr('data-original-stroke');
                    const origWidth = el.attr('data-original-width');
                    if (origStroke) {
                        el.style('stroke', origStroke)
                          .style('stroke-width', origWidth)
                          .classed('connector-highlighted', false);
                        el.attr('data-original-stroke', null)
                          .attr('data-original-width', null);
                    }
                });
            }
        });

        if (view === 'general-view') {
            const ctx = {
                ...buildRenderContext(width, height),
                buildGeneralViewGraph: (data: any) => buildGeneralViewGraphForView(data),
                renderGeneralChips,
                elkWorkerUrl,
            };
            await renderGeneralViewD3(ctx as any, dataToRender);
            setTimeout(() => {
                if (shouldPreserveZoom) {
                    restoreZoom();
                } else {
                    zoomToFit('auto');
                }
                updateDimensionsDisplay();
                finishRender();
            }, 100);
        } else if (view === 'interconnection-view') {
            const ctx = {
                ...buildRenderContext(width, height),
                elkWorkerUrl,
            };
            await renderIbdView(ctx as any, dataToRender);
            setTimeout(() => {
                if (shouldPreserveZoom) {
                    restoreZoom();
                } else {
                    zoomToFit('auto');
                }
                updateDimensionsDisplay();
                finishRender();
            }, 100);
            } else if (view === 'action-flow-view') {
                await renderActivityViewModule(buildRenderContext(width, height), dataToRender);
            } else if (view === 'state-transition-view') {
                await renderStateViewModule(buildRenderContext(width, height), dataToRender);
                setTimeout(() => {
                    if (shouldPreserveZoom) {
                        restoreZoom();
                    } else {
                        zoomToFit('auto');
                    }
                    updateDimensionsDisplay();
                    finishRender();
                }, 100);
            } else {
                renderPlaceholderView(width, height, 'Unknown View', 'The selected view is not yet implemented.', dataToRender);
            }

            // General view and interconnection view handle zoom/hide in their async .then(); others run here
            if (view !== 'general-view' && view !== 'interconnection-view' && view !== 'state-transition-view') {
                // If zoom was previously modified, restore it; otherwise zoom to fit
                if (shouldPreserveZoom) {
                    restoreZoom();
                } else {
                    // Delay zoom to fit to ensure rendering is complete
                    setTimeout(() => zoomToFit('auto'), 100);
                }

                // Show initial dimensions briefly
                setTimeout(() => {
                    updateDimensionsDisplay();
                    finishRender();
                }, 200);
            }

        // Update lastView after successful render start
        lastView = view;
        } catch (error) {
            webviewPerf('visualizer:webviewRenderFailed', {
                view,
                prepareMs,
                totalMs: Date.now() - renderStartedAt,
                error: error instanceof Error ? error.message : String(error),
            });
            console.error('Error during rendering:', error);
            finishRender();

            // Show error message to user
            const statusText = document.getElementById('status-text');
            if (statusText) {
                statusText.textContent = 'Error rendering visualization: ' + error.message;
            }
        }
    }

    // Tree View Renderer - implemented in renderers/tree.ts

    function expandTreeNodeDetails(nodeData, nodeGroup) {
        // Remove any existing expanded details
        g.selectAll('.expanded-details').remove();

        // Remove selection styling from all nodes - restore original strokes
        g.selectAll('.node-background').each(function() {
            const el = d3.select(this);
            el.style('stroke', el.attr('data-original-stroke') || 'var(--vscode-panel-border)');
            el.style('stroke-width', el.attr('data-original-width') || '1px');
        });
        g.selectAll('.node-group').classed('selected', false);

        // Add selection styling to clicked node
        nodeGroup.select('.node-background')
            .style('stroke', 'var(--vscode-charts-blue)')
            .style('stroke-width', '3px');
        nodeGroup.classed('selected', true);

        // Get the node's transform to position the details panel
        const transform = nodeGroup.attr('transform');
        const matches = transform.match(/translate[(]([^,]+),([^)]+)[)]/);
        const nodeX = parseFloat(matches[1]);
        const nodeY = parseFloat(matches[2]);

        // Calculate dynamic dimensions based on content
        const baseHeight = 85; // Base height for name, type, level
        const lineHeight = 15;
        const sectionSpacing = 10;
        let contentHeight = baseHeight;

        // Calculate documentation height
        const docHeight = nodeData.data.properties?.documentation
            ? Math.min(Math.ceil(String(nodeData.data.properties.documentation).length / 35), 3) * 14 + 30 + sectionSpacing
            : 0;
        contentHeight += docHeight;

        // Calculate attributes height
        const attributes = nodeData.data.attributes || {};
        const displayableAttributes = Object.entries(attributes).filter(([key]) =>
            !key.startsWith('is') && key !== 'visibility'
        );
        const attributesHeight = displayableAttributes.length > 0
            ? Math.min(displayableAttributes.length, 4) * lineHeight + 20 + sectionSpacing
            : 0;
        contentHeight += attributesHeight;

        // Calculate properties height
        const properties = nodeData.data.properties || {};
        const regularProperties = Object.entries(properties).filter(([key]) => key !== 'documentation');
        const propertiesHeight = regularProperties.length > 0
            ? Math.min(regularProperties.length, 3) * lineHeight + 20 + sectionSpacing
            : 0;
        contentHeight += propertiesHeight;

        // Calculate children height (with attributes showing)
        let childrenHeight = 0;
        if (nodeData.children && nodeData.children.length > 0) {
            const maxChildrenToShow = Math.min(nodeData.children.length, 4);
            let childContentHeight = 20 + sectionSpacing; // Header height

            nodeData.children.slice(0, maxChildrenToShow).forEach(child => {
                childContentHeight += lineHeight; // Child name line

                // Add height for child attributes
                if (child.data.attributes && Object.keys(child.data.attributes).length > 0) {
                    const childAttrs = Object.entries(child.data.attributes).filter(([key]) =>
                        !key.startsWith('is') && key !== 'visibility'
                    );
                    childContentHeight += Math.min(childAttrs.length, 3) * 12; // 12px per attribute line
                }
                childContentHeight += 5; // Spacing between children
            });

            if (nodeData.children.length > maxChildrenToShow) {
                childContentHeight += 15; // "... more children" line
            }

            childrenHeight = childContentHeight;
        }
        contentHeight += childrenHeight;

        // Add button height and padding
        const buttonHeight = 25;
        const totalHeight = contentHeight + buttonHeight;

        // Dynamic width based on content
        const maxNameLength = Math.max(
            nodeData.data.name.length,
            nodeData.data.type.length + 6, // "Type: " prefix
            ...(displayableAttributes.slice(0, 4).map(([k, v]) => (k + ': ' + String(v)).length)),
            ...(regularProperties.slice(0, 3).map(([k, v]) => (k + ': ' + String(v)).length)),
            ...(nodeData.children ? nodeData.children.slice(0, 4).map(child => {
                const childNameLength = ('• ' + child.data.name + ' [' + child.data.type + ']').length;
                const childAttrs = child.data.attributes ? Object.entries(child.data.attributes).filter(([key]) =>
                    !key.startsWith('is') && key !== 'visibility'
                ) : [];
                const maxAttrLength = childAttrs.length > 0 ? Math.max(...childAttrs.map(([k, v]) =>
                    ('    ' + k + ': ' + String(v)).length
                )) : 0;
                return Math.max(childNameLength, maxAttrLength);
            }) : [])
        );
        const dynamicWidth = Math.max(250, Math.min(450, maxNameLength * 7 + 60));

        const popupWidth = dynamicWidth;
        const popupHeight = totalHeight;
        const buttonY = popupHeight - 20;

        // Create expanded details panel positioned next to the node
        const detailsGroup = g.append('g')
            .attr('class', 'expanded-details')
            .attr('transform', 'translate(' + (nodeX + 20) + ',' + (nodeY - 50) + ')');

        // Panel background with dynamic dimensions
        detailsGroup.append('rect')
            .attr('x', 0)
            .attr('y', 0)
            .attr('width', popupWidth)
            .attr('height', popupHeight)
            .attr('rx', 8)
            .style('fill', 'var(--vscode-editor-background)')
            .style('stroke', 'var(--vscode-charts-blue)')
            .style('stroke-width', '2px')
            .style('filter', 'drop-shadow(3px 3px 6px rgba(0,0,0,0.4))');

        // Close button - adjusted for smaller panel
        detailsGroup.append('circle')
            .attr('cx', 185)
            .attr('cy', 15)
            .attr('r', 10)
            .style('fill', 'var(--vscode-charts-red)')
            .style('cursor', 'pointer')
            .on('click', () => {
                g.selectAll('.expanded-details').remove();
                g.selectAll('.node-background')
                    .style('stroke', 'var(--vscode-panel-border)')
                    .style('stroke-width', '1px');
            });

        detailsGroup.append('text')
            .attr('x', 185)
            .attr('y', 19)
            .attr('text-anchor', 'middle')
            .text('×')
            .style('fill', 'white')
            .style('font-size', '12px')
            .style('font-weight', 'bold')
            .style('cursor', 'pointer')
            .on('click', () => {
                g.selectAll('.expanded-details').remove();
                g.selectAll('.node-background')
                    .style('stroke', 'var(--vscode-panel-border)')
                    .style('stroke-width', '1px');
            });

        // Element name
        detailsGroup.append('text')
            .attr('x', 15)
            .attr('y', 25)
            .text(nodeData.data.name)
            .style('font-weight', 'bold')
            .style('font-size', '16px')
            .style('fill', 'var(--vscode-editor-foreground)');

        // Element type
        detailsGroup.append('text')
            .attr('x', 15)
            .attr('y', 45)
            .text('Type: ' + nodeData.data.type)
            .style('font-size', '12px')
            .style('fill', 'var(--vscode-descriptionForeground)');

        let yOffset = 65;

        // Library validation status
        if (isLibraryValidated(nodeData.data)) {
            const libKind = getLibraryKind(nodeData.data);
            const libChain = getLibraryChain(nodeData.data);

            detailsGroup.append('text')
                .attr('x', 15)
                .attr('y', yOffset)
                .text('✓ Standard Library Type')
                .style('font-size', '12px')
                .style('font-weight', 'bold')
                .style('fill', 'var(--vscode-charts-green)');

            yOffset += 20;

            if (libKind) {
                detailsGroup.append('text')
                    .attr('x', 15)
                    .attr('y', yOffset)
                    .text('Library Kind: ' + libKind)
                    .style('font-size', '11px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
                yOffset += 18;
            }

            if (libChain) {
                detailsGroup.append('text')
                    .attr('x', 15)
                    .attr('y', yOffset)
                    .text('Specialization: ' + libChain)
                    .style('font-size', '11px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
                yOffset += 18;
            }
        }

        // Hierarchy level
        detailsGroup.append('text')
            .attr('x', 15)
            .attr('y', yOffset)
            .text('Level: ' + nodeData.depth)
            .style('font-size', '12px')
            .style('fill', 'var(--vscode-descriptionForeground)');

        yOffset += 20;

        // Documentation section
        if (nodeData.data.properties && nodeData.data.properties.documentation) {
            detailsGroup.append('text')
                .attr('x', 15)
                .attr('y', yOffset)
                .text('Documentation:')
                .style('font-weight', 'bold')
                .style('font-size', '13px')
                .style('fill', 'var(--vscode-editor-foreground)');

            yOffset += 20;
            // Wrap long documentation text
            const docText = String(nodeData.data.properties.documentation);
            const maxLineLength = 35;
            const lines = [];

            if (docText.length > maxLineLength) {
                let currentLine = '';
                const words = docText.split(' ');

                for (const word of words) {
                    if ((currentLine + word).length > maxLineLength && currentLine.length > 0) {
                        lines.push(currentLine.trim());
                        currentLine = word + ' ';
                    } else {
                        currentLine += word + ' ';
                    }
                }
                if (currentLine.trim().length > 0) {
                    lines.push(currentLine.trim());
                }
            } else {
                lines.push(docText);
            }

            // Show first 3 lines of documentation
            lines.slice(0, 3).forEach(line => {
                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text(line)
                    .style('font-size', '10px')
                    .style('fill', 'var(--vscode-descriptionForeground)')
                    .style('font-style', 'italic');
                yOffset += 14;
            });

            if (lines.length > 3) {
                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text('... (' + (lines.length - 3) + ' more lines)')
                    .style('font-size', '9px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
                yOffset += 12;
            }

            yOffset += 10; // Extra spacing after documentation
        }

        // Attributes section - show SysML element attributes
        const nodeAttributes = nodeData.data.attributes || {};
        const displayAttributes = Object.entries(nodeAttributes).filter(([key]) =>
            // Filter out internal attributes that aren't useful for display
            !key.startsWith('is') && key !== 'visibility'
        );

        if (displayAttributes.length > 0) {
            detailsGroup.append('text')
                .attr('x', 15)
                .attr('y', yOffset)
                .text('Attributes:')
                .style('font-weight', 'bold')
                .style('font-size', '13px')
                .style('fill', 'var(--vscode-editor-foreground)');

            yOffset += 20;
            displayAttributes.slice(0, 4).forEach(([key, value]) => {
                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text(key + ': ' + (String(value).length > 25 ? String(value).substring(0, 22) + '...' : String(value)))
                    .style('font-size', '11px')
                    .style('fill', 'var(--vscode-charts-purple)');
                yOffset += 15;
            });

            if (displayAttributes.length > 4) {
                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text('... (' + (displayAttributes.length - 4) + ' more attributes)')
                    .style('font-size', '10px')
                    .style('font-style', 'italic')
                    .style('fill', 'var(--vscode-descriptionForeground)');
                yOffset += 15;
            }

            yOffset += 10; // Extra spacing after attributes
        }

        // Properties section (excluding documentation which is shown separately)
        const nodeProperties = nodeData.data.properties || {};
        const displayProperties = Object.entries(nodeProperties).filter(([key]) => key !== 'documentation');

        if (displayProperties.length > 0) {
            detailsGroup.append('text')
                .attr('x', 15)
                .attr('y', yOffset)
                .text('Properties:')
                .style('font-weight', 'bold')
                .style('font-size', '13px')
                .style('fill', 'var(--vscode-editor-foreground)');

            yOffset += 20;
            displayProperties.slice(0, 3).forEach(([key, value]) => {
                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text(key + ': ' + (String(value).length > 25 ? String(value).substring(0, 22) + '...' : String(value)))
                    .style('font-size', '11px')
                    .style('fill', 'var(--vscode-descriptionForeground)');
                yOffset += 15;
            });
        }

        // Children section - now shows more children with attributes
        if (nodeData.children && nodeData.children.length > 0) {
            detailsGroup.append('text')
                .attr('x', 15)
                .attr('y', yOffset)
                .text('Children (' + nodeData.children.length + '):')
                .style('font-weight', 'bold')
                .style('font-size', '13px')
                .style('fill', 'var(--vscode-editor-foreground)');

            yOffset += 20;
            const maxChildrenToShow = Math.min(nodeData.children.length, 4); // Show up to 4 children with attributes

            nodeData.children.slice(0, maxChildrenToShow).forEach(child => {
                // Child name and type
                const childText = '• ' + child.data.name + ' [' + child.data.type + ']';
                const truncatedText = childText.length > 40
                    ? childText.substring(0, 37) + '...'
                    : childText;

                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text(truncatedText)
                    .style('font-size', '11px')
                    .style('font-weight', 'bold')
                    .style('fill', 'var(--vscode-editor-foreground)');
                yOffset += 15;

                // Show child attributes if they exist
                if (child.data.attributes && Object.keys(child.data.attributes).length > 0) {
                    const childAttributes = Object.entries(child.data.attributes);
                    const maxAttrsToShow = Math.min(childAttributes.length, 3);

                    childAttributes.slice(0, maxAttrsToShow).forEach(([key, value]) => {
                        // Skip internal attributes that aren't useful for display
                        if (!key.startsWith('is') && key !== 'visibility') {
                            const attrText = '    ' + key + ': ' + String(value);
                            const truncatedAttr = attrText.length > 35
                                ? attrText.substring(0, 32) + '...'
                                : attrText;

                            detailsGroup.append('text')
                                .attr('x', 35)
                                .attr('y', yOffset)
                                .text(truncatedAttr)
                                .style('font-size', '10px')
                                .style('font-style', 'italic')
                                .style('fill', 'var(--vscode-charts-purple)');
                            yOffset += 12;
                        }
                    });

                    if (childAttributes.length > maxAttrsToShow) {
                        detailsGroup.append('text')
                            .attr('x', 35)
                            .attr('y', yOffset)
                            .text('    ... (' + (childAttributes.length - maxAttrsToShow) + ' more attrs)')
                            .style('font-size', '9px')
                            .style('font-style', 'italic')
                            .style('fill', 'var(--vscode-descriptionForeground)');
                        yOffset += 12;
                    }
                }

                yOffset += 5; // Extra spacing between children
            });

            if (nodeData.children.length > maxChildrenToShow) {
                detailsGroup.append('text')
                    .attr('x', 25)
                    .attr('y', yOffset)
                    .text('... and ' + (nodeData.children.length - maxChildrenToShow) + ' more children')
                    .style('font-size', '10px')
                    .style('font-style', 'italic')
                    .style('fill', 'var(--vscode-descriptionForeground)');
                yOffset += 15;
            }
        }

        // Action buttons - adjusted for smaller panel
        // const buttonY = 108; // Moved up to fit in smaller panel

        // Navigate button
        detailsGroup.append('rect')
            .attr('x', 15)
            .attr('y', buttonY)
            .attr('width', 70)
            .attr('height', 18)
            .attr('rx', 4)
            .style('fill', 'var(--vscode-button-background)')
            .style('stroke', 'var(--vscode-button-border)')
            .style('cursor', 'pointer')
            .on('click', () => {
                postJumpToElement((msg) => vscode.postMessage(msg), { name: nodeData.data.name, id: nodeData.data.id });
            });

        detailsGroup.append('text')
            .attr('x', 50)
            .attr('y', buttonY + 13)
            .attr('text-anchor', 'middle')
            .text('Navigate')
            .style('fill', 'var(--vscode-button-foreground)')
            .style('font-size', '10px')
            .style('cursor', 'pointer')
            .on('click', () => {
                postJumpToElement((msg) => vscode.postMessage(msg), { name: nodeData.data.name, id: nodeData.data.id });
            });
    }

    function renderRelationships() {
        const relationships = currentData?.graph
            ? (currentData.graph.edges || []).filter((e) => (e.type || '').toLowerCase() !== 'contains')
                .map((e) => ({ source: e.source, target: e.target, type: e.type, name: e.name }))
            : (currentData?.relationships || []);
        if (!relationships.length) {
            return;
        }

        // Get all tree nodes with their positions (match by name or id)
        const allNodes = [];
        g.selectAll('.node-group').each(function(d) {
            if (d && d.data) {
                const transform = d3.select(this).attr('transform');
                const matches = transform.match(/translate[(]([^,]+),([^)]+)[)]/);
                if (matches) {
                    allNodes.push({
                        name: d.data.name,
                        id: d.data.id,
                        x: parseFloat(matches[1]),
                        y: parseFloat(matches[2]),
                        element: this
                    });
                }
            }
        });

        const findNode = (key) => allNodes.find((n) => n.name === key || n.id === key);

        relationships.forEach((rel) => {
            const sourceNode = findNode(rel.source);
            const targetNode = findNode(rel.target);

            if (sourceNode && targetNode && sourceNode.x != null && sourceNode.y != null && targetNode.x != null && targetNode.y != null) {
                g.append('line')
                    .attr('class', 'relationship-link')
                    .attr('x1', sourceNode.x)
                    .attr('y1', sourceNode.y)
                    .attr('x2', targetNode.x)
                    .attr('y2', targetNode.y);
            }
        });
    }

    // convertToHierarchy, isMetadataElement, flattenElements, extractDocumentation imported from ./helpers
    // createLinksFromHierarchy imported from ./helpers

    function getHighlightedSvgBounds() {
        if (!g) {
            return null;
        }

        const highlighted = Array.from(g.node().querySelectorAll('.highlighted-element, .selected'));
        if (highlighted.length === 0) {
            return null;
        }

        let minX = Infinity;
        let minY = Infinity;
        let maxX = -Infinity;
        let maxY = -Infinity;

        highlighted.forEach(element => {
            if (!element || typeof element.getBBox !== 'function') {
                return;
            }
            try {
                const bbox = element.getBBox();
                if (!bbox || (bbox.width === 0 && bbox.height === 0)) {
                    return;
                }
                minX = Math.min(minX, bbox.x);
                minY = Math.min(minY, bbox.y);
                maxX = Math.max(maxX, bbox.x + bbox.width);
                maxY = Math.max(maxY, bbox.y + bbox.height);
            } catch (e) {
                // Some elements might not support getBBox
                return;
            }
        });

        if (!isFinite(minX) || !isFinite(minY) || !isFinite(maxX) || !isFinite(maxY)) {
            return null;
        }

        return {
            x: minX,
            y: minY,
            width: maxX - minX,
            height: maxY - minY
        };
    }

    function resetZoom() {
        // Home action: return to initial fit-and-center framing.
        zoomToFit('user');
    }

    function zoomToFit(trigger = 'user') {
        const isAuto = trigger === 'auto';
        if (!g || !svg) return;

        try {
            if (!isAuto) {
                window.userHasManuallyZoomed = true;
            }

            const selectionBounds = getHighlightedSvgBounds();
            const bounds = selectionBounds || g.node().getBBox();
            if (!bounds || bounds.width === 0 || bounds.height === 0) return;

            const svgWidth = +svg.attr('width');
            const svgHeight = +svg.attr('height');

            // Use tighter padding for selections, default padding otherwise
            const basePadding = selectionBounds ? 0.06 : 0.08;
            const padding = Math.min(svgWidth, svgHeight) * basePadding;

            const scaleX = (svgWidth - 2 * padding) / bounds.width;
            const scaleY = (svgHeight - 2 * padding) / bounds.height;
            const scale = Math.min(scaleX, scaleY);

            // For selections, allow zooming in more; for full view, cap at 1x
            const maxScale = selectionBounds ? 3 : 1;
            const finalScale = Math.max(Math.min(scale, maxScale), MIN_CANVAS_ZOOM);

            const centerX = svgWidth / 2;
            const centerY = svgHeight / 2;
            const boundsX = bounds.x + bounds.width / 2;
            const boundsY = bounds.y + bounds.height / 2;

            const translateX = centerX - boundsX * finalScale;
            const translateY = centerY - boundsY * finalScale;

            svg.transition()
                .duration(750)
                .call(zoom.transform, d3.zoomIdentity
                    .translate(translateX, translateY)
                    .scale(finalScale));
        } catch (error) {
            console.warn('Error in zoomToFit:', error);
            resetZoom();
        }
    }


    // Make export handlers globally accessible (from export.ts)
    window.exportPNG = (scale) => exportHandler.exportPNG(scale);
    window.exportSVG = () => exportHandler.exportSVG();
    window.exportJSON = () => exportHandler.exportJSON();
    window.resetZoom = resetZoom;
    window.zoomToFit = zoomToFit;

    // IBD/Interconnection View Renderer - implemented in renderers/ibd.ts

    // Activity/Action Flow View Renderer - implemented in renderers/activity.ts

    // State Transition View Renderer - implemented in renderers/state.ts

    // Use Case View Renderer - implemented in renderers/usecase.ts

    // Package View Renderer - implemented in renderers/package.ts

    // Placeholder renderer for views that cannot display a diagram (no data or not supported)
    function wrapTextToFit(line, maxCharsPerLine) {
        if (!line || line.length <= maxCharsPerLine) return [line];
        const words = line.split(/\s+/);
        const result = [];
        let current = '';
        for (const w of words) {
            const next = current ? current + ' ' + w : w;
            if (next.length <= maxCharsPerLine) {
                current = next;
            } else {
                if (current) result.push(current);
                if (w.length > maxCharsPerLine) {
                    for (let i = 0; i < w.length; i += maxCharsPerLine) {
                        result.push(w.substring(i, i + maxCharsPerLine));
                    }
                    current = '';
                } else {
                    current = w;
                }
            }
        }
        if (current) result.push(current);
        return result;
    }

    function renderPlaceholderView(width, height, viewName, message, data) {
        const centerX = width / 2;
        const centerY = height / 2;
        const messageGroup = g.append('g')
            .attr('class', 'placeholder-message')
            .attr('transform', 'translate(' + centerX + ',' + centerY + ')');

        // Message lines (handle both \n and escaped \\n)
        const rawLines = message.split(/\n|\\n/).filter(l => l.length > 0);
        const maxCharsPerLine = 38;
        const wrappedLines = [];
        rawLines.forEach(l => wrappedLines.push.apply(wrappedLines, wrapTextToFit(l, maxCharsPerLine)));
        const hasFooter = data && ((data.elements && data.elements.length > 0) || (data.graph?.nodes && data.graph.nodes.length > 0));

        // Subtle card background - height adapts to content
        const cardWidth = 320;
        const lineHeight = 22;
        const cardHeight = Math.max(120, 70 + wrappedLines.length * lineHeight + (hasFooter ? 30 : 0));
        messageGroup.append('rect')
            .attr('x', -cardWidth / 2)
            .attr('y', -cardHeight / 2)
            .attr('width', cardWidth)
            .attr('height', cardHeight)
            .attr('rx', 8)
            .attr('ry', 8)
            .style('fill', 'var(--vscode-editor-inactiveSelectionBackground)')
            .style('stroke', 'var(--vscode-panel-border)')
            .style('stroke-width', '1px');

        // View name
        messageGroup.append('text')
            .attr('x', 0)
            .attr('y', -cardHeight / 2 + 28)
            .attr('text-anchor', 'middle')
            .text(viewName)
            .style('font-size', '18px')
            .style('fill', 'var(--vscode-editor-foreground)')
            .style('font-weight', '600');

        // Render message lines (wrapped to fit card width)
        wrappedLines.forEach((line, i) => {
            messageGroup.append('text')
                .attr('x', 0)
                .attr('y', -cardHeight / 2 + 52 + (i * lineHeight))
                .attr('text-anchor', 'middle')
                .text(line)
                .style('font-size', '13px')
                .style('fill', 'var(--vscode-descriptionForeground)');
        });

        // Optional footer when model has elements
        const elementCount = (data?.elements?.length ?? 0) || (data?.graph?.nodes?.length ?? 0);
        if (data && elementCount > 0) {
            messageGroup.append('text')
                .attr('x', 0)
                .attr('y', cardHeight / 2 - 20)
                .attr('text-anchor', 'middle')
                .text(elementCount + ' element(s) in model')
                .style('font-size', '11px')
                .style('fill', 'var(--vscode-descriptionForeground)')
                .style('opacity', '0.8');
        }
    }

    // Add event listeners for view buttons (DOM should be ready since script is at end)
    const viewDropdownBtn = document.getElementById('view-dropdown-btn');
    const viewDropdownMenu = document.getElementById('view-dropdown-menu');

    if (viewDropdownBtn && viewDropdownMenu) {
        viewDropdownBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            const isVisible = viewDropdownMenu.classList.contains('show');
            viewDropdownMenu.classList.toggle('show', !isVisible);
        });
    }

    populateViewDropdown();

    // Set initial active view button
    updateActiveViewButton(currentView);

    // Add event listeners for action buttons
    document.getElementById('reset-btn').addEventListener('click', resetZoom);
    document.getElementById('layout-direction-btn').addEventListener('click', toggleLayoutDirection);

    // Legend popup toggle
    (function setupLegend() {
        const legendBtn = document.getElementById('legend-btn');
        const legendPopup = document.getElementById('legend-popup');
        const legendCloseBtn = document.getElementById('legend-close-btn');
        if (!legendBtn || !legendPopup) return;

        function showLegend() {
            legendPopup.style.display = 'block';
            legendPopup.style.top = '12px';
            legendPopup.style.right = '12px';
            legendPopup.style.left = '';
            legendPopup.style.bottom = '';
            legendBtn.classList.add('active');
            legendBtn.style.background = 'var(--vscode-button-background)';
            legendBtn.style.color = 'var(--vscode-button-foreground)';
        }

        function hideLegend() {
            legendPopup.style.display = 'none';
            legendBtn.classList.remove('active');
            legendBtn.style.background = '';
            legendBtn.style.color = '';
        }

        legendBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            const showing = legendPopup.style.display === 'block';
            if (showing) { hideLegend(); } else { showLegend(); }
        });

        if (legendCloseBtn) {
            legendCloseBtn.addEventListener('click', () => { hideLegend(); });
        }

        // Hide legend when clicking outside
        document.addEventListener('click', (e) => {
            if (legendPopup.style.display === 'block' &&
                !legendPopup.contains(e.target) &&
                !legendBtn.contains(e.target)) {
                hideLegend();
            }
        });
    })();

    // Legend drag support
    (function setupLegendDrag() {
        const legendPopup = document.getElementById('legend-popup');
        const legendHeader = document.getElementById('legend-header');
        if (!legendPopup || !legendHeader) return;

        let isDragging = false;
        let dragStartX = 0;
        let dragStartY = 0;
        let popupStartLeft = 0;
        let popupStartTop = 0;

        legendHeader.addEventListener('mousedown', (e) => {
            if (e.target.id === 'legend-close-btn') return;
            isDragging = true;
            dragStartX = e.clientX;
            dragStartY = e.clientY;
            const rect = legendPopup.getBoundingClientRect();
            const wrapperRect = legendPopup.parentElement.getBoundingClientRect();
            popupStartLeft = rect.left - wrapperRect.left;
            popupStartTop = rect.top - wrapperRect.top;
            legendPopup.style.right = '';
            legendPopup.style.left = popupStartLeft + 'px';
            legendPopup.style.top = popupStartTop + 'px';
            legendHeader.style.cursor = 'grabbing';
            e.preventDefault();
        });

        document.addEventListener('mousemove', (e) => {
            if (!isDragging) return;
            const dx = e.clientX - dragStartX;
            const dy = e.clientY - dragStartY;
            legendPopup.style.left = (popupStartLeft + dx) + 'px';
            legendPopup.style.top = (popupStartTop + dy) + 'px';
        });

        document.addEventListener('mouseup', () => {
            if (isDragging) {
                isDragging = false;
                legendHeader.style.cursor = 'grab';
            }
        });
    })();

    // Package dropdown toggle handler
    (function setupPkgDropdown() {
        const pkgBtn = document.getElementById('pkg-dropdown-btn');
        const pkgMenu = document.getElementById('pkg-dropdown-menu');
        if (!pkgBtn || !pkgMenu) return;

        pkgBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            pkgMenu.classList.toggle('show');
            // Close view dropdown if open
            if (viewDropdownMenu) viewDropdownMenu.classList.remove('show');
        });
    })();

    // Add export dropdown functionality
const exportBtn = document.getElementById('export-btn');
const exportMenu = document.getElementById('export-menu');

    exportBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isVisible = exportMenu.classList.contains('show');

        if (!isVisible) {
            // Position dropdown using fixed positioning for better visibility
            const btnRect = exportBtn.getBoundingClientRect();
            const menuWidth = 160;
            const menuHeight = 200; // Approximate height
            const viewportWidth = window.innerWidth;
            const viewportHeight = window.innerHeight;

            // Calculate optimal position
            let left = btnRect.right - menuWidth;
            let top = btnRect.bottom + 4;

            // Adjust if would overflow viewport
            if (left < 8) left = btnRect.left;
            if (left + menuWidth > viewportWidth - 8) left = viewportWidth - menuWidth - 8;
            if (top + menuHeight > viewportHeight - 8) top = btnRect.top - menuHeight - 4;

            exportMenu.style.left = left + 'px';
            exportMenu.style.top = top + 'px';
        }

        exportMenu.classList.toggle('show', !isVisible);
    });

    // Close dropdown when clicking outside
    document.addEventListener('click', (e) => {
        if (!exportBtn.contains(e.target) && !exportMenu.contains(e.target)) {
            exportMenu.classList.remove('show');
        }
        if (viewDropdownBtn && viewDropdownMenu &&
            !viewDropdownBtn.contains(e.target) &&
            !viewDropdownMenu.contains(e.target)) {
            viewDropdownMenu.classList.remove('show');
        }
        // Close pkg dropdown
        const pkgBtn = document.getElementById('pkg-dropdown-btn');
        const pkgMenu = document.getElementById('pkg-dropdown-menu');
        if (pkgBtn && pkgMenu && !pkgBtn.contains(e.target) && !pkgMenu.contains(e.target)) {
            pkgMenu.classList.remove('show');
        }
    });

    // Handle export menu item clicks
    document.querySelectorAll('.export-menu-item').forEach(item => {
        item.addEventListener('click', (e) => {
            const format = e.target.getAttribute('data-format');
            const scale = parseInt(e.target.getAttribute('data-scale')) || 2;

            // Don't close menu or export for parent PNG item (has submenu)
            if (format === 'png-parent') {
                e.stopPropagation();
                return;
            }

            exportMenu.classList.remove('show');

            switch(format) {
                case 'png':
                    exportHandler.exportPNG(scale);
                    break;
                case 'svg':
                    exportHandler.exportSVG();
                    break;
                case 'pdf':
                    console.warn('PDF export not implemented');
                    break;
                case 'json':
                    exportHandler.exportJSON();
                    break;
            }
        });
    });

    // webviewReady is sent from initializeLegacyBundle after vscode is set
