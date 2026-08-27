/**
 * Deterministic node-chrome stress corpus for the Chrome visual-review harness.
 *
 * These are authored `PreparedView` values -- the same contract `prepareViewData()` publishes --
 * so the harness exercises the renderer without inventing model semantics. Repository diagram
 * products (`tests/snapshots/generation/diagram_*.md`) cover the per-view regression corpus; these
 * cases isolate node chrome, header layout, disclosure controls, and compartment density, which
 * no single checked-in product happens to combine.
 */
import type { PreparedEdge, PreparedNode, PreparedView } from "../src/prepare/types";

export interface VisualCase {
  id: string;
  title: string;
  /** Node ids whose disclosure control the harness activates before the screenshot. */
  expand?: string[];
  prepared: PreparedView;
}

/**
 * Compartment members carry the id of the element they list, exactly as repository diagram
 * products do. The renderer drops a listed member once that element is drawn as its own node, so
 * omitting ids here would make the expanded cases unrepresentative.
 */
function members(entries: Array<string | [string, string]>): Array<{ id?: string; name: string }> {
  return entries.map((entry) =>
    typeof entry === "string" ? { name: entry } : { id: entry[0], name: entry[1] },
  );
}

function typedCompartment(
  kind: string,
  provenance: "direct" | "inherited",
  entries: Array<string | [string, string]>,
): Record<string, unknown> {
  return { kind, provenance, members: members(entries) };
}

function node(
  index: number,
  label: string,
  kind: string,
  notationRole: string,
  extra: Record<string, unknown> = {},
): PreparedNode {
  return {
    id: `n:${index}`,
    label,
    kind,
    uri: "memory://visual/harness.sysml",
    range: { start: { line: index, character: 0 }, end: { line: index, character: label.length } },
    attributes: { notationRole, owner: null, ...extra },
  };
}

function edge(index: number, source: string, target: string, edgeKind: string, label = ""): PreparedEdge {
  return { id: `e:${index}`, source, target, label, edgeKind };
}

const LONG_NAME = "VehicleThermalManagementSubsystemAssembly";

/** Collapsed General View root: direct + inherited compartments, hidden relationships, long name. */
const collapsedRoot: PreparedView = {
  title: "Collapsed root",
  view: "general-view",
  meta: { exposedRoots: ["n:0", "n:4"] },
  nodes: [
    node(0, LONG_NAME, "PartDefinition", "definition", {
      typedCompartments: [
        typedCompartment("attributes", "direct", [
          "coolantMass : MassValue",
          "nominalFlowRate : VolumetricFlowRateValue",
        ]),
        typedCompartment("parts", "direct", [
          ["n:1", "radiator : Radiator"],
          ["n:2", "pump : CoolantPump"],
        ]),
        typedCompartment("attributes", "inherited", ["serialNumber : String", "massMargin : Real"]),
        typedCompartment("parts", "inherited", [["n:3", "housing : Housing"]]),
      ],
    }),
    { ...node(1, "radiator", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0, partType: "Radiator" } },
    { ...node(2, "pump", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0, partType: "CoolantPump" } },
    { ...node(3, "housing", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0 } },
    node(4, "Radiator", "PartDefinition", "definition"),
    node(5, "CoolantPump", "PartDefinition", "definition"),
  ],
  edges: [
    edge(0, "n:1", "n:4", "typing"),
    edge(1, "n:2", "n:5", "typing"),
    edge(2, "n:3", "n:4", "reference"),
  ],
};

/** Definition / usage / reference-usage / unsupported chrome side by side. */
const nodeKinds: PreparedView = {
  title: "Node kinds",
  view: "general-view",
  nodes: [
    node(0, "CoolantPump", "PartDefinition", "definition", {
      typedCompartments: [typedCompartment("attributes", "direct", ["ratedPower : PowerValue"])],
    }),
    node(1, "pump", "PartUsage", "usage", {
      partType: "CoolantPump",
      typedCompartments: [typedCompartment("attributes", "direct", ["ratedPower = 120 [W]"])],
    }),
    node(2, "referencedPump", "ReferenceUsage", "reference-usage", { partType: "CoolantPump" }),
    node(3, "unsupportedNotation", "MetadataUsage", "unsupported"),
  ],
  edges: [edge(0, "n:1", "n:0", "typing"), edge(1, "n:2", "n:1", "reference")],
};

/** Nodes with no compartments at all. */
const noCompartments: PreparedView = {
  title: "No compartments",
  view: "general-view",
  nodes: [
    node(0, "Sensor", "PartDefinition", "definition"),
    node(1, "actuator", "PartUsage", "usage"),
    node(2, "ControlUnit", "PartDefinition", "definition"),
  ],
  edges: [edge(0, "n:1", "n:0", "typing"), edge(1, "n:2", "n:0", "specializes")],
};

/** Several compartments, long member names, and a truncated list. */
const denseCompartments: PreparedView = {
  title: "Dense compartments",
  view: "general-view",
  nodes: [
    node(0, "ThermalControlUnit", "PartDefinition", "definition", {
      typedCompartments: [
        typedCompartment("attributes", "direct", [
          "measuredCoolantInletTemperature : TemperatureValue",
          "measuredCoolantOutletTemperature : TemperatureValue",
          "commandedPumpDutyCycle : DimensionlessValue",
          "diagnosticTroubleCodeRegister : DiagnosticCodeSet",
          "calibrationRevisionIdentifier : String",
          "thermalDeratingThreshold : TemperatureValue",
          "coolantLevelWarningThreshold : VolumeValue",
          "controllerHeartbeatPeriod : DurationValue",
          "watchdogTimeoutPeriod : DurationValue",
          "overtemperatureShutdownDelay : DurationValue",
        ]),
        typedCompartment("ports", "direct", [
          "in coolantTemperatureSensorSignal : TemperatureSignal",
          "out pumpDutyCycleCommand : DutyCycleCommand",
        ]),
        typedCompartment("parts", "inherited", [
          "diagnosticLogBuffer : CircularDiagnosticLogBuffer",
          "calibrationTable : ThermalCalibrationTable",
        ]),
      ],
    }),
    node(1, "SignalConditioner", "PartDefinition", "definition", {
      typedCompartments: [
        typedCompartment("attributes", "direct", ["gain : Real", "offset : Real"]),
        typedCompartment("ports", "direct", ["in raw : Signal", "out conditioned : Signal"]),
      ],
    }),
  ],
  edges: [edge(0, "n:1", "n:0", "reference")],
};

/** Three nesting levels reachable through the disclosure control. */
const nestedExpansion: PreparedView = {
  title: "Nested expansion",
  view: "general-view",
  meta: { exposedRoots: ["n:0"] },
  nodes: [
    node(0, "Vehicle", "PartDefinition", "definition", {
      typedCompartments: [typedCompartment("parts", "direct", [["n:1", "powertrain : Powertrain"]])],
    }),
    { ...node(1, "powertrain", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0, partType: "Powertrain", typedCompartments: [typedCompartment("parts", "direct", [["n:2", "engine : Engine"]])] } },
    { ...node(2, "engine", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 1, partType: "Engine", typedCompartments: [typedCompartment("parts", "direct", [["n:3", "cylinderBank : CylinderBank"]])] } },
    { ...node(3, "cylinderBank", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 2, partType: "CylinderBank" } },
  ],
  edges: [edge(0, "n:1", "n:2", "reference"), edge(1, "n:2", "n:3", "reference")],
};

/** Dense relationships around a collapsed hub. */
const denseRelationships: PreparedView = {
  title: "Dense relationships",
  view: "general-view",
  meta: { exposedRoots: ["n:0", "n:5", "n:6", "n:7", "n:8", "n:9"] },
  nodes: [
    node(0, "IntegrationHub", "PartDefinition", "definition", {
      typedCompartments: [
        typedCompartment("parts", "direct", [
          ["n:1", "portA : PortA"],
          ["n:2", "portB : PortB"],
          ["n:3", "portC : PortC"],
          ["n:4", "portD : PortD"],
        ]),
      ],
    }),
    { ...node(1, "portA", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0 } },
    { ...node(2, "portB", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0 } },
    { ...node(3, "portC", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0 } },
    { ...node(4, "portD", "PartUsage", "usage"), attributes: { notationRole: "usage", owner: 0 } },
    node(5, "SubsystemAlpha", "PartDefinition", "definition"),
    node(6, "SubsystemBeta", "PartDefinition", "definition"),
    node(7, "SubsystemGamma", "PartDefinition", "definition"),
    node(8, "SubsystemDelta", "PartDefinition", "definition"),
    node(9, "SubsystemEpsilon", "PartDefinition", "definition"),
  ],
  edges: [
    edge(0, "n:1", "n:5", "typing"),
    edge(1, "n:2", "n:6", "typing"),
    edge(2, "n:3", "n:7", "typing"),
    edge(3, "n:4", "n:8", "typing"),
    edge(4, "n:1", "n:9", "reference"),
    edge(5, "n:2", "n:9", "reference"),
    edge(6, "n:5", "n:6", "specializes"),
    edge(7, "n:7", "n:8", "specializes"),
  ],
};

export const SYNTHETIC_CASES: VisualCase[] = [
  { id: "node-collapsed-root", title: "General View - collapsed root (long name, inherited, hidden relationships)", prepared: collapsedRoot },
  { id: "node-expanded-root", title: "General View - same root expanded", prepared: collapsedRoot, expand: ["n:0"] },
  { id: "node-kinds", title: "Definition / usage / reference / unsupported chrome", prepared: nodeKinds },
  { id: "node-no-compartments", title: "Nodes without compartments", prepared: noCompartments },
  { id: "node-dense-compartments", title: "Several compartments, long members, truncated list", prepared: denseCompartments },
  { id: "node-nested-expansion", title: "Nested expansion levels", prepared: nestedExpansion, expand: ["n:0", "n:1", "n:2"] },
  { id: "node-dense-relationships-collapsed", title: "Dense relationships around a collapsed hub", prepared: denseRelationships },
  { id: "node-dense-relationships-expanded", title: "Dense relationships around an expanded hub", prepared: denseRelationships, expand: ["n:0"] },
];
