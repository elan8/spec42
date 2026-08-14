import { describe, expect, it } from "vitest";

import type { PreparedView } from "../prepare";
import { edgeTooltipDescriptor, portTooltipDescriptor, tooltipFallbackText, tooltipText } from "./diagram-tooltip";

function edgeText(view: string, attributes: Record<string, unknown>, edgeKind = "relationship", fallback = false): string {
  const prepared: PreparedView = {
    title: "Test",
    view,
    nodes: [
      { id: "a", label: "Alpha", kind: "part" },
      { id: "b", label: "Beta", kind: "part" },
    ],
    edges: [{ id: "e", source: "a", target: "b", label: "named relation", edgeKind, attributes }],
  };
  const descriptor = edgeTooltipDescriptor(prepared.edges[0], prepared);
  return fallback ? tooltipFallbackText(descriptor) : tooltipText(descriptor);
}

describe("diagram tooltip descriptors", () => {
  it("includes conjugated type, direction, explicit multiplicity, and qualified name for a port", () => {
    const descriptor = portTooltipDescriptor({
      id: "System.receiver.input",
      name: "input",
      portType: "~OpticalPort",
      direction: "in",
      multiplicity: "[0..1]",
      semanticId: "Architecture::System::receiver::input",
    });
    const value = tooltipText(descriptor);
    expect(value).toContain("Type: ~OpticalPort");
    expect(value).toContain("Direction: in");
    expect(value).toContain("Multiplicity: [0..1]");
    expect(value).not.toContain("Qualified name:");
    expect(tooltipFallbackText(descriptor)).toContain("Qualified name: Architecture::System::receiver::input");
  });

  it("uses the implicit default multiplicity for a port usage", () => {
    const descriptor = portTooltipDescriptor({ name: "power" });
    expect(tooltipText(descriptor)).not.toContain("Multiplicity:");
    expect(tooltipFallbackText(descriptor)).toContain("Multiplicity: [1]");
  });

  it.each([
    ["general-view", { relationType: "typing", semanticId: "typing-1" }, ["Typing", "From: Alpha", "To: Beta"]],
    ["interconnection-view", { relationType: "bind", sourceExpression: "a.out", targetExpression: "b.in", sourcePortId: "a.out#1", targetPortId: "b.in#1" }, ["Bind", "From: a.out", "To: b.in"]],
    ["action-flow-view", { flowKind: "succession", guard: "ready", condition: "enabled" }, ["Succession", "Guard: ready", "Condition: enabled"]],
    ["state-transition-view", { relationType: "transition", trigger: "button", accept: "Start", guard: "armed", effect: "launch", send: "Started" }, ["Transition", "Trigger: button", "Accept: Start", "Guard: armed", "Effect: launch", "Send: Started"]],
    ["sequence-view", { kind: "message", messageKind: "async", order: 3 }, ["Message", "Message kind: async", "Order: 3"]],
  ])("describes %s relationships", (view, attributes, expected) => {
    const value = edgeText(view, attributes);
    for (const line of expected) expect(value).toContain(line);
  });

  it("keeps technical IBD identifiers out of the visual tooltip but in the SVG fallback", () => {
    const attributes = {
      relationType: "bind",
      sourceExpression: "Architecture::OpticalTransmitter::electricalDataInput",
      targetExpression: "Architecture::TransmitElectronics::txDataInput",
      sourcePortId: "occ:Architecture.transceiver.transmitter.electricalDataInput",
      targetPortId: "occ:Architecture.transceiver.transmitter.transmitElectronics.txDataInput",
      semanticId: "binding-1",
    };
    const visual = edgeText("interconnection-view", attributes);
    expect(visual).toContain("From: OpticalTransmitter.electricalDataInput");
    expect(visual).toContain("To: TransmitElectronics.txDataInput");
    expect(visual).not.toContain("Resolved source");
    expect(visual).not.toContain("Semantic ID");
    const fallback = edgeText("interconnection-view", attributes, "bind", true);
    expect(fallback).toContain("Resolved source: occ:Architecture.transceiver.transmitter.electricalDataInput");
    expect(fallback).toContain("Semantic ID: binding-1");
  });
});
