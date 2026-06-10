/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { interconnectionBannerCounts } from '../dtoAdapter';
import { VIEW_OPTIONS } from './constants';
import { resolveEmptyStateMessage } from './emptyStateHelpers';
import type { VisualizerContext } from './visualizerContext';

export const LAYOUT_DIRECTION_LABELS: Record<string, string> = {
    horizontal: 'Left → Right',
    vertical: 'Top → Down',
    auto: 'Auto Layout',
};

export const LAYOUT_DIRECTION_ICONS: Record<string, string> = {
    horizontal: 'codicon-arrow-right',
    vertical: 'codicon-arrow-down',
    auto: 'codicon-editor-layout',
};

export function getNextLayoutDirection(current: string): string {
    const modes = ['horizontal', 'vertical', 'auto'];
    const currentIndex = modes.indexOf(current);
    return modes[(currentIndex + 1) % modes.length];
}

export function updateLayoutDirectionButton(ctx: VisualizerContext, activeView: string): void {
    const layoutBtn = document.getElementById('layout-direction-btn');
    if (layoutBtn) {
        const effectiveDirection =
            activeView === 'action-flow-view' ? ctx.activityLayoutDirection : ctx.layoutDirection;
        const iconClass = LAYOUT_DIRECTION_ICONS[effectiveDirection] || 'codicon-arrow-right';
        const label = LAYOUT_DIRECTION_LABELS[effectiveDirection] || 'Left → Right';
        layoutBtn.innerHTML = '<span class="codicon ' + iconClass + '"></span> ' + label;

        const nextMode = getNextLayoutDirection(effectiveDirection);
        const nextLabel = LAYOUT_DIRECTION_LABELS[nextMode];
        layoutBtn.title = 'Switch to ' + nextLabel;

        ctx.stateLayoutOrientation = ctx.layoutDirection === 'auto' ? 'force' : ctx.layoutDirection;
    }
}

export function populateViewDropdown(ctx: VisualizerContext): void {
    const viewDropdownMenu = document.getElementById('view-dropdown-menu');
    if (!viewDropdownMenu) return;
    viewDropdownMenu.innerHTML = '';
    const viewCandidates = Array.isArray(ctx.currentData?.viewCandidates)
        ? ctx.currentData.viewCandidates
        : [];
    if (viewCandidates.length === 0) {
        const item = document.createElement('button');
        item.className = 'view-dropdown-item';
        item.disabled = true;
        item.style.opacity = '0.7';
        item.style.cursor = 'default';
        item.textContent = 'No model-defined views';
        item.title =
            resolveEmptyStateMessage(ctx.currentData) ||
            'Define a SysML view with expose (and optional filter) to use the visualizer.';
        viewDropdownMenu.appendChild(item);
        return;
    }
    viewCandidates.forEach((candidate: any) => {
        const option = candidate?.rendererView
            ? VIEW_OPTIONS[candidate.rendererView] || VIEW_OPTIONS['general-view']
            : { icon: 'question', label: 'Unsupported View', shortLabel: 'Unsupported' };
        const item = document.createElement('button');
        item.className = 'view-dropdown-item';
        item.setAttribute('data-view-id', candidate.id || candidate.name);
        const label = candidate?.supported
            ? candidate.name || 'Unnamed view'
            : `${candidate?.name || 'Unnamed view'} (Unsupported)`;
        const iconSpan = document.createElement('span');
        iconSpan.className = `codicon codicon-${option.icon} icon`;
        const textSpan = document.createElement('span');
        textSpan.className = 'view-text';
        textSpan.textContent = label;
        item.appendChild(iconSpan);
        item.appendChild(textSpan);
        if (!candidate?.supported || ctx.isExperimentalRendererView(candidate?.rendererView)) {
            const badge = document.createElement('span');
            badge.className = 'view-badge';
            badge.textContent = !candidate?.supported ? 'Unsupported' : 'Experimental';
            item.appendChild(badge);
        }
        if (!candidate?.supported) {
            const tooltipBits = [
                candidate?.viewType ? `Type: ${candidate.viewType}` : 'Unsupported SysML view type',
                candidate?.description || '',
            ].filter(Boolean);
            item.title = tooltipBits.join('\n');
        }
        if (
            (ctx.currentData?.selectedView || ctx.currentData?.selectedViewName) ===
                (candidate.id || candidate.name) ||
            ctx.currentData?.selectedViewName === candidate.name
        ) {
            item.classList.add('active');
        }
        item.addEventListener('click', (e) => {
            const selectedView = (e.currentTarget as HTMLElement).getAttribute('data-view-id');
            viewDropdownMenu.classList.remove('show');
            if (selectedView) {
                ctx.vscode.postMessage({
                    command: 'viewSelectionChanged',
                    viewId: selectedView,
                    rendererView: candidate?.supported ? candidate?.rendererView : undefined,
                });
            }
        });
        viewDropdownMenu.appendChild(item);
    });
}

export function updateViewStatusBanner(ctx: VisualizerContext, activeView: string): void {
    const banner = document.getElementById('view-status-banner');
    if (!banner) return;
    if (activeView === 'interconnection-view' && ctx.currentData) {
        const { partCount, connectorCount } = interconnectionBannerCounts(ctx.currentData);
        const selectedRoot = ctx.currentData?.selectedViewName || 'the selected view';

        if (partCount > 0 && connectorCount === 0) {
            banner.className = 'experimental';
            banner.textContent = `Interconnection View found no internal connectors for ${selectedRoot}. Try another block if you expected connections here.`;
            banner.style.display = 'block';
            return;
        }
    }
    const rendererView = ctx.resolveActiveRendererView(activeView);
    if (ctx.isExperimentalRendererView(rendererView)) {
        const option = VIEW_OPTIONS[rendererView];
        banner.className = 'experimental';
        banner.textContent =
            (option?.label || rendererView) +
            ' is experimental. Layout, routing, or element coverage may still be incomplete.';
        banner.style.display = 'block';
        return;
    }
    banner.className = '';
    banner.textContent = '';
    banner.style.display = 'none';
}

export function updateActiveViewButton(ctx: VisualizerContext, activeView: string): void {
    const layoutDirBtn = document.getElementById('layout-direction-btn');
    if (layoutDirBtn) {
        const showLayoutBtn = false;
        layoutDirBtn.style.display = showLayoutBtn ? 'inline-flex' : 'none';
    }

    const dropdownButton = document.getElementById('view-dropdown-btn');
    const selectedViewName = ctx.currentData?.selectedViewName || null;
    const selectedCandidate = Array.isArray(ctx.currentData?.viewCandidates)
        ? ctx.currentData.viewCandidates.find(
              (candidate: any) => candidate.id === ctx.currentData?.selectedView,
          )
        : null;
    const dropdownConfig = VIEW_OPTIONS[selectedCandidate?.rendererView || activeView];
    if (dropdownButton) {
        if (selectedViewName) {
            dropdownButton.classList.add('view-btn-active');
            dropdownButton.textContent = '';
            const chevron = document.createElement('span');
            chevron.className = 'codicon codicon-chevron-down';
            chevron.style.marginRight = '2px';
            const label = document.createElement('span');
            label.textContent = selectedViewName;
            dropdownButton.appendChild(chevron);
            dropdownButton.appendChild(label);
            dropdownButton.title = dropdownConfig
                ? `${selectedViewName} (${dropdownConfig.label})`
                : selectedViewName;
        } else {
            dropdownButton.classList.remove('view-btn-active');
            dropdownButton.textContent = '';
            const chevron = document.createElement('span');
            chevron.className = 'codicon codicon-chevron-down';
            chevron.style.marginRight = '2px';
            const label = document.createElement('span');
            label.textContent = 'Select SysML View';
            dropdownButton.appendChild(chevron);
            dropdownButton.appendChild(label);
            dropdownButton.title = 'Select a defined SysML view';
        }
    }

    document.querySelectorAll('.view-dropdown-item').forEach((item) => {
        const isMatch = item.getAttribute('data-view-id') === ctx.currentData?.selectedView;
        item.classList.toggle('active', isMatch);
    });

    updateLayoutDirectionButton(ctx, activeView);
    populateViewDropdown(ctx);
    updateViewStatusBanner(ctx, activeView);
}

export function updateDiagramSelector(ctx: VisualizerContext, activeView: string): void {
    const pkgDropdown = document.getElementById('pkg-dropdown');
    const pkgMenu = document.getElementById('pkg-dropdown-menu');
    const pkgLabel = document.getElementById('pkg-dropdown-label');
    const pkgSummary = document.getElementById('pkg-dropdown-summary');

    const setSelectorSummary = (text: string) => {
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

    if (!pkgDropdown || !pkgMenu || !ctx.currentData) {
        if (pkgDropdown) pkgDropdown.style.display = 'none';
        setSelectorSummary('');
        return;
    }

    const buildCountSummary = (item: any, kind: string) => {
        if (!item) return '';
        const packageText = item.packagePath ? item.packagePath : '';
        const metricParts =
            kind === 'Package'
                ? []
                : kind === 'Root'
                  ? [
                        typeof item.partCount === 'number' ? `${item.partCount} parts` : '',
                        typeof item.connectorCount === 'number'
                            ? `${item.connectorCount} connectors`
                            : '',
                    ]
                  : kind === 'State Machine'
                    ? [
                          typeof item.stateCount === 'number' ? `${item.stateCount} states` : '',
                          typeof item.transitionCount === 'number'
                              ? `${item.transitionCount} transitions`
                              : '',
                      ]
                    : [
                          typeof item.nodeCount === 'number' ? `${item.nodeCount} nodes` : '',
                          typeof item.flowCount === 'number' ? `${item.flowCount} flows` : '',
                      ];
        return [packageText, ...metricParts.filter(Boolean)].filter(Boolean).join(' | ');
    };

    const viewCandidates = Array.isArray(ctx.currentData?.viewCandidates)
        ? ctx.currentData.viewCandidates
        : [];
    const labelText = 'View';
    const items = viewCandidates.map((candidate: any) => ({
        id: candidate.id || candidate.name,
        name: candidate.name,
        label: candidate.name,
        description: candidate.description || '',
        packagePath: '',
    }));

    const diagrams = items.map((item: any, index: number) => ({
        ...item,
        id: item?.id || item?.name || `${activeView}-item-${index + 1}`,
        name: item?.name || `Item ${index + 1}`,
        label: item?.label || item?.name || `Item ${index + 1}`,
        packagePath: item?.packagePath || '',
    }));

    if (diagrams.length <= 1) {
        pkgDropdown.style.display = diagrams.length === 1 ? 'flex' : 'none';
        const before = { name: ctx.selectedDiagramName, index: ctx.selectedDiagramIndex };
        ctx.selectedDiagramIndex = 0;
        ctx.selectedDiagramId = diagrams.length === 1 ? diagrams[0].id : null;
        ctx.selectedDiagramName = diagrams.length === 1 ? diagrams[0].name : null;
        ctx.selectedDiagramPackagePath =
            diagrams.length === 1 ? diagrams[0].packagePath || null : null;
        ctx.logSelectionTransition('selector.single-option-reset', before, {
            activeView,
            diagramsLength: diagrams.length,
        });
        if (pkgLabel && diagrams[0]) {
            pkgLabel.textContent = `${labelText}: ${diagrams[0].name}`;
        }
        setSelectorSummary(buildCountSummary(diagrams[0], labelText));
        pkgMenu.innerHTML = '';
        return;
    }

    pkgDropdown.style.display = 'flex';
    const matchingIndex = diagrams.findIndex(
        (candidate: any) =>
            (ctx.selectedDiagramId && candidate.id === ctx.selectedDiagramId) ||
            (ctx.selectedDiagramName &&
                candidate.name === ctx.selectedDiagramName &&
                (!ctx.selectedDiagramPackagePath ||
                    candidate.packagePath === ctx.selectedDiagramPackagePath)),
    );
    if (matchingIndex >= 0) {
        const before = { name: ctx.selectedDiagramName, index: ctx.selectedDiagramIndex };
        ctx.selectedDiagramIndex = matchingIndex;
        ctx.selectedDiagramId = diagrams[matchingIndex].id;
        ctx.selectedDiagramName = diagrams[matchingIndex].name;
        ctx.selectedDiagramPackagePath = diagrams[matchingIndex].packagePath || null;
        ctx.logSelectionTransition('selector.restore-hit', before, {
            matchingIndex,
            selectedDiagramName: ctx.selectedDiagramName,
        });
    } else {
        const before = { name: ctx.selectedDiagramName, index: ctx.selectedDiagramIndex };
        ctx.selectedDiagramIndex = 0;
        ctx.selectedDiagramId = diagrams[0]?.id || null;
        ctx.selectedDiagramName = diagrams[0]?.name || null;
        ctx.selectedDiagramPackagePath = diagrams[0]?.packagePath || null;
        ctx.logSelectionTransition(
            ctx.selectedDiagramName ? 'selector.restore-miss-fallback' : 'selector.init-first',
            before,
        );
    }

    const selectedDiagram = diagrams[ctx.selectedDiagramIndex];
    if (pkgLabel && selectedDiagram) {
        pkgLabel.textContent = `${labelText}: ${selectedDiagram.name || labelText}`;
    }
    setSelectorSummary(buildCountSummary(selectedDiagram, labelText));

    pkgMenu.innerHTML = '';
    diagrams.forEach((d, idx) => {
        const item = document.createElement('button');
        item.className = 'view-dropdown-item';
        item.textContent = d.label || d.name || 'Diagram ' + (idx + 1);
        const itemSummary = buildCountSummary(d, labelText);
        if (itemSummary) item.title = itemSummary;
        if (idx === ctx.selectedDiagramIndex) item.classList.add('active');
        item.addEventListener('click', function () {
            const before = { name: ctx.selectedDiagramName, index: ctx.selectedDiagramIndex };
            ctx.selectedDiagramIndex = idx;
            ctx.selectedDiagramId = d.id;
            ctx.selectedDiagramName = d.name;
            ctx.selectedDiagramPackagePath = d.packagePath || null;
            ctx.logSelectionTransition('selector.user-click', before, {
                selectedName: d.name,
                selectedIdx: idx,
            });
            window.userHasManuallyZoomed = false;
            pkgMenu.querySelectorAll('.view-dropdown-item').forEach((i) => i.classList.remove('active'));
            item.classList.add('active');
            if (pkgLabel) pkgLabel.textContent = `${labelText}: ${d.name || labelText}`;
            setSelectorSummary(buildCountSummary(d, labelText));
            pkgMenu.classList.remove('show');
            ctx.vscode.postMessage({ command: 'viewSelectionChanged', viewId: d.id || d.name });
        });
        pkgMenu.appendChild(item);
    });

    if (ctx.selectedDiagramIndex >= diagrams.length) {
        const before = { name: ctx.selectedDiagramName, index: ctx.selectedDiagramIndex };
        ctx.selectedDiagramIndex = 0;
        ctx.selectedDiagramId = diagrams[0]?.id || null;
        ctx.selectedDiagramName = diagrams[0]?.name || null;
        ctx.selectedDiagramPackagePath = diagrams[0]?.packagePath || null;
        ctx.logSelectionTransition('selector.index-out-of-range-fallback', before, {
            diagramsLength: diagrams.length,
        });
        if (pkgLabel && diagrams[0]) {
            pkgLabel.textContent = `${labelText}: ${diagrams[0].name || labelText}`;
        }
        setSelectorSummary(buildCountSummary(diagrams[0], labelText));
    }
}
