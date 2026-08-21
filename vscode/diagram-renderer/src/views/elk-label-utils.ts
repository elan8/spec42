type ElkRenderedLabel = {
  id?: string;
  text?: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
};

export type ElkLabelBox = {
  id: string;
  text: string;
  x: number;
  y: number;
  width: number;
  height: number;
};

export function estimateElkLabelBox(
  id: string,
  text: string,
  options?: {
    paddingX?: number;
    paddingY?: number;
    minWidth?: number;
    minHeight?: number;
    charWidth?: number;
  },
): ElkLabelBox {
  const paddingX = options?.paddingX ?? 10;
  const paddingY = options?.paddingY ?? 8;
  const minWidth = options?.minWidth ?? 42;
  const minHeight = options?.minHeight ?? 18;
  const charWidth = options?.charWidth ?? 6;
  return {
    id,
    text,
    x: 0,
    y: 0,
    width: Math.max(minWidth, text.length * charWidth + paddingX),
    height: Math.max(minHeight, paddingY + 10),
  };
}

export function toAbsoluteElkLabelBox(
  label: ElkRenderedLabel | null | undefined,
  offset: { x: number; y: number } = { x: 0, y: 0 },
): ElkLabelBox | null {
  if (!label) return null;
  const id = String(label.id || "");
  const text = String(label.text || "");
  if (!id || !text) return null;
  return {
    id,
    text,
    x: (label.x ?? 0) + offset.x,
    y: (label.y ?? 0) + offset.y,
    width: label.width ?? 0,
    height: label.height ?? 0,
  };
}

export function collectElkEdgeLabels(
  elkNode: { edges?: unknown[]; children?: unknown[]; x?: number; y?: number } | null | undefined,
  offset: { x: number; y: number },
  acc: Map<string, ElkLabelBox[]>,
): void {
  (elkNode?.edges ?? []).forEach((edgeRaw) => {
    const edge = edgeRaw as { id?: string; labels?: ElkRenderedLabel[] };
    if (!edge?.id || !Array.isArray(edge.labels) || edge.labels.length === 0) {
      return;
    }
    const labels = edge.labels
      .map((label) => toAbsoluteElkLabelBox(label, offset))
      .filter((label): label is ElkLabelBox => Boolean(label));
    if (labels.length > 0) {
      acc.set(String(edge.id), labels);
    }
  });
  (elkNode?.children ?? []).forEach((childRaw) => {
    const child = childRaw as { x?: number; y?: number; edges?: unknown[]; children?: unknown[] };
    collectElkEdgeLabels(child, { x: offset.x + (child.x ?? 0), y: offset.y + (child.y ?? 0) }, acc);
  });
}

export function edgeLabelPositionFromSections(
  sections: Array<{
    startPoint?: { x: number; y: number };
    endPoint?: { x: number; y: number };
    bendPoints?: Array<{ x: number; y: number }>;
  }> | undefined,
): { x: number; y: number } | null {
  const section = sections?.[0];
  if (!section?.startPoint || !section?.endPoint) {
    return null;
  }
  const points = [section.startPoint, ...(section.bendPoints ?? []), section.endPoint];
  const midIndex = Math.floor((points.length - 1) / 2);
  const start = points[midIndex];
  const end = points[midIndex + 1] ?? points[midIndex];
  return {
    x: (start.x + end.x) / 2,
    y: (start.y + end.y) / 2 - 6,
  };
}
