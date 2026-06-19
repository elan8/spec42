/**
 * Thin pass-through normalization for LSP visualization DTOs.
 * Semantic shaping lives in semantic_core before serialization.
 */

type UnknownRecord = Record<string, unknown>;

function asArray<T>(value: unknown): T[] {
  return Array.isArray(value) ? (value as T[]) : [];
}

function countItems(value: unknown): number {
  return asArray(value).length;
}

function buildSelectorLabel(name: string, packagePath: string): string {
  return packagePath ? `${name} - ${packagePath}` : name;
}

function aliasDiagramsField(data: UnknownRecord, sourceKey: string): UnknownRecord {
  const source = asArray<UnknownRecord>(data[sourceKey]);
  return {
    ...data,
    diagrams: source,
  };
}

function activityCandidates(diagrams: UnknownRecord[]): UnknownRecord[] {
  return diagrams.map((diagram) => ({
    id: diagram.id,
    name: diagram.name,
    label: diagram.label ?? buildSelectorLabel(String(diagram.name ?? ""), String(diagram.packagePath ?? diagram.package_path ?? "")),
    packagePath: diagram.packagePath ?? diagram.package_path,
    sourceKind: diagram.sourceKind ?? diagram.source_kind,
    nodeCount: countItems(diagram.nodes ?? diagram.actions),
    flowCount: countItems(diagram.flows),
  }));
}

function stateMachineCandidates(machines: UnknownRecord[]): UnknownRecord[] {
  return machines.map((machine) => ({
    id: machine.id,
    name: machine.name,
    label: machine.label ?? buildSelectorLabel(String(machine.name ?? ""), String(machine.packagePath ?? machine.package_path ?? "")),
    packagePath: machine.packagePath ?? machine.package_path,
    stateCount: countItems(machine.states),
    transitionCount: countItems(machine.transitions),
  }));
}

function sequenceCandidates(diagrams: UnknownRecord[]): UnknownRecord[] {
  return diagrams.map((diagram) => ({
    id: diagram.id,
    name: diagram.name,
    label: diagram.label ?? buildSelectorLabel(String(diagram.name ?? ""), String(diagram.packagePath ?? diagram.package_path ?? "")),
    packagePath: diagram.packagePath ?? diagram.package_path,
    messageCount: countItems(diagram.messages),
    lifelineCount: countItems(diagram.lifelines),
  }));
}

export function normalizeVisualizationPayload(
  data: UnknownRecord | null | undefined,
): UnknownRecord | null | undefined {
  if (!data) {
    return data;
  }

  const view = String(data.view || "general-view");

  switch (view) {
    case "general-view":
      return data;

    case "interconnection-view": {
      if (data.interconnectionScene && typeof data.interconnectionScene === "object") {
        return data;
      }
      return {
        ...data,
        elements: [],
        parts: [],
        ports: [],
        connectors: [],
        containerGroups: [],
        packageContainerGroups: [],
      };
    }

    case "action-flow-view": {
      const activityDiagrams = asArray<UnknownRecord>(data.activityDiagrams);
      const diagrams = activityDiagrams.map((diagram) => ({
        ...diagram,
        nodes: diagram.nodes ?? diagram.actions,
        hasBehavioralFlow: countItems(diagram.flows) > 0,
        hasRenderableContent: countItems(diagram.flows) > 0 && countItems(diagram.nodes ?? diagram.actions) > 0,
      }));
      return {
        ...data,
        diagrams,
        activityDiagramCandidates: activityCandidates(diagrams),
      };
    }

    case "state-transition-view": {
      const stateMachines = asArray<UnknownRecord>(data.stateMachines);
      const flatStates = stateMachines.flatMap((machine) => asArray(machine.states));
      const flatTransitions = stateMachines.flatMap((machine) => asArray(machine.transitions));
      return {
        ...data,
        stateMachines,
        stateMachineCandidates: stateMachineCandidates(stateMachines),
        states: flatStates,
        transitions: flatTransitions,
      };
    }

    case "sequence-view": {
      const sequenceDiagrams = asArray<UnknownRecord>(data.sequenceDiagrams);
      return {
        ...aliasDiagramsField(data, "sequenceDiagrams"),
        sequenceDiagramCandidates: sequenceCandidates(sequenceDiagrams),
      };
    }

    default:
      return data;
  }
}
