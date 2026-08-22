import type { NodeChrome } from "./node-notation";
import { headerFillPath, nodeBodyChromeStyle, nodeInnerSpan, resolveNodeChrome } from "./node-notation";
import type { DiagramTheme } from "./theme";

export interface SysMLNodeDetailItem {
  name: string;
  typeName?: string | null;
  valueText?: string | null;
  declaredIn?: string | null;
  displayText: string;
}

export interface SysMLNodeSection {
  key: string;
  title: string;
  items: SysMLNodeDetailItem[];
  collapsed?: boolean;
  showAll?: boolean;
}

export interface SysMLNodeCompartments {
  header: { stereotype: string; name: string };
  typedByName?: string | null;
  attributes: SysMLNodeDetailItem[];
  parts: SysMLNodeDetailItem[];
  ports: SysMLNodeDetailItem[];
  collapsibleSections?: SysMLNodeSection[];
  other?: Array<{ title: string; lines: string[] }>;
}

export interface SysMLNodeConfig {
  showHeader?: boolean;
  showAttributes?: boolean;
  showParts?: boolean;
  showPorts?: boolean;
  showOther?: boolean;
  maxLinesPerCompartment?: number;
}

/**
 * Node chrome metrics.
 *
 * These are per-line and per-region primitives, not fixed node coordinates: header height is
 * derived from header content by `layoutNodeHeader`, and body height from the compartment blocks
 * by `layoutSysMLNode`. Layout (ELK sizing) and drawing both go through those functions, so the
 * measured box and the painted box cannot disagree.
 */
export const LINE_HEIGHT = 12;
export const COMPARTMENT_LABEL_HEIGHT = 15;
export const COMPARTMENT_PADDING = 4;
export const COMPARTMENT_GAP = 2;
export const PADDING = 8;
export const HEADER_PADDING_X = 8;
export const HEADER_PADDING_TOP = 7;
export const HEADER_PADDING_BOTTOM = 7;
export const STEREOTYPE_FONT_SIZE = 9;
export const STEREOTYPE_LINE_HEIGHT = 11;
export const NAME_FONT_SIZE = 11;
export const NAME_LINE_HEIGHT = 14;
export const NAME_MAX_LINES = 2;
export const TYPING_FONT_SIZE = 9.5;
export const TYPING_LINE_HEIGHT = 12;
export const COMPARTMENT_FONT_SIZE = 9;
/** Minimum practical pointer target for a disclosure control, in CSS pixels at zoom 1. */
export const DISCLOSURE_TARGET_SIZE = 24;
/** Painted size of the +/- disclosure box inside that target. */
export const DISCLOSURE_BOX_SIZE = 13;
export const BADGE_HEIGHT = 15;
/** Clearance between a control's focus ring and the node border. */
export const CONTROL_EDGE_INSET = 3;
export const BADGE_MIN_WIDTH = 18;
const OVERFLOW_LINE_HEIGHT = 12;
/**
 * Conservative average advance width as a fraction of font size for the diagram sans stack. Text
 * is never measured at layout time (layout must stay deterministic and DOM-free), so widths are
 * estimated and every truncated string keeps its full text in a `<title>`.
 */
const AVERAGE_GLYPH_RATIO_BOLD = 0.6;
const AVERAGE_GLYPH_RATIO_REGULAR = 0.54;

export const IBD_NODE_CONFIG: SysMLNodeConfig = {
  showHeader: true,
  showAttributes: false,
  showParts: true,
  showPorts: true,
  showOther: false,
  maxLinesPerCompartment: 6,
};

export const DEFAULT_SYSML_NODE_CONFIG: Required<SysMLNodeConfig> = {
  showHeader: true,
  showAttributes: true,
  showParts: true,
  showPorts: true,
  showOther: true,
  maxLinesPerCompartment: 8,
};

const DEFAULT_CONFIG = DEFAULT_SYSML_NODE_CONFIG;

type D3Selection = {
  append: (name: string) => D3Selection;
  attr: (name: string, value: unknown) => D3Selection;
  style: (name: string, value: unknown) => D3Selection;
  text: (value: unknown) => D3Selection;
  on: (type: string, handler: (event: any) => void) => D3Selection;
};

function asString(value: unknown, fallback = ""): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  return fallback;
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function normalizeUnitBrackets(text: string): string {
  let out = text;
  while (/\[\[[^\[\]]+\]\]/.test(out)) {
    out = out.replace(/\[\[([^\[\]]+)\]\]/g, "[$1]");
  }
  return out;
}

function normalizeDetailItem(item: unknown): SysMLNodeDetailItem | null {
  if (typeof item === "string") {
    const text = normalizeUnitBrackets(item.trim());
    return text ? { name: text, displayText: text } : null;
  }
  if (!item || typeof item !== "object") return null;
  const record = item as Record<string, unknown>;
  const name = asString(record.name).trim();
  const displayText = normalizeUnitBrackets(asString(record.displayText, name).trim());
  if (!displayText) return null;
  return {
    name: name || displayText,
    typeName: asString(record.typeName) || null,
    valueText: asString(record.valueText) || null,
    declaredIn: asString(record.declaredIn) || null,
    displayText,
  };
}

function detailItems(attributes: Record<string, unknown>, key: string): SysMLNodeDetailItem[] {
  return asArray(attributes[key])
    .map((item) => normalizeDetailItem(item))
    .filter((item): item is SysMLNodeDetailItem => Boolean(item));
}

function readAttributeBagValue(element: unknown, key: string): unknown[] {
  if (!element || typeof element !== "object") return [];
  const record = element as Record<string, unknown>;
  const attrs = record.attributes;
  let rawValue: unknown;
  if (attrs && typeof attrs === "object" && "get" in attrs && typeof (attrs as { get: (k: string) => unknown }).get === "function") {
    rawValue = (attrs as { get: (k: string) => unknown }).get(key);
  } else if (attrs && typeof attrs === "object") {
    rawValue = (attrs as Record<string, unknown>)[key];
  }
  return Array.isArray(rawValue) ? rawValue : [];
}

function detailItemsFromElementBag(element: unknown, key: string): SysMLNodeDetailItem[] {
  return readAttributeBagValue(element, key)
    .map((item) => normalizeDetailItem(item))
    .filter((item): item is SysMLNodeDetailItem => Boolean(item));
}

export function lineToDetailItem(line: string): SysMLNodeDetailItem {
  const text = normalizeUnitBrackets(line.trim());
  return { name: text, displayText: text };
}

/** Collect compartments from general-view element graph nodes. */
export function collectCompartmentsFromElement(element: unknown): SysMLNodeCompartments {
  const el = (element && typeof element === "object" ? element : {}) as Record<string, unknown>;
  const headerName = asString(el.name ?? el.elementName ?? el.label, "Unnamed");
  const result: SysMLNodeCompartments = {
    header: { stereotype: asString(el.type, "element").toLowerCase(), name: headerName },
    typedByName: null,
    attributes: [],
    parts: [],
    ports: [],
    collapsibleSections: [],
    other: [],
  };

  const attrs = el.attributes;
  if (attrs && typeof attrs === "object" && "get" in attrs && typeof (attrs as { get: (k: string) => unknown }).get === "function") {
    const bag = attrs as { get: (k: string) => unknown };
    result.typedByName =
      asString(bag.get("partType") || bag.get("type") || bag.get("typedBy")) || null;
  } else if (attrs && typeof attrs === "object") {
    const bag = attrs as Record<string, unknown>;
    result.typedByName = asString(bag.partType ?? bag.type ?? bag.typedBy) || null;
  }
  if (!result.typedByName && el.partType) {
    result.typedByName = asString(el.partType);
  }
  const typings = el.typings;
  if (!result.typedByName && Array.isArray(typings) && typings.length > 0) {
    result.typedByName = asString(typings[0]).replace(/^[:~]+/, "").trim();
  }
  if (!result.typedByName && el.typing) {
    result.typedByName = asString(el.typing).replace(/^[:~]+/, "").trim();
  }

  result.attributes = detailItemsFromElementBag(el, "generalViewDirectAttributes");
  result.parts = detailItemsFromElementBag(el, "generalViewDirectParts");
  result.ports = detailItemsFromElementBag(el, "generalViewDirectPorts");

  const inheritedAttributes = detailItemsFromElementBag(el, "generalViewInheritedAttributes");
  if (inheritedAttributes.length > 0) {
    result.collapsibleSections!.push({
      key: "inherited-attributes",
      title: "Attributes",
      items: inheritedAttributes,
      collapsed: true,
      showAll: false,
    });
  }
  const inheritedParts = detailItemsFromElementBag(el, "generalViewInheritedParts");
  if (inheritedParts.length > 0) {
    result.collapsibleSections!.push({
      key: "inherited-parts",
      title: "Parts",
      items: inheritedParts,
      collapsed: true,
      showAll: false,
    });
  }

  return result;
}

/** Collect compartments from IBD part nodes. */
export function collectCompartmentsFromPart(part: unknown, ports: unknown[]): SysMLNodeCompartments {
  const p = (part && typeof part === "object" ? part : {}) as Record<string, unknown>;
  const result: SysMLNodeCompartments = {
    header: { stereotype: asString(p.type, "part").toLowerCase(), name: asString(p.name, "Unnamed") },
    typedByName: null,
    attributes: [],
    parts: [],
    ports: [],
    collapsibleSections: [],
    other: [],
  };

  const attrs = p.attributes;
  if (attrs && typeof attrs === "object" && "get" in attrs && typeof (attrs as { get: (k: string) => unknown }).get === "function") {
    const bag = attrs as { get: (k: string) => unknown };
    result.typedByName = asString(bag.get("partType") || bag.get("type") || bag.get("typedBy")) || null;
  }
  if (!result.typedByName && p.partType) {
    result.typedByName = asString(p.partType);
  }

  const partName = asString(p.name);
  const partId = asString(p.id);
  const partQn = asString(p.qualifiedName);
  const partPorts = ports.filter((port) => {
    if (!port || typeof port !== "object") return false;
    const pr = port as Record<string, unknown>;
    const parentId = asString(pr.parentId);
    return parentId === partName || parentId === partId || parentId === partQn;
  });
  for (const port of partPorts) {
    const pr = port as Record<string, unknown>;
    const name = asString(pr.name);
    if (!name) continue;
    const portAttrs = pr.attributes;
    let portType: string | null = null;
    let direction: string | null = null;
    if (portAttrs && typeof portAttrs === "object" && "get" in portAttrs) {
      const bag = portAttrs as { get: (k: string) => unknown };
      portType = asString(bag.get("portType")) || null;
      direction = asString(bag.get("direction")) || null;
    } else if (portAttrs && typeof portAttrs === "object") {
      const bag = portAttrs as Record<string, unknown>;
      portType = asString(bag.portType) || null;
      direction = asString(bag.direction) || null;
    }
    const normalizedPortType = portType ? normalizeUnitBrackets(portType) : null;
    const directionPrefix = direction ? `${direction} ` : "";
    result.ports.push({
      name,
      typeName: normalizedPortType,
      displayText: (directionPrefix + name + (normalizedPortType ? ` : ${normalizedPortType}` : "")).trim(),
    });
  }

  for (const child of asArray(p.children)) {
    if (!child || typeof child !== "object") continue;
    const c = child as Record<string, unknown>;
    const childName = asString(c.name);
    const childType = asString(c.type);
    if (!childName || !childType) continue;
    if (childType === "part") {
      result.parts.push({ name: childName, displayText: childName });
    } else if (childType === "port") {
      const childAttrs = c.attributes;
      let portType: string | null = null;
      let direction: string | null = null;
      if (childAttrs && typeof childAttrs === "object" && "get" in childAttrs) {
        const bag = childAttrs as { get: (k: string) => unknown };
        portType = asString(bag.get("portType")) || null;
        direction = asString(bag.get("direction")) || null;
      } else if (childAttrs && typeof childAttrs === "object") {
        const bag = childAttrs as Record<string, unknown>;
        portType = asString(bag.portType) || null;
        direction = asString(bag.direction) || null;
      }
      const normalizedPortType = portType ? normalizeUnitBrackets(portType) : null;
      const directionPrefix = direction ? `${direction} ` : "";
      result.ports.push({
        name: childName,
        typeName: normalizedPortType,
        displayText: (directionPrefix + childName + (normalizedPortType ? ` : ${normalizedPortType}` : "")).trim(),
      });
    }
  }

  return result;
}

function fallbackDetailItems(attributes: Record<string, unknown>, key: string): SysMLNodeDetailItem[] {
  return asArray(attributes[key])
    .map((item) => normalizeDetailItem(item))
    .filter((item): item is SysMLNodeDetailItem => Boolean(item));
}

export function collectCompartments(node: {
  label: string;
  kind: string;
  attributes?: Record<string, unknown>;
}): SysMLNodeCompartments {
  const attributes = node.attributes ?? {};
  const typedByName =
    asString(attributes.typedByName) ||
    asString(attributes.partType) ||
    asString(attributes.type) ||
    asString(attributes.typedBy) ||
    asString(attributes.typing) ||
    null;
  const directAttributes = detailItems(attributes, "generalViewDirectAttributes");
  const directParts = detailItems(attributes, "generalViewDirectParts");
  const directPorts = detailItems(attributes, "generalViewDirectPorts");
  const inheritedAttributes = detailItems(attributes, "generalViewInheritedAttributes");
  const inheritedParts = detailItems(attributes, "generalViewInheritedParts");
  const packageMembers = [
    ...detailItems(attributes, "generalViewPackageMembers"),
    ...detailItems(attributes, "packageMembers"),
    ...detailItems(attributes, "members"),
  ];
  const imports = [
    ...detailItems(attributes, "generalViewImports"),
    ...detailItems(attributes, "imports"),
  ];
  const collapsibleSections: SysMLNodeSection[] = [];
  const typedCompartments = asArray(attributes.typedCompartments);
  const titleFor = (kind: string) => kind ? kind[0].toUpperCase() + kind.slice(1) : "Members";
  for (const raw of typedCompartments) {
    if (!raw || typeof raw !== "object") continue;
    const compartment = raw as Record<string, unknown>;
    const kind = asString(compartment.kind, "members");
    const inherited = asString(compartment.provenance) === "inherited";
    const items = asArray(compartment.members).map((member) => {
      const record = member && typeof member === "object" ? member as Record<string, unknown> : {};
      const name = asString(record.name, "Unnamed");
      const typeName = asString(record.typeName);
      return normalizeDetailItem({ name, typeName, displayText: typeName ? `${name} : ${typeName}` : name });
    }).filter((item): item is SysMLNodeDetailItem => Boolean(item));
    if (items.length > 0) {
      collapsibleSections.push({
        key: `${inherited ? "inherited" : "direct"}-${kind}`,
        title: titleFor(kind),
        items,
        collapsed: inherited,
      });
    }
  }
  if (inheritedAttributes.length > 0) {
    collapsibleSections.push({
      key: "inherited-attributes",
      title: "Attributes",
      items: inheritedAttributes,
      collapsed: true,
    });
  }
  if (inheritedParts.length > 0) {
    collapsibleSections.push({
      key: "inherited-parts",
      title: "Parts",
      items: inheritedParts,
      collapsed: true,
    });
  }
  if (packageMembers.length > 0) {
    collapsibleSections.push({
      key: "package-members",
      title: "Members",
      items: packageMembers,
      collapsed: false,
    });
  }
  if (imports.length > 0) {
    collapsibleSections.push({
      key: "imports",
      title: "Imports",
      items: imports,
      collapsed: true,
    });
  }
  // Renderer-owned presentation state: a compartment the viewer has opened or closed overrides the
  // default provenance-derived collapse. It never changes which members exist.
  const sectionState = attributes.compartmentSectionState;
  const resolvedSections =
    sectionState && typeof sectionState === "object"
      ? collapsibleSections.map((section) => {
          const state = (sectionState as Record<string, unknown>)[section.key];
          return typeof state === "boolean" ? { ...section, collapsed: !state } : section;
        })
      : collapsibleSections;

  return {
    header: { stereotype: node.kind || "element", name: node.label || "Unnamed" },
    typedByName,
    attributes: directAttributes.length > 0 ? directAttributes : fallbackDetailItems(attributes, "attributes"),
    parts: directParts.length > 0 ? directParts : fallbackDetailItems(attributes, "parts"),
    ports: directPorts.length > 0 ? directPorts : fallbackDetailItems(attributes, "ports"),
    collapsibleSections: resolvedSections,
  };
}


// ---------------------------------------------------------------------------------------------
// Node chrome layout
//
// One layout pass owns every coordinate inside a node: header regions, compartment blocks, and
// the total box height. `computeNodeHeight` (used by ELK sizing) and `renderSysMLNode` (used by
// drawing) both read it, so a node can never be measured at one size and painted at another.
// ---------------------------------------------------------------------------------------------

export interface NodeRegion {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface NodeChromeState {
  /** Present when the node owns nested elements that the disclosure control shows or hides. */
  disclosure?: "expanded" | "collapsed" | null;
  /** Relationships hidden because the node is collapsed; rendered as a header badge. */
  hiddenRelationshipCount?: number;
}

export interface NodeHeaderLayout {
  height: number;
  centerX: number;
  textLeft: number;
  textRight: number;
  stereotypeBaseline: number;
  nameLines: string[];
  nameBaselines: number[];
  typingText: string | null;
  typingBaseline: number | null;
  disclosureTarget: NodeRegion | null;
  badge: (NodeRegion & { text: string }) | null;
}

export interface CompartmentBlockLayout {
  key: string;
  title: string;
  collapsible: boolean;
  collapsed: boolean;
  totalItems: number;
  shownItems: SysMLNodeDetailItem[];
  overflowCount: number;
  /** y of the divider that opens this compartment. */
  dividerY: number;
  labelBaseline: number;
  labelTextX: number;
  itemBaselines: number[];
  overflowBaseline: number | null;
  /** Full-width pointer target for a collapsible compartment's label row. */
  labelRegion: NodeRegion;
  disclosureBox: NodeRegion | null;
  height: number;
}

export interface SysMLNodeLayout {
  header: NodeHeaderLayout;
  blocks: CompartmentBlockLayout[];
  /** Header fill is suppressed when the node has no body compartments to separate it from. */
  showHeaderFill: boolean;
  height: number;
}

export interface SysMLNodeLayoutInput {
  width: number;
  /** Only affects fills and dividers, never the measured height. */
  strokeWidthPx: number;
  config?: SysMLNodeConfig;
  state?: NodeChromeState;
}

function maxChars(availableWidth: number, fontSize: number, ratio: number): number {
  return Math.max(1, Math.floor(availableWidth / (fontSize * ratio)));
}

/** Deterministic ellipsis truncation; callers keep the untruncated text in a `<title>`. */
export function truncateToChars(value: string, limit: number): string {
  const text = String(value ?? "");
  if (text.length <= limit) return text;
  if (limit <= 1) return "…";
  return `${text.slice(0, limit - 1)}…`;
}

/**
 * Split a long element name over at most `maxLines` lines, breaking at separators and camel-case
 * boundaries before falling back to a hard break. Purely lexical: it never interprets the name.
 */
export function wrapElementName(name: string, limit: number, maxLines = NAME_MAX_LINES): string[] {
  const text = String(name ?? "").trim();
  if (!text) return [""];
  if (text.length <= limit || maxLines <= 1) return [truncateToChars(text, limit)];

  const segments: string[] = [];
  let current = "";
  for (let index = 0; index < text.length; index += 1) {
    const char = text[index];
    const next = text[index + 1];
    current += char;
    const separator = char === " " || char === "_" || char === "." || char === ":" || char === "-";
    const camelBoundary = next !== undefined && /[a-z0-9]/.test(char) && /[A-Z]/.test(next);
    if (separator || camelBoundary) {
      segments.push(current);
      current = "";
    }
  }
  if (current) segments.push(current);

  const lines: string[] = [];
  let index = 0;
  while (index < segments.length && lines.length < maxLines) {
    let line = segments[index];
    index += 1;
    while (index < segments.length && line.length + segments[index].length <= limit) {
      line += segments[index];
      index += 1;
    }
    lines.push(line.trimEnd());
  }
  const overflowed = index < segments.length;
  const rendered = lines.map((line) => truncateToChars(line, limit));
  if (overflowed) {
    rendered[rendered.length - 1] = truncateToChars(`${lines[lines.length - 1]}…`, limit);
  }
  return rendered;
}

function badgeWidthFor(text: string): number {
  return Math.max(BADGE_MIN_WIDTH, 10 + text.length * 6);
}

export function layoutNodeHeader(
  compartments: SysMLNodeCompartments,
  input: SysMLNodeLayoutInput,
): NodeHeaderLayout {
  const state = input.state ?? {};
  const hiddenCount = state.hiddenRelationshipCount ?? 0;
  const badgeText = hiddenCount > 0 ? String(hiddenCount) : null;
  const hasDisclosure = state.disclosure === "expanded" || state.disclosure === "collapsed";

  // Gutters are measured from the box edge, not from the stroke, so selecting a node (which
  // thickens its border) can never re-wrap the name and change the node's measured height.
  const controlReserve = hasDisclosure ? CONTROL_EDGE_INSET + DISCLOSURE_TARGET_SIZE + 2 : 0;
  const badgeReserve = badgeText ? CONTROL_EDGE_INSET + badgeWidthFor(badgeText) + 2 : 0;
  // Both sides reserve the same width so the stereotype and name stay centred on the node axis
  // while never sharing space with a control or a badge.
  const gutter = Math.max(controlReserve, badgeReserve, HEADER_PADDING_X);
  const textLeft = gutter;
  const textRight = Math.max(textLeft + 1, input.width - gutter);
  const availableText = textRight - textLeft;

  const nameLines = wrapElementName(
    compartments.header.name || "Unnamed",
    maxChars(availableText, NAME_FONT_SIZE, AVERAGE_GLYPH_RATIO_BOLD),
  );
  const typingRaw = compartments.typedByName ? `: ${compartments.typedByName}` : null;
  const typingText = typingRaw
    ? truncateToChars(typingRaw, maxChars(availableText, TYPING_FONT_SIZE, AVERAGE_GLYPH_RATIO_REGULAR))
    : null;

  const contentHeight =
    HEADER_PADDING_TOP +
    STEREOTYPE_LINE_HEIGHT +
    nameLines.length * NAME_LINE_HEIGHT +
    (typingText ? TYPING_LINE_HEIGHT : 0) +
    HEADER_PADDING_BOTTOM;
  const height = Math.max(contentHeight, DISCLOSURE_TARGET_SIZE + 4);

  const stereotypeBaseline = HEADER_PADDING_TOP + STEREOTYPE_LINE_HEIGHT - 3;
  const nameBaselines = nameLines.map(
    (_line, index) => HEADER_PADDING_TOP + STEREOTYPE_LINE_HEIGHT + (index + 1) * NAME_LINE_HEIGHT - 4,
  );
  const typingBaseline = typingText
    ? HEADER_PADDING_TOP + STEREOTYPE_LINE_HEIGHT + nameLines.length * NAME_LINE_HEIGHT + TYPING_LINE_HEIGHT - 3
    : null;

  const disclosureTarget: NodeRegion | null = hasDisclosure
    ? {
        x: CONTROL_EDGE_INSET,
        y: (height - DISCLOSURE_TARGET_SIZE) / 2,
        width: DISCLOSURE_TARGET_SIZE,
        height: DISCLOSURE_TARGET_SIZE,
      }
    : null;
  const badge = badgeText
    ? {
        x: input.width - CONTROL_EDGE_INSET - badgeWidthFor(badgeText),
        y: (height - BADGE_HEIGHT) / 2,
        width: badgeWidthFor(badgeText),
        height: BADGE_HEIGHT,
        text: badgeText,
      }
    : null;

  return {
    height,
    centerX: input.width / 2,
    textLeft,
    textRight,
    stereotypeBaseline,
    nameLines,
    nameBaselines,
    typingText,
    typingBaseline,
    disclosureTarget,
    badge,
  };
}

function compartmentSections(
  compartments: SysMLNodeCompartments,
  cfg: Required<SysMLNodeConfig>,
): SysMLNodeSection[] {
  const sections: SysMLNodeSection[] = [];
  if (cfg.showAttributes && compartments.attributes.length > 0) {
    sections.push({ key: "attributes", title: "Attributes", items: compartments.attributes });
  }
  if (cfg.showParts && compartments.parts.length > 0) {
    sections.push({ key: "parts", title: "Parts", items: compartments.parts });
  }
  if (cfg.showPorts && compartments.ports.length > 0) {
    sections.push({ key: "ports", title: "Ports", items: compartments.ports });
  }
  for (const section of compartments.collapsibleSections ?? []) {
    if (section.items.length > 0) sections.push(section);
  }
  if (cfg.showOther) {
    for (const section of compartments.other ?? []) {
      if (section.lines.length === 0) continue;
      sections.push({
        key: `other:${section.title}`,
        title: section.title,
        items: section.lines.map((line) => lineToDetailItem(line)),
      });
    }
  }
  return sections;
}

export function layoutSysMLNode(
  compartments: SysMLNodeCompartments,
  input: SysMLNodeLayoutInput,
): SysMLNodeLayout {
  const cfg = { ...DEFAULT_CONFIG, ...(input.config ?? {}) };
  const inset = input.strokeWidthPx / 2;
  const header = cfg.showHeader
    ? layoutNodeHeader(compartments, input)
    : {
        height: 0,
        centerX: input.width / 2,
        textLeft: inset,
        textRight: input.width - inset,
        stereotypeBaseline: 0,
        nameLines: [],
        nameBaselines: [],
        typingText: null,
        typingBaseline: null,
        disclosureTarget: null,
        badge: null,
      };

  const sections = compartmentSections(compartments, cfg);
  const blocks: CompartmentBlockLayout[] = [];
  let cursor = header.height;
  for (const section of sections) {
    const collapsible = Boolean(section.collapsed !== undefined);
    const collapsed = Boolean(section.collapsed);
    const limit =
      section.showAll || !cfg.maxLinesPerCompartment
        ? section.items.length
        : Math.min(section.items.length, cfg.maxLinesPerCompartment);
    const shownItems = collapsed ? [] : section.items.slice(0, limit);
    const overflowCount = collapsed ? 0 : section.items.length - shownItems.length;
    const labelTop = cursor + COMPARTMENT_PADDING;
    const labelBaseline = labelTop + COMPARTMENT_LABEL_HEIGHT - 5;
    const disclosureBox: NodeRegion | null = collapsible
      ? {
          x: PADDING,
          y: labelTop + (COMPARTMENT_LABEL_HEIGHT - DISCLOSURE_BOX_SIZE) / 2 - 1,
          width: DISCLOSURE_BOX_SIZE,
          height: DISCLOSURE_BOX_SIZE,
        }
      : null;
    const labelTextX = collapsible ? PADDING + DISCLOSURE_BOX_SIZE + 5 : PADDING;
    const itemTop = labelTop + COMPARTMENT_LABEL_HEIGHT;
    const itemBaselines = shownItems.map((_item, index) => itemTop + (index + 1) * LINE_HEIGHT - 3);
    const overflowBaseline =
      overflowCount > 0 ? itemTop + shownItems.length * LINE_HEIGHT + OVERFLOW_LINE_HEIGHT - 3 : null;
    const height =
      COMPARTMENT_PADDING +
      COMPARTMENT_LABEL_HEIGHT +
      shownItems.length * LINE_HEIGHT +
      (overflowCount > 0 ? OVERFLOW_LINE_HEIGHT : 0) +
      COMPARTMENT_PADDING;
    blocks.push({
      key: section.key,
      title: section.title,
      collapsible,
      collapsed,
      totalItems: section.items.length,
      shownItems,
      overflowCount,
      dividerY: cursor,
      labelBaseline,
      labelTextX,
      itemBaselines,
      overflowBaseline,
      labelRegion: {
        // Inset past the body stroke so a focus ring on the row never sits on the node border.
        x: inset + CONTROL_EDGE_INSET,
        y: cursor + 1,
        width: Math.max(0, input.width - (inset + CONTROL_EDGE_INSET) * 2),
        height: COMPARTMENT_PADDING + COMPARTMENT_LABEL_HEIGHT,
      },
      disclosureBox,
      height,
    });
    cursor += height;
  }

  return {
    header,
    blocks,
    showHeaderFill: blocks.length > 0,
    height: Math.max(cursor, header.height, DISCLOSURE_TARGET_SIZE + 4),
  };
}

export const NODE_WIDTH_MIN = 200;
export const NODE_WIDTH_MAX = 320;

/**
 * Content-derived node width, clamped to a bounded range.
 *
 * A fixed width forced every long member name and every long element name through the same
 * truncation regardless of how much room the diagram had. Width is now derived from the same
 * estimated text metrics the header and compartment layout use, so layout and drawing agree, and
 * clamped so one verbose element cannot dominate a diagram.
 */
export function computeNodeWidth(
  compartments: SysMLNodeCompartments,
  config: SysMLNodeConfig = {},
  state?: NodeChromeState,
): number {
  const cfg = { ...DEFAULT_CONFIG, ...config };
  const hiddenCount = state?.hiddenRelationshipCount ?? 0;
  const badgeText = hiddenCount > 0 ? String(hiddenCount) : null;
  const hasDisclosure = state?.disclosure === "expanded" || state?.disclosure === "collapsed";
  const gutter = Math.max(
    hasDisclosure ? CONTROL_EDGE_INSET + DISCLOSURE_TARGET_SIZE + 2 : 0,
    badgeText ? CONTROL_EDGE_INSET + badgeWidthFor(badgeText) + 2 : 0,
    HEADER_PADDING_X,
  );

  const name = compartments.header.name || "Unnamed";
  const perNameLine = Math.ceil(name.length / NAME_MAX_LINES);
  const headerContent = Math.max(
    perNameLine * NAME_FONT_SIZE * AVERAGE_GLYPH_RATIO_BOLD,
    formatStereotype(compartments.header.stereotype).length * STEREOTYPE_FONT_SIZE * AVERAGE_GLYPH_RATIO_REGULAR,
    compartments.typedByName
      ? (compartments.typedByName.length + 2) * TYPING_FONT_SIZE * AVERAGE_GLYPH_RATIO_REGULAR
      : 0,
  );
  let widest = gutter * 2 + headerContent;

  for (const section of compartmentSections(compartments, cfg)) {
    const labelWidth =
      PADDING + DISCLOSURE_BOX_SIZE + 5 + section.title.length * COMPARTMENT_FONT_SIZE * AVERAGE_GLYPH_RATIO_BOLD + PADDING;
    widest = Math.max(widest, labelWidth);
    if (section.collapsed) continue;
    const limit =
      section.showAll || !cfg.maxLinesPerCompartment
        ? section.items.length
        : Math.min(section.items.length, cfg.maxLinesPerCompartment);
    for (const item of section.items.slice(0, limit)) {
      widest = Math.max(
        widest,
        PADDING * 2 + 4 + item.displayText.length * COMPARTMENT_FONT_SIZE * AVERAGE_GLYPH_RATIO_REGULAR,
      );
    }
  }

  const rounded = Math.ceil(widest / 2) * 2;
  return Math.min(NODE_WIDTH_MAX, Math.max(NODE_WIDTH_MIN, rounded));
}

export function computeNodeHeight(
  compartments: SysMLNodeCompartments,
  config: SysMLNodeConfig = {},
  input?: { width?: number; strokeWidthPx?: number; state?: NodeChromeState },
): number {
  return layoutSysMLNode(compartments, {
    width: input?.width ?? 200,
    strokeWidthPx: input?.strokeWidthPx ?? 2,
    config,
    state: input?.state,
  }).height;
}

export function computeNodeHeightFromCompartments(
  compartments: SysMLNodeCompartments,
  config: SysMLNodeConfig,
  nodeWidth?: number,
): number {
  return computeNodeHeight(compartments, config, { width: nodeWidth });
}

const NOTATION_KEYWORDS: Record<string, string> = {
  PartDefinition: "part def", PartUsage: "part", PortDefinition: "port def", PortUsage: "port",
  AttributeDefinition: "attribute def", AttributeUsage: "attribute", ItemDefinition: "item def", ItemUsage: "item",
  OccurrenceDefinition: "occurrence def", OccurrenceUsage: "occurrence", ConnectionDefinition: "connection def", ConnectionUsage: "connection",
  InterfaceDefinition: "interface def", InterfaceUsage: "interface", AllocationDefinition: "allocation def", AllocationUsage: "allocation",
  ActionDefinition: "action def", ActionUsage: "action", StateDefinition: "state def", StateUsage: "state",
  CalculationDefinition: "calc def", CalculationUsage: "calc", ConstraintDefinition: "constraint def", ConstraintUsage: "constraint",
  RequirementDefinition: "requirement def", RequirementUsage: "requirement", ConcernDefinition: "concern def", ConcernUsage: "concern",
  CaseDefinition: "case def", CaseUsage: "case", AnalysisCaseDefinition: "analysis def", AnalysisCaseUsage: "analysis",
  VerificationCaseDefinition: "verification def", VerificationCaseUsage: "verification", UseCaseDefinition: "use case def", UseCaseUsage: "use case",
  ViewDefinition: "view def", ViewUsage: "view", ViewpointDefinition: "viewpoint def", ViewpointUsage: "viewpoint",
  RenderingDefinition: "rendering def", RenderingUsage: "rendering", MetadataDefinition: "metadata def", MetadataUsage: "metadata",
  ReferenceUsage: "ref",
};

export function formatStereotype(type: string): string {
  return `«${NOTATION_KEYWORDS[type] ?? type.replace(/_/g, " ")}»`;
}

export interface NodeDisclosureBinding {
  expanded: boolean;
  /** Accessible name, e.g. `Collapse Vehicle`. */
  label: string;
  /** Explanatory tooltip shown on hover and exported into the SVG. */
  tooltip: string;
  onToggle: (event: Event) => void;
}

export interface CompartmentDisclosureBinding {
  label: (section: CompartmentBlockLayout) => string;
  onToggle: (sectionKey: string, event: Event, currentlyExpanded: boolean) => void;
}

export interface RenderSysMLNodeOptions {
  x: number;
  y: number;
  width: number;
  height: number;
  nodeClass: string;
  dataElementName: string;
  strokeColor: string;
  kind?: string;
  isDefinition?: boolean;
  isReference?: boolean;
  chrome?: NodeChrome;
  selected: boolean;
  config?: SysMLNodeConfig;
  theme?: DiagramTheme;
  /** Renderer-owned presentation state; never derived from the model inside this module. */
  state?: NodeChromeState;
  disclosure?: NodeDisclosureBinding | null;
  compartmentDisclosure?: CompartmentDisclosureBinding | null;
  hiddenRelationshipTooltip?: string;
}

/** `+` / `-` glyph inside a disclosure box, drawn as filled bars so it stays legible at any zoom. */
function disclosureGlyphPaths(box: NodeRegion, expanded: boolean): string[] {
  const thickness = Math.max(1.4, box.width * 0.14);
  const arm = box.width * 0.54;
  const cx = box.x + box.width / 2;
  const cy = box.y + box.height / 2;
  const horizontal = `M${cx - arm / 2},${cy - thickness / 2}h${arm}v${thickness}h${-arm}Z`;
  if (expanded) return [horizontal];
  const vertical = `M${cx - thickness / 2},${cy - arm / 2}v${arm}h${thickness}v${-arm}Z`;
  return [horizontal, vertical];
}

function appendTitle(selection: D3Selection, text: string): void {
  selection.append("title").text(text);
}

export function renderSysMLNode(
  parent: D3Selection,
  compartments: SysMLNodeCompartments,
  options: RenderSysMLNodeOptions,
): D3Selection {
  const theme = options.theme;
  const nodeFill = theme?.nodeFill ?? "var(--vscode-editor-background)";
  const panelBackground = theme?.panelBackground ?? "var(--vscode-button-secondaryBackground)";
  const textPrimary = theme?.textPrimary ?? "var(--vscode-editor-foreground)";
  const textSecondary = theme?.textSecondary ?? "var(--vscode-descriptionForeground)";
  const divider = theme?.divider ?? "var(--vscode-panel-border)";
  const highlight = theme?.highlight ?? "#FFD700";
  const controlStroke = theme?.controlStroke ?? textSecondary;
  const controlFill = theme?.controlFill ?? nodeFill;
  const controlForeground = theme?.controlForeground ?? textPrimary;
  const badgeFill = theme?.badgeFill ?? panelBackground;
  const badgeText = theme?.badgeText ?? textPrimary;

  const chrome =
    options.chrome ??
    resolveNodeChrome(options.isReference ? "reference-usage" : options.isDefinition ? "definition" : "unsupported");
  const body = nodeBodyChromeStyle(chrome, { selected: options.selected, generalView: true });
  const layout = layoutSysMLNode(compartments, {
    width: options.width,
    strokeWidthPx: body.strokeWidthPx,
    config: options.config,
    state: options.state,
  });
  const span = nodeInnerSpan(options.width, body.strokeWidthPx);

  const node = parent
    .append("g")
    .attr(
      "class",
      `${options.nodeClass}${chrome.nodeClassSuffix}${options.selected ? " is-selected" : ""}`,
    )
    .attr("transform", `translate(${options.x},${options.y})`)
    .attr("data-element-name", options.dataElementName);

  node
    .append("rect")
    .attr("width", options.width)
    .attr("height", options.height)
    .attr("rx", body.cornerRadius)
    .attr("class", "graph-node-background sysml-node-bg")
    .attr("data-original-stroke", options.strokeColor)
    .attr("data-original-width", `${body.strokeWidthPx}px`)
    .style("fill", nodeFill)
    .style("stroke", options.selected ? highlight : options.strokeColor)
    .style("stroke-width", `${body.strokeWidthPx}px`)
    .style("stroke-dasharray", body.strokeDasharray);

  const header = layout.header;
  if (layout.showHeaderFill) {
    // Path, not a rect: it follows the body's own rounded corners inset by half the stroke, so the
    // outer border stays visually continuous and no header corner cuts across it.
    node
      .append("path")
      .attr("class", "sysml-header-compartment")
      .attr("d", headerFillPath(options.width, header.height, body.cornerRadius, body.strokeWidthPx))
      .style("fill", panelBackground);
  }

  node
    .append("text")
    .attr("class", "sysml-node-stereotype")
    .attr("x", header.centerX)
    .attr("y", header.stereotypeBaseline)
    .attr("text-anchor", "middle")
    .text(formatStereotype(compartments.header.stereotype))
    .style("font-size", `${STEREOTYPE_FONT_SIZE}px`)
    .style("fill", textSecondary);

  const nameGroup = node.append("g").attr("class", "node-name-text viz-node-name");
  header.nameLines.forEach((line, index) => {
    nameGroup
      .append("text")
      .attr("x", header.centerX)
      .attr("y", header.nameBaselines[index])
      .attr("text-anchor", "middle")
      .text(line)
      .style("font-size", `${NAME_FONT_SIZE}px`)
      .style("font-weight", "600")
      .style("fill", textPrimary);
  });
  appendTitle(nameGroup, compartments.header.name);

  if (header.typingText && header.typingBaseline !== null) {
    const typing = node
      .append("text")
      .attr("class", "sysml-node-typing")
      .attr("x", header.centerX)
      .attr("y", header.typingBaseline)
      .attr("text-anchor", "middle")
      .text(header.typingText)
      .style("font-size", `${TYPING_FONT_SIZE}px`)
      .style("font-style", "italic")
      .style("fill", textSecondary);
    appendTitle(typing, `: ${compartments.typedByName ?? ""}`);
  }

  if (header.disclosureTarget && options.disclosure) {
    const target = header.disclosureTarget;
    const binding = options.disclosure;
    const box: NodeRegion = {
      x: target.x + (target.width - DISCLOSURE_BOX_SIZE) / 2,
      y: target.y + (target.height - DISCLOSURE_BOX_SIZE) / 2,
      width: DISCLOSURE_BOX_SIZE,
      height: DISCLOSURE_BOX_SIZE,
    };
    const control = node
      .append("g")
      .attr("class", "general-node-toggle sysml-disclosure")
      .attr("role", "button")
      .attr("tabindex", 0)
      .attr("aria-label", binding.label)
      .attr("aria-expanded", binding.expanded ? "true" : "false")
      .attr("data-disclosure-state", binding.expanded ? "expanded" : "collapsed");
    appendTitle(control, binding.tooltip);
    control
      .append("rect")
      .attr("class", "sysml-disclosure-target")
      .attr("x", target.x)
      .attr("y", target.y)
      .attr("width", target.width)
      .attr("height", target.height)
      .attr("rx", 4)
      .style("fill", "transparent")
      .style("pointer-events", "all");
    control
      .append("rect")
      .attr("class", "sysml-disclosure-box")
      .attr("x", box.x)
      .attr("y", box.y)
      .attr("width", box.width)
      .attr("height", box.height)
      .attr("rx", 2)
      .style("fill", controlFill)
      .style("stroke", controlStroke)
      .style("stroke-width", "1px");
    for (const d of disclosureGlyphPaths(box, binding.expanded)) {
      control
        .append("path")
        .attr("class", "sysml-disclosure-glyph")
        .attr("d", d)
        .style("fill", controlForeground);
    }
    control.on("click", (event: Event) => {
      event.stopPropagation?.();
      binding.onToggle(event);
    });
    control.on("keydown", (event: KeyboardEvent) => {
      if (event.key !== "Enter" && event.key !== " " && event.key !== "Spacebar") return;
      event.preventDefault?.();
      event.stopPropagation?.();
      binding.onToggle(event);
    });
  }

  if (header.badge) {
    const badge = header.badge;
    const group = node
      .append("g")
      .attr("class", "general-hidden-relationships sysml-badge")
      .attr("role", "img")
      .attr("aria-label", options.hiddenRelationshipTooltip ?? `${badge.text} hidden relationships`);
    appendTitle(group, options.hiddenRelationshipTooltip ?? `${badge.text} hidden relationships`);
    group
      .append("rect")
      .attr("x", badge.x)
      .attr("y", badge.y)
      .attr("width", badge.width)
      .attr("height", badge.height)
      .attr("rx", badge.height / 2)
      .style("fill", badgeFill)
      .style("stroke", divider)
      .style("stroke-width", "1px");
    group
      .append("text")
      .attr("x", badge.x + badge.width / 2)
      .attr("y", badge.y + badge.height - 4.5)
      .attr("text-anchor", "middle")
      .text(badge.text)
      .style("font-size", "9px")
      .style("font-weight", "600")
      .style("fill", badgeText);
  }

  const itemChars = maxChars(
    Math.max(1, options.width - PADDING * 2 - 4),
    COMPARTMENT_FONT_SIZE,
    AVERAGE_GLYPH_RATIO_REGULAR,
  );
  for (const block of layout.blocks) {
    node
      .append("line")
      .attr("x1", span.x1)
      .attr("y1", block.dividerY)
      .attr("x2", span.x2)
      .attr("y2", block.dividerY)
      .attr("class", "sysml-compartment-divider")
      .style("stroke", divider)
      .style("stroke-width", "1px");

    const labelGroup = block.collapsible
      ? node
          .append("g")
          .attr("class", "sysml-compartment-label sysml-disclosure sysml-compartment-toggle")
          .attr("role", "button")
          .attr("tabindex", 0)
          .attr("aria-expanded", block.collapsed ? "false" : "true")
          .attr("data-compartment-key", block.key)
          .attr(
            "aria-label",
            options.compartmentDisclosure?.label(block)
              ?? `${block.collapsed ? "Show" : "Hide"} ${block.title}`,
          )
      : node.append("g").attr("class", "sysml-compartment-label").attr("data-compartment-key", block.key);
    appendTitle(
      labelGroup,
      block.collapsible
        ? `${block.title} — ${block.totalItems} member${block.totalItems === 1 ? "" : "s"}`
        : `${block.title} — ${block.totalItems} member${block.totalItems === 1 ? "" : "s"}`,
    );
    if (block.collapsible) {
      labelGroup
        .append("rect")
        .attr("class", "sysml-disclosure-target")
        .attr("x", block.labelRegion.x)
        .attr("y", block.labelRegion.y)
        .attr("width", block.labelRegion.width)
        .attr("height", block.labelRegion.height)
        .attr("rx", 3)
        .style("fill", "transparent")
        .style("pointer-events", "all");
    }
    if (block.disclosureBox) {
      labelGroup
        .append("rect")
        .attr("class", "sysml-disclosure-box")
        .attr("x", block.disclosureBox.x)
        .attr("y", block.disclosureBox.y)
        .attr("width", block.disclosureBox.width)
        .attr("height", block.disclosureBox.height)
        .attr("rx", 2)
        .style("fill", controlFill)
        .style("stroke", controlStroke)
        .style("stroke-width", "1px");
      for (const d of disclosureGlyphPaths(block.disclosureBox, !block.collapsed)) {
        labelGroup
          .append("path")
          .attr("class", "sysml-disclosure-glyph")
          .attr("d", d)
          .style("fill", controlForeground);
      }
    }
    labelGroup
      .append("text")
      .attr("x", block.labelTextX)
      .attr("y", block.labelBaseline)
      .text(block.title)
      .style("font-size", `${COMPARTMENT_FONT_SIZE}px`)
      .style("font-weight", "600")
      .style("letter-spacing", "0.02em")
      .style("fill", textSecondary)
      .style("pointer-events", "none");
    if (block.collapsible && options.compartmentDisclosure) {
      const binding = options.compartmentDisclosure;
      labelGroup.on("click", (event: Event) => {
        event.stopPropagation?.();
        binding.onToggle(block.key, event, !block.collapsed);
      });
      labelGroup.on("keydown", (event: KeyboardEvent) => {
        if (event.key !== "Enter" && event.key !== " " && event.key !== "Spacebar") return;
        event.preventDefault?.();
        event.stopPropagation?.();
        binding.onToggle(block.key, event, !block.collapsed);
      });
    }

    block.shownItems.forEach((item, index) => {
      const text = node
        .append("text")
        .attr("class", "sysml-compartment-item")
        .attr("x", PADDING)
        .attr("y", block.itemBaselines[index])
        .text(truncateToChars(item.displayText, itemChars))
        .style("font-size", `${COMPARTMENT_FONT_SIZE}px`)
        .style("fill", textSecondary);
      appendTitle(text, item.declaredIn ? `${item.displayText} (from ${item.declaredIn})` : item.displayText);
    });
    if (block.overflowCount > 0 && block.overflowBaseline !== null) {
      const overflow = node
        .append("text")
        .attr("class", "sysml-compartment-overflow")
        .attr("x", PADDING)
        .attr("y", block.overflowBaseline)
        .text(`+${block.overflowCount} more`)
        .style("font-size", `${COMPARTMENT_FONT_SIZE}px`)
        .style("font-style", "italic")
        .style("fill", textSecondary);
      appendTitle(overflow, `${block.overflowCount} further ${block.title.toLowerCase()} not shown`);
    }
  }

  return node;
}

/**
 * Decode the renderer-owned presentation state a projection carries on a node. Both ELK sizing
 * and drawing read it through this one function so the measured and painted header agree on how
 * much room the disclosure control and the hidden-relationship badge reserve.
 */
export function nodeChromeStateFromAttributes(
  attributes: Record<string, unknown> | undefined,
): NodeChromeState {
  const attrs = attributes ?? {};
  const disclosure = attrs.disclosure;
  const hidden = attrs.hiddenRelationshipCount;
  return {
    disclosure: disclosure === "expanded" || disclosure === "collapsed" ? disclosure : null,
    hiddenRelationshipCount: typeof hidden === "number" && hidden > 0 ? hidden : 0,
  };
}
