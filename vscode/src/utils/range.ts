import * as vscode from "vscode";
import type { RangeDTO } from "../providers/sysmlModelTypes";

export function rangeContainsPosition(
  range: RangeDTO,
  position: vscode.Position
): boolean {
  const afterStart =
    position.line > range.start.line ||
    (position.line === range.start.line &&
      position.character >= range.start.character);
  const beforeEnd =
    position.line < range.end.line ||
    (position.line === range.end.line &&
      position.character <= range.end.character);
  return afterStart && beforeEnd;
}

export function rangeSpanScore(range: RangeDTO): number {
  return (
    (range.end.line - range.start.line) * 10000 +
    (range.end.character - range.start.character)
  );
}
