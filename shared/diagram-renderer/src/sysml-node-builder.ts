import type { NodeChrome } from "./node-notation";
import { nodeBodyChromeStyle, resolveNodeChrome } from "./node-notation";
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
}

export interface SysMLNodeConfig {
  showAttributes?: boolean;
  showParts?: boolean;
  showPorts?: boolean;
  maxLinesPerCompartment?: number;
}

export const LINE_HEIGHT = 12;
const COMPARTMENT_LABEL_HEIGHT = 14;
const COMPARTMENT_GAP = 2;
const COMPARTMENT_PADDING = 4;
const HEADER_COMPARTMENT_HEIGHT = 44;
const TYPED_BY_HEIGHT = 14;
const PADDING = 6;
const SHOW_MORE_LINE_HEIGHT = 12;

const DEFAULT_CONFIG: Required<SysMLNodeConfig> = {
  showAttributes: true,
  showParts: true,
  showPorts: true,
  maxLinesPerCompartment: 8,
};

type D3Selection = {
  append: (name: string) => D3Selection;
  attr: (name: string, value: unknown) => D3Selection;
  style: (name: string, value: unknown) => D3Selection;
  text: (value: unknown) => D3Selection;
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
  if (inheritedAttributes.length > 0) {
    collapsibleSections.push({
      key: "inherited-attributes",
      title: "Inherited Attributes",
      items: inheritedAttributes,
      collapsed: true,
    });
  }
  if (inheritedParts.length > 0) {
    collapsibleSections.push({
      key: "inherited-parts",
      title: "Inherited Parts",
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
  return {
    header: { stereotype: node.kind.toLowerCase() || "element", name: node.label || "Unnamed" },
    typedByName,
    attributes: directAttributes.length > 0 ? directAttributes : fallbackDetailItems(attributes, "attributes"),
    parts: directParts.length > 0 ? directParts : fallbackDetailItems(attributes, "parts"),
    ports: directPorts.length > 0 ? directPorts : fallbackDetailItems(attributes, "ports"),
    collapsibleSections,
  };
}

export function computeNodeHeight(compartments: SysMLNodeCompartments, config: SysMLNodeConfig): number {
  const cfg = { ...DEFAULT_CONFIG, ...config };
  let height = PADDING * 2 + HEADER_COMPARTMENT_HEIGHT;
  if (compartments.typedByName) height += TYPED_BY_HEIGHT;

  const addCompartment = (items: SysMLNodeDetailItem[]) => {
    if (items.length === 0) return;
    const shown = cfg.maxLinesPerCompartment ? Math.min(items.length, cfg.maxLinesPerCompartment) : items.length;
    height += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + shown * LINE_HEIGHT + COMPARTMENT_GAP;
    if (cfg.maxLinesPerCompartment && items.length > cfg.maxLinesPerCompartment) {
      height += SHOW_MORE_LINE_HEIGHT;
    }
  };

  if (cfg.showAttributes) addCompartment(compartments.attributes);
  if (cfg.showParts) addCompartment(compartments.parts);
  if (cfg.showPorts) addCompartment(compartments.ports);
  for (const section of compartments.collapsibleSections ?? []) {
    if (section.items.length > 0) {
      height += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + COMPARTMENT_GAP;
      if (!section.collapsed) height += section.items.length * LINE_HEIGHT;
    }
  }
  return Math.max(70, height);
}

function truncate(value: string, max: number): string {
  return value.length > max ? `${value.slice(0, max - 2)}..` : value;
}

function formatStereotype(type: string): string {
  return `\u00ab${type.replace(/_/g, " ")}\u00bb`;
}

export function renderSysMLNode(
  parent: D3Selection,
  compartments: SysMLNodeCompartments,
  options: {
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
  },
): D3Selection {
  const cfg = { ...DEFAULT_CONFIG, ...(options.config ?? {}) };
  const theme = options.theme;
  const nodeFill = theme?.nodeFill ?? "var(--vscode-editor-background)";
  const panelBackground = theme?.panelBackground ?? "var(--vscode-button-secondaryBackground)";
  const textPrimary = theme?.textPrimary ?? "var(--vscode-editor-foreground)";
  const textSecondary = theme?.textSecondary ?? "var(--vscode-descriptionForeground)";
  const divider = theme?.divider ?? "var(--vscode-panel-border)";
  const highlight = theme?.highlight ?? "#FFD700";
  const chrome =
    options.chrome ??
    resolveNodeChrome(options.kind ?? "", {
      isDefinition: options.isDefinition,
      isReference: options.isReference,
    });
  const node = parent
    .append("g")
    .attr(
      "class",
      `${options.nodeClass}${chrome.nodeClassSuffix}${options.selected ? " is-selected" : ""}`,
    )
    .attr("transform", `translate(${options.x},${options.y})`)
    .attr("data-element-name", options.dataElementName);
  const body = nodeBodyChromeStyle(chrome, { selected: options.selected, generalView: true });

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

  const headerHeight = HEADER_COMPARTMENT_HEIGHT + (compartments.typedByName ? TYPED_BY_HEIGHT : 0);
  const headerRx = body.headerCornerRadius;
  node
    .append("rect")
    .attr("y", 0)
    .attr("width", options.width)
    .attr("height", headerHeight)
    .attr("rx", headerRx)
    .attr("class", "sysml-header-compartment")
    .style("fill", panelBackground);

  node
    .append("text")
    .attr("x", options.width / 2)
    .attr("y", 17)
    .attr("text-anchor", "middle")
    .text(formatStereotype(compartments.header.stereotype))
    .style("font-size", "9px")
    .style("fill", options.strokeColor);

  node
    .append("text")
    .attr("class", "node-name-text viz-node-name")
    .attr("x", options.width / 2)
    .attr("y", 31)
    .attr("text-anchor", "middle")
    .text(truncate(compartments.header.name, 26))
    .style("font-size", "11px")
    .style("font-weight", "bold")
    .style("fill", textPrimary);

  if (compartments.typedByName) {
    node
      .append("text")
      .attr("x", options.width / 2)
      .attr("y", 43)
      .attr("text-anchor", "middle")
      .text(`: ${truncate(compartments.typedByName, 22)}`)
      .style("font-size", "10px")
      .style("font-style", "italic")
      .style("fill", options.strokeColor);
  }

  let contentY = headerHeight + COMPARTMENT_PADDING;
  const renderCompartment = (title: string, items: SysMLNodeDetailItem[], collapsed = false) => {
    if (items.length === 0) return;
    const limit = cfg.maxLinesPerCompartment ? Math.min(items.length, cfg.maxLinesPerCompartment) : items.length;
    const shownItems = collapsed ? [] : items.slice(0, limit);
    node
      .append("line")
      .attr("x1", PADDING)
      .attr("y1", contentY)
      .attr("x2", options.width - PADDING)
      .attr("y2", contentY)
      .attr("class", "sysml-compartment-divider")
      .style("stroke", divider)
      .style("stroke-width", "1px");
    contentY += 4;
    node
      .append("text")
      .attr("x", PADDING)
      .attr("y", contentY + 9)
      .text(collapsed ? `> ${title}` : title)
      .style("font-size", "9px")
      .style("font-weight", "bold")
      .style("fill", textSecondary);
    contentY += COMPARTMENT_LABEL_HEIGHT;
    for (const item of shownItems) {
      node
        .append("text")
        .attr("x", PADDING)
        .attr("y", contentY + 9)
        .text(truncate(item.displayText, 32))
        .style("font-size", "9px")
        .style("fill", textSecondary)
        .append("title")
        .text(item.declaredIn ? `${item.displayText} (from ${item.declaredIn})` : item.displayText);
      contentY += LINE_HEIGHT;
    }
    if (!collapsed && cfg.maxLinesPerCompartment && items.length > cfg.maxLinesPerCompartment) {
      node
        .append("text")
        .attr("x", PADDING)
        .attr("y", contentY + 9)
        .text(`+${items.length - cfg.maxLinesPerCompartment} more`)
        .style("font-size", "9px")
        .style("font-weight", "bold")
        .style("fill", options.strokeColor);
      contentY += SHOW_MORE_LINE_HEIGHT;
    }
    contentY += COMPARTMENT_PADDING + COMPARTMENT_GAP;
  };

  if (cfg.showAttributes) renderCompartment("Attributes", compartments.attributes);
  if (cfg.showParts) renderCompartment("Parts", compartments.parts);
  if (cfg.showPorts) renderCompartment("Ports", compartments.ports);
  for (const section of compartments.collapsibleSections ?? []) {
    renderCompartment(section.title, section.items, Boolean(section.collapsed));
  }
  return node;
}
