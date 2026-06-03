/**
 * Shared constants for the visualizer webview.
 * View IDs match SysML v2 specification (Clause 8.2.3): frameless-view types.
 */

export const MIN_CANVAS_ZOOM = 0.04;
export const MAX_CANVAS_ZOOM = 20;
export const MIN_SYSML_ZOOM = 0.04;
export const MAX_SYSML_ZOOM = 20;

export const STRUCTURAL_VIEWS = new Set([
    'general-view',
    'interconnection-view',
]);

export const SYSML_ENABLED_VIEWS = [
    'general-view',
    'interconnection-view',
    'action-flow-view',
    'sequence-view',
    'state-transition-view',
    'browser-view',
    'grid-view',
    'geometry-view',
] as const;

/** Behavior renderer views that are supported — never show experimental badges/banners. */
export const GRADUATED_BEHAVIOR_VIEWS = new Set<string>([
    'action-flow-view',
    'state-transition-view',
    'sequence-view',
]);

/** Standard SysML views with provisional renderer support while spec notation details settle. */
export const PROVISIONAL_STANDARD_VIEWS = new Set<string>([
    'browser-view',
    'grid-view',
    'geometry-view',
]);

export const DEFAULT_ENABLED_VIEWS = SYSML_ENABLED_VIEWS;

/** Default release-enabled views. */
export const ENABLED_VIEWS = new Set(DEFAULT_ENABLED_VIEWS);

export const ORIENTATION_LABELS: Record<string, string> = {
    horizontal: 'Horizontal',
    linear: 'Linear (Top-Down)',
};

export const STATE_LAYOUT_LABELS: Record<string, string> = {
    horizontal: 'Left → Right',
    vertical: 'Top → Down',
    force: 'Auto-arrange',
};

export const STATE_LAYOUT_ICONS: Record<string, string> = {
    horizontal: '→',
    vertical: '↓',
    force: '⚡',
};

export const VIEW_OPTIONS: Record<string, { label: string; shortLabel: string; icon: string }> = {
    'general-view': { label: 'General View', shortLabel: 'General', icon: 'symbol-structure' },
    'interconnection-view': { label: 'Interconnection View', shortLabel: 'Interconnection', icon: 'plug' },
    'action-flow-view': { label: 'Action Flow View', shortLabel: 'Action Flow', icon: 'git-commit' },
    'sequence-view': { label: 'Sequence View', shortLabel: 'Sequence', icon: 'symbol-event' },
    'state-transition-view': { label: 'State Transition View', shortLabel: 'State Transition', icon: 'git-compare' },
    'browser-view': { label: 'Browser View', shortLabel: 'Browser', icon: 'list-tree' },
    'grid-view': { label: 'Grid View', shortLabel: 'Grid', icon: 'table' },
    'geometry-view': { label: 'Geometry View', shortLabel: 'Geometry', icon: 'symbol-ruler' },
};

/** Documentation: rendering technology per view. All views use D3 + ELK. */
export const VIEW_RENDERER_TECH: Record<string, string> = {
    'general-view': 'D3+ELK',
    'interconnection-view': 'D3+ELK',
    'action-flow-view': 'D3+ELK',
    'sequence-view': 'D3',
    'state-transition-view': 'D3+ELK',
    'browser-view': 'D3',
    'grid-view': 'D3',
    'geometry-view': 'D3',
};

/**
 * General View palette - Option C semantic (align with SysML pillars).
 * Structural: greens/teals | Behavior: ambers | Requirements: soft blues.
 */
export const GENERAL_VIEW_PALETTE = {
    structural: {
        part: '#2D8A6E',
        port: '#0E7C7B',
        attribute: '#4A9B7F',
        item: '#5A9B6E',
        interface: '#7BAA7D',
    },
    behavior: {
        action: '#D4A02C',
        state: '#B85C38',
        calc: '#C9A227',
    },
    requirements: {
        requirement: '#5B8FC4',
        useCase: '#6B9BD1',
    },
    other: {
        allocation: '#9CA3AF',
        constraint: '#E07C5A',
        default: 'var(--vscode-panel-border)',
    },
} as const;

export const GENERAL_VIEW_TYPE_COLORS: Record<string, string> = {
    'part def': GENERAL_VIEW_PALETTE.structural.part,
    part: GENERAL_VIEW_PALETTE.structural.part,
    'port def': GENERAL_VIEW_PALETTE.structural.port,
    port: GENERAL_VIEW_PALETTE.structural.port,
    'attribute def': GENERAL_VIEW_PALETTE.structural.attribute,
    attribute: GENERAL_VIEW_PALETTE.structural.attribute,
    'action def': GENERAL_VIEW_PALETTE.behavior.action,
    action: GENERAL_VIEW_PALETTE.behavior.action,
    'state def': GENERAL_VIEW_PALETTE.behavior.state,
    state: GENERAL_VIEW_PALETTE.behavior.state,
    'interface def': GENERAL_VIEW_PALETTE.structural.interface,
    interface: GENERAL_VIEW_PALETTE.structural.interface,
    'requirement def': GENERAL_VIEW_PALETTE.requirements.requirement,
    requirement: GENERAL_VIEW_PALETTE.requirements.requirement,
    'use case def': GENERAL_VIEW_PALETTE.requirements.useCase,
    'use case': GENERAL_VIEW_PALETTE.requirements.useCase,
    verification: GENERAL_VIEW_PALETTE.behavior.calc,
    analysis: GENERAL_VIEW_PALETTE.behavior.action,
    allocation: GENERAL_VIEW_PALETTE.other.allocation,
    'allocation def': GENERAL_VIEW_PALETTE.other.allocation,
    'item def': GENERAL_VIEW_PALETTE.structural.item,
    item: GENERAL_VIEW_PALETTE.structural.item,
    'calc def': GENERAL_VIEW_PALETTE.behavior.calc,
    calc: GENERAL_VIEW_PALETTE.behavior.calc,
    'constraint def': GENERAL_VIEW_PALETTE.other.constraint,
    constraint: GENERAL_VIEW_PALETTE.other.constraint,
    'enumeration def': GENERAL_VIEW_PALETTE.behavior.calc,
    enumeration: GENERAL_VIEW_PALETTE.behavior.calc,
    'metadata def': '#8B7355',
    metadata: '#8B7355',
    'occurrence def': GENERAL_VIEW_PALETTE.structural.item,
    occurrence: GENERAL_VIEW_PALETTE.structural.item,
    package: '#6B7280',
    default: GENERAL_VIEW_PALETTE.other.default,
};
