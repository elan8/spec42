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

  it("maps interconnection payload and filters invalid connectors", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedViewName: "Root",
      ibd: {
        parts: [
          { id: "p1", name: "Engine", type: "part" },
          { id: "p2", name: "Controller", type: "part" },
        ],
        connectors: [
          { id: "c1", sourcePartId: "p1", targetPartId: "p2", name: "connect" },
          { id: "c2", sourcePartId: "p1", targetPartId: "missing", name: "bad" },
        ],
      },
    });
    expect(prepared.nodes).toHaveLength(2);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].id).toBe("c1");
    expect(prepared.meta?.selectedRoot).toBe("Root");
  });

  it("prefers selectedIbdRoot over the SysML view name for interconnection scoping", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedViewName: "gridConnections",
      selectedIbdRoot: "architecture",
      ibd: {
        defaultRoot: "architecture",
        rootCandidates: ["tinyRoot", "architecture"],
        rootViews: {
          tinyRoot: {
            parts: [{ id: "tiny", name: "tinyRoot", qualifiedName: "Pkg.tinyRoot", type: "part" }],
            connectors: [],
            ports: [],
          },
          architecture: {
            parts: [
              { id: "feeder", name: "feederNorth", qualifiedName: "Pkg.architecture.feederNorth", type: "part" },
              { id: "cable", name: "cable01", qualifiedName: "Pkg.architecture.cable01", type: "part" },
            ],
            connectors: [
              { id: "c1", sourcePartId: "feeder", targetPartId: "cable", name: "connect" },
            ],
            ports: [],
          },
        },
        parts: [],
        connectors: [],
        ports: [],
      },
    });

    expect(prepared.meta?.selectedRoot).toBe("architecture");
    expect(prepared.nodes.map((node) => node.id).sort()).toEqual(["cable", "feeder"]);
    expect(prepared.edges.map((edge) => edge.id)).toEqual(["c1"]);
  });

  it("does not auto-scope an explicit SysML interconnection view to the IBD default root", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedView: "StedinRijnmondGridExpansion::Views::gridConnections",
      selectedViewName: "gridConnections",
      ibd: {
        defaultRoot: "batteryA",
        rootCandidates: ["batteryA"],
        rootViews: {
          batteryA: {
            parts: [],
            connectors: [],
            ports: [],
          },
        },
        parts: [
          { id: "feeder", name: "feederNorth", qualifiedName: "Pkg.architecture.feederNorth", type: "part" },
          { id: "cable", name: "cable01", qualifiedName: "Pkg.architecture.cable01", type: "part" },
        ],
        connectors: [
          { id: "c1", sourcePartId: "feeder", targetPartId: "cable", name: "connect" },
        ],
        ports: [],
      },
    });

    expect(prepared.title).toBe("Interconnection View");
    expect(prepared.meta?.selectedRoot).toBeNull();
    expect(prepared.nodes.map((node) => node.id).sort()).toEqual(["cable", "feeder"]);
    expect(prepared.edges.map((edge) => edge.id)).toEqual(["c1"]);
  });

  it("keeps interconnection container metadata for renderer parity", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedViewName: "ConnectedBlocks",
      ibd: {
        rootCandidates: ["ConnectedBlocks", "IT"],
        packageContainerGroups: [
          { id: "pkg1", name: "ConnectedBlocks", memberIds: ["Pkg.Engine", "Pkg.Controller"] },
        ],
        parts: [
          { id: "p1", name: "Engine", qualifiedName: "Pkg.Engine", type: "part" },
          { id: "p2", name: "Controller", qualifiedName: "Pkg.Controller", type: "part" },
        ],
        connectors: [
          { id: "c1", sourcePartId: "p1", targetPartId: "p2", sourceId: "Engine.out", targetId: "Controller.in", type: "flow" },
        ],
      },
    });
    expect(prepared.meta?.rootCandidates).toEqual(["ConnectedBlocks", "IT"]);
    expect(Array.isArray(prepared.meta?.packageContainerGroups)).toBe(true);
    expect(prepared.nodes.some((node) => node.id === "pkg1")).toBe(true);
    expect(prepared.nodes.find((node) => node.id === "p1")?.attributes?.containerId).toBe("pkg1");
    expect(prepared.edges[0].attributes?.relationType).toBe("flow");
  });

  it("normalizes interconnection containment aliases for compound layout", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      ibd: {
        parts: [
          { id: "root-id", name: "Drone", qualifiedName: "Pkg.Drone", type: "part def" },
          { id: "controller-id", name: "flightController", qualifiedName: "Pkg.Drone.flightController", containerId: "Pkg.Drone", type: "part" },
          { id: "gps-id", name: "gps", qualifiedName: "Pkg.Drone.flightController.gps", containerId: "Pkg.Drone.flightController", type: "part" },
        ],
      },
    });

    expect(prepared.nodes.find((node) => node.id === "controller-id")?.attributes?.containerId).toBe("root-id");
    expect(prepared.nodes.find((node) => node.id === "gps-id")?.attributes?.containerId).toBe("controller-id");
  });

  it("drops empty synthetic interconnection containers", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      ibd: {
        packageContainerGroups: [
          { id: "empty-root", label: "SurveillanceDrone", memberIds: ["missing.member"] },
        ],
        parts: [
          { id: "airframe", name: "airframe", qualifiedName: "Pkg.SurveillanceDrone.airframe", type: "part" },
        ],
      },
    });

    expect(prepared.nodes.map((node) => node.id)).toEqual(["airframe"]);
  });

  it("collapses package wrapper when scoped to an instance root", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      selectedViewName: "droneInstance",
      ibd: {
        defaultRoot: "droneInstance",
        rootViews: {
          droneInstance: {
            parts: [
              {
                id: "inst",
                name: "droneInstance",
                qualifiedName: "SurveillanceDrone.droneInstance",
                type: "part",
              },
              {
                id: "power",
                name: "power",
                qualifiedName: "SurveillanceDrone.droneInstance.power",
                containerId: "SurveillanceDrone.droneInstance",
                type: "part",
              },
            ],
            connectors: [],
            ports: [],
            containerGroups: [
              {
                id: "container:SurveillanceDrone",
                label: "SurveillanceDrone",
                qualifiedName: "SurveillanceDrone",
                memberPartIds: ["inst", "power"],
              },
            ],
          },
        },
      },
    });

    expect(
      prepared.nodes.some(
        (node) => node.label === "SurveillanceDrone" && node.attributes?.isSyntheticContainer,
      ),
    ).toBe(false);
    expect(prepared.nodes.find((node) => node.id === "inst")?.attributes?.containerId).toBeFalsy();
    expect(prepared.nodes.find((node) => node.id === "inst")?.attributes?.isDiagramRoot).toBe(true);
    expect(prepared.nodes.find((node) => node.id === "power")?.attributes?.containerId).toBe("inst");
  });

  it("normalizes IBD connector semantics for notation rendering", () => {
    const prepared = prepareViewData({
      view: "interconnection-view",
      ibd: {
        parts: [
          { id: "tank", name: "tank", qualifiedName: "Vehicle.tank", type: "part" },
          { id: "engine", name: "engine", qualifiedName: "Vehicle.engine", type: "part" },
          { id: "controller", name: "controller", qualifiedName: "Vehicle.controller", type: "part" },
        ],
        connectors: [
          {
            id: "fuel-flow",
            source: "Vehicle.tank.fuelOut",
            target: "Vehicle.engine.fuelIn",
            name: "flow",
            itemType: "Fuel",
          },
          {
            id: "control-interface",
            sourcePartId: "controller",
            targetPartId: "engine",
            type: "interface",
            interfaceName: "EngineControl",
          },
          {
            id: "engine-reference",
            sourcePartId: "controller",
            targetPartId: "engine",
            type: "reference",
          },
        ],
      },
    });

    expect(prepared.edges.find((edge) => edge.id === "fuel-flow")?.edgeKind).toBe("flow");
    expect(prepared.edges.find((edge) => edge.id === "fuel-flow")?.label).toBe("Fuel");
    expect(prepared.edges.find((edge) => edge.id === "fuel-flow")?.source).toBe("tank");
    expect(prepared.edges.find((edge) => edge.id === "fuel-flow")?.target).toBe("engine");
    expect(prepared.edges.find((edge) => edge.id === "control-interface")?.edgeKind).toBe("interface");
    expect(prepared.edges.find((edge) => edge.id === "control-interface")?.attributes?.interfaceName).toBe("EngineControl");
    expect(prepared.edges.find((edge) => edge.id === "engine-reference")?.edgeKind).toBe("reference");
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
