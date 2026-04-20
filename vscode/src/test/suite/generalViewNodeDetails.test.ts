import * as assert from "assert";
import { collectCompartmentsFromElement, computeNodeHeightFromCompartments } from "../../visualization/webview/renderers/sysmlNodeBuilder";

describe("General View node details", () => {
    it("reads structured detail groups from map-like attribute bags", () => {
        const attributeMap = new Map<string, unknown>([
            [
                "generalViewDirectAttributes",
                [{ name: "mass", typeName: "Kilogram", displayText: "mass : Kilogram" }]
            ],
            [
                "generalViewDirectParts",
                [{ name: "wheels", typeName: "WheelSet", displayText: "wheels : WheelSet" }]
            ],
            [
                "generalViewDirectPorts",
                [{ name: "powerIn", typeName: "PowerPort", displayText: "powerIn : PowerPort" }]
            ],
            [
                "generalViewInheritedAttributes",
                [{ name: "range", typeName: "Kilometer", displayText: "range : Kilometer", declaredIn: "Vehicle" }]
            ]
        ]);

        const compartments = collectCompartmentsFromElement({
            name: "Car",
            type: "part def",
            attributes: {
                get: (key: string) => attributeMap.get(key)
            },
            children: []
        });

        assert.deepStrictEqual(
            compartments.attributes.map((item) => item.displayText),
            ["mass : Kilogram"]
        );
        assert.deepStrictEqual(
            compartments.parts.map((item) => item.displayText),
            ["wheels : WheelSet"]
        );
        assert.deepStrictEqual(
            compartments.ports.map((item) => item.displayText),
            ["powerIn : PowerPort"]
        );
        assert.deepStrictEqual(
            compartments.collapsibleSections?.map((section) => section.title),
            ["Inherited Attributes"]
        );
    });

    it("prefers structured backend detail groups and keeps inherited sections collapsed", () => {
        const compartments = collectCompartmentsFromElement({
            name: "Car",
            type: "part def",
            attributes: {
                generalViewDirectAttributes: [
                    { name: "mass", typeName: "Kilogram", valueText: "1300", displayText: "mass : Kilogram = 1300" }
                ],
                generalViewDirectParts: [
                    { name: "wheels", typeName: "WheelSet", displayText: "wheels : WheelSet" }
                ],
                generalViewDirectPorts: [
                    { name: "powerIn", typeName: "PowerPort", displayText: "powerIn : PowerPort" }
                ],
                generalViewInheritedAttributes: [
                    { name: "range", typeName: "Kilometer", displayText: "range : Kilometer", declaredIn: "Vehicle" }
                ],
                generalViewInheritedParts: [
                    { name: "engine", typeName: "Engine", displayText: "engine : Engine", declaredIn: "Vehicle" }
                ]
            },
            children: [
                { name: "mass", type: "attribute", attributes: { dataType: "ScalarValues::Kilogram" } },
                { name: "powerIn", type: "port", attributes: { portType: "PowerPort" } }
            ]
        });

        assert.deepStrictEqual(
            compartments.attributes.map((item) => item.displayText),
            ["mass : Kilogram = 1300"],
            "structured direct attributes should be used without legacy duplication"
        );
        assert.deepStrictEqual(
            compartments.parts.map((item) => item.displayText),
            ["wheels : WheelSet"]
        );
        assert.deepStrictEqual(
            compartments.ports.map((item) => item.displayText),
            ["powerIn : PowerPort"]
        );
        assert.deepStrictEqual(
            compartments.collapsibleSections?.map((section) => ({
                title: section.title,
                collapsed: section.collapsed,
                items: section.items.map((item) => item.displayText)
            })),
            [
                { title: "Inherited Attributes", collapsed: true, items: ["range : Kilometer"] },
                { title: "Inherited Parts", collapsed: true, items: ["engine : Engine"] }
            ]
        );
    });

    it("ignores child part fallback and keeps structured detail rows only", () => {
        const compartments = collectCompartmentsFromElement({
            name: "S-IC",
            type: "part def",
            attributes: {
                generalViewDirectParts: [
                    { name: "engine1", typeName: "Engine", displayText: "engine1 : Engine" },
                    { name: "engine2", typeName: "Engine", displayText: "engine2 : Engine" }
                ]
            },
            children: [
                { name: "engine1", type: "part", attributes: { type: "Engine" } },
                { name: "engine2", type: "part", attributes: { type: "Engine" } }
            ]
        });

        assert.deepStrictEqual(
            compartments.parts.map((item) => item.displayText),
            ["engine1 : Engine", "engine2 : Engine"],
            "child part rows should be ignored when structured direct part rows exist"
        );
    });

    it("does not synthesize details from children when structured groups are absent", () => {
        const compartments = collectCompartmentsFromElement({
            name: "LegacyNode",
            type: "part def",
            attributes: {},
            children: [
                { name: "mass", type: "attribute", attributes: { dataType: "ScalarValues::Kilogram" } },
                { name: "engine1", type: "part", attributes: { type: "Engine" } },
                { name: "powerIn", type: "port", attributes: { portType: "PowerPort" } }
            ]
        });

        assert.deepStrictEqual(compartments.attributes, []);
        assert.deepStrictEqual(compartments.parts, []);
        assert.deepStrictEqual(compartments.ports, []);
    });

    it("normalizes doubled unit brackets in attribute display text", () => {
        const compartments = collectCompartmentsFromElement({
            name: "Tank",
            type: "part def",
            attributes: {
                generalViewDirectAttributes: [
                    { name: "mass", typeName: "[[kg]]", displayText: "mass : [[kg]] = 28500 [[kg]]" }
                ]
            },
            children: []
        });

        assert.deepStrictEqual(
            compartments.attributes.map((item) => item.displayText),
            ["mass : [kg] = 28500 [kg]"],
            "unit tokens should render with single bracket pairs"
        );
    });

    it("grows node height when inherited sections are expanded or show all rows", () => {
        const baseCompartments = collectCompartmentsFromElement({
            name: "Vehicle",
            type: "part def",
            attributes: {
                generalViewDirectAttributes: [
                    { name: "mass", typeName: "Kilogram", displayText: "mass : Kilogram" }
                ],
                generalViewInheritedAttributes: Array.from({ length: 10 }, (_, index) => ({
                    name: `attr${index}`,
                    displayText: `attr${index} : Scalar`,
                    declaredIn: "BaseVehicle"
                }))
            }
        });
        const config = {
            showHeader: true,
            showAttributes: true,
            showParts: true,
            showPorts: true,
            showOther: true,
            maxLinesPerCompartment: 4
        };

        const collapsedHeight = computeNodeHeightFromCompartments(baseCompartments, config, 200);
        const expandedHeight = computeNodeHeightFromCompartments(
            {
                ...baseCompartments,
                collapsibleSections: (baseCompartments.collapsibleSections || []).map((section) => ({
                    ...section,
                    collapsed: false,
                    showAll: true
                }))
            },
            config,
            200
        );

        assert.ok(expandedHeight > collapsedHeight, "expanded inherited sections should increase node height");
    });
});
