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
            activityDiagrams: [],
        });

        const result = prepareDataForView(data, "action-flow-view");
        assert.deepStrictEqual(result.diagrams, []);
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
                    id: "Mission::FlightController::performer",
                    name: "FlightController",
                    packagePath: "Mission",
                    label: "FlightController - Mission",
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
                    id: "Mission::LaunchSystem::performer",
                    name: "LaunchSystem",
                    packagePath: "Mission",
                    label: "LaunchSystem - Mission",
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
                    id: "Mission::ExecuteMission::actionDef",
                    name: "ExecuteMission",
                    packagePath: "Mission",
                    label: "ExecuteMission - Mission",
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
                },
                {
                    id: "Mission::FlightController::performer",
                    name: "FlightController",
                    packagePath: "Mission",
                    label: "FlightController - Mission",
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
            stateMachines: [
                {
                    id: "TimerStateMachine",
                    name: "TimerStateMachine",
                    label: "TimerStateMachine",
                    packagePath: "",
                    states: [
                        { id: "idle", name: "idle", kind: "state" },
                        { id: "running", name: "running", kind: "state" },
                    ],
                    transitions: [
                        { id: "t1", source: "idle", target: "running", name: "start", selfLoop: false },
                    ],
                },
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
            stateMachines: [
                {
                    id: "TimerStateMachine",
                    name: "TimerStateMachine",
                    label: "TimerStateMachine",
                    packagePath: "",
                    states: [
                        { id: "start", name: "start", kind: "initial" },
                        { id: "active", name: "active", kind: "state" },
                        { id: "done", name: "done", kind: "final" },
                    ],
                    transitions: [
                        { id: "t1", source: "start", target: "active", name: "boot", selfLoop: false },
                        { id: "t2", source: "active", target: "active", name: "tick", selfLoop: true },
                        { id: "t3", source: "active", target: "done", name: "finish", selfLoop: false },
                    ],
                },
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
            stateMachines: [
                {
                    id: "DoorStateMachine",
                    name: "DoorStateMachine",
                    label: "DoorStateMachine",
                    packagePath: "",
                    states: [
                        { id: "closed", name: "closed", kind: "state" },
                        { id: "open", name: "open", kind: "state" },
                    ],
                    transitions: [
                        { id: "t1", source: "closed", target: "open", name: "unlock", selfLoop: false },
                        { id: "t2", source: "closed", target: "open", name: "force_open", selfLoop: false },
                    ],
                },
            ],
        });
        const result = prepareDataForView(data, "state-transition-view");
        assert.strictEqual(result.stateMachines[0].transitions.length, 2);
    });

    it("state-transition-view prefers transition element names over synthetic relationship names", () => {
        const data = createMockData({
            stateMachines: [
                {
                    id: "FlightModeStateMachine",
                    name: "FlightModeStateMachine",
                    label: "FlightModeStateMachine",
                    packagePath: "",
                    states: [
                        { id: "manual", name: "manual", kind: "state" },
                        { id: "attitudeHold", name: "attitudeHold", kind: "state" },
                    ],
                    transitions: [
                        {
                            id: "transition-a",
                            source: "manual",
                            target: "attitudeHold",
                            name: "to_attitude_from_manual",
                            label: "to_attitude_from_manual",
                            selfLoop: false,
                        },
                    ],
                },
            ],
        });

        const result = prepareDataForView(data, "state-transition-view");
        const transition = result.stateMachines[0].transitions[0];

        assert.strictEqual(transition.name, "to_attitude_from_manual");
        assert.strictEqual(transition.label, "to_attitude_from_manual");
    });

    it("state-transition-view preserves parent-child links for composite states", () => {
        const data = createMockData({
            stateMachines: [
                {
                    id: "OvenStateMachine",
                    name: "OvenStateMachine",
                    label: "OvenStateMachine",
                    packagePath: "",
                    states: [
                        { id: "heating", name: "heating", kind: "composite", childIds: ["preheat"] },
                        { id: "preheat", name: "preheat", kind: "state", parentId: "heating" },
                        { id: "cook", name: "cook", kind: "state", parentId: "heating" },
                    ],
                    transitions: [
                        { id: "t1", source: "preheat", target: "cook", name: "ready", selfLoop: false },
                    ],
                },
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

    it("state-transition-view selector disambiguates same-name machines by package path", () => {
        const data = createMockData({
            stateMachines: [
                {
                    id: "PkgA::FlightModeStateMachine",
                    name: "FlightModeStateMachine",
                    label: "FlightModeStateMachine - PkgA",
                    packagePath: "PkgA",
                    states: [{ id: "PkgA::manual", name: "manual", kind: "state" }],
                    transitions: [],
                },
                {
                    id: "PkgB::FlightModeStateMachine",
                    name: "FlightModeStateMachine",
                    label: "FlightModeStateMachine - PkgB",
                    packagePath: "PkgB",
                    states: [{ id: "PkgB::auto", name: "auto", kind: "state" }],
                    transitions: [],
                },
            ],
        });

        const result = prepareDataForView(data, "state-transition-view");
        assert.strictEqual(result.stateMachineCandidates.length, 2);
        assert.deepStrictEqual(
            result.stateMachineCandidates.map((candidate: any) => candidate.packagePath).sort(),
            ["PkgA", "PkgB"]
        );
    });

    it("state-transition-view keeps real machines that have state content even without transitions", () => {
        const data = createMockData({
            stateMachines: [
                {
                    id: "FlightModeStateMachine",
                    name: "FlightModeStateMachine",
                    label: "FlightModeStateMachine",
                    packagePath: "",
                    states: [{ id: "manual", name: "manual", kind: "state" }],
                    transitions: [],
                },
            ],
        });

        const result = prepareDataForView(data, "state-transition-view");

        assert.deepStrictEqual(
            result.stateMachineCandidates.map((candidate: any) => candidate.name),
            ["FlightModeStateMachine"]
        );
    });

    it("action-flow-view selector disambiguates same-name actions by package path", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    id: "MissionA::ExecutePatrol",
                    name: "ExecutePatrol",
                    packagePath: "MissionA",
                    label: "ExecutePatrol - MissionA",
                    sourceKind: "actionDef",
                    actions: [{ name: "stepA", type: "action", kind: "perform", id: "stepA" }],
                    flows: [{ from: "stepA", to: "stepA" }],
                    decisions: [],
                    states: [],
                },
                {
                    id: "MissionB::ExecutePatrol",
                    name: "ExecutePatrol",
                    packagePath: "MissionB",
                    label: "ExecutePatrol - MissionB",
                    sourceKind: "actionDef",
                    actions: [{ name: "stepB", type: "action", kind: "perform", id: "stepB" }],
                    flows: [{ from: "stepB", to: "stepB" }],
                    decisions: [],
                    states: [],
                },
            ],
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
