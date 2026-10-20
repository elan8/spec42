// @vitest-environment jsdom
import {describe,it,expect} from "vitest";
import {populateViewSelect,withExportBackground} from "../../src/diagram/viewPresentation";

describe("Views panel presentation",()=>{
  it("keeps namespace sections even for one group and labels markup as text",()=>{
    const select=document.createElement("select");
    const views=[{handle:"h1",label:"<view>",group:"P::Nested",kindLabel:"General View"},
      {handle:"h2",label:"Other",group:"P::Nested",kindLabel:"Sequence View"}];
    populateViewSelect(select,views,"h2");
    expect(select.querySelectorAll("optgroup")).toHaveLength(1);
    expect(select.querySelector("optgroup")?.label).toBe("P::Nested");
    expect(select.options[0].textContent).toBe("<view>");
    expect(select.value).toBe("h2");
    expect(select.options[1].title).toBe("Sequence View");
    populateViewSelect(select,[views[0]],"h1");
    expect(select.disabled).toBe(false);
    populateViewSelect(select,[],"");
    expect(select.disabled).toBe(true);
  });

  const svg='<svg xmlns="http://www.w3.org/2000/svg" viewBox="-20 -10 200 100"><rect class="viz-bg" fill="#f6f7f9"/><g class="view-frame"><rect class="node-background" fill="#ffffff"/></g></svg>';
  it("exports a white canvas at the exact viewBox bounds by default",()=>{
    const parsed=new DOMParser().parseFromString(withExportBackground(svg,false),"image/svg+xml");
    const bg=parsed.querySelector(".view-export-background")!;
    expect(["x","y","width","height","fill"].map(key=>bg.getAttribute(key))).toEqual(["-20","-10","200","100","#ffffff"]);
    expect(parsed.querySelector(".viz-bg")).toBeNull();
    expect(parsed.querySelector(".node-background")?.getAttribute("fill")).toBe("#ffffff");
    expect(parsed.querySelectorAll(".view-frame")).toHaveLength(1);
  });
  it("exports a transparent canvas without changing semantic fills",()=>{
    const result=withExportBackground(withExportBackground(svg,false),true);
    const parsed=new DOMParser().parseFromString(result,"image/svg+xml");
    expect(parsed.querySelector(".view-export-background,.viz-bg")).toBeNull();
    expect(parsed.querySelector(".node-background")?.getAttribute("fill")).toBe("#ffffff");
    expect(parsed.documentElement.getAttribute("viewBox")).toBe("-20 -10 200 100");
    expect(()=>withExportBackground("<html/>",false)).toThrow();
    expect(()=>withExportBackground('<svg viewBox="0 0 0 0"/>',false)).toThrow();
  });
});
