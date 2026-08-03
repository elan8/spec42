import { describe, expect, it } from "vitest";

import type { PreparedView } from "../prepare";
import { edgeTooltipDescriptor, portTooltipDescriptor, tooltipText } from "./diagram-tooltip";

function edgeText(view: string, attributes: Record<string, unknown>, edgeKind = "relationship"): string {
  const prepared: PreparedView = {
    title: "Test",
    view,
    nodes: [
      { id: "a", label: "Alpha", kind: "part" },
      { id: "b", label: "Beta", kind: "part" },
    ],
    edges: [{ id: "e", source: "a", target: "b", label: "named relation", edgeKind, attributes }],
  };
  return tooltipText(edgeTooltipDescriptor(prepared.edges[0], prepared));
}

describe("diagram tooltip descriptors", () => {
  it("includes conjugated type, direction, explicit multiplicity, and qualified name for a port", () => {
    const value = tooltipText(portTooltipDescriptor({
      id: "System.receiver.input",
      name: "input",
      portType: "~OpticalPort",
      direction: "in",
      multiplicity: "[0..1]",
      semanticId: "Architecture::System::receiver::input",
    }));
    expect(value).toContain("Type: ~OpticalPort");
    expect(value).toContain("Direction: in");
    expect(value).toContain("Multiplicity: [0..1]");
    expect(value).toContain("Qualified name: Architecture::System::receiver::input");
  });

  it("uses the implicit default multiplicity for a port usage", () => {
    expect(tooltipText(portTooltipDescriptor({ name: "power" }))).toContain("Multiplicity: [1]");
  });

  it.each([
    ["general-view", { relationType: "typing", semanticId: "typing-1" }, ["Typing", "Source: Alpha (a)", "Target: Beta (b)", "Semantic ID: typing-1"]],
    ["interconnection-view", { relationType: "bind", sourceExpression: "a.out", targetExpression: "b.in", sourcePortId: "a.out#1", targetPortId: "b.in#1" }, ["Bind", "Source: a.out", "Target: b.in", "Resolved source: a.out#1", "Resolved target: b.in#1"]],
    ["action-flow-view", { flowKind: "succession", guard: "ready", condition: "enabled" }, ["Succession", "Guard: ready", "Condition: enabled"]],
    ["state-transition-view", { relationType: "transition", trigger: "button", accept: "Start", guard: "armed", effect: "launch", send: "Started" }, ["Transition", "Trigger: button", "Accept: Start", "Guard: armed", "Effect: launch", "Send: Started"]],
    ["sequence-view", { kind: "message", messageKind: "async", order: 3 }, ["Message", "Message kind: async", "Order: 3"]],
  ])("describes %s relationships", (view, attributes, expected) => {
    const value = edgeText(view, attributes);
    for (const line of expected) expect(value).toContain(line);
  });
});
