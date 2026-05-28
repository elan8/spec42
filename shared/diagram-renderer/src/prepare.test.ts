import { describe, expect, it } from "vitest";
import { normalizeEdgeKind } from "./graph-normalization";
import { prepareViewData } from "./prepare";

describe("shared graph normalization", () => {
  it("normalizes known relationship kinds", () => {
    expect(normalizeEdgeKind("typing")).toBe("typing");
    expect(normalizeEdgeKind("specialization")).toBe("specializes");
    expect(normalizeEdgeKind("owns")).toBe("hierarchy");
    expect(normalizeEdgeKind("")).toBe("relationship");
  });
});

describe("shared prepareViewData", () => {
  it("maps general graph payload and omits package nodes", () => {
    const prepared = prepareViewData({
      view: "general-view",
      selectedViewName: "General",
      graph: {
        nodes: [
          { id: "pkg", name: "Pkg", type: "package" },
          { id: "a", name: "A", type: "part_def" },
          { id: "b", name: "B", type: "part_def" },
        ],
        edges: [{ id: "rel", source: "a", target: "b", type: "typing" }],
      },
    });
    expect(prepared.nodes.map((n) => n.id)).toEqual(["a", "b"]);
    expect(prepared.edges).toHaveLength(1);
    expect(prepared.edges[0].edgeKind).toBe("typing");
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
    expect(prepared.nodes[0].id).toBe("__synthetic_initial__");
    expect(prepared.edges.some((edge) => edge.id === "transition-synthetic-initial")).toBe(true);
    expect(prepared.meta?.syntheticInitial).toBe(true);
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
});
