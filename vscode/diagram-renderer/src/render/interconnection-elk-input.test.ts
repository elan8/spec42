import { describe, expect, it } from "vitest";

import type { InterconnectionPreparedNode, InterconnectionPreparedView } from "../prepare";
import type { InterconnectionPreparedPortDetail } from "../prepare/types";
import { buildInterconnectionElkBuild } from "./interconnection-elk-input";

/**
 * `sideForPort` places a port on its authored side (see #126): an authored `direction` wins,
 * then connector topology, then a stable hash of the port's identity -- never a read of the
 * port's or its owner's name. These fixtures exercise exactly that ordering without depending on
 * the schema-5 prepare pipeline.
 */
function port(overrides: Partial<InterconnectionPreparedPortDetail>): InterconnectionPreparedPortDetail {
  const name = overrides.name ?? "p";
  return {
    id: overrides.id ?? `port::${name}`,
    name,
    attributes: { parentId: "n:0", scenePortId: overrides.id ?? `port::${name}`, sideHint: "" },
    ...overrides,
  };
}

function node(id: string, ports: InterconnectionPreparedPortDetail[]): InterconnectionPreparedNode {
  return {
    id,
    label: id,
    kind: "part",
    attributes: { qualifiedName: id, portDetails: ports },
  };
}

function prepared(nodes: InterconnectionPreparedNode[]): InterconnectionPreparedView {
  return {
    title: "t",
    view: "interconnection-view",
    nodes,
    edges: [],
    meta: { canonicalScene: true },
  };
}

function sidesFor(nodeId: string, ports: InterconnectionPreparedPortDetail[]): { west: string[]; east: string[] } {
  const build = buildInterconnectionElkBuild(prepared([node(nodeId, ports)]));
  return build.portDrawOrderFor(build.nodesById.get(nodeId)!);
}

describe("interconnection port side placement", () => {
  it("places an authored `in` port west and an authored `out` port east", () => {
    const { west, east } = sidesFor("n:0", [
      port({ id: "a", name: "a", direction: "in" }),
      port({ id: "b", name: "b", direction: "out" }),
    ]);
    expect(west).toEqual(["a"]);
    expect(east).toEqual(["b"]);
  });

  it("does not derive a side from the owning node's label", () => {
    // Every word here ("camera", "gimbal", "sensor", "battery", ...) matched the deleted
    // label-guessing regexes. Neither port authors a direction and neither has a connector, so
    // both must land the same way whichever node owns them.
    const ports = [port({ id: "data", name: "data" })];
    const { west: cameraWest, east: cameraEast } = sidesFor("CameraGimbalModule", ports);
    const { west: sensorWest, east: sensorEast } = sidesFor("SensorBatteryModule", ports);
    expect([cameraWest, cameraEast]).toEqual([sensorWest, sensorEast]);
  });

  it("does not derive a side from the port's own name", () => {
    // "sensorIn" / "cameraOut" matched the deleted name-substring guesses (`in` / `out` word
    // boundaries). The same port identity must land on the same side whichever of those two
    // names it carries -- only `id` may decide, per `stableSide`.
    const sideOf = (name: string): "west" | "east" => {
      const { west } = sidesFor("n:0", [port({ id: "x", name })]);
      return west.length > 0 ? "west" : "east";
    };
    expect(sideOf("sensorIn")).toBe(sideOf("cameraOut"));
  });

  it("is deterministic across renders for a port with no resolved signal", () => {
    const ports = [port({ id: "x", name: "unlabeled" })];
    const first = sidesFor("n:0", ports);
    const second = sidesFor("n:0", ports);
    expect(first).toEqual(second);
  });
});
