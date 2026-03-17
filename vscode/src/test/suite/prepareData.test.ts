import * as assert from "assert";
import { prepareDataForView, graphToElementTree } from "../../visualization/prepareData";

/**
 * Minimal mock data in the format produced by modelFetcher / fetchModelData.
 * With graph: nodes + edges (preferred). Without: elements + relationships (legacy).
 */
const createMockData = (overrides: Partial<{
    elements: unknown[];
    relationships: unknown[];
    sequenceDiagrams: unknown[];
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
    sequenceDiagrams: [
        {
            name: "Seq1",
            participants: [{ name: "Actor1", type: "actor" }],
            messages: [{ name: "msg1", from: "Actor1", to: "System", payload: "data", occurrence: 1 }]
        }
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

    it.skip("action-flow-view produces diagrams (disabled for release)", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "action-flow-view");
        assert.ok(Array.isArray(result.diagrams), "action-flow-view should have diagrams array");
    });

    it.skip("state-transition-view produces states and transitions (disabled for release)", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "state-transition-view");
        assert.ok(Array.isArray(result.states), "state-transition-view should have states array");
        assert.ok(Array.isArray(result.transitions), "state-transition-view should have transitions array");
    });

    it.skip("sequence-view produces sequenceDiagrams (disabled for release)", () => {
        const data = createMockData();
        const result = prepareDataForView(data, "sequence-view");
        assert.ok(Array.isArray(result.sequenceDiagrams), "sequence-view should have sequenceDiagrams array");
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

    describe("interconnection-view with backend IBD (no fallback)", () => {
        const mockIbdFromBackend = {
            parts: [
                { id: "SurveillanceDrone::SurveillanceQuadrotorDrone", name: "SurveillanceQuadrotorDrone", qualifiedName: "SurveillanceDrone.SurveillanceQuadrotorDrone", containerId: null, type: "part def", attributes: {} },
                { id: "SurveillanceDrone::SurveillanceQuadrotorDrone::propulsion", name: "propulsion", qualifiedName: "SurveillanceDrone.SurveillanceQuadrotorDrone.propulsion", containerId: "SurveillanceDrone.SurveillanceQuadrotorDrone", type: "part", attributes: {} },
                { id: "SurveillanceDrone::SurveillanceQuadrotorDrone::flightControl", name: "flightControl", qualifiedName: "SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl", containerId: "SurveillanceDrone.SurveillanceQuadrotorDrone", type: "part", attributes: {} },
                { id: "SurveillanceDrone::Propulsion", name: "Propulsion", qualifiedName: "SurveillanceDrone.Propulsion", containerId: null, type: "part def", attributes: {} },
                { id: "SurveillanceDrone::Propulsion::propulsionUnit1", name: "propulsionUnit1", qualifiedName: "SurveillanceDrone.Propulsion.propulsionUnit1", containerId: "SurveillanceDrone.Propulsion", type: "part", attributes: {} },
            ],
            ports: [] as { id: string; name: string; parentId: string }[],
            connectors: [] as { sourceId: string; targetId: string }[],
            rootCandidates: ["SurveillanceQuadrotorDrone", "Propulsion"],
            defaultRoot: "SurveillanceQuadrotorDrone",
        };

        it("uses defaultRoot when no selectedIbdRoot (SurveillanceQuadrotorDrone)", () => {
            const data = { graph: { nodes: [], edges: [] }, ibd: mockIbdFromBackend };
            const result = prepareDataForView(data, "interconnection-view");
            assert.strictEqual(result.selectedIbdRoot, "SurveillanceQuadrotorDrone", "selectedIbdRoot must be backend defaultRoot");
            assert.ok(Array.isArray(result.parts) && result.parts.length >= 2, "parts must include root and children");
            const rootPart = result.parts.find((p: { name: string }) => p.name === "SurveillanceQuadrotorDrone");
            assert.ok(rootPart, "root part must be SurveillanceQuadrotorDrone");
            const prefix = "SurveillanceDrone.SurveillanceQuadrotorDrone";
            const allUnderSqd = result.parts.every((p: { qualifiedName?: string }) => {
                const q = p.qualifiedName || "";
                return q === prefix || q.startsWith(prefix + ".");
            });
            assert.ok(allUnderSqd, "focused parts must be under SurveillanceQuadrotorDrone");
        });

        it("uses user selectedIbdRoot (Propulsion) when provided", () => {
            const data = { graph: { nodes: [], edges: [] }, ibd: mockIbdFromBackend, selectedIbdRoot: "Propulsion" };
            const result = prepareDataForView(data, "interconnection-view");
            assert.strictEqual(result.selectedIbdRoot, "Propulsion");
            const rootPart = result.parts.find((p: { name: string }) => p.name === "Propulsion");
            assert.ok(rootPart, "root part must be Propulsion when selected");
        });

        it("keeps only connectors whose endpoint paths stay inside the selected root", () => {
            const data = {
                graph: { nodes: [], edges: [] },
                ibd: {
                    ...mockIbdFromBackend,
                    ports: [
                        { id: "p1", name: "motorOut", parentId: "SurveillanceDrone.Propulsion.propulsionUnit1" },
                        { id: "p2", name: "flightIn", parentId: "SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl" },
                    ],
                    connectors: [
                        {
                            sourceId: "SurveillanceDrone.Propulsion.propulsionUnit1.motorOut",
                            targetId: "SurveillanceDrone.Propulsion.propulsionUnit1.motorOut",
                            type: "connection",
                            name: "internalLoop",
                        },
                        {
                            sourceId: "SurveillanceDrone.Propulsion.propulsionUnit1.motorOut",
                            targetId: "SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl.flightIn",
                            type: "connection",
                            name: "crossRootLink",
                        },
                    ],
                },
                selectedIbdRoot: "Propulsion",
            };
            const result = prepareDataForView(data, "interconnection-view");
            assert.strictEqual(result.selectedIbdRoot, "Propulsion");
            assert.strictEqual(result.connectors.length, 1, "only connectors fully inside the focused root should remain");
            assert.strictEqual(result.connectors[0].name, "internalLoop");
        });

        it("returns empty IBD when no backend ibd (no fallback)", () => {
            const data = { graph: { nodes: [{ id: "a", name: "A", type: "part def" }], edges: [] } };
            const result = prepareDataForView(data, "interconnection-view");
            assert.deepStrictEqual(result.parts, []);
            assert.deepStrictEqual(result.ports, []);
            assert.deepStrictEqual(result.connectors, []);
            assert.strictEqual(result.selectedIbdRoot, null);
        });

        it("prefers the richest root when no explicit root or default root is provided", () => {
            const data = {
                graph: { nodes: [], edges: [] },
                ibd: {
                    parts: [
                        { id: "Drone", name: "Drone", qualifiedName: "Demo.Drone", containerId: null, type: "part def", attributes: {} },
                        { id: "Drone::left", name: "left", qualifiedName: "Demo.Drone.left", containerId: "Demo.Drone", type: "part", attributes: {} },
                        { id: "Drone::right", name: "right", qualifiedName: "Demo.Drone.right", containerId: "Demo.Drone", type: "part", attributes: {} },
                        { id: "Power", name: "Power", qualifiedName: "Demo.Power", containerId: null, type: "part def", attributes: {} },
                        { id: "Power::unit", name: "unit", qualifiedName: "Demo.Power.unit", containerId: "Demo.Power", type: "part", attributes: {} },
                    ],
                    ports: [
                        { id: "p1", name: "leftOut", parentId: "Demo.Drone.left" },
                        { id: "p2", name: "rightIn", parentId: "Demo.Drone.right" },
                        { id: "p3", name: "powerOut", parentId: "Demo.Power.unit" },
                    ],
                    connectors: [
                        {
                            sourceId: "Demo.Drone.left.leftOut",
                            targetId: "Demo.Drone.right.rightIn",
                            type: "connection",
                            name: "internalLink",
                        },
                    ],
                    rootCandidates: ["Power", "Drone"],
                },
            };
            const result = prepareDataForView(data, "interconnection-view");
            assert.strictEqual(result.selectedIbdRoot, "Drone", "root with richer internal structure should be preferred");
            assert.ok(Array.isArray(result.ibdRootSummaries), "root summaries should be returned");
            const droneSummary = result.ibdRootSummaries.find((summary: { name: string }) => summary.name === "Drone");
            assert.ok(droneSummary, "Drone summary should exist");
            assert.strictEqual(droneSummary.connectorCount, 1);
            assert.strictEqual(droneSummary.partCount, 3);
        });
    });
});
