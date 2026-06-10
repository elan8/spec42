import type { PreparedView, UnknownRecord } from "./types";
import { asArray, asRecord, asString, buildBehaviorNode } from "./util";

export function normalizeDiagramKey(value: string): string {
  return value.replace(/::/g, ".").trim().toLowerCase();
}

export function diagramSimpleName(value: string): string {
  const normalized = value.replace(/::/g, ".");
  const segments = normalized.split(".").filter(Boolean);
  return segments[segments.length - 1] ?? normalized;
}

export function diagramMatchesSelection(
  diagram: UnknownRecord,
  selectedName?: string,
  selectedViewId?: string,
): boolean {
  const selectors = [selectedName, selectedViewId].filter((value): value is string => Boolean(value?.trim()));
  if (selectors.length === 0) return false;

  const diagramKeys = [
    asString(diagram.id),
    asString(diagram.name),
    `${asString(diagram.package_path)}::${asString(diagram.name)}`.replace(/^::+/, ""),
  ].filter(Boolean);

  return selectors.some((selector) => {
    const selectorKey = normalizeDiagramKey(selector);
    const selectorSimple = diagramSimpleName(selector).toLowerCase();
    return diagramKeys.some((candidate) => {
      const candidateKey = normalizeDiagramKey(candidate);
      const candidateSimple = diagramSimpleName(candidate).toLowerCase();
      return (
        candidateKey === selectorKey ||
        candidateSimple === selectorSimple ||
        candidateKey.endsWith(`.${selectorKey}`) ||
        selectorKey.endsWith(`.${candidateKey}`) ||
        candidateKey.includes(selectorSimple) ||
        selectorKey.includes(candidateSimple)
      );
    });
  });
}

export function selectNamedDiagram(
  diagramsInput: unknown,
  selectedName?: string,
  selectedViewId?: string,
): UnknownRecord | null {
  const diagrams = asArray(diagramsInput).map(asRecord);
  if (diagrams.length === 0) return null;
  if (!selectedName && !selectedViewId) return null;
  const matched = diagrams.find((diagram) => diagramMatchesSelection(diagram, selectedName, selectedViewId));
  if (matched) return matched;
  return diagrams.length === 1 ? diagrams[0] : null;
}

export function bestBehaviorDiagram(diagrams: UnknownRecord[]): UnknownRecord | null {
  if (diagrams.length === 0) return null;
  const score = (diagram: UnknownRecord): number => {
    const nodes = asArray(diagram.nodes ?? diagram.actions ?? diagram.steps);
    const edges = asArray(diagram.edges ?? diagram.flows ?? diagram.transitions);
    return nodes.length * 10 + edges.length;
  };
  return diagrams.slice().sort((a, b) => score(b) - score(a))[0] ?? null;
}

export function diagramToPrepared(diagramInput: unknown, view: string, fallbackTitle: string): PreparedView {
  const diagram = asRecord(diagramInput);
  let nodes = asArray(diagram.nodes ?? diagram.states).map((nodeRaw, index) => {
    const node = asRecord(nodeRaw);
    return buildBehaviorNode(node, index, {
      id: `node-${index}`,
      label: `Node ${index + 1}`,
      kind: asString(node.type ?? node.kind, view),
    });
  });
  let edges = asArray(diagram.edges ?? diagram.transitions).map((edgeRaw, index) => {
    const edge = asRecord(edgeRaw);
    return {
      id: asString(edge.id, `edge-${index}`),
      source: asString(edge.source ?? edge.from ?? edge.sourceId, ""),
      target: asString(edge.target ?? edge.to ?? edge.targetId, ""),
      label: asString(edge.name ?? edge.label ?? edge.type, ""),
    };
  });
  if (view === "sequence-view" && nodes.length === 0) {
    nodes = asArray(diagram.lifelines).map((lifelineRaw, index) => {
      const lifeline = asRecord(lifelineRaw);
      return buildBehaviorNode(lifeline, index, {
        id: `lifeline-${index}`,
        label: `Lifeline ${index + 1}`,
        kind: "lifeline",
      });
    });
    edges = asArray(diagram.messages).map((messageRaw, index) => {
      const message = asRecord(messageRaw);
      return {
        id: asString(message.id, `message-${index}`),
        source: asString(message.source ?? message.from ?? message.sourceId, ""),
        target: asString(message.target ?? message.to ?? message.targetId, ""),
        label: asString(message.name ?? message.label ?? message.type, ""),
      };
    });
  }
  const ids = new Set(nodes.map((node) => node.id));
  edges = edges.filter((edge) => ids.has(edge.source) && ids.has(edge.target));
  return { title: asString(diagram.name, fallbackTitle), view, nodes, edges };
}
