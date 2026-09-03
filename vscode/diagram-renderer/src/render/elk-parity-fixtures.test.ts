import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import type { PreparedEdge, PreparedNode, PreparedView } from "../prepare";
import { buildBehaviorElkGraphInput } from "../views/behavior-common";
import { buildGeneralElkGraphInput } from "./layout";

const fixtureDir = join(
  dirname(fileURLToPath(import.meta.url)),
  "../../../../tools/elkrs_parity/fixtures",
);

function node(id: string, kind: string, label = id, attributes?: Record<string, unknown>): PreparedNode {
  return { id, kind, label, attributes };
}

function edge(id: string, source: string, target: string, label = ""): PreparedEdge {
  return { id, source, target, label, edgeKind: "dependency" };
}

function prepared(
  title: string,
  view: string,
  nodes: PreparedNode[],
  edges: PreparedEdge[],
  meta?: Record<string, unknown>,
): PreparedView {
  return { title, view, nodes, edges, meta };
}

function assertCheckedInFixture(name: string, actual: Record<string, unknown>): void {
  const path = join(fixtureDir, name);
  if (process.env.UPDATE_ELK_FIXTURES === "1") {
    writeFileSync(path, `${JSON.stringify(actual, null, 2)}\n`, "utf8");
  }
  expect(existsSync(path), `missing checked-in parity fixture ${path}`).toBe(true);
  expect(actual).toEqual(JSON.parse(readFileSync(path, "utf8")));
}

const actionNodes = [
  node("start", "initial"),
  node("sense", "action"),
  node("decide", "decision"),
  node("act", "action"),
  node("done", "final"),
];
const actionEdges = [
  edge("e1", "start", "sense"),
  edge("e2", "sense", "decide", "sample ready"),
  edge("e3", "decide", "act", "accepted"),
  edge("e4", "act", "done"),
];

const stateNodes = [
  node("initial", "initial"),
  node("idle", "state"),
  node("active", "state"),
  node("fault", "state"),
  node("final", "final"),
];
const stateEdges = [
  edge("s1", "initial", "idle", "entry"),
  edge("s2", "idle", "active", "start [ready] / engage"),
  edge("s3", "active", "fault", "failure"),
  edge("s4", "active", "idle", "stop"),
  edge("s5", "fault", "final"),
];

describe("ELK parity fixtures", () => {
  it("keeps General View fixtures owned by the production graph builder", () => {
    const flat = prepared(
      "General Flat",
      "general-view",
      [
        node("system", "PartUsage", "system"),
        node("sensor", "PartUsage", "sensor"),
        node("controller", "PartUsage", "controller"),
        node("actuator", "PartUsage", "actuator"),
      ],
      [
        edge("owns-sensor", "system", "sensor"),
        edge("owns-controller", "system", "controller"),
        edge("controls", "controller", "actuator"),
      ],
    );
    const hierarchical = prepared(
      "General Hierarchical",
      "general-view",
      [
        node("sensor", "PartUsage", "sensor"),
        node("controller", "PartUsage", "controller"),
        node("actuator", "PartUsage", "actuator"),
      ],
      [edge("signals", "sensor", "controller"), edge("commands", "controller", "actuator")],
      {
        packageContainerGroups: [
          { id: "pkg:sensing", label: "Sensing", memberIds: ["sensor"] },
          { id: "pkg:control", label: "Control", memberIds: ["controller", "actuator"] },
        ],
      },
    );
    const wide = prepared(
      "Wide Siblings",
      "general-view",
      Array.from({ length: 12 }, (_, index) => node(`part-${index + 1}`, "PartUsage")),
      Array.from({ length: 11 }, (_, index) => edge(`w${index + 1}`, `part-${index + 1}`, `part-${index + 2}`)),
    );

    const flatInput = buildGeneralElkGraphInput(flat);
    const hierarchicalInput = buildGeneralElkGraphInput(hierarchical);
    const wideInput = buildGeneralElkGraphInput(wide);
    expect(flatInput).not.toBeNull();
    expect(hierarchicalInput).not.toBeNull();
    expect(wideInput).not.toBeNull();
    assertCheckedInFixture("general-flat.json", flatInput!);
    assertCheckedInFixture("general-hierarchical.json", hierarchicalInput!);
    assertCheckedInFixture("wide-siblings.json", wideInput!);
  });

  it("keeps action-flow fixtures owned by the production graph builder", () => {
    const view = prepared("action-flow", "action-flow-view", actionNodes, actionEdges);
    assertCheckedInFixture(
      "action-flow.json",
      buildBehaviorElkGraphInput(view, { horizontal: true, mode: "action" }),
    );
    assertCheckedInFixture(
      "action-flow-down.json",
      buildBehaviorElkGraphInput(view, { horizontal: false, mode: "action" }),
    );
  });

  it("keeps state-transition fixtures owned by the production graph builder", () => {
    const view = prepared("state-transition", "state-transition-view", stateNodes, stateEdges);
    assertCheckedInFixture(
      "state-transition.json",
      buildBehaviorElkGraphInput(view, { horizontal: false, mode: "state" }),
    );
    assertCheckedInFixture(
      "state-transition-right.json",
      buildBehaviorElkGraphInput(view, { horizontal: true, mode: "state" }),
    );
  });
});
