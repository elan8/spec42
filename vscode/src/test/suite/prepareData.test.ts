import * as assert from "assert";
import { prepareDataForView, graphToElementTree } from "../../visualization/prepareData";

/**
 * Minimal mock data in the format produced by modelFetcher / fetchModelData.
 * With graph: nodes + edges (preferred). Without: elements + relationships (legacy).
 */
const createMockData = (overrides: Partial<{
    elements: unknown[];
    relationships: unknown[];
    activityDiagrams: unknown[];
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
    ...overrides
});

describe("prepareDataForView", () => {
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
        assert.strictEqual(result, data, "Unknown view should return data unchanged");
    });

    it("returns null/undefined for null input", () => {
        assert.strictEqual(prepareDataForView(null, "general-view"), null);
    });

    it("interconnection-view produces parts and connectors", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "interconnection-view");
        assert.ok(Array.isArray(result.parts), "interconnection-view should have parts array");
        assert.ok(Array.isArray(result.connectors), "interconnection-view should have connectors array");
        assert.ok(Array.isArray(result.ports), "interconnection-view should have ports array");
    });

    it("interconnection-view preserves connectors that only provide source/target ids", () => {
        const result = prepareDataForView(
            {
                ibd: {
                    parts: [
                        { id: "P::capability", name: "capability", qualifiedName: "P.capability", type: "part" },
                        { id: "P::goal", name: "goal", qualifiedName: "P.goal", type: "part" },
                    ],
                    connectors: [
                        {
                            source: "P::capability",
                            target: "P::goal",
                            type: "connection",
                        },
                    ],
                },
            },
            "interconnection-view"
        );
        assert.strictEqual(result.connectors.length, 1);
        assert.strictEqual(result.connectors[0].source, "P::capability");
        assert.strictEqual(result.connectors[0].target, "P::goal");
    });

    it.skip("action-flow-view produces diagrams (disabled for release)", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "action-flow-view");
        assert.ok(Array.isArray(result.diagrams), "action-flow-view should have diagrams array");
    });

    it("action-flow-view keeps interface metadata out of behavioral nodes", () => {
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
        const diagram = result.diagrams[0];
        const nodeNames = diagram.nodes.map((node: any) => node.name);

        assert.deepStrictEqual(nodeNames, ["renderDisplay"]);
        assert.deepStrictEqual(diagram.interface.inputs, ["currentTime"]);
        assert.deepStrictEqual(diagram.interface.outputs, ["displayText"]);
        assert.deepStrictEqual(diagram.flows, []);
        assert.strictEqual(diagram.hasBehavioralFlow, false);
    });

    it("action-flow-view preserves explicit behavioral flows without synthesizing control edges", () => {
        const data = createMockData({
            activityDiagrams: [
                {
                    name: "ExecuteMission",
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
        assert.ok(Array.isArray(result.parts));
        assert.ok(Array.isArray(result.connectors));
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

    describe("interconnection-view with backend IBD payload", () => {
        const mockIbdFromServer = {
            parts: [
                { id: "SurveillanceDrone::SurveillanceQuadrotorDrone", name: "SurveillanceQuadrotorDrone", qualifiedName: "SurveillanceDrone.SurveillanceQuadrotorDrone", containerId: null, type: "part def", attributes: {} },
                { id: "SurveillanceDrone::SurveillanceQuadrotorDrone::propulsion", name: "propulsion", qualifiedName: "SurveillanceDrone.SurveillanceQuadrotorDrone.propulsion", containerId: "SurveillanceDrone.SurveillanceQuadrotorDrone", type: "part", attributes: {} },
                { id: "SurveillanceDrone::SurveillanceQuadrotorDrone::flightControl", name: "flightControl", qualifiedName: "SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl", containerId: "SurveillanceDrone.SurveillanceQuadrotorDrone", type: "part", attributes: {} },
                { id: "SurveillanceDrone::Propulsion", name: "Propulsion", qualifiedName: "SurveillanceDrone.Propulsion", containerId: null, type: "part def", attributes: {} },
                { id: "SurveillanceDrone::Propulsion::propulsionUnit1", name: "propulsionUnit1", qualifiedName: "SurveillanceDrone.Propulsion.propulsionUnit1", containerId: "SurveillanceDrone.Propulsion", type: "part", attributes: {} },
            ],
            ports: [] as { id: string; name: string; parentId: string }[],
            connectors: [] as { sourceId: string; targetId: string }[],
        };

        it("passes through backend parts without frontend root selection", () => {
            const data = { graph: { nodes: [], edges: [] }, ibd: mockIbdFromServer };
            const result = prepareDataForView(data, "interconnection-view");
            assert.strictEqual(result.parts.length, 5);
            assert.strictEqual(result.parts[0].name, "SurveillanceQuadrotorDrone");
        });

        it("passes through backend connectors without frontend pruning", () => {
            const data = {
                graph: { nodes: [], edges: [] },
                ibd: {
                    ...mockIbdFromServer,
                    connectors: [
                        {
                            sourceId: "SurveillanceDrone.Propulsion.propulsionUnit1.motorOut",
                            targetId: "SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl.flightIn",
                            type: "connection",
                            name: "crossPackageLink",
                        },
                    ],
                },
            };
            const result = prepareDataForView(data, "interconnection-view");
            assert.strictEqual(result.connectors.length, 1);
            assert.strictEqual(result.connectors[0].name, "crossPackageLink");
        });

        it("returns empty IBD when no server ibd (no fallback)", () => {
            const data = { graph: { nodes: [{ id: "a", name: "A", type: "part def" }], edges: [] } };
            const result = prepareDataForView(data, "interconnection-view");
            assert.deepStrictEqual(result.parts, []);
            assert.deepStrictEqual(result.ports, []);
            assert.deepStrictEqual(result.connectors, []);
        });
    });
});
