import type { PreparedPort } from "./types";

export const IBD_PORT_LABEL_FONT_SIZE = 8;
export const IBD_PORT_LABEL_HEIGHT = 10;
export const IBD_PORT_LABEL_MAX_LENGTH = 20;
const IBD_PORT_LABEL_CHARACTER_WIDTH = 5;

export function formatIbdPortLabel(name: string, detail?: PreparedPort): string {
  void detail;
  return String(name || "").trim();
}

export function ibdPortLabelText(name: string, detail?: PreparedPort): string {
  const label = formatIbdPortLabel(name, detail);
  return label.length > IBD_PORT_LABEL_MAX_LENGTH
    ? `${label.slice(0, IBD_PORT_LABEL_MAX_LENGTH - 1)}…`
    : label;
}

export function ibdPortLabelWidth(text: string): number {
  return Math.max(12, text.length * IBD_PORT_LABEL_CHARACTER_WIDTH);
}
