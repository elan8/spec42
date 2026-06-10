import { describe, expect, it } from "vitest";
import { prepareViewData } from "./prepare";

/** Fixtures mirroring LSP DTOs after dtoAdapter merge (view field set). */
const VIEW_FIXTURES: Array<{ view: string; data: Record<string, unknown> }> = [
  {
    view: "general-view",
    data: {
      view: "general-view",
      graph: {
        nodes: [{ id: "a", name: "PartA", type: "part def" }],
        edges: [],
      },
    },
  },
  {
    view: "interconnection-view",
    data: {
      view: "interconnection-view",
      ibd: {
        parts: [{ id: "p1", name: "Part1", type: "part" }],
        connectors: [],
        rootCandidates: [],
      },
    },
  },
  {
    view: "action-flow-view",
    data: {
      view: "action-flow-view",
      activityDiagrams: [
        {
          id: "act-1",
          name: "Mission",
          nodes: [{ id: "n1", name: "Start", type: "initial" }],
          flows: [],
        },
      ],
    },
  },
  {
    view: "state-transition-view",
    data: {
      view: "state-transition-view",
      stateMachines: [
        {
          id: "sm-1",
          name: "Timer",
          states: [{ id: "s1", name: "Idle", kind: "state" }],
          transitions: [],
        },
      ],
    },
  },
  {
    view: "sequence-view",
    data: {
      view: "sequence-view",
      sequenceDiagrams: [
        {
          id: "seq-1",
          name: "Interaction",
          lifelines: [{ id: "l1", name: "A" }],
          messages: [{ id: "m1", from: "l1", to: "l1", name: "self" }],
        },
      ],
    },
  },
  {
    view: "browser-view",
    data: {
      view: "browser-view",
      graph: { nodes: [{ id: "b1", name: "Block", type: "part def" }], edges: [] },
    },
  },
  {
    view: "grid-view",
    data: {
      view: "grid-view",
      graph: { nodes: [{ id: "g1", name: "Cell", type: "part def" }], edges: [] },
    },
  },
  {
    view: "geometry-view",
    data: {
      view: "geometry-view",
      graph: { nodes: [{ id: "geo1", name: "Shape", type: "part def" }], edges: [] },
    },
  },
];

describe("dto pipeline prepareViewData", () => {
  for (const { view, data } of VIEW_FIXTURES) {
    it(`prepares ${view}`, () => {
      const prepared = prepareViewData(data);
      expect(prepared.view).toBe(view);
      expect(Array.isArray(prepared.nodes)).toBe(true);
      expect(Array.isArray(prepared.edges)).toBe(true);
    });
  }
});
