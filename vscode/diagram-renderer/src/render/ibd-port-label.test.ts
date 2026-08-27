import { describe, expect, it } from "vitest";

import { formatIbdPortLabel, ibdPortLabelText } from "./ibd-port-label";

describe("IBD port labels", () => {
  it("shows only the port name", () => {
    expect(formatIbdPortLabel("powerInput", { name: "powerInput", portType: "~PowerRailPort" })).toBe("powerInput");
  });

  it("limits the rendered label to twenty characters including the ellipsis", () => {
    const label = ibdPortLabelText("electricalTransmitDataInput");
    expect(label).toBe("electricalTransmitD…");
    expect(label).toHaveLength(20);
  });
});
