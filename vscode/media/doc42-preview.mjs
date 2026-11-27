import { getDocument, PDFWorker } from "./pdf.min.mjs";

const vscode = acquireVsCodeApi();
const fitPage = document.getElementById("fit-page");
const zoomOut = document.getElementById("zoom-out");
const zoomIn = document.getElementById("zoom-in");
const zoomLevel = document.getElementById("zoom-level");
const status = document.getElementById("status");
const scrollView = document.getElementById("viewport");
const pageStack = document.getElementById("pages");
const minZoom = 0.15;
const maxZoom = 4;
const renderMargin = 200;
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
  const pages = await Promise.all(Array.from({ length: pdf.numPages }, async (_, index) => {
    const page = await pdf.getPage(index + 1);
    const { width, height } = page.getViewport({ scale: 1 });
    const element = document.createElement("section");
    element.className = "pdf-page";
    element.setAttribute("aria-label", `Page ${index + 1} of ${pdf.numPages}`);
    return { page, width, height, element };
  }));
  const fragment = document.createDocumentFragment();
  for (const { element } of pages) fragment.append(element);
  pageStack.append(fragment);

  const saved = vscode.getState();
  const clamp = (number, minimum, maximum) => Math.min(Math.max(number, minimum), maximum);
  let manualZoom = Number.isFinite(saved?.zoom) ? clamp(saved.zoom, minZoom, maxZoom) : null;
  let scale = 0;
  let nextFrame;
  let saveTimer;
  const rendered = new Map();

  function pageIndexAt(top) {
    let low = 0;
    let high = pages.length - 1;
    while (low < high) {
      const middle = Math.ceil((low + high) / 2);
      if (pages[middle].element.offsetTop <= top) low = middle;
      else high = middle - 1;
    }
    return low;
  }

  function position() {
    const center = scrollView.scrollTop + scrollView.clientHeight / 2;
    const index = pageIndexAt(center);
    const element = pages[index].element;
    const offset = clamp((center - element.offsetTop) / Math.max(element.offsetHeight, 1), 0, 1);
    return { index, offset };
  }

  function release(index, render) {
    rendered.delete(index);
    render.cancel();
    pages[index].element.replaceChildren();
  }

  function paint(index) {
    const { page, element, width, height } = pages[index];
    const density = Math.min(scale * window.devicePixelRatio, 4096 / Math.max(width, height),
      Math.sqrt(8_000_000 / (width * height)));
    const viewport = page.getViewport({ scale: density });
    const canvas = document.createElement("canvas");
    canvas.width = Math.ceil(viewport.width);
    canvas.height = Math.ceil(viewport.height);
    element.append(canvas);
    const render = page.render({ canvas, viewport });
    rendered.set(index, render);
    void render.promise.catch(() => {
      if (rendered.get(index) !== render) return;
      const error = document.createElement("p");
      error.textContent = "This page could not be rendered. You can open the PDF externally.";
      element.replaceChildren(error);
    }).finally(() => page.cleanup());
  }

  function savePosition() {
    const { index, offset } = position();
    vscode.setState({ page: index + 1, offset, zoom: manualZoom });
  }

  function updateViewport() {
    nextFrame = undefined;
    if (!scrollView.clientHeight) return;
    const top = scrollView.scrollTop;
    const first = pageIndexAt(top - renderMargin);
    const last = pageIndexAt(top + scrollView.clientHeight + renderMargin);
    for (const [index, render] of rendered) {
      if (index < first || index > last) release(index, render);
    }
    for (let index = first; index <= last; index++) {
      if (!rendered.has(index)) paint(index);
    }
    const label = `Page ${position().index + 1} of ${pdf.numPages}`;
    if (status.textContent !== label) status.textContent = label;
    clearTimeout(saveTimer);
    saveTimer = setTimeout(savePosition, 150);
  }

  function scheduleViewportUpdate() {
    nextFrame ??= requestAnimationFrame(updateViewport);
  }

  function layout(anchor, reposition = false) {
    const padding = getComputedStyle(pageStack);
    const availableWidth = scrollView.clientWidth - parseFloat(padding.paddingLeft) - parseFloat(padding.paddingRight);
    const availableHeight = scrollView.clientHeight - parseFloat(padding.paddingTop) - parseFloat(padding.paddingBottom);
    const active = pages[anchor.index];
    const fit = Math.max(0.01, Math.min(availableWidth / active.width, availableHeight / active.height));
    const nextScale = manualZoom ?? fit;
    const changed = nextScale !== scale;
    if (changed) {
      scale = nextScale;
      for (const [index, render] of rendered) release(index, render);
      for (const { element, width, height } of pages) {
        element.style.width = `${width * scale}px`;
        element.style.height = `${height * scale}px`;
      }
    }
    if (changed || reposition) {
      scrollView.scrollTop = active.element.offsetTop + active.height * scale * anchor.offset - scrollView.clientHeight / 2;
      scrollView.scrollLeft = (scrollView.scrollWidth - scrollView.clientWidth) / 2;
    }
    fitPage.disabled = false;
    fitPage.setAttribute("aria-pressed", String(manualZoom === null));
    zoomOut.disabled = scale <= minZoom;
    zoomIn.disabled = scale >= maxZoom;
    zoomLevel.textContent = `${Math.round(scale * 100)}%`;
    scheduleViewportUpdate();
  }

  function zoom(multiplier) {
    const anchor = position();
    manualZoom = clamp(scale * multiplier, minZoom, maxZoom);
    layout(anchor);
  }

  zoomOut.onclick = () => zoom(1 / 1.25);
  zoomIn.onclick = () => zoom(1.25);
  fitPage.onclick = () => {
    const { index } = position();
    manualZoom = null;
    layout({ index, offset: 0.5 }, true);
  };
  scrollView.addEventListener("scroll", scheduleViewportUpdate, { passive: true });
  const resize = new ResizeObserver(() => layout(position()));
  resize.observe(scrollView);
  window.addEventListener("pagehide", () => {
    clearTimeout(saveTimer);
    savePosition();
    resize.disconnect();
  }, { once: true });
  layout({
    index: clamp((saved?.page ?? 1) - 1, 0, pages.length - 1),
    offset: clamp(saved?.offset ?? 0.5, 0, 1),
  });
}

void openPreview().catch(() => {
  status.textContent = "The PDF could not be previewed. You can open it externally.";
});
