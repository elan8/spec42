import { prepareViewData } from "../../diagram-renderer/src/prepare";
import { renderVisualization } from "../../diagram-renderer/src/renderer";

declare function acquireVsCodeApi(): { postMessage(message: unknown): void };

type Product = {
  completeness: { status: string; reasons: Array<{ code: string }> };
  projection: { elements: unknown[] };
};

async function main(): Promise<void> {
  const target = document.getElementById("diagram");
  const source = document.getElementById("diagram-product");
  if (!(target instanceof HTMLElement) || !source?.textContent) return;
  const product = JSON.parse(source.textContent) as Product;
  if (product.projection.elements.length === 0 && product.completeness.status !== "complete") {
    const empty = document.createElement("div");
    empty.className = "empty";
    empty.textContent = product.completeness.reasons.map((reason) => reason.code).join(" ");
    target.appendChild(empty);
    return;
  }
  const prepared = prepareViewData(product);
  const vscode = acquireVsCodeApi();
  await renderVisualization(target, prepared, {
    theme: { colorScheme: "vscode" },
    onNodeClick: (node) => {
      const range = node.range;
      const uri = node.uri ?? node.sourcePath;
      if (!uri || !range?.start || !range.end) return;
      vscode.postMessage({
        type: "openSource",
        target: {
          uri,
          startLine: range.start.line,
          startCharacter: range.start.character ?? 0,
          endLine: range.end.line ?? range.start.line,
          endCharacter: range.end.character ?? range.start.character ?? 0,
        },
      });
    },
  });
}

void main();
