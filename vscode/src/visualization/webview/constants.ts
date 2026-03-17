/**
 * Shared constants for the visualizer webview.
 * View IDs match SysML v2 specification (Clause 8.2.3): frameless-view types.
 */

export const MIN_CANVAS_ZOOM = 0.04;
export const MAX_CANVAS_ZOOM = 20;
export const MIN_SYSML_ZOOM = 0.04;
export const MAX_SYSML_ZOOM = 20;

export const STRUCTURAL_VIEWS = new Set(['general-view']);

export const DEFAULT_ENABLED_VIEWS = ['general-view', 'interconnection-view'] as const;

export const EXPERIMENTAL_VIEWS = [
    'action-flow-view',
    'state-transition-view',
    'sequence-view',
] as const;

/** Default release-enabled views. Experimental views can be enabled from extension settings. */
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
    'state-transition-view': { label: 'State Transition View', shortLabel: 'State Transition', icon: 'git-compare' },
    'sequence-view': { label: 'Sequence View', shortLabel: 'Sequence', icon: 'list-ordered' },
};

/** Documentation: rendering technology per view. All views use D3 + ELK. */
export const VIEW_RENDERER_TECH: Record<string, string> = {
    'general-view': 'D3+ELK',
    'interconnection-view': 'D3+ELK',
    'action-flow-view': 'D3+ELK',
    'state-transition-view': 'D3+ELK',
    'sequence-view': 'D3+ELK',
};

/** General View type filter categories - def categories first for correct matching. */
export const GENERAL_VIEW_CATEGORIES = [
    { id: 'partDefs', label: 'Part defs', keywords: ['part def', 'part definition'], color: '#2D8A6E' },
    { id: 'parts', label: 'Parts', keywords: ['part'], color: '#2D8A6E' },
    { id: 'portDefs', label: 'Port defs', keywords: ['port def', 'port definition'], color: '#0E7C7B' },
    { id: 'ports', label: 'Ports', keywords: ['port'], color: '#0E7C7B' },
    { id: 'attributeDefs', label: 'Attribute defs', keywords: ['attribute def', 'attribute definition'], color: '#4A9B7F' },
    { id: 'attributes', label: 'Attributes', keywords: ['attribute'], color: '#4A9B7F' },
    { id: 'reqDefs', label: 'Requirement defs', keywords: ['requirement def', 'requirement definition'], color: '#5B8FC4' },
    { id: 'requirements', label: 'Requirements', keywords: ['requirement', 'req'], color: '#5B8FC4' },
    { id: 'actionDefs', label: 'Action defs', keywords: ['action def', 'action definition'], color: '#D4A02C' },
    { id: 'actions', label: 'Actions', keywords: ['action'], color: '#D4A02C' },
    { id: 'stateDefs', label: 'State defs', keywords: ['state def', 'state definition'], color: '#B85C38' },
    { id: 'states', label: 'States', keywords: ['state'], color: '#B85C38' },
    { id: 'interfaceDefs', label: 'Interface defs', keywords: ['interface def', 'interface definition'], color: '#7BAA7D' },
    { id: 'interfaces', label: 'Interfaces', keywords: ['interface'], color: '#7BAA7D' },
    { id: 'usecaseDefs', label: 'Use case defs', keywords: ['use case def', 'usecase def'], color: '#6B9BD1' },
    { id: 'usecases', label: 'Use cases', keywords: ['use case', 'usecase'], color: '#6B9BD1' },
    { id: 'allocationDefs', label: 'Allocation defs', keywords: ['allocation def', 'allocate def'], color: '#9CA3AF' },
    { id: 'allocations', label: 'Allocations', keywords: ['allocation', 'allocate'], color: '#9CA3AF' },
    { id: 'constraintDefs', label: 'Constraint defs', keywords: ['constraint def', 'constraint definition'], color: '#E07C5A' },
    { id: 'constraints', label: 'Constraints', keywords: ['constraint'], color: '#E07C5A' },
    { id: 'enumerations', label: 'Enumerations', keywords: ['enumeration', 'enum'], color: '#C9A227' },
    { id: 'metadata', label: 'Metadata', keywords: ['metadata'], color: '#8B7355' },
    { id: 'occurrences', label: 'Occurrences', keywords: ['occurrence'], color: '#5A9B6E' },
    { id: 'concerns', label: 'Concerns', keywords: ['concern', 'viewpoint', 'stakeholder', 'frame'], color: '#9CA3AF' },
    { id: 'items', label: 'Items', keywords: ['item'], color: '#5A9B6E' },
    { id: 'packages', label: 'Packages', keywords: ['package'], color: '#6B7280' },
    { id: 'other', label: 'Other', keywords: [] as string[], color: '#808080' },
] as const;

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
