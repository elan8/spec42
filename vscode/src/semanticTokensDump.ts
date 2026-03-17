/**
 * Debug dump of semantic tokens received from the LSP (frontend side).
 * Used when spec42.debug.dumpSemanticTokens is true.
 */

import * as fs from "fs";
import * as path from "path";
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

/**
 * Decode LSP semantic tokens (delta-encoded) and write a human-readable dump to outPath.
 * Server legend: 0=KEYWORD, 5=VARIABLE, 6=TYPE, 10=PROPERTY.
 */
export function dumpSemanticTokens(
  document: vscode.TextDocument,
  tokens: vscode.SemanticTokens,
  outPath: string
): void {
  const decoded = decodeSemanticTokens(document, tokens);
  const lines = document.getText().split(/\r?\n/);

  let report = "Frontend semantic tokens dump (from LSP)\n";
  report += "==========================================\n";
  report += `Document: ${document.uri.toString()}\n`;
  report += `Legend: 0=KEYWORD, 5=VARIABLE, 6=TYPE, 10=PROPERTY\n`;
  report += "Expected: in/out=KEYWORD, panAngle/current/velocity/position=!KEYWORD\n\n";
  report += "Source (first 30 lines):\n";
  lines.slice(0, 30).forEach((l, i) => {
    report += `  ${i}: ${l}\n`;
  });
  report += "\nTokens:\n";
  for (const t of decoded) {
    const typeName = SEMANTIC_TYPE_NAMES[t.type] ?? `?${t.type}`;
    report += `  ${t.line}:${t.start} len=${t.length} type=${typeName} ${JSON.stringify(t.text)}\n`;
  }
  report += "\nProblematic identifiers (should NOT be KEYWORD):\n";
  const problemIds = [
    "panAngle",
    "current",
    "velocity",
    "position",
    "tiltAngle",
    "mode",
    "voltage",
    "attitude",
    "altitude",
  ];
  for (const ident of problemIds) {
    const found = decoded.filter((t) => t.text === ident);
    if (found.length === 0) {
      report += `  ${ident}: NOT FOUND\n`;
    } else {
      for (const t of found) {
        const typeName = SEMANTIC_TYPE_NAMES[t.type] ?? `?${t.type}`;
        const status = t.type === 0 ? " *** WRONG (KEYWORD) ***" : " ok";
        report += `  ${ident}: ${typeName} at ${t.line}:${t.start}${status}\n`;
      }
    }
  }
  report += "\nTroubleshooting if identifiers still appear highlighted like keywords:\n";
  report += "  1. Ensure editor.semanticHighlighting.enabled is true (or configuredByTheme with a theme that has semanticHighlighting)\n";
  report += "  2. Try editor.renderWhitespace: 'none' (some themes misbehave with 'all')\n";
  report += "  3. Disable other SysML/KerML extensions that might provide conflicting TextMate grammars\n";
  report += "  4. Reload window (Ctrl+Shift+P → Developer: Reload Window)\n";
  report += "\n(Set spec42.debug.dumpSemanticTokens to false to stop dumps.)\n";

  try {
    const dir = path.dirname(outPath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(outPath, report, "utf8");
  } catch (e) {
    console.error("Failed to write semantic tokens dump:", e);
  }
}
