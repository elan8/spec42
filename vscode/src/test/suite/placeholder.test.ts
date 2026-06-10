import * as assert from "assert";
import {
    buildVisualizationEmptyStateHtml,
    resolveEmptyStateTitle,
} from "../../visualization/emptyStateContent";

describe("visualization placeholder", () => {
    it("uses a neutral title when no model-defined views exist", () => {
        assert.strictEqual(
            resolveEmptyStateTitle({ viewCandidates: [], rendererViewLabel: "General View" }),
            "No views defined",
        );

        const html = buildVisualizationEmptyStateHtml(
            "Define a SysML view with expose (and optional filter) to use the visualizer.",
            { viewLabel: "No views defined" },
        );

        assert.ok(html.includes("No views defined"));
        assert.ok(!html.includes("General View"));
        assert.ok(html.includes("Define a SysML view with expose"));
        assert.ok(html.includes("recommended example in the Spec42 sidebar"));
    });

    it("uses the renderer label when views exist but none match the active renderer", () => {
        assert.strictEqual(
            resolveEmptyStateTitle({
                viewCandidates: [{ id: "v1", name: "Structure" }],
                rendererViewLabel: "General View",
            }),
            "General View",
        );
    });

    it("prefers the selected view name for unsupported view types", () => {
        assert.strictEqual(
            resolveEmptyStateTitle({
                viewCandidates: [{ id: "v1", name: "Custom Matrix" }],
                selectedViewName: "Custom Matrix",
                rendererViewLabel: "General View",
            }),
            "Custom Matrix",
        );
    });

    it("shows alternate recovery guidance when the model already has elements", () => {
        const html = buildVisualizationEmptyStateHtml(
            "Define a SysML view typed by GeneralView to display something in this visualizer panel.",
            {
                viewLabel: "General View",
                data: {
                    graph: { nodes: [{ id: "a" }, { id: "b" }] },
                },
            },
        );

        assert.ok(html.includes("Try another visualizer view"));
        assert.ok(html.includes("2 element(s) in model"));
    });

    it("escapes HTML in messages", () => {
        const html = buildVisualizationEmptyStateHtml('<script>alert("x")</script>', {
            viewLabel: "General View",
        });

        assert.ok(!html.includes("<script>"));
        assert.ok(html.includes("&lt;script&gt;"));
    });
});
