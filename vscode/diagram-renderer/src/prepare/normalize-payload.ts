/**
 * Thin pass-through normalization for LSP visualization DTOs.
 * Semantic shaping and selector candidates live in semantic_core before serialization.
 */

type UnknownRecord = Record<string, unknown>;

function asArray<T>(value: unknown): T[] {
  return Array.isArray(value) ? (value as T[]) : [];
}

function countItems(value: unknown): number {
  return asArray(value).length;
}

function aliasDiagramsField(data: UnknownRecord, sourceKey: string): UnknownRecord {
  const source = asArray<UnknownRecord>(data[sourceKey]);
  return {
    ...data,
    diagrams: source,
  };
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
        hasRenderableContent:
          countItems(diagram.flows) > 0 && countItems(diagram.nodes ?? diagram.actions) > 0,
      }));
      return {
        ...data,
        diagrams,
      };
    }

    case "state-transition-view": {
      const stateMachines = asArray<UnknownRecord>(data.stateMachines);
      return {
        ...data,
        stateMachines,
        states: stateMachines.flatMap((machine) => asArray(machine.states)),
        transitions: stateMachines.flatMap((machine) => asArray(machine.transitions)),
      };
    }

    case "sequence-view":
      return aliasDiagramsField(data, "sequenceDiagrams");

    default:
      return data;
  }
}
