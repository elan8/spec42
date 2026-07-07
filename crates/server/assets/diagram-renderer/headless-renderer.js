"use strict";
var Spec42HeadlessRendererBundle = (() => {
  var __defProp = Object.defineProperty;
  var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __hasOwnProp = Object.prototype.hasOwnProperty;
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };
  var __copyProps = (to, from, except, desc) => {
    if (from && typeof from === "object" || typeof from === "function") {
      for (let key of __getOwnPropNames(from))
        if (!__hasOwnProp.call(to, key) && key !== except)
          __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
    }
    return to;
  };
  var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

  // shared/diagram-renderer/src/headless-export.ts
  var headless_export_exports = {};
  __export(headless_export_exports, {
    exportHeadlessSvg: () => exportHeadlessSvg
  });

  // shared/diagram-renderer/src/node-notation.ts
  function isDefinitionKind(kind) {
    const normalized = kind.trim().toLowerCase();
    return normalized.includes(" def") || normalized.includes("_def") || normalized.endsWith(" def") || normalized.includes("definition");
  }
  function isReferenceKind(kind) {
    const k = kind.trim().toLowerCase();
    if (k === "ref") return true;
    if (k.endsWith("-ref")) return true;
    if (k.endsWith(" ref")) return true;
    if (/\bref\b/.test(k) && !k.includes("refine")) return true;
    return false;
  }
  function nodeCategory(kind) {
    const k = kind.toLowerCase();
    if (k.includes("requirement") || k.includes("concern") || k.includes("viewpoint") || k.includes("stakeholder")) {
      return "requirement";
    }
    if (k.includes("action") || k.includes("state") || k.includes("calc") || k.includes("analysis") || k.includes("enumeration")) {
      return "behavior";
    }
    if (k.includes("part") || k.includes("port") || k.includes("item") || k.includes("attribute") || k.includes("interface") || k.includes("occurrence")) {
      return "structural";
    }
    return "other";
  }
  function usageCornerRadius(kind) {
    const cat = nodeCategory(kind);
    if (cat === "requirement") return 16;
    if (cat === "behavior") return 12;
    return 8;
  }
  function nodeBodyStrokeDasharray(chrome, isPackageContainer = false) {
    if (chrome.isContainer && isPackageContainer) return "none";
    return chrome.strokeDasharray ?? "none";
  }
  function nodeBodyChromeStyle(chrome, opts) {
    const selected = opts?.selected ?? false;
    const isContainer = opts?.isContainer ?? chrome.isContainer;
    let strokeWidthPx = 2;
    if (selected) strokeWidthPx = 4;
    else if (isContainer) strokeWidthPx = 2;
    else if (opts?.generalView) strokeWidthPx = chrome.isDefinition ? 3 : 2;
    else strokeWidthPx = chrome.isDefinition ? 2 : 3;
    return {
      cornerRadius: chrome.cornerRadius,
      strokeDasharray: nodeBodyStrokeDasharray(chrome, opts?.isPackageContainer),
      strokeWidthPx,
      headerCornerRadius: chrome.isDefinition ? 0 : Math.max(2, chrome.cornerRadius - 2)
    };
  }
  function resolveNodeChrome(kind, opts) {
    const normalized = kind.toLowerCase();
    const isContainer = opts?.isContainer ?? (normalized.includes("container") || normalized.includes("part_usage"));
    if (isContainer) {
      const isPackageContainer = opts?.isPackageContainer ?? false;
      return {
        isDefinition: false,
        isReference: false,
        isContainer: true,
        cornerRadius: 8,
        strokeDasharray: isPackageContainer ? null : "4,4",
        structureClass: "viz-node--container",
        nodeClassSuffix: ""
      };
    }
    const isReference = opts?.isReference ?? isReferenceKind(kind);
    const isDefinition = !isReference && (opts?.isDefinition ?? isDefinitionKind(kind));
    if (isReference) {
      return {
        isDefinition: false,
        isReference: true,
        isContainer: false,
        cornerRadius: usageCornerRadius(kind),
        strokeDasharray: "2,4",
        structureClass: "viz-node--reference",
        nodeClassSuffix: " reference-node"
      };
    }
    if (isDefinition) {
      return {
        isDefinition: true,
        isReference: false,
        isContainer: false,
        cornerRadius: 0,
        strokeDasharray: null,
        structureClass: "viz-node--definition",
        nodeClassSuffix: " definition-node"
      };
    }
    return {
      isDefinition: false,
      isReference: false,
      isContainer: false,
      cornerRadius: usageCornerRadius(kind),
      strokeDasharray: null,
      structureClass: "viz-node--usage",
      nodeClassSuffix: " usage-node"
    };
  }

  // shared/diagram-renderer/src/graph-normalization.ts
  function normalizeEdgeKind(relationshipType) {
    const type2 = relationshipType.trim().toLowerCase();
    if (!type2) return "relationship";
    if (type2.includes("item_flow") || type2.includes("item flow") || type2 === "flow" || type2.includes("flow")) return "flow";
    if (type2.includes("interface-connection") || type2.includes("interface connection")) return "interface";
    if (type2.includes("interface")) return "interface";
    if (type2.includes("binding-connection") || type2.includes("binding connection")) return "bind";
    if (type2.includes("connection") || type2 === "connect") return "connection";
    if (type2.includes("reference") || type2 === "ref") return "reference";
    if (type2.includes("satisfy")) return "satisfy";
    if (type2.includes("verify")) return "verify";
    if (type2.includes("derivation") || type2.includes("derive")) return "derivation";
    if (type2 === "typing" || type2 === "defined_by" || type2 === "defined by" || type2 === "definition") return "typing";
    if (type2 === "dependency" || type2.includes("depend") || type2.includes("binary-dependency")) return "dependency";
    if (type2 === "usage" || type2 === "usage-relationship") return "usage";
    if (type2.includes("redefin")) return "redefinition";
    if (type2 === "specializes" || type2 === "specialization") return "specializes";
    if (type2 === "bind" || type2 === "binding") return "bind";
    if (type2 === "allocate" || type2 === "allocation") return "allocate";
    if (type2 === "transition") return "transition";
    if (type2 === "composition") return "composition";
    if (type2 === "hierarchy" || type2 === "contains" || type2 === "owns" || type2 === "ownership" || type2 === "containment") return "hierarchy";
    return type2.replace(/[^a-z0-9_-]+/g, "_") || "relationship";
  }
  function isPackageElementType(elementType) {
    const normalized = elementType.trim().toLowerCase();
    return !normalized || normalized === "package" || normalized === "library package" || normalized.endsWith("_package") || normalized.includes("package_def");
  }
  function isNonDiagramSemanticElementType(elementType) {
    const normalized = elementType.trim().toLowerCase();
    if (!normalized) return true;
    return normalized === "import" || normalized === "diagnostic" || normalized.includes("diagnostic");
  }
  function isOverviewVisualElementType(elementType) {
    return !isPackageElementType(elementType) && !isNonDiagramSemanticElementType(elementType);
  }

  // shared/diagram-renderer/src/prepare/util.ts
  function asRecord(value) {
    return value && typeof value === "object" ? value : {};
  }
  function asArray(value) {
    return Array.isArray(value) ? value : [];
  }
  function asString(value, fallback = "") {
    if (typeof value === "string") return value;
    if (typeof value === "number" || typeof value === "boolean") return String(value);
    return fallback;
  }
  function elementTypeOf(node) {
    const attrs = asRecord(node.attributes);
    return asString(
      node.type ?? node.element_type ?? node.element_kind ?? attrs.element_type ?? attrs.element_kind ?? attrs.elementKind
    );
  }
  function isPackage(node) {
    return isPackageElementType(elementTypeOf(node));
  }
  function nodeUri(node) {
    return asString(node.uri ?? node.sourcePath ?? node.source_path) || null;
  }
  function nodeRange(node) {
    return node.range ?? null;
  }
  function buildBehaviorNode(node, index, defaults) {
    const attrs = asRecord(node.attributes);
    const qualifiedName = asString(node.qualifiedName ?? attrs.qualifiedName ?? node.id);
    return {
      id: asString(node.id ?? node.name, defaults.id),
      label: asString(node.name ?? node.label ?? node.id, defaults.label),
      kind: defaults.kind,
      sourcePath: nodeUri(node),
      uri: nodeUri(node),
      range: nodeRange(node),
      attributes: {
        ...attrs,
        ...qualifiedName ? { qualifiedName } : {},
        ...node.parentId != null ? { parentId: node.parentId } : {},
        ...node.parent != null ? { parent: node.parent } : {}
      }
    };
  }
  function isSyntheticPackage(node) {
    if (!isPackage(node)) return false;
    const attrs = asRecord(node.attributes);
    return Boolean(node.synthetic ?? node.isSynthetic ?? attrs.synthetic ?? attrs.isSyntheticContainer);
  }

  // shared/diagram-renderer/src/prepare/diagram-select.ts
  function normalizeDiagramKey(value) {
    return value.replace(/::/g, ".").trim().toLowerCase();
  }
  function diagramSimpleName(value) {
    const normalized = value.replace(/::/g, ".");
    const segments = normalized.split(".").filter(Boolean);
    return segments[segments.length - 1] ?? normalized;
  }
  function diagramMatchesSelection(diagram, selectedName, selectedViewId) {
    const selectors = [selectedName, selectedViewId].filter((value) => Boolean(value?.trim()));
    if (selectors.length === 0) return false;
    const diagramKeys = [
      asString(diagram.id),
      asString(diagram.name),
      `${asString(diagram.package_path)}::${asString(diagram.name)}`.replace(/^::+/, "")
    ].filter(Boolean);
    return selectors.some((selector) => {
      const selectorKey = normalizeDiagramKey(selector);
      const selectorSimple = diagramSimpleName(selector).toLowerCase();
      return diagramKeys.some((candidate) => {
        const candidateKey = normalizeDiagramKey(candidate);
        const candidateSimple = diagramSimpleName(candidate).toLowerCase();
        return candidateKey === selectorKey || candidateSimple === selectorSimple || candidateKey.endsWith(`.${selectorKey}`) || selectorKey.endsWith(`.${candidateKey}`) || candidateKey.includes(selectorSimple) || selectorKey.includes(candidateSimple);
      });
    });
  }
  function selectNamedDiagram(diagramsInput, selectedName, selectedViewId) {
    const diagrams = asArray(diagramsInput).map(asRecord);
    if (diagrams.length === 0) return null;
    if (!selectedName && !selectedViewId) return null;
    const matched = diagrams.find((diagram) => diagramMatchesSelection(diagram, selectedName, selectedViewId));
    if (matched) return matched;
    return diagrams.length === 1 ? diagrams[0] : null;
  }
  function bestBehaviorDiagram(diagrams) {
    if (diagrams.length === 0) return null;
    const score = (diagram) => {
      const nodes = asArray(diagram.nodes ?? diagram.actions ?? diagram.steps);
      const edges = asArray(diagram.edges ?? diagram.flows ?? diagram.transitions);
      return nodes.length * 10 + edges.length;
    };
    return diagrams.slice().sort((a, b) => score(b) - score(a))[0] ?? null;
  }
  function diagramToPrepared(diagramInput, view, fallbackTitle) {
    const diagram = asRecord(diagramInput);
    let nodes = asArray(diagram.nodes ?? diagram.states).map((nodeRaw, index) => {
      const node = asRecord(nodeRaw);
      return buildBehaviorNode(node, index, {
        id: `node-${index}`,
        label: `Node ${index + 1}`,
        kind: asString(node.type ?? node.kind, view)
      });
    });
    let edges = asArray(diagram.edges ?? diagram.transitions).map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source ?? edge.from ?? edge.sourceId, ""),
        target: asString(edge.target ?? edge.to ?? edge.targetId, ""),
        label: asString(edge.name ?? edge.label ?? edge.type, "")
      };
    });
    if (view === "sequence-view" && nodes.length === 0) {
      nodes = asArray(diagram.lifelines).map((lifelineRaw, index) => {
        const lifeline = asRecord(lifelineRaw);
        return buildBehaviorNode(lifeline, index, {
          id: `lifeline-${index}`,
          label: `Lifeline ${index + 1}`,
          kind: "lifeline"
        });
      });
      edges = asArray(diagram.messages).map((messageRaw, index) => {
        const message = asRecord(messageRaw);
        return {
          id: asString(message.id, `message-${index}`),
          source: asString(message.source ?? message.from ?? message.sourceId, ""),
          target: asString(message.target ?? message.to ?? message.targetId, ""),
          label: asString(message.name ?? message.label ?? message.type, "")
        };
      });
    }
    const ids = new Set(nodes.map((node) => node.id));
    edges = edges.filter((edge) => ids.has(edge.source) && ids.has(edge.target));
    return { title: asString(diagram.name, fallbackTitle), view, nodes, edges };
  }

  // shared/diagram-renderer/src/prepare/behavior/common.ts
  function buildActivityNodeAliasMap(nodes) {
    const aliases = /* @__PURE__ */ new Map();
    const register = (alias, nodeId) => {
      const key = asString(alias).trim();
      if (!key) return;
      if (!aliases.has(key)) aliases.set(key, nodeId);
      const normalized = key.replace(/::/g, ".");
      if (!aliases.has(normalized)) aliases.set(normalized, nodeId);
      const lastSegment = normalized.split(".").filter(Boolean).pop();
      if (lastSegment && !aliases.has(lastSegment)) aliases.set(lastSegment, nodeId);
    };
    for (const node of nodes) {
      register(node.id, node.id);
      register(node.label, node.id);
      register(asRecord(node.attributes).qualifiedName, node.id);
    }
    return aliases;
  }
  function resolveActivityNodeRef(value, aliases) {
    const key = asString(value).trim();
    if (!key) return "";
    const normalized = key.replace(/::/g, ".");
    const segments = normalized.split(".").filter(Boolean);
    const last = segments[segments.length - 1] || "";
    const first = segments[0] || "";
    return aliases.get(key) ?? aliases.get(normalized) ?? (last ? aliases.get(last) : void 0) ?? (first ? aliases.get(first) : void 0) ?? key;
  }

  // shared/diagram-renderer/src/prepare/behavior/action-flow.ts
  function activityDiagramCatalog(visualization) {
    const normalized = asArray(visualization.diagrams).map(asRecord);
    if (normalized.length > 0) {
      return normalized;
    }
    return asArray(visualization.activityDiagrams).map(asRecord);
  }
  function collectActivityNodes(diagram) {
    const allowedKinds = /* @__PURE__ */ new Set([
      "action",
      "perform",
      "assign",
      "for-loop",
      "decision",
      "merge",
      "fork",
      "join",
      "initial",
      "final",
      "terminate",
      "accept",
      "send"
    ]);
    const decisions = asArray(diagram.decisions).map((nodeRaw, index) => {
      const node = asRecord(nodeRaw);
      return buildBehaviorNode(node, index, {
        id: `decision-${index}`,
        label: "Decision",
        kind: "decision"
      });
    });
    const states = asArray(diagram.states).map((nodeRaw, index) => {
      const node = asRecord(nodeRaw);
      const kind = asString(node.type ?? node.stateType ?? node.kind, "state").toLowerCase();
      return buildBehaviorNode(node, index, {
        id: `state-${index}`,
        label: `State ${index + 1}`,
        kind
      });
    }).filter(
      (node) => ["initial", "final", "decision", "merge", "fork", "join", "assign", "for-loop", "terminate", "accept", "send"].some(
        (token) => node.kind.includes(token)
      )
    );
    const actions = asArray(diagram.nodes ?? diagram.actions ?? diagram.steps).map((nodeRaw, index) => {
      const node = asRecord(nodeRaw);
      const kind = asString(node.kind ?? node.type ?? node.action_type, "action").toLowerCase();
      const normalizedKind = kind.includes("perform") ? "perform" : kind.includes("decision") ? "decision" : kind.includes("merge") ? "merge" : kind.includes("fork") ? "fork" : kind.includes("join") ? "join" : kind.includes("assign") ? "assign" : kind.includes("for-loop") || kind.includes("forloop") ? "for-loop" : kind.includes("terminate") ? "terminate" : kind.includes("accept") ? "accept" : kind.includes("send") ? "send" : kind.includes("initial") ? "initial" : kind.includes("final") ? "final" : "action";
      return buildBehaviorNode(node, index, {
        id: `action-${index}`,
        label: `Action ${index + 1}`,
        kind: normalizedKind
      });
    });
    const enriched = [...actions, ...decisions, ...states].map((node) => {
      const attrs = asRecord(node.attributes);
      const swimLane = asString(attrs.swimLane ?? attrs.swim_lane, "");
      if (!swimLane) {
        return node;
      }
      return {
        ...node,
        attributes: {
          ...node.attributes ?? {},
          swimLane
        }
      };
    });
    return enriched.filter((node) => allowedKinds.has(node.kind));
  }
  function prepareActivity(visualization) {
    const catalog = activityDiagramCatalog(visualization);
    const selected = selectNamedDiagram(catalog, visualization?.selectedViewName, visualization?.selectedView);
    const effective = selected ?? bestBehaviorDiagram(catalog);
    const diagram = asRecord(effective);
    const nodes = collectActivityNodes(diagram);
    const nodeIds = new Set(nodes.map((node) => node.id));
    const aliases = buildActivityNodeAliasMap(nodes);
    const edges = asArray(diagram.flows ?? diagram.edges ?? diagram.transitions).map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      const source = resolveActivityNodeRef(edge.from ?? edge.source ?? edge.sourceId, aliases);
      const target = resolveActivityNodeRef(edge.to ?? edge.target ?? edge.targetId, aliases);
      const guard = asString(edge.guard ?? edge.type, "");
      const condition = asString(edge.condition, "");
      const guardLower = guard.toLowerCase();
      const succession = guardLower === "flow" || guardLower === "first" || guardLower === "succession";
      const conditional = condition.length > 0 || guard.length > 0 && !["flow", "first", "bind", "perform", "succession"].includes(guardLower);
      return {
        id: asString(edge.id, `flow-${index}`),
        source,
        target,
        label: asString(edge.name ?? edge.label ?? condition ?? guard, ""),
        attributes: {
          ...guard ? { guard } : {},
          ...condition ? { condition } : {},
          succession,
          conditional
        }
      };
    }).filter(
      (edge) => edge.source && edge.target && edge.source !== edge.target && nodeIds.has(edge.source) && nodeIds.has(edge.target)
    );
    const swimLanes = Array.from(
      new Set(
        nodes.map((node) => asString(asRecord(node.attributes).swimLane, "")).filter(Boolean)
      )
    );
    return {
      title: asString(diagram.name ?? visualization?.selectedViewName, "Action Flow View"),
      view: "action-flow-view",
      nodes,
      edges,
      meta: {
        selectedDiagramId: asString(diagram.id),
        nodeCount: nodes.length,
        edgeCount: edges.length,
        layoutDirection: asString(visualization?.activityLayoutDirection, "vertical"),
        activityDiagram: effective,
        parentContext: asString(diagram.name),
        swimLanes
      }
    };
  }

  // shared/diagram-renderer/src/prepare/behavior/state.ts
  function stateMachineCatalog(visualization) {
    const normalized = asArray(visualization.stateMachines).map(asRecord);
    if (normalized.length > 0) {
      return normalized;
    }
    return asArray(visualization.stateDiagrams).map(asRecord);
  }
  function attachCompositeRegions(nodes) {
    const childrenByRegion = /* @__PURE__ */ new Map();
    for (const node of nodes) {
      const regionId = asString(asRecord(node.attributes).regionId, "");
      if (!regionId) continue;
      const bucket = childrenByRegion.get(regionId) ?? [];
      bucket.push(node);
      childrenByRegion.set(regionId, bucket);
    }
    return nodes.map((node) => {
      if (!node.kind.includes("composite")) {
        return node;
      }
      const children2 = childrenByRegion.get(node.id) ?? [];
      if (children2.length === 0) {
        return node;
      }
      return {
        ...node,
        attributes: {
          ...node.attributes ?? {},
          regions: children2.map((child) => ({ name: child.label, id: child.id }))
        }
      };
    });
  }
  function attachExplicitRegions(nodes, machine) {
    const regions = asArray(machine.regions).map(asRecord);
    if (regions.length === 0) {
      return attachCompositeRegions(nodes);
    }
    const regionsByParent = /* @__PURE__ */ new Map();
    for (const region of regions) {
      const parentId = asString(region.parentId ?? region.parent_id, "");
      if (!parentId) continue;
      const bucket = regionsByParent.get(parentId) ?? [];
      bucket.push(region);
      regionsByParent.set(parentId, bucket);
    }
    return nodes.map((node) => {
      if (!node.kind.includes("composite")) {
        return node;
      }
      const explicit = regionsByParent.get(node.id) ?? [];
      if (explicit.length === 0) {
        return node;
      }
      return {
        ...node,
        attributes: {
          ...node.attributes ?? {},
          regions: explicit.map((region, index) => ({
            id: asString(region.id, `region-${index}`),
            name: asString(region.name, `region ${index + 1}`)
          }))
        }
      };
    });
  }
  function formatStateTransitionLabel(edge) {
    const parts = [];
    const guard = asString(edge.guard, "").trim();
    const effect = asString(edge.effect, "").trim();
    const accept = asString(edge.accept, "").trim();
    const send = asString(edge.send, "").trim();
    const label = asString(edge.label, "").trim();
    if (guard) parts.push(`[${guard}]`);
    if (effect) parts.push(effect);
    if (accept) parts.push(`accept ${accept}`);
    if (send) parts.push(`send ${send}`);
    if (parts.length > 0) {
      return parts.join(" / ");
    }
    return label;
  }
  function collectStateMachineNodes(machine) {
    const nodes = asArray(machine.states).map((stateRaw, index) => {
      const state = asRecord(stateRaw);
      const element = asRecord(state.element);
      const merged = {
        ...element,
        ...state,
        id: state.id ?? element.id,
        name: state.name ?? element.name,
        range: element.range ?? state.range,
        uri: element.uri ?? state.uri ?? element.sourcePath ?? state.sourcePath,
        qualifiedName: state.qualifiedName ?? element.qualifiedName ?? state.id,
        entry: state.entry ?? element.entry,
        do: state.do ?? element.do,
        exit: state.exit ?? element.exit,
        parentId: state.parentId ?? state.parent_id ?? element.parentId,
        regionId: state.regionId ?? state.region_id ?? element.regionId
      };
      const kind = asString(state.kind ?? state.type ?? element.type, "state").toLowerCase();
      const behaviorNode = buildBehaviorNode(asRecord(merged), index, {
        id: `state-${index}`,
        label: "State",
        kind: kind.includes("initial") ? "initial" : kind.includes("terminate") ? "terminate" : kind.includes("final") ? "final" : kind.includes("composite") ? "composite" : "state"
      });
      const entry = asString(merged.entry, "");
      const doAction = asString(merged.do, "");
      const exit = asString(merged.exit, "");
      const regionId = asString(merged.regionId, "");
      if (entry || doAction || exit || regionId) {
        behaviorNode.attributes = {
          ...behaviorNode.attributes ?? {},
          ...entry ? { entry } : {},
          ...doAction ? { do: doAction } : {},
          ...exit ? { exit } : {},
          ...regionId ? { regionId } : {}
        };
      }
      return behaviorNode;
    });
    return attachExplicitRegions(nodes, machine);
  }
  function prepareStateMachine(machine, visualization) {
    const nodes = collectStateMachineNodes(machine);
    const nodeIds = new Set(nodes.map((node) => node.id));
    const aliases = buildActivityNodeAliasMap(nodes);
    const edges = asArray(machine.transitions).map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      const source = resolveActivityNodeRef(edge.source ?? edge.sourceName ?? edge.from, aliases);
      const target = resolveActivityNodeRef(edge.target ?? edge.targetName ?? edge.to, aliases);
      const label = formatStateTransitionLabel(edge);
      return {
        id: asString(edge.id, `transition-${index}`),
        source,
        target,
        label,
        attributes: {
          selfLoop: Boolean(edge.selfLoop ?? source === target),
          guard: edge.guard,
          effect: edge.effect,
          accept: edge.accept,
          send: edge.send
        }
      };
    }).filter(
      (edge) => edge.source && edge.target && nodeIds.has(edge.source) && nodeIds.has(edge.target)
    );
    return {
      title: asString(machine.name ?? visualization?.selectedViewName, "State Transition View"),
      view: "state-transition-view",
      nodes,
      edges,
      meta: {
        selectedDiagramId: asString(machine.id),
        selectedDiagramName: asString(machine.name),
        layoutDirection: asString(visualization?.stateLayoutDirection, "horizontal"),
        stateMachine: machine,
        parentContext: asString(machine.name)
      }
    };
  }
  function prepareState(visualization) {
    const catalog = stateMachineCatalog(visualization);
    if (catalog.length > 0) {
      const selected = selectNamedDiagram(catalog, visualization?.selectedViewName, visualization?.selectedView);
      const effective = selected ?? catalog[0];
      if (effective) {
        return prepareStateMachine(asRecord(effective), visualization);
      }
    }
    const selectedStateDiagram = selectNamedDiagram(
      visualization.stateDiagrams,
      visualization?.selectedViewName,
      visualization?.selectedView
    );
    if (selectedStateDiagram) {
      const diagram = asRecord(selectedStateDiagram);
      const prepared = diagramToPrepared(diagram, "state-transition-view", "State Transition View");
      return {
        ...prepared,
        meta: {
          selectedDiagramId: asString(diagram.id),
          selectedDiagramName: asString(diagram.name),
          layoutDirection: asString(visualization?.stateLayoutDirection, "horizontal"),
          stateDiagram: diagram
        }
      };
    }
    const graph = asRecord(visualization?.graph);
    const stateNodes = asArray(graph.nodes).map(asRecord).filter((node) => asString(node.type ?? node.element_type).toLowerCase().includes("state"));
    const ids = new Set(stateNodes.map((node) => asString(node.id)));
    const nodes = stateNodes.map((node) => ({
      id: asString(node.id),
      label: asString(node.name ?? node.id, "State"),
      kind: asString(node.type ?? node.element_type, "state"),
      sourcePath: asString(node.sourcePath) || null,
      range: node.range ?? null,
      attributes: asRecord(node.attributes)
    }));
    const edges = asArray(graph.edges).map((edgeRaw, index) => {
      const edge = asRecord(edgeRaw);
      return {
        id: `transition-${index}`,
        source: asString(edge.source),
        target: asString(edge.target),
        label: asString(edge.name ?? edge.type ?? edge.rel_type, "")
      };
    }).filter((edge) => ids.has(edge.source) && ids.has(edge.target));
    const synthesizeInitial = visualization?.synthesizeInitialState === true;
    const hasInitial = nodes.some((node) => node.kind.toLowerCase().includes("initial") || node.label.toLowerCase() === "initial");
    const withSyntheticInitial = synthesizeInitial && !hasInitial && nodes.length > 0 ? [{ id: "__synthetic_initial__", label: "Initial", kind: "initial", attributes: { synthetic: true } }, ...nodes] : nodes;
    const idsWithInitial = new Set(withSyntheticInitial.map((node) => node.id));
    const edgesWithInitial = !hasInitial && withSyntheticInitial.length > 1 ? [
      {
        id: "transition-synthetic-initial",
        source: "__synthetic_initial__",
        target: withSyntheticInitial[1].id,
        label: "initial"
      },
      ...edges
    ] : edges;
    return {
      title: asString(visualization?.selectedViewName, "State Transition View"),
      view: "state-transition-view",
      nodes: withSyntheticInitial.filter((node) => idsWithInitial.has(node.id)),
      edges: edgesWithInitial.filter((edge) => idsWithInitial.has(edge.source) && idsWithInitial.has(edge.target)),
      meta: {
        syntheticInitial: synthesizeInitial && !hasInitial && nodes.length > 0
      }
    };
  }

  // shared/diagram-renderer/src/prepare/graph.ts
  function isGeneralViewDiagramNode(node) {
    if (isSyntheticPackage(node)) {
      return false;
    }
    return isOverviewVisualElementType(elementTypeOf(node));
  }
  function buildGeneralPackageContainerGroups(nodes) {
    const byPackage = /* @__PURE__ */ new Map();
    for (const node of nodes) {
      const qn = asString(asRecord(node.attributes).qualifiedName);
      const sep = qn.indexOf("::");
      if (sep <= 0) continue;
      const pkg = qn.slice(0, sep);
      const members = byPackage.get(pkg) ?? [];
      members.push(node.id);
      byPackage.set(pkg, members);
    }
    if (byPackage.size < 2) return [];
    return [...byPackage.entries()].map(([name, memberIds]) => ({
      id: `package:${name}`,
      name,
      memberIds
    }));
  }
  function prepareGraph(graphInput, visualization) {
    const graph = asRecord(graphInput);
    const rawNodes = asArray(graph.nodes).map(asRecord);
    const sourceNodes = rawNodes.filter((node) => isGeneralViewDiagramNode(node));
    const nodeIds = new Set(sourceNodes.map((node) => asString(node.id)));
    const nodes = sourceNodes.map((node) => ({
      id: asString(node.id),
      label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
      kind: elementTypeOf(node) || "Element",
      sourcePath: asString(node.sourcePath ?? node.source_path) || null,
      uri: nodeUri(node),
      range: node.range ?? null,
      attributes: {
        ...asRecord(node.attributes),
        qualifiedName: asString(node.qualifiedName ?? asRecord(node.attributes).qualifiedName),
        isPackage: isPackage(node),
        isDefinition: isDefinitionKind(asString(node.type ?? node.element_type, "")),
        isReference: isReferenceKind(asString(node.type ?? node.element_type, ""))
      }
    }));
    const edges = asArray(graph.edges).map(asRecord).filter((edge) => nodeIds.has(asString(edge.source)) && nodeIds.has(asString(edge.target))).map((edge, index) => {
      const relationType = asString(edge.type ?? edge.rel_type ?? edge.relationType ?? edge.name, "");
      const label = asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, "");
      return {
        id: asString(edge.id, `edge-${index}`),
        source: asString(edge.source),
        target: asString(edge.target),
        label,
        edgeKind: normalizeEdgeKind(relationType),
        attributes: {
          ...asRecord(edge.attributes),
          relationType: normalizeEdgeKind(relationType)
        }
      };
    });
    const packageContainerGroups = buildGeneralPackageContainerGroups(nodes);
    return {
      title: visualization?.selectedViewName || "SysML View",
      view: visualization?.view || "general-view",
      nodes,
      edges,
      meta: packageContainerGroups.length > 0 ? { packageContainerGroups } : void 0
    };
  }

  // shared/diagram-renderer/src/prepare/behavior/sequence.ts
  function prepareSequence(visualization) {
    const selected = selectNamedDiagram(
      visualization?.sequenceDiagrams,
      visualization?.selectedViewName,
      visualization?.selectedView
    );
    const fallbackDiagram = asArray(visualization?.sequenceDiagrams).map(asRecord)[0] ?? null;
    const effective = selected ?? fallbackDiagram;
    if (effective) {
      const prepared = diagramToPrepared(effective, "sequence-view", "Sequence View");
      return {
        ...prepared,
        meta: {
          selectedDiagramName: asString(asRecord(effective).name),
          sequenceDiagram: effective,
          parentContext: asString(asRecord(effective).name)
        }
      };
    }
    return prepareGraph(visualization?.graph, visualization);
  }

  // shared/diagram-renderer/src/prepare/normalize-payload.ts
  function asArray2(value) {
    return Array.isArray(value) ? value : [];
  }
  function countItems(value) {
    return asArray2(value).length;
  }
  function aliasDiagramsField(data, sourceKey) {
    const source = asArray2(data[sourceKey]);
    return {
      ...data,
      diagrams: source
    };
  }
  function normalizeVisualizationPayload(data) {
    if (!data) {
      return data;
    }
    const view = String(data.view || "general-view");
    switch (view) {
      case "general-view":
        return data;
      case "interconnection-view": {
        if (data.interconnectionScene && typeof data.interconnectionScene === "object") {
          return data;
        }
        return {
          ...data,
          elements: [],
          parts: [],
          ports: [],
          connectors: [],
          containerGroups: [],
          packageContainerGroups: []
        };
      }
      case "action-flow-view": {
        const activityDiagrams = asArray2(data.activityDiagrams);
        const diagrams = activityDiagrams.map((diagram) => ({
          ...diagram,
          nodes: diagram.nodes ?? diagram.actions,
          hasBehavioralFlow: countItems(diagram.flows) > 0,
          hasRenderableContent: countItems(diagram.flows) > 0 && countItems(diagram.nodes ?? diagram.actions) > 0
        }));
        return {
          ...data,
          diagrams
        };
      }
      case "state-transition-view": {
        const stateMachines = asArray2(data.stateMachines);
        return {
          ...data,
          stateMachines,
          states: stateMachines.flatMap((machine) => asArray2(machine.states)),
          transitions: stateMachines.flatMap((machine) => asArray2(machine.transitions))
        };
      }
      case "sequence-view":
        return aliasDiagramsField(data, "sequenceDiagrams");
      default:
        return data;
    }
  }

  // shared/diagram-renderer/src/prepare/interconnection-scene.ts
  function portsForNode(ownerNodeId, ports) {
    return ports.filter((port) => port.ownerNodeId === ownerNodeId);
  }
  function mapPortDetail(port) {
    return {
      id: port.id,
      name: port.name,
      direction: port.direction,
      portType: port.typeName,
      portSide: port.sideHint === "west" ? "left" : port.sideHint === "east" ? "right" : void 0,
      uri: port.uri,
      range: port.range,
      attributes: {
        parentId: port.ownerNodeId,
        scenePortId: port.id,
        sideHint: port.sideHint
      }
    };
  }
  function prepareInterconnectionScene(scene, visualization) {
    const nodeIds = new Set(scene.nodes.map((node) => node.id));
    const nodes = scene.nodes.map((node) => {
      const nodePorts = portsForNode(node.id, scene.ports);
      const portDetails = nodePorts.map(mapPortDetail);
      return {
        id: node.id,
        label: node.name,
        kind: node.kind === "ref" ? "part" : "part",
        uri: node.uri,
        range: node.range,
        attributes: {
          containerId: node.parentId ?? null,
          qualifiedName: node.qualifiedName,
          semanticId: node.semanticId,
          definitionId: node.definitionId,
          partType: node.typeName,
          ports: portDetails.map((port) => port.name),
          portDetails,
          isDefinition: isDefinitionKind(node.kind),
          isReference: isReferenceKind(node.kind) || node.kind === "ref",
          sceneNodeId: node.id
        }
      };
    });
    for (const container of scene.containers) {
      if (nodeIds.has(container.id)) continue;
      nodes.push({
        id: container.id,
        label: container.label,
        kind: "package",
        attributes: {
          isSyntheticContainer: true,
          containerId: container.parentId ?? null,
          qualifiedName: container.label,
          memberNodeIds: container.memberNodeIds,
          layoutDepth: container.depth
        }
      });
      nodeIds.add(container.id);
    }
    const edges = scene.edges.filter((edge) => nodeIds.has(edge.sourceNodeId) && nodeIds.has(edge.targetNodeId)).map((edge) => ({
      id: edge.id,
      source: edge.sourceNodeId,
      target: edge.targetNodeId,
      label: edge.label ?? edge.kind,
      edgeKind: normalizeEdgeKind(edge.kind),
      attributes: {
        sourceId: edge.sourcePortId,
        targetId: edge.targetPortId,
        sourcePortId: edge.sourcePortId,
        targetPortId: edge.targetPortId,
        sourceNodeId: edge.sourceNodeId,
        targetNodeId: edge.targetNodeId,
        semanticId: edge.semanticId,
        sourceExpression: edge.sourceExpression,
        targetExpression: edge.targetExpression,
        relationType: edge.kind,
        canonicalScene: true
      }
    }));
    return {
      title: scene.view.name || asString(visualization.selectedViewName) || "Interconnection View",
      view: "interconnection-view",
      nodes,
      edges,
      meta: {
        canonicalScene: true,
        schemaVersion: scene.schemaVersion,
        selectedRoot: scene.view.rootIds[0] ?? null,
        rootCandidates: scene.view.rootIds,
        diagnostics: scene.diagnostics
      }
    };
  }

  // shared/diagram-renderer/src/prepare/interconnection.ts
  function prepareInterconnection(visualization) {
    const scene = visualization.interconnectionScene;
    if (scene && scene.schemaVersion >= 2) {
      return prepareInterconnectionScene(scene, visualization);
    }
    return {
      title: String(visualization.selectedViewName || "Interconnection View"),
      view: "interconnection-view",
      nodes: [],
      edges: [],
      meta: {
        diagnostics: [
          {
            severity: "error",
            code: "missing_interconnection_scene",
            message: "Interconnection view requires interconnectionScene from the language server."
          }
        ]
      }
    };
  }

  // shared/diagram-renderer/src/prepare/standard-views.ts
  function graphNodesForStandardView(visualization) {
    const graph = asRecord(visualization?.generalViewGraph ?? visualization?.graph);
    return asArray(graph.nodes).map(asRecord);
  }
  function graphEdgesForStandardView(visualization) {
    const graph = asRecord(visualization?.generalViewGraph ?? visualization?.graph);
    return asArray(graph.edges).map(asRecord);
  }
  function qualifiedNameOf(node) {
    const attrs = asRecord(node.attributes);
    return asString(node.id ?? node.qualifiedName ?? attrs.qualifiedName ?? node.name);
  }
  function traceabilityLinkCount(nodeId, edges) {
    let links = 0;
    for (const edge of edges) {
      const relType = asString(edge.type ?? edge.rel_type).toLowerCase();
      if (!/(satisfy|derivation|derive|verify|subject)/.test(relType)) continue;
      const source = asString(edge.source);
      const target = asString(edge.target);
      if (source === nodeId || target === nodeId) {
        links += 1;
      }
    }
    return links;
  }
  function packageLabelOf(qualifiedName) {
    const segments = qualifiedName.split("::").filter(Boolean);
    return segments.length > 1 ? segments[0] : "";
  }
  function projectionHints(visualization) {
    return asRecord(visualization?.projectionHints);
  }
  function gridLayoutHint(visualization) {
    return asString(projectionHints(visualization).gridLayout) || void 0;
  }
  function gridSubtypeHint(visualization) {
    return asString(projectionHints(visualization).gridSubtype) || void 0;
  }
  function browserLayoutHint(visualization) {
    return asString(projectionHints(visualization).browserLayout) || void 0;
  }
  function treeRootHints(visualization) {
    return asArray(projectionHints(visualization).treeRoots).map((value) => asString(value)).filter(Boolean);
  }
  function optionalUri(node) {
    return nodeUri(node) ?? void 0;
  }
  function optionalRange(node) {
    return nodeRange(node) ?? void 0;
  }
  function buildHierarchyRows(graphNodes, treeRoots) {
    const byId = new Map(graphNodes.map((node) => [node.id, node]));
    const childrenByParent = /* @__PURE__ */ new Map();
    for (const node of graphNodes) {
      if (!node.parentId || !byId.has(node.parentId)) {
        continue;
      }
      const siblings = childrenByParent.get(node.parentId) ?? [];
      siblings.push(node);
      childrenByParent.set(node.parentId, siblings);
    }
    for (const siblings of childrenByParent.values()) {
      siblings.sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
    }
    const roots = treeRoots.length > 0 ? treeRoots.map((id2) => byId.get(id2)).filter((node) => Boolean(node)) : graphNodes.filter((node) => !node.parentId || !byId.has(node.parentId));
    const rows = [];
    const visit = (node, depth) => {
      const children2 = childrenByParent.get(node.id) ?? [];
      rows.push({
        ...node,
        depth,
        hasChildren: children2.length > 0
      });
      for (const child of children2) {
        visit(child, depth + 1);
      }
    };
    roots.sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
    for (const root2 of roots) {
      visit(root2, 0);
    }
    return rows;
  }
  function buildRelationshipMatrix(nodeIds, graphEdges) {
    const edgeByPair = /* @__PURE__ */ new Map();
    for (const edge of graphEdges) {
      const source = asString(edge.source);
      const target = asString(edge.target);
      if (!source || !target) continue;
      edgeByPair.set(`${source}::${target}`, asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, ""));
    }
    const cells = [];
    for (const source of nodeIds) {
      for (const target of nodeIds) {
        const label = edgeByPair.get(`${source}::${target}`) ?? "";
        cells.push({ source, target, present: label.length > 0, label });
      }
    }
    return cells;
  }
  function prepareBrowser(visualization) {
    const graphNodes = graphNodesForStandardView(visualization).map((node) => ({
      id: asString(node.id),
      label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
      kind: elementTypeOf(node) || "element",
      parentId: asString(node.parent_id ?? node.parentId ?? asRecord(node.attributes).parentId),
      qualifiedName: qualifiedNameOf(node),
      uri: optionalUri(node),
      range: optionalRange(node)
    }));
    const hierarchyLayout = browserLayoutHint(visualization) === "hierarchy";
    const rows = hierarchyLayout ? buildHierarchyRows(graphNodes, treeRootHints(visualization)) : graphNodes.map((row) => ({ ...row, depth: 0, hasChildren: false })).sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
    return {
      title: asString(visualization?.selectedViewName, "Browser View"),
      view: "browser-view",
      nodes: rows.map((row, index) => ({
        id: row.id || `browser-row-${index}`,
        label: row.label,
        kind: row.kind,
        uri: row.uri,
        range: row.range,
        attributes: { ...row }
      })),
      edges: [],
      meta: { rows, hierarchyLayout, provisional: !hierarchyLayout }
    };
  }
  function prepareGrid(visualization) {
    const graphEdges = graphEdgesForStandardView(visualization);
    const traceabilityLayout = gridLayoutHint(visualization) === "traceability";
    const relationshipMatrix = gridSubtypeHint(visualization) === "relationship_matrix";
    const cells = graphNodesForStandardView(visualization).map((node) => {
      const attrs = asRecord(node.attributes);
      const qualifiedName = qualifiedNameOf(node);
      const nodeId = asString(node.id);
      const linkCount = traceabilityLinkCount(nodeId, graphEdges);
      return {
        id: nodeId,
        name: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
        kind: elementTypeOf(node) || "element",
        package: packageLabelOf(qualifiedName),
        qualifiedName,
        linkCount,
        attributeCount: asArray(attrs.attributes).length,
        partCount: asArray(attrs.parts).length,
        portCount: asArray(attrs.ports).length,
        uri: optionalUri(node),
        range: optionalRange(node)
      };
    }).sort((left, right) => left.qualifiedName.localeCompare(right.qualifiedName));
    const nodeIds = cells.map((cell) => cell.id).filter(Boolean);
    const matrixCells = relationshipMatrix ? buildRelationshipMatrix(nodeIds, graphEdges) : [];
    return {
      title: asString(visualization?.selectedViewName, "Grid View"),
      view: "grid-view",
      nodes: cells.map((cell, index) => ({
        id: cell.id || `grid-row-${index}`,
        label: cell.name,
        kind: cell.kind,
        uri: cell.uri,
        range: cell.range,
        attributes: cell
      })),
      edges: [],
      meta: {
        cells,
        traceabilityTable: traceabilityLayout,
        relationshipMatrix,
        matrixRowIds: relationshipMatrix ? nodeIds : [],
        matrixColIds: relationshipMatrix ? nodeIds : [],
        matrixCells,
        provisional: !relationshipMatrix && !traceabilityLayout
      }
    };
  }
  function prepareGeometry(visualization) {
    const graphNodes = graphNodesForStandardView(visualization);
    const graphEdges = graphEdgesForStandardView(visualization);
    const hints = projectionHints(visualization);
    const elements = graphNodes.map((node) => ({
      id: asString(node.id),
      label: asString(node.name ?? node.qualifiedName ?? node.id, "Unnamed"),
      kind: elementTypeOf(node) || "element",
      qualifiedName: qualifiedNameOf(node),
      uri: optionalUri(node),
      range: optionalRange(node)
    }));
    const nodeIds = new Set(elements.map((element) => element.id));
    return {
      title: asString(visualization?.selectedViewName, "Geometry View"),
      view: "geometry-view",
      nodes: elements.map((element, index) => ({
        id: element.id || `geometry-node-${index}`,
        label: element.label,
        kind: element.kind,
        uri: element.uri,
        range: element.range,
        attributes: element
      })),
      edges: graphEdges.map((edge, index) => ({
        id: asString(edge.id, `geometry-edge-${index}`),
        source: asString(edge.source),
        target: asString(edge.target),
        label: asString(edge.name ?? edge.label ?? edge.type ?? edge.rel_type, "")
      })).filter((edge) => nodeIds.has(edge.source) && nodeIds.has(edge.target)),
      meta: {
        elements,
        geometryMode: asString(hints.geometryMode, "2d"),
        geometryProjection: asString(hints.geometryProjection, "orthographic"),
        provisional: true
      }
    };
  }

  // shared/diagram-renderer/src/prepare/types.ts
  function interconnectionPreparedForLayout(prepared) {
    return prepared;
  }

  // shared/diagram-renderer/src/prepare/index.ts
  function prepareViewData(visualizationInput) {
    const passthrough = asRecord(visualizationInput).preparedView;
    if (passthrough && typeof passthrough === "object") {
      const candidate = asRecord(passthrough);
      if (typeof candidate.view === "string" && Array.isArray(candidate.nodes) && Array.isArray(candidate.edges)) {
        return candidate;
      }
    }
    const normalized = normalizeVisualizationPayload(asRecord(visualizationInput));
    const visualization = asRecord(normalized);
    const view = visualization?.view || "general-view";
    if (view === "interconnection-view") return prepareInterconnection(visualization);
    if (view === "action-flow-view") return prepareActivity(visualization);
    if (view === "state-transition-view") return prepareState(visualization);
    if (view === "sequence-view") return prepareSequence(visualization);
    if (view === "browser-view") return prepareBrowser(visualization);
    if (view === "grid-view") return prepareGrid(visualization);
    if (view === "geometry-view") return prepareGeometry(visualization);
    return prepareGraph(visualization?.generalViewGraph ?? visualization?.graph, visualization);
  }

  // shared/diagram-renderer/node_modules/d3-dispatch/src/dispatch.js
  var noop = { value: () => {
  } };
  function dispatch() {
    for (var i = 0, n = arguments.length, _ = {}, t; i < n; ++i) {
      if (!(t = arguments[i] + "") || t in _ || /[\s.]/.test(t)) throw new Error("illegal type: " + t);
      _[t] = [];
    }
    return new Dispatch(_);
  }
  function Dispatch(_) {
    this._ = _;
  }
  function parseTypenames(typenames, types) {
    return typenames.trim().split(/^|\s+/).map(function(t) {
      var name = "", i = t.indexOf(".");
      if (i >= 0) name = t.slice(i + 1), t = t.slice(0, i);
      if (t && !types.hasOwnProperty(t)) throw new Error("unknown type: " + t);
      return { type: t, name };
    });
  }
  Dispatch.prototype = dispatch.prototype = {
    constructor: Dispatch,
    on: function(typename, callback) {
      var _ = this._, T = parseTypenames(typename + "", _), t, i = -1, n = T.length;
      if (arguments.length < 2) {
        while (++i < n) if ((t = (typename = T[i]).type) && (t = get(_[t], typename.name))) return t;
        return;
      }
      if (callback != null && typeof callback !== "function") throw new Error("invalid callback: " + callback);
      while (++i < n) {
        if (t = (typename = T[i]).type) _[t] = set(_[t], typename.name, callback);
        else if (callback == null) for (t in _) _[t] = set(_[t], typename.name, null);
      }
      return this;
    },
    copy: function() {
      var copy = {}, _ = this._;
      for (var t in _) copy[t] = _[t].slice();
      return new Dispatch(copy);
    },
    call: function(type2, that) {
      if ((n = arguments.length - 2) > 0) for (var args = new Array(n), i = 0, n, t; i < n; ++i) args[i] = arguments[i + 2];
      if (!this._.hasOwnProperty(type2)) throw new Error("unknown type: " + type2);
      for (t = this._[type2], i = 0, n = t.length; i < n; ++i) t[i].value.apply(that, args);
    },
    apply: function(type2, that, args) {
      if (!this._.hasOwnProperty(type2)) throw new Error("unknown type: " + type2);
      for (var t = this._[type2], i = 0, n = t.length; i < n; ++i) t[i].value.apply(that, args);
    }
  };
  function get(type2, name) {
    for (var i = 0, n = type2.length, c; i < n; ++i) {
      if ((c = type2[i]).name === name) {
        return c.value;
      }
    }
  }
  function set(type2, name, callback) {
    for (var i = 0, n = type2.length; i < n; ++i) {
      if (type2[i].name === name) {
        type2[i] = noop, type2 = type2.slice(0, i).concat(type2.slice(i + 1));
        break;
      }
    }
    if (callback != null) type2.push({ name, value: callback });
    return type2;
  }
  var dispatch_default = dispatch;

  // shared/diagram-renderer/node_modules/d3-selection/src/namespaces.js
  var xhtml = "http://www.w3.org/1999/xhtml";
  var namespaces_default = {
    svg: "http://www.w3.org/2000/svg",
    xhtml,
    xlink: "http://www.w3.org/1999/xlink",
    xml: "http://www.w3.org/XML/1998/namespace",
    xmlns: "http://www.w3.org/2000/xmlns/"
  };

  // shared/diagram-renderer/node_modules/d3-selection/src/namespace.js
  function namespace_default(name) {
    var prefix = name += "", i = prefix.indexOf(":");
    if (i >= 0 && (prefix = name.slice(0, i)) !== "xmlns") name = name.slice(i + 1);
    return namespaces_default.hasOwnProperty(prefix) ? { space: namespaces_default[prefix], local: name } : name;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/creator.js
  function creatorInherit(name) {
    return function() {
      var document2 = this.ownerDocument, uri = this.namespaceURI;
      return uri === xhtml && document2.documentElement.namespaceURI === xhtml ? document2.createElement(name) : document2.createElementNS(uri, name);
    };
  }
  function creatorFixed(fullname) {
    return function() {
      return this.ownerDocument.createElementNS(fullname.space, fullname.local);
    };
  }
  function creator_default(name) {
    var fullname = namespace_default(name);
    return (fullname.local ? creatorFixed : creatorInherit)(fullname);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selector.js
  function none() {
  }
  function selector_default(selector) {
    return selector == null ? none : function() {
      return this.querySelector(selector);
    };
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/select.js
  function select_default(select) {
    if (typeof select !== "function") select = selector_default(select);
    for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, subgroup = subgroups[j] = new Array(n), node, subnode, i = 0; i < n; ++i) {
        if ((node = group[i]) && (subnode = select.call(node, node.__data__, i, group))) {
          if ("__data__" in node) subnode.__data__ = node.__data__;
          subgroup[i] = subnode;
        }
      }
    }
    return new Selection(subgroups, this._parents);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/array.js
  function array(x2) {
    return x2 == null ? [] : Array.isArray(x2) ? x2 : Array.from(x2);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selectorAll.js
  function empty() {
    return [];
  }
  function selectorAll_default(selector) {
    return selector == null ? empty : function() {
      return this.querySelectorAll(selector);
    };
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/selectAll.js
  function arrayAll(select) {
    return function() {
      return array(select.apply(this, arguments));
    };
  }
  function selectAll_default(select) {
    if (typeof select === "function") select = arrayAll(select);
    else select = selectorAll_default(select);
    for (var groups = this._groups, m = groups.length, subgroups = [], parents = [], j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
        if (node = group[i]) {
          subgroups.push(select.call(node, node.__data__, i, group));
          parents.push(node);
        }
      }
    }
    return new Selection(subgroups, parents);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/matcher.js
  function matcher_default(selector) {
    return function() {
      return this.matches(selector);
    };
  }
  function childMatcher(selector) {
    return function(node) {
      return node.matches(selector);
    };
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/selectChild.js
  var find = Array.prototype.find;
  function childFind(match) {
    return function() {
      return find.call(this.children, match);
    };
  }
  function childFirst() {
    return this.firstElementChild;
  }
  function selectChild_default(match) {
    return this.select(match == null ? childFirst : childFind(typeof match === "function" ? match : childMatcher(match)));
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/selectChildren.js
  var filter = Array.prototype.filter;
  function children() {
    return Array.from(this.children);
  }
  function childrenFilter(match) {
    return function() {
      return filter.call(this.children, match);
    };
  }
  function selectChildren_default(match) {
    return this.selectAll(match == null ? children : childrenFilter(typeof match === "function" ? match : childMatcher(match)));
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/filter.js
  function filter_default(match) {
    if (typeof match !== "function") match = matcher_default(match);
    for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, subgroup = subgroups[j] = [], node, i = 0; i < n; ++i) {
        if ((node = group[i]) && match.call(node, node.__data__, i, group)) {
          subgroup.push(node);
        }
      }
    }
    return new Selection(subgroups, this._parents);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/sparse.js
  function sparse_default(update) {
    return new Array(update.length);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/enter.js
  function enter_default() {
    return new Selection(this._enter || this._groups.map(sparse_default), this._parents);
  }
  function EnterNode(parent, datum2) {
    this.ownerDocument = parent.ownerDocument;
    this.namespaceURI = parent.namespaceURI;
    this._next = null;
    this._parent = parent;
    this.__data__ = datum2;
  }
  EnterNode.prototype = {
    constructor: EnterNode,
    appendChild: function(child) {
      return this._parent.insertBefore(child, this._next);
    },
    insertBefore: function(child, next) {
      return this._parent.insertBefore(child, next);
    },
    querySelector: function(selector) {
      return this._parent.querySelector(selector);
    },
    querySelectorAll: function(selector) {
      return this._parent.querySelectorAll(selector);
    }
  };

  // shared/diagram-renderer/node_modules/d3-selection/src/constant.js
  function constant_default(x2) {
    return function() {
      return x2;
    };
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/data.js
  function bindIndex(parent, group, enter, update, exit, data) {
    var i = 0, node, groupLength = group.length, dataLength = data.length;
    for (; i < dataLength; ++i) {
      if (node = group[i]) {
        node.__data__ = data[i];
        update[i] = node;
      } else {
        enter[i] = new EnterNode(parent, data[i]);
      }
    }
    for (; i < groupLength; ++i) {
      if (node = group[i]) {
        exit[i] = node;
      }
    }
  }
  function bindKey(parent, group, enter, update, exit, data, key) {
    var i, node, nodeByKeyValue = /* @__PURE__ */ new Map(), groupLength = group.length, dataLength = data.length, keyValues = new Array(groupLength), keyValue;
    for (i = 0; i < groupLength; ++i) {
      if (node = group[i]) {
        keyValues[i] = keyValue = key.call(node, node.__data__, i, group) + "";
        if (nodeByKeyValue.has(keyValue)) {
          exit[i] = node;
        } else {
          nodeByKeyValue.set(keyValue, node);
        }
      }
    }
    for (i = 0; i < dataLength; ++i) {
      keyValue = key.call(parent, data[i], i, data) + "";
      if (node = nodeByKeyValue.get(keyValue)) {
        update[i] = node;
        node.__data__ = data[i];
        nodeByKeyValue.delete(keyValue);
      } else {
        enter[i] = new EnterNode(parent, data[i]);
      }
    }
    for (i = 0; i < groupLength; ++i) {
      if ((node = group[i]) && nodeByKeyValue.get(keyValues[i]) === node) {
        exit[i] = node;
      }
    }
  }
  function datum(node) {
    return node.__data__;
  }
  function data_default(value, key) {
    if (!arguments.length) return Array.from(this, datum);
    var bind = key ? bindKey : bindIndex, parents = this._parents, groups = this._groups;
    if (typeof value !== "function") value = constant_default(value);
    for (var m = groups.length, update = new Array(m), enter = new Array(m), exit = new Array(m), j = 0; j < m; ++j) {
      var parent = parents[j], group = groups[j], groupLength = group.length, data = arraylike(value.call(parent, parent && parent.__data__, j, parents)), dataLength = data.length, enterGroup = enter[j] = new Array(dataLength), updateGroup = update[j] = new Array(dataLength), exitGroup = exit[j] = new Array(groupLength);
      bind(parent, group, enterGroup, updateGroup, exitGroup, data, key);
      for (var i0 = 0, i1 = 0, previous, next; i0 < dataLength; ++i0) {
        if (previous = enterGroup[i0]) {
          if (i0 >= i1) i1 = i0 + 1;
          while (!(next = updateGroup[i1]) && ++i1 < dataLength) ;
          previous._next = next || null;
        }
      }
    }
    update = new Selection(update, parents);
    update._enter = enter;
    update._exit = exit;
    return update;
  }
  function arraylike(data) {
    return typeof data === "object" && "length" in data ? data : Array.from(data);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/exit.js
  function exit_default() {
    return new Selection(this._exit || this._groups.map(sparse_default), this._parents);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/join.js
  function join_default(onenter, onupdate, onexit) {
    var enter = this.enter(), update = this, exit = this.exit();
    if (typeof onenter === "function") {
      enter = onenter(enter);
      if (enter) enter = enter.selection();
    } else {
      enter = enter.append(onenter + "");
    }
    if (onupdate != null) {
      update = onupdate(update);
      if (update) update = update.selection();
    }
    if (onexit == null) exit.remove();
    else onexit(exit);
    return enter && update ? enter.merge(update).order() : update;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/merge.js
  function merge_default(context) {
    var selection2 = context.selection ? context.selection() : context;
    for (var groups0 = this._groups, groups1 = selection2._groups, m0 = groups0.length, m1 = groups1.length, m = Math.min(m0, m1), merges = new Array(m0), j = 0; j < m; ++j) {
      for (var group0 = groups0[j], group1 = groups1[j], n = group0.length, merge = merges[j] = new Array(n), node, i = 0; i < n; ++i) {
        if (node = group0[i] || group1[i]) {
          merge[i] = node;
        }
      }
    }
    for (; j < m0; ++j) {
      merges[j] = groups0[j];
    }
    return new Selection(merges, this._parents);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/order.js
  function order_default() {
    for (var groups = this._groups, j = -1, m = groups.length; ++j < m; ) {
      for (var group = groups[j], i = group.length - 1, next = group[i], node; --i >= 0; ) {
        if (node = group[i]) {
          if (next && node.compareDocumentPosition(next) ^ 4) next.parentNode.insertBefore(node, next);
          next = node;
        }
      }
    }
    return this;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/sort.js
  function sort_default(compare) {
    if (!compare) compare = ascending;
    function compareNode(a, b) {
      return a && b ? compare(a.__data__, b.__data__) : !a - !b;
    }
    for (var groups = this._groups, m = groups.length, sortgroups = new Array(m), j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, sortgroup = sortgroups[j] = new Array(n), node, i = 0; i < n; ++i) {
        if (node = group[i]) {
          sortgroup[i] = node;
        }
      }
      sortgroup.sort(compareNode);
    }
    return new Selection(sortgroups, this._parents).order();
  }
  function ascending(a, b) {
    return a < b ? -1 : a > b ? 1 : a >= b ? 0 : NaN;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/call.js
  function call_default() {
    var callback = arguments[0];
    arguments[0] = this;
    callback.apply(null, arguments);
    return this;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/nodes.js
  function nodes_default() {
    return Array.from(this);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/node.js
  function node_default() {
    for (var groups = this._groups, j = 0, m = groups.length; j < m; ++j) {
      for (var group = groups[j], i = 0, n = group.length; i < n; ++i) {
        var node = group[i];
        if (node) return node;
      }
    }
    return null;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/size.js
  function size_default() {
    let size = 0;
    for (const node of this) ++size;
    return size;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/empty.js
  function empty_default() {
    return !this.node();
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/each.js
  function each_default(callback) {
    for (var groups = this._groups, j = 0, m = groups.length; j < m; ++j) {
      for (var group = groups[j], i = 0, n = group.length, node; i < n; ++i) {
        if (node = group[i]) callback.call(node, node.__data__, i, group);
      }
    }
    return this;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/attr.js
  function attrRemove(name) {
    return function() {
      this.removeAttribute(name);
    };
  }
  function attrRemoveNS(fullname) {
    return function() {
      this.removeAttributeNS(fullname.space, fullname.local);
    };
  }
  function attrConstant(name, value) {
    return function() {
      this.setAttribute(name, value);
    };
  }
  function attrConstantNS(fullname, value) {
    return function() {
      this.setAttributeNS(fullname.space, fullname.local, value);
    };
  }
  function attrFunction(name, value) {
    return function() {
      var v = value.apply(this, arguments);
      if (v == null) this.removeAttribute(name);
      else this.setAttribute(name, v);
    };
  }
  function attrFunctionNS(fullname, value) {
    return function() {
      var v = value.apply(this, arguments);
      if (v == null) this.removeAttributeNS(fullname.space, fullname.local);
      else this.setAttributeNS(fullname.space, fullname.local, v);
    };
  }
  function attr_default(name, value) {
    var fullname = namespace_default(name);
    if (arguments.length < 2) {
      var node = this.node();
      return fullname.local ? node.getAttributeNS(fullname.space, fullname.local) : node.getAttribute(fullname);
    }
    return this.each((value == null ? fullname.local ? attrRemoveNS : attrRemove : typeof value === "function" ? fullname.local ? attrFunctionNS : attrFunction : fullname.local ? attrConstantNS : attrConstant)(fullname, value));
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/window.js
  function window_default(node) {
    return node.ownerDocument && node.ownerDocument.defaultView || node.document && node || node.defaultView;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/style.js
  function styleRemove(name) {
    return function() {
      this.style.removeProperty(name);
    };
  }
  function styleConstant(name, value, priority) {
    return function() {
      this.style.setProperty(name, value, priority);
    };
  }
  function styleFunction(name, value, priority) {
    return function() {
      var v = value.apply(this, arguments);
      if (v == null) this.style.removeProperty(name);
      else this.style.setProperty(name, v, priority);
    };
  }
  function style_default(name, value, priority) {
    return arguments.length > 1 ? this.each((value == null ? styleRemove : typeof value === "function" ? styleFunction : styleConstant)(name, value, priority == null ? "" : priority)) : styleValue(this.node(), name);
  }
  function styleValue(node, name) {
    return node.style.getPropertyValue(name) || window_default(node).getComputedStyle(node, null).getPropertyValue(name);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/property.js
  function propertyRemove(name) {
    return function() {
      delete this[name];
    };
  }
  function propertyConstant(name, value) {
    return function() {
      this[name] = value;
    };
  }
  function propertyFunction(name, value) {
    return function() {
      var v = value.apply(this, arguments);
      if (v == null) delete this[name];
      else this[name] = v;
    };
  }
  function property_default(name, value) {
    return arguments.length > 1 ? this.each((value == null ? propertyRemove : typeof value === "function" ? propertyFunction : propertyConstant)(name, value)) : this.node()[name];
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/classed.js
  function classArray(string) {
    return string.trim().split(/^|\s+/);
  }
  function classList(node) {
    return node.classList || new ClassList(node);
  }
  function ClassList(node) {
    this._node = node;
    this._names = classArray(node.getAttribute("class") || "");
  }
  ClassList.prototype = {
    add: function(name) {
      var i = this._names.indexOf(name);
      if (i < 0) {
        this._names.push(name);
        this._node.setAttribute("class", this._names.join(" "));
      }
    },
    remove: function(name) {
      var i = this._names.indexOf(name);
      if (i >= 0) {
        this._names.splice(i, 1);
        this._node.setAttribute("class", this._names.join(" "));
      }
    },
    contains: function(name) {
      return this._names.indexOf(name) >= 0;
    }
  };
  function classedAdd(node, names) {
    var list = classList(node), i = -1, n = names.length;
    while (++i < n) list.add(names[i]);
  }
  function classedRemove(node, names) {
    var list = classList(node), i = -1, n = names.length;
    while (++i < n) list.remove(names[i]);
  }
  function classedTrue(names) {
    return function() {
      classedAdd(this, names);
    };
  }
  function classedFalse(names) {
    return function() {
      classedRemove(this, names);
    };
  }
  function classedFunction(names, value) {
    return function() {
      (value.apply(this, arguments) ? classedAdd : classedRemove)(this, names);
    };
  }
  function classed_default(name, value) {
    var names = classArray(name + "");
    if (arguments.length < 2) {
      var list = classList(this.node()), i = -1, n = names.length;
      while (++i < n) if (!list.contains(names[i])) return false;
      return true;
    }
    return this.each((typeof value === "function" ? classedFunction : value ? classedTrue : classedFalse)(names, value));
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/text.js
  function textRemove() {
    this.textContent = "";
  }
  function textConstant(value) {
    return function() {
      this.textContent = value;
    };
  }
  function textFunction(value) {
    return function() {
      var v = value.apply(this, arguments);
      this.textContent = v == null ? "" : v;
    };
  }
  function text_default(value) {
    return arguments.length ? this.each(value == null ? textRemove : (typeof value === "function" ? textFunction : textConstant)(value)) : this.node().textContent;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/html.js
  function htmlRemove() {
    this.innerHTML = "";
  }
  function htmlConstant(value) {
    return function() {
      this.innerHTML = value;
    };
  }
  function htmlFunction(value) {
    return function() {
      var v = value.apply(this, arguments);
      this.innerHTML = v == null ? "" : v;
    };
  }
  function html_default(value) {
    return arguments.length ? this.each(value == null ? htmlRemove : (typeof value === "function" ? htmlFunction : htmlConstant)(value)) : this.node().innerHTML;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/raise.js
  function raise() {
    if (this.nextSibling) this.parentNode.appendChild(this);
  }
  function raise_default() {
    return this.each(raise);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/lower.js
  function lower() {
    if (this.previousSibling) this.parentNode.insertBefore(this, this.parentNode.firstChild);
  }
  function lower_default() {
    return this.each(lower);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/append.js
  function append_default(name) {
    var create2 = typeof name === "function" ? name : creator_default(name);
    return this.select(function() {
      return this.appendChild(create2.apply(this, arguments));
    });
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/insert.js
  function constantNull() {
    return null;
  }
  function insert_default(name, before) {
    var create2 = typeof name === "function" ? name : creator_default(name), select = before == null ? constantNull : typeof before === "function" ? before : selector_default(before);
    return this.select(function() {
      return this.insertBefore(create2.apply(this, arguments), select.apply(this, arguments) || null);
    });
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/remove.js
  function remove() {
    var parent = this.parentNode;
    if (parent) parent.removeChild(this);
  }
  function remove_default() {
    return this.each(remove);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/clone.js
  function selection_cloneShallow() {
    var clone = this.cloneNode(false), parent = this.parentNode;
    return parent ? parent.insertBefore(clone, this.nextSibling) : clone;
  }
  function selection_cloneDeep() {
    var clone = this.cloneNode(true), parent = this.parentNode;
    return parent ? parent.insertBefore(clone, this.nextSibling) : clone;
  }
  function clone_default(deep) {
    return this.select(deep ? selection_cloneDeep : selection_cloneShallow);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/datum.js
  function datum_default(value) {
    return arguments.length ? this.property("__data__", value) : this.node().__data__;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/on.js
  function contextListener(listener) {
    return function(event) {
      listener.call(this, event, this.__data__);
    };
  }
  function parseTypenames2(typenames) {
    return typenames.trim().split(/^|\s+/).map(function(t) {
      var name = "", i = t.indexOf(".");
      if (i >= 0) name = t.slice(i + 1), t = t.slice(0, i);
      return { type: t, name };
    });
  }
  function onRemove(typename) {
    return function() {
      var on = this.__on;
      if (!on) return;
      for (var j = 0, i = -1, m = on.length, o; j < m; ++j) {
        if (o = on[j], (!typename.type || o.type === typename.type) && o.name === typename.name) {
          this.removeEventListener(o.type, o.listener, o.options);
        } else {
          on[++i] = o;
        }
      }
      if (++i) on.length = i;
      else delete this.__on;
    };
  }
  function onAdd(typename, value, options) {
    return function() {
      var on = this.__on, o, listener = contextListener(value);
      if (on) for (var j = 0, m = on.length; j < m; ++j) {
        if ((o = on[j]).type === typename.type && o.name === typename.name) {
          this.removeEventListener(o.type, o.listener, o.options);
          this.addEventListener(o.type, o.listener = listener, o.options = options);
          o.value = value;
          return;
        }
      }
      this.addEventListener(typename.type, listener, options);
      o = { type: typename.type, name: typename.name, value, listener, options };
      if (!on) this.__on = [o];
      else on.push(o);
    };
  }
  function on_default(typename, value, options) {
    var typenames = parseTypenames2(typename + ""), i, n = typenames.length, t;
    if (arguments.length < 2) {
      var on = this.node().__on;
      if (on) for (var j = 0, m = on.length, o; j < m; ++j) {
        for (i = 0, o = on[j]; i < n; ++i) {
          if ((t = typenames[i]).type === o.type && t.name === o.name) {
            return o.value;
          }
        }
      }
      return;
    }
    on = value ? onAdd : onRemove;
    for (i = 0; i < n; ++i) this.each(on(typenames[i], value, options));
    return this;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/dispatch.js
  function dispatchEvent(node, type2, params) {
    var window2 = window_default(node), event = window2.CustomEvent;
    if (typeof event === "function") {
      event = new event(type2, params);
    } else {
      event = window2.document.createEvent("Event");
      if (params) event.initEvent(type2, params.bubbles, params.cancelable), event.detail = params.detail;
      else event.initEvent(type2, false, false);
    }
    node.dispatchEvent(event);
  }
  function dispatchConstant(type2, params) {
    return function() {
      return dispatchEvent(this, type2, params);
    };
  }
  function dispatchFunction(type2, params) {
    return function() {
      return dispatchEvent(this, type2, params.apply(this, arguments));
    };
  }
  function dispatch_default2(type2, params) {
    return this.each((typeof params === "function" ? dispatchFunction : dispatchConstant)(type2, params));
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/iterator.js
  function* iterator_default() {
    for (var groups = this._groups, j = 0, m = groups.length; j < m; ++j) {
      for (var group = groups[j], i = 0, n = group.length, node; i < n; ++i) {
        if (node = group[i]) yield node;
      }
    }
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/selection/index.js
  var root = [null];
  function Selection(groups, parents) {
    this._groups = groups;
    this._parents = parents;
  }
  function selection() {
    return new Selection([[document.documentElement]], root);
  }
  function selection_selection() {
    return this;
  }
  Selection.prototype = selection.prototype = {
    constructor: Selection,
    select: select_default,
    selectAll: selectAll_default,
    selectChild: selectChild_default,
    selectChildren: selectChildren_default,
    filter: filter_default,
    data: data_default,
    enter: enter_default,
    exit: exit_default,
    join: join_default,
    merge: merge_default,
    selection: selection_selection,
    order: order_default,
    sort: sort_default,
    call: call_default,
    nodes: nodes_default,
    node: node_default,
    size: size_default,
    empty: empty_default,
    each: each_default,
    attr: attr_default,
    style: style_default,
    property: property_default,
    classed: classed_default,
    text: text_default,
    html: html_default,
    raise: raise_default,
    lower: lower_default,
    append: append_default,
    insert: insert_default,
    remove: remove_default,
    clone: clone_default,
    datum: datum_default,
    on: on_default,
    dispatch: dispatch_default2,
    [Symbol.iterator]: iterator_default
  };
  var selection_default = selection;

  // shared/diagram-renderer/node_modules/d3-selection/src/select.js
  function select_default2(selector) {
    return typeof selector === "string" ? new Selection([[document.querySelector(selector)]], [document.documentElement]) : new Selection([[selector]], root);
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/sourceEvent.js
  function sourceEvent_default(event) {
    let sourceEvent;
    while (sourceEvent = event.sourceEvent) event = sourceEvent;
    return event;
  }

  // shared/diagram-renderer/node_modules/d3-selection/src/pointer.js
  function pointer_default(event, node) {
    event = sourceEvent_default(event);
    if (node === void 0) node = event.currentTarget;
    if (node) {
      var svg = node.ownerSVGElement || node;
      if (svg.createSVGPoint) {
        var point = svg.createSVGPoint();
        point.x = event.clientX, point.y = event.clientY;
        point = point.matrixTransform(node.getScreenCTM().inverse());
        return [point.x, point.y];
      }
      if (node.getBoundingClientRect) {
        var rect = node.getBoundingClientRect();
        return [event.clientX - rect.left - node.clientLeft, event.clientY - rect.top - node.clientTop];
      }
    }
    return [event.pageX, event.pageY];
  }

  // shared/diagram-renderer/node_modules/d3-drag/src/noevent.js
  var nonpassivecapture = { capture: true, passive: false };
  function noevent_default(event) {
    event.preventDefault();
    event.stopImmediatePropagation();
  }

  // shared/diagram-renderer/node_modules/d3-drag/src/nodrag.js
  function nodrag_default(view) {
    var root2 = view.document.documentElement, selection2 = select_default2(view).on("dragstart.drag", noevent_default, nonpassivecapture);
    if ("onselectstart" in root2) {
      selection2.on("selectstart.drag", noevent_default, nonpassivecapture);
    } else {
      root2.__noselect = root2.style.MozUserSelect;
      root2.style.MozUserSelect = "none";
    }
  }
  function yesdrag(view, noclick) {
    var root2 = view.document.documentElement, selection2 = select_default2(view).on("dragstart.drag", null);
    if (noclick) {
      selection2.on("click.drag", noevent_default, nonpassivecapture);
      setTimeout(function() {
        selection2.on("click.drag", null);
      }, 0);
    }
    if ("onselectstart" in root2) {
      selection2.on("selectstart.drag", null);
    } else {
      root2.style.MozUserSelect = root2.__noselect;
      delete root2.__noselect;
    }
  }

  // shared/diagram-renderer/node_modules/d3-color/src/define.js
  function define_default(constructor, factory, prototype) {
    constructor.prototype = factory.prototype = prototype;
    prototype.constructor = constructor;
  }
  function extend(parent, definition) {
    var prototype = Object.create(parent.prototype);
    for (var key in definition) prototype[key] = definition[key];
    return prototype;
  }

  // shared/diagram-renderer/node_modules/d3-color/src/color.js
  function Color() {
  }
  var darker = 0.7;
  var brighter = 1 / darker;
  var reI = "\\s*([+-]?\\d+)\\s*";
  var reN = "\\s*([+-]?(?:\\d*\\.)?\\d+(?:[eE][+-]?\\d+)?)\\s*";
  var reP = "\\s*([+-]?(?:\\d*\\.)?\\d+(?:[eE][+-]?\\d+)?)%\\s*";
  var reHex = /^#([0-9a-f]{3,8})$/;
  var reRgbInteger = new RegExp(`^rgb\\(${reI},${reI},${reI}\\)$`);
  var reRgbPercent = new RegExp(`^rgb\\(${reP},${reP},${reP}\\)$`);
  var reRgbaInteger = new RegExp(`^rgba\\(${reI},${reI},${reI},${reN}\\)$`);
  var reRgbaPercent = new RegExp(`^rgba\\(${reP},${reP},${reP},${reN}\\)$`);
  var reHslPercent = new RegExp(`^hsl\\(${reN},${reP},${reP}\\)$`);
  var reHslaPercent = new RegExp(`^hsla\\(${reN},${reP},${reP},${reN}\\)$`);
  var named = {
    aliceblue: 15792383,
    antiquewhite: 16444375,
    aqua: 65535,
    aquamarine: 8388564,
    azure: 15794175,
    beige: 16119260,
    bisque: 16770244,
    black: 0,
    blanchedalmond: 16772045,
    blue: 255,
    blueviolet: 9055202,
    brown: 10824234,
    burlywood: 14596231,
    cadetblue: 6266528,
    chartreuse: 8388352,
    chocolate: 13789470,
    coral: 16744272,
    cornflowerblue: 6591981,
    cornsilk: 16775388,
    crimson: 14423100,
    cyan: 65535,
    darkblue: 139,
    darkcyan: 35723,
    darkgoldenrod: 12092939,
    darkgray: 11119017,
    darkgreen: 25600,
    darkgrey: 11119017,
    darkkhaki: 12433259,
    darkmagenta: 9109643,
    darkolivegreen: 5597999,
    darkorange: 16747520,
    darkorchid: 10040012,
    darkred: 9109504,
    darksalmon: 15308410,
    darkseagreen: 9419919,
    darkslateblue: 4734347,
    darkslategray: 3100495,
    darkslategrey: 3100495,
    darkturquoise: 52945,
    darkviolet: 9699539,
    deeppink: 16716947,
    deepskyblue: 49151,
    dimgray: 6908265,
    dimgrey: 6908265,
    dodgerblue: 2003199,
    firebrick: 11674146,
    floralwhite: 16775920,
    forestgreen: 2263842,
    fuchsia: 16711935,
    gainsboro: 14474460,
    ghostwhite: 16316671,
    gold: 16766720,
    goldenrod: 14329120,
    gray: 8421504,
    green: 32768,
    greenyellow: 11403055,
    grey: 8421504,
    honeydew: 15794160,
    hotpink: 16738740,
    indianred: 13458524,
    indigo: 4915330,
    ivory: 16777200,
    khaki: 15787660,
    lavender: 15132410,
    lavenderblush: 16773365,
    lawngreen: 8190976,
    lemonchiffon: 16775885,
    lightblue: 11393254,
    lightcoral: 15761536,
    lightcyan: 14745599,
    lightgoldenrodyellow: 16448210,
    lightgray: 13882323,
    lightgreen: 9498256,
    lightgrey: 13882323,
    lightpink: 16758465,
    lightsalmon: 16752762,
    lightseagreen: 2142890,
    lightskyblue: 8900346,
    lightslategray: 7833753,
    lightslategrey: 7833753,
    lightsteelblue: 11584734,
    lightyellow: 16777184,
    lime: 65280,
    limegreen: 3329330,
    linen: 16445670,
    magenta: 16711935,
    maroon: 8388608,
    mediumaquamarine: 6737322,
    mediumblue: 205,
    mediumorchid: 12211667,
    mediumpurple: 9662683,
    mediumseagreen: 3978097,
    mediumslateblue: 8087790,
    mediumspringgreen: 64154,
    mediumturquoise: 4772300,
    mediumvioletred: 13047173,
    midnightblue: 1644912,
    mintcream: 16121850,
    mistyrose: 16770273,
    moccasin: 16770229,
    navajowhite: 16768685,
    navy: 128,
    oldlace: 16643558,
    olive: 8421376,
    olivedrab: 7048739,
    orange: 16753920,
    orangered: 16729344,
    orchid: 14315734,
    palegoldenrod: 15657130,
    palegreen: 10025880,
    paleturquoise: 11529966,
    palevioletred: 14381203,
    papayawhip: 16773077,
    peachpuff: 16767673,
    peru: 13468991,
    pink: 16761035,
    plum: 14524637,
    powderblue: 11591910,
    purple: 8388736,
    rebeccapurple: 6697881,
    red: 16711680,
    rosybrown: 12357519,
    royalblue: 4286945,
    saddlebrown: 9127187,
    salmon: 16416882,
    sandybrown: 16032864,
    seagreen: 3050327,
    seashell: 16774638,
    sienna: 10506797,
    silver: 12632256,
    skyblue: 8900331,
    slateblue: 6970061,
    slategray: 7372944,
    slategrey: 7372944,
    snow: 16775930,
    springgreen: 65407,
    steelblue: 4620980,
    tan: 13808780,
    teal: 32896,
    thistle: 14204888,
    tomato: 16737095,
    turquoise: 4251856,
    violet: 15631086,
    wheat: 16113331,
    white: 16777215,
    whitesmoke: 16119285,
    yellow: 16776960,
    yellowgreen: 10145074
  };
  define_default(Color, color, {
    copy(channels) {
      return Object.assign(new this.constructor(), this, channels);
    },
    displayable() {
      return this.rgb().displayable();
    },
    hex: color_formatHex,
    // Deprecated! Use color.formatHex.
    formatHex: color_formatHex,
    formatHex8: color_formatHex8,
    formatHsl: color_formatHsl,
    formatRgb: color_formatRgb,
    toString: color_formatRgb
  });
  function color_formatHex() {
    return this.rgb().formatHex();
  }
  function color_formatHex8() {
    return this.rgb().formatHex8();
  }
  function color_formatHsl() {
    return hslConvert(this).formatHsl();
  }
  function color_formatRgb() {
    return this.rgb().formatRgb();
  }
  function color(format) {
    var m, l;
    format = (format + "").trim().toLowerCase();
    return (m = reHex.exec(format)) ? (l = m[1].length, m = parseInt(m[1], 16), l === 6 ? rgbn(m) : l === 3 ? new Rgb(m >> 8 & 15 | m >> 4 & 240, m >> 4 & 15 | m & 240, (m & 15) << 4 | m & 15, 1) : l === 8 ? rgba(m >> 24 & 255, m >> 16 & 255, m >> 8 & 255, (m & 255) / 255) : l === 4 ? rgba(m >> 12 & 15 | m >> 8 & 240, m >> 8 & 15 | m >> 4 & 240, m >> 4 & 15 | m & 240, ((m & 15) << 4 | m & 15) / 255) : null) : (m = reRgbInteger.exec(format)) ? new Rgb(m[1], m[2], m[3], 1) : (m = reRgbPercent.exec(format)) ? new Rgb(m[1] * 255 / 100, m[2] * 255 / 100, m[3] * 255 / 100, 1) : (m = reRgbaInteger.exec(format)) ? rgba(m[1], m[2], m[3], m[4]) : (m = reRgbaPercent.exec(format)) ? rgba(m[1] * 255 / 100, m[2] * 255 / 100, m[3] * 255 / 100, m[4]) : (m = reHslPercent.exec(format)) ? hsla(m[1], m[2] / 100, m[3] / 100, 1) : (m = reHslaPercent.exec(format)) ? hsla(m[1], m[2] / 100, m[3] / 100, m[4]) : named.hasOwnProperty(format) ? rgbn(named[format]) : format === "transparent" ? new Rgb(NaN, NaN, NaN, 0) : null;
  }
  function rgbn(n) {
    return new Rgb(n >> 16 & 255, n >> 8 & 255, n & 255, 1);
  }
  function rgba(r, g, b, a) {
    if (a <= 0) r = g = b = NaN;
    return new Rgb(r, g, b, a);
  }
  function rgbConvert(o) {
    if (!(o instanceof Color)) o = color(o);
    if (!o) return new Rgb();
    o = o.rgb();
    return new Rgb(o.r, o.g, o.b, o.opacity);
  }
  function rgb(r, g, b, opacity) {
    return arguments.length === 1 ? rgbConvert(r) : new Rgb(r, g, b, opacity == null ? 1 : opacity);
  }
  function Rgb(r, g, b, opacity) {
    this.r = +r;
    this.g = +g;
    this.b = +b;
    this.opacity = +opacity;
  }
  define_default(Rgb, rgb, extend(Color, {
    brighter(k) {
      k = k == null ? brighter : Math.pow(brighter, k);
      return new Rgb(this.r * k, this.g * k, this.b * k, this.opacity);
    },
    darker(k) {
      k = k == null ? darker : Math.pow(darker, k);
      return new Rgb(this.r * k, this.g * k, this.b * k, this.opacity);
    },
    rgb() {
      return this;
    },
    clamp() {
      return new Rgb(clampi(this.r), clampi(this.g), clampi(this.b), clampa(this.opacity));
    },
    displayable() {
      return -0.5 <= this.r && this.r < 255.5 && (-0.5 <= this.g && this.g < 255.5) && (-0.5 <= this.b && this.b < 255.5) && (0 <= this.opacity && this.opacity <= 1);
    },
    hex: rgb_formatHex,
    // Deprecated! Use color.formatHex.
    formatHex: rgb_formatHex,
    formatHex8: rgb_formatHex8,
    formatRgb: rgb_formatRgb,
    toString: rgb_formatRgb
  }));
  function rgb_formatHex() {
    return `#${hex(this.r)}${hex(this.g)}${hex(this.b)}`;
  }
  function rgb_formatHex8() {
    return `#${hex(this.r)}${hex(this.g)}${hex(this.b)}${hex((isNaN(this.opacity) ? 1 : this.opacity) * 255)}`;
  }
  function rgb_formatRgb() {
    const a = clampa(this.opacity);
    return `${a === 1 ? "rgb(" : "rgba("}${clampi(this.r)}, ${clampi(this.g)}, ${clampi(this.b)}${a === 1 ? ")" : `, ${a})`}`;
  }
  function clampa(opacity) {
    return isNaN(opacity) ? 1 : Math.max(0, Math.min(1, opacity));
  }
  function clampi(value) {
    return Math.max(0, Math.min(255, Math.round(value) || 0));
  }
  function hex(value) {
    value = clampi(value);
    return (value < 16 ? "0" : "") + value.toString(16);
  }
  function hsla(h, s, l, a) {
    if (a <= 0) h = s = l = NaN;
    else if (l <= 0 || l >= 1) h = s = NaN;
    else if (s <= 0) h = NaN;
    return new Hsl(h, s, l, a);
  }
  function hslConvert(o) {
    if (o instanceof Hsl) return new Hsl(o.h, o.s, o.l, o.opacity);
    if (!(o instanceof Color)) o = color(o);
    if (!o) return new Hsl();
    if (o instanceof Hsl) return o;
    o = o.rgb();
    var r = o.r / 255, g = o.g / 255, b = o.b / 255, min2 = Math.min(r, g, b), max2 = Math.max(r, g, b), h = NaN, s = max2 - min2, l = (max2 + min2) / 2;
    if (s) {
      if (r === max2) h = (g - b) / s + (g < b) * 6;
      else if (g === max2) h = (b - r) / s + 2;
      else h = (r - g) / s + 4;
      s /= l < 0.5 ? max2 + min2 : 2 - max2 - min2;
      h *= 60;
    } else {
      s = l > 0 && l < 1 ? 0 : h;
    }
    return new Hsl(h, s, l, o.opacity);
  }
  function hsl(h, s, l, opacity) {
    return arguments.length === 1 ? hslConvert(h) : new Hsl(h, s, l, opacity == null ? 1 : opacity);
  }
  function Hsl(h, s, l, opacity) {
    this.h = +h;
    this.s = +s;
    this.l = +l;
    this.opacity = +opacity;
  }
  define_default(Hsl, hsl, extend(Color, {
    brighter(k) {
      k = k == null ? brighter : Math.pow(brighter, k);
      return new Hsl(this.h, this.s, this.l * k, this.opacity);
    },
    darker(k) {
      k = k == null ? darker : Math.pow(darker, k);
      return new Hsl(this.h, this.s, this.l * k, this.opacity);
    },
    rgb() {
      var h = this.h % 360 + (this.h < 0) * 360, s = isNaN(h) || isNaN(this.s) ? 0 : this.s, l = this.l, m2 = l + (l < 0.5 ? l : 1 - l) * s, m1 = 2 * l - m2;
      return new Rgb(
        hsl2rgb(h >= 240 ? h - 240 : h + 120, m1, m2),
        hsl2rgb(h, m1, m2),
        hsl2rgb(h < 120 ? h + 240 : h - 120, m1, m2),
        this.opacity
      );
    },
    clamp() {
      return new Hsl(clamph(this.h), clampt(this.s), clampt(this.l), clampa(this.opacity));
    },
    displayable() {
      return (0 <= this.s && this.s <= 1 || isNaN(this.s)) && (0 <= this.l && this.l <= 1) && (0 <= this.opacity && this.opacity <= 1);
    },
    formatHsl() {
      const a = clampa(this.opacity);
      return `${a === 1 ? "hsl(" : "hsla("}${clamph(this.h)}, ${clampt(this.s) * 100}%, ${clampt(this.l) * 100}%${a === 1 ? ")" : `, ${a})`}`;
    }
  }));
  function clamph(value) {
    value = (value || 0) % 360;
    return value < 0 ? value + 360 : value;
  }
  function clampt(value) {
    return Math.max(0, Math.min(1, value || 0));
  }
  function hsl2rgb(h, m1, m2) {
    return (h < 60 ? m1 + (m2 - m1) * h / 60 : h < 180 ? m2 : h < 240 ? m1 + (m2 - m1) * (240 - h) / 60 : m1) * 255;
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/basis.js
  function basis(t1, v0, v1, v2, v3) {
    var t2 = t1 * t1, t3 = t2 * t1;
    return ((1 - 3 * t1 + 3 * t2 - t3) * v0 + (4 - 6 * t2 + 3 * t3) * v1 + (1 + 3 * t1 + 3 * t2 - 3 * t3) * v2 + t3 * v3) / 6;
  }
  function basis_default(values) {
    var n = values.length - 1;
    return function(t) {
      var i = t <= 0 ? t = 0 : t >= 1 ? (t = 1, n - 1) : Math.floor(t * n), v1 = values[i], v2 = values[i + 1], v0 = i > 0 ? values[i - 1] : 2 * v1 - v2, v3 = i < n - 1 ? values[i + 2] : 2 * v2 - v1;
      return basis((t - i / n) * n, v0, v1, v2, v3);
    };
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/basisClosed.js
  function basisClosed_default(values) {
    var n = values.length;
    return function(t) {
      var i = Math.floor(((t %= 1) < 0 ? ++t : t) * n), v0 = values[(i + n - 1) % n], v1 = values[i % n], v2 = values[(i + 1) % n], v3 = values[(i + 2) % n];
      return basis((t - i / n) * n, v0, v1, v2, v3);
    };
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/constant.js
  var constant_default2 = (x2) => () => x2;

  // shared/diagram-renderer/node_modules/d3-interpolate/src/color.js
  function linear(a, d) {
    return function(t) {
      return a + t * d;
    };
  }
  function exponential(a, b, y2) {
    return a = Math.pow(a, y2), b = Math.pow(b, y2) - a, y2 = 1 / y2, function(t) {
      return Math.pow(a + t * b, y2);
    };
  }
  function gamma(y2) {
    return (y2 = +y2) === 1 ? nogamma : function(a, b) {
      return b - a ? exponential(a, b, y2) : constant_default2(isNaN(a) ? b : a);
    };
  }
  function nogamma(a, b) {
    var d = b - a;
    return d ? linear(a, d) : constant_default2(isNaN(a) ? b : a);
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/rgb.js
  var rgb_default = function rgbGamma(y2) {
    var color2 = gamma(y2);
    function rgb2(start2, end) {
      var r = color2((start2 = rgb(start2)).r, (end = rgb(end)).r), g = color2(start2.g, end.g), b = color2(start2.b, end.b), opacity = nogamma(start2.opacity, end.opacity);
      return function(t) {
        start2.r = r(t);
        start2.g = g(t);
        start2.b = b(t);
        start2.opacity = opacity(t);
        return start2 + "";
      };
    }
    rgb2.gamma = rgbGamma;
    return rgb2;
  }(1);
  function rgbSpline(spline) {
    return function(colors) {
      var n = colors.length, r = new Array(n), g = new Array(n), b = new Array(n), i, color2;
      for (i = 0; i < n; ++i) {
        color2 = rgb(colors[i]);
        r[i] = color2.r || 0;
        g[i] = color2.g || 0;
        b[i] = color2.b || 0;
      }
      r = spline(r);
      g = spline(g);
      b = spline(b);
      color2.opacity = 1;
      return function(t) {
        color2.r = r(t);
        color2.g = g(t);
        color2.b = b(t);
        return color2 + "";
      };
    };
  }
  var rgbBasis = rgbSpline(basis_default);
  var rgbBasisClosed = rgbSpline(basisClosed_default);

  // shared/diagram-renderer/node_modules/d3-interpolate/src/number.js
  function number_default(a, b) {
    return a = +a, b = +b, function(t) {
      return a * (1 - t) + b * t;
    };
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/string.js
  var reA = /[-+]?(?:\d+\.?\d*|\.?\d+)(?:[eE][-+]?\d+)?/g;
  var reB = new RegExp(reA.source, "g");
  function zero(b) {
    return function() {
      return b;
    };
  }
  function one(b) {
    return function(t) {
      return b(t) + "";
    };
  }
  function string_default(a, b) {
    var bi = reA.lastIndex = reB.lastIndex = 0, am, bm, bs, i = -1, s = [], q = [];
    a = a + "", b = b + "";
    while ((am = reA.exec(a)) && (bm = reB.exec(b))) {
      if ((bs = bm.index) > bi) {
        bs = b.slice(bi, bs);
        if (s[i]) s[i] += bs;
        else s[++i] = bs;
      }
      if ((am = am[0]) === (bm = bm[0])) {
        if (s[i]) s[i] += bm;
        else s[++i] = bm;
      } else {
        s[++i] = null;
        q.push({ i, x: number_default(am, bm) });
      }
      bi = reB.lastIndex;
    }
    if (bi < b.length) {
      bs = b.slice(bi);
      if (s[i]) s[i] += bs;
      else s[++i] = bs;
    }
    return s.length < 2 ? q[0] ? one(q[0].x) : zero(b) : (b = q.length, function(t) {
      for (var i2 = 0, o; i2 < b; ++i2) s[(o = q[i2]).i] = o.x(t);
      return s.join("");
    });
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/transform/decompose.js
  var degrees = 180 / Math.PI;
  var identity = {
    translateX: 0,
    translateY: 0,
    rotate: 0,
    skewX: 0,
    scaleX: 1,
    scaleY: 1
  };
  function decompose_default(a, b, c, d, e, f) {
    var scaleX, scaleY, skewX;
    if (scaleX = Math.sqrt(a * a + b * b)) a /= scaleX, b /= scaleX;
    if (skewX = a * c + b * d) c -= a * skewX, d -= b * skewX;
    if (scaleY = Math.sqrt(c * c + d * d)) c /= scaleY, d /= scaleY, skewX /= scaleY;
    if (a * d < b * c) a = -a, b = -b, skewX = -skewX, scaleX = -scaleX;
    return {
      translateX: e,
      translateY: f,
      rotate: Math.atan2(b, a) * degrees,
      skewX: Math.atan(skewX) * degrees,
      scaleX,
      scaleY
    };
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/transform/parse.js
  var svgNode;
  function parseCss(value) {
    const m = new (typeof DOMMatrix === "function" ? DOMMatrix : WebKitCSSMatrix)(value + "");
    return m.isIdentity ? identity : decompose_default(m.a, m.b, m.c, m.d, m.e, m.f);
  }
  function parseSvg(value) {
    if (value == null) return identity;
    if (!svgNode) svgNode = document.createElementNS("http://www.w3.org/2000/svg", "g");
    svgNode.setAttribute("transform", value);
    if (!(value = svgNode.transform.baseVal.consolidate())) return identity;
    value = value.matrix;
    return decompose_default(value.a, value.b, value.c, value.d, value.e, value.f);
  }

  // shared/diagram-renderer/node_modules/d3-interpolate/src/transform/index.js
  function interpolateTransform(parse, pxComma, pxParen, degParen) {
    function pop(s) {
      return s.length ? s.pop() + " " : "";
    }
    function translate(xa, ya, xb, yb, s, q) {
      if (xa !== xb || ya !== yb) {
        var i = s.push("translate(", null, pxComma, null, pxParen);
        q.push({ i: i - 4, x: number_default(xa, xb) }, { i: i - 2, x: number_default(ya, yb) });
      } else if (xb || yb) {
        s.push("translate(" + xb + pxComma + yb + pxParen);
      }
    }
    function rotate(a, b, s, q) {
      if (a !== b) {
        if (a - b > 180) b += 360;
        else if (b - a > 180) a += 360;
        q.push({ i: s.push(pop(s) + "rotate(", null, degParen) - 2, x: number_default(a, b) });
      } else if (b) {
        s.push(pop(s) + "rotate(" + b + degParen);
      }
    }
    function skewX(a, b, s, q) {
      if (a !== b) {
        q.push({ i: s.push(pop(s) + "skewX(", null, degParen) - 2, x: number_default(a, b) });
      } else if (b) {
        s.push(pop(s) + "skewX(" + b + degParen);
      }
    }
    function scale(xa, ya, xb, yb, s, q) {
      if (xa !== xb || ya !== yb) {
        var i = s.push(pop(s) + "scale(", null, ",", null, ")");
        q.push({ i: i - 4, x: number_default(xa, xb) }, { i: i - 2, x: number_default(ya, yb) });
      } else if (xb !== 1 || yb !== 1) {
        s.push(pop(s) + "scale(" + xb + "," + yb + ")");
      }
    }
    return function(a, b) {
      var s = [], q = [];
      a = parse(a), b = parse(b);
      translate(a.translateX, a.translateY, b.translateX, b.translateY, s, q);
      rotate(a.rotate, b.rotate, s, q);
      skewX(a.skewX, b.skewX, s, q);
      scale(a.scaleX, a.scaleY, b.scaleX, b.scaleY, s, q);
      a = b = null;
      return function(t) {
        var i = -1, n = q.length, o;
        while (++i < n) s[(o = q[i]).i] = o.x(t);
        return s.join("");
      };
    };
  }
  var interpolateTransformCss = interpolateTransform(parseCss, "px, ", "px)", "deg)");
  var interpolateTransformSvg = interpolateTransform(parseSvg, ", ", ")", ")");

  // shared/diagram-renderer/node_modules/d3-interpolate/src/zoom.js
  var epsilon2 = 1e-12;
  function cosh(x2) {
    return ((x2 = Math.exp(x2)) + 1 / x2) / 2;
  }
  function sinh(x2) {
    return ((x2 = Math.exp(x2)) - 1 / x2) / 2;
  }
  function tanh(x2) {
    return ((x2 = Math.exp(2 * x2)) - 1) / (x2 + 1);
  }
  var zoom_default = function zoomRho(rho, rho2, rho4) {
    function zoom(p0, p1) {
      var ux0 = p0[0], uy0 = p0[1], w0 = p0[2], ux1 = p1[0], uy1 = p1[1], w1 = p1[2], dx = ux1 - ux0, dy = uy1 - uy0, d2 = dx * dx + dy * dy, i, S;
      if (d2 < epsilon2) {
        S = Math.log(w1 / w0) / rho;
        i = function(t) {
          return [
            ux0 + t * dx,
            uy0 + t * dy,
            w0 * Math.exp(rho * t * S)
          ];
        };
      } else {
        var d1 = Math.sqrt(d2), b0 = (w1 * w1 - w0 * w0 + rho4 * d2) / (2 * w0 * rho2 * d1), b1 = (w1 * w1 - w0 * w0 - rho4 * d2) / (2 * w1 * rho2 * d1), r0 = Math.log(Math.sqrt(b0 * b0 + 1) - b0), r1 = Math.log(Math.sqrt(b1 * b1 + 1) - b1);
        S = (r1 - r0) / rho;
        i = function(t) {
          var s = t * S, coshr0 = cosh(r0), u = w0 / (rho2 * d1) * (coshr0 * tanh(rho * s + r0) - sinh(r0));
          return [
            ux0 + u * dx,
            uy0 + u * dy,
            w0 * coshr0 / cosh(rho * s + r0)
          ];
        };
      }
      i.duration = S * 1e3 * rho / Math.SQRT2;
      return i;
    }
    zoom.rho = function(_) {
      var _1 = Math.max(1e-3, +_), _2 = _1 * _1, _4 = _2 * _2;
      return zoomRho(_1, _2, _4);
    };
    return zoom;
  }(Math.SQRT2, 2, 4);

  // shared/diagram-renderer/node_modules/d3-timer/src/timer.js
  var frame = 0;
  var timeout = 0;
  var interval = 0;
  var pokeDelay = 1e3;
  var taskHead;
  var taskTail;
  var clockLast = 0;
  var clockNow = 0;
  var clockSkew = 0;
  var clock = typeof performance === "object" && performance.now ? performance : Date;
  var setFrame = typeof window === "object" && window.requestAnimationFrame ? window.requestAnimationFrame.bind(window) : function(f) {
    setTimeout(f, 17);
  };
  function now() {
    return clockNow || (setFrame(clearNow), clockNow = clock.now() + clockSkew);
  }
  function clearNow() {
    clockNow = 0;
  }
  function Timer() {
    this._call = this._time = this._next = null;
  }
  Timer.prototype = timer.prototype = {
    constructor: Timer,
    restart: function(callback, delay, time) {
      if (typeof callback !== "function") throw new TypeError("callback is not a function");
      time = (time == null ? now() : +time) + (delay == null ? 0 : +delay);
      if (!this._next && taskTail !== this) {
        if (taskTail) taskTail._next = this;
        else taskHead = this;
        taskTail = this;
      }
      this._call = callback;
      this._time = time;
      sleep();
    },
    stop: function() {
      if (this._call) {
        this._call = null;
        this._time = Infinity;
        sleep();
      }
    }
  };
  function timer(callback, delay, time) {
    var t = new Timer();
    t.restart(callback, delay, time);
    return t;
  }
  function timerFlush() {
    now();
    ++frame;
    var t = taskHead, e;
    while (t) {
      if ((e = clockNow - t._time) >= 0) t._call.call(void 0, e);
      t = t._next;
    }
    --frame;
  }
  function wake() {
    clockNow = (clockLast = clock.now()) + clockSkew;
    frame = timeout = 0;
    try {
      timerFlush();
    } finally {
      frame = 0;
      nap();
      clockNow = 0;
    }
  }
  function poke() {
    var now2 = clock.now(), delay = now2 - clockLast;
    if (delay > pokeDelay) clockSkew -= delay, clockLast = now2;
  }
  function nap() {
    var t0, t1 = taskHead, t2, time = Infinity;
    while (t1) {
      if (t1._call) {
        if (time > t1._time) time = t1._time;
        t0 = t1, t1 = t1._next;
      } else {
        t2 = t1._next, t1._next = null;
        t1 = t0 ? t0._next = t2 : taskHead = t2;
      }
    }
    taskTail = t0;
    sleep(time);
  }
  function sleep(time) {
    if (frame) return;
    if (timeout) timeout = clearTimeout(timeout);
    var delay = time - clockNow;
    if (delay > 24) {
      if (time < Infinity) timeout = setTimeout(wake, time - clock.now() - clockSkew);
      if (interval) interval = clearInterval(interval);
    } else {
      if (!interval) clockLast = clock.now(), interval = setInterval(poke, pokeDelay);
      frame = 1, setFrame(wake);
    }
  }

  // shared/diagram-renderer/node_modules/d3-timer/src/timeout.js
  function timeout_default(callback, delay, time) {
    var t = new Timer();
    delay = delay == null ? 0 : +delay;
    t.restart((elapsed) => {
      t.stop();
      callback(elapsed + delay);
    }, delay, time);
    return t;
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/schedule.js
  var emptyOn = dispatch_default("start", "end", "cancel", "interrupt");
  var emptyTween = [];
  var CREATED = 0;
  var SCHEDULED = 1;
  var STARTING = 2;
  var STARTED = 3;
  var RUNNING = 4;
  var ENDING = 5;
  var ENDED = 6;
  function schedule_default(node, name, id2, index, group, timing) {
    var schedules = node.__transition;
    if (!schedules) node.__transition = {};
    else if (id2 in schedules) return;
    create(node, id2, {
      name,
      index,
      // For context during callback.
      group,
      // For context during callback.
      on: emptyOn,
      tween: emptyTween,
      time: timing.time,
      delay: timing.delay,
      duration: timing.duration,
      ease: timing.ease,
      timer: null,
      state: CREATED
    });
  }
  function init(node, id2) {
    var schedule = get2(node, id2);
    if (schedule.state > CREATED) throw new Error("too late; already scheduled");
    return schedule;
  }
  function set2(node, id2) {
    var schedule = get2(node, id2);
    if (schedule.state > STARTED) throw new Error("too late; already running");
    return schedule;
  }
  function get2(node, id2) {
    var schedule = node.__transition;
    if (!schedule || !(schedule = schedule[id2])) throw new Error("transition not found");
    return schedule;
  }
  function create(node, id2, self) {
    var schedules = node.__transition, tween;
    schedules[id2] = self;
    self.timer = timer(schedule, 0, self.time);
    function schedule(elapsed) {
      self.state = SCHEDULED;
      self.timer.restart(start2, self.delay, self.time);
      if (self.delay <= elapsed) start2(elapsed - self.delay);
    }
    function start2(elapsed) {
      var i, j, n, o;
      if (self.state !== SCHEDULED) return stop();
      for (i in schedules) {
        o = schedules[i];
        if (o.name !== self.name) continue;
        if (o.state === STARTED) return timeout_default(start2);
        if (o.state === RUNNING) {
          o.state = ENDED;
          o.timer.stop();
          o.on.call("interrupt", node, node.__data__, o.index, o.group);
          delete schedules[i];
        } else if (+i < id2) {
          o.state = ENDED;
          o.timer.stop();
          o.on.call("cancel", node, node.__data__, o.index, o.group);
          delete schedules[i];
        }
      }
      timeout_default(function() {
        if (self.state === STARTED) {
          self.state = RUNNING;
          self.timer.restart(tick, self.delay, self.time);
          tick(elapsed);
        }
      });
      self.state = STARTING;
      self.on.call("start", node, node.__data__, self.index, self.group);
      if (self.state !== STARTING) return;
      self.state = STARTED;
      tween = new Array(n = self.tween.length);
      for (i = 0, j = -1; i < n; ++i) {
        if (o = self.tween[i].value.call(node, node.__data__, self.index, self.group)) {
          tween[++j] = o;
        }
      }
      tween.length = j + 1;
    }
    function tick(elapsed) {
      var t = elapsed < self.duration ? self.ease.call(null, elapsed / self.duration) : (self.timer.restart(stop), self.state = ENDING, 1), i = -1, n = tween.length;
      while (++i < n) {
        tween[i].call(node, t);
      }
      if (self.state === ENDING) {
        self.on.call("end", node, node.__data__, self.index, self.group);
        stop();
      }
    }
    function stop() {
      self.state = ENDED;
      self.timer.stop();
      delete schedules[id2];
      for (var i in schedules) return;
      delete node.__transition;
    }
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/interrupt.js
  function interrupt_default(node, name) {
    var schedules = node.__transition, schedule, active, empty2 = true, i;
    if (!schedules) return;
    name = name == null ? null : name + "";
    for (i in schedules) {
      if ((schedule = schedules[i]).name !== name) {
        empty2 = false;
        continue;
      }
      active = schedule.state > STARTING && schedule.state < ENDING;
      schedule.state = ENDED;
      schedule.timer.stop();
      schedule.on.call(active ? "interrupt" : "cancel", node, node.__data__, schedule.index, schedule.group);
      delete schedules[i];
    }
    if (empty2) delete node.__transition;
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/selection/interrupt.js
  function interrupt_default2(name) {
    return this.each(function() {
      interrupt_default(this, name);
    });
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/tween.js
  function tweenRemove(id2, name) {
    var tween0, tween1;
    return function() {
      var schedule = set2(this, id2), tween = schedule.tween;
      if (tween !== tween0) {
        tween1 = tween0 = tween;
        for (var i = 0, n = tween1.length; i < n; ++i) {
          if (tween1[i].name === name) {
            tween1 = tween1.slice();
            tween1.splice(i, 1);
            break;
          }
        }
      }
      schedule.tween = tween1;
    };
  }
  function tweenFunction(id2, name, value) {
    var tween0, tween1;
    if (typeof value !== "function") throw new Error();
    return function() {
      var schedule = set2(this, id2), tween = schedule.tween;
      if (tween !== tween0) {
        tween1 = (tween0 = tween).slice();
        for (var t = { name, value }, i = 0, n = tween1.length; i < n; ++i) {
          if (tween1[i].name === name) {
            tween1[i] = t;
            break;
          }
        }
        if (i === n) tween1.push(t);
      }
      schedule.tween = tween1;
    };
  }
  function tween_default(name, value) {
    var id2 = this._id;
    name += "";
    if (arguments.length < 2) {
      var tween = get2(this.node(), id2).tween;
      for (var i = 0, n = tween.length, t; i < n; ++i) {
        if ((t = tween[i]).name === name) {
          return t.value;
        }
      }
      return null;
    }
    return this.each((value == null ? tweenRemove : tweenFunction)(id2, name, value));
  }
  function tweenValue(transition2, name, value) {
    var id2 = transition2._id;
    transition2.each(function() {
      var schedule = set2(this, id2);
      (schedule.value || (schedule.value = {}))[name] = value.apply(this, arguments);
    });
    return function(node) {
      return get2(node, id2).value[name];
    };
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/interpolate.js
  function interpolate_default(a, b) {
    var c;
    return (typeof b === "number" ? number_default : b instanceof color ? rgb_default : (c = color(b)) ? (b = c, rgb_default) : string_default)(a, b);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/attr.js
  function attrRemove2(name) {
    return function() {
      this.removeAttribute(name);
    };
  }
  function attrRemoveNS2(fullname) {
    return function() {
      this.removeAttributeNS(fullname.space, fullname.local);
    };
  }
  function attrConstant2(name, interpolate, value1) {
    var string00, string1 = value1 + "", interpolate0;
    return function() {
      var string0 = this.getAttribute(name);
      return string0 === string1 ? null : string0 === string00 ? interpolate0 : interpolate0 = interpolate(string00 = string0, value1);
    };
  }
  function attrConstantNS2(fullname, interpolate, value1) {
    var string00, string1 = value1 + "", interpolate0;
    return function() {
      var string0 = this.getAttributeNS(fullname.space, fullname.local);
      return string0 === string1 ? null : string0 === string00 ? interpolate0 : interpolate0 = interpolate(string00 = string0, value1);
    };
  }
  function attrFunction2(name, interpolate, value) {
    var string00, string10, interpolate0;
    return function() {
      var string0, value1 = value(this), string1;
      if (value1 == null) return void this.removeAttribute(name);
      string0 = this.getAttribute(name);
      string1 = value1 + "";
      return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : (string10 = string1, interpolate0 = interpolate(string00 = string0, value1));
    };
  }
  function attrFunctionNS2(fullname, interpolate, value) {
    var string00, string10, interpolate0;
    return function() {
      var string0, value1 = value(this), string1;
      if (value1 == null) return void this.removeAttributeNS(fullname.space, fullname.local);
      string0 = this.getAttributeNS(fullname.space, fullname.local);
      string1 = value1 + "";
      return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : (string10 = string1, interpolate0 = interpolate(string00 = string0, value1));
    };
  }
  function attr_default2(name, value) {
    var fullname = namespace_default(name), i = fullname === "transform" ? interpolateTransformSvg : interpolate_default;
    return this.attrTween(name, typeof value === "function" ? (fullname.local ? attrFunctionNS2 : attrFunction2)(fullname, i, tweenValue(this, "attr." + name, value)) : value == null ? (fullname.local ? attrRemoveNS2 : attrRemove2)(fullname) : (fullname.local ? attrConstantNS2 : attrConstant2)(fullname, i, value));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/attrTween.js
  function attrInterpolate(name, i) {
    return function(t) {
      this.setAttribute(name, i.call(this, t));
    };
  }
  function attrInterpolateNS(fullname, i) {
    return function(t) {
      this.setAttributeNS(fullname.space, fullname.local, i.call(this, t));
    };
  }
  function attrTweenNS(fullname, value) {
    var t0, i0;
    function tween() {
      var i = value.apply(this, arguments);
      if (i !== i0) t0 = (i0 = i) && attrInterpolateNS(fullname, i);
      return t0;
    }
    tween._value = value;
    return tween;
  }
  function attrTween(name, value) {
    var t0, i0;
    function tween() {
      var i = value.apply(this, arguments);
      if (i !== i0) t0 = (i0 = i) && attrInterpolate(name, i);
      return t0;
    }
    tween._value = value;
    return tween;
  }
  function attrTween_default(name, value) {
    var key = "attr." + name;
    if (arguments.length < 2) return (key = this.tween(key)) && key._value;
    if (value == null) return this.tween(key, null);
    if (typeof value !== "function") throw new Error();
    var fullname = namespace_default(name);
    return this.tween(key, (fullname.local ? attrTweenNS : attrTween)(fullname, value));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/delay.js
  function delayFunction(id2, value) {
    return function() {
      init(this, id2).delay = +value.apply(this, arguments);
    };
  }
  function delayConstant(id2, value) {
    return value = +value, function() {
      init(this, id2).delay = value;
    };
  }
  function delay_default(value) {
    var id2 = this._id;
    return arguments.length ? this.each((typeof value === "function" ? delayFunction : delayConstant)(id2, value)) : get2(this.node(), id2).delay;
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/duration.js
  function durationFunction(id2, value) {
    return function() {
      set2(this, id2).duration = +value.apply(this, arguments);
    };
  }
  function durationConstant(id2, value) {
    return value = +value, function() {
      set2(this, id2).duration = value;
    };
  }
  function duration_default(value) {
    var id2 = this._id;
    return arguments.length ? this.each((typeof value === "function" ? durationFunction : durationConstant)(id2, value)) : get2(this.node(), id2).duration;
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/ease.js
  function easeConstant(id2, value) {
    if (typeof value !== "function") throw new Error();
    return function() {
      set2(this, id2).ease = value;
    };
  }
  function ease_default(value) {
    var id2 = this._id;
    return arguments.length ? this.each(easeConstant(id2, value)) : get2(this.node(), id2).ease;
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/easeVarying.js
  function easeVarying(id2, value) {
    return function() {
      var v = value.apply(this, arguments);
      if (typeof v !== "function") throw new Error();
      set2(this, id2).ease = v;
    };
  }
  function easeVarying_default(value) {
    if (typeof value !== "function") throw new Error();
    return this.each(easeVarying(this._id, value));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/filter.js
  function filter_default2(match) {
    if (typeof match !== "function") match = matcher_default(match);
    for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, subgroup = subgroups[j] = [], node, i = 0; i < n; ++i) {
        if ((node = group[i]) && match.call(node, node.__data__, i, group)) {
          subgroup.push(node);
        }
      }
    }
    return new Transition(subgroups, this._parents, this._name, this._id);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/merge.js
  function merge_default2(transition2) {
    if (transition2._id !== this._id) throw new Error();
    for (var groups0 = this._groups, groups1 = transition2._groups, m0 = groups0.length, m1 = groups1.length, m = Math.min(m0, m1), merges = new Array(m0), j = 0; j < m; ++j) {
      for (var group0 = groups0[j], group1 = groups1[j], n = group0.length, merge = merges[j] = new Array(n), node, i = 0; i < n; ++i) {
        if (node = group0[i] || group1[i]) {
          merge[i] = node;
        }
      }
    }
    for (; j < m0; ++j) {
      merges[j] = groups0[j];
    }
    return new Transition(merges, this._parents, this._name, this._id);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/on.js
  function start(name) {
    return (name + "").trim().split(/^|\s+/).every(function(t) {
      var i = t.indexOf(".");
      if (i >= 0) t = t.slice(0, i);
      return !t || t === "start";
    });
  }
  function onFunction(id2, name, listener) {
    var on0, on1, sit = start(name) ? init : set2;
    return function() {
      var schedule = sit(this, id2), on = schedule.on;
      if (on !== on0) (on1 = (on0 = on).copy()).on(name, listener);
      schedule.on = on1;
    };
  }
  function on_default2(name, listener) {
    var id2 = this._id;
    return arguments.length < 2 ? get2(this.node(), id2).on.on(name) : this.each(onFunction(id2, name, listener));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/remove.js
  function removeFunction(id2) {
    return function() {
      var parent = this.parentNode;
      for (var i in this.__transition) if (+i !== id2) return;
      if (parent) parent.removeChild(this);
    };
  }
  function remove_default2() {
    return this.on("end.remove", removeFunction(this._id));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/select.js
  function select_default3(select) {
    var name = this._name, id2 = this._id;
    if (typeof select !== "function") select = selector_default(select);
    for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, subgroup = subgroups[j] = new Array(n), node, subnode, i = 0; i < n; ++i) {
        if ((node = group[i]) && (subnode = select.call(node, node.__data__, i, group))) {
          if ("__data__" in node) subnode.__data__ = node.__data__;
          subgroup[i] = subnode;
          schedule_default(subgroup[i], name, id2, i, subgroup, get2(node, id2));
        }
      }
    }
    return new Transition(subgroups, this._parents, name, id2);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/selectAll.js
  function selectAll_default2(select) {
    var name = this._name, id2 = this._id;
    if (typeof select !== "function") select = selectorAll_default(select);
    for (var groups = this._groups, m = groups.length, subgroups = [], parents = [], j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
        if (node = group[i]) {
          for (var children2 = select.call(node, node.__data__, i, group), child, inherit2 = get2(node, id2), k = 0, l = children2.length; k < l; ++k) {
            if (child = children2[k]) {
              schedule_default(child, name, id2, k, children2, inherit2);
            }
          }
          subgroups.push(children2);
          parents.push(node);
        }
      }
    }
    return new Transition(subgroups, parents, name, id2);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/selection.js
  var Selection2 = selection_default.prototype.constructor;
  function selection_default2() {
    return new Selection2(this._groups, this._parents);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/style.js
  function styleNull(name, interpolate) {
    var string00, string10, interpolate0;
    return function() {
      var string0 = styleValue(this, name), string1 = (this.style.removeProperty(name), styleValue(this, name));
      return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : interpolate0 = interpolate(string00 = string0, string10 = string1);
    };
  }
  function styleRemove2(name) {
    return function() {
      this.style.removeProperty(name);
    };
  }
  function styleConstant2(name, interpolate, value1) {
    var string00, string1 = value1 + "", interpolate0;
    return function() {
      var string0 = styleValue(this, name);
      return string0 === string1 ? null : string0 === string00 ? interpolate0 : interpolate0 = interpolate(string00 = string0, value1);
    };
  }
  function styleFunction2(name, interpolate, value) {
    var string00, string10, interpolate0;
    return function() {
      var string0 = styleValue(this, name), value1 = value(this), string1 = value1 + "";
      if (value1 == null) string1 = value1 = (this.style.removeProperty(name), styleValue(this, name));
      return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : (string10 = string1, interpolate0 = interpolate(string00 = string0, value1));
    };
  }
  function styleMaybeRemove(id2, name) {
    var on0, on1, listener0, key = "style." + name, event = "end." + key, remove2;
    return function() {
      var schedule = set2(this, id2), on = schedule.on, listener = schedule.value[key] == null ? remove2 || (remove2 = styleRemove2(name)) : void 0;
      if (on !== on0 || listener0 !== listener) (on1 = (on0 = on).copy()).on(event, listener0 = listener);
      schedule.on = on1;
    };
  }
  function style_default2(name, value, priority) {
    var i = (name += "") === "transform" ? interpolateTransformCss : interpolate_default;
    return value == null ? this.styleTween(name, styleNull(name, i)).on("end.style." + name, styleRemove2(name)) : typeof value === "function" ? this.styleTween(name, styleFunction2(name, i, tweenValue(this, "style." + name, value))).each(styleMaybeRemove(this._id, name)) : this.styleTween(name, styleConstant2(name, i, value), priority).on("end.style." + name, null);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/styleTween.js
  function styleInterpolate(name, i, priority) {
    return function(t) {
      this.style.setProperty(name, i.call(this, t), priority);
    };
  }
  function styleTween(name, value, priority) {
    var t, i0;
    function tween() {
      var i = value.apply(this, arguments);
      if (i !== i0) t = (i0 = i) && styleInterpolate(name, i, priority);
      return t;
    }
    tween._value = value;
    return tween;
  }
  function styleTween_default(name, value, priority) {
    var key = "style." + (name += "");
    if (arguments.length < 2) return (key = this.tween(key)) && key._value;
    if (value == null) return this.tween(key, null);
    if (typeof value !== "function") throw new Error();
    return this.tween(key, styleTween(name, value, priority == null ? "" : priority));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/text.js
  function textConstant2(value) {
    return function() {
      this.textContent = value;
    };
  }
  function textFunction2(value) {
    return function() {
      var value1 = value(this);
      this.textContent = value1 == null ? "" : value1;
    };
  }
  function text_default2(value) {
    return this.tween("text", typeof value === "function" ? textFunction2(tweenValue(this, "text", value)) : textConstant2(value == null ? "" : value + ""));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/textTween.js
  function textInterpolate(i) {
    return function(t) {
      this.textContent = i.call(this, t);
    };
  }
  function textTween(value) {
    var t0, i0;
    function tween() {
      var i = value.apply(this, arguments);
      if (i !== i0) t0 = (i0 = i) && textInterpolate(i);
      return t0;
    }
    tween._value = value;
    return tween;
  }
  function textTween_default(value) {
    var key = "text";
    if (arguments.length < 1) return (key = this.tween(key)) && key._value;
    if (value == null) return this.tween(key, null);
    if (typeof value !== "function") throw new Error();
    return this.tween(key, textTween(value));
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/transition.js
  function transition_default() {
    var name = this._name, id0 = this._id, id1 = newId();
    for (var groups = this._groups, m = groups.length, j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
        if (node = group[i]) {
          var inherit2 = get2(node, id0);
          schedule_default(node, name, id1, i, group, {
            time: inherit2.time + inherit2.delay + inherit2.duration,
            delay: 0,
            duration: inherit2.duration,
            ease: inherit2.ease
          });
        }
      }
    }
    return new Transition(groups, this._parents, name, id1);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/end.js
  function end_default() {
    var on0, on1, that = this, id2 = that._id, size = that.size();
    return new Promise(function(resolve, reject) {
      var cancel = { value: reject }, end = { value: function() {
        if (--size === 0) resolve();
      } };
      that.each(function() {
        var schedule = set2(this, id2), on = schedule.on;
        if (on !== on0) {
          on1 = (on0 = on).copy();
          on1._.cancel.push(cancel);
          on1._.interrupt.push(cancel);
          on1._.end.push(end);
        }
        schedule.on = on1;
      });
      if (size === 0) resolve();
    });
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/transition/index.js
  var id = 0;
  function Transition(groups, parents, name, id2) {
    this._groups = groups;
    this._parents = parents;
    this._name = name;
    this._id = id2;
  }
  function transition(name) {
    return selection_default().transition(name);
  }
  function newId() {
    return ++id;
  }
  var selection_prototype = selection_default.prototype;
  Transition.prototype = transition.prototype = {
    constructor: Transition,
    select: select_default3,
    selectAll: selectAll_default2,
    selectChild: selection_prototype.selectChild,
    selectChildren: selection_prototype.selectChildren,
    filter: filter_default2,
    merge: merge_default2,
    selection: selection_default2,
    transition: transition_default,
    call: selection_prototype.call,
    nodes: selection_prototype.nodes,
    node: selection_prototype.node,
    size: selection_prototype.size,
    empty: selection_prototype.empty,
    each: selection_prototype.each,
    on: on_default2,
    attr: attr_default2,
    attrTween: attrTween_default,
    style: style_default2,
    styleTween: styleTween_default,
    text: text_default2,
    textTween: textTween_default,
    remove: remove_default2,
    tween: tween_default,
    delay: delay_default,
    duration: duration_default,
    ease: ease_default,
    easeVarying: easeVarying_default,
    end: end_default,
    [Symbol.iterator]: selection_prototype[Symbol.iterator]
  };

  // shared/diagram-renderer/node_modules/d3-ease/src/cubic.js
  function cubicInOut(t) {
    return ((t *= 2) <= 1 ? t * t * t : (t -= 2) * t * t + 2) / 2;
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/selection/transition.js
  var defaultTiming = {
    time: null,
    // Set on use.
    delay: 0,
    duration: 250,
    ease: cubicInOut
  };
  function inherit(node, id2) {
    var timing;
    while (!(timing = node.__transition) || !(timing = timing[id2])) {
      if (!(node = node.parentNode)) {
        throw new Error(`transition ${id2} not found`);
      }
    }
    return timing;
  }
  function transition_default2(name) {
    var id2, timing;
    if (name instanceof Transition) {
      id2 = name._id, name = name._name;
    } else {
      id2 = newId(), (timing = defaultTiming).time = now(), name = name == null ? null : name + "";
    }
    for (var groups = this._groups, m = groups.length, j = 0; j < m; ++j) {
      for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
        if (node = group[i]) {
          schedule_default(node, name, id2, i, group, timing || inherit(node, id2));
        }
      }
    }
    return new Transition(groups, this._parents, name, id2);
  }

  // shared/diagram-renderer/node_modules/d3-transition/src/selection/index.js
  selection_default.prototype.interrupt = interrupt_default2;
  selection_default.prototype.transition = transition_default2;

  // shared/diagram-renderer/node_modules/d3-brush/src/brush.js
  var { abs, max, min } = Math;
  function number1(e) {
    return [+e[0], +e[1]];
  }
  function number2(e) {
    return [number1(e[0]), number1(e[1])];
  }
  var X = {
    name: "x",
    handles: ["w", "e"].map(type),
    input: function(x2, e) {
      return x2 == null ? null : [[+x2[0], e[0][1]], [+x2[1], e[1][1]]];
    },
    output: function(xy) {
      return xy && [xy[0][0], xy[1][0]];
    }
  };
  var Y = {
    name: "y",
    handles: ["n", "s"].map(type),
    input: function(y2, e) {
      return y2 == null ? null : [[e[0][0], +y2[0]], [e[1][0], +y2[1]]];
    },
    output: function(xy) {
      return xy && [xy[0][1], xy[1][1]];
    }
  };
  var XY = {
    name: "xy",
    handles: ["n", "w", "e", "s", "nw", "ne", "sw", "se"].map(type),
    input: function(xy) {
      return xy == null ? null : number2(xy);
    },
    output: function(xy) {
      return xy;
    }
  };
  function type(t) {
    return { type: t };
  }

  // shared/diagram-renderer/node_modules/d3-path/src/path.js
  var pi = Math.PI;
  var tau = 2 * pi;
  var epsilon = 1e-6;
  var tauEpsilon = tau - epsilon;
  function append(strings) {
    this._ += strings[0];
    for (let i = 1, n = strings.length; i < n; ++i) {
      this._ += arguments[i] + strings[i];
    }
  }
  function appendRound(digits) {
    let d = Math.floor(digits);
    if (!(d >= 0)) throw new Error(`invalid digits: ${digits}`);
    if (d > 15) return append;
    const k = 10 ** d;
    return function(strings) {
      this._ += strings[0];
      for (let i = 1, n = strings.length; i < n; ++i) {
        this._ += Math.round(arguments[i] * k) / k + strings[i];
      }
    };
  }
  var Path = class {
    constructor(digits) {
      this._x0 = this._y0 = // start of current subpath
      this._x1 = this._y1 = null;
      this._ = "";
      this._append = digits == null ? append : appendRound(digits);
    }
    moveTo(x2, y2) {
      this._append`M${this._x0 = this._x1 = +x2},${this._y0 = this._y1 = +y2}`;
    }
    closePath() {
      if (this._x1 !== null) {
        this._x1 = this._x0, this._y1 = this._y0;
        this._append`Z`;
      }
    }
    lineTo(x2, y2) {
      this._append`L${this._x1 = +x2},${this._y1 = +y2}`;
    }
    quadraticCurveTo(x1, y1, x2, y2) {
      this._append`Q${+x1},${+y1},${this._x1 = +x2},${this._y1 = +y2}`;
    }
    bezierCurveTo(x1, y1, x2, y2, x3, y3) {
      this._append`C${+x1},${+y1},${+x2},${+y2},${this._x1 = +x3},${this._y1 = +y3}`;
    }
    arcTo(x1, y1, x2, y2, r) {
      x1 = +x1, y1 = +y1, x2 = +x2, y2 = +y2, r = +r;
      if (r < 0) throw new Error(`negative radius: ${r}`);
      let x0 = this._x1, y0 = this._y1, x21 = x2 - x1, y21 = y2 - y1, x01 = x0 - x1, y01 = y0 - y1, l01_2 = x01 * x01 + y01 * y01;
      if (this._x1 === null) {
        this._append`M${this._x1 = x1},${this._y1 = y1}`;
      } else if (!(l01_2 > epsilon)) ;
      else if (!(Math.abs(y01 * x21 - y21 * x01) > epsilon) || !r) {
        this._append`L${this._x1 = x1},${this._y1 = y1}`;
      } else {
        let x20 = x2 - x0, y20 = y2 - y0, l21_2 = x21 * x21 + y21 * y21, l20_2 = x20 * x20 + y20 * y20, l21 = Math.sqrt(l21_2), l01 = Math.sqrt(l01_2), l = r * Math.tan((pi - Math.acos((l21_2 + l01_2 - l20_2) / (2 * l21 * l01))) / 2), t01 = l / l01, t21 = l / l21;
        if (Math.abs(t01 - 1) > epsilon) {
          this._append`L${x1 + t01 * x01},${y1 + t01 * y01}`;
        }
        this._append`A${r},${r},0,0,${+(y01 * x20 > x01 * y20)},${this._x1 = x1 + t21 * x21},${this._y1 = y1 + t21 * y21}`;
      }
    }
    arc(x2, y2, r, a0, a1, ccw) {
      x2 = +x2, y2 = +y2, r = +r, ccw = !!ccw;
      if (r < 0) throw new Error(`negative radius: ${r}`);
      let dx = r * Math.cos(a0), dy = r * Math.sin(a0), x0 = x2 + dx, y0 = y2 + dy, cw = 1 ^ ccw, da = ccw ? a0 - a1 : a1 - a0;
      if (this._x1 === null) {
        this._append`M${x0},${y0}`;
      } else if (Math.abs(this._x1 - x0) > epsilon || Math.abs(this._y1 - y0) > epsilon) {
        this._append`L${x0},${y0}`;
      }
      if (!r) return;
      if (da < 0) da = da % tau + tau;
      if (da > tauEpsilon) {
        this._append`A${r},${r},0,1,${cw},${x2 - dx},${y2 - dy}A${r},${r},0,1,${cw},${this._x1 = x0},${this._y1 = y0}`;
      } else if (da > epsilon) {
        this._append`A${r},${r},0,${+(da >= pi)},${cw},${this._x1 = x2 + r * Math.cos(a1)},${this._y1 = y2 + r * Math.sin(a1)}`;
      }
    }
    rect(x2, y2, w, h) {
      this._append`M${this._x0 = this._x1 = +x2},${this._y0 = this._y1 = +y2}h${w = +w}v${+h}h${-w}Z`;
    }
    toString() {
      return this._;
    }
  };
  function path() {
    return new Path();
  }
  path.prototype = Path.prototype;

  // shared/diagram-renderer/node_modules/d3-shape/src/constant.js
  function constant_default4(x2) {
    return function constant() {
      return x2;
    };
  }

  // shared/diagram-renderer/node_modules/d3-shape/src/path.js
  function withPath(shape) {
    let digits = 3;
    shape.digits = function(_) {
      if (!arguments.length) return digits;
      if (_ == null) {
        digits = null;
      } else {
        const d = Math.floor(_);
        if (!(d >= 0)) throw new RangeError(`invalid digits: ${_}`);
        digits = d;
      }
      return shape;
    };
    return () => new Path(digits);
  }

  // shared/diagram-renderer/node_modules/d3-shape/src/array.js
  var slice = Array.prototype.slice;
  function array_default(x2) {
    return typeof x2 === "object" && "length" in x2 ? x2 : Array.from(x2);
  }

  // shared/diagram-renderer/node_modules/d3-shape/src/curve/linear.js
  function Linear(context) {
    this._context = context;
  }
  Linear.prototype = {
    areaStart: function() {
      this._line = 0;
    },
    areaEnd: function() {
      this._line = NaN;
    },
    lineStart: function() {
      this._point = 0;
    },
    lineEnd: function() {
      if (this._line || this._line !== 0 && this._point === 1) this._context.closePath();
      this._line = 1 - this._line;
    },
    point: function(x2, y2) {
      x2 = +x2, y2 = +y2;
      switch (this._point) {
        case 0:
          this._point = 1;
          this._line ? this._context.lineTo(x2, y2) : this._context.moveTo(x2, y2);
          break;
        case 1:
          this._point = 2;
        // falls through
        default:
          this._context.lineTo(x2, y2);
          break;
      }
    }
  };
  function linear_default(context) {
    return new Linear(context);
  }

  // shared/diagram-renderer/node_modules/d3-shape/src/point.js
  function x(p) {
    return p[0];
  }
  function y(p) {
    return p[1];
  }

  // shared/diagram-renderer/node_modules/d3-shape/src/line.js
  function line_default(x2, y2) {
    var defined = constant_default4(true), context = null, curve = linear_default, output = null, path2 = withPath(line);
    x2 = typeof x2 === "function" ? x2 : x2 === void 0 ? x : constant_default4(x2);
    y2 = typeof y2 === "function" ? y2 : y2 === void 0 ? y : constant_default4(y2);
    function line(data) {
      var i, n = (data = array_default(data)).length, d, defined0 = false, buffer;
      if (context == null) output = curve(buffer = path2());
      for (i = 0; i <= n; ++i) {
        if (!(i < n && defined(d = data[i], i, data)) === defined0) {
          if (defined0 = !defined0) output.lineStart();
          else output.lineEnd();
        }
        if (defined0) output.point(+x2(d, i, data), +y2(d, i, data));
      }
      if (buffer) return output = null, buffer + "" || null;
    }
    line.x = function(_) {
      return arguments.length ? (x2 = typeof _ === "function" ? _ : constant_default4(+_), line) : x2;
    };
    line.y = function(_) {
      return arguments.length ? (y2 = typeof _ === "function" ? _ : constant_default4(+_), line) : y2;
    };
    line.defined = function(_) {
      return arguments.length ? (defined = typeof _ === "function" ? _ : constant_default4(!!_), line) : defined;
    };
    line.curve = function(_) {
      return arguments.length ? (curve = _, context != null && (output = curve(context)), line) : curve;
    };
    line.context = function(_) {
      return arguments.length ? (_ == null ? context = output = null : output = curve(context = _), line) : context;
    };
    return line;
  }

  // shared/diagram-renderer/node_modules/d3-zoom/src/constant.js
  var constant_default5 = (x2) => () => x2;

  // shared/diagram-renderer/node_modules/d3-zoom/src/event.js
  function ZoomEvent(type2, {
    sourceEvent,
    target,
    transform: transform2,
    dispatch: dispatch2
  }) {
    Object.defineProperties(this, {
      type: { value: type2, enumerable: true, configurable: true },
      sourceEvent: { value: sourceEvent, enumerable: true, configurable: true },
      target: { value: target, enumerable: true, configurable: true },
      transform: { value: transform2, enumerable: true, configurable: true },
      _: { value: dispatch2 }
    });
  }

  // shared/diagram-renderer/node_modules/d3-zoom/src/transform.js
  function Transform(k, x2, y2) {
    this.k = k;
    this.x = x2;
    this.y = y2;
  }
  Transform.prototype = {
    constructor: Transform,
    scale: function(k) {
      return k === 1 ? this : new Transform(this.k * k, this.x, this.y);
    },
    translate: function(x2, y2) {
      return x2 === 0 & y2 === 0 ? this : new Transform(this.k, this.x + this.k * x2, this.y + this.k * y2);
    },
    apply: function(point) {
      return [point[0] * this.k + this.x, point[1] * this.k + this.y];
    },
    applyX: function(x2) {
      return x2 * this.k + this.x;
    },
    applyY: function(y2) {
      return y2 * this.k + this.y;
    },
    invert: function(location) {
      return [(location[0] - this.x) / this.k, (location[1] - this.y) / this.k];
    },
    invertX: function(x2) {
      return (x2 - this.x) / this.k;
    },
    invertY: function(y2) {
      return (y2 - this.y) / this.k;
    },
    rescaleX: function(x2) {
      return x2.copy().domain(x2.range().map(this.invertX, this).map(x2.invert, x2));
    },
    rescaleY: function(y2) {
      return y2.copy().domain(y2.range().map(this.invertY, this).map(y2.invert, y2));
    },
    toString: function() {
      return "translate(" + this.x + "," + this.y + ") scale(" + this.k + ")";
    }
  };
  var identity2 = new Transform(1, 0, 0);
  transform.prototype = Transform.prototype;
  function transform(node) {
    while (!node.__zoom) if (!(node = node.parentNode)) return identity2;
    return node.__zoom;
  }

  // shared/diagram-renderer/node_modules/d3-zoom/src/noevent.js
  function nopropagation2(event) {
    event.stopImmediatePropagation();
  }
  function noevent_default3(event) {
    event.preventDefault();
    event.stopImmediatePropagation();
  }

  // shared/diagram-renderer/node_modules/d3-zoom/src/zoom.js
  function defaultFilter(event) {
    return (!event.ctrlKey || event.type === "wheel") && !event.button;
  }
  function defaultExtent() {
    var e = this;
    if (e instanceof SVGElement) {
      e = e.ownerSVGElement || e;
      if (e.hasAttribute("viewBox")) {
        e = e.viewBox.baseVal;
        return [[e.x, e.y], [e.x + e.width, e.y + e.height]];
      }
      return [[0, 0], [e.width.baseVal.value, e.height.baseVal.value]];
    }
    return [[0, 0], [e.clientWidth, e.clientHeight]];
  }
  function defaultTransform() {
    return this.__zoom || identity2;
  }
  function defaultWheelDelta(event) {
    return -event.deltaY * (event.deltaMode === 1 ? 0.05 : event.deltaMode ? 1 : 2e-3) * (event.ctrlKey ? 10 : 1);
  }
  function defaultTouchable() {
    return navigator.maxTouchPoints || "ontouchstart" in this;
  }
  function defaultConstrain(transform2, extent, translateExtent) {
    var dx0 = transform2.invertX(extent[0][0]) - translateExtent[0][0], dx1 = transform2.invertX(extent[1][0]) - translateExtent[1][0], dy0 = transform2.invertY(extent[0][1]) - translateExtent[0][1], dy1 = transform2.invertY(extent[1][1]) - translateExtent[1][1];
    return transform2.translate(
      dx1 > dx0 ? (dx0 + dx1) / 2 : Math.min(0, dx0) || Math.max(0, dx1),
      dy1 > dy0 ? (dy0 + dy1) / 2 : Math.min(0, dy0) || Math.max(0, dy1)
    );
  }
  function zoom_default2() {
    var filter2 = defaultFilter, extent = defaultExtent, constrain = defaultConstrain, wheelDelta = defaultWheelDelta, touchable = defaultTouchable, scaleExtent = [0, Infinity], translateExtent = [[-Infinity, -Infinity], [Infinity, Infinity]], duration = 250, interpolate = zoom_default, listeners = dispatch_default("start", "zoom", "end"), touchstarting, touchfirst, touchending, touchDelay = 500, wheelDelay = 150, clickDistance2 = 0, tapDistance = 10;
    function zoom(selection2) {
      selection2.property("__zoom", defaultTransform).on("wheel.zoom", wheeled, { passive: false }).on("mousedown.zoom", mousedowned).on("dblclick.zoom", dblclicked).filter(touchable).on("touchstart.zoom", touchstarted).on("touchmove.zoom", touchmoved).on("touchend.zoom touchcancel.zoom", touchended).style("-webkit-tap-highlight-color", "rgba(0,0,0,0)");
    }
    zoom.transform = function(collection, transform2, point, event) {
      var selection2 = collection.selection ? collection.selection() : collection;
      selection2.property("__zoom", defaultTransform);
      if (collection !== selection2) {
        schedule(collection, transform2, point, event);
      } else {
        selection2.interrupt().each(function() {
          gesture(this, arguments).event(event).start().zoom(null, typeof transform2 === "function" ? transform2.apply(this, arguments) : transform2).end();
        });
      }
    };
    zoom.scaleBy = function(selection2, k, p, event) {
      zoom.scaleTo(selection2, function() {
        var k0 = this.__zoom.k, k1 = typeof k === "function" ? k.apply(this, arguments) : k;
        return k0 * k1;
      }, p, event);
    };
    zoom.scaleTo = function(selection2, k, p, event) {
      zoom.transform(selection2, function() {
        var e = extent.apply(this, arguments), t0 = this.__zoom, p0 = p == null ? centroid(e) : typeof p === "function" ? p.apply(this, arguments) : p, p1 = t0.invert(p0), k1 = typeof k === "function" ? k.apply(this, arguments) : k;
        return constrain(translate(scale(t0, k1), p0, p1), e, translateExtent);
      }, p, event);
    };
    zoom.translateBy = function(selection2, x2, y2, event) {
      zoom.transform(selection2, function() {
        return constrain(this.__zoom.translate(
          typeof x2 === "function" ? x2.apply(this, arguments) : x2,
          typeof y2 === "function" ? y2.apply(this, arguments) : y2
        ), extent.apply(this, arguments), translateExtent);
      }, null, event);
    };
    zoom.translateTo = function(selection2, x2, y2, p, event) {
      zoom.transform(selection2, function() {
        var e = extent.apply(this, arguments), t = this.__zoom, p0 = p == null ? centroid(e) : typeof p === "function" ? p.apply(this, arguments) : p;
        return constrain(identity2.translate(p0[0], p0[1]).scale(t.k).translate(
          typeof x2 === "function" ? -x2.apply(this, arguments) : -x2,
          typeof y2 === "function" ? -y2.apply(this, arguments) : -y2
        ), e, translateExtent);
      }, p, event);
    };
    function scale(transform2, k) {
      k = Math.max(scaleExtent[0], Math.min(scaleExtent[1], k));
      return k === transform2.k ? transform2 : new Transform(k, transform2.x, transform2.y);
    }
    function translate(transform2, p0, p1) {
      var x2 = p0[0] - p1[0] * transform2.k, y2 = p0[1] - p1[1] * transform2.k;
      return x2 === transform2.x && y2 === transform2.y ? transform2 : new Transform(transform2.k, x2, y2);
    }
    function centroid(extent2) {
      return [(+extent2[0][0] + +extent2[1][0]) / 2, (+extent2[0][1] + +extent2[1][1]) / 2];
    }
    function schedule(transition2, transform2, point, event) {
      transition2.on("start.zoom", function() {
        gesture(this, arguments).event(event).start();
      }).on("interrupt.zoom end.zoom", function() {
        gesture(this, arguments).event(event).end();
      }).tween("zoom", function() {
        var that = this, args = arguments, g = gesture(that, args).event(event), e = extent.apply(that, args), p = point == null ? centroid(e) : typeof point === "function" ? point.apply(that, args) : point, w = Math.max(e[1][0] - e[0][0], e[1][1] - e[0][1]), a = that.__zoom, b = typeof transform2 === "function" ? transform2.apply(that, args) : transform2, i = interpolate(a.invert(p).concat(w / a.k), b.invert(p).concat(w / b.k));
        return function(t) {
          if (t === 1) t = b;
          else {
            var l = i(t), k = w / l[2];
            t = new Transform(k, p[0] - l[0] * k, p[1] - l[1] * k);
          }
          g.zoom(null, t);
        };
      });
    }
    function gesture(that, args, clean) {
      return !clean && that.__zooming || new Gesture(that, args);
    }
    function Gesture(that, args) {
      this.that = that;
      this.args = args;
      this.active = 0;
      this.sourceEvent = null;
      this.extent = extent.apply(that, args);
      this.taps = 0;
    }
    Gesture.prototype = {
      event: function(event) {
        if (event) this.sourceEvent = event;
        return this;
      },
      start: function() {
        if (++this.active === 1) {
          this.that.__zooming = this;
          this.emit("start");
        }
        return this;
      },
      zoom: function(key, transform2) {
        if (this.mouse && key !== "mouse") this.mouse[1] = transform2.invert(this.mouse[0]);
        if (this.touch0 && key !== "touch") this.touch0[1] = transform2.invert(this.touch0[0]);
        if (this.touch1 && key !== "touch") this.touch1[1] = transform2.invert(this.touch1[0]);
        this.that.__zoom = transform2;
        this.emit("zoom");
        return this;
      },
      end: function() {
        if (--this.active === 0) {
          delete this.that.__zooming;
          this.emit("end");
        }
        return this;
      },
      emit: function(type2) {
        var d = select_default2(this.that).datum();
        listeners.call(
          type2,
          this.that,
          new ZoomEvent(type2, {
            sourceEvent: this.sourceEvent,
            target: zoom,
            type: type2,
            transform: this.that.__zoom,
            dispatch: listeners
          }),
          d
        );
      }
    };
    function wheeled(event, ...args) {
      if (!filter2.apply(this, arguments)) return;
      var g = gesture(this, args).event(event), t = this.__zoom, k = Math.max(scaleExtent[0], Math.min(scaleExtent[1], t.k * Math.pow(2, wheelDelta.apply(this, arguments)))), p = pointer_default(event);
      if (g.wheel) {
        if (g.mouse[0][0] !== p[0] || g.mouse[0][1] !== p[1]) {
          g.mouse[1] = t.invert(g.mouse[0] = p);
        }
        clearTimeout(g.wheel);
      } else if (t.k === k) return;
      else {
        g.mouse = [p, t.invert(p)];
        interrupt_default(this);
        g.start();
      }
      noevent_default3(event);
      g.wheel = setTimeout(wheelidled, wheelDelay);
      g.zoom("mouse", constrain(translate(scale(t, k), g.mouse[0], g.mouse[1]), g.extent, translateExtent));
      function wheelidled() {
        g.wheel = null;
        g.end();
      }
    }
    function mousedowned(event, ...args) {
      if (touchending || !filter2.apply(this, arguments)) return;
      var currentTarget = event.currentTarget, g = gesture(this, args, true).event(event), v = select_default2(event.view).on("mousemove.zoom", mousemoved, true).on("mouseup.zoom", mouseupped, true), p = pointer_default(event, currentTarget), x0 = event.clientX, y0 = event.clientY;
      nodrag_default(event.view);
      nopropagation2(event);
      g.mouse = [p, this.__zoom.invert(p)];
      interrupt_default(this);
      g.start();
      function mousemoved(event2) {
        noevent_default3(event2);
        if (!g.moved) {
          var dx = event2.clientX - x0, dy = event2.clientY - y0;
          g.moved = dx * dx + dy * dy > clickDistance2;
        }
        g.event(event2).zoom("mouse", constrain(translate(g.that.__zoom, g.mouse[0] = pointer_default(event2, currentTarget), g.mouse[1]), g.extent, translateExtent));
      }
      function mouseupped(event2) {
        v.on("mousemove.zoom mouseup.zoom", null);
        yesdrag(event2.view, g.moved);
        noevent_default3(event2);
        g.event(event2).end();
      }
    }
    function dblclicked(event, ...args) {
      if (!filter2.apply(this, arguments)) return;
      var t0 = this.__zoom, p0 = pointer_default(event.changedTouches ? event.changedTouches[0] : event, this), p1 = t0.invert(p0), k1 = t0.k * (event.shiftKey ? 0.5 : 2), t1 = constrain(translate(scale(t0, k1), p0, p1), extent.apply(this, args), translateExtent);
      noevent_default3(event);
      if (duration > 0) select_default2(this).transition().duration(duration).call(schedule, t1, p0, event);
      else select_default2(this).call(zoom.transform, t1, p0, event);
    }
    function touchstarted(event, ...args) {
      if (!filter2.apply(this, arguments)) return;
      var touches = event.touches, n = touches.length, g = gesture(this, args, event.changedTouches.length === n).event(event), started, i, t, p;
      nopropagation2(event);
      for (i = 0; i < n; ++i) {
        t = touches[i], p = pointer_default(t, this);
        p = [p, this.__zoom.invert(p), t.identifier];
        if (!g.touch0) g.touch0 = p, started = true, g.taps = 1 + !!touchstarting;
        else if (!g.touch1 && g.touch0[2] !== p[2]) g.touch1 = p, g.taps = 0;
      }
      if (touchstarting) touchstarting = clearTimeout(touchstarting);
      if (started) {
        if (g.taps < 2) touchfirst = p[0], touchstarting = setTimeout(function() {
          touchstarting = null;
        }, touchDelay);
        interrupt_default(this);
        g.start();
      }
    }
    function touchmoved(event, ...args) {
      if (!this.__zooming) return;
      var g = gesture(this, args).event(event), touches = event.changedTouches, n = touches.length, i, t, p, l;
      noevent_default3(event);
      for (i = 0; i < n; ++i) {
        t = touches[i], p = pointer_default(t, this);
        if (g.touch0 && g.touch0[2] === t.identifier) g.touch0[0] = p;
        else if (g.touch1 && g.touch1[2] === t.identifier) g.touch1[0] = p;
      }
      t = g.that.__zoom;
      if (g.touch1) {
        var p0 = g.touch0[0], l0 = g.touch0[1], p1 = g.touch1[0], l1 = g.touch1[1], dp = (dp = p1[0] - p0[0]) * dp + (dp = p1[1] - p0[1]) * dp, dl = (dl = l1[0] - l0[0]) * dl + (dl = l1[1] - l0[1]) * dl;
        t = scale(t, Math.sqrt(dp / dl));
        p = [(p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2];
        l = [(l0[0] + l1[0]) / 2, (l0[1] + l1[1]) / 2];
      } else if (g.touch0) p = g.touch0[0], l = g.touch0[1];
      else return;
      g.zoom("touch", constrain(translate(t, p, l), g.extent, translateExtent));
    }
    function touchended(event, ...args) {
      if (!this.__zooming) return;
      var g = gesture(this, args).event(event), touches = event.changedTouches, n = touches.length, i, t;
      nopropagation2(event);
      if (touchending) clearTimeout(touchending);
      touchending = setTimeout(function() {
        touchending = null;
      }, touchDelay);
      for (i = 0; i < n; ++i) {
        t = touches[i];
        if (g.touch0 && g.touch0[2] === t.identifier) delete g.touch0;
        else if (g.touch1 && g.touch1[2] === t.identifier) delete g.touch1;
      }
      if (g.touch1 && !g.touch0) g.touch0 = g.touch1, delete g.touch1;
      if (g.touch0) g.touch0[1] = this.__zoom.invert(g.touch0[0]);
      else {
        g.end();
        if (g.taps === 2) {
          t = pointer_default(t, this);
          if (Math.hypot(touchfirst[0] - t[0], touchfirst[1] - t[1]) < tapDistance) {
            var p = select_default2(this).on("dblclick.zoom");
            if (p) p.apply(this, arguments);
          }
        }
      }
    }
    zoom.wheelDelta = function(_) {
      return arguments.length ? (wheelDelta = typeof _ === "function" ? _ : constant_default5(+_), zoom) : wheelDelta;
    };
    zoom.filter = function(_) {
      return arguments.length ? (filter2 = typeof _ === "function" ? _ : constant_default5(!!_), zoom) : filter2;
    };
    zoom.touchable = function(_) {
      return arguments.length ? (touchable = typeof _ === "function" ? _ : constant_default5(!!_), zoom) : touchable;
    };
    zoom.extent = function(_) {
      return arguments.length ? (extent = typeof _ === "function" ? _ : constant_default5([[+_[0][0], +_[0][1]], [+_[1][0], +_[1][1]]]), zoom) : extent;
    };
    zoom.scaleExtent = function(_) {
      return arguments.length ? (scaleExtent[0] = +_[0], scaleExtent[1] = +_[1], zoom) : [scaleExtent[0], scaleExtent[1]];
    };
    zoom.translateExtent = function(_) {
      return arguments.length ? (translateExtent[0][0] = +_[0][0], translateExtent[1][0] = +_[1][0], translateExtent[0][1] = +_[0][1], translateExtent[1][1] = +_[1][1], zoom) : [[translateExtent[0][0], translateExtent[0][1]], [translateExtent[1][0], translateExtent[1][1]]];
    };
    zoom.constrain = function(_) {
      return arguments.length ? (constrain = _, zoom) : constrain;
    };
    zoom.duration = function(_) {
      return arguments.length ? (duration = +_, zoom) : duration;
    };
    zoom.interpolate = function(_) {
      return arguments.length ? (interpolate = _, zoom) : interpolate;
    };
    zoom.on = function() {
      var value = listeners.on.apply(listeners, arguments);
      return value === listeners ? zoom : value;
    };
    zoom.clickDistance = function(_) {
      return arguments.length ? (clickDistance2 = (_ = +_) * _, zoom) : Math.sqrt(clickDistance2);
    };
    zoom.tapDistance = function(_) {
      return arguments.length ? (tapDistance = +_, zoom) : tapDistance;
    };
    return zoom;
  }

  // shared/diagram-renderer/src/theme.ts
  var NOTATION_THEME_LIGHT = {
    canvasBackground: "#ffffff",
    panelBackground: "#f3f4f6",
    nodeFill: "#ffffff",
    nodeBorder: "#374151",
    textPrimary: "#111827",
    textSecondary: "#6b7280",
    divider: "#d1d5db",
    highlight: "#d97706",
    edge: { default: "#374151" },
    frame: { stroke: "#9ca3af", text: "#374151" }
  };
  var NOTATION_THEME_DARK = {
    canvasBackground: "#1e1e1e",
    panelBackground: "#2d2d2d",
    nodeFill: "#1e1e1e",
    nodeBorder: "#d4d4d4",
    textPrimary: "#e5e5e5",
    textSecondary: "#a3a3a3",
    divider: "#525252",
    highlight: "#fbbf24",
    edge: { default: "#d4d4d4" },
    frame: { stroke: "#737373", text: "#e5e5e5" }
  };
  var NOTATION_THEME_VSCODE = {
    canvasBackground: "var(--vscode-editor-background, transparent)",
    panelBackground: "var(--vscode-button-secondaryBackground)",
    nodeFill: "var(--vscode-editor-background)",
    nodeBorder: "var(--vscode-editor-foreground)",
    textPrimary: "var(--vscode-editor-foreground)",
    textSecondary: "var(--vscode-descriptionForeground)",
    divider: "var(--vscode-panel-border)",
    highlight: "var(--vscode-focusBorder, #d97706)",
    edge: { default: "var(--vscode-editor-foreground)" },
    frame: {
      stroke: "var(--vscode-panel-border)",
      text: "var(--vscode-editor-foreground)"
    }
  };
  function detectColorScheme(host) {
    if (typeof host !== "undefined" && host !== null) {
      const svg = host.closest?.(".sysml-viz-svg");
      const scheme = svg?.getAttribute("data-color-scheme");
      if (scheme === "light" || scheme === "dark") {
        return scheme;
      }
    }
    if (typeof window !== "undefined" && typeof window.matchMedia === "function") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
    return "light";
  }
  function baseThemeForScheme(scheme) {
    if (scheme === "vscode") {
      return NOTATION_THEME_VSCODE;
    }
    const resolved = scheme === "auto" ? detectColorScheme() : scheme;
    return resolved === "dark" ? NOTATION_THEME_DARK : NOTATION_THEME_LIGHT;
  }
  function resolveDiagramTheme(options) {
    const colorScheme = options?.colorScheme ?? "vscode";
    const base = baseThemeForScheme(colorScheme);
    return {
      ...base,
      ...options ?? {},
      colorScheme,
      edge: { ...base.edge, ...options?.edge ?? {} },
      frame: { ...base.frame, ...options?.frame ?? {} }
    };
  }
  function strokeColorForNode(theme) {
    return theme.nodeBorder;
  }
  function strokeColorForEdge(_kind, theme) {
    return theme.edge.default;
  }

  // shared/diagram-renderer/src/views/behavior-interaction.ts
  function nodeSupportsSourceNavigation(node) {
    const attrs = node.attributes ?? {};
    const qualifiedName = asString2(attrs.qualifiedName ?? node.id);
    const uri = asString2(node.uri ?? node.sourcePath);
    const range = node.range;
    const hasRange = Boolean(range?.start && typeof range.start.line === "number");
    return Boolean(uri && hasRange || qualifiedName.includes("::") || node.label.trim());
  }
  function clearBehaviorHighlights(root2) {
    root2.selectAll(".highlighted-element").each(function() {
      const group = select_default2(this);
      group.classed("highlighted-element", false);
      group.select(".node-background").each(function() {
        const el = select_default2(this);
        const origStroke = el.attr("data-original-stroke");
        const origWidth = el.attr("data-original-width");
        if (origStroke) {
          el.style("stroke", origStroke).style("stroke-width", origWidth);
        }
      });
    });
  }
  function attachBehaviorNodeClick(nodeGroup, node, theme, options, root2) {
    nodeGroup.style("cursor", options.onNodeClick && nodeSupportsSourceNavigation(node) ? "pointer" : "").on("click", (event) => {
      if (!options.onNodeClick || !nodeSupportsSourceNavigation(node)) {
        return;
      }
      event.stopPropagation();
      clearBehaviorHighlights(root2);
      nodeGroup.classed("highlighted-element", true);
      const background = nodeGroup.select(".node-background");
      if (!background.empty()) {
        background.style("stroke", theme.highlight).style("stroke-width", "3px");
      }
      options.onNodeClick(node);
    });
  }
  function asString2(value, fallback = "") {
    if (typeof value === "string") return value;
    if (typeof value === "number" || typeof value === "boolean") return String(value);
    return fallback;
  }

  // shared/diagram-renderer/src/headless-elk-shim.ts
  var HeadlessElk = class {
    constructor(options = {}) {
      const global = globalThis;
      const ElkCtor = global.__spec42ElkCtor;
      const WorkerCtor = global.__spec42ElkWorkerCtor;
      if (typeof ElkCtor !== "function" || typeof WorkerCtor !== "function") {
        throw new Error("Spec42 headless ELK constructors were not installed");
      }
      return new ElkCtor({
        ...options,
        workerFactory: () => new WorkerCtor()
      });
    }
  };

  // shared/diagram-renderer/src/views/elk-label-utils.ts
  function estimateElkLabelBox(id2, text, options) {
    const paddingX = options?.paddingX ?? 10;
    const paddingY = options?.paddingY ?? 8;
    const minWidth = options?.minWidth ?? 42;
    const minHeight = options?.minHeight ?? 18;
    const charWidth = options?.charWidth ?? 6;
    return {
      id: id2,
      text,
      x: 0,
      y: 0,
      width: Math.max(minWidth, text.length * charWidth + paddingX),
      height: Math.max(minHeight, paddingY + 10)
    };
  }
  function toAbsoluteElkLabelBox(label, offset = { x: 0, y: 0 }) {
    if (!label) return null;
    const id2 = String(label.id || "");
    const text = String(label.text || "");
    if (!id2 || !text) return null;
    return {
      id: id2,
      text,
      x: (label.x ?? 0) + offset.x,
      y: (label.y ?? 0) + offset.y,
      width: label.width ?? 0,
      height: label.height ?? 0
    };
  }
  function collectElkEdgeLabels(elkNode, offset, acc) {
    (elkNode?.edges ?? []).forEach((edgeRaw) => {
      const edge = edgeRaw;
      if (!edge?.id || !Array.isArray(edge.labels) || edge.labels.length === 0) {
        return;
      }
      const labels = edge.labels.map((label) => toAbsoluteElkLabelBox(label, offset)).filter((label) => Boolean(label));
      if (labels.length > 0) {
        acc.set(String(edge.id), labels);
      }
    });
    (elkNode?.children ?? []).forEach((childRaw) => {
      const child = childRaw;
      collectElkEdgeLabels(child, { x: offset.x + (child.x ?? 0), y: offset.y + (child.y ?? 0) }, acc);
    });
  }
  function edgeLabelPositionFromSections(sections) {
    const section = sections?.[0];
    if (!section?.startPoint || !section?.endPoint) {
      return null;
    }
    const points = [section.startPoint, ...section.bendPoints ?? [], section.endPoint];
    const midIndex = Math.floor((points.length - 1) / 2);
    const start2 = points[midIndex];
    const end = points[midIndex + 1] ?? points[midIndex];
    return {
      x: (start2.x + end.x) / 2,
      y: (start2.y + end.y) / 2 - 6
    };
  }

  // shared/diagram-renderer/src/render/elk-options.ts
  var COMMON_ELK_OPTIONS = {
    "elk.algorithm": "layered",
    "elk.edgeRouting": "ORTHOGONAL",
    "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
    "elk.separateConnectedComponents": "true",
    "elk.json.edgeCoords": "ROOT"
  };
  var PER_KIND_DEFAULTS = {
    general: {
      "elk.direction": "DOWN",
      "elk.spacing.nodeNode": "140",
      "elk.layered.spacing.nodeNodeBetweenLayers": "180",
      "elk.spacing.edgeNode": "90",
      "elk.spacing.edgeEdge": "80",
      "elk.aspectRatio": "1.4",
      "elk.padding": "[top=100,left=100,bottom=100,right=100]",
      "org.eclipse.elk.portConstraints": "FIXED_SIDE"
    },
    interconnection: {
      "elk.hierarchyHandling": "INCLUDE_CHILDREN",
      "elk.direction": "RIGHT",
      "elk.spacing.nodeNode": "150",
      "elk.layered.spacing.nodeNodeBetweenLayers": "220",
      "elk.spacing.edgeNode": "110",
      "elk.spacing.edgeEdge": "90",
      "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
      "elk.padding": "[top=70,left=70,bottom=70,right=70]",
      "org.eclipse.elk.portConstraints": "FIXED_ORDER",
      "org.eclipse.elk.portAlignment.default": "CENTER"
    },
    "behavior-state": {
      "elk.hierarchyHandling": "INCLUDE_CHILDREN",
      "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
      "elk.layered.spacing.nodeNodeBetweenLayers": "230",
      "elk.spacing.nodeNode": "190",
      "elk.spacing.edgeNode": "130",
      "elk.spacing.edgeEdge": "110",
      "elk.spacing.edgeLabel": "12",
      "elk.padding": "[top=100,left=90,bottom=90,right=90]"
    },
    "behavior-action": {
      "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
      "elk.spacing.edgeNode": "80",
      "elk.spacing.edgeEdge": "60",
      "elk.spacing.edgeLabel": "12",
      "elk.padding": "[top=80,left=80,bottom=80,right=80]"
    }
  };
  function buildElkLayoutOptions(kind, overrides = {}) {
    const merged = {
      ...COMMON_ELK_OPTIONS,
      ...PER_KIND_DEFAULTS[kind]
    };
    for (const [key, value] of Object.entries(overrides)) {
      if (value === void 0) {
        delete merged[key];
      } else {
        merged[key] = value;
      }
    }
    return merged;
  }

  // shared/diagram-renderer/src/views/behavior-common.ts
  var behaviorElk = new HeadlessElk();
  function nodeKind(node) {
    return String(node.kind || "action").toLowerCase();
  }
  function pathFromSections(sections) {
    if (!sections?.length) return null;
    const parts = [];
    for (const section of sections) {
      if (!section.startPoint || !section.endPoint) continue;
      parts.push(`M${section.startPoint.x},${section.startPoint.y}`);
      for (const point of section.bendPoints ?? []) {
        parts.push(`L${point.x},${point.y}`);
      }
      parts.push(`L${section.endPoint.x},${section.endPoint.y}`);
    }
    return parts.length ? parts.join(" ") : null;
  }
  function buildSelfLoopPath(node) {
    const startX = node.x + node.width;
    const startY = node.y + node.height / 2 - 8;
    const loopRadius = 28;
    return {
      path: `M${startX},${startY} C${startX + loopRadius},${startY - loopRadius} ${startX + loopRadius},${startY + loopRadius} ${startX},${startY + 18}`,
      labelX: startX + loopRadius + 8,
      labelY: startY
    };
  }
  function fallbackEdgePath(source, target, horizontal) {
    if (source.x === target.x && source.y === target.y && source.width === target.width && source.height === target.height) {
      return buildSelfLoopPath(source);
    }
    if (horizontal) {
      const startX2 = source.x + source.width;
      const startY2 = source.y + source.height / 2;
      const endX2 = target.x;
      const endY2 = target.y + target.height / 2;
      const midX = (startX2 + endX2) / 2;
      return {
        path: `M${startX2},${startY2} L${midX},${startY2} L${midX},${endY2} L${endX2},${endY2}`,
        labelX: midX,
        labelY: (startY2 + endY2) / 2 - 6
      };
    }
    const startX = source.x + source.width / 2;
    const startY = source.y + source.height;
    const endX = target.x + target.width / 2;
    const endY = target.y;
    const midY = (startY + endY) / 2;
    return {
      path: `M${startX},${startY} L${startX},${midY} L${endX},${midY} L${endX},${endY}`,
      labelX: (startX + endX) / 2,
      labelY: midY - 6
    };
  }
  function nodeDimensions(node, mode) {
    const kind = nodeKind(node);
    if (mode === "state") {
      if (kind.includes("initial") || kind.includes("final")) return { width: 34, height: 34 };
      if (kind.includes("composite")) return { width: 340, height: 320 };
      return { width: 240, height: 180 };
    }
    if (kind.includes("initial") || kind.includes("final") || kind.includes("start") || kind.includes("done")) {
      return { width: 40, height: 40 };
    }
    if (kind.includes("decision") || kind.includes("merge")) return { width: 76, height: 76 };
    if (kind.includes("fork") || kind.includes("join")) return { width: 220, height: 14 };
    return { width: 220, height: 68 };
  }
  function transitionDisplayLabel(label) {
    const trimmed = label.trim();
    if (!trimmed || trimmed.toLowerCase() === "entry") return "";
    return trimmed;
  }
  async function layoutBehaviorGraph(prepared, options) {
    const horizontal = options.horizontal ?? false;
    const positions = /* @__PURE__ */ new Map();
    const edgeSectionsById = /* @__PURE__ */ new Map();
    const edgeLabelsById = /* @__PURE__ */ new Map();
    const children2 = prepared.nodes.map((node) => {
      const size = nodeDimensions(node, options.mode);
      return { id: node.id, width: size.width, height: size.height };
    });
    const edges = prepared.edges.map((edge) => {
      const displayLabel = transitionDisplayLabel(edge.label);
      const base = {
        id: edge.id,
        sources: [edge.source],
        targets: [edge.target]
      };
      if (!displayLabel) {
        return base;
      }
      const labelBox = estimateElkLabelBox(`${edge.id}::label`, displayLabel, {
        minWidth: 38,
        minHeight: 16,
        paddingX: 8,
        paddingY: 6,
        charWidth: 6
      });
      return {
        ...base,
        labels: [
          {
            id: labelBox.id,
            text: labelBox.text,
            width: labelBox.width,
            height: labelBox.height,
            layoutOptions: {
              "org.eclipse.elk.edgeLabels.placement": "CENTER",
              "org.eclipse.elk.edgeLabels.inline": "false"
            }
          }
        ]
      };
    });
    const isState = options.mode === "state";
    const graph = {
      id: prepared.title || "behavior",
      layoutOptions: isState ? buildElkLayoutOptions("behavior-state", {
        "elk.direction": horizontal ? "RIGHT" : "DOWN"
      }) : buildElkLayoutOptions("behavior-action", {
        "elk.direction": horizontal ? "RIGHT" : "DOWN",
        "elk.spacing.nodeNode": horizontal ? "90" : "120",
        "elk.layered.spacing.nodeNodeBetweenLayers": horizontal ? "190" : "170"
      }),
      children: children2,
      edges
    };
    const laidOut = await behaviorElk.layout(graph);
    for (const child of laidOut.children ?? []) {
      positions.set(String(child.id), {
        x: child.x ?? 0,
        y: child.y ?? 0,
        width: child.width ?? 200,
        height: child.height ?? 80
      });
    }
    for (const edge of laidOut.edges ?? []) {
      if (edge.sections) {
        edgeSectionsById.set(String(edge.id), edge.sections);
      }
    }
    collectElkEdgeLabels(laidOut, { x: 0, y: 0 }, edgeLabelsById);
    return { positions, edgeSectionsById, edgeLabelsById };
  }
  function truncateLabel(text, max2) {
    const trimmed = text.trim();
    return trimmed.length > max2 ? `${trimmed.slice(0, max2 - 2)}..` : trimmed;
  }

  // shared/diagram-renderer/src/views/action-flow.ts
  function isInitial(kind) {
    return kind.includes("initial") || kind.includes("start");
  }
  function isFinal(kind) {
    return kind.includes("final") || kind.includes("done") || kind.includes("end");
  }
  function isFlowFinal(kind) {
    return kind.includes("flow-final") || kind.includes("flow final") || kind.includes("terminate");
  }
  function isDecision(kind) {
    return kind.includes("decision") || kind.includes("merge");
  }
  function isFork(kind) {
    return kind.includes("fork") || kind.includes("join");
  }
  function activityNodeKind(node) {
    const attrs = node.attributes ?? {};
    const typed = String(attrs.stateType ?? attrs.kind ?? node.kind ?? "").toLowerCase();
    return typed || nodeKind(node);
  }
  function drawActionNode(group, node, layout, theme) {
    const kind = activityNodeKind(node);
    const attrs = node.attributes ?? {};
    const inputs = Array.isArray(attrs.inputs) ? attrs.inputs : Array.isArray(attrs.inputParameters) ? attrs.inputParameters : [];
    const outputs = Array.isArray(attrs.outputs) ? attrs.outputs : Array.isArray(attrs.outputParameters) ? attrs.outputParameters : [];
    const isPerform = kind.includes("perform") || String(attrs.actionType ?? attrs.type ?? "").toLowerCase().includes("perform");
    const g = group.append("g").attr("class", `activity-action action-flow-node${isPerform ? " perform-action-node" : ""}`).attr("data-node-id", node.id).attr("transform", `translate(${layout.x},${layout.y})`);
    if (isInitial(kind) || isFinal(kind)) {
      g.append("circle").attr("class", "node-background").attr("data-original-stroke", theme.nodeBorder).attr("data-original-width", "2px").attr("cx", layout.width / 2).attr("cy", layout.height / 2).attr("r", layout.width / 2 - 2).style("fill", isInitial(kind) ? theme.edge.default : theme.canvasBackground).style("stroke", theme.nodeBorder).style("stroke-width", "2px");
      if (isFinal(kind) && !isFlowFinal(kind)) {
        g.append("circle").attr("cx", layout.width / 2).attr("cy", layout.height / 2).attr("r", 10).style("fill", theme.edge.default).style("stroke", "none");
      }
      if (isFlowFinal(kind)) {
        g.append("path").attr("class", "flow-final-x").attr("d", `M${layout.width / 2 - 8},${layout.height / 2 - 8} L${layout.width / 2 + 8},${layout.height / 2 + 8} M${layout.width / 2 + 8},${layout.height / 2 - 8} L${layout.width / 2 - 8},${layout.height / 2 + 8}`).style("stroke", theme.edge.default).style("stroke-width", "2px");
      }
    } else if (isDecision(kind)) {
      const cx = layout.width / 2;
      const cy = layout.height / 2;
      g.append("path").attr("class", "node-background").attr("data-original-stroke", theme.edge.default).attr("data-original-width", "2px").attr("d", `M${cx},0 L${layout.width},${cy} L${cx},${layout.height} L0,${cy} Z`).style("fill", theme.canvasBackground).style("stroke", theme.edge.default).style("stroke-width", "2px");
    } else if (isFork(kind)) {
      g.append("rect").attr("class", "node-background").attr("data-original-stroke", "none").attr("data-original-width", "0px").attr("width", layout.width).attr("height", layout.height).attr("rx", 3).style("fill", theme.nodeBorder).style("stroke", "none");
    } else {
      g.append("rect").attr("class", "node-background").attr("data-original-stroke", theme.nodeBorder).attr("data-original-width", "2px").attr("width", layout.width).attr("height", layout.height).attr("rx", 8).style("fill", theme.nodeFill).style("stroke", theme.nodeBorder).style("stroke-width", "2px").style("stroke-dasharray", isPerform ? "5,3" : "none");
      g.append("rect").attr("width", layout.width).attr("height", 6).attr("rx", 8).style("fill", theme.nodeBorder).style("stroke", "none");
      if (isPerform) {
        g.append("text").attr("class", "perform-action-stereotype").attr("x", layout.width / 2).attr("y", 20).attr("text-anchor", "middle").style("font-size", "9px").style("fill", theme.textSecondary).text("perform");
      }
    }
    const labelY = isFork(kind) ? layout.height + 14 : layout.height / 2 + (isPerform ? 12 : 4);
    g.append("text").attr("x", layout.width / 2).attr("y", labelY).attr("text-anchor", "middle").style("font-size", "12px").style("font-weight", "600").style("fill", theme.textPrimary).text(truncateLabel(node.label, 24));
    const drawParameter = (items, side) => {
      items.slice(0, 4).forEach((item, index) => {
        const parameter = item && typeof item === "object" ? item : { name: String(item) };
        const name = String(parameter.name ?? parameter.label ?? item ?? "");
        const y2 = 20 + index * 14;
        const x2 = side === "input" ? -9 : layout.width + 9;
        g.append("circle").attr("class", `action-parameter-badge action-parameter-${side}`).attr("cx", x2).attr("cy", y2).attr("r", 5).style("fill", theme.canvasBackground).style("stroke", theme.nodeBorder).style("stroke-width", "1.5px");
        g.append("text").attr("class", `action-parameter-label action-parameter-${side}-label`).attr("x", side === "input" ? x2 - 8 : x2 + 8).attr("y", y2 + 3).attr("text-anchor", side === "input" ? "end" : "start").style("font-size", "8px").style("fill", theme.textSecondary).text(truncateLabel(name, 14));
      });
    };
    if (!isInitial(kind) && !isFinal(kind) && !isDecision(kind) && !isFork(kind)) {
      drawParameter(inputs, "input");
      drawParameter(outputs, "output");
    }
    return g;
  }
  async function renderActionFlowView(ctx) {
    const horizontal = String(ctx.prepared.meta?.layoutDirection ?? "").toLowerCase() === "horizontal";
    const layout = await layoutBehaviorGraph(ctx.prepared, { horizontal, mode: "action" });
    const renderOptions = ctx.options ?? {};
    ctx.root.append("text").attr("x", 24).attr("y", 28).style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "Action Flow");
    const flowLayer = ctx.root.append("g").attr("class", "activity-flows");
    const laneLayer = ctx.root.insert("g", ".activity-flows").attr("class", "activity-swim-lanes");
    const nodeLayer = ctx.root.append("g").attr("class", "activity-actions");
    const laneExtents = /* @__PURE__ */ new Map();
    for (const node of ctx.prepared.nodes) {
      const position = layout.positions.get(node.id);
      if (!position) continue;
      const lane = String(node.attributes?.swimLane ?? "default");
      const current = laneExtents.get(lane) ?? {
        minX: position.x,
        maxX: position.x + position.width,
        minY: position.y,
        maxY: position.y + position.height
      };
      current.minX = Math.min(current.minX, position.x);
      current.maxX = Math.max(current.maxX, position.x + position.width);
      current.minY = Math.min(current.minY, position.y);
      current.maxY = Math.max(current.maxY, position.y + position.height);
      laneExtents.set(lane, current);
    }
    laneExtents.forEach((extent, lane) => {
      if (laneExtents.size <= 1) {
        return;
      }
      laneLayer.append("rect").attr("x", extent.minX - 24).attr("y", extent.minY - 36).attr("width", extent.maxX - extent.minX + 48).attr("height", extent.maxY - extent.minY + 56).attr("rx", 8).style("fill", ctx.theme.canvasBackground).style("stroke", ctx.theme.frame.stroke).style("stroke-dasharray", "6,4");
      laneLayer.append("text").attr("x", extent.minX - 12).attr("y", extent.minY - 18).style("font-size", "10px").style("font-weight", "700").style("fill", ctx.theme.textSecondary).text(truncateLabel(lane, 24));
    });
    for (const edge of ctx.prepared.edges) {
      const source = layout.positions.get(edge.source);
      const target = layout.positions.get(edge.target);
      if (!source || !target) continue;
      const sections = layout.edgeSectionsById.get(edge.id);
      const fallback = fallbackEdgePath(source, target, horizontal);
      const edgeAttrs = edge.attributes ?? {};
      const guard = String(edgeAttrs.guard ?? edge.label ?? "").toLowerCase();
      const succession = Boolean(edgeAttrs.succession) || guard === "flow" || guard === "first" || guard === "succession";
      const conditional = Boolean(edgeAttrs.conditional);
      flowLayer.append("path").attr(
        "class",
        succession ? conditional ? "activity-flow action-flow-edge aflow-succession aflow-conditional" : "activity-flow action-flow-edge aflow-succession" : "activity-flow action-flow-edge"
      ).attr("d", pathFromSections(sections) || fallback.path).style("fill", "none").style("stroke", ctx.theme.edge.default).style("stroke-width", "2px").style("marker-end", "url(#action-flow-arrow)");
      const label = truncateLabel(edge.label, 20);
      if (label && !["flow", "first", "bind"].includes(label.toLowerCase())) {
        const elkLabel = layout.edgeLabelsById.get(edge.id)?.[0];
        const labelFromSections = edgeLabelPositionFromSections(sections);
        const labelPosition = elkLabel ? { x: elkLabel.x + elkLabel.width / 2, y: elkLabel.y + elkLabel.height / 2 } : labelFromSections ?? { x: fallback.labelX, y: fallback.labelY };
        const displayLabel = label.startsWith("[") ? label : `[${label}]`;
        if (elkLabel) {
          flowLayer.append("rect").attr("x", elkLabel.x).attr("y", elkLabel.y).attr("width", elkLabel.width).attr("height", elkLabel.height).attr("rx", 3).style("fill", ctx.theme.canvasBackground).style("stroke", ctx.theme.edge.default).style("stroke-width", "1px");
        }
        flowLayer.append("text").attr("x", labelPosition.x).attr("y", labelPosition.y + (elkLabel ? 3 : 0)).attr("text-anchor", "middle").style("font-size", "10px").style("fill", ctx.theme.textSecondary).text(displayLabel);
      }
    }
    for (const node of ctx.prepared.nodes) {
      const position = layout.positions.get(node.id);
      if (!position) continue;
      const nodeGroup = drawActionNode(nodeLayer, node, position, ctx.theme);
      attachBehaviorNodeClick(nodeGroup, node, ctx.theme, renderOptions, ctx.root);
    }
    let minX = 0;
    let minY = 0;
    let maxX = ctx.width;
    let maxY = ctx.height;
    layout.positions.forEach((rect) => {
      minX = Math.min(minX, rect.x);
      minY = Math.min(minY, rect.y);
      maxX = Math.max(maxX, rect.x + rect.width);
      maxY = Math.max(maxY, rect.y + rect.height + 20);
    });
    return { minX: minX - 40, minY: minY - 40, maxX: maxX + 40, maxY: maxY + 40 };
  }
  function addActionFlowMarkers(defs, theme) {
    defs.selectAll("#action-flow-arrow").remove();
    defs.append("marker").attr("id", "action-flow-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 8).attr("refY", 0).attr("markerWidth", 6).attr("markerHeight", 6).attr("orient", "auto").append("path").attr("d", "M0,-5L10,0L0,5").style("fill", theme.edge.default);
  }

  // shared/diagram-renderer/src/views/sequence.ts
  var HEADER_Y = 64;
  var LIFELINE_TOP = 118;
  var LIFELINE_GAP = 220;
  var MESSAGE_GAP = 78;
  var LIFELINE_BOX_WIDTH = 132;
  var LIFELINE_BOX_HEIGHT = 38;
  function asRecord2(value) {
    return value && typeof value === "object" ? value : {};
  }
  function asArray3(value) {
    return Array.isArray(value) ? value : [];
  }
  function asString3(value, fallback = "") {
    if (typeof value === "string") return value;
    if (typeof value === "number" || typeof value === "boolean") return String(value);
    return fallback;
  }
  function messageRow(order) {
    return LIFELINE_TOP + 58 + (Math.max(1, order) - 1) * MESSAGE_GAP;
  }
  function messageRef(message) {
    return asString3(message.id ?? message.name ?? message.label);
  }
  function findPreparedLifeline(preparedNodes, lifeline) {
    const id2 = asString3(lifeline.id ?? lifeline.name);
    const name = asString3(lifeline.name ?? lifeline.label);
    return preparedNodes.find((node) => {
      const qualifiedName = asString3(node.attributes?.qualifiedName);
      if (id2 && (node.id === id2 || qualifiedName === id2)) return true;
      if (name && (node.label === name || qualifiedName === name)) return true;
      return false;
    });
  }
  function addSequenceMarkers(defs, theme) {
    defs.selectAll("#sequence-arrow-sync").remove();
    defs.append("marker").attr("id", "sequence-arrow-sync").attr("viewBox", "0 -5 10 10").attr("refX", 9).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-5L10,0L0,5").style("fill", theme.edge.default);
  }
  function renderSequenceView(ctx) {
    const diagram = asRecord2(ctx.prepared.meta?.sequenceDiagram);
    const lifelines = asArray3(diagram.lifelines).map(asRecord2);
    const messages = asArray3(diagram.messages).map(asRecord2).sort((a, b) => Number(a.order ?? 0) - Number(b.order ?? 0));
    const activations = asArray3(diagram.activations).map(asRecord2);
    const fragments = asArray3(diagram.fragments).map(asRecord2);
    const renderOptions = ctx.options ?? {};
    ctx.root.append("text").attr("x", ctx.width / 2).attr("y", 32).attr("text-anchor", "middle").style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "Sequence");
    if (lifelines.length === 0 || messages.length === 0) {
      ctx.root.append("text").attr("x", ctx.width / 2).attr("y", ctx.height / 2).attr("text-anchor", "middle").style("fill", ctx.theme.textSecondary).text("No sequence lifelines or messages in payload");
      return { minX: 0, minY: 0, maxX: ctx.width, maxY: ctx.height };
    }
    const xOffset = Math.max(80, (ctx.width - (Math.max(0, lifelines.length - 1) * LIFELINE_GAP + LIFELINE_BOX_WIDTH)) / 2);
    const lifelineX = /* @__PURE__ */ new Map();
    lifelines.forEach((lifeline, index) => {
      lifelineX.set(asString3(lifeline.id ?? lifeline.name), xOffset + index * LIFELINE_GAP);
    });
    const lastMessageY = messages.length ? messageRow(Number(messages[messages.length - 1].order ?? messages.length)) : LIFELINE_TOP + 100;
    const lifelineBottom = lastMessageY + 140;
    const lifelineLayer = ctx.root.append("g").attr("class", "sequence-lifelines");
    for (const lifeline of lifelines) {
      const id2 = asString3(lifeline.id ?? lifeline.name);
      const x2 = lifelineX.get(id2) ?? xOffset;
      const label = truncateLabel(asString3(lifeline.name ?? lifeline.label ?? id2), 18);
      const preparedNode = findPreparedLifeline(ctx.prepared.nodes, lifeline);
      const group = lifelineLayer.append("g").attr("class", "sequence-lifeline").attr("data-node-id", preparedNode?.id ?? id2);
      group.append("rect").attr("class", "node-background").attr("data-original-stroke", ctx.theme.nodeBorder).attr("data-original-width", "1.5px").attr("x", x2 - LIFELINE_BOX_WIDTH / 2).attr("y", HEADER_Y).attr("width", LIFELINE_BOX_WIDTH).attr("height", LIFELINE_BOX_HEIGHT).attr("rx", 6).style("fill", ctx.theme.nodeFill).style("stroke", ctx.theme.nodeBorder).style("stroke-width", "1.5px");
      group.append("text").attr("x", x2).attr("y", HEADER_Y + 24).attr("text-anchor", "middle").style("font-size", "11px").style("fill", ctx.theme.textPrimary).text(label);
      group.append("line").attr("x1", x2).attr("y1", LIFELINE_TOP).attr("x2", x2).attr("y2", lifelineBottom).style("stroke", ctx.theme.nodeBorder).style("stroke-dasharray", "6,4");
      if (preparedNode) {
        attachBehaviorNodeClick(group, preparedNode, ctx.theme, renderOptions, ctx.root);
      }
    }
    const messageLayer = ctx.root.append("g").attr("class", "sequence-messages");
    const messagePosition = /* @__PURE__ */ new Map();
    for (const message of messages) {
      const sourceId = asString3(message.source ?? message.from);
      const targetId = asString3(message.target ?? message.to);
      const sourceX = lifelineX.get(sourceId);
      const targetX = lifelineX.get(targetId);
      if (sourceX == null || targetX == null) continue;
      const y2 = messageRow(Number(message.order ?? 1));
      messagePosition.set(messageRef(message), { sourceX, targetX, y: y2 });
      const kind = asString3(message.kind ?? message.type).toLowerCase();
      const isReturn = kind.includes("return") || kind.includes("reply");
      const isSelf = sourceId === targetId;
      if (isSelf) {
        const path2 = `M${sourceX},${y2} C${sourceX + 84},${y2 - 18} ${sourceX + 84},${y2 + 34} ${sourceX},${y2 + 28}`;
        messageLayer.append("path").attr("class", `sequence-message sequence-message-self${isReturn ? " sequence-message-return" : ""}`).attr("d", path2).style("fill", "none").style("stroke", ctx.theme.edge.default).style("stroke-width", "1.8px").style("stroke-dasharray", isReturn ? "6,4" : "none").style("marker-end", "url(#sequence-arrow-sync)");
      } else {
        messageLayer.append("line").attr("class", `sequence-message${isReturn ? " sequence-message-return" : ""}`).attr("x1", sourceX).attr("y1", y2).attr("x2", targetX).attr("y2", y2).style("stroke", ctx.theme.edge.default).style("stroke-width", "1.8px").style("stroke-dasharray", isReturn ? "6,4" : "none").style("marker-end", "url(#sequence-arrow-sync)");
      }
      const label = truncateLabel(asString3(message.name ?? message.label), 28);
      if (label) {
        messageLayer.append("text").attr("x", (sourceX + targetX) / 2).attr("y", y2 - 8).attr("text-anchor", "middle").style("font-size", "10px").style("fill", ctx.theme.textSecondary).text(label);
      }
    }
    const activationLayer = ctx.root.append("g").attr("class", "sequence-activations");
    for (const activation of activations) {
      const lifelineId = asString3(activation.on_lifeline ?? activation.onLifeline ?? activation.lifeline ?? activation.on);
      const x2 = lifelineX.get(lifelineId);
      if (x2 == null) continue;
      const startRef = asString3(activation.start_message ?? activation.startMessage ?? activation.start);
      const finishRef = asString3(activation.finish_message ?? activation.finishMessage ?? activation.finish);
      const startY = messagePosition.get(startRef)?.y ?? LIFELINE_TOP + 36;
      const finishY = messagePosition.get(finishRef)?.y ?? startY + MESSAGE_GAP;
      activationLayer.append("rect").attr("class", "sequence-activation").attr("x", x2 - 7).attr("y", startY + 6).attr("width", 14).attr("height", Math.max(34, finishY - startY + 18)).attr("rx", 3).style("fill", ctx.theme.nodeFill).style("stroke", ctx.theme.nodeBorder).style("stroke-width", "1px");
    }
    const fragmentLayer = ctx.root.insert("g", ".sequence-messages").attr("class", "sequence-fragments");
    for (const fragment of fragments) {
      const kind = asString3(fragment.kind ?? fragment.type, "fragment");
      const operands = asArray3(fragment.operands).map(asRecord2);
      const referencedMessages = /* @__PURE__ */ new Set();
      for (const operand of operands) {
        asArray3(operand.message_ids ?? operand.messageIds ?? operand.messages).forEach((id2) => referencedMessages.add(asString3(id2)));
      }
      const matching = [...referencedMessages].map((id2) => messagePosition.get(id2)).filter((value) => Boolean(value));
      if (matching.length === 0 && messages.length > 0) {
        matching.push(...[...messagePosition.values()]);
      }
      if (matching.length === 0) continue;
      const minX = Math.min(...matching.map((item) => Math.min(item.sourceX, item.targetX))) - 58;
      const maxX = Math.max(...matching.map((item) => Math.max(item.sourceX, item.targetX))) + 58;
      const minY = Math.min(...matching.map((item) => item.y)) - 34;
      const maxY = Math.max(...matching.map((item) => item.y)) + 34 + Math.max(0, operands.length - 1) * 28;
      const box = fragmentLayer.append("g").attr("class", `sequence-fragment sequence-fragment-${kind.toLowerCase()}`);
      box.append("rect").attr("x", minX).attr("y", minY).attr("width", maxX - minX).attr("height", maxY - minY).attr("rx", 4).style("fill", "none").style("stroke", ctx.theme.nodeBorder).style("stroke-dasharray", "7,4");
      box.append("path").attr("d", `M${minX},${minY + 24} L${minX + 72},${minY + 24} L${minX + 90},${minY} L${minX},${minY}`).style("fill", ctx.theme.canvasBackground).style("stroke", ctx.theme.nodeBorder);
      box.append("text").attr("x", minX + 10).attr("y", minY + 16).style("font-size", "10px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(kind);
      operands.forEach((operand, index) => {
        const guard = asString3(operand.guard ?? operand.condition);
        if (index > 0) {
          const y2 = minY + 28 + index * 28;
          box.append("line").attr("x1", minX).attr("x2", maxX).attr("y1", y2).attr("y2", y2).style("stroke", ctx.theme.nodeBorder).style("stroke-dasharray", "4,3");
        }
        if (guard) {
          box.append("text").attr("class", "sequence-fragment-guard").attr("x", minX + 12).attr("y", minY + 44 + index * 28).style("font-size", "9px").style("fill", ctx.theme.textSecondary).text(`[${truncateLabel(guard, 28)}]`);
        }
      });
    }
    return {
      minX: 0,
      minY: 0,
      maxX: xOffset + lifelines.length * LIFELINE_GAP + 80,
      maxY: lifelineBottom + 60
    };
  }

  // shared/diagram-renderer/src/views/state-transition.ts
  function transitionDisplayLabel2(label) {
    const trimmed = label.trim();
    if (!trimmed || trimmed.toLowerCase() === "entry") return "";
    return trimmed;
  }
  function drawStateNode(group, node, layout, theme) {
    const kind = nodeKind(node);
    const attrs = node.attributes ?? {};
    const regions = Array.isArray(attrs.regions) ? attrs.regions : Array.isArray(attrs.children) ? attrs.children : [];
    const entry = String(attrs.entry ?? attrs.entryAction ?? "").trim();
    const doAction = String(attrs.do ?? attrs.doAction ?? "").trim();
    const exit = String(attrs.exit ?? attrs.exitAction ?? "").trim();
    const isComposite = kind.includes("composite") || regions.length > 0;
    const isTerminate = kind.includes("terminate");
    const g = group.append("g").attr("class", "state-node state-transition-node").attr("data-node-id", node.id).attr("transform", `translate(${layout.x},${layout.y})`);
    if (kind.includes("initial")) {
      g.append("circle").attr("class", "node-background").attr("data-original-stroke", theme.nodeBorder).attr("data-original-width", "2px").attr("cx", layout.width / 2).attr("cy", layout.height / 2).attr("r", layout.width / 2 - 2).style("fill", theme.edge.default).style("stroke", theme.nodeBorder).style("stroke-width", "2px");
    } else if (kind.includes("final") || isTerminate) {
      g.append("circle").attr("class", "node-background").attr("data-original-stroke", theme.nodeBorder).attr("data-original-width", "2px").attr("cx", layout.width / 2).attr("cy", layout.height / 2).attr("r", layout.width / 2 - 2).style("fill", theme.canvasBackground).style("stroke", theme.nodeBorder).style("stroke-width", "2px");
      if (isTerminate) {
        g.append("path").attr("class", "terminate-state-x").attr("d", `M${layout.width / 2 - 9},${layout.height / 2 - 9} L${layout.width / 2 + 9},${layout.height / 2 + 9} M${layout.width / 2 + 9},${layout.height / 2 - 9} L${layout.width / 2 - 9},${layout.height / 2 + 9}`).style("stroke", theme.edge.default).style("stroke-width", "2px");
      } else {
        g.append("circle").attr("cx", layout.width / 2).attr("cy", layout.height / 2).attr("r", 10).style("fill", theme.edge.default).style("stroke", "none");
      }
    } else {
      g.append("rect").attr("class", "node-background").attr("data-original-stroke", theme.nodeBorder).attr("data-original-width", "2px").attr("width", layout.width).attr("height", layout.height).attr("rx", isComposite ? 10 : 14).style("fill", theme.nodeFill).style("stroke", theme.nodeBorder).style("stroke-width", "2px");
      g.append("text").attr("x", layout.width / 2).attr("y", 22).attr("text-anchor", "middle").style("font-size", "12px").style("font-weight", "700").style("fill", theme.textPrimary).text(truncateLabel(node.label, 28));
      const actionLines = [
        entry ? `entry / ${entry}` : "",
        doAction ? `do / ${doAction}` : "",
        exit ? `exit / ${exit}` : ""
      ].filter(Boolean);
      if (actionLines.length > 0 || isComposite) {
        g.append("line").attr("class", "state-compartment-divider").attr("x1", 0).attr("x2", layout.width).attr("y1", 34).attr("y2", 34).style("stroke", theme.nodeBorder).style("stroke-width", "1px");
      }
      actionLines.forEach((line, index) => {
        g.append("text").attr("class", "state-action-compartment").attr("x", 12).attr("y", 54 + index * 16).style("font-size", "10px").style("fill", theme.textSecondary).text(truncateLabel(line, 34));
      });
      if (isComposite) {
        const regionTop = Math.max(80, 52 + actionLines.length * 16);
        const regionHeight = Math.max(32, (layout.height - regionTop - 14) / Math.max(1, regions.length || 1));
        const regionList = regions.length > 0 ? regions : [{ name: "region" }];
        regionList.slice(0, 4).forEach((region, index) => {
          const item = region && typeof region === "object" ? region : { name: String(region) };
          const y2 = regionTop + index * regionHeight;
          g.append("rect").attr("class", "state-region").attr("x", 12).attr("y", y2).attr("width", layout.width - 24).attr("height", Math.max(24, regionHeight - 8)).attr("rx", 5).style("fill", "none").style("stroke", theme.nodeBorder).style("stroke-dasharray", "4,3");
          g.append("text").attr("class", "state-region-label").attr("x", 20).attr("y", y2 + 17).style("font-size", "9px").style("fill", theme.textSecondary).text(truncateLabel(String(item.name ?? item.label ?? `region ${index + 1}`), 28));
        });
      }
    }
    return g;
  }
  async function renderStateTransitionView(ctx) {
    const layoutMode = String(ctx.prepared.meta?.layoutDirection ?? "horizontal").toLowerCase();
    const horizontal = layoutMode !== "vertical" && layoutMode !== "force";
    const layout = await layoutBehaviorGraph(ctx.prepared, { horizontal, mode: "state" });
    const renderOptions = ctx.options ?? {};
    ctx.root.append("text").attr("x", 24).attr("y", 28).style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "State Transition");
    const edgeLayer = ctx.root.append("g").attr("class", "state-transitions");
    const nodeLayer = ctx.root.append("g").attr("class", "state-nodes");
    for (const edge of ctx.prepared.edges) {
      const source = layout.positions.get(edge.source);
      const target = layout.positions.get(edge.target);
      if (!source || !target) continue;
      const sections = layout.edgeSectionsById.get(edge.id);
      const selfLoop = Boolean(edge.attributes?.selfLoop) || edge.source === edge.target;
      const fallback = selfLoop ? buildSelfLoopPath(source) : fallbackEdgePath(source, target, horizontal);
      const path2 = selfLoop ? fallback.path : pathFromSections(sections) || fallback.path;
      const edgeAttrs = edge.attributes ?? {};
      const guard = String(edgeAttrs.guard ?? "").trim();
      const effect = String(edgeAttrs.effect ?? "").trim();
      const accept = String(edgeAttrs.accept ?? "").trim();
      const send = String(edgeAttrs.send ?? "").trim();
      edgeLayer.append("path").attr("class", "state-transition-edge").attr("data-guard", guard || null).attr("data-effect", effect || null).attr("data-accept", accept || null).attr("data-send", send || null).attr("d", path2).style("fill", "none").style("stroke", ctx.theme.edge.default).style("stroke-width", "2px").style("marker-end", "url(#state-transition-arrow)");
      const label = transitionDisplayLabel2(edge.label);
      if (label) {
        const elkLabel = layout.edgeLabelsById.get(edge.id)?.[0];
        const labelFromSections = edgeLabelPositionFromSections(sections);
        const labelPosition = elkLabel ? { x: elkLabel.x + elkLabel.width / 2, y: elkLabel.y + elkLabel.height / 2 } : labelFromSections ?? { x: fallback.labelX, y: fallback.labelY };
        const labelWidth = elkLabel?.width ?? Math.max(42, label.length * 6 + 10);
        const labelHeight = elkLabel?.height ?? 18;
        edgeLayer.append("rect").attr("x", elkLabel ? elkLabel.x : labelPosition.x - labelWidth / 2).attr("y", elkLabel ? elkLabel.y : labelPosition.y - 10).attr("width", labelWidth).attr("height", labelHeight).attr("rx", 4).style("fill", ctx.theme.canvasBackground).style("stroke", ctx.theme.edge.default).style("stroke-width", "1px");
        edgeLayer.append("text").attr("x", labelPosition.x).attr("y", labelPosition.y + 3).attr("text-anchor", "middle").style("font-size", "10px").style("font-weight", "500").style("fill", ctx.theme.edge.default).text(label);
      }
    }
    for (const node of ctx.prepared.nodes) {
      const position = layout.positions.get(node.id);
      if (!position) continue;
      const nodeGroup = drawStateNode(nodeLayer, node, position, ctx.theme);
      attachBehaviorNodeClick(nodeGroup, node, ctx.theme, renderOptions, ctx.root);
    }
    let minX = 0;
    let minY = 0;
    let maxX = ctx.width;
    let maxY = ctx.height;
    layout.positions.forEach((rect) => {
      minX = Math.min(minX, rect.x);
      minY = Math.min(minY, rect.y);
      maxX = Math.max(maxX, rect.x + rect.width);
      maxY = Math.max(maxY, rect.y + rect.height + 20);
    });
    return { minX: minX - 40, minY: minY - 40, maxX: maxX + 40, maxY: maxY + 40 };
  }
  function addStateTransitionMarkers(defs, theme) {
    defs.selectAll("#state-transition-arrow").remove();
    defs.append("marker").attr("id", "state-transition-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 8).attr("refY", 0).attr("markerWidth", 6).attr("markerHeight", 6).attr("orient", "auto").append("path").attr("d", "M0,-5L10,0L0,5").style("fill", theme.edge.default);
  }

  // shared/diagram-renderer/src/views/standard-views-render.ts
  function drawProvisionalBadge(root2, theme, label = "provisional SysML notation") {
    const badge = root2.append("g").attr("class", "provisional-view-badge");
    badge.append("rect").attr("x", 22).attr("y", 42).attr("width", 176).attr("height", 24).attr("rx", 5).style("fill", theme.canvasBackground).style("stroke", theme.edge.default).style("stroke-dasharray", "4,3");
    badge.append("text").attr("x", 34).attr("y", 58).style("font-size", "10px").style("fill", theme.textSecondary).text(label);
  }
  function nodeFromMeta(row, fallback) {
    const id2 = asString(row.id);
    return fallback.find((node) => node.id === id2 || node.label === asString(row.label ?? row.name));
  }
  function shortMatrixLabel(id2) {
    const segments = id2.split("::").filter(Boolean);
    return truncateLabel(segments[segments.length - 1] ?? id2, 10);
  }
  function renderBrowserView(ctx) {
    const rows = asArray(ctx.prepared.meta?.rows).map(asRecord);
    const sourceRows = rows.length > 0 ? rows : ctx.prepared.nodes.map((node) => ({ id: node.id, label: node.label, kind: node.kind }));
    const hierarchyLayout = Boolean(ctx.prepared.meta?.hierarchyLayout);
    const rowHeight = 28;
    const left = 52;
    const top = 88;
    const width = Math.max(520, Math.min(920, ctx.width - 120));
    const collapsed = /* @__PURE__ */ new Set();
    ctx.root.append("text").attr("x", 24).attr("y", 28).style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "Browser View");
    if (!hierarchyLayout) {
      drawProvisionalBadge(ctx.root, ctx.theme);
    }
    const layer = ctx.root.append("g").attr("class", "browser-view-rows");
    const isRowVisible = (row, index) => {
      if (!hierarchyLayout) return true;
      const parentId = asString(row.parentId);
      if (!parentId) return true;
      for (let cursor = index - 1; cursor >= 0; cursor -= 1) {
        const ancestor = asRecord(sourceRows[cursor]);
        if (asString(ancestor.id) !== parentId) continue;
        if (!isRowVisible(ancestor, cursor) || collapsed.has(parentId)) {
          return false;
        }
        return true;
      }
      return !collapsed.has(parentId);
    };
    const redraw = () => {
      layer.selectAll("*").remove();
      let visibleIndex = 0;
      sourceRows.forEach((row, index) => {
        if (!isRowVisible(row, index)) return;
        const y2 = top + visibleIndex * rowHeight;
        visibleIndex += 1;
        const depth = hierarchyLayout ? Number(row.depth ?? 0) : Math.max(0, asString(row.qualifiedName).split("::").filter(Boolean).length - 1);
        const hasChildren = Boolean(row.hasChildren);
        const preparedNode = nodeFromMeta(row, ctx.prepared.nodes);
        const rowId = preparedNode?.id ?? asString(row.id, `browser-row-${index}`);
        const item = layer.append("g").attr("class", "browser-row").attr("data-node-id", rowId).attr("transform", `translate(${left},${y2})`);
        item.append("rect").attr("class", "node-background").attr("data-original-stroke", ctx.theme.nodeBorder).attr("data-original-width", "1px").attr("width", width).attr("height", rowHeight - 3).attr("rx", 4).style("fill", visibleIndex % 2 === 0 ? ctx.theme.nodeFill : ctx.theme.canvasBackground).style("stroke", ctx.theme.nodeBorder).style("stroke-width", "1px").style("opacity", 0.9);
        if (hasChildren) {
          const toggle = item.append("text").attr("x", 8 + depth * 16).attr("y", 18).attr("class", "browser-toggle").style("font-size", "11px").style("font-weight", "700").style("fill", ctx.theme.textSecondary).style("cursor", "pointer").text(collapsed.has(rowId) ? "\u25B8" : "\u25BE");
          toggle.on("click", (event) => {
            event.stopPropagation();
            if (collapsed.has(rowId)) {
              collapsed.delete(rowId);
            } else {
              collapsed.add(rowId);
            }
            redraw();
          });
        }
        item.append("text").attr("x", 14 + depth * 16 + (hasChildren ? 12 : 0)).attr("y", 18).style("font-size", "11px").style("font-weight", "600").style("fill", ctx.theme.textPrimary).text(truncateLabel(asString(row.label ?? row.name ?? row.id, "Unnamed"), 48));
        item.append("text").attr("x", width - 14).attr("y", 18).attr("text-anchor", "end").style("font-size", "10px").style("fill", ctx.theme.textSecondary).text(truncateLabel(asString(row.kind, "element"), 24));
        if (preparedNode) {
          attachBehaviorNodeClick(item, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
        }
      });
    };
    redraw();
    const visibleCount = hierarchyLayout ? sourceRows.filter((row, index) => isRowVisible(row, index)).length : sourceRows.length;
    return { minX: 0, minY: 0, maxX: left + width + 80, maxY: top + visibleCount * rowHeight + 80 };
  }
  function renderGridView(ctx) {
    const relationshipMatrix = Boolean(ctx.prepared.meta?.relationshipMatrix);
    if (relationshipMatrix) {
      return renderRelationshipMatrix(ctx);
    }
    const cells = asArray(ctx.prepared.meta?.cells).map(asRecord);
    const rows = cells.length > 0 ? cells : ctx.prepared.nodes.map((node) => asRecord(node.attributes));
    const left = 52;
    const top = 92;
    const traceabilityTable = Boolean(ctx.prepared.meta?.traceabilityTable);
    const columns = traceabilityTable ? [
      { key: "name", label: "Name", width: 240 },
      { key: "kind", label: "Kind", width: 130 },
      { key: "package", label: "Package", width: 170 },
      { key: "linkCount", label: "Links", width: 70 }
    ] : [
      { key: "name", label: "Name", width: 220 },
      { key: "kind", label: "Kind", width: 150 },
      { key: "attributeCount", label: "Attrs", width: 80 },
      { key: "partCount", label: "Parts", width: 80 },
      { key: "portCount", label: "Ports", width: 80 }
    ];
    const tableWidth = columns.reduce((sum, column) => sum + column.width, 0);
    const rowHeight = 30;
    ctx.root.append("text").attr("x", 24).attr("y", 28).style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "Grid View");
    if (Boolean(ctx.prepared.meta?.provisional)) {
      drawProvisionalBadge(ctx.root, ctx.theme);
    }
    const table = ctx.root.append("g").attr("class", "grid-view-table").attr("transform", `translate(${left},${top})`);
    let x2 = 0;
    columns.forEach((column) => {
      table.append("rect").attr("class", "grid-header-cell").attr("x", x2).attr("width", column.width).attr("height", rowHeight).style("fill", ctx.theme.nodeBorder).style("stroke", ctx.theme.nodeBorder);
      table.append("text").attr("x", x2 + 10).attr("y", 20).style("font-size", "11px").style("font-weight", "700").style("fill", ctx.theme.canvasBackground).text(column.label);
      x2 += column.width;
    });
    rows.forEach((row, rowIndex) => {
      x2 = 0;
      const preparedNode = nodeFromMeta(row, ctx.prepared.nodes);
      const group = table.append("g").attr("class", "grid-row").attr("data-node-id", preparedNode?.id ?? asString(row.id, `grid-row-${rowIndex}`)).attr("transform", `translate(0,${(rowIndex + 1) * rowHeight})`);
      columns.forEach((column) => {
        group.append("rect").attr("class", "grid-cell").attr("x", x2).attr("width", column.width).attr("height", rowHeight).style("fill", rowIndex % 2 === 0 ? ctx.theme.nodeFill : ctx.theme.canvasBackground).style("stroke", ctx.theme.nodeBorder).style("stroke-width", "1px");
        group.append("text").attr("x", x2 + 10).attr("y", 20).style("font-size", "10px").style("fill", ctx.theme.textPrimary).text(truncateLabel(asString(row[column.key]), column.width > 100 ? 28 : 8));
        x2 += column.width;
      });
      if (preparedNode) {
        attachBehaviorNodeClick(group, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
      }
    });
    return { minX: 0, minY: 0, maxX: left + tableWidth + 80, maxY: top + (rows.length + 2) * rowHeight + 80 };
  }
  function renderRelationshipMatrix(ctx) {
    const rowIds = asArray(ctx.prepared.meta?.matrixRowIds).map((value) => asString(value)).filter(Boolean);
    const colIds = asArray(ctx.prepared.meta?.matrixColIds).map((value) => asString(value)).filter(Boolean);
    const matrixCells = asArray(ctx.prepared.meta?.matrixCells).map(asRecord);
    const cellSize = 34;
    const headerSize = 120;
    const left = 180;
    const top = 92;
    ctx.root.append("text").attr("x", 24).attr("y", 28).style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "Relationship Matrix");
    const layer = ctx.root.append("g").attr("class", "grid-relationship-matrix").attr("transform", `translate(${left},${top})`);
    colIds.forEach((colId, colIndex) => {
      layer.append("text").attr("x", headerSize + colIndex * cellSize + cellSize / 2).attr("y", 16).attr("text-anchor", "middle").attr("transform", `rotate(-35, ${headerSize + colIndex * cellSize + cellSize / 2}, 16)`).style("font-size", "9px").style("fill", ctx.theme.textSecondary).text(shortMatrixLabel(colId));
    });
    rowIds.forEach((rowId, rowIndex) => {
      layer.append("text").attr("x", headerSize - 8).attr("y", headerSize + rowIndex * cellSize + cellSize / 2 + 4).attr("text-anchor", "end").style("font-size", "10px").style("fill", ctx.theme.textPrimary).text(shortMatrixLabel(rowId));
      colIds.forEach((colId, colIndex) => {
        const cell = matrixCells.find(
          (entry) => asString(entry.source) === rowId && asString(entry.target) === colId
        );
        const present = Boolean(cell?.present);
        const x2 = headerSize + colIndex * cellSize;
        const y2 = headerSize + rowIndex * cellSize;
        layer.append("rect").attr("x", x2).attr("y", y2).attr("width", cellSize - 2).attr("height", cellSize - 2).style("fill", present ? ctx.theme.nodeFill : ctx.theme.canvasBackground).style("stroke", ctx.theme.nodeBorder).style("stroke-width", "1px");
        if (present) {
          layer.append("text").attr("x", x2 + (cellSize - 2) / 2).attr("y", y2 + (cellSize - 2) / 2 + 4).attr("text-anchor", "middle").style("font-size", "12px").style("font-weight", "700").style("fill", ctx.theme.edge.default).text("\u25CF");
        }
      });
    });
    const width = headerSize + colIds.length * cellSize + 40;
    const height = headerSize + rowIds.length * cellSize + 40;
    return { minX: 0, minY: 0, maxX: left + width, maxY: top + height };
  }
  function renderGeometryView(ctx) {
    const elements = asArray(ctx.prepared.meta?.elements).map(asRecord);
    const nodes = elements.length > 0 ? elements : ctx.prepared.nodes.map((node) => ({ id: node.id, label: node.label, kind: node.kind }));
    const geometryMode = asString(ctx.prepared.meta?.geometryMode, "2d");
    const geometryProjection = asString(ctx.prepared.meta?.geometryProjection, "orthographic");
    const left = 64;
    const top = 88;
    const cellWidth = 128;
    const cellHeight = 72;
    const columns = Math.max(1, Math.ceil(Math.sqrt(nodes.length)));
    ctx.root.append("text").attr("x", 24).attr("y", 28).style("font-size", "14px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(ctx.prepared.title || "Geometry View");
    if (Boolean(ctx.prepared.meta?.provisional)) {
      drawProvisionalBadge(ctx.root, ctx.theme, `${geometryMode} ${geometryProjection} preview`);
    }
    const layer = ctx.root.append("g").attr("class", "geometry-view-scene").attr("transform", `translate(${left},${top})`);
    layer.append("rect").attr("width", columns * cellWidth + 24).attr("height", Math.ceil(nodes.length / columns) * cellHeight + 24).attr("rx", 8).style("fill", "none").style("stroke", ctx.theme.frame.stroke).style("stroke-dasharray", "8,6");
    nodes.forEach((node, index) => {
      const col = index % columns;
      const row = Math.floor(index / columns);
      const x2 = col * cellWidth + 12;
      const y2 = row * cellHeight + 12;
      const preparedNode = nodeFromMeta(node, ctx.prepared.nodes);
      const item = layer.append("g").attr("class", "geometry-object").attr("data-node-id", preparedNode?.id ?? asString(node.id, `geometry-node-${index}`)).attr("transform", `translate(${x2},${y2})`);
      item.append("rect").attr("class", "node-background").attr("data-original-stroke", ctx.theme.nodeBorder).attr("data-original-width", "1.5px").attr("width", cellWidth - 20).attr("height", cellHeight - 16).attr("rx", 6).style("fill", ctx.theme.nodeFill).style("stroke", ctx.theme.nodeBorder).style("stroke-width", "1.5px");
      item.append("text").attr("x", (cellWidth - 20) / 2).attr("y", 24).attr("text-anchor", "middle").style("font-size", "10px").style("font-weight", "700").style("fill", ctx.theme.textPrimary).text(truncateLabel(asString(node.label ?? node.name ?? node.id), 16));
      item.append("text").attr("x", (cellWidth - 20) / 2).attr("y", 42).attr("text-anchor", "middle").style("font-size", "8px").style("fill", ctx.theme.textSecondary).text(truncateLabel(asString(node.kind, "element"), 18));
      if (preparedNode) {
        attachBehaviorNodeClick(item, preparedNode, ctx.theme, ctx.options ?? {}, ctx.root);
      }
    });
    const width = columns * cellWidth + 80;
    const height = Math.ceil(nodes.length / columns) * cellHeight + 120;
    return { minX: 0, minY: 0, maxX: left + width, maxY: top + height };
  }

  // shared/diagram-renderer/src/render/types.ts
  var nodeWidth = 200;
  var nodeHeight = 70;
  var ibdNodeWidth = 280;
  var ibdNodeHeight = 140;
  function contentBoundsFromExtents(extents) {
    const width = extents.maxX - extents.minX;
    const height = extents.maxY - extents.minY;
    return {
      x: extents.minX,
      y: extents.minY,
      width: width > 0 ? width : 1,
      height: height > 0 ? height : 1
    };
  }
  function compareIbdPorts(node, a, b, usageForPort) {
    const usageA = usageForPort(node, a);
    const usageB = usageForPort(node, b);
    const degreeA = usageA.sourceCount + usageA.targetCount;
    const degreeB = usageB.sourceCount + usageB.targetCount;
    if (degreeB !== degreeA) return degreeB - degreeA;
    return a.name.localeCompare(b.name);
  }
  function splitIbdPortsBySide(node, ports, sideForPort, usageForPort) {
    const west = [];
    const east = [];
    for (const port of ports) {
      (sideForPort(port, node) === "WEST" ? west : east).push(port);
    }
    const compare = (a, b) => compareIbdPorts(node, a, b, usageForPort);
    west.sort(compare);
    east.sort(compare);
    return { west, east };
  }
  function computeIbdLeafHeight(node, ports, portRows) {
    const attrs = node.attributes ?? {};
    const headerHeight = attrs.partType ? 50 : 38;
    const children2 = Array.isArray(attrs.children) ? attrs.children : [];
    const contentLineCount = children2.filter(
      (child) => child && typeof child === "object" && String(child.name || "")
    ).length;
    const contentHeight = Math.min(contentLineCount, 8) * 12 + 10;
    const portSpacing = 26;
    const portsHeight = ports.length > 0 ? portRows * portSpacing + 22 : 0;
    return Math.min(340, Math.max(ibdNodeHeight, headerHeight + contentHeight + portsHeight));
  }

  // shared/diagram-renderer/src/render/export.ts
  function contentBounds(layout) {
    if (!layout.nodes.length) return { x: 0, y: 0, width: 100, height: 100 };
    const minX = Math.min(...layout.nodes.map((node) => node.x || 0));
    const minY = Math.min(...layout.nodes.map((node) => node.y || 0));
    const maxX = Math.max(...layout.nodes.map((node) => (node.x || 0) + (node.width || nodeWidth)));
    const maxY = Math.max(...layout.nodes.map((node) => (node.y || 0) + (node.height || nodeHeight)));
    return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
  }
  function applyFit(svg, zoom, root2, bounds, width, height, isInterconnectionView = false, delegateZoom = false) {
    const padding = 48;
    const minScale = isInterconnectionView ? 0.2 : 0.08;
    const maxScale = isInterconnectionView ? 1.1 : 1.3;
    const scale = Math.min(
      maxScale,
      Math.max(minScale, Math.min((width - padding * 2) / bounds.width, (height - padding * 2) / bounds.height))
    );
    const tx = (width - bounds.width * scale) / 2 - bounds.x * scale;
    const ty = (height - bounds.height * scale) / 2 - bounds.y * scale;
    const transform2 = identity2.translate(tx, ty).scale(scale);
    if (delegateZoom) {
      root2.attr("transform", transform2.toString());
      return transform2;
    }
    svg.transition().duration(180).call(zoom.transform, transform2);
    return transform2;
  }
  function addMarkers(svg, theme) {
    const defs = svg.append("defs");
    defs.append("marker").attr("id", "viz-arrow").attr("markerWidth", 10).attr("markerHeight", 10).attr("refX", 9).attr("refY", 3).attr("orient", "auto").attr("markerUnits", "strokeWidth").append("path").attr("d", "M0,0 L0,6 L9,3 z").attr("fill", theme.edge.default);
    defs.append("marker").attr("id", "general-d3-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 8).attr("refY", 0).attr("markerWidth", 5).attr("markerHeight", 5).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4").style("fill", theme.edge.default);
    defs.append("marker").attr("id", "general-d3-arrow-open").attr("viewBox", "0 -5 10 10").attr("refX", 9).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4").style("fill", "none").style("stroke", theme.edge.default).style("stroke-width", "1.3");
    defs.append("marker").attr("id", "general-d3-specializes").attr("viewBox", "0 -6 12 12").attr("refX", 11).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,0L10,-4L10,4Z").style("fill", theme.nodeFill).style("stroke", theme.edge.default).style("stroke-width", "1.2");
    defs.append("marker").attr("id", "general-d3-diamond").attr("viewBox", "0 -6 12 12").attr("refX", 2).attr("refY", 0).attr("markerWidth", 7).attr("markerHeight", 7).attr("orient", "auto").append("path").attr("d", "M0,0L5,-4L10,0L5,4Z").style("fill", theme.edge.default);
    defs.append("marker").attr("id", "ibd-connection-dot").attr("viewBox", "-5 -5 10 10").attr("refX", 0).attr("refY", 0).attr("markerWidth", 5).attr("markerHeight", 5).attr("orient", "auto").append("circle").attr("r", 3).style("fill", theme.nodeFill).style("stroke", theme.edge.default).style("stroke-width", "1.5");
    defs.append("marker").attr("id", "ibd-flow-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 10).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4Z").style("fill", theme.edge.default);
    defs.append("marker").attr("id", "ibd-interface-arrow").attr("viewBox", "0 -5 10 10").attr("refX", 10).attr("refY", 0).attr("markerWidth", 8).attr("markerHeight", 8).attr("orient", "auto").append("path").attr("d", "M0,-4L10,0L0,4Z").style("fill", "none").style("stroke", theme.edge.default).style("stroke-width", "1.5");
  }
  function exportSvg(svgNode2, bounds) {
    const clone = svgNode2.cloneNode(true);
    clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
    clone.setAttribute("viewBox", `${bounds.x - 40} ${bounds.y - 40} ${bounds.width + 80} ${bounds.height + 80}`);
    return new XMLSerializer().serializeToString(clone);
  }

  // shared/diagram-renderer/src/sysml-node-builder.ts
  var LINE_HEIGHT = 12;
  var COMPARTMENT_LABEL_HEIGHT = 14;
  var COMPARTMENT_GAP = 2;
  var COMPARTMENT_PADDING = 4;
  var HEADER_COMPARTMENT_HEIGHT = 44;
  var TYPED_BY_HEIGHT = 14;
  var PADDING = 6;
  var SHOW_MORE_LINE_HEIGHT = 12;
  var DEFAULT_SYSML_NODE_CONFIG = {
    showHeader: true,
    showAttributes: true,
    showParts: true,
    showPorts: true,
    showOther: true,
    maxLinesPerCompartment: 8
  };
  var DEFAULT_CONFIG = DEFAULT_SYSML_NODE_CONFIG;
  function asString4(value, fallback = "") {
    if (typeof value === "string") return value;
    if (typeof value === "number" || typeof value === "boolean") return String(value);
    return fallback;
  }
  function asArray4(value) {
    return Array.isArray(value) ? value : [];
  }
  function normalizeUnitBrackets(text) {
    let out = text;
    while (/\[\[[^\[\]]+\]\]/.test(out)) {
      out = out.replace(/\[\[([^\[\]]+)\]\]/g, "[$1]");
    }
    return out;
  }
  function normalizeDetailItem(item) {
    if (typeof item === "string") {
      const text = normalizeUnitBrackets(item.trim());
      return text ? { name: text, displayText: text } : null;
    }
    if (!item || typeof item !== "object") return null;
    const record = item;
    const name = asString4(record.name).trim();
    const displayText = normalizeUnitBrackets(asString4(record.displayText, name).trim());
    if (!displayText) return null;
    return {
      name: name || displayText,
      typeName: asString4(record.typeName) || null,
      valueText: asString4(record.valueText) || null,
      declaredIn: asString4(record.declaredIn) || null,
      displayText
    };
  }
  function detailItems(attributes, key) {
    return asArray4(attributes[key]).map((item) => normalizeDetailItem(item)).filter((item) => Boolean(item));
  }
  function fallbackDetailItems(attributes, key) {
    return asArray4(attributes[key]).map((item) => normalizeDetailItem(item)).filter((item) => Boolean(item));
  }
  function collectCompartments(node) {
    const attributes = node.attributes ?? {};
    const typedByName = asString4(attributes.partType) || asString4(attributes.type) || asString4(attributes.typedBy) || asString4(attributes.typing) || null;
    const directAttributes = detailItems(attributes, "generalViewDirectAttributes");
    const directParts = detailItems(attributes, "generalViewDirectParts");
    const directPorts = detailItems(attributes, "generalViewDirectPorts");
    const inheritedAttributes = detailItems(attributes, "generalViewInheritedAttributes");
    const inheritedParts = detailItems(attributes, "generalViewInheritedParts");
    const packageMembers = [
      ...detailItems(attributes, "generalViewPackageMembers"),
      ...detailItems(attributes, "packageMembers"),
      ...detailItems(attributes, "members")
    ];
    const imports = [
      ...detailItems(attributes, "generalViewImports"),
      ...detailItems(attributes, "imports")
    ];
    const collapsibleSections = [];
    if (inheritedAttributes.length > 0) {
      collapsibleSections.push({
        key: "inherited-attributes",
        title: "Inherited Attributes",
        items: inheritedAttributes,
        collapsed: true
      });
    }
    if (inheritedParts.length > 0) {
      collapsibleSections.push({
        key: "inherited-parts",
        title: "Inherited Parts",
        items: inheritedParts,
        collapsed: true
      });
    }
    if (packageMembers.length > 0) {
      collapsibleSections.push({
        key: "package-members",
        title: "Members",
        items: packageMembers,
        collapsed: false
      });
    }
    if (imports.length > 0) {
      collapsibleSections.push({
        key: "imports",
        title: "Imports",
        items: imports,
        collapsed: true
      });
    }
    return {
      header: { stereotype: node.kind.toLowerCase() || "element", name: node.label || "Unnamed" },
      typedByName,
      attributes: directAttributes.length > 0 ? directAttributes : fallbackDetailItems(attributes, "attributes"),
      parts: directParts.length > 0 ? directParts : fallbackDetailItems(attributes, "parts"),
      ports: directPorts.length > 0 ? directPorts : fallbackDetailItems(attributes, "ports"),
      collapsibleSections
    };
  }
  function computeNodeHeight(compartments, config = {}) {
    const cfg = { ...DEFAULT_CONFIG, ...config };
    let height = PADDING * 2;
    if (cfg.showHeader) {
      height += HEADER_COMPARTMENT_HEIGHT;
      if (compartments.typedByName) height += TYPED_BY_HEIGHT;
    }
    const hasBodyCompartments = cfg.showAttributes && compartments.attributes.length > 0 || cfg.showParts && compartments.parts.length > 0 || cfg.showPorts && compartments.ports.length > 0 || !!compartments.collapsibleSections?.some((section) => section.items.length > 0) || cfg.showOther && !!compartments.other?.some((section) => section.lines.length > 0);
    if (cfg.showHeader && hasBodyCompartments) {
      height += COMPARTMENT_PADDING;
    }
    const addCompartment = (items) => {
      if (items.length === 0) return;
      const shown = cfg.maxLinesPerCompartment ? Math.min(items.length, cfg.maxLinesPerCompartment) : items.length;
      height += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + shown * LINE_HEIGHT + COMPARTMENT_GAP;
      if (cfg.maxLinesPerCompartment && items.length > cfg.maxLinesPerCompartment) {
        height += SHOW_MORE_LINE_HEIGHT;
      }
    };
    const addCollapsibleSection = (section) => {
      if (section.items.length === 0) return;
      height += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + COMPARTMENT_GAP;
      if (!section.collapsed) {
        const shown = section.showAll || !cfg.maxLinesPerCompartment ? section.items.length : Math.min(section.items.length, cfg.maxLinesPerCompartment);
        height += shown * LINE_HEIGHT;
        if (cfg.maxLinesPerCompartment && section.items.length > cfg.maxLinesPerCompartment) {
          height += SHOW_MORE_LINE_HEIGHT;
        }
      }
    };
    if (cfg.showAttributes) addCompartment(compartments.attributes);
    if (cfg.showParts) addCompartment(compartments.parts);
    if (cfg.showPorts) addCompartment(compartments.ports);
    for (const section of compartments.collapsibleSections ?? []) {
      addCollapsibleSection(section);
    }
    if (cfg.showOther && compartments.other?.length) {
      for (const section of compartments.other) {
        const shown = cfg.maxLinesPerCompartment ? Math.min(section.lines.length, cfg.maxLinesPerCompartment) : section.lines.length;
        height += COMPARTMENT_PADDING * 2 + COMPARTMENT_LABEL_HEIGHT + shown * LINE_HEIGHT + COMPARTMENT_GAP;
      }
    }
    return Math.max(60, height);
  }
  function truncate(value, max2) {
    return value.length > max2 ? `${value.slice(0, max2 - 2)}..` : value;
  }
  function formatStereotype(type2) {
    return `\xAB${type2.replace(/_/g, " ")}\xBB`;
  }
  function renderSysMLNode(parent, compartments, options) {
    const cfg = { ...DEFAULT_CONFIG, ...options.config ?? {} };
    const theme = options.theme;
    const nodeFill = theme?.nodeFill ?? "var(--vscode-editor-background)";
    const panelBackground = theme?.panelBackground ?? "var(--vscode-button-secondaryBackground)";
    const textPrimary = theme?.textPrimary ?? "var(--vscode-editor-foreground)";
    const textSecondary = theme?.textSecondary ?? "var(--vscode-descriptionForeground)";
    const divider = theme?.divider ?? "var(--vscode-panel-border)";
    const highlight = theme?.highlight ?? "#FFD700";
    const chrome = options.chrome ?? resolveNodeChrome(options.kind ?? "", {
      isDefinition: options.isDefinition,
      isReference: options.isReference
    });
    const node = parent.append("g").attr(
      "class",
      `${options.nodeClass}${chrome.nodeClassSuffix}${options.selected ? " is-selected" : ""}`
    ).attr("transform", `translate(${options.x},${options.y})`).attr("data-element-name", options.dataElementName);
    const body = nodeBodyChromeStyle(chrome, { selected: options.selected, generalView: true });
    node.append("rect").attr("width", options.width).attr("height", options.height).attr("rx", body.cornerRadius).attr("class", "graph-node-background sysml-node-bg").attr("data-original-stroke", options.strokeColor).attr("data-original-width", `${body.strokeWidthPx}px`).style("fill", nodeFill).style("stroke", options.selected ? highlight : options.strokeColor).style("stroke-width", `${body.strokeWidthPx}px`).style("stroke-dasharray", body.strokeDasharray);
    const headerHeight = HEADER_COMPARTMENT_HEIGHT + (compartments.typedByName ? TYPED_BY_HEIGHT : 0);
    const headerRx = body.headerCornerRadius;
    node.append("rect").attr("y", 0).attr("width", options.width).attr("height", headerHeight).attr("rx", headerRx).attr("class", "sysml-header-compartment").style("fill", panelBackground);
    node.append("text").attr("x", options.width / 2).attr("y", 17).attr("text-anchor", "middle").text(formatStereotype(compartments.header.stereotype)).style("font-size", "9px").style("fill", options.strokeColor);
    node.append("text").attr("class", "node-name-text viz-node-name").attr("x", options.width / 2).attr("y", 31).attr("text-anchor", "middle").text(truncate(compartments.header.name, 26)).style("font-size", "11px").style("font-weight", "bold").style("fill", textPrimary);
    if (compartments.typedByName) {
      node.append("text").attr("x", options.width / 2).attr("y", 43).attr("text-anchor", "middle").text(`: ${truncate(compartments.typedByName, 22)}`).style("font-size", "10px").style("font-style", "italic").style("fill", options.strokeColor);
    }
    let contentY = headerHeight + COMPARTMENT_PADDING;
    const renderCompartment = (title, items, collapsed = false) => {
      if (items.length === 0) return;
      const limit = cfg.maxLinesPerCompartment ? Math.min(items.length, cfg.maxLinesPerCompartment) : items.length;
      const shownItems = collapsed ? [] : items.slice(0, limit);
      node.append("line").attr("x1", PADDING).attr("y1", contentY).attr("x2", options.width - PADDING).attr("y2", contentY).attr("class", "sysml-compartment-divider").style("stroke", divider).style("stroke-width", "1px");
      contentY += 4;
      node.append("text").attr("x", PADDING).attr("y", contentY + 9).text(collapsed ? `> ${title}` : title).style("font-size", "9px").style("font-weight", "bold").style("fill", textSecondary);
      contentY += COMPARTMENT_LABEL_HEIGHT;
      for (const item of shownItems) {
        node.append("text").attr("x", PADDING).attr("y", contentY + 9).text(truncate(item.displayText, 32)).style("font-size", "9px").style("fill", textSecondary).append("title").text(item.declaredIn ? `${item.displayText} (from ${item.declaredIn})` : item.displayText);
        contentY += LINE_HEIGHT;
      }
      if (!collapsed && cfg.maxLinesPerCompartment && items.length > cfg.maxLinesPerCompartment) {
        node.append("text").attr("x", PADDING).attr("y", contentY + 9).text(`+${items.length - cfg.maxLinesPerCompartment} more`).style("font-size", "9px").style("font-weight", "bold").style("fill", options.strokeColor);
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

  // shared/diagram-renderer/src/render/ibd-route.ts
  function pruneRoutePoints(points) {
    const pruned = [];
    for (const point of points) {
      const last = pruned[pruned.length - 1];
      if (last && Math.abs(last.x - point.x) < 1e-6 && Math.abs(last.y - point.y) < 1e-6) {
        continue;
      }
      pruned.push({ x: point.x, y: point.y });
      while (pruned.length >= 3) {
        const a = pruned[pruned.length - 3];
        const b = pruned[pruned.length - 2];
        const c = pruned[pruned.length - 1];
        const sameX = Math.abs(a.x - b.x) < 1e-6 && Math.abs(b.x - c.x) < 1e-6;
        const sameY = Math.abs(a.y - b.y) < 1e-6 && Math.abs(b.y - c.y) < 1e-6;
        if (!sameX && !sameY) break;
        pruned.splice(pruned.length - 2, 1);
      }
    }
    return pruned;
  }
  function pointsFromElkSections(sections, offset) {
    const points = [];
    for (const section of sections) {
      if (section.startPoint) {
        points.push({ x: section.startPoint.x + offset.x, y: section.startPoint.y + offset.y });
      }
      for (const bend of section.bendPoints ?? []) {
        points.push({ x: bend.x + offset.x, y: bend.y + offset.y });
      }
      if (section.endPoint) {
        points.push({ x: section.endPoint.x + offset.x, y: section.endPoint.y + offset.y });
      }
    }
    return pruneRoutePoints(points);
  }
  function containerChain(node, nodesById) {
    const chain = [];
    let current = node;
    while (current) {
      chain.push(current.id);
      const parentId = String(current.attributes?.containerId ?? "");
      const parentNode = parentId && nodesById.has(parentId) ? nodesById.get(parentId) : void 0;
      current = parentNode;
    }
    return chain;
  }
  function lcaOffsetForNodes(sourceNode, targetNode, laidOutNodes) {
    const sourceChain = containerChain(sourceNode, laidOutNodes);
    const targetSet = new Set(containerChain(targetNode, laidOutNodes));
    const lcaId = sourceChain.find((id2) => targetSet.has(id2));
    if (!lcaId) return { x: 0, y: 0 };
    const lca = laidOutNodes.get(lcaId);
    return lca ? { x: lca.x ?? 0, y: lca.y ?? 0 } : { x: 0, y: 0 };
  }
  function uniqueOffsets(offsets) {
    const seen = /* @__PURE__ */ new Set();
    const unique = [];
    for (const offset of offsets) {
      const key = `${offset.x.toFixed(3)},${offset.y.toFixed(3)}`;
      if (seen.has(key)) continue;
      seen.add(key);
      unique.push(offset);
    }
    return unique;
  }
  function routeEndpointError(points, source, target) {
    if (points.length < 2) return Number.POSITIVE_INFINITY;
    const start2 = points[0];
    const end = points[points.length - 1];
    return Math.hypot(start2.x - source.x, start2.y - source.y) + Math.hypot(end.x - target.x, end.y - target.y);
  }
  function samePoint(a, b) {
    return Math.abs(a.x - b.x) < 1e-6 && Math.abs(a.y - b.y) < 1e-6;
  }
  function isOrthogonalSegment(a, b) {
    return Math.abs(a.x - b.x) < 1e-6 || Math.abs(a.y - b.y) < 1e-6;
  }
  function stitchOrthogonalEndpoint(endpoint, routePoint) {
    if (samePoint(endpoint, routePoint)) return [{ x: endpoint.x, y: endpoint.y }];
    if (isOrthogonalSegment(endpoint, routePoint)) {
      return [
        { x: endpoint.x, y: endpoint.y },
        { x: routePoint.x, y: routePoint.y }
      ];
    }
    return [
      { x: endpoint.x, y: endpoint.y },
      { x: routePoint.x, y: endpoint.y },
      { x: routePoint.x, y: routePoint.y }
    ];
  }
  function snapRouteEndpoints(points, source, target) {
    if (points.length < 2) return points;
    let route = points.map((point) => ({ x: point.x, y: point.y }));
    if (source) {
      route = [...stitchOrthogonalEndpoint(source, route[0]), ...route.slice(1)];
    }
    if (target) {
      const lastRoutePoint = route[route.length - 1];
      const targetStitch = stitchOrthogonalEndpoint(target, lastRoutePoint).reverse();
      route = [...route.slice(0, -1), ...targetStitch];
    }
    return pruneRoutePoints(route);
  }
  function resolveRouteOffsetCandidates(edge) {
    const edgeOwnerOffset = edge.layout?.edgeOwnerOffset ?? { x: 0, y: 0 };
    const lcaOffset = edge.layout?.lcaOffset ?? { x: 0, y: 0 };
    return uniqueOffsets([
      { x: 0, y: 0 },
      edgeOwnerOffset,
      lcaOffset,
      { x: edgeOwnerOffset.x + lcaOffset.x, y: edgeOwnerOffset.y + lcaOffset.y }
    ]);
  }
  function resolveIbdRoutePoints(edge) {
    const sections = edge.layout?.sections;
    if (!sections?.length) return null;
    const attrs = edge.attributes ?? {};
    const sourcePort = attrs._sourcePortCenter ?? null;
    const targetPort = attrs._targetPortCenter ?? null;
    const candidates = resolveRouteOffsetCandidates(edge);
    let bestPoints = null;
    let bestError = Number.POSITIVE_INFINITY;
    for (const offset of candidates) {
      const points = pointsFromElkSections(sections, offset);
      if (points.length < 2) continue;
      const error = sourcePort && targetPort ? routeEndpointError(points, sourcePort, targetPort) : offset.x === 0 && offset.y === 0 ? 0 : Math.hypot(offset.x, offset.y);
      if (error < bestError) {
        bestError = error;
        bestPoints = points;
      }
    }
    if (!bestPoints) return null;
    return snapRouteEndpoints(bestPoints, sourcePort, targetPort);
  }

  // shared/diagram-renderer/src/render/interconnection-layout-dto.ts
  function createInterconnectionLayoutBuildState() {
    return { nodes: /* @__PURE__ */ new Map(), containers: [], diagnostics: [] };
  }
  function recordInterconnectionLayoutContainer(state, container) {
    state.containers.push(container);
  }
  function recordInterconnectionLayoutNode(state, node, portAnchors, portDrawOrder) {
    state.nodes.set(node.id, {
      ...node,
      portAnchors,
      portDrawOrder
    });
  }
  function finalizeInterconnectionLayoutDto(state, edges) {
    return {
      nodes: Array.from(state.nodes.values()),
      edges: edges.map((edge) => ({
        id: edge.id,
        routePoints: resolveIbdRoutePoints(edge) ?? [],
        sourcePortId: String(edge.attributes?.sourcePortId ?? ""),
        targetPortId: String(edge.attributes?.targetPortId ?? "")
      })),
      containers: [...state.containers],
      diagnostics: [...state.diagnostics]
    };
  }
  function buildInterconnectionLayoutLookup(layoutDto) {
    return {
      nodesById: new Map(layoutDto.nodes.map((node) => [node.id, node])),
      edgesById: new Map(layoutDto.edges.map((edge) => [edge.id, edge]))
    };
  }

  // shared/diagram-renderer/src/render/drawing.ts
  function truncate2(value, max2) {
    const text = String(value || "");
    return text.length > max2 ? `${text.slice(0, max2 - 1)}...` : text;
  }
  function drawEdges(root2, edges, isInterconnectionView, theme, layoutDto) {
    const layoutLookup = layoutDto ? buildInterconnectionLayoutLookup(layoutDto) : void 0;
    const group = root2.append("g").attr("class", "viz-edges");
    for (const edge of edges) {
      if (!edge.sourceNode || !edge.targetNode) continue;
      const path2 = isInterconnectionView ? pathForIbdEdge(edge, layoutLookup) : pathFromSimpleSection(edge.layout?.sections?.[0]);
      if (!path2) continue;
      const edgeKind = edge.edgeKind ?? normalizeEdgeKind(edge.label);
      const displayLabel = edgeDisplayLabel(edge, edgeKind, isInterconnectionView);
      const stroke = strokeColorForEdge(edgeKind, theme);
      const strokeWidth = edgeKind === "hierarchy" ? 1.4 : isInterconnectionView ? 2 : 1.8;
      const pathSelection = group.append("path").attr("class", `${isInterconnectionView ? "ibd-connector" : "general-connector"} viz-edge viz-edge--${edgeKind}`).attr("d", path2).attr("data-connector-id", edge.id).attr("data-source", edge.source).attr("data-target", edge.target).attr("data-type", String(edge.attributes?.relationType || edgeKind || "relationship")).style("fill", "none").style("stroke", stroke).style("stroke-width", strokeWidth).style("opacity", 0.9);
      applyEdgeMarker(pathSelection, edgeKind, isInterconnectionView, theme);
      if (shouldRenderEdgeLabel(edge, edgeKind, isInterconnectionView)) {
        const midpoint = edgeMidpoint(edge, isInterconnectionView, layoutLookup);
        group.append("text").attr("class", `viz-edge-label viz-edge-label--${edgeKind}`).attr("x", midpoint.x).attr("y", midpoint.y).attr("text-anchor", "middle").attr("dy", "-0.35em").attr("fill", theme.textPrimary).attr("font-size", 11).text(truncate2(displayLabel, 18));
      }
    }
  }
  function shouldRenderEdgeLabel(edge, edgeKind, isInterconnectionView) {
    return edgeDisplayLabel(edge, edgeKind, isInterconnectionView).length > 0;
  }
  function edgeDisplayLabel(edge, edgeKind, isInterconnectionView) {
    return isInterconnectionView ? ibdEdgeDisplayLabel(edge, edgeKind) : generalEdgeDisplayLabel(edge, edgeKind);
  }
  function generalEdgeDisplayLabel(edge, edgeKind) {
    const label = String(edge.label ?? "").trim();
    const relationType = String(edge.attributes?.relationType ?? "").trim();
    const generic = /* @__PURE__ */ new Set([
      "",
      "relationship",
      "edge",
      "connect",
      "connection",
      "dependency",
      "specializes",
      "specialization",
      "typing",
      "defined_by",
      "defined by",
      "definition",
      "hierarchy",
      "contains",
      "owns",
      "ownership",
      "containment",
      "allocate",
      "allocation",
      "satisfy",
      "verify",
      "bind",
      "binding"
    ]);
    const lowerLabel = label.toLowerCase();
    if (generic.has(lowerLabel)) return "";
    if (lowerLabel === relationType.toLowerCase() || lowerLabel === edgeKind.toLowerCase()) return "";
    return label;
  }
  function ibdEdgeDisplayLabel(edge, edgeKind) {
    const itemType = String(edge.attributes?.itemType ?? "").trim();
    if (edgeKind === "flow" && itemType) return itemType;
    const interfaceName = String(edge.attributes?.interfaceName ?? "").trim();
    if (edgeKind === "interface" && interfaceName) return interfaceName;
    const label = String(edge.label ?? "").trim();
    const relationType = String(edge.attributes?.relationType ?? "").trim();
    const generic = /* @__PURE__ */ new Set(["", "connect", "connection", "flow", "interface", "binding", "bind", "reference", "ref", "relationship"]);
    if (generic.has(label.toLowerCase()) || generic.has(relationType.toLowerCase())) return "";
    return label;
  }
  function drawNodes(root2, nodes, options, isInterconnectionView, theme, layoutDto) {
    const layoutLookup = layoutDto ? buildInterconnectionLayoutLookup(layoutDto) : void 0;
    const renderNodes = isInterconnectionView ? orderIbdNodesForPaint(nodes) : nodes;
    const groups = root2.append("g").attr("class", "viz-nodes").selectAll("g").data(renderNodes).enter().append("g").attr("class", (d) => {
      const clickable = options.onNodeClick && nodeSupportsSourceNavigation(d) ? "is-clickable" : "";
      const selected = options.selectedNodeId && d.id === options.selectedNodeId ? "is-selected" : "";
      const legacyClass = isInterconnectionView ? "ibd-part" : "general-node";
      const attrs = d.attributes ?? {};
      const isLayoutContainer = Boolean(
        attrs.isSyntheticContainer || attrs.isPackageContainer || attrs._isLayoutContainer
      );
      const structureClass = resolveNodeChrome(d.kind || "part", {
        ...typeof attrs.isDefinition === "boolean" ? { isDefinition: attrs.isDefinition } : {},
        ...typeof attrs.isReference === "boolean" ? { isReference: attrs.isReference } : {},
        isContainer: isLayoutContainer,
        isPackageContainer: Boolean(attrs.isPackageContainer)
      }).structureClass;
      return `${legacyClass} viz-node ${structureClass} ${clickable} ${selected}`.trim();
    }).attr("transform", (d) => `translate(${d.x || 0},${d.y || 0})`).attr("data-node-id", (d) => d.id).attr("data-element-name", (d) => d.label).attr(
      "data-bounds",
      (d) => [d.x || 0, d.y || 0, d.width || (isInterconnectionView ? ibdNodeWidth : nodeWidth), d.height || (isInterconnectionView ? ibdNodeHeight : nodeHeight)].join(",")
    ).style(
      "cursor",
      (d) => options.onNodeClick && nodeSupportsSourceNavigation(d) ? "pointer" : null
    ).on("click", (event, d) => {
      if (!options.onNodeClick || !nodeSupportsSourceNavigation(d)) {
        return;
      }
      event.stopPropagation?.();
      options.onNodeClick?.(d);
    });
    if (!isInterconnectionView) {
      groups.each(function(d) {
        const group = select_default2(this);
        group.selectAll("*").remove();
        const compartments = d.compartments ?? collectCompartments(d);
        const attrs = d.attributes ?? {};
        const chrome = resolveNodeChrome(d.kind, {
          ...typeof attrs.isDefinition === "boolean" ? { isDefinition: attrs.isDefinition } : {},
          ...typeof attrs.isReference === "boolean" ? { isReference: attrs.isReference } : {}
        });
        renderSysMLNode(group, compartments, {
          x: 0,
          y: 0,
          width: d.width || nodeWidth,
          height: d.height || computeNodeHeight(compartments, { maxLinesPerCompartment: 8 }),
          nodeClass: "",
          dataElementName: d.label,
          strokeColor: strokeColorForNode(theme),
          kind: d.kind,
          chrome,
          selected: Boolean(options.selectedNodeId && d.id === options.selectedNodeId),
          config: { maxLinesPerCompartment: 8 },
          theme
        });
      });
      return;
    }
    groups.each(function(d) {
      const group = select_default2(this);
      group.selectAll("*").remove();
      try {
        renderIbdNode(
          group,
          d,
          Boolean(options.selectedNodeId && d.id === options.selectedNodeId),
          theme,
          layoutLookup?.nodesById.get(d.id)
        );
      } catch (error) {
        console.error("[IBD] failed to render node", d.id, error);
      }
    });
    return;
    groups.append("rect").attr("width", (d) => d.width || nodeWidth).attr("height", (d) => d.height || nodeHeight).attr("rx", 8).attr("fill", "var(--vscode-editor-background, #1e1e1e)").attr("stroke", "var(--vscode-panel-border, #666)").attr("stroke-width", 1.6);
    if (isInterconnectionView) {
      groups.append("text").attr("class", "viz-node-kind").attr("x", 14).attr("y", 22).attr("text-anchor", "start").attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)").attr("font-size", 11).text((d) => `<<${truncate2(d.kind, 24)}>>`);
      groups.append("text").attr("class", "viz-node-name").attr("x", 14).attr("y", 44).attr("text-anchor", "start").attr("fill", "var(--vscode-editor-foreground, #d0d0d0)").attr("font-size", 12).text((d) => truncate2(d.label, 34));
      groups.append("line").attr("x1", 10).attr("x2", (d) => (d.width || ibdNodeWidth) - 10).attr("y1", 56).attr("y2", 56).attr("stroke", "currentColor").attr("opacity", 0.18);
      groups.append("text").attr("class", "viz-node-kind").attr("x", 14).attr("y", 74).attr("text-anchor", "start").attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)").attr("font-size", 10).text((d) => {
        const ports = Array.isArray(d.attributes?.ports) ? d.attributes.ports : [];
        if (ports.length === 0) return "ports: \u2014";
        return `ports: ${ports.slice(0, 6).map((value) => String(value)).join(", ")}${ports.length > 6 ? "..." : ""}`;
      });
      return;
    }
    groups.append("text").attr("class", "viz-node-kind").attr("x", nodeWidth / 2).attr("y", 22).attr("text-anchor", "middle").attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)").attr("font-size", 11).text((d) => `<<${truncate2(d.kind, 24)}>>`);
    groups.append("text").attr("class", "viz-node-name").attr("x", nodeWidth / 2).attr("y", 48).attr("text-anchor", "middle").attr("fill", "var(--vscode-editor-foreground, #d0d0d0)").attr("font-size", 12).text((d) => truncate2(d.label, 30));
    groups.append("line").attr("x1", 10).attr("x2", (d) => (d.width || nodeWidth) - 10).attr("y1", 58).attr("y2", 58).attr("stroke", "var(--vscode-panel-border, #666)").attr("opacity", 0.5);
    groups.append("text").attr("class", "viz-node-attrs").attr("x", 12).attr("y", 74).attr("text-anchor", "start").attr("fill", "var(--vscode-descriptionForeground, #a8a8a8)").attr("font-size", 10).text((d) => formatCompartmentSummary(d.attributes));
  }
  function orderIbdNodesForPaint(nodes) {
    return nodes.slice().sort((a, b) => {
      const aContainer = Boolean((a.attributes ?? {})._isLayoutContainer || (a.attributes ?? {}).isSyntheticContainer || (a.attributes ?? {}).isPackageContainer);
      const bContainer = Boolean((b.attributes ?? {})._isLayoutContainer || (b.attributes ?? {}).isSyntheticContainer || (b.attributes ?? {}).isPackageContainer);
      if (aContainer !== bContainer) return aContainer ? -1 : 1;
      const aDepth = Number((a.attributes ?? {})._layoutDepth ?? 0);
      const bDepth = Number((b.attributes ?? {})._layoutDepth ?? 0);
      if (aContainer && bContainer && aDepth !== bDepth) return aDepth - bDepth;
      if (!aContainer && !bContainer && aDepth !== bDepth) return aDepth - bDepth;
      return nodes.indexOf(a) - nodes.indexOf(b);
    });
  }
  function applyEdgeMarker(path2, edgeKind, isInterconnectionView, theme) {
    if (isInterconnectionView) {
      if (edgeKind === "flow") {
        path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).attr("stroke-width", 2.5).style("marker-end", "url(#ibd-flow-arrow)");
      } else if (edgeKind === "interface") {
        path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("stroke-dasharray", "8,4").style("marker-end", "url(#ibd-interface-arrow)");
      } else if (edgeKind === "bind" || edgeKind === "binding") {
        path2.attr("stroke", strokeColorForEdge("bind", theme)).style("stroke-dasharray", "6,4").style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
      } else if (edgeKind === "reference") {
        path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).attr("stroke-width", 1.6).style("stroke-dasharray", "4,4").style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
      } else if (edgeKind === "connection" || edgeKind === "relationship") {
        path2.attr("stroke", strokeColorForEdge("connection", theme)).attr("stroke-width", 2).style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
      } else {
        path2.style("marker-start", "url(#ibd-connection-dot)").style("marker-end", "url(#ibd-connection-dot)");
      }
      return;
    }
    if (edgeKind === "specializes") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-specializes)").style("stroke-width", "1.7px");
    } else if (edgeKind === "typing") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow-open)").style("stroke-dasharray", "5,3");
    } else if (edgeKind === "hierarchy") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-start", "url(#general-d3-diamond)").style("marker-end", "none");
    } else if (edgeKind === "bind") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("stroke-dasharray", "2,2").style("marker-end", "none");
    } else if (edgeKind === "allocate") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow)").style("stroke-dasharray", "8,4");
    } else if (edgeKind === "dependency" || edgeKind === "usage") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow-open)").style("stroke-dasharray", "4,4");
    } else if (edgeKind === "redefinition") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-specializes)").style("stroke-dasharray", "5,3");
    } else if (edgeKind === "composition") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-start", "url(#general-d3-diamond)").style("marker-end", "none").style("stroke-dasharray", "6,3");
    } else if (edgeKind === "satisfy" || edgeKind === "verify" || edgeKind === "derivation") {
      path2.attr("stroke", strokeColorForEdge(edgeKind, theme)).style("marker-end", "url(#general-d3-arrow-open)").style("stroke-dasharray", "7,4");
    } else {
      path2.style("marker-end", "url(#general-d3-arrow)");
    }
  }
  function renderIbdNode(group, node, selected, theme, layoutNode) {
    const attrs = node.attributes ?? {};
    const kind = (node.kind || "part").toLowerCase();
    const isContainer = Boolean(attrs.isSyntheticContainer) || Boolean(attrs.isPackageContainer) || Boolean(attrs._isLayoutContainer);
    const width = node.width ?? ibdNodeWidth;
    const height = node.height ?? ibdNodeHeight;
    const chrome = resolveNodeChrome(kind, {
      ...typeof attrs.isDefinition === "boolean" ? { isDefinition: attrs.isDefinition } : {},
      ...typeof attrs.isReference === "boolean" ? { isReference: attrs.isReference } : {},
      isContainer,
      isPackageContainer: Boolean(attrs.isPackageContainer)
    });
    const body = nodeBodyChromeStyle(chrome, {
      selected,
      isContainer,
      isPackageContainer: Boolean(attrs.isPackageContainer)
    });
    const stroke = selected ? theme.highlight : theme.nodeBorder;
    const headerHeight = isContainer ? 28 : attrs.partType ? 41 : 33;
    group.classed("ibd-container", isContainer);
    group.append("rect").attr("width", width).attr("height", height).attr("rx", body.cornerRadius).attr("class", "graph-node-background").attr("data-original-stroke", theme.nodeBorder).attr("data-original-width", `${body.strokeWidthPx}px`).style("fill", theme.nodeFill).style("stroke", stroke).style("stroke-width", `${body.strokeWidthPx}px`).style("stroke-dasharray", body.strokeDasharray);
    group.append("rect").attr("width", width).attr("height", headerHeight).attr("rx", 6).style("fill", theme.panelBackground);
    if (isContainer) {
      group.append("text").attr("x", width / 2).attr("y", headerHeight / 2 + 4).attr("text-anchor", "middle").text(node.label).style("font-size", "11px").style("font-weight", "bold").style("fill", theme.textPrimary);
      drawIbdPorts(group, node, width, headerHeight, theme, layoutNode);
      return;
    }
    const stereo = kind.includes("part def") ? "part def" : kind.includes("part") ? "part" : (node.kind || "part").replace(/_/g, " ");
    group.append("text").attr("x", width / 2).attr("y", 17).attr("text-anchor", "middle").text(`\xAB${stereo}\xBB`).style("font-size", "9px").style("fill", theme.textPrimary);
    group.append("text").attr("class", "node-name-text viz-node-name").attr("x", width / 2).attr("y", 31).attr("text-anchor", "middle").text(truncate2(node.label, 18)).style("font-size", "11px").style("font-weight", "bold").style("fill", theme.textPrimary);
    const typedBy = String(attrs.partType || "");
    if (typedBy) {
      group.append("text").attr("x", width / 2).attr("y", 43).attr("text-anchor", "middle").text(`: ${truncate2(typedBy, 18)}`).style("font-size", "10px").style("font-style", "italic").style("fill", theme.textPrimary);
    }
    const contentStartY = typedBy ? 50 : 38;
    const children2 = Array.isArray(attrs.children) ? attrs.children : [];
    children2.slice(0, 8).forEach((child, index) => {
      const childRecord = child && typeof child === "object" ? child : {};
      const childType = String(childRecord.type || "").toLowerCase();
      const prefix = childType.includes("attribute") ? "[attr] " : childType.includes("state") ? "[state] " : childType.includes("part") ? "[part] " : "";
      const name = String(childRecord.name || "");
      if (!name) return;
      group.append("text").attr("x", 6).attr("y", contentStartY + 8 + index * 12).text(truncate2(`${prefix}${name}`, 28)).style("font-size", "9px").style("fill", theme.textSecondary);
    });
    drawIbdPorts(group, node, width, contentStartY + 20, theme, layoutNode);
  }
  function drawIbdPorts(group, node, width, fallbackStartY, theme, layoutNode) {
    const attrs = node.attributes ?? {};
    const details = Array.isArray(attrs.portDetails) ? attrs.portDetails : [];
    const drawOrder = layoutNode?.portDrawOrder ?? null;
    const portNames = drawOrder ? [...drawOrder.west ?? [], ...drawOrder.east ?? []] : details.length > 0 ? details.map((port) => port.name) : Array.isArray(attrs.ports) ? attrs.ports.map((port) => String(port)) : [];
    const anchors = layoutNode?.portAnchors ?? {};
    const portSize = 10;
    const fallbackSpacing = 26;
    const drawPort = (name, sideIndex, side) => {
      const detail = details.find((port) => port.name === name);
      const sanitized = name.replace(/[^A-Za-z0-9_.-]/g, "_");
      const anchor = anchors[sanitized] ?? anchors[name];
      const resolvedSide = anchor?.side === "WEST" || anchor?.side === "EAST" ? anchor.side : side;
      const x2 = anchor?.x ?? (resolvedSide === "WEST" ? 0 : width);
      const y2 = anchor?.y ?? fallbackStartY + sideIndex * fallbackSpacing;
      const color2 = theme.nodeBorder;
      group.append("rect").attr("class", "port-icon").attr("data-port-name", name).attr("data-port-side", resolvedSide).attr("x", x2 - portSize / 2).attr("y", y2 - portSize / 2).attr("width", portSize).attr("height", portSize).style("fill", "none").style("stroke", color2).style("stroke-width", "1.8px");
      group.append("text").attr("x", resolvedSide === "WEST" ? Math.min(width - 10, x2 + 16) : Math.max(10, x2 - 16)).attr("y", y2 + 3).attr("text-anchor", resolvedSide === "WEST" ? "start" : "end").text(truncate2(formatIbdPortLabel(name, detail), 24)).style("font-size", "8px").style("font-weight", "500").style("fill", color2);
    };
    if (drawOrder) {
      (drawOrder.west ?? []).forEach((name, index) => drawPort(name, index, "WEST"));
      (drawOrder.east ?? []).forEach((name, index) => drawPort(name, index, "EAST"));
      return;
    }
    portNames.forEach((name, index) => {
      const sanitized = name.replace(/[^A-Za-z0-9_.-]/g, "_");
      const anchor = anchors[sanitized] ?? anchors[name];
      const side = anchor?.side === "WEST" ? "WEST" : anchor?.side === "EAST" ? "EAST" : name.toLowerCase().startsWith("in") ? "WEST" : "EAST";
      drawPort(name, index, side);
    });
  }
  function formatIbdPortLabel(name, detail) {
    const direction = String(detail?.direction || "").trim();
    const directionPrefix = direction ? `${direction} ` : "";
    const type2 = String(detail?.portType || detail?.attributes?.portType || "").trim();
    if (!type2) return `${directionPrefix}${name}`;
    const conjugated = type2.startsWith("~");
    const cleanType = type2.replace(/^~/, "").split(/::|\./).pop() || type2.replace(/^~/, "");
    return `${directionPrefix}${name}: ${conjugated ? "~" : ""}${cleanType}`;
  }
  function formatCompartmentSummary(attributes) {
    if (!attributes) return "";
    const parts = Array.isArray(attributes.parts) ? attributes.parts : [];
    const ports = Array.isArray(attributes.ports) ? attributes.ports : [];
    const attrs = Array.isArray(attributes.attributes) ? attributes.attributes : [];
    const summary = [];
    if (attrs.length > 0) summary.push(`attrs:${attrs.length}`);
    if (parts.length > 0) summary.push(`parts:${parts.length}`);
    if (ports.length > 0) summary.push(`ports:${ports.length}`);
    return summary.join("  ");
  }
  function drawGeneralPackageContainers(root2, prepared, nodes, theme) {
    const packageGroups = prepared.meta?.packageContainerGroups || [];
    if (packageGroups.length === 0) return;
    const nodeById = new Map(nodes.map((node) => [node.id, node]));
    const layer = root2.append("g").attr("class", "general-package-containers");
    for (const group of packageGroups) {
      const memberIds = Array.isArray(group.memberIds) ? group.memberIds.map((value) => String(value)) : [];
      const label = String(group.name || group.label || group.id || "");
      const memberNodes = memberIds.map((id2) => nodeById.get(id2)).filter((value) => Boolean(value));
      if (memberNodes.length === 0) continue;
      const minX = Math.min(...memberNodes.map((node) => node.x || 0));
      const minY = Math.min(...memberNodes.map((node) => node.y || 0));
      const maxX = Math.max(...memberNodes.map((node) => (node.x || 0) + (node.width || nodeWidth)));
      const maxY = Math.max(...memberNodes.map((node) => (node.y || 0) + (node.height || nodeHeight)));
      const padding = 28;
      const x2 = minX - padding;
      const y2 = minY - padding;
      const width = maxX - minX + padding * 2;
      const height = maxY - minY + padding * 2;
      layer.append("rect").attr("class", "general-package-frame").attr("x", x2).attr("y", y2).attr("width", width).attr("height", height).attr("rx", 18).style("fill", "transparent").style("stroke", theme.nodeBorder).style("stroke-width", "1.5px").style("opacity", 0.9);
      layer.append("text").attr("class", "general-package-label").attr("x", x2 + 14).attr("y", y2 + 21).style("font-size", "11px").style("font-weight", "700").style("fill", theme.nodeBorder).text(label);
    }
  }
  function drawInterconnectionContainers(root2, prepared, nodes, theme, layoutDto) {
    const layoutContainers = layoutDto?.containers ?? [];
    if (layoutContainers.length > 0) {
      const layer2 = root2.append("g").attr("class", "ibd-containers");
      for (const container of layoutContainers) {
        const label = container.label;
        const groupG = layer2.append("g").attr("class", "ibd-part ibd-container").attr("transform", `translate(${container.x},${container.y})`).attr("data-element-name", label);
        groupG.append("rect").attr("width", container.width).attr("height", container.height).attr("rx", 14).attr("fill", "none").attr("stroke", theme.nodeBorder).attr("stroke-width", 1.4).attr("stroke-dasharray", "6,4").attr("opacity", 0.7);
        groupG.append("text").attr("x", 12).attr("y", 20).attr("fill", theme.textSecondary).attr("font-size", 11).text(label);
      }
      return;
    }
    const packageGroups = prepared.meta?.packageContainerGroups || [];
    if (packageGroups.length === 0) return;
    const nodeById = new Map(nodes.map((node) => [node.id, node]));
    const layer = root2.append("g").attr("class", "ibd-containers");
    for (const group of packageGroups) {
      const memberIds = Array.isArray(group.memberIds) ? group.memberIds.map((value) => String(value)) : [];
      const label = String(group.name || group.label || group.id || "");
      const memberNodes = memberIds.map((id2) => nodeById.get(id2)).filter((value) => Boolean(value));
      if (memberNodes.length === 0) continue;
      const minX = Math.min(...memberNodes.map((node) => node.x || 0));
      const minY = Math.min(...memberNodes.map((node) => node.y || 0));
      const maxX = Math.max(...memberNodes.map((node) => (node.x || 0) + (node.width || ibdNodeWidth)));
      const maxY = Math.max(...memberNodes.map((node) => (node.y || 0) + (node.height || ibdNodeHeight)));
      const padding = 26;
      const x2 = minX - padding;
      const y2 = minY - padding;
      const width = maxX - minX + padding * 2;
      const height = maxY - minY + padding * 2;
      const groupG = layer.append("g").attr("class", "ibd-part ibd-container").attr("transform", `translate(${x2},${y2})`).attr("data-element-name", label);
      groupG.append("rect").attr("width", width).attr("height", height).attr("rx", 14).attr("fill", "none").attr("stroke", theme.nodeBorder).attr("stroke-width", 1.4).attr("stroke-dasharray", "6,4").attr("opacity", 0.7);
      groupG.append("text").attr("x", 12).attr("y", 20).attr("fill", theme.textSecondary).attr("font-size", 11).text(label);
    }
  }
  function shouldDrawIbdViewFrame(prepared) {
    return !prepared.nodes.some((node) => Boolean((node.attributes ?? {}).isDiagramRoot));
  }
  function drawIbdViewFrame(root2, prepared, bounds, theme) {
    const label = String(prepared.meta?.selectedRoot || prepared.title || "").trim();
    if (!label || bounds.width <= 0 || bounds.height <= 0) return;
    const padding = 20;
    const headerHeight = 18;
    const x2 = bounds.x - padding;
    const y2 = bounds.y - padding - headerHeight;
    const width = bounds.width + padding * 2;
    const height = bounds.height + padding * 2 + headerHeight;
    const frame2 = root2.append("g").attr("class", "ibd-view-frame").attr("data-view-name", label);
    frame2.append("rect").attr("x", x2).attr("y", y2).attr("width", width).attr("height", height).attr("rx", 6).style("fill", "none").style("stroke", theme.frame.stroke).style("stroke-width", "1.5px");
    frame2.append("text").attr("x", x2 + width / 2).attr("y", y2 + 13).attr("text-anchor", "middle").style("font-size", "11px").style("font-weight", "bold").style("fill", theme.frame.text).text(label);
  }
  function pointsToPathD(points) {
    if (points.length < 2) return "";
    return line_default().x((d) => d.x).y((d) => d.y)(points) || "";
  }
  function pathFromSimpleSection(section) {
    if (!section) return null;
    const points = [section.startPoint, ...section.bendPoints || [], section.endPoint].filter(Boolean);
    if (points.length < 2) return null;
    return pointsToPathD(points);
  }
  function pathForIbdEdge(edge, layoutLookup) {
    const layoutEdge = layoutLookup?.edgesById.get(edge.id);
    const points = layoutEdge && layoutEdge.routePoints.length >= 2 ? layoutEdge.routePoints : resolveIbdRoutePoints(edge);
    if (!points || points.length < 2) return null;
    return pointsToPathD(points);
  }
  function edgeMidpoint(edge, isInterconnectionView, layoutLookup) {
    if (isInterconnectionView) {
      const layoutEdge = layoutLookup?.edgesById.get(edge.id);
      const routePoints = layoutEdge?.routePoints?.length ? layoutEdge.routePoints : resolveIbdRoutePoints(edge);
      if (routePoints && routePoints.length > 0) {
        const index = Math.floor((routePoints.length - 1) / 2);
        return routePoints[index];
      }
    } else {
      const section = edge.layout?.sections?.[0];
      if (section) {
        const points = [section.startPoint, ...section.bendPoints || [], section.endPoint].filter(Boolean);
        if (points.length > 0) {
          const index = Math.floor((points.length - 1) / 2);
          return points[index];
        }
      }
    }
    const sourceNode = edge.sourceNode;
    const targetNode = edge.targetNode;
    if (sourceNode && targetNode) {
      const width = isInterconnectionView ? ibdNodeWidth : nodeWidth;
      const height = isInterconnectionView ? ibdNodeHeight : nodeHeight;
      return {
        x: ((sourceNode.x || 0) + (targetNode.x || 0) + width) / 2,
        y: ((sourceNode.y || 0) + (targetNode.y || 0) + height) / 2
      };
    }
    return { x: 0, y: 0 };
  }

  // shared/diagram-renderer/src/render/interconnection-elk-input.ts
  function buildInterconnectionElkBuild(prepared) {
    const nodesById = new Map(prepared.nodes.map((node) => [node.id, node]));
    const childrenByParent = /* @__PURE__ */ new Map();
    const roots = [];
    for (const node of prepared.nodes) {
      const attrs = node.attributes ?? {};
      const parentId = typeof attrs.containerId === "string" ? attrs.containerId : "";
      if (parentId && nodesById.has(parentId)) {
        const current = childrenByParent.get(parentId) ?? [];
        current.push(node);
        childrenByParent.set(parentId, current);
      } else {
        roots.push(node);
      }
    }
    const sanitizeId = (value) => value.replace(/[^A-Za-z0-9_.-]/g, "_");
    const elkIdFor = (preparedNodeId) => sanitizeId(preparedNodeId);
    const preparedIdForElkId = /* @__PURE__ */ new Map();
    const registerElkId = (preparedId) => {
      const elkId = elkIdFor(preparedId);
      preparedIdForElkId.set(elkId, preparedId);
      return elkId;
    };
    const portIdFor = (nodeId, portName) => `${sanitizeId(nodeId)}__port__${sanitizeId(portName)}`;
    const portDetailsFor = (node) => {
      const attrs = node.attributes ?? {};
      const details = Array.isArray(attrs.portDetails) ? attrs.portDetails : [];
      if (details.length > 0) {
        return details.map((item) => item && typeof item === "object" ? item : null).filter((item) => Boolean(item?.name));
      }
      return Array.isArray(attrs.ports) ? attrs.ports.map((name) => ({ name: String(name) })) : [];
    };
    const normalizeEndpoint = (value) => String(value ?? "").replace(/::/g, ".").trim();
    const portLayoutKeys = (node, port) => {
      const attrs = node.attributes ?? {};
      const keys = [];
      const explicit = normalizeEndpoint(port.id);
      if (explicit) keys.push(explicit);
      const parent = normalizeEndpoint(
        port.attributes?.parentId ?? port.parentId ?? attrs.qualifiedName ?? node.id ?? node.label
      );
      if (parent) keys.push(`${parent}.${port.name}`);
      keys.push(normalizeEndpoint(port.name));
      return [...new Set(keys.filter(Boolean))];
    };
    const portUsage = /* @__PURE__ */ new Map();
    const bumpPortUsage = (endpoint, role) => {
      const normalized = normalizeEndpoint(endpoint);
      if (!normalized) return;
      const current = portUsage.get(normalized) ?? { sourceCount: 0, targetCount: 0 };
      current[role] += 1;
      portUsage.set(normalized, current);
    };
    for (const edge of prepared.edges) {
      bumpPortUsage(edge.attributes?.sourceId ?? edge.source, "sourceCount");
      bumpPortUsage(edge.attributes?.targetId ?? edge.target, "targetCount");
    }
    const usageForPort = (node, port) => {
      for (const key of portLayoutKeys(node, port)) {
        const explicit = portUsage.get(key);
        if (explicit) return explicit;
      }
      const attrs = node.attributes ?? {};
      const parent = normalizeEndpoint(attrs.qualifiedName ?? node.id ?? node.label);
      const fallback = portUsage.get(`${parent}.${normalizeEndpoint(port.name)}`);
      if (fallback) return fallback;
      const aliases = [
        normalizeEndpoint(node.id),
        normalizeEndpoint(node.label),
        normalizeEndpoint(attrs.qualifiedName)
      ].filter(Boolean);
      const portName = normalizeEndpoint(port.name);
      const usage = { sourceCount: 0, targetCount: 0 };
      for (const [endpoint, counts] of portUsage) {
        if (!endpoint.endsWith(`.${portName}`) && endpoint !== portName) continue;
        const owner = endpoint === portName ? "" : endpoint.slice(0, -portName.length - 1);
        const matchesOwner = aliases.some(
          (alias) => owner === alias || owner.endsWith(`.${alias}`) || alias.endsWith(`.${owner}`) || owner.endsWith(`.${node.label}`)
        );
        if (!matchesOwner) continue;
        usage.sourceCount += counts.sourceCount;
        usage.targetCount += counts.targetCount;
      }
      return usage;
    };
    const connectorPortName = (node, endpoint) => {
      const endpointText = String(endpoint ?? "").trim();
      if (!endpointText) return null;
      const ports = portDetailsFor(node);
      const canonicalMatch = ports.find(
        (port) => port.id === endpointText || port.attributes?.scenePortId === endpointText
      );
      return canonicalMatch?.name ?? null;
    };
    const sideForPort = (port, node) => {
      const sideHint = String(port.attributes?.sideHint || "").toLowerCase();
      if (sideHint === "west") return "WEST";
      if (sideHint === "east") return "EAST";
      const explicit = String(port.portSide || port.attributes?.portSide || "").toLowerCase();
      if (explicit === "left" || explicit === "west") return "WEST";
      if (explicit === "right" || explicit === "east") return "EAST";
      const direction = String(port.direction || "").toLowerCase();
      if (direction === "in") return "WEST";
      if (direction === "out") return "EAST";
      const usage = usageForPort(node, port);
      if (usage.targetCount > usage.sourceCount) return "WEST";
      if (usage.sourceCount > usage.targetCount) return "EAST";
      const lower2 = port.name.toLowerCase();
      const portType = String(port.portType || port.attributes?.portType || "").toLowerCase();
      if (lower2.endsWith("in") || lower2.includes("input") || lower2.startsWith("in")) return "WEST";
      if (lower2.endsWith("out") || lower2.startsWith("out")) return "EAST";
      if (portType.startsWith("~") && /(powerport|telemetryport|sensordataport|gimbalcommandport|cameracontrolport)/.test(portType)) {
        return "WEST";
      }
      if (!portType.startsWith("~") && /(powerport|telemetryport|sensordataport)/.test(portType)) {
        return "EAST";
      }
      const nodeText = `${node.label} ${String(node.attributes?.qualifiedName || "")}`.toLowerCase();
      const prefersLeft = /(sensor|imu|barometer|gnss|receiver|battery|input|telemetryin|videoin|c2in|rcin|sensorin)/.test(nodeText) || /(cmd$|control$|input|telemetryin|videoin|sensorin|mainpower)/.test(lower2);
      const prefersRight = /(camera|gimbal|propulsion|motor|radio|communication|distribution|controller|payload|actuator)/.test(nodeText) || /(videoout|telemetryout|regulated|pwr|cmd|ctrl)/.test(lower2);
      if (prefersLeft && !prefersRight) return "WEST";
      if (prefersRight && !prefersLeft) return "EAST";
      return "EAST";
    };
    const rootHeaderHeight = 28;
    const containerTopInset = rootHeaderHeight + 20;
    const toElkNode = (node) => {
      const ports = portDetailsFor(node);
      const { west: westPorts, east: eastPorts } = splitIbdPortsBySide(node, ports, sideForPort, usageForPort);
      const portRows = Math.max(westPorts.length, eastPorts.length, ports.length > 0 ? 1 : 0);
      const children2 = (childrenByParent.get(node.id) ?? []).map((child) => toElkNode(child));
      const attrs = node.attributes ?? {};
      const isSyntheticPackage2 = Boolean(attrs.isSyntheticPackage);
      const isContainer = Boolean(attrs.isSyntheticContainer) || children2.length > 0;
      const baseWidth = isContainer ? 420 : ibdNodeWidth;
      let width = Math.max(
        baseWidth,
        180 + Math.max(node.label.length * 6, ...ports.map((item) => item.name.length * 5), 0)
      );
      let height = isContainer ? rootHeaderHeight + 72 : computeIbdLeafHeight(node, ports, portRows);
      if (isContainer && children2.length > 0) {
        const childWidthSum = children2.reduce((sum, child) => sum + Number(child.width ?? ibdNodeWidth), 0);
        width = isSyntheticPackage2 ? Math.max(width, Math.min(980, childWidthSum + children2.length * 44)) : Math.max(width, Math.min(1040, childWidthSum + children2.length * 72));
        height = isSyntheticPackage2 ? rootHeaderHeight + 72 : rootHeaderHeight + Math.max(72, Math.min(132, 58 + children2.length * 14));
      }
      const buildElkPort = (port, side, index) => ({
        id: portIdFor(node.id, port.name),
        width: 10,
        height: 10,
        layoutOptions: {
          "org.eclipse.elk.port.side": side,
          "org.eclipse.elk.port.index": String(index)
        }
      });
      return {
        id: registerElkId(node.id),
        width,
        height,
        ports: [
          ...westPorts.map((port, index) => buildElkPort(port, "WEST", index)),
          ...eastPorts.map((port, index) => buildElkPort(port, "EAST", index))
        ],
        children: children2,
        layoutOptions: children2.length ? {
          "elk.padding": isSyntheticPackage2 ? `[top=${rootHeaderHeight + 12},left=16,bottom=16,right=16]` : `[top=${containerTopInset},left=24,bottom=24,right=24]`,
          "elk.direction": isSyntheticPackage2 ? "DOWN" : "RIGHT",
          "org.eclipse.elk.portConstraints": "FIXED_ORDER",
          "org.eclipse.elk.portAlignment.default": "CENTER"
        } : {
          "org.eclipse.elk.portConstraints": "FIXED_ORDER",
          "org.eclipse.elk.portAlignment.default": "CENTER"
        }
      };
    };
    const elkEdges = prepared.edges.map((edge) => {
      const sourceNode = nodesById.get(edge.source);
      const targetNode = nodesById.get(edge.target);
      if (!sourceNode || !targetNode) return null;
      const edgeAttrs = edge.attributes;
      const sourceEndpoint = edgeAttrs?.sourcePortId ?? edgeAttrs?.sourceId;
      const targetEndpoint = edgeAttrs?.targetPortId ?? edgeAttrs?.targetId;
      const sourcePortName = connectorPortName(sourceNode, sourceEndpoint);
      const targetPortName = connectorPortName(targetNode, targetEndpoint);
      return {
        id: edge.id,
        sources: [sourcePortName ? portIdFor(sourceNode.id, sourcePortName) : elkIdFor(sourceNode.id)],
        targets: [targetPortName ? portIdFor(targetNode.id, targetPortName) : elkIdFor(targetNode.id)],
        sourcePortId: sourcePortName ? portIdFor(sourceNode.id, sourcePortName) : void 0,
        targetPortId: targetPortName ? portIdFor(targetNode.id, targetPortName) : void 0
      };
    }).filter((edge) => edge !== null);
    const elkGraphInput = {
      id: "root",
      layoutOptions: buildElkLayoutOptions("interconnection"),
      children: roots.map((node) => toElkNode(node)),
      edges: elkEdges.map((edge) => ({ id: edge.id, sources: edge.sources, targets: edge.targets }))
    };
    const portDrawOrderFor = (node) => {
      const ports = portDetailsFor(node);
      const { west, east } = splitIbdPortsBySide(node, ports, sideForPort, usageForPort);
      return { west: west.map((port) => port.name), east: east.map((port) => port.name) };
    };
    return { elkGraphInput, elkEdges, nodesById, preparedIdForElkId, portDrawOrderFor };
  }

  // shared/diagram-renderer/src/render/layout.ts
  var elk = new HeadlessElk();
  async function layoutPrepared(prepared) {
    if (!prepared.nodes.length) return { nodes: [], edges: [] };
    if (prepared.view === "interconnection-view") {
      return layoutInterconnectionPrepared(prepared);
    }
    if (prepared.view === "action-flow-view" || prepared.view === "state-transition-view" || prepared.view === "sequence-view" || prepared.view === "browser-view" || prepared.view === "grid-view" || prepared.view === "geometry-view") {
      return { nodes: [], edges: [] };
    }
    const diagramNodes = prepared.nodes.filter((node) => isOverviewVisualElementType(node.kind));
    const visibleIds = new Set(diagramNodes.map((node) => node.id));
    const diagramEdges = prepared.edges.filter(
      (edge) => visibleIds.has(edge.source) && visibleIds.has(edge.target)
    );
    if (!diagramNodes.length) return { nodes: [], edges: [] };
    const width = nodeWidth;
    const height = nodeHeight;
    const leafElkNode = (node) => {
      const compartments = collectCompartments(node);
      return {
        id: node.id,
        width,
        height: Math.max(height, computeNodeHeight(compartments, { maxLinesPerCompartment: 8 }))
      };
    };
    const packageGroups = prepared.meta?.packageContainerGroups ?? [];
    const useHierarchy = packageGroups.length >= 2;
    let children2;
    if (useHierarchy) {
      const memberToPackage = /* @__PURE__ */ new Map();
      for (const group of packageGroups) {
        for (const memberId of group.memberIds) memberToPackage.set(memberId, group.id);
      }
      const byPackage = /* @__PURE__ */ new Map();
      const orphans = [];
      for (const node of diagramNodes) {
        const pkgId = memberToPackage.get(node.id);
        const elkNode = leafElkNode(node);
        if (pkgId) {
          const list = byPackage.get(pkgId) ?? [];
          list.push(elkNode);
          byPackage.set(pkgId, list);
        } else {
          orphans.push(elkNode);
        }
      }
      const containers = packageGroups.filter((group) => (byPackage.get(group.id) ?? []).length > 0).map((group) => ({
        id: group.id,
        layoutOptions: {
          "elk.direction": "DOWN",
          "elk.padding": "[top=36,left=20,bottom=20,right=20]"
        },
        children: byPackage.get(group.id) ?? []
      }));
      children2 = [...containers, ...orphans];
    } else {
      children2 = diagramNodes.map(leafElkNode);
    }
    const graph = {
      id: "root",
      layoutOptions: buildElkLayoutOptions("general", {
        "elk.hierarchyHandling": useHierarchy ? "INCLUDE_CHILDREN" : void 0
      }),
      children: children2,
      edges: diagramEdges.map((edge) => ({ id: edge.id, sources: [edge.source], targets: [edge.target] }))
    };
    try {
      const laidOut = await elk.layout(graph);
      const byId = new Map(diagramNodes.map((node) => [node.id, node]));
      const layouts = /* @__PURE__ */ new Map();
      const visit = (elkNode, ox, oy) => {
        const absX = ox + (elkNode.x ?? 0);
        const absY = oy + (elkNode.y ?? 0);
        layouts.set(String(elkNode.id), { ...elkNode, x: absX, y: absY });
        for (const child of elkNode.children ?? []) visit(child, absX, absY);
      };
      for (const child of laidOut.children ?? []) visit(child, 0, 0);
      const edgesById = /* @__PURE__ */ new Map();
      const collectEdges = (elkNode) => {
        for (const elkEdge of elkNode.edges ?? []) {
          if (elkEdge?.id) edgesById.set(String(elkEdge.id), elkEdge);
        }
        for (const child of elkNode.children ?? []) collectEdges(child);
      };
      collectEdges(laidOut);
      return {
        nodes: diagramNodes.map((node) => {
          const compartments = collectCompartments(node);
          return { ...node, compartments, ...layouts.get(node.id) || {} };
        }),
        edges: diagramEdges.map((edge) => ({
          ...edge,
          sourceNode: byId.get(edge.source),
          targetNode: byId.get(edge.target),
          layout: edgesById.get(edge.id)
        }))
      };
    } catch {
      return { nodes: [], edges: [] };
    }
  }
  async function layoutInterconnectionPrepared(prepared) {
    const interconnection = interconnectionPreparedForLayout(prepared);
    const layoutBuildState = createInterconnectionLayoutBuildState();
    const { elkGraphInput, elkEdges, nodesById, preparedIdForElkId, portDrawOrderFor } = buildInterconnectionElkBuild(interconnection);
    const nodeBoundaryPoint = (node, role) => ({
      x: (node.x ?? 0) + (role === "source" ? node.width ?? ibdNodeWidth : 0),
      y: (node.y ?? 0) + (node.height ?? ibdNodeHeight) / 2
    });
    const fallbackEdgeSections = (sourceNode, targetNode, sourcePortCenter, targetPortCenter) => {
      if (!sourceNode || !targetNode) return void 0;
      const startPoint = sourcePortCenter ?? nodeBoundaryPoint(sourceNode, "source");
      const endPoint = targetPortCenter ?? nodeBoundaryPoint(targetNode, "target");
      const midX = (startPoint.x + endPoint.x) / 2;
      return [
        {
          startPoint,
          bendPoints: [
            { x: midX, y: startPoint.y },
            { x: midX, y: endPoint.y }
          ],
          endPoint
        }
      ];
    };
    try {
      const laidOut = await elk.layout(elkGraphInput);
      const laidOutNodes = /* @__PURE__ */ new Map();
      const portCenters = /* @__PURE__ */ new Map();
      const nodePortAnchors = /* @__PURE__ */ new Map();
      const visit = (elkNode, ox, oy, depth) => {
        const absX = ox + (elkNode.x ?? 0);
        const absY = oy + (elkNode.y ?? 0);
        const preparedId = preparedIdForElkId.get(String(elkNode.id)) ?? String(elkNode.id);
        const base = nodesById.get(preparedId);
        for (const port of elkNode.ports ?? []) {
          const pw = port.width ?? 10;
          const ph = port.height ?? 10;
          const side = port?.layoutOptions?.["org.eclipse.elk.port.side"];
          const x2 = side === "WEST" ? absX + (port.x ?? 0) : side === "EAST" ? absX + (port.x ?? 0) + pw : absX + (port.x ?? 0) + pw / 2;
          const y2 = absY + (port.y ?? 0) + ph / 2;
          portCenters.set(String(port.id), { x: x2, y: y2 });
          if (base) {
            const portName = String(port.id).split("__port__").pop() ?? String(port.id);
            const anchors = nodePortAnchors.get(base.id) ?? {};
            anchors[portName] = { x: x2 - absX, y: y2 - absY, side: String(side || "") };
            nodePortAnchors.set(base.id, anchors);
          }
        }
        if (base) {
          const attrs = base.attributes ?? {};
          const hasLayoutChildren = Array.isArray(elkNode.children) && elkNode.children.length > 0;
          const isContainerFrame = hasLayoutChildren || Boolean(attrs.isSyntheticContainer);
          const portDrawOrder = portDrawOrderFor(base);
          const portAnchors = nodePortAnchors.get(base.id) ?? {};
          const laidOutWidth = elkNode.width ?? ibdNodeWidth;
          const laidOutHeight = elkNode.height ?? ibdNodeHeight;
          recordInterconnectionLayoutNode(
            layoutBuildState,
            { id: base.id, x: absX, y: absY, width: laidOutWidth, height: laidOutHeight },
            portAnchors,
            portDrawOrder
          );
          if (isContainerFrame) {
            recordInterconnectionLayoutContainer(layoutBuildState, {
              id: base.id,
              label: base.label,
              x: absX,
              y: absY,
              width: laidOutWidth,
              height: laidOutHeight
            });
          }
          laidOutNodes.set(base.id, {
            ...base,
            x: absX,
            y: absY,
            width: laidOutWidth,
            height: laidOutHeight,
            attributes: {
              ...base.attributes ?? {},
              _isLayoutContainer: hasLayoutChildren,
              _layoutDepth: depth
            }
          });
        }
        for (const child of elkNode.children ?? []) {
          visit(child, absX, absY, depth + 1);
        }
      };
      for (const child of laidOut.children ?? []) {
        visit(child, 0, 0, 0);
      }
      const edgeLayout = /* @__PURE__ */ new Map();
      const collectElkEdgesWithOffsets = (elkNode, containerOffset) => {
        for (const elkEdge of elkNode.edges ?? []) {
          const edgeId = String(elkEdge?.id ?? "");
          if (!edgeId) continue;
          edgeLayout.set(edgeId, { edge: elkEdge, offset: containerOffset });
        }
        for (const child of elkNode.children ?? []) {
          collectElkEdgesWithOffsets(child, {
            x: containerOffset.x + (child.x ?? 0),
            y: containerOffset.y + (child.y ?? 0)
          });
        }
      };
      collectElkEdgesWithOffsets(laidOut, { x: 0, y: 0 });
      for (const elkEdge of laidOut.edges ?? []) {
        const edgeId = String(elkEdge?.id ?? "");
        if (!edgeId) continue;
        edgeLayout.set(edgeId, { edge: elkEdge, offset: { x: 0, y: 0 } });
      }
      const nodes = interconnection.nodes.map((node) => laidOutNodes.get(node.id)).filter((value) => Boolean(value));
      const edges = interconnection.edges.map((edge) => {
        const layoutRecord = edgeLayout.get(edge.id);
        const elkEdge = elkEdges.find((item) => item.id === edge.id);
        const sourceNode = laidOutNodes.get(edge.source);
        const targetNode = laidOutNodes.get(edge.target);
        const sourcePortCenter = elkEdge?.sourcePortId ? portCenters.get(elkEdge.sourcePortId) : void 0;
        const targetPortCenter = elkEdge?.targetPortId ? portCenters.get(elkEdge.targetPortId) : void 0;
        if ((edge.attributes?.sourcePortId || edge.attributes?.targetPortId) && (!sourcePortCenter || !targetPortCenter)) {
          layoutBuildState.diagnostics.push(
            `node-boundary fallback for edge ${edge.id}`
          );
        }
        return {
          ...edge,
          sourceNode,
          targetNode,
          layout: layoutRecord?.edge.sections?.length ? {
            sections: layoutRecord.edge.sections,
            edgeOwnerOffset: layoutRecord.offset,
            lcaOffset: sourceNode && targetNode ? lcaOffsetForNodes(sourceNode, targetNode, laidOutNodes) : { x: 0, y: 0 }
          } : {
            sections: fallbackEdgeSections(sourceNode, targetNode, sourcePortCenter, targetPortCenter),
            edgeOwnerOffset: { x: 0, y: 0 },
            lcaOffset: { x: 0, y: 0 }
          },
          attributes: {
            ...edge.attributes ?? {},
            _sourcePortCenter: sourcePortCenter,
            _targetPortCenter: targetPortCenter
          }
        };
      });
      return {
        nodes,
        edges,
        interconnectionLayout: finalizeInterconnectionLayoutDto(layoutBuildState, edges)
      };
    } catch {
      return { nodes: [], edges: [] };
    }
  }

  // shared/diagram-renderer/src/renderer.ts
  async function renderVisualization(target, prepared, options = {}) {
    const renderStartedAt = Date.now();
    target.innerHTML = "";
    const theme = resolveDiagramTheme(options.theme);
    const width = Math.max(720, target.clientWidth || 960);
    const height = Math.max(480, target.clientHeight || 640);
    const svg = select_default2(target).append("svg").attr("class", "sysml-viz-svg").attr("width", "100%").attr("height", "100%").attr("viewBox", `0 0 ${width} ${height}`).attr("role", "img").attr("aria-label", prepared.title || "SysML view").style("touch-action", "none").style("cursor", "grab");
    if (theme.colorScheme === "light" || theme.colorScheme === "dark" || theme.colorScheme === "auto") {
      const scheme = theme.colorScheme === "auto" ? typeof window !== "undefined" && window.matchMedia?.("(prefers-color-scheme: dark)")?.matches ? "dark" : "light" : theme.colorScheme;
      svg.attr("data-color-scheme", scheme);
    }
    svg.append("rect").attr("class", "viz-bg").attr("width", width).attr("height", height);
    svg.select(".viz-bg").attr("fill", theme.canvasBackground);
    addMarkers(svg, theme);
    const root2 = svg.append("g").attr("class", "viz-root");
    const delegateZoom = options.delegateZoom === true;
    const zoom = zoom_default2().scaleExtent([0.08, 5]).on("start", () => svg.style("cursor", "grabbing")).on("zoom", (event) => {
      root2.attr("transform", event.transform.toString());
    }).on("end", () => svg.style("cursor", "grab"));
    if (!delegateZoom) {
      svg.call(zoom).on("dblclick.zoom", null).on("wheel.zoom", function(event) {
        event.preventDefault();
        event.stopPropagation();
        const mouse = pointer_default(event, this);
        const currentTransform = transform(this);
        const factor = event.deltaY > 0 ? 0.7 : 1.45;
        const newScale = Math.min(Math.max(currentTransform.k * factor, 0.08), 5);
        const translateX = mouse[0] - (mouse[0] - currentTransform.x) * (newScale / currentTransform.k);
        const translateY = mouse[1] - (mouse[1] - currentTransform.y) * (newScale / currentTransform.k);
        select_default2(this).transition().duration(50).call(zoom.transform, identity2.translate(translateX, translateY).scale(newScale));
      });
    }
    const view = prepared.view;
    const isInterconnectionView = view === "interconnection-view";
    const isBehaviorView = view === "action-flow-view" || view === "state-transition-view" || view === "sequence-view" || view === "browser-view" || view === "grid-view" || view === "geometry-view";
    let bounds;
    if (view === "action-flow-view") {
      addActionFlowMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
      const drawStartedAt = Date.now();
      bounds = contentBoundsFromExtents(await renderActionFlowView({ root: root2, prepared, theme, width, height, options }));
      options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
    } else if (view === "state-transition-view") {
      addStateTransitionMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
      const drawStartedAt = Date.now();
      bounds = contentBoundsFromExtents(await renderStateTransitionView({ root: root2, prepared, theme, width, height, options }));
      options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
    } else if (view === "sequence-view") {
      addSequenceMarkers(svg.select("defs").empty() ? svg.append("defs") : svg.select("defs"), theme);
      const drawStartedAt = Date.now();
      bounds = contentBoundsFromExtents(renderSequenceView({ root: root2, prepared, theme, width, height, options }));
      options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
    } else if (view === "browser-view") {
      const drawStartedAt = Date.now();
      bounds = contentBoundsFromExtents(renderBrowserView({ root: root2, prepared, theme, width, height, options }));
      options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
    } else if (view === "grid-view") {
      const drawStartedAt = Date.now();
      bounds = contentBoundsFromExtents(renderGridView({ root: root2, prepared, theme, width, height, options }));
      options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
    } else if (view === "geometry-view") {
      const drawStartedAt = Date.now();
      bounds = contentBoundsFromExtents(renderGeometryView({ root: root2, prepared, theme, width, height, options }));
      options.onPerformance?.("sharedRenderer:draw", { view, drawMs: Date.now() - drawStartedAt });
    } else {
      const layoutStartedAt = Date.now();
      const layout = await layoutPrepared(prepared);
      const layoutMs = Date.now() - layoutStartedAt;
      const drawStartedAt = Date.now();
      if (isInterconnectionView) {
        if (shouldDrawIbdViewFrame(prepared)) {
          drawIbdViewFrame(root2, prepared, contentBounds(layout), theme);
        }
        drawInterconnectionContainers(root2, prepared, layout.nodes, theme, layout.interconnectionLayout);
        drawNodes(root2, layout.nodes, options, isInterconnectionView, theme, layout.interconnectionLayout);
        drawEdges(root2, layout.edges, isInterconnectionView, theme, layout.interconnectionLayout);
      } else {
        drawGeneralPackageContainers(root2, prepared, layout.nodes, theme);
        drawEdges(root2, layout.edges, isInterconnectionView, theme);
        drawNodes(root2, layout.nodes, options, isInterconnectionView, theme);
      }
      options.onPerformance?.("sharedRenderer:layout", {
        view,
        layoutMs,
        nodeCount: prepared.nodes.length,
        edgeCount: prepared.edges.length
      });
      options.onPerformance?.("sharedRenderer:draw", {
        view,
        drawMs: Date.now() - drawStartedAt,
        laidOutNodes: layout.nodes.length,
        laidOutEdges: layout.edges.length
      });
      bounds = contentBounds(layout);
    }
    let lastFitTransform = identity2;
    const fitView = () => {
      lastFitTransform = applyFit(
        svg,
        zoom,
        root2,
        bounds,
        width,
        height,
        isInterconnectionView || isBehaviorView,
        delegateZoom
      );
    };
    fitView();
    options.onPerformance?.("sharedRenderer:render", {
      view,
      totalMs: Date.now() - renderStartedAt,
      nodeCount: prepared.nodes.length,
      edgeCount: prepared.edges.length
    });
    return {
      reset: () => fitView(),
      getFitTransform: () => lastFitTransform,
      exportSvg: () => exportSvg(svg.node(), bounds),
      destroy: () => {
        target.innerHTML = "";
      }
    };
  }

  // shared/diagram-renderer/src/headless-export.ts
  function preparedViewFromPayload(payload) {
    const prepared = payload.preparedView;
    if (!prepared || typeof prepared !== "object") {
      return null;
    }
    const view = prepared;
    if (typeof view.view !== "string" || !Array.isArray(view.nodes) || !Array.isArray(view.edges)) {
      return null;
    }
    return prepared;
  }
  var VirtualStyle = class {
    constructor() {
      this.values = /* @__PURE__ */ new Map();
    }
    setProperty(name, value) {
      if (value == null) {
        this.values.delete(name);
        return;
      }
      this.values.set(name, String(value));
    }
    removeProperty(name) {
      this.values.delete(name);
    }
    getPropertyValue(name) {
      return this.values.get(name) ?? "";
    }
    toString() {
      return Array.from(this.values.entries()).map(([key, value]) => `${key}: ${value};`).join(" ");
    }
  };
  var VirtualElement = class _VirtualElement {
    constructor(ownerDocument, tagName, namespaceURI = "http://www.w3.org/1999/xhtml") {
      this.tagName = tagName;
      this.style = new VirtualStyle();
      this.children = [];
      this.parentNode = null;
      this.textContent = "";
      this.clientWidth = 0;
      this.clientHeight = 0;
      this.attrs = /* @__PURE__ */ new Map();
      this.ownerDocument = ownerDocument;
      this.namespaceURI = namespaceURI;
    }
    get childNodes() {
      return this.children;
    }
    get firstChild() {
      return this.children[0] ?? null;
    }
    get nextSibling() {
      if (!this.parentNode) return null;
      const index = this.parentNode.children.indexOf(this);
      return index >= 0 ? this.parentNode.children[index + 1] ?? null : null;
    }
    get innerHTML() {
      return this.children.map((child) => child.serialize()).join("");
    }
    set innerHTML(value) {
      this.children.splice(0, this.children.length);
      this.textContent = value ? String(value) : "";
    }
    setAttribute(name, value) {
      if (value == null) {
        this.attrs.delete(name);
        return;
      }
      this.attrs.set(name, String(value));
    }
    setAttributeNS(_namespace, name, value) {
      this.setAttribute(name, value);
    }
    getAttribute(name) {
      return this.attrs.get(name) ?? null;
    }
    hasAttribute(name) {
      return this.attrs.has(name);
    }
    removeAttribute(name) {
      this.attrs.delete(name);
    }
    appendChild(child) {
      child.parentNode?.removeChild(child);
      child.parentNode = this;
      this.children.push(child);
      return child;
    }
    insertBefore(child, before) {
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
    removeChild(child) {
      const index = this.children.indexOf(child);
      if (index >= 0) {
        this.children.splice(index, 1);
        child.parentNode = null;
      }
      return child;
    }
    remove() {
      this.parentNode?.removeChild(this);
    }
    addEventListener() {
    }
    removeEventListener() {
    }
    dispatchEvent() {
      return true;
    }
    querySelector(selector) {
      return this.querySelectorAll(selector)[0] ?? null;
    }
    querySelectorAll(selector) {
      const out = [];
      const visit = (node) => {
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
    matches(selector) {
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
    cloneNode(deep = false) {
      const clone = new _VirtualElement(this.ownerDocument, this.tagName, this.namespaceURI);
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
    serialize() {
      const style = this.style.toString();
      const attrs = new Map(this.attrs);
      if (style && !attrs.has("style")) {
        attrs.set("style", style);
      }
      const attrText = Array.from(attrs.entries()).map(([key, value]) => ` ${key}="${escapeXml(value)}"`).join("");
      const body = `${escapeXml(this.textContent)}${this.children.map((child) => child.serialize()).join("")}`;
      return `<${this.tagName}${attrText}>${body}</${this.tagName}>`;
    }
  };
  var VirtualDocument = class {
    constructor() {
      this.documentElement = new VirtualElement(this, "html");
      this.body = new VirtualElement(this, "body");
      this.documentElement.appendChild(this.body);
    }
    createElement(tagName) {
      return new VirtualElement(this, tagName);
    }
    createElementNS(namespaceURI, tagName) {
      return new VirtualElement(this, tagName, namespaceURI);
    }
    querySelector(selector) {
      return this.documentElement.querySelector(selector);
    }
    querySelectorAll(selector) {
      return this.documentElement.querySelectorAll(selector);
    }
  };
  var VirtualXmlSerializer = class {
    serializeToString(node) {
      return node.serialize();
    }
  };
  function ensureHeadlessDom() {
    const global = globalThis;
    if (global.document) {
      return global.document;
    }
    const document2 = new VirtualDocument();
    global.document = document2;
    global.window = {
      document: document2,
      matchMedia: () => ({ matches: false, addEventListener: () => {
      }, removeEventListener: () => {
      } })
    };
    global.XMLSerializer = VirtualXmlSerializer;
    global.SVGElement = VirtualElement;
    global.Element = VirtualElement;
    global.Node = VirtualElement;
    return document2;
  }
  async function exportHeadlessSvg(payload, options = {}) {
    const document2 = ensureHeadlessDom();
    const target = document2.createElement("div");
    target.clientWidth = options.width ?? 1280;
    target.clientHeight = options.height ?? 900;
    document2.body.appendChild(target);
    try {
      const prepared = preparedViewFromPayload(payload) ?? prepareViewData(payload);
      const controller = await renderVisualization(target, prepared, {
        delegateZoom: true,
        theme: { colorScheme: options.colorScheme ?? "light" }
      });
      const svg = controller.exportSvg();
      controller.destroy();
      return svg;
    } finally {
      target.remove();
    }
  }
  function escapeXml(value) {
    return value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
  }
  var globalApi = globalThis;
  globalApi.Spec42HeadlessRenderer = { exportHeadlessSvg };
  return __toCommonJS(headless_export_exports);
})();
