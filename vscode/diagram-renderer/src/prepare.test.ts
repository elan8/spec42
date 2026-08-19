import { describe, expect, it } from "vitest";
import { normalizeEdgeKind } from "./graph-normalization";
import { prepareViewData } from "./prepare";

describe("shared graph normalization", () => {
  it("normalizes known relationship kinds", () => {
    expect(normalizeEdgeKind("typing")).toBe("typing");
    expect(normalizeEdgeKind("specialization")).toBe("specializes");
    expect(normalizeEdgeKind("owns")).toBe("hierarchy");
    expect(normalizeEdgeKind("dependency")).toBe("dependency");
    expect(normalizeEdgeKind("defined by")).toBe("typing");
    expect(normalizeEdgeKind("allocation")).toBe("allocate");
    expect(normalizeEdgeKind("")).toBe("relationship");
  });
});

describe("shared prepareViewData", () => {
  it("expands normalized diagram indexes without deriving semantics", () => {
    const prepared = prepareViewData({
      schemaVersion: 2,
      modelDigest: "blake3:model",
      documents: [{ uri: "file:///model.sysml", sourceDomain: "workspace" }],
      sources: [{ document: 0, range: [1, 2, 1, 6] }],
      references: [
        { kind: "qualified-name", document: 0, qualifiedName: "P::view" },
        { kind: "qualified-name", document: 0, qualifiedName: "P::a" },
        { kind: "qualified-name", document: 0, qualifiedName: "P::b" },
        { kind: "relationship", source: 1, relationshipKind: "flow", ordinal: 0 },
      ],
      selectedView: { reference: 0, kind: "general-view", name: "view", source: 0 },
      completeness: { status: "complete", reasons: [] },
      projection: {
        kind: "general-view",
        exposedRoots: [0],
        nodes: [
          { reference: 1, metaclass: "PartUsage", name: "a", owner: null, source: 0 },
          { reference: 2, metaclass: "PartUsage", name: "b", owner: null, source: 0 },
        ],
        relationships: [],
        edges: [{ reference: 3, source: 0, target: 1, kind: "flow", provenance: "authored", navigation: 0 }],
        metadata: { roots: [0] },
      },
    });
    expect(prepared.nodes.map((node) => node.id)).toEqual(["n:0", "n:1"]);
    expect(prepared.edges[0]).toMatchObject({ id: "e:0", source: "n:0", target: "n:1", edgeKind: "flow" });
    expect(prepared.nodes[0]).toMatchObject({ uri: "file:///model.sysml", range: { start: { line: 1, character: 2 } } });
    expect(prepared.nodes[0]?.attributes?.semanticReference).toEqual({ kind: "qualified-name", document: 0, qualifiedName: "P::a" });
  });

  it("maps general graph payload and omits package namespace nodes", () => {
    const prepared = prepareViewData({
      view: "general-view",
      selectedViewName: "General",
      graph: {
        nodes: [
          { id: "pkg", name: "Pkg", type: "package" },
          { id: "a", name: "A", type: "part_def" },
          { id: "b", name: "B", type: "part_def" },
        ],
        edges: [
          { id: "contains", source: "pkg", target: "a", type: "contains" },
          { id: "rel", source: "a", target: "b", type: "typing" },
        ],
      },
    });
    expect(prepared.nodes.map((n) => n.id)).toEqual(["a", "b"]);
    expect(prepared.nodes.find((n) => n.id === "a")?.attributes?.isDefinition).toBe(true);
    expect(prepared.nodes.find((n) => n.id === "a")?.attributes?.isReference).toBe(false);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].edgeKind).toBe("typing");
  });

  it("omits documentation and comment nodes from general-view diagrams", () => {
    const prepared = prepareViewData({
      view: "general-view",
      selectedViewName: "Functional",
      generalViewGraph: {
        nodes: [
          { id: "ControlMission", name: "ControlMission", type: "action def" },
          { id: "ControlMission::_documentation", name: "", type: "documentation" },
          { id: "SenseEnvironment::_comment", name: "", type: "comment" },
        ],
        edges: [
          {
            id: "owns-doc",
            source: "ControlMission",
            target: "ControlMission::_documentation",
            type: "contains",
          },
          {
            id: "annotates",
            source: "ControlMission::_documentation",
            target: "ControlMission",
            type: "annotation",
          },
        ],
      },
    });
    expect(prepared.nodes.map((n) => n.id)).toEqual(["ControlMission"]);
    expect(prepared.edges).toHaveLength(0);
  });

  it("builds package container groups for multi-package general graphs", () => {
    const prepared = prepareViewData({
      view: "general-view",
      graph: {
        nodes: [
          { id: "a", name: "A", type: "part def", qualifiedName: "PkgA::A" },
          { id: "b", name: "B", type: "part def", qualifiedName: "PkgB::B" },
        ],
        edges: [],
      },
    });
    const groups = prepared.meta?.packageContainerGroups as Array<{ name: string; memberIds: string[] }>;
    expect(groups).toHaveLength(2);
    expect(groups?.map((g) => g.name).sort()).toEqual(["PkgA", "PkgB"]);
  });

  it("builds package container groups when the raw payload has no separate qualifiedName field", () => {
    // Regression test for O-4: graph nodes coming off the Rust side never carry a top-level or
    // `attributes.qualifiedName` field -- `id` *is* the qualified name (e.g.
    // "PhysicalArchitecture::BaseModule"). Without falling back to `id`, package grouping (and
    // the compact-layout chunking it enables) silently never activated for any real payload.
    const prepared = prepareViewData({
      view: "general-view",
      graph: {
        nodes: [
          { id: "PkgA::A", name: "A", type: "part def" },
          { id: "PkgB::B", name: "B", type: "part def" },
        ],
        edges: [],
      },
    });
    const groups = prepared.meta?.packageContainerGroups as Array<{ name: string; memberIds: string[] }>;
    expect(groups).toHaveLength(2);
    expect(groups?.map((g) => g.name).sort()).toEqual(["PkgA", "PkgB"]);
  });

  it("marks reference usages on prepared nodes", () => {
    const prepared = prepareViewData({
      view: "general-view",
      graph: {
        nodes: [
          { id: "def", name: "HitchBall", type: "part def" },
          { id: "ref", name: "hitchBall", type: "ref" },
        ],
        edges: [],
      },
    });
    expect(prepared.nodes.find((n) => n.id === "ref")?.attributes?.isReference).toBe(true);
    expect(prepared.nodes.find((n) => n.id === "ref")?.attributes?.isDefinition).toBe(false);
  });

  it("omits library package nodes from general graphs", () => {
    const prepared = prepareViewData({
      view: "general-view",
      graph: {
        nodes: [
          { id: "lib-pkg", name: "Lib", type: "library package" },
          { id: "part", name: "Part", type: "part def" },
        ],
        edges: [],
      },
    });

    expect(prepared.nodes.map((n) => n.id)).toEqual(["part"]);
  });

  it("omits real and synthetic package nodes from general graphs", () => {
    const prepared = prepareViewData({
      view: "general-view",
      graph: {
        nodes: [
          { id: "synthetic-pkg", name: "Synthetic", type: "package", attributes: { synthetic: true } },
          { id: "real-pkg", name: "Real", type: "package" },
          { id: "part", name: "Part", type: "part def" },
        ],
        edges: [],
      },
    });

    expect(prepared.nodes.map((n) => n.id)).toEqual(["part"]);
  });

  it("prefers canonical generalViewGraph when present", () => {
    const prepared = prepareViewData({
      view: "general-view",
      graph: {
        nodes: [{ id: "legacy", name: "Legacy", type: "part_def" }],
        edges: [],
      },
      generalViewGraph: {
        nodes: [{ id: "canonical", name: "Canonical", type: "part_def" }],
        edges: [],
      },
    });
    expect(prepared.nodes.map((n) => n.id)).toEqual(["canonical"]);
  });

  it("builds Browser View membership hierarchy when legacy parent_id is blank", () => {
    const prepared = prepareViewData({
      view: "browser-view",
      selectedViewName: "Structure Browser",
      projectionHints: {
        browserLayout: "hierarchy",
        treeRoots: ["root"],
      },
      generalViewGraph: {
        nodes: [
          { id: "root", name: "Root", type: "part def", parent_id: "", parentId: "" },
          { id: "child", name: "Child", type: "part", parent_id: "", parentId: "root" },
        ],
        edges: [],
      },
    });

    const rows = prepared.meta?.rows as Array<{
      id: string;
      depth: number;
      hasChildren: boolean;
      parentId: string;
    }>;
    expect(prepared.view).toBe("browser-view");
    expect(prepared.meta?.hierarchyLayout).toBe(true);
    expect(prepared.meta?.provisional).toBe(false);
    expect(rows.map((row) => [row.id, row.depth])).toEqual([
      ["root", 0],
      ["child", 1],
    ]);
    expect(rows[0]?.hasChildren).toBe(true);
    expect(rows[1]?.parentId).toBe("root");
  });

  it("surfaces visibility on Browser View rows when present in node attributes", () => {
    const prepared = prepareViewData({
      view: "browser-view",
      projectionHints: { browserLayout: "hierarchy" },
      generalViewGraph: {
        nodes: [
          {
            id: "privatePart",
            name: "privatePart",
            type: "part",
            parentId: "",
            attributes: { visibility: "Private" },
          },
          { id: "publicPart", name: "publicPart", type: "part", parentId: "" },
        ],
        edges: [],
      },
    });
    const rows = prepared.meta?.rows as Array<{ id: string; visibility?: string }>;
    expect(rows.find((row) => row.id === "privatePart")?.visibility).toBe("Private");
    expect(rows.find((row) => row.id === "publicPart")?.visibility).toBeUndefined();
  });

  it("preserves explicit treeRoots order in Browser View hierarchy instead of alphabetizing", () => {
    const prepared = prepareViewData({
      view: "browser-view",
      projectionHints: {
        browserLayout: "hierarchy",
        treeRoots: ["zRoot", "aRoot"],
      },
      generalViewGraph: {
        nodes: [
          { id: "zRoot", name: "zRoot", type: "part", parentId: "" },
          { id: "aRoot", name: "aRoot", type: "part", parentId: "" },
        ],
        edges: [],
      },
    });
    const rows = prepared.meta?.rows as Array<{ id: string }>;
    expect(rows.map((row) => row.id)).toEqual(["zRoot", "aRoot"]);
  });

  it("preserves sibling declaration order in Browser View hierarchy when no treeRoots hint is present", () => {
    const prepared = prepareViewData({
      view: "browser-view",
      projectionHints: { browserLayout: "hierarchy" },
      generalViewGraph: {
        nodes: [
          { id: "root", name: "root", type: "part", parentId: "" },
          { id: "zChild", name: "zChild", type: "part", parentId: "root" },
          { id: "aChild", name: "aChild", type: "part", parentId: "root" },
        ],
        edges: [],
      },
    });
    const rows = prepared.meta?.rows as Array<{ id: string }>;
    expect(rows.map((row) => row.id)).toEqual(["root", "zChild", "aChild"]);
  });

  it("keeps every relationship kind between the same pair of elements in the relationship matrix", () => {
    const prepared = prepareViewData({
      view: "grid-view",
      projectionHints: { gridSubtype: "relationship_matrix" },
      generalViewGraph: {
        nodes: [
          { id: "a", name: "a", type: "part" },
          { id: "b", name: "b", type: "part" },
        ],
        edges: [
          { source: "a", target: "b", type: "Dependency" },
          { source: "a", target: "b", type: "Satisfy" },
        ],
      },
    });
    expect(prepared.meta?.relationshipMatrix).toBe(true);
    const matrixCells = prepared.meta?.matrixCells as Array<{
      source: string;
      target: string;
      present: boolean;
      labels: string[];
    }>;
    const cell = matrixCells.find((entry) => entry.source === "a" && entry.target === "b");
    expect(cell?.present).toBe(true);
    expect(cell?.labels).toEqual(["Dependency", "Satisfy"]);
  });

  it("builds Grid View columns from columnView projection hints when present", () => {
    const prepared = prepareViewData({
      view: "grid-view",
      projectionHints: {
        columnViews: [{ label: "columnView[1]", renderingType: "asTextualNotation" }],
      },
      generalViewGraph: {
        nodes: [{ id: "a", name: "a", type: "part" }],
        edges: [],
      },
    });
    const columns = prepared.meta?.columns as Array<{ key: string; label: string }>;
    expect(columns).toEqual([{ key: "name", label: "columnView[1]" }]);
  });

  it("falls back to default Grid View columns when no columnView hints are present", () => {
    const prepared = prepareViewData({
      view: "grid-view",
      generalViewGraph: {
        nodes: [{ id: "a", name: "a", type: "part" }],
        edges: [],
      },
    });
    expect(prepared.meta?.columns).toBeUndefined();
  });

  it("prepares an ordinary GridView as a standard element table", () => {
    const prepared = prepareViewData({
      view: "grid-view",
      selectedViewName: "Parts",
      generalViewGraph: {
        nodes: [{ id: "robot", name: "robot", type: "part" }],
        edges: [],
      },
    });

    expect(prepared.view).toBe("grid-view");
    expect(prepared.meta?.relationshipMatrix).toBe(false);
    expect(prepared.meta?.provisional).toBe(false);
  });

  it("prepares interconnection from canonical scene fixture", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      interconnectionScene: {
        schemaVersion: 2,
        view: {
          id: "fixture-two-part",
          name: "TwoPartChain",
          type: "InterconnectionView",
          rootIds: [],
        },
        nodes: [
          { id: "occ:Demo.Source", semanticId: "Demo.Source", qualifiedName: "Demo.Source", name: "Source", kind: "part" },
          { id: "occ:Demo.Target", semanticId: "Demo.Target", qualifiedName: "Demo.Target", name: "Target", kind: "part" },
        ],
        ports: [
          { id: "occ:Demo.Source.out", semanticId: "Demo.Source.out", ownerNodeId: "occ:Demo.Source", name: "out", direction: "out", sideHint: "east" },
          { id: "occ:Demo.Target.in", semanticId: "Demo.Target.in", ownerNodeId: "occ:Demo.Target", name: "in", direction: "in", sideHint: "west" },
        ],
        edges: [
          {
            id: "edge:Demo.Source.out->Demo.Target.in:0",
            kind: "connection",
            sourcePortId: "occ:Demo.Source.out",
            targetPortId: "occ:Demo.Target.in",
            sourceNodeId: "occ:Demo.Source",
            targetNodeId: "occ:Demo.Target",
          },
        ],
        containers: [],
        diagnostics: [],
      },
    });
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(prepared.nodes).toHaveLength(2);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].target).toBe("occ:Demo.Target");
  });

  it("returns empty prepared view when interconnectionScene is missing", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      ibd: {
        parts: [{ id: "p1", name: "Engine", type: "part" }],
        connectors: [],
      },
    });
    expect(prepared.nodes).toHaveLength(0);
    expect(prepared.edges).toHaveLength(0);
    const diagnostics = prepared.meta?.diagnostics as Array<{ code?: string }> | undefined;
    expect(diagnostics?.some((item) => item.code === "missing_interconnection_scene")).toBe(true);
  });

  it("adds synthetic initial state when missing", () => {
    const prepared = prepareViewData({
      view: "state-transition-view",
      selectedViewName: "TimerStateMachine",
      synthesizeInitialState: true,
      graph: {
        nodes: [
          { id: "s1", name: "Idle", type: "state" },
          { id: "s2", name: "Running", type: "state" },
        ],
        edges: [{ source: "s1", target: "s2", type: "transition", name: "start" }],
      },
    });
    const initialNode = prepared.nodes.find((node) => node.kind === "initial");
    expect(initialNode).toBeTruthy();
    expect(prepared.edges.some((edge) => edge.label === "initial" || edge.id.includes("entry"))).toBe(true);
  });

  it("matches sequence diagram when view usage name differs in case from diagram name", () => {
    const prepared = prepareViewData({
      view: "sequence-view",
      selectedViewName: "orderEventFanout",
      sequenceDiagrams: [
        {
          id: "WebShopArchitecture::OrderEventFanout::sequence",
          name: "OrderEventFanout",
          package_path: "WebShopArchitecture",
          lifelines: [
            { id: "WebShopArchitecture::checkoutService", name: "checkoutService" },
            { id: "WebShopArchitecture::ordersEventsTopic", name: "ordersEventsTopic" },
          ],
          messages: [
            {
              id: "m1",
              source: "WebShopArchitecture::checkoutService",
              target: "WebShopArchitecture::ordersEventsTopic",
              name: "order-created",
              type: "async",
            },
          ],
        },
      ],
    });
    expect(prepared.view).toBe("sequence-view");
    expect(prepared.nodes.length).toBeGreaterThanOrEqual(2);
    expect(prepared.edges.length).toBeGreaterThanOrEqual(1);
    expect(prepared.meta?.selectedDiagramName).toBe("OrderEventFanout");
  });

  it("resolves action-flow flows when node ids are qualified but flow endpoints are simple names", () => {
    const prepared = prepareViewData({
      view: "action-flow-view",
      activityDiagrams: [
        {
          id: "WebShopBehavior::CheckoutPipeline",
          name: "CheckoutPipeline",
          actions: [
            { id: "WebShopBehavior::CheckoutPipeline::validateCart", name: "validateCart", type: "action" },
            { id: "WebShopBehavior::CheckoutPipeline::authorizePayment", name: "authorizePayment", type: "action" },
            { id: "WebShopBehavior::CheckoutPipeline::reserveInventory", name: "reserveInventory", type: "action" },
          ],
          flows: [
            { from: "validateCart", to: "authorizePayment" },
            { from: "authorizePayment", to: "reserveInventory" },
          ],
        },
      ],
    });
    expect(prepared.edges).toHaveLength(2);
    expect(prepared.edges[0]?.source).toBe("WebShopBehavior::CheckoutPipeline::validateCart");
    expect(prepared.edges[1]?.target).toBe("WebShopBehavior::CheckoutPipeline::reserveInventory");
  });

  it("distinguishes streaming flows from successions", () => {
    const prepared = prepareViewData({
      view: "action-flow-view",
      activityDiagrams: [{
        id: "Demo::Behavior",
        name: "Behavior",
        actions: [
          { id: "Demo::Behavior::a", name: "a", type: "action" },
          { id: "Demo::Behavior::b", name: "b", type: "action" },
          { id: "Demo::Behavior::c", name: "c", type: "action" },
        ],
        flows: [
          { id: "stream", from: "a", to: "b", guard: "flow" },
          { id: "sequence", from: "b", to: "c", guard: "succession" },
        ],
      }],
    });

    expect(prepared.edges[0]?.attributes).toMatchObject({
      streamingFlow: true,
      succession: false,
      flowKind: "streaming",
    });
    expect(prepared.edges[1]?.attributes).toMatchObject({
      streamingFlow: false,
      succession: true,
      flowKind: "succession",
    });
  });

  it("matches action-flow diagram when view usage name differs in case from diagram name", () => {
    const prepared = prepareViewData({
      view: "action-flow-view",
      selectedViewName: "checkoutPipeline",
      activityDiagrams: [
        {
          id: "WebShopBehavior::CheckoutPipeline",
          name: "CheckoutPipeline",
          nodes: [
            { id: "validateCart", name: "validateCart" },
            { id: "authorizePayment", name: "authorizePayment" },
          ],
          edges: [{ id: "e1", source: "validateCart", target: "authorizePayment", name: "flow" }],
        },
      ],
    });
    expect(prepared.title).toBe("CheckoutPipeline");
    expect(prepared.nodes).toHaveLength(2);
    expect(prepared.edges).toHaveLength(1);
  });

  it("prefers richest action-flow candidate", () => {
    const prepared = prepareViewData({
      view: "action-flow-view",
      activityDiagrams: [
        { id: "d1", name: "Simple", nodes: [{ id: "n1", name: "A" }], edges: [] },
        {
          id: "d2",
          name: "Rich",
          nodes: [{ id: "n1", name: "A" }, { id: "n2", name: "B" }],
          edges: [{ id: "e1", source: "n1", target: "n2", name: "flow" }],
        },
      ],
    });
    expect(prepared.title).toBe("Rich");
    expect(prepared.meta?.selectedDiagramId).toBe("d2");
  });

  it("prepares state machines with alias-resolved transitions and navigation metadata", () => {
    const prepared = prepareViewData({
      view: "state-transition-view",
      selectedViewName: "OrderLifecycleStateMachine",
      stateMachines: [
        {
          id: "WebShopBehavior::OrderLifecycleStateMachine",
          name: "OrderLifecycleStateMachine",
          states: [
            {
              id: "WebShopBehavior::OrderLifecycleStateMachine::Pending",
              name: "Pending",
              kind: "state",
              element: {
                uri: "file:///webshop.sysml",
                range: { start: { line: 40, character: 8 }, end: { line: 40, character: 20 } },
              },
            },
            {
              id: "WebShopBehavior::OrderLifecycleStateMachine::Shipped",
              name: "Shipped",
              kind: "state",
              element: {
                uri: "file:///webshop.sysml",
                range: { start: { line: 44, character: 8 }, end: { line: 44, character: 20 } },
              },
            },
          ],
          transitions: [
            { id: "t1", source: "Pending", target: "Shipped", label: "ship" },
          ],
        },
      ],
    });

    expect(prepared.nodes).toHaveLength(2);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0]?.source).toBe("WebShopBehavior::OrderLifecycleStateMachine::Pending");
    expect(prepared.edges[0]?.target).toBe("WebShopBehavior::OrderLifecycleStateMachine::Shipped");
    expect(prepared.nodes[0]?.uri).toBe("file:///webshop.sysml");
    expect(prepared.nodes[0]?.range?.start?.line).toBe(40);
    expect(prepared.meta?.parentContext).toBe("OrderLifecycleStateMachine");
  });
});
