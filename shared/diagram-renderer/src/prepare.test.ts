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

  it("prepares interconnection from canonical scene fixture", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      interconnectionScene: {
        schemaVersion: 1,
        view: {
          id: "fixture-two-part",
          name: "TwoPartChain",
          type: "InterconnectionView",
          rootIds: [],
        },
        nodes: [
          { id: "node:Demo.Source", semanticId: "Demo.Source", qualifiedName: "Demo.Source", name: "Source", kind: "part" },
          { id: "node:Demo.Target", semanticId: "Demo.Target", qualifiedName: "Demo.Target", name: "Target", kind: "part" },
        ],
        ports: [
          { id: "port:Demo.Source.out", semanticId: "Demo.Source.out", ownerNodeId: "node:Demo.Source", name: "out", direction: "out", sideHint: "east" },
          { id: "port:Demo.Target.in", semanticId: "Demo.Target.in", ownerNodeId: "node:Demo.Target", name: "in", direction: "in", sideHint: "west" },
        ],
        edges: [
          {
            id: "edge:Demo.Source.out->Demo.Target.in:0",
            kind: "connection",
            sourcePortId: "port:Demo.Source.out",
            targetPortId: "port:Demo.Target.in",
            sourceNodeId: "node:Demo.Source",
            targetNodeId: "node:Demo.Target",
          },
        ],
        containers: [],
        diagnostics: [],
      },
    });
    expect(prepared.meta?.canonicalScene).toBe(true);
    expect(prepared.nodes).toHaveLength(2);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].target).toBe("node:Demo.Target");
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
