// @ts-nocheck
import assert from "assert";
import { describe, it } from "vitest";
import { graphToElementTree } from "../graph-to-element-tree";
import { normalizeVisualizationPayload } from "./normalize-payload";

function prepareDataForView(data: Record<string, unknown> | null, view: string): Record<string, unknown> | null | undefined {
    if (!data) return data;
    return normalizeVisualizationPayload({ ...data, view });
}

/**
 * Minimal mock data in the format produced by modelFetcher / fetchModelData.
 * With graph: nodes + edges (preferred). Without: elements + relationships (legacy).
 */
const createMockData = (overrides: Partial<{
    elements: unknown[];
    relationships: unknown[];
    activityDiagrams: unknown[];
    sequenceDiagrams: unknown[];
}> = {}) => ({
    elements: [
        {
            name: "SurveillanceDrone",
            type: "package",
            id: "SurveillanceDrone",
            attributes: {},
            properties: {},
            typing: undefined,
            typings: [],
            children: [
                {
                    name: "Propulsion",
                    type: "part def",
                    id: "Propulsion",
                    children: [
                        {
                            name: "propulsionUnit1",
                            type: "part",
                            id: "propulsionUnit1",
                            typing: "PropulsionUnit",
                            typings: ["PropulsionUnit"],
                            children: [],
                            relationships: []
                        }
                    ],
                    relationships: []
                },
                {
                    name: "PatrolOverwatch",
                    type: "use case def",
                    id: "PatrolOverwatch",
                    children: [],
                    relationships: []
                },
                {
                    name: "Operator",
                    type: "item def",
                    id: "Operator",
                    children: [],
                    relationships: []
                },
                {
                    name: "FlightModeStateMachine",
                    type: "state def",
                    id: "FlightModeStateMachine",
                    children: [
                        { name: "manual", type: "state", id: "manual", children: [], relationships: [] }
                    ],
                    relationships: []
                }
            ],
            relationships: [
                { type: "typing", source: "propulsionUnit1", target: "PropulsionUnit" }
            ]
        }
    ],
    relationships: [
        { type: "specializes", source: "A", target: "B" },
        { type: "connection", source: "X", target: "Y" }
    ],
    activityDiagrams: [
        {
            name: "Act1",
            actions: [{ name: "start", type: "initial", kind: "initial", id: "start" }],
            flows: [{ from: "start", to: "done" }],
            decisions: []
        }
    ],
    sequenceDiagrams: [],
    ...overrides
});

describe("normalizeVisualizationPayload", () => {
    const VIEW_IDS = ["general-view", "interconnection-view"];

    VIEW_IDS.forEach((viewId) => {
        it(`returns non-null for view "${viewId}"`, () => {
            const data = createMockData();
            const result = prepareDataForView(data, viewId);
            assert.ok(result != null, `prepareDataForView for "${viewId}" should return non-null`);
            assert.ok(result !== undefined, `prepareDataForView for "${viewId}" should return defined`);
        });
    });

    it("returns data unchanged for unknown view (pass-through)", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "unknown");
        assert.deepStrictEqual(result, { ...data, view: "unknown" }, "Unknown view should pass through payload with view field");
    });

    it("returns null/undefined for null input", () => {
        assert.strictEqual(prepareDataForView(null, "general-view"), null);
    });

    it("interconnection-view passes through canonical scene payload", () => {
        const scene = {
            schemaVersion: 2,
            view: { id: "v1", name: "Demo", type: "InterconnectionView", rootIds: [] },
            nodes: [],
            ports: [],
            edges: [],
            containers: [],
            diagnostics: [],
        };
        const result = prepareDataForView({ interconnectionScene: scene }, "interconnection-view");
        assert.strictEqual(result.interconnectionScene, scene);
    });

    it("interconnection-view without scene returns empty ibd arrays", () => {
        const result = prepareDataForView({ ibd: { parts: [{ id: "p1", name: "Part" }] } }, "interconnection-view");
        assert.deepStrictEqual(result.parts, []);
        assert.deepStrictEqual(result.connectors, []);
        assert.deepStrictEqual(result.ports, []);
    });

    it("action-flow-view produces diagrams", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "action-flow-view");
        assert.ok(Array.isArray(result.diagrams), "action-flow-view should have diagrams array");
    });

    it("action-flow-view filters interface-only action definitions from the selector", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    name: "UpdateDisplay",
                    actions: [
                        { name: "renderDisplay", type: "action", kind: "perform", id: "renderDisplay" },
                    ],
                    interface: {
                        inputs: ["currentTime"],
                        outputs: ["displayText"],
                    },
                    flows: [],
                    decisions: [],
                    states: [],
                }
            ]
        });

        const result = prepareDataForView(data, "action-flow-view");
        assert.deepStrictEqual(result.diagrams, []);
        assert.ok(Array.isArray(result.activityDiagramCandidates));
        assert.deepStrictEqual(result.activityDiagramCandidates, []);
    });

    it("action-flow-view preserves explicit behavioral flows without synthesizing control edges", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    id: "ExecuteMission::actionDef",
                    name: "ExecuteMission",
                    packagePath: "Mission::Control",
                    sourceKind: "actionDef",
                    actions: [
                        { name: "captureVideo", type: "action", kind: "perform", id: "captureVideo" },
                        { name: "sendReport", type: "action", kind: "perform", id: "sendReport" },
                    ],
                    interface: {
                        inputs: ["route"],
                        outputs: ["report"],
                    },
                    flows: [
                        { from: "captureVideo", to: "sendReport", guard: "whenReady" },
                    ],
                    decisions: [],
                    states: [],
                }
            ]
        });

        const result = prepareDataForView(data, "action-flow-view");
        const diagram = result.diagrams[0];

        assert.deepStrictEqual(
            diagram.nodes.map((node: any) => node.name),
            ["captureVideo", "sendReport"],
        );
        assert.strictEqual(diagram.flows.length, 1);
        assert.strictEqual(diagram.flows[0].from, "captureVideo");
        assert.strictEqual(diagram.flows[0].to, "sendReport");
        assert.strictEqual(diagram.flows[0].guard, "whenReady");
        assert.strictEqual(diagram.hasBehavioralFlow, true);
        assert.strictEqual(result.activityDiagramCandidates[0].flowCount, 1);
    });

    it("action-flow-view filters empty backend action diagrams and keeps performer contexts", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    id: "Mission::CatalogAction::actionDef",
                    name: "CatalogAction",
                    packagePath: "Mission",
                    sourceKind: "actionDef",
                    actions: [],
                    interface: {
                        inputs: ["route"],
                        outputs: ["status"],
                    },
                    flows: [],
                    decisions: [],
                    states: [],
                },
                {
                    id: "Mission::FlightController::performer",
                    name: "FlightController",
                    packagePath: "Mission",
                    sourceKind: "performer",
                    actions: [
                        { name: "assessVehicleState", type: "action", kind: "perform", id: "assessVehicleState" },
                        { name: "manageMissionEvents", type: "action", kind: "perform", id: "manageMissionEvents" },
                    ],
                    flows: [
                        { from: "assessVehicleState", to: "manageMissionEvents" },
                    ],
                    decisions: [],
                    states: [],
                }
            ]
        });

        const result = prepareDataForView(data, "action-flow-view");

        assert.deepStrictEqual(
            result.activityDiagramCandidates.map((candidate: any) => candidate.name),
            ["FlightController"]
        );
        assert.strictEqual(result.activityDiagramCandidates[0].sourceKind, "performer");
        assert.strictEqual(result.diagrams[0].hasBehavioralFlow, true);
    });

    it("action-flow-view filters node-only diagrams that still lack behavioral flows", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    id: "Mission::ExecuteOutboundJourney::actionDef",
                    name: "ExecuteOutboundJourney",
                    packagePath: "Mission",
                    sourceKind: "actionDef",
                    actions: [
                        { name: "prep", type: "action", kind: "action", id: "prep" },
                        { name: "launch", type: "action", kind: "action", id: "launch" },
                    ],
                    flows: [],
                    decisions: [],
                    states: [],
                },
                {
                    id: "Mission::LaunchSystem::performer",
                    name: "LaunchSystem",
                    packagePath: "Mission",
                    sourceKind: "performer",
                    actions: [
                        { name: "provideStage1Thrust", type: "action", kind: "perform", id: "provideStage1Thrust" },
                        { name: "provideStage2Thrust", type: "action", kind: "perform", id: "provideStage2Thrust" },
                    ],
                    flows: [
                        { from: "provideStage1Thrust", to: "provideStage2Thrust" },
                    ],
                    decisions: [],
                    states: [],
                }
            ]
        });

        const result = prepareDataForView(data, "action-flow-view");

        assert.deepStrictEqual(
            result.activityDiagramCandidates.map((candidate: any) => candidate.name),
            ["LaunchSystem"]
        );
    });

    it("action-flow-view ranks explicit action-def diagrams ahead of performer contexts", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    id: "Mission::FlightController::performer",
                    name: "FlightController",
                    packagePath: "Mission",
                    sourceKind: "performer",
                    actions: [
                        { name: "assessVehicleState", type: "action", kind: "perform", id: "assessVehicleState" },
                        { name: "manageMissionEvents", type: "action", kind: "perform", id: "manageMissionEvents" },
                    ],
                    flows: [
                        { from: "assessVehicleState", to: "manageMissionEvents" },
                    ],
                    decisions: [],
                    states: [],
                },
                {
                    id: "Mission::ExecuteMission::actionDef",
                    name: "ExecuteMission",
                    packagePath: "Mission",
                    sourceKind: "actionDef",
                    actions: [
                        { name: "captureVideo", type: "action", kind: "perform", id: "captureVideo" },
                        { name: "sendReport", type: "action", kind: "perform", id: "sendReport" },
                    ],
                    flows: [
                        { from: "captureVideo", to: "sendReport" },
                    ],
                    decisions: [],
                    states: [],
                }
            ]
        });

        const result = prepareDataForView(data, "action-flow-view");

        assert.strictEqual(result.activityDiagramCandidates[0].name, "ExecuteMission");
        assert.strictEqual(result.activityDiagramCandidates[0].sourceKind, "actionDef");
        assert.strictEqual(result.activityDiagramCandidates[1].name, "FlightController");
    });

    it("sequence-view normalizes lifelines, messages, activations, and fragments", () => {
        const data = createMockData({
            sequenceDiagrams: [
                {
                    id: "OrderFlow",
                    name: "OrderFlow",
                    packagePath: "Examples::Software",
                    lifelines: [
                        { id: "client", name: "Client", type: "APIClient" },
                        { id: "service", name: "Service", type: "OrderService" },
                    ],
                    messages: [
                        { id: "m1", from: "client", to: "service", kind: "sync", order: 1, label: "placeOrder" },
                        { id: "m2", from: "service", to: "client", kind: "return", order: 2, label: "accepted" },
                    ],
                    activations: [
                        { id: "a1", lifeline: "service", startMessage: "m1", finishMessage: "m2", order: 1 },
                    ],
                    fragments: [
                        {
                            id: "f1",
                            kind: "opt",
                            messageIds: ["m1", "m2"],
                            operands: [{ id: "op1", guard: "orderValid", messageIds: ["m1", "m2"] }],
                            order: 1,
                        },
                    ],
                },
            ],
        });

        const result = prepareDataForView(data, "sequence-view");
        assert.ok(Array.isArray(result.diagrams));
        assert.strictEqual(result.diagrams.length, 1);
        assert.strictEqual(result.diagrams[0].lifelines.length, 2);
        assert.strictEqual(result.diagrams[0].messages[1].kind, "return");
        assert.strictEqual(result.diagrams[0].activations[0].lifeline, "service");
        assert.strictEqual(result.diagrams[0].fragments[0].operands[0].guard, "orderValid");
        assert.strictEqual(result.sequenceDiagramCandidates[0].messageCount, 2);
    });

    it("state-transition-view produces normalized state machines", () => {
        const data = createMockData({
            elements: [
                {
                    name: "TimerStateMachine",
                    type: "state def",
                    id: "TimerStateMachine",
                    children: [
                        { name: "idle", type: "state", id: "idle", children: [], relationships: [] },
                        { name: "running", type: "state", id: "running", children: [], relationships: [] },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "idle", target: "running", name: "start" },
            ],
        });
        const result = prepareDataForView(data, "state-transition-view");
        assert.ok(Array.isArray(result.stateMachines), "state-transition-view should have stateMachines array");
        assert.strictEqual(result.stateMachines.length, 1, "should detect one state machine");
        assert.ok(Array.isArray(result.states), "state-transition-view should have states array");
        assert.ok(Array.isArray(result.transitions), "state-transition-view should have transitions array");
        assert.ok(
            result.stateMachines[0].states.length >= 2,
            "state machine should contain normalized states",
        );
        assert.ok(
            result.stateMachines[0].transitions.length >= 1,
            "state machine should contain transitions",
        );
        assert.ok(Array.isArray(result.stateMachineCandidates), "state-transition-view should expose selector candidates");
        assert.strictEqual(result.stateMachineCandidates[0].name, "TimerStateMachine");
    });

    it("state-transition-view classifies initial/final states and self loops", () => {
        const data = createMockData({
            elements: [
                {
                    name: "TimerStateMachine",
                    type: "state def",
                    id: "TimerStateMachine",
                    children: [
                        { name: "start", type: "initial state", id: "start", children: [], relationships: [] },
                        { name: "active", type: "state", id: "active", children: [], relationships: [] },
                        { name: "done", type: "final state", id: "done", children: [], relationships: [] },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "start", target: "active", name: "boot" },
                { type: "transition", source: "active", target: "active", name: "tick" },
                { type: "transition", source: "active", target: "done", name: "finish" },
            ],
        });
        const result = prepareDataForView(data, "state-transition-view");
        const machine = result.stateMachines[0];
        const start = machine.states.find((state: any) => state.id === "start");
        const done = machine.states.find((state: any) => state.id === "done");
        const selfLoop = machine.transitions.find((transition: any) => transition.name === "tick");

        assert.strictEqual(start.kind, "initial");
        assert.strictEqual(done.kind, "final");
        assert.strictEqual(selfLoop.selfLoop, true, "self-loop transition should be marked");
    });

    it("state-transition-view preserves multiple transitions between the same states", () => {
        const data = createMockData({
            elements: [
                {
                    name: "DoorStateMachine",
                    type: "state def",
                    id: "DoorStateMachine",
                    children: [
                        { name: "closed", type: "state", id: "closed", children: [], relationships: [] },
                        { name: "open", type: "state", id: "open", children: [], relationships: [] },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "closed", target: "open", name: "unlock" },
                { type: "transition", source: "closed", target: "open", name: "force_open" },
            ],
        });
        const result = prepareDataForView(data, "state-transition-view");
        assert.strictEqual(
            result.stateMachines[0].transitions.filter((transition: any) => transition.name !== "entry").length,
            2,
        );
    });

    it("state-transition-view prefers transition element names over synthetic relationship names", () => {
        const data = createMockData({
            elements: [
                {
                    name: "FlightModeStateMachine",
                    type: "state def",
                    id: "FlightModeStateMachine",
                    children: [
                        { name: "manual", type: "state", id: "manual", children: [], relationships: [] },
                        { name: "attitudeHold", type: "state", id: "attitudeHold", children: [], relationships: [] },
                        {
                            name: "to_attitude_from_manual",
                            type: "transition",
                            id: "transition-a",
                            source: "manual",
                            target: "attitudeHold",
                            children: [],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "manual", target: "attitudeHold", name: "transition_15" },
            ],
        });

        const result = prepareDataForView(data, "state-transition-view");
        const transition = result.stateMachines[0].transitions.find((candidate: any) => candidate.name !== "entry");

        assert.strictEqual(transition.name, "to_attitude_from_manual");
        assert.strictEqual(transition.label, "to_attitude_from_manual");
    });

    it("state-transition-view preserves parent-child links for composite states", () => {
        const data = createMockData({
            elements: [
                {
                    name: "OvenStateMachine",
                    type: "state def",
                    id: "OvenStateMachine",
                    children: [
                        {
                            name: "heating",
                            type: "state",
                            id: "heating",
                            children: [
                                { name: "preheat", type: "state", id: "preheat", children: [], relationships: [] },
                                { name: "cook", type: "state", id: "cook", children: [], relationships: [] },
                            ],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "preheat", target: "cook", name: "ready" },
            ],
        });
        const result = prepareDataForView(data, "state-transition-view");
        const machine = result.stateMachines[0];
        const composite = machine.states.find((state: any) => state.id === "heating");
        const child = machine.states.find((state: any) => state.id === "preheat");

        assert.strictEqual(composite.kind, "composite");
        assert.ok(Array.isArray(composite.childIds) && composite.childIds.includes("preheat"));
        assert.strictEqual(child.parentId, "heating");
    });

    it("state-transition-view synthesizes an entry node when no explicit initial state exists", () => {
        const data = createMockData({
            elements: [
                {
                    name: "FlightModeStateMachine",
                    type: "state def",
                    id: "FlightModeStateMachine",
                    children: [
                        { name: "manual", type: "state", id: "manual", children: [], relationships: [] },
                        { name: "gpsHold", type: "state", id: "gpsHold", children: [], relationships: [] },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "manual", target: "gpsHold", name: "to_gps" },
            ],
        });
        const result = prepareDataForView(data, "state-transition-view");
        const machine = result.stateMachines[0];
        const entry = machine.states.find((state: any) => state.kind === "initial");
        const entryTransition = machine.transitions.find((transition: any) => transition.source === entry.id);

        assert.ok(entry, "an entry state should be synthesized");
        assert.strictEqual(entry.name, "entry");
        assert.strictEqual(entryTransition.target, "manual");
    });

    it("state-transition-view selector disambiguates same-name machines by package path", () => {
        const data = createMockData({
            elements: [
                {
                    name: "PkgA",
                    type: "package",
                    id: "PkgA",
                    children: [
                        {
                            name: "FlightModeStateMachine",
                            type: "state def",
                            id: "PkgA::FlightModeStateMachine",
                            children: [{ name: "manual", type: "state", id: "PkgA::manual", children: [], relationships: [] }],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
                {
                    name: "PkgB",
                    type: "package",
                    id: "PkgB",
                    children: [
                        {
                            name: "FlightModeStateMachine",
                            type: "state def",
                            id: "PkgB::FlightModeStateMachine",
                            children: [{ name: "auto", type: "state", id: "PkgB::auto", children: [], relationships: [] }],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
            ],
            relationships: [],
        });

        const result = prepareDataForView(data, "state-transition-view");
        assert.strictEqual(result.stateMachineCandidates.length, 2);
        assert.deepStrictEqual(
            result.stateMachineCandidates.map((candidate: any) => candidate.packagePath).sort(),
            ["PkgA", "PkgB"]
        );
    });

    it("state-transition-view excludes typed exhibit-state usages from machine selector candidates", () => {
        const data = createMockData({
            elements: [
                {
                    name: "FlightController",
                    type: "part def",
                    id: "FlightController",
                    children: [
                        {
                            name: "flightMode",
                            type: "exhibit state",
                            id: "FlightController::flightMode",
                            attributes: { type: "FlightModeStateMachine" },
                            children: [],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
                {
                    name: "FlightModeStateMachine",
                    type: "state def",
                    id: "FlightModeStateMachine",
                    children: [
                        { name: "manual", type: "state", id: "manual", children: [], relationships: [] },
                        { name: "auto", type: "state", id: "auto", children: [], relationships: [] },
                    ],
                    relationships: [],
                },
            ],
            relationships: [
                { type: "transition", source: "manual", target: "auto", name: "engage_auto" },
            ],
        });

        const result = prepareDataForView(data, "state-transition-view");
        assert.deepStrictEqual(
            result.stateMachineCandidates.map((candidate: any) => candidate.name),
            ["FlightModeStateMachine"]
        );
    });

    it("state-transition-view keeps real machines that have state content even without transitions", () => {
        const data = createMockData({
            elements: [
                {
                    name: "FlightModeStateMachine",
                    type: "state def",
                    id: "FlightModeStateMachine",
                    children: [
                        { name: "manual", type: "state", id: "manual", children: [], relationships: [] },
                    ],
                    relationships: [],
                },
            ],
            relationships: [],
        });

        const result = prepareDataForView(data, "state-transition-view");

        assert.deepStrictEqual(
            result.stateMachineCandidates.map((candidate: any) => candidate.name),
            ["FlightModeStateMachine"]
        );
    });

    it("action-flow-view selector disambiguates same-name actions by package path", () => {
        const data = createMockData({
            elements: [
                {
                    name: "MissionA",
                    type: "package",
                    id: "MissionA",
                    children: [
                        {
                            name: "ExecutePatrol",
                            type: "action def",
                            id: "MissionA::ExecutePatrol",
                            children: [{ name: "stepA", type: "perform action", id: "MissionA::stepA", children: [], relationships: [] }],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
                {
                    name: "MissionB",
                    type: "package",
                    id: "MissionB",
                    children: [
                        {
                            name: "ExecutePatrol",
                            type: "action def",
                            id: "MissionB::ExecutePatrol",
                            children: [{ name: "stepB", type: "perform action", id: "MissionB::stepB", children: [], relationships: [] }],
                            relationships: [],
                        },
                    ],
                    relationships: [],
                },
            ],
            activityDiagrams: [],
            relationships: [],
        });

        const result = prepareDataForView(data, "action-flow-view");
        assert.strictEqual(result.activityDiagramCandidates.length, 2);
        assert.deepStrictEqual(
            result.activityDiagramCandidates.map((candidate: any) => candidate.packagePath).sort(),
            ["MissionA", "MissionB"]
        );
    });

    it("handles empty elements", () => {
        const data = createMockData({ elements: [], relationships: [] });
        const result = prepareDataForView(data, "general-view");
        assert.ok(result != null);
        assert.ok(Array.isArray(result.elements));
        assert.strictEqual(result.elements.length, 0);
    });

    it("handles graph input and produces elements for views", () => {
        const graphData = {
            graph: {
                nodes: [
                    { id: "pkg1", name: "pkg1", type: "package", range: { start: { line: 0, character: 0 }, end: { line: 1, character: 0 } }, attributes: {} },
                    { id: "pkg1::el1", name: "el1", type: "part def", parentId: "pkg1", range: { start: { line: 1, character: 0 }, end: { line: 2, character: 0 } }, attributes: {} },
                ],
                edges: [
                    { source: "pkg1", target: "pkg1::el1", type: "contains" },
                    { source: "pkg1::el1", target: "Other", type: "typing" },
                ],
            },
        };
        const result = prepareDataForView(graphData, "interconnection-view");
        assert.deepStrictEqual(result.parts, []);
        assert.deepStrictEqual(result.connectors, []);
    });

    it("graphToElementTree builds tree from contains edges", () => {
        const graph = {
            nodes: [
                { id: "root", name: "root", type: "package", range: { start: { line: 0, character: 0 }, end: { line: 1, character: 0 } }, attributes: {} },
                { id: "root::child", name: "child", type: "part", parentId: "root", range: { start: { line: 1, character: 0 }, end: { line: 2, character: 0 } }, attributes: {} },
            ],
            edges: [{ source: "root", target: "root::child", type: "contains" }],
        };
        const roots = graphToElementTree(graph);
        assert.strictEqual(roots.length, 1);
        assert.strictEqual(roots[0].name, "root");
        assert.strictEqual(roots[0].children?.length, 1);
        assert.strictEqual(roots[0].children[0].name, "child");
    });

    it("graphToElementTree omits builder diagnostic nodes", () => {
        const graph = {
            nodes: [
                { id: "Pkg", name: "Pkg", type: "package", range: { start: { line: 0, character: 0 }, end: { line: 1, character: 0 } }, attributes: {} },
                { id: "Pkg::unresolved_allocate_source", name: "unresolved_allocate_source", type: "diagnostic", parentId: "Pkg", range: { start: { line: 1, character: 0 }, end: { line: 2, character: 0 } }, attributes: {} },
            ],
            edges: [{ source: "Pkg", target: "Pkg::unresolved_allocate_source", type: "contains" }],
        };
        const roots = graphToElementTree(graph);
        assert.strictEqual(roots.length, 1);
        assert.strictEqual(roots[0].name, "Pkg");
        assert.strictEqual(roots[0].children?.length ?? 0, 0);
    });

});
