import { prepareViewData } from "./prepare";
import { renderVisualization } from "./renderer";
import type { UnknownRecord } from "./prepare/types";

export interface HeadlessExportOptions {
  width?: number;
  height?: number;
  colorScheme?: "light" | "dark";
}

type AttrValue = string | number | boolean | null | undefined;

class VirtualStyle {
  private values = new Map<string, string>();

  setProperty(name: string, value: AttrValue): void {
    if (value == null) {
      this.values.delete(name);
      return;
    }
    this.values.set(name, String(value));
  }

  removeProperty(name: string): void {
    this.values.delete(name);
  }

  getPropertyValue(name: string): string {
    return this.values.get(name) ?? "";
  }

  toString(): string {
    return Array.from(this.values.entries())
      .map(([key, value]) => `${key}: ${value};`)
      .join(" ");
  }
}

class VirtualElement {
  readonly namespaceURI: string;
  readonly ownerDocument: VirtualDocument;
  readonly style = new VirtualStyle();
  readonly children: VirtualElement[] = [];
  parentNode: VirtualElement | null = null;
  textContent = "";
  clientWidth = 0;
  clientHeight = 0;
  private attrs = new Map<string, string>();

  constructor(
    ownerDocument: VirtualDocument,
    readonly tagName: string,
    namespaceURI = "http://www.w3.org/1999/xhtml",
  ) {
    this.ownerDocument = ownerDocument;
    this.namespaceURI = namespaceURI;
  }

  get childNodes(): VirtualElement[] {
    return this.children;
  }

  get firstChild(): VirtualElement | null {
    return this.children[0] ?? null;
  }

  get nextSibling(): VirtualElement | null {
    if (!this.parentNode) return null;
    const index = this.parentNode.children.indexOf(this);
    return index >= 0 ? this.parentNode.children[index + 1] ?? null : null;
  }

  get innerHTML(): string {
    return this.children.map((child) => child.serialize()).join("");
  }

  set innerHTML(value: string) {
    this.children.splice(0, this.children.length);
    this.textContent = value ? String(value) : "";
  }

  setAttribute(name: string, value: AttrValue): void {
    if (value == null) {
      this.attrs.delete(name);
      return;
    }
    this.attrs.set(name, String(value));
  }

  setAttributeNS(_namespace: string | null, name: string, value: AttrValue): void {
    this.setAttribute(name, value);
  }

  getAttribute(name: string): string | null {
    return this.attrs.get(name) ?? null;
  }

  hasAttribute(name: string): boolean {
    return this.attrs.has(name);
  }

  removeAttribute(name: string): void {
    this.attrs.delete(name);
  }

  appendChild(child: VirtualElement): VirtualElement {
    child.parentNode?.removeChild(child);
    child.parentNode = this;
    this.children.push(child);
    return child;
  }

  insertBefore(child: VirtualElement, before: VirtualElement | null): VirtualElement {
    child.parentNode?.removeChild(child);
    child.parentNode = this;
    if (!before) {
      this.children.push(child);
      return child;
    }
    const index = this.children.indexOf(before);
    if (index < 0) {
      this.children.push(child);
    } else {
      this.children.splice(index, 0, child);
    }
    return child;
  }

  removeChild(child: VirtualElement): VirtualElement {
    const index = this.children.indexOf(child);
    if (index >= 0) {
      this.children.splice(index, 1);
      child.parentNode = null;
    }
    return child;
  }

  remove(): void {
    this.parentNode?.removeChild(this);
  }

  addEventListener(): void {}
  removeEventListener(): void {}
  dispatchEvent(): boolean {
    return true;
  }

  querySelector(selector: string): VirtualElement | null {
    return this.querySelectorAll(selector)[0] ?? null;
  }

  querySelectorAll(selector: string): VirtualElement[] {
    const out: VirtualElement[] = [];
    const visit = (node: VirtualElement) => {
      for (const child of node.children) {
        if (child.matches(selector)) {
          out.push(child);
        }
        visit(child);
      }
    };
    visit(this);
    return out;
  }

  matches(selector: string): boolean {
    const trimmed = selector.trim();
    if (!trimmed) return false;
    if (trimmed === "*") return true;
    if (trimmed.startsWith(".")) {
      const classes = (this.getAttribute("class") ?? "").split(/\s+/);
      return classes.includes(trimmed.slice(1));
    }
    if (trimmed.startsWith("#")) {
      return this.getAttribute("id") === trimmed.slice(1);
    }
    if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
      const attr = trimmed.slice(1, -1).split("=")[0]?.trim();
      return Boolean(attr && this.hasAttribute(attr));
    }
    return this.tagName.toLowerCase() === trimmed.toLowerCase();
  }

  cloneNode(deep = false): VirtualElement {
    const clone = new VirtualElement(this.ownerDocument, this.tagName, this.namespaceURI);
    clone.textContent = this.textContent;
    clone.clientWidth = this.clientWidth;
    clone.clientHeight = this.clientHeight;
    for (const [key, value] of this.attrs.entries()) {
      clone.setAttribute(key, value);
    }
    const style = this.style.toString();
    if (style) {
      clone.setAttribute("style", style);
    }
    if (deep) {
      for (const child of this.children) {
        clone.appendChild(child.cloneNode(true));
      }
    }
    return clone;
  }

  serialize(): string {
    const style = this.style.toString();
    const attrs = new Map(this.attrs);
    if (style && !attrs.has("style")) {
      attrs.set("style", style);
    }
    const attrText = Array.from(attrs.entries())
      .map(([key, value]) => ` ${key}="${escapeXml(value)}"`)
      .join("");
    const body = `${escapeXml(this.textContent)}${this.children.map((child) => child.serialize()).join("")}`;
    return `<${this.tagName}${attrText}>${body}</${this.tagName}>`;
  }
}

class VirtualDocument {
  readonly documentElement: VirtualElement;
  readonly body: VirtualElement;

  constructor() {
    this.documentElement = new VirtualElement(this, "html");
    this.body = new VirtualElement(this, "body");
    this.documentElement.appendChild(this.body);
  }

  createElement(tagName: string): VirtualElement {
    return new VirtualElement(this, tagName);
  }

  createElementNS(namespaceURI: string, tagName: string): VirtualElement {
    return new VirtualElement(this, tagName, namespaceURI);
  }

  querySelector(selector: string): VirtualElement | null {
    return this.documentElement.querySelector(selector);
  }

  querySelectorAll(selector: string): VirtualElement[] {
    return this.documentElement.querySelectorAll(selector);
  }
}

class VirtualXmlSerializer {
  serializeToString(node: VirtualElement): string {
    return node.serialize();
  }
}

function ensureHeadlessDom(): VirtualDocument {
  const global = globalThis as unknown as {
    document?: VirtualDocument;
    window?: Record<string, unknown>;
    XMLSerializer?: typeof VirtualXmlSerializer;
    SVGElement?: typeof VirtualElement;
    Element?: typeof VirtualElement;
    Node?: typeof VirtualElement;
  };
  if (global.document) {
    return global.document;
  }
  const document = new VirtualDocument();
  global.document = document;
  global.window = {
    document,
    matchMedia: () => ({ matches: false, addEventListener: () => {}, removeEventListener: () => {} }),
  };
  global.XMLSerializer = VirtualXmlSerializer;
  global.SVGElement = VirtualElement;
  global.Element = VirtualElement;
  global.Node = VirtualElement;
  return document;
}

export async function exportHeadlessSvg(
  payload: UnknownRecord,
  options: HeadlessExportOptions = {},
): Promise<string> {
  const document = ensureHeadlessDom();
  const target = document.createElement("div");
  target.clientWidth = options.width ?? 1280;
  target.clientHeight = options.height ?? 900;
  document.body.appendChild(target);
  try {
    const prepared = prepareViewData(payload);
    const controller = await renderVisualization(target as unknown as HTMLElement, prepared, {
      delegateZoom: true,
      theme: { colorScheme: options.colorScheme ?? "light" },
    });
    const svg = controller.exportSvg();
    controller.destroy();
    return svg;
  } finally {
    target.remove();
  }
}

function escapeXml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

const globalApi = globalThis as unknown as {
  Spec42HeadlessRenderer?: { exportHeadlessSvg: typeof exportHeadlessSvg };
};
globalApi.Spec42HeadlessRenderer = { exportHeadlessSvg };
