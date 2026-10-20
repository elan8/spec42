import type { SourceNavigation } from "./diagramViewerCore";

export type ViewChoice = {
  handle: string;
  label: string;
  group: string;
  kindLabel?: string;
  definition?: SourceNavigation;
};

export function populateViewSelect(select: HTMLSelectElement, views: ViewChoice[], selected: string): void {
  const groups = new Map<string, HTMLOptGroupElement>();
  for (const view of views) {
    let group = groups.get(view.group);
    if (!group) {
      group = document.createElement("optgroup");
      group.label = view.group;
      groups.set(view.group, group);
    }
    const option = document.createElement("option");
    option.value = view.handle;
    option.textContent = view.label;
    option.title = view.kindLabel ?? "";
    option.selected = view.handle === selected;
    group.appendChild(option);
  }
  select.replaceChildren(...groups.values());
  select.disabled = views.length === 0;
}

/** Change only the canvas background, retaining semantic element fills and the viewBox. */
export function withExportBackground(svg: string, transparent: boolean): string {
  const parsed = new DOMParser().parseFromString(svg, "image/svg+xml");
  const root = parsed.documentElement;
  if (root.localName !== "svg" || parsed.querySelector("parsererror")) throw new Error("Invalid view SVG for export.");
  root.querySelectorAll(".viz-bg, .view-export-background").forEach((background) => background.remove());
  root.style.background = "none";
  if (!transparent) {
    const box = root.getAttribute("viewBox")?.trim().split(/\s+/).map(Number);
    if (!box || box.length !== 4 || !box.every(Number.isFinite) || box[2] <= 0 || box[3] <= 0) {
      throw new Error("The view has no valid export bounds.");
    }
    const rect = parsed.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("class", "view-export-background");
    ["x","y","width","height"].forEach((key,index) => rect.setAttribute(key, String(box[index])));
    rect.setAttribute("fill", "#ffffff");
    root.insertBefore(rect, root.firstChild);
  }
  return new XMLSerializer().serializeToString(root);
}
