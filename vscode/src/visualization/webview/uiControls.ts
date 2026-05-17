export type VisualizerControlsConfig = {
    getCurrentView: () => string;
    resetZoom: () => void;
    toggleLayoutDirection: () => void;
    populateViewDropdown: () => void;
    updateActiveViewButton: (view: string) => void;
    exportPNG: (scale: number) => void;
    exportSVG: () => void;
    exportJSON: () => void;
};

export function setupVisualizerControls(config: VisualizerControlsConfig): void {
    const viewDropdownBtn = document.getElementById('view-dropdown-btn');
    const viewDropdownMenu = document.getElementById('view-dropdown-menu');

    if (viewDropdownBtn && viewDropdownMenu) {
        viewDropdownBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            const isVisible = viewDropdownMenu.classList.contains('show');
            viewDropdownMenu.classList.toggle('show', !isVisible);
        });
    }

    config.populateViewDropdown();
    config.updateActiveViewButton(config.getCurrentView());

    document.getElementById('reset-btn')?.addEventListener('click', config.resetZoom);
    document.getElementById('layout-direction-btn')?.addEventListener('click', config.toggleLayoutDirection);

    setupLegend();
    setupLegendDrag();
    setupPackageDropdown(viewDropdownMenu);
    setupExportDropdown(config, viewDropdownBtn, viewDropdownMenu);
}

function setupLegend(): void {
    const legendBtn = document.getElementById('legend-btn');
    const legendPopup = document.getElementById('legend-popup') as HTMLElement | null;
    const legendCloseBtn = document.getElementById('legend-close-btn');
    if (!legendBtn || !legendPopup) return;

    function showLegend() {
        legendPopup.style.display = 'block';
        legendPopup.style.top = '12px';
        legendPopup.style.right = '12px';
        legendPopup.style.left = '';
        legendPopup.style.bottom = '';
        legendBtn.classList.add('active');
        (legendBtn as HTMLElement).style.background = 'var(--vscode-button-background)';
        (legendBtn as HTMLElement).style.color = 'var(--vscode-button-foreground)';
    }

    function hideLegend() {
        legendPopup.style.display = 'none';
        legendBtn.classList.remove('active');
        (legendBtn as HTMLElement).style.background = '';
        (legendBtn as HTMLElement).style.color = '';
    }

    legendBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const showing = legendPopup.style.display === 'block';
        if (showing) { hideLegend(); } else { showLegend(); }
    });

    if (legendCloseBtn) {
        legendCloseBtn.addEventListener('click', () => { hideLegend(); });
    }

    document.addEventListener('click', (e) => {
        const target = e.target as Node;
        if (legendPopup.style.display === 'block' &&
            !legendPopup.contains(target) &&
            !legendBtn.contains(target)) {
            hideLegend();
        }
    });
}

function setupLegendDrag(): void {
    const legendPopup = document.getElementById('legend-popup') as HTMLElement | null;
    const legendHeader = document.getElementById('legend-header') as HTMLElement | null;
    if (!legendPopup || !legendHeader) return;

    let isDragging = false;
    let dragStartX = 0;
    let dragStartY = 0;
    let popupStartLeft = 0;
    let popupStartTop = 0;

    legendHeader.addEventListener('mousedown', (e) => {
        if ((e.target as HTMLElement).id === 'legend-close-btn') return;
        isDragging = true;
        dragStartX = e.clientX;
        dragStartY = e.clientY;
        const rect = legendPopup.getBoundingClientRect();
        const wrapperRect = legendPopup.parentElement?.getBoundingClientRect();
        if (!wrapperRect) return;
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
}

function setupPackageDropdown(viewDropdownMenu: HTMLElement | null): void {
    const pkgBtn = document.getElementById('pkg-dropdown-btn');
    const pkgMenu = document.getElementById('pkg-dropdown-menu') as HTMLElement | null;
    if (!pkgBtn || !pkgMenu) return;

    const repositionPackageMenu = () => {
        const buttonRect = pkgBtn.getBoundingClientRect();
        const diagramCanvas = document.getElementById('visualization');
        const panel = document.getElementById('visualization-wrapper');
        const boundaryRect = diagramCanvas?.getBoundingClientRect() ?? panel?.getBoundingClientRect();
        const viewportPadding = 8;
        const menuGap = 4;
        const preferredMenuHeight = 420;
        const boundaryLeft = boundaryRect ? boundaryRect.left + viewportPadding : viewportPadding;
        const boundaryRight = boundaryRect ? boundaryRect.right - viewportPadding : window.innerWidth - viewportPadding;
        const boundaryTop = boundaryRect ? boundaryRect.top + viewportPadding : viewportPadding;
        const boundaryBottom = boundaryRect ? boundaryRect.bottom - viewportPadding : window.innerHeight - viewportPadding;
        const measuredWidth = Math.max(
            pkgBtn.getBoundingClientRect().width,
            pkgMenu.scrollWidth,
            220,
        );
        const maxLeft = boundaryRight - measuredWidth;
        const left = Math.max(boundaryLeft, Math.min(buttonRect.left, maxLeft));
        const spaceBelow = boundaryBottom - buttonRect.bottom;
        const spaceAbove = buttonRect.top - boundaryTop;
        const shouldOpenUpward = spaceBelow < 220 && spaceAbove > spaceBelow;
        const availableSpace = shouldOpenUpward ? spaceAbove : spaceBelow;
        const maxHeight = Math.max(0, Math.min(preferredMenuHeight, availableSpace - menuGap));
        const top = shouldOpenUpward
            ? Math.max(boundaryTop, buttonRect.top - maxHeight - menuGap)
            : Math.min(boundaryBottom - maxHeight, buttonRect.bottom + menuGap);

        pkgMenu.style.position = 'fixed';
        pkgMenu.style.left = left + 'px';
        pkgMenu.style.top = top + 'px';
        pkgMenu.style.bottom = 'auto';
        pkgMenu.style.minWidth = Math.round(Math.max(buttonRect.width, 180)) + 'px';
        pkgMenu.style.maxHeight = Math.round(maxHeight) + 'px';
    };

    pkgBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const shouldShow = !pkgMenu.classList.contains('show');
        pkgMenu.classList.toggle('show', shouldShow);
        viewDropdownMenu?.classList.remove('show');
        if (shouldShow) {
            repositionPackageMenu();
            requestAnimationFrame(() => {
                const activeItem = pkgMenu.querySelector('.view-dropdown-item.active') as HTMLElement | null;
                activeItem?.scrollIntoView({ block: 'nearest' });
            });
        }
    });

    window.addEventListener('resize', () => {
        if (!pkgMenu.classList.contains('show')) {
            return;
        }
        repositionPackageMenu();
    });
}

function setupExportDropdown(
    config: VisualizerControlsConfig,
    viewDropdownBtn: HTMLElement | null,
    viewDropdownMenu: HTMLElement | null,
): void {
    const exportBtn = document.getElementById('export-btn');
    const exportMenu = document.getElementById('export-menu') as HTMLElement | null;
    if (!exportBtn || !exportMenu) return;

    exportBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        const isVisible = exportMenu.classList.contains('show');

        if (!isVisible) {
            const btnRect = exportBtn.getBoundingClientRect();
            const menuWidth = 160;
            const menuHeight = 200;
            const viewportWidth = window.innerWidth;
            const viewportHeight = window.innerHeight;

            let left = btnRect.right - menuWidth;
            let top = btnRect.bottom + 4;

            if (left < 8) left = btnRect.left;
            if (left + menuWidth > viewportWidth - 8) left = viewportWidth - menuWidth - 8;
            if (top + menuHeight > viewportHeight - 8) top = btnRect.top - menuHeight - 4;

            exportMenu.style.left = left + 'px';
            exportMenu.style.top = top + 'px';
        }

        exportMenu.classList.toggle('show', !isVisible);
    });

    document.addEventListener('click', (e) => {
        const target = e.target as Node;
        if (!exportBtn.contains(target) && !exportMenu.contains(target)) {
            exportMenu.classList.remove('show');
        }
        if (viewDropdownBtn && viewDropdownMenu &&
            !viewDropdownBtn.contains(target) &&
            !viewDropdownMenu.contains(target)) {
            viewDropdownMenu.classList.remove('show');
        }
        const pkgBtn = document.getElementById('pkg-dropdown-btn');
        const pkgMenu = document.getElementById('pkg-dropdown-menu');
        if (pkgBtn && pkgMenu && !pkgBtn.contains(target) && !pkgMenu.contains(target)) {
            pkgMenu.classList.remove('show');
        }
    });

    document.querySelectorAll('.export-menu-item').forEach(item => {
        item.addEventListener('click', (e) => {
            const target = e.target as HTMLElement;
            const format = target.getAttribute('data-format');
            const scale = parseInt(target.getAttribute('data-scale') || '', 10) || 2;

            if (format === 'png-parent') {
                e.stopPropagation();
                return;
            }

            exportMenu.classList.remove('show');

            switch(format) {
                case 'png':
                    config.exportPNG(scale);
                    break;
                case 'svg':
                    config.exportSVG();
                    break;
                case 'pdf':
                    console.warn('PDF export not implemented');
                    break;
                case 'json':
                    config.exportJSON();
                    break;
            }
        });
    });
}
