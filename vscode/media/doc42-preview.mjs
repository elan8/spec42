import { getDocument, PDFWorker } from "./pdf.min.mjs";

const vscode = acquireVsCodeApi();
const previous = document.getElementById("previous");
const next = document.getElementById("next");
const status = document.getElementById("status");
const canvas = document.getElementById("page");
document.getElementById("save").onclick = () => vscode.postMessage("save");
document.getElementById("open").onclick = () => vscode.postMessage("open");

async function openPreview() {
  // VS Code webviews load workers from blob URLs. PDF.js ships a standalone worker module.
  const workerResponse = await fetch(new URL("./pdf.worker.min.mjs", import.meta.url));
  if (!workerResponse.ok) throw new Error("The PDF worker could not be loaded.");
  const workerUrl = URL.createObjectURL(new Blob([await workerResponse.arrayBuffer()], { type: "text/javascript" }));
  const port = new Worker(workerUrl, { type: "module" });
  const worker = new PDFWorker({ port });
  const task = getDocument({
    url: document.body.dataset.pdf,
    worker,
    isEvalSupported: false,
    useWasm: false,
  });
  window.addEventListener("pagehide", () => {
    void task.destroy().catch(() => {}).finally(() => {
      worker.destroy();
      port.terminate();
      URL.revokeObjectURL(workerUrl);
    });
  }, { once: true });
  const pdf = await task.promise;
  let currentPage = Math.min(Math.max(vscode.getState()?.page ?? 1, 1), pdf.numPages);

  const render = async () => {
    previous.disabled = true;
    next.disabled = true;
    try {
      const page = await pdf.getPage(currentPage);
      const size = page.getViewport({ scale: 1 });
      const viewport = page.getViewport({ scale: Math.min(2, 2800 / Math.max(size.width, size.height)) });
      canvas.width = Math.ceil(viewport.width);
      canvas.height = Math.ceil(viewport.height);
      await page.render({ canvas, viewport }).promise;
      canvas.hidden = false;
      page.cleanup();
      status.textContent = `Page ${currentPage} of ${pdf.numPages}`;
      vscode.setState({ page: currentPage });
    } catch {
      canvas.hidden = true;
      status.textContent = "This page could not be rendered. You can open the PDF externally.";
    } finally {
      previous.disabled = currentPage <= 1;
      next.disabled = currentPage >= pdf.numPages;
    }
  };
  previous.onclick = () => { currentPage--; void render(); };
  next.onclick = () => { currentPage++; void render(); };
  await render();
}

void openPreview().catch(() => {
  status.textContent = "The PDF could not be previewed. You can open it externally.";
});
