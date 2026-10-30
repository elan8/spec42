// @vitest-environment jsdom
import { expect, it, vi } from "vitest";
import type { DisclosureState, ViewportTransform, RenderOptions } from "./render/types";

type MountedController = {
  viewport: ViewportTransform; disclosure: DisclosureState;
  getViewport(): ViewportTransform; getDisclosureState(): DisclosureState;
  destroy: ReturnType<typeof vi.fn>; reset: ReturnType<typeof vi.fn>;
};
const harness = vi.hoisted(() => ({ calls: [] as RenderOptions[], controllers: [] as MountedController[], blocked: undefined as Promise<void> | undefined }));
vi.mock("./prepare", () => ({ prepareViewData: () => ({title:"View",view:"general-view",nodes:[],edges:[]}) }));
vi.mock("./renderer", () => ({
  isNativeDiagramView: () => true,
  renderVisualization: async (_canvas: HTMLElement, _prepared: unknown, options: RenderOptions) => {
    harness.calls.push(options);
    const controller: MountedController = {
      viewport: options.viewport ?? {x:0,y:0,k:1},
      disclosure: options.disclosureState ?? {expandedNodeIds:[],sectionStates:[]},
      getViewport() { return this.viewport; },
      getDisclosureState() { return this.disclosure; },
      destroy: vi.fn(), reset: vi.fn(),
    };
    harness.controllers.push(controller);
    const blocked = harness.blocked;
    harness.blocked = undefined;
    if (blocked) await blocked;
    return controller;
  },
}));

it("retains the last mounted non-default presentation through supersession/recovery and isolates views", async () => {
  document.body.innerHTML = ["diagram","status","home","copy-json","view-definition","view-kind","export-svg","export-png","limitations"]
    .map(id => `<div id="${id}"></div>`).join("") + '<select id="view-select"></select><input id="transparent-export" type="checkbox">';
  Object.defineProperty(document, "visibilityState", {value:"visible",configurable:true});
  const postMessage = vi.fn();
  vi.stubGlobal("acquireVsCodeApi", () => ({postMessage}));
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => setTimeout(() => callback(performance.now()), 0));
  vi.stubGlobal("cancelAnimationFrame", (id: ReturnType<typeof setTimeout>) => clearTimeout(id));
  await import("../../src/diagram/diagramWebview");
  let sequence = 0;
  const send = (keys: string[], handle = "view:a") => {
    const renderId = ++sequence;
    const modelDigest = `model:${sequence}`;
    window.dispatchEvent(new MessageEvent("message", {data:{type:"render", views:[], selectedHandle:handle, header:"View",
      measurement:{renderId,modelDigest,viewHandle:handle}, productJson:JSON.stringify({modelDigest,
        selectedView:{kind:"general-view"}, completeness:{status:"complete",reasons:[]},
        projection:{kind:"general-view",nodes:keys.map(presentationKey=>({presentationKey}))}})}}));
    return renderId;
  };
  const complete = async (id: number) => vi.waitFor(() => expect(postMessage.mock.calls.some(([event]) => event.type === "renderComplete" && event.renderId === id)).toBe(true));
  await complete(send(["a", "b"]));
  const viewport: ViewportTransform = {x:125,y:-40,k:1.8};
  const disclosure: DisclosureState = {expandedNodeIds:["n:1"],sectionStates:[{nodeId:"n:1",sectionKey:"parts",expanded:false}]};
  harness.controllers[0].viewport = viewport;
  harness.controllers[0].disclosure = disclosure;

  let release!: () => void;
  harness.blocked = new Promise<void>(resolve => { release = resolve; });
  const superseded = send(["inserted", "a", "b"]);
  await vi.waitFor(() => expect(harness.calls).toHaveLength(2));
  expect(harness.calls[1].viewport).toEqual(viewport);
  expect(harness.calls[1].disclosureState?.expandedNodeIds).toEqual(["n:2"]);
  const current = send(["b", "a", "inserted"]);
  await complete(current);
  expect(harness.calls[2].viewport).toEqual(viewport);
  expect(harness.calls[2].disclosureState).toEqual({expandedNodeIds:["n:0"],sectionStates:[{nodeId:"n:0",sectionKey:"parts",expanded:false}]});
  expect(postMessage.mock.calls.find(([event]) => event.renderId === current)?.[0]).toMatchObject({viewportReused:true,disclosureReused:true});
  release();
  await vi.waitFor(() => expect(harness.controllers[1].destroy).toHaveBeenCalledOnce());
  expect(postMessage.mock.calls.some(([event]) => event.type === "renderComplete" && event.renderId === superseded)).toBe(false);

  window.dispatchEvent(new MessageEvent("message", {data:{type:"render",views:[],selectedHandle:"view:a",header:"Loading",placeholder:"Loading",loading:true}}));
  await complete(send(["a", "b"]));
  expect(harness.calls[3].viewport).toEqual(viewport);
  expect(harness.calls[3].disclosureState).toEqual(disclosure);
  window.dispatchEvent(new MessageEvent("message", {data:{type:"render",views:[],selectedHandle:"view:a",header:"Recovery",productJson:"invalid"}}));
  await complete(send(["b", "a"]));
  expect(harness.calls[4].disclosureState?.expandedNodeIds).toEqual(["n:0"]);
  harness.blocked = new Promise<void>(resolve => { release = resolve; });
  const themeSuperseded = send(["a", "b"]);
  await vi.waitFor(() => expect(harness.calls).toHaveLength(6));
  document.body.classList.add("vscode-dark");
  await vi.waitFor(() => expect(harness.calls).toHaveLength(7));
  expect(harness.calls[6].viewport).toEqual(viewport);
  expect(harness.calls[6].disclosureState).toEqual(disclosure);
  release();
  await vi.waitFor(() => expect(harness.controllers[5].destroy).toHaveBeenCalledOnce());
  expect(postMessage.mock.calls.some(([event]) => event.type === "renderComplete" && event.renderId === themeSuperseded)).toBe(false);
  await complete(send(["b", "a"], "view:other"));
  expect(harness.calls[7].viewport).toBeUndefined();
  expect(harness.calls[7].disclosureState).toBeUndefined();
});
