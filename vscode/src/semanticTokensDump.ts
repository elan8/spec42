/**
 * Semantic token decode helpers used by debugging commands.
 */

import * as vscode from "vscode";

export const SEMANTIC_TYPE_NAMES = [
  "KEYWORD",
  "STRING",
  "NUMBER",
  "COMMENT",
  "OPERATOR",
  "VARIABLE",
  "TYPE",
  "NAMESPACE",
  "CLASS",
  "INTERFACE",
  "PROPERTY",
  "FUNCTION",
];

/** Decoded token: line, start (UTF-16), length (UTF-16), type index, and extracted text */
export interface DecodedToken {
  line: number;
  start: number;
  length: number;
  type: number;
  text: string;
}

/**
 * Decode LSP semantic tokens (delta-encoded) into an array of DecodedToken.
 */
export function decodeSemanticTokens(
  document: vscode.TextDocument,
  tokens: vscode.SemanticTokens
): DecodedToken[] {
  const data = tokens.data;
  const lines = document.getText().split(/\r?\n/);
  const decoded: DecodedToken[] = [];
  let line = 0;
  let startChar = 0;
  for (let i = 0; i + 5 <= data.length; i += 5) {
    line += data[i];
    startChar = data[i] === 0 ? startChar + data[i + 1] : data[i + 1];
    const length = data[i + 2];
    const type = data[i + 3];
    const lineStr = lines[line] ?? "";
    const text = lineStr.slice(startChar, startChar + length); // slice uses UTF-16 indices
    decoded.push({ line, start: startChar, length, type, text });
  }
  return decoded;
}

/**
 * Find the token that contains the given position (0-based line and character, UTF-16).
 */
export function getTokenAtPosition(
  decoded: DecodedToken[],
  line: number,
  character: number
): DecodedToken | undefined {
  return decoded.find(
    (t) =>
      t.line === line &&
      character >= t.start &&
      character < t.start + t.length
  );
}

