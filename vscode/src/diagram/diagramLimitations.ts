import type { DiagramProduct } from "../generated/diagram-product/DiagramProduct";
import type { DiagramNode } from "../generated/diagram-product/DiagramNode";
import type { IncompleteReason } from "../generated/diagram-product/IncompleteReason";

export type LimitationTarget = { uri: string; startLine: number; startCharacter: number; endLine: number; endCharacter: number };
export type LimitationDetail = { code: string; explanation: string; links: Array<{ label: string; target: LimitationTarget }> };

function referenceLabel(product: DiagramProduct, index: number): string {
  const reference = product.references[index];
  if (!reference) return "Unavailable element";
  switch (reference.kind) {
    case "qualified-name": return reference.qualifiedName;
    case "tooling-element-id": return reference.elementId;
    case "source-anchor": return `Anonymous ${reference.metaclass}`;
    case "relationship": return `${reference.relationshipKind} #${reference.ordinal}`;
  }
}

function sourceTarget(product: DiagramProduct, index: number): LimitationTarget | undefined {
  const reference = product.references[index];
  const sourceIndex = reference?.kind === "source-anchor" ? reference.source
    : product.projection.nodes.find((node) => node.reference === index)?.source;
  const source = sourceIndex === undefined ? undefined : product.sources[sourceIndex];
  const document = source && product.documents[source.document];
  if (!source || !document) return undefined;
  return { uri: document.uri, startLine: source.range[0], startCharacter: source.range[1], endLine: source.range[2], endCharacter: source.range[3] };
}

function nodeLabel(product: DiagramProduct, index: number): string {
  const names: string[] = [];
  const seen = new Set<number>();
  let current: number | null = index;
  while (current !== null && !seen.has(current)) {
    seen.add(current);
    const node: DiagramNode | undefined = product.projection.nodes[current];
    if (!node) break;
    names.unshift(node.name ?? `Anonymous ${node.metaclass}`);
    current = node.owner;
  }
  return names.join("::");
}

function nodeReferences(product: DiagramProduct, nodes: number[]): number[] {
  return nodes.flatMap((index) => {
    const node = product.projection.nodes[index];
    return node ? [node.reference] : [];
  });
}

function explanation(product: DiagramProduct, reason: IncompleteReason): { text: string; references: number[] } {
  switch (reason.code) {
    case "parse-recovery": return { text: "Source contains syntax errors. This drawing includes only the content recovered by the compiler.", references: [] };
    case "unsupported-syntax": return { text: "The publication contains syntax whose semantics are not supported yet. The drawing may omit content.", references: [] };
    case "non-converged": return { text: "Semantic analysis did not converge. Some results remain unsettled.", references: [] };
    case "exposure-unresolved": return { text: "An expose target cannot be resolved. Check its name and imports.", references: [reason.exposure] };
    case "exposure-ambiguous": return { text: "An expose target names several elements. Qualify the intended target.", references: [reason.exposure] };
    case "exposure-unsupported": return { text: "This expose target cannot currently be projected.", references: [reason.exposure] };
    case "relationship-unresolved": return { text: `A ${reason.relationshipKind} endpoint cannot be resolved. Check the referenced name and its scope.`, references: [] };
    case "relationship-ambiguous": return { text: `A ${reason.relationshipKind} endpoint names several elements. Qualify the intended endpoint.`, references: [] };
    case "relationship-unsupported": return { text: `A ${reason.relationshipKind} relationship cannot currently be projected.`, references: [] };
    case "view-filter-unresolved": return { text: "A view filter cannot be resolved. Check its references.", references: [] };
    case "view-filter-ambiguous": return { text: "A view filter is ambiguous. Qualify its references.", references: [] };
    case "view-filter-unsupported": return { text: "A view filter uses an unsupported form. Its selected content is unsettled.", references: [] };
    case "geometry-facts-unavailable": return { text: "Geometry facts are not available for this view.", references: [] };
    case "sequence-message-endpoint-outside-lifeline": return { text: "A message endpoint is outside the exposed lifelines. Expose the relevant participant or correct the endpoint.", references: [] };
    case "sequence-content-omitted": return {
      text: "Messages inside nested occurrences or structured behavior are not projected by this sequence view. Expose a supported scenario separately; this view is incomplete.",
      references: nodeReferences(product, reason.elements),
    };
    case "sequence-ordering-cycle": return {
      text: "The modeled precedence contains a cycle: " + reason.witness.map((step) =>
        `${nodeLabel(product, step.source)} → ${nodeLabel(product, step.target)} (${step.provenance === "authored" ? "authored succession" : "event causality"}${step.events.length ? "; " + step.events.map((event) => nodeLabel(product, event)).join(" → ") : ""})`
      ).join("; ") + ". Correct this contradictory ordering; messages blocked by it have no drawable order.",
      references: nodeReferences(product, reason.witness.flatMap((step) => [step.source, step.target, ...step.events])),
    };
  }
}

/** Presentation only: every reason and witness comes from the same canonical product as the SVG. */
export function diagramLimitations(product: DiagramProduct): LimitationDetail[] {
  return product.completeness.reasons.map((reason) => {
    const detail = explanation(product, reason);
    const links = [...new Set(detail.references)].flatMap((reference) => {
      const target = sourceTarget(product, reference);
      return target ? [{ label: referenceLabel(product, reference), target }] : [];
    });
    if (links.length === 0) {
      const source = product.sources[product.selectedView.source];
      const document = source && product.documents[source.document];
      if (source && document) links.push({ label: "View definition", target: { uri: document.uri, startLine: source.range[0], startCharacter: source.range[1], endLine: source.range[2], endCharacter: source.range[3] } });
    }
    return { code: reason.code, explanation: detail.text, links };
  });
}
