import type { PreparedPort } from "./types";

export const IBD_PORT_LABEL_FONT_SIZE = 8;
export const IBD_PORT_LABEL_HEIGHT = 10;
const IBD_PORT_LABEL_MAX_LENGTH = 24;
const IBD_PORT_LABEL_CHARACTER_WIDTH = 5;

export function formatIbdPortLabel(name: string, detail?: PreparedPort): string {
  const direction = String(detail?.direction || "").trim();
  const directionPrefix = direction ? `${direction} ` : "";
  const type = String(detail?.portType || detail?.attributes?.portType || "").trim();
  if (!type) return `${directionPrefix}${name}`;
  const conjugated = type.startsWith("~");
  const cleanType = type.replace(/^~/, "").split(/::|\./).pop() || type.replace(/^~/, "");
  return `${directionPrefix}${name}: ${conjugated ? "~" : ""}${cleanType}`;
}

export function ibdPortLabelText(name: string, detail?: PreparedPort): string {
  const label = formatIbdPortLabel(name, detail);
  return label.length > IBD_PORT_LABEL_MAX_LENGTH
    ? `${label.slice(0, IBD_PORT_LABEL_MAX_LENGTH - 1)}...`
    : label;
}

export function ibdPortLabelWidth(text: string): number {
  return Math.max(12, text.length * IBD_PORT_LABEL_CHARACTER_WIDTH);
}
