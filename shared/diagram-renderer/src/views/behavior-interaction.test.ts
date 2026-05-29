// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import * as d3 from "d3";
import type { PreparedNode } from "../prepare";
import {
  attachBehaviorNodeClick,
  clearBehaviorHighlights,
  jumpPayloadFromNode,
  nodeSupportsSourceNavigation,
} from "./behavior-interaction";
import { resolveDiagramTheme } from "../theme";

function sampleNode(overrides: Partial<PreparedNode> = {}): PreparedNode {
  return {
    id: "Pkg::Diagram::validateCart",
    label: "validateCart",
    kind: "action",
    sourcePath: "file:///model.sysml",
    uri: "file:///model.sysml",
    range: { start: { line: 12, character: 4 }, end: { line: 12, character: 20 } },
    attributes: { qualifiedName: "Pkg::Diagram::validateCart" },
    ...overrides,
  };
}

describe("behavior interaction", () => {
  it("builds jump payload with qualified id, uri, range, and parentContext", () => {
    const payload = jumpPayloadFromNode(sampleNode(), "CheckoutPipeline");
    expect(payload.name).toBe("validateCart");
    expect(payload.id).toBe("Pkg::Diagram::validateCart");
    expect(payload.uri).toBe("file:///model.sysml");
    expect(payload.range?.start?.line).toBe(12);
    expect(payload.parentContext).toBe("CheckoutPipeline");
  });

  it("omits id when node is not qualified", () => {
    const payload = jumpPayloadFromNode(
      sampleNode({ id: "validateCart", attributes: { qualifiedName: "validateCart" } }),
    );
    expect(payload.id).toBeUndefined();
  });

  it("detects when a node has enough metadata for source navigation", () => {
    expect(nodeSupportsSourceNavigation(sampleNode())).toBe(true);
    expect(
      nodeSupportsSourceNavigation({
        id: "n1",
        label: "",
        kind: "action",
        sourcePath: null,
        uri: null,
        range: null,
        attributes: {},
      }),
    ).toBe(false);
  });

  it("applies highlight class and invokes onNodeClick", () => {
    const svg = d3.select(document.body).append("svg");
    const root = svg.append("g");
    const nodeGroup = root
      .append("g")
      .attr("class", "activity-action")
      .append("g");
    nodeGroup
      .append("rect")
      .attr("class", "node-background")
      .attr("data-original-stroke", "#ccc")
      .attr("data-original-width", "2px")
      .style("stroke", "#ccc")
      .style("stroke-width", "2px");

    const theme = resolveDiagramTheme({ colorScheme: "light" });
    const onNodeClick = vi.fn();
    attachBehaviorNodeClick(nodeGroup, sampleNode(), theme, { onNodeClick }, root);

    const event = new MouseEvent("click", { bubbles: true, cancelable: true });
    nodeGroup.node()?.dispatchEvent(event);

    expect(onNodeClick).toHaveBeenCalledTimes(1);
    expect(nodeGroup.classed("highlighted-element")).toBe(true);
    expect(nodeGroup.select(".node-background").style("stroke")).toBe(theme.highlight);

    clearBehaviorHighlights(root);
    expect(nodeGroup.classed("highlighted-element")).toBe(false);
    svg.remove();
  });
});
