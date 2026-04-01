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
    const id = String(label.id || '');
    const text = String(label.text || '');
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
