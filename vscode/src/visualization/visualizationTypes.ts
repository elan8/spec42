export type SysmlRange = {
    start: { line: number; character: number };
    end: { line: number; character: number };
};

export type GraphNodeDto = {
    id?: string;
    parentId?: string | null;
    name?: string;
    qualifiedName?: string;
    qualified_name?: string;
    type?: string;
    elementKind?: string;
    element_kind?: string;
    uri?: string;
    range?: SysmlRange;
    attributes?: Record<string, unknown>;
    properties?: Record<string, unknown>;
    children?: GraphNodeDto[];
    relationships?: GraphEdgeDto[];
    typing?: string;
    typings?: string[];
};

export type GraphEdgeDto = {
    id?: string;
    source?: string;
    target?: string;
    type?: string;
    rel_type?: string;
    label?: string;
};

export type GraphPayloadDto = {
    nodes?: GraphNodeDto[];
    edges?: GraphEdgeDto[];
};

export type IbdPartDto = GraphNodeDto & {
    containerId?: string | null;
    container_id?: string | null;
};

export type IbdConnectorDto = GraphEdgeDto & {
    sourceId?: string;
    targetId?: string;
    sourcePortId?: string;
    targetPortId?: string;
};

export type IbdPayloadDto = {
    parts?: IbdPartDto[];
    ports?: GraphNodeDto[];
    connectors?: IbdConnectorDto[];
    containerGroups?: unknown[];
    packageContainerGroups?: unknown[];
    rootViews?: Record<string, IbdPayloadDto>;
    rootCandidates?: string[];
    defaultRoot?: string;
};

export type ActivityDiagramDto = {
    id?: string;
    name?: string;
    packagePath?: string;
    sourceKind?: string;
    actions?: GraphNodeDto[];
    flows?: GraphEdgeDto[];
    decisions?: GraphNodeDto[];
    states?: GraphNodeDto[];
    interface?: { inputs?: string[]; outputs?: string[] };
};

export type SequenceDiagramDto = {
    id?: string;
    name?: string;
    lifelines?: unknown[];
    messages?: unknown[];
    fragments?: unknown[];
    activations?: unknown[];
};

export type StateMachineDto = {
    id?: string;
    name?: string;
    states?: GraphNodeDto[];
    transitions?: GraphEdgeDto[];
};

export type VisualizationDataDto = {
    graph?: GraphPayloadDto;
    elements?: GraphNodeDto[];
    relationships?: GraphEdgeDto[];
    ibd?: IbdPayloadDto;
    activityDiagrams?: ActivityDiagramDto[];
    sequenceDiagrams?: SequenceDiagramDto[];
    stateMachines?: StateMachineDto[];
    viewCandidates?: unknown[];
    selectedView?: string;
    [key: string]: unknown;
};
