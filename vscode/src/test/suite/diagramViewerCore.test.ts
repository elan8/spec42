import { DIAGRAM_PRODUCT_SCHEMA_VERSION } from "../../generated/diagram-product/schemaVersion";
import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  allDiagramViewOptions,
  authoredDiagramViewOptions,
  diagramRenderIsStale,
  diagramViewsForDocument,
  diagramViewKindForHandle,
  DIAGRAM_VIEWS,
  isPathInsideWorkspace,
  MAX_DIAGRAM_PRODUCT_BYTES,
  oversizedProductMessage,
  parseDiagramProduct,
  parseDiagramViewCatalog,
  parseLspDiagramResult,
  parseStateTransitionViewCatalog,
  parseSourceNavigation,
  reconcileDelayMs,
  resolveDiagramSelection,
  visibleSourceColumn,
} from "../../diagram/diagramViewerCore";
import { isEmptyIncompleteDiagramProduct, sameProjectedNodes } from "../../diagram/diagramProductState";
import { isPresentedFrame, parseRenderMeasurement, RenderMeasurementTracker, type RenderMeasurement } from "../../diagram/diagramPerformance";

describe("diagram performance correlation", () => {
  const frame = (identity: { renderId: number; modelDigest: string; viewHandle: string }): RenderMeasurement => ({
    ...identity, type: "renderComplete", outcome: "rendered",
    webviewTotalMs: 15, webviewPrepareMs: 2, webviewRenderMs: 8, frameWaitMs: 4,
    framesObserved: 2, visible: true, viewportReused: true, disclosureReused: false,
  });

  it("accepts one acknowledgement for the exact current render, product and view", () => {
    const tracker = new RenderMeasurementTracker();
    const old = tracker.begin("old", "general:one", 10);
    const current = tracker.begin("current", "general:one", 30);
    assert.equal(tracker.complete(frame(old), 60), undefined);
    assert.equal(tracker.complete({ ...frame(current), modelDigest: "other" }, 60), undefined);
    assert.equal(tracker.complete({ ...frame(current), viewHandle: "other" }, 60), undefined);
    assert.equal(tracker.complete(frame(current), 20), undefined);
    const accepted = tracker.complete(frame(current), 60);
    assert.ok(accepted);
    assert.equal(accepted.postToAckMs, 30);
    assert.ok(isPresentedFrame(accepted));
    assert.equal(tracker.complete(frame(current), 70), undefined);
  });

  it("drops pending render identity on publication replacement, hide or disposal", () => {
    const tracker = new RenderMeasurementTracker();
    const identity = tracker.begin("current", "general:one", 10);
    tracker.invalidate();
    assert.equal(tracker.complete(frame(identity), 60), undefined);
  });

  it("keeps hidden, incomplete and unavailable renders explicit instead of counting a frame", () => {
    const value = frame({ renderId: 1, modelDigest: "current", viewHandle: "general:one" });
    for (const outcome of ["empty", "failed", "unavailable"] as const) {
      assert.equal(isPresentedFrame({ ...value, outcome }), false);
    }
    assert.equal(isPresentedFrame({ ...value, visible: false }), false);
    assert.equal(isPresentedFrame({ ...value, framesObserved: 1 }), false);
  });

  it("rejects malformed durations and strips fields outside the measurement contract", () => {
    const value = frame({ renderId: 1, modelDigest: "current", viewHandle: "general:one" });
    for (const extra of [{ webviewRenderMs: NaN }, { webviewRenderMs: -1 }, { framesObserved: 3 },
      { renderId: 1.5 }, { visible: "yes" }, { outcome: "success" }, { webviewTotalMs: 1 }]) {
      assert.equal(parseRenderMeasurement({ ...value, ...extra }), undefined);
    }
    assert.equal(parseRenderMeasurement(null), undefined);
    const accepted = parseRenderMeasurement({ ...value, event: "backend:publication", extensionHostMs: -1 });
    assert.ok(accepted);
    assert.equal("event" in accepted, false);
    assert.equal("extensionHostMs" in accepted, false);
  });
});

describe("diagram viewer core", () => {
  it("reuses disclosure only while a product projects the same nodes", () => {
    const node = (name: string) => ({ reference: 0, name });
    const product = (digest: string, names: string[]) => ({
      modelDigest: digest, references: [{ kind: "qualified-name" }], projection: { nodes: names.map(node) },
    });
    const before = product("blake3:a", ["root", "child"]);
    assert.equal(sameProjectedNodes(before, product("blake3:a", ["root", "child"])), true);
    // Loading a browser branch keeps the digest but adds nodes and shifts their indexes.
    assert.equal(sameProjectedNodes(before, product("blake3:a", ["root", "loaded", "child"])), false);
    assert.equal(sameProjectedNodes(before, product("blake3:b", ["root", "child"])), false);
    assert.equal(sameProjectedNodes(undefined, before), false);
  });

  it("reuses an already-visible source column and otherwise requests a new one", () => {
    const editors = [
      { uri: "file:///workspace/other.sysml", viewColumn: 1 },
      { uri: "file:///workspace/model.sysml", viewColumn: 3 },
    ];
    assert.equal(visibleSourceColumn("file:///workspace/model.sysml", editors), 3);
    assert.equal(visibleSourceColumn("file:///workspace/missing.sysml", editors), undefined);
  });

  it("declares all renderer views", () => {
    assert.deepEqual(DIAGRAM_VIEWS.map((view) => view.id), [
      "general-view", "interconnection-view", "action-flow-view", "state-transition-view",
      "sequence-view", "browser-view", "grid-view", "geometry-view",
    ]);
  });

  it("validates the native diagram result", () => {
    const value = { modelDigest: "blake3:model", productJson: "{}", durationUs: 1000 };
    assert.deepEqual(parseLspDiagramResult(value), value);
    assert.throws(() => parseLspDiagramResult({ modelDigest: "blake3:model", durationUs: 1 }));
    assert.throws(() => parseLspDiagramResult(null));
  });

  it("validates typed state-transition view choices", () => {
    const value = {
      modelDigest: "blake3:model",
      views: [{
        handle: "view:one",
        semanticId: "semantic:one",
        name: "operations",
        exposedMachine: { semanticId: "machine:one", label: "Operations" },
        source: { uri: "file:///workspace/model.sysml", range: {} },
      }],
    };
    assert.deepEqual(parseStateTransitionViewCatalog(value), {
      modelDigest: value.modelDigest,
      views: [{ ...value.views[0], source: { uri: value.views[0].source.uri } }],
    });
    assert.equal(parseStateTransitionViewCatalog({
      modelDigest: value.modelDigest,
      views: [{
        handle: "view:legacy",
        semantic_id: "semantic:legacy",
        name: "legacy",
        exposed_machine: { semantic_id: "machine:legacy", label: "Legacy" },
        source: { uri: "file:///workspace/model.sysml" },
      }],
    }).views[0].exposedMachine.label, "Legacy");
    assert.throws(() => parseStateTransitionViewCatalog({ ...value, views: [{ name: "missing identity" }] }));
  });

  it("offers only diagram capabilities authored by the active document", () => {
    const catalog = parseDiagramViewCatalog({
      modelDigest: "blake3:model",
      views: [{
        handle: "view:state",
        kind: "state-transition-view",
        reference: { kind: "qualified-name", document: "file:///workspace/views.sysml", qualifiedName: "P::lifecycle", sourceDomain: "workspace" },
        name: "lifecycle",
        source: { uri: "file:///workspace/views.sysml" },
      }, {
        handle: "view:general",
        kind: "general-view",
        reference: { kind: "qualified-name", document: "file:///workspace/views.sysml", qualifiedName: "P::structure", sourceDomain: "workspace" },
        name: "structure",
        source: { uri: "file:///workspace/views.sysml" },
      }, {
        handle: "view:grid",
        kind: "grid-view",
        reference: { kind: "qualified-name", document: "file:///workspace/other.sysml", qualifiedName: "Q::matrix", sourceDomain: "workspace" },
        name: "matrix",
        source: { uri: "file:///workspace/other.sysml" },
      }],
    });
    assert.deepEqual(
      diagramViewsForDocument(catalog, "file:///workspace/views.sysml").map((view) => view.id),
      ["general-view", "state-transition-view"]
    );
    assert.deepEqual(diagramViewsForDocument(catalog, "file:///workspace/structure.sysml"), []);
  });

  it("explains an oversized product by its size and element counts", () => {
    const message = oversizedProductMessage(JSON.stringify({
      projection: { nodes: [{}, {}], edges: [{}], relationships: [] },
    }), 20 * 1024 * 1024);
    assert.match(message, /20\.0 MiB \(2 elements, 1 edges, 0 relationships\), over the 16\.0 MiB viewer limit/);
    assert.match(oversizedProductMessage("{", 17 * 1024 * 1024), /^This view's diagram product is 17\.0 MiB, over/);
    assert.throws(
      () => parseDiagramProduct(" ".repeat(MAX_DIAGRAM_PRODUCT_BYTES + 1)),
      /over the 16\.0 MiB viewer limit/,
    );
  });

  it("validates the versioned render product and explicit incompleteness", () => {
    const value = {
      schemaVersion: DIAGRAM_PRODUCT_SCHEMA_VERSION,
      modelDigest: "blake3:model",
      documents: [{ uri: "file:///workspace/model.sysml", sourceDomain: "workspace" }],
      sources: [{ document: 0, range: [0, 0, 0, 1] }],
      references: [{ kind: "qualified-name", document: 0, qualifiedName: "P::general" }],
      selectedView: {
        reference: 0,
        kind: "general-view",
        name: "General View",
        source: 0,
      },
      completeness: {
        status: "incomplete",
        reasons: [{ code: "parse-recovery" }],
      },
      projection: {
        kind: "general-view",
        exposedRoots: [],
        nodes: [],
        relationships: [],
        edges: [],
        metadata: { roots: [] },
        scene: { kind: "general" },
      },
    };
    const product = parseDiagramProduct(JSON.stringify(value));
    assert.deepEqual(product, value);
    assert.equal(isEmptyIncompleteDiagramProduct(product), true);
    assert.equal(isEmptyIncompleteDiagramProduct({
      ...product,
      projection: {
        ...product.projection,
        nodes: [{ reference: 0, metaclass: "PartUsage", name: "visible", owner: null, source: 0 }],
      },
    }), false);
    const node = {
      reference: 0,
      metaclass: "PartUsage",
      notationRole: "usage",
      name: "visible",
      owner: null,
      source: 0,
      typing: { status: "absent" },
      compartments: [{ kind: "parts", provenance: "direct", members: [0] }],
    };
    assert.equal(parseDiagramProduct(JSON.stringify({
      ...value,
      projection: { ...value.projection, exposedRoots: [0], nodes: [node] },
    })).projection.nodes.length, 1);
    const typedNodes = {
      ...value,
      projection: {
        ...value.projection,
        exposedRoots: [0],
        nodes: [{ ...node, typing: { status: "resolved", types: [{ reference: 0, label: "P" }] } }],
      },
    };
    assert.equal(parseDiagramProduct(JSON.stringify(typedNodes)).projection.nodes.length, 1);
    assert.throws(() => parseDiagramProduct(JSON.stringify({
      ...typedNodes,
      projection: { ...typedNodes.projection, nodes: [{ ...node, typing: undefined }] },
    })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({
      ...typedNodes,
      projection: {
        ...typedNodes.projection,
        nodes: [{ ...node, typing: { status: "resolved", types: [] } }],
      },
    })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({
      ...value,
      projection: { ...value.projection, nodes: [{ ...node, compartments: undefined }] },
    })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, schemaVersion: 4 })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, schemaVersion: 5 })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, schemaVersion: 6 })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, schemaVersion: 8 })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, schemaVersion: 9 })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, projection: { ...value.projection, kind: "grid-view" } })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, selectedView: { ...value.selectedView, reference: 1 } })));
    assert.throws(() => parseDiagramProduct(JSON.stringify({ ...value, sources: [{ document: 1, range: [0, 0, 0, 1] }] })));
    const withReason = (reason: unknown) => JSON.stringify({
      ...value,
      completeness: { status: "incomplete", reasons: [reason] },
      projection: { ...value.projection, exposedRoots: [0], nodes: [node] },
    });
    assert.equal(parseDiagramProduct(withReason({ code: "sequence-content-omitted", elements: [0] })).completeness.reasons.length, 1);
    assert.equal(parseDiagramProduct(withReason({
      code: "sequence-ordering-cycle",
      witness: [{ source: 0, target: 0, events: [0], provenance: "implied" }],
    })).completeness.reasons.length, 1);
    assert.throws(() => parseDiagramProduct(withReason({ code: "sequence-content-omitted", elements: [1] })));
    assert.throws(() => parseDiagramProduct(withReason({ code: "sequence-content-omitted" })));
    assert.throws(() => parseDiagramProduct(withReason({ code: "sequence-ordering-cycle" })));
    assert.throws(() => parseDiagramProduct(withReason({
      code: "sequence-ordering-cycle",
      witness: [{ source: 0, target: 3, events: [], provenance: "authored" }],
    })));
    assert.throws(() => parseDiagramProduct(withReason({
      code: "sequence-ordering-cycle",
      witness: [{ source: 0, target: 0, provenance: "authored" }],
    })));
  });

  it("lists every authored view in the model, grouped by canonical namespace and keyed by handle", () => {
    const catalog = parseDiagramViewCatalog({
      modelDigest: "blake3:model",
      views: [{
        handle: "view:b",
        kind: "sequence-view",
        reference: { kind: "qualified-name", document: "file:///w/views.sysml", qualifiedName: "P::flow", sourceDomain: "workspace" },
        namespace: "P",
        name: "checkoutFlow",
        source: { uri: "file:///w/views.sysml" },
      }, {
        handle: "view:a",
        kind: "general-view",
        reference: { kind: "qualified-name", document: "file:///w/views.sysml", qualifiedName: "P::structure", sourceDomain: "workspace" },
        namespace: "P",
        name: "structure",
        source: { uri: "file:///w/views.sysml" },
      }, {
        handle: "view:elsewhere",
        kind: "grid-view",
        reference: { kind: "qualified-name", document: "file:///w/aaa.sysml", qualifiedName: "Q::matrix", sourceDomain: "workspace" },
        namespace: "Q",
        name: "matrix",
        source: { uri: "file:///w/aaa.sysml" },
      }],
    });
    const all = allDiagramViewOptions(catalog);
    // Namespace and view-name order take precedence over file and kind.
    assert.deepEqual(all.map((option) => option.handle), ["view:b", "view:a", "view:elsewhere"]);
    assert.deepEqual(all.map((option) => option.group), ["P", "P", "Q"]);
    assert.equal(all[1].label, "structure");

    const authored = authoredDiagramViewOptions(catalog, "file:///w/views.sysml");
    assert.deepEqual(authored.map((option) => option.handle), ["view:b", "view:a"]);
    assert.equal(diagramViewKindForHandle(catalog, "view:b"), "sequence-view");
    assert.equal(diagramViewKindForHandle(catalog, "missing"), undefined);
  });

  it("keeps the remembered view when it survives a catalog refresh, else falls back", () => {
    const options = [
      { handle: "h1", kind: "general-view" as const, name: "a", label: "a · General View", documentUri: "file:///w/x.sysml", group: "x.sysml" },
      { handle: "h2", kind: "sequence-view" as const, name: "b", label: "b · Sequence View", documentUri: "file:///w/x.sysml", group: "x.sysml" },
    ];
    assert.equal(resolveDiagramSelection(options, "h2")?.handle, "h2");
    assert.equal(resolveDiagramSelection(options, "gone")?.handle, "h1");
    assert.equal(resolveDiagramSelection([], "h1"), undefined);
  });

  it("uses canonical namespace and source ranges without splitting quoted qualified names", () => {
    const catalog = parseDiagramViewCatalog({modelDigest:"m", views:[{
      handle:"h",kind:"general-view",name:"A::quoted name",namespace:"P::'Namespace::with separators'",
      reference:{kind:"qualified-name",document:"file:///w/views.sysml",qualifiedName:"opaque",sourceDomain:"workspace"},
      source:{uri:"file:///w/views.sysml",range:{startLine:10,startCharacter:4,endLine:12,endCharacter:5}},
    }]});
    const [view] = allDiagramViewOptions(catalog);
    assert.equal(view.group, "P::'Namespace::with separators'");
    assert.equal(view.label, "A::quoted name");
    assert.equal(view.kindLabel, "General View");
    assert.deepEqual(view.definition, {uri:"file:///w/views.sysml",startLine:10,startCharacter:4,endLine:12,endCharacter:5});
    const root = allDiagramViewOptions({...catalog, views:[{...catalog.views[0], namespace:null}]});
    assert.equal(root[0].group,"(Root namespace)");
    assert.equal(allDiagramViewOptions({...catalog, views:[{...catalog.views[0], namespace:""}]} )[0].group, "(Unnamed namespace)");
    const legacy = allDiagramViewOptions({...catalog, views:[{...catalog.views[0], namespace:undefined, source:{uri:"file:///w/views.sysml"}}]});
    assert.equal(legacy[0].group,"(Namespace unavailable)");
    assert.equal(legacy[0].definition, undefined);
    assert.throws(()=>parseDiagramViewCatalog({...catalog, views:[{...catalog.views[0], namespace:42}]}));
    assert.throws(()=>parseDiagramViewCatalog({...catalog, views:[{...catalog.views[0], source:{uri:"file:///w/views.sysml",range:{startLine:-1}}}]}));
    const multiple = allDiagramViewOptions({...catalog, views:[catalog.views[0], {...catalog.views[0],handle:"h2",kind:"sequence-view"}]});
    assert.deepEqual(multiple.map(view=>view.label),["A::quoted name · General View", "A::quoted name · Sequence View"]);
  });

  it("backs the reconcile loop off exponentially, then settles slow", () => {
    assert.equal(reconcileDelayMs(0, false), 600);
    assert.equal(reconcileDelayMs(1, false), 1_200);
    assert.equal(reconcileDelayMs(4, false), 9_600);
    assert.equal(reconcileDelayMs(6, false), 15_000);
    assert.equal(reconcileDelayMs(20, false), 15_000, "capped");
    assert.equal(reconcileDelayMs(-3, false), 600, "clamped to attempt 0");
    assert.equal(reconcileDelayMs(0, true), 45_000, "settled cadence ignores attempt");
  });

  it("treats a publication digest as stale only against a known rendered digest", () => {
    assert.equal(diagramRenderIsStale(undefined, "blake3:new"), false);
    assert.equal(diagramRenderIsStale("blake3:old", "blake3:new"), true);
    assert.equal(diagramRenderIsStale("blake3:same", "blake3:same"), false);
  });

  it("validates bounded source navigation", () => {
    const valid = { uri: "file:///w/model.sysml", startLine: 1, startCharacter: 2, endLine: 1, endCharacter: 4 };
    assert.deepEqual(parseSourceNavigation(valid), valid);
    assert.equal(parseSourceNavigation({ ...valid, startLine: -1 }), undefined);
    assert.equal(parseSourceNavigation({ ...valid, endCharacter: 1 }), undefined);
  });

  it("confines source paths to workspace roots", () => {
    assert.equal(isPathInsideWorkspace("/workspace/model.sysml", ["/workspace"]), true);
    assert.equal(isPathInsideWorkspace("/workspace-other/model.sysml", ["/workspace"]), false);
    assert.equal(isPathInsideWorkspace("/outside/model.sysml", ["/workspace"]), false);
  });
});
